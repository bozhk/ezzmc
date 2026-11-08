//! Облака. VANILLA: CloudRenderer (26.3) — ячейки 12×4×12 из `environment/clouds.png`,
//! грани строятся на CPU (`buildMesh`), шейдер `clouds.wgsl` тянет вершины из storage-буфера.

use crate::shaders::Shader;
use crate::uniforms::{CloudInfoUniform, DynamicTransformsUniform};
use glam::Mat4;

const CELL: f32 = 12.0;
const TICKS_PER_CELL: i64 = 400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RelPos {
    Above,
    Inside,
    Below,
}

/// Режим облаков (`CloudStatus`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CloudStatus {
    Fast,
    Fancy,
}

/// Ячейка: 0 — пусто; иначе `color << 4 | n<<3 | e<<2 | s<<1 | w` (бит = сосед пуст).
struct Texture {
    cells: Vec<u64>,
    w: i32,
    h: i32,
}

fn empty(c: u32) -> bool {
    (c >> 24) < 10
}

impl Texture {
    /// `px` — ARGB.
    fn new(px: &[u32], w: u32, h: u32) -> Self {
        let (w, h) = (w as i32, h as i32);
        let at = |x: i32, y: i32| px[(x.rem_euclid(w) + y.rem_euclid(h) * w) as usize];
        let mut cells = vec![0u64; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let c = at(x, y);
                if empty(c) {
                    continue;
                }
                // Как в ваниле (floorMod(x±1, height) для x — баг-совместимо; текстура квадратная).
                let n = empty(at(x, y - 1));
                let e = empty(at(x + 1, y));
                let s = empty(at(x, y + 1));
                let wst = empty(at(x - 1, y));
                cells[(x + y * w) as usize] = (u64::from(c) << 4)
                    | (u64::from(n) << 3)
                    | (u64::from(e) << 2)
                    | (u64::from(s) << 1)
                    | u64::from(wst);
            }
        }
        Texture { cells, w, h }
    }
}

pub struct CloudDraw {
    tex: Option<Texture>,
    ubo: wgpu::Buffer,
    dyn_buf: wgpu::Buffer,
    group0: wgpu::BindGroup,
    layout1: wgpu::BindGroupLayout,
    faces: Option<(wgpu::Buffer, wgpu::BindGroup, usize)>,
    index: Option<(wgpu::Buffer, usize)>,
    p_fancy: wgpu::RenderPipeline,
    p_flat: wgpu::RenderPipeline,
    quads: u32,
    need_rebuild: bool,
    prev: (i32, i32, RelPos, Option<CloudStatus>),
    status: CloudStatus,
}

