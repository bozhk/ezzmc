//! Частицы (TODO-155, часть): ядро `ParticleEngine`, физика `Particle#move`, осколки блоков
//! (`TerrainParticle`), капли дождя (`WaterDropParticle`), дым (`SmokeParticle`), спавн брызг дождя
//! (`ClientLevel#tickWeatherEffects`), разрушение блока (`ClientLevel#addDestroyBlockEffect`),
//! пакеты `level_event` 2001 и `level_particles`. Остальные типы частиц — позже (TODO-155).

use crate::flammable::FLAMMABLE;
use ezzmc_assets::bake::SpriteUv;
use ezzmc_mesh::blockinfo;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::types::BlockPos;
use ezzmc_render::particledraw::{ParticleAtlas, ParticleQuad};
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::block::{BlockState, FLAG_CAN_OCCLUDE};
use ezzmc_world::chunk_packet::HeightmapType;
use ezzmc_world::level::{Level, ParticlesPacket};
use ezzmc_world::noise::LegacyRandom;
use ezzmc_world::precipitation::{precipitation_at, Precipitation};
use std::collections::{HashMap, VecDeque};

/// VANILLA: ParticleEngine.MAX_PARTICLES_PER_LAYER.
const MAX_PARTICLES: usize = 16384;
/// VANILLA-UNSURE: Level#getSeaLevel overworld.
const SEA_LEVEL: i32 = 63;
const MAX_COLLISION_VELOCITY_SQ: f64 = 100.0 * 100.0;

/// Спрайты по именам файлов `particles/<name>.json` (VANILLA: ParticleResources/SpriteSet).
#[derive(Default)]
pub struct SpriteSets {
    sets: HashMap<String, Vec<SpriteUv>>,
}

impl SpriteSets {
    pub fn insert(&mut self, name: &str, sprites: Vec<SpriteUv>) {
        if !sprites.is_empty() {
            self.sets.insert(name.to_owned(), sprites);
        }
    }
    fn get(&self, name: &str) -> Option<&Vec<SpriteUv>> {
        self.sets.get(name)
    }
}

/// Спрайт и множитель цвета осколка блока (считает вызывающий: нужны модели и биомы).
#[derive(Clone, Copy, Debug)]
pub struct TerrainLook {
    pub sprite: SpriteUv,
    /// Тинт блока, 0..1 (VANILLA: BlockTintSource#colorAsTerrainParticle).
    pub tint: [f32; 3],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    WaterDrop,
    Terrain,
    /// BaseAshSmokeParticle (дым).
    Smoke,
    /// SculkChargeParticle / SculkChargePopParticle (full-bright, спрайт по возрасту).
    Sculk,
    /// ShriekParticle (двойной повёрнутый квад, задержка).
    Shriek,
    /// FlameParticle (RisingParticle).
    Flame,
    /// LavaParticle.
    Lava,
    /// DripParticle (все стадии, см. `Stage`).
    Drip,
    /// BubbleParticle.
    Bubble,
    /// ExplodeParticle (poof).
    Explode,
    /// HugeExplosionParticle.
    Huge,
    /// PortalParticle (жемчуг Энда, портал).
    Portal,
    /// CritParticle (крит-стрела, удар критом).
    Crit,
    /// GlowParticle (светящийся кальмар).
    Glow,
    /// DragonBreathParticle (фиолетовое дыхание дракона).
    DragonBreath,
    /// EndRodParticle (SimpleAnimatedParticle с затуханием цвета).
    EndRod,
    /// FireworkParticles.SparkParticle (SimpleAnimatedParticle + twinkle + fade).
    Firework,
    /// FireworkParticles.OverlayParticle (вспышка 4 тика).
    Flash,
    /// FlyTowardsPositionParticle.VaultConnectionProvider (светится, LifetimeAlpha 0→0.6).
    VaultConnection,
}

/// VANILLA: DripParticle.* — стадия капли.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    HangWater,
    HangLava,
    FallWater,
    FallLava,
    LandLava,
    /// VANILLA: DripParticle.DripstoneWaterHangProvider / DripstoneLavaHangProvider.
    StoneHangWater,
    StoneHangLava,
    /// VANILLA: DripstoneFallAndLandParticle (со звуком падения капли).
    StoneFallWater,
    StoneFallLava,
}

impl Stage {
    fn set_name(self) -> &'static str {
        match self {
            Stage::HangWater => "dripping_water",
            Stage::HangLava => "dripping_lava",
            Stage::FallWater => "falling_water",
            Stage::FallLava | Stage::StoneFallLava => "falling_lava",
            Stage::LandLava => "landing_lava",
            Stage::StoneHangWater => "dripping_water",
            Stage::StoneHangLava => "dripping_lava",
            Stage::StoneFallWater => "falling_water",
        }
    }

    fn is_hang(self) -> bool {
        matches!(
            self,
            Stage::HangWater | Stage::HangLava | Stage::StoneHangWater | Stage::StoneHangLava
        )
    }

    fn lava(self) -> bool {
        matches!(
            self,
            Stage::HangLava
                | Stage::FallLava
                | Stage::LandLava
                | Stage::StoneHangLava
                | Stage::StoneFallLava
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DripFluid {
    Empty,
    Water,
    Lava,
}

/// VANILLA: SpeleothemBlock#isFreeHangingStalactite.
fn free_hanging_stalactite(st: BlockState) -> bool {
    st.name() == "pointed_dripstone"
        && st.property("vertical_direction") == Some("down")
        && st.property("thickness") == Some("tip")
        && st.property("waterlogged") != Some("true")
}

/// VANILLA: PointedDripstoneBlock#getFluidAboveStalactite (findRootBlock, 11 шагов).
/// Возвращает жидкость над корнем и позицию блока над корнем.
fn fluid_above_stalactite(level: &Level, x: i32, y: i32, z: i32) -> Option<(DripFluid, [i32; 3])> {
    let st = level.chunks.block(x, y, z);
    let is_stalactite = |s: BlockState| {
        s.name() == "pointed_dripstone" && s.property("vertical_direction") == Some("down")
    };
    if !is_stalactite(st) {
        return None;
    }
    // findRootBlock: вверх до первого блока, не являющегося pointed_dripstone; путь — только tip_direction=down
    let mut root_y = None;
    for i in 1..11 {
        let s = level.chunks.block(x, y + i, z);
        if s.name() != "pointed_dripstone" {
            root_y = Some(y + i);
            break;
        }
        if !is_stalactite(s) {
            return None;
        }
    }
    let root_y = root_y?;
    let above = [x, root_y + 1, z];
    let a = level.chunks.block(above[0], above[1], above[2]);
    // VANILLA-UNSURE: mud + !WATER_EVAPORATES → вода
    let nether = level
        .dimension
        .as_ref()
        .is_some_and(|d| d.has_ceiling && !d.has_skylight);
    let fluid = if a.name() == "mud" && !nether {
        DripFluid::Water
    } else {
        match blockinfo::fluid(a) {
            Some(f) if f.lava => DripFluid::Lava,
            Some(_) => DripFluid::Water,
            None => DripFluid::Empty,
        }
    };
    Some((fluid, above))
}

struct Particle {
    kind: Kind,
    atlas: ParticleAtlas,
    sprite: SpriteUv,
    /// Для смены спрайта по возрасту.
    set: Option<Vec<SpriteUv>>,
    xo: f64,
    yo: f64,
    zo: f64,
    x: f64,
    y: f64,
    z: f64,
    xd: f64,
    yd: f64,
    zd: f64,
    bb_w: f32,
    bb_h: f32,
    bb: [f64; 6],
    on_ground: bool,
    has_physics: bool,
    stopped: bool,
    removed: bool,
    age: i32,
    lifetime: i32,
    gravity: f32,
    friction: f32,
    speed_up_blocked: bool,
    quad_size: f32,
    rgb: [f32; 3],
    alpha: f32,
    uo: f32,
    vo: f32,
    rng: LegacyRandom,
    /// PortalParticle: стартовая позиция.
    start: [f64; 3],
    /// LavaParticle#tick: дым, который надо породить после тика (x, y, z, xd, yd, zd).
    pending_smoke: Option<[f64; 6]>,
    drip: Option<Stage>,
    /// Следующая стадия капли / splash, порождаемая после тика: (стадия или None=splash, x, y, z).
    pending_drip: Option<(Option<Stage>, [f64; 3])>,
    /// ShriekParticle#delay.
    delay: i32,
    /// SculkChargeParticle#roll.
    roll: f32,
    /// FireworkParticles.SparkParticle: целевой цвет затухания и мерцание.
    fade: Option<[f32; 3]>,
    twinkle: bool,
    /// SparkParticle#trail и флаг «породить искру шлейфа после тика».
    trail: bool,
    pending_trail: bool,
}

fn collision_boxes(state: BlockState) -> &'static [[f64; 6]] {
    state
        .props()
        .map_or(&[], |p| ezzmc_world::block::shape(p.collision_shape))
}

/// VANILLA: VoxelShape#max(axis Y, x, z).
fn shape_max_y(boxes: &[[f64; 6]], x: f64, z: f64) -> f64 {
    boxes
        .iter()
        .filter(|b| x >= b[0] && x < b[3] && z >= b[2] && z < b[5])
        .map(|b| b[4])
        .fold(0.0, f64::max)
}

/// VANILLA: FluidState#getHeight (над жидкостью та же жидкость → 1).
fn fluid_height(level: &Level, x: i32, y: i32, z: i32) -> f64 {
    let Some(f) = blockinfo::fluid(level.chunks.block(x, y, z)) else {
        return 0.0;
    };
    if f.same(blockinfo::fluid(level.chunks.block(x, y + 1, z))) {
        1.0
    } else {
        f64::from(f.own_height())
    }
}

/// VANILLA: Shapes#collide по одной оси (eps 1e-7).
fn collide_axis(axis: usize, bb: &[f64; 6], boxes: &[[f64; 6]], mut d: f64) -> f64 {
    if d == 0.0 {
        return d;
    }
    let (a1, a2) = match axis {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };
    for b in boxes {
        // перекрытие по двум другим осям (строгое)
        if bb[a1 + 3] <= b[a1] + 1e-7
            || bb[a1] >= b[a1 + 3] - 1e-7
            || bb[a2 + 3] <= b[a2] + 1e-7
            || bb[a2] >= b[a2 + 3] - 1e-7
        {
            continue;
        }
        if d > 0.0 && b[axis] >= bb[axis + 3] - 1e-7 {
            d = d.min(b[axis] - bb[axis + 3]);
        } else if d < 0.0 && b[axis + 3] <= bb[axis] + 1e-7 {
            d = d.max(b[axis + 3] - bb[axis]);
        }
    }
    d
}

fn shift(bb: &mut [f64; 6], axis: usize, d: f64) {
    bb[axis] += d;
    bb[axis + 3] += d;
}

/// VANILLA: Entity#collideBoundingBox (порядок осей Y, затем X/Z по большему сдвигу).
fn collide_bb(level: &Level, bb: &[f64; 6], mv: [f64; 3]) -> [f64; 3] {
    // боксы блоков в расширенном по движению AABB
    let lo = |a: usize| (bb[a].min(bb[a] + mv[a]) - 1e-7).floor() as i32 - 1;
    let hi = |a: usize| (bb[a + 3].max(bb[a + 3] + mv[a]) + 1e-7).floor() as i32 + 1;
    let mut boxes: Vec<[f64; 6]> = Vec::new();
    for x in lo(0)..=hi(0) {
        for y in lo(1)..=hi(1) {
            for z in lo(2)..=hi(2) {
                let st = level.chunks.block(x, y, z);
                for b in collision_boxes(st) {
                    boxes.push([
                        b[0] + f64::from(x),
                        b[1] + f64::from(y),
                        b[2] + f64::from(z),
                        b[3] + f64::from(x),
                        b[4] + f64::from(y),
                        b[5] + f64::from(z),
                    ]);
                }
            }
        }
    }
    if boxes.is_empty() {
        return mv;
    }
    let mut cur = *bb;
    let mut r = mv;
    if r[1] != 0.0 {
        r[1] = collide_axis(1, &cur, &boxes, r[1]);
        shift(&mut cur, 1, r[1]);
    }
    let z_first = r[0].abs() < r[2].abs();
    if z_first && r[2] != 0.0 {
        r[2] = collide_axis(2, &cur, &boxes, r[2]);
        shift(&mut cur, 2, r[2]);
    }
    if r[0] != 0.0 {
        r[0] = collide_axis(0, &cur, &boxes, r[0]);
        shift(&mut cur, 0, r[0]);
    }
    if !z_first && r[2] != 0.0 {
        r[2] = collide_axis(2, &cur, &boxes, r[2]);
    }
    r
}

impl Particle {
    /// VANILLA: Particle(level, x, y, z).
    fn base(kind: Kind, x: f64, y: f64, z: f64, seed: i64) -> Particle {
        let mut rng = LegacyRandom::new(seed);
        let lifetime = (4.0 / (rng.next_float() * 0.9 + 0.1)) as i32;
        let mut p = Particle {
            kind,
            atlas: ParticleAtlas::Particles,
            sprite: SpriteUv {
                u0: 0.0,
                v0: 0.0,
                u1: 1.0,
                v1: 1.0,
            },
            set: None,
            xo: x,
            yo: y,
            zo: z,
            x,
            y,
            z,
            xd: 0.0,
            yd: 0.0,
            zd: 0.0,
            bb_w: 0.6,
            bb_h: 1.8,
            bb: [0.0; 6],
            on_ground: false,
            has_physics: true,
            stopped: false,
            removed: false,
            age: 0,
            lifetime,
            gravity: 0.0,
            friction: 0.98,
            speed_up_blocked: false,
            quad_size: 0.0,
            rgb: [1.0; 3],
            alpha: 1.0,
            uo: 0.0,
            vo: 0.0,
            rng,
            start: [x, y, z],
            pending_smoke: None,
            drip: None,
            pending_drip: None,
            delay: 0,
            roll: 0.0,
            fade: None,
            twinkle: false,
            trail: false,
            pending_trail: false,
        };
        p.set_size(0.2, 0.2);
        p.set_pos(x, y, z);
        p
    }

    /// VANILLA: Particle(level, x, y, z, xa, ya, za) + SingleQuadParticle quadSize.
    #[allow(clippy::too_many_arguments)]
    fn with_speed(
        kind: Kind,
        x: f64,
        y: f64,
        z: f64,
        xa: f64,
        ya: f64,
        za: f64,
        seed: i64,
    ) -> Self {
        let mut p = Self::base(kind, x, y, z, seed);
        let r = &mut p.rng;
        p.xd = xa + f64::from(r.next_float() * 2.0 - 1.0) * f64::from(0.4_f32);
        p.yd = ya + f64::from(r.next_float() * 2.0 - 1.0) * f64::from(0.4_f32);
        p.zd = za + f64::from(r.next_float() * 2.0 - 1.0) * f64::from(0.4_f32);
        let speed = f64::from((r.next_float() + r.next_float() + 1.0) * 0.15);
        let dd = (p.xd * p.xd + p.yd * p.yd + p.zd * p.zd).sqrt();
        p.xd = p.xd / dd * speed * f64::from(0.4_f32);
        p.yd = p.yd / dd * speed * f64::from(0.4_f32) + f64::from(0.1_f32);
        p.zd = p.zd / dd * speed * f64::from(0.4_f32);
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0;
        p
    }

    fn set_size(&mut self, w: f32, h: f32) {
        if w != self.bb_w || h != self.bb_h {
            self.bb_w = w;
            self.bb_h = h;
            let b = self.bb;
            let nx = (b[0] + b[3] - f64::from(w)) / 2.0;
            let nz = (b[2] + b[5] - f64::from(w)) / 2.0;
            self.bb = [
                nx,
                b[1],
                nz,
                nx + f64::from(w),
                b[1] + f64::from(h),
                nz + f64::from(w),
            ];
        }
    }

    fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        let w = f64::from(self.bb_w) / 2.0;
        let h = f64::from(self.bb_h);
        self.bb = [x - w, y, z - w, x + w, y + h, z + w];
    }

    /// VANILLA: Particle#move.
    fn do_move(&mut self, level: &Level, mut xa: f64, mut ya: f64, mut za: f64) {
        if self.stopped {
            return;
        }
        let (oxa, oya, oza) = (xa, ya, za);
        if self.has_physics
            && (xa != 0.0 || ya != 0.0 || za != 0.0)
            && xa * xa + ya * ya + za * za < MAX_COLLISION_VELOCITY_SQ
        {
            let m = collide_bb(level, &self.bb, [xa, ya, za]);
            (xa, ya, za) = (m[0], m[1], m[2]);
        }
        if xa != 0.0 || ya != 0.0 || za != 0.0 {
            shift(&mut self.bb, 0, xa);
            shift(&mut self.bb, 1, ya);
            shift(&mut self.bb, 2, za);
            self.x = (self.bb[0] + self.bb[3]) / 2.0;
            self.y = self.bb[1];
            self.z = (self.bb[2] + self.bb[5]) / 2.0;
        }
        if oya.abs() >= f64::from(1.0e-5_f32) && ya.abs() < f64::from(1.0e-5_f32) {
            self.stopped = true;
        }
        self.on_ground = oya != ya && oya < 0.0;
        if oxa != xa {
            self.xd = 0.0;
        }
        if oza != za {
            self.zd = 0.0;
        }
    }

    /// VANILLA: Particle#tick.
    fn base_tick(&mut self, level: &Level) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
        let expired = self.age >= self.lifetime;
        self.age += 1;
        if expired {
            self.removed = true;
            return;
        }
        self.yd -= 0.04 * f64::from(self.gravity);
        let (xd, yd, zd) = (self.xd, self.yd, self.zd);
        self.do_move(level, xd, yd, zd);
        if self.speed_up_blocked && self.y == self.yo {
            self.xd *= 1.1;
            self.zd *= 1.1;
        }
        let f = f64::from(self.friction);
        self.xd *= f;
        self.yd *= f;
        self.zd *= f;
        if self.on_ground {
            self.xd *= f64::from(0.7_f32);
            self.zd *= f64::from(0.7_f32);
        }
    }

    fn tick(&mut self, level: &Level) {
        match self.kind {
            Kind::Terrain => self.base_tick(level),
            Kind::Smoke => {
                self.base_tick(level);
                self.sprite_from_age();
            }
            // VANILLA: CritParticle#tick.
            Kind::Crit => {
                self.base_tick(level);
                self.rgb[1] *= 0.96;
                self.rgb[2] *= 0.9;
            }
            Kind::WaterDrop => self.water_drop_tick(level),
            Kind::Bubble => self.bubble_tick(level),
            Kind::VaultConnection => {
                // VANILLA: FlyTowardsPositionParticle#tick.
                self.xo = self.x;
                self.yo = self.y;
                self.zo = self.z;
                let expired = self.age >= self.lifetime;
                self.age += 1;
                if expired {
                    self.removed = true;
                } else {
                    let pos = 1.0 - self.age as f32 / self.lifetime as f32;
                    let pp = (1.0 - pos).powi(4);
                    let pos = f64::from(pos);
                    self.x = self.start[0] + self.xd * pos;
                    self.y = self.start[1] + self.yd * pos - f64::from(pp * 1.2);
                    self.z = self.start[2] + self.zd * pos;
                }
            }
            Kind::Portal => {
                // VANILLA: PortalParticle#tick.
                self.xo = self.x;
                self.yo = self.y;
                self.zo = self.z;
                let expired = self.age >= self.lifetime;
                self.age += 1;
                if expired {
                    self.removed = true;
                } else {
                    let a = self.age as f32 / self.lifetime as f32;
                    let pos = 1.0 - (-a + a * a * 2.0);
                    let pos = f64::from(pos);
                    self.x = self.start[0] + self.xd * pos;
                    self.y = self.start[1] + self.yd * pos + f64::from(1.0 - a);
                    self.z = self.start[2] + self.zd * pos;
                }
            }
            // VANILLA: DragonBreathParticle#tick (hasPhysics=false → onGround/hasHitGround недостижимы).
            Kind::DragonBreath => {
                self.xo = self.x;
                self.yo = self.y;
                self.zo = self.z;
                let expired = self.age >= self.lifetime;
                self.age += 1;
                if expired {
                    self.removed = true;
                } else {
                    self.sprite_from_age();
                    let (xd, yd, zd) = (self.xd, self.yd, self.zd);
                    self.do_move(level, xd, yd, zd);
                    if self.y == self.yo {
                        self.xd *= 1.1;
                        self.zd *= 1.1;
                    }
                    self.xd *= f64::from(self.friction);
                    self.zd *= f64::from(self.friction);
                }
            }
            // VANILLA: FireworkParticles.SparkParticle#tick (SimpleAnimatedParticle).
            Kind::Firework => {
                self.base_tick(level);
                self.sprite_from_age();
                if self.age > self.lifetime / 2 {
                    self.alpha = 1.0
                        - (self.age as f32 - (self.lifetime / 2) as f32)
                            / self.lifetime.max(1) as f32;
                    if let Some(fade) = self.fade {
                        for (c, f) in self.rgb.iter_mut().zip(fade) {
                            *c += (f - *c) * 0.2;
                        }
                    }
                }
                // VANILLA: SparkParticle#tick — искра шлейфа каждые 2 тика первой половины жизни.
                if self.trail
                    && !self.removed
                    && self.age < self.lifetime / 2
                    && (self.age + self.lifetime) % 2 == 0
                {
                    self.pending_trail = true;
                }
            }
            // VANILLA: OverlayParticle — Particle#tick (gravity 0, скорость 0), lifetime 4;
            // альфа считается в extract.
            Kind::Flash => self.base_tick(level),
            // VANILLA: SimpleAnimatedParticle#tick (EndRodParticle: fade 0xF2DEC9, без коллизий).
            Kind::EndRod => {
                self.base_tick(level);
                self.sprite_from_age();
                if self.age > self.lifetime / 2 {
                    self.alpha =
                        1.0 - (self.age as f32 - (self.lifetime / 2) as f32) / self.lifetime as f32;
                    let fade = [242.0 / 255.0, 222.0 / 255.0, 201.0 / 255.0];
                    for (c, f) in self.rgb.iter_mut().zip(fade) {
                        *c += (f - *c) * 0.2;
                    }
                }
            }
            Kind::Flame => self.base_tick(level),
            Kind::Sculk => {
                self.base_tick(level);
                self.sprite_from_age();
            }
            // VANILLA: ShriekParticle#tick.
            Kind::Shriek => {
                if self.delay > 0 {
                    self.delay -= 1;
                } else {
                    self.base_tick(level);
                }
            }
            // VANILLA: GlowParticle#tick.
            Kind::Glow => {
                self.base_tick(level);
                self.sprite_from_age();
            }
            Kind::Explode => {
                self.base_tick(level);
                self.sprite_from_age();
            }
            Kind::Huge => {
                // VANILLA: HugeExplosionParticle#tick — без движения, только смена спрайта.
                self.xo = self.x;
                self.yo = self.y;
                self.zo = self.z;
                let expired = self.age >= self.lifetime;
                self.age += 1;
                if expired {
                    self.removed = true;
                } else {
                    self.sprite_from_age();
                }
            }
            Kind::Drip => self.drip_tick(level),
            Kind::Lava => {
                // VANILLA: LavaParticle#tick
                self.base_tick(level);
                if !self.removed {
                    let odds = self.age as f32 / self.lifetime as f32;
                    if self.rng.next_float() > odds {
                        self.pending_smoke =
                            Some([self.x, self.y, self.z, self.xd, self.yd, self.zd]);
                    }
                }
            }
        }
    }

    /// VANILLA: DripParticle#tick (+ Hang/Cooling/FallAndLand/Land).
    fn drip_tick(&mut self, level: &Level) {
        let Some(stage) = self.drip else {
            return;
        };
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
        // preMoveUpdate
        if matches!(stage, Stage::HangLava | Stage::StoneHangLava) {
            let l = self.lifetime as f32;
            self.rgb = [1.0, 16.0 / (40.0 - l + 16.0), 4.0 / (40.0 - l + 8.0)];
        }
        let was = self.lifetime;
        self.lifetime -= 1;
        if was <= 0 {
            self.removed = true;
            if stage.is_hang() {
                let next = match stage {
                    Stage::HangLava => Stage::FallLava,
                    Stage::StoneHangLava => Stage::StoneFallLava,
                    Stage::StoneHangWater => Stage::StoneFallWater,
                    _ => Stage::FallWater,
                };
                self.pending_drip = Some((Some(next), [self.x, self.y, self.z]));
            }
            return;
        }
        self.yd -= f64::from(self.gravity);
        let (xd, yd, zd) = (self.xd, self.yd, self.zd);
        self.do_move(level, xd, yd, zd);
        // postMoveUpdate
        if stage.is_hang() {
            self.xd *= 0.02;
            self.yd *= 0.02;
            self.zd *= 0.02;
        } else if self.on_ground && stage != Stage::LandLava {
            self.removed = true;
            let next = if matches!(stage, Stage::FallLava | Stage::StoneFallLava) {
                Some(Stage::LandLava)
            } else {
                None
            };
            if matches!(stage, Stage::StoneFallWater | Stage::StoneFallLava) {
                // VANILLA: DripstoneFallAndLandParticle#postMoveUpdate (randomBetween(0.3, 1.0))
                let ev = if stage == Stage::StoneFallLava {
                    "block.pointed_dripstone.drip_lava"
                } else {
                    "block.pointed_dripstone.drip_water"
                };
                let vol = 0.3 + self.rng.next_float() * 0.7;
                crate::audio::play_at(
                    ev,
                    crate::audio::Cat::Blocks,
                    self.x,
                    self.y,
                    self.z,
                    vol,
                    1.0,
                );
            }
            self.pending_drip = Some((next, [self.x, self.y, self.z]));
            return;
        }
        let f = f64::from(0.98_f32);
        self.xd *= f;
        self.yd *= f;
        self.zd *= f;
        let (bx, by, bz) = (
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        );
        if let Some(fl) = blockinfo::fluid(level.chunks.block(bx, by, bz)) {
            if fl.lava == stage.lava() && self.y < f64::from(by) + fluid_height(level, bx, by, bz) {
                self.removed = true;
            }
        }
    }

    /// VANILLA: SpriteSet#get(age, lifetime) = textures[age * (n−1) / lifetime].
    fn sprite_from_age(&mut self) {
        if self.removed {
            return;
        }
        if let Some(set) = &self.set {
            let i = (self.age * (set.len() as i32 - 1) / self.lifetime.max(1)).max(0) as usize;
            if let Some(s) = set.get(i.min(set.len() - 1)) {
                self.sprite = *s;
            }
        }
    }

    /// VANILLA: BubbleParticle#tick.
    fn bubble_tick(&mut self, level: &Level) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
        let was = self.lifetime;
        self.lifetime -= 1;
        if was <= 0 {
            self.removed = true;
            return;
        }
        self.yd += 0.002;
        let (xd, yd, zd) = (self.xd, self.yd, self.zd);
        self.do_move(level, xd, yd, zd);
        let f = f64::from(0.85_f32);
        self.xd *= f;
        self.yd *= f;
        self.zd *= f;
        let st = level.chunks.block(
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        );
        if !blockinfo::fluid(st).is_some_and(|f| !f.lava) {
            self.removed = true;
        }
    }

    /// VANILLA: WaterDropParticle#tick.
    fn water_drop_tick(&mut self, level: &Level) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
        let was = self.lifetime;
        self.lifetime -= 1;
        if was <= 0 {
            self.removed = true;
            return;
        }
        self.yd -= f64::from(self.gravity);
        let (xd, yd, zd) = (self.xd, self.yd, self.zd);
        self.do_move(level, xd, yd, zd);
        let f = f64::from(0.98_f32);
        self.xd *= f;
        self.yd *= f;
        self.zd *= f;
        if self.on_ground {
            if self.rng.next_float() < 0.5 {
                self.removed = true;
            }
            self.xd *= f64::from(0.7_f32);
            self.zd *= f64::from(0.7_f32);
        }
        let (bx, by, bz) = (
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        );
        let block_top = shape_max_y(
            collision_boxes(level.chunks.block(bx, by, bz)),
            self.x - f64::from(bx),
            self.z - f64::from(bz),
        );
        let offset = block_top.max(fluid_height(level, bx, by, bz));
        if offset > 0.0 && self.y < f64::from(by) + offset {
            self.removed = true;
        }
    }

    /// VANILLA: getLightCoords (Flame: addSmoothBlockEmission; Lava: block 15). Единицы 0..240.
    fn light(&self, level: &Level, partial: f32) -> (u16, u16) {
        let (b, sky) = light_at(level, self.x, self.y, self.z);
        let (b, sky) = (u16::from(b) * 16, u16::from(sky) * 16);
        match self.kind {
            Kind::Lava => (240, sky),
            // VANILLA: SimpleAnimatedParticle#getLightCoords = 15728880.
            Kind::EndRod | Kind::Firework | Kind::Flash => (240, 240),
            // VANILLA: HugeExplosionParticle#getLightCoords = 15728880.
            Kind::Huge => (240, 240),
            // VANILLA: ShriekParticle#getLightCoords = withBlock(…, 15).
            Kind::Shriek | Kind::Sculk => (240, sky),
            // VANILLA: FlyTowardsPositionParticle#getLightCoords — isGlowing → withBlock(…, 15).
            Kind::VaultConnection => (240, sky),
            // VANILLA: GlowParticle#getLightCoords — addSmoothBlockEmission(age/lifetime).
            Kind::Flame | Kind::Glow => {
                let t = ((self.age as f32 + partial) / self.lifetime as f32).clamp(0.0, 1.0);
                ((f32::from(b) + t * 240.0).min(240.0) as u16, sky)
            }
            // VANILLA: PortalParticle#getLightCoords — свечение (age/lifetime)^4.
            Kind::Portal => {
                let t = (self.age as f32 / self.lifetime as f32)
                    .clamp(0.0, 1.0)
                    .powi(4);
                ((f32::from(b) + t * 240.0).min(240.0) as u16, sky)
            }
            _ => (b, sky),
        }
    }

    fn quad_size(&self, partial: f32) -> f32 {
        match self.kind {
            // VANILLA: ShriekParticle#getQuadSize.
            Kind::Shriek => {
                self.quad_size
                    * ((self.age as f32 + partial) / self.lifetime as f32 * 0.75).clamp(0.0, 1.0)
            }
            Kind::Smoke | Kind::Crit | Kind::DragonBreath => {
                self.quad_size
                    * ((self.age as f32 + partial) / self.lifetime as f32 * 32.0).clamp(0.0, 1.0)
            }
            // VANILLA: FlameParticle#getQuadSize / LavaParticle#getQuadSize
            Kind::Flame => {
                let s = (self.age as f32 + partial) / self.lifetime as f32;
                self.quad_size * (1.0 - s * s * 0.5)
            }
            Kind::Lava => {
                let s = (self.age as f32 + partial) / self.lifetime as f32;
                self.quad_size * (1.0 - s * s)
            }
            // VANILLA: PortalParticle#getQuadSize.
            Kind::Portal => {
                let s = (self.age as f32 + partial) / self.lifetime as f32;
                let s = 1.0 - s;
                self.quad_size * (1.0 - s * s)
            }
            // VANILLA: FireworkParticles.OverlayParticle#getQuadSize.
            Kind::Flash => {
                // Отрицательный размер = квад, повёрнутый на 180°, как в ванили.
                7.1 * ((self.age as f32 + partial - 1.0) * 0.25 * std::f32::consts::PI).sin()
            }
            _ => self.quad_size,
        }
    }

    fn uv(&self) -> (f32, f32, f32, f32) {
        let s = &self.sprite;
        let u = |f: f32| s.u0 + (s.u1 - s.u0) * f;
        let v = |f: f32| s.v0 + (s.v1 - s.v0) * f;
        if self.kind == Kind::Terrain {
            // VANILLA: TerrainParticle#getU0..getV1 — случайный 1/4 спрайта.
            (
                u((self.uo + 1.0) / 4.0),
                u(self.uo / 4.0),
                v(self.vo / 4.0),
                v((self.vo + 1.0) / 4.0),
            )
        } else {
            (s.u0, s.u1, s.v0, s.v1)
        }
    }
}

