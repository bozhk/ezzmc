//! Меш жидкостей (TODO-145): порт `LiquidBlockRenderer#tesselate` (1.21.11).
use crate::blockinfo::{self, Fluid, SOLID_RENDER};
use crate::cull::{fluid_face_occluded, offset, opposite};
use crate::lighting::get_shade;
use crate::padded::Padded;
use crate::vertex::PackedVertex;
use ezzmc_assets::bake::SpriteUv;
use ezzmc_assets::model::Direction;
use ezzmc_world::block::{BlockState, FLAG_BLOCKS_MOTION, FLAG_SOLID};

/// Спрайты жидкостей (`ModelBakery.WATER_STILL…`).
#[derive(Clone, Copy, Debug)]
pub struct FluidSprites {
    pub water_still: SpriteUv,
    pub water_flow: SpriteUv,
    pub water_overlay: SpriteUv,
    pub lava_still: SpriteUv,
    pub lava_flow: SpriteUv,
}

const MAX_FLUID_HEIGHT: f32 = 0.888_888_9;
const EPS: f32 = 0.001;

type P = [i32; 3];

fn at(p: P, d: Direction) -> P {
    let o = offset(d);
    [p[0] + o[0], p[1] + o[1], p[2] + o[2]]
}

fn add3(p: P, x: i32, y: i32, z: i32) -> P {
    [p[0] + x, p[1] + y, p[2] + z]
}

/// Источник блоков для `flow` (секция с паддингом или весь мир).
pub trait BlockGet {
    fn block(&self, p: P) -> BlockState;
}

impl BlockGet for Padded {
    fn block(&self, p: P) -> BlockState {
        Padded::block(self, p)
    }
}

/// Мир по замыканию (физика клиента).
pub struct FnBlocks<F: Fn(P) -> BlockState>(pub F);

impl<F: Fn(P) -> BlockState> BlockGet for FnBlocks<F> {
    fn block(&self, p: P) -> BlockState {
        (self.0)(p)
    }
}

fn fl<G: BlockGet + ?Sized>(pad: &G, p: P) -> Option<Fluid> {
    blockinfo::fluid(pad.block(p))
}

fn is_solid(state: BlockState) -> bool {
    state.props().is_some_and(|p| p.has(FLAG_SOLID))
}

/// VANILLA: LiquidBlockRenderer#getHeight(level, fluid, pos, state, fluidState)
fn height(pad: &Padded, f: &Fluid, p: P) -> f32 {
    let st = pad.block(p);
    let fs = blockinfo::fluid(st);
    if f.same(fs) {
        if f.same(fl(pad, at(p, Direction::Up))) {
            1.0
        } else {
            fs.map_or(0.0, |x| x.own_height())
        }
    } else if !is_solid(st) {
        0.0
    } else {
        -1.0
    }
}

fn add_weighted(acc: &mut [f32; 2], h: f32) {
    if h >= 0.8 {
        acc[0] += h * 10.0;
        acc[1] += 10.0;
    } else if h >= 0.0 {
        acc[0] += h;
        acc[1] += 1.0;
    }
}

/// VANILLA: LiquidBlockRenderer#calculateAverageHeight
fn average_height(pad: &Padded, f: &Fluid, own: f32, a: f32, b: f32, diag: P) -> f32 {
    if b >= 1.0 || a >= 1.0 {
        return 1.0;
    }
    let mut acc = [0.0f32; 2];
    if b > 0.0 || a > 0.0 {
        let d = height(pad, f, diag);
        if d >= 1.0 {
            return 1.0;
        }
        add_weighted(&mut acc, d);
    }
    add_weighted(&mut acc, own);
    add_weighted(&mut acc, b);
    add_weighted(&mut acc, a);
    acc[0] / acc[1]
}

/// Грань блока «прочная» (`isFaceSturdy(FULL)`): приближённо — полный куб коллизии/окклюзии.
// VANILLA-UNSURE: getBlockSupportShape не снят в данных; полный куб достаточен для воды у стен.
fn sturdy(state: BlockState) -> bool {
    blockinfo::has(state, SOLID_RENDER | blockinfo::COLLISION_FULL)
}

