//! Текстовый рендерер GUI: ванильный растровый шрифт `textures/font/ascii.png` (8×8, 16×16 ячеек).
//!
//! VANILLA: BitmapProvider (ширина глифа = правый непрозрачный столбец + 1, шаг = ширина + 1,
//! пробел = 4), Font#drawInBatch (тень: сдвиг (1,1), цвет ×0.25), GUI scale (Window#calculateScale:
//! наибольшее целое, при котором экран вмещает 320×240).
use wgpu::util::DeviceExt;

const CELL: u32 = 8;
const GRID: u32 = 16;
const MAX_VERTS: usize = 4 * 16384;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    screen: wgpu::Buffer,
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    atlas_px: u32,
    /// Ширина (без интервала) каждого из 256 кодов.
    widths: [u8; 256],
    verts: Vec<Vertex>,
    /// GUI scale текущего кадра.
    scale: u32,
    /// Опция «GUI Scale» (0 = авто).
    scale_opt: u32,
    /// Аффинное преобразование (центр, угол°, масштаб) для `push`/`fill` — сплэш.
    xf: Option<(f32, f32, f32, f32)>,
}

/// VANILLA: Window#calculateScale (guiScale = 0 «авто», без ограничения максимума).
pub fn gui_scale(w: u32, h: u32) -> u32 {
    let mut s = 1;
    while w / (s + 1) >= 320 && h / (s + 1) >= 240 {
        s += 1;
    }
    s
}

/// VANILLA: Window#calculateScale — `opt` > 0 ограничивает авто-максимум.
pub fn gui_scale_opt(w: u32, h: u32, opt: u32) -> u32 {
    let auto = gui_scale(w, h);
    if opt == 0 {
        auto
    } else {
        opt.min(auto).max(1)
    }
}