/// VANILLA: LightCoordsUtil.getLightCoords (нет чанка → небо 15).
pub(crate) fn light_at(level: &Level, x: f64, y: f64, z: f64) -> (u8, u8) {
    let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let Some(c) = level.chunks.get(bx >> 4, bz >> 4) else {
        return (0, 15);
    };
    let (lx, lz) = ((bx & 15) as usize, (bz & 15) as usize);
    (
        c.block_light(lx, by, lz).unwrap_or(0),
        c.sky_light(lx, by, lz).unwrap_or(15),
    )
}

pub struct ParticleEngine {
    list: VecDeque<Particle>,
    sets: SpriteSets,
    rng: LegacyRandom,
    /// Размер отладочного счётчика для F3 (`P:`).
    last_extracted: usize,
    /// VANILLA: ClientLevel#rainSoundTime.
    rain_sound_time: i32,
    /// VANILLA: Options#particles (0 all, 1 decreased, 2 minimal).
    pub status: u8,
    /// HugeExplosionSeedParticle: (позиция, возраст).
    seeds: Vec<([f64; 3], i32)>,
    /// ClientExplosionTracker: ожидающие взрывы.
    explosions: Vec<ExplosionInfo>,
    /// FireworkParticles.Starter (NoRenderParticle) в работе.
    starters: Vec<Starter>,
    /// Хранилища (vault) рядом с камерой: кладёт worldview каждый тик.
    vaults: Vec<VaultInfo>,
    /// Игроки мира для connection-частиц: (uuid, позиция ног, высота бокса).
    players: Vec<(u128, [f64; 3], f64)>,
}

/// Клиентское состояние хранилища. VANILLA: VaultSharedData (из update-тега `shared_data`)
/// + свойства блока FACING/OMINOUS.
#[derive(Clone, Debug, PartialEq)]
pub struct VaultInfo {
    pub pos: [i32; 3],
    /// Direction#getStepX/Z свойства `facing`.
    pub facing: [i32; 2],
    pub ominous: bool,
    pub has_display_item: bool,
    pub connected: Vec<u128>,
    /// VANILLA: VaultConfig.DEFAULT.deactivationRange = 4.5.
    pub range: f64,
}

impl VaultInfo {
    /// Разбор блок-сущности хранилища. `tag` — update-тег (`shared_data` внутри).
    pub fn from_block(pos: [i32; 3], facing: &str, ominous: bool, tag: Option<&Tag>) -> Self {
        let sd = tag.and_then(|t| t.get("shared_data"));
        let connected = match sd.and_then(|t| t.get("connected_players")) {
            // VANILLA: UUIDUtil.CODEC — int[4], старшие слова первыми.
            Some(Tag::List(l)) => l
                .iter()
                .filter_map(|u| match u {
                    Tag::IntArray(a) if a.len() == 4 => Some(
                        a.iter()
                            .fold(0u128, |acc, w| (acc << 32) | u128::from(*w as u32)),
                    ),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        let range = match sd.and_then(|t| t.get("connected_particles_range")) {
            Some(Tag::Double(d)) => *d,
            _ => 4.5,
        };
        let facing = match facing {
            "north" => [0, -1],
            "south" => [0, 1],
            "west" => [-1, 0],
            "east" => [1, 0],
            _ => [0, 0],
        };
        VaultInfo {
            pos,
            facing,
            ominous,
            has_display_item: sd.and_then(|t| t.get("display_item")).is_some(),
            connected,
            range,
        }
    }

    fn flame(&self) -> &'static str {
        if self.ominous {
            "soul_fire_flame"
        } else {
            "small_flame"
        }
    }
}

/// VANILLA: ClientExplosionTracker.ExplosionInfo; частицы — (имя, scaling, speed, weight).
pub struct ExplosionInfo {
    pub center: [f64; 3],
    pub radius: f32,
    pub block_count: i32,
    pub particles: Vec<(String, f32, f32, i32)>,
}

impl ParticleEngine {
    pub fn new(sets: SpriteSets) -> Self {
        ParticleEngine {
            list: VecDeque::new(),
            sets,
            rng: LegacyRandom::new(0x5EED_0001),
            last_extracted: 0,
            rain_sound_time: 0,
            status: 0,
            seeds: Vec::new(),
            explosions: Vec::new(),
            starters: Vec::new(),
            vaults: Vec::new(),
            players: Vec::new(),
        }
    }

    /// Обновляет кеш хранилищ и игроков (перед `vault_tick` и событиями 3015).
    pub fn set_vaults(&mut self, vaults: Vec<VaultInfo>, players: Vec<(u128, [f64; 3], f64)>) {
        self.vaults = vaults;
        self.players = players;
    }

    /// VANILLA: VaultBlockEntity.Client#tick (без updateDisplayItemSpin — это рендер предмета).
    pub fn vault_tick(&mut self, game_time: i64) {
        let vaults = std::mem::take(&mut self.vaults);
        for v in &vaults {
            if game_time % 20 == 0 {
                self.vault_connection(v);
            }
            // emitIdleParticles
            if self.rng.next_float() <= 0.5 {
                let p = self.vault_inside_cage(v.pos);
                self.add_smoke(p[0], p[1], p[2], 0.0, 0.0, 0.0);
                if v.has_display_item {
                    self.add_flame(v.flame(), p[0], p[1], p[2]);
                }
            }
            // playIdleSounds
            if v.has_display_item && self.rng.next_float() <= 0.02 {
                let vol = self.rng.next_float() * 0.25 + 0.75;
                let pitch = self.rng.next_float() + 0.5;
                let c = |i: usize| f64::from(v.pos[i]) + 0.5;
                crate::audio::play_at(
                    "block.vault.ambient",
                    crate::audio::Cat::Blocks,
                    c(0),
                    c(1),
                    c(2),
                    vol,
                    pitch,
                );
            }
        }
        self.vaults = vaults;
    }

    /// VANILLA: VaultBlockEntity.Client#randomPosInsideCage.
    fn vault_inside_cage(&mut self, p: [i32; 3]) -> [f64; 3] {
        [
            f64::from(p[0]) + 0.1 + self.rng.next_double() * 0.8,
            f64::from(p[1]) + 0.25 + self.rng.next_double() * 0.5,
            f64::from(p[2]) + 0.1 + self.rng.next_double() * 0.8,
        ]
    }

    /// VANILLA: VaultBlockEntity.Client#emitConnectionParticlesForNearbyPlayers.
    fn vault_connection(&mut self, v: &VaultInfo) {
        if v.connected.is_empty() {
            return;
        }
        // keyholePos: atBottomCenterOf(pos) + (stepX*0.5, 1.75, stepZ*0.5).
        let key = [
            f64::from(v.pos[0]) + 0.5 + f64::from(v.facing[0]) * 0.5,
            f64::from(v.pos[1]) + 1.75,
            f64::from(v.pos[2]) + 0.5 + f64::from(v.facing[1]) * 0.5,
        ];
        let players = std::mem::take(&mut self.players);
        for uuid in &v.connected {
            let Some((_, pp, h)) = players.iter().find(|p| p.0 == *uuid) else {
                continue;
            };
            // isWithinConnectionRange: blockPosition().distSqr(vaultPos) <= range².
            let d = |i: usize| f64::from(pp[i].floor() as i32 - v.pos[i]);
            if d(0) * d(0) + d(1) * d(1) + d(2) * d(2) > v.range * v.range {
                continue;
            }
            // emitConnectionParticlesForPlayer
            let dir = [pp[0] - key[0], pp[1] + h / 2.0 - key[1], pp[2] - key[2]];
            let n = 2 + self.rng.next_int_bound(4);
            for _ in 0..n {
                let mut r = [0.0; 3];
                for (k, rk) in r.iter_mut().enumerate() {
                    *rk = dir[k] + f64::from(self.rng.next_float() - 0.5);
                }
                self.add_vault_connection(key, r);
            }
        }
        self.players = players;
    }

    /// VANILLA: FlyTowardsPositionParticle.VaultConnectionProvider.
    fn add_vault_connection(&mut self, at: [f64; 3], d: [f64; 3]) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("vault_connection").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::VaultConnection, at[0], at[1], at[2], seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.xd = d[0];
        p.yd = d[1];
        p.zd = d[2];
        // xo = x + xd; x = xo (стартовая точка у игрока, частица летит к замочной скважине).
        p.xo = at[0] + d[0];
        p.yo = at[1] + d[1];
        p.zo = at[2] + d[2];
        p.x = p.xo;
        p.y = p.yo;
        p.z = p.zo;
        p.has_physics = false;
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.2) * 1.5;
        let br = p.rng.next_float() * 0.6 + 0.4;
        p.rgb = [0.9 * br, 0.9 * br, br];
        p.lifetime = (p.rng.next_float() * 10.0) as i32 + 30;
        self.add(p);
    }