impl CloudDraw {
    /// `px` — ARGB-пиксели `clouds.png` (None → облаков нет).
    pub fn new(
        device: &wgpu::Device,
        color: wgpu::TextureFormat,
        depth: wgpu::TextureFormat,
        projection: &wgpu::Buffer,
        fog: &wgpu::Buffer,
        image: Option<(&[u32], u32, u32)>,
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
        let l0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("clouds.group0"),
            entries: &[uni(0), uni(1), uni(3), uni(6)],
        });
        let layout1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("clouds.group1"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let mk = |label, size: usize| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let ubo = mk("clouds.info", CloudInfoUniform::SIZE);
        let dyn_buf = mk("clouds.dyn", DynamicTransformsUniform::SIZE);
        let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("clouds.group0"),
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
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: ubo.as_entire_binding(),
                },
            ],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("clouds.pl"),
            bind_group_layouts: &[&l0, &layout1],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("clouds"),
            source: wgpu::ShaderSource::Wgsl(Shader::Clouds.source().into()),
        });
        let make = |label: &str, cull: Option<wgpu::Face>| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&pl),
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
                        format: color,
                        // BlendFunction.TRANSLUCENT
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
                primitive: wgpu::PrimitiveState {
                    cull_mode: cull,
                    ..Default::default()
                },
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
            })
        };
        CloudDraw {
            tex: image.map(|(px, w, h)| Texture::new(px, w, h)),
            ubo,
            dyn_buf,
            group0,
            layout1,
            faces: None,
            index: None,
            p_fancy: make("clouds.fancy", Some(wgpu::Face::Back)),
            p_flat: make("clouds.flat", None),
            quads: 0,
            need_rebuild: true,
            prev: (i32::MIN, i32::MIN, RelPos::Inside, None),
            status: CloudStatus::Fancy,
        }
    }

    pub fn mark_for_rebuild(&mut self) {
        self.need_rebuild = true;
    }

    /// VANILLA: CloudRenderer#prepare. `color` — ARGB; `range_chunks` — cloudRange (по умолчанию 128).
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_view: Mat4,
        color: u32,
        status: CloudStatus,
        bottom_y: f32,
        range_chunks: i32,
        cam: [f64; 3],
        game_time: i64,
        partial: f32,
    ) {
        self.status = status;
        let Some(tex) = &self.tex else {
            self.quads = 0;
            return;
        };
        let radius_cells = ((range_chunks * 16) as f32 / CELL).ceil() as i32;
        let max_cells = (radius_cells + 1) * 2 * (radius_cells + 1) * 2 / 2;
        let max_faces = (max_cells * 4 + 54) as usize;
        if self.faces.as_ref().is_none_or(|f| f.2 != max_faces) {
            let buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("clouds.faces"),
                size: (max_faces * 3 * 4) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("clouds.group1"),
                layout: &self.layout1,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buf.as_entire_binding(),
                }],
            });
            self.faces = Some((buf, bg, max_faces));
            self.need_rebuild = true;
        }
        if self.index.as_ref().is_none_or(|i| i.1 != max_faces) {
            let mut idx = Vec::with_capacity(max_faces * 6);
            for q in 0..max_faces as u32 {
                let b = q * 4;
                idx.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
            }
            let buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("clouds.index"),
                size: (idx.len() * 4) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(&buf, 0, bytemuck_u32(&idx));
            self.index = Some((buf, max_faces));
        }
        let rel_bottom = (f64::from(bottom_y) - cam[1]) as f32;
        let rel_top = rel_bottom + 4.0;
        let rel = if rel_top < 0.0 {
            RelPos::Above
        } else if rel_bottom > 0.0 {
            RelPos::Below
        } else {
            RelPos::Inside
        };
        let offset = (game_time % (i64::from(tex.w) * TICKS_PER_CELL)) as f32 + partial;
        let mut cx = cam[0] + f64::from(offset * 0.030_000_001);
        let mut cz = cam[2] + f64::from(3.96_f32);
        let (tw, th) = (f64::from(tex.w) * 12.0, f64::from(tex.h) * 12.0);
        cx -= (cx / tw).floor() * tw;
        cz -= (cz / th).floor() * th;
        let cell_x = (cx / 12.0).floor() as i32;
        let cell_z = (cz / 12.0).floor() as i32;
        let x_in = (cx - f64::from(cell_x as f32 * 12.0)) as f32;
        let z_in = (cz - f64::from(cell_z as f32 * 12.0)) as f32;
        if self.need_rebuild || self.prev != (cell_x, cell_z, rel, Some(status)) {
            self.need_rebuild = false;
            self.prev = (cell_x, cell_z, rel, Some(status));
            let mut out: Vec<i32> = Vec::new();
            build_mesh(
                tex,
                rel,
                &mut out,
                cell_x,
                cell_z,
                status == CloudStatus::Fancy,
                radius_cells,
            );
            out.truncate(max_faces * 3);
            self.quads = (out.len() / 3) as u32;
            if let Some((buf, _, _)) = &self.faces {
                queue.write_buffer(buf, 0, bytemuck_i32(&out));
            }
        }
        if self.quads == 0 {
            return;
        }
        let c = |s: u32| ((color >> s) & 255) as f32 / 255.0;
        queue.write_buffer(
            &self.ubo,
            0,
            &CloudInfoUniform {
                cloud_color: [c(16), c(8), c(0), c(24)],
                cloud_offset: [-x_in, rel_bottom, -z_in],
                cell_size: [12.0, 4.0, 12.0],
            }
            .to_bytes(),
        );
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
        let (Some((_, bg, _)), Some((ib, _))) = (&self.faces, &self.index) else {
            return;
        };
        if self.quads == 0 {
            return;
        }
        pass.set_pipeline(if self.status == CloudStatus::Fancy {
            &self.p_fancy
        } else {
            &self.p_flat
        });
        pass.set_bind_group(0, &self.group0, &[]);
        pass.set_bind_group(1, bg, &[]);
        pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.quads * 6, 0, 0..1);
    }
}

