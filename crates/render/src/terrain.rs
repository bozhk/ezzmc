//! Индирект-рендер террейна (TODO-144).
//!
//! * Все секции лежат в одном большом вершинном буфере (упакованные 16-байтные вершины), под память
//!   отвечает [`RangeAllocator`] (first-fit со слиянием свободных диапазонов).
//! * Индексный буфер = `[общий шаблон 0,1,2,2,3,0 на PATTERN_QUADS квадов][арена отсортированных индексов]`:
//!   непрозрачные слои рисуются по шаблону (`base_vertex` = начало вершин секции), прозрачный слой хранит
//!   собственные индексы в арене (TODO-142: пересортировка на месте, длина не меняется).
//! * Origin секции берётся шейдером из storage-буфера по `instance_index`; слот секции передаётся как
//!   `first_instance` индирект-команды (нужен `INDIRECT_FIRST_INSTANCE`, иначе — цикл `draw_indexed`).
//! * Один вызов `multi_draw_indexed_indirect` на слой; слои идут в порядке solid → cutout → translucent → tripwire.
use ezzmc_mesh::layers::RenderLayer;
use ezzmc_mesh::section::{SectionMesh, SectionPos};
use ezzmc_mesh::sort::{PointOfView, SortState};
use ezzmc_mesh::vertex::PackedVertex;
use std::collections::HashMap;
use thiserror::Error;

/// Размер общего шаблона индексов в квадах (секция вмещает ≤ 24576 граней полных кубов).
pub const PATTERN_QUADS: u32 = 1 << 16;
const TRANSLUCENT: usize = RenderLayer::Translucent as usize;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ArenaError {
    #[error("нет места в вершинном буфере ({0} вершин)")]
    VertexSpace(u32),
    #[error("нет места в арене индексов ({0} индексов)")]
    IndexSpace(u32),
    #[error("слишком много квадов в слое секции: {0}")]
    TooManyQuads(u32),
    #[error("исчерпаны слоты секций")]
    SlotLimit,
}

/// Аллокатор диапазонов `u32` (first-fit, свободные диапазоны отсортированы и слиты).
#[derive(Debug, Clone)]
pub struct RangeAllocator {
    capacity: u32,
    free: Vec<(u32, u32)>,
    used: u32,
}

impl RangeAllocator {
    pub fn new(capacity: u32) -> Self {
        RangeAllocator {
            capacity,
            free: if capacity > 0 {
                vec![(0, capacity)]
            } else {
                Vec::new()
            },
            used: 0,
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn used(&self) -> u32 {
        self.used
    }

    /// Выделяет `len` элементов; `len == 0` ничего не занимает.
    pub fn alloc(&mut self, len: u32) -> Option<u32> {
        if len == 0 {
            return Some(0);
        }
        let i = self.free.iter().position(|&(_, l)| l >= len)?;
        let (start, l) = self.free[i];
        if l == len {
            self.free.remove(i);
        } else {
            self.free[i] = (start + len, l - len);
        }
        self.used += len;
        Some(start)
    }

    pub fn free(&mut self, start: u32, len: u32) {
        if len == 0 {
            return;
        }
        self.used = self.used.saturating_sub(len);
        let i = self.free.partition_point(|&(s, _)| s < start);
        let merge_prev = i > 0 && self.free[i - 1].0 + self.free[i - 1].1 == start;
        let merge_next = i < self.free.len() && start + len == self.free[i].0;
        match (merge_prev, merge_next) {
            (true, true) => {
                self.free[i - 1].1 += len + self.free[i].1;
                self.free.remove(i);
            }
            (true, false) => self.free[i - 1].1 += len,
            (false, true) => self.free[i] = (start, len + self.free[i].1),
            (false, false) => self.free.insert(i, (start, len)),
        }
    }
}

/// Размещение одного слоя секции.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayerPlacement {
    pub first_vertex: u32,
    pub quads: u32,
    /// Абсолютный индекс в индексном буфере (для шаблона — 0).
    pub first_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub slot: u32,
    pub layers: [LayerPlacement; RenderLayer::COUNT],
}

/// Одна команда `DrawIndexedIndirect` (20 байт).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndirectCmd {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub base_vertex: i32,
    pub first_instance: u32,
}

