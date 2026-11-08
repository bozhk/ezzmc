//! Конвертер ванильного меша секции (`PackedVertex`) в `VoxelVertex` + `QuadMeta` (путь `Renderer`).
//!
//! Берёт из слоёв Solid/Cutout только «цельные» квады: все 4 вершины на целочисленной решётке секции
//! (0..=31), плоскость осевая, UV — осевой прямоугольник с вершинами в его углах. Всё остальное
//! (лестницы, кресты, жидкости, дробные модели) и слои Translucent/Tripwire остаются ванильными и
//! рисуются старым путём (`TerrainArena`), так что в режиме флага ничего не пропадает.
//!
//! Потери точности (допустимы: путь только по флагу):
//! * свет: 0..240 → 4 бита (шаг 16), округление к ближайшему;
//! * AO: цвет вершины уже содержит тинт × грань × AO; тинт берётся с самой яркой вершины
//!   (делённый на тень грани), AO остальных — ближайшая из 4 ступеней `AO_SHADE` относительно неё.
use crate::voxel_vertex::{QuadMeta, VoxelVertex};
use ezzmc_mesh::layers::RenderLayer;
use ezzmc_mesh::section::SectionMesh;
use ezzmc_mesh::vertex::{PackedVertex, POS_BIAS, POS_SCALE};
use std::collections::HashMap;

/// Должно совпадать с `FACE_SHADE` / `AO_SHADE` в `shaders/voxel_draw.wgsl`.
pub const FACE_SHADE: [f32; 6] = [0.5, 1.0, 0.8, 0.8, 0.6, 0.6];
pub const AO_SHADE: [f32; 4] = [0.4, 0.6, 0.8, 1.0];
const CELL: u16 = POS_SCALE as u16;

/// Слои, которые уходят в `Renderer`.
pub const CONVERTED_LAYERS: [RenderLayer; 2] = [RenderLayer::Solid, RenderLayer::Cutout];

/// Таблица UV-прямоугольников: id (`QuadMeta::texture`) → `[u0, v0, u1, v1]`.
#[derive(Default)]
pub struct RectTable {
    map: HashMap<[u16; 4], u32>,
    pub rects: Vec<[f32; 4]>,
    /// Появились новые прямоугольники — буфер на GPU надо перезалить.
    pub dirty: bool,
}

impl RectTable {
    fn intern(&mut self, key: [u16; 4]) -> u32 {
        if let Some(&i) = self.map.get(&key) {
            return i;
        }
        let i = self.rects.len() as u32;
        self.rects.push(key.map(|v| f32::from(v) / 65535.0));
        self.map.insert(key, i);
        self.dirty = true;
        i
    }
}

#[derive(Default, Debug)]
pub struct Converted {
    pub verts: Vec<VoxelVertex>,
    pub quads: Vec<QuadMeta>,
}

/// Делит меш: цельные квады → `Converted`, остальное → `SectionMesh` для ванильной арены.
pub fn split_mesh(mesh: &SectionMesh, rects: &mut RectTable) -> (Converted, SectionMesh) {
    let mut out = Converted::default();
    let mut rest = mesh.clone();
    for layer in CONVERTED_LAYERS {
        let src = &mesh.layers[layer.index()];
        let dst = &mut rest.layers[layer.index()];
        dst.clear();
        for q in src.chunks_exact(4) {
            let q: &[PackedVertex; 4] = q.try_into().expect("chunks_exact(4)");
            match convert_quad(q, rects) {
                Some((v, m)) => {
                    out.verts.extend_from_slice(&v);
                    out.quads.push(m);
                }
                None => dst.extend_from_slice(q),
            }
        }
    }
    (out, rest)
}

fn lattice(v: u16) -> Option<u8> {
    if v % CELL != 0 {
        return None;
    }
    let c = i32::from(v / CELL) - POS_BIAS as i32;
    (0..=31).contains(&c).then_some(c as u8)
}

