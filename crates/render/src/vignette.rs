//! Виньетка. VANILLA: Hud#extractVignette/updateVignetteBrightness (26.3), RenderPipelines.VIGNETTE
//! (blend: цвет ZERO·src + dst·(1−src.rgb), альфа ZERO/ONE), текстура textures/misc/vignette.png (blur).
//! Яркость растёт в темноте: +(1−lightLevel−b)·0.01 за тик. Красный сигнал у границы мира не сделан
//! (VANILLA-UNSURE).
use wgpu::util::DeviceExt;

const SHADER: &str = r"
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var<uniform> color: vec4<f32>;
struct VsOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> }
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VsOut {
    var p = array<vec2<f32>, 3>(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    var o: VsOut;
    o.pos = vec4<f32>(p[i], 0.0, 1.0);
    o.uv = vec2<f32>((p[i].x + 1.0) * 0.5, (1.0 - p[i].y) * 0.5);
    return o;
}
@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    return textureSample(tex, samp, i.uv) * color;
}
";

pub struct VignetteDraw {
    pipe: wgpu::RenderPipeline,
    bind: wgpu::BindGroup,
    color: wgpu::Buffer,
}

impl VignetteDraw {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_fmt: wgpu::TextureFormat,
        depth_fmt: wgpu::TextureFormat,
        rgba: &[u8],
        w: u32,
        h: u32,
    ) -> Self {
        let tex = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("vignette"),
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
            ..Default::default()
        });
        let color = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vignette.color"),
            contents: &[0u8; 16],
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty,
            count: None,
        };
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("vignette"),
            entries: &[
                entry(
                    0,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
                entry(
                    1,
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                ),
                entry(
                    2,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
            ],
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("vignette"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&samp),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: color.as_entire_binding(),
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("vignette"),
            bind_group_layouts: &[&bgl],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vignette"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("vignette"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_fmt,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::Zero,
                            dst_factor: wgpu::BlendFactor::OneMinusSrc,
                            operation: wgpu::BlendOperation::Add,
                        },
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_fmt,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        VignetteDraw { pipe, bind, color }
    }

    /// `brightness` = Hud#vignetteBrightness (0..1).
    pub fn prepare(&self, queue: &wgpu::Queue, brightness: f32) {
        let b = brightness.clamp(0.0, 1.0);
        let c = [b, b, b, 1.0f32];
        // SAFETY: POD f32×4.
        let bytes = unsafe { std::slice::from_raw_parts(c.as_ptr().cast::<u8>(), 16) };
        queue.write_buffer(&self.color, 0, bytes);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipe);
        pass.set_bind_group(0, &self.bind, &[]);
        pass.draw(0..3, 0..1);
    }
}

/// VANILLA: SKY_LIGHT_LEVEL (15 × дорожка day.json, linear) → Level#skyDarken = (int)(15 − уровень).
/// Погодные модификаторы не учтены (VANILLA-UNSURE).
pub fn sky_darken(day_time: i64) -> i32 {
    let t = day_time.rem_euclid(24000) as f32;
    let keys = [
        (133.0, 1.0),
        (11867.0, 1.0),
        (13670.0, 0.266_666_68),
        (22330.0, 0.266_666_68),
        (24133.0, 1.0),
    ];
    let t = if t < 133.0 { t + 24000.0 } else { t };
    let mut v = 1.0;
    for w in keys.windows(2) {
        if t >= w[0].0 && t <= w[1].0 {
            v = w[0].1 + (w[1].1 - w[0].1) * (t - w[0].0) / (w[1].0 - w[0].0);
            break;
        }
    }
    (15.0 - 15.0 * v) as i32
}

/// VANILLA: Lightmap#getBrightness (ambientLight Overworld = 0) для `level` 0..15.
pub fn level_brightness(level: i32, ambient_light: f32) -> f32 {
    let v = level as f32 / 15.0;
    let curved = v / (4.0 - 3.0 * v);
    curved + ambient_light * (1.0 - curved)
}

/// VANILLA: Hud#updateVignetteBrightness (один тик).
pub fn tick_brightness(cur: f32, level: i32, ambient_light: f32) -> f32 {
    let b = (1.0 - level_brightness(level, ambient_light)).clamp(0.0, 1.0);
    cur + (b - cur) * 0.01
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn darkness_raises_brightness() {
        let mut b = 0.0;
        for _ in 0..300 {
            b = tick_brightness(b, 0, 0.0);
        }
        assert!(b > 0.9);
        let mut b = 1.0;
        for _ in 0..300 {
            b = tick_brightness(b, 15, 0.0);
        }
        assert!(b < 0.1);
    }

    #[test]
    fn sky_darken_day_night() {
        assert_eq!(sky_darken(6000), 0);
        assert_eq!(sky_darken(18000), 11);
    }
}
