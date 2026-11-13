//! Титры по центру экрана (`/title`): заголовок ×4, подзаголовок ×2, плавное появление/исчезание.
//! VANILLA: Hud#setTitle/setSubtitle/setTimes/clearTitles/resetTitleTimes/extractTitle/tick (26.3),
//! ClientPacketListener#setTitleText/setSubtitleText/setTitlesAnimation/handleTitlesClear.

use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_render::text::TextRenderer;

use crate::chat::Span;

/// Событие титров из сети.
#[derive(Debug, Clone)]
pub enum TitleEvt {
    Title(Component),
    Subtitle(Component),
    /// fadeIn, stay, fadeOut (тики; < 0 — не менять).
    Times(i32, i32, i32),
    /// `true` — ещё и сбросить времена к 10/70/20.
    Clear(bool),
}

/// Разбор `set_titles_animation` (три int) и `clear_titles` (bool). Текстовые пакеты — через Component.
pub fn parse_times(body: &[u8]) -> Option<TitleEvt> {
    let mut r = Reader::new(body);
    Some(TitleEvt::Times(r.i32().ok()?, r.i32().ok()?, r.i32().ok()?))
}

pub fn parse_clear(body: &[u8]) -> Option<TitleEvt> {
    Some(TitleEvt::Clear(Reader::new(body).bool().ok()?))
}

#[derive(Debug, Clone)]
pub struct Titles {
    time: i32,
    title: Option<Component>,
    subtitle: Option<Component>,
    fade_in: i32,
    stay: i32,
    fade_out: i32,
}

impl Default for Titles {
    fn default() -> Self {
        let mut t = Self {
            time: 0,
            title: None,
            subtitle: None,
            fade_in: 0,
            stay: 0,
            fade_out: 0,
        };
        t.reset_times();
        t
    }
}

impl Titles {
    /// VANILLA: Hud#resetTitleTimes — 10/70/20.
    pub fn reset_times(&mut self) {
        self.fade_in = 10;
        self.stay = 70;
        self.fade_out = 20;
    }

    pub fn apply(&mut self, e: TitleEvt) {
        match e {
            // VANILLA: Hud#setTitle — таймер заводится заново.
            TitleEvt::Title(c) => {
                self.title = Some(c);
                self.time = self.fade_in + self.stay + self.fade_out;
            }
            // VANILLA: Hud#setSubtitle — таймер не трогает (покажется со следующим title).
            TitleEvt::Subtitle(c) => self.subtitle = Some(c),
            TitleEvt::Times(i, s, o) => {
                // VANILLA: Hud#setTimes — отрицательные не меняют, идущий титр перезапускается.
                if i >= 0 {
                    self.fade_in = i;
                }
                if s >= 0 {
                    self.stay = s;
                }
                if o >= 0 {
                    self.fade_out = o;
                }
                if self.time > 0 {
                    self.time = self.fade_in + self.stay + self.fade_out;
                }
            }
            TitleEvt::Clear(reset) => {
                self.title = None;
                self.subtitle = None;
                self.time = 0;
                if reset {
                    self.reset_times();
                }
            }
        }
    }

