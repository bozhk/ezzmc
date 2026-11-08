//! Молния. VANILLA: LightningBoltRenderer#submit (26.3), RenderTypes.lightning
//! (pipeline LIGHTNING: POSITION_COLOR, QUADS, blend SRC_ALPHA/ONE, глубина LEQUAL с записью).
use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use ezzmc_world::noise::LegacyRandom;
use glam::Mat4;

const VERTEX_SIZE: usize = 16;
/// (0.45, 0.45, 0.5, 0.3) в unorm8.
const COLOR: [u8; 4] = [115, 115, 128, 77];

/// Одна молния: позиция сущности и зерно формы.
#[derive(Clone, Copy, Debug)]
pub struct BoltInstance {
    pub pos: [f64; 3],
    pub seed: i64,
}

#[allow(clippy::too_many_arguments)]
fn quad(
    out: &mut Vec<[f32; 3]>,
    start: [f32; 2],
    end: [f32; 2],
    seg: i32,
    top_r: f32,
    bottom_r: f32,
    right_xp: bool,
    right_zp: bool,
    left_xp: bool,
    left_zp: bool,
) {
    let s = |p: bool, r: f32| if p { r } else { -r };
    let (y0, y1) = ((seg * 16) as f32, ((seg + 1) * 16) as f32);
    out.push([
        start[0] + s(right_xp, bottom_r),
        y0,
        start[1] + s(right_zp, bottom_r),
    ]);
    out.push([end[0] + s(right_xp, top_r), y1, end[1] + s(right_zp, top_r)]);
    out.push([end[0] + s(left_xp, top_r), y1, end[1] + s(left_zp, top_r)]);
    out.push([
        start[0] + s(left_xp, bottom_r),
        y0,
        start[1] + s(left_zp, bottom_r),
    ]);
}

/// VANILLA: LightningBoltRenderer#submit — вершины (по 4 на квад) относительно основания молнии.
pub fn build_bolt(seed: i64) -> Vec<[f32; 3]> {
    let mut x_off = [0.0f32; 8];
    let mut z_off = [0.0f32; 8];
    let (mut xo, mut zo) = (0.0f32, 0.0f32);
    let mut rnd = LegacyRandom::new(seed);
    for i in (0..8).rev() {
        x_off[i] = xo;
        z_off[i] = zo;
        xo += (rnd.next_int_bound(11) - 5) as f32;
        zo += (rnd.next_int_bound(11) - 5) as f32;
    }
    let mut out = Vec::with_capacity(4 * 14 * 4 * 4);
    for layer in 0..4 {
        let mut r = LegacyRandom::new(seed);
        for branch in 0..3 {
            let trunk = branch == 0;
            let start_seg = 7 - branch;
            let end_seg = if trunk { 0 } else { start_seg - 3 + 1 };
            let mut sx = x_off[start_seg as usize] - xo;
            let mut sz = z_off[start_seg as usize] - zo;
            for seg in (end_seg..=start_seg).rev() {
                let (ex, ez) = (sx, sz);
                if trunk {
                    sx += (r.next_int_bound(11) - 5) as f32;
                    sz += (r.next_int_bound(11) - 5) as f32;
                } else {
                    sx += (r.next_int_bound(31) - 15) as f32;
                    sz += (r.next_int_bound(31) - 15) as f32;
                }
                let mut top = 0.1 + layer as f32 * 0.2;
                let mut bottom = top;
                if trunk {
                    top *= seg as f32 * 0.1 + 1.0;
                    bottom *= (seg - 1) as f32 * 0.1 + 1.0;
                }
                let (s, e) = ([sx, sz], [ex, ez]);
                quad(&mut out, s, e, seg, top, bottom, false, false, true, false);
                quad(&mut out, s, e, seg, top, bottom, true, false, true, true);
                quad(&mut out, s, e, seg, top, bottom, true, true, false, true);
                quad(&mut out, s, e, seg, top, bottom, false, true, false, false);
            }
        }
    }
    out
}