/// VANILLA: FlowingFluid#isSolidFace
fn is_solid_face<G: BlockGet + ?Sized>(pad: &G, f: &Fluid, p: P, d: Direction) -> bool {
    let st = pad.block(p);
    if f.same(blockinfo::fluid(st)) {
        false
    } else if d == Direction::Up {
        true
    } else if matches!(st.name(), "ice" | "frosted_ice") {
        false
    } else {
        sturdy(st)
    }
}

const HORIZONTAL: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

/// VANILLA: FlowingFluid#getFlow → (x, z), нормализован.
pub fn flow<G: BlockGet + ?Sized>(pad: &G, p: P, f: &Fluid) -> (f64, f64) {
    let v = flow3(pad, p, f);
    (v[0], v[2])
}

/// VANILLA: FlowingFluid#getFlow → (x, y, z), нормализован (y ≠ 0 у падающей воды у стены).
pub fn flow3<G: BlockGet + ?Sized>(pad: &G, p: P, f: &Fluid) -> [f64; 3] {
    let affects = |o: Option<Fluid>| o.is_none() || f.same(o);
    let (mut fx, mut fz) = (0.0f64, 0.0f64);
    for d in HORIZONTAL {
        let n = at(p, d);
        let ns = fl(pad, n);
        if affects(ns) {
            let mut h = ns.map_or(0.0, |x| x.own_height());
            let mut diff = 0.0f32;
            if h == 0.0 {
                if !pad
                    .block(n)
                    .props()
                    .is_some_and(|q| q.has(FLAG_BLOCKS_MOTION))
                {
                    let below = fl(pad, at(n, Direction::Down));
                    if affects(below) {
                        h = below.map_or(0.0, |x| x.own_height());
                        if h > 0.0 {
                            diff = f.own_height() - (h - MAX_FLUID_HEIGHT);
                        }
                    }
                }
            } else if h > 0.0 {
                diff = f.own_height() - h;
            }
            if diff != 0.0 {
                let o = offset(d);
                fx += f64::from(o[0]) * f64::from(diff);
                fz += f64::from(o[2]) * f64::from(diff);
            }
        }
    }
    let norm = |x: f64, y: f64, z: f64| {
        let l = (x * x + y * y + z * z).sqrt();
        if l < 1.0e-4 {
            (0.0, 0.0, 0.0)
        } else {
            (x / l, y / l, z / l)
        }
    };
    let mut v = (fx, 0.0, fz);
    if f.falling {
        for d in HORIZONTAL {
            let n = at(p, d);
            if is_solid_face(pad, f, n, d) || is_solid_face(pad, f, at(n, Direction::Up), d) {
                let (a, b, c) = norm(v.0, v.1, v.2);
                v = (a, b - 6.0, c);
                break;
            }
        }
    }
    let (x, y, z) = norm(v.0, v.1, v.2);
    [x, y, z]
}

/// VANILLA: LiquidBlockRenderer#shouldRenderFace
fn should_render_face(state: BlockState, f: &Fluid, d: Direction, neighbor: Option<Fluid>) -> bool {
    !fluid_face_occluded(opposite(d), 1.0, state) && !f.same(neighbor)
}

/// VANILLA: FluidState#shouldRenderBackwardUpFace
fn backward_up(pad: &Padded, f: &Fluid, p: P) -> bool {
    for dx in -1..=1 {
        for dz in -1..=1 {
            let q = add3(p, dx, 0, dz);
            if !f.same(fl(pad, q)) && !blockinfo::has(pad.block(q), SOLID_RENDER) {
                return true;
            }
        }
    }
    false
}

/// VANILLA: LiquidBlockRenderer#getLightColor — максимум света клетки и клетки над ней.
fn light(pad: &Padded, p: P) -> u32 {
    let up = at(p, Direction::Up);
    let a = pad.light_color(pad.block(p), p);
    let b = pad.light_color(pad.block(up), up);
    (a & 0xFF).max(b & 0xFF) | ((a >> 16 & 0xFF).max(b >> 16 & 0xFF)) << 16
}

