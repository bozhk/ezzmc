//! Освещение квадов: smooth lighting (AO) и плоский свет.
//! VANILLA: ModelBlockRenderer#tesselateWithAO / tesselateWithoutAO / calculateShape /
//! AmbientOcclusionRenderStorage#calculate / AmbientVertexRemap / blend (26.3).
use crate::ao_table::ADJACENCY;
use crate::blockinfo::{self, COLLISION_FULL, SOLID_RENDER};
use crate::cull::offset;
use crate::padded::{light_block, light_sky, pack_light, Padded};
use ezzmc_assets::bake::BakedQuad;
use ezzmc_assets::model::Direction;
use ezzmc_world::block::BlockState;

/// Результат по четырём вершинам квада.
#[derive(Clone, Copy, Debug, Default)]
pub struct QuadLight {
    pub brightness: [f32; 4],
    /// Ванильный packed `block << 4 | sky << 20`.
    pub lightmap: [u32; 4],
}

/// Форма квада в блоке (`CommonRenderStorage` + `faceShape`).
#[derive(Clone, Copy, Debug)]
pub struct FaceInfo {
    /// `SizeInfo`: DOWN,UP,NORTH,SOUTH,WEST,EAST, затем FLIP_* (= 1 − значение).
    pub face_shape: [f32; 12],
    pub partial: bool,
    pub cubic: bool,
}

/// VANILLA: AmbientVertexRemap, индекс — направление (Down, Up, North, South, West, East).
const REMAP: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [2, 3, 0, 1],
    [3, 0, 1, 2],
    [0, 1, 2, 3],
    [3, 0, 1, 2],
    [1, 2, 3, 0],
];

fn add(p: [i32; 3], d: Direction) -> [i32; 3] {
    let o = offset(d);
    [p[0] + o[0], p[1] + o[1], p[2] + o[2]]
}

/// VANILLA: ModelBlockRenderer#calculateShape
pub fn calculate_shape(state: BlockState, q: &BakedQuad) -> FaceInfo {
    let mut mn = [32.0f32; 3];
    let mut mx = [-32.0f32; 3];
    for v in &q.pos {
        for a in 0..3 {
            mn[a] = mn[a].min(v[a]);
            mx[a] = mx[a].max(v[a]);
        }
    }
    let (x0, y0, z0, x1, y1, z1) = (mn[0], mn[1], mn[2], mx[0], mx[1], mx[2]);
    let mut fs = [0.0f32; 12];
    fs[4] = x0;
    fs[5] = x1;
    fs[0] = y0;
    fs[1] = y1;
    fs[2] = z0;
    fs[3] = z1;
    for i in 0..6 {
        fs[6 + i] = 1.0 - fs[i];
    }
    const LO: f32 = 1.0e-4;
    const HI: f32 = 0.9999;
    let partial = match q.direction {
        Direction::Down | Direction::Up => x0 >= LO || z0 >= LO || x1 <= HI || z1 <= HI,
        Direction::North | Direction::South => x0 >= LO || y0 >= LO || x1 <= HI || y1 <= HI,
        Direction::West | Direction::East => y0 >= LO || z0 >= LO || y1 <= HI || z1 <= HI,
    };
    let full = || blockinfo::has(state, COLLISION_FULL);
    let cubic = match q.direction {
        Direction::Down => y0 == y1 && (y0 < LO || full()),
        Direction::Up => y0 == y1 && (y1 > HI || full()),
        Direction::North => z0 == z1 && (z0 < LO || full()),
        Direction::South => z0 == z1 && (z1 > HI || full()),
        Direction::West => x0 == x1 && (x0 < LO || full()),
        Direction::East => x0 == x1 && (x1 > HI || full()),
    };
    FaceInfo {
        face_shape: fs,
        partial,
        cubic,
    }
}

/// VANILLA: LightCoordsUtil#smoothBlend (26.3)
fn blend4(mut a: u32, mut b: u32, mut c: u32, d: u32) -> u32 {
    let sky = |x: u32| (x >> 20) & 15;
    let block = |x: u32| (x >> 4) & 15;
    if sky(d) > 2 || block(d) > 2 {
        for n in [&mut a, &mut b, &mut c] {
            if *n == 0 {
                *n = d;
            } else if sky(*n) == 0 {
                *n |= d & 0xFF_0000;
            }
        }
    }
    (a.wrapping_add(b).wrapping_add(c).wrapping_add(d) >> 2) & 0x00FF_00FF
}