    /// VANILLA: LevelRenderer#addParticle + calculateParticleLevel (MINIMAL — частицы не создаются;
    /// DECREASED — каждая 10-я как MINIMAL).
    fn allow(&mut self) -> bool {
        let mut st = self.status;
        if st == 1 && self.rng.next_int_bound(10) == 0 {
            st = 2;
        }
        st != 2
    }

    #[allow(dead_code)] // F3 `P:` — отладочный экран (TODO-155)
    pub fn starter_count(&self) -> usize {
        self.starters.len()
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    /// Живые частицы связи хранилища (MCP state).
    pub fn vault_connection_count(&self) -> usize {
        self.list
            .iter()
            .filter(|p| p.kind == Kind::VaultConnection)
            .count()
    }

    /// Хранилища в кеше клиентского тика (MCP state).
    pub fn vault_count(&self) -> usize {
        self.vaults.len()
    }

    #[allow(dead_code)] // сброс при смене мира (TODO-155)
    pub fn clear(&mut self) {
        self.list.clear();
    }

    fn seed(&mut self) -> i64 {
        self.rng.next_long()
    }

    fn add(&mut self, p: Particle) {
        if self.list.len() >= MAX_PARTICLES {
            self.list.pop_front();
        }
        self.list.push_back(p);
    }

    /// VANILLA: ParticleEngine#tick — тик всех частиц, удаление мёртвых.
    pub fn tick(&mut self, level: &Level, cam: [f64; 3]) {
        let mut smokes = Vec::new();
        let mut drips = Vec::new();
        let mut trails = Vec::new();
        for p in &mut self.list {
            p.tick(level);
            if std::mem::take(&mut p.pending_trail) {
                trails.push((p.x, p.y, p.z, p.rgb, p.fade, p.twinkle));
            }
            if let Some(s) = p.pending_smoke.take() {
                smokes.push(s);
            }
            if let Some(d) = p.pending_drip.take() {
                drips.push(d);
            }
        }
        for (stage, pos) in drips {
            match stage {
                Some(st) => self.add_drip(st, pos[0], pos[1], pos[2]),
                None => self.add_water_drop("splash", 0.04, pos[0], pos[1], pos[2]),
            }
        }
        self.list.retain(|p| !p.removed);
        for s in smokes {
            self.add_smoke(s[0], s[1], s[2], s[3], s[4], s[5]);
        }
        for (x, y, z, rgb, fade, twinkle) in trails {
            self.add_trail_spark(x, y, z, rgb, fade, twinkle);
        }
        self.tick_starters(cam);
        self.tick_explosions(level);
    }

    /// VANILLA: HugeExplosionSeedParticle#tick + ClientExplosionTracker#tick.
    fn tick_explosions(&mut self, level: &Level) {
        let seeds = std::mem::take(&mut self.seeds);
        for (pos, age) in seeds {
            for _ in 0..6 {
                let mut d = || (self.rng.next_double() - self.rng.next_double()) * 4.0;
                let (dx, dy, dz) = (d(), d(), d());
                self.add_huge(pos[0] + dx, pos[1] + dy, pos[2] + dz, f64::from(age) / 8.0);
            }
            if age + 1 < 8 {
                self.seeds.push((pos, age + 1));
            }
        }
        let list = std::mem::take(&mut self.explosions);
        if list.is_empty() || self.status != 0 {
            return;
        }
        let total: i32 = list.iter().map(|e| e.block_count.max(0)).sum();
        for _ in 0..total.min(512) {
            let mut pick = self.rng.next_int_bound(total.max(1));
            let Some(info) = list.iter().find(|e| {
                pick -= e.block_count.max(0);
                pick < 0
            }) else {
                continue;
            };
            let r = &mut self.rng;
            let dir = [
                f64::from(r.next_float() * 2.0 - 1.0),
                f64::from(r.next_float() * 2.0 - 1.0),
                f64::from(r.next_float() * 2.0 - 1.0),
            ];
            let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
            if len == 0.0 {
                continue;
            }
            let dir = [dir[0] / len, dir[1] / len, dir[2] / len];
            let radius = f64::from(r.next_float()).cbrt() * f64::from(info.radius);
            let local = [dir[0] * radius, dir[1] * radius, dir[2] * radius];
            let pos = [
                info.center[0] + local[0],
                info.center[1] + local[1],
                info.center[2] + local[2],
            ];
            let b = level.chunks.block(
                pos[0].floor() as i32,
                pos[1].floor() as i32,
                pos[2].floor() as i32,
            );
            if !b.is_air() {
                continue;
            }
            let speed = 0.5 / (radius / f64::from(info.radius) + 0.1)
                * f64::from(r.next_float())
                * f64::from(r.next_float())
                + 0.3;
            let wsum: i32 = info.particles.iter().map(|p| p.3.max(0)).sum();
            if wsum <= 0 {
                continue;
            }
            let mut w = r.next_int_bound(wsum);
            let Some((name, scaling, pspeed, _)) = info.particles.iter().find(|p| {
                w -= p.3.max(0);
                w < 0
            }) else {
                continue;
            };
            let (sc, sp) = (f64::from(*scaling), speed * f64::from(*pspeed));
            let (x, y, z) = (
                info.center[0] + local[0] * sc,
                info.center[1] + local[1] * sc,
                info.center[2] + local[2] * sc,
            );
            let (xa, ya, za) = (dir[0] * sp, dir[1] * sp, dir[2] * sp);
            match name.as_str() {
                "poof" => self.add_poof(x, y, z, xa, ya, za),
                "smoke" => self.add_smoke(x, y, z, xa, ya, za),
                _ => {}
            }
        }
    }

    /// VANILLA: ClientLevel#trackExplosionEffects.
    pub fn track_explosion(&mut self, info: ExplosionInfo) {
        if !info.particles.is_empty() {
            self.explosions.push(info);
        }
    }

    /// VANILLA: HugeExplosionSeedParticle (explosion_emitter).
    pub fn add_explosion_emitter(&mut self, x: f64, y: f64, z: f64) {
        if self.allow() {
            self.seeds.push(([x, y, z], 0));
        }
    }

    /// VANILLA: HugeExplosionParticle#<init> (explosion).
    pub fn add_huge(&mut self, x: f64, y: f64, z: f64, size: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("explosion").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Huge, x, y, z, seed);
        p.lifetime = 6 + p.rng.next_int_bound(4);
        let col = p.rng.next_float() * 0.6 + 0.4;
        p.rgb = [col; 3];
        p.quad_size = 2.0 * (1.0 - size as f32 * 0.5);
        p.sprite = set[0];
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: ExplodeParticle#<init> (poof).
    pub fn add_poof(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("poof").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Explode, x, y, z, seed);
        p.gravity = -0.1;
        p.friction = 0.9;
        let r = &mut p.rng;
        p.xd = xa + f64::from((r.next_float() * 2.0 - 1.0) * 0.05);
        p.yd = ya + f64::from((r.next_float() * 2.0 - 1.0) * 0.05);
        p.zd = za + f64::from((r.next_float() * 2.0 - 1.0) * 0.05);
        let col = r.next_float() * 0.3 + 0.7;
        p.rgb = [col; 3];
        p.quad_size = 0.1 * (r.next_float() * r.next_float() * 6.0 + 1.0);
        p.lifetime = (16.0 / (f64::from(r.next_float()) * 0.8 + 0.2)) as i32 + 2;
        p.sprite = set[0];
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: FlameParticle.Provider / RisingParticle#<init> (`name`: flame, soul_fire_flame, …).
    pub fn add_flame(&mut self, name: &str, x: f64, y: f64, z: f64) {
        self.add_flame_v(name, x, y, z, [0.0; 3]);
    }

    /// VANILLA: RisingParticle (xd = xd*0.01 + xa) — пламя со скоростью.
    pub fn add_flame_v(&mut self, name: &str, x: f64, y: f64, z: f64, v: [f64; 3]) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Flame, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        if name == "small_flame" {
            // VANILLA: FlameParticle.SmallFlameProvider — scale(0.5).
            p.quad_size *= 0.5;
        }
        p.has_physics = false;
        p.friction = 0.96;
        p.xd = p.xd * f64::from(0.01_f32) + v[0];
        p.yd = p.yd * f64::from(0.01_f32) + v[1];
        p.zd = p.zd * f64::from(0.01_f32) + v[2];
        let r = &mut p.rng;
        p.x += f64::from((r.next_float() - r.next_float()) * 0.05);
        p.y += f64::from((r.next_float() - r.next_float()) * 0.05);
        p.z += f64::from((r.next_float() - r.next_float()) * 0.05);
        p.lifetime = (8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32 + 4;
        let (px, py, pz) = (p.x, p.y, p.z);
        p.set_pos(px, py, pz);
        p.xo = px;
        p.yo = py;
        p.zo = pz;
        self.add(p);
    }

    /// VANILLA: SculkChargePopParticle.Provider (без физики, friction 0.96, lifetime 6..9).
    pub fn add_sculk_pop(&mut self, x: f64, y: f64, z: f64, v: [f64; 3]) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("sculk_charge_pop").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Sculk, x, y, z, v[0], v[1], v[2], seed);
        p.xd = v[0];
        p.yd = v[1];
        p.zd = v[2];
        p.friction = 0.96;
        p.has_physics = false;
        p.gravity = 0.0;
        p.lifetime = p.rng.next_int_bound(4) + 6;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: SculkChargeParticle.Provider (scale 1.5, lifetime nextInt(12)+8, roll).
    pub fn add_sculk_charge(&mut self, x: f64, y: f64, z: f64, v: [f64; 3], roll: f32) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("sculk_charge").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Sculk, x, y, z, v[0], v[1], v[2], seed);
        p.xd = v[0];
        p.yd = v[1];
        p.zd = v[2];
        p.friction = 0.96;
        p.has_physics = false;
        p.gravity = 0.0;
        p.quad_size *= 1.5;
        p.roll = roll;
        p.lifetime = p.rng.next_int_bound(12) + 8;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: LevelEventHandler 3006 (count > 0): звук + sculk_charge по граням.
    pub fn sculk_charge(&mut self, pos: [i32; 3], data: i32) {
        let count = data >> 6;
        let (cx, cy, cz) = (
            f64::from(pos[0]) + 0.5,
            f64::from(pos[1]) + 0.5,
            f64::from(pos[2]) + 0.5,
        );
        if self.rng.next_float() < 0.3 + count as f32 * 0.1 {
            let c = count as f32;
            let vol = 0.15 + 0.02 * c * c * self.rng.next_float();
            let pitch = 0.4 + 0.3 * c * self.rng.next_float();
            crate::audio::play_at(
                "block.sculk.charge",
                crate::audio::Cat::Blocks,
                cx,
                cy,
                cz,
                vol,
                pitch,
            );
        }
        let bits = data & 63;
        // Direction.values(): DOWN, UP, NORTH, SOUTH, WEST, EAST
        let dirs = [
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [-1, 0, 0],
            [1, 0, 0],
        ];
        for (i, step) in dirs.into_iter().enumerate() {
            let (roll, factor) = if bits == 0 {
                (
                    if i == 0 { std::f32::consts::PI } else { 0.0 },
                    if i < 2 { 0.65 } else { 0.57 },
                )
            } else if bits & (1 << i) != 0 {
                (if i == 1 { std::f32::consts::PI } else { 0.0 }, 0.35)
            } else {
                continue;
            };
            let n = self.rng.next_int_bound(count + 1);
            for _ in 0..n {
                let mut sp = [0.0; 3];
                for s in &mut sp {
                    *s = (self.rng.next_double() * 2.0 - 1.0) * f64::from(0.005_f32);
                }
                let c = [cx, cy, cz];
                let mut q = [0.0; 3];
                let mut v = [0.0; 3];
                for a in 0..3 {
                    if step[a] == 0 {
                        q[a] = c[a] + (self.rng.next_double() - 0.5);
                        v[a] = sp[a];
                    } else {
                        q[a] = c[a] + f64::from(step[a]) * factor;
                    }
                }
                self.add_sculk_charge(q[0], q[1], q[2], v, roll);
            }
        }
    }

    /// VANILLA: TrialSpawnerDetectionParticle.Provider (scale 1.5).
    pub fn add_trial_detect(&mut self, x: f64, y: f64, z: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("trial_spawner_detection").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Smoke, x, y, z, 0.0, 0.0, 0.0, seed);
        p.friction = 0.96;
        p.gravity = -0.1;
        p.speed_up_blocked = true;
        p.xd *= 0.0;
        p.yd *= 0.9;
        p.zd *= 0.0;
        p.quad_size *= 0.75 * 1.5;
        let r = 0.5 + p.rng.next_float() * 0.5;
        p.lifetime = ((8.0 / r * 1.5) as i32).max(1);
        p.has_physics = true;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: LevelEventHandler 3007 → ShriekParticle.Provider (delay = i*5).
    pub fn add_shriek(&mut self, x: f64, y: f64, z: f64, delay: i32) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("shriek").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Shriek, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.quad_size = 0.85;
        p.delay = delay;
        p.lifetime = 30;
        p.gravity = 0.0;
        p.xd = 0.0;
        p.yd = 0.1;
        p.zd = 0.0;
        self.add(p);
    }

    /// VANILLA: TrialSpawner#addSpawnParticles (20× smoke + flame).
    fn trial_spawn(&mut self, pos: [i32; 3], flame: &str) {
        for _ in 0..20 {
            let x = f64::from(pos[0]) + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
            let y = f64::from(pos[1]) + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
            let z = f64::from(pos[2]) + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
            self.add_smoke(x, y, z, 0.0, 0.0, 0.0);
            self.add_flame(flame, x, y, z);
        }
    }

    /// VANILLA: TrialSpawner#addEjectItemParticles (20× small_flame + smoke).
    fn trial_eject(&mut self, pos: [i32; 3]) {
        for _ in 0..20 {
            let x = f64::from(pos[0]) + 0.4 + self.rng.next_double() * 0.2;
            let y = f64::from(pos[1]) + 0.4 + self.rng.next_double() * 0.2;
            let z = f64::from(pos[2]) + 0.4 + self.rng.next_double() * 0.2;
            let (xa, ya, za) = (
                self.rng.next_gaussian() * 0.02,
                self.rng.next_gaussian() * 0.02,
                self.rng.next_gaussian() * 0.02,
            );
            self.add_flame_v("small_flame", x, y, z, [xa, ya, za * 0.25]);
            self.add_smoke(x, y, z, xa, ya, za);
        }
    }

    /// VANILLA: ParticleUtils#spawnSmashAttackParticles (dust_pillar по блоку под ударом).
    pub fn smash(
        &mut self,
        pos: BlockPos,
        state: BlockState,
        count: i32,
        look: &mut dyn FnMut(BlockState, BlockPos) -> Option<TerrainLook>,
    ) {
        if !spawns_terrain_particles(state) {
            return;
        }
        let Some(l) = look(state, pos) else {
            return;
        };
        let c = [
            f64::from(pos.x) + 0.5,
            f64::from(pos.y) + 1.0,
            f64::from(pos.z) + 0.5,
        ];
        let n1 = (count as f32 / 3.0).ceil() as i32;
        let n2 = (count as f32 / 1.5).ceil() as i32;
        let f02 = f64::from(0.2_f32);
        let f005 = f64::from(0.05_f32);
        for i in 0..(n1 + n2) {
            let (x, z, k) = if i < n1 {
                (
                    c[0] + self.rng.next_gaussian() / 2.0,
                    c[2] + self.rng.next_gaussian() / 2.0,
                    f02,
                )
            } else {
                let j = f64::from(i - n1);
                (
                    c[0] + 3.5 * j.cos() + self.rng.next_gaussian() / 2.0,
                    c[2] + 3.5 * j.sin() + self.rng.next_gaussian() / 2.0,
                    f005,
                )
            };
            let (xd, yd, zd) = (
                self.rng.next_gaussian() * k,
                self.rng.next_gaussian() * k,
                self.rng.next_gaussian() * k,
            );
            self.add_terrain(x, c[1], z, xd, yd, zd, l);
            // DustPillarProvider: setParticleSpeed + lifetime.
            if let Some(p) = self.list.back_mut() {
                p.xd = p.rng.next_gaussian() / 30.0;
                p.yd = yd + p.rng.next_gaussian() / 2.0;
                p.zd = p.rng.next_gaussian() / 30.0;
                p.lifetime = p.rng.next_int_bound(20) + 20;
            }
        }
    }

    /// VANILLA: LevelEventHandler case 3006 (count == 0): sculk_charge_pop.
    pub fn sculk_pop(&mut self, pos: [i32; 3], solid: bool) {
        let (n, spread) = if solid {
            (40, 0.45_f32)
        } else {
            (20, 0.25_f32)
        };
        for _ in 0..n {
            let v = [
                2.0 * self.rng.next_float() - 1.0,
                2.0 * self.rng.next_float() - 1.0,
                2.0 * self.rng.next_float() - 1.0,
            ];
            let sp = f64::from(spread);
            self.add_sculk_pop(
                f64::from(pos[0]) + 0.5 + f64::from(v[0]) * sp,
                f64::from(pos[1]) + 0.5 + f64::from(v[1]) * sp,
                f64::from(pos[2]) + 0.5 + f64::from(v[2]) * sp,
                [
                    f64::from(v[0] * 0.07),
                    f64::from(v[1] * 0.07),
                    f64::from(v[2] * 0.07),
                ],
            );
        }
    }

    /// VANILLA: SuspendedTownParticle.EggCrackProvider (без физики, friction 0.99).
    #[allow(clippy::too_many_arguments)]
    pub fn add_egg_crack(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("egg_crack").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Smoke, x, y, z, xa, ya, za, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        let br = p.rng.next_float() * 0.1 + 0.2;
        let _ = br; // setColor(1,1,1) из провайдера перекрывает серый
        p.rgb = [1.0; 3];
        p.set_size(0.02, 0.02);
        p.quad_size *= p.rng.next_float() * 0.6 + 0.5;
        p.xd *= f64::from(0.02_f32);
        p.yd *= f64::from(0.02_f32);
        p.zd *= f64::from(0.02_f32);
        p.lifetime = (20.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        p.has_physics = false;
        p.friction = 0.99;
        p.gravity = 0.0;
        self.add(p);
    }

    /// VANILLA: LevelEventHandler#singleBlockTeleportationParticles (portal вдоль линии to→pos).
    fn block_teleport(&mut self, pos: [i32; 3], data: i32, rad: [i32; 3]) {
        let to = [
            f64::from(pos[0] + ((data >> 16) & 0xFF) - rad[0]),
            f64::from(pos[1] + ((data >> 8) & 0xFF) - rad[1]),
            f64::from(pos[2] + (data & 0xFF) - rad[2]),
        ];
        let from = [f64::from(pos[0]), f64::from(pos[1]), f64::from(pos[2])];
        for _ in 0..128 {
            let d = self.rng.next_double();
            let xa = f64::from((self.rng.next_float() - 0.5) * 0.2);
            let ya = f64::from((self.rng.next_float() - 0.5) * 0.2);
            let za = f64::from((self.rng.next_float() - 0.5) * 0.2);
            let x = to[0] + (from[0] - to[0]) * d + (self.rng.next_double() - 0.5) + 0.5;
            let y = to[1] + (from[1] - to[1]) * d + self.rng.next_double() - 0.5;
            let z = to[2] + (from[2] - to[2]) * d + (self.rng.next_double() - 0.5) + 0.5;
            self.add_portal(x, y, z, xa, ya, za);
        }
    }

    /// VANILLA: SuspendedParticle.UnderwaterProvider (SuspendedParticle#<init>, без физики).
    pub fn add_underwater(&mut self, x: f64, y: f64, z: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("underwater").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Smoke, x, y - 0.125, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.set_size(0.01, 0.01);
        p.quad_size *= p.rng.next_float() * 0.6 + 0.2;
        p.lifetime = (16.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        p.has_physics = false;
        p.friction = 1.0;
        p.gravity = 0.0;
        self.add(p);
    }

    /// VANILLA: LavaParticle#<init>.
    pub fn add_lava(&mut self, x: f64, y: f64, z: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("lava").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Lava, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.gravity = 0.75;
        p.friction = 0.999;
        p.xd *= f64::from(0.8_f32);
        p.zd *= f64::from(0.8_f32);
        p.yd = f64::from(p.rng.next_float() * 0.4 + 0.05);
        p.quad_size *= p.rng.next_float() * 2.0 + 0.2;
        p.lifetime = (16.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        self.add(p);
    }

    /// VANILLA: ClientLevel#animateTick + doAnimateTick (667 × радиусы 16 и 32) для факелов и лавы.
    /// VANILLA-UNSURE: остальные блоки/жидкости (redstone torch, fire, капли, ambient particles) — позже.
    pub fn animate_tick(&mut self, level: &Level, cx: i32, cy: i32, cz: i32) {
        for _ in 0..667 {
            for r in [16, 32] {
                let d = |e: &mut Self| e.rng.next_int_bound(r) - e.rng.next_int_bound(r);
                let (x, y, z) = (cx + d(self), cy + d(self), cz + d(self));
                let st = level.chunks.block(x, y, z);
                let name = st.name();
                let (fx, fy, fz) = (f64::from(x), f64::from(y), f64::from(z));
                let flame = match name {
                    "torch" | "wall_torch" => Some("flame"),
                    "soul_torch" | "soul_wall_torch" => Some("soul_fire_flame"),
                    "copper_torch" | "copper_wall_torch" => Some("copper_fire_flame"),
                    _ => None,
                };
                if let Some(flame) = flame {
                    // VANILLA: TorchBlock / WallTorchBlock#animateTick
                    let (mut px, py, mut pz) = (fx + 0.5, fy + 0.7, fz + 0.5);
                    let wall = name.ends_with("wall_torch");
                    let (mut ox, mut oz) = (0.0, 0.0);
                    if wall {
                        let (sx, sz) = match st.property("facing") {
                            Some("north") => (0.0, -1.0),
                            Some("south") => (0.0, 1.0),
                            Some("west") => (-1.0, 0.0),
                            Some("east") => (1.0, 0.0),
                            _ => (0.0, 0.0),
                        };
                        // opposite = facing.getOpposite()
                        (ox, oz) = (-sx * 0.27, -sz * 0.27);
                    }
                    px += ox;
                    pz += oz;
                    let py = if wall { py + 0.22 } else { py };
                    self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
                    self.add_flame(flame, px, py, pz);
                } else if name == "pointed_dripstone" {
                    // VANILLA: PointedDripstoneBlock#animateTick
                    if free_hanging_stalactite(st) {
                        let rv = self.rng.next_float();
                        if rv <= 0.12 {
                            if let Some((fluid, above)) = fluid_above_stalactite(level, x, y, z) {
                                if rv < 0.02 || fluid != DripFluid::Empty {
                                    self.drip_from_stalactite(level, x, y, z, fluid, above);
                                }
                            }
                        }
                    }
                } else if name == "lava" {
                    // VANILLA: LavaFluid#animateTick (lava_pop + звуки lava.pop / lava.ambient)
                    let above = level.chunks.block(x, y + 1, z);
                    if above.is_air() {
                        if self.rng.next_int_bound(100) == 0 {
                            let (lx, lz) =
                                (fx + self.rng.next_double(), fz + self.rng.next_double());
                            self.add_lava(lx, fy + 1.0, lz);
                            let vol = 0.2 + self.rng.next_float() * 0.2;
                            let pit = 0.9 + self.rng.next_float() * 0.15;
                            crate::audio::play_at(
                                "block.lava.pop",
                                crate::audio::Cat::Ambient,
                                lx,
                                fy + 1.0,
                                lz,
                                vol,
                                pit,
                            );
                        }
                        if self.rng.next_int_bound(200) == 0 {
                            let vol = 0.2 + self.rng.next_float() * 0.2;
                            let pit = 0.9 + self.rng.next_float() * 0.15;
                            crate::audio::play_at(
                                "block.lava.ambient",
                                crate::audio::Cat::Ambient,
                                fx,
                                fy,
                                fz,
                                vol,
                                pit,
                            );
                        }
                    }
                } else if (name == "redstone_torch" || name == "redstone_wall_torch")
                    && st.property("lit") == Some("true")
                {
                    // VANILLA: RedstoneTorchBlock / RedstoneWallTorchBlock#animateTick
                    let (mut ox, mut oz, mut oy) = (0.0, 0.0, 0.0);
                    if name == "redstone_wall_torch" {
                        let (sx, sz) = match st.property("facing") {
                            Some("north") => (0.0, -1.0),
                            Some("south") => (0.0, 1.0),
                            Some("west") => (-1.0, 0.0),
                            Some("east") => (1.0, 0.0),
                            _ => (0.0, 0.0),
                        };
                        (ox, oz, oy) = (-sx * 0.27, -sz * 0.27, 0.22);
                    }
                    let r = &mut self.rng;
                    let px = fx + 0.5 + ox + (r.next_double() - 0.5) * 0.2;
                    let py = fy + 0.7 + oy + (r.next_double() - 0.5) * 0.2;
                    let pz = fz + 0.5 + oz + (r.next_double() - 0.5) * 0.2;
                    self.add_ash(
                        "dust",
                        1.0,
                        Some([1.0, 0.0, 0.0]),
                        px,
                        py,
                        pz,
                        0.0,
                        0.0,
                        0.0,
                    );
                } else if name == "redstone_wire" {
                    self.wire_tick(st, x, y, z);
                } else if name == "repeater" && st.property("powered") == Some("true") {
                    // VANILLA: RepeaterBlock#animateTick
                    let (sx, sz) = match st.property("facing") {
                        Some("north") => (0.0, -1.0),
                        Some("south") => (0.0, 1.0),
                        Some("west") => (-1.0, 0.0),
                        Some("east") => (1.0, 0.0),
                        _ => (0.0, 0.0),
                    };
                    let r = &mut self.rng;
                    let px = fx + 0.5 + (r.next_double() - 0.5) * 0.2;
                    let py = fy + 0.4 + (r.next_double() - 0.5) * 0.2;
                    let pz = fz + 0.5 + (r.next_double() - 0.5) * 0.2;
                    let mut offset = -5.0_f32;
                    if r.next_boolean() {
                        let delay: i32 = st
                            .property("delay")
                            .and_then(|d| d.parse().ok())
                            .unwrap_or(1);
                        offset = (delay * 2 - 1) as f32;
                    }
                    let offset = f64::from(offset / 16.0);
                    self.add_ash(
                        "dust",
                        1.0,
                        Some([1.0, 0.0, 0.0]),
                        px + offset * sx,
                        py,
                        pz + offset * sz,
                        0.0,
                        0.0,
                        0.0,
                    );
                } else if (name == "redstone_ore" || name == "deepslate_redstone_ore")
                    && st.property("lit") == Some("true")
                {
                    // VANILLA: RedStoneOreBlock#animateTick → spawnParticles
                    self.ore_particles(level, x, y, z);
                } else if name == "end_rod" {
                    // VANILLA: EndRodBlock#animateTick.
                    let r = &mut self.rng;
                    let px = fx + 0.55 - f64::from(r.next_float() * 0.1);
                    let py = fy + 0.55 - f64::from(r.next_float() * 0.1);
                    let pz = fz + 0.55 - f64::from(r.next_float() * 0.1);
                    let o = f64::from(0.4_f32 - (r.next_float() + r.next_float()) * 0.4);
                    if r.next_int_bound(5) == 0 {
                        let (sx, sy, sz) = match st.property("facing") {
                            Some("north") => (0.0, 0.0, -1.0),
                            Some("south") => (0.0, 0.0, 1.0),
                            Some("west") => (-1.0, 0.0, 0.0),
                            Some("east") => (1.0, 0.0, 0.0),
                            Some("down") => (0.0, -1.0, 0.0),
                            _ => (0.0, 1.0, 0.0),
                        };
                        let g = |e: &mut Self| e.rng.next_gaussian() * 0.005;
                        let (a, b, c) = (g(self), g(self), g(self));
                        self.add_end_rod(px + sx * o, py + sy * o, pz + sz * o, a, b, c);
                    }
                } else if name == "fire" || name == "soul_fire" {
                    self.fire_tick(level, name == "soul_fire", x, y, z);
                } else if (name == "campfire" || name == "soul_campfire")
                    && st.property("lit") == Some("true")
                {
                    // VANILLA: CampfireBlock#animateTick (звук 1/10, искры 1/5).
                    if self.rng.next_int_bound(10) == 0 {
                        let vol = 0.5 + self.rng.next_float();
                        let pit = self.rng.next_float() * 0.7 + 0.6;
                        crate::audio::play_at(
                            "block.campfire.crackle",
                            crate::audio::Cat::Blocks,
                            fx + 0.5,
                            fy + 0.5,
                            fz + 0.5,
                            vol,
                            pit,
                        );
                    }
                    if self.rng.next_int_bound(5) == 0 {
                        for _ in 0..(self.rng.next_int_bound(1) + 1) {
                            self.add_lava(fx + 0.5, fy + 0.5, fz + 0.5);
                        }
                    }
                } else if name == "ender_chest" {
                    // VANILLA: EnderChestBlock#animateTick (3 частицы portal вокруг сундука).
                    for _ in 0..3 {
                        let flip_x = f64::from(self.rng.next_int_bound(2) * 2 - 1);
                        let flip_z = f64::from(self.rng.next_int_bound(2) * 2 - 1);
                        let px = fx + 0.5 + 0.25 * flip_x;
                        let py = fy + f64::from(self.rng.next_float());
                        let pz = fz + 0.5 + 0.25 * flip_z;
                        let xa = f64::from(self.rng.next_float()) * flip_x;
                        let ya = (f64::from(self.rng.next_float()) - 0.5) * 0.125;
                        let za = f64::from(self.rng.next_float()) * flip_z;
                        self.add_portal(px, py, pz, xa, ya, za);
                    }
                } else if name == "nether_portal" {
                    // VANILLA: NetherPortalBlock#animateTick (звук 1/100 + 4 частицы portal).
                    if self.rng.next_int_bound(100) == 0 {
                        let pit = self.rng.next_float() * 0.4 + 0.8;
                        crate::audio::play_at(
                            "block.portal.ambient",
                            crate::audio::Cat::Blocks,
                            fx + 0.5,
                            fy + 0.5,
                            fz + 0.5,
                            0.5,
                            pit,
                        );
                    }
                    let x_axis = level.chunks.block(x - 1, y, z).name() == "nether_portal"
                        || level.chunks.block(x + 1, y, z).name() == "nether_portal";
                    for _ in 0..4 {
                        let mut px = fx + self.rng.next_double();
                        let py = fy + self.rng.next_double();
                        let mut pz = fz + self.rng.next_double();
                        let mut xa = (f64::from(self.rng.next_float()) - 0.5) * 0.5;
                        let ya = (f64::from(self.rng.next_float()) - 0.5) * 0.5;
                        let mut za = (f64::from(self.rng.next_float()) - 0.5) * 0.5;
                        let flip = f64::from(self.rng.next_int_bound(2) * 2 - 1);
                        if x_axis {
                            pz = fz + 0.5 + 0.25 * flip;
                            za = f64::from(self.rng.next_float() * 2.0) * flip;
                        } else {
                            px = fx + 0.5 + 0.25 * flip;
                            xa = f64::from(self.rng.next_float() * 2.0) * flip;
                        }
                        self.add_portal(px, py, pz, xa, ya, za);
                    }
                } else if matches!(name, "furnace" | "blast_furnace" | "smoker")
                    && st.property("lit") == Some("true")
                {
                    // VANILLA: FurnaceBlock / BlastFurnaceBlock / SmokerBlock#animateTick.
                    if self.rng.next_double() < 0.1 {
                        let ev = match name {
                            "blast_furnace" => "block.blastfurnace.fire_crackle",
                            "smoker" => "block.smoker.smoke",
                            _ => "block.furnace.fire_crackle",
                        };
                        crate::audio::play_at(
                            ev,
                            crate::audio::Cat::Blocks,
                            fx + 0.5,
                            fy,
                            fz + 0.5,
                            1.0,
                            1.0,
                        );
                    }
                    if name == "smoker" {
                        self.add_smoke(fx + 0.5, fy + 1.1, fz + 0.5, 0.0, 0.0, 0.0);
                    } else {
                        let ss = self.rng.next_double() * 0.6 - 0.3;
                        let (dx, dz) = match st.property("facing") {
                            Some("west") => (-0.52, ss),
                            Some("east") => (0.52, ss),
                            Some("north") => (ss, -0.52),
                            Some("south") => (ss, 0.52),
                            _ => (ss, 0.52),
                        };
                        let dy_max = if name == "blast_furnace" { 9.0 } else { 6.0 };
                        let dy = self.rng.next_double() * dy_max / 16.0;
                        let (px, py, pz) = (fx + 0.5 + dx, fy + dy, fz + 0.5 + dz);
                        self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
                        if name == "furnace" {
                            self.add_flame("flame", px, py, pz);
                        }
                    }
                } else if name == "brewing_stand" {
                    // VANILLA: BrewingStandBlock#animateTick.
                    let px = fx + 0.4 + f64::from(self.rng.next_float()) * 0.2;
                    let py = fy + 0.7 + f64::from(self.rng.next_float()) * 0.3;
                    let pz = fz + 0.4 + f64::from(self.rng.next_float()) * 0.2;
                    self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
                } else if name == "end_portal" {
                    // VANILLA: EndPortalBlock#animateTick.
                    let px = fx + self.rng.next_double();
                    let pz = fz + self.rng.next_double();
                    self.add_smoke(px, fy + 0.8, pz, 0.0, 0.0, 0.0);
                } else if name == "end_gateway" {
                    // VANILLA: EndGatewayBlock#animateTick (1 частица portal; без BE getParticleAmount).
                    let mut px = fx + self.rng.next_double();
                    let py = fy + self.rng.next_double();
                    let mut pz = fz + self.rng.next_double();
                    let mut xa = (self.rng.next_double() - 0.5) * 0.5;
                    let ya = (self.rng.next_double() - 0.5) * 0.5;
                    let mut za = (self.rng.next_double() - 0.5) * 0.5;
                    let flip = f64::from(self.rng.next_int_bound(2) * 2 - 1);
                    if self.rng.next_boolean() {
                        pz = fz + 0.5 + 0.25 * flip;
                        za = f64::from(self.rng.next_float() * 2.0) * flip;
                    } else {
                        px = fx + 0.5 + 0.25 * flip;
                        xa = f64::from(self.rng.next_float() * 2.0) * flip;
                    }
                    self.add_portal(px, py, pz, xa, ya, za);
                } else if name == "bubble_column" {
                    // VANILLA: BubbleColumnBlock#animateTick.
                    if st.property("drag") == Some("true") {
                        self.add_named_bubble(
                            "current_down",
                            fx + 0.5,
                            fy + 0.8,
                            fz,
                            0.0,
                            0.0,
                            0.0,
                        );
                        if self.rng.next_int_bound(200) == 0 {
                            let vol = 0.2 + self.rng.next_float() * 0.2;
                            let pit = 0.9 + self.rng.next_float() * 0.15;
                            crate::audio::play_at(
                                "block.bubble_column.whirlpool_ambient",
                                crate::audio::Cat::Blocks,
                                fx,
                                fy,
                                fz,
                                vol,
                                pit,
                            );
                        }
                    } else {
                        self.add_named_bubble(
                            "bubble_column_up",
                            fx + 0.5,
                            fy,
                            fz + 0.5,
                            0.0,
                            0.04,
                            0.0,
                        );
                        let (rx, ry, rz) = (
                            f64::from(self.rng.next_float()),
                            f64::from(self.rng.next_float()),
                            f64::from(self.rng.next_float()),
                        );
                        self.add_named_bubble(
                            "bubble_column_up",
                            fx + rx,
                            fy + ry,
                            fz + rz,
                            0.0,
                            0.04,
                            0.0,
                        );
                        if self.rng.next_int_bound(200) == 0 {
                            let vol = 0.2 + self.rng.next_float() * 0.2;
                            let pit = 0.9 + self.rng.next_float() * 0.15;
                            crate::audio::play_at(
                                "block.bubble_column.upwards_ambient",
                                crate::audio::Cat::Blocks,
                                fx,
                                fy,
                                fz,
                                vol,
                                pit,
                            );
                        }
                    }
                }
                // VANILLA: ClientLevel#doAnimateTick — капли жидкости
                if let Some(fl) = blockinfo::fluid(st) {
                    if !fl.lava {
                        // VANILLA: WaterFluid#animateTick (течение: water.ambient 1/64; иначе underwater 1/10).
                        if !fl.source && !fl.falling {
                            if self.rng.next_int_bound(64) == 0 {
                                let vol = self.rng.next_float() * 0.25 + 0.75;
                                let pit = self.rng.next_float() + 0.5;
                                crate::audio::play_at(
                                    "block.water.ambient",
                                    crate::audio::Cat::Ambient,
                                    fx + 0.5,
                                    fy + 0.5,
                                    fz + 0.5,
                                    vol,
                                    pit,
                                );
                            }
                        } else if self.rng.next_int_bound(10) == 0 {
                            let (ux, uy, uz) = (
                                fx + self.rng.next_double(),
                                fy + self.rng.next_double(),
                                fz + self.rng.next_double(),
                            );
                            self.add_underwater(ux, uy, uz);
                        }
                    }
                    if self.rng.next_int_bound(10) == 0 {
                        let stage = if fl.lava {
                            Stage::HangLava
                        } else {
                            Stage::HangWater
                        };
                        // VANILLA-UNSURE: isFaceSturdy(DOWN) ≈ FLAG_CAN_OCCLUDE
                        let solid_bottom = st.props().is_some_and(|p| p.has(FLAG_CAN_OCCLUDE));
                        self.try_spawn_drip(level, x, y - 1, z, stage, solid_bottom);
                    }
                }
            }
        }
    }

    /// VANILLA: PointedDripstoneBlock#spawnDripParticle (данные: фактический блок над корнем).
    fn drip_from_stalactite(
        &mut self,
        level: &Level,
        x: i32,
        y: i32,
        z: i32,
        fluid: DripFluid,
        above: [i32; 3],
    ) {
        let st = level.chunks.block(x, y, z);
        let off = blockinfo::block_offset(st, x, z);
        let (px, py, pz) = (
            f64::from(x) + 0.5 + f64::from(off[0]),
            // STALACTITE_DRIP_START_PIXEL (5/16) − 1/16
            f64::from(y) + 0.3125 - 0.0625,
            f64::from(z) + 0.5 + f64::from(off[2]),
        );
        let lava = match fluid {
            DripFluid::Lava => true,
            DripFluid::Water => false,
            // VANILLA-UNSURE: DEFAULT_DRIPSTONE_PARTICLE — в Нижнем мире lava, иначе water
            DripFluid::Empty => level
                .dimension
                .as_ref()
                .is_some_and(|d| d.has_ceiling && !d.has_skylight),
        };
        let _ = above;
        let stage = if lava {
            Stage::StoneHangLava
        } else {
            Stage::StoneHangWater
        };
        self.add_drip(stage, px, py, pz);
    }

    /// VANILLA: LevelEventHandler 1504 → PointedDripstoneBlock#spawnDripParticle.
    pub fn stalactite_drip(&mut self, level: &Level, pos: [i32; 3]) {
        if let Some((fluid, above)) = fluid_above_stalactite(level, pos[0], pos[1], pos[2]) {
            self.drip_from_stalactite(level, pos[0], pos[1], pos[2], fluid, above);
        }
    }

    /// VANILLA: RedStoneWireBlock#COLORS.
    pub fn wire_color(power: i32) -> [f32; 3] {
        let p = power as f32 / 15.0;
        let red = p * 0.6 + if p > 0.0 { 0.4 } else { 0.3 };
        let green = (p * p * 0.7 - 0.5).clamp(0.0, 1.0);
        let blue = (p * p * 0.6 - 0.7).clamp(0.0, 1.0);
        [red, green, blue]
    }

    /// VANILLA: RedStoneWireBlock#spawnParticlesAlongLine.
    #[allow(clippy::too_many_arguments)]
    fn wire_line(
        &mut self,
        (x, y, z): (i32, i32, i32),
        color: [f32; 3],
        side: [f64; 3],
        along: [f64; 3],
        from: f32,
        to: f32,
    ) {
        let span = to - from;
        if self.rng.next_float() >= 0.2 * span {
            return;
        }
        let side_of_block = f64::from(0.4375_f32);
        let t = f64::from(from + span * self.rng.next_float());
        let px = f64::from(x) + 0.5 + side_of_block * side[0] + t * along[0];
        let py = f64::from(y) + 0.5 + side_of_block * side[1] + t * along[1];
        let pz = f64::from(z) + 0.5 + side_of_block * side[2] + t * along[2];
        self.add_ash("dust", 1.0, Some(color), px, py, pz, 0.0, 0.0, 0.0);
    }

    /// VANILLA: RedStoneWireBlock#animateTick.
    fn wire_tick(&mut self, st: BlockState, x: i32, y: i32, z: i32) {
        let power: i32 = st
            .property("power")
            .and_then(|p| p.parse().ok())
            .unwrap_or(0);
        if power == 0 {
            return;
        }
        let color = Self::wire_color(power);
        let pos = (x, y, z);
        // Plane.HORIZONTAL: north, east, south, west
        for (prop, dir) in [
            ("north", [0.0, 0.0, -1.0]),
            ("east", [1.0, 0.0, 0.0]),
            ("south", [0.0, 0.0, 1.0]),
            ("west", [-1.0, 0.0, 0.0]),
        ] {
            let down = [0.0, -1.0, 0.0];
            match st.property(prop) {
                Some("up") => {
                    self.wire_line(pos, color, dir, [0.0, 1.0, 0.0], -0.5, 0.5);
                    self.wire_line(pos, color, down, dir, 0.0, 0.5);
                }
                Some("side") => self.wire_line(pos, color, down, dir, 0.0, 0.5),
                _ => self.wire_line(pos, color, down, dir, 0.0, 0.3),
            }
        }
    }

    /// VANILLA: RedStoneOreBlock#spawnParticles.
    fn ore_particles(&mut self, level: &Level, x: i32, y: i32, z: i32) {
        let offset = 0.5625;
        // Direction.values(): down, up, north, south, west, east
        for (axis, step) in [(1, -1), (1, 1), (2, -1), (2, 1), (0, -1), (0, 1)] {
            let mut d = [0, 0, 0];
            d[axis] = step;
            let rel = level.chunks.block(x + d[0], y + d[1], z + d[2]);
            // VANILLA-UNSURE: isSolidRender ≈ FLAG_CAN_OCCLUDE
            if rel.props().is_some_and(|p| p.has(FLAG_CAN_OCCLUDE)) {
                continue;
            }
            let mut o = [0.0_f64; 3];
            for (i, v) in o.iter_mut().enumerate() {
                *v = if i == axis {
                    0.5 + offset * f64::from(step)
                } else {
                    f64::from(self.rng.next_float())
                };
            }
            self.add_ash(
                "dust",
                1.0,
                Some([1.0, 0.0, 0.0]),
                f64::from(x) + o[0],
                f64::from(y) + o[1],
                f64::from(z) + o[2],
                0.0,
                0.0,
                0.0,
            );
        }
    }

    /// VANILLA: LevelEventHandler#shootParticles (событие 2000, дым раздатчика/выстрела).
    pub fn shoot_smoke(&mut self, data: i32, px: i32, py: i32, pz: i32) {
        // Direction.from3DDataValue: 0 down,1 up,2 north,3 south,4 west,5 east
        let (nx, ny, nz) = match data.rem_euclid(6) {
            0 => (0.0, -1.0, 0.0),
            1 => (0.0, 1.0, 0.0),
            2 => (0.0, 0.0, -1.0),
            3 => (0.0, 0.0, 1.0),
            4 => (-1.0, 0.0, 0.0),
            _ => (1.0, 0.0, 0.0),
        };
        for _ in 0..10 {
            let pow = self.rng.next_double() * 0.2 + 0.01;
            let x = f64::from(px)
                + nx * 0.6
                + 0.5
                + nx * 0.01
                + (self.rng.next_double() - 0.5) * nz * 0.5;
            let y = f64::from(py)
                + ny * 0.6
                + 0.5
                + ny * 0.01
                + (self.rng.next_double() - 0.5) * ny * 0.5;
            let z = f64::from(pz)
                + nz * 0.6
                + 0.5
                + nz * 0.01
                + (self.rng.next_double() - 0.5) * nx * 0.5;
            let vx = nx * pow + self.rng.next_gaussian() * 0.01;
            let vy = ny * pow + self.rng.next_gaussian() * 0.01;
            let vz = nz * pow + self.rng.next_gaussian() * 0.01;
            self.add_smoke(x, y, z, vx, vy, vz);
        }
    }

    /// VANILLA: NoteParticle (color = note/24).
    pub fn add_note(&mut self, x: f64, y: f64, z: f64, color: f64) {
        let Some(set) = self.sets.get("note").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Smoke, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[0];
        p.friction = 0.66;
        p.speed_up_blocked = true;
        p.gravity = 0.0;
        p.xd = 0.0;
        p.yd = 0.0;
        p.zd = 0.0;
        p.yd += 0.2;
        let c = color as f32;
        let tau = std::f32::consts::TAU;
        let ch = |o: f32| ((c + o) * tau).sin().mul_add(0.65, 0.35).max(0.0);
        p.rgb = [ch(0.0), ch(0.333_333_34), ch(0.666_666_7)];
        p.quad_size *= 1.5;
        p.lifetime = 6;
        p.set = Some(set);
        self.add(p);
    }

    /// VANILLA: BaseFireBlock#animateTick (без звука); canBurn: fire = igniteOdds>0, soul_fire = true.
    fn fire_tick(&mut self, level: &Level, soul: bool, x: i32, y: i32, z: i32) {
        let can_burn = |st: BlockState| soul || FLAMMABLE.contains(&st.name());
        let below = level.chunks.block(x, y - 1, z);
        // VANILLA-UNSURE: isFaceSturdy(UP) ≈ FLAG_CAN_OCCLUDE
        let sturdy = below.props().is_some_and(|p| p.has(FLAG_CAN_OCCLUDE));
        let (fx, fy, fz) = (f64::from(x), f64::from(y), f64::from(z));
        if !can_burn(below) && !sturdy {
            let tenth = f64::from(0.1_f32);
            // west, east, north, south, above
            let sides: [((i32, i32, i32), u8); 5] = [
                ((-1, 0, 0), 0),
                ((1, 0, 0), 1),
                ((0, 0, -1), 2),
                ((0, 0, 1), 3),
                ((0, 1, 0), 4),
            ];
            for ((dx, dy, dz), side) in sides {
                if !can_burn(level.chunks.block(x + dx, y + dy, z + dz)) {
                    continue;
                }
                for _ in 0..2 {
                    let r = &mut self.rng;
                    let (px, py, pz) = match side {
                        0 => (
                            fx + r.next_double() * tenth,
                            fy + r.next_double(),
                            fz + r.next_double(),
                        ),
                        1 => (
                            fx + 1.0 - r.next_double() * tenth,
                            fy + r.next_double(),
                            fz + r.next_double(),
                        ),
                        2 => (
                            fx + r.next_double(),
                            fy + r.next_double(),
                            fz + r.next_double() * tenth,
                        ),
                        3 => (
                            fx + r.next_double(),
                            fy + r.next_double(),
                            fz + 1.0 - r.next_double() * tenth,
                        ),
                        _ => (
                            fx + r.next_double(),
                            fy + 1.0 - r.next_double() * tenth,
                            fz + r.next_double(),
                        ),
                    };
                    self.add_ash("large_smoke", 2.5, None, px, py, pz, 0.0, 0.0, 0.0);
                }
            }
        } else {
            for _ in 0..3 {
                let r = &mut self.rng;
                let (px, py, pz) = (
                    fx + r.next_double(),
                    fy + r.next_double() * 0.5 + 0.5,
                    fz + r.next_double(),
                );
                self.add_ash("large_smoke", 2.5, None, px, py, pz, 0.0, 0.0, 0.0);
            }
        }
    }

    /// VANILLA: ClientLevel#trySpawnDripParticles (pos = клетка под жидкостью).
    fn try_spawn_drip(
        &mut self,
        level: &Level,
        x: i32,
        y: i32,
        z: i32,
        stage: Stage,
        top_solid: bool,
    ) {
        let st = level.chunks.block(x, y, z);
        if blockinfo::fluid(st).is_some() {
            return;
        }
        let boxes = collision_boxes(st);
        let top = boxes.iter().map(|b| b[4]).fold(0.0, f64::max);
        let (fx, fy, fz) = (f64::from(x), f64::from(y), f64::from(z));
        let spawn = |e: &mut Self, x1: f64, x2: f64, z1: f64, z2: f64, yy: f64| {
            let (a, b) = (e.rng.next_double(), e.rng.next_double());
            e.add_drip(stage, x1 + (x2 - x1) * a, yy, z1 + (z2 - z1) * b);
        };
        if top < 1.0 {
            if top_solid {
                spawn(self, fx, fx + 1.0, fz, fz + 1.0, fy + 1.0 - 0.05);
            }
        } else if !(st.name().ends_with("glass") || st.name() == "barrier") {
            // VANILLA: BlockTags.IMPERMEABLE ≈ стекло/barrier
            let bottom = boxes.iter().map(|b| b[1]).fold(f64::MAX, f64::min);
            let (x1, x2, z1, z2) = boxes
                .iter()
                .fold((f64::MAX, f64::MIN, f64::MAX, f64::MIN), |a, b| {
                    (a.0.min(b[0]), a.1.max(b[3]), a.2.min(b[2]), a.3.max(b[5]))
                });
            let bottom = if boxes.is_empty() { 0.0 } else { bottom };
            let (x1, x2, z1, z2) = if boxes.is_empty() {
                (0.0, 0.0, 0.0, 0.0)
            } else {
                (x1, x2, z1, z2)
            };
            if bottom > 0.0 {
                spawn(self, fx + x1, fx + x2, fz + z1, fz + z2, fy + bottom - 0.05);
            } else {
                let below = level.chunks.block(x, y - 1, z);
                let btop = collision_boxes(below)
                    .iter()
                    .map(|b| b[4])
                    .fold(0.0, f64::max);
                if btop < 1.0 && blockinfo::fluid(below).is_none() {
                    spawn(self, fx + x1, fx + x2, fz + z1, fz + z2, fy - 0.05);
                }
            }
        }
    }

    /// VANILLA: Entity#doWaterSplashEffect — пузырьки и брызги при входе в воду.
    /// `width` — ширина хитбокса, `vel` — deltaMovement.
    pub fn water_splash_effect(&mut self, pos: [f64; 3], width: f64, vel: [f64; 3]) {
        let yt = pos[1].floor() + 1.0;
        let n = (1.0 + width as f32 * 20.0).ceil() as i32;
        for _ in 0..n {
            let xo = (self.rng.next_double() * 2.0 - 1.0) * width;
            let zo = (self.rng.next_double() * 2.0 - 1.0) * width;
            let dy = vel[1] - f64::from(self.rng.next_float() * 0.2);
            self.add_bubble(pos[0] + xo, yt, pos[2] + zo, vel[0], dy, vel[2]);
        }
        for _ in 0..n {
            let xo = (self.rng.next_double() * 2.0 - 1.0) * width;
            let zo = (self.rng.next_double() * 2.0 - 1.0) * width;
            self.add_splash(pos[0] + xo, yt, pos[2] + zo, vel[0], vel[1], vel[2]);
        }
    }

    /// VANILLA: BubbleParticle#<init>.
    /// VANILLA: BubbleColumnUpParticle / WaterCurrentDownParticle (упрощённо через Bubble).
    pub fn add_named_bubble(
        &mut self,
        name: &str,
        x: f64,
        y: f64,
        z: f64,
        xa: f64,
        ya: f64,
        za: f64,
    ) {
        if !self.allow() {
            return;
        }
        let Some(set) = self
            .sets
            .get(name)
            .or_else(|| self.sets.get("bubble"))
            .cloned()
        else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Bubble, x, y, z, seed);
        p.sprite = set[0];
        p.set_size(0.02, 0.02);
        p.quad_size =
            0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0 * (p.rng.next_float() * 0.6 + 0.2);
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        p.lifetime = (8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        self.add(p);
    }

    pub fn add_bubble(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("bubble").cloned() else {
            return;
        };
        if set.is_empty() {
            return;
        }
        let seed = self.seed();
        let mut p = Particle::base(Kind::Bubble, x, y, z, seed);
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0;
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.set_size(0.02, 0.02);
        p.quad_size *= p.rng.next_float() * 0.6 + 0.2;
        let r = &mut p.rng;
        p.xd = xa * f64::from(0.2_f32) + f64::from((r.next_float() * 2.0 - 1.0) * 0.02);
        p.yd = ya * f64::from(0.2_f32) + f64::from((r.next_float() * 2.0 - 1.0) * 0.02);
        p.zd = za * f64::from(0.2_f32) + f64::from((r.next_float() * 2.0 - 1.0) * 0.02);
        p.lifetime = (8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        self.add(p);
    }

    /// VANILLA: DragonBreathParticle#<init> + Provider (power=1 → setPower не меняет скорость).
    pub fn add_dragon_breath(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("dragon_breath").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::DragonBreath, x, y, z, seed);
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0;
        p.friction = 0.96;
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        let r = &mut p.rng;
        p.rgb = [
            0.7176471 + r.next_float() * (0.8745098 - 0.7176471),
            r.next_float() * 0.0,
            0.8235294 + r.next_float() * (0.9764706 - 0.8235294),
        ];
        p.quad_size *= 0.75;
        p.lifetime = (20.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        p.has_physics = false;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: EndRodParticle (gravity 0.0125, friction 0.91, lifetime 60..71, quadSize·0.75).
    pub fn add_end_rod(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("end_rod").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::EndRod, x, y, z, seed);
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0 * 0.75;
        p.friction = 0.91;
        p.gravity = 0.0125;
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        p.lifetime = 60 + p.rng.next_int_bound(12);
        p.has_physics = false;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: DragonLandingPhase#doClientTick (motion = Some) / DragonSittingFlamingPhase#doClientTick (None).
    /// `look` уже нормализован; Vec3#yRot в ваниле результат отбрасывает → вектор не вращается.
    pub fn add_dragon_breath_burst(
        &mut self,
        head: [f64; 3],
        look: [f64; 3],
        motion: Option<[f64; 3]>,
    ) {
        let (hx, hy, hz) = (head[0], head[1] + 0.5, head[2]);
        let k08 = f64::from(0.08_f32);
        for _ in 0..8 {
            let px = hx + self.rng.next_gaussian() / 2.0;
            let py = hy + self.rng.next_gaussian() / 2.0;
            let pz = hz + self.rng.next_gaussian() / 2.0;
            match motion {
                Some(m) => self.add_dragon_breath(
                    px,
                    py,
                    pz,
                    -look[0] * k08 + m[0],
                    -look[1] * f64::from(0.3_f32) + m[1],
                    -look[2] * k08 + m[2],
                ),
                None => {
                    for j in 0..6 {
                        let j = f64::from(j);
                        self.add_dragon_breath(
                            px,
                            py,
                            pz,
                            -look[0] * k08 * j,
                            -look[1] * f64::from(0.6_f32),
                            -look[2] * k08 * j,
                        );
                    }
                }
            }
        }
    }

    /// VANILLA: PortalParticle#<init>.
    pub fn add_portal(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("portal").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Portal, x, y, z, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        p.has_physics = false;
        p.quad_size = 0.1 * (p.rng.next_float() * 0.2 + 0.5);
        let br = p.rng.next_float() * 0.6 + 0.4;
        p.rgb = [br * 0.9, br * 0.3, br];
        p.lifetime = (p.rng.next_float() * 10.0) as i32 + 40;
        self.add(p);
    }

    /// VANILLA: CritParticle#<init> (провайдер CRIT).
    pub fn add_crit(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("crit").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Crit, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[0];
        p.friction = 0.7;
        p.gravity = 0.5;
        p.xd = p.xd * f64::from(0.1_f32) + xa * 0.4;
        p.yd = p.yd * f64::from(0.1_f32) + ya * 0.4;
        p.zd = p.zd * f64::from(0.1_f32) + za * 0.4;
        let col = p.rng.next_float() * 0.3 + 0.6;
        p.rgb = [col; 3];
        p.quad_size *= 0.75;
        p.lifetime = ((6.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.6)) as i32).max(1);
        p.has_physics = false;
        self.add(p);
    }

    /// VANILLA: ThrownEnderpearl#onHit — 32 частиц portal над точкой попадания.
    pub fn pearl_hit(&mut self, pos: [f64; 3]) {
        let seed = self.seed();
        let mut r = LegacyRandom::new(seed);
        for _ in 0..32 {
            let y = pos[1] + r.next_double() * 2.0;
            let (gx, gz) = (r.next_gaussian(), r.next_gaussian());
            self.add_portal(pos[0], y, pos[2], gx, 0.0, gz);
        }
    }

    /// VANILLA: LivingEntity#makePoofParticles — 20 poof вокруг хитбокса.
    pub fn death_poof(&mut self, pos: [f64; 3], w: f64, h: f64) {
        let mut r = LegacyRandom::new(self.seed());
        for _ in 0..20 {
            let (xa, ya, za) = (
                r.next_gaussian() * 0.02,
                r.next_gaussian() * 0.02,
                r.next_gaussian() * 0.02,
            );
            self.add_poof(
                pos[0] + w * (r.next_double() * 2.0 - 1.0) - xa * 10.0,
                pos[1] + h * r.next_double() - ya * 10.0,
                pos[2] + w * (r.next_double() * 2.0 - 1.0) - za * 10.0,
                xa,
                ya,
                za,
            );
        }
    }

    /// VANILLA: SplashParticle#<init> со скоростью (xa, ya, za).
    pub fn add_splash(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        if self.sets.get("splash").is_none_or(|s| s.is_empty()) {
            return;
        }
        self.add_water_drop("splash", 0.04, x, y, z);
        if ya == 0.0 && (xa != 0.0 || za != 0.0) {
            if let Some(p) = self.list.back_mut() {
                p.xd = xa;
                p.yd = 0.1;
                p.zd = za;
            }
        }
    }

    /// VANILLA: WaterDropParticle.Provider (ParticleTypes.RAIN).
    pub fn add_rain(&mut self, x: f64, y: f64, z: f64) {
        if !self.allow() {
            return;
        }
        self.add_water_drop("rain", 0.06, x, y, z);
    }

    /// VANILLA: WaterDropParticle#<init> (RAIN: gravity .06; SplashParticle: .04).
    fn add_water_drop(&mut self, name: &str, gravity: f32, x: f64, y: f64, z: f64) {
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::WaterDrop, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.xd *= f64::from(0.3_f32);
        p.yd = f64::from(p.rng.next_float() * 0.2 + 0.1);
        p.zd *= f64::from(0.3_f32);
        p.set_size(0.01, 0.01);
        p.gravity = gravity;
        p.lifetime = (8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        self.add(p);
    }

    /// VANILLA: DripParticle.*Provider (вода: цвет .2 .3 1; лава падает: 1 .2857143 .083333336).
    fn add_drip(&mut self, stage: Stage, x: f64, y: f64, z: f64) {
        let Some(set) = self.sets.get(stage.set_name()).cloned() else {
            return;
        };
        let seed = self.seed();
        // DripParticle не получает случайной скорости (Particle(level,x,y,z)).
        let mut p = Particle::base(Kind::Drip, x, y, z, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0;
        p.set_size(0.01, 0.01);
        p.gravity = 0.06;
        p.drip = Some(stage);
        match stage {
            Stage::HangWater | Stage::StoneHangWater => {
                p.gravity *= 0.02;
                p.lifetime = 40;
                p.rgb = [0.2, 0.3, 1.0];
            }
            Stage::HangLava | Stage::StoneHangLava => {
                p.gravity *= 0.02;
                p.lifetime = 40;
            }
            Stage::FallWater | Stage::StoneFallWater => {
                p.lifetime = (64.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
                p.rgb = [0.2, 0.3, 1.0];
            }
            Stage::FallLava | Stage::StoneFallLava => {
                p.lifetime = (64.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
                p.rgb = [1.0, 0.2857143, 0.083_333_336];
            }
            Stage::LandLava => {
                p.lifetime = (16.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
                p.rgb = [1.0, 0.2857143, 0.083_333_336];
            }
        }
        self.add(p);
    }

    /// VANILLA: SmokeParticle.Provider (scale 1), BaseAshSmokeParticle#<init>.
    pub fn add_smoke(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        self.add_ash("smoke", 1.0, None, x, y, z, xa, ya, za);
    }

    /// Гаусс из генератора движка (для random.nextGaussian() сущностей).
    pub fn gaussian(&mut self) -> f64 {
        self.rng.next_gaussian()
    }

    /// random.nextFloat() из генератора движка (клиентский тик сущностей).
    pub fn rand_f32(&mut self) -> f32 {
        self.rng.next_float()
    }

    pub fn rand3(&mut self) -> (f64, f64, f64) {
        (
            self.rng.next_double(),
            self.rng.next_double(),
            self.rng.next_double(),
        )
    }

    /// VANILLA: GlowParticle.GlowSquidProvider (GlowSquid#aiStep: GLOW с нулевой скоростью).
    pub fn add_glow_squid(&mut self, x: f64, y: f64, z: f64) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get("glow").cloned() else {
            return;
        };
        let (xa, za) = (0.5 - self.rng.next_double(), 0.5 - self.rng.next_double());
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Glow, x, y, z, xa, 0.0, za, seed);
        p.friction = 0.96;
        p.speed_up_blocked = true;
        p.quad_size *= 0.75;
        p.has_physics = false;
        p.rgb = if self.rng.next_boolean() {
            [0.6, 1.0, 0.8]
        } else {
            [0.08, 0.4, 0.4]
        };
        p.yd *= f64::from(0.2_f32);
        p.xd *= f64::from(0.1_f32);
        p.zd *= f64::from(0.1_f32);
        p.lifetime = (8.0 / (self.rng.next_double() * 0.8 + 0.2)) as i32;
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: Blaze#aiStep (2 LARGE_SMOKE), EnderMan/Endermite#aiStep (2 PORTAL) — клиентский тик моба.
    pub fn mob_ambient(&mut self, name: &str, pos: [f64; 3], w: f64, h: f64) {
        for _ in 0..2 {
            let mut r = [0.0_f64; 5];
            for v in &mut r {
                *v = self.rng.next_double();
            }
            let (x, y, z) = (
                pos[0] + w * (2.0 * r[0] - 1.0) * 0.5,
                pos[1] + h * r[1],
                pos[2] + w * (2.0 * r[2] - 1.0) * 0.5,
            );
            match name {
                "blaze" => self.add_ash("large_smoke", 2.5, None, x, y, z, 0.0, 0.0, 0.0),
                _ => {
                    let (xa, ya, za) = (
                        (self.rng.next_double() - 0.5) * 2.0,
                        -self.rng.next_double(),
                        (self.rng.next_double() - 0.5) * 2.0,
                    );
                    self.add_portal(x, y, z, xa, ya, za);
                }
            }
        }
    }

    /// VANILLA: BaseAshSmokeParticle#<init>: smoke (scale 1), large_smoke (2.5), dust (redstone).
    /// `tint` = Some(rgb): DustParticleBase (цвет случайный вокруг tint).
    #[allow(clippy::too_many_arguments)]
    fn add_ash(
        &mut self,
        name: &str,
        scale: f32,
        tint: Option<[f32; 3]>,
        x: f64,
        y: f64,
        z: f64,
        xa: f64,
        ya: f64,
        za: f64,
    ) {
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Smoke, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[0];
        p.friction = 0.96;
        p.gravity = -0.1;
        p.speed_up_blocked = true;
        p.xd *= f64::from(0.1_f32);
        p.yd *= f64::from(0.1_f32);
        p.zd *= f64::from(0.1_f32);
        p.xd += xa;
        p.yd += ya;
        p.zd += za;
        if let Some(c) = tint {
            let base = p.rng.next_float() * 0.4 + 0.6;
            for (o, ci) in p.rgb.iter_mut().zip(c) {
                *o = (p.rng.next_float() * 0.2 + 0.8) * ci * base;
            }
        } else {
            let col = p.rng.next_float() * 0.3;
            p.rgb = [col; 3];
        }
        p.quad_size *= 0.75 * scale;
        p.lifetime =
            ((8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2) * f64::from(scale)) as i32).max(1);
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: TerrainParticle#<init>.
    #[allow(clippy::too_many_arguments)]
    pub fn add_terrain(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        xa: f64,
        ya: f64,
        za: f64,
        look: TerrainLook,
    ) {
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Terrain, x, y, z, xa, ya, za, seed);
        p.atlas = ParticleAtlas::Blocks;
        p.sprite = look.sprite;
        p.gravity = 1.0;
        p.rgb = [0.6 * look.tint[0], 0.6 * look.tint[1], 0.6 * look.tint[2]];
        p.quad_size /= 2.0;
        p.uo = p.rng.next_float() * 3.0;
        p.vo = p.rng.next_float() * 3.0;
        self.add(p);
    }

    /// VANILLA: Entity#spawnSprintParticle — частица блока под ногами при беге (не в воде).
    pub fn sprint(
        &mut self,
        pos: [f64; 3],
        vel: [f64; 3],
        width: f64,
        state: BlockState,
        block: BlockPos,
        look: &mut dyn FnMut(BlockState, BlockPos) -> Option<TerrainLook>,
    ) {
        if !spawns_terrain_particles(state) {
            return;
        }
        let Some(look) = look(state, block) else {
            return;
        };
        let x = pos[0] + (self.rng.next_double() - 0.5) * width;
        let z = pos[2] + (self.rng.next_double() - 0.5) * width;
        if self.allow() {
            self.add_terrain(x, pos[1] + 0.1, z, vel[0] * -4.0, 1.5, vel[2] * -4.0, look);
        }
    }

    /// VANILLA: ParticleEngine#crack — частица на грани блока, пока его ломают (каждый тик).
    /// `face`: 0 down, 1 up, 2 north, 3 south, 4 west, 5 east.
    pub fn crack(
        &mut self,
        pos: BlockPos,
        state: BlockState,
        face: u8,
        look: &mut dyn FnMut(BlockState, BlockPos) -> Option<TerrainLook>,
    ) {
        if !spawns_terrain_particles(state) {
            return;
        }
        let Some(look) = look(state, pos) else {
            return;
        };
        let boxes = blockinfo::outline_boxes(state);
        let mut b = [1.0_f64, 1.0, 1.0, 0.0, 0.0, 0.0];
        for o in boxes.iter() {
            for i in 0..3 {
                b[i] = b[i].min(o[i]);
                b[i + 3] = b[i + 3].max(o[i + 3]);
            }
        }
        if boxes.is_empty() {
            b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        }
        let (x, y, z) = (f64::from(pos.x), f64::from(pos.y), f64::from(pos.z));
        let r = |e: &mut Self, lo: f64, hi: f64| e.rng.next_double() * (hi - lo - 0.2) + 0.1 + lo;
        let mut xp = x + r(self, b[0], b[3]);
        let mut yp = y + r(self, b[1], b[4]);
        let mut zp = z + r(self, b[2], b[5]);
        match face {
            0 => yp = y + b[1] - 0.1,
            1 => yp = y + b[4] + 0.1,
            2 => zp = z + b[2] - 0.1,
            3 => zp = z + b[5] + 0.1,
            4 => xp = x + b[0] - 0.1,
            _ => xp = x + b[3] + 0.1,
        }
        self.add_terrain(xp, yp, zp, 0.0, 0.0, 0.0, look);
        // setPower(0.2F).scale(0.6F) — над только что добавленной частицей.
        if let Some(p) = self.list.back_mut() {
            p.xd *= 0.2;
            p.yd = (p.yd - 0.1) * 0.2 + 0.1;
            p.zd *= 0.2;
            p.quad_size *= 0.6;
        }
    }

    /// VANILLA: ClientLevel#addDestroyBlockEffect.
    pub fn destroy_block(
        &mut self,
        pos: BlockPos,
        state: BlockState,
        look: &mut dyn FnMut(BlockState, BlockPos) -> Option<TerrainLook>,
    ) {
        if !spawns_terrain_particles(state) {
            return;
        }
        let Some(look) = look(state, pos) else {
            return;
        };
        for b in blockinfo::outline_boxes(state) {
            let (wx, wy, wz) = (
                (b[3] - b[0]).min(1.0),
                (b[4] - b[1]).min(1.0),
                (b[5] - b[2]).min(1.0),
            );
            let cnt = |w: f64| ((w / 0.25).ceil() as i32).max(2);
            let (cx, cy, cz) = (cnt(wx), cnt(wy), cnt(wz));
            for xx in 0..cx {
                for yy in 0..cy {
                    for zz in 0..cz {
                        let rx = (f64::from(xx) + 0.5) / f64::from(cx);
                        let ry = (f64::from(yy) + 0.5) / f64::from(cy);
                        let rz = (f64::from(zz) + 0.5) / f64::from(cz);
                        self.add_terrain(
                            f64::from(pos.x) + rx * wx + b[0],
                            f64::from(pos.y) + ry * wy + b[1],
                            f64::from(pos.z) + rz * wz + b[2],
                            rx - 0.5,
                            ry - 0.5,
                            rz - 0.5,
                            look,
                        );
                    }
                }
            }
        }
    }

    /// Простая цветная частица из набора спрайтов `name` (для эффектов зелий, happy_villager, wax, cloud).
    #[allow(clippy::too_many_arguments)]
    pub fn add_tinted(
        &mut self,
        name: &str,
        x: f64,
        y: f64,
        z: f64,
        xa: f64,
        ya: f64,
        za: f64,
        rgb: [f32; 3],
        gravity: f32,
        lifetime: i32,
    ) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Smoke, x, y, z, seed);
        p.sprite = set[0];
        p.set = Some(set);
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        p.rgb = rgb;
        p.gravity = gravity;
        p.friction = 0.96;
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0;
        p.lifetime = lifetime.max(1);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: GlowParticle.{ElectricSpark,Scrape,WaxOff,WaxOn}Provider — `aux` = xAux/yAux/zAux.
    #[allow(clippy::too_many_arguments)]
    pub fn add_glow_variant(&mut self, name: &str, x: f64, y: f64, z: f64, aux: [f64; 3]) {
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Glow, x, y, z, 0.0, 0.0, 0.0, seed);
        p.friction = 0.96;
        p.speed_up_blocked = true;
        p.quad_size *= 0.75;
        p.has_physics = false;
        let (rgb, speed, life) = match name {
            "electric_spark" => (
                [1.0, 0.9, 1.0],
                [aux[0] * 0.25, aux[1] * 0.25, aux[2] * 0.25],
                p.rng.next_int_bound(2) + 2,
            ),
            "scrape" => (
                if p.rng.next_boolean() {
                    [0.29, 0.58, 0.51]
                } else {
                    [0.43, 0.77, 0.62]
                },
                [aux[0] * 0.01, aux[1] * 0.01, aux[2] * 0.01],
                p.rng.next_int_bound(30) + 10,
            ),
            _ => (
                if name == "wax_on" {
                    [0.91, 0.55, 0.08]
                } else {
                    [1.0, 0.9, 1.0]
                },
                [aux[0] * 0.01 / 2.0, aux[1] * 0.01, aux[2] * 0.01 / 2.0],
                p.rng.next_int_bound(30) + 10,
            ),
        };
        p.rgb = rgb;
        p.xd = speed[0];
        p.yd = speed[1];
        p.zd = speed[2];
        p.lifetime = life.max(1);
        p.set = Some(set);
        p.sprite_from_age();
        self.add(p);
    }

    /// VANILLA: ParticleUtils#spawnParticlesOnBlockFaces (скорости getRandomSpeedRanges, step 0.55).
    fn on_block_faces(&mut self, name: &str, pos: [i32; 3], lo: i32, hi: i32) {
        let c = [
            f64::from(pos[0]) + 0.5,
            f64::from(pos[1]) + 0.5,
            f64::from(pos[2]) + 0.5,
        ];
        // Direction.values(): DOWN, UP, NORTH, SOUTH, WEST, EAST
        for step in [
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [-1, 0, 0],
            [1, 0, 0],
        ] {
            let n = lo + self.rng.next_int_bound(hi - lo + 1);
            for _ in 0..n {
                let sp = [
                    self.rng.next_double() - 0.5,
                    self.rng.next_double() - 0.5,
                    self.rng.next_double() - 0.5,
                ];
                let mut q = [0.0; 3];
                let mut v = [0.0; 3];
                for a in 0..3 {
                    if step[a] == 0 {
                        q[a] = c[a] + (self.rng.next_double() - 0.5);
                        v[a] = sp[a];
                    } else {
                        q[a] = c[a] + f64::from(step[a]) * 0.55;
                    }
                }
                if name == "egg_crack" {
                    self.add_egg_crack(q[0], q[1], q[2], v[0], v[1], v[2]);
                } else {
                    self.add_glow_variant(name, q[0], q[1], q[2], v);
                }
            }
        }
    }

    /// VANILLA: ParticleUtils#spawnParticlesAlongAxis (radius 0.125, 10..19 искр).
    fn along_axis(&mut self, name: &str, pos: [i32; 3], axis: usize) {
        let c = [
            f64::from(pos[0]) + 0.5,
            f64::from(pos[1]) + 0.5,
            f64::from(pos[2]) + 0.5,
        ];
        let n = 10 + self.rng.next_int_bound(10);
        for _ in 0..n {
            let mut q = [0.0; 3];
            let mut v = [0.0; 3];
            for a in 0..3 {
                q[a] = c[a]
                    + (self.rng.next_double() * 2.0 - 1.0) * if a == axis { 0.5 } else { 0.125 };
            }
            for (a, va) in v.iter_mut().enumerate() {
                if a == axis {
                    *va = self.rng.next_double() * 2.0 - 1.0;
                }
            }
            self.add_glow_variant(name, q[0], q[1], q[2], v);
        }
    }

    /// VANILLA: LevelEventHandler#levelEvent (26.3) — частицы событий мира.
    pub fn level_event(&mut self, ty: i32, pos: [i32; 3], data: i32) {
        let (fx, fy, fz) = (f64::from(pos[0]), f64::from(pos[1]), f64::from(pos[2]));
        match ty {
            2000 => self.shoot_smoke(data, pos[0], pos[1], pos[2]),
            2010 => {
                // VANILLA: LevelEventHandler#shootParticles(WHITE_SMOKE).
                let (dx, dy, dz) = match data {
                    0 => (0.0, -1.0, 0.0),
                    1 => (0.0, 1.0, 0.0),
                    2 => (0.0, 0.0, -1.0),
                    3 => (0.0, 0.0, 1.0),
                    4 => (-1.0, 0.0, 0.0),
                    5 => (1.0, 0.0, 0.0),
                    _ => (0.0, 0.0, 0.0),
                };
                for _ in 0..10 {
                    let sp = self.rng.next_double() * 0.2 + 0.01;
                    let (px, py, pz) = (
                        fx + 0.5 + dx * 0.6 + (self.rng.next_double() - 0.5) * 0.3,
                        fy + 0.5 + dy * 0.6 + (self.rng.next_double() - 0.5) * 0.3,
                        fz + 0.5 + dz * 0.6 + (self.rng.next_double() - 0.5) * 0.3,
                    );
                    let (xa, ya, za) = (
                        dx * sp + self.rng.next_gaussian() * 0.01,
                        dy * sp + self.rng.next_gaussian() * 0.01,
                        dz * sp + self.rng.next_gaussian() * 0.01,
                    );
                    self.add_ash(
                        "white_smoke",
                        1.0,
                        Some([0.9, 0.9, 0.9]),
                        px,
                        py,
                        pz,
                        xa,
                        ya,
                        za,
                    );
                }
            }
            1500 => {
                // VANILLA: ComposterBlock#handleFill — happy_villager сверху блока.
                for _ in 0..10 {
                    let px = fx + self.rng.next_double() * 0.75 + 0.125;
                    let py = fy + 0.6 + self.rng.next_double() * 0.4;
                    let pz = fz + self.rng.next_double() * 0.75 + 0.125;
                    let (xa, ya, za) = (
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                    );
                    self.add_tinted(
                        "happy_villager",
                        px,
                        py,
                        pz,
                        xa,
                        ya,
                        za,
                        [1.0, 1.0, 1.0],
                        0.0,
                        20,
                    );
                }
            }
            1501 => {
                // VANILLA: LevelEventHandler case 1501 — шипение лавы (8× LARGE_SMOKE).
                for _ in 0..8 {
                    let px = fx + self.rng.next_double();
                    let pz = fz + self.rng.next_double();
                    self.add_ash("large_smoke", 2.5, None, px, fy + 1.2, pz, 0.0, 0.0, 0.0);
                }
            }
            1502 => {
                // VANILLA: LevelEventHandler case 1502 — перегорание редстоун-факела (5× SMOKE).
                for _ in 0..5 {
                    let px = fx + self.rng.next_double() * 0.6 + 0.2;
                    let py = fy + self.rng.next_double() * 0.6 + 0.2;
                    let pz = fz + self.rng.next_double() * 0.6 + 0.2;
                    self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
                }
            }
            1503 => {
                // VANILLA: LevelEventHandler case 1503 — вставка Ока Эндера в рамку (16× SMOKE).
                for _ in 0..16 {
                    let px = fx + (5.0 + self.rng.next_double() * 6.0) / 16.0;
                    let pz = fz + (5.0 + self.rng.next_double() * 6.0) / 16.0;
                    self.add_smoke(px, fy + 0.8125, pz, 0.0, 0.0, 0.0);
                }
            }
            1505 | 2011 | 2012 => {
                // VANILLA: BoneMealItem#addGrowthParticles / ParticleUtils#spawnParticleInBlock.
                let n = if data <= 0 { 15 } else { data.min(64) };
                for _ in 0..n {
                    let px = fx + self.rng.next_double();
                    let py = fy + self.rng.next_double();
                    let pz = fz + self.rng.next_double();
                    let (xa, ya, za) = (
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                    );
                    self.add_tinted(
                        "happy_villager",
                        px,
                        py,
                        pz,
                        xa,
                        ya,
                        za,
                        [1.0, 1.0, 1.0],
                        0.0,
                        20,
                    );
                }
            }
            2002 | 2007 => {
                // VANILLA: LevelEventHandler#potionSplashParticles (EFFECT / INSTANT_EFFECT, RGB из data).
                let sprite = if ty == 2007 {
                    "instant_effect"
                } else {
                    "effect"
                };
                let r = ((data >> 16) & 0xFF) as f32 / 255.0;
                let g = ((data >> 8) & 0xFF) as f32 / 255.0;
                let b = (data & 0xFF) as f32 / 255.0;
                let (cx, cy, cz) = (fx + 0.5, fy + 0.1, fz + 0.5);
                for _ in 0..100 {
                    let dist = self.rng.next_double() * 4.0;
                    let ang = self.rng.next_double() * std::f64::consts::TAU;
                    let xa = ang.cos() * dist;
                    let ya = 0.01 + self.rng.next_double() * 0.5;
                    let za = ang.sin() * dist;
                    let k = 0.75 + self.rng.next_float() * 0.25;
                    self.add_tinted(
                        sprite,
                        cx + xa * 0.1,
                        cy + 0.3,
                        cz + za * 0.1,
                        xa * 0.1,
                        ya * 0.1,
                        za * 0.1,
                        [r * k, g * k, b * k],
                        -0.02,
                        16,
                    );
                }
            }
            2003 => {
                // VANILLA: LevelEventHandler case 2003 — кольцо portal при разрушении Ока Эндера.
                let (cx, cy, cz) = (fx + 0.5, fy - 0.4, fz + 0.5);
                let step = std::f64::consts::TAU / 40.0;
                for i in 0..40 {
                    let ang = i as f64 * step;
                    let (ca, sa) = (ang.cos(), ang.sin());
                    self.add_portal(cx + ca * 5.0, cy, cz + sa * 5.0, ca * -5.0, 0.0, sa * -5.0);
                    self.add_portal(cx + ca * 5.0, cy, cz + sa * 5.0, ca * -7.0, 0.0, sa * -7.0);
                }
            }
            2004 => {
                // VANILLA: LevelEventHandler case 2004 — спавнер (20× SMOKE + FLAME).
                for _ in 0..20 {
                    let px = fx + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
                    let py = fy + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
                    let pz = fz + 0.5 + (self.rng.next_double() - 0.5) * 2.0;
                    self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
                    self.add_flame("flame", px, py, pz);
                }
            }
            2006 => {
                // VANILLA: LevelEventHandler case 2006 — взрыв шара дракона (200× DRAGON_BREATH).
                for _ in 0..200 {
                    let dist = f64::from(self.rng.next_float()) * 4.0;
                    let ang = f64::from(self.rng.next_float()) * std::f64::consts::TAU;
                    let xa = ang.cos() * dist;
                    let ya = 0.01 + self.rng.next_double() * 0.5;
                    let za = ang.sin() * dist;
                    self.add_dragon_breath(fx + xa * 0.1, fy + 0.3, fz + za * 0.1, xa, ya, za);
                }
            }
            2008 => self.add_huge(fx + 0.5, fy + 0.5, fz + 0.5, 1.0),
            2009 => {
                // VANILLA: LevelEventHandler case 2009 — испарение мокрой губки в Незере (8× CLOUD).
                for _ in 0..8 {
                    let px = fx + self.rng.next_double();
                    let pz = fz + self.rng.next_double();
                    self.add_tinted(
                        "cloud",
                        px,
                        fy + 1.2,
                        pz,
                        0.0,
                        0.0,
                        0.0,
                        [1.0, 1.0, 1.0],
                        0.0,
                        20,
                    );
                }
            }
            2018 => {
                // VANILLA: LevelEventHandler case 2018 — телепорт эндермена (BlockUtil#unpackDifferenceInPosition).
                let dx = ((data >> 16) & 0xFF) - 127;
                let dy = ((data >> 8) & 0xFF) - 127;
                let dz = (data & 0xFF) - 127;
                let (tx, ty2, tz) = (fx + f64::from(dx), fy + f64::from(dy), fz + f64::from(dz));
                for _ in 0..128 {
                    let d = self.rng.next_double();
                    let xa = f64::from((self.rng.next_float() - 0.5) * 0.2);
                    let ya = f64::from((self.rng.next_float() - 0.5) * 0.2);
                    let za = f64::from((self.rng.next_float() - 0.5) * 0.2);
                    let px = tx + (fx - tx) * d + (self.rng.next_double() - 0.5) * 1.2;
                    let py = ty2 + (fy - ty2) * d + self.rng.next_double() * 2.9;
                    let pz = tz + (fz - tz) * d + (self.rng.next_double() - 0.5) * 1.2;
                    self.add_portal(px, py, pz, xa, ya, za);
                }
            }
            3000 => self.add_explosion_emitter(fx + 0.5, fy + 0.5, fz + 0.5),
            3002 => {
                // VANILLA: LevelEventHandler case 3002 (молния по оси / по граням).
                if (0..3).contains(&data) {
                    self.along_axis("electric_spark", pos, data as usize);
                } else {
                    self.on_block_faces("electric_spark", pos, 3, 5);
                }
            }
            2015 => self.block_teleport(pos, data, [16, 8, 16]),
            2016 => self.block_teleport(pos, data, [8, 8, 8]),
            2017 => self.block_teleport(pos, data, [127, 127, 127]),
            3009 => self.on_block_faces("egg_crack", pos, 3, 6),
            3011 => self.trial_spawn(
                pos,
                if data == 1 {
                    "soul_fire_flame"
                } else {
                    "flame"
                },
            ),
            3012 => self.trial_spawn(
                pos,
                if data == 1 {
                    "soul_fire_flame"
                } else {
                    "flame"
                },
            ),
            3013 => {
                // VANILLA: TrialSpawner#addDetectPlayerParticles.
                let n = 30 + data.clamp(0, 10) * 5;
                for _ in 0..n {
                    let sx = f64::from(2.0 * self.rng.next_float() - 1.0) * 0.65;
                    let sz = f64::from(2.0 * self.rng.next_float() - 1.0) * 0.65;
                    let y = fy + 0.1 + f64::from(self.rng.next_float()) * 0.8;
                    self.add_trial_detect(fx + 0.5 + sx, y, fz + 0.5 + sz);
                }
            }
            3014 | 3017 => self.trial_eject(pos),
            3015 => {
                // VANILLA: VaultBlockEntity.Client#emitActivationParticles.
                if let Some(v) = self.vaults.iter().find(|v| v.pos == pos).cloned() {
                    self.vault_connection(&v);
                }
                let flame = if data == 0 {
                    "small_flame"
                } else {
                    "soul_fire_flame"
                };
                for _ in 0..20 {
                    let x = fx + 0.1 + self.rng.next_double() * 0.8;
                    let y = fy + 0.25 + self.rng.next_double() * 0.5;
                    let z = fz + 0.1 + self.rng.next_double() * 0.8;
                    self.add_smoke(x, y, z, 0.0, 0.0, 0.0);
                    self.add_flame(flame, x, y, z);
                }
            }
            3016 => {
                // VANILLA: VaultBlockEntity.Client#emitDeactivationParticles.
                let flame = if data == 0 {
                    "small_flame"
                } else {
                    "soul_fire_flame"
                };
                for _ in 0..20 {
                    let x = fx + 0.4 + self.rng.next_double() * 0.2;
                    let y = fy + 0.4 + self.rng.next_double() * 0.2;
                    let z = fz + 0.4 + self.rng.next_double() * 0.2;
                    let v = [
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                    ];
                    self.add_flame_v(flame, x, y, z, v);
                }
            }
            3003 => self.on_block_faces("wax_on", pos, 3, 5),
            3004 => self.on_block_faces("wax_off", pos, 3, 5),
            3005 => self.on_block_faces("scrape", pos, 3, 5),
            3018 => {
                // VANILLA: LevelEventHandler case 3018 — 10× POOF (паутина/weaving).
                for _ in 0..10 {
                    let px = fx + self.rng.next_double();
                    let py = fy + self.rng.next_double();
                    let pz = fz + self.rng.next_double();
                    let (xa, ya, za) = (
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                        self.rng.next_gaussian() * 0.02,
                    );
                    self.add_poof(px, py, pz, xa, ya, za);
                }
            }
            _ => {}
        }
    }

    /// VANILLA: ClientPacketListener#handleParticleEvent. Поддержаны `block`, `rain`, `smoke`.
    pub fn handle_packet(
        &mut self,
        p: &ParticlesPacket,
        look: &mut dyn FnMut(BlockState, BlockPos) -> Option<TerrainLook>,
    ) {
        let mut spawn =
            |e: &mut Self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64| match p.name {
                "rain" => e.add_rain(x, y, z),
                "smoke" => e.add_smoke(x, y, z, xa, ya, za),
                "portal" => e.add_portal(x, y, z, xa, ya, za),
                // VANILLA-UNSURE: power из пакета не читаем (=1.0).
                "dragon_breath" => e.add_dragon_breath(x, y, z, xa, ya, za),
                "end_rod" => e.add_end_rod(x, y, z, xa, ya, za),
                // VANILLA: NoteParticle — xa = цвет (JukeboxSongPlayer#spawnMusicParticles).
                "note" => e.add_note(x, y, z, xa),
                "flame" | "soul_fire_flame" | "copper_fire_flame" => e.add_flame(p.name, x, y, z),
                "large_smoke" => e.add_ash("large_smoke", 2.5, None, x, y, z, xa, ya, za),
                "white_smoke" => e.add_ash(
                    "white_smoke",
                    1.0,
                    Some([0.9, 0.9, 0.9]),
                    x,
                    y,
                    z,
                    xa,
                    ya,
                    za,
                ),
                "cloud" => e.add_tinted("cloud", x, y, z, xa, ya, za, [1.0, 1.0, 1.0], 0.0, 16),
                "happy_villager" => e.add_tinted(
                    "happy_villager",
                    x,
                    y,
                    z,
                    xa,
                    ya,
                    za,
                    [1.0, 1.0, 1.0],
                    0.0,
                    20,
                ),
                "poof" => e.add_poof(x, y, z, xa, ya, za),
                "crit" => e.add_crit(x, y, z, xa, ya, za),
                "explosion" => e.add_huge(x, y, z, 1.0),
                "explosion_emitter" => e.add_explosion_emitter(x, y, z),
                "lava" => e.add_lava(x, y, z),
                "bubble" => e.add_bubble(x, y, z, xa, ya, za),
                "bubble_column_up" => e.add_named_bubble("bubble_column_up", x, y, z, xa, ya, za),
                "splash" => e.add_splash(x, y, z, xa, ya, za),
                "glow" => e.add_glow_squid(x, y, z),
                "electric_spark" | "wax_on" | "wax_off" | "scrape" => {
                    e.add_glow_variant(p.name, x, y, z, [xa, ya, za]);
                }
                "block" => {
                    if let Some(st) = p.block.and_then(BlockState::from_id) {
                        if spawns_terrain_particles(st) {
                            let pos = BlockPos {
                                x: x.floor() as i32,
                                y: y.floor() as i32,
                                z: z.floor() as i32,
                            };
                            if let (true, Some(l)) = (e.allow(), look(st, pos)) {
                                e.add_terrain(x, y, z, xa, ya, za, l);
                            }
                        }
                    }
                }
                _ => {}
            };
        let sp = f64::from(p.max_speed);
        if p.count == 0 {
            let (xa, ya, za) = (
                sp * f64::from(p.dist[0]),
                sp * f64::from(p.dist[1]),
                sp * f64::from(p.dist[2]),
            );
            spawn(self, p.pos[0], p.pos[1], p.pos[2], xa, ya, za);
        } else {
            for _ in 0..p.count.clamp(0, 1024) {
                let g = |e: &mut Self| e.rng.next_gaussian();
                let (xo, yo, zo) = (
                    g(self) * f64::from(p.dist[0]),
                    g(self) * f64::from(p.dist[1]),
                    g(self) * f64::from(p.dist[2]),
                );
                let (xa, ya, za) = (g(self) * sp, g(self) * sp, g(self) * sp);
                spawn(
                    self,
                    p.pos[0] + xo,
                    p.pos[1] + yo,
                    p.pos[2] + zo,
                    xa,
                    ya,
                    za,
                );
            }
        }
    }

    /// VANILLA: ClientLevel#tickWeatherEffects — брызги дождя (звук — после аудио, TODO-288).
    pub fn tick_weather(
        &mut self,
        level: &Level,
        biomes: &[Option<BiomeColors>],
        cam: [f64; 3],
        radius: i32,
    ) {
        let rain = level.rain_level;
        if rain <= 0.0 {
            return;
        }
        let mut random = LegacyRandom::new(level.game_time.wrapping_mul(312_987_231));
        let (cx, cy, cz) = (
            cam[0].floor() as i32,
            cam[1].floor() as i32,
            cam[2].floor() as i32,
        );
        let diameter = 2 * radius + 1;
        let area = diameter * diameter;
        let count = (0.225 * area as f32 * rain * rain) as i32;
        let mut rain_pos: Option<(i32, i32, i32)> = None;
        for _ in 0..count {
            let x = cx + random.next_int_bound(diameter) - radius;
            let z = cz + random.next_int_bound(diameter) - radius;
            let Some(chunk) = level.chunks.get(x >> 4, z >> 4) else {
                continue;
            };
            let (lx, lz) = ((x & 15) as usize, (z & 15) as usize);
            let Some(top) = chunk.height(HeightmapType::MotionBlocking, lx, lz) else {
                continue;
            };
            // heightmapPosition.y > minY && в пределах ±10 от камеры && осадки = дождь
            if top <= chunk.chunk.min_y() || top > cy + 10 || top < cy - 10 {
                continue;
            }
            let Some(Some(b)) = biomes.get(chunk.chunk.biome(lx, top, lz) as usize) else {
                continue;
            };
            if precipitation_at(b, x, top, z, SEA_LEVEL) != Precipitation::Rain {
                continue;
            }
            let by = top - 1;
            rain_pos = Some((x, by, z));
            let (bx, bz) = (random.next_double(), random.next_double());
            let state = level.chunks.block(x, by, z);
            let fl = blockinfo::fluid(state);
            let top_y =
                shape_max_y(collision_boxes(state), bx, bz).max(fluid_height(level, x, by, z));
            let smoke = fl.is_some_and(|f| f.lava)
                || state.name() == "magma_block"
                || (state.name() == "campfire" && state.property("lit") == Some("true"));
            let (px, py, pz) = (f64::from(x) + bx, f64::from(by) + top_y, f64::from(z) + bz);
            if smoke {
                self.add_smoke(px, py, pz, 0.0, 0.0, 0.0);
            } else {
                self.add_rain(px, py, pz);
            }
        }
        // VANILLA: ClientLevel#tickRainParticles — звук дождя у последней точки осадков.
        if let Some((x, y, z)) = rain_pos {
            let t = self.rain_sound_time;
            self.rain_sound_time += 1;
            if random.next_int_bound(3) < t {
                self.rain_sound_time = 0;
                let covered = level
                    .chunks
                    .get(cx >> 4, cz >> 4)
                    .and_then(|c| {
                        c.height(
                            HeightmapType::MotionBlocking,
                            (cx & 15) as usize,
                            (cz & 15) as usize,
                        )
                    })
                    .is_some_and(|h| h > cy);
                let (ev, v, p) = if y > cy + 1 && covered {
                    ("weather.rain.above", 0.1, 0.5)
                } else {
                    ("weather.rain", 0.2, 1.0)
                };
                crate::audio::play_at(
                    ev,
                    crate::audio::Cat::Weather,
                    f64::from(x) + 0.5,
                    f64::from(y) + 0.5,
                    f64::from(z) + 0.5,
                    v,
                    p,
                );
            }
        }
    }

    /// Квады для отрисовки: позиции относительно камеры, интерполяция по `partial`.
    pub fn extract(&mut self, level: &Level, cam: [f64; 3], partial: f32) -> Vec<ParticleQuad> {
        let mut out = Vec::with_capacity(self.list.len());
        let col = |f: f32| (f.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        for p in &self.list {
            // VANILLA: FireworkParticles.SparkParticle#extract — мерцание.
            if p.twinkle && p.age >= p.lifetime / 3 && (p.age + p.lifetime) / 3 % 2 != 0 {
                continue;
            }
            let lerp = |a: f64, b: f64| a + f64::from(partial) * (b - a);
            let (u0, u1, v0, v1) = p.uv();
            if p.kind == Kind::Shriek {
                // VANILLA: ShriekParticle#extract — два повёрнутых квада, альфа убывает.
                if p.delay > 0 {
                    continue;
                }
                let alpha = 1.0 - ((p.age as f32 + partial) / p.lifetime as f32).clamp(0.0, 1.0);
                // rotationX(-MAGICAL_X_ROT) и rotationYXZ(-π, MAGICAL_X_ROT, 0)
                let (s, c) = (-SHRIEK_X_ROT).sin_cos();
                let up1 = [0.0, c, s];
                let (s2, c2) = SHRIEK_X_ROT.sin_cos();
                let (sy, cy) = (-std::f32::consts::PI).sin_cos();
                let rot_y = |v: [f32; 3]| [v[0] * cy + v[2] * sy, v[1], -v[0] * sy + v[2] * cy];
                let right2 = rot_y([1.0, 0.0, 0.0]);
                let up2 = rot_y([0.0, c2, s2]);
                let pos = [
                    (lerp(p.xo, p.x) - cam[0]) as f32,
                    (lerp(p.yo, p.y) - cam[1]) as f32,
                    (lerp(p.zo, p.z) - cam[2]) as f32,
                ];
                for axes in [([1.0, 0.0, 0.0], up1), (right2, up2)] {
                    out.push(ParticleQuad {
                        atlas: p.atlas,
                        pos,
                        size: p.quad_size(partial),
                        u0,
                        u1,
                        v0,
                        v1,
                        color: [col(p.rgb[0]), col(p.rgb[1]), col(p.rgb[2]), col(alpha)],
                        light: p.light(level, partial),
                        axes: Some(axes),
                        roll: 0.0,
                    });
                }
                continue;
            }
            // VANILLA: OverlayParticle#extract — setAlpha(0.6 - (age + a - 1) * 0.25 * 0.5).
            let alpha = match p.kind {
                Kind::Flash => 0.6 - (p.age as f32 + partial - 1.0) * 0.25 * 0.5,
                // VANILLA: Particle.LifetimeAlpha(0, 0.6, 0.25, 1)#currentAlphaForAge.
                Kind::VaultConnection => {
                    let t = ((p.age as f32 + partial) / p.lifetime as f32 - 0.25) / 0.75;
                    t.clamp(0.0, 1.0) * 0.6
                }
                _ => p.alpha,
            };
            out.push(ParticleQuad {
                atlas: p.atlas,
                pos: [
                    (lerp(p.xo, p.x) - cam[0]) as f32,
                    (lerp(p.yo, p.y) - cam[1]) as f32,
                    (lerp(p.zo, p.z) - cam[2]) as f32,
                ],
                size: p.quad_size(partial),
                u0,
                u1,
                v0,
                v1,
                color: [col(p.rgb[0]), col(p.rgb[1]), col(p.rgb[2]), col(alpha)],
                light: p.light(level, partial),
                axes: None,
                roll: p.roll,
            });
        }
        self.last_extracted = out.len();
        out
    }
}

/// VANILLA: ShriekParticle#MAGICAL_X_ROT (литерал 1.0472F, не π/3).
#[allow(clippy::approx_constant)]
const SHRIEK_X_ROT: f32 = 1.0472;

/// VANILLA: FireworkExplosion.Shape (26.3), id по STREAM_CODEC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FireworkShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl FireworkShape {
    /// VANILLA: FireworkExplosion.Shape#byId (ByIdMap.continuous, ZERO → SMALL_BALL).
    pub fn by_id(id: i32) -> Self {
        match id {
            1 => FireworkShape::LargeBall,
            2 => FireworkShape::Star,
            3 => FireworkShape::Creeper,
            4 => FireworkShape::Burst,
            _ => FireworkShape::SmallBall,
        }
    }

    /// VANILLA: FireworkExplosion.Shape#getSerializedName.
    pub fn name(self) -> &'static str {
        match self {
            FireworkShape::SmallBall => "small_ball",
            FireworkShape::LargeBall => "large_ball",
            FireworkShape::Star => "star",
            FireworkShape::Creeper => "creeper",
            FireworkShape::Burst => "burst",
        }
    }
}

/// VANILLA: FireworkExplosion — форма, цвета, цвета затухания, шлейф, мерцание.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FireworkSpec {
    pub shape: FireworkShape,
    pub colors: Vec<i32>,
    pub fade_colors: Vec<i32>,
    pub trail: bool,
    pub twinkle: bool,
}

/// Список взрывов из компонента `fireworks` (Fireworks.STREAM_CODEC: VarInt flight_duration,
/// List<FireworkExplosion> ≤ 256). Битые данные → пустой список (как отсутствие компонента).
pub fn parse_fireworks(bytes: &[u8]) -> Vec<FireworkSpec> {
    parse_fireworks_full(bytes).map(|f| f.1).unwrap_or_default()
}

/// Компонент `fireworks` целиком: (flight_duration, взрывы). Битые данные → `None`.
pub fn parse_fireworks_full(bytes: &[u8]) -> Option<(i32, Vec<FireworkSpec>)> {
    let mut r = ezzmc_protocol::wire::Reader::new(bytes);
    let flight = r.var_int().ok()?;
    let n = r.length("firework explosions", 256).ok()?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_explosion(&mut r).ok()?);
    }
    Some((flight, out))
}

