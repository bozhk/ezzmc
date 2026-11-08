//! GUI главного меню (VANILLA: TitleScreen, JoinMultiplayerScreen, OptionsScreen и подэкраны).
//! Режим «немедленного рисования»: каждый кадр экран объявляет виджеты, клик обрабатывается на месте.
pub mod lang;
pub mod options;
pub mod servers;

use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use lang::Lang;
use options::{Def, Kind, Options};
use servers::Server;
use std::path::PathBuf;

const WHITE: u32 = 0xFFFFFF;
const GRAY: u32 = 0xA0A0A0;

/// Текстуры GUI: (имя в рисовалке, путь внутри `assets/minecraft/textures/gui/`).
pub const TEXTURES: &[(&str, &str)] = &[
    ("button", "sprites/widget/button"),
    ("button_hl", "sprites/widget/button_highlighted"),
    ("button_off", "sprites/widget/button_disabled"),
    ("slider", "sprites/widget/slider"),
    ("slider_hl", "sprites/widget/slider_highlighted"),
    ("handle", "sprites/widget/slider_handle"),
    ("handle_hl", "sprites/widget/slider_handle_highlighted"),
    ("logo", "title/minecraft"),
    ("edition", "title/edition"),
    ("overlay", "title/background/panorama_overlay"),
    ("list_bg", "menu_list_background"),
    ("header_sep", "header_separator"),
    ("footer_sep", "footer_separator"),
    ("icon_language", "sprites/icon/language"),
    ("icon_access", "sprites/icon/accessibility"),
    ("panorama_0", "title/background/panorama_0"),
    ("panorama_1", "title/background/panorama_1"),
    ("panorama_2", "title/background/panorama_2"),
    ("panorama_3", "title/background/panorama_3"),
    ("panorama_4", "title/background/panorama_4"),
    ("panorama_5", "title/background/panorama_5"),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Video,
    Sound,
    Skin,
    Chat,
    Accessibility,
    Controls,
    Language,
    ResourcePacks,
    Telemetry,
    Credits,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Page::Video => "options.videoTitle",
            Page::Sound => "options.sounds.title",
            Page::Skin => "options.skinCustomisation.title",
            Page::Chat => "options.chat.title",
            Page::Accessibility => "options.accessibility.title",
            Page::Controls => "controls.title",
            Page::Language => "options.language.title",
            Page::ResourcePacks => "resourcePack.title",
            Page::Telemetry => "telemetry_info.screen.title",
            Page::Credits => "credits_and_attribution.screen.title",
        }
    }

    fn defs(self) -> &'static [Def] {
        match self {
            Page::Video => options::VIDEO,
            Page::Sound => options::SOUND,
            Page::Skin => options::SKIN,
            Page::Chat => options::CHAT,
            Page::Accessibility => options::ACCESSIBILITY,
            _ => &[],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Multiplayer,
    Direct,
    /// `None` — новый сервер.
    EditServer(Option<usize>),
    Singleplayer,
    Options,
    Page(Page),
    /// Меню паузы (в игре).
    Pause,
}

/// Рабочий порт локального сервера (xtask server 26.3 25611).
pub const DEFAULT_SERVER: &str = "localhost:25611";

pub enum Action {
    None,
    /// Подключиться к `host[:port]`.
    Join(String),
    Quit,
    /// Закрыть меню паузы и вернуться в игру.
    Resume,
    /// Выйти с сервера в главное меню.
    Disconnect,
}

struct State {
    stack: Vec<Screen>,
    servers: Vec<Server>,
    sel: Option<usize>,
    scroll: f32,
    fields: [String; 2],
    focus: usize,
    last_click: (Option<usize>, u64),
    confirm_delete: bool,
    servers_path: PathBuf,
    splash: String,
}

pub struct Gui {
    st: State,
    lang: Lang,
    pub opts: Options,
    mx: f32,
    my: f32,
    click: bool,
    down: bool,
    drag: Option<u32>,
    wheel: f32,
    pending_keys: Vec<UiKey>,
    pending_text: String,
    spin: f32,
    in_game: bool,
}

#[derive(Clone, Copy)]
pub enum UiKey {
    Escape,
    Enter,
    Backspace,
    Tab,
}

fn cur(st: &State) -> Screen {
    *st.stack.last().unwrap_or(&Screen::Title)
}

