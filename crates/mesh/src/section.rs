//! Меш секции 16³: обход блоков, отсечение граней, AO/плоский свет, жидкости, VisGraph.
//! VANILLA: SectionCompiler#compile + ModelBlockRenderer#tesselateBlock (1.21.11).
use crate::blockinfo::{self, SOLID_RENDER};
use crate::cull::{offset, should_render_face};
use crate::fluid::{mesh_fluid, FluidCtx, FluidSprites};
use crate::layers::RenderLayer;
use crate::lighting::{
    ao_light, calculate_shape, flat_light, with_emission, QuadLight, CARDINAL_DEFAULT,
};
use crate::padded::Padded;
use crate::vertex::{pack_quad, PackedVertex};
use crate::vis::{VisGraph, VisibilitySet};
use ezzmc_assets::bake::{BakedQuad, QuadCollection};
use ezzmc_assets::model::Direction;
use ezzmc_world::block::BlockState;
use ezzmc_world::storage::ClientChunk;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SectionPos {
    pub x: i32,
    /// Абсолютный индекс секции (`block_y >> 4`).
    pub y: i32,
    pub z: i32,
}

/// Одна «часть» модели состояния (после выбора варианта).
#[derive(Clone, Copy)]
pub struct ModelPart<'a> {
    pub quads: &'a QuadCollection,
    /// `useAmbientOcclusion` модели.
    pub ao: bool,
}

/// Источник baked-моделей и тинтов (реализуется в клиенте поверх `assets`).
pub trait ModelProvider: Send + Sync {
    /// VANILLA: BlockStateModel#collectParts (вариант выбирается по позиции).
    fn parts<'a>(&'a self, state: BlockState, x: i32, y: i32, z: i32, out: &mut Vec<ModelPart<'a>>);
    fn layer(&self, state: BlockState) -> RenderLayer;
    /// Цвет тинта для `tint_index`.
    #[allow(clippy::too_many_arguments)]
    fn tint(
        &self,
        state: BlockState,
        tint_index: i32,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
    ) -> [u8; 3];
    fn cutout_leaves(&self) -> bool {
        true
    }
    /// `CardinalLighting` измерения (`getShade`).
    fn cardinal(&self) -> [f32; 6] {
        CARDINAL_DEFAULT
    }
    /// Опция «Smooth Lighting».
    fn smooth_lighting(&self) -> bool {
        true
    }
    fn fluid_sprites(&self) -> Option<&FluidSprites> {
        None
    }
    fn water_tint(
        &self,
        _hood: &Neighborhood,
        _base: SectionPos,
        _x: i32,
        _y: i32,
        _z: i32,
    ) -> [u8; 3] {
        [255; 3]
    }
}

/// Колонки 3×3 вокруг секции; индекс `(dz+1)*3 + (dx+1)`.
#[derive(Clone, Default)]
pub struct Neighborhood {
    pub chunks: [Option<Arc<ClientChunk>>; 9],
}

impl Neighborhood {
    pub fn center(&self) -> Option<&Arc<ClientChunk>> {
        self.chunks[4].as_ref()
    }

    fn chunk_at(&self, base: SectionPos, wx: i32, wz: i32) -> Option<&Arc<ClientChunk>> {
        let dx = (wx >> 4) - base.x;
        let dz = (wz >> 4) - base.z;
        if !(-1..=1).contains(&dx) || !(-1..=1).contains(&dz) {
            return None;
        }
        self.chunks[((dz + 1) * 3 + dx + 1) as usize].as_ref()
    }

    /// Незагруженный чанк и вне мира — воздух.
    pub fn block(&self, base: SectionPos, wx: i32, wy: i32, wz: i32) -> BlockState {
        match self.chunk_at(base, wx, wz) {
            Some(c) if wy >= c.chunk.min_y() && wy < c.chunk.max_y() => {
                c.chunk.block((wx & 15) as usize, wy, (wz & 15) as usize)
            }
            _ => BlockState::AIR,
        }
    }

