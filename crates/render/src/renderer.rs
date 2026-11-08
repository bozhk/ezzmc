//! Воксельный рендер секций, путь `ComputeIndirect` (mesh-путь — заглушка, см. `voxel_mesh_stub.txt`).
//!
//! Все вершины — в одном монолитном storage-буфере, CPU пишет только изменившийся диапазон секции.
//! Кадр: `encode_cull` (compute: frustum + Hi-Z → indirect-команды) → `draw` (multi-draw-indirect)
//! → `HiZ::build` из глубины кадра (используется в culling следующего кадра).
//!
//! Потокобезопасность: все wgpu-ресурсы `Send + Sync`; мутирующие методы — `&mut self`
//! (владелец один, меш-потоки передают готовые данные через канал), рисование — `&self`.
use crate::gpu::Gpu;
use crate::hiz::HiZ;
use crate::pipeline_mode::RenderPipelineMode;
use crate::terrain::{quad_pattern, RangeAllocator};
use crate::voxel_vertex::{QuadMeta, VoxelVertex};
use std::collections::HashMap;

const CULL_SHADER: &str = include_str!("shaders/voxel_cull.wgsl");
const DRAW_SHADER: &str = include_str!("shaders/voxel_draw.wgsl");
const SECTION_SIZE: u64 = 32;
const INDIRECT_SIZE: u64 = 20;
pub const PARAMS_SIZE: u64 = 208;
/// Максимум квадов в секции (16³/2 блоков × 6 граней в худшем «шахматном» случае, с запасом).
pub const MAX_QUADS_PER_SECTION: u32 = 16384;

/// Камера кадра. Матрицы — camera-relative (камера в начале координат), z ∈ [0,1].
#[derive(Clone, Copy, Debug)]
pub struct CullCamera {
    pub view_proj: glam::Mat4,
    pub pos: [f64; 3],
    /// Матрица и позиция камеры кадра, из чьей глубины построен Hi-Z.
    pub prev_view_proj: glam::Mat4,
    pub prev_pos: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
struct Placed {
    slot: u32,
    first_vertex: u32,
    vertex_count: u32,
}

pub struct Renderer {
    mode: RenderPipelineMode,
    compact: bool,
    vertices: wgpu::Buffer,
    quads: wgpu::Buffer,
    sections: wgpu::Buffer,
    draws: wgpu::Buffer,
    count: wgpu::Buffer,
    params: wgpu::Buffer,
    fog: wgpu::Buffer,
    rects: wgpu::Buffer,
    index: wgpu::Buffer,
    dummy_hiz: HiZ,
    cull: wgpu::ComputePipeline,
    draw: wgpu::RenderPipeline,
    atlas_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    lightmap_view: wgpu::TextureView,
    lm_sampler: wgpu::Sampler,
    draw_bg: wgpu::BindGroup,
    alloc: RangeAllocator,
    placed: HashMap<[i32; 3], Placed>,
    free_slots: Vec<u32>,
    next_slot: u32,
    max_sections: u32,
}

/// Минимальные требования пути ComputeIndirect (иначе — старый `TerrainArena`).
pub fn supported(gpu: &Gpu) -> bool {
    let dl = gpu.adapter.get_downlevel_capabilities().flags;
    gpu.device
        .features()
        .contains(wgpu::Features::INDIRECT_FIRST_INSTANCE)
        && dl.contains(
            wgpu::DownlevelFlags::COMPUTE_SHADERS
                | wgpu::DownlevelFlags::VERTEX_STORAGE
                | wgpu::DownlevelFlags::INDIRECT_EXECUTION,
        )
        && gpu.device.limits().max_storage_buffers_per_shader_stage >= 5
}

impl Renderer {
    /// `None` — GPU не поддерживает путь (см. `supported`) или режим не `ComputeIndirect`.
    pub fn new(
        gpu: &Gpu,
        mode: RenderPipelineMode,
        color_format: wgpu::TextureFormat,
        vertex_capacity: u32,
        max_sections: u32,
    ) -> Option<Self> {
        if mode != RenderPipelineMode::ComputeIndirect || !supported(gpu) {
            return None;
        }
        let device = &gpu.device;
        let compact = device
            .features()
            .contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT);
        let buf = |label, size: u64, usage| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size.max(4),
                usage: usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let st = wgpu::BufferUsages::STORAGE;
        let vertices = buf(
            "voxel vertices",
            u64::from(vertex_capacity) * VoxelVertex::SIZE as u64,
            st,
        );
        let quads = buf(
            "voxel quads",
            u64::from(vertex_capacity / 4) * size_of::<QuadMeta>() as u64,
            st,
        );
        let sections = buf("voxel sections", u64::from(max_sections) * SECTION_SIZE, st);
        let draws = buf(
            "voxel draws",
            u64::from(max_sections) * INDIRECT_SIZE,
            st | wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_SRC,
        );
        let count = buf(
            "voxel count",
            4,
            st | wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_SRC,
        );
        let params = buf("voxel params", PARAMS_SIZE, wgpu::BufferUsages::UNIFORM);
        let rects = buf("voxel atlas rects", 16, st);
        gpu.queue
            .write_buffer(&rects, 0, as_bytes(&[[0.0f32, 0.0, 1.0, 1.0]]));
        let index = buf(
            "voxel quad indices",
            u64::from(MAX_QUADS_PER_SECTION) * 6 * 4,
            wgpu::BufferUsages::INDEX,
        );
        gpu.queue
            .write_buffer(&index, 0, as_bytes(&quad_pattern(MAX_QUADS_PER_SECTION)));