impl Gui {
    pub fn new(lang_json: &[u8], splashes: &str, dir: &std::path::Path) -> Self {
        let servers_path = dir.join("servers.dat");
        // Первый запуск: локальный сервер в списке по умолчанию.
        if !servers_path.exists() {
            servers::save(
                &servers_path,
                &[servers::Server {
                    name: "Local Server".into(),
                    ip: DEFAULT_SERVER.into(),
                }],
            );
        }
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as usize);
        let lines: Vec<&str> = splashes.lines().filter(|l| !l.trim().is_empty()).collect();
        let splash = if lines.is_empty() {
            String::new()
        } else {
            lines[seed % lines.len()].to_owned()
        };
        // Отладка: EZZMC_MENU_SCREEN=options|video|sound|multiplayer — открыть экран сразу.
        let mut stack = vec![Screen::Title];
        match std::env::var("EZZMC_MENU_SCREEN").as_deref() {
            Ok("options") => stack.push(Screen::Options),
            Ok("video") => stack.extend([Screen::Options, Screen::Page(Page::Video)]),
            Ok("sound") => stack.extend([Screen::Options, Screen::Page(Page::Sound)]),
            Ok("multiplayer") => stack.push(Screen::Multiplayer),
            _ => {}
        }
        Self {
            st: State {
                stack,
                servers: servers::load(&servers_path),
                sel: None,
                scroll: 0.0,
                fields: [String::new(), DEFAULT_SERVER.to_owned()],
                focus: 0,
                last_click: (None, 0),
                confirm_delete: false,
                servers_path,
                splash,
            },
            lang: Lang::load(lang_json),
            opts: Options::load(&dir.join("options.txt")),
            mx: 0.0,
            my: 0.0,
            click: false,
            down: false,
            drag: None,
            wheel: 0.0,
            pending_keys: Vec::new(),
            pending_text: String::new(),
            spin: 0.0,
            in_game: false,
        }
    }

    /// VANILLA: Minecraft#pauseGame — меню паузы поверх мира.
    pub fn open_pause(&mut self) {
        self.st.stack = vec![Screen::Pause];
        self.st.scroll = 0.0;
        self.in_game = true;
    }

    /// Вернуться в главное меню (после выхода с сервера).
    pub fn reset_title(&mut self) {
        self.st.stack = vec![Screen::Title];
        self.in_game = false;
    }

    /// Курсор в физических пикселях.
    pub fn on_cursor(&mut self, x: f64, y: f64) {
        self.mx = x as f32;
        self.my = y as f32;
    }

    pub fn on_left_button(&mut self, pressed: bool) {
        if pressed {
            self.click = true;
        } else {
            self.drag = None;
        }
        self.down = pressed;
    }

    pub fn on_wheel(&mut self, y: f32) {
        self.wheel += y;
    }

    pub fn on_key(&mut self, k: UiKey) {
        self.pending_keys.push(k);
    }

    pub fn on_text(&mut self, s: &str) {
        self.pending_text
            .extend(s.chars().filter(|c| !c.is_control()));
    }

    fn push(&mut self, s: Screen) {
        self.st.stack.push(s);
        self.st.scroll = 0.0;
        self.st.sel = None;
        self.st.confirm_delete = false;
    }

    fn pop(&mut self) {
        if self.st.stack.len() > 1 {
            self.st.stack.pop();
        }
        self.st.scroll = 0.0;
        self.st.confirm_delete = false;
        self.opts.save();
    }

    /// Рисует текущий экран. Возвращает действие для приложения.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        w: u32,
        h: u32,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        dt_s: f32,
        now_ms: u64,
    ) -> Action {
        let speed = self.opts.f32(&options::ACCESSIBILITY[3]);
        self.spin = (self.spin + dt_s * 20.0 * speed * 0.1) % 360.0;
        let guiscale = self.opts.f32(&options::VIDEO[3]) as u32;
        t.set_scale_option(guiscale);
        t.begin(w, h);
        p.begin(t.scale());
        let k = t.scale() as f32;
        let (gw, gh) = (w as f32 / k, h as f32 / k);
        let mut action = Action::None;
        let keys = std::mem::take(&mut self.pending_keys);
        let typed = std::mem::take(&mut self.pending_text);
        let wheel = std::mem::take(&mut self.wheel);
        let click = std::mem::take(&mut self.click);
        let mut esc = false;
        {
            let mut f = Frame {
                t,
                p,
                lang: &self.lang,
                mx: self.mx / k,
                my: self.my / k,
                click,
                down: self.down,
                drag: &mut self.drag,
                id: 0,
                w: gw,
                h: gh,
            };
            let st = &mut self.st;
            let opts = &mut self.opts;
            let screen = cur(st);
            // Фон: панорама + затемнение (VANILLA: Screen#extractPanorama; размытие не делаем).
            if !self.in_game {
                f.p.blit(
                    "overlay",
                    0.0,
                    0.0,
                    gw,
                    gh,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFFFFFF,
                );
            }
            if screen != Screen::Title {
                f.t.fill(0.0, 0.0, gw, gh, 0x70000000);
            }
            let mut next: Option<Screen> = None;
            let mut back = false;
            match screen {
                Screen::Title => title(&mut f, st, now_ms, &mut next, &mut action),
                Screen::Multiplayer => multiplayer(
                    &mut f,
                    st,
                    wheel,
                    now_ms,
                    &keys,
                    &mut next,
                    &mut back,
                    &mut action,
                ),
                Screen::Direct => form(
                    &mut f,
                    st,
                    &keys,
                    &typed,
                    now_ms,
                    true,
                    &mut back,
                    &mut action,
                    &mut next,
                ),
                Screen::EditServer(_) => form(
                    &mut f,
                    st,
                    &keys,
                    &typed,
                    now_ms,
                    false,
                    &mut back,
                    &mut action,
                    &mut next,
                ),
                Screen::Singleplayer => {
                    f.center("selectWorld.title", 8.0, WHITE);
                    f.center_raw("(single-player: later)", gh / 2.0 - 4.0, GRAY);
                    if f.button(
                        gw / 2.0 - 100.0,
                        gh - 27.0,
                        200.0,
                        20.0,
                        &f.lang.tr("gui.back"),
                        true,
                    ) {
                        back = true;
                    }
                }
                Screen::Pause => pause(&mut f, &mut next, &mut action),
                Screen::Options => options_hub(&mut f, st, opts, &mut next, &mut back),
                Screen::Page(pg) => page(&mut f, st, opts, pg, wheel, &mut back),
            }
            if keys.iter().any(|k| matches!(k, UiKey::Escape)) && screen != Screen::Title {
                if screen == Screen::Pause {
                    action = Action::Resume;
                } else {
                    esc = true;
                }
            }
            if let Some(n) = next {
                self.push(n);
                if matches!(n, Screen::EditServer(_) | Screen::Direct) {
                    prepare_form(&mut self.st, n);
                }
            } else if back || esc {
                self.pop();
            }
        }
        // Панорама (куб) + спрайты, затем текст/заливки.
        let pano = if self.in_game {
            None
        } else {
            Some((10.0, -self.spin))
        };
        p.flush(dev, queue, view, w, h, pano);
        t.flush(dev, queue, view, w, h);
        action
    }
}

