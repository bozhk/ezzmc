//! Compute-culling секций террейна на GPU (TODO-490).
//!
//! CPU отдаёт только флаги ванильного cave-culling (`TerrainArena::set_visible`, при изменении графа),
//! frustum-тест и сборку индирект-команд делает compute-шейдер каждый кадр:
//! * есть `MULTI_DRAW_INDIRECT_COUNT` — компактный вывод + счётчик, `multi_draw_indexed_indirect_count`;
//! * иначе — вывод «слот→слот» с `instance_count` 0/1, `multi_draw_indexed_indirect` по числу слотов.
//!
//! Прозрачный слой сюда не входит: ваниль рисует его строго от дальних секций к ближним, а GPU-вывод
//! порядок не хранит (§0 — вид не меняется), поэтому он остаётся на CPU-пути `prepare_draws`.
//! Без `INDIRECT_FIRST_INSTANCE` GPU-путь недоступен (`GpuCuller::new` → `None`).
use crate::terrain::{IndirectCmd, TerrainArena};
use ezzmc_mesh::frustum::Frustum;
use ezzmc_mesh::layers::RenderLayer;

const SHADER: &str = include_str!("shaders/cull.wgsl");
const PARAMS_SIZE: u64 = 6 * 16 + 16 + 16 + 16;
const WORKGROUP: u32 = 64;

/// Слои, которые отсекаются на GPU (всё, кроме прозрачного).
pub fn gpu_layer_mask() -> u32 {
    RenderLayer::ALL
        .iter()
        .filter(|l| **l != RenderLayer::Translucent)
        .fold(0, |m, l| m | (1 << l.index()))
}

pub struct GpuCuller {
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    params: wgpu::Buffer,
    dst: wgpu::Buffer,
    counts: wgpu::Buffer,
    compact: bool,
    max_slots: u32,
}

