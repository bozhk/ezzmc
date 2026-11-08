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
