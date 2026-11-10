//! Текстовый рендерер GUI: ванильный растровый шрифт `textures/font/ascii.png` (8×8, 16×16 ячеек).
//!
//! VANILLA: BitmapProvider (ширина глифа = правый непрозрачный столбец + 1, шаг = ширина + 1,
//! пробел = 4), Font#drawInBatch (тень: сдвиг (1,1), цвет ×0.25), GUI scale (Window#calculateScale:
//! наибольшее целое, при котором экран вмещает 320×240).
use wgpu::util::DeviceExt;

const MAX_VERTS: usize = 4 * 16384;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

/// Лист растрового шрифта (один `bitmap`-провайдер font/include/default.json).
pub struct FontSheet<'a> {
    pub rgba: &'a [u8],
    pub w: u32,
    pub h: u32,
    /// Строки `chars` (код U+0000 — пустая ячейка).
    pub rows: Vec<Vec<char>>,
    pub height: f32,
    pub ascent: f32,
}

/// Готовый глиф в объединённом атласе (пиксели атласа / GUI-пиксели).
#[derive(Clone, Copy)]
struct Glyph {
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
    w: f32,
    h: f32,
    /// Смещение верха от верха строки: 7 - ascent.
    top: f32,
    adv: f32,
}

pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    screen: wgpu::Buffer,
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    glyphs: rustc_hash::FxHashMap<char, Glyph>,
    verts: Vec<Vertex>,
    /// GUI scale текущего кадра.
    scale: u32,
    /// Опция «GUI Scale» (0 = авто).
    scale_opt: u32,
    /// Аффинное преобразование (центр, угол°, масштаб) для `push`/`fill` — сплэш.
    xf: Option<(f32, f32, f32, f32)>,
    rng: u32,
}

