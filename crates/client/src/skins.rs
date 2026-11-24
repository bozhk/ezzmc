//! Загрузка скинов игроков по URL из свойства профиля `textures` (фоновый поток, без блокировки кадра).
//! VANILLA: SkinManager / TextureManager — скин качается асинхронно; пока не пришёл, рисуется Стив.

use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

/// Готовый скин: RGBA8 64×64.
type Rgba = Arc<Vec<u8>>;

struct Inner {
    tx: Mutex<Sender<String>>,
    rx: Mutex<Receiver<(String, Rgba)>>,
    asked: Mutex<HashSet<String>>,
    ready: Mutex<HashMap<String, Rgba>>,
}

/// Общий кэш скинов (потокобезопасный): им пользуются Tab и рендер игроков.
pub struct Skins(Inner);

/// Имя gui-текстуры скина.
pub fn tex_name(url: &str) -> String {
    format!("skin:{url}")
}

/// Скины приходят только с домена Mojang; чужие URL от сервера не качаем.
fn allowed(url: &str) -> bool {
    url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .and_then(|r| r.split('/').next())
        == Some("textures.minecraft.net")
}

pub fn global() -> &'static Skins {
    static G: OnceLock<Skins> = OnceLock::new();
    G.get_or_init(Skins::new)
}

impl Skins {
    fn new() -> Self {
        let (tx, wrx) = channel::<String>();
        let (wtx, rx) = channel::<(String, Rgba)>();
        std::thread::Builder::new()
            .name("skins".into())
            .spawn(move || {
                while let Ok(url) = wrx.recv() {
                    if let Some(rgba) = fetch(&url) {
                        let _ = wtx.send((url, Arc::new(rgba)));
                    }
                }
            })
            .ok();
        Self(Inner {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
            asked: Mutex::default(),
            ready: Mutex::default(),
        })
    }

    /// Запросить скин (повторные запросы игнорируются).
    pub fn request(&self, url: &str) {
        if !allowed(url) {
            return;
        }
        let fresh = self
            .0
            .asked
            .lock()
            .is_ok_and(|mut a| a.insert(url.to_owned()));
        if fresh {
            if let Ok(tx) = self.0.tx.lock() {
                let _ = tx.send(url.to_owned());
            }
        }
    }

    /// Скачанный скин (None — ещё не пришёл или не удался).
    pub fn get(&self, url: &str) -> Option<Rgba> {
        if let (Ok(rx), Ok(mut ready)) = (self.0.rx.lock(), self.0.ready.lock()) {
            ready.extend(rx.try_iter());
            return ready.get(url).cloned();
        }
        None
    }
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .ok()?;
    let mut data = Vec::new();
    resp.into_reader()
        .take(1 << 20)
        .read_to_end(&mut data)
        .ok()?;
    let img = ezzmc_assets::texture::Image::decode_png(&data).ok()?;
    if img.width != 64 || !(img.height == 64 || img.height == 32) {
        return None;
    }
    // Старый формат 64×32 дополняем прозрачным низом до 64×64: лицо и шапка остаются на местах.
    let mut rgba = img.to_rgba8();
    rgba.resize(64 * 64 * 4, 0);
    Some(rgba)
}

/// Стандартный base64 (с `=` и без).
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let (mut out, mut acc, mut bits) = (Vec::with_capacity(s.len() * 3 / 4), 0u32, 0);
    for c in s.trim_end_matches('=').bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_and_url() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        let j = r#"{"textures":{"SKIN":{"url":"http://textures.minecraft.net/texture/ab"}}}"#;
        let b = "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWIifX19";
        assert_eq!(base64_decode(b).unwrap(), j.as_bytes());
        assert_eq!(
            crate::tablist::skin_url_from_property(b),
            Some(("http://textures.minecraft.net/texture/ab".to_owned(), false))
        );
        assert!(allowed("http://textures.minecraft.net/texture/ab"));
        assert!(!allowed("http://evil.test/textures.minecraft.net"));
    }

    #[test]
    fn slim_flag() {
        let j = r#"{"textures":{"SKIN":{"url":"http://textures.minecraft.net/texture/cd","metadata":{"model":"slim"}}}}"#;
        let b = {
            const T: &[u8; 64] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut o = String::new();
            for c in j.as_bytes().chunks(3) {
                let n = c
                    .iter()
                    .enumerate()
                    .fold(0u32, |a, (i, &x)| a | u32::from(x) << (16 - 8 * i));
                for k in 0..=c.len() {
                    o.push(T[(n >> (18 - 6 * k) & 63) as usize] as char);
                }
            }
            o
        };
        assert_eq!(crate::tablist::skin_url_from_property(&b).unwrap().1, true);
    }
}
