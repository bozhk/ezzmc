//! Список игроков (Tab): player_info_update/remove, tab_list (header/footer) и отрисовка.
//! VANILLA: ClientboundPlayerInfoUpdatePacket, ClientboundPlayerInfoRemovePacket,
//! ClientboundTabListPacket, PlayerTabOverlay (без счёта/сердец scoreboard, команд и голов — см. STATUS).
use crate::chat;
use crate::ui::lang::Lang;
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::text::TextRenderer;
use std::collections::HashMap;

pub const SPRITES: &[(&str, &str)] = &[
    ("tab_ping_unknown", "sprites/icon/ping_unknown"),
    ("tab_ping_1", "sprites/icon/ping_1"),
    ("tab_ping_2", "sprites/icon/ping_2"),
    ("tab_ping_3", "sprites/icon/ping_3"),
    ("tab_ping_4", "sprites/icon/ping_4"),
    ("tab_ping_5", "sprites/icon/ping_5"),
];

/// VANILLA: GameType.SPECTATOR.id
const SPECTATOR: i32 = 3;

#[derive(Clone, Debug, Default)]
pub struct Entry {
    pub name: String,
    pub display: Option<Component>,
    pub latency: i32,
    pub game_mode: i32,
    pub listed: bool,
    pub order: i32,
}

/// Изменения одной записи из player_info_update (None — действие не пришло).
#[derive(Clone, Debug, Default)]
pub struct Delta {
    pub name: Option<String>,
    pub game_mode: Option<i32>,
    pub listed: Option<bool>,
    pub latency: Option<i32>,
    pub display: Option<Option<Component>>,
    pub order: Option<i32>,
}

#[derive(Clone, Debug)]
pub enum TabEvt {
    Update(Vec<(u128, Delta)>),
    Remove(Vec<u128>),
    HeaderFooter(Box<(Option<Component>, Option<Component>)>),
}

#[derive(Default)]
pub struct TabList {
    pub entries: HashMap<u128, Entry>,
    /// Имена профилей, пережившие remove (VANILLA: сущность игрока держит GameProfile со спавна).
    pub profiles: HashMap<u128, String>,
    pub header: Option<Component>,
    pub footer: Option<Component>,
}

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

/// VANILLA: ClientboundPlayerInfoUpdatePacket.Action (порядок битов = ordinal).
pub fn parse_update(body: &[u8]) -> ProtocolResult<Vec<(u128, Delta)>> {
    let mut r = Reader::new(body);
    let actions = r.u8()?; // EnumSet из 8 значений = 1 байт
    let n = r.length("player_info entries", 1024)?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let id = r.uuid()?;
        let mut d = Delta::default();
        for bit in 0..8 {
            if actions & (1 << bit) == 0 {
                continue;
            }
            match bit {
                0 => {
                    d.name = Some(r.string(16)?);
                    let props = r.length("profile properties", 16)?;
                    for _ in 0..props {
                        r.string(64)?;
                        r.string(32767)?;
                        r.optional(|r| r.string(1024))?;
                    }
                }
                1 => {
                    if r.bool()? {
                        r.uuid()?; // sessionId
                        r.i64()?; // expiresAt
                        let k = r.length("chat key", 512)?;
                        r.bytes(k)?;
                        let s = r.length("chat key signature", 4096)?;
                        r.bytes(s)?;
                    }
                }
                2 => d.game_mode = Some(r.var_int()?),
                3 => d.listed = Some(r.bool()?),
                4 => d.latency = Some(r.var_int()?),
                5 => {
                    d.display = Some(if r.bool()? {
                        Some(component(&mut r)?)
                    } else {
                        None
                    })
                }
                6 => d.order = Some(r.var_int()?),
                _ => {
                    r.bool()?; // UPDATE_HAT
                }
            }
        }
        out.push((id, d));
    }
    Ok(out)
}

pub fn parse_remove(body: &[u8]) -> ProtocolResult<Vec<u128>> {
    let mut r = Reader::new(body);
    let n = r.length("player_info_remove", 1024)?;
    (0..n).map(|_| r.uuid()).collect()
}

/// VANILLA: ClientPacketListener#handleTabListCustomisation — пустой текст = null.
pub fn parse_header_footer(body: &[u8]) -> ProtocolResult<(Option<Component>, Option<Component>)> {
    let mut r = Reader::new(body);
    let (h, f) = (component(&mut r)?, component(&mut r)?);
    let keep = |c: Component| {
        let empty = chat::flatten(&c, &Lang::default(), false)
            .iter()
            .all(|s| s.text.is_empty());
        (!empty).then_some(c)
    };
    Ok((keep(h), keep(f)))
}

impl TabList {
    pub fn apply(&mut self, e: TabEvt) {
        match e {
            TabEvt::Update(v) => {
                for (id, d) in v {
                    // VANILLA: ADD_PLAYER создаёт запись; остальные действия без неё игнорируются.
                    if let Some(name) = d.name {
                        self.profiles.insert(id, name.clone());
                        self.entries.entry(id).or_default().name = name;
                    }
                    let Some(en) = self.entries.get_mut(&id) else {
                        continue;
                    };
                    if let Some(x) = d.game_mode {
                        en.game_mode = x;
                    }
                    if let Some(x) = d.listed {
                        en.listed = x;
                    }
                    if let Some(x) = d.latency {
                        en.latency = x;
                    }
                    if let Some(x) = d.display {
                        en.display = x;
                    }
                    if let Some(x) = d.order {
                        en.order = x;
                    }
                }
            }
            TabEvt::Remove(v) => {
                for id in v {
                    self.entries.remove(&id);
                }
            }
            TabEvt::HeaderFooter(hf) => {
                (self.header, self.footer) = *hf;
            }
        }
    }