    /// VANILLA: Hud#tick — по истечении title и subtitle сбрасываются.
    pub fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
            if self.time <= 0 {
                self.title = None;
                self.subtitle = None;
            }
        }
    }

    /// Активный титр (агенту/F3).
    pub fn current(&self) -> Option<(&Component, Option<&Component>, i32)> {
        self.title
            .as_ref()
            .filter(|_| self.time > 0)
            .map(|t| (t, self.subtitle.as_ref(), self.time))
    }

    /// Прозрачность 0..255 при доле тика `partial`. VANILLA: Hud#extractTitle.
    pub fn alpha(&self, partial: f32) -> i32 {
        let t = self.time as f32 - partial;
        let mut a = 255;
        if self.time > self.fade_out + self.stay {
            let tm = (self.fade_in + self.stay + self.fade_out) as f32 - t;
            a = (tm * 255.0 / self.fade_in as f32) as i32;
        }
        if self.time <= self.fade_out {
            a = (t * 255.0 / self.fade_out as f32) as i32;
        }
        a.clamp(0, 255)
    }

    /// VANILLA: Hud#extractTitle — центр экрана; title ×4 с y=−10, subtitle ×2 с y=5, тень,
    /// подложка только при backgroundForChatOnly=false (не поддерживается — по умолчанию нет).
    pub fn draw(
        &self,
        t: &mut TextRenderer,
        lang: &crate::ui::lang::Lang,
        gw: f32,
        gh: f32,
        partial: f32,
    ) {
        let Some((title, sub, _)) = self.current() else {
            return;
        };
        let alpha = self.alpha(partial);
        if alpha <= 0 {
            return;
        }
        let a = alpha as f32 / 255.0;
        let (cx, cy) = ((gw / 2.0).floor(), (gh / 2.0).floor());
        for (c, k, y) in [(Some(title), 4.0, -10.0), (sub, 2.0, 5.0)] {
            let Some(c) = c else { continue };
            let spans = crate::chat::flatten(c, lang, true);
            let w = spans_width(t, &spans).floor();
            t.set_transform(Some((cx, cy, 0.0, k)));
            draw_spans(t, &spans, -(w / 2.0).floor(), y, a, true);
            t.set_transform(None);
        }
    }
}

/// Ширина строки из спанов (жирный: +1 на символ).
pub fn spans_width(t: &TextRenderer, spans: &[Span]) -> f32 {
    spans.iter().map(|s| t.width_fmt(&s.text, s.bold)).sum()
}

/// Спаны с их цветами/стилями и общей прозрачностью. VANILLA: GuiGraphicsExtractor#text (dropShadow).
pub fn draw_spans(t: &mut TextRenderer, spans: &[Span], x: f32, y: f32, alpha: f32, shadow: bool) {
    let mut cx = x;
    for (i, s) in spans.iter().enumerate() {
        let f = ezzmc_render::text::Fmt {
            bold: s.bold,
            italic: s.italic,
            underline: s.underline,
            strike: s.strike,
            obfuscated: s.obf,
            line_start: i == 0,
        };
        cx = t.draw_fmt(&s.text, cx, y, s.color, alpha, f, shadow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn txt(s: &str) -> Component {
        Component::text(s)
    }

    #[test]
    fn timeline_vanilla() {
        let mut t = Titles::default();
        t.apply(TitleEvt::Subtitle(txt("b")));
        assert!(
            t.current().is_none(),
            "подзаголовок без title не показывается"
        );
        t.apply(TitleEvt::Title(txt("a")));
        assert_eq!(t.time, 100);
        // Начало появления: alpha 0, середина fade_in (t=95) — 127.
        assert_eq!(t.alpha(0.0), 0);
        for _ in 0..5 {
            t.tick();
        }
        assert_eq!(t.alpha(0.0), 127);
        for _ in 0..5 {
            t.tick();
        }
        assert_eq!(t.alpha(0.0), 255);
        // Конец: последние 20 тиков — затухание.
        for _ in 0..80 {
            t.tick();
        }
        assert_eq!(t.time, 10);
        assert_eq!(t.alpha(0.0), 127);
        for _ in 0..10 {
            t.tick();
        }
        assert!(t.current().is_none());
        assert!(t.subtitle.is_none());
    }

    #[test]
    fn times_and_clear() {
        let mut t = Titles::default();
        t.apply(TitleEvt::Times(5, -1, 7));
        assert_eq!((t.fade_in, t.stay, t.fade_out), (5, 70, 7));
        assert_eq!(t.time, 0, "без активного титра таймер не заводится");
        t.apply(TitleEvt::Title(txt("a")));
        assert_eq!(t.time, 82);
        t.apply(TitleEvt::Clear(false));
        assert!(t.current().is_none());
        assert_eq!(t.fade_in, 5);
        t.apply(TitleEvt::Clear(true));
        assert_eq!((t.fade_in, t.stay, t.fade_out), (10, 70, 20));
        assert!(matches!(
            parse_times(&[0, 0, 0, 1, 0, 0, 0, 2, 255, 255, 255, 255]),
            Some(TitleEvt::Times(1, 2, -1))
        ));
        assert!(matches!(parse_clear(&[1]), Some(TitleEvt::Clear(true))));
    }
}
