//! Экран достижений. VANILLA: AdvancementsScreen, AdvancementTab, AdvancementTabType,
//! AdvancementWidget, AdvancementWidgetType (26.3). Данные — `advancements::Advancements`.
use std::collections::{HashMap, HashSet};

use crate::advancements::{Adv, Advancements, Kind};
use crate::chat;
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const SPRITES: &[(&str, &str)] = &[
    ("adv_window", "advancements/window"),
    ("adv_box_obtained", "sprites/advancements/box_obtained"),
    ("adv_box_unobtained", "sprites/advancements/box_unobtained"),
    (
        "adv_challenge_frame_obtained",
        "sprites/advancements/challenge_frame_obtained",
    ),
    (
        "adv_challenge_frame_unobtained",
        "sprites/advancements/challenge_frame_unobtained",
    ),
    (
        "adv_goal_frame_obtained",
        "sprites/advancements/goal_frame_obtained",
    ),
    (
        "adv_goal_frame_unobtained",
        "sprites/advancements/goal_frame_unobtained",
    ),
    (
        "adv_task_frame_obtained",
        "sprites/advancements/task_frame_obtained",
    ),
    (
        "adv_task_frame_unobtained",
        "sprites/advancements/task_frame_unobtained",
    ),
    ("adv_title_box", "sprites/advancements/title_box"),
    ("adv_tab_above_left", "sprites/advancements/tab_above_left"),
    (
        "adv_tab_above_left_selected",
        "sprites/advancements/tab_above_left_selected",
    ),
    (
        "adv_tab_above_middle",
        "sprites/advancements/tab_above_middle",
    ),
    (
        "adv_tab_above_middle_selected",
        "sprites/advancements/tab_above_middle_selected",
    ),
    (
        "adv_tab_above_right",
        "sprites/advancements/tab_above_right",
    ),
    (
        "adv_tab_above_right_selected",
        "sprites/advancements/tab_above_right_selected",
    ),
    ("adv_tab_below_left", "sprites/advancements/tab_below_left"),
    (
        "adv_tab_below_left_selected",
        "sprites/advancements/tab_below_left_selected",
    ),
    (
        "adv_tab_below_middle",
        "sprites/advancements/tab_below_middle",
    ),
    (
        "adv_tab_below_middle_selected",
        "sprites/advancements/tab_below_middle_selected",
    ),
    (
        "adv_tab_below_right",
        "sprites/advancements/tab_below_right",
    ),
    (
        "adv_tab_below_right_selected",
        "sprites/advancements/tab_below_right_selected",
    ),
    ("adv_tab_left_top", "sprites/advancements/tab_left_top"),
    (
        "adv_tab_left_top_selected",
        "sprites/advancements/tab_left_top_selected",
    ),
    (
        "adv_tab_left_middle",
        "sprites/advancements/tab_left_middle",
    ),
    (
        "adv_tab_left_middle_selected",
        "sprites/advancements/tab_left_middle_selected",
    ),
    (
        "adv_tab_left_bottom",
        "sprites/advancements/tab_left_bottom",
    ),
    (
        "adv_tab_left_bottom_selected",
        "sprites/advancements/tab_left_bottom_selected",
    ),
    ("adv_tab_right_top", "sprites/advancements/tab_right_top"),
    (
        "adv_tab_right_top_selected",
        "sprites/advancements/tab_right_top_selected",
    ),
    (
        "adv_tab_right_middle",
        "sprites/advancements/tab_right_middle",
    ),
    (
        "adv_tab_right_middle_selected",
        "sprites/advancements/tab_right_middle_selected",
    ),
    (
        "adv_tab_right_bottom",
        "sprites/advancements/tab_right_bottom",
    ),
    (
        "adv_tab_right_bottom_selected",
        "sprites/advancements/tab_right_bottom_selected",
    ),
];

const WIN_W: i32 = 252;
const WIN_H: i32 = 140;
const IN_W: i32 = 234;
const IN_H: i32 = 113;

/// VANILLA: AdvancementTabType (порядок = порядок заполнения индексов).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TabType {
    Above,
    Below,
    Left,
    Right,
}

impl TabType {
    const ALL: [TabType; 4] = [
        TabType::Above,
        TabType::Below,
        TabType::Left,
        TabType::Right,
    ];

    fn size(self) -> (i32, i32) {
        match self {
            TabType::Above | TabType::Below => (28, 32),
            TabType::Left | TabType::Right => (32, 28),
        }
    }

