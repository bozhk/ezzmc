//! Всплывающие уведомления справа вверху. VANILLA: ToastManager, Toast, AdvancementToast (26.3).
//! Не портировано: SystemToast, TutorialToast. NowPlayingToast — `NowPlaying`.
use crate::advancements::{Display, Kind};
use crate::audio::{self, Cat};
use crate::chat;
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use std::time::Instant;

pub const SPRITES: &[(&str, &str)] = &[
    ("toast_adv", "sprites/toast/advancement"),
    ("toast_recipe", "sprites/toast/recipe"),
    ("toast_np", "sprites/toast/now_playing"),
    ("music_notes", "sprites/icon/music_notes"),
];

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
    /// VANILLA: RecipeToast — одно на все новые рецепты, дополняется (addOrUpdate).
    Recipe {
        items: Vec<(Option<ItemStack>, Option<ItemStack>)>,
        changed: bool,
        last_changed: f32,
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
            Toast::Advancement { .. } | Toast::Recipe { .. } => 32.0,
        }
    }

    /// VANILLA: Toast#occcupiedSlotCount.
    fn slots(&self) -> usize {
        (self.height() / 32.0).ceil().max(1.0) as usize
    }

    /// VANILLA: Toast#update + getWantedVisibility (SHOW?), время показа 5000 мс
    /// (×notificationDisplayTime = 1).
    fn wanted(&mut self, full_for: f32) -> bool {
        let t = 5000.0 * display_mult();
        match self {
            Toast::Advancement { .. } => full_for < t,
            Toast::Recipe {
                items,
                changed,
                last_changed,
            } => {
                if *changed {
                    *last_changed = full_for;
                    *changed = false;
                }
                !items.is_empty() && full_for - *last_changed < t
            }
        }
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
        let wanted = self.toast.wanted(self.full_for);
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

    /// VANILLA: RecipeToast#addOrUpdate.
    pub fn add_recipe(&mut self, station: Option<ItemStack>, result: Option<ItemStack>) {
        let found = self
            .visible
            .iter_mut()
            .map(|i| &mut i.toast)
            .chain(self.queued.iter_mut())
            .find(|t| matches!(t, Toast::Recipe { .. }));
        if let Some(Toast::Recipe { items, changed, .. }) = found {
            items.push((station, result));
            *changed = true;
        } else {
            self.queued.push_back(Toast::Recipe {
                items: vec![(station, result)],
                changed: true,
                last_changed: 0.0,
            });
        }
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
                Toast::Recipe { items, .. } => ("recipe", format!("{}", items.len()), i.portion),
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
                Toast::Advancement { .. } | Toast::Recipe { .. } => {
                    let sprite = if matches!(i.toast, Toast::Recipe { .. }) {
                        "toast_recipe"
                    } else {
                        "toast_adv"
                    };
                    p.blit(sprite, x, y, 160.0, 32.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF)
                }
            }
        }
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        for i in self.visible.iter().filter(|i| !i.finished) {
            let (x, y) = origin(i);
            match &i.toast {
                Toast::Recipe { .. } => {
                    // VANILLA: RecipeToast#extractRenderState — -11534256 / чёрный, без тени.
                    t.draw(
                        &lang.tr("recipe.toast.title"),
                        x + 30.0,
                        y + 7.0,
                        0x500050,
                        false,
                    );
                    t.draw(
                        &lang.tr("recipe.toast.description"),
                        x + 30.0,
                        y + 18.0,
                        0x000000,
                        false,
                    );
                }
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
            match &i.toast {
                // VANILLA: graphics.fakeItem(iconItem, 8, 8).
                Toast::Advancement { icon, .. } => {
                    crate::hud::stack_icon(p, dev, q, rm, icon, x + 8.0, y + 8.0)
                }
                Toast::Recipe { items, .. } => {
                    let n = items.len().max(1);
                    let idx = (i.full_for / (5000.0 / n as f32).max(1.0)) as usize % n;
                    if let Some((_, Some(res))) = items.get(idx) {
                        crate::hud::stack_icon(p, dev, q, rm, res, x + 8.0, y + 8.0);
                    }
                }
            }
        }
        p.flush(dev, q, view, w, h, None);
        // RecipeToast: значок станции в масштабе 0.6 в точке (3, 3).
        p.begin_f(t.scale() as f32 * 0.6);
        let mut any = false;
        for i in self.visible.iter().filter(|i| !i.finished) {
            if let Toast::Recipe { items, .. } = &i.toast {
                let (x, y) = origin(i);
                let n = items.len().max(1);
                let idx = (i.full_for / (5000.0 / n as f32).max(1.0)) as usize % n;
                if let Some((Some(st), _)) = items.get(idx) {
                    crate::hud::stack_icon(p, dev, q, rm, st, (x + 1.8) / 0.6, (y + 1.8) / 0.6);
                    any = true;
                }
            }
        }
        if any {
            p.flush(dev, q, view, w, h, None);
        }
    }
}