    /// (block, sky).
    pub fn light(&self, base: SectionPos, wx: i32, wy: i32, wz: i32) -> (u8, u8) {
        match self.chunk_at(base, wx, wz) {
            Some(c) => (
                c.block_light((wx & 15) as usize, wy, (wz & 15) as usize)
                    .unwrap_or(0),
                c.sky_light((wx & 15) as usize, wy, (wz & 15) as usize)
                    .unwrap_or(0),
            ),
            None => (0, 0),
        }
    }

    /// Id биома (по реестру) в клетке; `None` — чанк не загружен.
    pub fn biome(&self, base: SectionPos, wx: i32, wy: i32, wz: i32) -> Option<u32> {
        let c = self.chunk_at(base, wx, wz)?;
        Some(c.chunk.biome((wx & 15) as usize, wy, (wz & 15) as usize))
    }
}

#[derive(Debug, Default, Clone)]
pub struct SectionMesh {
    /// Вершины по слоям; квад = 4 подряд идущие вершины (индексы — шаблон `0,1,2,2,3,0`).
    pub layers: [Vec<PackedVertex>; RenderLayer::COUNT],
    /// VANILLA: SectionCompiler.Results#visibilitySet
    pub visibility: VisibilitySet,
}

impl SectionMesh {
    pub fn is_empty(&self) -> bool {
        self.layers.iter().all(Vec::is_empty)
    }

    pub fn quad_count(&self) -> usize {
        self.layers.iter().map(|l| l.len() / 4).sum()
    }
}

/// Кэш тинтов одного блока по `tint_index`.
#[derive(Default)]
struct TintCache(Vec<(i32, [u8; 3])>);

impl TintCache {
    #[allow(clippy::too_many_arguments)]
    fn get(
        &mut self,
        models: &dyn ModelProvider,
        state: BlockState,
        idx: i32,
        hood: &Neighborhood,
        base: SectionPos,
        w: [i32; 3],
    ) -> [u8; 3] {
        if let Some(e) = self.0.iter().find(|e| e.0 == idx) {
            return e.1;
        }
        let c = models.tint(state, idx, hood, base, w[0], w[1], w[2]);
        self.0.push((idx, c));
        c
    }
}

