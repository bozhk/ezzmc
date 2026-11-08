//! Изображения ARGB, PNG-декодер, мипмапы (TODO-092): порт `NativeImage`-логики 1.21.11.
use crate::error::{AssetError, AssetResult};
use std::sync::OnceLock;

/// Пиксели ARGB (0xAARRGGBB), как `NativeImage#getPixel` в ванили.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub px: Vec<u32>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            px: vec![0; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> u32 {
        self.px[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, v: u32) {
        self.px[(y * self.width + x) as usize] = v;
    }

    /// PNG → ARGB (любой формат PNG приводится к RGBA8).
    pub fn decode_png(data: &[u8]) -> AssetResult<Image> {
        let img = image::load_from_memory_with_format(data, image::ImageFormat::Png)
            .map_err(|e| AssetError::Json {
                path: "png".into(),
                msg: e.to_string(),
            })?
            .to_rgba8();
        let (w, h) = img.dimensions();
        let px = img
            .pixels()
            .map(|p| {
                let [r, g, b, a] = p.0;
                u32::from(a) << 24 | u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b)
            })
            .collect();
        Ok(Image {
            width: w,
            height: h,
            px,
        })
    }

    /// Байты RGBA для загрузки в GPU.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.px.len() * 4);
        for &p in &self.px {
            out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8, (p >> 24) as u8]);
        }
        out
    }

    /// Копия прямоугольника (для кадров анимации).
    pub fn crop(&self, x: u32, y: u32, w: u32, h: u32) -> Image {
        let mut o = Image::new(w, h);
        for yy in 0..h {
            for xx in 0..w {
                o.set(xx, yy, self.get(x + xx, y + yy));
            }
        }
        o
    }
}

#[inline]
fn alpha(p: u32) -> u32 {
    p >> 24
}
#[inline]
fn red(p: u32) -> u32 {
    p >> 16 & 0xFF
}
#[inline]
fn green(p: u32) -> u32 {
    p >> 8 & 0xFF
}
#[inline]
fn blue(p: u32) -> u32 {
    p & 0xFF
}
#[inline]
fn argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    (a & 0xFF) << 24 | (r & 0xFF) << 16 | (g & 0xFF) << 8 | (b & 0xFF)
}
fn as_8bit(f: f32) -> u32 {
    (f.clamp(0.0, 1.0) * 255.0 + 0.5).floor() as u32 // Mth.floor(f*255+0.5) с зажимом
}

// VANILLA: ARGB#SRGB_TO_LINEAR / LINEAR_TO_SRGB (1.21.11)
struct Tables {
    to_linear: [i16; 256],
    to_srgb: [u8; 1024],
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mut to_linear = [0i16; 256];
        for (i, v) in to_linear.iter_mut().enumerate() {
            let f = i as f32 / 255.0;
            let l = if f >= 0.04045 {
                ((f as f64 + 0.055) / 1.055).powf(2.4) as f32
            } else {
                f / 12.92
            };
            *v = (l * 1023.0).round() as i16;
        }
        let mut to_srgb = [0u8; 1024];
        for (i, v) in to_srgb.iter_mut().enumerate() {
            let f = i as f32 / 1023.0;
            let s = if f >= 0.003_130_8 {
                (1.055 * (f as f64).powf(0.416_666_666_666_666_7) - 0.055) as f32
            } else {
                12.92 * f
            };
            *v = (s * 255.0).round() as u8;
        }
        Tables { to_linear, to_srgb }
    })
}

fn srgb_to_linear_channel(c: u32) -> f32 {
    f32::from(tables().to_linear[c as usize]) / 1023.0
}

fn linear_to_srgb_channel(f: f32) -> u32 {
    u32::from(tables().to_srgb[((f * 1023.0).floor() as i32).clamp(0, 1023) as usize])
}

fn linear_mean(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let t = tables();
    let s = (i32::from(t.to_linear[a as usize])
        + i32::from(t.to_linear[b as usize])
        + i32::from(t.to_linear[c as usize])
        + i32::from(t.to_linear[d as usize]))
        / 4;
    u32::from(t.to_srgb[s as usize])
}

// VANILLA: ARGB#meanLinear
fn mean_linear(p: [u32; 4]) -> u32 {
    argb(
        (alpha(p[0]) + alpha(p[1]) + alpha(p[2]) + alpha(p[3])) / 4,
        linear_mean(red(p[0]), red(p[1]), red(p[2]), red(p[3])),
        linear_mean(green(p[0]), green(p[1]), green(p[2]), green(p[3])),
        linear_mean(blue(p[0]), blue(p[1]), blue(p[2]), blue(p[3])),
    )
}

