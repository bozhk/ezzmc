//! Книга над столом зачарований в мире (блок-сущность).
//! VANILLA: EnchantingTableBlockEntity#bookAnimationTick, EnchantTableRenderer#extractRenderState/#submit (26.3).
//! VANILLA-UNSURE: «ближайший игрок» — только локальный; ноги = глаза − 1.62.

use crate::enchbook::{book_model, setup_anim};
use ezzmc_render::entitymodel::Model;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::f32::consts::PI;

/// Текстура под `textures/entity/`.
const TEXTURE: &str = "enchantment/enchanting_table_book";
const SCAN_RADIUS: i32 = 5;

fn is_table_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| s.name() == "enchanting_table"))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

/// Поля EnchantingTableBlockEntity.
#[derive(Clone, Copy, Default)]
struct Book {
    time: i32,
    flip: f32,
    o_flip: f32,
    flip_t: f32,
    flip_a: f32,
    open: f32,
    o_open: f32,
    rot: f32,
    o_rot: f32,
    t_rot: f32,
}

fn wrap(mut a: f32) -> f32 {
    while a >= PI {
        a -= 2.0 * PI;
    }
    while a < -PI {
        a += 2.0 * PI;
    }
    a
}

impl Book {
    /// VANILLA: EnchantingTableBlockEntity#bookAnimationTick; `near` — (xd, zd) до игрока в 3 блоках.
    fn tick(&mut self, near: Option<(f64, f64)>, rng: &mut u64) {
        let mut rnd = |n: u32| {
            *rng ^= *rng << 13;
            *rng ^= *rng >> 7;
            *rng ^= *rng << 17;
            ((*rng >> 33) as u32 % n) as i32
        };
        self.o_open = self.open;
        self.o_rot = self.rot;
        if let Some((xd, zd)) = near {
            self.t_rot = zd.atan2(xd) as f32;
            self.open += 0.1;
            if self.open < 0.5 || rnd(40) == 0 {
                let old = self.flip_t;
                while old == self.flip_t {
                    self.flip_t += (rnd(4) - rnd(4)) as f32;
                }
            }
        } else {
            self.t_rot += 0.02;
            self.open -= 0.1;
        }
        self.rot = wrap(self.rot);
        self.t_rot = wrap(self.t_rot);
        let dir = wrap(self.t_rot - self.rot);
        self.rot += dir * 0.4;
        self.open = self.open.clamp(0.0, 1.0);
        self.time += 1;
        self.o_flip = self.flip;
        let diff = ((self.flip_t - self.flip) * 0.4).clamp(-0.2, 0.2);
        self.flip_a += (diff - self.flip_a) * 0.9;
        self.flip += self.flip_a;
    }
}

pub struct Tables {
    model: Model,
    list: Vec<[i32; 3]>,
    books: HashMap<[i32; 3], Book>,
    last_tick: Option<i64>,
    scan_block_version: Option<u64>,
    scan_version: Option<u64>,
    scan_cam_chunk: Option<(i32, i32)>,
    scan_tick: i64,
    rng: u64,
}

impl Default for Tables {
    fn default() -> Self {
        Tables {
            model: book_model(),
            list: Vec::new(),
            books: HashMap::new(),
            last_tick: None,
            scan_block_version: None,
            scan_version: None,
            scan_cam_chunk: None,
            scan_tick: 0,
            rng: 0x9E37_79B9_7F4A_7C15,
        }
    }
}

