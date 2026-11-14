//! Всплывающие уведомления справа вверху. VANILLA: ToastManager, Toast, AdvancementToast (26.3).
//! Не портировано: RecipeToast (TODO-527), SystemToast, TutorialToast, NowPlayingToast.
use crate::advancements::{Display, Kind};
use crate::audio::{self, Cat};
use crate::chat;
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use std::time::Instant;

pub const SPRITES: &[(&str, &str)] = &[("toast_adv", "sprites/toast/advancement")];

/// VANILLA: ToastInstance.SLIDE_ANIMATION_DURATION_MS.
const SLIDE_MS: f32 = 600.0;
const SLOTS: usize = 5;

pub enum Toast {
    /// VANILLA: AdvancementToast.
    Advancement {
        title: String,
        kind: Kind,
        icon: ItemStack,
    },
}

impl Toast {
    pub fn advancement(d: &Display, lang: &Lang) -> Toast {
        Toast::Advancement {
            title: chat::spans_to_legacy(&chat::flatten(&d.title, lang, true), 0xFFFFFF),
            kind: d.kind,
            icon: d.icon.clone(),
        }
    }

    fn height(&self) -> f32 {
        match self {
            Toast::Advancement { .. } => 32.0,
        }
    }

    /// VANILLA: Toast#occcupiedSlotCount.
    fn slots(&self) -> usize {
        (self.height() / 32.0).ceil().max(1.0) as usize
    }

    /// Время показа, мс (×notificationDisplayTime = 1).
    fn display_ms(&self) -> f32 {
        5000.0
    }

    /// VANILLA: AdvancementToast#getSoundEvent.
    fn sound(&self) -> Option<&'static str> {
        match self {
            Toast::Advancement {
                kind: Kind::Challenge,
                ..
            } => Some("ui.toast.challenge_complete"),
            _ => None,
        }
    }
}

/// VANILLA: ToastManager.ToastInstance.
struct Instance {
    toast: Toast,
    first_slot: usize,
    slots: usize,
    anim_start: Option<Instant>,
    became_full: Instant,
    /// Toast.Visibility.SHOW.
    show: bool,
    full_for: f32,
    portion: f32,
    finished: bool,
}

impl Instance {
    /// VANILLA: ToastInstance#update. `true` — видимость сменилась (звук in/out).
    fn update(&mut self, now: Instant) -> bool {
        let before = self.show;
        let start = match self.anim_start {
            Some(s) => s,
            None => {
                self.anim_start = Some(now);
                self.show = true;
                now
            }
        };
        let since = now.duration_since(start).as_secs_f32() * 1000.0;
        if self.show && since <= SLIDE_MS {
            self.became_full = now;
        }
        self.full_for = now.duration_since(self.became_full).as_secs_f32() * 1000.0;
        // calculateVisiblePortion
        let p = (since / SLIDE_MS).clamp(0.0, 1.0);
        let p = p * p;
        self.portion = if self.show { p } else { 1.0 - p };
        let wanted = self.full_for < self.toast.display_ms();
        let mut start = start;
        if wanted != self.show {
            let back = std::time::Duration::from_millis(((1.0 - self.portion) * SLIDE_MS) as u64);
            start = now.checked_sub(back).unwrap_or(now);
            self.anim_start = Some(start);
            self.show = wanted;
        }
        self.finished = !self.show && now.duration_since(start).as_secs_f32() * 1000.0 > SLIDE_MS;
        self.show != before
    }
}

#[derive(Default)]
pub struct ToastManager {
    visible: Vec<Instance>,
    occupied: [bool; SLOTS],
    queued: std::collections::VecDeque<Toast>,
}

impl ToastManager {
    pub fn add(&mut self, t: Toast) {
        self.queued.push_back(t);
    }

    pub fn clear(&mut self) {
        self.visible.clear();
        self.occupied = [false; SLOTS];
        self.queued.clear();
    }

    /// Для агента: (вид, заголовок, видимая доля 0..1).
    pub fn snapshot(&self) -> Vec<(&'static str, String, f32)> {
        self.visible
            .iter()
            .map(|i| match &i.toast {
                Toast::Advancement { title, .. } => ("advancement", title.clone(), i.portion),
            })
            .collect()
    }

    pub fn queued(&self) -> usize {
        self.queued.len()
    }

    /// VANILLA: ToastManager#findFreeSlotsIndex.
    fn find_free(&self, need: usize) -> Option<usize> {
        if self.occupied.iter().filter(|o| !**o).count() < need {
            return None;
        }
        let mut run = 0;
        for i in 0..SLOTS {
            if self.occupied[i] {
                run = 0;
            } else {
                run += 1;
                if run == need {
                    return Some(i + 1 - run);
                }
            }
        }
        None
    }

