//! Портал Энда и шлюз. VANILLA: TheEndPortalRenderer / TheEndGatewayRenderer, RenderTypes.endPortal/endGateway
//! (pipeline END_PORTAL: POSITION, QUADS, без смешения, глубина LEQUAL с записью; слои 15 / 16).
use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use glam::Mat4;

/// Вершина: позиция относительно камеры + число слоёв (15 портал, 16 шлюз).
pub type PortalVertex = [f32; 4];

pub struct EndPortalDraw {
    pipeline: wgpu::RenderPipeline,
    group0: wgpu::BindGroup,
    group1: wgpu::BindGroup,
    dyn_buf: wgpu::Buffer,
    time_buf: wgpu::Buffer,
    vb: Option<wgpu::Buffer>,
    ib: wgpu::Buffer,
    ib_quads: u32,
    quads: u32,
}

/// RGBA8-картинка.
pub struct Rgba<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
}

fn make_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &Rgba<'_>,
    label: &str,
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: img.width,
        height: img.height,
        depth_or_array_layers: 1,
    };
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
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

impl EndPortalDraw {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        sky: &Rgba<'_>,
        portal: &Rgba<'_>,
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
            label: Some("end_portal.group0"),
            entries: &[uni(0), uni(1), uni(3), uni(4)],
        });
        let tex_entry = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("end_portal.group1"),
            entries: &[
                tex_entry(0),
                tex_entry(1),
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let dyn_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("end_portal.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let time_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("end_portal.time"),
            size: 16,
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
            label: Some("end_portal.group0"),
            layout: &l0,
            entries: &[
                entry(0, projection),
                entry(1, fog),
                entry(3, &dyn_buf),
                entry(4, &time_buf),
            ],
        });
        let sky_view = make_texture(device, queue, sky, "end_portal.sky");
        let portal_view = make_texture(device, queue, portal, "end_portal.layers");
        // Текстуры без блюра, с повторением (RenderType: blur=false, wrap по умолчанию GL_REPEAT).
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("end_portal.sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("end_portal.group1"),
            layout: &l1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sky_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&portal_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("end_portal.pl"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("end_portal"),
            source: wgpu::ShaderSource::Wgsl(Shader::EndPortal.source().into()),
        });
        let attrs = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: 12,
                shader_location: 1,
            },
        ];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("end_portal"),
            layout: Some(&pl),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 16,
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
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
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
            label: Some("end_portal.ib"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        EndPortalDraw {
            pipeline,
            group0,
            group1,
            dyn_buf,
            time_buf,
            vb: None,
            ib,
            ib_quads: 0,
            quads: 0,
        }
    }

    /// `verts` — по 4 вершины на квад. `game_time` — тики мира, `partial` — доля тика.
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        verts: &[PortalVertex],
        game_time: i64,
        partial: f32,
    ) {
        self.quads = (verts.len() / 4) as u32;
        if self.quads == 0 {
            return;
        }
        let quads = self.quads;
        let data: Vec<u8> = verts
            .iter()
            .flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes()))
            .collect();
        if self
            .vb
            .as_ref()
            .is_none_or(|b| b.size() < data.len() as u64)
        {
            self.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("end_portal.vb"),
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
                label: Some("end_portal.ib"),
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
        // VANILLA: GlobalSettingsUniform — (gameTime % 24000 + partial) / 24000.
        let t = ((game_time.rem_euclid(24000)) as f32 + partial) / 24000.0;
        let mut tb = [0u8; 16];
        tb[..4].copy_from_slice(&t.to_le_bytes());
        queue.write_buffer(&self.time_buf, 0, &tb);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vb) = &self.vb else { return };
        if self.quads == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_bind_group(1, &self.group1, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, vb.slice(..));
        pass.draw_indexed(0..self.quads * 6, 0, 0..1);
    }
}