pub struct LightningDraw {
    pipeline: wgpu::RenderPipeline,
    group0: wgpu::BindGroup,
    dyn_buf: wgpu::Buffer,
    vb: Option<wgpu::Buffer>,
    ib: wgpu::Buffer,
    ib_quads: u32,
    quads: u32,
}

impl LightningDraw {
    pub fn new(
        device: &wgpu::Device,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
    ) -> Self {
        let uni = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let l0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lightning.group0"),
            entries: &[uni(0), uni(1), uni(3)],
        });
        let dyn_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lightning.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        fn entry(binding: u32, b: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
            wgpu::BindGroupEntry {
                binding,
                resource: b.as_entire_binding(),
            }
        }
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lightning.group0"),
            layout: &l0,
            entries: &[entry(0, projection), entry(1, fog), entry(3, &dyn_buf)],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("lightning.pl"),
            bind_group_layouts: &[&l0],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("lightning"),
            source: wgpu::ShaderSource::Wgsl(Shader::Lightning.source().into()),
        });
        let attrs = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Unorm8x4,
                offset: 12,
                shader_location: 1,
            },
        ];
        // BlendFunction.LIGHTNING: SRC_ALPHA / ONE (и для цвета, и для альфы).
        let comp = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("lightning"),
            layout: Some(&pl),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: VERTEX_SIZE as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &attrs,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color,
                    blend: Some(wgpu::BlendState {
                        color: comp,
                        alpha: comp,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // Квады видны с обеих сторон (в ванили cull отключён в RenderPipeline по умолчанию).
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let ib = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lightning.ib"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        LightningDraw {
            pipeline,
            group0,
            dyn_buf,
            vb: None,
            ib,
            ib_quads: 0,
            quads: 0,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        cam: [f64; 3],
        bolts: &[BoltInstance],
    ) {
        self.quads = 0;
        if bolts.is_empty() {
            return;
        }
        let mut data = Vec::new();
        for b in bolts {
            let o = [
                (b.pos[0] - cam[0]) as f32,
                (b.pos[1] - cam[1]) as f32,
                (b.pos[2] - cam[2]) as f32,
            ];
            for v in build_bolt(b.seed) {
                for c in [v[0] + o[0], v[1] + o[1], v[2] + o[2]] {
                    data.extend_from_slice(&c.to_le_bytes());
                }
                data.extend_from_slice(&COLOR);
            }
        }
        let quads = (data.len() / (VERTEX_SIZE * 4)) as u32;
        if self
            .vb
            .as_ref()
            .is_none_or(|b| b.size() < data.len() as u64)
        {
            self.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("lightning.vb"),
                size: (data.len() as u64).next_power_of_two(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        if let Some(vb) = &self.vb {
            queue.write_buffer(vb, 0, &data);
        }
        if self.ib_quads < quads {
            let n = quads.next_power_of_two();
            let mut idx = Vec::with_capacity(n as usize * 24);
            for q in 0..n {
                for k in [0u32, 1, 2, 2, 3, 0] {
                    idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
                }
            }
            self.ib = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("lightning.ib"),
                size: idx.len() as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&self.ib, 0, &idx);
            self.ib_quads = n;
        }
        queue.write_buffer(
            &self.dyn_buf,
            0,
            &DynamicTransformsUniform {
                model_view_mat: model_view,
                color_modulator: [1.0; 4],
                model_offset: [0.0; 3],
                texture_mat: Mat4::IDENTITY,
            }
            .to_bytes(),
        );
        self.quads = quads;
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vb) = &self.vb else { return };
        if self.quads == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, vb.slice(..));
        pass.draw_indexed(0..self.quads * 6, 0, 0..1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bolt_has_56_segments_of_four_quads() {
        // 4 слоя × (ствол 8 + две ветки по 3) сегментов × 4 квада × 4 вершины.
        assert_eq!(build_bolt(12345).len(), 4 * 14 * 4 * 4);
    }

    #[test]
    fn bolt_is_deterministic_and_spans_128_blocks() {
        let a = build_bolt(99);
        assert_eq!(a, build_bolt(99));
        let top = a.iter().map(|v| v[1]).fold(0.0, f32::max);
        assert_eq!(top, 128.0);
    }
}