/// VANILLA: AmbientOcclusionRenderStorage#blend(int×4, float×4)
fn blend_w(a: u32, b: u32, c: u32, d: u32, w: [f32; 4]) -> u32 {
    let sky = (((a >> 16 & 0xFF) as f32 * w[0]
        + (b >> 16 & 0xFF) as f32 * w[1]
        + (c >> 16 & 0xFF) as f32 * w[2]
        + (d >> 16 & 0xFF) as f32 * w[3]) as i32
        & 0xFF) as u32;
    let blk = (((a & 0xFF) as f32 * w[0]
        + (b & 0xFF) as f32 * w[1]
        + (c & 0xFF) as f32 * w[2]
        + (d & 0xFF) as f32 * w[3]) as i32
        & 0xFF) as u32;
    sky << 16 | blk
}

/// VANILLA: AmbientOcclusionRenderStorage#calculate (+ `getShade` на выходе).
pub fn ao_light(
    pad: &Padded,
    state: BlockState,
    pos: [i32; 3],
    dir: Direction,
    shade: Option<Direction>,
    info: &FaceInfo,
    cardinal: &[f32; 6],
) -> QuadLight {
    let base = if info.cubic { add(pos, dir) } else { pos };
    let adj = &ADJACENCY[dir.index()];
    let c = adj.corners;
    let p = [
        add(base, c[0]),
        add(base, c[1]),
        add(base, c[2]),
        add(base, c[3]),
    ];
    let st = [
        pad.block(p[0]),
        pad.block(p[1]),
        pad.block(p[2]),
        pad.block(p[3]),
    ];
    let l: [u32; 4] = std::array::from_fn(|i| pad.light_color(st[i], p[i]));
    let s: [f32; 4] = std::array::from_fn(|i| pad.shade_brightness(p[i]));
    // «прозрачный» угол: !isViewBlocking || getLightBlock == 0
    let t: [bool; 4] = std::array::from_fn(|i| {
        let b = pad.block(add(p[i], dir));
        // VANILLA: BlockState#isLightPermeable (26.3) = !solidRender || lightDampening == 0
        !blockinfo::has(b, SOLID_RENDER) || b.props().is_none_or(|p| p.light_block == 0)
    });
    let diag = |a: usize, b: usize| {
        let q = add(p[a], c[b]);
        let b = pad.block(q);
        (pad.shade_brightness(q), pad.light_color(b, q))
    };
    let (sh02, l02) = if !t[2] && !t[0] {
        (s[0], l[0])
    } else {
        diag(0, 2)
    };
    let (sh03, l03) = if !t[3] && !t[0] {
        (s[0], l[0])
    } else {
        diag(0, 3)
    };
    let (sh12, l12) = if !t[2] && !t[1] {
        (s[0], l[0])
    } else {
        diag(1, 2)
    };
    let (sh13, l13) = if !t[3] && !t[1] {
        (s[0], l[0])
    } else {
        diag(1, 3)
    };

    let mut center = pad.light_color(state, pos);
    let front = add(pos, dir);
    let fb = pad.block(front);
    if info.cubic || !blockinfo::has(fb, SOLID_RENDER) {
        center = pad.light_color(fb, front);
    }
    let sc = if info.cubic {
        pad.shade_brightness(base)
    } else {
        pad.shade_brightness(pos)
    };
    let remap = &REMAP[dir.index()];
    let ba = (s[3] + s[0] + sh03 + sc) * 0.25;
    let bb = (s[2] + s[0] + sh02 + sc) * 0.25;
    let bc = (s[2] + s[1] + sh12 + sc) * 0.25;
    let bd = (s[3] + s[1] + sh13 + sc) * 0.25;
    let la = blend4(l[3], l[0], l03, center);
    let lb = blend4(l[2], l[0], l02, center);
    let lc = blend4(l[2], l[1], l12, center);
    let ld = blend4(l[3], l[1], l13, center);
    let mut out = QuadLight::default();
    if info.partial && adj.non_cubic_weight {
        let fs = &info.face_shape;
        for v in 0..4 {
            let w: [f32; 4] = std::array::from_fn(|k| {
                fs[usize::from(adj.vert_weights[v][2 * k])]
                    * fs[usize::from(adj.vert_weights[v][2 * k + 1])]
            });
            out.brightness[remap[v]] =
                (ba * w[0] + bb * w[1] + bc * w[2] + bd * w[3]).clamp(0.0, 1.0);
            out.lightmap[remap[v]] = blend_w(la, lb, lc, ld, w);
        }
    } else {
        out.lightmap[remap[0]] = la;
        out.lightmap[remap[1]] = lb;
        out.lightmap[remap[2]] = lc;
        out.lightmap[remap[3]] = ld;
        out.brightness[remap[0]] = ba;
        out.brightness[remap[1]] = bb;
        out.brightness[remap[2]] = bc;
        out.brightness[remap[3]] = bd;
    }
    let k = get_shade(cardinal, dir, shade);
    for b in &mut out.brightness {
        *b *= k;
    }
    out
}