    fn max(self) -> usize {
        match self {
            TabType::Above | TabType::Below => 8,
            _ => 5,
        }
    }

    fn x(self, i: usize) -> i32 {
        let (w, _) = self.size();
        match self {
            TabType::Above | TabType::Below => (w + 4) * i as i32,
            TabType::Left => -w + 4,
            TabType::Right => 248,
        }
    }

    fn y(self, i: usize) -> i32 {
        let (_, h) = self.size();
        match self {
            TabType::Above => -h + 4,
            TabType::Below => 136,
            TabType::Left | TabType::Right => h * i as i32,
        }
    }

    /// `i` — индекс внутри типа (в ваниле индекс сквозной: first при 0, last при max-1).
    fn sprite(self, i: usize, selected: bool) -> String {
        let (side, a, b, c) = match self {
            TabType::Above => ("above", "left", "middle", "right"),
            TabType::Below => ("below", "left", "middle", "right"),
            TabType::Left => ("left", "top", "middle", "bottom"),
            TabType::Right => ("right", "top", "middle", "bottom"),
        };
        let pos = if i == 0 {
            a
        } else if i == self.max() - 1 {
            c
        } else {
            b
        };
        format!(
            "adv_tab_{side}_{pos}{}",
            if selected { "_selected" } else { "" }
        )
    }

    fn icon_off(self) -> (i32, i32) {
        match self {
            TabType::Above => (6, 9),
            TabType::Below => (6, 6),
            TabType::Left => (10, 5),
            TabType::Right => (6, 5),
        }
    }

    fn over(self, xo: i32, yo: i32, i: usize, mx: f32, my: f32) -> bool {
        let (x, y) = (xo + self.x(i), yo + self.y(i));
        let (w, h) = self.size();
        mx > x as f32 && mx < (x + w) as f32 && my > y as f32 && my < (y + h) as f32
    }
}

struct Widget {
    id: String,
    x: i32,
    y: i32,
    parent: Option<usize>,
    children: Vec<usize>,
    title: Vec<String>,
    desc: Vec<String>,
    width: i32,
    icon: ItemStack,
    kind: Kind,
    hidden: bool,
}

struct Tab {
    root: String,
    ty: TabType,
    /// Индекс внутри типа.
    sub: usize,
    title: String,
    background: String,
    icon: ItemStack,
    widgets: Vec<Widget>,
    min: (i32, i32),
    max: (i32, i32),
}

impl Tab {
    fn can_h(&self) -> bool {
        self.max.0 - self.min.0 > IN_W
    }
    fn can_v(&self) -> bool {
        self.max.1 - self.min.1 > IN_H
    }
}

#[derive(Default, Clone, Copy)]
struct Scroll {
    x: f64,
    y: f64,
    centered: bool,
}

#[derive(Default)]
pub struct AdvScreen {
    version: Option<u64>,
    tabs: Vec<Tab>,
    scroll: HashMap<String, Scroll>,
    /// VANILLA: AdvancementTab#fade / hovered (тик 20 Гц).
    fade: f32,
    hovered: Option<usize>,
    tick_acc: f32,
    /// Правая кнопка зажата / началась прокрутка (isScrolling).
    pressed: bool,
    pub scrolling: bool,
    last: (f32, f32),
    bg_loaded: HashSet<String>,
    /// OPENED_TAB при открытии экрана уже отправлен.
    announced: bool,
}

/// Строка → текст с цветом из компонента.
fn legacy(c: &ezzmc_protocol::text::Component, lang: &Lang, base: u32) -> String {
    chat::spans_to_legacy(&chat::flatten_styled(c, lang, base, false), base)
}

fn max_width(t: &TextRenderer, lines: &[String]) -> f32 {
    lines.iter().map(|l| t.text_width(l)).fold(0.0, f32::max)
}

/// VANILLA: AdvancementWidget#findOptimalLines.
fn optimal_lines(t: &TextRenderer, s: &str, preferred: i32) -> Vec<String> {
    let mut best: Option<Vec<String>> = None;
    let mut best_d = f32::MAX;
    for m in [0, 10, -10, 25, -25] {
        let split = crate::ui::wrap_n(t, s, (preferred - m) as f32, 256);
        let d = (max_width(t, &split) - preferred as f32).abs();
        if d <= 10.0 {
            return split;
        }
        if d < best_d {
            best_d = d;
            best = Some(split);
        }
    }
    best.unwrap_or_default()
}

