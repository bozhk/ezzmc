//! Стена границы мира (силовое поле).
//! VANILLA: WorldBorderRenderer#extract/prepare/rebuildWorldBorderBuffer/render,
//! WorldBorderRenderState#closestBorder, RenderPipelines.WORLD_BORDER (26.3):
//! POSITION_TEX, QUADS, blend OVERLAY (SRC_ALPHA, ONE / ONE, ZERO), без отсечения граней,
//! глубина с записью, depth bias 3/3, текстура `misc/forcefield.png` (MipmappedTexture, 4 уровня).
use crate::shaders::Shader;
use crate::uniforms::DynamicTransformsUniform;
use ezzmc_world::border::WorldBorder;
use glam::Mat4;

const VERTEX_SIZE: usize = 20;
const MIP_LEVELS: u32 = 4;

/// Вершина стены: позиция, UV.
pub type BorderVertex = ([f32; 3], [f32; 2]);

/// VANILLA: WorldBorderRenderState.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BorderState {
    pub min_x: f64,
    pub max_x: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub tint: u32,
    pub alpha: f64,
}

/// VANILLA: WorldBorderRenderer#extract — видимость и прозрачность по расстоянию камеры до стены.
pub fn extract(b: &WorldBorder, partial: f32, cam: [f64; 3], render_distance: f64) -> BorderState {
    let mut s = BorderState {
        min_x: b.min_x(partial),
        max_x: b.max_x(partial),
        min_z: b.min_z(partial),
        max_z: b.max_z(partial),
        ..Default::default()
    };
    let (x, z, d) = (cam[0], cam[2], render_distance);
    let near_wall = x >= s.max_x - d || x <= s.min_x + d || z >= s.max_z - d || z <= s.min_z + d;
    let not_far_out = x >= s.min_x - d && x <= s.max_x + d && z >= s.min_z - d && z <= s.max_z + d;
    if near_wall && not_far_out {
        let a = 1.0 - b.distance_to(x, z) / d;
        s.alpha = a.powi(4).clamp(0.0, 1.0);
        s.tint = b.status().color();
    }
    s
}

/// Стороны, ближе `render_distance`, в порядке удаления: индексы Direction#get2DDataValue
/// (0 юг, 1 запад, 2 север, 3 восток). VANILLA: WorldBorderRenderState#closestBorder + draw.
pub fn sides(s: &BorderState, x: f64, z: f64, render_distance: f64) -> Vec<u32> {
    let mut v = [
        (2u32, z - s.min_z),
        (0, s.max_z - z),
        (1, x - s.min_x),
        (3, s.max_x - x),
    ];
    v.sort_by(|a, b| a.1.total_cmp(&b.1));
    v.iter()
        .filter(|(_, d)| *d < render_distance)
        .map(|(i, _)| *i)
        .collect()
}

/// VANILLA: WorldBorderRenderer#rebuildWorldBorderBuffer — 4 квада (юг, запад, север, восток)
/// относительно (minX, minZ) видимого участка; возвращает вершины (pos, uv) и (minX, minZ).
pub fn build(
    s: &BorderState,
    render_distance: f64,
    cam_x: f64,
    cam_z: f64,
    half_h: f32,
    v1: f32,
    v0: f32,
) -> (Vec<BorderVertex>, f64, f64) {
    let bmin_x = s.min_x + 0.01;
    let bmax_x = s.max_x - 0.01;
    let bmin_z = s.min_z + 0.01;
    let bmax_z = s.max_z - 0.01;
    let min_z = ((cam_z - render_distance).floor()).max(bmin_z);
    let max_z = ((cam_z + render_distance).ceil()).min(bmax_z);
    let u0z = ((min_z.floor() as i64) & 1) as f32 * 0.5;
    let u1z = (max_z - min_z) as f32 / 2.0;
    let min_x = ((cam_x - render_distance).floor()).max(bmin_x);
    let max_x = ((cam_x + render_distance).ceil()).min(bmax_x);
    let u0x = ((min_x.floor() as i64) & 1) as f32 * 0.5;
    let u1x = (max_x - min_x) as f32 / 2.0;
    let (dx, dz) = ((max_x - min_x) as f32, (max_z - min_z) as f32);
    let (sz, ex) = ((bmax_z - min_z) as f32, (bmax_x - min_x) as f32);
    let h = half_h;
    let v = vec![
        ([0.0, -h, sz], [u0x, v1]),
        ([dx, -h, sz], [u1x + u0x, v1]),
        ([dx, h, sz], [u1x + u0x, v0]),
        ([0.0, h, sz], [u0x, v0]),
        ([0.0, -h, 0.0], [u0z, v1]),
        ([0.0, -h, dz], [u1z + u0z, v1]),
        ([0.0, h, dz], [u1z + u0z, v0]),
        ([0.0, h, 0.0], [u0z, v0]),
        ([dx, -h, 0.0], [u0x, v1]),
        ([0.0, -h, 0.0], [u1x + u0x, v1]),
        ([0.0, h, 0.0], [u1x + u0x, v0]),
        ([dx, h, 0.0], [u0x, v0]),
        ([ex, -h, dz], [u0z, v1]),
        ([ex, -h, 0.0], [u1z + u0z, v1]),
        ([ex, h, 0.0], [u1z + u0z, v0]),
        ([ex, h, dz], [u0z, v0]),
    ];
    (v, min_x, min_z)
}

