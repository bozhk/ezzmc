//! Скриншоты (TODO-041): PNG в `screenshots/` с ванильным именем.
//! VANILLA: net.minecraft.client.Screenshot#getFile — `yyyy-MM-dd_HH.mm.ss[_N].png`, каталог `screenshots`.
#![allow(dead_code)]
use std::path::{Path, PathBuf};

pub const SCREENSHOT_DIR: &str = "screenshots";

/// VANILLA: Util#getFilenameFormattedDateTime.
pub fn timestamp_name(now: chrono::NaiveDateTime) -> String {
    now.format("%Y-%m-%d_%H.%M.%S").to_string()
}

/// VANILLA: Screenshot#getFile — первый свободный `<ts>.png`, `<ts>_2.png`, …
pub fn unique_path(dir: &Path, ts: &str) -> PathBuf {
    let mut n = 1u32;
    loop {
        let name = if n == 1 {
            format!("{ts}.png")
        } else {
            format!("{ts}_{n}.png")
        };
        let p = dir.join(name);
        if !p.exists() {
            return p;
        }
        n += 1;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScreenshotError {
    #[error("каталог скриншотов: {0}")]
    Io(#[from] std::io::Error),
    #[error("PNG: {0}")]
    Png(#[from] image::ImageError),
    #[error("неверный размер буфера")]
    BadBuffer,
}

/// Сохраняет RGB (альфа отбрасывается, как в ванили) в `<game_dir>/screenshots/`.
pub fn save(game_dir: &Path, shot: &ezzmc_render::Screenshot) -> Result<PathBuf, ScreenshotError> {
    let dir = game_dir.join(SCREENSHOT_DIR);
    std::fs::create_dir_all(&dir)?;
    let ts = timestamp_name(chrono::Local::now().naive_local());
    let path = unique_path(&dir, &ts);
    let rgb: Vec<u8> = shot
        .rgba
        .chunks_exact(4)
        .flat_map(|p| [p[0], p[1], p[2]])
        .collect();
    let img = image::RgbImage::from_raw(shot.width, shot.height, rgb)
        .ok_or(ScreenshotError::BadBuffer)?;
    img.save_with_format(&path, image::ImageFormat::Png)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("ezzmc-shot-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("mkdir");
        d
    }

    #[test]
    fn vanilla_timestamp_format() {
        let t = NaiveDate::from_ymd_opt(2026, 11, 7)
            .and_then(|d| d.and_hms_opt(9, 5, 3))
            .expect("date");
        assert_eq!(timestamp_name(t), "2026-11-07_09.05.03");
    }

    #[test]
    fn suffix_on_collision() {
        let d = tmp("uniq");
        assert_eq!(unique_path(&d, "t"), d.join("t.png"));
        std::fs::write(d.join("t.png"), b"").expect("w");
        assert_eq!(unique_path(&d, "t"), d.join("t_2.png"));
        std::fs::write(d.join("t_2.png"), b"").expect("w");
        assert_eq!(unique_path(&d, "t"), d.join("t_3.png"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn saves_valid_png() {
        let d = tmp("save");
        let shot = ezzmc_render::Screenshot {
            width: 2,
            height: 1,
            rgba: vec![255, 0, 0, 255, 0, 0, 255, 255],
        };
        let p = save(&d, &shot).expect("save");
        assert!(p.starts_with(d.join("screenshots")));
        let img = image::open(&p).expect("open").to_rgb8();
        assert_eq!(img.dimensions(), (2, 1));
        assert_eq!(img.get_pixel(1, 0).0, [0, 0, 255]);
        let _ = std::fs::remove_dir_all(&d);
    }
}

/// Для агента (не ваниль): быстрый разбор кадра по сетке 48×27 — доля доминирующего цвета, яркость,
/// разброс. Ловит «смотрит в камень/пустоту/чёрный кадр» до сверки с эталоном.
pub fn analyze(shot: &ezzmc_render::Screenshot) -> serde_json::Value {
    let (w, h) = (shot.width as usize, shot.height as usize);
    if w == 0 || h == 0 {
        return serde_json::json!({});
    }
    let (gx, gy) = (48usize, 27usize);
    let mut buckets: std::collections::HashMap<u16, u32> = std::collections::HashMap::new();
    let (mut sum, mut sum2, mut n) = (0.0f64, 0.0f64, 0.0f64);
    for j in 0..gy {
        for i in 0..gx {
            let (x, y) = ((i * w + w / 2) / gx, (j * h + h / 2) / gy);
            let k = (y.min(h - 1) * w + x.min(w - 1)) * 4;
            let Some(p) = shot.rgba.get(k..k + 3) else {
                continue;
            };
            let q =
                (u16::from(p[0] >> 5) << 6) | (u16::from(p[1] >> 5) << 3) | u16::from(p[2] >> 5);
            *buckets.entry(q).or_default() += 1;
            let l = 0.2126 * f64::from(p[0]) + 0.7152 * f64::from(p[1]) + 0.0722 * f64::from(p[2]);
            sum += l;
            sum2 += l * l;
            n += 1.0;
        }
    }
    let mean = sum / n.max(1.0);
    let std = (sum2 / n.max(1.0) - mean * mean).max(0.0).sqrt();
    let top = buckets.values().copied().max().unwrap_or(0);
    let top_frac = f64::from(top) / n.max(1.0);
    serde_json::json!({
        "mean_luma": mean.round(), "luma_std": std.round(),
        "dominant_color_fraction": (top_frac * 100.0).round() / 100.0,
        "distinct_colors": buckets.len(),
        "flat": top_frac > 0.55 || std < 12.0,
        "dark": mean < 25.0,
    })
}