    /// VANILLA: ToastManager#update (раз в кадр).
    pub fn update(&mut self) {
        let now = Instant::now();
        let mut sound_played = false;
        let mut i = 0;
        while i < self.visible.len() {
            let inst = &mut self.visible[i];
            if inst.update(now) && !sound_played {
                sound_played = true;
                let ev = if inst.show {
                    "ui.toast.in"
                } else {
                    "ui.toast.out"
                };
                audio::play_ui(ev, Cat::Ui, 1.0, 1.0);
            }
            if inst.finished {
                for s in inst.first_slot..inst.first_slot + inst.slots {
                    self.occupied[s] = false;
                }
                self.visible.remove(i);
            } else {
                i += 1;
            }
        }
        if !self.queued.is_empty() && self.occupied.iter().any(|o| !*o) {
            let mut played: Vec<&'static str> = Vec::new();
            let mut rest = std::collections::VecDeque::new();
            while let Some(t) = self.queued.pop_front() {
                let need = t.slots();
                let Some(first) = self.find_free(need) else {
                    rest.push_back(t);
                    continue;
                };
                for s in first..first + need {
                    self.occupied[s] = true;
                }
                if let Some(ev) = t.sound() {
                    if !played.contains(&ev) {
                        played.push(ev);
                        audio::play_ui(ev, Cat::Ui, 1.0, 1.0);
                    }
                }
                self.visible.push(Instance {
                    toast: t,
                    first_slot: first,
                    slots: need,
                    anim_start: None,
                    became_full: now,
                    show: false,
                    full_for: 0.0,
                    portion: 0.0,
                    finished: false,
                });
            }
            self.queued = rest;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.visible.is_empty()
    }

    /// VANILLA: ToastManager#extractRenderState. Три прохода: фон (спрайты), текст, иконки.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        dev: &wgpu::Device,
        q: &wgpu::Queue,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        rm: &ResourceManager,
        lang: &Lang,
    ) {
        if self.visible.is_empty() {
            return;
        }
        let gw = (w as f32 / t.scale() as f32).floor();
        // VANILLA: Toast#xPos / yPos.
        let origin = |i: &Instance| {
            (
                gw - 160.0 * i.portion,
                i.first_slot as f32 * i.toast.height(),
            )
        };
        p.begin(t.scale());
        for i in self.visible.iter().filter(|i| !i.finished) {
            let (x, y) = origin(i);
            match &i.toast {
                Toast::Advancement { .. } => p.blit(
                    "toast_adv",
                    x,
                    y,
                    160.0,
                    32.0,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFF_FFFF,
                ),
            }
        }
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        for i in self.visible.iter().filter(|i| !i.finished) {
            let (x, y) = origin(i);
            match &i.toast {
                Toast::Advancement { title, kind, .. } => {
                    let lines = crate::ui::wrap_n(t, title, 125.0, 16);
                    // VANILLA: -30465 (0xFF88FF) у испытания, иначе -256 (жёлтый).
                    let title_color = if *kind == Kind::Challenge {
                        0xFF88FF
                    } else {
                        0xFFFF00
                    };
                    let name = lang.tr(kind.toast_key());
                    if lines.len() <= 1 {
                        t.draw(&name, x + 30.0, y + 7.0, title_color, false);
                        if let Some(l) = lines.first() {
                            t.draw(l, x + 30.0, y + 18.0, 0xFFFFFF, false);
                        }
                    } else if i.full_for < 1500.0 {
                        let a = ((1500.0 - i.full_for) / 300.0).clamp(0.0, 1.0);
                        let a = (a * 255.0).floor() / 255.0;
                        t.draw_alpha(&name, x + 30.0, y + 11.0, title_color, a);
                    } else {
                        let a = ((i.full_for - 1500.0) / 300.0).clamp(0.0, 1.0);
                        let a = (a * 252.0).floor() / 255.0;
                        let mut ly = y + 16.0 - (lines.len() as f32 * 9.0 / 2.0).floor();
                        for l in &lines {
                            t.draw_alpha(l, x + 30.0, ly, 0xFFFFFF, a);
                            ly += 9.0;
                        }
                    }
                }
            }
        }
        t.flush(dev, q, view, w, h);
        p.begin(t.scale());
        for i in self.visible.iter().filter(|i| !i.finished) {
            let (x, y) = origin(i);
            let Toast::Advancement { icon, .. } = &i.toast;
            // VANILLA: graphics.fakeItem(iconItem, 8, 8).
            crate::hud::stack_icon(p, dev, q, rm, icon, x + 8.0, y + 8.0);
        }
        p.flush(dev, q, view, w, h, None);
    }
}

/// Кадр уведомлений: новые достижения → очередь, обновление и отрисовка (если `show`).
#[allow(clippy::too_many_arguments)]
pub fn frame(
    v: &mut crate::worldview::WorldView,
    show: bool,
    t: &mut TextRenderer,
    p: &mut GuiPainter,
    g: &ezzmc_render::Gpu,
    view: &wgpu::TextureView,
    size: (u32, u32),
    rm: &ResourceManager,
    lang: &Lang,
) {
    for id in std::mem::take(&mut v.toast_in) {
        if let Some(d) = v.advancements.get(&id).and_then(|a| a.display.as_ref()) {
            v.toasts.add(Toast::advancement(d, lang));
        }
    }
    v.toasts.update();
    if show {
        v.toasts
            .draw(t, p, &g.device, &g.queue, view, size, rm, lang);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adv() -> Toast {
        Toast::Advancement {
            title: "t".into(),
            kind: Kind::Task,
            icon: ItemStack {
                item: 1,
                count: 1,
                patch: Default::default(),
            },
        }
    }

    #[test]
    fn slots_and_queue() {
        let mut m = ToastManager::default();
        for _ in 0..6 {
            m.add(adv());
        }
        m.update();
        // 5 слотов по 32 px: шестой ждёт в очереди.
        assert_eq!(m.visible.len(), 5);
        assert_eq!(m.queued(), 1);
        assert_eq!(m.visible[4].first_slot, 4);
        assert_eq!(m.find_free(1), None);
        m.occupied[2] = false;
        assert_eq!(m.find_free(1), Some(2));
        assert_eq!(m.find_free(2), None);
    }
}
