//! Сетевой поток клиента (M1): логин offline → Configuration → Play, пакеты мира кладутся в общий `Level`,
//! главному потоку уходят события через crossbeam-канал. Сетевых паник нет: любая ошибка → `Disconnected`.
use crossbeam_channel::{unbounded, Receiver, Sender};
use ezzmc_net::play::{MoveTracker, PlayEvent, PlayHandler};
use ezzmc_net::session::{configure, login_offline, ConfigOutcome, PacketIds};
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
    /// СКМ по блоку (key.pickItem): pos, includeData (Ctrl в креативе).
    PickBlock([i32; 3], bool),
    /// Возрождение (экран смерти).
    Respawn,
    /// 232: клик по слоту контейнера.
    ContainerClick {
        container: i32,
        state_id: i32,
        slot: i16,
        button: i8,
        input: i32,
        /// Изменённые слоты (предсказанные) и курсор: (id предмета, количество).
        changed: Vec<(i16, Option<(u32, i32)>)>,
        carried: Option<(u32, i32)>,
    },
    ContainerClose(i32),
    /// Реплика чата / команда (без `/`).
    Chat(String),
    Command(String),
    /// Запрос подсказок (id, вся строка ввода).
    Suggest(i32, String),
    /// 232g: слот креатива (-1 — выбросить).
    CreativeSlot(i16, Option<ezzmc_protocol::item::ItemStack>),
    /// 233: выбран слот хотбара (0–8).
    SetCarried(i16),
    /// Смена настроек (дальность, части скина, рука, частицы, цвета чата).
    ClientInfo(Box<ezzmc_net::session::ClientInformation>),
    Attack {
        entity_id: i32,
    },
    UseItem {
        seq: i32,
        yaw: f32,
        pitch: f32,
    },
    UseItemOn {
        pos: [i32; 3],
        face: u8,
        rel: [f32; 3],
        inside: bool,
        seq: i32,
    },
}

pub struct ExplosionEvt {
    pub center: [f64; 3],
    pub radius: f32,
    pub block_count: i32,
    pub knockback: Option<[f64; 3]>,
    pub particle: &'static str,
    /// (имя, scaling, speed, weight)
    pub block_particles: Vec<(String, f32, f32, i32)>,
}

