//! Субтитры звуков (опция showSubtitles). VANILLA: SubtitleOverlay (26.3).
//! Источник — `audio::SUB_QUEUE`: звуковой поток кладёт событие при старте звука с `subtitle`
//! в sounds.json (SoundEngine#notifyListeners).
use crate::audio;
use crate::ui::lang::Lang;
use ezzmc_render::text::TextRenderer;
use std::sync::atomic::Ordering;
use std::time::Instant;

struct Sub {
    /// Ключ перевода (`subtitles.*`).
    key: String,
    range: f32,
    /// VANILLA: SoundPlayedAt (location, time).
    played: Vec<([f64; 3], Instant)>,
    looping: bool,
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

impl Sub {
    /// VANILLA: Subtitle#getBestSubtitleCandidate — самый свежий в пределах слышимости
    /// (при равном времени — дальний).
    fn best(&self, pos: [f64; 3]) -> Option<([f64; 3], Instant)> {
        if self.played.len() <= 1 {
            return self.played.first().copied();
        }
        self.played
            .iter()
            .filter(|p| dist(p.0, pos) < f64::from(self.range))
            .max_by(|a, b| {
                a.1.cmp(&b.1)
                    .then(dist(a.0, pos).total_cmp(&dist(b.0, pos)))
            })
            .copied()
    }

    /// VANILLA: Subtitle#isAudibleFrom.
    fn audible(&self, cam: [f64; 3]) -> bool {
        if self.range.is_infinite() {
            return true;
        }
        self.best(cam)
            .is_some_and(|p| dist(cam, p.0) < f64::from(self.range))
    }

    /// VANILLA: Subtitle#getBrightness — 255 → 75 за 3 с (у зацикленных первые 0,5 с полная).
    fn brightness(&self, at: Instant, mult: f64) -> u32 {
        let full = if self.looping { 500.0 } else { 0.0 };
        let age = (at.elapsed().as_secs_f64() * 1000.0 - full).max(0.0);
        let k = (age / ((3000.0 - full) * mult)).clamp(0.0, 1.0);
        (255.0 + (75.0 - 255.0) * k).floor() as u32
    }
}

#[derive(Default)]
pub struct Subtitles {
    list: Vec<Sub>,
}

impl Subtitles {
    /// VANILLA: SubtitleOverlay#onPlaySound.
    fn on_play(&mut self, e: audio::SubEvent) {
        let now = Instant::now();
        if let Some(s) = self.list.iter_mut().find(|s| s.key == e.key) {
            // VANILLA: Subtitle#refresh.
            s.played.retain(|p| p.0 != e.pos);
            s.played.push((e.pos, now));
            return;
        }
        self.list.push(Sub {
            key: e.key,
            range: e.range,
            played: vec![(e.pos, now)],
            looping: e.looping,
        });
    }

