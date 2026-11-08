//! GPU-часть неба Overworld: диск неба, веер заката, солнце, луна, звёзды, тёмный диск.
//!
//! VANILLA: SkyRenderer#renderSkyDisc/renderSunriseAndSunset/renderSunMoonAndStars/renderDarkDisc,
//! RenderPipelines SKY/SUNRISE_SUNSET/STARS/CELESTIAL (1.21.11). TRIANGLE_FAN превращён в
//! список треугольников (wgpu веера не поддерживает), culling выключен (камера всегда внутри).

use crate::shaders::Shader;
use crate::sky::{self, SkyState};
use crate::uniforms::DynamicTransformsUniform;
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

/// Шаг слотов DynamicTransforms (min_uniform_buffer_offset_alignment по умолчанию 256).
const SLOT: u64 = 256;
const SLOTS: u64 = 8;

/// Слоты: 0 диск неба, 1 заката, 2 солнце, 3 луна, 4 звёзды, 5 тёмный диск.
#[derive(Clone, Copy)]
enum Slot {
    Disc = 0,
    Sunrise,
    Sun,
    Moon,
    Stars,
    Dark,
}

/// Спрайты светил: RGBA8, все одного размера; `moons[i]` — `MoonPhase.index() == i`.
pub struct CelestialImages {
    pub width: u32,
    pub height: u32,
    pub sun: Vec<u8>,
    pub moons: [Vec<u8>; 8],
}

pub struct SkyDraw {
    dyn_buf: wgpu::Buffer,
    group0: wgpu::BindGroup,
    group1: wgpu::BindGroup,
    p_sky: wgpu::RenderPipeline,
    p_sunrise: wgpu::RenderPipeline,
    p_stars: wgpu::RenderPipeline,
    p_celestial: wgpu::RenderPipeline,
    top_vb: wgpu::Buffer,
    top_ib: wgpu::Buffer,
    top_n: u32,
    bottom_vb: wgpu::Buffer,
    star_vb: wgpu::Buffer,
    star_ib: wgpu::Buffer,
    star_n: u32,
    sunrise_vb: wgpu::Buffer,
    sunrise_ib: wgpu::Buffer,
    sunrise_n: u32,
    quad_vb: wgpu::Buffer,
    quad_ib: wgpu::Buffer,
    /// Видимое состояние последнего `prepare`.
    state: Option<(SkyState, bool)>,
}

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: только для POD-типов ([f32;N], u32, [u8;N]) без паддинга.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

fn fan_indices(n: u32) -> Vec<u32> {
    (1..n - 1).flat_map(|i| [0, i, i + 1]).collect()
}

fn quad_indices(quads: u32) -> Vec<u32> {
    (0..quads)
        .flat_map(|q| {
            let b = q * 4;
            [b, b + 1, b + 2, b, b + 2, b + 3]
        })
        .collect()
}