/// Компонент `firework_explosion` (звезда фейерверка). Битые данные → `None`.
pub fn parse_firework_explosion(bytes: &[u8]) -> Option<FireworkSpec> {
    read_explosion(&mut ezzmc_protocol::wire::Reader::new(bytes)).ok()
}

/// VANILLA: FireworkExplosion#STREAM_CODEC — VarInt shape, List<Int> colors, List<Int> fade,
/// Bool trail, Bool twinkle.
fn read_explosion(
    r: &mut ezzmc_protocol::wire::Reader,
) -> ezzmc_protocol::ProtocolResult<FireworkSpec> {
    fn color_list(
        r: &mut ezzmc_protocol::wire::Reader,
    ) -> ezzmc_protocol::ProtocolResult<Vec<i32>> {
        let n = r.length("firework colors", 1 << 16)?;
        (0..n).map(|_| r.i32()).collect()
    }
    let shape = FireworkShape::by_id(r.var_int()?);
    let colors = color_list(r)?;
    let fade_colors = color_list(r)?;
    let trail = r.bool()?;
    let twinkle = r.bool()?;
    Ok(FireworkSpec {
        shape,
        colors,
        fade_colors,
        trail,
        twinkle,
    })
}

/// VANILLA: FireworkParticles.Starter#STAR_PARTICLE_COORDS.
const STAR_COORDS: &[[f64; 2]] = &[
    [0.0, 1.0],
    [0.3455, 0.309],
    [0.9511, 0.309],
    [0.379_591_836_734_693_9, -0.126_530_612_244_897_95],
    [0.612_244_897_959_183_7, -0.804_081_632_653_061_2],
    [0.0, -0.359_183_673_469_387_73],
];

