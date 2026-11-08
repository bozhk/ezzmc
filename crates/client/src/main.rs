mod avatar;
mod clipboard;
mod debugscreen;
mod dig;
mod display;
mod focus;
mod input;
#[allow(dead_code)] // 230: модель готова, в netthread/WorldView ещё не подключена
mod hud;
mod inventory;
mod keymap;
mod log;
mod netthread;
mod particles;
mod physics;
#[allow(dead_code)] // face/hit/inside и дальности — для 185/186
mod pick;
mod screenshot;
mod threads;
mod timer;
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
    cfg: window::WindowConfig,
    wstate: window::WindowState,
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
    /// Меню на экране (мир не показывается).
    menu_active: bool,
    /// Игрок выбрал сервер в меню: мир грузится в начале следующего кадра.
    join_pending: bool,
}

impl App {
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

impl App {
    /// Загрузка ресурсов GUI (шрифт, спрайты, панорама, язык) и показ главного меню.
    fn init_gui(&mut self) {
        let (Some(g), Some(sc)) = (&self.gpu, &self.swapchain) else {
            return;
        };
        let rm = match ezzmc_assets::install::GameInstall::locate("26.3")
            .and_then(|i| i.resource_manager())
        {
            Ok(rm) => rm,
            Err(e) => {
                tracing::error!("ресурсы игры 26.3: {e}");
                return;
            }
        };
        self.text = rm
            .read("assets/minecraft/textures/font/ascii.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            .filter(|i| i.width == i.height && i.width % 128 == 0)
            .map(|i| {
                ezzmc_render::text::TextRenderer::new(
                    &g.device,
                    &g.queue,
                    sc.config.format,
                    &i.to_rgba8(),
                    i.width,
                )
            });
        let mut p = ezzmc_render::gui::GuiPainter::new(&g.device, sc.config.format);
        for (name, path) in ui::TEXTURES.iter().chain(hud::SPRITES) {
            let full = format!("assets/minecraft/textures/gui/{path}.png");
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
                    name.starts_with("panorama"),
                ),
                None => tracing::warn!("текстура GUI не найдена: {full}"),
            }
        }
        let lang = rm
            .read("assets/minecraft/lang/en_us.json")
            .unwrap_or_default();
        let splashes = rm
            .read("assets/minecraft/texts/splashes.txt")
            .map(|d| String::from_utf8_lossy(&d).into_owned())
            .unwrap_or_default();
        let gui = ui::Gui::new(&lang, &splashes, std::path::Path::new("."));
        let max_fps = gui.opts.f32(&ui::options::VIDEO[1]) as u32;
        self.fps_limit.set_limit(max_fps);
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
            .and_then(|i| i.resource_manager())
        {
            Ok(rm) => rm,
            Err(e) => {
                tracing::error!("ресурсы игры 26.3: {e}");
                return;
            }
        };
        // Шрифт HUD — ванильный ascii.png (128×128); если меню уже загрузило — не трогаем.
        if self.text.is_none() {
            self.text = rm
                .read("assets/minecraft/textures/font/ascii.png")
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
                .filter(|i| i.width == i.height && i.width % 128 == 0)
                .map(|i| {
                    ezzmc_render::text::TextRenderer::new(
                        &g.device,
                        &g.queue,
                        sc.config.format,
                        &i.to_rgba8(),
                        i.width,
                    )
                });
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
            self.mouse.on_motion(dx, dy);
            self.fps_limit.on_input(self.clock.now_ms());
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::KeyboardInput { event, .. } if self.menu_active => {
                if event.state == ElementState::Pressed {
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
            WindowEvent::CursorMoved { position, .. } => {
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
                        self.f3_pending = false;
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
                                self.keys.get(&format!("key.hotbar.{i}")).is_some_and(|b| {
                                    b.key == keymap::Input::Key(k)
                                })
                            });
                            if let (Some(n), Some(v)) = (n, &mut self.view) {
                                v.select_slot(n as u8 - 1);
                            }
                        }
                        self.input.on_key(k, event.state);
                        self.keys
                            .on_input(keymap::Input::Key(k), event.state == ElementState::Pressed);
                    }
                    // VANILLA: Esc в игре — меню паузы (курсор освобождается).
                    if k == KeyCode::Escape && event.state == ElementState::Pressed {
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
                }
            }
            WindowEvent::MouseInput { state, button, .. } if self.menu_active => {
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
                if state == ElementState::Pressed && !self.mouse.grabbed {
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
                } else if self.mouse.grabbed && !self.input.key_down(KeyCode::ControlLeft) {
                    if let Some(v) = &mut self.view {
                        v.scroll_hotbar(y);
                    }
                } else if self.mouse.grabbed {
                    self.fly_mult = (self.fly_mult * 1.25_f64.powf(y)).clamp(0.05, 50.0);
                }
            }
            WindowEvent::Focused(true) => self.focus.set_active(true),
            WindowEvent::Focused(false) => {
                // VANILLA: потеря фокуса — отпустить клавиши и курсор.
                self.focus.set_active(false);
                self.input.release_all();
                self.keys.release_all();
                if let Some(w) = &self.window {
                    self.mouse.release(w);
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
                self.frames = self.frames.saturating_add(1);
                self.fps_count += 1;
                let t = self.clock.now_ms();
                if t >= self.fps_t0 + 1000 {
                    self.fps = self.fps_count;
                    self.fps_count = 0;
                    self.fps_t0 = t;
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
                let (y, p) = input::apply_turn(self.yaw, self.pitch, self.mouse.take_turn());
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
                            if let Some(b) = self.keys.get_mut("key.attack") {
                                while b.consume_click() {
                                    v.player.swing();
                                    v.click_attack();
                                }
                            }
                            if let Some(b) = self.keys.get_mut("key.use") {
                                while b.consume_click() {
                                    v.use_item();
                                }
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
                            let mut hud = self.painter.as_mut().zip(self.hud_rm.as_ref());
                            let fps = self.fps;
                            let debug_on = self.debug_on;
                            let fps_limit = self.fps_limit.limit();
                            let pitch = self.pitch;
                            let vsync = std::env::var("EZZMC_VSYNC").map_or(true, |v| v != "0");
                            sc.draw_and_present(g, f, cap, |view, w, h| {
                                v.render(g, view, w, h, sky);
                                if let Some(t) = text {
                                    t.begin(w, h);
                                    let (gw, gh) = t.gui_size(w, h);
                                    if let Some((p, rm)) = hud.as_mut() {
                                        p.begin(t.scale());
                                        hud::draw_hotbar(
                                            p, t, &g.device, &g.queue, rm, &v.inv, gw as f32,
                                            gh as f32,
                                        );
                                        p.flush(&g.device, &g.queue, view, w, h, None);
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
                                            .and_then(|i| i.resource_manager())
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
                            match screenshot::save(std::path::Path::new("."), &shot) {
                                Ok(p) => tracing::info!("скриншот сохранён: {}", p.display()),
                                Err(e) => tracing::error!("скриншот: {e}"),
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

fn main() {
    let packets = std::env::args().any(|a| a == "--log-packets");
    let _ = log::init(&log::LogConfig {
        log_packets: packets,
        ..Default::default()
    });
    tracing::info!("ezzmc");
    let el = match EventLoop::new() {
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
        menu_active: false,
        join_pending: false,
    };
    if let Err(e) = el.run_app(&mut app) {
        tracing::error!("event loop: {e}");
    }
}