impl IndirectCmd {
    pub const SIZE: u64 = 20;

    pub fn to_bytes(&self) -> [u8; 20] {
        let mut o = [0u8; 20];
        o[0..4].copy_from_slice(&self.index_count.to_le_bytes());
        o[4..8].copy_from_slice(&self.instance_count.to_le_bytes());
        o[8..12].copy_from_slice(&self.first_index.to_le_bytes());
        o[12..16].copy_from_slice(&self.base_vertex.to_le_bytes());
        o[16..20].copy_from_slice(&self.first_instance.to_le_bytes());
        o
    }
}

/// Учёт памяти и слотов без GPU (тестируется отдельно).
#[derive(Debug)]
pub struct ArenaLayout {
    vertices: RangeAllocator,
    indices: RangeAllocator,
    index_base: u32,
    max_slots: u32,
    slots: Vec<Option<SectionPos>>,
    free_slots: Vec<u32>,
    sections: HashMap<SectionPos, Placement>,
}

impl ArenaLayout {
    pub fn new(vertex_capacity: u32, index_arena_capacity: u32, max_slots: u32) -> Self {
        ArenaLayout {
            vertices: RangeAllocator::new(vertex_capacity),
            indices: RangeAllocator::new(index_arena_capacity),
            index_base: PATTERN_QUADS * 6,
            max_slots,
            slots: Vec::new(),
            free_slots: Vec::new(),
            sections: HashMap::new(),
        }
    }

    pub fn get(&self, pos: SectionPos) -> Option<&Placement> {
        self.sections.get(&pos)
    }

    /// Секция, занимающая слот.
    pub fn slot_pos(&self, slot: u32) -> Option<SectionPos> {
        self.slots.get(slot as usize).copied().flatten()
    }

    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Число выданных слотов (верхняя граница индексов слотов; часть может быть свободна).
    pub fn slot_count(&self) -> u32 {
        self.slots.len() as u32
    }

    pub fn used_vertices(&self) -> u32 {
        self.vertices.used()
    }

    pub fn remove(&mut self, pos: SectionPos) -> Option<Placement> {
        let p = self.sections.remove(&pos)?;
        for (i, l) in p.layers.iter().enumerate() {
            self.vertices.free(l.first_vertex, l.quads * 4);
            if i == TRANSLUCENT {
                self.indices
                    .free(l.first_index - self.index_base, l.quads * 6);
            }
        }
        self.slots[p.slot as usize] = None;
        self.free_slots.push(p.slot);
        Some(p)
    }

    /// Заменяет секцию (`quads` — число квадов по слоям). Транзакционно: при ошибке старая версия уже удалена,
    /// но ничего не утекло.
    pub fn insert(
        &mut self,
        pos: SectionPos,
        quads: [u32; RenderLayer::COUNT],
    ) -> Result<Placement, ArenaError> {
        self.remove(pos);
        if let Some(&q) = quads.iter().find(|&&q| q > PATTERN_QUADS) {
            return Err(ArenaError::TooManyQuads(q));
        }
        let slot = match self.free_slots.pop() {
            Some(s) => s,
            None if (self.slots.len() as u32) < self.max_slots => {
                self.slots.push(None);
                self.slots.len() as u32 - 1
            }
            None => return Err(ArenaError::SlotLimit),
        };
        let mut layers = [LayerPlacement::default(); RenderLayer::COUNT];
        for i in 0..RenderLayer::COUNT {
            let n = quads[i];
            let Some(v) = self.vertices.alloc(n * 4) else {
                self.rollback(&layers, i);
                self.free_slots.push(slot);
                return Err(ArenaError::VertexSpace(n * 4));
            };
            layers[i] = LayerPlacement {
                first_vertex: v,
                quads: n,
                first_index: 0,
            };
            if i == TRANSLUCENT {
                let Some(ix) = self.indices.alloc(n * 6) else {
                    self.vertices.free(v, n * 4);
                    layers[i].quads = 0;
                    self.rollback(&layers, i);
                    self.free_slots.push(slot);
                    return Err(ArenaError::IndexSpace(n * 6));
                };
                layers[i].first_index = self.index_base + ix;
            }
        }
        self.slots[slot as usize] = Some(pos);
        let p = Placement { slot, layers };
        self.sections.insert(pos, p);
        Ok(p)
    }

