mod ambient;
mod armor;
mod atlasanim;
mod audio;
mod avatar;
mod blocksound;
mod charpreview;
mod chat;
mod chatnet;
mod clipboard;
mod control;
mod cooldown;
#[allow(dead_code)] // 230: модель готова, в netthread/WorldView ещё не подключена
mod creative_data;
mod creativescreen;
mod cursor;
mod debugscreen;
mod dig;
mod display;
mod editbox;
mod entityshadow;
mod flammable;
mod focus;
mod glibc_compat;
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
mod mobs;
mod music;
mod netthread;
mod packrepo;
mod particles;
mod physics;
#[allow(dead_code)] // face/hit/inside и дальности — для 185/186
mod pick;
mod pistonview;
mod playersound;
mod screenshot;
mod settings;
mod threads;
mod timer;
mod tooltip;
mod ui;
mod weather;
mod window;
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
    applied_fs: Option<bool>,
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
    /// Главное меню (GUI); `None` до загрузки ресурсов.
    gui: Option<ui::Gui>,
    painter: Option<ezzmc_render::gui::GuiPainter>,
    /// Ресурсы игры для ленивой загрузки иконок HUD.
    hud_rm: Option<ezzmc_assets::pack::ResourceManager>,
    item_lang: ui::lang::Lang,
    clipboard: clipboard::Clipboard,
    /// Экран инвентаря/контейнера открыт.
    inv_open: bool,
    /// Экран смерти (здоровье 0): кнопка «Respawn».
    dead_screen: bool,
    /// Курсор уже захвачен после входа в мир.
    was_joined: bool,
    char_preview: Option<charpreview::CharPreview>,
    /// Показан креатив-экран (а не InventoryMenu/контейнер).
    inv_creative: bool,
    creative: creativescreen::State,
    /// Сообщение над хотбаром (VANILLA: Gui#setOverlayMessage, 60 тиков): текст и срок.
    overlay: Option<(String, std::time::Instant)>,
    chat: chat::Chat,
    chat_cfg: chat::Cfg,
    chat_colors: bool,
    chat_links: bool,
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
    music: music::MusicManager,
    /// Игрок выбрал сервер в меню: мир грузится в начале следующего кадра.
    join_pending: bool,
    /// Смена ресурспаков ждёт применения в начале кадра.
    reload_pending: bool,
    /// `--control`: сокет управления для агента (см. `control.rs`).
    control: Option<control::Control>,
    control_pending: Vec<control::Pending>,
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

    pub fn inv_mouse(&mut self, pressed: bool, right: bool, left: bool, middle: bool) {
        if self.inv_creative {
            self.creative_mouse(pressed, right, left, middle);
            return;
        }
        if !pressed {
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
                let now = std::time::Instant::now();
                let double = matches!((slot, self.last_slot_click), (Some(a), Some((b, t)))
                            if a == b && now.duration_since(t).as_millis() < 250);
                self.last_slot_click = if double { None } else { slot.map(|s| (s, now)) };
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
            return;
        };
        let (joined, dead) = (v.joined, v.health <= 0.0);
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
                if !self.was_joined || w.has_focus() {
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
        if dead && !self.dead_screen {
            self.dead_screen = true;
            if self.inv_open {
                self.close_inv(true);
            }
            self.input.release_all();
            self.keys.release_all();
            if let Some(w) = &self.window {
                self.mouse.release(w);
            }
        } else if !dead && self.dead_screen {
            self.dead_screen = false;
            if let Some(w) = &self.window {
                self.mouse.grab(w);
            }
        }
    }

    /// Кнопка «Respawn» экрана смерти (GUI-координаты): 200×20 по центру, y = 72.
    fn respawn_button(&self) -> Option<(f32, f32, f32, f32)> {
        let (gw, _, _, _) = self.gui_cursor()?;
        let x = gw / 2.0 - 100.0;
        Some((x, 72.0, x + 200.0, 92.0))
    }

    fn dead_click(&mut self) {
        let (Some((x0, y0, x1, y1)), Some((_, _, mx, my))) =
            (self.respawn_button(), self.gui_cursor())
        else {
            return;
        };
        if mx >= x0 && mx < x1 && my >= y0 && my < y1 {
            if let Some(v) = &self.view {
                v.send(netthread::NetCmd::Respawn);
            }
        }
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
                v.send(match msg.strip_prefix('/') {
                    Some(c) => netthread::NetCmd::Command(c.to_owned()),
                    None => netthread::NetCmd::Chat(msg),
                });
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
        let (Some((_, gh, mx, my)), Some(t)) = (self.gui_cursor(), self.text.as_ref()) else {
            return;
        };
        let cfg = self.chat_cfg;
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
                if self.chat_links && u.starts_with("http://") || u.starts_with("https://") =>
            {
                let _ = std::process::Command::new("xdg-open").arg(u).spawn();
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

    /// VANILLA: Screen#onClose → ServerboundContainerClosePacket (`notify` — закрыл игрок, не сервер).
    fn close_inv(&mut self, notify: bool) {
        self.inv_open = false;
        if let Some(v) = &mut self.view {
            if notify {
                v.send_container_close();
            }
            v.inv.open = None;
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
                msg,
                std::time::Instant::now() + std::time::Duration::from_secs(3),
            ));
        }
    }

    /// Клавиши экрана инвентаря: цифры/F над слотом — SWAP, Q — THROW.
    fn inv_key(&mut self, k: KeyCode) {
        let Some((gw, gh, mx, my)) = self.gui_cursor() else {
            return;
        };
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
        let Some(v) = &self.view else { return };
        let lay = invscreen::layout(&v.inv);
        let Some(slot) = lay.slot_at(gw, gh, mx, my) else {
            return;
        };
        let cmd = if let Some(b) = swap_btn {
            invscreen::click_cmd(&v.inv, slot as i16, b, invscreen::SWAP)
        } else if drop {
            invscreen::click_cmd(&v.inv, slot as i16, i8::from(ctrl), invscreen::THROW)
        } else {
            return;
        };
        v.send(cmd);
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
        let vsync = o.get(&ui::options::VIDEO[2]) == "true"
            && std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0");
        if self.applied_vsync != Some(vsync) {
            if let (Some(g), Some(sc)) = (&self.gpu, &mut self.swapchain) {
                sc.set_present_mode(g, display::present_mode(vsync));
                self.applied_vsync = Some(vsync);
            }
        }
        // VANILLA: Options#fullscreen ↔ окно (F11 пишет опцию обратно).
        let fs = o.get(&ui::options::VIDEO[5]) == "true";
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
        // Иконки паков: pack.png ванили и unknown_pack (VANILLA: PackSelectionScreen#DEFAULT_ICON).
        for (name, path) in [
            (
                "pack_unknown",
                "assets/minecraft/textures/misc/unknown_pack.png",
            ),
            ("pack_vanilla", "pack.png"),
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
        let max_fps = gui.opts.f32(&ui::options::VIDEO[1]) as u32;
        self.fps_limit.set_limit(max_fps);
        // VANILLA: Options#load — key_* из options.txt.
        self.keys.load_options(gui.opts.pairs());
        self.gui = Some(gui);
        self.painter = Some(p);
        self.hud_rm = Some(rm);
        self.menu_active = true;
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
        match worldview::WorldView::new(g, sc.config.format, &rm, net, threads, {
            let mut ro = worldview::RenderOptions::from_env_args();
            let o = ui::options::Options::load(std::path::Path::new("./options.txt"));
            ro.cutout_leaves = o.get(&ui::options::VIDEO[ui::options::CUTOUT_LEAVES]) == "true";
            ro
        }) {
            Ok(v) => self.view = Some(v),
            Err(e) => tracing::error!("мир: {e}"),
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
            WindowEvent::KeyboardInput { event, .. } if self.dead_screen => {
                if event.state == ElementState::Pressed
                    && event.physical_key == PhysicalKey::Code(KeyCode::Enter)
                {
                    if let Some(v) = &self.view {
                        v.send(netthread::NetCmd::Respawn);
                    }
                }
            }
            WindowEvent::MouseInput { state, .. } if self.dead_screen => {
                if state == ElementState::Pressed {
                    self.dead_click();
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
            WindowEvent::KeyboardInput { event, .. } if self.inv_open => {
                // VANILLA: Screen.hasShiftDown/hasControlDown — состояние клавиш должно
                // обновляться и в экране инвентаря (иначе Shift+клик не виден).
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.input.on_key(k, event.state);
                    self.keys
                        .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                }
                let eaten =
                    event.state == ElementState::Pressed && self.creative_search_input(&event);
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
                        let close = k == KeyCode::Escape
                            || self
                                .keys
                                .get("key.inventory")
                                .is_some_and(|b| b.key == keymap::Input::Key(k));
                        if close {
                            self.close_inv(true);
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
                if self.inv_creative {
                    let y = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => f64::from(y),
                        winit::event::MouseScrollDelta::PixelDelta(p) => p.y,
                    };
                    self.creative.scroll(y);
                }
            }
            WindowEvent::KeyboardInput { event, .. } if self.menu_active => {
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
                            match k {
                                KeyCode::Escape => g.on_key(ui::UiKey::Escape),
                                KeyCode::Enter | KeyCode::NumpadEnter => g.on_key(ui::UiKey::Enter),
                                KeyCode::Backspace => g.on_key(ui::UiKey::Backspace),
                                KeyCode::Tab => g.on_key(ui::UiKey::Tab),
                                _ => {}
                            }
                        }
                        if let Some(t) = &event.text {
                            g.on_text(t.as_str());
                        }
                    }
                }
            }
            WindowEvent::CursorEntered { .. } => {
                if let Some(w) = &self.window {
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
                        if event.state == ElementState::Pressed && !event.repeat {
                            self.f3_pending = true;
                        } else if event.state == ElementState::Released && self.f3_pending {
                            self.f3_pending = false;
                            self.debug_on = !self.debug_on;
                        }
                    } else if event.state == ElementState::Pressed {
                        // VANILLA: KeyboardHandler — F3+H переключает расширенные подсказки (advanced tooltips).
                        if k == KeyCode::KeyH && self.f3_pending && !event.repeat {
                            tooltip::ADVANCED.fetch_xor(true, std::sync::atomic::Ordering::Relaxed);
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
                        // Отладка (не ваниль): F8 — переключить compute-рендер (сравнение FPS).
                        if event.state == ElementState::Pressed && k == KeyCode::F8 {
                            if let (Some(v), Some(g)) = (&mut self.view, &self.gpu) {
                                let on = !v.compute_renderer_enabled();
                                v.set_compute_renderer(g, on);
                                tracing::info!(
                                    "compute-рендер: {}",
                                    if v.compute_renderer_enabled() {
                                        "вкл"
                                    } else {
                                        "выкл"
                                    }
                                );
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
            WindowEvent::Focused(true) => {
                self.focus.set_active(true);
                // Захват, сделанный до получения фокуса (запуск сразу на сервер), не действует —
                // перезахватываем при получении фокуса.
                let playing = !self.menu_active
                    && !self.inv_open
                    && !self.chat.open
                    && !self.dead_screen
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
                self.apply_settings();
                // VANILLA: Window#selectCursor — запрос прошлого кадра, при allowCursorChanges=false стрелка.
                if let Some(w) = &self.window {
                    w.set_cursor(cursor::take(self.allow_cursor));
                }
                {
                    let m = match &self.view {
                        Some(v) if v.joined => v.situational_music(),
                        _ => Some(music::menu()),
                    };
                    self.music.update(m.as_ref(), false);
                }
                // Отладка: EZZMC_MENU_SHOT=<кадр> — скриншот меню и выход.
                if let (true, Some(n)) = (self.menu_active, menu_shot_frame()) {
                    if self.frames == n {
                        self.want_screenshot = true;
                    } else if self.frames > n + 10 {
                        el.exit();
                    }
                }
                if self.join_pending {
                    self.join_pending = false;
                    self.init_world();
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
                    self.chat_colors = o.get(&CHAT[0]) == "true";
                    self.chat.sg.auto = o.get(&CHAT[3]) == "true";
                    self.chat_links = o.get(&CHAT[1]) == "true";
                    self.chat_cfg = chat::Cfg {
                        opacity: o.f32(&CHAT[4]),
                        bg: o.f32(&CHAT[5]),
                        scale: o.f32(&CHAT[6]).max(0.01),
                        width: o.f32(&CHAT[7]),
                        h_focused: o.f32(&CHAT[8]),
                        h_unfocused: o.f32(&CHAT[9]),
                        spacing: o.f32(&CHAT[10]),
                    };
                }
                if let Some(v) = &mut self.view {
                    for (id, s, l, items) in std::mem::take(&mut v.sugg_in) {
                        self.chat.sg.on_reply(id, s, l, items, &self.chat.edit);
                    }
                    if self.chat.open {
                        if let Some((id, text)) = self.chat.sg.on_edit(&self.chat.edit) {
                            v.send(netthread::NetCmd::Suggest(id, text));
                        }
                    }
                    for (c, tag) in std::mem::take(&mut v.chat_in) {
                        match tag {
                            Some(tag) => self.chat.add(
                                &c,
                                self.tick_count,
                                tag,
                                &self.item_lang,
                                self.chat_colors,
                            ),
                            None => {
                                // VANILLA: Gui#setOverlayMessage — 60 тиков.
                                let m = chat::flatten(&c, &self.item_lang, self.chat_colors)
                                    .iter()
                                    .map(|s| s.text.as_str())
                                    .collect::<String>();
                                self.overlay = Some((
                                    m,
                                    std::time::Instant::now()
                                        + std::time::Duration::from_millis(3000),
                                ));
                            }
                        }
                    }
                }
                let (Some(g), Some(sc)) = (&self.gpu, &mut self.swapchain) else {
                    return;
                };
                // VANILLA: Minecraft#runTick — до MAX_TICKS_PER_UPDATE тиков за кадр, остаток отбрасывается.
                let n = self.timer.advance_time(self.clock.now_ms(), true);
                for _ in 0..n.min(timer::MAX_TICKS_PER_UPDATE) {
                    self.tick_count += 1;
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
                let (y, p) = input::apply_turn(self.yaw, self.pitch, turn);
                self.yaw = y;
                self.pitch = p;
                let _partial = self.timer.partial_tick(false);
                // VANILLA: FramerateLimitTracker + limitDisplayFPS.
                let now = self.clock.now_ms();
                let iconified = self
                    .window
                    .as_ref()
                    .and_then(|w| w.is_minimized())
                    .unwrap_or(false);
                let t = self.fps_limit.throttle(now, iconified, false, false);
                self.limiter.wait(self.fps_limit.effective(t));
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
                        v.walk(dt, physics::Input::default());
                    }
                    v.update(g);
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
                match sc.acquire(g) {
                    Acquire::Frame(f) => {
                        let cap = std::mem::take(&mut self.want_screenshot);
                        let sky = [0.47, 0.65, 1.0];
                        let shot = if let (Some(v), false) = (&mut self.view, self.menu_active) {
                            let text = &mut self.text;
                            let chat = &mut self.chat;
                            let mut hud = self.painter.as_mut().zip(self.hud_rm.as_ref());
                            let inv_open = self.inv_open;
                            let dead = self.dead_screen;
                            let cross = (
                                v.game_mode,
                                v.target.is_some() || v.entity_target.is_some(),
                                v.attack_strength(0.0),
                                v.entity_target.is_some(),
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
                            };
                            // VANILLA: gameMode.canHurtPlayer/hasExperience — выживание и приключение.
                            let survival = matches!(v.game_mode, 0 | 2);
                            let xp_recent = v
                                .xp_changed_at
                                .is_some_and(|t| t.elapsed().as_secs_f32() < 5.0);
                            let loc_bar =
                                hudbars::show_locator(!v.waypoints.is_empty(), survival, xp_recent);
                            let loc_wps = if loc_bar {
                                v.waypoints.clone()
                            } else {
                                Default::default()
                            };
                            let (loc_cam, loc_yaw, loc_pitch, loc_fov) =
                                (v.cam, v.yaw, v.pitch, v.fov_deg());
                            let overlay = self
                                .overlay
                                .as_ref()
                                .filter(|(_, t)| std::time::Instant::now() < *t)
                                .map(|(m, _)| m.clone());
                            let drag_prev = self
                                .drag
                                .as_ref()
                                .filter(|d| d.2)
                                .map(|d| (d.0, d.1.clone()));
                            let inv_creative = self.inv_creative;
                            let creative = &self.creative;
                            if inv_open && self.char_preview.is_none() {
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
                            let lang = &self.item_lang;
                            let cur = self.cursor_px;
                            let fps = self.fps;
                            let gui_scale_opt = self
                                .gui
                                .as_ref()
                                .map(|g| g.opts.f32(&ui::options::VIDEO[3]) as u32);
                            let debug_on = self.debug_on;
                            let fps_limit = self.fps_limit.limit();
                            let pitch = self.pitch;
                            let vsync = std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0");
                            sc.draw_and_present(g, f, cap, |view, w, h| {
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
                                        p.begin(t.scale());
                                        if !inv_open && !dead {
                                            hud::draw_crosshair(
                                                p, gwf, ghf, cross.0, cross.1, cross.2, cross.3,
                                                cross.4,
                                            );
                                        }
                                        hud::draw_hotbar(
                                            p, t, &g.device, &g.queue, rm, &v.inv, gwf, ghf,
                                        );
                                        if survival && !dead {
                                            hudbars::draw_health(p, hs, &stats, gwf, ghf);
                                        }
                                        if loc_bar && !dead {
                                            hudbars::draw_locator(
                                                p, &loc_wps, loc_cam, loc_yaw, loc_pitch, loc_fov,
                                                gwf, ghf,
                                            );
                                        } else if survival && !dead {
                                            hudbars::draw_xp_bar(p, &stats, gwf, ghf);
                                        }
                                        p.flush(&g.device, &g.queue, view, w, h, None);
                                        if survival && !dead {
                                            hudbars::draw_xp_level(t, stats.xp_level, gwf, ghf);
                                        }
                                        if let Some(m) = &overlay {
                                            // VANILLA: Gui#extractOverlayMessage — над хотбаром (gh − 68).
                                            let mw = t.text_width(m);
                                            t.draw(
                                                m,
                                                gwf / 2.0 - mw / 2.0,
                                                ghf - 68.0,
                                                0xFFFFFF,
                                                true,
                                            );
                                        }
                                        {
                                            let (ccfg, tick) = (self.chat_cfg, self.tick_count);
                                            chat.draw(t, ghf, tick, &ccfg);
                                            if chat.open {
                                                chat.draw_input(t, gwf, ghf, ccfg.bg);
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
                                            }
                                        }
                                        if dead {
                                            // VANILLA: DeathScreen — красное затемнение, «You died!», кнопка Respawn.
                                            let k = f64::from(t.scale());
                                            let (mx, my) = ((cur.0 / k) as f32, (cur.1 / k) as f32);
                                            t.fill(0.0, 0.0, gwf, ghf, 0x6050_0000);
                                            let bx = gwf / 2.0 - 100.0;
                                            let hot = mx >= bx
                                                && mx < bx + 200.0
                                                && (72.0..92.0).contains(&my);
                                            t.fill(bx - 1.0, 71.0, bx + 201.0, 93.0, 0xFF00_0000);
                                            t.fill(
                                                bx,
                                                72.0,
                                                bx + 200.0,
                                                92.0,
                                                if hot { 0xFF7080D0 } else { 0xFF6C6C6C },
                                            );
                                            let tw = t.text_width("You died!");
                                            t.draw(
                                                "You died!",
                                                gwf / 2.0 - tw / 2.0,
                                                30.0,
                                                0xFFFFFF,
                                                true,
                                            );
                                            let rw = t.text_width("Respawn");
                                            t.draw(
                                                "Respawn",
                                                gwf / 2.0 - rw / 2.0,
                                                78.0,
                                                0xFFFFFF,
                                                true,
                                            );
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
                                            } else {
                                                // VANILLA: во время перетаскивания видно раздачу по слотам.
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
                                                invscreen::draw_sprites(
                                                    p, &g.device, &g.queue, rm, invr, &lay, gwf,
                                                    ghf, dsl,
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
                                                p.flush(&g.device, &g.queue, view, w, h, None);
                                                invscreen::draw_text(
                                                    t, invr, lang, &lay, gwf, ghf, mx, my,
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
                                    if let Some(st) = &v.status {
                                        t.draw(
                                            &format!("Disconnected: {st}"),
                                            2.0,
                                            2.0,
                                            0xFF5555,
                                            true,
                                        );
                                    }
                                    t.flush(&g.device, &g.queue, view, w, h);
                                }
                            })
                        } else if let (true, Some(gui), Some(p), Some(t)) = (
                            self.menu_active,
                            &mut self.gui,
                            &mut self.painter,
                            &mut self.text,
                        ) {
                            let mut act = ui::Action::None;
                            let mut world = self.view.as_mut();
                            let shot = sc.draw_and_present(g, f, cap, |view, w, h| {
                                if let Some(v) = world.as_mut() {
                                    v.render(g, view, w, h, sky);
                                }
                                act = gui.render(
                                    &g.device, &g.queue, view, w, h, t, p, dt as f32, now_ms,
                                );
                            });
                            self.fps_limit
                                .set_limit(gui.opts.f32(&ui::options::VIDEO[1]) as u32);
                            match act {
                                ui::Action::None => {}
                                ui::Action::Quit => el.exit(),
                                ui::Action::ReloadResources => {
                                    // VANILLA: Minecraft#reloadResourcePacks — на лету; если размер атласа
                                    // блоков изменился (пак другого разрешения) — перезапуск процесса.
                                    gui.opts.save();
                                    self.reload_pending = true;
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
                                ui::Action::Disconnect => {
                                    gui.opts.save();
                                    gui.reset_title();
                                    self.view = None;
                                    self.connect = None;
                                }
                                ui::Action::Join(addr) => {
                                    self.connect = Some(addr);
                                    self.menu_active = false;
                                    self.world_pending = false;
                                    self.join_pending = true;
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
    let packets = std::env::args().any(|a| a == "--log-packets");
    let _ = log::init(&log::LogConfig {
        log_packets: packets,
        ..Default::default()
    });
    tracing::info!("ezzmc");
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
        applied_fs: None,
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
        player_name: arg_value("--name").unwrap_or_else(|| "EzzBot".into()),
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
        gui: None,
        painter: None,
        hud_rm: None,
        item_lang: ui::lang::Lang::default(),
        clipboard: clipboard::Clipboard::default(),
        menu_active: false,
        music: music::MusicManager::new(),
        inv_open: false,
        char_preview: None,
        inv_container: false,
        inv_creative: false,
        dead_screen: false,
        was_joined: false,
        last_slot_click: None,
        held_button: None,
        drag: None,
        creative: creativescreen::State::default(),
        overlay: None,
        chat: chat::Chat::default(),
        chat_cfg: chat::Cfg::default(),
        chat_colors: true,
        chat_links: true,
        cursor_px: (0.0, 0.0),
        join_pending: false,
        reload_pending: false,
        control: start_control(),
        control_pending: Vec::new(),
        shot_reply: None,
    };
    if let Err(e) = el.run_app(&mut app) {
        tracing::error!("event loop: {e}");
    }
}