        // «пустой» Hi-Z 1×1 (глубина 1.0 = ничего не закрывает) для кадров без Hi-Z
        let dummy_hiz = HiZ::new(device, 1, 1);

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("voxel_cull.wgsl"),
            source: wgpu::ShaderSource::Wgsl(CULL_SHADER.into()),
        });
        let cull = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("voxel cull"),
            layout: None,
            module: &module,
            entry_point: Some("cs_main"),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &[("COMPACT", if compact { 1.0 } else { 0.0 })],
                ..Default::default()
            },
            cache: None,
        });

        let dmod = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("voxel_draw.wgsl"),
            source: wgpu::ShaderSource::Wgsl(DRAW_SHADER.into()),
        });
        let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("voxel draw"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &dmod,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &dmod,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(color_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                // VANILLA: LEQUAL — оверлей травы лежит в той же плоскости, что и основа, и должен её перекрыть.
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                // Как у Cutout/Tripwire в terrain.rs: ковры/мох лежат в этом же буфере вплотную
                // к кубам, без смещения глубины мерцают (Z-fighting).
                bias: wgpu::DepthBiasState {
                    constant: -2,
                    slope_scale: -1.0,
                    clamp: 0.0,
                },
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        let atlas = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("voxel atlas placeholder"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        gpu.queue.write_texture(
            atlas.as_image_copy(),
            &[255, 255, 255, 255],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        let atlas_view = atlas.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let lm_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let lightmap_view = atlas_view.clone();
        let fog = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("voxel fog"),
            size: crate::uniforms::FogUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let draw_bg = draw_bind_group(
            device,
            &draw,
            [&params, &sections, &vertices, &quads, &rects, &fog],
            &atlas_view,
            &sampler,
            &lightmap_view,
            &lm_sampler,
        );
        Some(Self {
            mode,
            compact,
            vertices,
            quads,
            sections,
            draws,
            count,
            params,
            fog,
            rects,
            index,
            dummy_hiz,
            cull,
            draw,
            draw_bg,
            atlas_view,
            lightmap_view,
            lm_sampler,
            sampler,
            alloc: RangeAllocator::new(vertex_capacity),
            placed: HashMap::new(),
            free_slots: Vec::new(),
            next_slot: 0,
            max_sections,
        })
    }

    fn rebuild_draw_bg(&mut self, device: &wgpu::Device) {
        self.draw_bg = draw_bind_group(
            device,
            &self.draw,
            [
                &self.params,
                &self.sections,
                &self.vertices,
                &self.quads,
                &self.rects,
                &self.fog,
            ],
            &self.atlas_view,
            &self.sampler,
            &self.lightmap_view,
            &self.lm_sampler,
        );
    }

    /// Туман кадра (те же значения, что у ванильного пути).
    pub fn set_fog(&self, queue: &wgpu::Queue, fog: &crate::uniforms::FogUniform) {
        queue.write_buffer(&self.fog, 0, &fog.to_bytes());
    }

    /// Атлас блоков и прямоугольники UV по texture id (`[u0, v0, u1, v1]`, индекс = `QuadMeta::texture`).
    pub fn set_atlas(
        &mut self,
        gpu: &Gpu,
        atlas: wgpu::TextureView,
        lightmap: wgpu::TextureView,
        rects: &[[f32; 4]],
    ) {
        let size = (rects.len().max(1) * 16) as u64;
        if size > self.rects.size() {
            self.rects = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("voxel atlas rects"),
                size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        gpu.queue.write_buffer(&self.rects, 0, as_bytes(rects));
        self.atlas_view = atlas;
        self.lightmap_view = lightmap;
        self.rebuild_draw_bg(&gpu.device);
    }

    pub fn mode(&self) -> RenderPipelineMode {
        self.mode
    }
    pub fn slot_count(&self) -> u32 {
        self.next_slot
    }
    pub fn section_count(&self) -> usize {
        self.placed.len()
    }
    pub fn used_vertices(&self) -> u32 {
        self.alloc.used()
    }

    /// Принимает меш секции `pos` (индексы секции), заменяя прежний. `false` — нет места
    /// (вершин, слотов или квадов > `MAX_QUADS_PER_SECTION`); прежний меш при этом уже удалён.
    pub fn upload_section(
        &mut self,
        queue: &wgpu::Queue,
        pos: [i32; 3],
        verts: &[VoxelVertex],
        quads: &[QuadMeta],
    ) -> bool {
        assert_eq!(verts.len(), quads.len() * 4, "4 вершины на квад");
        self.remove_section(queue, pos);
        if quads.is_empty() {
            return true;
        }
        if quads.len() as u32 > MAX_QUADS_PER_SECTION {
            return false;
        }
        let n = verts.len() as u32;
        let slot = match self.free_slots.pop() {
            Some(s) => s,
            None if self.next_slot < self.max_sections => {
                self.next_slot += 1;
                self.next_slot - 1
            }
            None => return false,
        };
        let Some(first) = self.alloc.alloc(n) else {
            self.free_slots.push(slot);
            return false;
        };
        queue.write_buffer(&self.vertices, u64::from(first) * 4, as_bytes(verts));
        queue.write_buffer(&self.quads, u64::from(first / 4) * 8, as_bytes(quads));
        let o = pos.map(|c| c * 16);
        let w = [o[0] as u32, o[1] as u32, o[2] as u32, first, n / 4, 0, 0, 0];
        queue.write_buffer(&self.sections, u64::from(slot) * SECTION_SIZE, as_bytes(&w));
        self.placed.insert(
            pos,
            Placed {
                slot,
                first_vertex: first,
                vertex_count: n,
            },
        );
        true
    }

    pub fn remove_section(&mut self, queue: &wgpu::Queue, pos: [i32; 3]) {
        if let Some(p) = self.placed.remove(&pos) {
            self.alloc.free(p.first_vertex, p.vertex_count);
            // quad_count = 0 → cull пропускает слот
            queue.write_buffer(
                &self.sections,
                u64::from(p.slot) * SECTION_SIZE + 16,
                &0u32.to_le_bytes(),
            );
            self.free_slots.push(p.slot);
        }
    }

    fn write_params(&self, queue: &wgpu::Queue, cam: &CullCamera, hiz: Option<&HiZ>) {
        let mut b = Vec::with_capacity(PARAMS_SIZE as usize);
        b.extend_from_slice(as_bytes(&cam.view_proj.to_cols_array()));
        b.extend_from_slice(as_bytes(&cam.prev_view_proj.to_cols_array()));
        for pos in [cam.pos, cam.prev_pos] {
            let fl = pos.map(f64::floor);
            b.extend_from_slice(as_bytes(&[fl[0] as i32, fl[1] as i32, fl[2] as i32, 0]));
            b.extend_from_slice(as_bytes(&[
                (pos[0] - fl[0]) as f32,
                (pos[1] - fl[1]) as f32,
                (pos[2] - fl[2]) as f32,
                0.0,
            ]));
        }
        let mut o = b;
        o.extend_from_slice(as_bytes(&[
            self.next_slot,
            u32::from(hiz.is_some()),
            hiz.map_or(1, HiZ::mips),
            0,
        ]));
        debug_assert_eq!(o.len() as u64, PARAMS_SIZE);
        queue.write_buffer(&self.params, 0, &o);
    }

    /// Culling на GPU: frustum и (если есть `hiz`) Hi-Z прошлого кадра. Пишет indirect-команды.
    pub fn encode_cull(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        cam: &CullCamera,
        hiz: Option<&HiZ>,
    ) {
        if self.next_slot == 0 {
            return;
        }
        self.write_params(&gpu.queue, cam, hiz);
        if self.compact {
            encoder.clear_buffer(&self.count, 0, None);
        }
        let hv = hiz.unwrap_or(&self.dummy_hiz).view();
        let e = |binding, resource| wgpu::BindGroupEntry { binding, resource };
        let bg = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("voxel cull"),
            layout: &self.cull.get_bind_group_layout(0),
            entries: &[
                e(0, self.params.as_entire_binding()),
                e(1, self.sections.as_entire_binding()),
                e(2, self.draws.as_entire_binding()),
                e(3, self.count.as_entire_binding()),
                e(4, wgpu::BindingResource::TextureView(hv)),
            ],
        });
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.cull);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(self.next_slot.div_ceil(64), 1, 1);
    }

    /// Рисует секции, прошедшие culling. Вызывать после `encode_cull` в том же энкодере.
    /// Глубина — `Depth32Float` со `Less`.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.next_slot == 0 {
            return;
        }
        pass.set_pipeline(&self.draw);
        pass.set_bind_group(0, &self.draw_bg, &[]);
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        if self.compact {
            pass.multi_draw_indexed_indirect_count(&self.draws, 0, &self.count, 0, self.next_slot);
        } else {
            pass.multi_draw_indexed_indirect(&self.draws, 0, self.next_slot);
        }
    }

    /// Сколько секций прошло culling (блокирующее чтение с GPU — для тестов и отладки, не для кадра).
    pub fn read_visible_count(&self, gpu: &Gpu) -> u32 {
        let n = u64::from(self.next_slot);
        let read = |src: &wgpu::Buffer, bytes: u64| -> Vec<u32> {
            let dst = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("readback"),
                size: bytes,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let mut enc = gpu.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(src, 0, &dst, 0, bytes);
            gpu.queue.submit([enc.finish()]);
            dst.slice(..).map_async(wgpu::MapMode::Read, |_| ());
            let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
            let v = dst
                .slice(..)
                .get_mapped_range()
                .chunks_exact(4)
                .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            v
        };
        if self.compact {
            read(&self.count, 4)[0]
        } else {
            read(&self.draws, n * INDIRECT_SIZE)
                .chunks_exact(5)
                .map(|c| c[1])
                .sum()
        }
    }

    pub fn is_compact(&self) -> bool {
        self.compact
    }
}