    fn rollback(&mut self, layers: &[LayerPlacement], upto: usize) {
        for (i, l) in layers.iter().enumerate().take(upto) {
            self.vertices.free(l.first_vertex, l.quads * 4);
            if i == TRANSLUCENT {
                self.indices
                    .free(l.first_index - self.index_base, l.quads * 6);
            }
        }
    }

    /// Команды слоя для секций в заданном порядке (пустые слои пропускаются).
    pub fn commands(
        &self,
        layer: RenderLayer,
        order: impl IntoIterator<Item = SectionPos>,
    ) -> Vec<IndirectCmd> {
        order
            .into_iter()
            .filter_map(|pos| {
                let p = self.sections.get(&pos)?;
                let l = p.layers[layer.index()];
                (l.quads > 0).then_some(IndirectCmd {
                    index_count: l.quads * 6,
                    instance_count: 1,
                    first_index: l.first_index,
                    base_vertex: l.first_vertex as i32,
                    first_instance: p.slot,
                })
            })
            .collect()
    }
}

fn u32_bytes(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// Индексы шаблона `0,1,2,2,3,0` на `quads` квадов.
pub fn quad_pattern(quads: u32) -> Vec<u32> {
    ezzmc_mesh::sort::identity_indices(quads as usize)
}

struct Translucent {
    sort: SortState,
    pov: Option<PointOfView>,
}

/// GPU-арена террейна.
pub struct TerrainArena {
    layout: ArenaLayout,
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    /// storage: `vec4<i32>` origin секции по слоту.
    pub origins_buf: wgpu::Buffer,
    indirect_buf: wgpu::Buffer,
    /// PERF (TODO-490): исходные команды всех резидентных секций по слотам (`[слой][слот]`), читает compute-culling.
    pub(crate) src_cmds_buf: wgpu::Buffer,
    /// PERF (TODO-490): `u32` на слот — 1, если секция прошла CPU cave-culling (см. `set_visible`).
    pub(crate) flags_buf: wgpu::Buffer,
    /// Слоты, освобождённые `remove`, но ещё не обнулённые в `src_cmds_buf`.
    stale_slots: Vec<u32>,
    max_slots: u32,
    first_instance_ok: bool,
    sort: HashMap<SectionPos, Translucent>,
    /// Команды последнего `prepare_draws` по слоям.
    cmds: [Vec<IndirectCmd>; RenderLayer::COUNT],
}

impl TerrainArena {
    /// Фича, которую стоит запросить у устройства (если адаптер её умеет).
    pub const WANTED_FEATURES: wgpu::Features = wgpu::Features::INDIRECT_FIRST_INSTANCE;

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertex_capacity: u32,
        index_arena_capacity: u32,
        max_sections: u32,
    ) -> Self {
        let vbytes = u64::from(vertex_capacity) * PackedVertex::SIZE as u64;
        let ibytes = (u64::from(PATTERN_QUADS) * 6 + u64::from(index_arena_capacity)) * 4;
        let mk = |label: &str, size: u64, usage: wgpu::BufferUsages| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size.max(16),
                usage: usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let vertex_buf = mk("terrain.vertices", vbytes, wgpu::BufferUsages::VERTEX);
        let index_buf = mk("terrain.indices", ibytes, wgpu::BufferUsages::INDEX);
        let origins_buf = mk(
            "terrain.origins",
            u64::from(max_sections) * 16,
            wgpu::BufferUsages::STORAGE,
        );
        let src_cmds_buf = mk(
            "terrain.src_cmds",
            u64::from(max_sections) * IndirectCmd::SIZE * RenderLayer::COUNT as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let flags_buf = mk(
            "terrain.vis_flags",
            u64::from(max_sections) * 4,
            wgpu::BufferUsages::STORAGE,
        );
        let indirect_buf = mk(
            "terrain.indirect",
            u64::from(max_sections) * IndirectCmd::SIZE * RenderLayer::COUNT as u64,
            wgpu::BufferUsages::INDIRECT,
        );
        queue.write_buffer(&index_buf, 0, &u32_bytes(&quad_pattern(PATTERN_QUADS)));
        TerrainArena {
            layout: ArenaLayout::new(vertex_capacity, index_arena_capacity, max_sections),
            vertex_buf,
            index_buf,
            origins_buf,
            indirect_buf,
            src_cmds_buf,
            flags_buf,
            stale_slots: Vec::new(),
            max_slots: max_sections,
            first_instance_ok: device
                .features()
                .contains(wgpu::Features::INDIRECT_FIRST_INSTANCE),
            sort: HashMap::new(),
            cmds: Default::default(),
        }
    }

    pub fn layout(&self) -> &ArenaLayout {
        &self.layout
    }

    /// Загружает (заменяет) меш секции; `camera` — мировая позиция камеры для начальной сортировки.
    pub fn upload(
        &mut self,
        queue: &wgpu::Queue,
        pos: SectionPos,
        mesh: &SectionMesh,
        camera: [f64; 3],
    ) -> Result<(), ArenaError> {
        self.sort.remove(&pos);
        if let Some(old) = self.layout.get(pos).map(|p| p.slot) {
            self.write_src_cmds(queue, old, None);
        }
        let quads = std::array::from_fn(|i| (mesh.layers[i].len() / 4) as u32);
        let p = self.layout.insert(pos, quads)?;
        let origin = [pos.x * 16, pos.y * 16, pos.z * 16, 0i32];
        let ob: Vec<u8> = origin.iter().flat_map(|v| v.to_le_bytes()).collect();
        queue.write_buffer(&self.origins_buf, u64::from(p.slot) * 16, &ob);
        self.write_src_cmds(queue, p.slot, Some(&p));
        for (i, l) in p.layers.iter().enumerate() {
            let v = &mesh.layers[i];
            if v.is_empty() {
                continue;
            }
            let bytes: Vec<u8> = v.iter().flat_map(PackedVertex::to_bytes).collect();
            queue.write_buffer(
                &self.vertex_buf,
                u64::from(l.first_vertex) * PackedVertex::SIZE as u64,
                &bytes,
            );
        }
        if p.layers[TRANSLUCENT].quads > 0 {
            self.sort.insert(
                pos,
                Translucent {
                    sort: SortState::from_vertices(&mesh.layers[TRANSLUCENT]),
                    pov: None,
                },
            );
            self.resort(queue, pos, camera);
        }
        Ok(())
    }

    /// Пишет (или обнуляет) исходные команды слота для GPU-culling.
    fn write_src_cmds(&self, queue: &wgpu::Queue, slot: u32, p: Option<&Placement>) {
        let mut bytes = Vec::with_capacity(RenderLayer::COUNT * 20);
        let mut offsets = [0u64; RenderLayer::COUNT];
        for layer in RenderLayer::ALL {
            let i = layer.index();
            let cmd = p.and_then(|p| {
                let l = p.layers[i];
                (l.quads > 0).then_some(IndirectCmd {
                    index_count: l.quads * 6,
                    instance_count: 1,
                    first_index: l.first_index,
                    base_vertex: l.first_vertex as i32,
                    first_instance: slot,
                })
            });
            offsets[i] =
                (i as u64 * u64::from(self.max_slots) + u64::from(slot)) * IndirectCmd::SIZE;
            bytes.clear();
            bytes.extend_from_slice(&cmd.map_or([0u8; 20], |c| c.to_bytes()));
            queue.write_buffer(&self.src_cmds_buf, offsets[i], &bytes);
        }
    }

    /// Результат CPU cave-culling для GPU-пути: видимым — флаг 1, остальным 0 (TODO-490).
    pub fn set_visible(&self, queue: &wgpu::Queue, visible: &[SectionPos]) {
        let mut flags = vec![0u32; self.layout.slot_count() as usize];
        for pos in visible {
            if let Some(p) = self.layout.get(*pos) {
                flags[p.slot as usize] = 1;
            }
        }
        if !flags.is_empty() {
            queue.write_buffer(&self.flags_buf, 0, &u32_bytes(&flags));
        }
    }

    pub fn slot_count(&self) -> u32 {
        self.layout.slot_count()
    }

    pub fn max_slots(&self) -> u32 {
        self.max_slots
    }

    pub fn first_instance_ok(&self) -> bool {
        self.first_instance_ok
    }

    pub fn remove(&mut self, pos: SectionPos) {
        self.sort.remove(&pos);
        if let Some(p) = self.layout.remove(pos) {
            self.stale_slots.push(p.slot);
        }
    }

    /// Обнуляет исходные команды освобождённых слотов (вызывать перед GPU-culling).
    pub fn flush_removed(&mut self, queue: &wgpu::Queue) {
        for slot in std::mem::take(&mut self.stale_slots) {
            if self.layout.slot_pos(slot).is_none() {
                self.write_src_cmds(queue, slot, None);
            }
        }
    }

    /// Пересортирует прозрачные квады, если нужно (`PointOfView::needs_resort`). Возвращает, была ли запись.
    pub fn resort(&mut self, queue: &wgpu::Queue, pos: SectionPos, camera: [f64; 3]) -> bool {
        let (Some(p), Some(t)) = (self.layout.get(pos), self.sort.get_mut(&pos)) else {
            return false;
        };
        let pov = PointOfView::of(camera, [pos.x, pos.y, pos.z]);
        if !PointOfView::needs_resort(t.pov, pov) {
            return false;
        }
        t.pov = Some(pov);
        let rel = [
            (camera[0] - f64::from(pos.x * 16)) as f32,
            (camera[1] - f64::from(pos.y * 16)) as f32,
            (camera[2] - f64::from(pos.z * 16)) as f32,
        ];
        let data = u32_bytes(&t.sort.sorted_indices(rel));
        queue.write_buffer(
            &self.index_buf,
            u64::from(p.layers[TRANSLUCENT].first_index) * 4,
            &data,
        );
        true
    }

    /// Собирает команды слоёв для видимых секций и заливает их в индирект-буфер.
    /// Непрозрачные слои — ближние первыми (раннее отсечение по глубине), прозрачные — дальние первыми.
    pub fn prepare_draws(&mut self, queue: &wgpu::Queue, visible: &[SectionPos], camera: [f64; 3]) {
        let dist = |p: &SectionPos| {
            let d = |c: i32, v: f64| f64::from(c * 16 + 8) - v;
            let (x, y, z) = (d(p.x, camera[0]), d(p.y, camera[1]), d(p.z, camera[2]));
            x * x + y * y + z * z
        };
        let mut near: Vec<SectionPos> = visible.to_vec();
        near.sort_by(|a, b| dist(a).total_cmp(&dist(b)));
        let far: Vec<SectionPos> = near.iter().rev().copied().collect();
        for layer in RenderLayer::ALL {
            let order = if layer == RenderLayer::Translucent {
                &far
            } else {
                &near
            };
            let cmds = self.layout.commands(layer, order.iter().copied());
            let cap = self.max_slots as usize;
            let cmds = &cmds[..cmds.len().min(cap)];
            let bytes: Vec<u8> = cmds.iter().flat_map(IndirectCmd::to_bytes).collect();
            if !bytes.is_empty() {
                queue.write_buffer(&self.indirect_buf, self.layer_offset(layer), &bytes);
            }
            self.cmds[layer.index()] = cmds.to_vec();
        }
    }

    fn layer_offset(&self, layer: RenderLayer) -> u64 {
        layer.index() as u64 * u64::from(self.max_slots) * IndirectCmd::SIZE
    }

    /// Число секций, нарисованных в слое при последнем `prepare_draws`.
    pub fn draw_count(&self, layer: RenderLayer) -> usize {
        self.cmds[layer.index()].len()
    }

    /// Рисует слой (конвейер, группы привязок и камера — на вызывающем).
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, layer: RenderLayer) {
        let cmds = &self.cmds[layer.index()];
        if cmds.is_empty() {
            return;
        }
        pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint32);
        if self.first_instance_ok {
            pass.multi_draw_indexed_indirect(
                &self.indirect_buf,
                self.layer_offset(layer),
                cmds.len() as u32,
            );
        } else {
            // запасной путь: прямые вызовы (first_instance в прямых командах разрешён всегда)
            for c in cmds {
                pass.draw_indexed(
                    c.first_index..c.first_index + c.index_count,
                    c.base_vertex,
                    c.first_instance..c.first_instance + 1,
                );
            }
        }
    }
}