/// Цепочка мипов усреднением 2×2 (VANILLA: MipmappedTexture → MipmapGenerator, без альфа-хитростей).
fn mips(rgba: &[u8], w: u32, h: u32) -> Vec<(Vec<u8>, u32, u32)> {
    let mut out = vec![(rgba.to_vec(), w, h)];
    for _ in 1..MIP_LEVELS {
        let (src, sw, sh) = out.last().cloned().unwrap_or_default();
        if sw < 2 || sh < 2 {
            break;
        }
        let (nw, nh) = (sw / 2, sh / 2);
        let mut d = vec![0u8; (nw * nh * 4) as usize];
        for y in 0..nh {
            for x in 0..nw {
                for c in 0..4 {
                    let px = |xx: u32, yy: u32| u32::from(src[((yy * sw + xx) * 4 + c) as usize]);
                    let s = px(2 * x, 2 * y)
                        + px(2 * x + 1, 2 * y)
                        + px(2 * x, 2 * y + 1)
                        + px(2 * x + 1, 2 * y + 1);
                    d[((y * nw + x) * 4 + c) as usize] = (s / 4) as u8;
                }
            }
        }
        out.push((d, nw, nh));
    }
    out
}

pub struct BorderDraw {
    pipeline: wgpu::RenderPipeline,
    group0: wgpu::BindGroup,
    group1: wgpu::BindGroup,
    dyn_buf: wgpu::Buffer,
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    sides: Vec<u32>,
    last: Option<(BorderState, f64, f64, f64)>,
    origin: (f64, f64),
}

