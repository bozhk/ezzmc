//! Частицы (TODO-155, часть): ядро `ParticleEngine`, физика `Particle#move`, осколки блоков
//! (`TerrainParticle`), капли дождя (`WaterDropParticle`), дым (`SmokeParticle`), спавн брызг дождя
//! (`ClientLevel#tickWeatherEffects`), разрушение блока (`ClientLevel#addDestroyBlockEffect`),
//! пакеты `level_event` 2001 и `level_particles`. Остальные типы частиц — позже (TODO-155).

use crate::flammable::FLAMMABLE;
use ezzmc_assets::bake::SpriteUv;
use ezzmc_mesh::blockinfo;
use ezzmc_protocol::types::BlockPos;
use ezzmc_render::particledraw::{ParticleAtlas, ParticleQuad};
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::block::{BlockState, FLAG_CAN_OCCLUDE};
use ezzmc_world::chunk_packet::HeightmapType;
use ezzmc_world::generated::p26_3::SHAPES;
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
}

/// VANILLA: DripParticle.* — стадия капли.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    HangWater,
    HangLava,
    FallWater,
    FallLava,
    LandLava,
}

impl Stage {
    fn set_name(self) -> &'static str {
        match self {
            Stage::HangWater => "dripping_water",
            Stage::HangLava => "dripping_lava",
            Stage::FallWater => "falling_water",
            Stage::FallLava => "falling_lava",
            Stage::LandLava => "landing_lava",
        }
    }

    fn is_hang(self) -> bool {
        matches!(self, Stage::HangWater | Stage::HangLava)
    }

    fn lava(self) -> bool {
        matches!(self, Stage::HangLava | Stage::FallLava | Stage::LandLava)
    }
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
}

fn collision_boxes(state: BlockState) -> &'static [[f64; 6]] {
    state
        .props()
        .and_then(|p| SHAPES.get(usize::from(p.collision_shape)).copied())
        .unwrap_or(&[])
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
            Kind::Flame => self.base_tick(level),
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
        if stage == Stage::HangLava {
            let l = self.lifetime as f32;
            self.rgb = [1.0, 16.0 / (40.0 - l + 16.0), 4.0 / (40.0 - l + 8.0)];
        }
        let was = self.lifetime;
        self.lifetime -= 1;
        if was <= 0 {
            self.removed = true;
            if stage.is_hang() {
                let next = if stage == Stage::HangLava {
                    Stage::FallLava
                } else {
                    Stage::FallWater
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
            let next = if stage == Stage::FallLava {
                Some(Stage::LandLava)
            } else {
                None
            };
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
            // VANILLA: HugeExplosionParticle#getLightCoords = 15728880.
            Kind::Huge => (240, 240),
            Kind::Flame => {
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
            Kind::Smoke | Kind::Crit => {
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
        }
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
    pub fn count(&self) -> usize {
        self.list.len()
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
    pub fn tick(&mut self, level: &Level) {
        let mut smokes = Vec::new();
        let mut drips = Vec::new();
        for p in &mut self.list {
            p.tick(level);
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
        if !self.allow() {
            return;
        }
        let Some(set) = self.sets.get(name).cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::Flame, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        p.has_physics = false;
        p.friction = 0.96;
        p.xd *= f64::from(0.01_f32);
        p.yd *= f64::from(0.01_f32);
        p.zd *= f64::from(0.01_f32);
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
                } else if name == "lava" {
                    // VANILLA: LavaFluid#animateTick (звук — после аудио)
                    let above = level.chunks.block(x, y + 1, z);
                    if above.is_air() && self.rng.next_int_bound(100) == 0 {
                        let (lx, lz) = (fx + self.rng.next_double(), fz + self.rng.next_double());
                        self.add_lava(lx, fy + 1.0, lz);
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
                } else if name == "fire" || name == "soul_fire" {
                    self.fire_tick(level, name == "soul_fire", x, y, z);
                } else if (name == "campfire" || name == "soul_campfire")
                    && st.property("lit") == Some("true")
                    && self.rng.next_int_bound(5) == 0
                {
                    // VANILLA: CampfireBlock#animateTick (звук — после аудио; дым — block entity)
                    for _ in 0..(self.rng.next_int_bound(1) + 1) {
                        self.add_lava(fx + 0.5, fy + 0.5, fz + 0.5);
                    }
                }
                // VANILLA: ClientLevel#doAnimateTick — капли жидкости
                if let Some(fl) = blockinfo::fluid(st) {
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
            Stage::HangWater => {
                p.gravity *= 0.02;
                p.lifetime = 40;
                p.rgb = [0.2, 0.3, 1.0];
            }
            Stage::HangLava => {
                p.gravity *= 0.02;
                p.lifetime = 40;
            }
            Stage::FallWater => {
                p.lifetime = (64.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
                p.rgb = [0.2, 0.3, 1.0];
            }
            Stage::FallLava => {
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
            let lerp = |a: f64, b: f64| a + f64::from(partial) * (b - a);
            let (u0, u1, v0, v1) = p.uv();
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
                color: [col(p.rgb[0]), col(p.rgb[1]), col(p.rgb[2]), col(p.alpha)],
                light: p.light(level, partial),
            });
        }
        self.last_extracted = out.len();
        out
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
        s
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
            e.tick(&level);
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
            e.tick(&level);
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
