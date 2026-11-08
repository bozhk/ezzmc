//! Частицы (TODO-155, часть): ядро `ParticleEngine`, физика `Particle#move`, осколки блоков
//! (`TerrainParticle`), капли дождя (`WaterDropParticle`), дым (`SmokeParticle`), спавн брызг дождя
//! (`ClientLevel#tickWeatherEffects`), разрушение блока (`ClientLevel#addDestroyBlockEffect`),
//! пакеты `level_event` 2001 и `level_particles`. Остальные типы частиц — позже (TODO-155).

use ezzmc_assets::bake::SpriteUv;
use ezzmc_mesh::blockinfo;
use ezzmc_protocol::types::BlockPos;
use ezzmc_render::particledraw::{ParticleAtlas, ParticleQuad};
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::block::BlockState;
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
            Kind::WaterDrop => self.water_drop_tick(level),
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

    fn quad_size(&self, partial: f32) -> f32 {
        match self.kind {
            Kind::Smoke => {
                self.quad_size
                    * ((self.age as f32 + partial) / self.lifetime as f32 * 32.0).clamp(0.0, 1.0)
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
}

impl ParticleEngine {
    pub fn new(sets: SpriteSets) -> Self {
        ParticleEngine {
            list: VecDeque::new(),
            sets,
            rng: LegacyRandom::new(0x5EED_0001),
            last_extracted: 0,
        }
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
        for p in &mut self.list {
            p.tick(level);
        }
        self.list.retain(|p| !p.removed);
    }

    /// VANILLA: WaterDropParticle.Provider (ParticleTypes.RAIN).
    pub fn add_rain(&mut self, x: f64, y: f64, z: f64) {
        let Some(set) = self.sets.get("rain").cloned() else {
            return;
        };
        let seed = self.seed();
        let mut p = Particle::with_speed(Kind::WaterDrop, x, y, z, 0.0, 0.0, 0.0, seed);
        p.sprite = set[p.rng.next_int_bound(set.len() as i32) as usize];
        // WaterDropParticle#<init>
        p.xd *= f64::from(0.3_f32);
        p.yd = f64::from(p.rng.next_float() * 0.2 + 0.1);
        p.zd *= f64::from(0.3_f32);
        p.set_size(0.01, 0.01);
        p.gravity = 0.06;
        p.lifetime = (8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32;
        self.add(p);
    }

    /// VANILLA: SmokeParticle.Provider (scale 1), BaseAshSmokeParticle#<init>.
    pub fn add_smoke(&mut self, x: f64, y: f64, z: f64, xa: f64, ya: f64, za: f64) {
        let Some(set) = self.sets.get("smoke").cloned() else {
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
        let col = p.rng.next_float() * 0.3;
        p.rgb = [col; 3];
        p.quad_size *= 0.75;
        p.lifetime = ((8.0 / (f64::from(p.rng.next_float()) * 0.8 + 0.2)) as i32).max(1);
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
                "block" => {
                    if let Some(st) = p.block.and_then(BlockState::from_id) {
                        if spawns_terrain_particles(st) {
                            let pos = BlockPos {
                                x: x.floor() as i32,
                                y: y.floor() as i32,
                                z: z.floor() as i32,
                            };
                            if let Some(l) = look(st, pos) {
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
                light: light_at(level, p.x, p.y, p.z),
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