impl Tables {
    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        (world_version, block_version): (u64, u64),
        mobs: &mut crate::mobs::Mobs,
    ) {
        let now = level.game_time;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            Some(_) => 0,
            None => 1,
        };
        let cam_chunk = (
            (cam[0] / 16.0).floor() as i32,
            (cam[2] / 16.0).floor() as i32,
        );
        let block_changed = self.scan_block_version != Some(block_version);
        let world_changed = (self.scan_version != Some(world_version)
            && now - self.scan_tick >= 10)
            || self.scan_cam_chunk != Some(cam_chunk);
        let safety = now - self.scan_tick >= 100 || now < self.scan_tick;
        if block_changed || world_changed || safety {
            self.scan_version = Some(world_version);
            self.scan_block_version = Some(block_version);
            self.scan_tick = now;
            self.scan_cam_chunk = Some(cam_chunk);
            self.rescan(level, cam_chunk);
            let list = &self.list;
            self.books.retain(|p, _| list.contains(p));
        }
        let feet = [cam[0], cam[1] - 1.62, cam[2]];
        for &p in &self.list {
            let c = [
                f64::from(p[0]) + 0.5,
                f64::from(p[1]) + 0.5,
                f64::from(p[2]) + 0.5,
            ];
            // VANILLA: EnchantingTableBlockEntity#bookAnimationTick — Level#getNearestPlayer(x+.5,y+.5,z+.5, 3.0):
            // ближайший из ВСЕХ игроков (локальный + чужие), не только локальный.
            let pd = |q: [f64; 3]| {
                let (xd, yd, zd) = (q[0] - c[0], q[1] - c[1], q[2] - c[2]);
                (xd * xd + yd * yd + zd * zd, xd, zd)
            };
            let mut best = pd(feet);
            if let Some(pt) = ezzmc_world::entities::player_type() {
                for e in level.entities.iter().filter(|e| e.ty == pt) {
                    let o = pd(e.pos);
                    if o.0 < best.0 {
                        best = o;
                    }
                }
            }
            let near = (best.0 < 9.0).then_some((best.1, best.2));
            let b = self.books.entry(p).or_default();
            for _ in 0..ticks {
                b.tick(near, &mut self.rng);
            }
        }
        let pt = level.partial_tick();
        for &p in &self.list {
            let d = |i: usize| f64::from(p[i]) + 0.5 - cam[i];
            if d(0) * d(0) + d(1) * d(1) + d(2) * d(2) > 64.0 * 64.0 {
                continue;
            }
            let Some(b) = self.books.get(&p) else {
                continue;
            };
            // VANILLA: EnchantTableRenderer#extractRenderState.
            let lerp = |a: f32, c: f32| a + (c - a) * pt;
            let flip = lerp(b.o_flip, b.flip);
            let open = lerp(b.o_open, b.open);
            let time = b.time as f32 + pt;
            let or = wrap(b.rot - b.o_rot);
            let y_rot = b.o_rot + or * pt;
            // VANILLA: EnchantTableRenderer#submit.
            let frac = |x: f32| x - x.floor();
            let f1 = (frac(flip + 0.25) * 1.6 - 0.3).clamp(0.0, 1.0);
            let f2 = (frac(flip + 0.75) * 1.6 - 0.3).clamp(0.0, 1.0);
            let m = &mut self.model;
            m.reset_pose();
            // BookModel.State#forAnimation
            setup_anim(m, ((time * 0.02).sin() * 0.1 + 1.25) * open, f1, f2);
            let root = Mat4::from_translation(Vec3::new(
                (f64::from(p[0]) - cam[0]) as f32,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32,
            )) * Mat4::from_translation(Vec3::new(0.5, 0.75, 0.5))
                * Mat4::from_translation(Vec3::new(0.0, 0.1 + (time * 0.1).sin() * 0.01, 0.0))
                * Mat4::from_rotation_y(-y_rot)
                * Mat4::from_rotation_z(80.0f32.to_radians());
            let light = crate::particles::light_at(
                level,
                f64::from(p[0]),
                f64::from(p[1]),
                f64::from(p[2]),
            );
            m.bake(root, [255; 4], light, mobs.out(TEXTURE));
        }
    }

    fn rescan(&mut self, level: &Level, (cx, cz): (i32, i32)) {
        self.list.clear();
        for (dx, dz) in (-SCAN_RADIUS..=SCAN_RADIUS)
            .flat_map(|x| (-SCAN_RADIUS..=SCAN_RADIUS).map(move |z| (x, z)))
        {
            let Some(ch) = level.chunks.get(cx + dx, cz + dz) else {
                continue;
            };
            let chunk = &ch.chunk;
            for (si, sec) in chunk.sections().iter().enumerate() {
                if sec.has_only_air() || !sec.states().maybe_has(is_table_id) {
                    continue;
                }
                let y0 = chunk.min_y() + si as i32 * 16;
                for y in 0..16usize {
                    for z in 0..16usize {
                        for x in 0..16usize {
                            if is_table_id(sec.block(x, y, z).id()) {
                                self.list.push([
                                    ch.pos.x * 16 + x as i32,
                                    y0 + y as i32,
                                    ch.pos.z * 16 + z as i32,
                                ]);
                            }
                        }
                    }
                }
            }
        }
    }
}
