//! Дождь и снег. VANILLA: WeatherEffectRenderer#prepareInstances/render (26.3).
//! Колонки собирает клиент (`ColumnInstance`), тут — вершины (формат PARTICLE) и отрисовка
//! шейдером `particle.wgsl` (текстура rain.png / snow.png, свет из lightmap).

use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use glam::Mat4;

/// VANILLA: WeatherEffectRenderer.ColumnInstance.
#[derive(Clone, Copy, Debug)]
pub struct ColumnInstance {
    pub x: i32,
    pub z: i32,
    pub bottom_y: i32,
    pub top_y: i32,
    pub u_offset: f32,
    pub v_offset: f32,
    /// block, sky (0..15).
    pub light: (u8, u8),
}

/// Таблица `columnSizeX/Z` 32×32.
fn column_sizes() -> ([f32; 1024], [f32; 1024]) {
    let mut sx = [0.0; 1024];
    let mut sz = [0.0; 1024];
    for z in 0..32 {
        for x in 0..32 {
            let (dx, dz) = (x as f32 - 16.0, z as f32 - 16.0);
            let d = dx.hypot(dz);
            sx[z * 32 + x] = -dz / d;
            sz[z * 32 + x] = dx / d;
        }
    }
    (sx, sz)
}

const VERTEX_SIZE: usize = 32;