pub enum NetEvt {
    /// Чат: оформленный текст и индикатор; overlay=true — action bar.
    Chat(ezzmc_protocol::text::Component, crate::chat::Tag),
    Overlay(ezzmc_protocol::text::Component),
    /// ClientboundCommandSuggestionsPacket: id, start, length, варианты.
    Suggestions(i32, i32, i32, Vec<String>),
    /// Пакет инвентаря/контейнера (230c).
    Inventory(crate::inventory::InvEvent),
    /// GameType локального игрока (Login/Respawn/game_event 3).
    GameMode(i8),
    /// ClientboundSetHealthPacket: здоровье локального игрока.
    Health(f32, i32, f32),
    /// ClientboundSetExperiencePacket: прогресс, уровень.
    Experience(f32, i32),
    /// UpdateAttributes локального игрока: (имя атрибута без minecraft:, значение).
    Attribute(&'static str, f32),
    Hardcore(bool),
    /// ClientboundTrackedWaypointPacket: операция (0 TRACK, 1 UNTRACK, 2 UPDATE).
    Waypoint(u32, crate::hudbars::Wp),
    Joined {
        biomes: Vec<Option<BiomeColors>>,
        min_y: i32,
        height: i32,
        view_radius: i32,
        /// VANILLA: CommonPlayerSpawnInfo#isFlat → ClientLevelData (горизонт, порог затемнения у пустоты).
        is_flat: bool,
    },
    Teleport {
        pos: [f64; 3],
        vel: [f64; 3],
        yaw: f32,
        pitch: f32,
    },
    /// ClientboundExplodePacket (звук играет сетевой поток).
    Explosion(Box<ExplosionEvt>),
    /// Уровень прав оператора локального игрока (entity event 24..28 → 0..4).
    OpLevel(u8),
    /// entity event 9 (USE_ITEM_COMPLETE) для локального игрока.
    UseComplete,
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
        &crate::settings::client_info(&crate::ui::options::Options::load(std::path::Path::new(
            "./options.txt",
        ))),
    )
    .map_err(|x| e(&x))?
    else {
        return Err("сервер просит transfer (не поддержано в M1)".into());
    };
    let reg = |n: &str| cfg.registries.iter().find(|r| r.registry == n);
    if let Ok(mut g) = crate::mobs::VARIANT_REGS.lock() {
        *g = cfg
            .registries
            .iter()
            .filter(|r| r.registry.ends_with("_variant"))
            .map(|r| {
                (
                    r.registry.clone(),
                    r.entries.iter().map(|(n, _)| n.clone()).collect(),
                )
            })
            .collect();
    }
    let chat_types = crate::chatnet::chat_types(reg("minecraft:chat_type"));
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
    // Зачарования для подсказок: (имя, max_level), порядок = holder id.
    if let (Some(r), Ok(mut g)) = (
        reg("minecraft:enchantment"),
        crate::tooltip::ENCHANTS.lock(),
    ) {
        *g = r
            .entries
            .iter()
            .map(|(n, t)| {
                let max = t
                    .as_ref()
                    .and_then(|t| t.get("max_level"))
                    .and_then(|t| t.as_i64());
                (n.clone(), max.unwrap_or(1) as i32)
            })
            .collect();
    }
    // Трим брони: имена материалов/паттернов в порядке holder id.
    if let Ok(mut g) = crate::armor::TRIM_REGISTRY.lock() {
        let names = |k: &str| {
            reg(k)
                .map(|r| r.entries.iter().map(|(n, _)| n.clone()).collect())
                .unwrap_or_default()
        };
        *g = (names("minecraft:trim_material"), names("minecraft:trim_pattern"));
    }
    // Id часов overworld в реестре world_clock (holder = индекс в порядке registry_data).
    let clock_id = reg("minecraft:world_clock").and_then(|r| {
        r.entries
            .iter()
            .position(|(n, _)| n == "minecraft:overworld")
            .map(|i| i as i32)
    });
    let mut h = PlayHandler::new(ids);
    let mut loaded_sent = false;
    let mut batch_size = ezzmc_net::chunkbatch::ChunkBatchSize::default();
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
                NetCmd::PickBlock(bp, data) => {
                    // Версии до 1.21.5 пакета не знают — молча игнорируем.
                    let _ = h.send_pick_item_from_block(&mut c, bp, data);
                }
                NetCmd::Attack { entity_id } => {
                    h.send_attack(&mut c, entity_id).map_err(|x| e(&x))?
                }
                NetCmd::ContainerClick {
                    container,
                    state_id,
                    slot,
                    button,
                    input,
                    changed,
                    carried,
                } => h
                    .send_container_click(
                        &mut c, container, state_id, slot, button, input, &changed, carried,
                    )
                    .map_err(|x| e(&x))?,
                NetCmd::CreativeSlot(slot, stack) => h
                    .send_set_creative_slot(&mut c, slot, stack.as_ref())
                    .map_err(|x| e(&x))?,
                NetCmd::Chat(m) => h.send_chat(&mut c, &m, unix_ms()).map_err(|x| e(&x))?,
                NetCmd::Command(m) => h
                    .send_chat_command(&mut c, &m, unix_ms())
                    .map_err(|x| e(&x))?,
                NetCmd::Suggest(id, t) => h
                    .send_command_suggestion(&mut c, id, &t)
                    .map_err(|x| e(&x))?,
                NetCmd::ContainerClose(id) => {
                    h.send_container_close(&mut c, id).map_err(|x| e(&x))?
                }
                NetCmd::ClientInfo(i) => {
                    h.send_client_information(&mut c, &i).map_err(|x| e(&x))?
                }
                NetCmd::SetCarried(s) => h.send_set_carried_item(&mut c, s).map_err(|x| e(&x))?,
                NetCmd::Respawn => h.send_respawn(&mut c).map_err(|x| e(&x))?,
                NetCmd::Swing => h.send_swing(&mut c).map_err(|x| e(&x))?,
                NetCmd::UseItem { seq, yaw, pitch } => h
                    .send_use_item(&mut c, seq, yaw, pitch)
                    .map_err(|x| e(&x))?,
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
                    // partial_tick считаем от фактического применения тика: сетевой поток может
                    // запаздывать относительно сетки, и сущности иначе замирают и прыгают.
                    l.tick_time_at(std::time::Instant::now());
                    crate::itemphys::tick(l);
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
                let (dim_type, radius, is_flat) = match &ev {
                    PlayEvent::Login(l) => {
                        crate::itemdyn::set_dimension(&l.spawn.dimension);
                        crate::itemdyn::set_death(
                            l.spawn
                                .last_death
                                .as_ref()
                                .map(|(d, p)| (d.clone(), p.x, p.z)),
                        );
                        let _ = tx.send(NetEvt::GameMode(l.spawn.game_type));
                        let _ = tx.send(NetEvt::Hardcore(l.hardcore));
                        view_radius = l.chunk_radius;
                        (l.spawn.dimension_type, l.chunk_radius, l.spawn.is_flat)
                    }
                    PlayEvent::Respawn { spawn, .. } => {
                        crate::itemdyn::set_dimension(&spawn.dimension);
                        crate::itemdyn::set_death(
                            spawn
                                .last_death
                                .as_ref()
                                .map(|(d, p)| (d.clone(), p.x, p.z)),
                        );
                        // После экрана загрузки ваниль снова шлёт Player Loaded.
                        loaded_sent = false;
                        let _ = tx.send(NetEvt::GameMode(spawn.game_type));
                        (spawn.dimension_type, view_radius, spawn.is_flat)
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
                    lv.audio = ezzmc_world::audio_attr::AudioAttrs::from_attributes(
                        dim.attributes.as_ref(),
                    );
                    *g = Some(lv);
                }
                let _ = tx.send(NetEvt::Joined {
                    biomes: biome_colors.clone(),
                    min_y: dim.min_y,
                    height: dim.height,
                    view_radius: radius,
                    is_flat,
                });
            }
            PlayEvent::Abilities {
                flags, fly_speed, ..
            } => {
                was_flying = flags & 2 != 0;
                let _ = tx.send(NetEvt::Abilities { flags, fly_speed });
            }
            PlayEvent::UseComplete => {
                let _ = tx.send(NetEvt::UseComplete);
            }
            PlayEvent::OpLevel(l) => {
                let _ = tx.send(NetEvt::OpLevel(l));
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
            PlayEvent::DefaultSpawn(d) => {
                // VANILLA: handleSetSpawn → ClientLevel#setRespawnData.
                crate::itemdyn::set_spawn(&d.dimension, d.pos.x, d.pos.z);
            }
            PlayEvent::Other(_) => {
                let Some(pname) = pname else { continue };
                if pname == "set_health" {
                    // VANILLA: ClientPacketListener#handleSetHealth (float health, varint food, float saturation).
                    let mut rd = ezzmc_protocol::wire::Reader::new(&body);
                    if let (Ok(hp), Ok(food), Ok(sat)) = (rd.f32(), rd.var_int(), rd.f32()) {
                        let _ = tx.send(NetEvt::Health(hp, food, sat));
                    }
                    continue;
                }
                if pname == "set_experience" {
                    // VANILLA: handleSetExperience (float progress, varint level, varint total).
                    let mut rd = ezzmc_protocol::wire::Reader::new(&body);
                    if let (Ok(pr), Ok(lv)) = (rd.f32(), rd.var_int()) {
                        let _ = tx.send(NetEvt::Experience(pr, lv));
                    }
                    continue;
                }
                if pname == "cooldown" {
                    // VANILLA: handleItemCooldown (Identifier group, varint duration).
                    let mut rd = ezzmc_protocol::wire::Reader::new(&body);
                    if let (Ok(g), Ok(d)) = (rd.string(256), rd.var_int()) {
                        crate::cooldown::add(&g, d);
                    }
                    continue;
                }
                if pname == "waypoint" {
                    if let Some((op, wp)) = parse_waypoint(&body) {
                        let _ = tx.send(NetEvt::Waypoint(op, wp));
                    }
                    continue;
                }
                if pname == "update_attributes" {
                    parse_attributes(&body, h.player_id, &tx);
                    continue;
                }
                if matches!(pname, "system_chat" | "player_chat" | "disguised_chat") {
                    use crate::chat::Tag;
                    let r = match pname {
                        "system_chat" => crate::chatnet::parse_system(&body).map(|(c, o)| {
                            if o {
                                NetEvt::Overlay(c)
                            } else {
                                NetEvt::Chat(c, Tag::System)
                            }
                        }),
                        "player_chat" => crate::chatnet::parse_player(&body, &chat_types)
                            .map(|c| NetEvt::Chat(c, Tag::NotSecure)),
                        _ => crate::chatnet::parse_disguised(&body, &chat_types)
                            .map(|c| NetEvt::Chat(c, Tag::System)),
                    };
                    match r {
                        Ok(ev) => {
                            let _ = tx.send(ev);
                        }
                        Err(x) => tracing::warn!("{pname}: {x}"),
                    }
                    continue;
                }
                if pname == "command_suggestions" {
                    match crate::chatnet::parse_suggestions(&body) {
                        Ok((id, st, len, v)) => {
                            let _ = tx.send(NetEvt::Suggestions(id, st, len, v));
                        }
                        Err(x) => tracing::warn!("{pname}: {x}"),
                    }
                    continue;
                }
                if pname == "take_item_entity" {
                    // VANILLA: handleTakeItemEntity — звук до удаления сущности из мира.
                    if let Ok(id) = ezzmc_protocol::wire::Reader::new(&body).var_int() {
                        let hit = shared.lock().ok().and_then(|g| {
                            g.as_ref().and_then(|l| {
                                l.entities.map.get(&id).map(|e| {
                                    let xp = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                        .get(e.ty)
                                        .is_some_and(|n| *n == "experience_orb");
                                    (e.pos, xp)
                                })
                            })
                        });
                        if let Some((pos, xp)) = hit {
                            crate::levelsound::pickup(pos, xp);
                        }
                    }
                }
                if pname == "explode" {
                    if let Some(ex) = handle_explode(&body) {
                        let _ = tx.send(NetEvt::Explosion(Box::new(ex)));
                    }
                    continue;
                }
                if matches!(pname, "sound" | "sound_entity" | "stop_sound") {
                    handle_sound_packet(pname, &body, shared);
                    continue;
                }
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
                    LevelEvent::BatchStart => batch_size.on_batch_start(),
                    LevelEvent::BatchFinished { size } => {
                        batch_size.on_batch_finished(*size);
                        tracing::info!(
                            "chunk batch: size={size} desired/tick={:.1}",
                            batch_size.desired_chunks_per_tick()
                        );
                        h.send_chunk_batch_received(&mut c, batch_size.desired_chunks_per_tick())
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
        }
    }
}

/// VANILLA: Holder<SoundEvent> (ByteBufCodecs#holder): varint id+1, 0 → inline Identifier + Optional<Float>.
fn read_sound_holder(r: &mut ezzmc_protocol::wire::Reader) -> Option<String> {
    let id = r.var_int().ok()?;
    if id == 0 {
        let name = r.string(32767).ok()?;
        if r.bool().ok()? {
            r.f32().ok()?;
        }
        return Some(name);
    }
    ezzmc_protocol::generated::r26_3::SOUND_EVENT
        .get(usize::try_from(id - 1).ok()?)
        .map(|s| (*s).to_owned())
}

/// VANILLA: ClientPacketListener#handleExplosion — звук (BLOCKS, громкость 4, высота
/// (1+(r−r)*0.2)*0.7, если playSound — последний байт) и событие для частиц/отдачи.
fn handle_explode(body: &[u8]) -> Option<ExplosionEvt> {
    let mut r = ezzmc_protocol::wire::Reader::new(body);
    let c = [r.f64().ok()?, r.f64().ok()?, r.f64().ok()?];
    let radius = r.f32().ok()?;
    let block_count = r.i32().ok()?;
    let knockback = if r.bool().ok()? {
        Some([r.f64().ok()?, r.f64().ok()?, r.f64().ok()?])
    } else {
        None
    };
    let names = ezzmc_protocol::generated::r26_3::PARTICLE_TYPE;
    let pid = usize::try_from(r.var_int().ok()?).ok()?;
    // частицы с данными (block, dust…) не разбираем — у взрывов их нет
    let particle: &'static str = match *names.get(pid)? {
        "explosion" => "explosion",
        "explosion_emitter" => "explosion_emitter",
        _ => return None,
    };
    let ev = read_sound_holder(&mut r)?;
    let mut block_particles = Vec::new();
    let n = r.var_int().ok()?;
    for _ in 0..n.clamp(0, 64) {
        let pid = usize::try_from(r.var_int().ok()?).ok()?;
        let name = *names.get(pid)?;
        if !matches!(name, "poof" | "smoke" | "large_smoke") {
            break; // частица с данными — дальше разбор невозможен
        }
        let (scaling, speed, weight) = (r.f32().ok()?, r.f32().ok()?, r.var_int().ok()?);
        block_particles.push((name.to_owned(), scaling, speed, weight));
    }
    if body.last().copied().unwrap_or(0) != 0 {
        let rf = crate::levelsound::rf;
        crate::audio::play_at(
            &ev,
            crate::audio::Cat::Blocks,
            c[0],
            c[1],
            c[2],
            4.0,
            (1.0 + (rf() - rf()) * 0.2) * 0.7,
        );
    }
    Some(ExplosionEvt {
        center: c,
        radius,
        block_count,
        knockback,
        particle,
        block_particles,
    })
}

