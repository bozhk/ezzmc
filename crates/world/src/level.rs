//! Клиентский уровень (TODO-118/119/129): хранилище чанков + обработка пакетов мира.
//! VANILLA: ClientPacketListener#handleLevelChunkWithLight/handleForgetLevelChunk/
//! handleChunkBatch*/handleSetChunkCacheCenter/Radius/handleLightUpdatePacket/
//! handleBlockUpdate/handleChunkBlocksUpdate/handleBlockEntityData/handleBlockEvent/
//! handleBlockDestruction/handleSetTime (1.21.11).
//! Вход — имя пакета (из таблицы `generated/v*.rs`) и тело; net от world не зависит.
use crate::block::BlockState;
use crate::chunk::ChunkFormat;
use crate::chunk_packet::{BlockEntityInfo, LevelChunkPacket};
use crate::light::LightUpdate;
use crate::storage::ChunkCache;
use ezzmc_protocol::nbt;
use ezzmc_protocol::types::BlockPos;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::{ProtocolError, ProtocolResult};

#[derive(Clone, Debug, PartialEq)]
pub enum LevelEvent {
    /// Пакет не относится к миру (или не обработан).
    None,
    ChunkLoaded {
        x: i32,
        z: i32,
    },
    ChunkDropped {
        x: i32,
        z: i32,
    },
    LightUpdated {
        x: i32,
        z: i32,
    },
    /// Изменённые блоки (мировые координаты).
    BlocksChanged(Vec<BlockPos>),
    BlockEntityChanged(BlockPos),
    BatchStart,
    /// Размер пачки из пакета; клиент отвечает chunk_batch_received.
    BatchFinished {
        size: i32,
    },
    BlockEvent {
        pos: BlockPos,
        b0: u8,
        b1: u8,
        block: u32,
    },
    BlockDestruction {
        id: i32,
        pos: BlockPos,
        progress: u8,
    },
    TimeUpdated,
    WeatherChanged,
    /// VANILLA: ClientboundLevelEventPacket (звук/частицы события: 2001 = разрушение блока).
    Effect {
        ty: i32,
        pos: BlockPos,
        data: i32,
    },
    /// VANILLA: ClientboundLevelParticlesPacket. `block` — состояние для block-подобных частиц.
    Particles(ParticlesPacket),
}

/// VANILLA: ClientboundLevelParticlesPacket (только то, что нужно клиентским частицам).
#[derive(Clone, Debug, PartialEq)]
pub struct ParticlesPacket {
    /// Имя из реестра `particle_type` (без `minecraft:`).
    pub name: &'static str,
    pub pos: [f64; 3],
    pub dist: [f32; 3],
    pub max_speed: f32,
    pub count: i32,
    /// Состояние блока для `block`/`block_marker`/`falling_dust`/`dust_pillar`/`block_crumble`.
    pub block: Option<u32>,
}

/// VANILLA: ClientClockManager.ClientClockInstance.
#[derive(Clone, Copy, Debug)]
pub struct ClientClock {
    pub total_ticks: i64,
    pub partial_tick: f32,
    pub rate: f32,
}

use crate::entities::{unpack_degrees, Entity};

/// VANILLA: LpVec3#read.
fn read_lp_vec3(r: &mut Reader) -> ProtocolResult<[f64; 3]> {
    let lowest = r.u8()?;
    if lowest == 0 {
        return Ok([0.0; 3]);
    }
    let middle = u64::from(r.u8()?);
    let high = u64::from(r.i32()? as u32);
    let buf = high << 16 | middle << 8 | u64::from(lowest);
    let mut scale = u64::from(lowest & 3);
    if lowest & 4 != 0 {
        scale |= u64::from(r.var_int()? as u32) << 2;
    }
    let scale = scale as f64;
    let un = |shift: u32| (((buf >> shift) & 32767).min(32766) as f64) * 2.0 / 32766.0 - 1.0;
    Ok([un(3) * scale, un(18) * scale, un(33) * scale])
}

/// Id `lightning_bolt` в реестре `entity_type` (26.3).
static LIGHTNING_BOLT_TYPE: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "lightning_bolt")
});

/// Id `tnt` в реестре `entity_type` (26.3).
static PEARL_TYPE: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "ender_pearl")
});
static TNT_TYPE: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "tnt")
});

/// Id `item` в реестре `entity_type` (26.3).
static ITEM_TYPE: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "item")
});

/// VANILLA: ItemPickupParticle (EntityRenderState предмета заморожен в момент пакета).
pub struct Pickup {
    pub id: i32,
    pub target: i32,
    pub pos: [f64; 3],
    pub age: f32,
    pub stack: ezzmc_protocol::item::ItemStack,
    pub life: u32,
}