fn bytemuck_u32(v: &[u32]) -> &[u8] {
    // SAFETY: u32 → байты, выравнивание 1.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}
fn bytemuck_i32(v: &[i32]) -> &[u8] {
    // SAFETY: i32 → байты, выравнивание 1.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

// Direction#get3DDataValue: DOWN 0, UP 1, NORTH 2, SOUTH 3, WEST 4, EAST 5.
const DOWN: i32 = 0;
const UP: i32 = 1;
const NORTH: i32 = 2;
const SOUTH: i32 = 3;
const WEST: i32 = 4;
const EAST: i32 = 5;

/// VANILLA: encodeFace — 3 байта (как i8, расширенные до i32 для storage-буфера).
fn encode(out: &mut Vec<i32>, x: i32, z: i32, dir: i32, flags: i32) {
    let mut d = dir | flags;
    d |= (x & 1) << 7;
    d |= (z & 1) << 6;
    out.push(i32::from((x >> 1) as i8));
    out.push(i32::from((z >> 1) as i8));
    out.push(i32::from(d as u8 as i8));
}

fn build_mesh(
    t: &Texture,
    rel: RelPos,
    out: &mut Vec<i32>,
    cx: i32,
    cz: i32,
    extrude: bool,
    radius: i32,
) {
    let mut cell = |rx: i32, rz: i32| {
        let ix = (cx + rx).rem_euclid(t.w);
        let iy = (cz + rz).rem_euclid(t.h);
        let d = t.cells[(ix + iy * t.w) as usize];
        if d == 0 {
            return;
        }
        if !extrude {
            encode(out, rx, rz, DOWN, 32);
            return;
        }
        if rel != RelPos::Below {
            encode(out, rx, rz, UP, 0);
        }
        if rel != RelPos::Above {
            encode(out, rx, rz, DOWN, 0);
        }
        if (d >> 3) & 1 != 0 && rz > 0 {
            encode(out, rx, rz, NORTH, 0);
        }
        if (d >> 1) & 1 != 0 && rz < 0 {
            encode(out, rx, rz, SOUTH, 0);
        }
        if d & 1 != 0 && rx > 0 {
            encode(out, rx, rz, WEST, 0);
        }
        if (d >> 2) & 1 != 0 && rx < 0 {
            encode(out, rx, rz, EAST, 0);
        }
        if rx.abs() <= 1 && rz.abs() <= 1 {
            for dir in 0..6 {
                encode(out, rx, rz, dir, 16);
            }
        }
    };
    for ring in 0..=2 * radius {
        for rx in -ring..=ring {
            let rz = ring - rx.abs();
            if rz >= 0 && rz <= radius && rx * rx + rz * rz <= radius * radius {
                if rz != 0 {
                    cell(rx, -rz);
                }
                cell(rx, rz);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cell_fancy_faces() {
        // 3×3 текстура, одна непустая ячейка в центре.
        let mut px = vec![0u32; 9];
        px[4] = 0xFFFF_FFFF;
        let t = Texture::new(&px, 3, 3);
        assert_ne!(t.cells[4], 0);
        assert_eq!(t.cells[4] & 15, 15); // все соседи пусты
        let mut out = Vec::new();
        build_mesh(&t, RelPos::Below, &mut out, 1, 1, true, 2);
        // Камера в ячейке (0,0): DOWN + 6 внутренних; боковые грани у центра не добавляются (rz=rx=0).
        assert_eq!(out.len() / 3, 1 + 6);
    }

    #[test]
    fn flat_one_face_per_cell() {
        let px = vec![0xFFFF_FFFFu32; 4];
        let t = Texture::new(&px, 2, 2);
        let mut out = Vec::new();
        build_mesh(&t, RelPos::Below, &mut out, 0, 0, false, 1);
        // радиус 1: (0,0), (±1,0), (0,±1) = 5 ячеек
        assert_eq!(out.len() / 3, 5);
    }
}
