//! GUI-рисовалка: текстурные квады (спрайты, 9-slice) в GUI-пикселях и куб панорамы главного меню.
//!
//! VANILLA: GuiGraphics#blit / blitSprite (nine_slice, stretch), CubeMap#render (FOV 85°, near 0.05,
//! far 10, модель: rotX(π)·rotX(rx)·rotY(ry)).
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

const MAX_QUADS: usize = 8192;

#[repr(C)]
#[derive(Clone, Copy)]
struct V {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

struct Tex {
    sprite: wgpu::BindGroup,
    pano: wgpu::BindGroup,
    _tex: wgpu::Texture,
}

pub struct GuiPainter {
    sprite_pipe: wgpu::RenderPipeline,
    glint_pipe: wgpu::RenderPipeline,
    invert_pipe: wgpu::RenderPipeline,
    /// (имя текстуры, квад) — рисуются после всего с инверсным смешиванием (прицел).
    inverted: Vec<(String, u32)>,
    glint_bgl: wgpu::BindGroupLayout,
    glint_buf: wgpu::Buffer,
    glint_group: Option<wgpu::BindGroup>,
    _glint_tex: Option<wgpu::Texture>,
    /// m (mat2 столбцами), offset.xy, alpha
    glint_params: [f32; 8],
    pano_pipe: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    nearest: wgpu::Sampler,
    linear: wgpu::Sampler,
    screen: wgpu::Buffer,
    mvp: wgpu::Buffer,
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    cube: wgpu::Buffer,
    texs: HashMap<String, Tex>,
    verts: Vec<V>,
    /// (имя текстуры, первый квад, число квадов)
    batches: Vec<(String, u32, u32, bool)>,
    scale: f32,
}

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: POD (#[repr(C)], f32/u32), чтение как байты.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

impl GuiPainter {
    pub fn new(device: &wgpu::Device, color: wgpu::TextureFormat) -> Self {
        let entry = |binding, visibility, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty,
            count: None,
        };
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gui"),
            entries: &[
                entry(
                    0,
                    wgpu::ShaderStages::VERTEX,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(
                    1,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
                entry(
                    2,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                ),
            ],
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gui"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/gui.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gui"),
            bind_group_layouts: &[&bgl],
            immediate_size: 0,
        });
        let mk = |vs: &str, label: &str| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(vs),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<V>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
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
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        // VANILLA: прицел — RenderPipelines.CROSSHAIR: ONE_MINUS_DST_COLOR / ONE_MINUS_SRC_COLOR, альфа ONE/ZERO.
        let invert_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gui invert"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_sprite"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<V>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::OneMinusDst,
                            dst_factor: wgpu::BlendFactor::OneMinusSrc,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let glint_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gui glint"),
            entries: &[
                entry(
                    0,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(
                    1,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
                entry(
                    2,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                ),
            ],
        });
        let glint_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gui glint"),
            bind_group_layouts: &[&bgl, &glint_bgl],
            immediate_size: 0,
        });
        let add = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let glint_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gui glint"),
            layout: Some(&glint_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_sprite"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<V>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_glint"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color,
                    blend: Some(wgpu::BlendState {
                        color: add,
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::Zero,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let samp = |f| {
            device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: f,
                min_filter: f,
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                ..Default::default()
            })
        };
        let ubuf = |label| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: 64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        // Куб: 6 граней × 4 вершины в порядке CubeMap#initializeVertices; x,y в pos, z — в color.x.
        const FACES: [[[f32; 3]; 4]; 6] = [
            [[-1., -1., 1.], [-1., 1., 1.], [1., 1., 1.], [1., -1., 1.]],
            [[1., -1., 1.], [1., 1., 1.], [1., 1., -1.], [1., -1., -1.]],
            [
                [1., -1., -1.],
                [1., 1., -1.],
                [-1., 1., -1.],
                [-1., -1., -1.],
            ],
            [
                [-1., -1., -1.],
                [-1., 1., -1.],
                [-1., 1., 1.],
                [-1., -1., 1.],
            ],
            [
                [-1., -1., -1.],
                [-1., -1., 1.],
                [1., -1., 1.],
                [1., -1., -1.],
            ],
            [[-1., 1., 1.], [-1., 1., -1.], [1., 1., -1.], [1., 1., 1.]],
        ];
        const UV: [[f32; 2]; 4] = [[0., 0.], [0., 1.], [1., 1.], [1., 0.]];
        let cube_v: Vec<V> = FACES
            .iter()
            .flat_map(|f| {
                f.iter().zip(UV).map(|(p, uv)| V {
                    pos: [p[0], p[1]],
                    uv,
                    color: [p[2], 0.0, 0.0, 0.0],
                })
            })
            .collect();
        let idx: Vec<u32> = (0..(MAX_QUADS as u32))
            .flat_map(|q| [q * 4, q * 4 + 1, q * 4 + 2, q * 4, q * 4 + 2, q * 4 + 3])
            .collect();
        Self {
            sprite_pipe: mk("vs_sprite", "gui sprite"),
            glint_pipe,
            invert_pipe,
            inverted: Vec::new(),
            glint_bgl,
            glint_buf: ubuf("gui glint"),
            glint_group: None,
            _glint_tex: None,
            glint_params: [0.0; 8],
            pano_pipe: mk("vs_pano", "gui pano"),
            bgl,
            nearest: samp(wgpu::FilterMode::Nearest),
            linear: samp(wgpu::FilterMode::Linear),
            screen: ubuf("gui screen"),
            mvp: ubuf("gui mvp"),
            vb: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gui vb"),
                size: (MAX_QUADS * 4 * std::mem::size_of::<V>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            ib: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gui ib"),
                contents: bytes(&idx),
                usage: wgpu::BufferUsages::INDEX,
            }),
            cube: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gui cube"),
                contents: bytes(&cube_v),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            texs: HashMap::new(),
            verts: Vec::new(),
            batches: Vec::new(),
            scale: 1.0,
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.texs.contains_key(name)
    }

    /// Загрузить текстуру RGBA8 под именем `name` (`linear` — билинейная фильтрация, для панорамы).
    #[allow(clippy::too_many_arguments)]
    pub fn add_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        rgba: &[u8],
        w: u32,
        h: u32,
        linear: bool,
    ) {
        let tex = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some(name),
                size: wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            rgba,
        );
        let view = tex.create_view(&Default::default());
        let samp = if linear { &self.linear } else { &self.nearest };
        let mk = |buf: &wgpu::Buffer| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(name),
                layout: &self.bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(samp),
                    },
                ],
            })
        };
        let t = Tex {
            sprite: mk(&self.screen),
            pano: mk(&self.mvp),
            _tex: tex,
        };
        self.texs.insert(name.to_owned(), t);
    }

    /// Начало кадра; `scale` — GUI scale (как у `TextRenderer`).
    pub fn begin(&mut self, scale: u32) {
        self.verts.clear();
        self.batches.clear();
        self.inverted.clear();
        self.scale = scale as f32;
    }

    /// Прямоугольник текстуры `name` с UV (0..1), `argb` — множитель цвета.
    #[allow(clippy::too_many_arguments)]
    pub fn blit(&mut self, name: &str, x: f32, y: f32, w: f32, h: f32, uv: [f32; 4], argb: u32) {
        if self.verts.len() / 4 >= MAX_QUADS || !self.texs.contains_key(name) {
            return;
        }
        let c = |sh: u32| ((argb >> sh) & 255) as f32 / 255.0;
        let color = [c(16), c(8), c(0), c(24)];
        let k = self.scale;
        let [u0, v0, u1, v1] = uv;
        for (p, t) in [
            ([x, y], [u0, v0]),
            ([x + w, y], [u1, v0]),
            ([x + w, y + h], [u1, v1]),
            ([x, y + h], [u0, v1]),
        ] {
            self.verts.push(V {
                pos: [p[0] * k, p[1] * k],
                uv: t,
                color,
            });
        }
        let q = (self.verts.len() / 4 - 1) as u32;
        match self.batches.last_mut() {
            Some(b) if b.0 == name && !b.3 => b.2 += 1,
            _ => self.batches.push((name.to_owned(), q, 1, false)),
        }
    }

    /// Спрайт с инверсным смешиванием, рисуется поверх всего (VANILLA: прицел).
    pub fn blit_inverted(&mut self, name: &str, x: f32, y: f32, w: f32, h: f32, uv: [f32; 4]) {
        let before = self.verts.len();
        self.blit(name, x, y, w, h, uv, 0xFFFF_FFFF);
        if self.verts.len() == before {
            return;
        }
        let q = (self.verts.len() / 4 - 1) as u32;
        // квад не должен попасть в обычную пачку
        if let Some(b) = self.batches.last_mut() {
            if b.1 + b.2 > q {
                b.2 -= 1;
                if b.2 == 0 {
                    self.batches.pop();
                }
            }
        }
        self.inverted.push((name.to_owned(), q));
    }

    /// Блеск зачарования поверх иконки `name` (аддитивно, по альфе иконки).
    /// Вызывать сразу после `blit` той же иконки (порядок отрисовки сохраняется).
    pub fn glint(&mut self, name: &str, x: f32, y: f32, w: f32, h: f32) {
        if self.glint_group.is_none() || !self.texs.contains_key(name) {
            return;
        }
        self.blit(name, x, y, w, h, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
        if let Some(b) = self.batches.last_mut() {
            if b.3 {
                return;
            }
            // blit мог дописать квад в предыдущую пачку: отделяем его в glint-пачку
            if b.2 > 1 {
                b.2 -= 1;
                let q = b.1 + b.2;
                self.batches.push((name.to_owned(), q, 1, true));
            } else {
                b.3 = true;
            }
        }
    }

    /// Загрузить текстуру блеска (RGBA8, повтор, билинейно).
    pub fn set_glint_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        w: u32,
        h: u32,
    ) {
        let tex = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("glint"),
                size: wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            rgba,
        );
        let view = tex.create_view(&Default::default());
        let samp = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            ..Default::default()
        });
        self.glint_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glint"),
            layout: &self.glint_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.glint_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&samp),
                },
            ],
        }));
        self._glint_tex = Some(tex);
    }

    /// Параметры блеска кадра: матрица текстуры (2×2 столбцами), смещение, сила.
    pub fn set_glint_params(&mut self, m: [f32; 4], off: [f32; 2], alpha: f32) {
        self.glint_params = [m[0], m[1], m[2], m[3], off[0], off[1], alpha, 0.0];
    }

    /// Плитка `name` (тайлинг не нужен для 16×16 фона — UV за пределы 0..1 с Repeat не используем:
    /// фон меню рисуется ячейками 16×16 GUI-пикселей).
    #[allow(clippy::too_many_arguments)]
    pub fn tile(&mut self, name: &str, x: f32, y: f32, w: f32, h: f32, cell: f32, argb: u32) {
        let mut cy = 0.0;
        while cy < h {
            let mut cx = 0.0;
            let ch = cell.min(h - cy);
            while cx < w {
                let cw = cell.min(w - cx);
                self.blit(
                    name,
                    x + cx,
                    y + cy,
                    cw,
                    ch,
                    [0.0, 0.0, cw / cell, ch / cell],
                    argb,
                );
                cx += cell;
            }
            cy += cell;
        }
    }

    /// VANILLA: nine_slice (stretch). `size` — логический размер спрайта (200×20), `b` — границы
    /// [left, top, right, bottom].
    #[allow(clippy::too_many_arguments)]
    pub fn nine_slice(
        &mut self,
        name: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        size: [f32; 2],
        b: [f32; 4],
        argb: u32,
    ) {
        let xs = [0.0, b[0], w - b[2], w];
        let ys = [0.0, b[1], h - b[3], h];
        let us = [0.0, b[0] / size[0], 1.0 - b[2] / size[0], 1.0];
        let vs = [0.0, b[1] / size[1], 1.0 - b[3] / size[1], 1.0];
        for j in 0..3 {
            for i in 0..3 {
                let (cw, ch) = (xs[i + 1] - xs[i], ys[j + 1] - ys[j]);
                if cw > 0.0 && ch > 0.0 {
                    self.blit(
                        name,
                        x + xs[i],
                        y + ys[j],
                        cw,
                        ch,
                        [us[i], vs[j], us[i + 1], vs[j + 1]],
                        argb,
                    );
                }
            }
        }
    }

    /// Рисует панораму (если есть `panorama_0..5`) и накопленные спрайты поверх `target`.
    /// `pano` = (rotX°, rotY°). Панорама — первым (LoadOp::Clear чёрным), спрайты — поверх.
    pub fn flush(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        w: u32,
        h: u32,
        pano: Option<(f32, f32)>,
    ) {
        let mut su = [0.0f32; 16];
        su[0] = w as f32;
        su[1] = h as f32;
        queue.write_buffer(&self.screen, 0, bytes(&su));
        if let Some((rx, ry)) = pano {
            // VANILLA: CubeMap#render
            let proj = Mat4::perspective_rh(85f32.to_radians(), w as f32 / h as f32, 0.05, 10.0);
            let model = Mat4::from_axis_angle(Vec3::X, std::f32::consts::PI)
                * Mat4::from_axis_angle(Vec3::X, rx.to_radians())
                * Mat4::from_axis_angle(Vec3::Y, ry.to_radians());
            queue.write_buffer(&self.mvp, 0, bytes(&(proj * model).to_cols_array()));
        }
        queue.write_buffer(&self.glint_buf, 0, bytes(&self.glint_params));
        queue.write_buffer(&self.vb, 0, bytes(&self.verts));
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("gui") });
        {
            let load = if pano.is_some() {
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            } else {
                wgpu::LoadOp::Load
            };
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
            if pano.is_some() {
                pass.set_pipeline(&self.pano_pipe);
                pass.set_vertex_buffer(0, self.cube.slice(..));
                for f in 0..6u32 {
                    if let Some(t) = self.texs.get(&format!("panorama_{f}")) {
                        pass.set_bind_group(0, &t.pano, &[]);
                        pass.draw_indexed(f * 6..f * 6 + 6, 0, 0..1);
                    }
                }
            }
            pass.set_pipeline(&self.sprite_pipe);
            pass.set_vertex_buffer(0, self.vb.slice(..));
            let mut glinting = false;
            for (name, q, n, g) in &self.batches {
                if let Some(t) = self.texs.get(name) {
                    if *g != glinting {
                        glinting = *g;
                        if *g {
                            pass.set_pipeline(&self.glint_pipe);
                            pass.set_bind_group(1, self.glint_group.as_ref(), &[]);
                        } else {
                            pass.set_pipeline(&self.sprite_pipe);
                        }
                    }
                    pass.set_bind_group(0, &t.sprite, &[]);
                    pass.draw_indexed(q * 6..(q + n) * 6, 0, 0..1);
                }
            }
            if !self.inverted.is_empty() {
                pass.set_pipeline(&self.invert_pipe);
                for (name, q) in &self.inverted {
                    if let Some(t) = self.texs.get(name) {
                        pass.set_bind_group(0, &t.sprite, &[]);
                        pass.draw_indexed(q * 6..(q + 1) * 6, 0, 0..1);
                    }
                }
            }
        }
        queue.submit([enc.finish()]);
    }
}