/// VANILLA: BlockModelLighter#getDirectionalBrightness (26.3) — override вместо флага `shade`.
pub fn get_shade(cardinal: &[f32; 6], dir: Direction, shade_override: Option<Direction>) -> f32 {
    cardinal[shade_override.unwrap_or(dir).index()]
}

/// Таблица `getShade` по умолчанию (оверворлд/энд).
pub const CARDINAL_DEFAULT: [f32; 6] = [0.5, 1.0, 0.8, 0.8, 0.6, 0.6];
/// `CardinalLightType.NETHER`.
pub const CARDINAL_NETHER: [f32; 6] = [0.9, 0.9, 0.8, 0.8, 0.6, 0.6];

/// VANILLA: renderModelFaceFlat — свет одной клетки на все 4 вершины.
pub fn flat_light(
    pad: &Padded,
    state: BlockState,
    pos: [i32; 3],
    cull_side: Option<Direction>,
    q: &BakedQuad,
    info: &FaceInfo,
    cardinal: &[f32; 6],
) -> QuadLight {
    let light_pos = match cull_side {
        Some(d) => add(pos, d),
        None if info.cubic => add(pos, q.direction),
        None => pos,
    };
    let l = pad.light_color(state, light_pos);
    let b = get_shade(cardinal, q.direction, q.shade_direction_override);
    QuadLight {
        brightness: [b; 4],
        lightmap: [l; 4],
    }
}

/// VANILLA: LightTexture#lightCoordsWithEmission
pub fn with_emission(l: u32, emission: u8) -> u32 {
    if emission == 0 {
        return l;
    }
    let e = u32::from(emission);
    pack_light(light_block(l).max(e), light_sky(l).max(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_math() {
        // 4 клетки с блоковым светом 15 (=240) и небом 0 → 240
        let l = pack_light(15, 0);
        assert_eq!(blend4(l, l, l, l), l);
        // нулевая клетка заменяется центром
        assert_eq!(blend4(0, l, l, l), l);
        // среднее 0 и 15 по небу
        let a = pack_light(0, 15);
        let b = pack_light(0, 0);
        assert_eq!(blend4(a, b, b, b), (pack_light(0, 15) / 4) & 0x00FF_00FF);
        // вес 1/0/0/0 возвращает первый
        assert_eq!(blend_w(a, b, b, b, [1.0, 0.0, 0.0, 0.0]), a & 0x00FF_00FF);
        assert_eq!(light_block(pack_light(7, 3)), 7);
        assert_eq!(light_sky(pack_light(7, 3)), 3);
        assert_eq!(with_emission(pack_light(2, 9), 12), pack_light(12, 12));
        assert_eq!(with_emission(pack_light(2, 9), 0), pack_light(2, 9));
    }

    #[test]
    fn shade_tables() {
        assert_eq!(get_shade(&CARDINAL_DEFAULT, Direction::Down, None), 0.5);
        assert_eq!(
            get_shade(&CARDINAL_DEFAULT, Direction::East, Some(Direction::Up)),
            1.0
        );
        assert_eq!(
            get_shade(&CARDINAL_NETHER, Direction::East, Some(Direction::Up)),
            0.9
        );
        assert_eq!(get_shade(&CARDINAL_NETHER, Direction::Up, None), 0.9);
    }
}