/// VANILLA: FireworkParticles.Starter#CREEPER_PARTICLE_COORDS.
const CREEPER_COORDS: &[[f64; 2]] = &[
    [0.0, 0.2],
    [0.2, 0.2],
    [0.2, 0.6],
    [0.6, 0.6],
    [0.6, 0.2],
    [0.2, 0.2],
    [0.2, 0.0],
    [0.4, 0.0],
    [0.4, -0.6],
    [0.2, -0.6],
    [0.2, -0.4],
    [0.0, -0.4],
];

/// VANILLA: DyeColor.BLACK.getFireworkColor().
const BLACK_FIREWORK: i32 = 0x1E1B1B;

fn rgb_i32(c: i32) -> [f32; 3] {
    [
        ((c >> 16) & 0xFF) as f32 / 255.0,
        ((c >> 8) & 0xFF) as f32 / 255.0,
        (c & 0xFF) as f32 / 255.0,
    ]
}

/// VANILLA: FireworkParticles.Starter (NoRenderParticle): по одному взрыву каждые 2 тика.
struct Starter {
    pos: [f64; 3],
    vel: [f64; 3],
    life: i32,
    lifetime: i32,
    explosions: Vec<FireworkSpec>,
    play_sound: bool,
    twinkle_delay: bool,
}