pub struct FluidCtx<'a> {
    pub pad: &'a Padded,
    pub sprites: &'a FluidSprites,
    pub cardinal: &'a [f32; 6],
    /// Тинт воды (0xRRGGBB); у лавы не используется.
    pub water_tint: [u8; 3],
}

/// Один квад жидкости — 4 вершины; `leaf_or_glass` определяет overlay-спрайт.
pub fn mesh_fluid(
    cx: &FluidCtx,
    p: P,
    state: BlockState,
    f: &Fluid,
    out: &mut Vec<[PackedVertex; 4]>,
) {
    let pad = cx.pad;
    let lava = f.lava;
    let (still, flowing) = if lava {
        (cx.sprites.lava_still, cx.sprites.lava_flow)
    } else {
        (cx.sprites.water_still, cx.sprites.water_flow)
    };
    let tint = if lava {
        [1.0f32; 3]
    } else {
        cx.water_tint.map(|c| f32::from(c) / 255.0)
    };
    let nb = |d: Direction| {
        let q = at(p, d);
        (q, pad.block(q), blockinfo::fluid(pad.block(q)))
    };
    let (pd, sd, fd) = nb(Direction::Down);
    let (pu, su, fu) = nb(Direction::Up);
    let (pn, sn, fn_) = nb(Direction::North);
    let (ps, ss, fs) = nb(Direction::South);
    let (pw, sw, fw) = nb(Direction::West);
    let (pe, se, fe) = nb(Direction::East);
    let _ = (pd, pu);
    let up_visible = !f.same(fu);
    let down_visible = should_render_face(state, f, Direction::Down, fd)
        && !fluid_face_occluded(Direction::Down, MAX_FLUID_HEIGHT, sd);
    let north = should_render_face(state, f, Direction::North, fn_);
    let south = should_render_face(state, f, Direction::South, fs);
    let west = should_render_face(state, f, Direction::West, fw);
    let east = should_render_face(state, f, Direction::East, fe);
    if !(up_visible || down_visible || east || west || north || south) {
        return;
    }
    let shade_down = get_shade(cx.cardinal, Direction::Down, None);
    let shade_up = get_shade(cx.cardinal, Direction::Up, None);
    let shade_z = get_shade(cx.cardinal, Direction::North, None);
    let shade_x = get_shade(cx.cardinal, Direction::West, None);
    let own = height(pad, f, p);
    let (mut h_ne, mut h_nw, mut h_se, mut h_sw);
    if own >= 1.0 {
        h_ne = 1.0;
        h_nw = 1.0;
        h_se = 1.0;
        h_sw = 1.0;
    } else {
        let hn = height(pad, f, pn);
        let hs = height(pad, f, ps);
        let he = height(pad, f, pe);
        let hw = height(pad, f, pw);
        h_ne = average_height(pad, f, own, hn, he, at(pn, Direction::East));
        h_nw = average_height(pad, f, own, hn, hw, at(pn, Direction::West));
        h_se = average_height(pad, f, own, hs, he, at(ps, Direction::East));
        h_sw = average_height(pad, f, own, hs, hw, at(ps, Direction::West));
    }
    let (x, y, z) = (p[0] as f32, p[1] as f32, p[2] as f32);
    let bottom_eps = if down_visible { EPS } else { 0.0 };
    let color = |k: f32| -> [u8; 4] {
        [
            (k * tint[0] * 255.0) as u8,
            (k * tint[1] * 255.0) as u8,
            (k * tint[2] * 255.0) as u8,
            255,
        ]
    };
    let quad = |v: [([f32; 3], [f32; 2]); 4], c: [u8; 4], l: u32| -> [PackedVertex; 4] {
        std::array::from_fn(|i| {
            PackedVertex::pack(
                v[i].0,
                v[i].1,
                c,
                (l & 0xFF) as u8,
                ((l >> 16) & 0xFF) as u8,
            )
        })
    };

    if up_visible && !fluid_face_occluded(Direction::Up, h_nw.min(h_sw).min(h_se.min(h_ne)), su) {
        h_nw -= EPS;
        h_sw -= EPS;
        h_se -= EPS;
        h_ne -= EPS;
        let (fx, fz) = flow(pad, p, f);
        let (u0, v0, u1, v1, u2, v2, u3, v3);
        if fx == 0.0 && fz == 0.0 {
            u0 = still.get_u(0.0);
            v0 = still.get_v(0.0);
            u1 = u0;
            v1 = still.get_v(1.0);
            u2 = still.get_u(1.0);
            v2 = v1;
            u3 = u2;
            v3 = v0;
        } else {
            let ang = fz.atan2(fx) as f32 - std::f32::consts::FRAC_PI_2;
            let s = ang.sin() * 0.25;
            let c = ang.cos() * 0.25;
            u0 = flowing.get_u(0.5 + (-c - s));
            v0 = flowing.get_v(0.5 + (-c + s));
            u1 = flowing.get_u(0.5 + (-c + s));
            v1 = flowing.get_v(0.5 + (c + s));
            u2 = flowing.get_u(0.5 + (c + s));
            v2 = flowing.get_v(0.5 + (c - s));
            u3 = flowing.get_u(0.5 + (c - s));
            v3 = flowing.get_v(0.5 + (-c - s));
        }
        let l = light(pad, p);
        let col = color(shade_up);
        out.push(quad(
            [
                ([x, y + h_nw, z], [u0, v0]),
                ([x, y + h_sw, z + 1.0], [u1, v1]),
                ([x + 1.0, y + h_se, z + 1.0], [u2, v2]),
                ([x + 1.0, y + h_ne, z], [u3, v3]),
            ],
            col,
            l,
        ));
        if backward_up(pad, f, at(p, Direction::Up)) {
            out.push(quad(
                [
                    ([x, y + h_nw, z], [u0, v0]),
                    ([x + 1.0, y + h_ne, z], [u3, v3]),
                    ([x + 1.0, y + h_se, z + 1.0], [u2, v2]),
                    ([x, y + h_sw, z + 1.0], [u1, v1]),
                ],
                col,
                l,
            ));
        }
    }

    if down_visible {
        let l = light(pad, at(p, Direction::Down));
        let col = color(shade_down);
        let (ua, ub, va, vb) = (still.u0, still.u1, still.v0, still.v1);
        out.push(quad(
            [
                ([x, y + bottom_eps, z + 1.0], [ua, vb]),
                ([x, y + bottom_eps, z], [ua, va]),
                ([x + 1.0, y + bottom_eps, z], [ub, va]),
                ([x + 1.0, y + bottom_eps, z + 1.0], [ub, vb]),
            ],
            col,
            l,
        ));
    }

    let l = light(pad, p);
    for d in HORIZONTAL {
        let (h0, h1, x0, x1, z0, z1, vis, nstate) = match d {
            Direction::North => (h_nw, h_ne, x, x + 1.0, z + EPS, z + EPS, north, sn),
            Direction::South => (
                h_se,
                h_sw,
                x + 1.0,
                x,
                z + 1.0 - EPS,
                z + 1.0 - EPS,
                south,
                ss,
            ),
            Direction::West => (h_sw, h_nw, x + EPS, x + EPS, z + 1.0, z, west, sw),
            _ => (
                h_ne,
                h_se,
                x + 1.0 - EPS,
                x + 1.0 - EPS,
                z,
                z + 1.0,
                east,
                se,
            ),
        };
        if !vis || fluid_face_occluded(d, h0.max(h1), nstate) {
            continue;
        }
        let mut sprite = flowing;
        let mut overlay = false;
        if !lava && is_half_transparent_or_leaves(nstate) {
            sprite = cx.sprites.water_overlay;
            overlay = true;
        }
        let u_a = sprite.get_u(0.0);
        let u_b = sprite.get_u(0.5);
        let v_a = sprite.get_v((1.0 - h0) * 0.5);
        let v_b = sprite.get_v((1.0 - h1) * 0.5);
        let v_c = sprite.get_v(0.5);
        let k = shade_up
            * if matches!(d, Direction::North | Direction::South) {
                shade_z
            } else {
                shade_x
            };
        let col = color(k);
        out.push(quad(
            [
                ([x0, y + h0, z0], [u_a, v_a]),
                ([x1, y + h1, z1], [u_b, v_b]),
                ([x1, y + bottom_eps, z1], [u_b, v_c]),
                ([x0, y + bottom_eps, z0], [u_a, v_c]),
            ],
            col,
            l,
        ));
        if !overlay {
            out.push(quad(
                [
                    ([x0, y + bottom_eps, z0], [u_a, v_c]),
                    ([x1, y + bottom_eps, z1], [u_b, v_c]),
                    ([x1, y + h1, z1], [u_b, v_b]),
                    ([x0, y + h0, z0], [u_a, v_a]),
                ],
                col,
                l,
            ));
        }
    }
}