fn prepare_form(st: &mut State, s: Screen) {
    st.focus = 0;
    st.fields = match s {
        Screen::EditServer(Some(i)) => st
            .servers
            .get(i)
            .map_or([String::new(), String::new()], |s| {
                [s.name.clone(), s.ip.clone()]
            }),
        Screen::EditServer(None) => [String::new(), String::new()],
        _ => [st.fields[1].clone(), String::new()],
    };
    if s == Screen::EditServer(None) {
        st.fields[0] = String::new();
    }
}

struct Frame<'a> {
    t: &'a mut TextRenderer,
    p: &'a mut GuiPainter,
    lang: &'a Lang,
    mx: f32,
    my: f32,
    click: bool,
    down: bool,
    drag: &'a mut Option<u32>,
    id: u32,
    w: f32,
    h: f32,
}

impl Frame<'_> {
    fn hover(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mx >= x && self.my >= y && self.mx < x + w && self.my < y + h
    }

    fn center_raw(&mut self, s: &str, y: f32, rgb: u32) {
        let x = (self.w - self.t.text_width(s)) / 2.0;
        self.t.draw(s, x, y, rgb, true);
    }

    fn center(&mut self, key: &str, y: f32, rgb: u32) {
        let s = self.lang.tr(key);
        self.center_raw(&s, y, rgb);
    }

    /// VANILLA: Button — спрайты button / highlighted / disabled (nine_slice 200×20, граница 3).
    fn button(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, active: bool) -> bool {
        let hov = self.hover(x, y, w, h) && active;
        let sprite = if !active {
            "button_off"
        } else if hov {
            "button_hl"
        } else {
            "button"
        };
        self.p
            .nine_slice(sprite, x, y, w, h, [200.0, 20.0], [3.0; 4], 0xFFFFFFFF);
        let tw = self.t.text_width(label);
        let col = if active { WHITE } else { GRAY };
        self.t
            .draw(label, x + (w - tw) / 2.0, y + (h - 8.0) / 2.0, col, true);
        if hov && self.click {
            self.click = false;
            return true;
        }
        false
    }

    /// VANILLA: AbstractSliderButton. `v` в 0..1; возвращает true, если значение менялось.
    fn slider(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, v: &mut f32) -> bool {
        self.id += 1;
        let hov = self.hover(x, y, w, h);
        if hov && self.click {
            self.click = false;
            *self.drag = Some(self.id);
        }
        let mut changed = false;
        if *self.drag == Some(self.id) && self.down {
            let nv = ((self.mx - (x + 4.0)) / (w - 8.0)).clamp(0.0, 1.0);
            if (nv - *v).abs() > f32::EPSILON {
                *v = nv;
                changed = true;
            }
        }
        let hl = hov || *self.drag == Some(self.id);
        self.p.nine_slice(
            if hl { "slider_hl" } else { "slider" },
            x,
            y,
            w,
            h,
            [200.0, 20.0],
            [1.0; 4],
            0xFFFFFFFF,
        );
        let hx = x + (w - 8.0) * *v;
        self.p.nine_slice(
            if hl { "handle_hl" } else { "handle" },
            hx.round(),
            y,
            8.0,
            h,
            [8.0, 20.0],
            [2.0, 2.0, 2.0, 3.0],
            0xFFFFFFFF,
        );
        let tw = self.t.text_width(label);
        self.t
            .draw(label, x + (w - tw) / 2.0, y + (h - 8.0) / 2.0, WHITE, true);
        changed
    }

    /// Фон списка между шапкой и подвалом (VANILLA: menu_list_background + separators).
    fn list_bg(&mut self, top: f32, bottom: f32) {
        let (w, h) = (self.w, bottom - top);
        self.p.tile("list_bg", 0.0, top, w, h, 16.0, 0xFFFFFFFF);
        self.p
            .tile("header_sep", 0.0, top - 2.0, w, 2.0, 32.0, 0xFFFFFFFF);
        self.p
            .tile("footer_sep", 0.0, bottom, w, 2.0, 32.0, 0xFFFFFFFF);
    }

    #[allow(clippy::too_many_arguments)]
    fn edit_box(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        text: &str,
        focused: bool,
        caret: bool,
    ) -> bool {
        let border = if focused { 0xFFFFFFFF } else { 0xFFA0A0A0 };
        self.t
            .fill(x - 1.0, y - 1.0, x + w + 1.0, y + h + 1.0, border);
        self.t.fill(x, y, x + w, y + h, 0xFF000000);
        let mut s = text.to_owned();
        // Хвост строки, если не помещается.
        while self.t.text_width(&s) > w - 8.0 {
            s.remove(0);
        }
        self.t
            .draw(&s, x + 4.0, y + (h - 8.0) / 2.0, 0xE0E0E0, true);
        if focused && caret {
            let cx = x + 4.0 + self.t.text_width(&s);
            self.t.fill(cx, y + 2.0, cx + 1.0, y + h - 2.0, 0xFFD0D0D0);
        }
        let hit = self.hover(x, y, w, h) && self.click;
        if hit {
            self.click = false;
        }
        hit
    }
}

