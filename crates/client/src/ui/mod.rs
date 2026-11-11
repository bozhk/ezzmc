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
    ("menu_bg", "menu_background"),
    ("iw_menu_bg", "inworld_menu_background"),
    ("iw_list_bg", "inworld_menu_list_background"),
    ("iw_header_sep", "inworld_header_separator"),
    ("iw_footer_sep", "inworld_footer_separator"),
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
    Mouse,
    KeyBinds,
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
            Page::Mouse => "options.mouse_settings.title",
            Page::KeyBinds => "controls.keybinds.title",
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
            Page::Controls => options::CONTROLS,
            Page::Mouse => options::MOUSE,
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
    /// Набор ресурспаков изменён — перезагрузить ресурсы (перезапуск процесса).
    ReloadResources,
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
    /// Список доступных ресурспаков (скан `resourcepacks/` при входе на экран).
    packs_avail: Option<Vec<crate::packrepo::PackInfo>>,
    /// Строка поиска, пак на подтверждении (несовместимый), выбор при входе на экран, ввод кадра.
    pack_search: String,
    pack_confirm: Option<String>,
    pack_initial: Option<String>,
    pack_typed: String,
    pack_bs: bool,
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
    /// Строки экрана привязок клавиш (App обновляет, пока `wants_binds`).
    pub binds: Vec<BindRow>,
    /// Привязка, ожидающая нажатия клавиши/кнопки (её ловит App).
    pub listening: Option<&'static str>,
    pub reset_keys: bool,
    pub reset_one: Option<&'static str>,
}

