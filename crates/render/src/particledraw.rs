//! Отрисовка квад-частиц. VANILLA: QuadParticleRenderState#renderRotatedQuad + core/particle.vsh/fsh.
//! Частицы приходят уже с позицией относительно камеры; квад строится по правой/верхней оси камеры
//! (LOOKAT_XYZ: поворот камеры, углы (1,−1),(1,1),(−1,1),(−1,−1) × quad_size).

use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use crate::weatherdraw::make_texture;
use glam::Mat4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleAtlas {
    Blocks,
    Particles,
}

/// Одна частица-квад (VANILLA: QuadParticleRenderState#add).
#[derive(Clone, Copy, Debug)]
pub struct ParticleQuad {
    pub atlas: ParticleAtlas,
    /// Центр относительно камеры.
    pub pos: [f32; 3],
    pub size: f32,
    pub u0: f32,
    pub u1: f32,
    pub v0: f32,
    pub v1: f32,
    pub color: [u8; 4],
    /// block, sky в единицах 0..240 (уровень*16; Flame: плавное addSmoothBlockEmission).
    pub light: (u16, u16),
}

const VERTEX_SIZE: usize = 32;

pub fn push_vertex(out: &mut Vec<u8>, p: [f32; 3], uv: [f32; 2], c: [u8; 4], light: (u8, u8)) {
    push_vertex_raw(
        out,
        p,
        uv,
        c,
        (u16::from(light.0) * 16, u16::from(light.1) * 16),
    );
}

/// То же, свет уже в 0..240.
pub fn push_vertex_raw(
    out: &mut Vec<u8>,
    p: [f32; 3],
    uv: [f32; 2],
    c: [u8; 4],
    light: (u16, u16),
) {
    // Один блок 32 байта: один reserve/копирование вместо 8 extend_from_slice по 4 байта.
    let mut b = [0u8; VERTEX_SIZE];
    b[0..4].copy_from_slice(&p[0].to_le_bytes());
    b[4..8].copy_from_slice(&p[1].to_le_bytes());
    b[8..12].copy_from_slice(&p[2].to_le_bytes());
    b[12..16].copy_from_slice(&uv[0].to_le_bytes());
    b[16..20].copy_from_slice(&uv[1].to_le_bytes());
    b[20..24].copy_from_slice(&c);
    b[24..28].copy_from_slice(&i32::from(light.0).to_le_bytes());
    b[28..32].copy_from_slice(&i32::from(light.1).to_le_bytes());
    out.extend_from_slice(&b);
}

fn build(out: &mut Vec<u8>, q: &ParticleQuad, right: [f32; 3], up: [f32; 3]) {
    let corner = |x: f32, y: f32| {
        [
            q.pos[0] + (right[0] * x + up[0] * y) * q.size,
            q.pos[1] + (right[1] * x + up[1] * y) * q.size,
            q.pos[2] + (right[2] * x + up[2] * y) * q.size,
        ]
    };
    push_vertex_raw(out, corner(1.0, -1.0), [q.u1, q.v1], q.color, q.light);
    push_vertex_raw(out, corner(1.0, 1.0), [q.u1, q.v0], q.color, q.light);
    push_vertex_raw(out, corner(-1.0, 1.0), [q.u0, q.v0], q.color, q.light);
    push_vertex_raw(out, corner(-1.0, -1.0), [q.u0, q.v1], q.color, q.light);
}

pub struct ParticleDraw {
    pipeline: wgpu::RenderPipeline,
    group0: wgpu::BindGroup,
    dyn_buf: wgpu::Buffer,
    group_blocks: wgpu::BindGroup,
    group_particles: wgpu::BindGroup,
    vb: Option<wgpu::Buffer>,
    ib: wgpu::Buffer,
    ib_quads: u32,
    blocks_quads: u32,
    particle_quads: u32,
    texture_mat: Mat4,
    /// color_modulator.a: у блеска — GlintAlpha (0.75, Options.glintStrength), иначе 1.
    alpha: f32,
    /// Блеск предметов: берутся только квады с битом FOIL_FLAG в свете.
    foil_only: bool,
}

