//! Сетевой поток клиента (M1): логин offline → Configuration → Play, пакеты мира кладутся в общий `Level`,
//! главному потоку уходят события через crossbeam-канал. Сетевых паник нет: любая ошибка → `Disconnected`.
use crossbeam_channel::{unbounded, Receiver, Sender};
use ezzmc_net::play::{MoveTracker, PlayEvent, PlayHandler};
use ezzmc_net::session::{configure, login_offline, ClientInformation, ConfigOutcome, PacketIds};
use ezzmc_net::{Connection, Intent};
use ezzmc_protocol::{Direction, State};
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::dimension::DimensionType;
use ezzmc_world::level::{Level, LevelEvent};
use ezzmc_world::palette::ceil_log2;
use std::sync::{Arc, Mutex};

/// Протокол 26.3.
const PROTOCOL: i32 = 777;

/// Команды главного потока сетевому.
pub enum NetCmd {
    PlayerAction {
        action: i32,
        pos: [i32; 3],
        face: u8,
        seq: i32,
    },
    Swing,
    /// 232: клик по слоту контейнера.
    ContainerClick {
        container: i32,
        state_id: i32,
        slot: i16,
        button: i8,
        input: i32,
    },
    ContainerClose(i32),
    /// 233: выбран слот хотбара (0–8).
    SetCarried(i16),
    Attack {
        entity_id: i32,
    },
    UseItemOn {
        pos: [i32; 3],
        face: u8,
        rel: [f32; 3],
        inside: bool,
        seq: i32,
    },
}

pub enum NetEvt {
    /// Пакет инвентаря/контейнера (230c).
    Inventory(crate::inventory::InvEvent),
    /// GameType локального игрока (Login/Respawn/game_event 3).
    GameMode(i8),
    Joined {
        biomes: Vec<Option<BiomeColors>>,
        min_y: i32,
        height: i32,
        view_radius: i32,
    },
    Teleport {
        pos: [f64; 3],
        vel: [f64; 3],
        yaw: f32,
        pitch: f32,
    },
    Abilities {
        flags: u8,
        fly_speed: f32,
    },
    /// Set Entity Motion для локального игрока.
    Motion([f64; 3]),
    Level(LevelEvent),
    Disconnected(String),
}

pub type SharedLevel = Arc<Mutex<Option<Level>>>;

/// Положение игрока для сервера: x, y, z, yaw, pitch, флаги. `None` — телепорт при входе ещё не пришёл.
/// Флаги (битовая маска в f64): `FLAG_*`, байт player_input — со сдвигом `FLAG_INPUT_SHIFT`.
pub const FLAG_ON_GROUND: u32 = 1;
pub const FLAG_HCOL: u32 = 2;
pub const FLAG_SPRINTING: u32 = 4;
pub const FLAG_FLYING: u32 = 8;
pub const FLAG_INPUT_SHIFT: u32 = 8;

pub type SharedPos = Arc<Mutex<Option<[f64; 6]>>>;

pub struct NetHandle {
    pub rx: Receiver<NetEvt>,
    pub pos: SharedPos,
    pub level: SharedLevel,
    pub cmd: Sender<NetCmd>,
}

pub fn spawn(addr: String, name: String) -> NetHandle {
    let (tx, rx) = unbounded();
    let level: SharedLevel = Arc::new(Mutex::new(None));
    let lv = level.clone();
    let pos: SharedPos = Arc::new(Mutex::new(None));
    let pp = pos.clone();
    let (cmd_tx, cmd_rx) = unbounded();
    let _ = std::thread::Builder::new()
        .name("net".into())
        .spawn(move || {
            if let Err(e) = run(&addr, &name, &tx, &lv, &pp, &cmd_rx) {
                let _ = tx.send(NetEvt::Disconnected(e));
            }
        });
    NetHandle {
        rx,
        pos,
        level,
        cmd: cmd_tx,
    }
}