fn light4(x: u8) -> u8 {
    ((u16::from(x) + 8) / 16).min(15) as u8
}

fn convert_quad(
    q: &[PackedVertex; 4],
    rects: &mut RectTable,
) -> Option<([VoxelVertex; 4], QuadMeta)> {
    let mut p = [[0u8; 3]; 4];
    for (i, v) in q.iter().enumerate() {
        p[i] = [lattice(v.pos[0])?, lattice(v.pos[1])?, lattice(v.pos[2])?];
    }
    // ровно одна постоянная ось
    let consts: Vec<usize> = (0..3)
        .filter(|&a| p.iter().all(|c| c[a] == p[0][a]))
        .collect();
    let [axis] = consts[..] else { return None };
    // нормаль по обходу вершин (ваниль: CCW спереди)
    let d = |i: usize, j: usize| [0, 1, 2].map(|a| i32::from(p[i][a]) - i32::from(p[j][a]));
    let (e1, e2) = (d(1, 0), d(2, 0));
    let n = [
        e1[1] * e2[2] - e1[2] * e2[1],
        e1[2] * e2[0] - e1[0] * e2[2],
        e1[0] * e2[1] - e1[1] * e2[0],
    ][axis];
    if n == 0 {
        return None;
    }
    let face: u8 = match (axis, n > 0) {
        (1, false) => 0,
        (1, true) => 1,
        (2, false) => 2,
        (2, true) => 3,
        (0, false) => 4,
        _ => 5,
    };
    // UV: осевой прямоугольник, вершины в разных углах
    let us = q.map(|v| v.uv[0]);
    let vs = q.map(|v| v.uv[1]);
    let (umin, umax) = (*us.iter().min()?, *us.iter().max()?);
    let (vmin, vmax) = (*vs.iter().min()?, *vs.iter().max()?);
    if umin == umax || vmin == vmax {
        return None;
    }
    let mut corner = [0u8; 4];
    let mut seen = 0u8;
    for i in 0..4 {
        let fu = match us[i] {
            u if u == umin => 0,
            u if u == umax => 1,
            _ => return None,
        };
        let fv = match vs[i] {
            v if v == vmin => 0,
            v if v == vmax => 1,
            _ => return None,
        };
        corner[i] = match (fu, fv) {
            (0, 0) => 0,
            (1, 0) => 1,
            (1, 1) => 2,
            _ => 3,
        };
        seen |= 1 << corner[i];
    }
    if seen != 0b1111 {
        return None;
    }
    // цвет: тинт с самой яркой вершины, AO — относительная яркость
    let luma = |c: [u8; 4]| u32::from(c[0]) + u32::from(c[1]) + u32::from(c[2]);
    let m = (0..4).max_by_key(|&i| luma(q[i].color))?;
    let shade = FACE_SHADE[face as usize];
    let tint = q[m].color;
    let tint_rgb = [0, 1, 2].map(|a| (f32::from(tint[a]) / shade).round().clamp(0.0, 255.0) as u8);
    let lm = luma(tint).max(1) as f32;
    let ao = |i: usize| -> u8 {
        let r = luma(q[i].color) as f32 / lm;
        (0..4u8)
            .min_by(|&a, &b| {
                (AO_SHADE[a as usize] - r)
                    .abs()
                    .total_cmp(&(AO_SHADE[b as usize] - r).abs())
            })
            .unwrap_or(3)
    };
    let id = rects.intern([umin, vmin, umax, vmax]);
    let verts = std::array::from_fn(|i| {
        let l = q[i].light;
        VoxelVertex::pack(
            p[i],
            face,
            ao(i),
            light4((l & 255) as u8),
            light4((l >> 8) as u8),
            corner[i],
        )
    });
    let meta = QuadMeta {
        texture: id,
        tint: u32::from_le_bytes([tint_rgb[0], tint_rgb[1], tint_rgb[2], tint[3]]),
    };
    Some((verts, meta))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pv(pos: [f32; 3], uv: [f32; 2], color: [u8; 4], block: u8, sky: u8) -> PackedVertex {
        PackedVertex::pack(pos, uv, color, block, sky)
    }

    /// Верхняя грань блока (1,2,3): y = 3, обход CCW сверху.
    fn top_quad(color: [[u8; 4]; 4]) -> [PackedVertex; 4] {
        let (x, y, z) = (1.0, 3.0, 3.0);
        [
            pv([x, y, z + 1.0], [0.25, 0.5], color[0], 240, 112),
            pv([x + 1.0, y, z + 1.0], [0.5, 0.5], color[1], 240, 112),
            pv([x + 1.0, y, z], [0.5, 0.25], color[2], 240, 112),
            pv([x, y, z], [0.25, 0.25], color[3], 240, 112),
        ]
    }

    #[test]
    fn cube_face_converts_with_rounded_light_and_ao() {
        let w = [255, 255, 255, 255];
        let g = [153, 153, 153, 255]; // 0.6 от яркого
        let mut mesh = SectionMesh::default();
        mesh.layers[0].extend(top_quad([w, w, g, w]));
        let mut rects = RectTable::default();
        let (c, rest) = split_mesh(&mesh, &mut rects);
        assert_eq!(c.quads.len(), 1);
        assert!(rest.is_empty());
        assert_eq!(rects.rects.len(), 1);
        let v = c.verts;
        assert_eq!(v[0].face(), 1);
        assert_eq!(v[0].pos(), [1, 3, 4]);
        assert_eq!((v[0].block_light(), v[0].sky_light()), (15, 7));
        assert_eq!((v[0].ao(), v[2].ao()), (3, 1));
        let corners: Vec<u8> = v.iter().map(|x| x.corner()).collect();
        assert_eq!(corners, [3, 2, 1, 0]);
        // верхняя грань: тень 1.0 → тинт = цвет самой яркой вершины
        assert_eq!(c.quads[0].tint, u32::MAX);
    }

    #[test]
    fn non_cube_and_other_layers_stay_vanilla() {
        let w = [255; 4];
        let mut mesh = SectionMesh::default();
        // дробная позиция (ступенька)
        let mut half = top_quad([w; 4]);
        half[0].pos[1] += 1024;
        half[1].pos[1] += 1024;
        half[2].pos[1] += 1024;
        half[3].pos[1] += 1024;
        mesh.layers[0].extend(half);
        // перекос UV
        let mut skew = top_quad([w; 4]);
        skew[0].uv[0] += 7;
        mesh.layers[1].extend(skew);
        // прозрачный слой — целый квад, но слой не конвертируется
        mesh.layers[2].extend(top_quad([w; 4]));
        let mut rects = RectTable::default();
        let (c, rest) = split_mesh(&mesh, &mut rects);
        assert!(c.quads.is_empty());
        assert_eq!(rest.quad_count(), 3);
    }

    #[test]
    fn side_face_shade_is_divided_out() {
        // грань «юг» (z=4), тень 0.8: цвет вершины = 204 → тинт 255
        let c = [204, 204, 204, 255];
        let q = [
            pv([1.0, 2.0, 4.0], [0.25, 0.5], c, 0, 240),
            pv([2.0, 2.0, 4.0], [0.5, 0.5], c, 0, 240),
            pv([2.0, 3.0, 4.0], [0.5, 0.25], c, 0, 240),
            pv([1.0, 3.0, 4.0], [0.25, 0.25], c, 0, 240),
        ];
        let mut mesh = SectionMesh::default();
        mesh.layers[0].extend(q);
        let (conv, _) = split_mesh(&mesh, &mut RectTable::default());
        assert_eq!(conv.verts[0].face(), 3);
        assert_eq!(conv.quads[0].tint & 0xFF, 255);
        assert_eq!(conv.verts[0].sky_light(), 15);
    }
}
