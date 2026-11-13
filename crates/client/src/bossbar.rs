//! Босс-бар: пакет boss_event и отрисовка полос сверху экрана.
//! VANILLA: ClientboundBossEventPacket, BossHealthOverlay, LerpingBossEvent.
//! Не портировано: darkenScreen / playBossMusic / createWorldFog (флаги разбираются и хранятся).
use crate::chat;
use crate::ui::lang::Lang;
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::text::TextRenderer;
use std::time::Instant;

/// Спрайты полос: цвет (pink, blue, red, green, yellow, purple, white) × (фон, прогресс).
pub const SPRITES: &[(&str, &str)] = &[
    ("boss_pink_bg", "sprites/boss_bar/pink_background"),
    ("boss_blue_bg", "sprites/boss_bar/blue_background"),
    ("boss_red_bg", "sprites/boss_bar/red_background"),
    ("boss_green_bg", "sprites/boss_bar/green_background"),
    ("boss_yellow_bg", "sprites/boss_bar/yellow_background"),
    ("boss_purple_bg", "sprites/boss_bar/purple_background"),
    ("boss_white_bg", "sprites/boss_bar/white_background"),
    ("boss_pink_fg", "sprites/boss_bar/pink_progress"),
    ("boss_blue_fg", "sprites/boss_bar/blue_progress"),
    ("boss_red_fg", "sprites/boss_bar/red_progress"),
    ("boss_green_fg", "sprites/boss_bar/green_progress"),
    ("boss_yellow_fg", "sprites/boss_bar/yellow_progress"),
    ("boss_purple_fg", "sprites/boss_bar/purple_progress"),
    ("boss_white_fg", "sprites/boss_bar/white_progress"),
    ("boss_n6_bg", "sprites/boss_bar/notched_6_background"),
    ("boss_n10_bg", "sprites/boss_bar/notched_10_background"),
    ("boss_n12_bg", "sprites/boss_bar/notched_12_background"),
    ("boss_n20_bg", "sprites/boss_bar/notched_20_background"),
    ("boss_n6_fg", "sprites/boss_bar/notched_6_progress"),
    ("boss_n10_fg", "sprites/boss_bar/notched_10_progress"),
    ("boss_n12_fg", "sprites/boss_bar/notched_12_progress"),
    ("boss_n20_fg", "sprites/boss_bar/notched_20_progress"),
];

const BG: [&str; 7] = [
    "boss_pink_bg",
    "boss_blue_bg",
    "boss_red_bg",
    "boss_green_bg",
    "boss_yellow_bg",
    "boss_purple_bg",
    "boss_white_bg",
];
const FG: [&str; 7] = [
    "boss_pink_fg",
    "boss_blue_fg",
    "boss_red_fg",
    "boss_green_fg",
    "boss_yellow_fg",
    "boss_purple_fg",
    "boss_white_fg",
];
const OV_BG: [&str; 4] = ["boss_n6_bg", "boss_n10_bg", "boss_n12_bg", "boss_n20_bg"];
const OV_FG: [&str; 4] = ["boss_n6_fg", "boss_n10_fg", "boss_n12_fg", "boss_n20_fg"];

/// VANILLA: ClientboundBossEventPacket.Operation.
#[derive(Debug)]
pub enum BossOp {
    Add {
        name: Component,
        progress: f32,
        color: i32,
        overlay: i32,
        flags: u8,
    },
    Remove,
    Progress(f32),
    Name(Component),
    Style(i32, i32),
    Properties(u8),
}

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

/// `boss_event`: (uuid, операция).
pub fn parse(body: &[u8]) -> ProtocolResult<(u128, BossOp)> {
    let mut r = Reader::new(body);
    let id = r.uuid()?;
    let op = match r.var_int()? {
        0 => BossOp::Add {
            name: component(&mut r)?,
            progress: r.f32()?,
            color: r.var_int()?,
            overlay: r.var_int()?,
            flags: r.u8()?,
        },
        1 => BossOp::Remove,
        2 => BossOp::Progress(r.f32()?),
        3 => BossOp::Name(component(&mut r)?),
        4 => BossOp::Style(r.var_int()?, r.var_int()?),
        _ => BossOp::Properties(r.u8()?),
    };
    Ok((id, op))
}