pub struct Level {
    pub chunks: ChunkCache,
    /// VANILLA: Level#getGameTime / getDayTime / tickDayTime.
    pub game_time: i64,
    pub day_time: i64,
    /// day_time предыдущего тика — для интерполяции неба по partial_tick.
    pub prev_day_time: i64,
    last_tick_at: std::time::Instant,
    pub tick_day_time: bool,
    /// VANILLA: ClientClockManager — часы по id в реестре `world_clock`.
    pub clocks: std::collections::HashMap<i32, ClientClock>,
    /// Id часов overworld в реестре `world_clock` (None → берём первые пришедшие).
    pub overworld_clock: Option<i32>,
    last_tick_game_time: i64,
    /// VANILLA: BlockStatePredictionHandler (позиция → (seq, известное состояние сервера)).
    predictions: std::collections::HashMap<(i32, i32, i32), (i32, BlockState)>,
    pred_seq: i32,
    pending_seq: i32,
    /// VANILLA: LevelData#isRaining.
    pub raining: bool,
    /// VANILLA: Level#rainLevel / thunderLevel (0..1).
    pub rain_level: f32,
    pub thunder_level: f32,
    /// Молнии (`lightning_bolt`) — единственные сущности, которые клиент пока заводит.
    pub bolts: crate::lightning::Bolts,
    /// Позиции ударов молнии, ждущие звука (клиент забирает `take`).
    pub thunder: Vec<[f64; 3]>,
    pub entities: crate::entities::Entities,
    /// VANILLA: ItemPickupParticle — анимация подбора (3 тика).
    pub pickups: Vec<Pickup>,
    /// Места попадания жемчуга Энда (частицы portal); забирает клиент.
    pub pearl_hits: Vec<[f64; 3]>,
    /// VANILLA: LivingEntity#makePoofParticles (entity_event 60): (позиция, ширина, высота).
    pub death_poofs: Vec<([f64; 3], f64, f64)>,
    /// VANILLA: ClientLevel#skyFlashTime (молния ставит 2, каждый тик −1).
    pub sky_flash_time: i32,
    /// VANILLA: DimensionType#skybox (Overworld/End/None) — что рисовать вместо неба.
    pub skybox: crate::dimension::Skybox,
    /// Полный DimensionType текущего измерения (атрибуты среды: туман, лайтмап Нижнего мира).
    pub dimension: Option<crate::dimension::DimensionType>,
    /// Звуковые атрибуты dimension_type (музыка, mood).
    pub audio: crate::audio_attr::AudioAttrs,
    /// VANILLA: PistonMovingBlockEntity по позиции блока `moving_piston`.
    pub pistons: std::collections::HashMap<(i32, i32, i32), crate::piston::MovingPiston>,
}

impl Level {
    /// Запоминает VarInt-метаданную моба (индекс ≥16) для вариантов/окрасов.
    fn set_int_meta(&mut self, id: i32, idx: u8, v: i32) {
        if idx < 16 {
            return;
        }
        if let Some(e) = self.entities.map.get_mut(&id) {
            match e.ints.iter_mut().find(|(i, _)| *i == idx) {
                Some(slot) => slot.1 = v,
                None => e.ints.push((idx, v)),
            }
        }
    }

    pub fn new(format: ChunkFormat, view_radius: i32) -> Self {
        Level {
            chunks: ChunkCache::new(format, view_radius),
            game_time: 0,
            day_time: 0,
            prev_day_time: 0,
            last_tick_at: std::time::Instant::now(),
            tick_day_time: true,
            clocks: std::collections::HashMap::new(),
            overworld_clock: None,
            last_tick_game_time: 0,
            predictions: std::collections::HashMap::new(),
            pred_seq: 0,
            pending_seq: 0,
            raining: false,
            rain_level: 0.0,
            thunder_level: 0.0,
            bolts: Default::default(),
            thunder: Vec::new(),
            entities: Default::default(),
            pickups: Vec::new(),
            pearl_hits: Vec::new(),
            death_poofs: Vec::new(),
            sky_flash_time: 0,
            skybox: crate::dimension::Skybox::Overworld,
            dimension: None,
            audio: Default::default(),
            pistons: Default::default(),
        }
    }

    /// VANILLA: Level#getThunderLevel(partial) = lerp(oThunder, thunder) * getRainLevel.
    /// Клиентские сеттеры (setRainLevel/setThunderLevel) пишут и o*, и текущее значение,
    /// а клиент сам уровни не двигает (это делает ServerLevel) → lerp вырождается в значение.
    pub fn thunder(&self) -> f32 {
        self.thunder_level * self.rain_level
    }

    /// VANILLA: partialTick — доля текущего 50-мс тика, прошедшая с последнего `tick_time`.
    pub fn partial_tick(&self) -> f32 {
        (self.last_tick_at.elapsed().as_secs_f32() * 20.0).clamp(0.0, 1.0)
    }

    /// VANILLA: ClientLevel#tickTime — вызывать каждый клиентский тик (20/с).
    pub fn tick_time(&mut self) {
        self.tick_time_at(std::time::Instant::now());
    }