/// Строка экрана привязок. `key` — ключ перевода (`key.keyboard.w`).
pub struct BindRow {
    pub name: &'static str,
    pub category: &'static str,
    pub key: String,
    pub conflict: bool,
    pub default: bool,
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
            Ok("controls") => stack.extend([Screen::Options, Screen::Page(Page::Controls)]),
            Ok("keybinds") => stack.extend([
                Screen::Options,
                Screen::Page(Page::Controls),
                Screen::Page(Page::KeyBinds),
            ]),
            Ok("mouse") => stack.extend([
                Screen::Options,
                Screen::Page(Page::Controls),
                Screen::Page(Page::Mouse),
            ]),
            Ok("packs") => stack.extend([Screen::Options, Screen::Page(Page::ResourcePacks)]),
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
                packs_avail: None,
                pack_search: String::new(),
                pack_confirm: None,
                pack_initial: None,
                pack_typed: String::new(),
                pack_bs: false,
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
            binds: Vec::new(),
            listening: None,
            reset_keys: false,
            reset_one: None,
        }
    }

    pub fn wants_binds(&self) -> bool {
        // Строки нужны и на странице Controls, и на экране самих привязок (иначе они не обновляются).
        matches!(cur(&self.st), Screen::Page(Page::Controls | Page::KeyBinds))
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

    /// Файл брошен в окно (VANILLA: PackSelectionScreen#onFilesDrop) — только на экране паков.
    pub fn on_drop(&mut self, p: &std::path::Path) {
        if cur(&self.st) == Screen::Page(Page::ResourcePacks) && crate::packrepo::import(p) {
            self.st.packs_avail = None;
        }
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
        self.st.packs_avail = None;
    }

    fn pop(&mut self) {
        if self.st.stack.len() > 1 {
            self.st.stack.pop();
        }
        self.st.scroll = 0.0;
        self.st.confirm_delete = false;
        self.st.packs_avail = None;
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
        if cur(&self.st) == Screen::Page(Page::ResourcePacks) {
            let st = &mut self.st;
            if st.packs_avail.is_none() {
                let list = crate::packrepo::scan();
                for pk in &list {
                    if let Some((px, iw, ih)) = &pk.icon {
                        p.add_texture(
                            dev,
                            queue,
                            &format!("packicon:{}", pk.id),
                            px,
                            *iw,
                            *ih,
                            false,
                        );
                    }
                }
                st.packs_avail = Some(list);
            }
            if st.pack_initial.is_none() {
                st.pack_initial = Some(
                    self.opts
                        .raw("resourcePacks")
                        .unwrap_or("[\"vanilla\"]")
                        .to_owned(),
                );
            }
            st.pack_typed = typed.clone();
            st.pack_bs = keys.iter().any(|k| matches!(k, UiKey::Backspace));
        }
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
                in_game: self.in_game,
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
                // VANILLA: Screen#extractMenuBackground — тайл 32×32 (menu_background / inworld_).
                let bg = if self.in_game {
                    "iw_menu_bg"
                } else {
                    "menu_bg"
                };
                f.p.tile(bg, 0.0, 0.0, gw, gh, 32.0, 0xFFFFFFFF);
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
                Screen::Page(Page::KeyBinds) => controls(
                    &mut f,
                    st,
                    &self.binds,
                    &mut self.listening,
                    &mut self.reset_keys,
                    &mut self.reset_one,
                    wheel,
                    &mut back,
                ),
                Screen::Page(Page::ResourcePacks) => {
                    resource_packs(&mut f, st, opts, wheel, &mut back)
                }
                Screen::Page(pg) => page(&mut f, st, opts, pg, wheel, &mut next, &mut back),
            }
            if keys.iter().any(|k| matches!(k, UiKey::Escape))
                && screen != Screen::Title
                && self.listening.is_none()
            {
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
                if screen == Screen::Page(Page::ResourcePacks) {
                    let now = self.opts.raw("resourcePacks").unwrap_or("[\"vanilla\"]");
                    if self.st.pack_initial.as_deref().is_some_and(|i| i != now) {
                        action = Action::ReloadResources;
                    }
                    self.st.pack_initial = None;
                    self.st.pack_search.clear();
                    self.st.pack_confirm = None;
                }
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
    in_game: bool,
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
        if self.hover(x, y, w, h) {
            crate::cursor::request(if active {
                crate::cursor::Kind::PointingHand
            } else {
                crate::cursor::Kind::NotAllowed
            });
        }
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
            // VANILLA: AbstractButton#playDownSound → SimpleSoundInstance.forUI(UI_BUTTON_CLICK, 1.0F).
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
            return true;
        }
        false
    }

    /// VANILLA: AbstractSliderButton. `v` в 0..1; возвращает true, если значение менялось.
    fn slider(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, v: &mut f32) -> bool {
        self.id += 1;
        let hov = self.hover(x, y, w, h);
        if hov || *self.drag == Some(self.id) {
            crate::cursor::request(if *self.drag == Some(self.id) && self.down {
                crate::cursor::Kind::ResizeEw
            } else {
                crate::cursor::Kind::PointingHand
            });
        }
        if hov && self.click {
            self.click = false;
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
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
        // VANILLA: AbstractSelectionList#extractListBackground/Separators (texture 32×32 → ячейка 32).
        let (lb, hs, fs) = if self.in_game {
            ("iw_list_bg", "iw_header_sep", "iw_footer_sep")
        } else {
            ("list_bg", "header_sep", "footer_sep")
        };
        self.p.tile(lb, 0.0, top, w, h, 32.0, 0xFFFFFFFF);
        // Разделители — текстуры 32×2: по вертикали UV 0..1 (обе строки), по горизонтали тайл 32.
        for (tex, y) in [(hs, top - 2.0), (fs, bottom)] {
            let mut cx = 0.0;
            while cx < w {
                let cw = 32.0_f32.min(w - cx);
                self.p
                    .blit(tex, cx, y, cw, 2.0, [0.0, 0.0, cw / 32.0, 1.0], 0xFFFFFFFF);
                cx += 32.0;
            }
        }
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
        if self.hover(x, y, w, h) {
            crate::cursor::request(crate::cursor::Kind::Ibeam);
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
/// VANILLA: KeyBindsScreen/KeyBindsList — категории, кнопка клавиши (75), Reset (50).
#[allow(clippy::too_many_arguments)]
fn controls(
    f: &mut Frame,
    st: &mut State,
    binds: &[BindRow],
    listening: &mut Option<&'static str>,
    reset_all: &mut bool,
    reset_one: &mut Option<&'static str>,
    wheel: f32,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (33.0, h - 33.0);
    f.list_bg(top, bottom);
    // Высота: заголовок категории 20, строка 24.
    let mut total = 4.0;
    let mut last = "";
    for b in binds {
        if b.category != last {
            total += 20.0;
            last = b.category;
        }
        total += 24.0;
    }
    let max_scroll = (total - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    let mut y = top + 4.0 - st.scroll;
    let mut last = "";
    for b in binds {
        if b.category != last {
            last = b.category;
            if y >= top - 10.0 && y + 10.0 <= bottom {
                let s = f.lang.tr(b.category);
                f.center_raw(&s, y + 9.0, WHITE);
            }
            y += 20.0;
        }
        if y >= top && y + 20.0 <= bottom {
            let label = f.lang.tr(b.name);
            let ty = y + 6.0;
            f.t.draw(&label, w / 2.0 - 155.0, ty, WHITE, true);
            if f.button(
                w / 2.0 + 105.0,
                y,
                50.0,
                20.0,
                &f.lang.tr("controls.reset"),
                !b.default,
            ) {
                *reset_one = Some(b.name);
            }
            // VANILLA: InputConstants.Type.KEYBOARD#displayTextSupplier — один символ → заглавная буква.
            let key = match b.key.strip_prefix("key.keyboard.") {
                Some(c) if c.chars().count() == 1 => c.to_uppercase(),
                _ => f.lang.tr(&b.key),
            };
            let label = if *listening == Some(b.name) {
                format!("§e> §f{key}§e <")
            } else if b.conflict {
                format!("§c[ §f{key}§c ]")
            } else {
                key
            };
            if f.button(w / 2.0 + 25.0, y, 75.0, 20.0, &label, true) {
                *listening = Some(b.name);
            }
        }
        y += 24.0;
    }
    f.center("controls.keybinds.title", 15.0, WHITE);
    if f.button(
        w / 2.0 - 155.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("controls.resetAll"),
        true,
    ) {
        *reset_all = true;
    }
    if f.button(
        w / 2.0 + 5.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        *listening = None;
        *back = true;
    }
}

fn page(
    f: &mut Frame,
    st: &mut State,
    opts: &mut Options,
    pg: Page,
    wheel: f32,
    next: &mut Option<Screen>,
    back: &mut bool,
) {
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (33.0, h - 33.0);
    f.list_bg(top, bottom);
    let defs = pg.defs();
    // VANILLA: ControlsScreen — первая строка: Mouse Settings / Key Binds, дальше опции.
    let head = usize::from(pg == Page::Controls) * 2;
    let rows = (defs.len() + head).div_ceil(2) as f32;
    let max_scroll = (rows * 24.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    for (n, (key, target)) in [
        ("options.mouse_settings", Page::Mouse),
        ("controls.keybinds", Page::KeyBinds),
    ]
    .into_iter()
    .take(head)
    .enumerate()
    {
        let x = w / 2.0 + if n == 0 { -155.0 } else { 5.0 };
        let y = top + 4.0 - st.scroll;
        if y >= top && y + 20.0 <= bottom && f.button(x, y, 150.0, 20.0, &f.lang.tr(key), true) {
            *next = Some(Screen::Page(target));
        }
    }
    for (i, d) in defs.iter().enumerate() {
        let c = i + head;
        let y = top + 4.0 - st.scroll + (c / 2) as f32 * 24.0;
        if y < top || y + 20.0 > bottom {
            continue;
        }
        let x = w / 2.0 + if c % 2 == 0 { -155.0 } else { 5.0 };
        option_row(f, opts, d, x, y, 150.0);
    }
    if defs.is_empty() && head == 0 {
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

/// Обрезка строки по ширине с «...».
fn clip(f: &Frame, s: &str, max: f32) -> String {
    if f.t.text_width(s) <= max {
        return s.to_owned();
    }
    let mut o: String = s.to_owned();
    while !o.is_empty() && f.t.text_width(&format!("{o}...")) > max {
        o.pop();
    }
    format!("{o}...")
}

/// VANILLA: PackSelectionScreen — «Available» слева, «Selected» справа (сверху — высший приоритет).
/// Клик по доступному — включить; по выбранному — выключить; ^/v — порядок. Применяется после перезапуска.
fn resource_packs(f: &mut Frame, st: &mut State, opts: &mut Options, wheel: f32, back: &mut bool) {
    use crate::packrepo::{self as pr, PackInfo};
    let (w, h) = (f.w, f.h);
    let (top, bottom) = (64.0, h - 48.0);
    let avail_all = st.packs_avail.clone().unwrap_or_default();
    // Ввод в поле поиска (VANILLA: PackSelectionScreen — EditBox фильтрует оба списка).
    if st.pack_bs {
        st.pack_search.pop();
    }
    if st.pack_search.chars().count() < 64 {
        let typed = std::mem::take(&mut st.pack_typed);
        st.pack_search.push_str(&typed);
    }
    let q = st.pack_search.to_lowercase();
    let matches = |title: &str, desc: &str| {
        q.is_empty() || title.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q)
    };
    let mut sel = pr::parse_selected(opts.raw("resourcePacks").unwrap_or("[\"vanilla\"]"));
    sel.retain(|id| id == pr::VANILLA || avail_all.iter().any(|p| &p.id == id));
    let avail: Vec<&PackInfo> = avail_all
        .iter()
        .filter(|p| !sel.contains(&p.id) && matches(&p.title, &p.desc))
        .collect();
    let vanilla_name = f.lang.tr("resourcePack.vanilla.name");
    let vanilla_desc = f.lang.tr("resourcePack.vanilla.description");
    let shown_sel: Vec<usize> = (0..sel.len())
        .rev()
        .filter(|&i| {
            let id = &sel[i];
            if id == pr::VANILLA {
                matches(&vanilla_name, &vanilla_desc)
            } else {
                avail_all
                    .iter()
                    .find(|p| &p.id == id)
                    .is_some_and(|p| matches(&p.title, &p.desc))
            }
        })
        .collect();
    let modal = st.pack_confirm.is_some();
    let mclick = f.click && modal;
    if modal {
        f.click = false;
    }
    let rows = avail.len().max(shown_sel.len()) as f32;
    let max_scroll = (rows * 36.0 + 4.0 - (bottom - top)).max(0.0);
    st.scroll = (st.scroll - wheel * 12.0).clamp(0.0, max_scroll);
    f.list_bg(top, bottom);
    let (lx, rx, cw) = (w / 2.0 - 205.0, w / 2.0 + 5.0, 200.0);
    let draw_row =
        |f: &mut Frame, x: f32, y: f32, icon: &str, title: &str, desc: &str, red: bool| {
            f.t.fill(x, y, x + cw, y + 32.0, 0x80000000);
            let icon = if f.p.has(icon) { icon } else { "pack_unknown" };
            f.p.blit(icon, x, y, 32.0, 32.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
            f.t.draw(
                &clip(f, title, cw - 36.0 - 28.0),
                x + 36.0,
                y + 3.0,
                WHITE,
                true,
            );
            let col = if red { 0xFF5555 } else { GRAY };
            f.t.draw(&clip(f, desc, cw - 40.0), x + 36.0, y + 15.0, col, true);
        };
    let compat_text = |f: &Frame, p: &PackInfo| match p.compat {
        Some(ezzmc_assets::pack::Compat::TooOld) => (f.lang.tr("pack.incompatible.old"), true),
        Some(ezzmc_assets::pack::Compat::TooNew) => (f.lang.tr("pack.incompatible.new"), true),
        None => (f.lang.tr("pack.incompatible.unknown"), true),
        _ => (p.desc.clone(), false),
    };
    let compatible = |p: &PackInfo| {
        !matches!(
            p.compat,
            Some(ezzmc_assets::pack::Compat::TooOld)
                | Some(ezzmc_assets::pack::Compat::TooNew)
                | None
        )
    };
    let mut new_sel: Option<Vec<String>> = None;
    for (i, p) in avail.iter().enumerate() {
        let y = top + 4.0 - st.scroll + i as f32 * 36.0;
        if y < top || y + 32.0 > bottom {
            continue;
        }
        let (d, red) = compat_text(f, p);
        let src = f.lang.tr("pack.source.local");
        draw_row(
            f,
            lx,
            y,
            &format!("packicon:{}", p.id),
            &f.lang.fmt("pack.nameAndSource", &[&p.title, &src]),
            &d,
            red,
        );
        if !modal && f.hover(lx, y, cw, 32.0) {
            crate::cursor::request(crate::cursor::Kind::PointingHand);
            if f.click {
                f.click = false;
                if compatible(p) {
                    let mut s = sel.clone();
                    s.push(p.id.clone());
                    new_sel = Some(s);
                } else {
                    // VANILLA: PackSelectionScreen — подтверждение несовместимого пака.
                    st.pack_confirm = Some(p.id.clone());
                }
            }
        }
    }
    // Выбранные: сверху — последний в списке (высший приоритет).
    for (n, &idx) in shown_sel.iter().enumerate() {
        let y = top + 4.0 - st.scroll + n as f32 * 36.0;
        if y < top || y + 32.0 > bottom {
            continue;
        }
        let id = sel[idx].clone();
        if id == pr::VANILLA {
            let src = f.lang.tr("pack.source.builtin");
            draw_row(
                f,
                rx,
                y,
                "pack_vanilla",
                &f.lang.fmt("pack.nameAndSource", &[&vanilla_name, &src]),
                &vanilla_desc,
                false,
            );
            continue;
        }
        let Some(p) = avail_all.iter().find(|p| p.id == id) else {
            continue;
        };
        let (d, red) = compat_text(f, p);
        let src = f.lang.tr("pack.source.local");
        draw_row(
            f,
            rx,
            y,
            &format!("packicon:{}", p.id),
            &f.lang.fmt("pack.nameAndSource", &[&p.title, &src]),
            &d,
            red,
        );
        // Кнопки порядка: ^ — выше (к концу списка), v — ниже (но не под vanilla).
        let (bx, by) = (rx + cw - 22.0, y + 1.0);
        let up_ok = idx + 1 < sel.len();
        let down_ok = idx > 1;
        if f.button(bx, by, 10.0, 14.0, "^", up_ok) && up_ok {
            let mut s = sel.clone();
            s.swap(idx, idx + 1);
            new_sel = Some(s);
        }
        if f.button(bx + 11.0, by, 10.0, 14.0, "v", down_ok) && down_ok {
            let mut s = sel.clone();
            s.swap(idx, idx - 1);
            new_sel = Some(s);
        }
        if f.hover(rx, y + 16.0, cw, 16.0) && f.click {
            f.click = false;
            let mut s = sel.clone();
            s.remove(idx);
            new_sel = Some(s);
        }
    }
    // Шапка: заголовок, подсказка про drag-and-drop, поиск, заголовки колонок.
    f.center("resourcePack.title", 8.0, WHITE);
    f.center_raw(&f.lang.tr("pack.dropInfo"), 20.0, GRAY);
    let caret = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis())
        / 500)
        % 2
        == 0;
    if st.pack_search.is_empty() {
        let hint = f.lang.tr("gui.packSelection.search");
        f.t.fill(
            w / 2.0 - 100.0 - 1.0,
            31.0,
            w / 2.0 + 101.0,
            47.0,
            0xFFA0A0A0,
        );
        f.t.fill(w / 2.0 - 100.0, 32.0, w / 2.0 + 100.0, 46.0, 0xFF000000);
        f.t.draw(&hint, w / 2.0 - 96.0, 35.0, 0x707070, true);
    } else {
        f.edit_box(
            w / 2.0 - 100.0,
            32.0,
            200.0,
            14.0,
            &st.pack_search,
            true,
            caret,
        );
    }
    for (x, key) in [(lx, "pack.available.title"), (rx, "pack.selected.title")] {
        let t = f.lang.tr(key);
        let tx = x + (cw - f.t.text_width(&t)) / 2.0;
        f.t.draw(&t, tx, 51.0, WHITE, true);
    }
    // VANILLA: PackSelectionScreen — футер: «Open Pack Folder» (Blaze3D.openPath) + «Done», по 150 px, зазор 8.
    f.center_raw(&f.lang.tr("pack.folderInfo"), h - 40.0, GRAY);
    if f.button(
        w / 2.0 - 154.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("pack.openFolder"),
        !modal,
    ) && !modal
    {
        let dir = pr::dir();
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::process::Command::new("xdg-open").arg(&dir).spawn();
    }
    if f.button(
        w / 2.0 + 4.0,
        h - 27.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        !modal,
    ) && !modal
    {
        *back = true;
    }
    if let Some(id) = st.pack_confirm.clone() {
        // Модальный диалог (VANILLA: ConfirmScreen).
        f.click = mclick;
        f.t.fill(0.0, 0.0, w, h, 0xC0101010);
        let msg_key = match avail_all.iter().find(|p| p.id == id).and_then(|p| p.compat) {
            Some(ezzmc_assets::pack::Compat::TooOld) => "pack.incompatible.confirm.old",
            Some(ezzmc_assets::pack::Compat::TooNew) => "pack.incompatible.confirm.new",
            _ => "pack.incompatible.confirm.unknown",
        };
        f.center("pack.incompatible.confirm.title", h / 2.0 - 40.0, WHITE);
        // Перенос по словам (VANILLA: ConfirmScreen — MultiLineLabel).
        let (msg, mut line, mut ly) = (f.lang.tr(msg_key), String::new(), h / 2.0 - 26.0);
        for word in msg.split(' ') {
            let try_line = if line.is_empty() {
                word.to_owned()
            } else {
                format!("{line} {word}")
            };
            if f.t.text_width(&try_line) > w - 40.0 && !line.is_empty() {
                f.center_raw(&line, ly, WHITE);
                ly += 10.0;
                line = word.to_owned();
            } else {
                line = try_line;
            }
        }
        f.center_raw(&line, ly, WHITE);
        if f.button(
            w / 2.0 - 155.0,
            h / 2.0 + 4.0,
            150.0,
            20.0,
            &f.lang.tr("gui.yes"),
            true,
        ) {
            let mut s = sel.clone();
            s.push(id);
            new_sel = Some(s);
            st.pack_confirm = None;
        } else if f.button(
            w / 2.0 + 5.0,
            h / 2.0 + 4.0,
            150.0,
            20.0,
            &f.lang.tr("gui.no"),
            true,
        ) {
            st.pack_confirm = None;
        }
    }
    if let Some(s) = new_sel {
        opts.set("resourcePacks", pr::encode_selected(&s));
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
        Kind::Log(..) => format!("{v:.2}"),
        Kind::Slider(min, max, step) => match d.key {
            // VANILLA: Options#sensitivity — 0 «*yawn*», 1 «HYPERSPEED!!!», иначе 2·value в процентах.
            "mouseSensitivity" if v <= 0.0 => lang.tr("options.sensitivity.min"),
            "mouseSensitivity" if v >= 1.0 => lang.tr("options.sensitivity.max"),
            "mouseSensitivity" => format!("{}%", (2.0 * v * 100.0).round()),
            "sprintWindow" if v <= 0.0 => lang.tr("options.off"),
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
        Kind::Log(min, max) => {
            let cur = (f64::from(opts.f32(d)).max(1e-3).log10() * 100.0).round() as f32;
            let mut n = ((cur - min as f32) / (max - min) as f32).clamp(0.0, 1.0);
            if f.slider(x, y, w, 20.0, &label, &mut n) {
                let s = (min as f32 + n * (max - min) as f32).round();
                opts.set(d.key, format!("{}", 10f64.powf(f64::from(s) / 100.0)));
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
