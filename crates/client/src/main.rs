mod advancements;
mod adventure;
mod advscreen;
mod ambient;
mod armor;
mod atlasanim;
mod audio;
mod avatar;
mod banner;
mod beacon;
mod beaconscreen;
mod bell;
mod berscan;
mod blocksound;
mod boat;
mod bookview;
mod bossbar;
mod charpreview;
mod chat;
mod chatnet;
mod chest;
mod clipboard;
mod control;
mod control_world;
mod cooldown;
#[allow(dead_code)] // 230: модель готова, в netthread/WorldView ещё не подключена
mod creative_data;
mod creativescreen;
mod cursor;
mod debugscreen;
mod dialog;
mod dig;
mod display;
mod editbox;
mod effects;
mod effectshud;
mod enchbook;
mod enchtable;
mod endportal;
mod entityshadow;
mod flammable;
mod focus;
mod glibc_compat;
mod gmswitcher;
mod handitem;
mod hotbars;
mod hud;
mod hudbars;
mod input;
mod inventory;
mod invscreen;
mod itemdata;
mod itemdyn;
mod itemicon;
mod itemphys;
mod keymap;
mod levelsound;
mod log;
mod loomscreen;
mod maps;
mod merchantscreen;
mod mobs;
mod music;
mod netthread;
mod packrepo;
mod painting;
mod particles;
mod physics;
#[allow(dead_code)] // face/hit/inside и дальности — для 185/186
mod pick;
mod pistonview;
mod placement;
mod playersound;
mod recipebook;
mod recipescreen;
mod sandbox;
mod scoreboard;
mod screenshot;
mod settings;
mod shulkerbox;
mod sign;
mod skins;
mod skull;
mod stats;
mod statsscreen;
mod subtitles;
mod support;
mod tablist;
mod threads;
mod timer;
mod titles;
mod toasts;
mod toolspeed;
mod tooltip;
mod ui;
mod weather;
mod window;
mod winscreen;
mod worldview;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct App {
    hud_state: hudbars::HudState,
    cfg: window::WindowConfig,
    wstate: window::WindowState,
    /// Последние применённые настройки (vsync, fullscreen, ClientInformation).
    applied_vsync: Option<bool>,
    /// Окно закрыто другим окном (Wayland/XWayland не шлёт кадры, Fifo-present блокирует весь игровой цикл).
    occluded: bool,
    applied_fs: Option<bool>,
    applied_compute: Option<bool>,
    /// VANILLA: Window#quitShortcuts (Options#quitShortcuts) + состояние Alt/F4 для onQuitRequested.
    quit_shortcuts: bool,
    allow_cursor: bool,
    alt_down: bool,
    f4_down: bool,
    applied_info: Option<String>,
    window: Option<Arc<Window>>,
    gpu: Option<ezzmc_render::Gpu>,
    swapchain: Option<ezzmc_render::Swapchain>,
    clock: timer::Clock,
    timer: timer::Timer,
    tick_count: u64,
    mouse: input::Mouse,
    input: input::InputState,
    keys: keymap::KeyBindings,
    focus: focus::Focus,
    want_screenshot: bool,
    fps_limit: display::FramerateLimit,
    limiter: display::FrameLimiter,
    yaw: f32,
    title_label: &'static str,
    pitch: f32,
    /// `--connect host:port` — адрес сервера.
    connect: Option<String>,
    player_name: String,
    view: Option<worldview::WorldView>,
    last_ms: u64,
    autoshot_at: Option<u64>,
    /// Отладка: EZZMC_TOUR=1 — облёт тестовой платформы блоков сверху со скриншотом каждого тайла.
    tour: Vec<[f64; 2]>,
    tour_i: usize,
    tour_t: u64,
    /// Множитель скорости свободного полёта (колесо мыши, TODO-455).
    fly_mult: f64,
    autoshot_done: bool,
    text: Option<ezzmc_render::text::TextRenderer>,
    /// Мир грузим после первых показанных кадров: иначе окно не появляется, пока читаются ассеты.
    world_pending: bool,
    frames: u32,
    fps: u32,
    fps_count: u32,
    fps_t0: u64,
    /// F3: экран отладки показан; `f3_pending` — F3 зажата без комбинаций (переключение при отпускании).
    debug_on: bool,
    f3_pending: bool,
    /// F3 физически зажата (комбинации F3+X можно цепочкой).
    f3_held: bool,
    /// F1: HUD скрыт (VANILLA: Hud#toggle / isHidden).
    hud_hidden: bool,
    /// Главное меню (GUI); `None` до загрузки ресурсов.
    gui: Option<ui::Gui>,
    painter: Option<ezzmc_render::gui::GuiPainter>,
    /// Ресурсы игры для ленивой загрузки иконок HUD.
    hud_rm: Option<ezzmc_assets::pack::ResourceManager>,
    item_lang: ui::lang::Lang,
    clipboard: clipboard::Clipboard,
    /// Экран инвентаря/контейнера открыт.
    inv_open: bool,
    /// Экран достижений (VANILLA: AdvancementsScreen); при открытии `inv_open` тоже true.
    adv: Option<advscreen::AdvScreen>,
    /// VANILLA: StatsScreen (TODO-267).
    stats_screen: Option<statsscreen::StatsScreen>,
    /// F3+F4: переключатель режима игры (GameModeSwitcherScreen).
    gms: Option<gmswitcher::GmSwitcher>,
    stats_down: bool,
    /// Экран открыт из паузы — при закрытии вернуть паузу (VANILLA: parent screen).
    screen_from_pause: bool,
    /// Экран смерти (здоровье 0): кнопка «Respawn».
    dead_screen: bool,
    /// Экран концовки (End Poem + титры, TODO-357/358).
    win_screen: Option<winscreen::WinScreen>,
    /// Фон LevelLoadingScreen с причиной END_PORTAL (создаётся при первом показе).
    loading_bg: Option<ezzmc_render::creditsbg::CreditsBackground>,
    /// VANILLA: InBedChatScreen открыт (локальный игрок спит).
    in_bed: bool,
    /// Курсор уже захвачен после входа в мир.
    was_joined: bool,
    /// Поток сборки мира (WorldView::new) — окно не блокируется.
    world_job: Option<std::thread::JoinHandle<Result<worldview::WorldView, String>>>,
    /// Фоновая сборка атласа и моделей блоков (пока игрок в меню): без фриза при входе.
    prewarm: Option<
        std::thread::JoinHandle<
            Option<(
                ezzmc_assets::block_atlas::BlockAtlas,
                ezzmc_mesh::models::BlockModels,
                bool,
            )>,
        >,
    >,
    /// 0 — нет, 1 — «Connecting…», 2 — «Loading terrain…» (ConnectScreen/LevelLoadingScreen).
    connect_phase: u8,
    /// Момент входа в Play (таймаут ожидания чанка).
    loading_since: Option<std::time::Instant>,
    char_preview: Option<charpreview::CharPreview>,
    ench_book: Option<enchbook::EnchBook>,
    anvil: invscreen::AnvilName,
    /// Показан креатив-экран (а не InventoryMenu/контейнер).
    inv_creative: bool,
    creative: creativescreen::State,
    /// Сообщение над хотбаром (VANILLA: Gui#setOverlayMessage, 60 тиков): текст и срок.
    overlay: Option<(Vec<chat::Span>, std::time::Instant)>,
    /// Hud#animateOverlayMessageColor (радужный цвет «Now Playing»).
    overlay_anim: bool,
    chat: chat::Chat,
    chat_cfg: chat::Cfg,
    /// Options#chatDelay (с) и очередь отложенных сообщений чата.
    chat_delay: f64,
    /// VANILLA: ChatVisiblity id (0 full, 1 system, 2 hidden).
    chat_vis: i32,
    chat_pending: std::collections::VecDeque<(
        std::time::Instant,
        ezzmc_protocol::text::Component,
        chat::Tag,
    )>,
    /// Фон не-чатового текста (Options#getBackgroundOpacity).
    sub_bg: f32,
    /// Options#showSubtitles.
    show_subtitles: bool,
    chat_colors: bool,
    chat_links: bool,
    chat_links_prompt: bool,
    /// Открыт серверный контейнер (не InventoryMenu).
    inv_container: bool,
    /// Последний клик по слоту (для двойного клика, 250 мс).
    /// Перетаскивание стека с курсора по слотам (QUICK_CRAFT): (правая кнопка, слоты, активно).
    /// Перетаскивание по слотам: (тип 0 ЛКМ/1 ПКМ/2 СКМ, слоты, началось ли).
    drag: Option<(u8, Vec<usize>, bool)>,
    last_slot_click: Option<(usize, std::time::Instant)>,
    /// Зажатая агентом кнопка мыши в экране инвентаря (right, left, middle).
    held_button: Option<(bool, bool, bool)>,
    /// Курсор в физических пикселях окна.
    cursor_px: (f64, f64),
    /// Меню на экране (мир не показывается).
    menu_active: bool,
    /// Был ли захвачен курсор в прошлом кадре (потеря захвата = открыт экран → releaseAll).
    was_grabbed: bool,
    music: music::MusicManager,
    now_playing: toasts::NowPlaying,
    /// Игрок выбрал сервер в меню: мир грузится в начале следующего кадра.
    join_pending: bool,
    join_delay: u8,
    /// Смена ресурспаков ждёт применения в начале кадра.
    reload_pending: bool,
    /// `--control`: сокет управления для агента (см. `control.rs`).
    control: Option<control::Control>,
    control_pending: Vec<control::Pending>,
    control_task: Option<control_world::Task>,
    shot_reply: Option<std::sync::mpsc::Sender<serde_json::Value>>,
}

impl App {
    /// Курсор и размер экрана в GUI-пикселях: (gw, gh, mx, my).
    fn gui_cursor(&self) -> Option<(f32, f32, f32, f32)> {
        let (w, t) = (self.window.as_ref()?, self.text.as_ref()?);
        let s = window::framebuffer_size(w);
        let (gw, gh) = t.gui_size(s.width, s.height);
        let k = f64::from(t.scale());
        Some((
            gw as f32,
            gh as f32,
            (self.cursor_px.0 / k) as f32,
            (self.cursor_px.1 / k) as f32,
        ))
    }

    /// Центр слота в физических пикселях окна (для управления агентом).
    /// Точка меню (GUI-px от левого верхнего угла) → пиксели окна.
    pub fn menu_px(&self, x: f32, y: f32) -> Option<(f64, f64)> {
        let v = self.view.as_ref()?;
        let (gw, gh, _, _) = self.gui_cursor()?;
        let lay = invscreen::layout(&v.inv);
        let k = f64::from(self.text.as_ref()?.scale());
        Some((
            f64::from(lay.left(gw) + x) * k,
            f64::from(lay.top(gh) + y) * k,
        ))
    }

    pub fn slot_px(&self, slot: usize) -> Option<(f64, f64)> {
        let v = self.view.as_ref()?;
        let (gw, gh, _, _) = self.gui_cursor()?;
        let lay = invscreen::layout(&v.inv);
        let (x, y) = lay.slots.get(slot).copied().flatten()?;
        let k = f64::from(self.text.as_ref()?.scale());
        Some((
            f64::from(lay.left(gw) + x + 8.0) * k,
            f64::from(lay.top(gh) + y + 8.0) * k,
        ))
    }

    pub fn inv_cursor_moved(&mut self) {
        if self.adv.is_some() {
            if let (Some((_, _, mx, my)), Some(v), Some(a)) =
                (self.gui_cursor(), &self.view, &mut self.adv)
            {
                a.cursor(&v.advancements, mx, my);
            }
            return;
        }
        // VANILLA: LoomScreen#mouseDragged — перетаскивание скроллбара.
        if self.inv_open && !self.inv_creative {
            if let (Some((_, gh, _, my)), Some(v)) = (self.gui_cursor(), &mut self.view) {
                if v.inv.open.as_ref().is_some_and(|o| o.scrolling) {
                    let lay = invscreen::layout(&v.inv);
                    loomscreen::drag(&mut v.inv, lay.top(gh), my);
                }
                if v.inv
                    .open
                    .as_ref()
                    .and_then(|o| o.trades.as_ref())
                    .is_some_and(|t| t.dragging)
                {
                    let lay = invscreen::layout(&v.inv);
                    merchantscreen::drag(&mut v.inv, lay.top(gh), my);
                }
            }
        }
        if self.inv_creative && self.inv_open {
            if let Some((gw, gh, mx, my)) = self.gui_cursor() {
                if self.creative.scrolling {
                    self.creative.drag_scroll(gh, my);
                }
                let h = creativescreen::hit(&self.creative, gw, gh, mx, my);
                // VANILLA: mouseDragged — слот добавляется, пока carried.count > slots.size.
                let has_cursor = self
                    .view
                    .as_ref()
                    .and_then(|v| v.inv.cursor.as_ref())
                    .map_or(0, |c| c.count as usize);
                if let (
                    creativescreen::Hit::Slot(creativescreen::Slot::Inv(sl)),
                    Some((ty, slots, active)),
                ) = (h, &mut self.drag)
                {
                    if !slots.contains(&sl) && (*ty == 2 || has_cursor > slots.len()) {
                        slots.push(sl);
                        *active = true;
                    }
                }
            }
            return;
        }
        if self.drag.is_some() && self.inv_open {
            if let (Some((gw, gh, mx, my)), Some(v)) = (self.gui_cursor(), &self.view) {
                let sl = invscreen::layout(&v.inv).slot_at(gw, gh, mx, my);
                if let (Some(sl), Some((_, slots, active))) = (sl, &mut self.drag) {
                    if !slots.contains(&sl)
                        && v.inv
                            .cursor
                            .as_ref()
                            .is_some_and(|c| c.count as usize > slots.len())
                    {
                        slots.push(sl);
                        *active = true;
                    }
                }
            }
        }
    }

    /// Применить клик креатив-экрана и отправить пакеты.
    fn creative_do(&mut self, h: &creativescreen::Hit, button: i8, mode: i32) {
        let Some(v) = &mut self.view else { return };
        let cmds = creativescreen::click(
            &mut self.creative,
            &mut v.inv,
            h,
            button,
            mode,
            &self.item_lang,
        );
        for c in cmds {
            v.send(c);
        }
    }

    /// VANILLA: CreativeModeInventoryScreen / AbstractContainerScreen#mouseClicked/mouseReleased.
    fn creative_mouse(&mut self, pressed: bool, right: bool, left: bool, middle: bool) {
        let Some((gw, gh, mx, my)) = self.gui_cursor() else {
            return;
        };
        if !pressed {
            if left {
                self.creative.scrolling = false;
            }
            if let (Some((r, slots, active)), Some(v)) = (self.drag.take(), &mut self.view) {
                if active && slots.len() > 1 {
                    for c in creativescreen::finish_drag(&mut v.inv, &slots, r) {
                        v.send(c);
                    }
                } else if r == 2 {
                    // VANILLA: СКМ с предметом на курсоре без раздачи — CLONE ничего не делает.
                } else if let Some(&s) = slots.first() {
                    let on = matches!(
                        creativescreen::hit(&self.creative, gw, gh, mx, my),
                        creativescreen::Hit::Slot(creativescreen::Slot::Inv(i)) if i == s
                    );
                    if on || !active {
                        let h = creativescreen::Hit::Slot(creativescreen::Slot::Inv(s));
                        self.creative_do(&h, r as i8, invscreen::PICKUP);
                    }
                }
            }
            return;
        }
        if !(left || right || middle) {
            return;
        }
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        let h = creativescreen::hit(&self.creative, gw, gh, mx, my);
        let inv_slot = match h {
            creativescreen::Hit::Slot(creativescreen::Slot::Inv(i)) => Some(i),
            _ => None,
        };
        let has_cursor = self.view.as_ref().is_some_and(|v| v.inv.cursor.is_some());
        if let (Some(sl), true, false) = (inv_slot, has_cursor, shift) {
            if middle
                || self
                    .last_slot_click
                    .is_none_or(|(b, t)| b != sl || t.elapsed().as_millis() >= 250)
            {
                if !middle {
                    self.last_slot_click = Some((sl, std::time::Instant::now()));
                }
                // VANILLA: mouseClicked — тип раздачи: ЛКМ 0, ПКМ 1, СКМ (pick item) 2.
                self.drag = Some((if middle { 2 } else { u8::from(right) }, vec![sl], false));
                return;
            }
        }
        let now = std::time::Instant::now();
        let double = matches!((inv_slot, self.last_slot_click), (Some(a), Some((b, t)))
                    if a == b && now.duration_since(t).as_millis() < 250);
        self.last_slot_click = if double {
            None
        } else {
            inv_slot.map(|s| (s, now))
        };
        let (button, mode) = if middle {
            (2, creativescreen::CLONE)
        } else if shift {
            (i8::from(right), invscreen::QUICK_MOVE)
        } else if double && left && has_cursor {
            (0, invscreen::PICKUP_ALL)
        } else {
            (i8::from(right), invscreen::PICKUP)
        };
        self.creative_do(&h, button, mode);
    }

    /// Ввод в креатив-экране на вкладке поиска. `true` — событие поглощено.
    /// VANILLA: CreativeModeInventoryScreen#keyPressed/charTyped (цифры над предметом — SWAP).
    fn creative_search_input(&mut self, event: &winit::event::KeyEvent) -> bool {
        if !(self.inv_creative && self.creative.is_search()) {
            return false;
        }
        let PhysicalKey::Code(k) = event.physical_key else {
            return false;
        };
        if k == KeyCode::Escape {
            return false;
        }
        let digit = match k {
            KeyCode::Digit1 => Some(0),
            KeyCode::Digit2 => Some(1),
            KeyCode::Digit3 => Some(2),
            KeyCode::Digit4 => Some(3),
            KeyCode::Digit5 => Some(4),
            KeyCode::Digit6 => Some(5),
            KeyCode::Digit7 => Some(6),
            KeyCode::Digit8 => Some(7),
            KeyCode::Digit9 => Some(8),
            _ => None,
        };
        if let (Some(b), Some((gw, gh, mx, my))) = (digit, self.gui_cursor()) {
            let h = creativescreen::hit(&self.creative, gw, gh, mx, my);
            let cursor_empty = self.view.as_ref().is_some_and(|v| v.inv.cursor.is_none());
            let ok = match h {
                creativescreen::Hit::Slot(creativescreen::Slot::Cell(c)) => {
                    creativescreen::cell_has_item(&self.creative, c)
                }
                creativescreen::Hit::Slot(_) => true,
                _ => false,
            };
            if ok && cursor_empty && !event.repeat {
                self.creative_do(&h, b, invscreen::SWAP);
                return true;
            }
        }
        use creativescreen::Move;
        let ctrl =
            self.input.key_down(KeyCode::ControlLeft) || self.input.key_down(KeyCode::ControlRight);
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        // VANILLA: EditBox#keyPressed — Ctrl+A/C/X/V, стрелки, Home/End, Delete, Backspace.
        match k {
            KeyCode::Backspace => self.creative.erase(-1, ctrl, &self.item_lang),
            KeyCode::Delete => self.creative.erase(1, ctrl, &self.item_lang),
            KeyCode::ArrowLeft => self.creative.move_cursor(Move::Left, ctrl, shift),
            KeyCode::ArrowRight => self.creative.move_cursor(Move::Right, ctrl, shift),
            KeyCode::Home => self.creative.move_cursor(Move::Home, false, shift),
            KeyCode::End => self.creative.move_cursor(Move::End, false, shift),
            KeyCode::KeyA if ctrl => self.creative.select_all(),
            KeyCode::KeyC if ctrl => {
                let t = self.creative.selected_text();
                self.clipboard.set(&t);
            }
            KeyCode::KeyX if ctrl => {
                let t = self.creative.cut(&self.item_lang);
                self.clipboard.set(&t);
            }
            KeyCode::KeyV if ctrl => {
                let t = self.clipboard.get();
                self.creative.type_text(&t, &self.item_lang);
            }
            _ if ctrl => {}
            _ => {
                if let Some(t) = &event.text {
                    self.creative.type_text(t.as_str(), &self.item_lang);
                }
            }
        }
        true
    }

    /// Ввод в поле имени наковальни. `true` — событие поглощено.
    /// VANILLA: AnvilScreen#keyPressed (EditBox получает всё, кроме Esc; E не закрывает экран).
    fn anvil_input(&mut self, event: &winit::event::KeyEvent) -> bool {
        let Some(v) = self.view.as_mut() else {
            return false;
        };
        if !invscreen::layout(&v.inv).is_anvil() {
            return false;
        }
        let Some(o) = v.inv.open.as_ref() else {
            return false;
        };
        if !invscreen::AnvilName::editable(o) {
            return false;
        }
        let PhysicalKey::Code(k) = event.physical_key else {
            return false;
        };
        if k == KeyCode::Escape {
            return false;
        }
        use editbox::Move;
        let ctrl =
            self.input.key_down(KeyCode::ControlLeft) || self.input.key_down(KeyCode::ControlRight);
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        let e = &mut self.anvil.edit;
        let mut changed = false;
        match k {
            KeyCode::Backspace => changed = e.erase(-1, ctrl),
            KeyCode::Delete => changed = e.erase(1, ctrl),
            KeyCode::ArrowLeft => e.move_cursor(Move::Left, ctrl, shift),
            KeyCode::ArrowRight => e.move_cursor(Move::Right, ctrl, shift),
            KeyCode::Home => e.move_cursor(Move::Home, false, shift),
            KeyCode::End => e.move_cursor(Move::End, false, shift),
            KeyCode::KeyA if ctrl => e.select_all(),
            KeyCode::KeyC if ctrl => {
                let t = e.selected_text();
                self.clipboard.set(&t);
            }
            KeyCode::KeyX if ctrl => {
                let t = e.cut();
                self.clipboard.set(&t);
                changed = true;
            }
            KeyCode::KeyV if ctrl => {
                let t = self.clipboard.get();
                changed = e.type_text(&t);
            }
            _ if ctrl => {}
            _ => {
                if let Some(t) = &event.text {
                    changed = e.type_text(t.as_str());
                }
            }
        }
        if changed {
            if let Some(n) = self.anvil.changed(&self.item_lang, o) {
                v.send_rename_item(n);
            }
        }
        true
    }