/// Параметры одной искры от Starter#createParticle.
#[derive(Clone, Copy)]
struct SparkStyle<'a> {
    colors: &'a [i32],
    fade: &'a [i32],
    trail: bool,
    twinkle: bool,
}

impl ParticleEngine {
    /// Новая SparkParticle: (lifetime, quadSize) из её собственного random.
    /// VANILLA: SparkParticle(...) — SimpleAnimatedParticle(gravity 0.1, friction 0.91),
    /// quadSize *= 0.75, lifetime 48 + nextInt(12), hasPhysics по умолчанию (true).
    fn spark(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) -> Option<Particle> {
        let set = self.sets.get("firework").cloned()?;
        let seed = self.seed();
        let mut p = Particle::base(Kind::Firework, x, y, z, seed);
        p.quad_size = 0.1 * (p.rng.next_float() * 0.5 + 0.5) * 2.0 * 0.75;
        p.friction = 0.91;
        p.gravity = 0.1;
        p.xd = xa;
        p.yd = ya;
        p.zd = za;
        p.lifetime = 48 + p.rng.next_int_bound(12);
        p.alpha = 0.99;
        p.set = Some(set);
        p.sprite_from_age();
        Some(p)
    }

    /// VANILLA: SparkProvider — level.addParticle(FIREWORK) (шлейф летящей ракеты), белая искра.
    pub fn add_firework(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        if !self.allow() {
            return;
        }
        if let Some(p) = self.spark(x, y, z, xa, ya, za) {
            self.add(p);
        }
    }