impl GpuCuller {
    pub fn new(device: &wgpu::Device, arena: &TerrainArena) -> Option<Self> {
        if !arena.first_instance_ok() {
            return None;
        }
        let compact = device
            .features()
            .contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT);
        let max_slots = arena.max_slots();
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cull.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let entry = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let rw = wgpu::BufferBindingType::Storage { read_only: false };
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cull.bgl"),
            entries: &[
                entry(0, wgpu::BufferBindingType::Uniform),
                entry(1, ro),
                entry(2, ro),
                entry(3, ro),
                entry(4, rw),
                entry(5, rw),
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cull.layout"),
            bind_group_layouts: &[&bgl],
            immediate_size: 0,
        });
        let constants = [("COMPACT", if compact { 1.0 } else { 0.0 })];
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("cull.pipeline"),
            layout: Some(&layout),
            module: &module,
            entry_point: Some("cull"),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
            cache: None,
        });
        let mk = |label: &str, size: u64, usage: wgpu::BufferUsages| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size.max(16),
                usage,
                mapped_at_creation: false,
            })
        };
        let params = mk(
            "cull.params",
            PARAMS_SIZE,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );
        let dst = mk(
            "cull.dst",
            u64::from(max_slots) * IndirectCmd::SIZE * RenderLayer::COUNT as u64,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::INDIRECT
                | wgpu::BufferUsages::COPY_SRC,
        );
        let counts = mk(
            "cull.counts",
            RenderLayer::COUNT as u64 * 4,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::INDIRECT
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cull.bg"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: arena.origins_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: arena.flags_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: arena.src_cmds_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: dst.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: counts.as_entire_binding(),
                },
            ],
        });
        Some(GpuCuller {
            pipeline,
            bind_group,
            params,
            dst,
            counts,
            compact,
            max_slots,
        })
    }

    pub fn is_compact(&self) -> bool {
        self.compact
    }

    /// Записывает compute-проход отсечения. Вызывать до render pass, в котором идёт `draw`.
    pub fn cull(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        arena: &mut TerrainArena,
        frustum: &Frustum,
    ) {
        arena.flush_removed(queue);
        let slots = arena.slot_count();
        let cam = frustum.camera();
        let fl = cam.floor();
        let mut b = Vec::with_capacity(PARAMS_SIZE as usize);
        for p in frustum.planes() {
            for v in p.to_array() {
                b.extend_from_slice(&v.to_le_bytes());
            }
        }
        for v in [fl.x as i32, fl.y as i32, fl.z as i32, 0] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in [
            (cam.x - fl.x) as f32,
            (cam.y - fl.y) as f32,
            (cam.z - fl.z) as f32,
            0.0,
        ] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in [
            slots,
            self.max_slots,
            gpu_layer_mask(),
            RenderLayer::COUNT as u32,
        ] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        queue.write_buffer(&self.params, 0, &b);
        encoder.clear_buffer(&self.counts, 0, None);
        if slots == 0 {
            return;
        }
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("terrain.cull"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.dispatch_workgroups(slots.div_ceil(WORKGROUP), 1, 1);
    }

    /// Рисует отсечённый слой (конвейер и привязки — на вызывающем). Только непрозрачные слои.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, arena: &TerrainArena, layer: RenderLayer) {
        debug_assert!(layer != RenderLayer::Translucent);
        let off = layer.index() as u64 * u64::from(self.max_slots) * IndirectCmd::SIZE;
        pass.set_vertex_buffer(0, arena.vertex_buf.slice(..));
        pass.set_index_buffer(arena.index_buf.slice(..), wgpu::IndexFormat::Uint32);
        if self.compact {
            pass.multi_draw_indexed_indirect_count(
                &self.dst,
                off,
                &self.counts,
                layer.index() as u64 * 4,
                self.max_slots,
            );
        } else {
            pass.multi_draw_indexed_indirect(&self.dst, off, arena.slot_count());
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_mesh::frustum::look_matrix;
    use ezzmc_mesh::section::{SectionMesh, SectionPos};
    use ezzmc_mesh::vertex::PackedVertex;
    use glam::Mat4;

    #[test]
    fn cull_shader_validates() {
        let module = naga::front::wgsl::parse_str(SHADER).unwrap();
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();
    }

    fn read(
        gpu: &crate::Gpu,
        src: &wgpu::Buffer,
        size: u64,
        enc_extra: impl FnOnce(&mut wgpu::CommandEncoder),
    ) -> Vec<u32> {
        let stage = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut enc = gpu.device.create_command_encoder(&Default::default());
        enc_extra(&mut enc);
        enc.copy_buffer_to_buffer(src, 0, &stage, 0, size);
        gpu.queue.submit([enc.finish()]);
        stage
            .slice(..)
            .map_async(wgpu::MapMode::Read, |r| r.unwrap());
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        let data = stage.slice(..).get_mapped_range();
        data.chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect()
    }

    /// GPU-отсечение даёт тот же набор секций, что и CPU `Frustum::section_visible`.
    #[test]
    fn gpu_matches_cpu_frustum() {
        let Ok(gpu) = crate::Gpu::headless(crate::BackendChoice::Auto) else {
            eprintln!("нет адаптера — пропуск");
            return;
        };
        let mut arena = TerrainArena::new(&gpu.device, &gpu.queue, 1 << 14, 1 << 12, 2048);
        arena.track_src_cmds = true;
        let Some(culler) = GpuCuller::new(&gpu.device, &arena) else {
            eprintln!("нет INDIRECT_FIRST_INSTANCE — пропуск");
            return;
        };
        let mut mesh = SectionMesh::default();
        let p = |x, y| PackedVertex::pack([x, y, 0.0], [0.0; 2], [255; 4], 0, 240);
        mesh.layers[0].extend([p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0), p(1.0, 0.0)]);
        let mut all = Vec::new();
        for x in -6..=6 {
            for y in 0..8 {
                for z in -6..=6 {
                    let pos = SectionPos { x, y, z };
                    arena.upload(&gpu.queue, pos, &mesh, [0.0; 3]).unwrap();
                    all.push(pos);
                }
            }
        }
        arena.set_visible(&gpu.queue, &all);
        let proj = Mat4::perspective_rh(70f32.to_radians(), 16.0 / 9.0, 0.05, 512.0);
        let mut f = Frustum::new(proj, look_matrix(35.0, 10.0), true);
        f.prepare(0.37, 64.81, -0.5);
        let expect: std::collections::HashSet<u32> = all
            .iter()
            .filter(|p| f.section_visible(p.x, p.y, p.z))
            .map(|p| arena.layout().get(*p).unwrap().slot)
            .collect();
        assert!(!expect.is_empty() && expect.len() < all.len());

        let max = arena.max_slots() as usize;
        let size = (max * 20 * RenderLayer::COUNT) as u64;
        let words = read(&gpu, &culler.dst, size, |enc| {
            culler.cull(&gpu.queue, enc, &mut arena, &f);
        });
        let counts = read(&gpu, &culler.counts, RenderLayer::COUNT as u64 * 4, |_| {});
        let solid = RenderLayer::Solid.index();
        let base = solid * max * 5;
        let got: std::collections::HashSet<u32> = if culler.is_compact() {
            (0..counts[solid] as usize)
                .map(|i| words[base + i * 5 + 4])
                .collect()
        } else {
            (0..arena.slot_count() as usize)
                .filter(|i| words[base + i * 5 + 1] == 1)
                .map(|i| words[base + i * 5 + 4])
                .collect()
        };
        assert_eq!(got, expect);
    }
}