/// VANILLA: TitleScreen — логотип, сплэш, кнопки, копирайт.
fn title(
    f: &mut Frame,
    st: &mut State,
    now_ms: u64,
    next: &mut Option<Screen>,
    action: &mut Action,
) {
    let (w, h) = (f.w, f.h);
    // Логотип 256×44 (текстура 256×64), edition 128×14 (текстура 128×16).
    f.p.blit(
        "logo",
        (w / 2.0).floor() - 128.0,
        30.0,
        256.0,
        44.0,
        [0.0, 0.0, 1.0, 44.0 / 64.0],
        0xFFFFFFFF,
    );
    f.p.blit(
        "edition",
        (w / 2.0).floor() - 64.0,
        30.0 + 44.0 - 7.0,
        128.0,
        14.0,
        [0.0, 0.0, 1.0, 14.0 / 16.0],
        0xFFFFFFFF,
    );
    // VANILLA: SplashRenderer — центр (w/2+123, 69), поворот −20°, пульсация масштаба.
    if !st.splash.is_empty() {
        let tw = f.t.text_width(&st.splash);
        let ph = 1.8
            - ((now_ms % 1000) as f32 / 1000.0 * std::f32::consts::TAU)
                .sin()
                .abs()
                * 0.1;
        let sc = ph * 100.0 / (tw + 32.0);
        f.t.set_transform(Some((w / 2.0 + 123.0, 69.0, -20.0, sc)));
        f.t.draw(&st.splash, -((tw / 2.0).floor()), -8.0, 0xFFFF00, true);
        f.t.set_transform(None);
    }
    let top = (h / 4.0).floor() + 48.0;
    let cx = (w / 2.0).floor();
    if f.button(
        cx - 100.0,
        top,
        200.0,
        20.0,
        &f.lang.tr("menu.singleplayer"),
        true,
    ) {
        *next = Some(Screen::Singleplayer);
    }
    if f.button(
        cx - 100.0,
        top + 24.0,
        200.0,
        20.0,
        &f.lang.tr("menu.multiplayer"),
        true,
    ) {
        *next = Some(Screen::Multiplayer);
    }
    let row = top + 72.0;
    if f.button(
        cx - 100.0,
        row,
        98.0,
        20.0,
        &f.lang.tr("menu.options"),
        true,
    ) {
        *next = Some(Screen::Options);
    }
    if f.button(cx + 2.0, row, 98.0, 20.0, &f.lang.tr("menu.quit"), true) {
        *action = Action::Quit;
    }
    // Иконки «язык» / «спец. возможности» слева и справа от ряда.
    if f.button(cx - 124.0, row, 20.0, 20.0, "", true) {
        *next = Some(Screen::Page(Page::Language));
    }
    f.p.blit(
        "icon_language",
        cx - 124.0 + 2.5,
        row + 2.5,
        15.0,
        15.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFFFFFF,
    );
    if f.button(cx + 104.0, row, 20.0, 20.0, "", true) {
        *next = Some(Screen::Page(Page::Accessibility));
    }
    f.p.blit(
        "icon_access",
        cx + 104.0 + 2.5,
        row + 2.5,
        15.0,
        15.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFFFFFF,
    );
    let c = f.lang.tr("title.credits");
    let cw = f.t.text_width(&c);
    f.t.draw(&c, w - cw - 2.0, h - 10.0, WHITE, true);
    f.t.draw("Minecraft 26.3 (ezzmc)", 2.0, h - 10.0, WHITE, true);
}