/// VANILLA: ClientPacketListener#handleSoundEvent / handleSoundEntityEvent / handleStopSoundEvent.
fn handle_sound_packet(pname: &str, body: &[u8], shared: &SharedLevel) {
    use crate::audio::{self, Cat};
    let mut r = ezzmc_protocol::wire::Reader::new(body);
    match pname {
        "sound" => {
            let Some(ev) = read_sound_holder(&mut r) else {
                return;
            };
            let (Ok(src), Ok(x), Ok(y), Ok(z), Ok(vol), Ok(pitch)) =
                (r.var_int(), r.i32(), r.i32(), r.i32(), r.f32(), r.f32())
            else {
                return;
            };
            audio::play_at(
                &ev,
                Cat::from_ordinal(src),
                f64::from(x) / 8.0,
                f64::from(y) / 8.0,
                f64::from(z) / 8.0,
                vol,
                pitch,
            );
        }
        "sound_entity" => {
            let Some(ev) = read_sound_holder(&mut r) else {
                return;
            };
            let (Ok(src), Ok(id), Ok(vol), Ok(pitch)) =
                (r.var_int(), r.var_int(), r.f32(), r.f32())
            else {
                return;
            };
            let pos = shared.lock().ok().and_then(|g| {
                g.as_ref()
                    .and_then(|l| l.entities.map.get(&id).map(|e| e.pos))
            });
            if let Some(p) = pos {
                audio::play_at(&ev, Cat::from_ordinal(src), p[0], p[1], p[2], vol, pitch);
            }
        }
        _ => {
            // stop_sound: byte flags (1 = source, 2 = identifier)
            let Ok(flags) = r.u8() else { return };
            let cat = if flags & 1 != 0 {
                r.var_int().ok().map(Cat::from_ordinal)
            } else {
                None
            };
            let ev = if flags & 2 != 0 {
                r.string(32767).ok()
            } else {
                None
            };
            audio::stop(ev.as_deref(), cat);
        }
    }
}