/// Флаги форматирования текста (ChatFormatting / Style).
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Fmt {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub obfuscated: bool,
    /// Первый символ строки: подчёркивание начинается на 1 px левее.
    pub line_start: bool,
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
    /// `sheets` — листы шрифта в порядке провайдеров (первый, где есть символ, выигрывает).
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        sheets: &[FontSheet],
    ) -> Self {
        let (atlas_rgba, atlas_w, atlas_h, glyphs) = build_atlas(sheets);
        let tex = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("font"),
                size: wgpu::Extent3d {
                    width: atlas_w,
                    height: atlas_h,
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
            &atlas_rgba,
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
            glyphs,
            verts: Vec::new(),
            scale: 1,
            scale_opt: 0,
            xf: None,
            rng: 12345,
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
            // Округление до целого пикселя окна: полпиксельные сдвиги при центрировании текста
            // давали обрезанные столбцы у букв (nearest-выборка).
            None => [(p[0] * k).round(), (p[1] * k).round()],
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
            self.glyph(c).map_or(0.0, |g| g.adv)
        }
    }

    /// Глиф символа; нет в шрифте → '?'.
    fn glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&c).or_else(|| self.glyphs.get(&'?'))
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
                    Some('e') => cur = 0xFFFF55,
                    Some('f') => cur = 0xFFFFFF,
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
        self.push_ex(s, x, y, color, false, false);
    }

    /// VANILLA: BakedSheetGlyph — жирный: второй проход со сдвигом 1; курсив: сдвиг верха/низа
    /// `1 - 0.25 * (y - 3)`.
    fn push_ex(&mut self, s: &str, x: f32, y: f32, color: [f32; 4], bold: bool, italic: bool) {
        let mut cx = x;
        for c in s.chars() {
            let adv = self.advance(c) + if bold { 1.0 } else { 0.0 };
            if c != ' ' && self.verts.len() + 8 <= MAX_VERTS {
                if let Some(g) = self.glyph(c).copied() {
                    let passes: &[f32] = if bold { &[0.0, 1.0] } else { &[0.0] };
                    for off in passes {
                        let (x0, x1) = (cx + off, cx + off + g.w);
                        let (y0, y1) = (y + g.top, y + g.top + g.h);
                        let sh = |rel: f32| {
                            if italic {
                                1.0 - 0.25 * (rel - 3.0)
                            } else {
                                0.0
                            }
                        };
                        let (s0, s1) = (sh(g.top), sh(g.top + g.h));
                        for (p, uv) in [
                            ([x0 + s0, y0], [g.u0, g.v0]),
                            ([x1 + s0, y0], [g.u1, g.v0]),
                            ([x1 + s1, y1], [g.u1, g.v1]),
                            ([x0 + s1, y1], [g.u0, g.v1]),
                        ] {
                            self.verts.push(Vertex {
                                pos: self.tf(p),
                                uv,
                                color,
                            });
                        }
                    }
                }
            }
            cx += adv;
        }
    }

    /// Ширина строки одним стилем (жирный: +1 к шагу каждого символа).
    pub fn width_fmt(&self, s: &str, bold: bool) -> f32 {
        s.chars()
            .map(|c| self.advance(c) + if bold { 1.0 } else { 0.0 })
            .sum()
    }

    /// Случайный символ с тем же шагом (VANILLA: «магический» текст §k).
    fn obfuscate(&mut self, c: char) -> char {
        if c == ' ' {
            return c;
        }
        let w = self.advance(c);
        self.rng = self.rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let n = 0x21 + (self.rng >> 16) % 0x5E;
        let cand = char::from_u32(n).unwrap_or('?');
        if (self.advance(cand) - w).abs() < f32::EPSILON {
            return cand;
        }
        // Подбор по соседям ASCII с тем же шагом.
        (0x21u32..0x7F)
            .cycle()
            .skip((n - 0x21) as usize)
            .take(0x5E)
            .filter_map(char::from_u32)
            .find(|ch| (self.advance(*ch) - w).abs() < f32::EPSILON)
            .unwrap_or(c)
    }

    /// Рисует строку со стилем Component (цвет 0xRRGGBB, прозрачность `alpha`), возвращает x после неё.
    /// VANILLA: Font.StringRenderOutput#accept.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_fmt(
        &mut self,
        s: &str,
        x: f32,
        y: f32,
        rgb: u32,
        alpha: f32,
        f: Fmt,
        shadow: bool,
    ) -> f32 {
        let mut cx = x;
        let col = |m: f32| {
            [
                ((rgb >> 16) & 255) as f32 / 255.0 * m,
                ((rgb >> 8) & 255) as f32 / 255.0 * m,
                (rgb & 255) as f32 / 255.0 * m,
                alpha,
            ]
        };
        let bold = f.bold;
        for (i, c) in s.chars().enumerate() {
            let adv = self.advance(c) + if bold { 1.0 } else { 0.0 };
            let ch = if f.obfuscated { self.obfuscate(c) } else { c };
            let mut tmp = [0u8; 4];
            let cs: &str = ch.encode_utf8(&mut tmp);
            if shadow {
                self.push_ex(cs, cx + 1.0, y + 1.0, col(0.25), bold, f.italic);
            }
            self.push_ex(cs, cx, y, col(1.0), bold, f.italic);
            let ex0 = if f.line_start && i == 0 { cx - 1.0 } else { cx };
            for (on, a, b) in [(f.strike, 3.5, 4.5), (f.underline, 8.0, 9.0)] {
                if on {
                    if shadow {
                        self.fill_f(
                            ex0 + 1.0,
                            y + a + 1.0,
                            cx + adv + 1.0,
                            y + b + 1.0,
                            col(0.25),
                        );
                    }
                    self.fill_f(ex0, y + a, cx + adv, y + b, col(1.0));
                }
            }
            cx += adv;
        }
        cx
    }

    fn fill_f(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4]) {
        if self.verts.len() + 4 > MAX_VERTS {
            return;
        }
        for p in [[x0, y0], [x1, y0], [x1, y1], [x0, y1]] {
            self.verts.push(Vertex {
                pos: self.tf(p),
                uv: [-1.0, -1.0],
                color,
            });
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

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: только POD (u32, Vertex из f32) без padding.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

/// VANILLA: BitmapProvider — ширина глифа = индекс правого непрозрачного столбца ячейки + 1.
fn cell_right(rgba: &[u8], px: u32, gx: usize, gy: usize, cw: usize, ch: usize) -> usize {
    let mut right = 0;
    for x in 0..cw {
        for y in 0..ch {
            if rgba[((gy + y) * px as usize + gx + x) * 4 + 3] != 0 {
                right = x + 1;
            }
        }
    }
    right
}

/// Склеивает листы в один атлас (вертикально) и строит таблицу глифов.
/// VANILLA: BitmapProvider (scale = height / высота ячейки; шаг = округл(ширина*scale) + 1).
fn build_atlas(sheets: &[FontSheet]) -> (Vec<u8>, u32, u32, rustc_hash::FxHashMap<char, Glyph>) {
    let aw = sheets.iter().map(|s| s.w).max().unwrap_or(1).max(1);
    let ah: u32 = sheets.iter().map(|s| s.h).sum::<u32>().max(1);
    let mut img = vec![0u8; (aw * ah * 4) as usize];
    let mut glyphs = rustc_hash::FxHashMap::default();
    let mut oy = 0u32;
    for sh in sheets {
        for y in 0..sh.h {
            let src = (y * sh.w * 4) as usize;
            let dst = (((oy + y) * aw) * 4) as usize;
            img[dst..dst + (sh.w * 4) as usize]
                .copy_from_slice(&sh.rgba[src..src + (sh.w * 4) as usize]);
        }
        let cols = sh.rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
        let (cw, chh) = (sh.w as usize / cols, sh.h as usize / sh.rows.len().max(1));
        let scale = sh.height / chh.max(1) as f32;
        for (r, row) in sh.rows.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '\0' || glyphs.contains_key(&ch) {
                    continue;
                }
                let (gx, gy) = (c * cw, r * chh);
                let right = cell_right(sh.rgba, sh.w, gx, gy, cw, chh);
                if right == 0 {
                    continue;
                }
                let pw = right as f32 * scale;
                glyphs.insert(
                    ch,
                    Glyph {
                        u0: gx as f32 / aw as f32,
                        v0: (oy as usize + gy) as f32 / ah as f32,
                        u1: (gx + right) as f32 / aw as f32,
                        v1: (oy as usize + gy + chh) as f32 / ah as f32,
                        w: pw,
                        h: sh.height,
                        top: 7.0 - sh.ascent,
                        adv: (pw + 0.5).floor() + 1.0,
                    },
                );
            }
        }
        oy += sh.h;
    }
    (img, aw, ah, glyphs)
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
        // Ячейка (1, 4) 8×8; закрасить столбец x=4 в строке 0.
        px[((4 * 8) * 128 + 8 + 4) * 4 + 3] = 255;
        assert_eq!(cell_right(&px, 128, 8, 32, 8, 8), 5);
        assert_eq!(cell_right(&px, 128, 16, 32, 8, 8), 0);
    }

    #[test]
    fn multi_sheet_cyrillic() {
        let mut px = vec![0u8; 16 * 8 * 8 * 4];
        px[3 * 4 + 3] = 255; // ячейка 0, столбец 3 → ширина 4
        let sheet = FontSheet {
            rgba: &px,
            w: 16 * 8,
            h: 8,
            rows: vec![std::iter::once('Ж')
                .chain(std::iter::repeat_n('\0', 15))
                .collect()],
            height: 8.0,
            ascent: 7.0,
        };
        let (_, _, _, g) = build_atlas(&[sheet]);
        assert_eq!(g[&'Ж'].adv, 5.0);
    }
}