    /// Слышимые сейчас строки: (текст, стрелка −1/0/+1, яркость 75..255). Для отрисовки и агента.
    /// `mult` — Options#notificationDisplayTime.
    pub fn visible(
        &mut self,
        enabled: bool,
        lang: &Lang,
        cam: [f64; 3],
        yaw: f32,
        pitch: f32,
        mult: f64,
    ) -> Vec<(String, i8, u32)> {
        // VANILLA: SubtitleOverlay#extractRenderState — слушатель включён, пока включена опция.
        audio::SUB_LISTEN.store(enabled, Ordering::Relaxed);
        if !enabled {
            self.list.clear();
            return Vec::new();
        }
        let queued = audio::SUB_QUEUE
            .lock()
            .map(|mut q| std::mem::take(&mut *q))
            .unwrap_or_default();
        for e in queued {
            self.on_play(e);
        }
        // VANILLA: ListenerTransform — forward/right камеры.
        let (ys, yc) = f64::from(yaw).to_radians().sin_cos();
        let (ps, pc) = f64::from(pitch).to_radians().sin_cos();
        let fwd = [-ys * pc, -ps, yc * pc];
        let right = [-yc, 0.0, -ys];
        let mut out = Vec::new();
        for s in &mut self.list {
            // VANILLA: Subtitle#purgeOldInstances(3000 · displayTimeMultiplier).
            s.played
                .retain(|p| p.1.elapsed().as_secs_f64() * 1000.0 <= 3000.0 * mult);
            if s.played.is_empty() || !s.audible(cam) {
                continue;
            }
            let Some((at, time)) = s.best(cam) else {
                continue;
            };
            let d = [at[0] - cam[0], at[1] - cam[1], at[2] - cam[2]];
            let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            // VANILLA: Vec3#normalize — нулевой вектор остаётся нулём.
            let n = if len < 1.0e-5 {
                [0.0; 3]
            } else {
                [d[0] / len, d[1] / len, d[2] / len]
            };
            let dot = |a: [f64; 3]| a[0] * n[0] + a[1] * n[1] + a[2] * n[2];
            let (rightness, forwardness) = (dot(right), dot(fwd));
            let arrow = if forwardness > 0.5 {
                0
            } else if rightness > 0.0 {
                1
            } else if rightness < 0.0 {
                -1
            } else {
                0
            };
            out.push((lang.tr(&s.key), arrow, s.brightness(time, mult)));
        }
        out
    }

    /// VANILLA: SubtitleOverlay#extractRenderState — плашки справа внизу, снизу вверх.
    /// `bg` — Options#getBackgroundOpacity(0.8).
    pub fn draw(t: &mut TextRenderer, rows: &[(String, i8, u32)], gw: f32, gh: f32, bg: f32) {
        if rows.is_empty() {
            return;
        }
        let w = |t: &TextRenderer, s: &str| t.text_width(s).floor();
        let mut width = rows.iter().map(|r| w(t, &r.0)).fold(0.0, f32::max);
        width += w(t, "<") + w(t, " ") + w(t, ">") + w(t, " ");
        let half = (width / 2.0).floor();
        let half_h = 4.0;
        let argb = (((bg.clamp(0.0, 1.0) * 255.0) as u32) << 24) & 0xFF00_0000;
        for (row, (text, arrow, b)) in rows.iter().enumerate() {
            let (cx, cy) = (gw - half - 2.0, gh - 35.0 - row as f32 * 10.0);
            t.fill(
                cx - half - 1.0,
                cy - half_h - 1.0,
                cx + half + 1.0,
                cy + half_h + 1.0,
                argb,
            );
            let color = b << 16 | b << 8 | b;
            match arrow {
                1 => t.draw(">", cx + half - w(t, ">"), cy - half_h, color, true),
                -1 => t.draw("<", cx - half, cy - half_h, color, true),
                _ => {}
            }
            let tw = w(t, text);
            t.draw(text, cx - (tw / 2.0).trunc(), cy - half_h, color, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_and_range() {
        let mut s = Subtitles::default();
        let ev = |pos| audio::SubEvent {
            key: "subtitles.block.generic.break".into(),
            range: 16.0,
            pos,
            looping: false,
        };
        s.on_play(ev([0.0, 0.0, 5.0]));
        s.on_play(ev([0.0, 0.0, 5.0]));
        s.on_play(ev([40.0, 0.0, 0.0]));
        assert_eq!(s.list.len(), 1);
        assert_eq!(s.list[0].played.len(), 2);
        // Ближний слышен, свежий дальний — вне дальности: выбирается ближний.
        assert!(s.list[0].audible([0.0; 3]));
        assert_eq!(s.list[0].best([0.0; 3]).map(|p| p.0), Some([0.0, 0.0, 5.0]));
        assert!(!s.list[0].audible([0.0, 100.0, 0.0]));
        assert!(s.list[0].brightness(Instant::now(), 1.0) >= 254);
    }
}