/// VANILLA: SectionCompiler#compile
pub fn mesh_section(
    pos: SectionPos,
    hood: &Neighborhood,
    models: &dyn ModelProvider,
) -> SectionMesh {
    let mut mesh = SectionMesh::default();
    let Some(center) = hood.center() else {
        return mesh;
    };
    if pos.y * 16 < center.chunk.min_y() || pos.y * 16 >= center.chunk.max_y() {
        return mesh;
    }
    let pad = Padded::fill(hood, pos);
    let cutout_leaves = models.cutout_leaves();
    let cardinal = models.cardinal();
    let smooth = models.smooth_lighting();
    let fluid_sprites = models.fluid_sprites();
    let (bx, by, bz) = (pos.x * 16, pos.y * 16, pos.z * 16);
    let mut graph = VisGraph::default();
    let mut parts: Vec<ModelPart> = Vec::new();
    let mut fluid_quads: Vec<[PackedVertex; 4]> = Vec::new();
    for ly in 0..16i32 {
        for lz in 0..16i32 {
            for lx in 0..16i32 {
                let p = [lx, ly, lz];
                let state = pad.block(p);
                if state.is_air() {
                    continue;
                }
                let w = [bx + lx, by + ly, bz + lz];
                if blockinfo::has(state, SOLID_RENDER) {
                    graph.set_opaque(lx as usize, ly as usize, lz as usize);
                }
                // VANILLA: жидкость рисуется отдельно от модели блока
                if let (Some(f), Some(sp)) = (blockinfo::fluid(state), fluid_sprites) {
                    fluid_quads.clear();
                    let cx = FluidCtx {
                        pad: &pad,
                        sprites: sp,
                        cardinal: &cardinal,
                        water_tint: if f.lava {
                            [255; 3]
                        } else {
                            models.water_tint(hood, pos, w[0], w[1], w[2])
                        },
                    };
                    mesh_fluid(&cx, p, state, &f, &mut fluid_quads);
                    let layer = if f.lava {
                        RenderLayer::Solid
                    } else {
                        RenderLayer::Translucent
                    };
                    for q in &fluid_quads {
                        mesh.layers[layer.index()].extend_from_slice(q);
                    }
                }
                parts.clear();
                models.parts(state, w[0], w[1], w[2], &mut parts);
                if parts.is_empty() {
                    continue;
                }
                let layer = models.layer(state).index();
                let emission = state.props().map_or(0, |s| s.light_emission);
                let off = blockinfo::block_offset(state, w[0], w[2]);
                let origin = [lx as f32 + off[0], ly as f32 + off[1], lz as f32 + off[2]];
                let mut tints = TintCache::default();
                for part in &parts {
                    let use_ao = smooth && emission == 0 && part.ao;
                    let mut emit = |q: &BakedQuad, cull: Option<Direction>| {
                        let info = calculate_shape(state, q);
                        let ql: QuadLight = if use_ao {
                            ao_light(
                                &pad,
                                state,
                                p,
                                q.direction,
                                q.shade_direction_override,
                                &info,
                                &cardinal,
                            )
                        } else {
                            flat_light(&pad, state, p, cull, q, &info, &cardinal)
                        };
                        let tint = if q.is_tinted() {
                            tints.get(models, state, q.tint_index, hood, pos, w)
                        } else {
                            [255; 3]
                        };
                        let color: [[u8; 4]; 4] = std::array::from_fn(|i| {
                            let b = ql.brightness[i];
                            [
                                (f32::from(tint[0]) * b) as u8,
                                (f32::from(tint[1]) * b) as u8,
                                (f32::from(tint[2]) * b) as u8,
                                255,
                            ]
                        });
                        let light = ql.lightmap.map(|l| with_emission(l, q.light_emission));
                        mesh.layers[layer].extend(pack_quad(q, origin, color, light));
                    };
                    for q in part.quads.get_quads(None) {
                        emit(q, None);
                    }
                    for dir in Direction::ALL {
                        let quads = part.quads.get_quads(Some(dir));
                        if quads.is_empty() {
                            continue;
                        }
                        let o = offset(dir);
                        let neighbor = pad.block([lx + o[0], ly + o[1], lz + o[2]]);
                        if !should_render_face(state, neighbor, dir, cutout_leaves) {
                            continue;
                        }
                        for q in quads {
                            emit(q, Some(dir));
                        }
                    }
                }
            }
        }
    }
    mesh.visibility = graph.resolve();
    mesh
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_world::chunk::{Chunk, ChunkFormat};
    use ezzmc_world::generated::b26_3::BLOCKS;
    use ezzmc_world::storage::ChunkLight;

    const F: ChunkFormat = ChunkFormat {
        min_y: 0,
        height: 32,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    fn st(name: &str) -> BlockState {
        BlockState::from_id(
            BLOCKS
                .iter()
                .find(|b| b.name == name)
                .unwrap()
                .default_state,
        )
        .unwrap()
    }

    /// Единичный куб с 6 культивируемыми гранями.
    struct Cube(QuadCollection);

    fn cube() -> Cube {
        let mut c = QuadCollection::default();
        let faces: [(Direction, [[f32; 3]; 4]); 6] = [
            (
                Direction::Down,
                [[0., 0., 1.], [0., 0., 0.], [1., 0., 0.], [1., 0., 1.]],
            ),
            (
                Direction::Up,
                [[0., 1., 0.], [0., 1., 1.], [1., 1., 1.], [1., 1., 0.]],
            ),
            (
                Direction::North,
                [[1., 1., 0.], [1., 0., 0.], [0., 0., 0.], [0., 1., 0.]],
            ),
            (
                Direction::South,
                [[0., 1., 1.], [0., 0., 1.], [1., 0., 1.], [1., 1., 1.]],
            ),
            (
                Direction::West,
                [[0., 1., 0.], [0., 0., 0.], [0., 0., 1.], [0., 1., 1.]],
            ),
            (
                Direction::East,
                [[1., 1., 1.], [1., 0., 1.], [1., 0., 0.], [1., 1., 0.]],
            ),
        ];
        for (d, pos) in faces {
            c.culled[d.index()].push(BakedQuad {
                pos,
                uv: [[0.0; 2]; 4],
                tint_index: -1,
                direction: d,
                sprite: ezzmc_assets::atlas::Identifier::new("minecraft", "block/stone"),
                shade_direction_override: None,
                light_emission: 0,
            });
        }
        Cube(c)
    }

    impl ModelProvider for Cube {
        fn parts<'a>(
            &'a self,
            s: BlockState,
            _: i32,
            _: i32,
            _: i32,
            out: &mut Vec<ModelPart<'a>>,
        ) {
            if !s.is_air() {
                out.push(ModelPart {
                    quads: &self.0,
                    ao: true,
                });
            }
        }
        fn layer(&self, _: BlockState) -> RenderLayer {
            RenderLayer::Solid
        }
        fn tint(
            &self,
            _: BlockState,
            _: i32,
            _: &Neighborhood,
            _: SectionPos,
            _: i32,
            _: i32,
            _: i32,
        ) -> [u8; 3] {
            [255; 3]
        }
    }

    fn hood(blocks: &[([usize; 3], BlockState)]) -> Neighborhood {
        let mut chunk = Chunk::empty(&F);
        for (p, s) in blocks {
            chunk.set_block(p[0], p[1] as i32, p[2], *s);
        }
        let cc = Arc::new(ClientChunk {
            pos: ezzmc_protocol::types::ChunkPos { x: 0, z: 0 },
            chunk,
            heightmaps: Vec::new(),
            block_entities: Default::default(),
            light: ChunkLight::new(2),
        });
        let mut h = Neighborhood::default();
        h.chunks[4] = Some(cc);
        h
    }

    #[test]
    fn single_block_has_six_faces_and_shaded_colors() {
        let m = mesh_section(
            SectionPos { x: 0, y: 0, z: 0 },
            &hood(&[([5, 5, 5], st("stone"))]),
            &cube(),
        );
        assert_eq!(m.quad_count(), 6);
        // нет света и AO вокруг одиночного блока → яркость = только getShade; низ 0.5, верх 1.0
        let col = |i: usize| m.layers[0][i * 4].color[0];
        let mut seen: Vec<u8> = (0..6).map(col).collect();
        seen.sort_unstable();
        assert_eq!(seen, vec![127, 153, 153, 204, 204, 255]);
        // одиночный блок: мало непрозрачных → всё видно
        assert!(m.visibility.visible(Direction::Down, Direction::Up));
    }

    #[test]
    fn adjacent_blocks_cull_inner_faces() {
        let m = mesh_section(
            SectionPos { x: 0, y: 0, z: 0 },
            &hood(&[([5, 5, 5], st("stone")), ([6, 5, 5], st("stone"))]),
            &cube(),
        );
        assert_eq!(m.quad_count(), 10);
    }

    #[test]
    fn ao_darkens_corner_next_to_wall() {
        // пол 3×3 и стена: вершина пола у стены темнее
        let mut b = Vec::new();
        for x in 4..7 {
            for z in 4..7 {
                b.push(([x, 4, z], st("stone")));
            }
        }
        b.push(([4, 5, 5], st("stone")));
        let m = mesh_section(SectionPos { x: 0, y: 0, z: 0 }, &hood(&b), &cube());
        let min = m.layers[0].iter().map(|v| v.color[0]).min().unwrap();
        // AO даёт значения ниже плоской тени 0.6·255 для боковых граней, но выше 0 (0.2-блоки усредняются)
        assert!(min < 127, "{min}");
    }
}