    /// VANILLA: PlayerTabOverlay#getPlayerInfos — listed, порядок, наблюдатели в конец, имя без регистра, ≤80.
    fn sorted(&self) -> Vec<&Entry> {
        let mut v: Vec<&Entry> = self.entries.values().filter(|e| e.listed).collect();
        v.sort_by(|a, b| {
            (-a.order, i32::from(a.game_mode == SPECTATOR))
                .cmp(&(-b.order, i32::from(b.game_mode == SPECTATOR)))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        v.truncate(80);
        v
    }

    /// VANILLA: PlayerTabOverlay#extractRenderState. Возвращает значки связи (x, y, спрайт),
    /// их рисует вызывающий поверх (отдельный проход GuiPainter).
    pub fn draw(
        &self,
        t: &mut TextRenderer,
        lang: &Lang,
        gw: f32,
    ) -> Vec<(f32, f32, &'static str)> {
        let infos = self.sorted();
        let legacy =
            |c: &Component, base: u32| chat::spans_to_legacy(&chat::flatten(c, lang, true), base);
        let names: Vec<String> = infos
            .iter()
            .map(|e| {
                let s = e
                    .display
                    .as_ref()
                    .map_or_else(|| e.name.clone(), |c| legacy(c, 0xFFFFFF));
                if e.game_mode == SPECTATOR {
                    format!("§o{s}")
                } else {
                    s
                }
            })
            .collect();
        let max_name = names
            .iter()
            .map(|n| t.text_width(n))
            .fold(0.0_f32, f32::max)
            .ceil();
        let slots = infos.len();
        let (mut rows, mut cols) = (slots, 1);
        while rows > 20 {
            cols += 1;
            rows = slots.div_ceil(cols);
        }
        let slot_w = ((cols as f32 * (max_name + 13.0)).min(gw - 50.0) / cols as f32).floor();
        let cols_f = cols as f32;
        let xxo = (gw / 2.0 - (slot_w * cols_f + (cols_f - 1.0) * 5.0) / 2.0).floor();
        let mut yyo = 10.0;
        let mut max_line = slot_w * cols_f + (cols_f - 1.0) * 5.0;
        let lines = |c: &Option<Component>| -> Vec<String> {
            c.as_ref()
                .map(|c| legacy(c, 0xFFFFFF).split('\n').map(str::to_owned).collect())
                .unwrap_or_default()
        };
        let (hl, fl) = (lines(&self.header), lines(&self.footer));
        for l in hl.iter().chain(&fl) {
            max_line = max_line.max(t.text_width(l));
        }
        let half = (max_line / 2.0).floor();
        let (x0, x1) = (gw / 2.0 - half - 1.0, gw / 2.0 + half + 1.0);
        let center = |t: &mut TextRenderer, l: &str, y: f32| {
            let w = t.text_width(l);
            t.draw(l, (gw / 2.0 - w / 2.0).floor(), y, 0xFFFFFF, true);
        };
        if !hl.is_empty() {
            t.fill(x0, yyo - 1.0, x1, yyo + hl.len() as f32 * 9.0, 0x8000_0000);
            for l in &hl {
                center(t, l, yyo);
                yyo += 9.0;
            }
            yyo += 1.0;
        }
        t.fill(x0, yyo - 1.0, x1, yyo + rows as f32 * 9.0, 0x8000_0000);
        let mut icons = Vec::new();
        for (i, (e, name)) in infos.iter().zip(&names).enumerate() {
            let (col, row) = (i / rows, i % rows);
            let xo = xxo + (col as f32) * slot_w + (col as f32) * 5.0;
            let yo = yyo + row as f32 * 9.0;
            t.fill(xo, yo, xo + slot_w, yo + 8.0, 0x20FF_FFFF);
            let c = if e.game_mode == SPECTATOR {
                0xBFBFBF
            } else {
                0xFFFFFF
            };
            t.draw(name, xo, yo, c, true);
            let sprite = match e.latency {
                l if l < 0 => "tab_ping_unknown",
                l if l < 150 => "tab_ping_5",
                l if l < 300 => "tab_ping_4",
                l if l < 600 => "tab_ping_3",
                l if l < 1000 => "tab_ping_2",
                _ => "tab_ping_1",
            };
            icons.push((xo + slot_w - 11.0, yo, sprite));
        }
        if !fl.is_empty() {
            yyo += rows as f32 * 9.0 + 1.0;
            t.fill(x0, yyo - 1.0, x1, yyo + fl.len() as f32 * 9.0, 0x8000_0000);
            for l in &fl {
                center(t, l, yyo);
                yyo += 9.0;
            }
        }
        icons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_parses() {
        let mut w = ezzmc_protocol::wire::Writer::new();
        w.var_int(1).uuid(7);
        assert_eq!(parse_remove(&w.into_vec()).unwrap(), vec![7]);
    }

    #[test]
    fn update_add_listed_latency() {
        let mut w = ezzmc_protocol::wire::Writer::new();
        w.u8(1 | 8 | 16).var_int(1).uuid(5);
        w.string("Bob").var_int(0); // ADD_PLAYER
        w.bool(true); // listed
        w.var_int(42); // latency
        let v = parse_update(&w.into_vec()).unwrap();
        let mut t = TabList::default();
        t.apply(TabEvt::Update(v));
        let e = &t.entries[&5];
        assert_eq!((e.name.as_str(), e.listed, e.latency), ("Bob", true, 42));
        t.apply(TabEvt::Remove(vec![5]));
        assert!(t.entries.is_empty());
    }
}