fn unix_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

/// VANILLA: ClientboundUpdateAttributesPacket → AttributeInstance#calculateValue (armor, max_health, max_absorption).
fn parse_attributes(body: &[u8], own_id: i32, tx: &crossbeam_channel::Sender<NetEvt>) {
    let mut r = ezzmc_protocol::wire::Reader::new(body);
    let (Ok(id), Ok(n)) = (r.var_int(), r.var_int()) else {
        return;
    };
    if id != own_id {
        return;
    }
    for _ in 0..n {
        let (Ok(aid), Ok(base), Ok(mc)) = (r.var_int(), r.f64(), r.var_int()) else {
            return;
        };
        let (mut add, mut mul_base, mut mul_total) = (0.0f64, 0.0f64, 1.0f64);
        for _ in 0..mc {
            let (Ok(_), Ok(amount), Ok(op)) = (r.string(256), r.f64(), r.var_int()) else {
                return;
            };
            match op {
                0 => add += amount,
                1 => mul_base += amount,
                _ => mul_total *= 1.0 + amount,
            }
        }
        let v = base + add;
        let value = (v + v * mul_base) * mul_total;
        let name = ezzmc_protocol::generated::r26_3::ATTRIBUTE
            .get(aid as usize)
            .copied();
        let key = match name {
            Some("armor") => "armor",
            Some("max_health") => "max_health",
            Some("max_absorption") => "max_absorption",
            _ => continue,
        };
        let _ = tx.send(NetEvt::Attribute(key, value as f32));
    }
}