    /// VANILLA: SparkParticle#tick — неподвижная искра шлейфа с возрастом lifetime/2.
    fn add_trail_spark(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        rgb: [f32; 3],
        fade: Option<[f32; 3]>,
        twinkle: bool,
    ) {
        if let Some(mut p) = self.spark(x, y, z, 0.0, 0.0, 0.0) {
            p.rgb = rgb;
            p.age = p.lifetime / 2;
            p.fade = fade;
            p.twinkle = twinkle;
            p.sprite_from_age();
            self.add(p);
        }
    }

    /// VANILLA: Starter#createParticle — engine.createParticle(FIREWORK) без фильтра настроек.
    fn starter_spark(&mut self, pos: [f64; 3], xa: f64, ya: f64, za: f64, st: SparkStyle<'_>) {
        if let Some(mut p) = self.spark(pos[0], pos[1], pos[2], xa, ya, za) {
            p.trail = st.trail;
            p.twinkle = st.twinkle;
            // VANILLA: Util.getRandom(rgbColors, random) — random самого Starter.
            let c = st.colors[self.rng.next_int_bound(st.colors.len() as i32) as usize];
            p.rgb = rgb_i32(c);
            if !st.fade.is_empty() {
                let f = st.fade[self.rng.next_int_bound(st.fade.len() as i32) as usize];
                p.fade = Some(rgb_i32(f));
            }
            self.add(p);
        }
    }