struct Event {
    id: u128,
    name: Component,
    /// LerpingBossEvent: прошлое/целевое значение и момент установки.
    progress: f32,
    target: f32,
    set_at: Instant,
    color: usize,
    /// 0 = PROGRESS, 1..=4 = NOTCHED_6/10/12/20.
    overlay: usize,
    #[allow(dead_code)]
    flags: u8,
}

impl Event {
    /// VANILLA: LerpingBossEvent#getProgress — линейно за 100 мс.
    fn progress(&self) -> f32 {
        let t = (self.set_at.elapsed().as_secs_f32() / 0.1).clamp(0.0, 1.0);
        self.progress + (self.target - self.progress) * t
    }
}

/// Полосы в порядке добавления (VANILLA: LinkedHashMap).
#[derive(Default)]
pub struct BossBars {
    events: Vec<Event>,
}

impl BossBars {
    pub fn apply(&mut self, id: u128, op: BossOp) {
        if let BossOp::Add {
            name,
            progress,
            color,
            overlay,
            flags,
        } = op
        {
            self.events.retain(|e| e.id != id);
            self.events.push(Event {
                id,
                name,
                progress,
                target: progress,
                set_at: Instant::now(),
                color: (color.max(0) as usize).min(6),
                overlay: (overlay.max(0) as usize).min(4),
                flags,
            });
            return;
        }
        if matches!(op, BossOp::Remove) {
            self.events.retain(|e| e.id != id);
            return;
        }
        let Some(e) = self.events.iter_mut().find(|e| e.id == id) else {
            return;
        };
        match op {
            BossOp::Progress(p) => {
                e.progress = e.progress();
                e.target = p;
                e.set_at = Instant::now();
            }
            BossOp::Name(n) => e.name = n,
            BossOp::Style(c, o) => {
                e.color = (c.max(0) as usize).min(6);
                e.overlay = (o.max(0) as usize).min(4);
            }
            BossOp::Properties(f) => e.flags = f,
            BossOp::Add { .. } | BossOp::Remove => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// VANILLA: BossHealthOverlay#extractRenderState. Текст рисуется сразу, возвращает
    /// спрайты (имя, x, y, ширина в пикселях) для отдельного прохода GuiPainter.
    pub fn draw(
        &self,
        t: &mut TextRenderer,
        lang: &Lang,
        gw: f32,
        gh: f32,
    ) -> Vec<(&'static str, f32, f32, f32)> {
        let mut sprites = Vec::new();
        let mut y = 12.0_f32;
        for e in &self.events {
            let x = (gw / 2.0).floor() - 91.0;
            let width = (e.progress().clamp(0.0, 1.0) * 182.0).round();
            sprites.push((BG[e.color], x, y, 182.0));
            if e.overlay > 0 {
                sprites.push((OV_BG[e.overlay - 1], x, y, 182.0));
            }
            if width > 0.0 {
                sprites.push((FG[e.color], x, y, width));
                if e.overlay > 0 {
                    sprites.push((OV_FG[e.overlay - 1], x, y, width));
                }
            }
            let msg = chat::spans_to_legacy(&chat::flatten(&e.name, lang, true), 0xFFFFFF);
            let w = t.text_width(&msg);
            t.draw(&msg, (gw / 2.0 - w / 2.0).floor(), y - 9.0, 0xFFFFFF, true);
            y += 19.0;
            if y >= gh / 3.0 {
                break;
            }
        }
        sprites
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_and_progress_parse() {
        let mut w = ezzmc_protocol::wire::Writer::new();
        w.uuid(9).var_int(2).f32(0.5);
        let (id, op) = parse(&w.into_vec()).unwrap();
        assert_eq!(id, 9);
        let mut b = BossBars::default();
        b.apply(
            9,
            BossOp::Add {
                name: Component::text("Wither"),
                progress: 1.0,
                color: 2,
                overlay: 1,
                flags: 0,
            },
        );
        b.apply(id, op);
        assert_eq!(b.events[0].target, 0.5);
        b.apply(9, BossOp::Remove);
        assert!(b.events.is_empty());
    }
}