/// VANILLA: JoinMultiplayerScreen + ServerSelectionList (строка 36 px, двойной клик — вход).
#[allow(clippy::too_many_arguments)]
fn multiplayer(
    f: &mut Frame,
    st: &mut State,
    wheel: f32,
    now_ms: u64,
    keys: &[UiKey],
    next: &mut Option<Screen>,
    back: &mut bool,
    action: &mut Action,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (32.0, h - 64.0);
    f.center("multiplayer.title", 20.0 - 4.0, WHITE);
    f.list_bg(top, bottom);
    let rows = st.servers.len() as f32;
    let max_scroll = (rows * 36.0 - (bottom - top) + 4.0).max(0.0);
    st.scroll = (st.scroll - wheel * 18.0).clamp(0.0, max_scroll);
    let x0 = w / 2.0 - 125.0;
    if st.confirm_delete {
        // Подтверждение удаления (VANILLA: ConfirmScreen).
        f.t.fill(0.0, 0.0, w, h, 0xA0000000);
        f.center("selectServer.deleteQuestion", h / 2.0 - 30.0, WHITE);
        if let Some(s) = st.sel.and_then(|i| st.servers.get(i)) {
            let l = format!("'{}' will be lost forever! (A long time!)", s.name);
            f.center_raw(&l, h / 2.0 - 16.0, GRAY);
        }
        if f.button(
            w / 2.0 - 105.0,
            h / 2.0 + 4.0,
            100.0,
            20.0,
            &f.lang.tr("selectServer.deleteButton"),
            true,
        ) {
            if let Some(i) = st.sel.take().filter(|i| *i < st.servers.len()) {
                st.servers.remove(i);
                servers::save(&st.servers_path, &st.servers);
            }
            st.confirm_delete = false;
        }
        if f.button(
            w / 2.0 + 5.0,
            h / 2.0 + 4.0,
            100.0,
            20.0,
            &f.lang.tr("gui.cancel"),
            true,
        ) {
            st.confirm_delete = false;
        }
        return;
    }
    let mut join = None;
    for (i, s) in st.servers.iter().enumerate() {
        let y = top + 4.0 - st.scroll + i as f32 * 36.0;
        if y + 36.0 < top || y > bottom {
            continue;
        }
        let in_view = f.my >= top && f.my < bottom;
        if st.sel == Some(i) {
            f.t.fill(x0 - 2.0, y - 2.0, x0 + 252.0, y + 34.0, 0xFF808080);
            f.t.fill(x0 - 1.0, y - 1.0, x0 + 251.0, y + 33.0, 0xFF000000);
        }
        if f.hover(x0, y, 250.0, 32.0) && in_view && f.click {
            f.click = false;
            if st.last_click.0 == Some(i) && now_ms.saturating_sub(st.last_click.1) < 250 {
                join = Some(i);
            }
            st.sel = Some(i);
            st.last_click = (Some(i), now_ms);
        }
        f.t.draw(&s.name, x0 + 35.0, y + 1.0, WHITE, true);
        f.t.draw(&s.ip, x0 + 35.0, y + 12.0, GRAY, true);
    }
    if st.servers.is_empty() {
        f.center_raw("-", top + 12.0, GRAY);
    }
    // Нижние кнопки (VANILLA: два ряда по h-52 и h-28).
    let sel = st.sel.filter(|i| *i < st.servers.len());
    let (r1, r2) = (h - 52.0, h - 28.0);
    if f.button(
        w / 2.0 - 154.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.select"),
        sel.is_some(),
    ) {
        join = sel;
    }
    if f.button(
        w / 2.0 - 50.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.direct"),
        true,
    ) {
        *next = Some(Screen::Direct);
    }
    if f.button(
        w / 2.0 + 4.0 + 50.0,
        r1,
        100.0,
        20.0,
        &f.lang.tr("selectServer.add"),
        true,
    ) {
        *next = Some(Screen::EditServer(None));
    }
    if f.button(
        w / 2.0 - 154.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.edit"),
        sel.is_some(),
    ) {
        *next = sel.map(|i| Screen::EditServer(Some(i)));
    }
    if f.button(
        w / 2.0 - 74.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.delete"),
        sel.is_some(),
    ) {
        st.confirm_delete = true;
    }
    if f.button(
        w / 2.0 + 4.0,
        r2,
        70.0,
        20.0,
        &f.lang.tr("selectServer.refresh"),
        true,
    ) {
        // Пинг серверов — позже (TODO).
    }
    if f.button(
        w / 2.0 + 4.0 + 76.0,
        r2,
        75.0,
        20.0,
        &f.lang.tr("gui.back"),
        true,
    ) {
        *back = true;
    }
    if keys.iter().any(|k| matches!(k, UiKey::Enter)) {
        join = join.or(sel);
    }
    if let Some(i) = join {
        *action = Action::Join(st.servers[i].ip.clone());
    }
}