fn run(
    addr: &str,
    name: &str,
    tx: &Sender<NetEvt>,
    shared: &SharedLevel,
    pos: &SharedPos,
    cmds: &Receiver<NetCmd>,
) -> Result<(), String> {
    let e = |x: &dyn std::fmt::Display| x.to_string();
    let ids = PacketIds::v26_3();
    let (host, port) = ezzmc_net::srv::parse_address(addr);
    let mut c = Connection::connect(addr).map_err(|x| e(&x))?;
    c.handshake(PROTOCOL, &host, port, Intent::Login)
        .map_err(|x| e(&x))?;
    login_offline(&mut c, &ids, name).map_err(|x| e(&x))?;
    let ConfigOutcome::Play(cfg) = configure(
        &mut c,
        &ids,
        &ClientInformation {
            // VANILLA: Options#renderDistance по умолчанию 12 (отладка: EZZMC_RENDER_DISTANCE).
            view_distance: std::env::var("EZZMC_RENDER_DISTANCE")
                .ok()
                .and_then(|v| v.parse::<i8>().ok())
                .map_or(12, |v| v.clamp(2, 32)),
            ..ClientInformation::default()
        },
    )
    .map_err(|x| e(&x))?
    else {
        return Err("сервер просит transfer (не поддержано в M1)".into());
    };
    let reg = |n: &str| cfg.registries.iter().find(|r| r.registry == n);
    let (Some(dims), Some(biomes)) = (
        reg("minecraft:dimension_type"),
        reg("minecraft:worldgen/biome"),
    ) else {
        return Err("в Configuration нет dimension_type/biome".into());
    };
    let biome_colors: Vec<Option<BiomeColors>> = biomes
        .entries
        .iter()
        .map(|(_, t)| t.as_ref().and_then(BiomeColors::from_nbt))
        .collect();
    // Id часов overworld в реестре world_clock (holder = индекс в порядке registry_data).
    let clock_id = reg("minecraft:world_clock").and_then(|r| {
        r.entries
            .iter()
            .position(|(n, _)| n == "minecraft:overworld")
            .map(|i| i as i32)
    });
    let mut h = PlayHandler::new(ids);
    let mut loaded_sent = false;
    // VANILLA: LocalPlayer#sendChanges каждый клиентский тик: player_input при изменении, команда спринта,
    // затем один из MovePlayer (PosRot/Pos/Rot/StatusOnly) по порогу 2e-4 и heartbeat раз в 20 тиков.
    // Без этого сервер считает игрока стоящим на месте и не шлёт новые чанки.
    let mut tracker = MoveTracker::new([f64::NAN; 3], [f32::NAN; 2]);
    let mut last_input = 0u8;
    let mut was_sprinting = false;
    let mut was_flying = false;
    let mut view_radius = 12;
    // VANILLA: Minecraft#runTick — тики по фиксированной сетке 50 мс с догоном (не «50 мс после прошлого»),
    // иначе game_time клиента отстаёт от серверного и текстуры дождя/снега прыгают при каждом set_time.
    let tick_len = std::time::Duration::from_millis(50);
    let mut next_tick = std::time::Instant::now() + tick_len;
    loop {
        let wait = next_tick.saturating_duration_since(std::time::Instant::now());
        let polled = c.poll(wait.max(std::time::Duration::from_millis(1)));
        while let Ok(cmd) = cmds.try_recv() {
            match cmd {
                NetCmd::PlayerAction {
                    action,
                    pos: bp,
                    face,
                    seq,
                } => h
                    .send_player_action(&mut c, action, bp, face, seq)
                    .map_err(|x| e(&x))?,
                NetCmd::Attack { entity_id } => {
                    h.send_attack(&mut c, entity_id).map_err(|x| e(&x))?
                }
                NetCmd::ContainerClick {
                    container,
                    state_id,
                    slot,
                    button,
                    input,
                } => h
                    .send_container_click(&mut c, container, state_id, slot, button, input)
                    .map_err(|x| e(&x))?,
                NetCmd::ContainerClose(id) => {
                    h.send_container_close(&mut c, id).map_err(|x| e(&x))?
                }
                NetCmd::SetCarried(s) => h.send_set_carried_item(&mut c, s).map_err(|x| e(&x))?,
                NetCmd::Swing => h.send_swing(&mut c).map_err(|x| e(&x))?,
                NetCmd::UseItemOn {
                    pos: bp,
                    face,
                    rel,
                    inside,
                    seq,
                } => h
                    .send_use_item_on(&mut c, bp, face, rel, inside, seq)
                    .map_err(|x| e(&x))?,
            }
        }
        let cur = pos.lock().ok().and_then(|g| *g);
        let mut ticked = false;
        while std::time::Instant::now() >= next_tick {
            if let Ok(mut g) = shared.lock() {
                if let Some(l) = g.as_mut() {
                    l.tick_time_at(next_tick);
                }
            }
            next_tick += tick_len;
            ticked = true;
            // Отстали больше чем на секунду (пауза/нагрузка) — не догоняем.
            if next_tick + std::time::Duration::from_secs(1) < std::time::Instant::now() {
                next_tick = std::time::Instant::now() + tick_len;
            }
        }
        if ticked {
            if let Some(cur) = cur {
                let flags = cur[5] as u32;
                let (ground, hcol) = (flags & FLAG_ON_GROUND != 0, flags & FLAG_HCOL != 0);
                let sprinting = flags & FLAG_SPRINTING != 0;
                let input = (flags >> FLAG_INPUT_SHIFT) as u8;
                if input != last_input {
                    h.send_player_input(&mut c, input).map_err(|x| e(&x))?;
                    last_input = input;
                }
                // VANILLA: Player#onUpdateAbilities → ServerboundPlayerAbilitiesPacket при смене полёта.
                let flying = flags & FLAG_FLYING != 0;
                if flying != was_flying {
                    h.send_abilities(&mut c, flying).map_err(|x| e(&x))?;
                    was_flying = flying;
                }
                if sprinting != was_sprinting {
                    h.send_sprint(&mut c, sprinting).map_err(|x| e(&x))?;
                    was_sprinting = sprinting;
                }
                h.player.pos = ezzmc_protocol::types::Vec3::new(cur[0], cur[1], cur[2]);
                h.player.yaw = cur[3] as f32;
                h.player.pitch = cur[4] as f32;
                if let Some(kind) = tracker.step(
                    [cur[0], cur[1], cur[2]],
                    [cur[3] as f32, cur[4] as f32],
                    ground,
                    hcol,
                ) {
                    h.send_move(&mut c, kind, ground, hcol).map_err(|x| e(&x))?;
                }
                h.send_client_tick_end(&mut c).map_err(|x| e(&x))?;
            }
        }
        let Some(p) = polled.map_err(|x| e(&x))? else {
            continue;
        };
        let pid = p.id;
        let pname = ids.name_of(State::Play, Direction::Clientbound, p.id);
        let body = p.body.clone();
        let ev = h.handle(&mut c, p).map_err(|x| e(&x))?;
        match ev {
            PlayEvent::Login(_) | PlayEvent::Respawn { .. } => {
                // VANILLA: handleLogin / handleRespawn — новый ClientLevel (смена измерения).
                let (dim_type, radius) = match &ev {
                    PlayEvent::Login(l) => {
                        let _ = tx.send(NetEvt::GameMode(l.spawn.game_type));
                        view_radius = l.chunk_radius;
                        (l.spawn.dimension_type, l.chunk_radius)
                    }
                    PlayEvent::Respawn { spawn, .. } => {
                        // После экрана загрузки ваниль снова шлёт Player Loaded.
                        loaded_sent = false;
                        let _ = tx.send(NetEvt::GameMode(spawn.game_type));
                        (spawn.dimension_type, view_radius)
                    }
                    _ => continue,
                };
                let tag = dims
                    .entries
                    .get(dim_type as usize)
                    .and_then(|x| x.1.as_ref())
                    .ok_or("нет dimension_type в реестре")?;
                let dim = DimensionType::from_nbt(tag).map_err(|x| e(&x))?;
                let fmt = dim.chunk_format(
                    ceil_log2(ezzmc_world::generated::b26_3::STATE_COUNT),
                    ceil_log2(biomes.entries.len() as u32),
                );
                if let Ok(mut g) = shared.lock() {
                    let mut lv = Level::new(fmt, radius);
                    lv.overworld_clock = clock_id;
                    lv.skybox = dim.skybox;
                    *g = Some(lv);
                }
                let _ = tx.send(NetEvt::Joined {
                    biomes: biome_colors.clone(),
                    min_y: dim.min_y,
                    height: dim.height,
                    view_radius: radius,
                });
            }
            PlayEvent::Abilities {
                flags, fly_speed, ..
            } => {
                was_flying = flags & 2 != 0;
                let _ = tx.send(NetEvt::Abilities { flags, fly_speed });
            }
            PlayEvent::Motion(v) => {
                let _ = tx.send(NetEvt::Motion(v));
            }
            PlayEvent::Teleport { state, .. } => {
                // Позиция и событие уходят под одним локом `pos`: главный поток берёт лок до разбора
                // событий, поэтому старая камера не затрёт телепорт (иначе «moved too quickly» и откат).
                if let Ok(mut g) = pos.lock() {
                    *g = Some([
                        state.pos.x,
                        state.pos.y,
                        state.pos.z,
                        f64::from(state.yaw),
                        f64::from(state.pitch),
                        0.0,
                    ]);
                    // VANILLA: handleMovePlayer обновляет xLast/yRotLast — следующий пакет считается от телепорта.
                    tracker.reset(
                        [state.pos.x, state.pos.y, state.pos.z],
                        [state.yaw, state.pitch],
                    );
                    let _ = tx.send(NetEvt::Teleport {
                        pos: [state.pos.x, state.pos.y, state.pos.z],
                        vel: [state.delta.x, state.delta.y, state.delta.z],
                        yaw: state.yaw,
                        pitch: state.pitch,
                    });
                }
            }
            PlayEvent::Disconnect(t) => return Err(t),
            PlayEvent::GameEvent { event, param } => {
                tracing::trace!("game_event {event} {param}");
                if event == ezzmc_net::play::game_event::CHANGE_GAME_MODE {
                    let _ = tx.send(NetEvt::GameMode(param as i8));
                }
                // Погодные события (дождь/гроза) разбирает Level по сырому телу пакета.
                if let Ok(mut g) = shared.lock() {
                    if let Some(l) = g.as_mut() {
                        if let Ok(LevelEvent::WeatherChanged) = l.handle("game_event", &body) {
                            tracing::debug!(
                                "погода: raining={} rain={} thunder={}",
                                l.raining,
                                l.rain_level,
                                l.thunder_level
                            );
                        }
                    }
                }
            }
            PlayEvent::Other(_) => {
                let Some(pname) = pname else { continue };
                match crate::inventory::parse_packet(pname, &body) {
                    Ok(Some(iev)) => {
                        let _ = tx.send(NetEvt::Inventory(iev));
                        continue;
                    }
                    Ok(None) => {}
                    Err(err) => {
                        tracing::warn!("{pname}: {err} [len={}]", body.len());
                        continue;
                    }
                }
                let ev = {
                    let Ok(mut g) = shared.lock() else { continue };
                    let Some(level) = g.as_mut() else { continue };
                    match level.handle(pname, &body) {
                        Ok(ev) => ev,
                        Err(err) => {
                            tracing::warn!(
                                "{pname}: {err} [id={} len={} head={:02x?}]",
                                pid,
                                body.len(),
                                &body[..body.len().min(24)]
                            );
                            continue;
                        }
                    }
                };
                match &ev {
                    LevelEvent::BatchFinished { .. } => {
                        h.send_chunk_batch_received(&mut c, 9.0)
                            .map_err(|x| e(&x))?;
                    }
                    LevelEvent::ChunkLoaded { .. } if !loaded_sent => {
                        loaded_sent = true;
                        h.send_player_loaded(&mut c).map_err(|x| e(&x))?;
                    }
                    LevelEvent::WeatherChanged => {
                        if let Ok(g) = shared.lock() {
                            if let Some(l) = g.as_ref() {
                                tracing::debug!(
                                    "погода: raining={} rain={} thunder={}",
                                    l.raining,
                                    l.rain_level,
                                    l.thunder_level
                                );
                            }
                        }
                    }
                    _ => {}
                }
                if !matches!(ev, LevelEvent::None) {
                    let _ = tx.send(NetEvt::Level(ev));
                }
            }
            _ => {}
        }
    }
}