/// Бит в `uv2.x` вершины: у предмета есть блеск (шейдер его маскирует `& 4095`).
pub const FOIL_FLAG: i32 = 8192;

fn only_foil(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for q in src.chunks_exact(VERTEX_SIZE * 4) {
        let l = i32::from_le_bytes([q[24], q[25], q[26], q[27]]);
        if l & FOIL_FLAG != 0 {
            out.extend_from_slice(q);
        }
    }
    out
}

impl ParticleDraw {
    /// `particles` — RGBA8 атласа частиц (ширина, высота, пиксели); `blocks` — вид атласа блоков.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        particles: Option<(&[u8], u32, u32)>,
    ) -> Self {
        Self::new_ex(
            device, queue, color, depth, projection, fog, lightmap, blocks, particles, false,
            false, false, None,
        )
    }

    /// Тени сущностей. VANILLA: RenderTypes#entityShadow — translucent, без записи глубины,
    /// polygon offset (−1, −10) (тень лежит на грани блока).
    #[allow(clippy::too_many_arguments)]
    pub fn new_shadow(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        texture: Option<(&[u8], u32, u32)>,
    ) -> Self {
        Self::new_ex(
            device, queue, color, depth, projection, fog, lightmap, blocks, texture, true, false,
            false, None,
        )
    }

    /// Модели сущностей с отсечением обратных граней. VANILLA: RenderTypes#entityCutoutCull (стрела).
    #[allow(clippy::too_many_arguments)]
    pub fn new_culled(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        texture: Option<(&[u8], u32, u32)>,
    ) -> Self {
        Self::new_ex(
            device, queue, color, depth, projection, fog, lightmap, blocks, texture, false, true,
            false, None,
        )
    }

    /// Блеск зачарований брони/сущностей. VANILLA: RenderTypes#armorEntityGlint — текстура блеска с матрицей
    /// текстуры (`set_texture_mat`), blend GLINT (SRC_COLOR, ONE), без записи глубины.
    #[allow(clippy::too_many_arguments)]
    pub fn new_glint(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        texture: Option<(&[u8], u32, u32)>,
    ) -> Self {
        Self::new_ex(
            device, queue, color, depth, projection, fog, lightmap, blocks, texture, false, false,
            true, None,
        )
    }

    /// Блеск на 3D-предметах (рука, выброшенные). VANILLA: RenderTypes#entityGlint — спрайт предмета задаёт
    /// форму (альфа), текстура блеска сдвигается матрицей текстуры (`set_texture_mat`); UV — атласа предметов.
    /// `blocks` — атлас блоков, `atlas` — атлас предметов (как у `hand_item_draw`).
    #[allow(clippy::too_many_arguments)]
    pub fn new_item_glint(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        atlas: Option<(&[u8], u32, u32)>,
        glint: Option<(&[u8], u32, u32)>,
    ) -> Self {
        let white = ([255u8; 4], 1u32, 1u32);
        let g = glint.unwrap_or((&white.0, 1, 1));
        Self::new_ex(
            device,
            queue,
            color,
            depth,
            projection,
            fog,
            lightmap,
            blocks,
            atlas,
            false,
            false,
            true,
            Some(g),
        )
    }

    pub fn set_texture_mat(&mut self, m: Mat4) {
        self.texture_mat = m;
    }

    #[allow(clippy::too_many_arguments)]
    fn new_ex(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        blocks: &wgpu::TextureView,
        particles: Option<(&[u8], u32, u32)>,
        shadow: bool,
        cull: bool,
        glint: bool,
        item_glint: Option<(&[u8], u32, u32)>,
    ) -> Self {
        let vf = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let uni = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: vf,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let tex_entry = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: vf,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let l0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("particle.group0"),
            entries: &[uni(0), uni(1), uni(3)],
        });
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("particle.group1"),
            entries: &[
                tex_entry(0),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                tex_entry(2),
                tex_entry(3),
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let dyn_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("particle.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("particle.group0"),
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
        let (filter, addr) = if glint && !item_glint.is_some() {
            (wgpu::FilterMode::Linear, wgpu::AddressMode::Repeat)
        } else {
            (wgpu::FilterMode::Nearest, wgpu::AddressMode::ClampToEdge)
        };
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: addr,
            address_mode_v: addr,
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let item_mode = item_glint.is_some();
        let (gd, gw, gh) = item_glint.unwrap_or((&[255u8; 4], 1, 1));
        let gview = make_texture(device, queue, "particle.glint", gd, gw, gh);
        let gsampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mk_group = |label: &str, view: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &l1,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(lightmap),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&gview),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::Sampler(&gsampler),
                    },
                ],
            })
        };
        let white = [255u8; 4];
        let (d, w, h) = particles.unwrap_or((&white, 1, 1));
        let pview = make_texture(device, queue, "particle.atlas", d, w, h);
        let group_blocks = mk_group("particle.blocks", blocks);
        let group_particles = mk_group("particle.particles", &pview);
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("particle.pl"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("particle"),
            source: wgpu::ShaderSource::Wgsl(Shader::Particle.source().into()),
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
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Sint32x2,
                offset: 24,
                shader_location: 3,
            },
        ];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("particle"),
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
                entry_point: Some(if item_mode {
                    "fs_glint_item"
                } else if glint {
                    "fs_glint"
                } else {
                    "fs_main"
                }),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color,
                    blend: Some(if glint {
                        wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::Src,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::Zero,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }
                    } else {
                        wgpu::BlendState::ALPHA_BLENDING
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: cull.then_some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth,
                depth_write_enabled: !shadow && !glint,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: if shadow {
                    wgpu::DepthBiasState {
                        constant: -10,
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
        });
        let ib = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("particle.ib"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        ParticleDraw {
            pipeline,
            group0,
            dyn_buf,
            group_blocks,
            group_particles,
            vb: None,
            ib,
            ib_quads: 0,
            blocks_quads: 0,
            particle_quads: 0,
            texture_mat: Mat4::IDENTITY,
            alpha: if glint { 0.75 } else { 1.0 },
            foil_only: item_mode,
        }
    }

    /// `right`/`up` — оси камеры в мире (вправо и вверх от взгляда).
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        right: [f32; 3],
        up: [f32; 3],
        quads: &[ParticleQuad],
    ) {
        self.blocks_quads = 0;
        self.particle_quads = 0;
        if quads.is_empty() {
            return;
        }
        let mut data = Vec::with_capacity(quads.len() * 4 * VERTEX_SIZE);
        for q in quads.iter().filter(|q| q.atlas == ParticleAtlas::Blocks) {
            build(&mut data, q, right, up);
            self.blocks_quads += 1;
        }
        for q in quads.iter().filter(|q| q.atlas == ParticleAtlas::Particles) {
            build(&mut data, q, right, up);
            self.particle_quads += 1;
        }
        let n = quads.len() as u32;
        if self
            .vb
            .as_ref()
            .is_none_or(|b| b.size() < data.len() as u64)
        {
            self.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("particle.vb"),
                size: (data.len() as u64).next_power_of_two(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        if let Some(vb) = &self.vb {
            queue.write_buffer(vb, 0, &data);
        }
        if self.ib_quads < n {
            let cap = n.next_power_of_two();
            let mut idx = Vec::with_capacity(cap as usize * 24);
            for q in 0..cap {
                for k in [0u32, 1, 2, 2, 3, 0] {
                    idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
                }
            }
            self.ib = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("particle.ib"),
                size: idx.len() as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&self.ib, 0, &idx);
            self.ib_quads = cap;
        }
        queue.write_buffer(
            &self.dyn_buf,
            0,
            &DynamicTransformsUniform {
                model_view_mat: model_view,
                color_modulator: [1.0, 1.0, 1.0, self.alpha],
                model_offset: [0.0; 3],
                texture_mat: self.texture_mat,
            }
            .to_bytes(),
        );
    }

    /// Готовые квады (формат `push_vertex`, по 4 вершины) — для моделей сущностей:
    /// рисуются с текстурой `particles` (в `new` туда передаётся скин). `model_view` — поворот камеры.
    pub fn prepare_raw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        data: &[u8],
    ) {
        self.prepare_split(device, queue, model_view, &[], data);
    }

    /// Как `prepare_raw`, но две группы квадов: сначала с атласом блоков, затем с `particles`-текстурой.
    pub fn prepare_split(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        blocks: &[u8],
        particles: &[u8],
    ) {
        self.blocks_quads = 0;
        self.particle_quads = 0;
        let (fb, fp);
        let (blocks, particles) = if self.foil_only {
            fb = only_foil(blocks);
            fp = only_foil(particles);
            (&fb[..], &fp[..])
        } else {
            (blocks, particles)
        };
        let nb = (blocks.len() / (VERTEX_SIZE * 4)) as u32;
        let np = (particles.len() / (VERTEX_SIZE * 4)) as u32;
        let n = nb + np;
        if n == 0 {
            return;
        }
        let mut data = Vec::with_capacity(blocks.len() + particles.len());
        data.extend_from_slice(blocks);
        data.extend_from_slice(particles);
        let data = &data[..];
        if self
            .vb
            .as_ref()
            .is_none_or(|b| b.size() < data.len() as u64)
        {
            self.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("particle.vb"),
                size: (data.len() as u64).next_power_of_two(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        if let Some(vb) = &self.vb {
            queue.write_buffer(vb, 0, data);
        }
        if self.ib_quads < n {
            let cap = n.next_power_of_two();
            let mut idx = Vec::with_capacity(cap as usize * 24);
            for q in 0..cap {
                for k in [0u32, 1, 2, 2, 3, 0] {
                    idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
                }
            }
            self.ib = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("particle.ib"),
                size: idx.len() as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&self.ib, 0, &idx);
            self.ib_quads = cap;
        }
        queue.write_buffer(
            &self.dyn_buf,
            0,
            &DynamicTransformsUniform {
                model_view_mat: model_view,
                color_modulator: [1.0, 1.0, 1.0, self.alpha],
                model_offset: [0.0; 3],
                texture_mat: self.texture_mat,
            }
            .to_bytes(),
        );
        self.blocks_quads = nb;
        self.particle_quads = np;
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vb) = &self.vb else { return };
        if self.blocks_quads + self.particle_quads == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, vb.slice(..));
        if self.blocks_quads > 0 {
            pass.set_bind_group(1, &self.group_blocks, &[]);
            pass.draw_indexed(0..self.blocks_quads * 6, 0, 0..1);
        }
        if self.particle_quads > 0 {
            pass.set_bind_group(1, &self.group_particles, &[]);
            pass.draw_indexed(
                self.blocks_quads * 6..(self.blocks_quads + self.particle_quads) * 6,
                0,
                0..1,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_has_four_vertices_facing_camera() {
        let q = ParticleQuad {
            atlas: ParticleAtlas::Blocks,
            pos: [0.0, 0.0, 5.0],
            size: 0.5,
            u0: 0.0,
            u1: 1.0,
            v0: 0.0,
            v1: 1.0,
            color: [255; 4],
            light: (0, 240),
        };
        let mut out = Vec::new();
        build(&mut out, &q, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(out.len(), 4 * VERTEX_SIZE);
        // первая вершина (1,−1): x = +0.5, y = −0.5, u1 v1
        let f = |i: usize| f32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().expect("4 байта"));
        assert_eq!((f(0), f(1), f(2)), (0.5, -0.5, 5.0));
        assert_eq!((f(3), f(4)), (1.0, 1.0));
    }
}