impl TextRenderer {
    /// `atlas_rgba` — `ascii.png` как RGBA8, квадрат `GRID*CELL`×`GRID*CELL` (или кратный).
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        atlas_rgba: &[u8],
        atlas_px: u32,
    ) -> Self {
        let tex = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("font"),
                size: wgpu::Extent3d {
                    width: atlas_px,
                    height: atlas_px,
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
            atlas_rgba,
        );
        let view = tex.create_view(&Default::default());
        let samp = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let screen = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text screen"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vb = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text vb"),
            size: (MAX_VERTS * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let idx: Vec<u32> = (0..(MAX_VERTS / 4) as u32)
            .flat_map(|q| [q * 4, q * 4 + 1, q * 4 + 2, q * 4, q * 4 + 2, q * 4 + 3])
            .collect();
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text ib"),
            contents: bytes(&idx),
            usage: wgpu::BufferUsages::INDEX,
        });
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen.as_entire_binding(),
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
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/text.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text"),
            bind_group_layouts: &[&bgl],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
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
        });
        Self {
            pipeline,
            bind_group,
            screen,
            vb,
            ib,
            atlas_px,
            widths: glyph_widths(atlas_rgba, atlas_px),
            verts: Vec::new(),
            scale: 1,
            scale_opt: 0,
            xf: None,
        }
    }

    /// Начало кадра: сбросить накопленный текст и выбрать GUI scale под размер окна.
    pub fn begin(&mut self, w: u32, h: u32) {
        self.verts.clear();
        self.scale = gui_scale_opt(w, h, self.scale_opt);
        self.xf = None;
    }

    pub fn set_scale_option(&mut self, opt: u32) {
        self.scale_opt = opt;
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    /// Преобразование для последующих `draw`/`fill`: поворот `deg`° и масштаб вокруг (cx, cy);
    /// координаты рисования — относительно этой точки. `None` — сброс.
    pub fn set_transform(&mut self, xf: Option<(f32, f32, f32, f32)>) {
        self.xf = xf;
    }

    fn tf(&self, p: [f32; 2]) -> [f32; 2] {
        let k = self.scale as f32;
        match self.xf {
            None => [p[0] * k, p[1] * k],
            Some((cx, cy, deg, sc)) => {
                let (s, c) = deg.to_radians().sin_cos();
                let (x, y) = (p[0] * sc, p[1] * sc);
                [(cx + x * c - y * s) * k, (cy + x * s + y * c) * k]
            }
        }
    }

    /// Размер экрана в GUI-пикселях (то, во что считаются координаты `draw`).
    pub fn gui_size(&self, w: u32, h: u32) -> (u32, u32) {
        (w / self.scale, h / self.scale)
    }

    /// Шаг символа в GUI-пикселях.
    fn advance(&self, c: char) -> f32 {
        if c == ' ' {
            4.0
        } else {
            f32::from(self.widths[code(c)]) + 1.0
        }
    }

    pub fn text_width(&self, s: &str) -> f32 {
        s.chars()
            .filter(|c| *c != '§')
            .scan(false, |skip, c| {
                let was = std::mem::replace(skip, false);
                Some(if was { 0.0 } else { self.advance(c) })
            })
            .sum()
    }

    /// Рисует строку с левым верхним углом (x, y) в GUI-пикселях. `rgb` — 0xRRGGBB.
    /// `shadow` — ванильная тень.
    pub fn draw(&mut self, s: &str, x: f32, y: f32, rgb: u32, shadow: bool) {
        // VANILLA: коды форматирования §a (зелёный) / §c (красный) / §r (сброс); подчёркивание не рисуем.
        let mut cx = x;
        let mut cur = rgb;
        let mut rest = s;
        while !rest.is_empty() {
            let end = rest.find('§').unwrap_or(rest.len());
            let (seg, tail) = rest.split_at(end);
            let c = |m: f32| {
                [
                    ((cur >> 16) & 255) as f32 / 255.0 * m,
                    ((cur >> 8) & 255) as f32 / 255.0 * m,
                    (cur & 255) as f32 / 255.0 * m,
                    1.0,
                ]
            };
            if !seg.is_empty() {
                if shadow {
                    self.push(seg, cx + 1.0, y + 1.0, c(0.25));
                }
                self.push(seg, cx, y, c(1.0));
                cx += seg.chars().map(|ch| self.advance(ch)).sum::<f32>();
            }
            let mut it = tail.chars();
            if it.next().is_some() {
                match it.next() {
                    Some('a') => cur = 0x55FF55,
                    Some('c') => cur = 0xFF5555,
                    Some('r') => cur = rgb,
                    _ => {}
                }
            }
            rest = it.as_str();
        }
    }

    /// Заливка прямоугольника (GUI-пиксели), `argb` — 0xAARRGGBB. VANILLA: GuiGraphics#fill.
    pub fn fill(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, argb: u32) {
        if self.verts.len() + 4 > MAX_VERTS {
            return;
        }
        let c = |sh: u32| ((argb >> sh) & 255) as f32 / 255.0;
        let color = [c(16), c(8), c(0), c(24)];
        for p in [[x0, y0], [x1, y0], [x1, y1], [x0, y1]] {
            self.verts.push(Vertex {
                pos: self.tf(p),
                uv: [-1.0, -1.0],
                color,
            });
        }
    }

    fn push(&mut self, s: &str, x: f32, y: f32, color: [f32; 4]) {
        let px = self.atlas_px as f32;
        let tex_per_cell = px / (GRID * CELL) as f32;
        let mut cx = x;
        for c in s.chars() {
            let adv = self.advance(c);
            if c != ' ' && self.verts.len() + 4 <= MAX_VERTS {
                let i = code(c) as u32;
                let w = f32::from(self.widths[i as usize]);
                let (u0, v0) = (
                    (i % GRID * CELL) as f32 * tex_per_cell / px,
                    (i / GRID * CELL) as f32 * tex_per_cell / px,
                );
                let u1 = u0 + w * tex_per_cell / px;
                let v1 = v0 + CELL as f32 * tex_per_cell / px;
                let (x0, x1) = (cx, cx + w);
                let (y0, y1) = (y, y + CELL as f32);
                for (p, uv) in [
                    ([x0, y0], [u0, v0]),
                    ([x1, y0], [u1, v0]),
                    ([x1, y1], [u1, v1]),
                    ([x0, y1], [u0, v1]),
                ] {
                    self.verts.push(Vertex {
                        pos: self.tf(p),
                        uv,
                        color,
                    });
                }
            }
            cx += adv;
        }
    }

    /// Рисует накопленное поверх `target` (LoadOp::Load) своим проходом.
    pub fn flush(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        w: u32,
        h: u32,
    ) {
        if self.verts.is_empty() {
            return;
        }
        let mut u = Vec::with_capacity(16);
        for v in [w as f32, h as f32, 0.0, 0.0] {
            u.extend_from_slice(&v.to_le_bytes());
        }
        queue.write_buffer(&self.screen, 0, &u);
        queue.write_buffer(&self.vb, 0, bytes(&self.verts));
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("text"),
        });
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_vertex_buffer(0, self.vb.slice(..));
            pass.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..(self.verts.len() / 4 * 6) as u32, 0, 0..1);
        }
        queue.submit([enc.finish()]);
    }
}

fn code(c: char) -> usize {
    let n = c as usize;
    if n < 256 {
        n
    } else {
        usize::from(b'?')
    }
}

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: только POD (u32, Vertex из f32) без padding.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

/// VANILLA: BitmapProvider — ширина = индекс правого непрозрачного столбца ячейки + 1 (0 для пустой).
fn glyph_widths(rgba: &[u8], px: u32) -> [u8; 256] {
    let cell = (px / GRID) as usize;
    let scale = cell / CELL as usize;
    let mut out = [0u8; 256];
    for (i, o) in out.iter_mut().enumerate() {
        let (gx, gy) = (i % GRID as usize * cell, i / GRID as usize * cell);
        let mut right = 0;
        for x in 0..cell {
            for y in 0..cell {
                if rgba[((gy + y) * px as usize + gx + x) * 4 + 3] != 0 {
                    right = x + 1;
                }
            }
        }
        // Ширина в «ванильных» 8-пиксельных единицах.
        *o = right.div_ceil(scale.max(1)) as u8;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_matches_vanilla() {
        assert_eq!(gui_scale(854, 480), 2);
        assert_eq!(gui_scale(1920, 1080), 4);
        assert_eq!(gui_scale(320, 240), 1);
        assert_eq!(gui_scale(100, 100), 1);
    }

    #[test]
    fn widths_from_alpha() {
        let mut px = vec![0u8; 128 * 128 * 4];
        // Код 65 ('A'): ячейка (1, 4); закрасить столбец x=4 в строке 0.
        px[((4 * 8) * 128 + 8 + 4) * 4 + 3] = 255;
        let w = glyph_widths(&px, 128);
        assert_eq!(w[65], 5);
        assert_eq!(w[66], 0);
    }
}