impl BorderDraw {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        forcefield: Option<(&[u8], u32, u32)>,
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
            label: Some("border.group0"),
            entries: &[uni(0), uni(3)],
        });
        let l1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("border.group1"),
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
            label: Some("border.dyn"),
            size: DynamicTransformsUniform::SIZE as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("border.group0"),
            layout: &l0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: dyn_buf.as_entire_binding(),
                },
            ],
        });
        let white = [255u8; 4];
        let (px, w, h) = forcefield.unwrap_or((&white, 1, 1));
        let levels = mips(px, w, h);
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("border.forcefield"),
            size: wgpu::Extent3d {
                width: w.max(1),
                height: h.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: levels.len() as u32,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for (i, (d, lw, lh)) in levels.iter().enumerate() {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex,
                    mip_level: i as u32,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                d,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(lw * 4),
                    rows_per_image: Some(*lh),
                },
                wgpu::Extent3d {
                    width: *lw,
                    height: *lh,
                    depth_or_array_layers: 1,
                },
            );
        }
        let view = tex.create_view(&Default::default());
        // VANILLA-UNSURE: сэмплер MipmappedTexture — повтор, nearest с выбором мипа.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("border.sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("border.group1"),
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
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("border.pl"),
            bind_group_layouts: &[&l0, &l1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world_border"),
            source: wgpu::ShaderSource::Wgsl(Shader::WorldBorder.source().into()),
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
        ];
        // VANILLA: BlendFunction.OVERLAY = (SRC_ALPHA, ONE, ONE, ZERO).
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world_border"),
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
                            dst_factor: wgpu::BlendFactor::One,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                // VANILLA-UNSURE: DepthStencilState(…, 3.0F, 3.0F) — polygon offset от камеры.
                bias: wgpu::DepthBiasState {
                    constant: 3,
                    slope_scale: 3.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let vb = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("border.vb"),
            size: (16 * VERTEX_SIZE) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut idx = Vec::with_capacity(24 * 4);
        for q in 0..4u32 {
            for k in [0u32, 1, 2, 2, 3, 0] {
                idx.extend_from_slice(&(q * 4 + k).to_le_bytes());
            }
        }
        let ib = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("border.ib"),
            size: idx.len() as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&ib, 0, &idx);
        BorderDraw {
            pipeline,
            group0,
            group1,
            dyn_buf,
            vb,
            ib,
            sides: Vec::new(),
            last: None,
            origin: (0.0, 0.0),
        }
    }

    /// VANILLA: WorldBorderRenderer#prepare + prepareDynamicTransforms. `millis` — Util.getMillis.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        queue: &wgpu::Queue,
        s: &BorderState,
        cam: [f64; 3],
        render_distance: f64,
        depth_far: f32,
        model_view: Mat4,
        millis: u64,
    ) {
        self.sides.clear();
        if s.alpha <= 0.0 {
            return;
        }
        // VANILLA: Mth.frac(double) = x − floor(x).
        let hy = cam[1] * 0.5;
        let v0 = -((hy - hy.floor()) as f32);
        let v1 = v0 + depth_far;
        // VANILLA: shouldRebuildWorldBorderBuffer — смена границ или invalidate() при переходе
        // камеры в другую секцию (LevelRenderer#repositionCamera).
        let sec = [
            (cam[0] / 16.0).floor(),
            (cam[1] / 16.0).floor(),
            (cam[2] / 16.0).floor(),
        ];
        let rebuild = self.last.is_none_or(|(l, sx, sy, sz)| {
            (l.min_x, l.min_z, l.max_x, l.max_z) != (s.min_x, s.min_z, s.max_x, s.max_z)
                || [sx, sy, sz] != sec
        });
        if rebuild {
            let (verts, mx, mz) = build(s, render_distance, cam[0], cam[2], depth_far, v1, v0);
            let mut data = Vec::with_capacity(16 * VERTEX_SIZE);
            for (p, uv) in verts {
                for c in p.iter().chain(uv.iter()) {
                    data.extend_from_slice(&c.to_le_bytes());
                }
            }
            queue.write_buffer(&self.vb, 0, &data);
            self.origin = (mx, mz);
            self.last = Some((*s, sec[0], sec[1], sec[2]));
        }
        let c = |sh: u32| ((s.tint >> sh) & 255) as f32 / 255.0;
        let off = (millis % 3000) as f32 / 3000.0;
        queue.write_buffer(
            &self.dyn_buf,
            0,
            &DynamicTransformsUniform {
                model_view_mat: model_view,
                color_modulator: [c(16), c(8), c(0), s.alpha as f32],
                model_offset: [
                    (self.origin.0 - cam[0]) as f32,
                    (-cam[1]) as f32,
                    (self.origin.1 - cam[2]) as f32,
                ],
                texture_mat: Mat4::from_translation(glam::Vec3::new(off, off, 0.0)),
            }
            .to_bytes(),
        );
        self.sides = sides(s, cam[0], cam[2], render_distance);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.sides.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_bind_group(1, &self.group1, &[]);
        pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, self.vb.slice(..));
        for &i in &self.sides {
            pass.draw_indexed(i * 6..i * 6 + 6, 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_and_sides() {
        let mut b = WorldBorder::default();
        b.set_size(100.0); // −50..50
        let s = extract(&b, 0.0, [45.0, 64.0, 0.0], 160.0);
        // 5 блоков до восточной стены: (1 − 5/160)^4.
        assert!((s.alpha - (1.0f64 - 5.0 / 160.0).powi(4)).abs() < 1e-9);
        assert_eq!(s.tint, 2138367);
        assert_eq!(
            sides(&s, 45.0, 0.0, 160.0),
            [3, 2, 0, 1],
            "север раньше юга при равенстве"
        );
        // Далеко снаружи — не рисуется.
        assert_eq!(extract(&b, 0.0, [500.0, 64.0, 0.0], 160.0).alpha, 0.0);
    }

    #[test]
    fn geometry_matches_vanilla_layout() {
        let s = BorderState {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
            tint: 0,
            alpha: 1.0,
        };
        let (v, mx, mz) = build(&s, 160.0, 0.0, 0.0, 512.0, 512.0, 0.0);
        assert_eq!(v.len(), 16);
        assert!((mx + 9.99).abs() < 1e-9 && (mz + 9.99).abs() < 1e-9);
        // Восточная стена (4-й квад) на borderMaxX − minX = 19.98.
        assert!((v[12].0[0] - 19.98).abs() < 1e-4);
    }
}