/// Общая форма: «Прямое подключение» (`direct`) и «Правка сервера».
#[allow(clippy::too_many_arguments)]
fn form(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    now_ms: u64,
    direct: bool,
    back: &mut bool,
    action: &mut Action,
    _next: &mut Option<Screen>,
) {
    let (w, h) = (f.w, f.h);
    let n = if direct { 1 } else { 2 };
    for k in keys {
        match k {
            UiKey::Tab => st.focus = (st.focus + 1) % n,
            UiKey::Backspace => {
                st.fields[st.focus].pop();
            }
            _ => {}
        }
    }
    if st.fields[st.focus].chars().count() < 128 {
        st.fields[st.focus].push_str(typed);
    }
    let caret = (now_ms / 500) % 2 == 0;
    let cx = w / 2.0 - 100.0;
    let (title, y0) = if direct {
        ("selectServer.direct", 116.0)
    } else {
        ("addServer.title", 66.0)
    };
    f.center(title, 17.0, WHITE);
    let ip_idx = n - 1;
    if !direct {
        f.t.draw(&f.lang.tr("addServer.enterName"), cx, 53.0, GRAY, true);
        if f.edit_box(cx, y0, 200.0, 20.0, &st.fields[0], st.focus == 0, caret) {
            st.focus = 0;
        }
    }
    let ip_y = if direct { y0 } else { 106.0 };
    f.t.draw(&f.lang.tr("addServer.enterIp"), cx, ip_y - 13.0, GRAY, true);
    if f.edit_box(
        cx,
        ip_y,
        200.0,
        20.0,
        &st.fields[ip_idx.min(1) * usize::from(!direct)],
        st.focus == ip_idx,
        caret,
    ) {
        st.focus = ip_idx;
    }
    let ip = st.fields[usize::from(!direct)].trim().to_owned();
    let ok = !ip.is_empty();
    let done_key = if direct {
        "selectServer.select"
    } else {
        "addServer.add"
    };
    let by = (h / 4.0).floor() + 96.0 + 18.0;
    let enter = keys.iter().any(|k| matches!(k, UiKey::Enter));
    if f.button(cx, by, 200.0, 20.0, &f.lang.tr(done_key), ok) || (enter && ok) {
        if direct {
            *action = Action::Join(ip);
        } else {
            let name = if st.fields[0].trim().is_empty() {
                f.lang.tr("selectServer.defaultName")
            } else {
                st.fields[0].trim().to_owned()
            };
            let s = Server { name, ip };
            match cur(st) {
                Screen::EditServer(Some(i)) if i < st.servers.len() => st.servers[i] = s,
                _ => st.servers.push(s),
            }
            servers::save(&st.servers_path, &st.servers);
            *back = true;
        }
    }
    if f.button(cx, by + 24.0, 200.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        *back = true;
    }
}