    /// Ввод в поле поиска книги рецептов. `true` — событие поглощено.
    /// VANILLA: RecipeBookComponent#keyPressed/charTyped (EditBox).
    fn recipe_search_input(&mut self, event: &winit::event::KeyEvent) -> bool {
        let Some(v) = self.view.as_mut() else {
            return false;
        };
        if !(v.recipes.search_focus && invscreen::layout(&v.inv).book) {
            return false;
        }
        let PhysicalKey::Code(k) = event.physical_key else {
            return false;
        };
        if k == KeyCode::Escape {
            return false;
        }
        use editbox::Move;
        let ctrl =
            self.input.key_down(KeyCode::ControlLeft) || self.input.key_down(KeyCode::ControlRight);
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        let e = &mut v.recipes.search;
        let mut changed = false;
        match k {
            KeyCode::Backspace => changed = e.erase(-1, ctrl),
            KeyCode::Delete => changed = e.erase(1, ctrl),
            KeyCode::ArrowLeft => e.move_cursor(Move::Left, ctrl, shift),
            KeyCode::ArrowRight => e.move_cursor(Move::Right, ctrl, shift),
            KeyCode::Home => e.move_cursor(Move::Home, false, shift),
            KeyCode::End => e.move_cursor(Move::End, false, shift),
            KeyCode::KeyA if ctrl => e.select_all(),
            KeyCode::KeyC if ctrl => {
                let t = e.selected_text();
                self.clipboard.set(&t);
            }
            KeyCode::KeyX if ctrl => {
                let t = e.cut();
                self.clipboard.set(&t);
                changed = true;
            }
            KeyCode::KeyV if ctrl => {
                let t = self.clipboard.get();
                changed = e.type_text(&t);
            }
            _ if ctrl => {}
            _ => {
                if let Some(t) = &event.text {
                    changed = e.type_text(t.as_str());
                }
            }
        }
        if changed {
            v.recipes.page = 0;
        }
        true
    }

    /// Открыт локальный предпросмотр модового меню (control `mod_menu_preview`) — только
    /// просмотр: клики и клавиши слотов не меняют инвентарь.
    fn inv_preview(&self) -> bool {
        self.view.as_ref().is_some_and(|v| {
            v.inv
                .open
                .as_ref()
                .is_some_and(|o| o.id == inventory::PREVIEW_CONTAINER)
        })
    }

    pub fn inv_mouse(&mut self, pressed: bool, right: bool, left: bool, middle: bool) {
        if self.inv_creative {
            self.creative_mouse(pressed, right, left, middle);
            return;
        }
        if self.inv_preview() {
            return;
        }
        // VANILLA: BookViewScreen#mouseClicked — кнопки книги и ПКМ по тексту (кафедра).
        if let (Some((gw, _, mx, my)), Some(v), Some(t)) =
            (self.gui_cursor(), &self.view, self.text.as_ref())
        {
            if invscreen::layout(&v.inv).is_lectern() {
                if pressed && (right || left) {
                    if let Some(o) = &v.inv.open {
                        let may_build = v.game_mode < 2;
                        match bookview::click(t, &self.item_lang, o, gw, mx, my, right, may_build) {
                            bookview::Act::Pass => {}
                            bookview::Act::Send(c) => v.send(c),
                            bookview::Act::Close(cmd) => {
                                if let Some(c) = cmd {
                                    v.send(netthread::NetCmd::Command(
                                        c.strip_prefix('/').unwrap_or(&c).to_owned(),
                                    ));
                                }
                                self.close_inv(true);
                            }
                        }
                    }
                }
                return;
            }
        }
        if !pressed {
            if let Some(v) = &mut self.view {
                loomscreen::release(&mut v.inv);
                merchantscreen::release(&mut v.inv);
            }
            // VANILLA: AbstractContainerScreen#mouseReleased — конец перетаскивания или отложенный клик.
            let gc = self.gui_cursor();
            if let (Some((ty, slots, active)), Some(v)) = (self.drag.take(), &mut self.view) {
                if active && slots.len() > 1 {
                    v.quick_craft(&slots, ty);
                } else if let (Some(&s), Some((gw, gh, mx, my))) = (slots.first(), gc) {
                    let lay = invscreen::layout(&v.inv);
                    if lay.slot_at(gw, gh, mx, my) == Some(s) || !active {
                        v.click_slot(s as i16, i8::from(right), invscreen::PICKUP);
                    }
                }
            }
        }
        if pressed && (right || left) {
            let shift =
                self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
            if let (Some((gw, gh, mx, my)), Some(v)) = (self.gui_cursor(), &mut self.view) {
                let lay = invscreen::layout(&v.inv);
                let slot = lay.slot_at(gw, gh, mx, my);
                if let (Some(sl), true, false) = (slot, v.inv.cursor.is_some(), shift) {
                    if self
                        .last_slot_click
                        .is_none_or(|(b, t)| b != sl || t.elapsed().as_millis() >= 250)
                    {
                        self.last_slot_click = Some((sl, std::time::Instant::now()));
                        self.drag = Some((u8::from(right), vec![sl], false));
                        return;
                    }
                }
                // VANILLA: RecipeBookComponent#slotClicked — клик по результату/сетке гасит призрак.
                if slot.is_some_and(|s| {
                    s <= match lay.book_kind() {
                        Some(recipebook::BookKind::Crafting) if lay.is_player() => 4,
                        Some(recipebook::BookKind::Crafting) => 9,
                        Some(_) => 2,
                        None => 0,
                    }
                }) {
                    v.ghost.clear();
                }
                let now = std::time::Instant::now();
                let double = matches!((slot, self.last_slot_click), (Some(a), Some((b, t)))
                            if a == b && now.duration_since(t).as_millis() < 250);
                self.last_slot_click = if double { None } else { slot.map(|s| (s, now)) };
                // VANILLA: RecipeBookComponent#mouseClicked — книга и её кнопка раньше слотов.
                match recipescreen::click(
                    &mut v.recipes,
                    &v.inv,
                    &lay,
                    gw,
                    gh,
                    mx,
                    my,
                    left,
                    right,
                    shift,
                ) {
                    recipescreen::Act::Toggle => {
                        v.toggle_crafting_book(
                            lay.book_kind().unwrap_or(recipebook::BookKind::Crafting),
                        );
                        return;
                    }
                    recipescreen::Act::Filter => {
                        let k = lay.book_kind().unwrap_or(recipebook::BookKind::Crafting);
                        let f = &mut v.recipes.settings.of_mut(k).filtering;
                        *f = !*f;
                        v.send_book_settings(k);
                        return;
                    }
                    recipescreen::Act::Place(id, max) => {
                        v.ghost.clear();
                        v.send_place_recipe(id, max);
                        return;
                    }
                    recipescreen::Act::Handled => return,
                    recipescreen::Act::None => {}
                }
                // VANILLA: LoomScreen#mouseClicked — выбор узора, захват скроллбара.
                if lay.is_loom() && !right {
                    let mut out = Vec::new();
                    let act =
                        loomscreen::click(&mut v.inv, &mut out, lay.left(gw), lay.top(gh), mx, my);
                    for c in out {
                        v.send(c);
                    }
                    if matches!(act, loomscreen::Act::Handled) {
                        return;
                    }
                }
                // VANILLA: MerchantScreen — строки предложений (SelectTrade) и скроллбар.
                if lay.is_merchant() && !right {
                    let mut out = Vec::new();
                    let act = merchantscreen::click(
                        &mut v.inv,
                        &mut out,
                        lay.left(gw),
                        lay.top(gh),
                        mx,
                        my,
                    );
                    for c in out {
                        v.send(c);
                    }
                    if matches!(act, merchantscreen::Act::Handled) {
                        return;
                    }
                }
                // VANILLA: BeaconScreen — кнопки эффектов/подтверждения/отмены (левая кнопка).
                if lay.is_beacon() && !right {
                    match beaconscreen::click(&mut v.inv, lay.left(gw), lay.top(gh), mx, my) {
                        beaconscreen::Act::Pass => {}
                        beaconscreen::Act::Handled => return,
                        beaconscreen::Act::Close(c) => {
                            if let Some(c) = c {
                                v.send(c);
                            }
                            self.close_inv(true);
                            return;
                        }
                    }
                }
                if let Some(c) =
                    invscreen::on_click(&v.inv, &lay, gw, gh, mx, my, right, shift, double)
                {
                    if let netthread::NetCmd::ContainerClick {
                        slot,
                        button,
                        input,
                        ..
                    } = &c
                    {
                        let (slot, button, input) = (*slot, *button, *input);
                        v.click_slot(slot, button, input);
                    } else {
                        v.send(c);
                    }
                }
            }
        }
    }

    /// VANILLA: Esc в игре — меню паузы (курсор освобождается).
    pub(crate) fn open_pause_menu(&mut self) {
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
        if self.view.is_some() {
            if let Some(g) = &mut self.gui {
                g.open_pause();
                self.menu_active = true;
                self.input.release_all();
                self.keys.release_all();
            }
        }
    }

    /// VANILLA: захват мыши при входе в мир; экран смерти при health<=0 (DeathScreen).
    fn sync_screens(&mut self) {
        let Some(v) = &self.view else {
            self.was_joined = false;
            self.dead_screen = false;
            self.connect_phase = 0;
            return;
        };
        let (joined, dead) = (v.joined, v.health <= 0.0 || self.win_screen.is_some());
        // VANILLA: ConnectScreen → LevelLoadingScreen (LevelLoadTracker#isLevelReady: чанк игрока
        // принят; VANILLA-UNSURE: таймаут 30 с как у LevelLoadStatusManager).
        // VANILLA: WIN_GAME заменяет LevelLoadingScreen на WinScreen (сервер чанков Энда уже не шлёт).
        let ready = joined
            && (self.win_screen.is_some()
                || v.win_game
                || !v.respawn_wait
                    && v.net.level.lock().ok().is_some_and(|g| {
                        g.as_ref().is_some_and(|l| {
                            l.chunks
                                .get(
                                    (v.cam[0] / 16.0).floor() as i32,
                                    (v.cam[2] / 16.0).floor() as i32,
                                )
                                .is_some()
                        })
                    })
                || self
                    .loading_since
                    .is_some_and(|t| t.elapsed() >= std::time::Duration::from_secs(30)));
        // Экран подключения открыт кнопкой «Join» ещё до создания мира — продолжаем фазы с него.
        if self.connect_phase == 0 && self.gui.as_ref().is_some_and(|g| g.is_connecting()) {
            self.connect_phase = 1;
        }
        if v.status.is_none() {
            let want = if !joined {
                1
            } else if self.connect_phase != 0 && !ready {
                2
            } else {
                0
            };
            if want != self.connect_phase && (want != 0 || self.connect_phase != 0) {
                if let Some(g) = &mut self.gui {
                    match want {
                        1 | 2 => {
                            if want == 1 {
                                g.open_connecting();
                            } else {
                                g.open_loading();
                                self.loading_since = Some(std::time::Instant::now());
                            }
                            self.menu_active = true;
                            self.input.release_all();
                            self.keys.release_all();
                            if let Some(w) = &self.window {
                                self.mouse.release(w);
                            }
                        }
                        _ => {
                            if g.is_connecting() {
                                self.menu_active = false;
                            }
                            self.loading_since = None;
                        }
                    }
                    self.connect_phase = want;
                }
            }
        }
        // VANILLA: TooltipFlag#isCreative — режим игрока для hide_tooltip (покадрово, не только при открытом инвентаре).
        tooltip::CREATIVE.store(v.game_mode == 1, std::sync::atomic::Ordering::Relaxed);
        // Захват может не удаться, пока окно без фокуса (Wayland) — повторяем каждый кадр,
        // пока не получится (после этого `grabbed` и ручное освобождение работают как раньше).
        if joined
            && !self.menu_active
            && !self.inv_open
            && !self.chat.open
            && !dead
            && !self.mouse.grabbed
        {
            if let Some(w) = &self.window {
                // Захват только с фокусом: без него XGrabPointer (XWayland) блокирует поток навсегда;
                // при получении фокуса захват делает обработчик Focused(true).
                if w.has_focus() && self.focus.active {
                    self.mouse.grab(w);
                }
            }
        }
        self.was_joined = joined;
        if joined && !self.menu_active && !self.inv_open && !self.chat.open && !dead {
            if let Some(w) = &self.window {
                self.mouse.maintain(w);
            }
        }
        // VANILLA: handlePlayerCombatKill → DeathScreen; handleRespawn → закрыть.
        let (hardcore, score) = (v.hardcore, v.score);
        self.sync_in_bed();
        let opened = self.view.as_mut().and_then(|v| v.death_in.take());
        if self
            .view
            .as_mut()
            .is_some_and(|v| std::mem::take(&mut v.win_game))
            && self.win_screen.is_none()
        {
            if let Some(rm) = &self.hud_rm {
                let pname = self.player_name.clone();
                tracing::info!("WinScreen: открыт (WIN_GAME)");
                let mut ws = winscreen::WinScreen::load(rm, &pname, true);
                if let (Some(g), Some(sc)) = (&self.gpu, &self.swapchain) {
                    ws.init_background(rm, &g.device, &g.queue, sc.config.format);
                }
                self.win_screen = Some(ws);
                // VANILLA: Gui#setScreen закрывает прежний экран (инвентарь/чат), ввод отпускается.
                if self.inv_open {
                    self.close_inv(true);
                }
                self.chat.open = false;
                self.input.release_all();
                self.keys.release_all();
                if let Some(w) = &self.window {
                    self.mouse.release(w);
                }
            }
        }
        let respawned = self
            .view
            .as_mut()
            .is_some_and(|v| std::mem::take(&mut v.respawn_in));
        if let (Some(cause), Some(g)) = (opened, &mut self.gui) {
            g.open_death(cause.as_ref(), hardcore, score);
            self.dead_screen = true;
            self.menu_active = true;
            if self.inv_open {
                self.close_inv(true);
            }
            self.chat.open = false;
            self.input.release_all();
            self.keys.release_all();
            if let Some(w) = &self.window {
                self.mouse.release(w);
            }
        } else if respawned && self.dead_screen {
            if let Some(g) = &mut self.gui {
                g.close_death();
            }
            self.dead_screen = false;
            self.menu_active = false;
            if let Some(w) = &self.window {
                self.mouse.grab(w);
            }
        }
        // VANILLA: handleRespawn → startWaitingForNewLevel: LevelLoadingScreen с причиной
        // (смерть — OTHER, Нижний мир/Энд — фон портала); закрывается, когда чанк игрока принят.
        let loading = self.view.as_mut().and_then(|v| v.loading_in.take());
        if let Some(reason) = loading.filter(|_| self.gui.is_some()) {
            tracing::info!("LevelLoadingScreen: respawn, причина {reason:?}");
            if self.inv_open {
                self.close_inv(true);
            }
            if let Some(g) = &mut self.gui {
                g.open_loading_reason(reason);
            }
            if self.connect_phase != 2 {
                self.loading_since = Some(std::time::Instant::now());
            }
            self.connect_phase = 2;
            self.menu_active = true;
            self.chat.open = false;
            self.input.release_all();
            self.keys.release_all();
            if let Some(w) = &self.window {
                self.mouse.release(w);
            }
        }
    }

    /// VANILLA: Gui#tick — спящий игрок получает InBedChatScreen; после пробуждения экран закрывается
    /// (с текстом в поле — остаётся обычный чат).
    /// VANILLA: WinScreen#keyPressed/keyReleased + Screen#keyPressed (Esc → onClose → respawn).
    fn win_key(&mut self, k: KeyCode, pressed: bool) {
        let Some(ws) = &mut self.win_screen else {
            return;
        };
        match k {
            KeyCode::Escape if pressed => {
                tracing::info!("WinScreen: Esc → respawn");
                self.win_screen = None;
                if let Some(v) = &self.view {
                    v.send_respawn();
                }
                if let Some(w) = &self.window {
                    self.mouse.grab(w);
                }
            }
            KeyCode::Space => ws.speedup = pressed,
            KeyCode::ControlLeft => ws.ctrl[0] = pressed,
            KeyCode::ControlRight => ws.ctrl[1] = pressed,
            KeyCode::ArrowUp => ws.reverse = pressed,
            _ => {}
        }
    }

    fn sync_in_bed(&mut self) {
        let sleeping = self.view.as_ref().is_some_and(|v| v.sleeping);
        if sleeping && !self.in_bed && !self.menu_active && !self.inv_open && !self.dead_screen {
            self.in_bed = true;
            self.open_chat("");
        } else if !sleeping && self.in_bed {
            self.in_bed = false;
            if self.chat.open && self.chat.edit.text.is_empty() {
                self.close_chat();
            }
        }
    }

    /// VANILLA: InBedChatScreen#sendWakeUp.
    fn stop_sleeping(&self) {
        if let Some(v) = &self.view {
            v.send(netthread::NetCmd::StopSleeping);
        }
    }

    /// VANILLA: InBedChatScreen — кнопка «multiplayer.stopSleeping» (width/2−100, height−40, 200×20).
    fn leave_bed_rect(gw: f32, gh: f32) -> (f32, f32, f32, f32) {
        (gw / 2.0 - 100.0, gh - 40.0, 200.0, 20.0)
    }