fn draw_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::RenderPipeline,
    b: [&wgpu::Buffer; 6],
    atlas: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
    lightmap: &wgpu::TextureView,
    lm_sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    let e = |binding, resource| wgpu::BindGroupEntry { binding, resource };
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("voxel draw"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            e(0, b[0].as_entire_binding()),
            e(1, b[1].as_entire_binding()),
            e(5, b[2].as_entire_binding()),
            e(6, b[3].as_entire_binding()),
            e(7, b[4].as_entire_binding()),
            e(10, b[5].as_entire_binding()),
            e(8, wgpu::BindingResource::TextureView(atlas)),
            e(9, wgpu::BindingResource::Sampler(sampler)),
            e(11, wgpu::BindingResource::TextureView(lightmap)),
            e(12, wgpu::BindingResource::Sampler(lm_sampler)),
        ],
    })
}

fn as_bytes<T: Copy>(s: &[T]) -> &[u8] {
    // SAFETY: T — POD без padding (u32/i32/f32/массивы, VoxelVertex, QuadMeta).
    unsafe { std::slice::from_raw_parts(s.as_ptr().cast(), std::mem::size_of_val(s)) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cull_and_draw_shaders_valid() {
        for src in [super::CULL_SHADER, super::DRAW_SHADER] {
            let m = naga::front::wgsl::parse_str(src).expect("wgsl");
            naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::all())
                .validate(&m)
                .expect("valid");
        }
    }
}