/// VANILLA: OptionsScreen — FOV, «Online», сетка 2×N кнопок, «Done».
/// VANILLA: PauseScreen (Back to Game / Options / Disconnect).
fn pause(f: &mut Frame, next: &mut Option<Screen>, action: &mut Action) {
    let (w, h) = (f.w, f.h);
    f.center("menu.game", 40.0, WHITE);
    let x = w / 2.0 - 102.0;
    let y0 = (h / 4.0).floor() + 24.0;
    if f.button(x, y0, 204.0, 20.0, &f.lang.tr("menu.returnToGame"), true) {
        *action = Action::Resume;
    }
    if f.button(x, y0 + 48.0, 204.0, 20.0, &f.lang.tr("menu.options"), true) {
        *next = Some(Screen::Options);
    }
    if f.button(
        x,
        y0 + 96.0,
        204.0,
        20.0,
        &f.lang.tr("menu.disconnect"),
        true,
    ) {
        *action = Action::Disconnect;
    }
}

fn options_hub(
    f: &mut Frame,
    _st: &mut State,
    opts: &mut Options,
    next: &mut Option<Screen>,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    f.center("options.title", 12.0, WHITE);
    option_row(f, opts, &options::FOV, w / 2.0 - 155.0, 29.0, 150.0);
    f.button(
        w / 2.0 + 5.0,
        29.0,
        150.0,
        20.0,
        &f.lang.tr("options.online"),
        false,
    );
    let items: [(&str, Option<Page>); 10] = [
        ("options.skinCustomisation", Some(Page::Skin)),
        ("options.sounds", Some(Page::Sound)),
        ("options.video", Some(Page::Video)),
        ("options.controls", Some(Page::Controls)),
        ("options.language", Some(Page::Language)),
        ("options.chat", Some(Page::Chat)),
        ("options.resourcepack", Some(Page::ResourcePacks)),
        ("options.accessibility", Some(Page::Accessibility)),
        ("options.telemetry", Some(Page::Telemetry)),
        ("options.credits_and_attribution", Some(Page::Credits)),
    ];
    for (i, (key, pg)) in items.iter().enumerate() {
        let (x, y) = (
            w / 2.0 + if i % 2 == 0 { -154.0 } else { 4.0 },
            65.0 + (i / 2) as f32 * 24.0,
        );
        // VANILLA: «Telemetry» неактивна (нет аккаунта) — как при отключённой телеметрии.
        let active = *pg != Some(Page::Telemetry);
        if f.button(x, y, 150.0, 20.0, &f.lang.tr(key), active) {
            *next = pg.map(Screen::Page);
        }
    }
    if f.button(
        w / 2.0 - 100.0,
        h - 27.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *back = true;
    }
}

