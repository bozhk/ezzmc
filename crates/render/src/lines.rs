//! Линии заданной толщины поверх мира (контур выбранного блока).
//!
//! VANILLA: LevelRenderer#renderHitOutline → ShapeRenderer#renderShape, RenderPipelines.LINES.
//! wgpu не умеет `lineWidth`, поэтому отрезок рисуется квадом в экранном пространстве.
use glam::Mat4;

const MAX_SEGMENTS: usize = 256;

pub struct LineDraw {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform: wgpu::Buffer,
    vb: wgpu::Buffer,
    count: u32,
}

impl LineDraw {
    pub fn new(
        device: &wgpu::Device,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
    ) -> Self {
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lines u"),
            size: 96,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vb = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lines vb"),
            size: (MAX_SEGMENTS * 24) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lines"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lines"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("lines"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/lines.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("lines"),
            bind_group_layouts: &[&bgl],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("lines"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            bind_group,
            uniform,
            vb,
            count: 0,
        }
    }

    /// Отрезки в координатах относительно камеры. `width` — в пикселях экрана.
    pub fn set(
        &mut self,
        queue: &wgpu::Queue,
        view_proj: Mat4,
        screen: [f32; 2],
        color: [f32; 4],
        width: f32,
        segments: &[([f32; 3], [f32; 3])],
    ) {
        let n = segments.len().min(MAX_SEGMENTS);
        self.count = n as u32;
        if n == 0 {
            return;
        }
        let mut u: Vec<f32> = view_proj.to_cols_array().to_vec();
        u.extend_from_slice(&color);
        u.extend_from_slice(&[screen[0], screen[1], width, 5e-5]);
        queue.write_buffer(&self.uniform, 0, floats(&u));
        let v: Vec<f32> = segments[..n]
            .iter()
            .flat_map(|(a, b)| [a[0], a[1], a[2], b[0], b[1], b[2]])
            .collect();
        queue.write_buffer(&self.vb, 0, floats(&v));
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vb.slice(..));
        pass.draw(0..6, 0..self.count);
    }
}

fn floats(v: &[f32]) -> &[u8] {
    // SAFETY: f32 — POD без padding.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

/// 12 рёбер бокса `[x0,y0,z0,x1,y1,z1]`, сдвинутого на `off`.
pub fn box_edges(b: &[f64; 6], off: [f32; 3]) -> Vec<([f32; 3], [f32; 3])> {
    let p = |x: usize, y: usize, z: usize| {
        [
            b[x] as f32 + off[0],
            b[y] as f32 + off[1],
            b[z] as f32 + off[2],
        ]
    };
    let mut v = Vec::with_capacity(12);
    for (y, z) in [(1, 2), (1, 5), (4, 2), (4, 5)] {
        v.push((p(0, y, z), p(3, y, z)));
    }
    for (x, z) in [(0, 2), (0, 5), (3, 2), (3, 5)] {
        v.push((p(x, 1, z), p(x, 4, z)));
    }
    for (x, y) in [(0, 1), (0, 4), (3, 1), (3, 4)] {
        v.push((p(x, y, 2), p(x, y, 5)));
    }
    v
}