    /// VANILLA: FlashProvider — OverlayParticle, sprite.get(random), lifetime 4.
    fn add_flash(&mut self, pos: [f64; 3], color: i32) {
        let Some(set) = self.sets.get("flash").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::base(Kind::Flash, pos[0], pos[1], pos[2], seed);
        p.sprite = set[self.rng.next_int_bound(set.len() as i32) as usize];
        p.lifetime = 4;
        p.rgb = rgb_i32(color);
        self.add(p);
    }

    /// VANILLA: ClientLevel#createFireworks (26.3): без взрывов — 2..4 POOF, иначе Starter.
    pub fn create_fireworks(
        &mut self,
        pos: [f64; 3],
        vel: [f64; 3],
        explosions: Vec<FireworkSpec>,
        play_sound: bool,
    ) {
        if explosions.is_empty() {
            let mut i = 0;
            while i < self.rng.next_int_bound(3) + 2 {
                let xa = self.rng.next_gaussian() * 0.05;
                let za = self.rng.next_gaussian() * 0.05;
                self.add_poof(pos[0], pos[1], pos[2], xa, 0.005, za);
                i += 1;
            }
            return;
        }
        let twinkle_delay = explosions.iter().any(|e| e.twinkle);
        let lifetime = explosions.len() as i32 * 2 - 1 + if twinkle_delay { 15 } else { 0 };
        self.starters.push(Starter {
            pos,
            vel,
            life: 0,
            lifetime,
            explosions,
            play_sound,
            twinkle_delay,
        });
    }

    /// VANILLA: Starter#tick.
    fn tick_starters(&mut self, cam: [f64; 3]) {
        let mut list = std::mem::take(&mut self.starters);
        for s in &mut list {
            let far = || {
                (s.pos[0] - cam[0]).powi(2)
                    + (s.pos[1] - cam[1]).powi(2)
                    + (s.pos[2] - cam[2]).powi(2)
                    >= 256.0
            };
            if s.life == 0 && s.play_sound {
                let large = s.explosions.len() >= 3
                    || s.explosions
                        .iter()
                        .any(|e| e.shape == FireworkShape::LargeBall);
                let sound = match (large, far()) {
                    (true, true) => "entity.firework_rocket.large_blast_far",
                    (true, false) => "entity.firework_rocket.large_blast",
                    (false, true) => "entity.firework_rocket.blast_far",
                    (false, false) => "entity.firework_rocket.blast",
                };
                let pitch = 0.95 + self.rng.next_float() * 0.1;
                crate::audio::play_at(
                    sound,
                    crate::audio::Cat::Ambient,
                    s.pos[0],
                    s.pos[1],
                    s.pos[2],
                    20.0,
                    pitch,
                );
            }
            if s.life % 2 == 0 && ((s.life / 2) as usize) < s.explosions.len() {
                let e = &s.explosions[(s.life / 2) as usize];
                let colors: &[i32] = if e.colors.is_empty() {
                    &[BLACK_FIREWORK]
                } else {
                    &e.colors
                };
                let st = SparkStyle {
                    colors,
                    fade: &e.fade_colors,
                    trail: e.trail,
                    twinkle: e.twinkle,
                };
                match e.shape {
                    FireworkShape::SmallBall => self.particle_ball(s.pos, 0.25, 2, st),
                    FireworkShape::LargeBall => self.particle_ball(s.pos, 0.5, 4, st),
                    FireworkShape::Star => self.particle_shape(s.pos, 0.5, STAR_COORDS, st, false),
                    FireworkShape::Creeper => {
                        self.particle_shape(s.pos, 0.5, CREEPER_COORDS, st, true)
                    }
                    FireworkShape::Burst => self.particle_burst(s.pos, s.vel, st),
                }
                self.add_flash(s.pos, colors[0]);
            }
            s.life += 1;
            if s.life > s.lifetime {
                if s.twinkle_delay && s.play_sound {
                    let sound = if far() {
                        "entity.firework_rocket.twinkle_far"
                    } else {
                        "entity.firework_rocket.twinkle"
                    };
                    let pitch = 0.9 + self.rng.next_float() * 0.15;
                    crate::audio::play_at(
                        sound,
                        crate::audio::Cat::Ambient,
                        s.pos[0],
                        s.pos[1],
                        s.pos[2],
                        20.0,
                        pitch,
                    );
                }
                s.life = -1;
            }
        }
        list.retain(|s| s.life >= 0);
        list.append(&mut self.starters);
        self.starters = list;
    }

    /// VANILLA: Starter#createParticleBall.
    fn particle_ball(&mut self, pos: [f64; 3], base_speed: f64, steps: i32, st: SparkStyle<'_>) {
        for y_step in -steps..=steps {
            for x_step in -steps..=steps {
                let mut z_step = -steps;
                while z_step <= steps {
                    let r = &mut self.rng;
                    let xa = f64::from(x_step) + (r.next_double() - r.next_double()) * 0.5;
                    let ya = f64::from(y_step) + (r.next_double() - r.next_double()) * 0.5;
                    let za = f64::from(z_step) + (r.next_double() - r.next_double()) * 0.5;
                    let len = (xa * xa + ya * ya + za * za).sqrt() / base_speed
                        + r.next_gaussian() * 0.05;
                    self.starter_spark(pos, xa / len, ya / len, za / len, st);
                    if y_step != -steps && y_step != steps && x_step != -steps && x_step != steps {
                        z_step += steps * 2 - 1;
                    }
                    z_step += 1;
                }
            }
        }
    }

    /// VANILLA: Starter#createParticleShape.
    fn particle_shape(
        &mut self,
        pos: [f64; 3],
        base_speed: f64,
        coords: &[[f64; 2]],
        st: SparkStyle<'_>,
        flat: bool,
    ) {
        let [sx, sy] = coords[0];
        self.starter_spark(pos, sx * base_speed, sy * base_speed, 0.0, st);
        let base_angle = self.rng.next_float() * std::f32::consts::PI;
        let angle_mod = if flat { 0.034 } else { 0.34 };
        let pi_f = f64::from(std::f32::consts::PI);
        for angle_step in 0..3 {
            let angle = f64::from(base_angle) + f64::from(angle_step) * pi_f * angle_mod;
            let (mut ox, mut oy) = (sx, sy);
            for &[tx, ty] in &coords[1..] {
                let mut sub_step = 0.25;
                while sub_step <= 1.0 {
                    let mut xa = (ox + sub_step * (tx - ox)) * base_speed;
                    let ya = (oy + sub_step * (ty - oy)) * base_speed;
                    let za = xa * angle.sin();
                    xa *= angle.cos();
                    let mut flip = -1.0;
                    while flip <= 1.0 {
                        self.starter_spark(pos, xa * flip, ya, za * flip, st);
                        flip += 2.0;
                    }
                    sub_step += 0.25;
                }
                ox = tx;
                oy = ty;
            }
        }
    }

    /// VANILLA: Starter#createParticleBurst.
    fn particle_burst(&mut self, pos: [f64; 3], vel: [f64; 3], st: SparkStyle<'_>) {
        let base_off_x = self.rng.next_gaussian() * 0.05;
        let base_off_z = self.rng.next_gaussian() * 0.05;
        for _ in 0..70 {
            let xa = vel[0] * 0.5 + self.rng.next_gaussian() * 0.15 + base_off_x;
            let za = vel[2] * 0.5 + self.rng.next_gaussian() * 0.15 + base_off_z;
            let ya = vel[1] * 0.5 + self.rng.next_double() * 0.5;
            self.starter_spark(pos, xa, ya, za, st);
        }
    }
}

/// VANILLA: BlockState#shouldSpawnTerrainParticles (+ не воздух и не moving_piston).
pub fn spawns_terrain_particles(s: BlockState) -> bool {
    !s.is_air() && !matches!(s.name(), "moving_piston" | "barrier" | "structure_void")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sets() -> SpriteSets {
        let uv = SpriteUv {
            u0: 0.0,
            v0: 0.0,
            u1: 0.25,
            v1: 0.25,
        };
        let mut s = SpriteSets::default();
        s.insert("rain", vec![uv; 4]);
        s.insert("smoke", vec![uv; 8]);
        s.insert("firework", vec![uv; 8]);
        s.insert("flash", vec![uv; 1]);
        s.insert("vault_connection", vec![uv; 1]);
        s
    }

    #[test]
    fn fireworks_component_parses() {
        // flight 1; 1 взрыв: star(2), цвета [0xFF0000, 0x00FF00], fade [0x0000FF], trail, !twinkle.
        let mut b = vec![1u8, 1, 2, 2];
        b.extend_from_slice(&0xFF0000_i32.to_be_bytes());
        b.extend_from_slice(&0x00FF00_i32.to_be_bytes());
        b.push(1);
        b.extend_from_slice(&0x0000FF_i32.to_be_bytes());
        b.extend_from_slice(&[1, 0]);
        let v = parse_fireworks(&b);
        assert_eq!(
            v,
            vec![FireworkSpec {
                shape: FireworkShape::Star,
                colors: vec![0xFF0000, 0x00FF00],
                fade_colors: vec![0x0000FF],
                trail: true,
                twinkle: false,
            }]
        );
        assert!(parse_fireworks(&b[..5]).is_empty());
    }

    #[test]
    fn vault_connection_flies_to_keyhole() {
        let uuid: u128 = 0x0000_0001_0000_0002_0000_0003_0000_0004;
        let tag = Tag::Compound(vec![(
            "shared_data".into(),
            Tag::Compound(vec![(
                "connected_players".into(),
                Tag::List(vec![Tag::IntArray(vec![1, 2, 3, 4])]),
            )]),
        )]);
        let v = VaultInfo::from_block([0, 64, 0], "south", false, Some(&tag));
        assert_eq!(v.connected, [uuid]);
        assert_eq!(v.range, 4.5);
        assert!(!v.has_display_item);
        let mut e = ParticleEngine::new(sets());
        // Игрок далеко (blockPosition distSqr > 4.5²) — частиц связи нет.
        e.set_vaults(vec![v.clone()], vec![(uuid, [10.5, 64.0, 0.5], 1.8)]);
        e.vault_connection(&v);
        assert_eq!(e.count(), 0);
        e.set_vaults(vec![v.clone()], vec![(uuid, [2.5, 64.0, 3.5], 1.8)]);
        e.vault_connection(&v);
        let n = e.count();
        assert!((2..=5).contains(&n), "{n}");
        // Скважина south: (0.5, 65.75, 1.0); стартовая точка у игрока, последняя — у скважины.
        let level = empty_level();
        let p = e.list.front().unwrap();
        assert_eq!(p.start, [0.5, 65.75, 1.0]);
        assert!((p.x - 2.5).abs() <= 0.5 && (p.z - 3.5).abs() <= 0.5);
        for _ in 0..29 {
            e.tick(&level, [0.0; 3]);
        }
        let p = e.list.front().unwrap();
        let pos = 1.0 - p.age as f64 / p.lifetime as f64;
        assert!((p.x - (0.5 + p.xd * pos)).abs() < 1e-9);
    }

    #[test]
    fn firework_starter_spaces_explosions() {
        let mut e = ParticleEngine::new(sets());
        let level = empty_level();
        let ball = FireworkSpec {
            shape: FireworkShape::SmallBall,
            colors: Vec::new(),
            fade_colors: Vec::new(),
            trail: false,
            twinkle: false,
        };
        e.create_fireworks([0.5, 100.0, 0.5], [0.0; 3], vec![ball.clone(), ball], false);
        e.tick(&level, [0.0; 3]);
        // VANILLA: createParticleBall(0.25, 2): 16 краевых столбцов по 5 + 9 внутренних по 2 = 98, + FLASH.
        assert_eq!(e.count(), 99);
        e.tick(&level, [0.0; 3]);
        assert_eq!(e.count(), 99);
        e.tick(&level, [0.0; 3]);
        assert_eq!(e.count(), 198);
        // lifetime = 2 * 2 - 1 = 3: удаляется, когда life > 3 (4-й тик).
        assert_eq!(e.starters.len(), 1);
        e.tick(&level, [0.0; 3]);
        assert!(e.starters.is_empty());
        // Без взрывов — только POOF (2..4), Starter не создаётся.
        let mut e = ParticleEngine::new(sets());
        e.create_fireworks([0.0; 3], [0.0; 3], Vec::new(), false);
        assert!(e.starters.is_empty());
    }

    fn empty_level() -> Level {
        Level::new(
            ezzmc_world::chunk::ChunkFormat {
                min_y: 0,
                height: 256,
                block_global_bits: 15,
                biome_global_bits: 6,
            },
            2,
        )
    }

    #[test]
    fn rain_drop_lives_and_dies() {
        let mut e = ParticleEngine::new(sets());
        e.add_rain(0.5, 70.0, 0.5);
        assert_eq!(e.count(), 1);
        let level = empty_level();
        for _ in 0..200 {
            e.tick(&level, [0.0; 3]);
        }
        assert_eq!(e.count(), 0);
    }

    #[test]
    fn terrain_particles_fall_with_gravity() {
        let mut e = ParticleEngine::new(sets());
        let look = TerrainLook {
            sprite: SpriteUv {
                u0: 0.0,
                v0: 0.0,
                u1: 1.0,
                v1: 1.0,
            },
            tint: [1.0; 3],
        };
        e.add_terrain(0.0, 100.0, 0.0, 0.0, 0.0, 0.0, look);
        let level = empty_level();
        let yd0 = e.list[0].yd;
        // lifetime осколка ≥ 4 тиков — 3 тика гарантированно живы
        for _ in 0..3 {
            e.tick(&level, [0.0; 3]);
        }
        // гравитация 1.0 → vy падает на 0.04/тик (с трением 0.98)
        assert!(e.list[0].yd < yd0 - 0.05);
        let q = e.extract(&level, [0.0, 100.0, 0.0], 0.5);
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].atlas, ParticleAtlas::Blocks);
        // 0.6 * 255 = 153
        assert_eq!(q[0].color[0], 153);
    }

    #[test]
    fn collision_axis_stops_on_floor() {
        // пол: бокс [0,0,0]-[1,1,1]; частица 0.2×0.2 над ним падает на 1.0
        let floor = [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];
        let bb = [0.4, 1.5, 0.4, 0.6, 1.7, 0.6];
        let d = collide_axis(1, &bb, &floor, -1.0);
        assert!((d - -0.5).abs() < 1e-9);
        // вне по X — не мешает
        let bb2 = [2.0, 1.5, 0.4, 2.2, 1.7, 0.6];
        assert_eq!(collide_axis(1, &bb2, &floor, -1.0), -1.0);
    }

    #[test]
    fn shape_height_lookup() {
        let slab = [[0.0, 0.0, 0.0, 1.0, 0.5, 1.0]];
        assert_eq!(shape_max_y(&slab, 0.3, 0.3), 0.5);
        assert_eq!(shape_max_y(&[], 0.3, 0.3), 0.0);
    }
}