/// VANILLA: OptionsSubScreen — шапка 33 px, прокручиваемый список в 2 колонки, «Done».
fn page(f: &mut Frame, st: &mut State, opts: &mut Options, pg: Page, wheel: f32, back: &mut bool) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (33.0, h - 33.0);
    f.list_bg(top, bottom);
    let defs = pg.defs();
    let rows = defs.len().div_ceil(2) as f32;
    let max_scroll = (rows * 24.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    for (i, d) in defs.iter().enumerate() {
        let y = top + 4.0 - st.scroll + (i / 2) as f32 * 24.0;
        if y < top || y + 20.0 > bottom {
            continue;
        }
        let x = w / 2.0 + if i % 2 == 0 { -155.0 } else { 5.0 };
        option_row(f, opts, d, x, y, 150.0);
    }
    if defs.is_empty() {
        f.center_raw("-", h / 2.0 - 4.0, GRAY);
    }
    f.center(pg.title(), 15.0, WHITE);
    if f.button(
        w / 2.0 - 100.0,
        h - 27.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *back = true;
    }
}

/// Значение опции для подписи (VANILLA: OptionInstance#valueSetter/captions).
fn value_text(lang: &Lang, d: &Def, raw: &str) -> String {
    let v: f32 = raw.parse().unwrap_or(0.0);
    match d.kind {
        Kind::Toggle => lang.tr(if raw == "true" {
            "options.on"
        } else {
            "options.off"
        }),
        Kind::Cycle(c) => c
            .iter()
            .find(|(val, _)| *val == raw)
            .map_or_else(|| raw.to_owned(), |(_, k)| lang.tr(k)),
        Kind::Slider(min, max, step) => match d.key {
            "fov" if v == 70.0 => lang.tr("options.fov.min"),
            "fov" if v >= max => lang.tr("options.fov.max"),
            "maxFps" if v >= max => lang.tr("options.framerateLimit.max"),
            "maxFps" => lang.fmt("options.framerate", &[&format!("{v}")]),
            "guiScale" if v < 1.0 => lang.tr("options.guiScale.auto"),
            "renderDistance" => lang.fmt("options.chunks", &[&format!("{v}")]),
            "gamma" if v <= min => lang.tr("options.gamma.min"),
            "gamma" if (v - 0.5).abs() < 0.005 => lang.tr("options.gamma.default"),
            "gamma" if v >= max => lang.tr("options.gamma.max"),
            _ if step >= 1.0 => format!("{}", v.round()),
            _ if d.key.starts_with("soundCategory") && v <= 0.0 => lang.tr("options.off"),
            _ => format!("{}%", (v * 100.0).round()),
        },
    }
}

fn option_row(f: &mut Frame, opts: &mut Options, d: &Def, x: f32, y: f32, w: f32) {
    let raw = opts.get(d).to_owned();
    let caption = f.lang.tr(d.lang);
    let label = f.lang.fmt(
        "options.generic_value",
        &[&caption, &value_text(f.lang, d, &raw)],
    );
    match d.kind {
        Kind::Toggle => {
            if f.button(x, y, w, 20.0, &label, true) {
                opts.set(d.key, (raw != "true").to_string());
            }
        }
        Kind::Cycle(c) => {
            if f.button(x, y, w, 20.0, &label, true) {
                let i = c.iter().position(|(v, _)| *v == raw).map_or(0, |i| i + 1) % c.len();
                opts.set(d.key, c[i].0.to_owned());
            }
        }
        Kind::Slider(min, max, step) => {
            let mut n = ((opts.f32(d) - min) / (max - min)).clamp(0.0, 1.0);
            if f.slider(x, y, w, 20.0, &label, &mut n) {
                let mut v = min + n * (max - min);
                if step > 0.0 {
                    v = (min + ((v - min) / step).round() * step).clamp(min, max);
                }
                opts.set(d.key, options::format_num(v, step));
            }
        }
    }
}
