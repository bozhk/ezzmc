//! Пинг серверов экрана мультиплеера (VANILLA: ServerStatusPinger + ServerData#State).
use super::lang::Lang;
use crossbeam_channel::{Receiver, Sender};
use ezzmc_net::status::{self, StatusError, StatusReply};
use ezzmc_protocol::text::Component;
use ezzmc_render::gui::GuiPainter;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PingState {
    Pinging,
    Successful,
    Incompatible,
    Unreachable,
}

pub struct Info {
    pub state: PingState,
    /// MOTD в формате `§`-кодов (базовый цвет серый 0x808080).
    pub motd: String,
    /// «online/max» (пусто, пока нет ответа).
    pub status: String,
    pub version: String,
    pub ping: u64,
    /// Строки подсказки со списком игроков.
    pub players: Vec<String>,
}

type Msg = (u32, String, Result<StatusReply, StatusError>);

pub struct Pinger {
    pub map: HashMap<String, Info>,
    generation: u32,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
}

impl Default for Pinger {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self {
            map: HashMap::new(),
            generation: 0,
            tx,
            rx,
        }
    }
}

/// Имя текстуры иконки сервера в `GuiPainter`.
pub fn icon_name(ip: &str) -> String {
    format!("srvicon:{ip}")
}

impl Pinger {
    /// Сбросить результаты (кнопка «Refresh»): пинг пойдёт заново.
    pub fn refresh(&mut self) {
        self.generation += 1;
        self.map.clear();
    }

    /// Запустить пинг, если для `ip` ещё не пинговали (VANILLA: state INITIAL → PINGING).
    pub fn ensure(&mut self, ip: &str, lang: &Lang) {
        if self.map.contains_key(ip) {
            return;
        }
        self.map.insert(
            ip.to_owned(),
            Info {
                state: PingState::Pinging,
                motd: lang.tr("multiplayer.status.pinging"),
                status: String::new(),
                version: String::new(),
                ping: 0,
                players: Vec::new(),
            },
        );
        let (tx, gen, ip) = (self.tx.clone(), self.generation, ip.to_owned());
        std::thread::spawn(move || {
            let r = status::ping_server(&ip, crate::netthread::PROTOCOL);
            let _ = tx.send((gen, ip, r));
        });
    }

    /// Забрать готовые ответы; иконки грузятся в текстуры.
    pub fn poll(
        &mut self,
        lang: &Lang,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        p: &mut GuiPainter,
    ) {
        while let Ok((gen, ip, r)) = self.rx.try_recv() {
            if gen != self.generation {
                continue;
            }
            let info = match r {
                Ok(reply) => parse(&reply, &ip, lang, dev, queue, p),
                Err(e) => Info {
                    state: PingState::Unreachable,
                    // VANILLA: CANT_RESOLVE/CANT_CONNECT_TEXT — красным (-65536).
                    motd: format!(
                        "§x§f§f§0§0§0§0{}",
                        lang.tr(match e {
                            StatusError::CantResolve => "multiplayer.status.cannot_resolve",
                            StatusError::CantConnect => "multiplayer.status.cannot_connect",
                        })
                    ),
                    status: String::new(),
                    version: String::new(),
                    ping: 0,
                    players: Vec::new(),
                },
            };
            self.map.insert(ip, info);
        }
    }
}

fn parse(
    r: &StatusReply,
    ip: &str,
    lang: &Lang,
    dev: &wgpu::Device,
    queue: &wgpu::Queue,
    p: &mut GuiPainter,
) -> Info {
    let v: Value = serde_json::from_str(&r.json).unwrap_or(Value::Null);
    let motd = match v.get("description") {
        // Простая строка: `§`-коды рисует сам шрифт (как в ванили).
        Some(Value::String(s)) => s.clone(),
        Some(d) => Component::from_json(d)
            .map(|c| {
                let spans = crate::chat::flatten_styled(&c, lang, 0x808080, false);
                crate::chat::spans_to_legacy(&spans, 0x808080)
            })
            .unwrap_or_default(),
        None => String::new(),
    };
    let version = v["version"]["name"].as_str().unwrap_or("").to_owned();
    let protocol = v["version"]["protocol"].as_i64().unwrap_or(-1);
    let (online, max) = (
        v["players"]["online"].as_i64().unwrap_or(0),
        v["players"]["max"].as_i64().unwrap_or(0),
    );
    let mut players = Vec::new();
    if let Some(s) = v["players"]["sample"].as_array() {
        for e in s {
            let n = e["name"].as_str().unwrap_or("");
            players.push(if n.is_empty() {
                lang.tr("multiplayer.status.anonymous_player")
            } else {
                n.to_owned()
            });
        }
        if online > s.len() as i64 {
            players.push(
                lang.tr("multiplayer.status.and_more")
                    .replace("%s", &(online - s.len() as i64).to_string()),
            );
        }
    }
    if let Some(fav) = v["favicon"].as_str() {
        // VANILLA: ServerData#getIconBytes + FaviconTexture — только PNG 64×64.
        let b64 = fav.strip_prefix("data:image/png;base64,").unwrap_or("");
        if let Some(img) = base64(b64)
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            .filter(|i| i.width == 64 && i.height == 64)
        {
            p.add_texture(dev, queue, &icon_name(ip), &img.to_rgba8(), 64, 64, false);
        }
    }
    let compatible = protocol == i64::from(crate::netthread::PROTOCOL);
    Info {
        state: if compatible {
            PingState::Successful
        } else {
            PingState::Incompatible
        },
        motd,
        // VANILLA: formatPlayerCount — число серым, максимум тёмно-серым.
        status: format!("§7{online}§r/§8{max}"),
        version,
        ping: r.ping_ms,
        players,
    }
}

fn base64(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let (mut acc, mut bits) = (0u32, 0);
    for c in s.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' => break,
            b'\n' | b'\r' | b' ' => continue,
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
    #[test]
    fn base64_decodes() {
        assert_eq!(super::base64("aGVsbG8="), Some(b"hello".to_vec()));
        assert_eq!(super::base64("aGk"), Some(b"hi".to_vec()));
    }
}