fn first_visible_parent<'a>(adv: &'a Advancements, a: &'a Adv) -> Option<&'a Adv> {
    let mut cur = a;
    loop {
        cur = adv.get(cur.parent.as_deref()?)?;
        if cur.display.is_some() {
            return Some(cur);
        }
    }
}

fn root_of<'a>(adv: &'a Advancements, a: &'a Adv) -> &'a Adv {
    let mut cur = a;
    while let Some(p) = cur.parent.as_deref().and_then(|p| adv.get(p)) {
        cur = p;
    }
    cur
}

impl AdvScreen {
    /// VANILLA: AdvancementsScreen#onAdvancementsUpdated — пересборка вкладок и виджетов.
    fn rebuild(&mut self, adv: &Advancements, t: &TextRenderer, lang: &Lang) {
        self.tabs.clear();
        for root in adv.list.iter().filter(|a| a.parent.is_none()) {
            let Some(d) = &root.display else { continue };
            let Some(bg) = &d.background else { continue };
            let mut index = self.tabs.len();
            let mut found = None;
            for ty in TabType::ALL {
                if index < ty.max() {
                    found = Some((ty, index));
                    break;
                }
                index -= ty.max();
            }
            let Some((ty, sub)) = found else { continue };
            let mut tab = Tab {
                root: root.id.clone(),
                ty,
                sub,
                title: legacy(&d.title, lang, 0xFFFFFF),
                background: bg.clone(),
                icon: d.icon.clone(),
                widgets: Vec::new(),
                min: (i32::MAX, i32::MAX),
                max: (i32::MIN, i32::MIN),
            };
            let members = std::iter::once(root).chain(adv.list.iter().filter(|a| {
                a.id != root.id && a.display.is_some() && root_of(adv, a).id == root.id
            }));
            for a in members {
                let Some(d) = &a.display else { continue };
                let title = crate::ui::wrap_n(t, &legacy(&d.title, lang, 0xFFFFFF), 163.0, 64);
                let tw = (max_width(t, &title) as i32).max(80);
                let n = a.requirements.len();
                let prog_w = if n > 1 {
                    let s = lang.fmt("advancements.progress", &[&n.to_string(), &n.to_string()]);
                    t.text_width(&s) as i32 + 8
                } else {
                    0
                };
                let mut longest = 29 + tw + prog_w;
                let col = d.kind.chat_color();
                let desc = optimal_lines(t, &legacy(&d.description, lang, col), longest);
                for l in &desc {
                    longest = longest.max(t.text_width(l) as i32);
                }
                let x = (a.x * 28.0).floor() as i32;
                let y = (a.y * 27.0).floor() as i32;
                tab.min = (tab.min.0.min(x), tab.min.1.min(y));
                tab.max = (tab.max.0.max(x + 28), tab.max.1.max(y + 27));
                tab.widgets.push(Widget {
                    id: a.id.clone(),
                    x,
                    y,
                    parent: None,
                    children: Vec::new(),
                    title,
                    desc,
                    width: longest + 3 + 5,
                    icon: d.icon.clone(),
                    kind: d.kind,
                    hidden: d.hidden,
                });
            }
            // VANILLA: AdvancementWidget#attachToParent — первый видимый предок.
            for i in 0..tab.widgets.len() {
                let Some(a) = adv.get(&tab.widgets[i].id) else {
                    continue;
                };
                let Some(p) = first_visible_parent(adv, a) else {
                    continue;
                };
                if let Some(pi) = tab.widgets.iter().position(|w| w.id == p.id) {
                    tab.widgets[i].parent = Some(pi);
                    tab.widgets[pi].children.push(i);
                }
            }
            self.tabs.push(tab);
        }
    }

    /// Вкладка для OPENED_TAB при смене выбора (или `None`).
    fn selected(&self, adv: &Advancements) -> Option<usize> {
        let id = adv.selected.as_deref()?;
        self.tabs.iter().position(|t| t.root == id)
    }

    /// VANILLA: AdvancementsScreen#init / onAdvancementsUpdated — выбрать первую вкладку,
    /// если выбранной нет. Возвращает id вкладки для ServerboundSeenAdvancementsPacket.
    pub fn sync(
        &mut self,
        adv: &mut Advancements,
        t: &TextRenderer,
        lang: &Lang,
    ) -> Option<String> {
        if self.version != Some(adv.version) {
            self.version = Some(adv.version);
            self.rebuild(adv, t, lang);
        }
        if self.selected(adv).is_none() {
            if let Some(first) = self.tabs.first() {
                let id = first.root.clone();
                adv.select(Some(id.clone()));
                self.version = Some(adv.version);
                return Some(id);
            }
        }
        None
    }