/// Вершинный формат террейна: `Uint16x4` позиция+свет, `Unorm16x2` UV, `Unorm8x4` цвет.
pub fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    const ATTRS: [wgpu::VertexAttribute; 3] = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Uint16x4,
            offset: 0,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Unorm16x2,
            offset: 8,
            shader_location: 1,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Unorm8x4,
            offset: 12,
            shader_location: 2,
        },
    ];
    wgpu::VertexBufferLayout {
        array_stride: PackedVertex::SIZE as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRS,
    }
}

/// Раскладки привязок и конвейеры слоёв террейна.
pub struct TerrainPipelines {
    pub group0: wgpu::BindGroupLayout,
    pub group1: wgpu::BindGroupLayout,
    pipelines: [wgpu::RenderPipeline; RenderLayer::COUNT],
}

impl TerrainPipelines {
    pub fn new(
        device: &wgpu::Device,
        color: wgpu::TextureFormat,
        depth: Option<wgpu::TextureFormat>,
    ) -> Self {
        let vf = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let uniform = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: vf,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let group0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("terrain.group0"),
            entries: &[
                uniform(0),
                uniform(1),
                uniform(2),
                uniform(4),
                wgpu::BindGroupLayoutEntry {
                    binding: 8,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let tex = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let samp = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        let group1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("terrain.group1"),
            entries: &[tex(0), samp(1), tex(2), samp(3)],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("terrain"),
            bind_group_layouts: &[&group0, &group1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("terrain"),
            source: wgpu::ShaderSource::Wgsl(crate::shaders::Shader::Terrain.source().into()),
        });
        // VANILLA: RenderPipelines translucent/tripwire — BlendFunction.TRANSLUCENT
        let blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let pipelines = RenderLayer::ALL.map(|layer| {
            let constants = [("alpha_cutout", f64::from(layer.alpha_cutout()))];
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(&format!("terrain.{layer:?}")),
                layout: Some(&pl),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &constants,
                        ..Default::default()
                    },
                    buffers: &[vertex_layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &constants,
                        ..Default::default()
                    },
                    targets: &[Some(wgpu::ColorTargetState {
                        format: color,
                        blend: layer.translucent().then_some(blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: depth.map(|format| wgpu::DepthStencilState {
                    format,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    // Накладки (ковры, мох, растения, снег) лежат вплотную к кубам: чуть ближе
                    // к камере, чтобы не мерцали (Z-fighting). Не ваниль, но визуально незаметно.
                    bias: if matches!(layer, RenderLayer::Cutout | RenderLayer::Tripwire) {
                        wgpu::DepthBiasState {
                            constant: -2,
                            slope_scale: -1.0,
                            clamp: 0.0,
                        }
                    } else {
                        Default::default()
                    },
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        });
        TerrainPipelines {
            group0,
            group1,
            pipelines,
        }
    }

    pub fn pipeline(&self, layer: RenderLayer) -> &wgpu::RenderPipeline {
        &self.pipelines[layer.index()]
    }

    /// `group(0)`: projection, fog, globals, chunk-section uniform-буферы + origin-буфер арены.
    pub fn bind_group0(
        &self,
        device: &wgpu::Device,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        globals: &wgpu::Buffer,
        chunk: &wgpu::Buffer,
        arena: &TerrainArena,
    ) -> wgpu::BindGroup {
        fn e(binding: u32, b: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
            wgpu::BindGroupEntry {
                binding,
                resource: b.as_entire_binding(),
            }
        }
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("terrain.group0"),
            layout: &self.group0,
            entries: &[
                e(0, projection),
                e(1, fog),
                e(2, globals),
                e(4, chunk),
                e(8, &arena.origins_buf),
            ],
        })
    }

    pub fn bind_group1(
        &self,
        device: &wgpu::Device,
        atlas: &wgpu::TextureView,
        atlas_sampler: &wgpu::Sampler,
        lightmap: &wgpu::TextureView,
        lightmap_sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("terrain.group1"),
            layout: &self.group1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(atlas_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(lightmap),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(lightmap_sampler),
                },
            ],
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sp(x: i32) -> SectionPos {
        SectionPos { x, y: 0, z: 0 }
    }

    #[test]
    fn allocator_first_fit_and_coalesce() {
        let mut a = RangeAllocator::new(100);
        let x = a.alloc(30).unwrap();
        let y = a.alloc(30).unwrap();
        let z = a.alloc(30).unwrap();
        assert_eq!((x, y, z), (0, 30, 60));
        assert!(a.alloc(20).is_none());
        a.free(y, 30);
        assert_eq!(a.alloc(10), Some(30)); // дыра используется first-fit
        a.free(30, 10);
        a.free(x, 30);
        a.free(z, 30);
        assert_eq!(a.used(), 0);
        assert_eq!(a.alloc(100), Some(0)); // всё слилось в один диапазон
        assert_eq!(a.alloc(0), Some(0));
    }

    #[test]
    fn layout_places_and_reuses() {
        let mut l = ArenaLayout::new(1000, 600, 4);
        let a = l.insert(sp(0), [2, 1, 3, 0]).unwrap();
        assert_eq!(a.slot, 0);
        assert_eq!(a.layers[0].first_vertex, 0);
        assert_eq!(a.layers[1].first_vertex, 8);
        assert_eq!(a.layers[2].first_index, PATTERN_QUADS * 6); // арена индексов за шаблоном
        let b = l.insert(sp(1), [1, 0, 0, 0]).unwrap();
        assert_eq!(b.slot, 1);
        l.remove(sp(0));
        let c = l.insert(sp(2), [1, 0, 0, 0]).unwrap();
        assert_eq!((c.slot, c.layers[0].first_vertex), (0, 0)); // слот и память переиспользуются
                                                                // замена секции не плодит слоты
        let again = l.insert(sp(2), [2, 0, 0, 0]).unwrap();
        assert_eq!(l.len(), 2);
        assert!(again.slot < 4);
    }

    #[test]
    fn layout_errors_do_not_leak() {
        let mut l = ArenaLayout::new(16, 12, 2);
        assert_eq!(
            l.insert(sp(0), [0, 0, 3, 0]),
            Err(ArenaError::IndexSpace(18))
        );
        assert_eq!(l.used_vertices(), 0);
        assert_eq!(
            l.insert(sp(0), [5, 0, 0, 0]),
            Err(ArenaError::VertexSpace(20))
        );
        assert_eq!(
            l.insert(sp(0), [PATTERN_QUADS + 1, 0, 0, 0]),
            Err(ArenaError::TooManyQuads(PATTERN_QUADS + 1))
        );
        l.insert(sp(0), [1, 0, 0, 0]).unwrap();
        l.insert(sp(1), [1, 0, 0, 0]).unwrap();
        assert_eq!(l.insert(sp(2), [1, 0, 0, 0]), Err(ArenaError::SlotLimit));
        assert_eq!(l.used_vertices(), 8);
    }

    #[test]
    fn commands_per_layer() {
        let mut l = ArenaLayout::new(1000, 600, 8);
        l.insert(sp(0), [2, 0, 1, 0]).unwrap();
        l.insert(sp(1), [0, 0, 2, 0]).unwrap();
        let solid = l.commands(RenderLayer::Solid, [sp(0), sp(1), sp(9)]);
        assert_eq!(solid.len(), 1); // пустой слой и отсутствующая секция пропущены
        assert_eq!(
            solid[0],
            IndirectCmd {
                index_count: 12,
                instance_count: 1,
                first_index: 0,
                base_vertex: 0,
                first_instance: 0
            }
        );
        let tr = l.commands(RenderLayer::Translucent, [sp(1), sp(0)]);
        assert_eq!(tr.len(), 2);
        assert_eq!(tr[0].first_instance, 1);
        assert_eq!(tr[0].index_count, 12);
        assert_eq!(tr[0].to_bytes().len(), 20);
    }

    #[test]
    fn vertex_layout_matches_packed_vertex() {
        let v = vertex_layout();
        assert_eq!(v.array_stride, 16);
        assert_eq!(v.attributes.len(), 3);
    }

    /// GPU-проверка: конвейеры создаются, арена принимает меш и рисует; без адаптера пропуск.
    #[test]
    fn gpu_smoke_if_adapter_present() {
        let Ok(gpu) = crate::Gpu::headless(crate::BackendChoice::Auto) else {
            eprintln!("нет адаптера — пропуск");
            return;
        };
        let fmt = wgpu::TextureFormat::Rgba8Unorm;
        let pipes =
            TerrainPipelines::new(&gpu.device, fmt, Some(wgpu::TextureFormat::Depth32Float));
        let mut arena = TerrainArena::new(&gpu.device, &gpu.queue, 4096, 4096, 16);
        let mut mesh = SectionMesh::default();
        let q = |z: f32| {
            let p = |x, y| PackedVertex::pack([x, y, z], [0.0; 2], [255; 4], 0, 240);
            [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0), p(1.0, 0.0)]
        };
        mesh.layers[0].extend(q(0.0));
        mesh.layers[2].extend(q(1.0));
        mesh.layers[2].extend(q(3.0));
        arena.upload(&gpu.queue, sp(0), &mesh, [0.0; 3]).unwrap();
        arena.prepare_draws(&gpu.queue, &[sp(0)], [0.0; 3]);
        assert_eq!(arena.draw_count(RenderLayer::Solid), 1);
        assert_eq!(arena.draw_count(RenderLayer::Cutout), 0);
        assert_eq!(arena.draw_count(RenderLayer::Translucent), 1);
        for l in RenderLayer::ALL {
            let _ = pipes.pipeline(l);
        }
        arena.remove(sp(0));
        assert!(arena.layout().is_empty());
    }
}
