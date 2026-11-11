//! Колокол: тело — блок-сущность (каркас рисует мешер, тело — здесь).
//! VANILLA: BellRenderer#submit/extractRenderState, BellModel (createBodyLayer, setupAnim),
//! BellBlockEntity#triggerEvent (событие 1: направление удара) и tick (качание до 50 тиков).

use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::collections::HashMap;

/// Текстура под `textures/entity/`.
const TEXTURE: &str = "bell/bell_body";
/// Окно скана в колонках (как у сундуков: getViewDistance = 64).
const SCAN_RADIUS: i32 = 5;

fn is_bell_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| s.name() == "bell"))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

/// VANILLA: BellModel.createBodyLayer.
pub fn model() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let body = m.add(
        "bell_body",
        None,
        Pose::offset(8.0, 12.0, 8.0),
        &[BoxDef::new([0.0, 0.0], [-3.0, -6.0, -3.0], [6.0, 7.0, 6.0])],
    );
    m.add(
        "bell_base",
        Some(body),
        Pose::offset(-8.0, -12.0, -8.0),
        &[BoxDef::new([0.0, 13.0], [4.0, 4.0, 4.0], [8.0, 2.0, 8.0])],
    );
    m
}

/// VANILLA: BellModel#setupAnim — (xRot, zRot) по направлению удара (3D data value).
fn shake(ticks: f32, dir: i32) -> (f32, f32) {
    let base = (ticks / std::f32::consts::PI).sin() / (4.0 + ticks / 3.0);
    match dir {
        2 => (-base, 0.0), // NORTH
        3 => (base, 0.0),  // SOUTH
        5 => (0.0, -base), // EAST
        4 => (0.0, base),  // WEST
        _ => (0.0, 0.0),
    }
}

/// Состояние качания блок-сущности (`ticks`, `clickDirection`).
#[derive(Clone, Copy)]
struct Swing {
    ticks: i32,
    dir: i32,
}

pub struct Bells {
    model: Model,
    list: Vec<[i32; 3]>,
    swings: HashMap<[i32; 3], Swing>,
    last_tick: Option<i64>,
    scan_block_version: Option<u64>,
    scan_version: Option<u64>,
    scan_cam_chunk: Option<(i32, i32)>,
    scan_tick: i64,
}

impl Default for Bells {
    fn default() -> Self {
        Bells {
            model: model(),
            list: Vec::new(),
            swings: HashMap::new(),
            last_tick: None,
            scan_block_version: None,
            scan_version: None,
            scan_cam_chunk: None,
            scan_tick: 0,
        }
    }
}

impl Bells {
    /// VANILLA: BellBlockEntity#triggerEvent (b0 == 1): направление, ticks = 0, shaking.
    pub fn block_event(&mut self, pos: [i32; 3], b0: i32, b1: i32) {
        if b0 == 1 {
            self.swings.insert(pos, Swing { ticks: 0, dir: b1 });
        }
    }

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
        }
        // VANILLA: BellBlockEntity#tick — ticks++ пока shaking; с 50 качание кончается.
        for _ in 0..ticks {
            for s in self.swings.values_mut() {
                s.ticks += 1;
            }
        }
        self.swings.retain(|_, s| s.ticks < 50);
        let pt = level.partial_tick();
        for &p in &self.list {
            let d = |i: usize| f64::from(p[i]) + 0.5 - cam[i];
            if d(0) * d(0) + d(1) * d(1) + d(2) * d(2) > 64.0 * 64.0 {
                continue;
            }
            let (x_rot, z_rot) = self
                .swings
                .get(&p)
                .map_or((0.0, 0.0), |s| shake(s.ticks as f32 + pt, s.dir));
            let m = &mut self.model;
            m.reset_pose();
            if let Some(i) = m.index("bell_body") {
                m.parts[i].pose.x_rot = x_rot;
                m.parts[i].pose.z_rot = z_rot;
            }
            let root = Mat4::from_translation(Vec3::new(
                (f64::from(p[0]) - cam[0]) as f32,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32,
            ));
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
                if sec.has_only_air() || !sec.states().maybe_has(is_bell_id) {
                    continue;
                }
                let y0 = chunk.min_y() + si as i32 * 16;
                for y in 0..16usize {
                    for z in 0..16usize {
                        for x in 0..16usize {
                            if is_bell_id(sec.block(x, y, z).id()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shake_axes() {
        let (x, z) = shake(5.0, 3);
        assert!(x > 0.0 && z == 0.0);
        let (x, z) = shake(5.0, 5);
        assert!(x == 0.0 && z < 0.0);
        assert_eq!(shake(5.0, 1), (0.0, 0.0));
    }

    #[test]
    fn model_has_two_parts() {
        let m = model();
        assert!(m.index("bell_body").is_some() && m.index("bell_base").is_some());
    }
}