    /// Первая отправка при открытии экрана (setSelectedTab(…, true) всегда шлёт пакет).
    pub fn opened_tab(&self, adv: &Advancements) -> Option<String> {
        self.selected(adv).map(|i| self.tabs[i].root.clone())
    }

    fn origin(gw: f32, gh: f32) -> (i32, i32) {
        (((gw as i32) - WIN_W) / 2, ((gh as i32) - WIN_H) / 2)
    }

    /// VANILLA: AdvancementsScreen#mouseClicked (button 1) — выбор вкладки. Возвращает id.
    pub fn click_tab(&self, gw: f32, gh: f32, mx: f32, my: f32) -> Option<String> {
        let (xo, yo) = Self::origin(gw, gh);
        self.tabs
            .iter()
            .find(|t| t.ty.over(xo, yo, t.sub, mx, my))
            .map(|t| t.root.clone())
    }

    /// VANILLA: AdvancementTab#scroll.
    pub fn scroll_by(&mut self, adv: &Advancements, dx: f64, dy: f64) {
        let Some(i) = self.selected(adv) else { return };
        let tab = &self.tabs[i];
        let (ch, cv) = (tab.can_h(), tab.can_v());
        let (maxx, maxy) = (tab.max.0, tab.max.1);
        let s = self.scroll.entry(tab.root.clone()).or_default();
        if ch {
            s.x = (s.x + dx).clamp(-f64::from(maxx - IN_W), 0.0);
        }
        if cv {
            s.y = (s.y + dy).clamp(-f64::from(maxy - IN_H), 0.0);
        }
    }

    /// VANILLA: AdvancementsScreen#mouseScrolled — шаг 16.
    pub fn wheel(&mut self, adv: &Advancements, x: f32, y: f32) {
        self.scroll_by(adv, f64::from(x) * 16.0, f64::from(y) * 16.0);
    }

    /// Правая кнопка: нажатие/отпускание (mouseDragged/mouseReleased).
    pub fn press(&mut self, down: bool) {
        self.pressed = down;
        if !down {
            self.scrolling = false;
        }
    }

    /// VANILLA: AdvancementsScreen#mouseDragged (button 1): первый вызов включает прокрутку.
    pub fn cursor(&mut self, adv: &Advancements, mx: f32, my: f32) {
        let (dx, dy) = (mx - self.last.0, my - self.last.1);
        self.last = (mx, my);
        if !self.pressed {
            return;
        }
        if !self.scrolling {
            self.scrolling = true;
        } else {
            self.scroll_by(adv, f64::from(dx), f64::from(dy));
        }
    }

    /// Для агента: (вкладки (id, заголовок), выбранная, виджетов во вкладке).
    pub fn snapshot(&self, adv: &Advancements) -> (Vec<(String, String)>, Option<String>, usize) {
        let sel = self.selected(adv);
        (
            self.tabs
                .iter()
                .map(|t| (t.root.clone(), t.title.clone()))
                .collect(),
            sel.map(|i| self.tabs[i].root.clone()),
            sel.map_or(0, |i| self.tabs[i].widgets.len()),
        )
    }

    fn ensure_bg(
        &mut self,
        p: &mut GuiPainter,
        dev: &wgpu::Device,
        q: &wgpu::Queue,
        rm: &ResourceManager,
        id: &str,
    ) -> String {
        let name = format!("advbg:{id}");
        if self.bg_loaded.insert(name.clone()) && !p.has(&name) {
            let (ns, path) = id.split_once(':').unwrap_or(("minecraft", id));
            let path = path.strip_suffix(".png").unwrap_or(path);
            let path = if path.starts_with("textures/") {
                path.to_owned()
            } else {
                format!("textures/{path}")
            };
            let full = format!("assets/{ns}/{path}.png");
            match rm
                .read(&full)
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            {
                Some(i) => p.add_texture(dev, q, &name, &i.to_rgba8(), i.width, i.height, false),
                None => tracing::warn!("фон достижений не найден: {full}"),
            }
        }
        name
    }
}

/// Прямоугольник GUI-пикселей → пиксели окна для scissor.
fn px(r: (i32, i32, i32, i32), k: f32) -> [u32; 4] {
    let f = |v: i32| (v.max(0) as f32 * k) as u32;
    [f(r.0), f(r.1), f(r.2 - r.0), f(r.3 - r.1)]
}