    /// Тик, запланированный на момент `at` (для partial_tick).
    pub fn tick_time_at(&mut self, at: std::time::Instant) {
        self.last_tick_at = at;
        self.game_time += 1;
        self.tick_clocks(self.game_time);
        self.sync_day_time();
        // VANILLA: ClientLevel#tick уменьшает skyFlashTime, затем тикают сущности (LightningBolt).
        if self.sky_flash_time > 0 {
            self.sky_flash_time -= 1;
        }
        self.entities.tick();
        {
            let chunks = &self.chunks;
            let solid = |x: f64, y: f64, z: f64| {
                let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32);
                let st = chunks.block(bx, by, bz);
                let boxes = st
                    .props()
                    .and_then(|p| {
                        crate::generated::p26_3::SHAPES
                            .get(usize::from(p.collision_shape))
                            .copied()
                    })
                    .unwrap_or(&[]);
                let (fx, fy, fz) = (x - f64::from(bx), y - f64::from(by), z - f64::from(bz));
                boxes.iter().any(|b| {
                    fx >= b[0] && fx <= b[3] && fy >= b[1] && fy <= b[4] && fz >= b[2] && fz <= b[5]
                })
            };
            let player_ty = crate::entities::player_type();
            type Other = (i32, ([f64; 3], [f64; 3]));
            let others: Vec<Other> = self
                .entities
                .map
                .values()
                .filter(|e| Some(e.ty) == player_ty)
                .filter_map(|e| Some((e.id, e.aabb()?)))
                .collect();
            let mut dead = Vec::new();
            for e in self.entities.map.values_mut() {
                e.tick_projectile(&solid, &others);
                if e.dead {
                    dead.push(e.id);
                }
            }
            for id in dead {
                self.entities.remove(id);
            }
        }
        for p in &mut self.pickups {
            p.life += 1;
        }
        self.pickups.retain(|p| p.life < 3);
        let bt = self.bolts.tick();
        if bt.flash {
            self.sky_flash_time = 2;
        }
        if !bt.strikes.is_empty() {
            self.thunder.extend(bt.strikes);
        }
    }

    // VANILLA: ClientClockManager#tick
    fn tick_clocks(&mut self, game_time: i64) {
        let delta = game_time - self.last_tick_game_time;
        self.last_tick_game_time = game_time;
        for c in self.clocks.values_mut() {
            let p = f64::from(c.partial_tick) + delta as f64 * f64::from(c.rate);
            let full = p.floor();
            c.partial_tick = (p - full) as f32;
            c.total_ticks += full as i64;
        }
    }

    fn sync_day_time(&mut self) {
        let c = match self.overworld_clock {
            Some(id) => self.clocks.get(&id),
            None => self.clocks.values().next(),
        };
        if let Some(c) = c {
            self.prev_day_time = self.day_time;
            self.day_time = c.total_ticks;
            self.tick_day_time = c.rate != 0.0;
        }
    }

    /// Обработать пакет мира. Неизвестные имена → `LevelEvent::None`.
    pub fn handle(&mut self, name: &str, body: &[u8]) -> ProtocolResult<LevelEvent> {
        let mut r = Reader::new(body);
        Ok(match name {
            "level_chunk_with_light" => {
                let p = LevelChunkPacket::read(self.chunks.format(), &mut r)?;
                let (x, z) = (p.x, p.z);
                if self.chunks.apply_packet(p) {
                    LevelEvent::ChunkLoaded { x, z }
                } else {
                    // VANILLA: «Ignoring chunk since it's not in the view range».
                    tracing::warn!(x, z, "чанк вне диапазона просмотра — пропущен");
                    LevelEvent::None
                }
            }
            "forget_level_chunk" => {
                let pos = ezzmc_protocol::types::ChunkPos::from_long(r.i64()?);
                if self.chunks.drop_chunk(pos.x, pos.z) {
                    LevelEvent::ChunkDropped { x: pos.x, z: pos.z }
                } else {
                    LevelEvent::None
                }
            }
            "set_chunk_cache_center" => {
                let (x, z) = (r.var_int()?, r.var_int()?);
                self.chunks.set_center(x, z);
                LevelEvent::None
            }
            "set_chunk_cache_radius" => {
                self.chunks.set_view_radius(r.var_int()?);
                LevelEvent::None
            }
            "chunk_batch_start" => LevelEvent::BatchStart,
            "chunk_batch_finished" => LevelEvent::BatchFinished { size: r.var_int()? },
            "light_update" => {
                let (x, z) = (r.var_int()?, r.var_int()?);
                let u = LightUpdate::read(&mut r)?;
                if self.chunks.apply_light(x, z, &u) {
                    LevelEvent::LightUpdated { x, z }
                } else {
                    LevelEvent::None
                }
            }
            "block_changed_ack" => {
                let seq = r.var_int()?;
                LevelEvent::BlocksChanged(self.ack_predictions(seq))
            }
            "block_update" => {
                let pos = BlockPos::read(&mut r)?;
                let state = read_state(&mut r)?;
                tracing::debug!(
                    target: "dbg",
                    "block_update pos=({},{},{}) state={}",
                    pos.x,
                    pos.y,
                    pos.z,
                    state.name()
                );
                self.set_block_verified(pos, state);
                LevelEvent::BlocksChanged(vec![pos])
            }
            "section_blocks_update" => {
                // SectionPos: long (x:22 | z:22 | y:20), знаковые.
                let v = r.i64()?;
                let (sx, sy, sz) = (v >> 42, (v << 44) >> 44, (v << 22) >> 42);
                let n = r.length("section updates", 4096)?;
                let mut changed = Vec::with_capacity(n);
                for _ in 0..n {
                    let e = r.var_long()?;
                    let local = (e & 4095) as i32;
                    let state = state_from(e >> 12)?;
                    let pos = BlockPos::new(
                        (sx as i32) * 16 + (local >> 8 & 15),
                        (sy as i32) * 16 + (local & 15),
                        (sz as i32) * 16 + (local >> 4 & 15),
                    );
                    self.set_block_verified(pos, state);
                    changed.push(pos);
                }
                LevelEvent::BlocksChanged(changed)
            }
            "block_entity_data" => {
                let pos = BlockPos::read(&mut r)?;
                let type_id =
                    u32::try_from(r.var_int()?).map_err(|_| ProtocolError::InvalidValue {
                        what: "block entity type id",
                        value: -1,
                    })?;
                let tag = nbt::read_network(&mut r)?;
                if self.set_block_entity(pos, type_id, tag) {
                    LevelEvent::BlockEntityChanged(pos)
                } else {
                    LevelEvent::None
                }
            }
            "block_event" => LevelEvent::BlockEvent {
                pos: BlockPos::read(&mut r)?,
                b0: r.u8()?,
                b1: r.u8()?,
                block: u32::try_from(r.var_int()?).unwrap_or(0),
            },
            "level_event" => LevelEvent::Effect {
                ty: r.i32()?,
                pos: BlockPos::read(&mut r)?,
                data: r.i32()?,
            },
            "level_particles" => {
                let _override_limiter = r.bool()?;
                let _always_show = r.bool()?;
                let pos = [r.f64()?, r.f64()?, r.f64()?];
                let dist = [r.f32()?, r.f32()?, r.f32()?];
                let max_speed = r.f32()?;
                let count = r.i32()?;
                let id = usize::try_from(r.var_int()?).unwrap_or(usize::MAX);
                match ezzmc_protocol::generated::r26_3::PARTICLE_TYPE.get(id) {
                    Some(&name) => {
                        let block = if matches!(
                            name,
                            "block"
                                | "block_marker"
                                | "falling_dust"
                                | "dust_pillar"
                                | "block_crumble"
                        ) {
                            u32::try_from(r.var_int()?).ok()
                        } else {
                            None
                        };
                        LevelEvent::Particles(ParticlesPacket {
                            name,
                            pos,
                            dist,
                            max_speed,
                            count,
                            block,
                        })
                    }
                    None => LevelEvent::None,
                }
            }
            "block_destruction" => LevelEvent::BlockDestruction {
                id: r.var_int()?,
                pos: BlockPos::read(&mut r)?,
                progress: r.u8()?,
            },
            "set_time" => {
                // VANILLA: ClientboundSetTimePacket (26.x): gameTime + Map<Holder<WorldClock>, ClockNetworkState>.
                let game_time = r.i64()?;
                let n = r.var_int()?;
                let mut updates = Vec::new();
                for _ in 0..n.clamp(0, 64) {
                    let id = r.var_int()?;
                    let total = r.var_long()?;
                    let partial = r.f32()?;
                    let rate = r.f32()?;
                    updates.push((id, total, partial, rate));
                }
                self.game_time = game_time;
                // VANILLA: ClientClockManager#handleUpdates: tick(gameTime), затем перезапись.
                self.tick_clocks(game_time);
                for (id, total_ticks, partial_tick, rate) in updates {
                    self.clocks.insert(
                        id,
                        ClientClock {
                            total_ticks,
                            partial_tick,
                            rate,
                        },
                    );
                }
                self.sync_day_time();
                LevelEvent::TimeUpdated
            }
            "add_entity" => {
                // VANILLA: ClientboundAddEntityPacket: id, uuid, type, x, y, z, LpVec3 movement,
                // xRot, yRot, yHeadRot (байты), data.
                let id = r.var_int()?;
                let uuid = r.uuid()?;
                let ty = r.var_int()?;
                let pos = [r.f64()?, r.f64()?, r.f64()?];
                if usize::try_from(ty).ok() == *LIGHTNING_BOLT_TYPE {
                    self.bolts.spawn(id, pos);
                }
                if let Ok(ty) = usize::try_from(ty) {
                    let rots = read_lp_vec3(&mut r)
                        .and_then(|v| Ok((v, r.i8()?, r.i8()?, r.i8()?, r.var_int()?)));
                    if let Ok((vel, x_rot, y_rot, head, data)) = rots {
                        let (yaw, pitch, head) = (
                            unpack_degrees(y_rot),
                            unpack_degrees(x_rot),
                            unpack_degrees(head),
                        );
                        let mut e = Entity::new(id, uuid, ty, pos, yaw, pitch, head);
                        e.vel = vel;
                        // VANILLA: AbstractArrow#recreateFromPacket — data = id владельца.
                        e.owner = data;
                        e.snap = ITEM_TYPE.is_some_and(|t| t == ty);
                        // VANILLA: PrimedTnt#DATA_FUSE_ID по умолчанию 80.
                        if TNT_TYPE.is_some_and(|t| t == ty) {
                            e.fuse = 80;
                        }
                        self.entities.add(e);
                    }
                }
                LevelEvent::None
            }
            "entity_event" => {
                // VANILLA: LivingEntity#handleEntityEvent: 2/33/36/37/44/57 = hurt (hurtTime=10), 3 = death.
                let (id, ev) = (r.i32()?, r.u8()?);
                if let Some(e) = self.entities.map.get_mut(&id) {
                    match ev {
                        2 | 33 | 36 | 37 | 44 | 57 => {
                            e.hurt_at = Some(std::time::Instant::now());
                            e.hurt_dur = 0.5;
                        }
                        3 => {
                            e.hurt_at = Some(std::time::Instant::now());
                            e.hurt_dur = 1.0;
                            e.death_at.get_or_insert_with(std::time::Instant::now);
                        }
                        60 => {
                            // VANILLA: LivingEntity#handleEntityEvent 60 → makePoofParticles.
                            let (w, h) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                                .get(e.ty)
                                .and_then(|n| crate::entity_size::size(n))
                                .unwrap_or((0.6, 1.8));
                            self.death_poofs.push((e.pos, f64::from(w), f64::from(h)));
                        }
                        _ => {}
                    }
                }
                LevelEvent::None
            }
            "damage_event" | "hurt_animation" => {
                // VANILLA: ClientPacketListener#handleDamageEvent / handleHurtAnimation → hurtTime=10.
                let id = r.var_int()?;
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.hurt_at = Some(std::time::Instant::now());
                    e.hurt_dur = 0.5;
                }
                LevelEvent::None
            }
            "animate" => {
                // VANILLA: handleAnimate: 0 swing main, 3 swing off, 4 crit, 5 magic crit.
                let (id, action) = (r.var_int()?, r.u8()?);
                if let Some(e) = self.entities.map.get_mut(&id) {
                    match action {
                        0 | 3 => e.swing_seq = e.swing_seq.wrapping_add(1),
                        4 | 5 => e.crit_seq = (e.crit_seq.0.wrapping_add(1), action - 3),
                        _ => {}
                    }
                }
                LevelEvent::None
            }
            "set_equipment" => {
                // VANILLA: handleSetEquipment: цикл (slot | 0x80 = ещё) + стак; 0 mainhand, 1 offhand.
                let id = r.var_int()?;
                loop {
                    let b = r.u8()?;
                    let st = ezzmc_protocol::item::read_stack(&mut r)?;
                    if let Some(e) = self.entities.map.get_mut(&id) {
                        match b & 0x7F {
                            0 => e.held = st,
                            1 => e.held_off = st,
                            // EquipmentSlot: FEET 2, LEGS 3, CHEST 4, HEAD 5.
                            n @ 2..=5 => e.armor[(5 - n) as usize] = st,
                            _ => {}
                        }
                    }
                    if b & 0x80 == 0 {
                        break;
                    }
                }
                LevelEvent::None
            }
            "take_item_entity" => {
                // VANILLA: handleTakeItemEntity: ItemPickupParticle + shrink(amount); пустой стак → removeEntity.
                let (item, target, amount) = (r.var_int()?, r.var_int()?, r.var_int()?);
                if let Some(e) = self.entities.map.get_mut(&item) {
                    if let Some(stack) = e.item.clone() {
                        self.pickups.push(Pickup {
                            id: item,
                            target,
                            pos: e.pos,
                            age: e.age as f32 + 1.0,
                            stack,
                            life: 0,
                        });
                    }
                    if let Some(st) = e.item.as_mut() {
                        st.count -= amount;
                        if st.count <= 0 {
                            self.entities.remove(item);
                        }
                    }
                }
                LevelEvent::None
            }
            "remove_entities" => {
                // VANILLA: ClientboundRemoveEntitiesPacket: VarInt-список id.
                let n = r.var_int()?;
                for _ in 0..n.clamp(0, 4096) {
                    let id = r.var_int()?;
                    self.bolts.remove(id);
                    if let Some(e) = self.entities.map.get(&id) {
                        if PEARL_TYPE.is_some_and(|t| t == e.ty) {
                            self.pearl_hits.push(e.pos);
                        }
                    }
                    self.entities.remove(id);
                }
                LevelEvent::None
            }
            "set_entity_data" => {
                // VANILLA: ClientboundSetEntityDataPacket: id, затем записи (индекс u8, тип VarInt,
                // значение), конец — 0xFF. Читаем #0 (флаги) и #8 (ItemEntity#DATA_ITEM, ITEM_STACK = 7);
                // значения базовых типов пропускаем, на неизвестном типе разбор останавливается.
                // VANILLA-UNSURE: прочие метаданные (поза, имя и т.д.) пока не читаются.
                let id = r.var_int()?;
                loop {
                    let idx = r.u8()?;
                    if idx == 0xFF {
                        break;
                    }
                    let ty = r.var_int()?;
                    match ty {
                        0 => {
                            let v = r.u8()?;
                            if idx == 0 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    e.flags = v;
                                }
                            } else if idx == 16 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    e.byte16 = v;
                                }
                            } else if idx == 15 || idx == 18 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    if idx == 15 {
                                        e.mob_flags = v;
                                    } else {
                                        e.byte18 = v;
                                    }
                                }
                            } else if idx == 8 {
                                // AbstractArrow#ID_FLAGS (у предметов #8 — другой тип).
                                if let Some(e) =
                                    self.entities.map.get_mut(&id).filter(|e| e.proj == 1)
                                {
                                    e.arrow_flags = v;
                                }
                            }
                        }
                        1 | 20 => {
                            let v = r.var_int()?;
                            if ty == 1 {
                                self.set_int_meta(id, idx, v);
                            }
                            if ty == 1 && idx == 8 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    if e.fuse >= 0 {
                                        e.fuse = v;
                                    }
                                }
                            }
                            if ty == 1 && idx == 16 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    e.swell_dir = v;
                                }
                            }
                        }
                        // BLOCK_STATE/OPTIONAL_BLOCK_STATE, OPTIONAL_UNSIGNED_INT, варианты (реестровые
                        // холдеры), состояния — все VarInt (VANILLA: EntityDataSerializers, порядок регистрации).
                        14 | 15 | 19 | 21..=32 | 34..=38 => {
                            let v = r.var_int()?;
                            self.set_int_meta(id, idx, v);
                        }
                        // OPTIONAL_LIVING_ENTITY_REFERENCE: bool + UUID.
                        13 => {
                            if r.bool()? {
                                r.bytes(16)?;
                            }
                        }
                        // LONG, FLOAT, STRING, ROTATIONS, BLOCK_POS, DIRECTION — пропускаем,
                        // чтобы дойти до индексов мобов (#15+).
                        2 => {
                            r.var_long()?;
                        }
                        3 => {
                            r.f32()?;
                        }
                        4 => {
                            r.string(32767)?;
                        }
                        9 => {
                            r.bytes(12)?;
                        }
                        10 => {
                            r.bytes(8)?;
                        }
                        11 => {
                            if r.bool()? {
                                r.bytes(8)?;
                            }
                        }
                        12 => {
                            r.var_int()?;
                        }
                        8 => {
                            let v = r.u8()?;
                            if idx == 16 || idx == 17 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    if idx == 16 {
                                        e.baby = v != 0;
                                    } else {
                                        e.bool17 = v != 0;
                                    }
                                }
                            }
                            // AbstractArrow#IN_GROUND (#10).
                            if idx == 10 {
                                if let Some(e) =
                                    self.entities.map.get_mut(&id).filter(|e| e.proj == 1)
                                {
                                    e.stuck = v != 0;
                                    if v != 0 {
                                        e.vel = [0.0; 3];
                                    }
                                }
                            }
                        }
                        6 => {
                            if r.bool()? {
                                break;
                            }
                        }
                        7 => {
                            let st = ezzmc_protocol::item::read_stack(&mut r)?;
                            if idx == 8 {
                                if let Some(e) = self.entities.map.get_mut(&id) {
                                    e.item = st;
                                }
                            }
                        }
                        _ => break,
                    }
                }
                LevelEvent::None
            }
            "move_entity_pos" | "move_entity_pos_rot" | "move_entity_rot" => {
                // VANILLA: handleMoveEntity (ClientboundMoveEntityPacket.*, 26.3).
                let id = r.var_int()?;
                let (on_ground, mut path, mut rot);
                path = None;
                rot = None;
                if name != "move_entity_rot" {
                    let props = r.var_int()? as u32;
                    on_ground = props & 1 != 0;
                    let steps = (props >> 1) as usize;
                    let base = self.entities.map.get(&id).map(|e| e.target_pos());
                    let mut cur = base.unwrap_or([0.0; 3]);
                    let n = if steps == 0 {
                        1
                    } else {
                        steps.min(r.remaining() / 7)
                    };
                    for _ in 0..n {
                        if steps > 0 {
                            r.var_int()?; // ticks шага (тайминг шагов не портирован)
                        }
                        let d = [r.i16()?, r.i16()?, r.i16()?];
                        for i in 0..3 {
                            if d[i] != 0 {
                                cur[i] = ((cur[i] * 4096.0).round() + f64::from(d[i])) / 4096.0;
                            }
                        }
                    }
                    path = Some(cur);
                } else {
                    on_ground = r.bool()?;
                }
                if name != "move_entity_pos" {
                    let (y, x) = (r.i8()?, r.i8()?);
                    rot = Some((unpack_degrees(y), unpack_degrees(x)));
                }
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.on_ground = on_ground;
                    e.set_target(path, rot);
                }
                LevelEvent::None
            }
            "teleport_entity" => {
                // VANILLA: handleTeleportEntity: PositionMoveRotation + Set<Relative> + onGround.
                let id = r.var_int()?;
                let pos = [r.f64()?, r.f64()?, r.f64()?];
                let delta = [r.f64()?, r.f64()?, r.f64()?];
                let (y_rot, x_rot) = (r.f32()?, r.f32()?);
                let rel = r.i32()?;
                let on_ground = r.bool()?;
                if let Some(e) = self.entities.map.get_mut(&id) {
                    let cur = e.target_pos();
                    let p = [0, 1, 2].map(|i| {
                        if rel & (1 << i) != 0 {
                            cur[i] + pos[i]
                        } else {
                            pos[i]
                        }
                    });
                    let (cy, cx) = e.target_yaw_pitch();
                    let y = if rel & 8 != 0 { cy + y_rot } else { y_rot };
                    let x = if rel & 16 != 0 { cx + x_rot } else { x_rot };
                    e.on_ground = on_ground;
                    // VANILLA: PositionMoveRotation#calculate — delta тоже относительный (биты 5..7).
                    for (i, (vel, d)) in e.vel.iter_mut().zip(delta).enumerate() {
                        *vel = if rel & (1 << (5 + i)) != 0 {
                            *vel + d
                        } else {
                            d
                        };
                    }
                    e.set_target(Some(p), Some((y, x)));
                }
                LevelEvent::None
            }
            "set_entity_motion" => {
                // VANILLA: handleSetEntityMotion → Entity#lerpMotion.
                let id = r.var_int()?;
                let v = read_lp_vec3(&mut r)?;
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.vel = v;
                    if v != [0.0; 3] {
                        e.stuck = false;
                    }
                }
                LevelEvent::None
            }
            "entity_position_sync" => {
                // VANILLA: handleEntityPositionSync: PositionPath (LINEAR|STEPPED), yRot, xRot, onGround.
                let id = r.var_int()?;
                let end = match r.var_int()? {
                    0 => [r.f64()?, r.f64()?, r.f64()?],
                    _ => {
                        let n = r.var_int()?.clamp(0, 4096);
                        let mut last = [0.0; 3];
                        for _ in 0..n {
                            last = [r.f64()?, r.f64()?, r.f64()?];
                            r.var_int()?;
                        }
                        last
                    }
                };
                let (y, x) = (r.f32()?, r.f32()?);
                let on_ground = r.bool()?;
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.on_ground = on_ground;
                    e.set_target(Some(end), Some((y, x)));
                }
                LevelEvent::None
            }
            "rotate_head" => {
                let (id, head) = (r.var_int()?, r.i8()?);
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.set_head_target(unpack_degrees(head));
                }
                LevelEvent::None
            }
            "game_event" => {
                // VANILLA: handleGameEvent — только погодные события (остальные — в net/play).
                let (event, param) = (r.u8()?, r.f32()?);
                match event {
                    1 => {
                        self.raining = true;
                        self.rain_level = 0.0;
                    }
                    2 => {
                        self.raining = false;
                        self.rain_level = 1.0;
                    }
                    7 => self.rain_level = param.clamp(0.0, 1.0),
                    8 => self.thunder_level = param.clamp(0.0, 1.0),
                    _ => return Ok(LevelEvent::None),
                }
                LevelEvent::WeatherChanged
            }
            _ => LevelEvent::None,
        })
    }

    /// VANILLA: BlockStatePredictionHandler#startPredicting → sequence для пакета.
    pub fn start_prediction(&mut self) -> i32 {
        self.pred_seq += 1;
        self.pending_seq = self.pred_seq;
        self.pred_seq
    }

    /// VANILLA: ClientLevel#setServerVerifiedBlockState-обход: локальная смена блока с запоминанием серверного.
    pub fn predict_block(&mut self, pos: BlockPos, state: BlockState) {
        let old = self.chunks.block(pos.x, pos.y, pos.z);
        let seq = self.pending_seq;
        self.predictions
            .entry((pos.x, pos.y, pos.z))
            .and_modify(|e| e.0 = seq)
            .or_insert((seq, old));
        self.chunks.set_block(pos.x, pos.y, pos.z, state);
    }

    /// VANILLA: endPredictionsUpTo — откат/подтверждение; возвращает затронутые позиции.
    fn ack_predictions(&mut self, seq: i32) -> Vec<BlockPos> {
        let done: Vec<_> = self
            .predictions
            .iter()
            .filter(|(_, v)| v.0 <= seq)
            .map(|(k, v)| (*k, v.1))
            .collect();
        let mut out = Vec::new();
        for (k, st) in done {
            self.predictions.remove(&k);
            let pos = BlockPos::new(k.0, k.1, k.2);
            self.set_block_verified(pos, st);
            out.push(pos);
        }
        out
    }

    /// VANILLA: setServerVerifiedBlockState; блок-сущность удаляется, если блок сменил тип.
    fn set_block_verified(&mut self, pos: BlockPos, state: BlockState) {
        // VANILLA: updateKnownServerState — пока есть предсказание, запоминаем состояние, мир не трогаем.
        if let Some(e) = self.predictions.get_mut(&(pos.x, pos.y, pos.z)) {
            e.1 = state;
            return;
        }
        let old = self.chunks.block(pos.x, pos.y, pos.z);
        if self.chunks.set_block(pos.x, pos.y, pos.z, state) && old.block_id() != state.block_id() {
            self.chunks.remove_block_entity(pos);
        }
    }

    fn set_block_entity(&mut self, pos: BlockPos, type_id: u32, tag: Option<nbt::Tag>) -> bool {
        self.chunks.set_block_entity(
            pos,
            BlockEntityInfo {
                x: (pos.x & 15) as u8,
                z: (pos.z & 15) as u8,
                y: pos.y,
                type_id,
                tag,
            },
        )
    }
}

