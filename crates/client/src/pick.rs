//! Луч взгляда: ближайший блок по контуру (OUTLINE) или жидкость. Нужен F3 и контуру выбора.
//!
//! VANILLA: Entity#pick(20, 0, fluid) → ClipContext(Block.OUTLINE, Fluid.NONE|ANY) → BlockGetter#clip.
//! Пересечение луча с боксами формы блока (slab-метод), обход вокселей — DDA (Amanatides–Woo).
use ezzmc_mesh::blockinfo as bi;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;

/// VANILLA: Attributes.BLOCK_INTERACTION_RANGE 4.5 (креатив +0.5 → 5.0).
pub const BLOCK_REACH: f64 = 4.5;
pub const CREATIVE_BLOCK_REACH: f64 = 5.0;

/// Дальность F3 (`DebugEntryLookingAt.RANGE`).
pub const REACH: f64 = 20.0;

#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub pos: [i32; 3],
    pub state: BlockState,
    /// Грань попадания (порядок Direction: 0 down, 1 up, 2 north, 3 south, 4 west, 5 east).
    pub face: u8,
    /// Точка попадания в мировых координатах.
    pub hit: [f64; 3],
    /// Глаза внутри формы блока (BlockHitResult#isInside).
    pub inside: bool,
}

/// Грань, в которую вошёл луч по оси `axis` при направлении `d`.
fn face_of(axis: usize, d: f64) -> u8 {
    // Входим через минимальную грань, если летим в +; иначе через максимальную.
    let positive_dir = d > 0.0;
    match (axis, positive_dir) {
        (0, true) => 4,
        (0, false) => 5,
        (1, true) => 0,
        (1, false) => 1,
        (2, true) => 2,
        _ => 3,
    }
}

const FULL: [[f64; 6]; 1] = [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];

/// Параметр t первого входа луча в бокс (`None` — мимо). Начало внутри бокса даёт 0.
fn clip_box(o: [f64; 3], d: [f64; 3], b: [f64; 6], max_t: f64) -> Option<f64> {
    clip_box_face(o, d, b, max_t).map(|(t, _)| t)
}

/// Как `clip_box`, плюс ось входа (`None` — начало внутри бокса).
fn clip_box_face(
    o: [f64; 3],
    d: [f64; 3],
    b: [f64; 6],
    max_t: f64,
) -> Option<(f64, Option<usize>)> {
    let (mut t0, mut t1) = (0.0f64, max_t);
    let mut axis = None;
    for i in 0..3 {
        if d[i].abs() < 1e-12 {
            if o[i] < b[i] || o[i] > b[i + 3] {
                return None;
            }
        } else {
            let (a, c) = ((b[i] - o[i]) / d[i], (b[i + 3] - o[i]) / d[i]);
            if a.min(c) > t0 {
                t0 = a.min(c);
                axis = Some(i);
            }
            t1 = t1.min(a.max(c));
            if t0 > t1 {
                return None;
            }
        }
    }
    Some((t0, axis))
}

/// `fluid == false`: блоки по `outline_shape` (жидкости пропускаются);
/// `fluid == true`: ещё и жидкости (VANILLA-UNSURE: форма жидкости — целый куб, а не её высота).
pub fn raycast(l: &Level, o: [f64; 3], d: [f32; 3], fluid: bool) -> Option<Target> {
    raycast_t(l, o, d, fluid, REACH).map(|(t, _)| t)
}