impl AdvScreen {
    /// VANILLA: AdvancementsScreen#extractRenderState. Возвращает вкладку для OPENED_TAB (смена).
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        adv: &mut Advancements,
        lang: &Lang,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        g: &ezzmc_render::Gpu,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        rm: &ResourceManager,
        (mx, my): (f32, f32),
        dt: f32,
    ) -> Option<String> {
        let (dev, q) = (&g.device, &g.queue);
        let mut send = self.sync(adv, t, lang);
        // VANILLA: AdvancementsScreen#init — setSelectedTab(…, true) шлёт пакет при открытии.
        if !self.announced && !self.tabs.is_empty() {
            self.announced = true;
            send = send.or_else(|| self.opened_tab(adv));
        }
        let k = t.scale() as f32;
        let (gw, gh) = t.gui_size(w, h);
        let (gwf, ghf) = (gw as f32, gh as f32);
        let (xo, yo) = Self::origin(gwf, ghf);
        let sel = self.selected(adv);
        // Тик 20 Гц (AdvancementsScreen#tick → AdvancementTab#tick).
        self.tick_acc += dt * 20.0;
        let ticks = self.tick_acc.floor() as u32;
        self.tick_acc -= ticks as f32;
        if let Some(i) = sel {
            let (rx, ry) = ((mx.floor() as i32) - xo - 9, (my.floor() as i32) - yo - 18);
            let s = self
                .scroll
                .get(&self.tabs[i].root)
                .copied()
                .unwrap_or_default();
            let (sx, sy) = (s.x.floor() as i32, s.y.floor() as i32);
            for _ in 0..ticks.min(20) {
                let hov = if rx > 0 && rx < IN_W && ry > 0 && ry < IN_H {
                    self.tabs[i].widgets.iter().position(|wd| {
                        (!wd.hidden || adv.is_done(&wd.id))
                            && rx >= sx + wd.x
                            && rx <= sx + wd.x + 26
                            && ry >= sy + wd.y
                            && ry <= sy + wd.y + 26
                    })
                } else {
                    None
                };
                if hov.is_some() {
                    self.fade = (self.fade + 0.06).clamp(0.0, 0.3);
                    self.hovered = hov;
                } else {
                    self.fade = (self.fade - 0.12).clamp(0.0, 1.0);
                    self.hovered = None;
                }
            }
        }
        // Затемнение фона (Screen#extractTransparentBackground).
        t.begin(w, h);
        t.fill_gradient(0.0, 0.0, gwf, ghf, 0xC010_1010, 0xD010_1010);
        t.flush(dev, q, view, w, h);
        // --- Содержимое (scissor 234×113) ---
        let (cx, cy) = (xo + 9, yo + 18);
        let rect = (cx, cy, cx + IN_W, cy + IN_H);
        match sel {
            None => {
                t.begin(w, h);
                t.fill(
                    cx as f32,
                    cy as f32,
                    (cx + IN_W) as f32,
                    (cy + IN_H) as f32,
                    0xFF00_0000,
                );
                let mid = (cx + IN_W / 2) as f32;
                let a = lang.tr("advancements.empty");
                let b = lang.tr("advancements.sad_label");
                t.draw(
                    &a,
                    mid - t.text_width(&a) / 2.0,
                    (cy + 56 - 4) as f32,
                    0xFFFFFF,
                    true,
                );
                t.draw(
                    &b,
                    mid - t.text_width(&b) / 2.0,
                    (cy + IN_H - 9) as f32,
                    0xFFFFFF,
                    true,
                );
                t.flush(dev, q, view, w, h);
            }
            Some(i) => {
                // VANILLA: AdvancementTab#extractContents — центрирование при первом показе.
                let (rootid, minx, maxx, miny, maxy, bg) = {
                    let tb = &self.tabs[i];
                    (
                        tb.root.clone(),
                        tb.min.0,
                        tb.max.0,
                        tb.min.1,
                        tb.max.1,
                        tb.background.clone(),
                    )
                };
                let s = self.scroll.entry(rootid.clone()).or_default();
                if !s.centered {
                    s.x = f64::from(117 - (maxx + minx) / 2);
                    s.y = f64::from(56 - (maxy + miny) / 2);
                    s.centered = true;
                }
                let (sx, sy) = (s.x.floor() as i32, s.y.floor() as i32);
                let bgname = self.ensure_bg(p, dev, q, rm, &bg);
                p.begin(t.scale());
                let (left, top) = (sx % 16, sy % 16);
                for x in -1..=15 {
                    for y in -1..=8 {
                        p.blit(
                            &bgname,
                            (cx + left + 16 * x) as f32,
                            (cy + top + 16 * y) as f32,
                            16.0,
                            16.0,
                            [0.0, 0.0, 1.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
                let tb = &self.tabs[i];
                let line = |p: &mut GuiPainter, x0: i32, x1: i32, y: i32, col: u32| {
                    let (a, b) = if x1 < x0 { (x1, x0) } else { (x0, x1) };
                    p.blit(
                        "white",
                        (cx + a) as f32,
                        (cy + y) as f32,
                        (b - a + 1) as f32,
                        1.0,
                        [0.0, 0.0, 1.0, 1.0],
                        col,
                    );
                };
                let vline = |p: &mut GuiPainter, x: i32, y0: i32, y1: i32, col: u32| {
                    let (a, b) = if y1 < y0 { (y1, y0) } else { (y0, y1) };
                    p.blit(
                        "white",
                        (cx + x) as f32,
                        (cy + a) as f32,
                        1.0,
                        (b - a + 1) as f32,
                        [0.0, 0.0, 1.0, 1.0],
                        col,
                    );
                };
                // VANILLA: AdvancementWidget#extractConnectivity (фон, затем белые линии).
                for background in [true, false] {
                    let col = if background { 0xFF00_0000 } else { 0xFFFF_FFFF };
                    for wd in &tb.widgets {
                        let Some(pi) = wd.parent else { continue };
                        let par = &tb.widgets[pi];
                        let (dep_x, split_x, dep_y) =
                            (sx + par.x + 13, sx + par.x + 26 + 4, sy + par.y + 13);
                        let (my_x, my_y) = (sx + wd.x + 13, sy + wd.y + 13);
                        if background {
                            line(p, split_x, dep_x, dep_y - 1, col);
                            line(p, split_x + 1, dep_x, dep_y, col);
                            line(p, split_x, dep_x, dep_y + 1, col);
                            line(p, my_x, split_x - 1, my_y - 1, col);
                            line(p, my_x, split_x - 1, my_y, col);
                            line(p, my_x, split_x - 1, my_y + 1, col);
                            vline(p, split_x - 1, my_y, dep_y, col);
                            vline(p, split_x + 1, my_y, dep_y, col);
                        } else {
                            line(p, split_x, dep_x, dep_y, col);
                            line(p, my_x, split_x, my_y, col);
                            vline(p, split_x, my_y, dep_y, col);
                        }
                    }
                }
                // Рамки виджетов.
                for wd in &tb.widgets {
                    if !wd.hidden || adv.is_done(&wd.id) {
                        let done = adv.percent(&wd.id) >= 1.0;
                        p.blit(
                            &frame_sprite(done, wd.kind),
                            (cx + sx + wd.x + 3) as f32,
                            (cy + sy + wd.y) as f32,
                            26.0,
                            26.0,
                            [0.0, 0.0, 1.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
                p.set_scissor(Some(px(rect, k)));
                p.flush(dev, q, view, w, h, None);
                p.set_scissor(None);
                p.begin(t.scale());
                for wd in &tb.widgets {
                    if !wd.hidden || adv.is_done(&wd.id) {
                        crate::hud::stack_icon(
                            p,
                            dev,
                            q,
                            rm,
                            &wd.icon,
                            (cx + sx + wd.x + 8) as f32,
                            (cy + sy + wd.y + 5) as f32,
                        );
                    }
                }
                p.set_scissor(Some(px(rect, k)));
                p.flush(dev, q, view, w, h, None);
                p.set_scissor(None);
            }
        }
        // --- Окно, вкладки, заголовок ---
        p.begin(t.scale());
        p.blit(
            "adv_window",
            xo as f32,
            yo as f32,
            WIN_W as f32,
            WIN_H as f32,
            [0.0, 0.0, WIN_W as f32 / 256.0, WIN_H as f32 / 256.0],
            0xFFFF_FFFF,
        );
        if self.tabs.len() > 1 {
            for (i, tb) in self.tabs.iter().enumerate() {
                let (sw, sh) = tb.ty.size();
                p.blit(
                    &tb.ty.sprite(tb.sub, sel == Some(i)),
                    (xo + tb.ty.x(tb.sub)) as f32,
                    (yo + tb.ty.y(tb.sub)) as f32,
                    sw as f32,
                    sh as f32,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFF_FFFF,
                );
            }
        }
        p.flush(dev, q, view, w, h, None);
        if self.tabs.len() > 1 {
            p.begin(t.scale());
            for tb in &self.tabs {
                let (ox, oy) = tb.ty.icon_off();
                crate::hud::stack_icon(
                    p,
                    dev,
                    q,
                    rm,
                    &tb.icon,
                    (xo + tb.ty.x(tb.sub) + ox) as f32,
                    (yo + tb.ty.y(tb.sub) + oy) as f32,
                );
            }
            p.flush(dev, q, view, w, h, None);
        }
        t.begin(w, h);
        let title = sel.map_or_else(
            || lang.tr("gui.advancements"),
            |i| self.tabs[i].title.clone(),
        );
        t.draw(&title, (xo + 8) as f32, (yo + 6) as f32, 0x404040, false);
        t.flush(dev, q, view, w, h);
        // --- Подсказки ---
        if let Some(i) = sel {
            let alpha = ((self.fade * 255.0).floor() as u32).min(255);
            t.begin(w, h);
            t.fill(
                cx as f32,
                cy as f32,
                (cx + IN_W) as f32,
                (cy + IN_H) as f32,
                alpha << 24,
            );
            t.flush(dev, q, view, w, h);
            if let Some(hi) = self.hovered.filter(|&hi| hi < self.tabs[i].widgets.len()) {
                let s = self
                    .scroll
                    .get(&self.tabs[i].root)
                    .copied()
                    .unwrap_or_default();
                let (sx, sy) = (s.x.floor() as i32, s.y.floor() as i32);
                self.draw_hover(
                    adv,
                    i,
                    hi,
                    (sx, sy),
                    (cx, cy),
                    gw as i32,
                    lang,
                    t,
                    p,
                    g,
                    view,
                    (w, h),
                    rm,
                );
            }
        }
        if self.tabs.len() > 1 {
            for tb in &self.tabs {
                if tb.ty.over(xo, yo, tb.sub, mx, my) {
                    t.begin(w, h);
                    crate::tooltip::draw_colored(
                        t,
                        &[(tb.title.clone(), 0xFFFFFF)],
                        mx,
                        my,
                        gwf,
                        ghf,
                    );
                    t.flush(dev, q, view, w, h);
                }
            }
        }
        send
    }

    /// VANILLA: AdvancementWidget#extractHover. `(cx, cy)` — начало области содержимого.
    #[allow(clippy::too_many_arguments)]
    fn draw_hover(
        &self,
        adv: &Advancements,
        ti: usize,
        wi: usize,
        (sx, sy): (i32, i32),
        (cx, cy): (i32, i32),
        screen_w: i32,
        lang: &Lang,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        g: &ezzmc_render::Gpu,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        rm: &ResourceManager,
    ) {
        let (dev, q) = (&g.device, &g.queue);
        let wd = &self.tabs[ti].widgets[wi];
        let (xo, yo) = (sx, sy);
        let tl = wd.title.len() as i32;
        let bar_h = 9 * tl + 9 + 8;
        let title_top = yo + wd.y + (26 - bar_h) / 2;
        let bar_bottom = title_top + bar_h;
        let desc_text_h = wd.desc.len() as i32 * 9;
        let desc_h = 6 + desc_text_h;
        // screenxo = leftPos (окно), xo — прокрутка.
        let left_pos = cx - 9;
        let left_side = left_pos + xo + wd.x + wd.width + 26 >= screen_w;
        let progress = adv.progress_counts(&wd.id);
        let ptext = progress
            .map(|(a, b)| lang.fmt("advancements.progress", &[&a.to_string(), &b.to_string()]));
        let pw = ptext.as_ref().map_or(0, |s| t.text_width(s) as i32);
        let top_side = bar_bottom + desc_h >= IN_H;
        let amount = adv.percent(&wd.id);
        let mut first_w = (amount * wd.width as f32).floor() as i32;
        // (первая половина, вторая половина, рамка) — `true` = получено.
        let (fh, sh, fr);
        if amount >= 1.0 {
            first_w = wd.width / 2;
            (fh, sh, fr) = (true, true, true);
        } else if first_w < 2 {
            first_w = wd.width / 2;
            (fh, sh, fr) = (false, false, false);
        } else if first_w > wd.width - 2 {
            first_w = wd.width / 2;
            (fh, sh, fr) = (true, true, false);
        } else {
            (fh, sh, fr) = (true, false, false);
        }
        let second_w = wd.width - first_w;
        let title_left = if left_side {
            xo + wd.x - wd.width + 26 + 6
        } else {
            xo + wd.x
        };
        let bg_h = bar_h + desc_h;
        let k = t.scale() as f32;
        // Всё рисуется со смещением области содержимого.
        let (ox, oy) = (cx as f32, cy as f32);
        let ns = |p: &mut GuiPainter, name: &str, x: i32, y: i32, ww: i32, hh: i32| {
            p.nine_slice(
                name,
                ox + x as f32,
                oy + y as f32,
                ww as f32,
                hh as f32,
                [200.0, 26.0],
                [10.0; 4],
                0xFFFF_FFFF,
            );
        };
        p.begin(t.scale());
        if !wd.desc.is_empty() {
            let y = if top_side {
                bar_bottom - bg_h
            } else {
                title_top
            };
            ns(p, "adv_title_box", title_left, y, wd.width, bg_h);
        }
        let box_name = |done: bool| {
            if done {
                "adv_box_obtained"
            } else {
                "adv_box_unobtained"
            }
        };
        if fh != sh {
            // VANILLA-UNSURE: половинки рисуются как обрезка полного nine-slice (в ваниле — регион 200×h).
            let r1 = (
                cx + title_left,
                cy + title_top,
                cx + title_left + first_w,
                cy + title_top + bar_h,
            );
            ns(p, box_name(fh), title_left, title_top, wd.width, bar_h);
            p.set_scissor(Some(px(r1, k)));
            p.flush(dev, q, view, w, h, None);
            p.set_scissor(None);
            p.begin(t.scale());
            let r2 = (r1.2, r1.1, r1.2 + second_w, r1.3);
            ns(p, box_name(sh), title_left, title_top, wd.width, bar_h);
            p.set_scissor(Some(px(r2, k)));
            p.flush(dev, q, view, w, h, None);
            p.set_scissor(None);
            p.begin(t.scale());
        } else {
            ns(p, box_name(fh), title_left, title_top, wd.width, bar_h);
        }
        p.blit(
            &frame_sprite(fr, wd.kind),
            ox + (xo + wd.x + 3) as f32,
            oy + (yo + wd.y) as f32,
            26.0,
            26.0,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        let desc_left = title_left + 5;
        let text = |t: &mut TextRenderer, s: &str, x: i32, y: i32, col: u32| {
            t.draw(s, ox + x as f32, oy + y as f32, col, true);
        };
        if left_side {
            for (i, l) in wd.title.iter().enumerate() {
                text(t, l, desc_left, title_top + 9 + i as i32 * 9, 0xFFFFFF);
            }
            if let Some(s) = &ptext {
                text(t, s, xo + wd.x - pw, title_top + 9, 0xFFFFFF);
            }
        } else {
            for (i, l) in wd.title.iter().enumerate() {
                text(t, l, xo + wd.x + 32, title_top + 9 + i as i32 * 9, 0xFFFFFF);
            }
            if let Some(s) = &ptext {
                text(t, s, xo + wd.x + wd.width - pw - 5, title_top + 9, 0xFFFFFF);
            }
        }
        let dy = if top_side {
            title_top - desc_text_h + 1
        } else {
            bar_bottom
        };
        for (i, l) in wd.desc.iter().enumerate() {
            text(t, l, desc_left, dy + i as i32 * 9, wd.kind.chat_color());
        }
        t.flush(dev, q, view, w, h);
        p.begin(t.scale());
        crate::hud::stack_icon(
            p,
            dev,
            q,
            rm,
            &wd.icon,
            ox + (xo + wd.x + 8) as f32,
            oy + (yo + wd.y + 5) as f32,
        );
        p.flush(dev, q, view, w, h, None);
    }
}

/// VANILLA: AdvancementWidgetType#frameSprite.
fn frame_sprite(done: bool, kind: Kind) -> String {
    let k = match kind {
        Kind::Task => "task",
        Kind::Challenge => "challenge",
        Kind::Goal => "goal",
    };
    format!(
        "adv_{k}_frame_{}",
        if done { "obtained" } else { "unobtained" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_positions() {
        assert_eq!(TabType::Above.x(1), 32);
        assert_eq!(TabType::Above.y(0), -28);
        assert_eq!(TabType::Left.x(0), -28);
        assert_eq!(TabType::Right.y(2), 56);
        assert_eq!(
            TabType::Above.sprite(0, true),
            "adv_tab_above_left_selected"
        );
        assert_eq!(TabType::Above.sprite(7, false), "adv_tab_above_right");
        assert_eq!(TabType::Left.sprite(4, false), "adv_tab_left_bottom");
        assert!(TabType::Below.over(0, 0, 0, 5.0, 140.0));
    }
}