/// VANILLA: ColorLerper.MUSIC_NOTE_COLORS — textureDiffuseColor DyeColor (WHITE — особый случай).
const NOTE_DIFFUSE: [u32; 12] = [
    0xF9FFFE, // WHITE (не используется: -1644826)
    0x9D9D97, // LIGHT_GRAY
    0x3AB3DA, // LIGHT_BLUE
    0x3C44AA, // BLUE
    0x169C9C, // CYAN
    0x5E7C16, // GREEN
    0x80C71F, // LIME
    0xFED83D, // YELLOW
    0xF9801D, // ORANGE
    0xF38BAA, // PINK
    0xB02E26, // RED
    0xC74EBD, // MAGENTA
];

/// VANILLA: ColorLerper.Type.MUSIC_NOTE (colorDuration 30, brightness 1.25).
fn note_color(tick: i32) -> u32 {
    let modified = |i: usize| -> u32 {
        if i == 0 {
            return 0xFFE6_E6E6;
        }
        let c = NOTE_DIFFUSE[i];
        let f = |v: u32| ((v as f32 * 1.25).floor() as i32).clamp(0, 255) as u32;
        0xFF00_0000 | (f(c >> 16 & 255) << 16) | (f(c >> 8 & 255) << 8) | f(c & 255)
    };
    let value = (tick / 30) as usize;
    let (c1, c2) = (value % 12, (value + 1) % 12);
    let sub = (tick % 30) as f32 / 30.0;
    let (a, b) = (modified(c1), modified(c2));
    let ch = |sh: u32| {
        let (x, y) = ((a >> sh & 255) as i32, (b >> sh & 255) as i32);
        (x + (sub * (y - x) as f32).floor() as i32) as u32
    };
    0xFF00_0000 | (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

/// VANILLA: NowPlayingToast + его ToastInstance (анимация выезда 600 мс, показ 5 с).
pub struct NowPlaying {
    /// updateToast: wanted пересчитывается, пока тост «жив».
    active: bool,
    wanted: bool,
    show: bool,
    anim_start: Option<Instant>,
    became_full: Instant,
    full_for: f32,
    portion: f32,
    finished: bool,
    note_tick: i32,
    note_last: Instant,
    note_color: u32,
}

impl Default for NowPlaying {
    fn default() -> Self {
        let now = Instant::now();
        NowPlaying {
            active: false,
            wanted: false,
            show: false,
            anim_start: None,
            became_full: now,
            full_for: 0.0,
            portion: 0.0,
            finished: true,
            note_tick: 0,
            note_last: now,
            note_color: 0xFFFF_FFFF,
        }
    }
}

impl NowPlaying {
    /// VANILLA: ToastManager#showNowPlayingToast (resetToast + showToast).
    pub fn show_toast(&mut self) {
        self.anim_start = None;
        self.finished = false;
        self.active = true;
        self.wanted = true;
    }

    /// VANILLA: ToastManager#hideNowPlayingToast.
    pub fn hide(&mut self) {
        self.wanted = false;
    }

    /// VANILLA: NowPlayingToast#tickMusicNotes (раз в 25 мс меняет тик цвета).
    pub fn tick_notes(&mut self, playing: bool) {
        if !playing {
            return;
        }
        let now = Instant::now();
        if now.duration_since(self.note_last).as_millis() > 25 {
            self.note_tick += 1;
            self.note_last = now;
            self.note_color = note_color(self.note_tick);
        }
    }

    /// VANILLA: ToastInstance#update + NowPlayingToast#update (`mult` = notificationDisplayTime).
    pub fn update(&mut self, playing: bool, mult: f32) {
        if self.finished {
            return;
        }
        let now = Instant::now();
        let start = *self.anim_start.get_or_insert_with(|| {
            self.show = true;
            now
        });
        let since = now.duration_since(start).as_secs_f32() * 1000.0;
        if self.show && since <= SLIDE_MS {
            self.became_full = now;
        }
        self.full_for = now.duration_since(self.became_full).as_secs_f32() * 1000.0;
        let p = (since / SLIDE_MS).clamp(0.0, 1.0);
        let p = p * p;
        self.portion = if self.show { p } else { 1.0 - p };
        if self.active {
            self.wanted = self.full_for < 5000.0 * mult;
            self.tick_notes(playing);
        }
        let mut since = since;
        if self.wanted != self.show {
            let back = std::time::Duration::from_millis(((1.0 - self.portion) * SLIDE_MS) as u64);
            self.anim_start = Some(now.checked_sub(back).unwrap_or(now));
            since = back.as_secs_f32() * 1000.0;
            self.show = self.wanted;
        }
        if !self.show && since > SLIDE_MS {
            self.finished = true;
            self.active = false;
        }
    }

    /// Ширина по тексту: 30 + width + 7.
    fn width(t: &TextRenderer, name: &str) -> f32 {
        30.0 + t.text_width(name) + 7.0
    }

    /// VANILLA: NowPlayingToast#extractToast в точке (x, 0).
    #[allow(clippy::too_many_arguments)]
    fn draw_at(
        &self,
        x: f32,
        name: &str,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        dev: &wgpu::Device,
        q: &wgpu::Queue,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
    ) {
        let bw = Self::width(t, name);
        p.begin(t.scale());
        p.nine_slice(
            "toast_np",
            x,
            0.0,
            bw,
            30.0,
            [160.0, 32.0],
            [4.0; 4],
            0xFFFF_FFFF,
        );
        // music_notes.png 16×128 — 8 кадров, frametime 2 тика (100 мс).
        let ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        let f = (ms / 100 % 8) as f32;
        p.blit(
            "music_notes",
            x + 7.0,
            7.0,
            16.0,
            16.0,
            [0.0, f / 8.0, 1.0, (f + 1.0) / 8.0],
            self.note_color,
        );
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        // TEXT_COLOR = DyeColor.LIGHT_GRAY.getTextColor().
        t.draw(name, x + 30.0, 15.0 - 4.0, 0xD3D3D3, true);
        t.flush(dev, q, view, w, h);
    }
}

/// Кадр NowPlayingToast. `mode` — опция musicToast (never/pause/pause_and_toast).
/// В паузе — постоянно у (0,0) (PauseScreen), иначе — выезжающий тост (ToastManager).
#[allow(clippy::too_many_arguments)]
pub fn now_playing_frame(
    np: &mut NowPlaying,
    key: Option<&str>,
    mode: &str,
    in_pause: bool,
    show_hud: bool,
    t: &mut TextRenderer,
    p: &mut GuiPainter,
    g: &ezzmc_render::Gpu,
    view: &wgpu::TextureView,
    size: (u32, u32),
    lang: &Lang,
) {
    let mode = mode.trim_matches('"');
    let (pause_ok, toast_ok) = (
        mode != "never" && !mode.is_empty(),
        mode == "pause_and_toast",
    );
    np.update(key.is_some(), display_mult());
    let Some(key) = key else { return };
    let name = lang.tr(key);
    if in_pause {
        if pause_ok && audio::music_volume() > 0.0 {
            np.tick_notes(true);
            np.draw_at(0.0, &name, t, p, &g.device, &g.queue, view, size);
        }
    } else if show_hud && toast_ok && !np.finished {
        let bw = NowPlaying::width(t, &name);
        let x = bw * np.portion - bw;
        np.draw_at(x, &name, t, p, &g.device, &g.queue, view, size);
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

static DISPLAY_MULT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x3F80_0000); // 1.0

/// VANILLA: Options#notificationDisplayTime.
pub fn set_display_mult(v: f32) {
    DISPLAY_MULT.store(v.to_bits(), std::sync::atomic::Ordering::Relaxed);
}

pub fn display_mult() -> f32 {
    f32::from_bits(DISPLAY_MULT.load(std::sync::atomic::Ordering::Relaxed))
}
