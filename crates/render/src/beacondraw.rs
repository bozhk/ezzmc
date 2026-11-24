//! Луч маяка (и шлюза Энда). VANILLA: BeaconRenderer#submitBeaconBeam, RenderTypes.beaconBeam,
//! pipelines BEACON_BEAM_OPAQUE (без смешения, глубина с записью) и BEACON_BEAM_TRANSLUCENT
//! (смешение TRANSLUCENT, глубина без записи). Геометрию строит клиент (`beam_quads`).
use crate::endportaldraw::Rgba;
use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use glam::Mat4;

/// Позиция (относительно камеры), UV, цвет RGBA8.
pub type BeamVertex = ([f32; 3], [f32; 2], [u8; 4]);

/// Число текстур луча: 0 — маяк, 1 — шлюз Энда.
pub const TEXTURES: usize = 2;

/// Вершины одного кадра: `[текстура][0 = сплошной слой, 1 = свечение]`, по 4 вершины на квад.
pub type BeamBatches = [[Vec<BeamVertex>; 2]; TEXTURES];

#[derive(Default)]
struct Layer {
    vb: Option<wgpu::Buffer>,
    quads: u32,
}

pub struct BeaconDraw {
    pipelines: [wgpu::RenderPipeline; 2],
    group0: wgpu::BindGroup,
    groups: Vec<wgpu::BindGroup>,
    dyn_buf: wgpu::Buffer,
    layers: [[Layer; 2]; TEXTURES],
    ib: wgpu::Buffer,
    ib_quads: u32,
}

fn make_view(device: &wgpu::Device, queue: &wgpu::Queue, img: &Rgba<'_>) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: img.width,
        height: img.height,
        depth_or_array_layers: 1,
    };
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("beacon_beam.tex"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        img.data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(img.width * 4),
            rows_per_image: Some(img.height),
        },
        size,
    );
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}

impl BeaconDraw {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        textures: [&Rgba<'_>; TEXTURES],
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
            label: Some("beacon_beam.group0"),
            entries: &[uni(0), uni(1), uni(3)],
        });
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("beacon_beam.group1"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let dyn_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("beacon_beam.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("beacon_beam.group0"),
            layout: &l0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fog.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: dyn_buf.as_entire_binding(),
                },
            ],
        });
        // RenderType: blur=false, mipmap=false; GL_REPEAT (V прокручивается за пределы 0..1).
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("beacon_beam.sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let groups = textures
            .iter()
            .map(|t| {
                let view = make_view(device, queue, t);
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("beacon_beam.group1"),
                    layout: &l1,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                })
            })
            .collect();
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("beacon_beam.pl"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("beacon_beam"),
            source: wgpu::ShaderSource::Wgsl(Shader::BeaconBeam.source().into()),
        });
        let attrs = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 12,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Unorm8x4,
                offset: 20,
                shader_location: 2,
            },
        ];
        // VANILLA: ColorTargetState(BlendFunction.TRANSLUCENT).
        let translucent = wgpu::BlendState {
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
        let make = |label: &str, blend: Option<wgpu::BlendState>, write: bool| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&pl),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 24,
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
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                // VANILLA-UNSURE: отсечение граней не включено (ориентация квадов не сверена).
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: depth,
                    depth_write_enabled: write,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipelines = [
            make("beacon_beam.opaque", None, true),
            make("beacon_beam.translucent", Some(translucent), false),
        ];
        let ib = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("beacon_beam.ib"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        BeaconDraw {
            pipelines,
            group0,
            groups,
            dyn_buf,
            layers: Default::default(),
            ib,
            ib_quads: 0,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        batches: &BeamBatches,
    ) {
        let max_quads = batches
            .iter()
            .flatten()
            .map(|v| (v.len() / 4) as u32)
            .max()
            .unwrap_or(0);
        if self.ib_quads < max_quads {
            let n = max_quads.next_power_of_two();
            let mut idx = Vec::with_capacity(n as usize * 24);
            for q in 0..n {
                for k in [0u32, 1, 2, 2, 3, 0] {
                    idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
                }
            }
            self.ib = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("beacon_beam.ib"),
                size: idx.len() as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&self.ib, 0, &idx);
            self.ib_quads = n;
        }
        for (t, pair) in batches.iter().enumerate() {
            for (k, verts) in pair.iter().enumerate() {
                let layer = &mut self.layers[t][k];
                layer.quads = (verts.len() / 4) as u32;
                if layer.quads == 0 {
                    continue;
                }
                let mut data = Vec::with_capacity(verts.len() * 24);
                for (p, uv, c) in verts {
                    for f in p.iter().chain(uv.iter()) {
                        data.extend_from_slice(&f.to_le_bytes());
                    }
                    data.extend_from_slice(c);
                }
                if layer
                    .vb
                    .as_ref()
                    .is_none_or(|b| b.size() < data.len() as u64)
                {
                    layer.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("beacon_beam.vb"),
                        size: (data.len() as u64).next_power_of_two(),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    }));
                }
                if let Some(vb) = &layer.vb {
                    queue.write_buffer(vb, 0, &data);
                }
            }
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
    }

    /// `translucent=false` — сплошной слой (до прозрачного), `true` — свечение (после).
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, translucent: bool) {
        let k = usize::from(translucent);
        if self.layers.iter().all(|l| l[k].quads == 0) {
            return;
        }
        pass.set_pipeline(&self.pipelines[k]);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        for (t, pair) in self.layers.iter().enumerate() {
            let layer = &pair[k];
            let Some(vb) = &layer.vb else { continue };
            if layer.quads == 0 {
                continue;
            }
            pass.set_bind_group(1, &self.groups[t], &[]);
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw_indexed(0..layer.quads * 6, 0, 0..1);
        }
    }
}