/// VANILLA: ClientboundTrackedWaypointPacket#STREAM_CODEC + TrackedWaypoint#STREAM_CODEC.
fn parse_waypoint(body: &[u8]) -> Option<(u32, crate::hudbars::Wp)> {
    use crate::hudbars::{Wp, WpKind};
    let mut rd = ezzmc_protocol::wire::Reader::new(body);
    let op = rd.var_int().ok()? as u32;
    let (key, hash) = if rd.bool().ok()? {
        let u = rd.uuid().ok()?;
        let x = ((u >> 64) as u64) ^ (u as u64);
        (format!("{u:032x}"), ((x >> 32) as i32) ^ (x as i32))
    } else {
        let st = rd.string(32767).ok()?;
        // Java String#hashCode.
        let h = st
            .encode_utf16()
            .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(i32::from(c)));
        (st, h)
    };
    let _style = rd.string(32767).ok()?;
    let color = if rd.bool().ok()? {
        let b = rd.bytes(3).ok()?;
        Some((u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]))
    } else {
        None
    };
    let kind = match rd.var_int().ok()? {
        0 => WpKind::Empty,
        1 => WpKind::Pos(rd.var_int().ok()?, rd.var_int().ok()?, rd.var_int().ok()?),
        2 => WpKind::Chunk(rd.var_int().ok()?, rd.var_int().ok()?),
        3 => WpKind::Azimuth(rd.f32().ok()?),
        _ => return None,
    };
    Some((
        op,
        Wp {
            key,
            hash,
            color,
            kind,
        },
    ))
}