// VANILLA: MipmapGenerator#darkenedAlphaBlend
fn darkened_alpha_blend(p: [u32; 4]) -> u32 {
    let (mut a, mut r, mut g, mut b) = (0f32, 0f32, 0f32, 0f32);
    for &q in &p {
        if alpha(q) != 0 {
            a += srgb_to_linear_channel(alpha(q));
            r += srgb_to_linear_channel(red(q));
            g += srgb_to_linear_channel(green(q));
            b += srgb_to_linear_channel(blue(q));
        }
    }
    argb(
        linear_to_srgb_channel(a / 4.0),
        linear_to_srgb_channel(r / 4.0),
        linear_to_srgb_channel(g / 4.0),
        linear_to_srgb_channel(b / 4.0),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MipmapStrategy {
    Auto,
    Mean,
    Cutout,
    StrictCutout,
    DarkCutout,
}

// VANILLA: TextureUtil#solidify — заливка прозрачных пикселей цветом ближайшего непрозрачного (BFS).
pub fn solidify(img: &mut Image) {
    let (w, h) = (img.width as usize, img.height as usize);
    let mut col = vec![0u32; w * h];
    let mut dist = vec![i32::MAX; w * h];
    let mut q = std::collections::VecDeque::new();
    for x in 0..w {
        for y in 0..h {
            let p = img.px[y * w + x];
            if alpha(p) != 0 {
                dist[y * w + x] = 0;
                col[y * w + x] = p;
                q.push_back(y * w + x);
            }
        }
    }
    while let Some(i) = q.pop_front() {
        let (cx, cy) = ((i % w) as i32, (i / w) as i32);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (cx + dx, cy + dy);
            if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                let n = ny as usize * w + nx as usize;
                if dist[n] > dist[i] + 1 {
                    dist[n] = dist[i] + 1;
                    col[n] = col[i];
                    q.push_back(n);
                }
            }
        }
    }
    for (p, c) in img.px.iter_mut().zip(&col) {
        if alpha(*p) == 0 {
            *p = c & 0x00FF_FFFF; // alpha 0, цвет соседа
        }
    }
}

// VANILLA: TextureUtil#fillEmptyAreasWithDarkColor
pub fn fill_empty_with_dark(img: &mut Image) {
    let mut best: Option<(u32, u32)> = None; // (сумма, пиксель)
    for x in 0..img.width {
        for y in 0..img.height {
            let p = img.get(x, y);
            if alpha(p) != 0 {
                let s = red(p) + green(p) + blue(p);
                if best.is_none_or(|(bs, _)| s < bs) {
                    best = Some((s, p));
                }
            }
        }
    }
    let Some((_, p)) = best else { return };
    let c = argb(0, 3 * red(p) / 4, 3 * green(p) / 4, 3 * blue(p) / 4);
    for v in &mut img.px {
        if alpha(*v) == 0 {
            *v = c;
        }
    }
}

fn has_transparent(img: &Image) -> bool {
    img.px.iter().any(|&p| alpha(p) == 0)
}

// VANILLA: MipmapGenerator#alphaTestCoverage
fn alpha_test_coverage(img: &Image, cutoff: f32, scale: f32) -> f32 {
    let (w, h) = (img.width, img.height);
    if w < 2 || h < 2 {
        return 0.0;
    }
    let a = |x: u32, y: u32| (alpha(img.get(x, y)) as f32 / 255.0 * scale).clamp(0.0, 1.0);
    let mut total = 0f32;
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let (a0, a1, a2, a3) = (a(x, y), a(x + 1, y), a(x, y + 1), a(x + 1, y + 1));
            let mut n = 0f32;
            for j in 0..4 {
                let fy = (j as f32 + 0.5) / 4.0;
                for i in 0..4 {
                    let fx = (i as f32 + 0.5) / 4.0;
                    let v = a0 * (1.0 - fx) * (1.0 - fy)
                        + a1 * fx * (1.0 - fy)
                        + a2 * (1.0 - fx) * fy
                        + a3 * fx * fy;
                    if v > cutoff {
                        n += 1.0;
                    }
                }
            }
            total += n / 16.0;
        }
    }
    total / ((w - 1) * (h - 1)) as f32
}

// VANILLA: MipmapGenerator#scaleAlphaToCoverage
fn scale_alpha_to_coverage(img: &mut Image, desired: f32, cutoff: f32, bias: f32) {
    let (mut lo, mut hi, mut s, mut best_s, mut best_d) = (0f32, 4f32, 1f32, 1f32, f32::MAX);
    for _ in 0..5 {
        let c = alpha_test_coverage(img, cutoff, s);
        let d = (c - desired).abs();
        if d < best_d {
            best_d = d;
            best_s = s;
        }
        if c < desired {
            lo = s;
        } else if c > desired {
            hi = s;
        } else {
            break;
        }
        s = (lo + hi) * 0.5;
    }
    for p in &mut img.px {
        let a = (alpha(*p) as f32 / 255.0 * best_s + bias + 0.025).clamp(0.0, 1.0);
        *p = as_8bit(a) << 24 | *p & 0x00FF_FFFF;
    }
}