impl SkyDraw {
    /// `projection`/`fog` — буферы мира (общие с террейном).
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        images: &CelestialImages,
    ) -> Self {
        let vf = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let uni = |binding, dynamic| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: vf,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: dynamic,
                min_binding_size: None,
            },
            count: None,
        };
        let l0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sky.group0"),
            entries: &[uni(0, false), uni(1, false), uni(3, true)],
        });
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sky.group1"),
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
            label: Some("sky.dyn"),
            size: SLOT * SLOTS,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sky.group0"),
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
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &dyn_buf,
                        offset: 0,
                        size: wgpu::BufferSize::new(DynamicTransformsUniform::SIZE as u64),
                    }),
                },
            ],
        });

        // Атлас светил: вертикальная полоса 9 спрайтов (0 — солнце, 1..=8 — луна по индексу фазы + 1).
        let (w, h) = (images.width, images.height);
        let atlas = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas.celestials"),
            size: wgpu::Extent3d {
                width: w,
                height: h * 9,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let mut data = images.sun.clone();
        for m in &images.moons {
            data.extend_from_slice(m);
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &atlas,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h * 9),
            },
            wgpu::Extent3d {
                width: w,
                height: h * 9,
                depth_or_array_layers: 1,
            },
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let view = atlas.create_view(&Default::default());
        let group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sky.group1"),
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
        });

        // Конвейеры.
        let pl0 = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sky.pl0"),
            bind_group_layouts: &[&l0],
            immediate_size: 0,
        });
        let pl1 = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sky.pl1"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
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
        // BlendFunction.OVERLAY
        let overlay = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let pos_attr = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        }];
        let pos_layout = wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &pos_attr,
        };
        let pc_attr = [
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
        let pc_layout = wgpu::VertexBufferLayout {
            array_stride: 16,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &pc_attr,
        };
        let pt_attr = [
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
        ];
        let pt_layout = wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &pt_attr,
        };
        let make = |label: &str,
                    shader: Shader,
                    layout: &wgpu::PipelineLayout,
                    vb: wgpu::VertexBufferLayout<'_>,
                    blend: Option<wgpu::BlendState>| {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(shader.source().into()),
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[vb],
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
                primitive: wgpu::PrimitiveState::default(),
                // Небо рисуется первым, глубину не пишет (withDepthWrite(false)).
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: depth,
                    depth_write_enabled: false,
                    depth_compare: wgpu::CompareFunction::Always,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let p_sky = make("sky", Shader::Sky, &pl0, pos_layout.clone(), None);
        let p_sunrise = make(
            "sunrise",
            Shader::PositionColor,
            &pl0,
            pc_layout,
            Some(translucent),
        );
        let p_stars = make("stars", Shader::Stars, &pl0, pos_layout, Some(overlay));
        let p_celestial = make(
            "celestial",
            Shader::PositionTex,
            &pl1,
            pt_layout,
            Some(overlay),
        );

        // Геометрия.
        let vbuf = |label: &str, data: &[u8]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX,
            })
        };
        let ibuf = |label: &str, idx: &[u32]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytes(idx),
                usage: wgpu::BufferUsages::INDEX,
            })
        };
        let top = sky::build_sky_disc(16.0);
        let bottom = sky::build_sky_disc(-16.0);
        let top_idx = fan_indices(top.len() as u32);
        let stars = sky::build_stars();
        let star_idx = quad_indices((stars.len() / 4) as u32);
        let fan = sky::build_sunrise_fan();
        let mut fan_v: Vec<[f32; 4]> = Vec::new(); // 16 байт: xyz + RGBA8 как f32-слово
        for (p, c) in &fan {
            // ARGB → байты R,G,B,A
            let rgba = [(c >> 16) as u8, (c >> 8) as u8, *c as u8, (c >> 24) as u8];
            fan_v.push([p[0], p[1], p[2], f32::from_bits(u32::from_le_bytes(rgba))]);
        }
        let fan_idx = fan_indices(fan.len() as u32);
        // Квад светила: позиция + uv (uv выставляются на draw через смещение вершин — 9 спрайтов).
        // Солнце: 4 вершины; луна: 8 фаз × 4 вершины. Порядок/UV — как SkyRenderer#buildCelestialQuad.
        let step = 1.0 / 9.0;
        let mut q: Vec<[f32; 5]> = Vec::new();
        let (u0, u1) = (0.0_f32, 1.0_f32);
        let (v0, v1) = (0.0_f32, step);
        q.push([-1.0, 0.0, -1.0, u0, v0]);
        q.push([1.0, 0.0, -1.0, u1, v0]);
        q.push([1.0, 0.0, 1.0, u1, v1]);
        q.push([-1.0, 0.0, 1.0, u0, v1]);
        for phase in 0..8 {
            let v0 = step * (phase as f32 + 1.0);
            let v1 = v0 + step;
            q.push([-1.0, 0.0, -1.0, u1, v1]);
            q.push([1.0, 0.0, -1.0, u0, v1]);
            q.push([1.0, 0.0, 1.0, u0, v0]);
            q.push([-1.0, 0.0, 1.0, u1, v0]);
        }
        let quad_idx = quad_indices(1);

        Self {
            dyn_buf,
            group0,
            group1,
            p_sky,
            p_sunrise,
            p_stars,
            p_celestial,
            top_vb: vbuf("sky.top", bytes(&top)),
            top_ib: ibuf("sky.top.i", &top_idx),
            top_n: top_idx.len() as u32,
            bottom_vb: vbuf("sky.bottom", bytes(&bottom)),
            star_vb: vbuf("sky.stars", bytes(&stars)),
            star_ib: ibuf("sky.stars.i", &star_idx),
            star_n: star_idx.len() as u32,
            sunrise_vb: vbuf("sky.sunrise", bytes(&fan_v)),
            sunrise_ib: ibuf("sky.sunrise.i", &fan_idx),
            sunrise_n: fan_idx.len() as u32,
            quad_vb: vbuf("sky.quads", bytes(&q)),
            quad_ib: ibuf("sky.quad.i", &quad_idx),
            state: None,
        }
    }

    fn write(&self, queue: &wgpu::Queue, slot: Slot, mv: Mat4, color: [f32; 4]) {
        let u = DynamicTransformsUniform {
            model_view_mat: mv,
            color_modulator: color,
            model_offset: [0.0; 3],
            texture_mat: Mat4::IDENTITY,
        };
        queue.write_buffer(&self.dyn_buf, slot as u64 * SLOT, &u.to_bytes());
    }

    /// Обновляет матрицы слотов. `camera` — вращение камеры (без переноса),
    /// `dark_disc` — глаза ниже горизонта (`shouldRenderDarkDisc`).
    pub fn prepare(&mut self, queue: &wgpu::Queue, camera: Mat4, st: &SkyState, dark_disc: bool) {
        let argb_f = |c: u32| {
            [
                ((c >> 16) & 255) as f32 / 255.0,
                ((c >> 8) & 255) as f32 / 255.0,
                (c & 255) as f32 / 255.0,
                (c >> 24) as f32 / 255.0,
            ]
        };
        // renderSkyDisc: modelView, цвет неба.
        self.write(
            queue,
            Slot::Disc,
            camera,
            argb_f(st.sky_color | 0xFF00_0000),
        );
        // renderSunriseAndSunset: X(90°) · Z(180?+90°) · scale(1,1,alpha)
        let alpha = (st.sunrise_color >> 24) as f32 / 255.0;
        let flip = if st.sun_angle.sin() < 0.0 {
            180.0_f32
        } else {
            0.0
        };
        let sunrise = camera
            * Mat4::from_rotation_x(90.0_f32.to_radians())
            * Mat4::from_rotation_z((flip + 90.0).to_radians())
            * Mat4::from_scale(Vec3::new(1.0, 1.0, alpha));
        self.write(queue, Slot::Sunrise, sunrise, argb_f(st.sunrise_color));
        // renderSunMoonAndStars: Y(-90°) · X(angle)
        let base = camera * Mat4::from_rotation_y((-90.0_f32).to_radians());
        let white_a = [1.0, 1.0, 1.0, st.rain_brightness];
        let sun = base
            * Mat4::from_rotation_x(st.sun_angle)
            * Mat4::from_translation(Vec3::new(0.0, 100.0, 0.0))
            * Mat4::from_scale(Vec3::new(30.0, 1.0, 30.0));
        self.write(queue, Slot::Sun, sun, white_a);
        let moon = base
            * Mat4::from_rotation_x(st.moon_angle)
            * Mat4::from_translation(Vec3::new(0.0, 100.0, 0.0))
            * Mat4::from_scale(Vec3::new(20.0, 1.0, 20.0));
        self.write(queue, Slot::Moon, moon, white_a);
        let b = st.star_brightness;
        self.write(
            queue,
            Slot::Stars,
            base * Mat4::from_rotation_x(st.star_angle),
            [b, b, b, b],
        );
        // renderDarkDisc: translate(0,12,0), чёрный.
        self.write(
            queue,
            Slot::Dark,
            camera * Mat4::from_translation(Vec3::new(0.0, 12.0, 0.0)),
            [0.0, 0.0, 0.0, 1.0],
        );
        self.state = Some((*st, dark_disc));
    }

    /// Рисует небо в уже открытом проходе мира (до террейна). Нужен вызов `prepare`.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some((st, dark)) = &self.state else {
            return;
        };
        let off = |s: Slot| [s as u32 * SLOT as u32];
        // Диск неба.
        pass.set_pipeline(&self.p_sky);
        pass.set_bind_group(0, &self.group0, &off(Slot::Disc));
        pass.set_vertex_buffer(0, self.top_vb.slice(..));
        pass.set_index_buffer(self.top_ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.top_n, 0, 0..1);
        // Закат/восход.
        if st.sunrise_color >> 24 > 0 && (st.sunrise_color >> 24) as f32 / 255.0 > 0.001 {
            pass.set_pipeline(&self.p_sunrise);
            pass.set_bind_group(0, &self.group0, &off(Slot::Sunrise));
            pass.set_vertex_buffer(0, self.sunrise_vb.slice(..));
            pass.set_index_buffer(self.sunrise_ib.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.sunrise_n, 0, 0..1);
        }
        // Солнце и луна.
        pass.set_pipeline(&self.p_celestial);
        pass.set_bind_group(1, &self.group1, &[]);
        pass.set_vertex_buffer(0, self.quad_vb.slice(..));
        pass.set_index_buffer(self.quad_ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_bind_group(0, &self.group0, &off(Slot::Sun));
        pass.draw_indexed(0..6, 0, 0..1);
        pass.set_bind_group(0, &self.group0, &off(Slot::Moon));
        pass.draw_indexed(0..6, 4 + 4 * i32::from(st.moon_phase), 0..1);
        // Звёзды.
        if st.star_brightness > 0.0 {
            pass.set_pipeline(&self.p_stars);
            pass.set_bind_group(0, &self.group0, &off(Slot::Stars));
            pass.set_vertex_buffer(0, self.star_vb.slice(..));
            pass.set_index_buffer(self.star_ib.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.star_n, 0, 0..1);
        }
        // Тёмный диск под горизонтом.
        if *dark {
            pass.set_pipeline(&self.p_sky);
            pass.set_bind_group(0, &self.group0, &off(Slot::Dark));
            pass.set_vertex_buffer(0, self.bottom_vb.slice(..));
            pass.set_index_buffer(self.top_ib.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.top_n, 0, 0..1);
        }
    }
}