/// То же с дальностью `max` и расстоянием до точки попадания (Camera#getMaxZoom).
pub fn raycast_t(
    l: &Level,
    o: [f64; 3],
    d: [f32; 3],
    fluid: bool,
    max: f64,
) -> Option<(Target, f64)> {
    let d = [f64::from(d[0]), f64::from(d[1]), f64::from(d[2])];
    let mut cell = [
        o[0].floor() as i32,
        o[1].floor() as i32,
        o[2].floor() as i32,
    ];
    let mut step = [0i32; 3];
    let mut t_max = [f64::INFINITY; 3];
    let mut t_delta = [f64::INFINITY; 3];
    for i in 0..3 {
        if d[i] > 0.0 {
            step[i] = 1;
            t_delta[i] = 1.0 / d[i];
            t_max[i] = (f64::from(cell[i]) + 1.0 - o[i]) / d[i];
        } else if d[i] < 0.0 {
            step[i] = -1;
            t_delta[i] = -1.0 / d[i];
            t_max[i] = (f64::from(cell[i]) - o[i]) / d[i];
        }
    }
    loop {
        let st = l.chunks.block(cell[0], cell[1], cell[2]);
        if !st.is_air() {
            let boxes: &[[f64; 6]] = if fluid && bi::fluid(st).is_some() {
                &FULL
            } else if bi::fluid(st).is_some() && !fluid {
                &[]
            } else {
                bi::outline_boxes(st)
            };
            let off = bi::block_offset(st, cell[0], cell[2]);
            let rel = [
                o[0] - f64::from(cell[0]) - f64::from(off[0]),
                o[1] - f64::from(cell[1]) - f64::from(off[1]),
                o[2] - f64::from(cell[2]) - f64::from(off[2]),
            ];
            let best = boxes
                .iter()
                .filter_map(|b| clip_box_face(rel, d, *b, max))
                .fold(None::<(f64, Option<usize>)>, |acc, c| match acc {
                    Some(a) if a.0 <= c.0 => Some(a),
                    _ => Some(c),
                });
            if let Some((hit, axis)) = best {
                // Начало внутри формы: грань — против доминирующей оси взгляда.
                let (ax, inside) = match axis {
                    Some(a) => (a, false),
                    None => {
                        let a = (0..3)
                            .max_by(|&i, &j| d[i].abs().total_cmp(&d[j].abs()))
                            .unwrap_or(0);
                        (a, true)
                    }
                };
                let face = if inside {
                    face_of(ax, -d[ax])
                } else {
                    face_of(ax, d[ax])
                };
                return Some((
                    Target {
                        pos: cell,
                        state: st,
                        face,
                        hit: [o[0] + d[0] * hit, o[1] + d[1] * hit, o[2] + d[2] * hit],
                        inside,
                    },
                    hit,
                ));
            }
        }
        let k = if t_max[0] < t_max[1] {
            if t_max[0] < t_max[2] {
                0
            } else {
                2
            }
        } else if t_max[1] < t_max[2] {
            1
        } else {
            2
        };
        if t_max[k] > max || !t_max[k].is_finite() {
            return None;
        }
        cell[k] += step[k];
        t_max[k] += t_delta[k];
    }
}

/// VANILLA: ProjectileUtil#getEntityHitResult (pickRadius 0): ближайшая сущность на луче дальностью `max`.
pub fn raycast_entity(l: &Level, o: [f64; 3], d: [f32; 3], max: f64) -> Option<(i32, f64)> {
    let d = [f64::from(d[0]), f64::from(d[1]), f64::from(d[2])];
    let mut best: Option<(i32, f64)> = None;
    for e in l.entities.map.values() {
        let Some((lo, hi)) = e.aabb() else { continue };
        // Слэб-тест луча с AABB (AABB#clip); начало внутри коробки → расстояние 0.
        let (mut t0, mut t1) = (0.0f64, max);
        let mut hit = true;
        for i in 0..3 {
            if d[i].abs() < 1e-12 {
                if o[i] < lo[i] || o[i] > hi[i] {
                    hit = false;
                    break;
                }
            } else {
                let (a, b) = ((lo[i] - o[i]) / d[i], (hi[i] - o[i]) / d[i]);
                t0 = t0.max(a.min(b));
                t1 = t1.min(a.max(b));
            }
        }
        if hit && t0 <= t1 && best.is_none_or(|(_, bt)| t0 < bt) {
            best = Some((e.id, t0));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_nearest_entity() {
        let mut l = Level::new(
            ezzmc_world::chunk::ChunkFormat {
                min_y: 0,
                height: 256,
                block_global_bits: 15,
                biome_global_bits: 6,
            },
            2,
        );
        let z = ezzmc_world::entities::player_type().unwrap_or(0);
        l.entities.add(ezzmc_world::entities::Entity::new(
            7,
            1,
            z,
            [0.0, 0.0, 5.0],
            0.0,
            0.0,
            0.0,
        ));
        l.entities.add(ezzmc_world::entities::Entity::new(
            8,
            2,
            z,
            [0.0, 0.0, 8.0],
            0.0,
            0.0,
            0.0,
        ));
        let h = raycast_entity(&l, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 3.0);
        assert_eq!(h, None);
        let h = raycast_entity(&l, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 6.0)
            .map(|(i, t)| (i, (t * 10.0).round()));
        assert_eq!(h, Some((7, 47.0)));
    }

    #[test]
    fn faces_by_entry_axis() {
        let b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        // летим в +X → входим через west (4)
        let (t, ax) =
            clip_box_face([-1.0, 0.5, 0.5], [1.0, 0.0, 0.0], b, 10.0).unwrap_or((0.0, None));
        assert_eq!((t, ax), (1.0, Some(0)));
        assert_eq!(face_of(0, 1.0), 4);
        assert_eq!(face_of(1, -1.0), 1); // падаем вниз → верхняя грань
                                         // начало внутри
        let r = clip_box_face([0.5; 3], [1.0, 0.0, 0.0], b, 10.0);
        assert_eq!(r.map(|x| x.1), Some(None));
    }
}