/// `HalfTransparentBlock` (стекло, лёд, слизь, мёд, tinted) или `LeavesBlock`.
// VANILLA: instanceof HalfTransparentBlock || LeavesBlock
fn is_half_transparent_or_leaves(s: BlockState) -> bool {
    let n = s.name();
    matches!(
        n,
        "glass"
            | "tinted_glass"
            | "ice"
            | "frosted_ice"
            | "packed_ice"
            | "blue_ice"
            | "slime_block"
            | "honey_block"
    ) || n.ends_with("_stained_glass")
        || n.ends_with("_leaves")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_world::generated::b26_3::BLOCKS;

    fn st(name: &str) -> BlockState {
        let b = BLOCKS.iter().find(|b| b.name == name).unwrap();
        BlockState::from_id(b.default_state).unwrap()
    }

    fn sprite() -> SpriteUv {
        SpriteUv {
            u0: 0.0,
            v0: 0.0,
            u1: 1.0,
            v1: 1.0,
        }
    }

    fn pad_with(f: impl Fn(i32, i32, i32) -> BlockState) -> Padded {
        Padded::from_fn(f)
    }

    #[test]
    fn still_water_surface_and_sides() {
        let water = st("water");
        let stone = st("stone");
        // вода в (8,8,8) на камне, окружена воздухом: верх + 4 стороны
        let pad = pad_with(|x, y, z| {
            if (x, y, z) == (8, 8, 8) {
                water
            } else if y < 8 {
                stone
            } else {
                BlockState::AIR
            }
        });
        let sp = FluidSprites {
            water_still: sprite(),
            water_flow: sprite(),
            water_overlay: sprite(),
            lava_still: sprite(),
            lava_flow: sprite(),
        };
        let cx = FluidCtx {
            pad: &pad,
            sprites: &sp,
            cardinal: &crate::lighting::CARDINAL_DEFAULT,
            water_tint: [255; 3],
        };
        let mut out = Vec::new();
        mesh_fluid(&cx, [8, 8, 8], water, &fluid_of(water), &mut out);
        // верх (+обратная сторона) и 4 боковых грани ×2 (двусторонние); низ скрыт камнем
        assert_eq!(out.len(), 2 + 8);
        // угол одиноко стоящего источника: (8/9·10)/12 = 0.7407, минус 0.001
        let top = out[0][0].position()[1] - 8.0;
        assert!((top - (0.740_740_7 - 0.001)).abs() < 1e-3, "{top}");
        // стоящая вода: поток нулевой
        assert_eq!(flow(&pad, [8, 8, 8], &fluid_of(water)), (0.0, 0.0));
    }

    fn fluid_of(s: BlockState) -> Fluid {
        blockinfo::fluid(s).unwrap()
    }

    #[test]
    fn same_fluid_neighbors_cull_and_flow_points_downhill() {
        let water = st("water");
        let low = water.with("level", "4").unwrap();
        // источник и течение уровнем 4 справа (x+1): течёт на восток
        let pad = pad_with(|x, y, z| match (x, y, z) {
            (8, 8, 8) => water,
            (9, 8, 8) => low,
            _ if y < 8 => st("stone"),
            _ => BlockState::AIR,
        });
        let (fx, fz) = flow(&pad, [8, 8, 8], &fluid_of(water));
        assert!(fx > 0.9 && fz.abs() < 1e-9, "{fx} {fz}");
    }
}