fn push_vertex(out: &mut Vec<u8>, p: [f32; 3], uv: [f32; 2], alpha: f32, light: (u8, u8)) {
    for v in p.into_iter().chain(uv) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    // ARGB.white(alpha): rgb = 255, a = floor(alpha*255+0.5).
    let a = (alpha.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    out.extend_from_slice(&[255, 255, 255, a]);
    // UV2 = (block*16, sky*16): lightmap читается как texelFetch(UV2/16).
    out.extend_from_slice(&(i32::from(light.0) * 16).to_le_bytes());
    out.extend_from_slice(&(i32::from(light.1) * 16).to_le_bytes());
}

/// VANILLA: prepareInstances — 4 вершины на колонку. Возвращает число вершин.
#[allow(clippy::too_many_arguments)]
fn build(
    out: &mut Vec<u8>,
    cols: &[ColumnInstance],
    cam: [f64; 3],
    max_alpha: f32,
    radius: i32,
    intensity: f32,
    sizes: &([f32; 1024], [f32; 1024]),
) {
    let radius_sq = (radius * radius) as f32;
    let (cbx, cbz) = (cam[0].floor() as i32, cam[2].floor() as i32);
    for c in cols {
        let rx = (f64::from(c.x) + 0.5 - cam[0]) as f32;
        let rz = (f64::from(c.z) + 0.5 - cam[2]) as f32;
        let dist_sq = rx * rx + rz * rz;
        let t = (dist_sq / radius_sq).min(1.0);
        let alpha = (max_alpha + (0.5 - max_alpha) * t) * intensity;
        let index = ((c.z - cbz + 16) * 32 + c.x - cbx + 16) as usize;
        let (hx, hz) = (sizes.0[index] / 2.0, sizes.1[index] / 2.0);
        let (x0, x1) = (rx - hx, rx + hx);
        let (z0, z1) = (rz - hz, rz + hz);
        let y1 = (f64::from(c.top_y) - cam[1]) as f32;
        let y0 = (f64::from(c.bottom_y) - cam[1]) as f32;
        let (u0, u1) = (c.u_offset, c.u_offset + 1.0);
        let v0 = c.bottom_y as f32 * 0.25 + c.v_offset;
        let v1 = c.top_y as f32 * 0.25 + c.v_offset;
        push_vertex(out, [x0, y1, z0], [u0, v0], alpha, c.light);
        push_vertex(out, [x1, y1, z1], [u1, v0], alpha, c.light);
        push_vertex(out, [x1, y0, z1], [u1, v1], alpha, c.light);
        push_vertex(out, [x0, y0, z0], [u0, v1], alpha, c.light);
    }
}

pub struct WeatherDraw {
    sizes: ([f32; 1024], [f32; 1024]),
    pipeline: wgpu::RenderPipeline,
    group0: wgpu::BindGroup,
    dyn_buf: wgpu::Buffer,
    group_rain: wgpu::BindGroup,
    group_snow: wgpu::BindGroup,
    vb: Option<wgpu::Buffer>,
    ib: wgpu::Buffer,
    ib_quads: u32,
    rain_cols: u32,
    snow_cols: u32,
}

pub(crate) fn make_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    rgba: &[u8],
    w: u32,
    h: u32,
) -> wgpu::TextureView {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: w.max(1),
            height: h.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    if !rgba.is_empty() {
        queue.write_texture(
            t.as_image_copy(),
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
    }
    t.create_view(&Default::default())
}

impl WeatherDraw {
    /// `rain`/`snow` — RGBA8 (ширина, высота, пиксели); `None` → белый пиксель.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        lightmap: &wgpu::TextureView,
        rain: Option<(&[u8], u32, u32)>,
        snow: Option<(&[u8], u32, u32)>,
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
        let tex_entry = |binding, filterable| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let l0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("weather.group0"),
            entries: &[uni(0), uni(1), uni(3)],
        });
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("weather.group1"),
            entries: &[
                tex_entry(0, true),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                tex_entry(2, true),
            ],
        });
        let dyn_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("weather.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("weather.group0"),
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
        // Сэмплер текстуры по умолчанию: повтор, без сглаживания.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let white = [255u8; 4];
        let mk_group = |label: &str, img: Option<(&[u8], u32, u32)>| {
            let (d, w, h) = img.unwrap_or((&white, 1, 1));
            let view = make_texture(device, queue, label, d, w, h);
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
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
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(lightmap),
                    },
                ],
            })
        };
        let group_rain = mk_group("weather.rain", rain);
        let group_snow = mk_group("weather.snow", snow);
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("weather.pl"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("weather"),
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
            label: Some("weather"),
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
                    }),
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
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let ib = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("weather.ib"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        WeatherDraw {
            sizes: column_sizes(),
            pipeline,
            group0,
            dyn_buf,
            group_rain,
            group_snow,
            vb: None,
            ib,
            ib_quads: 0,
            rain_cols: 0,
            snow_cols: 0,
        }
    }

    /// VANILLA: WeatherEffectRenderer#prepare. Пустые списки — ничего не рисуется.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        cam: [f64; 3],
        rain: &[ColumnInstance],
        snow: &[ColumnInstance],
        radius: i32,
        intensity: f32,
    ) {
        self.rain_cols = rain.len() as u32;
        self.snow_cols = snow.len() as u32;
        let n = rain.len() + snow.len();
        if n == 0 {
            return;
        }
        let mut data = Vec::with_capacity(n * 4 * VERTEX_SIZE);
        build(&mut data, rain, cam, 1.0, radius, intensity, &self.sizes);
        build(&mut data, snow, cam, 0.8, radius, intensity, &self.sizes);
        if self
            .vb
            .as_ref()
            .is_none_or(|b| b.size() < data.len() as u64)
        {
            self.vb = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("weather.vb"),
                size: (data.len() as u64).next_power_of_two(),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        if let Some(vb) = &self.vb {
            queue.write_buffer(vb, 0, &data);
        }
        if self.ib_quads < n as u32 {
            let quads = (n as u32).next_power_of_two();
            let mut idx = Vec::with_capacity(quads as usize * 24);
            for q in 0..quads {
                for k in [0u32, 1, 2, 2, 3, 0] {
                    idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
                }
            }
            self.ib = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("weather.ib"),
                size: idx.len() as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&self.ib, 0, &idx);
            self.ib_quads = quads;
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

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vb) = &self.vb else { return };
        if self.rain_cols + self.snow_cols == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, vb.slice(..));
        if self.rain_cols > 0 {
            pass.set_bind_group(1, &self.group_rain, &[]);
            pass.draw_indexed(0..self.rain_cols * 6, 0, 0..1);
        }
        if self.snow_cols > 0 {
            pass.set_bind_group(1, &self.group_snow, &[]);
            pass.draw_indexed(
                self.rain_cols * 6..(self.rain_cols + self.snow_cols) * 6,
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
    fn column_table_is_unit_length() {
        let (sx, sz) = column_sizes();
        let i = 5 * 32 + 20;
        assert!((sx[i].hypot(sz[i]) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn build_emits_four_vertices_per_column() {
        let c = ColumnInstance {
            x: 3,
            z: 4,
            bottom_y: 60,
            top_y: 70,
            u_offset: 0.0,
            v_offset: 0.5,
            light: (0, 15),
        };
        let mut out = Vec::new();
        build(
            &mut out,
            &[c],
            [3.5, 65.0, 4.5],
            1.0,
            8,
            1.0,
            &(column_sizes()),
        );
        assert_eq!(out.len(), 4 * VERTEX_SIZE);
    }
}