/// Мип-цепочка `[0..=levels]` из базового изображения.
/// `name_path` — путь спрайта (для `item/` solidify не применяется).
// VANILLA: MipmapGenerator#generateMipLevels (1.21.11)
pub fn generate_mip_levels(
    name_path: &str,
    base: Image,
    levels: u32,
    strategy: MipmapStrategy,
    alpha_bias: f32,
) -> Vec<Image> {
    let mut strategy = strategy;
    if strategy == MipmapStrategy::Auto {
        strategy = if has_transparent(&base) {
            MipmapStrategy::Cutout
        } else {
            MipmapStrategy::Mean
        };
    }
    let mut base = base;
    if !name_path.starts_with("item/") {
        match strategy {
            MipmapStrategy::Cutout | MipmapStrategy::StrictCutout => solidify(&mut base),
            MipmapStrategy::DarkCutout => fill_empty_with_dark(&mut base),
            _ => {}
        }
    }
    let cut = matches!(
        strategy,
        MipmapStrategy::Cutout | MipmapStrategy::StrictCutout | MipmapStrategy::DarkCutout
    );
    let cutoff = if strategy == MipmapStrategy::StrictCutout {
        0.3
    } else {
        0.5
    };
    let coverage = if cut {
        alpha_test_coverage(&base, cutoff, 1.0)
    } else {
        0.0
    };
    let mut out = vec![base];
    for _ in 1..=levels {
        let prev = &out[out.len() - 1];
        let (w, h) = (prev.width >> 1, prev.height >> 1);
        let mut next = Image::new(w, h);
        for x in 0..w {
            for y in 0..h {
                let p = [
                    prev.get(x * 2, y * 2),
                    prev.get(x * 2 + 1, y * 2),
                    prev.get(x * 2, y * 2 + 1),
                    prev.get(x * 2 + 1, y * 2 + 1),
                ];
                next.set(
                    x,
                    y,
                    if strategy == MipmapStrategy::DarkCutout {
                        darkened_alpha_blend(p)
                    } else {
                        mean_linear(p)
                    },
                );
            }
        }
        if cut {
            scale_alpha_to_coverage(&mut next, coverage, cutoff, alpha_bias);
        }
        out.push(next);
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn decode_real_png_and_mean() {
        // 2×2 PNG, сформированный через image
        let mut buf = Vec::new();
        let rgba = image::RgbaImage::from_fn(2, 2, |x, y| {
            if (x + y) % 2 == 0 {
                image::Rgba([255, 0, 0, 255])
            } else {
                image::Rgba([0, 0, 255, 255])
            }
        });
        image::DynamicImage::ImageRgba8(rgba)
            .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();
        let im = Image::decode_png(&buf).unwrap();
        assert_eq!(im.get(0, 0), 0xFFFF_0000);
        assert_eq!(im.get(1, 0), 0xFF00_00FF);
        assert_eq!(&im.to_rgba8()[..4], &[255, 0, 0, 255]);
        let m = generate_mip_levels("block/x", im, 1, MipmapStrategy::Mean, 0.0);
        assert_eq!(m.len(), 2);
        let p = m[1].get(0, 0);
        assert_eq!(alpha(p), 255);
        // линейное среднее красного и синего (2 из 4 пикселей) ярче sRGB-среднего (127/128)
        assert!(red(p) > 170 && red(p) == blue(p), "{p:08x}");
        assert_eq!(green(p), 0);
        assert!(Image::decode_png(b"nope").is_err());
    }

    #[test]
    fn solidify_fills_transparent_with_neighbor() {
        let mut im = Image::new(3, 1);
        im.set(0, 0, 0xFF11_2233);
        solidify(&mut im);
        assert_eq!(im.get(1, 0), 0x0011_2233);
        assert_eq!(im.get(2, 0), 0x0011_2233);
        assert_eq!(im.get(0, 0), 0xFF11_2233);
    }

    #[test]
    fn cutout_keeps_coverage() {
        let mut im = Image::new(8, 8);
        for y in 0..8 {
            for x in 0..4 {
                im.set(x, y, 0xFF00_FF00);
            }
        }
        let m = generate_mip_levels("block/leaf", im, 2, MipmapStrategy::Auto, 0.0);
        assert_eq!(m.len(), 3);
        assert_eq!((m[1].width, m[2].width), (4, 2));
        // левая половина остаётся непрозрачной, правая — прозрачной
        assert!(alpha(m[1].get(0, 0)) > 200 && alpha(m[1].get(3, 0)) < 60);
    }

    #[test]
    fn dark_fill_uses_darkest() {
        let mut im = Image::new(2, 1);
        im.set(0, 0, 0xFF64_6464);
        fill_empty_with_dark(&mut im);
        assert_eq!(im.get(1, 0), 0x004B_4B4B);
    }
}