fn read_state(r: &mut Reader<'_>) -> ProtocolResult<BlockState> {
    state_from(i64::from(r.var_int()?))
}

/// Block.BLOCK_STATE_REGISTRY.byId: неизвестный id → воздух (default value реестра).
fn state_from(id: i64) -> ProtocolResult<BlockState> {
    u32::try_from(id)
        .map_err(|_| ProtocolError::InvalidValue {
            what: "block state id",
            value: id,
        })
        .map(|id| BlockState::from_id(id).unwrap_or(BlockState::AIR))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    const F: ChunkFormat = ChunkFormat {
        min_y: -64,
        height: 384,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    fn empty_chunk_packet(x: i32, z: i32) -> Vec<u8> {
        let mut sec = Writer::new();
        for _ in 0..24 {
            sec.i16(0).i16(0).u8(0).var_int(0).u8(0).var_int(0);
        }
        let sec = sec.into_vec();
        let mut w = Writer::new();
        w.i32(x).i32(z).var_int(0);
        w.var_int(sec.len() as i32).bytes(&sec).var_int(0);
        for _ in 0..4 {
            w.bitset_bytes(&[]);
        }
        w.var_int(0).var_int(0);
        w.into_vec()
    }

    #[test]
    fn chunk_lifecycle_and_block_updates() {
        let mut l = Level::new(F, 4);
        let mut w = Writer::new();
        w.var_int(2).var_int(-3);
        assert_eq!(
            l.handle("set_chunk_cache_center", &w.buf).unwrap(),
            LevelEvent::None
        );
        assert_eq!(
            l.handle("chunk_batch_start", &[]).unwrap(),
            LevelEvent::BatchStart
        );
        assert_eq!(
            l.handle("level_chunk_with_light", &empty_chunk_packet(2, -3))
                .unwrap(),
            LevelEvent::ChunkLoaded { x: 2, z: -3 }
        );
        assert_eq!(
            l.handle("level_chunk_with_light", &empty_chunk_packet(99, 99))
                .unwrap(),
            LevelEvent::None
        );
        let mut w = Writer::new();
        w.var_int(1);
        assert_eq!(
            l.handle("chunk_batch_finished", &w.buf).unwrap(),
            LevelEvent::BatchFinished { size: 1 }
        );
        // block_update (2*16+5, 70, -3*16+7) = камень
        let mut w = Writer::new();
        w.block_pos(37, 70, -41).var_int(1);
        l.handle("block_update", &w.buf).unwrap();
        assert_eq!(l.chunks.block(37, 70, -41).id(), 1);
        // section_blocks_update: секция (2, 4, -3), local x=5,z=7,y=3 → воздух→камень(id 1)
        let sec = ((2i64 & 0x3F_FFFF) << 42) | ((-3i64 & 0x3F_FFFF) << 20) | (4i64 & 0xF_FFFF);
        let mut w = Writer::new();
        w.i64(sec)
            .var_int(1)
            .var_long(1 << 12 | (5 << 8 | 7 << 4 | 3));
        let ev = l.handle("section_blocks_update", &w.buf).unwrap();
        assert_eq!(
            ev,
            LevelEvent::BlocksChanged(vec![BlockPos::new(37, 67, -41)])
        );
        assert_eq!(l.chunks.block(37, 67, -41).id(), 1);
        // предсказание: локально воздух, сервер прислал камень → мир не трогаем, ack откатывает к серверному
        let seq = l.start_prediction();
        l.predict_block(BlockPos::new(37, 70, -41), BlockState::AIR);
        assert!(l.chunks.block(37, 70, -41).is_air());
        let mut w = Writer::new();
        w.block_pos(37, 70, -41).var_int(1);
        l.handle("block_update", &w.buf).unwrap();
        assert!(l.chunks.block(37, 70, -41).is_air());
        let mut w = Writer::new();
        w.var_int(seq);
        assert_eq!(
            l.handle("block_changed_ack", &w.buf).unwrap(),
            LevelEvent::BlocksChanged(vec![BlockPos::new(37, 70, -41)])
        );
        assert_eq!(l.chunks.block(37, 70, -41).id(), 1);
        // set_entity_data: флаги #0 (BYTE) → плавание/присед; без записи #0 — без изменений
        l.entities.add(crate::entities::Entity::new(
            7, 1, 0, [0.0; 3], 0.0, 0.0, 0.0,
        ));
        let mut w = Writer::new();
        w.var_int(7).u8(0).var_int(0).u8(0x12).u8(0xFF);
        l.handle("set_entity_data", &w.buf).unwrap();
        let e = &l.entities.map[&7];
        assert!(e.swimming() && e.crouching());
        let mut w = Writer::new();
        w.var_int(7).u8(6).var_int(21).var_int(3).u8(0xFF);
        l.handle("set_entity_data", &w.buf).unwrap();
        assert!(l.entities.map[&7].swimming());
        // block entity: приходит, потом блок меняется → удаляется
        let mut w = Writer::new();
        w.block_pos(37, 70, -41).var_int(3).u8(0x0a).u8(0);
        assert!(matches!(
            l.handle("block_entity_data", &w.buf).unwrap(),
            LevelEvent::BlockEntityChanged(_)
        ));
        assert_eq!(l.chunks.get(2, -3).unwrap().block_entities.len(), 1);
        let mut w = Writer::new();
        w.block_pos(37, 70, -41).var_int(0);
        l.handle("block_update", &w.buf).unwrap();
        assert!(l.chunks.get(2, -3).unwrap().block_entities.is_empty());
        // forget
        let mut w = Writer::new();
        w.i64(ezzmc_protocol::types::ChunkPos { x: 2, z: -3 }.as_long());
        assert_eq!(
            l.handle("forget_level_chunk", &w.buf).unwrap(),
            LevelEvent::ChunkDropped { x: 2, z: -3 }
        );
        assert_eq!(l.chunks.loaded_count(), 0);
    }

    #[test]
    fn time_and_misc_and_garbage() {
        let mut l = Level::new(F, 4);
        let mut w = Writer::new();
        w.i64(100)
            .var_int(1)
            .var_int(0)
            .var_long(-6000)
            .f32(0.0)
            .f32(0.0);
        assert_eq!(
            l.handle("set_time", &w.buf).unwrap(),
            LevelEvent::TimeUpdated
        );
        assert_eq!(
            (l.game_time, l.day_time, l.tick_day_time),
            (100, -6000, false)
        );
        let mut w = Writer::new();
        w.u8(1).f32(0.0);
        assert_eq!(
            l.handle("game_event", &w.buf).unwrap(),
            LevelEvent::WeatherChanged
        );
        assert!(l.raining);
        let mut w = Writer::new();
        w.u8(8).f32(2.5);
        l.handle("game_event", &w.buf).unwrap();
        assert_eq!(l.thunder_level, 1.0);
        // VANILLA: getThunderLevel = thunder * rain (rain после события 1 = 0).
        assert_eq!(l.thunder(), 0.0);
        let mut w = Writer::new();
        w.u8(2).f32(0.0);
        l.handle("game_event", &w.buf).unwrap();
        assert!(!l.raining && l.rain_level == 1.0);
        let mut w = Writer::new();
        w.u8(3).f32(1.0);
        assert_eq!(l.handle("game_event", &w.buf).unwrap(), LevelEvent::None);
        assert_eq!(l.handle("something_else", &[1]).unwrap(), LevelEvent::None);
        for name in [
            "level_chunk_with_light",
            "block_update",
            "section_blocks_update",
            "light_update",
            "block_entity_data",
        ] {
            for len in 0..12 {
                let _ = l.handle(name, &vec![0xff; len]);
            }
        }
    }
}