    /// VANILLA: Minecraft#openChatScreen — key.chat (пусто) / key.command ("/").
    fn open_chat(&mut self, prefill: &str) {
        if self.view.is_none() {
            return;
        }
        self.chat.show(prefill);
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    fn close_chat(&mut self) {
        self.chat.close();
        if let Some(w) = &self.window {
            self.mouse.grab(w);
        }
    }

    /// VANILLA: ChatScreen#handleChatInput — normalize, история, `/` → команда.
    fn chat_submit(&mut self) {
        let msg = chat::normalize(&self.chat.edit.text);
        if !msg.is_empty() {
            self.chat.hist.add(&msg);
            if let Some(v) = &self.view {
                // VANILLA: ChatAbilities#canSendMessages/canSendCommands — SYSTEM: только команды, HIDDEN: ничего.
                match msg.strip_prefix('/') {
                    Some(c) if self.chat_vis < 2 => {
                        v.send(netthread::NetCmd::Command(c.to_owned()))
                    }
                    None if self.chat_vis < 1 => v.send(netthread::NetCmd::Chat(msg)),
                    _ => {}
                }
            }
        }
        self.close_chat();
    }

    /// Клавиши ChatScreen. VANILLA: ChatScreen#keyPressed, EditBox#keyPressed.
    fn chat_key(&mut self, event: &winit::event::KeyEvent) {
        let PhysicalKey::Code(k) = event.physical_key else {
            return;
        };
        let ctrl =
            self.input.key_down(KeyCode::ControlLeft) || self.input.key_down(KeyCode::ControlRight);
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        use editbox::Move;
        let cfg = self.chat_cfg;
        match k {
            KeyCode::Escape | KeyCode::Tab | KeyCode::ArrowUp | KeyCode::ArrowDown
                if self.chat.sg.shown
                    && self.chat.sg.key(
                        match k {
                            KeyCode::Escape => chat::SuggKey::Esc,
                            KeyCode::Tab => chat::SuggKey::Tab,
                            KeyCode::ArrowUp => chat::SuggKey::Up,
                            _ => chat::SuggKey::Down,
                        },
                        shift,
                        &mut self.chat.edit,
                    ) => {}
            KeyCode::Tab => {
                // VANILLA: CommandSuggestions#keyPressed(Tab) → showSuggestions(true) при autoSuggestions=false.
                self.chat.sg.force = true;
                self.chat.sg.seen_reset();
            }
            KeyCode::Escape if self.in_bed => self.stop_sleeping(),
            KeyCode::Escape => self.close_chat(),
            KeyCode::Enter | KeyCode::NumpadEnter => self.chat_submit(),
            KeyCode::ArrowUp | KeyCode::ArrowDown => {
                let d = if k == KeyCode::ArrowUp { -1 } else { 1 };
                let cur = self.chat.edit.text.clone();
                if let Some(t) = self.chat.hist.step(d, &cur) {
                    self.chat.edit.set(&t);
                }
            }
            KeyCode::PageUp => self.chat.page(true, &cfg),
            KeyCode::PageDown => self.chat.page(false, &cfg),
            KeyCode::Backspace => {
                self.chat.edit.erase(-1, ctrl);
            }
            KeyCode::Delete => {
                self.chat.edit.erase(1, ctrl);
            }
            KeyCode::ArrowLeft => self.chat.edit.move_cursor(Move::Left, ctrl, shift),
            KeyCode::ArrowRight => self.chat.edit.move_cursor(Move::Right, ctrl, shift),
            KeyCode::Home => self.chat.edit.move_cursor(Move::Home, false, shift),
            KeyCode::End => self.chat.edit.move_cursor(Move::End, false, shift),
            KeyCode::KeyA if ctrl => self.chat.edit.select_all(),
            KeyCode::KeyC if ctrl => {
                let t = self.chat.edit.selected_text();
                self.clipboard.set(&t);
            }
            KeyCode::KeyX if ctrl => {
                let t = self.chat.edit.cut();
                self.clipboard.set(&t);
            }
            KeyCode::KeyV if ctrl => {
                let t = self.clipboard.get();
                self.chat.edit.type_text(&t);
            }
            _ if ctrl => {}
            _ => {
                if let Some(t) = &event.text {
                    self.chat.edit.type_text(t.as_str());
                }
            }
        }
    }

    /// ЛКМ по тексту чата. VANILLA: ChatScreen#mouseClicked → handleComponentClicked.
    fn chat_click(&mut self) {
        use ezzmc_protocol::text::ClickEvent;
        let (Some((gw, gh, mx, my)), Some(t)) = (self.gui_cursor(), self.text.as_ref()) else {
            return;
        };
        let cfg = self.chat_cfg;
        if self.in_bed {
            let (x, y, w, h) = Self::leave_bed_rect(gw, gh);
            if mx >= x && mx < x + w && my >= y && my < y + h {
                self.stop_sleeping();
                return;
            }
        }
        if self.chat.sg.click(mx, my, &mut self.chat.edit) {
            return;
        }
        let Some(span) = self.chat.span_at(t, gh, mx, my, &cfg) else {
            return;
        };
        let shift =
            self.input.key_down(KeyCode::ShiftLeft) || self.input.key_down(KeyCode::ShiftRight);
        if shift {
            if let Some(ins) = &span.insertion {
                self.chat.edit.type_text(ins);
            }
            return;
        }
        match span.click {
            Some(ClickEvent::RunCommand(c)) => {
                if let Some(v) = &self.view {
                    v.send(netthread::NetCmd::Command(
                        c.strip_prefix('/').unwrap_or(&c).to_owned(),
                    ));
                }
                self.close_chat();
            }
            Some(ClickEvent::SuggestCommand(c)) => self.chat.edit.set(&c),
            Some(ClickEvent::CopyToClipboard(c)) => self.clipboard.set(&c),
            Some(ClickEvent::OpenUrl(u))
                if self.chat_links && (u.starts_with("http://") || u.starts_with("https://")) =>
            {
                if self.chat_links_prompt {
                    self.close_chat();
                    if let Some(g) = &mut self.gui {
                        g.open_link_confirm(u);
                    }
                } else {
                    let _ = std::process::Command::new("xdg-open").arg(u).spawn();
                }
            }
            _ => {}
        }
    }

    fn open_inv(&mut self) {
        let Some(v) = &self.view else { return };
        self.inv_container = v.inv.open.is_some();
        self.inv_creative = v.game_mode == 1 && !self.inv_container;
        // VANILLA: Player#canUseGameMasterBlocks (instabuild && права ≥ GAMEMASTER=2) && options.operatorItemsTab.
        let op_opt = self.gui.as_ref().is_some_and(|g| {
            ui::options::CONTROLS
                .iter()
                .find(|d| d.key == "operatorItemsTab")
                .is_some_and(|d| g.opts.get(d) == "true")
        });
        let op_on = op_opt && v.game_mode == 1 && v.op_level >= 2;
        self.creative.set_op_tab(op_on, &self.item_lang);
        self.inv_open = true;
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    /// VANILLA: Minecraft#handleKeybinds — key.advancements → AdvancementsScreen.
    fn open_adv(&mut self) {
        if self.view.is_none() || self.adv.is_some() {
            return;
        }
        self.adv = Some(advscreen::AdvScreen::default());
        self.inv_open = true;
        self.inv_container = false;
        self.inv_creative = false;
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    /// VANILLA: Minecraft#handleKeybinds — key.socialInteractions → SocialInteractionsScreen.
    pub(crate) fn open_social(&mut self) {
        let (Some(v), Some(g)) = (&self.view, &mut self.gui) else {
            return;
        };
        let me = v
            .tab
            .entries
            .iter()
            .find(|(_, e)| e.name == v.own_name)
            .map_or(0, |(u, _)| *u);
        g.open_social(me, self.connect.clone().unwrap_or_default());
        self.menu_active = true;
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    /// VANILLA: AdvancementsScreen#removed → ServerboundSeenAdvancementsPacket(closedScreen).
    fn close_adv(&mut self) {
        if self.adv.take().is_none() {
            return;
        }
        self.inv_open = false;
        if let Some(v) = &self.view {
            v.send(netthread::NetCmd::SeenAdvancements(None));
        }
        if std::mem::take(&mut self.screen_from_pause) {
            self.menu_active = true;
        }
    }

    /// VANILLA: PauseScreen → StatsScreen; ServerboundClientCommand(REQUEST_STATS) при открытии.
    pub(crate) fn open_stats(&mut self) {
        if self.view.is_none() || self.stats_screen.is_some() {
            return;
        }
        if let Some(v) = &mut self.view {
            v.stats = Default::default();
            v.send(netthread::NetCmd::RequestStats);
        }
        self.stats_screen = Some(statsscreen::StatsScreen::default());
        self.inv_open = true;
        self.inv_container = false;
        self.inv_creative = false;
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    /// VANILLA: KeyboardHandler — F3+F4: открыть переключатель (нужен op ≥ 2) или выбрать следующий режим.
    pub(crate) fn gms_key(&mut self) {
        if let Some(g) = &mut self.gms {
            g.next();
            return;
        }
        let Some(v) = &mut self.view else { return };
        if self.menu_active || self.inv_open || self.chat.open {
            return;
        }
        if v.op_level < 2 {
            v.debug_feedback("No permission to open the game mode switcher");
            return;
        }
        self.gms = Some(gmswitcher::GmSwitcher::new(v.prev_game_mode, v.game_mode));
        self.inv_open = true;
        self.inv_container = false;
        self.inv_creative = false;
        self.input.release_all();
        self.keys.release_all();
        if let Some(w) = &self.window {
            self.mouse.release(w);
        }
    }

    /// Отпускание F3: применить выбранный режим и закрыть переключатель.
    pub(crate) fn close_gms(&mut self) {
        let Some(g) = self.gms.take() else { return };
        self.inv_open = false;
        if let Some(v) = &mut self.view {
            let m = g.selected_mode();
            if m != v.game_mode && v.op_level >= 2 {
                v.send(netthread::NetCmd::ChangeGameMode(m));
            }
        }
        if let Some(w) = &self.window {
            self.mouse.grab(w);
        }
    }

    pub(crate) fn close_stats(&mut self) {
        if self.stats_screen.take().is_none() {
            return;
        }
        self.inv_open = false;
        self.stats_down = false;
        self.back_to_pause();
    }

    /// Экран открыт из паузы — вернуться в неё.
    fn back_to_pause(&mut self) {
        if std::mem::take(&mut self.screen_from_pause) {
            self.menu_active = true;
        } else if let Some(w) = &self.window {
            self.mouse.grab(w);
        }
    }

    /// VANILLA: Screen#onClose → ServerboundContainerClosePacket (`notify` — закрыл игрок, не сервер).
    fn close_inv(&mut self, notify: bool) {
        self.inv_open = false;
        if let Some(v) = &mut self.view {
            if notify {
                v.send_container_close();
            }
            v.inv.open = None;
            v.ghost.clear();
            v.recipes.overlay = None;
            // VANILLA: ItemPickerMenu#getCarried — курсор живёт в InventoryMenu; клиентское
            // removed() его не трогает, поэтому после закрытия креатив-экрана предмет остаётся.
        }
        if let Some(w) = &self.window {
            self.mouse.grab(w);
        }
    }

    /// Цифровая клавиша хотбара в мире. VANILLA: Minecraft#handleKeybinds — в креативе с зажатым
    /// C/X загрузка и сохранение набора вместо выбора слота (X важнее C).
    fn hotbar_key(&mut self, n: usize) {
        let down = |a: &Self, name: &str| a.keys.get(name).is_some_and(|b| b.is_down());
        let save = down(self, "key.saveToolbarActivator");
        let load = down(self, "key.loadToolbarActivator");
        let Some(v) = &mut self.view else { return };
        if v.game_mode != 1 || !(save || load) {
            v.select_slot(n as u8);
            return;
        }
        // VANILLA: CreativeModeInventoryScreen#handleHotbarLoadOrSave.
        if load {
            let items = self.creative.hotbars.get(n).to_vec();
            for (i, s) in items.into_iter().enumerate() {
                v.inv.slots[crate::inventory::HOTBAR_START + i] = s.clone();
                v.send(netthread::NetCmd::CreativeSlot(
                    (crate::inventory::HOTBAR_START + i) as i16,
                    s,
                ));
            }
        } else {
            let hb: Vec<_> = v.inv.hotbar().to_vec();
            self.creative.hotbars.store(n, &hb);
            let label = |a: &Self, name: &str| {
                a.keys
                    .get(name)
                    .map_or_else(String::new, |b| a.item_lang.tr(&b.key.name()))
            };
            let msg = self
                .item_lang
                .tr("inventory.hotbarSaved")
                .replacen("%1$s", &label(self, "key.loadToolbarActivator"), 1)
                .replacen("%2$s", &label(self, &format!("key.hotbar.{}", n + 1)), 1);
            self.overlay = Some((
                vec![chat::Span::plain(msg)],
                std::time::Instant::now() + std::time::Duration::from_secs(3),
            ));
            self.overlay_anim = false;
        }
    }

    /// Клавиши экрана инвентаря: цифры/F над слотом — SWAP, Q — THROW.
    fn inv_key(&mut self, k: KeyCode) {
        let Some((gw, gh, mx, my)) = self.gui_cursor() else {
            return;
        };
        if self.inv_preview() {
            return;
        }
        let is = |name: &str| {
            self.keys
                .get(name)
                .is_some_and(|b| b.key == keymap::Input::Key(k))
        };
        let swap_btn = (1..=9)
            .find(|i| is(&format!("key.hotbar.{i}")))
            .map(|i| i as i8 - 1)
            .or(is("key.swapOffhand").then_some(40));
        let drop = is("key.drop");
        let ctrl = self.input.key_down(KeyCode::ControlLeft);
        if self.inv_creative {
            // VANILLA: key.chat на вкладке не-поиска открывает поиск.
            if is("key.chat") && !self.creative.is_search() {
                self.creative
                    .select_tab(creativescreen::SEARCH_TAB, &self.item_lang);
                return;
            }
            let h = creativescreen::hit(&self.creative, gw, gh, mx, my);
            let creativescreen::Hit::Slot(sl) = h else {
                return;
            };
            if let creativescreen::Slot::Cell(c) = sl {
                if !creativescreen::cell_has_item(&self.creative, c) {
                    return;
                }
            }
            let cursor_empty = self.view.as_ref().is_some_and(|v| v.inv.cursor.is_none());
            // VANILLA: AbstractContainerScreen#checkHotbarKeyPressed — только при пустом курсоре.
            if let Some(b) = swap_btn {
                if cursor_empty {
                    self.creative_do(&h, b, invscreen::SWAP);
                }
            } else if drop {
                self.creative_do(&h, i8::from(ctrl), invscreen::THROW);
            }
            return;
        }
        let Some(v) = &mut self.view else { return };
        let lay = invscreen::layout(&v.inv);
        let Some(slot) = lay.slot_at(gw, gh, mx, my) else {
            return;
        };
        // VANILLA: предсказание + заявка изменённых слотов (как в клике мышью).
        if let Some(b) = swap_btn {
            v.click_slot(slot as i16, b, invscreen::SWAP);
        } else if drop {
            v.click_slot(slot as i16, i8::from(ctrl), invscreen::THROW);
        }
    }

    /// Записывает привязки клавиш в options.txt (`key_<name>`).
    /// VANILLA: KeyBindsScreen#keyPressed/mouseClicked — ожидающая строка получает нажатую клавишу (Esc = Unbound).
    fn capture_bind(&mut self, input: keymap::Input) -> bool {
        let Some(n) = self.gui.as_mut().and_then(|g| g.listening.take()) else {
            return false;
        };
        let inp = if input == keymap::Input::Key(KeyCode::Escape) {
            keymap::Input::Unbound
        } else {
            input
        };
        self.keys.rebind(n, inp);
        self.sync_keys();
        true
    }

    fn sync_keys(&mut self) {
        let Some(g) = &mut self.gui else { return };
        for b in self.keys.iter() {
            g.opts.set(&format!("key_{}", b.name), b.key.name());
        }
        g.opts.save();
        self.keys.release_all();
    }

    /// Экран Controls: сброс по флагам и список строк для отрисовки.
    fn update_binds(&mut self) {
        use keymap::Category::*;
        let Some(g) = &mut self.gui else { return };
        let (all, one) = (std::mem::take(&mut g.reset_keys), g.reset_one.take());
        let mut changed = false;
        if all {
            self.keys.reset_all();
            changed = true;
        }
        if let Some(n) = one {
            if let Some(d) = self.keys.get(n).map(|b| b.default) {
                self.keys.rebind(n, d);
                changed = true;
            }
        }
        if changed {
            self.sync_keys();
        }
        let Some(g) = &mut self.gui else { return };
        if !g.wants_binds() {
            return;
        }
        // VANILLA: KeyMapping.Category — порядок экрана.
        let order = [
            (Movement, "key.categories.movement"),
            (Gameplay, "key.categories.gameplay"),
            (Inventory, "key.categories.inventory"),
            (Creative, "key.categories.creative"),
            (Spectator, "key.categories.spectator"),
            (Multiplayer, "key.categories.multiplayer"),
            (Misc, "key.categories.misc"),
        ];
        g.binds.clear();
        for (cat, label) in order {
            for b in self.keys.iter().filter(|b| b.category == cat) {
                let key = match b.key {
                    keymap::Input::Unbound => "key.keyboard.unknown".to_owned(),
                    k => k.name(),
                };
                g.binds.push(ui::BindRow {
                    name: b.name,
                    category: label,
                    key,
                    conflict: !self.keys.conflicts(b.name).is_empty(),
                    default: b.is_default(),
                });
            }
        }
    }

    /// Применяет options.txt к живой игре (раз в кадр, меняет только то, что изменилось).
    fn apply_settings(&mut self) {
        let Some(gui) = &self.gui else { return };
        let o = &gui.opts;
        // VANILLA: Options#enableVsync → смена режима present на лету (EZZMC_VSYNC=0 — отладочное форс-выключение).
        let vsync = o.get(&ui::options::VIDEO[ui::options::VSYNC]) == "true"
            && std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0");
        if self.applied_vsync != Some(vsync) {
            if let (Some(g), Some(sc)) = (&self.gpu, &mut self.swapchain) {
                sc.set_present_mode(g, display::present_mode(vsync));
                self.applied_vsync = Some(vsync);
            }
        }
        // Не ваниль: опция computeRenderer ↔ путь отрисовки (живое переключение).
        let want = o.get(&ui::options::VIDEO[ui::options::COMPUTE_RENDERER]) == "true";
        if self.applied_compute != Some(want) {
            if let (Some(v), Some(g)) = (&mut self.view, &self.gpu) {
                if v.compute_renderer_enabled() != want {
                    v.set_compute_renderer(g, want);
                    tracing::info!("{}", v.render_label());
                }
                self.applied_compute = Some(want);
            }
        }
        // VANILLA: Options#fullscreen ↔ окно (F11 пишет опцию обратно).
        let fs = o.get(&ui::options::VIDEO[ui::options::FULLSCREEN]) == "true";
        if self.applied_fs != Some(fs) {
            if let Some(w) = &self.window {
                if w.fullscreen().is_some() != fs {
                    window::toggle_fullscreen(w, &mut self.wstate);
                }
                self.applied_fs = Some(fs);
            }
        }
        self.update_binds();
        let Some(gui) = &self.gui else { return };
        let o = &gui.opts;
        for (name, def) in ["key.sneak", "key.sprint", "key.attack", "key.use"]
            .into_iter()
            .zip(&ui::options::CONTROLS[..4])
        {
            self.keys.set_toggle(name, o.get(def) == "true");
        }
        // VANILLA: Options#sensitivity/invertMouseX/Y/mouseWheelSensitivity/discreteMouseScroll.
        self.quit_shortcuts = o.get(
            &ui::options::CONTROLS[ui::options::CONTROLS
                .iter()
                .position(|d| d.key == "quitShortcuts")
                .unwrap_or(0)],
        ) == "true";
        self.allow_cursor = o.get(
            &ui::options::CONTROLS[ui::options::CONTROLS
                .iter()
                .position(|d| d.key == "allowCursorChanges")
                .unwrap_or(0)],
        ) == "true";
        self.mouse.sensitivity = f64::from(o.f32(&ui::options::MOUSE[0]));
        self.mouse.wheel_sens = f64::from(o.f32(&ui::options::MOUSE[1]));
        self.mouse.discrete_scroll = o.get(&ui::options::MOUSE[2]) == "true";
        self.mouse.invert_x = o.get(&ui::options::MOUSE[3]) == "true";
        self.mouse.invert_y = o.get(&ui::options::MOUSE[4]) == "true";
        let cfg = settings::ViewCfg::from_options(o);
        let info = settings::client_info(o);
        let key = format!("{info:?}");
        if let Some(v) = &mut self.view {
            v.set_cfg(cfg);
            // VANILLA: Options#broadcastOptions — при смене дальности/частей скина/руки/частиц/цвета чата.
            match &self.applied_info {
                None => self.applied_info = Some(key),
                Some(last) if *last != key => {
                    v.send(netthread::NetCmd::ClientInfo(Box::new(info)));
                    self.applied_info = Some(key);
                }
                _ => {}
            }
        } else {
            self.applied_info = None;
        }
    }

    fn init_gpu(&mut self, el: &ActiveEventLoop) {
        let Some(w) = self.window.clone() else { return };
        match ezzmc_render::Gpu::new(w.clone(), ezzmc_render::BackendChoice::from_env()) {
            Ok(g) => {
                let s = window::framebuffer_size(&w);
                self.swapchain = ezzmc_render::Swapchain::new(
                    &g,
                    s.width,
                    s.height,
                    display::present_mode(std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0")),
                );
                self.gpu = Some(g);
                self.world_pending = true;
            }
            Err(e) => {
                tracing::error!("{e}");
                el.exit();
            }
        }
    }
}

/// VANILLA: font/include/default.json — провайдеры `bitmap` (nonlatin_european, accented, ascii).
fn load_text_renderer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    rm: &ezzmc_assets::pack::ResourceManager,
) -> Option<ezzmc_render::text::TextRenderer> {
    let json: serde_json::Value =
        serde_json::from_slice(&rm.read("assets/minecraft/font/include/default.json")?).ok()?;
    let mut images = Vec::new();
    for p in json.get("providers")?.as_array()? {
        if p.get("type").and_then(|t| t.as_str()) != Some("bitmap") {
            continue;
        }
        let file = p.get("file")?.as_str()?.trim_start_matches("minecraft:");
        let img = ezzmc_assets::texture::Image::decode_png(
            &rm.read(&format!("assets/minecraft/textures/{file}"))?,
        )
        .ok()?;
        let rows: Vec<Vec<char>> = p
            .get("chars")?
            .as_array()?
            .iter()
            .filter_map(|r| r.as_str().map(|s| s.chars().collect()))
            .collect();
        let height = p.get("height").and_then(|h| h.as_f64()).unwrap_or(8.0) as f32;
        let ascent = p.get("ascent").and_then(|h| h.as_f64()).unwrap_or(7.0) as f32;
        images.push((img.to_rgba8(), img.width, img.height, rows, height, ascent));
    }
    // VANILLA: font/alt.json (шрифт зачарований «Standard Galactic Alphabet», ascii_sga.png).
    // Символы переносятся в область частного использования U+E000+c (`invscreen::alt`).
    if let Some(alt) = rm
        .read("assets/minecraft/font/alt.json")
        .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok())
    {
        for p in alt
            .get("providers")
            .and_then(|p| p.as_array())
            .into_iter()
            .flatten()
        {
            if p.get("type").and_then(|t| t.as_str()) != Some("bitmap") {
                continue;
            }
            let Some(file) = p.get("file").and_then(|f| f.as_str()) else {
                continue;
            };
            let file = file.trim_start_matches("minecraft:");
            let Some(img) = rm
                .read(&format!("assets/minecraft/textures/{file}"))
                .and_then(|b| ezzmc_assets::texture::Image::decode_png(&b).ok())
            else {
                continue;
            };
            let rows: Vec<Vec<char>> = p
                .get("chars")
                .and_then(|c| c.as_array())
                .into_iter()
                .flatten()
                .filter_map(|r| r.as_str())
                .map(|s| {
                    s.chars()
                        .map(|c| {
                            if c == '\0' {
                                c
                            } else {
                                crate::invscreen::alt_char(c)
                            }
                        })
                        .collect()
                })
                .collect();
            let height = p.get("height").and_then(|h| h.as_f64()).unwrap_or(8.0) as f32;
            let ascent = p.get("ascent").and_then(|h| h.as_f64()).unwrap_or(7.0) as f32;
            images.push((img.to_rgba8(), img.width, img.height, rows, height, ascent));
        }
    }
    let sheets: Vec<_> = images
        .iter()
        .map(
            |(rgba, w, h, rows, height, ascent)| ezzmc_render::text::FontSheet {
                rgba,
                w: *w,
                h: *h,
                rows: rows.clone(),
                height: *height,
                ascent: *ascent,
            },
        )
        .collect();
    (!sheets.is_empty())
        .then(|| ezzmc_render::text::TextRenderer::new(device, queue, format, &sheets))
}

impl App {
    /// VANILLA: KeyboardHandler#handleDebugKeys — комбинации F3+клавиша (зовут окно и агентское управление).
    /// Вызывать только при нажатии (не автоповторе); сбрасывать `f3_pending` — на вызывающем.
    pub(crate) fn f3_combo(&mut self, k: KeyCode) {
        if !self.f3_held {
            return;
        }
        match k {
            KeyCode::KeyH => {
                let on = !tooltip::ADVANCED.fetch_xor(true, std::sync::atomic::Ordering::Relaxed);
                if let Some(v) = &mut self.view {
                    v.debug_feedback(if on {
                        "Advanced tooltips: shown"
                    } else {
                        "Advanced tooltips: hidden"
                    });
                }
            }
            KeyCode::KeyA => {
                if let Some(v) = &mut self.view {
                    v.reload_chunks();
                }
            }
            KeyCode::KeyB => {
                if let Some(v) = &mut self.view {
                    v.toggle_hitboxes();
                }
            }
            KeyCode::KeyG => {
                if let Some(v) = &mut self.view {
                    v.toggle_chunk_borders();
                }
            }
            KeyCode::KeyC => {
                if let Some(v) = &mut self.view {
                    let t = v.location_command();
                    v.debug_feedback("Copied location to clipboard");
                    self.clipboard.set(&t);
                }
            }
            KeyCode::KeyD => self.chat.clear_messages(),
            KeyCode::F4 => self.gms_key(),
            KeyCode::KeyN => {
                if let Some(v) = &mut self.view {
                    v.debug_spectate();
                }
            }
            KeyCode::KeyT => {
                if let Some(v) = &mut self.view {
                    v.debug_feedback("Reloaded resource packs");
                }
                self.reload_resources();
            }
            KeyCode::KeyP => {
                self.focus.pause_on_lost_focus = !self.focus.pause_on_lost_focus;
                let on = self.focus.pause_on_lost_focus;
                if let Some(g) = &mut self.gui {
                    g.opts.set("pauseOnLostFocus", on.to_string());
                    g.opts.save();
                }
                if let Some(v) = &mut self.view {
                    v.debug_feedback(if on {
                        "Pause on lost focus: enabled"
                    } else {
                        "Pause on lost focus: disabled"
                    });
                }
            }
            KeyCode::KeyQ => {
                if let Some(v) = &mut self.view {
                    v.debug_feedback("F3 + Q = Show this list");
                    for l in [
                        "F3 + A = Reload chunks",
                        "F3 + B = Show hitboxes",
                        "F3 + C = Copy location",
                        "F3 + D = Clear chat",
                        "F3 + G = Show chunk boundaries",
                        "F3 + H = Advanced tooltips",
                        "F3 + N = Cycle creative <-> spectator",
                        "F3 + P = Pause on lost focus",
                        "F3 + T = Reload resource packs",
                    ] {
                        v.debug_feedback(l);
                    }
                }
            }
            _ => {}
        }
    }

    /// Шрифт, спрайты GUI, панорама, блеск, язык из текущих паков (VANILLA: ReloadableResourceManager).
    /// Возвращает новый painter и байты lang-файла.
    fn load_gui_assets(
        &mut self,
        rm: &ezzmc_assets::pack::ResourceManager,
    ) -> Option<(ezzmc_render::gui::GuiPainter, Vec<u8>)> {
        let (Some(g), Some(sc)) = (&self.gpu, &self.swapchain) else {
            return None;
        };
        self.text = load_text_renderer(&g.device, &g.queue, sc.config.format, rm);
        let mut p = ezzmc_render::gui::GuiPainter::new(&g.device, sc.config.format);
        p.add_texture(&g.device, &g.queue, "white", &[255; 4], 1, 1, false);
        // Панорама меню: в jar лежат заглушки 1×1, настоящие кубмапы — в assets/objects (индекс).
        let store = ezzmc_assets::install::GameInstall::locate("26.3")
            .ok()
            .and_then(|i| {
                let idx = i.index_path().ok()??;
                let objects = i.assets_dir.as_ref()?.join("objects");
                ezzmc_assets::source::ObjectStore::open(&idx, &objects).ok()
            });
        for (name, path) in ui::TEXTURES
            .iter()
            .chain(hud::SPRITES)
            .chain(hudbars::SPRITES)
            .chain(effectshud::SPRITES)
            .chain(tablist::SPRITES)
            .chain(bossbar::SPRITES)
            .chain(toasts::SPRITES)
            .chain(advscreen::SPRITES)
            .chain(statsscreen::SPRITES)
            .chain(recipescreen::SPRITES)
            .chain(gmswitcher::SPRITES)
            .chain(ui::social::TEXTURES)
        {
            let full = format!("assets/minecraft/textures/gui/{path}.png");
            let from_store = if name.starts_with("panorama") {
                store.as_ref().and_then(|s| {
                    use ezzmc_assets::source::ResourceSource;
                    s.read(&full)
                })
            } else {
                None
            };
            match from_store
                .or_else(|| rm.read(&full))
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            {
                Some(i) => p.add_texture(
                    &g.device,
                    &g.queue,
                    name,
                    &i.to_rgba8(),
                    i.width,
                    i.height,
                    name.starts_with("panorama"),
                ),
                None => tracing::warn!("текстура GUI не найдена: {full}"),
            }
        }
        // Текстуры флага баннера (превью/кнопки ткацкого станка): textures/entity/banner/<имя>.png.
        for n in loomscreen::texture_names() {
            let full = format!("assets/minecraft/textures/entity/banner/{n}.png");
            if let Some(i) = rm
                .read(&full)
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            {
                p.add_texture(
                    &g.device,
                    &g.queue,
                    &format!("lmb_{n}"),
                    &i.to_rgba8(),
                    i.width,
                    i.height,
                    false,
                );
            }
        }
        // Иконки эффектов лежат вне gui/: textures/mob_effect/*.png.
        for (name, path) in effectshud::MOB_EFFECT_SPRITES
            .iter()
            .chain(&[("soc_steve", "entity/player/wide/steve")])
        {
            let full = format!("assets/minecraft/textures/{path}.png");
            match rm
                .read(&full)
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            {
                Some(i) => p.add_texture(
                    &g.device,
                    &g.queue,
                    name,
                    &i.to_rgba8(),
                    i.width,
                    i.height,
                    false,
                ),
                None => tracing::warn!("иконка эффекта не найдена: {full}"),
            }
        }
        // Иконки паков: pack.png ванили и unknown_pack (VANILLA: PackSelectionScreen#DEFAULT_ICON).
        for (name, path) in [
            (
                "pack_unknown",
                "assets/minecraft/textures/misc/unknown_pack.png",
            ),
            ("pack_vanilla", "pack.png"),
            (
                "unknown_server",
                "assets/minecraft/textures/misc/unknown_server.png",
            ),
        ] {
            let data = rm.read(path);
            if let Some(i) = data.and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok()) {
                p.add_texture(
                    &g.device,
                    &g.queue,
                    name,
                    &i.to_rgba8(),
                    i.width,
                    i.height,
                    false,
                );
            }
        }
        match rm
            .read("assets/minecraft/textures/misc/enchanted_glint_item.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        {
            Some(i) => p.set_glint_texture(&g.device, &g.queue, &i.to_rgba8(), i.width, i.height),
            None => tracing::warn!("текстура блеска не найдена"),
        }
        let lang = rm
            .read("assets/minecraft/lang/en_us.json")
            .unwrap_or_default();
        self.item_lang = ui::lang::Lang::load(&lang);
        Some((p, lang))
    }

    /// Применить смену ресурспаков без перезапуска. Возвращает false, если на лету нельзя
    /// (изменился размер атласа блоков) — тогда вызывающий перезапускает процесс.
    fn reload_resources(&mut self) -> bool {
        let rm = match ezzmc_assets::install::GameInstall::locate("26.3")
            .and_then(|i| packrepo::build_resource_manager(&i))
        {
            Ok(rm) => rm,
            Err(e) => {
                tracing::error!("ресурсы игры 26.3: {e}");
                return true;
            }
        };
        self.prewarm = None;
        if let (Some(v), Some(g)) = (&mut self.view, &self.gpu) {
            if let Err(e) = v.reload_resources(g, &rm) {
                tracing::warn!("перезагрузка на лету невозможна: {e}");
                return false;
            }
        }
        if let Some((p, _lang)) = self.load_gui_assets(&rm) {
            self.painter = Some(p);
        }
        self.hud_rm = Some(rm);
        true
    }

    /// Загрузка ресурсов GUI (шрифт, спрайты, панорама, язык) и показ главного меню.
    fn init_gui(&mut self) {
        let rm = match ezzmc_assets::install::GameInstall::locate("26.3")
            .and_then(|i| packrepo::build_resource_manager(&i))
        {
            Ok(rm) => rm,
            Err(e) => {
                tracing::error!("ресурсы игры 26.3: {e}");
                return;
            }
        };
        let Some((p, lang)) = self.load_gui_assets(&rm) else {
            return;
        };
        let splashes = rm
            .read("assets/minecraft/texts/splashes.txt")
            .map(|d| String::from_utf8_lossy(&d).into_owned())
            .unwrap_or_default();
        let gui = ui::Gui::new(&lang, &splashes, std::path::Path::new("."));
        let max_fps = gui.opts.f32(&ui::options::VIDEO[ui::options::MAX_FPS]) as u32;
        self.fps_limit.set_limit(max_fps);
        // VANILLA: Options#load — key_* из options.txt.
        self.keys.load_options(gui.opts.pairs());
        let mut gui = gui;
        gui.set_languages(language_list(&rm));
        if let Some(code) = gui
            .opts
            .raw("lang")
            .filter(|c| *c != "en_us")
            .map(str::to_owned)
        {
            gui.set_lang(build_lang(&rm, &code));
            self.item_lang = build_lang(&rm, &code);
        }
        self.gui = Some(gui);
        self.painter = Some(p);
        self.hud_rm = Some(rm);
        self.menu_active = true;
        self.start_prewarm();
    }

    /// Собрать атлас и модели блоков в фоне (VANILLA: ресурсы грузятся при старте, не при входе).
    fn start_prewarm(&mut self) {
        let cutout = ui::options::Options::load(std::path::Path::new("./options.txt"))
            .get(&ui::options::VIDEO[ui::options::CUTOUT_LEAVES])
            == "true";
        self.prewarm = std::thread::Builder::new()
            .name("prewarm".into())
            .spawn(move || {
                let t0 = std::time::Instant::now();
                let rm = ezzmc_assets::install::GameInstall::locate("26.3")
                    .and_then(|i| packrepo::build_resource_manager(&i))
                    .ok()?;
                let atlas = ezzmc_assets::block_atlas::BlockAtlas::build(&rm, 4).ok()?;
                let models = ezzmc_mesh::models::BlockModels::build(&rm, &atlas, cutout);
                tracing::info!("prewarm: атлас и модели за {:?}", t0.elapsed());
                Some((atlas, models, cutout))
            })
            .ok();
    }

    /// Подключение к серверу и загрузка ресурсов игры (если задан `--connect`).
    fn init_world(&mut self) {
        let (Some(addr), Some(g), Some(sc)) = (self.connect.clone(), &self.gpu, &self.swapchain)
        else {
            return;
        };
        let rm = match ezzmc_assets::install::GameInstall::locate("26.3")
            .and_then(|i| packrepo::build_resource_manager(&i))
        {
            Ok(rm) => rm,
            Err(e) => {
                tracing::error!("ресурсы игры 26.3: {e}");
                return;
            }
        };
        // Шрифт HUD — ванильный ascii.png (128×128); если меню уже загрузило — не трогаем.
        if self.text.is_none() {
            self.text = load_text_renderer(&g.device, &g.queue, sc.config.format, &rm);
        }
        let net = netthread::spawn(addr, self.player_name.clone());
        let threads = std::thread::available_parallelism()
            .map_or(2, std::num::NonZero::get)
            .saturating_sub(2)
            .max(1);
        let mut ro = worldview::RenderOptions::from_env_args();
        let o = ui::options::Options::load(std::path::Path::new("./options.txt"));
        ro.cutout_leaves = o.get(&ui::options::VIDEO[ui::options::CUTOUT_LEAVES]) == "true";
        ro.use_compute_renderer |=
            o.get(&ui::options::VIDEO[ui::options::COMPUTE_RENDERER]) == "true";
        // Готовые атлас/модели из фона — только если опция листвы не менялась.
        let prebuilt_h = self.prewarm.take();
        let (format, own_name) = (sc.config.format, self.player_name.clone());
        // Сборка мира уходит в поток: окно остаётся живым (Cancel нажимается, VANILLA: ConnectScreen).
        let gpu = ezzmc_render::Gpu {
            instance: g.instance.clone(),
            adapter: g.adapter.clone(),
            device: g.device.clone(),
            queue: g.queue.clone(),
            surface: None,
            device_lost: g.device_lost.clone(),
        };
        if let Some(gui) = &mut self.gui {
            gui.open_connecting();
            self.menu_active = true;
        }
        self.world_job = std::thread::Builder::new()
            .name("world-build".into())
            .spawn(move || {
                let prebuilt = prebuilt_h
                    .and_then(|h| h.join().ok().flatten())
                    .filter(|(_, _, c)| *c == ro.cutout_leaves)
                    .map(|(a, m, _)| (a, m));
                worldview::WorldView::new(&gpu, format, &rm, net, threads, ro, prebuilt).map(
                    |mut v| {
                        v.own_name = own_name;
                        v
                    },
                )
            })
            .ok();
    }

    /// Забрать готовый мир из потока сборки (каждый кадр).
    fn poll_world_job(&mut self) {
        if !self.world_job.as_ref().is_some_and(|h| h.is_finished()) {
            return;
        }
        let Some(h) = self.world_job.take() else {
            return;
        };
        match h.join() {
            Ok(Ok(v)) => self.view = Some(v),
            Ok(Err(e)) => tracing::error!("мир: {e}"),
            Err(_) => tracing::error!("мир: поток сборки упал"),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        match window::create(el, &self.cfg) {
            Ok(w) => {
                self.window = Some(Arc::new(w));
                self.init_gpu(el);
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            Err(e) => {
                tracing::error!("{e}");
                el.exit();
            }
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _id: DeviceId, ev: DeviceEvent) {
        if let (DeviceEvent::MouseMotion { delta: (dx, dy) }, true) = (ev, self.focus.active) {
            // Захват, сделанный без участия пользователя (запуск сразу на сервер), может не
            // действовать, пока нет реального ввода; Esc→выход его чинит (release + grab).
            if self.mouse.fresh && !self.menu_active && !self.inv_open && !self.chat.open {
                if let Some(w) = &self.window {
                    self.mouse.regrab(w);
                }
            }
            if !self.chat.open {
                self.mouse.on_motion(dx, dy);
            }
            self.fps_limit.on_input(self.clock.now_ms());
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match &ev {
            WindowEvent::ModifiersChanged(m) => self.alt_down = m.state().alt_key(),
            WindowEvent::KeyboardInput { event, .. }
                if event.physical_key == PhysicalKey::Code(KeyCode::F4) =>
            {
                self.f4_down = event.state == ElementState::Pressed;
            }
            _ => {}
        }
        match ev {
            // VANILLA: Window#onQuitRequested — Alt+F4 закрывает окно только при options.quitShortcuts.
            WindowEvent::DroppedFile(path) => {
                if let Some(g) = self.gui.as_mut() {
                    g.on_drop(&path);
                }
            }
            WindowEvent::CloseRequested => {
                if self.quit_shortcuts || !(self.alt_down && self.f4_down) {
                    el.exit();
                }
            }
            // VANILLA: GLFW charCallback — IME присылает готовый текст отдельным событием.
            WindowEvent::Ime(winit::event::Ime::Commit(t)) => {
                if self.chat.open {
                    self.chat.edit.type_text(&t);
                } else if self.menu_active {
                    if let Some(g) = &mut self.gui {
                        g.on_text(&t);
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.win_screen.is_some() => {
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.win_key(k, event.state == ElementState::Pressed);
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.chat.open => {
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                }
                if event.state == ElementState::Pressed {
                    self.chat_key(&event);
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.chat.open => {
                if state == ElementState::Pressed && button == winit::event::MouseButton::Left {
                    self.chat_click();
                }
            }
            WindowEvent::MouseWheel { delta, .. } if self.chat.open => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => f64::from(y),
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y,
                };
                // VANILLA: ChatScreen#mouseScrolled — шаг ±1, без Shift ×7.
                let shift = self.input.key_down(KeyCode::ShiftLeft)
                    || self.input.key_down(KeyCode::ShiftRight);
                let d = y.clamp(-1.0, 1.0) * if shift { 1.0 } else { 7.0 };
                let cfg = self.chat_cfg;
                self.chat.scroll_by(d as i32, &cfg);
            }
            WindowEvent::KeyboardInput { event, .. } if self.stats_screen.is_some() => {
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                    self.keys
                        .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                    // VANILLA: StatsScreen#keyPressed — Esc закрывает.
                    if event.state == ElementState::Pressed && !event.repeat && k == KeyCode::Escape
                    {
                        self.close_stats();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.stats_screen.is_some() => {
                if button == winit::event::MouseButton::Left {
                    let down = state == ElementState::Pressed;
                    self.stats_down = down;
                    if let (true, Some((gw, gh, mx, my)), Some(v), Some(s)) =
                        (down, self.gui_cursor(), &self.view, &mut self.stats_screen)
                    {
                        if s.click(&v.stats, (gw, gh), (mx, my)) == statsscreen::Click::Close {
                            self.close_stats();
                        }
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } if self.stats_screen.is_some() => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let gh = self.gui_cursor().map(|c| c.1);
                if let (Some(s), Some(gh)) = (&mut self.stats_screen, gh) {
                    s.wheel(gh, y);
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.adv.is_some() => {
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                    self.keys
                        .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                    // VANILLA: AdvancementsScreen#keyPressed — Esc / key.advancements закрывают.
                    let close = k == KeyCode::Escape
                        || self
                            .keys
                            .get("key.advancements")
                            .is_some_and(|b| b.key == keymap::Input::Key(k));
                    if event.state == ElementState::Pressed && !event.repeat && close {
                        self.close_adv();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.adv.is_some() => {
                // VANILLA: AdvancementsScreen#mouseClicked/mouseDragged — кнопка 1 (правая).
                if button == winit::event::MouseButton::Right {
                    let down = state == ElementState::Pressed;
                    if let (true, Some((gw, gh, mx, my)), Some(v), Some(a)) =
                        (down, self.gui_cursor(), &mut self.view, &mut self.adv)
                    {
                        if let Some(id) = a.click_tab(gw, gh, mx, my) {
                            v.advancements.select(Some(id.clone()));
                            v.send(netthread::NetCmd::SeenAdvancements(Some(id)));
                        }
                    }
                    if let Some(a) = &mut self.adv {
                        a.press(down);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } if self.adv.is_some() => {
                let (x, y) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    winit::event::MouseScrollDelta::PixelDelta(p) => (p.x as f32, p.y as f32),
                };
                if let (Some(a), Some(v)) = (&mut self.adv, &self.view) {
                    a.wheel(&v.advancements, x, y);
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.inv_open => {
                // VANILLA: Screen.hasShiftDown/hasControlDown — состояние клавиш должно
                // обновляться и в экране инвентаря (иначе Shift+клик не виден).
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                    self.keys
                        .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                }
                let eaten = event.state == ElementState::Pressed
                    && (self.creative_search_input(&event)
                        || self.recipe_search_input(&event)
                        || self.anvil_input(&event));
                // VANILLA: Screen#keyPressed приходит и на автоповтор; для остальных клавиш повтор
                // бессмыслен (обмен хотбара идемпотентен), поэтому повторяем только выброс.
                let repeat_drop = |a: &Self, k: KeyCode| {
                    a.keys
                        .get("key.drop")
                        .is_some_and(|b| b.key == keymap::Input::Key(k))
                };
                if let (PhysicalKey::Code(k), ElementState::Pressed, false) =
                    (event.physical_key, event.state, eaten)
                {
                    if !event.repeat || repeat_drop(self, k) {
                        // VANILLA: LecternScreen — не AbstractContainerScreen: E не закрывает, ←/→ листают.
                        let lectern = self
                            .view
                            .as_ref()
                            .is_some_and(|v| invscreen::layout(&v.inv).is_lectern());
                        let close = k == KeyCode::Escape
                            || (!lectern
                                && self
                                    .keys
                                    .get("key.inventory")
                                    .is_some_and(|b| b.key == keymap::Input::Key(k)));
                        if close {
                            self.close_inv(true);
                        } else if lectern {
                            if let (Some(v), true) = (
                                &self.view,
                                matches!(k, KeyCode::ArrowLeft | KeyCode::ArrowRight),
                            ) {
                                if let Some(o) = &v.inv.open {
                                    v.send(bookview::arrow(o, k == KeyCode::ArrowRight));
                                }
                            }
                        } else {
                            self.inv_key(k);
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.inv_open => {
                let right = button == winit::event::MouseButton::Right;
                let left = button == winit::event::MouseButton::Left;
                let middle = button == winit::event::MouseButton::Middle;
                self.inv_mouse(state == ElementState::Pressed, right, left, middle);
            }
            WindowEvent::MouseWheel { delta, .. } if self.inv_open => {
                let gc = self.gui_cursor();
                if self.inv_creative {
                    let y = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => f64::from(y),
                        winit::event::MouseScrollDelta::PixelDelta(p) => p.y,
                    };
                    self.creative.scroll(y);
                } else if let Some(v) = &mut self.view {
                    let y = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                        winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                    };
                    loomscreen::wheel(&mut v.inv, y);
                    merchantscreen::wheel(&mut v.inv, y);
                    // VANILLA: BundleMouseActions#onMouseScrolled — колесо над бандлом выбирает предмет.
                    if let Some((gw, gh, mx, my)) = gc {
                        let lay = invscreen::layout(&v.inv);
                        if let Some(sl) = lay.slot_at(gw, gh, mx, my) {
                            if let Some(st) = invscreen::slot_stack_pub(&v.inv, sl) {
                                if let Some((a, b)) = tooltip::bundle_wheel(sl, st, y) {
                                    v.send_select_bundle(a, b);
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.menu_active => {
                // Без этого Ctrl/Shift в меню всегда «отпущены» (input обновляется только в других ветках)
                // и Ctrl+A/C/X/V, Shift+стрелки в полях не работают.
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                }
                if event.state == ElementState::Pressed {
                    // VANILLA: KeyBindsScreen#keyPressed — Esc снимает привязку, иначе клавиша становится привязкой.
                    if let (Some(g), PhysicalKey::Code(k)) = (&mut self.gui, event.physical_key) {
                        if let Some(n) = g.listening.take() {
                            let inp = if k == KeyCode::Escape {
                                keymap::Input::Unbound
                            } else {
                                keymap::Input::Key(k)
                            };
                            self.keys.rebind(n, inp);
                            self.sync_keys();
                            return;
                        }
                    }
                    if let Some(g) = &mut self.gui {
                        if let PhysicalKey::Code(k) = event.physical_key {
                            use editbox::Move;
                            use ui::UiKey;
                            let ctrl = self.input.key_down(KeyCode::ControlLeft)
                                || self.input.key_down(KeyCode::ControlRight);
                            let shift = self.input.key_down(KeyCode::ShiftLeft)
                                || self.input.key_down(KeyCode::ShiftRight);
                            // VANILLA: EditBox#keyPressed — Ctrl+A/C/X/V, стрелки, Home/End, Delete, Backspace.
                            match k {
                                KeyCode::Escape => g.on_key(UiKey::Escape),
                                KeyCode::Enter | KeyCode::NumpadEnter => g.on_key(UiKey::Enter),
                                KeyCode::Backspace => g.on_key(UiKey::Erase(-1, ctrl)),
                                KeyCode::Delete => g.on_key(UiKey::Erase(1, ctrl)),
                                KeyCode::ArrowUp => g.on_key(UiKey::Up),
                                KeyCode::ArrowDown => g.on_key(UiKey::Down),
                                KeyCode::Tab if shift => g.on_key(UiKey::BackTab),
                                KeyCode::Tab => g.on_key(UiKey::Tab),
                                KeyCode::ArrowLeft => {
                                    g.on_key(UiKey::Move(Move::Left, ctrl, shift))
                                }
                                KeyCode::ArrowRight => {
                                    g.on_key(UiKey::Move(Move::Right, ctrl, shift))
                                }
                                KeyCode::Home => g.on_key(UiKey::Move(Move::Home, false, shift)),
                                KeyCode::End => g.on_key(UiKey::Move(Move::End, false, shift)),
                                KeyCode::KeyA if ctrl => g.on_key(UiKey::SelectAll),
                                KeyCode::KeyC if ctrl => g.on_key(UiKey::Copy),
                                KeyCode::KeyX if ctrl => g.on_key(UiKey::Cut),
                                KeyCode::KeyV if ctrl => g.on_text(&self.clipboard.get()),
                                _ => {}
                            }
                            if !ctrl {
                                if let Some(t) = &event.text {
                                    g.on_text(t.as_str());
                                }
                            }
                        }
                        if let Some(t) = g.take_clip() {
                            self.clipboard.set(&t);
                        }
                    }
                }
            }
            WindowEvent::CursorEntered { .. } => {
                // XGrabPointer без фокуса (XWayland) не получает ответа: xcb_wait_for_reply висит вечно.
                if let (Some(w), true) = (&self.window, self.focus.active) {
                    self.mouse.regrab(w);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_px = (position.x, position.y);
                self.inv_cursor_moved();
                if let Some(g) = &mut self.gui {
                    g.on_cursor(position.x, position.y);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(k) = event.physical_key {
                    // VANILLA: KeyboardHandler — F3 переключает экран отладки при отпускании,
                    // если пока она была зажата, не нажимали другие клавиши (комбинации F3+*).
                    if k == KeyCode::F3 {
                        self.f3_held = event.state == ElementState::Pressed;
                        if !self.f3_held {
                            self.close_gms();
                        }
                        if event.state == ElementState::Pressed && !event.repeat {
                            self.f3_pending = true;
                        } else if event.state == ElementState::Released && self.f3_pending {
                            self.f3_pending = false;
                            self.debug_on = !self.debug_on;
                        }
                    } else if event.state == ElementState::Pressed {
                        if !event.repeat {
                            self.f3_combo(k);
                        }
                        self.f3_pending = false;
                    }
                    // VANILLA: KeyboardHandler#keyPress — автоповтор (action 2) тоже вызывает
                    // KeyMapping.click, поэтому зажатая Q выбрасывает предметы подряд.
                    if event.state == ElementState::Pressed
                        && !self.menu_active
                        && self
                            .keys
                            .get("key.drop")
                            .is_some_and(|b| b.key == keymap::Input::Key(k))
                    {
                        let all = self.input.key_down(KeyCode::ControlLeft);
                        if let Some(v) = &mut self.view {
                            v.drop_item(all);
                        }
                    }
                    if !event.repeat {
                        // VANILLA: KeyboardHandler — key.fullscreen → Window#toggleFullScreen.
                        if event.state == ElementState::Pressed
                            && self
                                .keys
                                .get("key.fullscreen")
                                .is_some_and(|b| b.key == keymap::Input::Key(k))
                        {
                            if let Some(w) = &self.window {
                                window::toggle_fullscreen(w, &mut self.wstate);
                                if let Some(g) = &mut self.gui {
                                    g.opts.set("fullscreen", self.wstate.fullscreen.to_string());
                                    g.opts.save();
                                }
                                self.applied_fs = Some(self.wstate.fullscreen);
                            }
                        }
                        // VANILLA: KeyboardHandler — key.screenshot по нажатию.
                        if event.state == ElementState::Pressed
                            && self
                                .keys
                                .get("key.screenshot")
                                .is_some_and(|b| b.key == keymap::Input::Key(k))
                        {
                            self.want_screenshot = true;
                        }
                        // VANILLA: Gui#handleKeybinds — key.toggleGui → Hud#toggle.
                        if event.state == ElementState::Pressed
                            && !self.menu_active
                            && self
                                .keys
                                .get("key.toggleGui")
                                .is_some_and(|b| b.key == keymap::Input::Key(k))
                        {
                            self.hud_hidden = !self.hud_hidden;
                        }
                        // VANILLA: Minecraft#handleKeybinds — key.togglePerspective → CameraType#cycle.
                        if event.state == ElementState::Pressed
                            && self
                                .keys
                                .get("key.togglePerspective")
                                .is_some_and(|b| b.key == keymap::Input::Key(k))
                        {
                            if let Some(v) = &mut self.view {
                                v.cycle_perspective();
                            }
                        }
                        // Не ваниль: key.renderMode (F8) — compute ↔ ванильный путь; пишет опцию,
                        // применяет apply_settings.
                        if event.state == ElementState::Pressed
                            && self
                                .keys
                                .get("key.renderMode")
                                .is_some_and(|b| b.key == keymap::Input::Key(k))
                        {
                            if let (Some(v), Some(g)) = (&self.view, &mut self.gui) {
                                let on = !v.compute_renderer_enabled();
                                g.opts.set("computeRenderer", on.to_string());
                                g.opts.save();
                            }
                        }
                        if event.state == ElementState::Pressed && !self.menu_active {
                            // VANILLA: Minecraft#handleKeybinds — key.hotbar.1..9.
                            let n = (1..=9).find(|i| {
                                self.keys
                                    .get(&format!("key.hotbar.{i}"))
                                    .is_some_and(|b| b.key == keymap::Input::Key(k))
                            });
                            if let Some(n) = n {
                                self.hotbar_key(n - 1);
                            }
                            let is = |name: &str| {
                                self.keys
                                    .get(name)
                                    .is_some_and(|b| b.key == keymap::Input::Key(k))
                            };
                            if !event.repeat && is("key.chat") {
                                self.open_chat("");
                            } else if !event.repeat && is("key.command") {
                                self.open_chat("/");
                            } else if !event.repeat && is("key.inventory") {
                                self.open_inv();
                            } else if !event.repeat && is("key.advancements") {
                                self.open_adv();
                            } else if !event.repeat && is("key.socialInteractions") {
                                self.open_social();
                            } else if !event.repeat && is("key.swapOffhand") {
                                if let Some(v) = &mut self.view {
                                    v.swap_offhand();
                                }
                            }
                        }
                        self.input.on_key(k, event.state);
                        self.keys
                            .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                    }
                    // VANILLA: Esc в игре — меню паузы (курсор освобождается).
                    if k == KeyCode::Escape && event.state == ElementState::Pressed {
                        self.open_pause_menu();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.menu_active => {
                if let (Some(g), ElementState::Pressed) = (&mut self.gui, state) {
                    if let Some(n) = g.listening.take() {
                        self.keys.rebind(n, keymap::Input::Mouse(button));
                        self.sync_keys();
                        return;
                    }
                }
                if button == winit::event::MouseButton::Left {
                    if let Some(g) = &mut self.gui {
                        g.on_left_button(state == ElementState::Pressed);
                    }
                }
            }
            // VANILLA: WinScreen — экран без виджетов, клики в мир не идут.
            WindowEvent::MouseInput { .. } if self.win_screen.is_some() => {}
            WindowEvent::MouseInput { state, button, .. } => {
                self.input.on_button(button, state);
                self.keys
                    .on_input(keymap::Input::Mouse(button), state == ElementState::Pressed);
                if state == ElementState::Pressed
                    && !self.mouse.grabbed
                    && !self.chat.open
                    && !self.inv_open
                    && !self.dead_screen
                {
                    if let Some(w) = &self.window {
                        self.mouse.grab(w);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => f64::from(y),
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y / 40.0,
                };
                if self.menu_active {
                    if let Some(g) = &mut self.gui {
                        g.on_wheel(y as f32);
                    }
                } else if self.mouse.grabbed {
                    // VANILLA: MouseHandler#onScroll — Ctrl не влияет на прокрутку хотбара.
                    let steps = self.mouse.scroll_steps(y);
                    if let Some(v) = &mut self.view {
                        for _ in 0..steps.abs() {
                            v.scroll_hotbar(f64::from(steps.signum()));
                        }
                    }
                }
            }
            WindowEvent::Occluded(o) => self.occluded = o,
            WindowEvent::Focused(true) => {
                self.focus.set_active(true);
                // Захват, сделанный до получения фокуса (запуск сразу на сервер), не действует —
                // перезахватываем при получении фокуса.
                let playing = !self.menu_active
                    && !self.inv_open
                    && !self.chat.open
                    && !self.dead_screen
                    && self.win_screen.is_none()
                    && self.view.as_ref().is_some_and(|v| v.joined);
                if let (true, Some(w)) = (playing, &self.window) {
                    self.mouse.release(w);
                    self.mouse.grab(w);
                }
            }
            WindowEvent::Focused(false) => {
                // VANILLA: потеря фокуса — отпустить клавиши и курсор.
                self.focus.set_active(false);
                self.input.release_all();
                self.keys.release_all();
                if let Some(w) = &self.window {
                    self.mouse.release(w);
                }
                // VANILLA: Minecraft#setWindowActive(false) → pauseGame(false) (pauseOnLostFocus):
                // в игре без открытого экрана открывается меню паузы, чтобы после возврата курсор не «залипал».
                let playing = !self.menu_active
                    && !self.inv_open
                    && !self.chat.open
                    && !self.dead_screen
                    && self.win_screen.is_none()
                    && self.view.as_ref().is_some_and(|v| v.joined);
                if playing && self.focus.pause_on_lost_focus {
                    self.open_pause_menu();
                }
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(w) = &self.window {
                    let s = window::framebuffer_size(w);
                    let l = s.to_logical::<f64>(w.scale_factor());
                    self.wstate
                        .on_resized(l.width.round() as u32, l.height.round() as u32);
                    if let (Some(g), Some(sc)) = (&self.gpu, &mut self.swapchain) {
                        sc.resize(g, s.width, s.height);
                    }
                    tracing::debug!(
                        "размер окна {}x{} (scale {})",
                        s.width,
                        s.height,
                        w.scale_factor()
                    );
                }
            }
            WindowEvent::RedrawRequested => {
                use ezzmc_render::Acquire;
                if self.world_pending && self.frames >= 3 {
                    self.world_pending = false;
                    self.init_gui();
                    if self.connect.is_some() {
                        self.menu_active = false;
                        self.init_world();
                    }
                }
                // VANILLA: ClientCommonPacketListenerImpl#onDisconnect → DisconnectedScreen.
                if let Some((why, joined)) = self
                    .view
                    .as_ref()
                    .and_then(|v| v.status.clone().map(|s| (s, v.joined)))
                {
                    if let Some(g) = &mut self.gui {
                        g.open_disconnected(why, !joined);
                        self.view = None;
                        self.connect = None;
                        self.start_prewarm();
                        self.menu_active = true;
                        self.input.release_all();
                        self.keys.release_all();
                        if let Some(w) = &self.window {
                            self.mouse.release(w);
                        }
                    }
                }
                self.apply_settings();
                // VANILLA: Window#selectCursor — запрос прошлого кадра, при allowCursorChanges=false стрелка.
                if let Some(w) = &self.window {
                    w.set_cursor(cursor::take(self.allow_cursor));
                }
                {
                    // VANILLA: Minecraft#getSituationalMusic — сначала screen.getBackgroundMusic().
                    let m = match &self.view {
                        _ if self.win_screen.is_some() => Some(music::credits()),
                        Some(v) if v.joined => v.situational_music(),
                        _ => Some(music::menu()),
                    };
                    // VANILLA: WinScreen#removed → stopPlaying(Musics.CREDITS).
                    if self.win_screen.is_none() {
                        self.music.stop_playing("music.credits", m.as_ref());
                    }
                    if let Some(gui) = &self.gui {
                        let f =
                            music::Frequency::parse(gui.opts.raw("musicFrequency").unwrap_or(""));
                        self.music.set_frequency(f, m.as_ref());
                    }
                    self.music.update(m.as_ref(), false);
                    // VANILLA: MusicManager#startPlaying/stopPlaying → NowPlayingToast.
                    let toast_on = self
                        .gui
                        .as_ref()
                        .and_then(|g| g.opts.raw("musicToast"))
                        .is_some_and(|m| m.trim_matches('"') == "pause_and_toast");
                    if self.music.take_started() && toast_on && audio::music_volume() > 0.0 {
                        self.now_playing.show_toast();
                    }
                    if self.music.take_hide() {
                        self.now_playing.hide();
                    }
                }
                // Отладка: EZZMC_MENU_SHOT=<кадр> — скриншот меню и выход.
                if let (true, Some(n)) = (self.menu_active, menu_shot_frame()) {
                    if self.frames == n {
                        self.want_screenshot = true;
                    } else if self.frames > n + 10 {
                        el.exit();
                    }
                }
                // Два кадра с экраном «Connecting…», затем тяжёлая сборка мира (VANILLA: ConnectScreen).
                if self.join_pending {
                    if self.join_delay > 0 {
                        self.join_delay -= 1;
                    } else {
                        self.join_pending = false;
                        self.init_world();
                    }
                }
                if self.reload_pending {
                    self.reload_pending = false;
                    if !self.reload_resources() {
                        // Атлас другого размера: перезапуск с теми же аргументами (+ текущий сервер).
                        let mut args: Vec<String> = std::env::args().skip(1).collect();
                        if let (Some(a), false) =
                            (&self.connect, args.iter().any(|x| x == "--connect"))
                        {
                            args.extend(["--connect".into(), a.clone()]);
                        }
                        if let Ok(exe) = std::env::current_exe() {
                            let _ = std::process::Command::new(exe).args(args).spawn();
                        }
                        el.exit();
                    }
                }
                self.poll_world_job();
                self.control_tick(el);
                self.sync_screens();
                self.frames = self.frames.saturating_add(1);
                self.fps_count += 1;
                let t = self.clock.now_ms();
                if t >= self.fps_t0 + 1000 {
                    self.fps = self.fps_count;
                    self.fps_count = 0;
                    self.fps_t0 = t;
                }
                if let (Some(v), false) = (&self.view, self.menu_active) {
                    if v.inv.open.is_some() && !self.inv_open {
                        self.open_inv();
                    } else if self.inv_open && self.inv_container && v.inv.open.is_none() {
                        self.close_inv(false);
                    }
                }
                if let Some(gui) = &self.gui {
                    use ui::options::CHAT;
                    let o = &gui.opts;
                    self.chat_vis = o.get(&CHAT[0]).parse().unwrap_or(0);
                    self.chat_colors = o.get(&CHAT[1]) == "true";
                    self.chat.sg.auto = o.get(&CHAT[4]) == "true";
                    self.chat_links = o.get(&CHAT[2]) == "true";
                    self.chat_links_prompt = o.raw("chatLinksPrompt") != Some("false");
                    self.show_subtitles = o.raw("showSubtitles") == Some("true");
                    ui::HIDE_SPLASH.store(
                        o.acc_bool("hideSplashTexts"),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    toasts::set_display_mult(o.acc("notificationDisplayTime"));
                    self.chat_delay = f64::from(o.acc("chatDelay"));
                    self.sub_bg = if o.acc_bool("backgroundForChatOnly") {
                        0.8
                    } else {
                        o.f32(&CHAT[6])
                    };
                    self.chat_cfg = chat::Cfg {
                        opacity: o.f32(&CHAT[5]),
                        bg: o.f32(&CHAT[6]),
                        scale: o.f32(&CHAT[7]).max(0.01),
                        width: o.f32(&CHAT[8]),
                        h_focused: o.f32(&CHAT[9]),
                        h_unfocused: o.f32(&CHAT[10]),
                        spacing: o.f32(&CHAT[11]),
                    };
                }
                if let Some(v) = &mut self.view {
                    v.chat_vis = self.chat_vis;
                    for (id, s, l, items) in std::mem::take(&mut v.sugg_in) {
                        self.chat.sg.on_reply(id, s, l, items, &self.chat.edit);
                    }
                    if self.chat.open {
                        if let Some((id, text)) = self.chat.sg.on_edit(&self.chat.edit) {
                            v.send(netthread::NetCmd::Suggest(id, text));
                        }
                    }
                    // VANILLA: LocalPlayer#openItemGui — перо в руке: экран правки книги (только клиент).
                    if let Some((slot, pages)) = v.book_in.take() {
                        if let Some(g) = &mut self.gui {
                            g.open_book(slot, pages, &v.own_name);
                            self.menu_active = true;
                            self.input.release_all();
                            self.keys.release_all();
                            if let Some(w) = &self.window {
                                self.mouse.release(w);
                            }
                        }
                    }
                    for (p, front) in std::mem::take(&mut v.sign_in) {
                        let info =
                            v.net.level.lock().ok().and_then(|g| {
                                g.as_ref().and_then(|l| sign::edit_info(l, p, front))
                            });
                        if let (Some((name, lines, color)), Some(g)) = (info, &mut self.gui) {
                            g.open_sign(p, front, &name, &lines, color);
                            self.menu_active = true;
                            self.input.release_all();
                            self.keys.release_all();
                            if let Some(w) = &self.window {
                                self.mouse.release(w);
                            }
                        }
                    }
                    // VANILLA: LocalPlayer#openCommandBlock + CommandBlockEditScreen#updateGui.
                    let cmd_open = std::mem::take(&mut v.cmd_in);
                    let be_changed = std::mem::take(&mut v.be_changed);
                    if let Some(g) = &mut self.gui {
                        let fill = |g: &mut ui::Gui, p: [i32; 3]| {
                            if let Ok(lv) = v.net.level.lock() {
                                if let Some(l) = lv.as_ref() {
                                    let cond =
                                        l.chunks.block(p[0], p[1], p[2]).property("conditional")
                                            == Some("true");
                                    g.cmd_block_data(p, cond, berscan::block_entity_tag(l, p));
                                }
                            }
                        };
                        if let Some((p, name)) = cmd_open.last().copied() {
                            if name == "test_block" {
                                let tag = v.net.level.lock().ok().and_then(|lv| {
                                    lv.as_ref()
                                        .and_then(|l| berscan::block_entity_tag(l, p).cloned())
                                });
                                g.open_testblock(p, tag.as_ref());
                            } else if name == "jigsaw" {
                                let tag = v.net.level.lock().ok().and_then(|lv| {
                                    lv.as_ref()
                                        .and_then(|l| berscan::block_entity_tag(l, p).cloned())
                                });
                                let orient = v
                                    .net
                                    .level
                                    .lock()
                                    .ok()
                                    .and_then(|lv| {
                                        lv.as_ref().and_then(|l| {
                                            l.chunks.block(p[0], p[1], p[2]).property("orientation")
                                        })
                                    })
                                    .unwrap_or("north_up");
                                g.open_jigsaw(p, orient, tag.as_ref());
                            } else if name == "structure_block" {
                                let tag = v.net.level.lock().ok().and_then(|lv| {
                                    lv.as_ref()
                                        .and_then(|l| berscan::block_entity_tag(l, p).cloned())
                                });
                                g.open_structure(p, tag.as_ref());
                            } else {
                                g.open_cmd_block(p, name);
                                fill(g, p);
                            }
                            self.menu_active = true;
                            self.input.release_all();
                            self.keys.release_all();
                            if let Some(w) = &self.window {
                                self.mouse.release(w);
                            }
                        }
                        for p in be_changed {
                            if g.cmd_block_pos() == Some(p) {
                                fill(g, p);
                            }
                        }
                    }
                    for d in std::mem::take(&mut v.dialog_in) {
                        if let Some(g) = &mut self.gui {
                            match d {
                                Some(d) => {
                                    g.open_dialog(d);
                                    self.menu_active = true;
                                    self.input.release_all();
                                    self.keys.release_all();
                                    if let Some(w) = &self.window {
                                        self.mouse.release(w);
                                    }
                                }
                                None => {
                                    if g.close_dialog() {
                                        self.menu_active = false;
                                        if let Some(w) = &self.window {
                                            self.mouse.grab(w);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    while self
                        .chat_pending
                        .front()
                        .is_some_and(|p| p.0 <= std::time::Instant::now())
                    {
                        if let Some((_, c, tag)) = self.chat_pending.pop_front() {
                            self.chat.add(
                                &c,
                                self.tick_count,
                                tag,
                                &self.item_lang,
                                self.chat_colors,
                            );
                        }
                    }
                    for (c, tag) in std::mem::take(&mut v.chat_in) {
                        match tag {
                            Some(tag) if self.chat_delay > 0.0 => {
                                // VANILLA: ChatListener#handleMessage — отложенная выдача (chatDelay).
                                self.chat_pending.push_back((
                                    std::time::Instant::now()
                                        + std::time::Duration::from_secs_f64(self.chat_delay),
                                    c,
                                    tag,
                                ));
                            }
                            Some(tag) => self.chat.add(
                                &c,
                                self.tick_count,
                                tag,
                                &self.item_lang,
                                self.chat_colors,
                            ),
                            None => {
                                // VANILLA: Gui#setOverlayMessage — 60 тиков; Component со своими
                                // цветами (опция chatColors на action bar не влияет).
                                let m = chat::flatten(&c, &self.item_lang, true);
                                self.overlay = Some((
                                    m,
                                    std::time::Instant::now()
                                        + std::time::Duration::from_millis(3000),
                                ));
                                self.overlay_anim = false;
                            }
                        }
                    }
                }
                if let Some(w) = &self.window {
                    self.mouse.finish_release(w);
                }
                if let Some((x, y)) = self.mouse.recentered.take() {
                    self.cursor_px = (x, y);
                    self.inv_cursor_moved();
                    if let Some(g) = &mut self.gui {
                        g.on_cursor(x, y);
                    }
                }
                let (Some(g), Some(sc)) = (&self.gpu, &mut self.swapchain) else {
                    return;
                };
                // VANILLA: Minecraft#runTick — до MAX_TICKS_PER_UPDATE тиков за кадр, остаток отбрасывается.
                let n = self.timer.advance_time(self.clock.now_ms(), true);
                for _ in 0..n.min(timer::MAX_TICKS_PER_UPDATE) {
                    self.tick_count += 1;
                    // VANILLA: Hud#tick — таймер титров.
                    if let Some(v) = &mut self.view {
                        v.titles.tick();
                    }
                }
                // VANILLA: pauseGame(false) — меню паузы появится с GUI (гл. 11); пока курсор освобождён.
                if self.focus.frame(self.clock.now_ms()) {
                    if let Some(w) = &self.window {
                        self.mouse.release(w);
                    }
                }
                let turn = self.mouse.take_turn();
                let turn = if self.chat.open {
                    Default::default()
                } else {
                    turn
                };
                let (mut y, p) = input::apply_turn(self.yaw, self.pitch, turn);
                // VANILLA: Entity#turn → vehicle.onPassengerTurned → AbstractBoat#clampRotation (±105° от носа).
                if let Some(by) = self.view.as_ref().and_then(|v| v.ride_boat_yaw) {
                    y = boat::clamp_passenger_yaw(y, by);
                }
                self.yaw = y;
                self.pitch = p;
                let gui_partial = self.timer.partial_tick(false);
                // VANILLA: FramerateLimitTracker + limitDisplayFPS.
                let now = self.clock.now_ms();
                let iconified = self
                    .window
                    .as_ref()
                    .and_then(|w| w.is_minimized())
                    .unwrap_or(false);
                let t = self.fps_limit.throttle(now, iconified, false, false);
                let mut cap = self.fps_limit.effective(t);
                if self.applied_vsync == Some(true) {
                    // vsync эмулируется Mailbox + лимитером по частоте монитора.
                    let hz = self
                        .window
                        .as_ref()
                        .and_then(|w| w.current_monitor())
                        .and_then(|m| m.refresh_rate_millihertz())
                        .map_or(60, |m| (m + 500) / 1000);
                    cap = if cap == 0 { hz } else { cap.min(hz) };
                }
                if !self.focus.active || self.occluded {
                    cap = if cap == 0 { 60 } else { cap.min(60) };
                }
                self.limiter.wait(cap);
                let now_ms = self.clock.now_ms();
                let dt = (now_ms.saturating_sub(self.last_ms)) as f64 / 1000.0;
                self.last_ms = now_ms;
                if let Some(v) = &mut self.view {
                    v.yaw = self.yaw;
                    // Статус рендера в заголовке окна; меняется только при смене режима (F8).
                    let label = v.render_label();
                    if self.title_label != label {
                        self.title_label = label;
                        if let Some(w) = &self.window {
                            w.set_title(&format!("ezzmc — {label}"));
                        }
                    }
                    if !self.tour.is_empty() {
                        v.no_send = true;
                        if v.have_pos && v.resident_sections() > 0 {
                            if self.tour_t == 0 {
                                self.tour_t = now_ms;
                            }
                            let wait = if self.tour_i == 0 { 5000 } else { 1500 };
                            if let Some(&[x, z]) = self.tour.get(self.tour_i) {
                                v.cam = [x, 211.0, z];
                                self.yaw = 0.0;
                                self.pitch = 89.9;
                                v.yaw = 0.0;
                                if now_ms >= self.tour_t + wait {
                                    self.want_screenshot = true;
                                    self.tour_i += 1;
                                    self.tour_t = now_ms;
                                }
                            } else if now_ms >= self.tour_t + 1500 {
                                el.exit();
                            }
                        }
                    }
                    v.pitch = self.pitch;
                    let down = |keys: &keymap::KeyBindings, n: &str| {
                        keys.get(n).is_some_and(keymap::Binding::is_down)
                    };
                    let ax = |p: bool, m: bool| f32::from(u8::from(p)) - f32::from(u8::from(m));
                    // VANILLA: Minecraft#setScreen(≠null) → KeyMapping.releaseAll: любой открытый экран
                    // (инвентарь, чат, пауза, креатив…) отпускает ЛКМ/ПКМ/движение, ломание прерывается.
                    if self.was_grabbed && !self.mouse.grabbed {
                        self.keys.release_all();
                        self.input.release_all();
                        v.attack_held = false;
                    }
                    self.was_grabbed = self.mouse.grabbed;
                    if self.mouse.grabbed && self.tour.is_empty() {
                        if v.walking {
                            // VANILLA: Minecraft#startAttack → player.swing(MAIN_HAND) на каждый клик.
                            // VANILLA: Minecraft#handleKeybinds — при isUsingItem: отпущенный use → release,
                            // клики attack/use/pickItem проглатываются.
                            let using = v.using.is_some();
                            if using && !down(&self.keys, "key.use") {
                                v.release_using();
                            }
                            let using = v.using.is_some();
                            if let Some(b) = self.keys.get_mut("key.attack") {
                                while b.consume_click() {
                                    if !using {
                                        v.click_attack();
                                    }
                                }
                            }
                            // VANILLA: Minecraft#pickBlockOrEntity (key.pickItem) — блок под прицелом.
                            let ctrl = self.input.key_down(KeyCode::ControlLeft);
                            if let Some(b) = self.keys.get_mut("key.pickItem") {
                                while b.consume_click() {
                                    if v.game_mode != 3 && !using {
                                        if let Some(t) = v.target {
                                            v.send(netthread::NetCmd::PickBlock(
                                                t.pos,
                                                ctrl && v.game_mode == 1,
                                            ));
                                        }
                                    }
                                }
                            }
                            if let Some(b) = self.keys.get_mut("key.use") {
                                while b.consume_click() {
                                    if !using {
                                        v.use_item();
                                    }
                                }
                            }
                            // VANILLA: Minecraft#handleKeybinds — удержание ПКМ повторяет use каждые 4 тика.
                            if down(&self.keys, "key.use")
                                && v.right_click_delay == 0
                                && v.using.is_none()
                            {
                                v.use_item();
                            }
                            v.attack_held = down(&self.keys, "key.attack");
                            v.walk(
                                dt,
                                physics::Input {
                                    forward: down(&self.keys, "key.forward"),
                                    back: down(&self.keys, "key.back"),
                                    left: down(&self.keys, "key.left"),
                                    right: down(&self.keys, "key.right"),
                                    jump: down(&self.keys, "key.jump"),
                                    sneak: down(&self.keys, "key.sneak"),
                                    sprint: down(&self.keys, "key.sprint"),
                                },
                            );
                        } else {
                            v.fly(
                                dt.min(0.1) * self.fly_mult,
                                (
                                    ax(
                                        down(&self.keys, "key.forward"),
                                        down(&self.keys, "key.back"),
                                    ),
                                    ax(down(&self.keys, "key.right"), down(&self.keys, "key.left")),
                                    ax(down(&self.keys, "key.jump"), down(&self.keys, "key.sneak")),
                                ),
                                down(&self.keys, "key.sprint"),
                            );
                        }
                    } else if v.walking && self.tour.is_empty() {
                        // VANILLA: Minecraft#setScreen → KeyMapping.releaseAll — открытый экран отпускает
                        // атаку, ломание блока прерывается (Digger получает attack=false → ABORT).
                        v.attack_held = false;
                        v.walk(dt, physics::Input::default());
                    }
                    v.zoom_held = down(&self.keys, "key.zoom") && !self.chat.open;
                    v.tick_zoom(dt);
                    self.mouse.zoom_scale = f64::from(v.zoom_factor);
                    v.update(g);
                    // Вид — владелец поворота после тика (клэмп в лодке, телепорт, взгляд по носу при посадке).
                    self.yaw = v.yaw + std::mem::take(&mut v.render_yaw_off);
                    self.pitch = v.pitch;
                    // VANILLA: Camera#setup спящего — ориентация кровати вместо поворота головы.
                    if let Some((y, p)) = v.sleep_camera() {
                        (self.yaw, self.pitch, v.yaw, v.pitch) = (y, p, y, p);
                    }
                    // Отладка: EZZMC_AUTOSHOT=<сек> — скриншот и выход через N секунд после появления мира.
                    if let Some(sec) = autoshot_secs() {
                        let n = v.resident_sections();
                        if n > 0 && self.autoshot_at.is_none() {
                            self.autoshot_at = Some(now_ms + sec * 1000);
                        }
                        if let Some(t) = self.autoshot_at {
                            if now_ms >= t && !self.autoshot_done {
                                self.autoshot_done = true;
                                self.want_screenshot = true;
                                tracing::info!("autoshot: секций в арене {n}");
                            } else if self.autoshot_done && now_ms >= t + 1500 {
                                el.exit();
                            }
                        }
                    }
                }
                let t_acq = std::time::Instant::now();
                let acq = sc.acquire(g);
                mobs::prof_phase(3, "acquire", t_acq.elapsed());
                mobs::prof_frame();
                match acq {
                    Acquire::Frame(f) => {
                        let cap = std::mem::take(&mut self.want_screenshot);
                        let sky = [0.47, 0.65, 1.0];
                        // VANILLA: под DeathScreen Gui рисует хотбар/бары/чат — идём игровой веткой, экран смерти дорисовывает GUI.
                        let mut death_act = ui::Action::None;
                        let shot = if let (Some(v), true) =
                            (&mut self.view, !self.menu_active || self.dead_screen)
                        {
                            let text = &mut self.text;
                            let in_bed = self.in_bed;
                            let v_in_bed = in_bed;
                            let chat = &mut self.chat;
                            let item_lang = &self.item_lang;
                            let tab_down =
                                self.keys.get("key.playerlist").is_some_and(|b| b.is_down());
                            let mut hud = self.painter.as_mut().zip(self.hud_rm.as_ref());
                            let inv_open = self.inv_open
                                && self.adv.is_none()
                                && self.stats_screen.is_none()
                                && self.gms.is_none();
                            let adv_screen = &mut self.adv;
                            let win_screen = &mut self.win_screen;
                            let stats_screen = &mut self.stats_screen;
                            let gms = &mut self.gms;
                            let stats_down = self.stats_down;
                            let dead = self.dead_screen;
                            let cross = (
                                v.game_mode,
                                v.target.is_some() || v.entity_target.is_some(),
                                v.attack_strength(0.0),
                                v.target_living_alive(),
                                v.attack_delay(),
                            );
                            let hs = &mut self.hud_state;
                            let stats = hudbars::Stats {
                                health: v.health,
                                max_health: v.max_health,
                                food: v.food,
                                saturation: v.saturation,
                                armor: v.armor,
                                xp_progress: v.xp_progress,
                                xp_level: v.xp_level,
                                hardcore: v.hardcore,
                                tick: self.tick_count,
                                absorption: v.absorption,
                                // VANILLA: Hud.HeartType#forPlayer — иссушение важнее отравления.
                                heart_kind: if v.effects.has("wither") {
                                    2
                                } else if v.effects.has("poison") {
                                    1
                                } else {
                                    0
                                },
                                regen: v.effects.has("regeneration"),
                            };
                            // VANILLA: gameMode.canHurtPlayer/hasExperience — выживание и приключение.
                            let survival = matches!(v.game_mode, 0 | 2);
                            let xp_recent = v
                                .xp_changed_at
                                .is_some_and(|t| t.elapsed().as_secs_f32() < 5.0);
                            let loc_bar =
                                hudbars::show_locator(!v.waypoints.is_empty(), survival, xp_recent);
                            let loc_wps = if loc_bar {
                                let mut wps = v.waypoints.clone();
                                if let Ok(g) = v.net.level.lock() {
                                    if let Some(l) = g.as_ref() {
                                        let pt = f64::from(l.partial_tick());
                                        for w in wps.values_mut() {
                                            let hudbars::WpKind::Pos(x, y, z) = w.kind else {
                                                continue;
                                            };
                                            let Ok(u) = u128::from_str_radix(&w.key, 16) else {
                                                continue;
                                            };
                                            // VANILLA: Vec3iWaypoint#position — manhattan ≤ 3 от вектора.
                                            if let Some(e) = l.entities.iter().find(|e| {
                                                e.uuid == u
                                                    && (e.pos[0].floor() as i32 - x).abs()
                                                        + (e.pos[1].floor() as i32 - y).abs()
                                                        + (e.pos[2].floor() as i32 - z).abs()
                                                        <= 3
                                            }) {
                                                w.eye = Some([
                                                    e.prev_pos[0] + (e.pos[0] - e.prev_pos[0]) * pt,
                                                    e.prev_pos[1]
                                                        + (e.pos[1] - e.prev_pos[1]) * pt
                                                        + 1.62,
                                                    e.prev_pos[2] + (e.pos[2] - e.prev_pos[2]) * pt,
                                                ]);
                                            }
                                        }
                                    }
                                }
                                wps
                            } else {
                                Default::default()
                            };
                            let (loc_cam, loc_yaw, loc_pitch, loc_fov) =
                                (v.cam, v.yaw, v.pitch, v.fov_deg());
                            // VANILLA: Hud#setNowPlaying — translatable record.nowPlaying, 60 тиков, анимация цвета.
                            if let Some(id) = v.now_playing.take() {
                                if let Some(d) = tooltip::song_description(&self.item_lang, id) {
                                    let msg = self
                                        .item_lang
                                        .tr("record.nowPlaying")
                                        .replacen("%s", &d, 1)
                                        .replacen("%1$s", &d, 1);
                                    self.overlay = Some((
                                        vec![chat::Span::plain(msg)],
                                        std::time::Instant::now()
                                            + std::time::Duration::from_millis(3000),
                                    ));
                                    self.overlay_anim = true;
                                }
                            }
                            let overlay_anim = self.overlay_anim;
                            let overlay = self
                                .overlay
                                .as_ref()
                                .filter(|(_, t)| std::time::Instant::now() < *t)
                                .map(|(m, t)| {
                                    let left =
                                        t.saturating_duration_since(std::time::Instant::now());
                                    (m.clone(), left.as_secs_f32() * 20.0)
                                });
                            let drag_prev = self
                                .drag
                                .as_ref()
                                .filter(|d| d.2)
                                .map(|d| (d.0, d.1.clone()));
                            let inv_creative = self.inv_creative;
                            let creative = &self.creative;
                            // Прогрев при входе в мир (а не при первом открытии инвентаря): иначе микрофриз.
                            if self.char_preview.is_none() {
                                self.char_preview = hud
                                    .as_ref()
                                    .map(|(_, rm)| charpreview::CharPreview::new(rm));
                            }
                            if inv_open {
                                if let Some(cp) = self.char_preview.as_mut() {
                                    cp.set_snap(v.preview_snapshot());
                                }
                            }
                            let preview = &mut self.char_preview;
                            if self.ench_book.is_none() {
                                self.ench_book =
                                    hud.as_ref().map(|(_, rm)| enchbook::EnchBook::new(rm));
                            }
                            let ench_book = &mut self.ench_book;
                            let anvil = &mut self.anvil;
                            let lang = &self.item_lang;
                            let cur = self.cursor_px;
                            let fps = self.fps;
                            let gui_scale_opt = self.gui.as_ref().map(|g| {
                                g.opts.f32(&ui::options::VIDEO[ui::options::GUI_SCALE]) as u32
                            });
                            let dead_now = self.dead_screen;
                            let music_toast = self
                                .gui
                                .as_ref()
                                .and_then(|g| g.opts.raw("musicToast"))
                                .unwrap_or("")
                                .to_owned();
                            let mut gui_dead = self.gui.as_mut().filter(|_| dead_now);
                            let debug_on = self.debug_on;
                            let hid = self.hud_hidden;
                            let fps_limit = self.fps_limit.limit();
                            let pitch = self.pitch;
                            let vsync = std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0");
                            let shot_in = sc.draw_and_present(g, f, cap, |view, w, h| {
                                // VANILLA: 3D-прицел виден при открытом F3 (DebugScreenEntries#THREE_DIMENSIONAL_CROSSHAIR).
                                v.hud_hidden = hid;
                                v.debug_crosshair = (debug_on && !hid).then(|| {
                                    ezzmc_render::text::gui_scale_opt(
                                        w,
                                        h,
                                        gui_scale_opt.unwrap_or(0),
                                    )
                                });
                                v.render(g, view, w, h, sky);
                                if let Some(t) = text {
                                    // VANILLA: guiScale из настроек действует и в игре (раньше
                                    // применялся только после показа меню).
                                    if let Some(o) = gui_scale_opt {
                                        t.set_scale_option(o);
                                    }
                                    t.begin(w, h);
                                    let (gw, gh) = t.gui_size(w, h);
                                    if let Some((p, rm)) = hud.as_mut() {
                                        let (gwf, ghf) = (gw as f32, gh as f32);
                                        hud::new_frame();
                                        p.begin(t.scale());
                                        // VANILLA: ScreenEffectRenderer#submitBlockSprite.
                                        if let Some(sp) = v.block_overlay() {
                                            hud::draw_block_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                v.blocks_atlas(),
                                                gwf,
                                                ghf,
                                                sp,
                                            );
                                        }
                                        // VANILLA: ScreenEffectRenderer#submitWater.
                                        if let Some(br) = v.water_overlay() {
                                            hud::draw_water_overlay(
                                                p, &g.device, &g.queue, rm, gwf, ghf, br, v.yaw,
                                                v.pitch,
                                            );
                                        }
                                        // VANILLA: ScreenEffectRenderer#submitFire.
                                        if let Some(fr) = v.fire_overlay_frame() {
                                            hud::draw_fire_overlay(
                                                p, &g.device, &g.queue, rm, gwf, ghf, fr,
                                            );
                                        }
                                        // VANILLA: Hud#extractCameraOverlays — подзорная труба вместо тыквы.
                                        if v.is_scoping() {
                                            hud::draw_spyglass_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                rm,
                                                gwf,
                                                ghf,
                                                v.scope_scale,
                                            );
                                        } else if v.pumpkin_overlay() {
                                            hud::draw_texture_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                rm,
                                                gwf,
                                                ghf,
                                                "pumpkinblur",
                                                1.0,
                                                true,
                                            );
                                        }
                                        let fz = v.frozen_percent();
                                        if fz > 0.0 {
                                            hud::draw_texture_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                rm,
                                                gwf,
                                                ghf,
                                                "powder_snow_outline",
                                                fz,
                                                false,
                                            );
                                        }
                                        // VANILLA: Hud#extractSleepOverlay.
                                        if !hid {
                                            hud::draw_sleep_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                gwf,
                                                ghf,
                                                v.sleep_timer(),
                                            );
                                        }
                                        // VANILLA: Hud#extractPortalOverlay.
                                        let pin = v.portal_intensity_now();
                                        if pin > 0.0 && !hid {
                                            hud::draw_portal_overlay(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                rm,
                                                gwf,
                                                ghf,
                                                pin,
                                                v.portal_frame(),
                                            );
                                        }
                                        // VANILLA: Hud#extractCrosshair — при 3D-прицеле (F3) плоский не рисуется.
                                        if !inv_open
                                            && !dead
                                            && !hid
                                            && !(debug_on && v.perspective == 0)
                                        {
                                            // attackIndicator: 1 = у прицела (иначе только сам прицел).
                                            let ind = v.cfg.attack_indicator;
                                            hud::draw_crosshair(
                                                p,
                                                gwf,
                                                ghf,
                                                cross.0,
                                                cross.1,
                                                if ind == 1 { cross.2 } else { 1.0 },
                                                cross.3 && ind == 1,
                                                cross.4,
                                            );
                                        }
                                        // VANILLA: Hud#extractHotbarAndDecorations — в spectator вместо
                                        // хотбара предметов SpectatorGui#extractHotbar (пуст без меню).
                                        let spectator = v.game_mode == 3;
                                        if !hid && !spectator {
                                            hud::draw_hotbar(
                                                p, t, &g.device, &g.queue, rm, &v.inv, gwf, ghf,
                                            );
                                        }
                                        if v.cfg.attack_indicator == 2 && !hid && !spectator {
                                            hud::draw_hotbar_attack(
                                                p,
                                                gwf,
                                                ghf,
                                                cross.2,
                                                v.cfg.left_handed,
                                            );
                                        }
                                        if survival && !hid {
                                            hudbars::draw_health(p, hs, &stats, gwf, ghf);
                                        }
                                        if !inv_open && !hid {
                                            // VANILLA: Hud#extractEffects
                                            let icons = effectshud::layout(v.effects.iter(), gwf);
                                            effectshud::draw(p, &icons);
                                        }
                                        if loc_bar && !hid {
                                            hudbars::draw_locator(
                                                p, &loc_wps, loc_cam, loc_yaw, loc_pitch, loc_fov,
                                                gwf, ghf,
                                            );
                                        } else if survival && !hid {
                                            hudbars::draw_xp_bar(p, &stats, gwf, ghf);
                                        }
                                        // VANILLA: InBedChatScreen — спрайт кнопки «Покинуть кровать» (текст рисуется с чатом).
                                        if v_in_bed && !hid {
                                            let (bx, by, bw, bh) = Self::leave_bed_rect(gwf, ghf);
                                            let k = f64::from(w) / f64::from(gwf);
                                            let (mx, my) = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                            let hov = mx >= bx
                                                && mx < bx + bw
                                                && my >= by
                                                && my < by + bh;
                                            p.nine_slice(
                                                if hov { "button_hl" } else { "button" },
                                                bx,
                                                by,
                                                bw,
                                                bh,
                                                [200.0, 20.0],
                                                [3.0; 4],
                                                0xFFFF_FFFF,
                                            );
                                        }
                                        p.flush(&g.device, &g.queue, view, w, h, None);
                                        if survival && !hid {
                                            hudbars::draw_xp_level(t, stats.xp_level, gwf, ghf);
                                        }
                                        if !hid {
                                            v.draw_nametags(t, item_lang, gwf, ghf);
                                            v.draw_signs(t, w, h);
                                            v.draw_map_labels(t, item_lang, w, h);
                                        }
                                        if !hid {
                                            // VANILLA: Hud#extractScoreboardSidebar — до overlay/титров.
                                            v.scoreboard.draw_sidebar(
                                                t,
                                                item_lang,
                                                &self.player_name,
                                                gwf,
                                                ghf,
                                            );
                                        }
                                        if let (Some((m, ticks)), false) = (&overlay, hid) {
                                            // VANILLA: Hud#extractOverlayMessage — над хотбаром (gh − 68),
                                            // alpha = t·255/20, при animate цвет hsv(t/50, 0.7, 0.6); тень,
                                            // подложки нет (backgroundForChatOnly=true по умолчанию).
                                            let alpha = ((ticks * 255.0 / 20.0) as i32).min(255);
                                            if alpha > 0 {
                                                let a = alpha as f32 / 255.0;
                                                let mw = titles::spans_width(t, m).floor();
                                                let (x, y) = (
                                                    (gwf / 2.0).floor() - (mw / 2.0).floor(),
                                                    ghf - 68.0 - 4.0,
                                                );
                                                if overlay_anim {
                                                    let rgb = hsv_rgb(ticks / 50.0, 0.7, 0.6);
                                                    let m: Vec<chat::Span> = m
                                                        .iter()
                                                        .map(|s| chat::Span {
                                                            color: rgb,
                                                            ..s.clone()
                                                        })
                                                        .collect();
                                                    titles::draw_spans(t, &m, x, y, a, true);
                                                } else {
                                                    titles::draw_spans(t, m, x, y, a, true);
                                                }
                                            }
                                        }
                                        if !hid {
                                            // VANILLA: Hud#extractTitle — после overlay, до чата.
                                            v.titles.draw(t, item_lang, gwf, ghf, gui_partial);
                                        }
                                        if !hid {
                                            let (ccfg, tick) = (self.chat_cfg, self.tick_count);
                                            chat.draw(t, ghf, tick, &ccfg);
                                            if chat.open {
                                                chat.draw_input(t, gwf, ghf, ccfg.bg);
                                                if in_bed {
                                                    // VANILLA: InBedChatScreen — кнопка «Покинуть кровать».
                                                    let (bx, by, bw, bh) =
                                                        Self::leave_bed_rect(gwf, ghf);
                                                    let (mx, my) = (
                                                        (cur.0 / f64::from(t.scale())) as f32,
                                                        (cur.1 / f64::from(t.scale())) as f32,
                                                    );
                                                    let hov = mx >= bx
                                                        && mx < bx + bw
                                                        && my >= by
                                                        && my < by + bh;
                                                    let label = self
                                                        .item_lang
                                                        .fmt("multiplayer.stopSleeping", &[]);
                                                    let lw = t.width_fmt(&label, false);
                                                    t.draw(
                                                        &label,
                                                        bx + (bw - lw) / 2.0,
                                                        by + 6.0,
                                                        if hov { 0xFFFF_A0 } else { 0xFF_FFFF },
                                                        true,
                                                    );
                                                }
                                                // VANILLA: ChatComponent#extractDeferredElements — рука над текстом с clickEvent.
                                                let k = f64::from(t.scale());
                                                let (mx, my) =
                                                    ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                                if chat
                                                    .span_at(t, ghf, mx, my, &ccfg)
                                                    .is_some_and(|s| s.click.is_some())
                                                {
                                                    cursor::request(cursor::Kind::PointingHand);
                                                }
                                                let tip = chat.hover_lines(
                                                    t,
                                                    gwf,
                                                    ghf,
                                                    mx,
                                                    my,
                                                    &ccfg,
                                                    &self.item_lang,
                                                );
                                                tooltip::draw_colored(
                                                    t,
                                                    &tip,
                                                    cur.0 as f32 / t.scale() as f32,
                                                    cur.1 as f32 / t.scale() as f32,
                                                    gwf,
                                                    ghf,
                                                );
                                            }
                                        }
                                        {
                                            // VANILLA: Hud — SubtitleOverlay (слушатель живёт, пока включена опция).
                                            // VANILLA-UNSURE: notificationDisplayTime = 1.0 и фон 0.8
                                            // (backgroundForChatOnly по умолчанию) — опций нет.
                                            let (cam, yaw, pitch) = (v.cam, v.yaw, v.pitch);
                                            v.subtitle_rows = v.subtitles.visible(
                                                self.show_subtitles,
                                                item_lang,
                                                cam,
                                                yaw,
                                                pitch,
                                                f64::from(toasts::display_mult()),
                                            );
                                            if !hid {
                                                subtitles::Subtitles::draw(
                                                    t,
                                                    &v.subtitle_rows,
                                                    gwf,
                                                    ghf,
                                                    self.sub_bg,
                                                );
                                            }
                                        }
                                        // VANILLA: BossHealthOverlay — полосы сверху (текст, затем спрайты).
                                        if !hid && !v.boss.is_empty() {
                                            let bars = v.boss.draw(t, item_lang, gwf, ghf);
                                            t.flush(&g.device, &g.queue, view, w, h);
                                            t.begin(w, h);
                                            p.begin(t.scale());
                                            for (s, x, y, bw) in bars {
                                                p.blit(
                                                    s,
                                                    x,
                                                    y,
                                                    bw,
                                                    5.0,
                                                    [0.0, 0.0, bw / 182.0, 1.0],
                                                    0xFFFFFFFF,
                                                );
                                            }
                                            p.flush(&g.device, &g.queue, view, w, h, None);
                                        }
                                        // VANILLA: Hud#extractTabList — пока зажат key.playerlist (не при открытом чате/инвентаре).
                                        if tab_down && !hid && !inv_open && !chat.open && !dead {
                                            let icons = v.tab.draw(
                                                t,
                                                item_lang,
                                                gwf,
                                                &v.scoreboard,
                                                self.tick_count,
                                            );
                                            t.flush(&g.device, &g.queue, view, w, h);
                                            t.begin(w, h);
                                            p.begin(t.scale());
                                            for (x, y, s, sw, sh) in icons {
                                                p.blit(
                                                    s,
                                                    x,
                                                    y,
                                                    sw,
                                                    sh,
                                                    [0.0, 0.0, 1.0, 1.0],
                                                    0xFFFFFFFF,
                                                );
                                            }
                                            // VANILLA: PlayerFaceExtractor — лицо (8,8) + шапка (40,8) 8×8 из скина.
                                            for (x, y, url) in v.tab.heads.borrow().iter() {
                                                let name = match url {
                                                    Some(u) => {
                                                        let sk = skins::global();
                                                        sk.request(u);
                                                        let n = skins::tex_name(u);
                                                        if !p.has(&n) {
                                                            if let Some(img) = sk.get(u) {
                                                                p.add_texture(
                                                                    &g.device, &g.queue, &n, &img,
                                                                    64, 64, false,
                                                                );
                                                            }
                                                        }
                                                        if p.has(&n) {
                                                            n
                                                        } else {
                                                            "soc_steve".to_owned()
                                                        }
                                                    }
                                                    None => "soc_steve".to_owned(),
                                                };
                                                for u in [8.0, 40.0] {
                                                    p.blit(
                                                        &name,
                                                        *x,
                                                        *y,
                                                        8.0,
                                                        8.0,
                                                        [
                                                            u / 64.0,
                                                            8.0 / 64.0,
                                                            (u + 8.0) / 64.0,
                                                            16.0 / 64.0,
                                                        ],
                                                        0xFFFFFFFF,
                                                    );
                                                }
                                            }
                                            p.flush(&g.device, &g.queue, view, w, h, None);
                                        }
                                        if inv_open {
                                            let k = f64::from(t.scale());
                                            let (mx, my) = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                            let lay = invscreen::layout(&v.inv);
                                            invscreen::draw_dim(t, gwf, ghf);
                                            t.flush(&g.device, &g.queue, view, w, h);
                                            t.begin(w, h);
                                            p.begin(t.scale());
                                            if inv_creative {
                                                let pv;
                                                let (invr, dsl): (&_, &[usize]) =
                                                    if let Some((r, sl)) = &drag_prev {
                                                        let mut c = v.inv.clone();
                                                        c.predict_quick_craft(sl, *r);
                                                        pv = c;
                                                        (&pv, sl)
                                                    } else {
                                                        (&v.inv, &[])
                                                    };
                                                creativescreen::draw_sprites(
                                                    p,
                                                    &g.device,
                                                    &g.queue,
                                                    rm,
                                                    creative,
                                                    invr,
                                                    t.scale(),
                                                    preview.as_mut(),
                                                    dsl,
                                                    gwf,
                                                    ghf,
                                                    mx,
                                                    my,
                                                );
                                                p.flush(&g.device, &g.queue, view, w, h, None);
                                                creativescreen::draw_text(
                                                    t, creative, invr, lang, gwf, ghf,
                                                );
                                                if let Some(c) = &invr.cursor {
                                                    hud::cursor_item(
                                                        p,
                                                        t,
                                                        g,
                                                        view,
                                                        (w, h),
                                                        rm,
                                                        c,
                                                        mx,
                                                        my,
                                                    );
                                                }
                                                if v.inv.cursor.is_none() {
                                                    if let Some(n) = creativescreen::hover_text(
                                                        &*t, creative, &v.inv, lang, gwf, ghf, mx,
                                                        my,
                                                    ) {
                                                        tooltip::draw_colored(
                                                            t, &n, mx, my, gwf, ghf,
                                                        );
                                                    }
                                                }
                                            } else if lay.is_lectern() {
                                                if let Some(o) = &v.inv.open {
                                                    let mb = v.game_mode < 2;
                                                    bookview::draw_sprites(
                                                        p, &*t, lang, o, gwf, mx, my, mb,
                                                    );
                                                    p.flush(&g.device, &g.queue, view, w, h, None);
                                                    bookview::draw_text(t, lang, o, gwf, mb);
                                                }
                                            } else {
                                                // VANILLA: во время перетаскивания видно раздачу по слотам.
                                                recipescreen::sync_names(&mut v.recipes, lang);
                                                let pv;
                                                let (invr, dsl): (&_, &[usize]) =
                                                    if let Some((r, sl)) = &drag_prev {
                                                        let mut c = v.inv.clone();
                                                        c.predict_quick_craft(sl, *r);
                                                        pv = c;
                                                        (&pv, sl)
                                                    } else {
                                                        (&v.inv, &[])
                                                    };
                                                if let (true, Some(o)) =
                                                    (lay.is_anvil(), &invr.open)
                                                {
                                                    if let Some(n) = anvil.sync(lang, o) {
                                                        v.send_rename_item(n);
                                                    }
                                                }
                                                invscreen::draw_sprites(
                                                    p, &g.device, &g.queue, rm, invr, &lay, gwf,
                                                    ghf, dsl, mx, my,
                                                );
                                                recipescreen::draw_ghost(
                                                    p,
                                                    &v.ghost,
                                                    &lay,
                                                    gwf,
                                                    ghf,
                                                    now_ms as u128,
                                                    &mut |p, s, x, y| {
                                                        hud::stack_icon(
                                                            p, &g.device, &g.queue, rm, s, x, y,
                                                        )
                                                    },
                                                );
                                                recipescreen::draw(
                                                    p,
                                                    &v.recipes,
                                                    invr,
                                                    &lay,
                                                    gwf,
                                                    ghf,
                                                    mx,
                                                    my,
                                                    &mut |p, s, x, y| {
                                                        hud::stack_icon(
                                                            p, &g.device, &g.queue, rm, s, x, y,
                                                        )
                                                    },
                                                );
                                                recipescreen::draw_overlay(
                                                    p,
                                                    &v.recipes,
                                                    mx,
                                                    my,
                                                    now_ms as u128,
                                                    &mut |p, s, x, y, sz| {
                                                        hud::stack_icon_sized(
                                                            p, &g.device, &g.queue, rm, s, x, y, sz,
                                                        )
                                                    },
                                                );
                                                if let (true, Some(cp)) =
                                                    (lay.is_player(), preview.as_mut())
                                                {
                                                    cp.set_armor(&invr.slots);
                                                    cp.draw(
                                                        p,
                                                        &g.device,
                                                        &g.queue,
                                                        t.scale(),
                                                        lay.left(gwf),
                                                        lay.top(ghf),
                                                        mx,
                                                        my,
                                                        charpreview::RECT,
                                                        charpreview::SIZE,
                                                    );
                                                }
                                                if let (Some(b), Some(o), true) = (
                                                    ench_book.as_mut(),
                                                    &invr.open,
                                                    lay.is_enchanting(),
                                                ) {
                                                    b.draw(
                                                        p,
                                                        &g.device,
                                                        &g.queue,
                                                        t.scale(),
                                                        lay.left(gwf),
                                                        lay.top(ghf),
                                                        enchbook::fingerprint(o.slots.first()),
                                                        o.data[..3].iter().any(|c| *c != 0),
                                                    );
                                                }
                                                p.flush(&g.device, &g.queue, view, w, h, None);
                                                invscreen::draw_text(
                                                    t,
                                                    invr,
                                                    lang,
                                                    &lay,
                                                    gwf,
                                                    ghf,
                                                    mx,
                                                    my,
                                                    anvil,
                                                    (now_ms / 500) % 2 == 0,
                                                );
                                                recipescreen::draw_search(
                                                    t,
                                                    &mut v.recipes,
                                                    lang,
                                                    &lay,
                                                    gwf,
                                                    ghf,
                                                    (now_ms / 500) % 2 == 0,
                                                );
                                                if let Some((s, cx, cy)) = recipescreen::page_label(
                                                    &v.recipes, invr, &lay, gwf, ghf,
                                                ) {
                                                    t.draw(
                                                        &s,
                                                        cx - t.text_width(&s) / 2.0,
                                                        cy,
                                                        0xFFFFFF,
                                                        true,
                                                    );
                                                }
                                                if let Some(c) = &invr.cursor {
                                                    hud::cursor_item(
                                                        p,
                                                        t,
                                                        g,
                                                        view,
                                                        (w, h),
                                                        rm,
                                                        c,
                                                        mx,
                                                        my,
                                                    );
                                                }
                                                if v.inv.cursor.is_none() {
                                                    if let Some(st) = recipescreen::hover_stack(
                                                        &v.recipes, invr, &lay, gwf, ghf, mx, my,
                                                    ) {
                                                        let lines =
                                                            tooltip::stack_lines(lang, &*t, &st);
                                                        tooltip::draw_colored(
                                                            t, &lines, mx, my, gwf, ghf,
                                                        );
                                                    }
                                                    if let Some(st) = recipescreen::hover_ghost(
                                                        &v.ghost,
                                                        &lay,
                                                        gwf,
                                                        ghf,
                                                        now_ms as u128,
                                                        mx,
                                                        my,
                                                    ) {
                                                        let lines =
                                                            tooltip::stack_lines(lang, &*t, &st);
                                                        tooltip::draw_colored(
                                                            t, &lines, mx, my, gwf, ghf,
                                                        );
                                                    }
                                                    if let Some(n) = recipescreen::hover_filter(
                                                        &v.recipes, &lay, lang, gwf, ghf, mx, my,
                                                    ) {
                                                        tooltip::draw_colored(
                                                            t, &n, mx, my, gwf, ghf,
                                                        );
                                                    }
                                                    if let (true, Some(o)) =
                                                        (lay.is_loom(), &invr.open)
                                                    {
                                                        if let Some(n) = loomscreen::hover(
                                                            lang,
                                                            o,
                                                            lay.left(gwf),
                                                            lay.top(ghf),
                                                            mx,
                                                            my,
                                                        ) {
                                                            tooltip::draw_colored(
                                                                t, &n, mx, my, gwf, ghf,
                                                            );
                                                        }
                                                    }
                                                    if let (true, Some(o)) =
                                                        (lay.is_merchant(), &invr.open)
                                                    {
                                                        let (ml, mt) =
                                                            (lay.left(gwf), lay.top(ghf));
                                                        if let Some(st) =
                                                            merchantscreen::hover_stack(
                                                                o, ml, mt, mx, my,
                                                            )
                                                        {
                                                            let lines = tooltip::stack_lines(
                                                                lang, &*t, &st,
                                                            );
                                                            tooltip::draw_colored(
                                                                t, &lines, mx, my, gwf, ghf,
                                                            );
                                                        } else if let Some(n) =
                                                            merchantscreen::hover_text(
                                                                lang, o, ml, mt, mx, my,
                                                            )
                                                        {
                                                            tooltip::draw_colored(
                                                                t, &n, mx, my, gwf, ghf,
                                                            );
                                                        }
                                                    }
                                                    if let (true, Some(o)) =
                                                        (lay.is_beacon(), &invr.open)
                                                    {
                                                        if let Some(n) = beaconscreen::hover(
                                                            lang,
                                                            o,
                                                            lay.left(gwf),
                                                            lay.top(ghf),
                                                            mx,
                                                            my,
                                                        ) {
                                                            tooltip::draw_colored(
                                                                t, &n, mx, my, gwf, ghf,
                                                            );
                                                        }
                                                    }
                                                    if let Some(st) = invscreen::hover_item(
                                                        invr, &lay, gwf, ghf, mx, my,
                                                    ) {
                                                        let lines =
                                                            tooltip::stack_lines(lang, &*t, st);
                                                        tooltip::draw_colored(
                                                            t, &lines, mx, my, gwf, ghf,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // VANILLA: WinScreen (End Poem / титры, TODO-357/358).
                                    if let Some(ws) = win_screen.as_mut() {
                                        if ws.update(gh as i32) {
                                            tracing::info!(
                                                "WinScreen: титры закончились → respawn"
                                            );
                                            *win_screen = None;
                                            v.send_respawn();
                                        } else if let Some((p, _)) = hud.as_mut() {
                                            ws.draw(t, p, g, view, (w, h), v.level_time());
                                        }
                                    }
                                    // VANILLA: AdvancementsScreen — поверх мира, под тултипами/тостами.
                                    if let (Some(a), Some((p, rm))) =
                                        (adv_screen.as_mut(), hud.as_mut())
                                    {
                                        let k = f64::from(t.scale());
                                        let m = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                        if let Some(tab) = a.draw(
                                            &mut v.advancements,
                                            item_lang,
                                            t,
                                            p,
                                            g,
                                            view,
                                            (w, h),
                                            rm,
                                            m,
                                            dt as f32,
                                        ) {
                                            v.send(netthread::NetCmd::SeenAdvancements(Some(tab)));
                                        }
                                    }
                                    // VANILLA: GameModeSwitcherScreen — поверх мира.
                                    if let (Some(s), Some((p, rm))) = (gms.as_mut(), hud.as_mut()) {
                                        let k = f64::from(t.scale());
                                        let m = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                        s.draw(item_lang, "F4", t, p, g, view, (w, h), rm, m);
                                    }
                                    // VANILLA: StatsScreen — поверх мира.
                                    if let (Some(s), Some((p, rm))) =
                                        (stats_screen.as_mut(), hud.as_mut())
                                    {
                                        let k = f64::from(t.scale());
                                        let m = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                        s.draw(
                                            &v.stats,
                                            item_lang,
                                            t,
                                            p,
                                            g,
                                            view,
                                            (w, h),
                                            rm,
                                            m,
                                            stats_down,
                                            now_ms,
                                        );
                                    }
                                    {
                                        let hov = if inv_open && !inv_creative {
                                            let k = f64::from(t.scale());
                                            let (mx, my) = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                            invscreen::layout(&v.inv)
                                                .slot_at(gw as f32, gh as f32, mx, my)
                                        } else {
                                            None
                                        };
                                        if let Some((a, b)) = tooltip::bundle_unhover(hov) {
                                            v.send_select_bundle(a, b);
                                        }
                                    }
                                    if let Some((p, rm)) = hud.as_mut() {
                                        tooltip::draw_block(p, t, item_lang, g, view, (w, h), rm);
                                    }
                                    if debug_on {
                                        let info = g.adapter.get_info();
                                        let e = debugscreen::Env {
                                            fps,
                                            fps_limit,
                                            present: if vsync { " (fifo)" } else { " (immediate)" },
                                            yaw: v.yaw,
                                            pitch,
                                            window: (w, h),
                                            gpu: &info,
                                        };
                                        let (l, r) = debugscreen::build(v, &e);
                                        debugscreen::draw(t, &l, &r, gw as f32);
                                    }
                                    t.flush(&g.device, &g.queue, view, w, h);
                                    if !hid {
                                        v.flush_nametags(t, g, view, w, h);
                                    }
                                    if let (Some(gui), Some((p, _))) =
                                        (gui_dead.as_mut(), hud.as_mut())
                                    {
                                        death_act = gui.render(
                                            &g.device, &g.queue, view, w, h, t, p, dt as f32,
                                            now_ms,
                                        );
                                    }
                                    // VANILLA: NowPlayingToast (ToastManager#extractRenderState).
                                    if let Some((p, _)) = hud.as_mut() {
                                        let key = self.music.track_key();
                                        toasts::now_playing_frame(
                                            &mut self.now_playing,
                                            key.as_deref(),
                                            &music_toast,
                                            false,
                                            !hid,
                                            t,
                                            p,
                                            g,
                                            view,
                                            (w, h),
                                            item_lang,
                                        );
                                    }
                                    // VANILLA: ToastManager — поверх HUD и экранов, скрыт вместе с HUD (F1).
                                    if let Some((p, rm)) = hud.as_mut() {
                                        toasts::frame(
                                            v,
                                            !hid,
                                            t,
                                            p,
                                            g,
                                            view,
                                            (w, h),
                                            rm,
                                            item_lang,
                                        );
                                    }
                                }
                            });
                            match death_act {
                                ui::Action::Respawn => {
                                    if let Some(v) = &self.view {
                                        v.send(netthread::NetCmd::Respawn);
                                    }
                                }
                                ui::Action::Disconnect => {
                                    if let Some(gui) = &mut self.gui {
                                        gui.opts.save();
                                        gui.reset_title();
                                    }
                                    self.view = None;
                                    self.connect = None;
                                    self.start_prewarm();
                                }
                                _ => {}
                            }
                            shot_in
                        } else if let (true, Some(gui), Some(p), Some(t)) = (
                            self.menu_active,
                            &mut self.gui,
                            &mut self.painter,
                            &mut self.text,
                        ) {
                            let mut act = ui::Action::None;
                            if let Some(v) = &self.view {
                                gui.social_sync(social_players(v), &v.hidden_players);
                            }
                            // VANILLA: LevelLoadingScreen#extractBackground — фон портала поверх мира.
                            let portal = gui.loading_portal();
                            if portal == Some(ui::LoadingReason::EndPortal)
                                && self.loading_bg.is_none()
                            {
                                if let Some(rm) = &self.hud_rm {
                                    self.loading_bg = winscreen::end_background(
                                        rm,
                                        &g.device,
                                        &g.queue,
                                        sc.config.format,
                                    );
                                }
                            }
                            let loading_bg = &mut self.loading_bg;
                            let mut world = self.view.as_mut();
                            let shot = sc.draw_and_present(g, f, cap, |view, w, h| {
                                if let Some(v) = world.as_mut() {
                                    v.render(g, view, w, h, sky);
                                    // Мировой текст (таблички, неймтеги) виден и за меню/редактором.
                                    if !self.hud_hidden {
                                        t.begin(w, h);
                                        v.draw_signs(t, w, h);
                                        v.draw_map_labels(t, &self.item_lang, w, h);
                                        v.flush_nametags(t, g, view, w, h);
                                    }
                                }
                                match portal {
                                    Some(ui::LoadingReason::EndPortal) => {
                                        if let Some(bg) = loading_bg.as_mut() {
                                            let (gt, pt) =
                                                world.as_ref().map_or((0, 0.0), |v| v.level_time());
                                            bg.draw(&g.device, &g.queue, view, gt, pt, false);
                                        }
                                    }
                                    // VANILLA: blitSprite(GUI_OPAQUE_TEXTURED_BACKGROUND, nether_portal)
                                    // на весь экран; кадр анимации — по клиентским тикам (50 мс).
                                    Some(ui::LoadingReason::NetherPortal) => {
                                        if let Some(rm) = self.hud_rm.as_ref() {
                                            let k = t.scale();
                                            p.begin(k);
                                            hud::draw_portal_loading_bg(
                                                p,
                                                &g.device,
                                                &g.queue,
                                                rm,
                                                w as f32 / k as f32,
                                                h as f32 / k as f32,
                                                (now_ms / 50) as u32,
                                            );
                                            p.flush(&g.device, &g.queue, view, w, h, None);
                                        }
                                    }
                                    _ => {}
                                }
                                act = gui.render(
                                    &g.device, &g.queue, view, w, h, t, p, dt as f32, now_ms,
                                );
                                {
                                    let key = self.music.track_key();
                                    toasts::now_playing_frame(
                                        &mut self.now_playing,
                                        key.as_deref(),
                                        gui.opts.raw("musicToast").unwrap_or(""),
                                        gui.is_pause(),
                                        !self.hud_hidden,
                                        t,
                                        p,
                                        g,
                                        view,
                                        (w, h),
                                        &self.item_lang,
                                    );
                                }
                                if let (Some(v), Some(rm)) = (world.as_mut(), self.hud_rm.as_ref())
                                {
                                    let show = !self.hud_hidden;
                                    toasts::frame(
                                        v,
                                        show,
                                        t,
                                        p,
                                        g,
                                        view,
                                        (w, h),
                                        rm,
                                        &self.item_lang,
                                    );
                                }
                            });
                            self.fps_limit.set_limit(
                                gui.opts.f32(&ui::options::VIDEO[ui::options::MAX_FPS]) as u32,
                            );
                            for o in gui.take_dialog_out() {
                                if let Some(v) = &self.view {
                                    v.send(match o {
                                        ui::dialog::Out::Command(c) => {
                                            netthread::NetCmd::Command(c)
                                        }
                                        ui::dialog::Out::Custom(id, p) => {
                                            netthread::NetCmd::CustomClick(id, p)
                                        }
                                        ui::dialog::Out::OpenUrl(_) => continue,
                                    });
                                }
                            }
                            for (id, hide, name) in gui.take_social_toggles() {
                                // VANILLA: PlayerEntry#onHiddenOrShown → PlayerSocialManager + системное сообщение.
                                if let Some(v) = &mut self.view {
                                    if hide {
                                        v.hidden_players.insert(id);
                                    } else {
                                        v.hidden_players.remove(&id);
                                    }
                                    let key = if hide {
                                        "gui.socialInteractions.hidden_in_chat"
                                    } else {
                                        "gui.socialInteractions.shown_in_chat"
                                    };
                                    let msg = self.item_lang.fmt(key, &[&name]);
                                    self.chat.add(
                                        &ezzmc_protocol::text::Component::text(msg),
                                        self.tick_count,
                                        chat::Tag::System,
                                        &self.item_lang,
                                        self.chat_colors,
                                    );
                                }
                            }
                            match act {
                                ui::Action::None => {}
                                ui::Action::Quit => el.exit(),
                                ui::Action::CloseSocial => {
                                    gui.close_social();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::SetLanguage(code) => {
                                    // VANILLA: LanguageSelectScreen#onDone → LanguageManager#setSelected + перезагрузка.
                                    gui.opts.save();
                                    if let Some(rm) = &self.hud_rm {
                                        gui.set_lang(build_lang(rm, &code));
                                        self.item_lang = build_lang(rm, &code);
                                    }
                                }
                                ui::Action::ReloadResources => {
                                    // VANILLA: Minecraft#reloadResourcePacks — на лету; если размер атласа
                                    // блоков изменился (пак другого разрешения) — перезапуск процесса.
                                    gui.opts.save();
                                    self.reload_pending = true;
                                }
                                ui::Action::CloseSign => {
                                    if let Some((p, f, l)) = gui.take_sign_out() {
                                        if let Some(v) = &self.view {
                                            v.send(netthread::NetCmd::SignUpdate(p, f, l));
                                        }
                                    }
                                    gui.close_sign();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseCmdBlock => {
                                    if let Some(o) = gui.take_cmd_out() {
                                        if let Some(v) = &self.view {
                                            let flags = u8::from(o.track)
                                                | u8::from(o.conditional) << 1
                                                | u8::from(o.auto) << 2;
                                            v.send(netthread::NetCmd::SetCommandBlock(
                                                o.pos, o.command, o.mode, flags,
                                            ));
                                        }
                                    }
                                    gui.close_cmd_block();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseStructure => {
                                    if let Some(o) = gui.take_structure_out() {
                                        if let Some(v) = &self.view {
                                            v.send(netthread::NetCmd::SetStructureBlock(Box::new(
                                                o,
                                            )));
                                        }
                                    }
                                    gui.close_structure();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseTestBlock => {
                                    if let Some(o) = gui.take_testblock_out() {
                                        if let Some(v) = &self.view {
                                            v.send(netthread::NetCmd::SetTestBlock(
                                                o.pos, o.mode, o.message,
                                            ));
                                        }
                                    }
                                    gui.close_testblock();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseJigsaw => {
                                    if let Some(o) = gui.take_jigsaw_out() {
                                        if let Some(v) = &self.view {
                                            v.send(netthread::NetCmd::SetJigsawBlock(Box::new(o)));
                                        }
                                    }
                                    gui.close_jigsaw();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseBook => {
                                    if let Some(o) = gui.take_book_out() {
                                        if let Some(v) = &self.view {
                                            v.send(netthread::NetCmd::EditBook(
                                                o.slot, o.pages, o.title,
                                            ));
                                        }
                                    }
                                    gui.close_book();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::CloseDialog => {
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                }
                                ui::Action::Resume => {
                                    gui.opts.save();
                                    self.menu_active = false;
                                    if let Some(w) = &self.window {
                                        self.mouse.grab(w);
                                    }
                                    let on = gui
                                        .opts
                                        .get(&ui::options::VIDEO[ui::options::CUTOUT_LEAVES])
                                        == "true";
                                    if let Some(v) = &mut self.view {
                                        match ezzmc_assets::install::GameInstall::locate("26.3")
                                            .and_then(|i| packrepo::build_resource_manager(&i))
                                        {
                                            Ok(rm) => {
                                                if let Err(e) = v.set_cutout_leaves(&rm, on) {
                                                    tracing::error!("листва: {e}");
                                                }
                                            }
                                            Err(e) => tracing::error!("ресурсы игры 26.3: {e}"),
                                        }
                                    }
                                }
                                ui::Action::OpenLink(u) => {
                                    // VANILLA: Blaze3D.openUri; только http/https (проверено при разборе).
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg(&u)
                                        .stdin(std::process::Stdio::null())
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn();
                                }
                                ui::Action::OpenServerLinks => {
                                    gui.open_server_links();
                                }
                                ui::Action::OpenAdv => {
                                    self.menu_active = false;
                                    self.open_adv();
                                    self.screen_from_pause = self.adv.is_some();
                                }
                                ui::Action::OpenStats => {
                                    self.menu_active = false;
                                    self.open_stats();
                                    self.screen_from_pause = self.stats_screen.is_some();
                                }
                                ui::Action::Respawn => {
                                    if let Some(v) = &self.view {
                                        v.send(netthread::NetCmd::Respawn);
                                    }
                                }
                                ui::Action::Disconnect => {
                                    gui.opts.save();
                                    gui.reset_title();
                                    self.view = None;
                                    self.connect = None;
                                    self.start_prewarm();
                                }
                                ui::Action::CancelConnect => {
                                    // VANILLA: ConnectScreen «Cancel» → aborted, setScreen(parent).
                                    gui.back_to_servers();
                                    self.view = None;
                                    self.connect = None;
                                    self.start_prewarm();
                                    self.connect_phase = 0;
                                    self.join_pending = false;
                                    self.world_job = None;
                                }
                                ui::Action::Join(addr) => {
                                    self.connect = Some(addr);
                                    gui.open_connecting();
                                    self.world_pending = false;
                                    self.join_pending = true;
                                    self.join_delay = 2;
                                }
                            }
                            shot
                        } else {
                            sc.clear_and_present(g, f, [0.12, 0.02, 0.06], cap)
                        };
                        if let Some(shot) = shot {
                            // VANILLA: каталог — gameDirectory (пока текущий; options — TODO-026).
                            let saved = screenshot::save(std::path::Path::new("."), &shot);
                            match &saved {
                                Ok(p) => tracing::info!("скриншот сохранён: {}", p.display()),
                                Err(e) => tracing::error!("скриншот: {e}"),
                            }
                            if let Some(r) = self.shot_reply.take() {
                                let _ = r.send(match saved {
                                    Ok(p) => control::ok(serde_json::json!({
                                        "path": std::fs::canonicalize(&p).unwrap_or(p),
                                        "width": shot.width,
                                        "height": shot.height,
                                        "frame": screenshot::analyze(&shot),
                                        "view": self.ctl_view_report(),
                                    })),
                                    Err(e) => control::err(e),
                                });
                            }
                        }
                    }
                    Acquire::Skip => {}
                    Acquire::DeviceLost => {
                        self.gpu = None;
                        self.swapchain = None;
                        self.init_gpu(el);
                    }
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }
}

/// Тайлы 6×7 клеток тестовой платформы (клетки через 2 блока): центры камеры по x, z.
fn tour_tiles() -> Vec<[f64; 2]> {
    if std::env::var("EZZMC_TOUR").is_err() {
        return Vec::new();
    }
    let mut v = Vec::new();
    for j in 0..5 {
        for k in 0..7 {
            v.push([6.0 + 12.0 * f64::from(k), 7.0 + 14.0 * f64::from(j)]);
        }
    }
    v
}

fn menu_shot_frame() -> Option<u32> {
    std::env::var("EZZMC_MENU_SHOT").ok()?.parse().ok()
}

fn autoshot_secs() -> Option<u64> {
    std::env::var("EZZMC_AUTOSHOT").ok()?.parse().ok()
}

/// Самый свежий `*.ezbake` в `reference/_cache/forge-bake/` — от текущей папки или от бинарника
/// (`target/<профиль>/ezzmc-client` → корень проекта).
fn find_mod_bake() -> Option<std::path::PathBuf> {
    let rel = std::path::Path::new("reference/_cache/forge-bake");
    let mut dirs = vec![rel.to_path_buf()];
    if let Some(root) = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent()?.parent()?.parent().map(|r| r.join(rel)))
    {
        dirs.push(root);
    }
    dirs.iter()
        .filter_map(|d| std::fs::read_dir(d).ok())
        .flatten()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ezbake"))
        .filter_map(|p| Some((std::fs::metadata(&p).ok()?.modified().ok()?, p)))
        .max_by_key(|(t, _)| *t)
        .map(|(_, p)| p)
}

fn arg_value(flag: &str) -> Option<String> {
    let a: Vec<String> = std::env::args().collect();
    a.iter()
        .position(|x| x == flag)
        .and_then(|i| a.get(i + 1).cloned())
}

/// `--control [host:port]` — сокет управления агентом; без флага выключен.
fn start_control() -> Option<control::Control> {
    let a: Vec<String> = std::env::args().collect();
    let i = a.iter().position(|x| x == "--control")?;
    let port = std::env::var("EZZMC_CONTROL_PORT").ok();
    let addr = match (a.get(i + 1).filter(|x| !x.starts_with("--")), port) {
        (Some(x), _) => x.clone(),
        (None, Some(p)) => format!("127.0.0.1:{p}"),
        (None, None) => control::DEFAULT_ADDR.to_string(),
    };
    match control::Control::start(&addr) {
        Ok(c) => {
            tracing::info!("управление: {addr}");
            Some(c)
        }
        Err(e) => {
            tracing::error!("управление {addr}: {e}");
            None
        }
    }
}

fn main() {
    // Безоконная песочница физики/мира (без игры и сервера): `--sandbox [файл | -e "скрипт"]`.
    {
        let a: Vec<String> = std::env::args().skip(1).collect();
        if let Some(i) = a.iter().position(|x| x == "--sandbox") {
            std::process::exit(sandbox::main(&a[i + 1..]));
        }
    }
    let packets = std::env::args().any(|a| a == "--log-packets");
    let _ = log::init(&log::LogConfig {
        log_packets: packets,
        ..Default::default()
    });
    tracing::info!("ezzmc");
    // DEC-018: дамп моделей модов Forge 1.20.1 (`--mod-bake <файл.ezbake>` или EZZMC_MOD_BAKE)
    // Без флага/переменной — самый свежий дамп из reference/_cache/forge-bake/ (рядом с проектом);
    // `--mod-bake off` — без модов.
    match arg_value("--mod-bake") {
        Some(p) if p == "off" => std::env::remove_var("EZZMC_MOD_BAKE"),
        // до запуска потоков: читается только через modbake::global()
        Some(p) => std::env::set_var("EZZMC_MOD_BAKE", p),
        None if std::env::var_os("EZZMC_MOD_BAKE").is_none() => {
            if let Some(p) = find_mod_bake() {
                tracing::info!("дамп модов найден: {}", p.display());
                std::env::set_var("EZZMC_MOD_BAKE", p);
            }
        }
        None => {}
    }
    if let Some(b) = ezzmc_assets::modbake::global() {
        ezzmc_mesh::models::install_mod_table(b);
    }
    audio::init();
    // Всегда X11 (XWayland), если не задано EZZMC_BACKEND=wayland: на Wayland захват курсора
    // не включался при запуске сразу на сервер.
    #[allow(unused_mut)]
    let mut elb = EventLoop::builder();
    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var("EZZMC_BACKEND").as_deref() != Ok("wayland") {
        use winit::platform::x11::EventLoopBuilderExtX11;
        elb.with_x11();
    }
    let el = match elb.build() {
        Ok(el) => el,
        Err(e) => {
            tracing::error!("event loop: {e}");
            return;
        }
    };
    let clock = timer::Clock::start();
    let mut app = App {
        cfg: window::WindowState::default().config("ezzmc"),
        wstate: window::WindowState::default(),
        applied_vsync: None,
        occluded: false,
        applied_fs: None,
        applied_compute: None,
        quit_shortcuts: true,
        allow_cursor: true,
        alt_down: false,
        f4_down: false,
        applied_info: None,
        window: None,
        gpu: None,
        swapchain: None,
        clock,
        timer: timer::Timer::new(timer::TICKS_PER_SECOND, 0),
        tick_count: 0,
        mouse: input::Mouse::default(),
        input: input::InputState::default(),
        keys: keymap::KeyBindings::default(),
        focus: focus::Focus::new(0),
        want_screenshot: false,
        fps_limit: display::FramerateLimit::new(
            std::env::var("EZZMC_MAX_FPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(display::DEFAULT_MAX_FPS),
        ),
        limiter: display::FrameLimiter::default(),
        yaw: 0.0,
        title_label: "",
        pitch: std::env::var("EZZMC_PITCH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        connect: arg_value("--connect"),
        player_name: netthread::session()
            .map(|s| s.name.clone())
            .or_else(|| arg_value("--name"))
            .unwrap_or_else(|| "EzzBot".into()),
        view: None,
        last_ms: 0,
        autoshot_at: None,
        hud_state: hudbars::HudState::default(),
        tour: tour_tiles(),
        tour_i: 0,
        tour_t: 0,
        fly_mult: 1.0,
        autoshot_done: false,
        text: None,
        world_pending: false,
        frames: 0,
        fps: 0,
        fps_count: 0,
        fps_t0: 0,
        debug_on: std::env::var("EZZMC_F3").is_ok(),
        f3_pending: false,
        f3_held: false,
        hud_hidden: false,
        gui: None,
        painter: None,
        hud_rm: None,
        item_lang: ui::lang::Lang::default(),
        clipboard: clipboard::Clipboard::default(),
        menu_active: false,
        was_grabbed: false,
        music: music::MusicManager::new(),
        now_playing: toasts::NowPlaying::default(),
        inv_open: false,
        adv: None,
        stats_screen: None,
        gms: None,
        stats_down: false,
        screen_from_pause: false,
        char_preview: None,
        ench_book: None,
        anvil: invscreen::AnvilName::default(),
        inv_container: false,
        inv_creative: false,
        dead_screen: false,
        win_screen: None,
        loading_bg: None,
        in_bed: false,
        was_joined: false,
        world_job: None,
        prewarm: None,
        connect_phase: 0,
        loading_since: None,
        last_slot_click: None,
        held_button: None,
        drag: None,
        creative: creativescreen::State::default(),
        overlay: None,
        overlay_anim: false,
        chat: chat::Chat::default(),
        chat_cfg: chat::Cfg::default(),
        chat_delay: 0.0,
        chat_vis: 0,
        chat_pending: std::collections::VecDeque::new(),
        sub_bg: 0.8,
        show_subtitles: false,
        chat_colors: true,
        chat_links: true,
        chat_links_prompt: true,
        cursor_px: (0.0, 0.0),
        join_pending: false,
        join_delay: 0,
        reload_pending: false,
        control: start_control(),
        control_pending: Vec::new(),
        control_task: None,
        shot_reply: None,
    };
    if let Err(e) = el.run_app(&mut app) {
        tracing::error!("event loop: {e}");
    }
}

/// VANILLA: Mth#hsvToRgb.
fn hsv_rgb(hue: f32, s: f32, v: f32) -> u32 {
    let h = (hue * 6.0) as i32 % 6;
    let f = hue * 6.0 - (hue * 6.0).floor();
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - f * s), v * (1.0 - (1.0 - f) * s));
    let (r, g, b) = match h {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    let c = |x: f32| (x * 255.0 + 0.5) as u32;
    (c(r) << 16) | (c(g) << 8) | c(b)
}

/// Файл из `assets/objects` по индексу (языки и `pack.mcmeta` лежат только там, не в jar).
fn store_read(path: &str) -> Option<Vec<u8>> {
    use ezzmc_assets::source::ResourceSource;
    let i = ezzmc_assets::install::GameInstall::locate("26.3").ok()?;
    let idx = i.index_path().ok()??;
    let objects = i.assets_dir.as_ref()?.join("objects");
    ezzmc_assets::source::ObjectStore::open(&idx, &objects)
        .ok()?
        .read(path)
}

/// VANILLA: ClientLanguage#loadFrom — en_us, поверх него выбранный язык.
fn build_lang(rm: &ezzmc_assets::pack::ResourceManager, code: &str) -> ui::lang::Lang {
    let mut l = ui::lang::Lang::load(
        &rm.read("assets/minecraft/lang/en_us.json")
            .unwrap_or_default(),
    );
    // Код — только [a-z0-9_] (защита от выхода из каталога).
    if code != "en_us" && code.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        if let Some(d) = rm
            .read(&format!("assets/minecraft/lang/{code}.json"))
            .or_else(|| store_read(&format!("assets/minecraft/lang/{code}.json")))
        {
            l.merge(&d);
        }
    }
    l
}

/// VANILLA: LanguageManager#extractLanguages — секция `language` из `pack.mcmeta` (индекс ассетов).
/// Подпись `LanguageInfo#toComponent` = «name (region)»; сортировка по коду (TreeMap).
fn language_list(rm: &ezzmc_assets::pack::ResourceManager) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = store_read("assets/pack.mcmeta")
        .or_else(|| rm.read("assets/pack.mcmeta"))
        .and_then(|d| serde_json::from_slice::<serde_json::Value>(&d).ok())
        .and_then(|j| {
            j.get("language")?.as_object().map(|m| {
                m.iter()
                    .filter_map(|(k, i)| {
                        Some((
                            k.clone(),
                            format!(
                                "{} ({})",
                                i.get("name")?.as_str()?,
                                i.get("region")?.as_str()?
                            ),
                        ))
                    })
                    .collect()
            })
        })
        .unwrap_or_else(|| vec![("en_us".to_owned(), "English (US)".to_owned())]);
    v.sort();
    v
}

/// Игроки для экрана социальных взаимодействий: таб-лист + скрытые, которых уже нет в сети.
fn social_players(v: &worldview::WorldView) -> Vec<ui::social::Player> {
    let mut out: Vec<_> = v
        .tab
        .entries
        .iter()
        .map(|(id, e)| ui::social::Player {
            id: *id,
            name: e.name.clone(),
            online: true,
        })
        .collect();
    for id in &v.hidden_players {
        if !v.tab.entries.contains_key(id) {
            out.push(ui::social::Player {
                id: *id,
                name: v
                    .tab
                    .profiles
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| "?".into()),
                online: false,
            });
        }
    }
    out
}
