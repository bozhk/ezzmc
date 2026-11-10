//! Управление клиентом снаружи (агент играет без рук): TCP-сокет на localhost, одна JSON-команда
//! в строке → один JSON-ответ. Включается флагом `--control [host:port]` (по умолчанию
//! `127.0.0.1:25573`; порт переопределяется env `EZZMC_CONTROL_PORT`). Не ваниль, не влияет на игру.
//! Обёртка MCP для агента — `tools/ezzmc_mcp.py`.

use serde_json::{json, Value};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::mpsc::{channel, Receiver, Sender},
    time::{Duration, Instant},
};

pub const DEFAULT_ADDR: &str = "127.0.0.1:25573";

pub struct Request {
    pub cmd: Value,
    pub reply: Sender<Value>,
}

pub struct Control {
    pub rx: Receiver<Request>,
}

/// Команда, завершающаяся позже: отпускание клавиши, ожидание события.
pub struct Pending {
    pub reply: Option<Sender<Value>>,
    pub until: Instant,
    pub what: Wait,
}

pub enum Wait {
    Time,
    Release(crate::keymap::Input),
    Joined,
}

impl Control {
    pub fn start(addr: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let (tx, rx) = channel::<Request>();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let tx = tx.clone();
                std::thread::spawn(move || {
                    let Ok(read) = stream.try_clone() else { return };
                    let mut write = stream;
                    for line in BufReader::new(read).lines().map_while(Result::ok) {
                        if line.trim().is_empty() {
                            continue;
                        }
                        let answer = match serde_json::from_str::<Value>(&line) {
                            Ok(cmd) => {
                                let (rtx, rrx) = channel();
                                if tx.send(Request { cmd, reply: rtx }).is_err() {
                                    return;
                                }
                                rrx.recv_timeout(Duration::from_secs(120))
                                    .unwrap_or_else(|_| err("timeout"))
                            }
                            Err(e) => err(format!("bad json: {e}")),
                        };
                        if writeln!(write, "{answer}").is_err() {
                            return;
                        }
                    }
                });
            }
        });
        Ok(Self { rx })
    }
}

pub fn ok(v: Value) -> Value {
    let mut v = if v.is_object() {
        v
    } else {
        json!({ "value": v })
    };
    v["ok"] = json!(true);
    v
}

pub fn err(msg: impl std::fmt::Display) -> Value {
    json!({"ok": false, "error": msg.to_string()})
}

/// Имя клавиши агента → ввод клиента (`w`, `space`, `shift`, `ctrl`, `esc`, `1`..`9`, стрелки…).
pub fn parse_input(name: &str) -> Option<crate::keymap::Input> {
    let n = name.to_ascii_lowercase();
    let full = match n.as_str() {
        "shift" | "lshift" => "key.keyboard.left.shift".to_string(),
        "ctrl" | "control" | "lctrl" => "key.keyboard.left.control".to_string(),
        "esc" | "escape" => "key.keyboard.escape".to_string(),
        "enter" | "return" => "key.keyboard.enter".to_string(),
        "up" | "down" | "left" | "right" => format!("key.keyboard.{n}"),
        "left_click" => "key.mouse.left".to_string(),
        "right_click" => "key.mouse.right".to_string(),
        _ => format!("key.keyboard.{n}"),
    };
    crate::keymap::Input::parse(&full)
}

use crate::{keymap::Input, App};
use winit::{event::ElementState, event_loop::ActiveEventLoop, keyboard::KeyCode};

impl App {
    /// Нажать/отпустить ввод агента: те же пути, что у настоящей клавиатуры (опрос движения,
    /// хотбар, инвентарь, выброс, левая рука).
    fn control_press(&mut self, input: Input, down: bool) {
        let state = if down {
            ElementState::Pressed
        } else {
            ElementState::Released
        };
        if down && self.menu_active && self.capture_bind(input) {
            return;
        }
        // Та же логика F3 / F3+H, что в `WindowEvent::KeyboardInput` (main.rs): агентский ввод её обходит.
        if let Input::Key(KeyCode::F3) = input {
            if down {
                self.f3_pending = true;
            } else if self.f3_pending {
                self.f3_pending = false;
                self.debug_on = !self.debug_on;
            }
            return;
        }
        if down {
            if let Input::Key(KeyCode::KeyH) = input {
                if self.f3_pending {
                    crate::tooltip::ADVANCED.fetch_xor(true, std::sync::atomic::Ordering::Relaxed);
                }
            }
            self.f3_pending = false;
        }
        if down {
            let is = |app: &Self, n: &str| app.keys.get(n).is_some_and(|b| b.key == input);
            if let Input::Key(KeyCode::Escape) = input {
                if self.inv_open {
                    self.close_inv(true);
                    return;
                }
            }
            if let (Input::Key(KeyCode::Escape), false) = (input, self.menu_active) {
                if !self.inv_open {
                    self.open_pause_menu();
                    return;
                }
            }
            if self.inv_open && is(self, "key.inventory") {
                self.close_inv(true);
                return;
            }
            let hotbar = (1..=9).find(|i| is(self, &format!("key.hotbar.{i}")));
            if let (Some(n), false) = (hotbar, self.inv_open) {
                self.hotbar_key(n - 1);
            }
            if self.inv_open {
                // экран: как настоящая клавиатура — цифры/F/Q над слотом + состояние ввода
                if let Input::Key(k) = input {
                    self.inv_key(k);
                }
            } else if is(self, "key.inventory") {
                self.open_inv();
                return;
            } else if is(self, "key.drop") {
                let all = self.input.key_down(KeyCode::ControlLeft);
                if let Some(v) = &mut self.view {
                    v.drop_item(all);
                }
            } else if is(self, "key.swapOffhand") {
                if let Some(v) = &mut self.view {
                    v.swap_offhand();
                }
            } else if is(self, "key.togglePerspective") {
                if let Some(v) = &mut self.view {
                    v.cycle_perspective();
                }
            }
        }
        match input {
            Input::Key(k) => self.input.on_key(k, state),
            Input::Mouse(b) => self.input.on_button(b, state),
            Input::Unbound => return,
        }
        self.keys.on_input(input, down);
    }

    fn control_state(&self) -> Value {
        let screen = if self.view.is_none() {
            "menu"
        } else if self.menu_active {
            "pause"
        } else if self.inv_open {
            "inventory"
        } else if self.chat.open {
            "chat"
        } else {
            "play"
        };
        let mut s = json!({
            "audio": {
                "active": crate::audio::ACTIVE.load(std::sync::atomic::Ordering::Relaxed),
                "started": crate::audio::STARTED.load(std::sync::atomic::Ordering::Relaxed),
            },
            "screen": screen,
            "frames": self.frames,
            "fps": self.fps,
            "yaw": self.yaw,
            "pitch": self.pitch,
        });
        if let Some(v) = &self.view {
            s["joined"] = json!(v.joined && v.have_pos);
            s["pos"] = json!(v.player.pos);
            s["vel"] = json!(v.player.vel);
            s["on_ground"] = json!(v.player.on_ground);
            s["flying"] = json!(v.player.flying);
            s["sprinting"] = json!(v.player.sprinting);
            // Скорость за последний тик, блоков/с (как строка Speed в F3).
            let (p, q) = (v.player.pos, v.player.prev_pos);
            let (dx, dy, dz) = (p[0] - q[0], p[1] - q[1], p[2] - q[2]);
            s["speed"] = json!((dx * dx + dy * dy + dz * dz).sqrt() * 20.0);
            s["speed_xz"] = json!((dx * dx + dz * dz).sqrt() * 20.0);
            s["crouching"] = json!(v.player.crouching);
            s["food"] = json!(v.food);
            s["using"] = json!(v.using.is_some());
            s["use_remaining"] = json!(v.using.as_ref().map(|u| u.remaining));
            s["sneaking"] = json!(v.player.crouching);
            s["game_mode"] = json!(v.game_mode);
            s["held_slot"] = json!(v.inv.selected);
            s["sections"] = json!(v.resident_sections());
        }
        s["chat"] = json!({
            "open": self.chat.open,
            "input": self.chat.edit.text,
            "lines": self.chat.plain_lines(10),
        });
        if self.inv_open && self.inv_creative {
            s["creative"] = json!({
                "tab": self.creative.tab,
                "row": self.creative.row,
                "search": self.creative.search,
            });
        }
        s
    }

    /// Раз в кадр: завершить отложенные команды и принять новые из сокета.
    pub(crate) fn control_tick(&mut self, _el: &ActiveEventLoop) {
        if self.control.is_none() {
            return;
        }
        // Мир без реального захвата курсора: «захват» включён, пока агент в игре.
        if self.view.is_some() && !self.menu_active && !self.inv_open && !self.chat.open {
            self.mouse.grabbed = true;
        }
        self.focus.pause_on_lost_focus = false;
        let now = std::time::Instant::now();
        let joined = self.view.as_ref().is_some_and(|v| v.joined && v.have_pos);
        let mut i = 0;
        while i < self.control_pending.len() {
            let p = &self.control_pending[i];
            let timed_out = now >= p.until;
            let done = match p.what {
                Wait::Time | Wait::Release(_) => timed_out,
                Wait::Joined => joined || timed_out,
            };
            if !done {
                i += 1;
                continue;
            }
            let p = self.control_pending.remove(i);
            if let Wait::Release(input) = p.what {
                self.control_press(input, false);
            }
            let failed = timed_out && matches!(p.what, Wait::Joined);
            if let Some(r) = p.reply {
                let _ = r.send(if failed {
                    err("timeout")
                } else {
                    ok(json!({}))
                });
            }
        }
        while let Some(req) = self.control.as_ref().and_then(|c| c.rx.try_recv().ok()) {
            if let Some(answer) = self.control_command(&req.cmd, &req.reply) {
                let _ = req.reply.send(answer);
            }
        }
    }

    fn control_command(&mut self, c: &Value, reply: &Sender<Value>) -> Option<Value> {
        let cmd = c["cmd"].as_str().unwrap_or("");
        let ms = c["ms"].as_u64().unwrap_or(100);
        let f = |k: &str| c[k].as_f64();
        let later = |app: &mut Self, what: Wait, reply: Option<Sender<Value>>, ms: u64| {
            app.control_pending.push(Pending {
                reply,
                until: Instant::now() + Duration::from_millis(ms),
                what,
            });
        };
        let button = || match c["button"].as_str().unwrap_or("left") {
            "right" => Some(Input::Mouse(winit::event::MouseButton::Right)),
            "middle" => Some(Input::Mouse(winit::event::MouseButton::Middle)),
            "left" => Some(Input::Mouse(winit::event::MouseButton::Left)),
            _ => None,
        };
        Some(match cmd {
            "ping" => ok(json!({"pong": true})),
            // Чат: chat_open {text} открывает экран; chat_type {text} печатает в поле;
            // chat_send {text?} отправляет (как Enter); chat_key {key: up|down|pageup|pagedown|esc}.
            "chat_open" => {
                self.open_chat(c["text"].as_str().unwrap_or(""));
                ok(json!({}))
            }
            "chat_type" => {
                if !self.chat.open {
                    return Some(err("chat not open"));
                }
                self.chat.edit.type_text(c["text"].as_str().unwrap_or(""));
                ok(json!({}))
            }
            "chat_send" => {
                if !self.chat.open {
                    self.open_chat("");
                }
                if let Some(t) = c["text"].as_str() {
                    self.chat.edit.set(t);
                }
                self.chat_submit();
                ok(json!({}))
            }
            "chat_key" => {
                let cfg = self.chat_cfg;
                match c["key"].as_str().unwrap_or("") {
                    "up" | "down" if self.chat.open => {
                        let d = if c["key"] == "up" { -1 } else { 1 };
                        let cur = self.chat.edit.text.clone();
                        if let Some(t) = self.chat.hist.step(d, &cur) {
                            self.chat.edit.set(&t);
                        }
                    }
                    "pageup" => self.chat.page(true, &cfg),
                    "pagedown" => self.chat.page(false, &cfg),
                    "esc" => self.close_chat(),
                    _ => return Some(err("bad key or chat closed")),
                }
                ok(json!({}))
            }
            "state" => ok(self.control_state()),
            "key" | "tap" => {
                let Some(input) = c["key"].as_str().and_then(parse_input) else {
                    return Some(err("unknown key"));
                };
                if cmd == "key" {
                    self.control_press(input, c["down"].as_bool().unwrap_or(true));
                    ok(json!({}))
                } else {
                    self.control_press(input, true);
                    later(self, Wait::Release(input), Some(reply.clone()), ms);
                    return None;
                }
            }
            "mouse" | "click" => {
                let Some(input) = button() else {
                    return Some(err("unknown button"));
                };
                if self.view.is_none() || self.menu_active || self.inv_open {
                    return Some(err("not in play screen"));
                }
                if cmd == "mouse" {
                    self.control_press(input, c["down"].as_bool().unwrap_or(true));
                    ok(json!({}))
                } else {
                    self.control_press(input, true);
                    later(self, Wait::Release(input), Some(reply.clone()), ms);
                    return None;
                }
            }
            "slot_click" => {
                // Агент: навести курсор на слот открытого экрана и нажать/отпустить кнопку.
                if !self.inv_open || self.inv_creative {
                    return Some(err("inventory screen not open"));
                }
                let Some(px) = c["slot"].as_u64().and_then(|s| self.slot_px(s as usize)) else {
                    return Some(err("no such slot"));
                };
                let right = c["button"].as_str() == Some("right");
                self.cursor_px = px;
                self.inv_cursor_moved();
                if c["hover"].as_bool() == Some(true) {
                    // Только навести курсор (проверка подсказок).
                    return Some(ok(json!({})));
                }
                self.inv_mouse(true, right, !right, false);
                if let Some(to) = c["drag_to"].as_array() {
                    for s in to.iter().filter_map(Value::as_u64) {
                        if let Some(px) = self.slot_px(s as usize) {
                            self.cursor_px = px;
                            self.inv_cursor_moved();
                        }
                    }
                }
                self.inv_mouse(false, right, !right, false);
                ok(json!({}))
            }
            "creative_click" => {
                // Агент: навести курсор на элемент креатив-экрана (tab|cell|slot|trash|outside)
                // и нажать/отпустить кнопку; `drag_to` — слоты инвентаря для перетаскивания.
                if !self.inv_open || !self.inv_creative {
                    return Some(err("creative screen not open"));
                }
                let Some((gw, gh, _, _)) = self.gui_cursor() else {
                    return Some(err("no window"));
                };
                use crate::creativescreen as cs;
                let target = |app: &Self, c: &Value| -> Option<(f32, f32)> {
                    if let Some(t) = c["tab"].as_u64() {
                        cs::tab_center(gw, gh, t as usize)
                    } else if let Some(n) = c["cell"].as_u64() {
                        cs::slot_center(&app.creative, gw, gh, cs::Slot::Cell(n as usize))
                    } else if let Some(n) = c["slot"].as_u64() {
                        cs::slot_center(&app.creative, gw, gh, cs::Slot::Inv(n as usize))
                    } else if c["trash"].as_bool() == Some(true) {
                        cs::slot_center(&app.creative, gw, gh, cs::Slot::Trash)
                    } else if c["outside"].as_bool() == Some(true) {
                        Some((2.0, 2.0))
                    } else {
                        Some((c["x"].as_f64()? as f32, c["y"].as_f64()? as f32))
                    }
                };
                let Some((x, y)) = target(self, c) else {
                    return Some(err("no such target"));
                };
                let k = self.text.as_ref().map_or(1.0, |t| f64::from(t.scale()));
                let b = c["button"].as_str().unwrap_or("left");
                let (right, left, middle) = (b == "right", b == "left", b == "middle");
                self.cursor_px = (f64::from(x) * k, f64::from(y) * k);
                self.inv_cursor_moved();
                if c["hover"].as_bool() == Some(true) {
                    // Только навести курсор (проверка подсказок креатива).
                    return Some(ok(json!({})));
                }
                self.inv_mouse(true, right, left, middle);
                if let Some(to) = c["drag_to"].as_array() {
                    for n in to.iter().filter_map(Value::as_u64) {
                        if let Some((x, y)) =
                            cs::slot_center(&self.creative, gw, gh, cs::Slot::Inv(n as usize))
                        {
                            self.cursor_px = (f64::from(x) * k, f64::from(y) * k);
                            self.inv_cursor_moved();
                        }
                    }
                }
                if c["hold"].as_bool() == Some(true) {
                    // Кнопка остаётся зажатой: для скриншота превью раздачи; отпустить — creative_release.
                    self.held_button = Some((right, left, middle));
                } else {
                    self.inv_mouse(false, right, left, middle);
                }
                ok(json!({}))
            }
            "creative_release" => {
                let Some((right, left, middle)) = self.held_button.take() else {
                    return Some(err("no held button"));
                };
                self.inv_mouse(false, right, left, middle);
                ok(json!({}))
            }
            "creative_scroll" => {
                if !self.inv_open || !self.inv_creative {
                    return Some(err("creative screen not open"));
                }
                self.creative.scroll(c["dy"].as_f64().unwrap_or(-1.0));
                ok(json!({"row": self.creative.row}))
            }
            "creative_search" => {
                if !self.inv_open || !self.inv_creative {
                    return Some(err("creative screen not open"));
                }
                if !self.creative.is_search() {
                    return Some(err("search tab not selected"));
                }
                self.creative
                    .type_text(c["text"].as_str().unwrap_or(""), &self.item_lang);
                ok(json!({"search": self.creative.search}))
            }
            "raw_move" => {
                // TEMP-RAW-MOVE: разовый пакет move_player_pos_rot с точными x,y,z (для A/B).
                // x_bits: точный f64 через битовое представление (serde_json без float_roundtrip может промахнуться на 1 ULP).
                let x = c["x_bits"].as_u64().map(f64::from_bits).or_else(|| f("x"));
                if let (Some(v), Some(x), Some(y), Some(z)) = (self.view.as_ref(), x, f("y"), f("z")) {
                    v.send(crate::netthread::NetCmd::RawMove {
                        pos: [x, y, z],
                        yaw: f("yaw").unwrap_or(0.0) as f32,
                        pitch: f("pitch").unwrap_or(0.0) as f32,
                        on_ground: c["on_ground"].as_bool().unwrap_or(true),
                    });
                    ok(json!({}))
                } else {
                    err("raw_move: нет вида или x/y/z")
                }
            }
            "look" => {
                if let Some(y) = f("yaw") {
                    self.yaw = y as f32;
                }
                if let Some(p) = f("pitch") {
                    self.pitch = (p as f32).clamp(-90.0, 90.0);
                }
                ok(json!({}))
            }
            "turn" => {
                self.yaw += f("dyaw").unwrap_or(0.0) as f32;
                self.pitch = (self.pitch + f("dpitch").unwrap_or(0.0) as f32).clamp(-90.0, 90.0);
                ok(json!({}))
            }
            "sound" => {
                // Проиграть событие у слушателя (UI-категория без позиции, если нет pos).
                let ev = c["event"].as_str().unwrap_or("ui.button.click");
                let vol = f("volume").unwrap_or(1.0) as f32;
                let pitch = f("pitch").unwrap_or(1.0) as f32;
                crate::audio::play_ui(ev, crate::audio::Cat::Master, vol, pitch);
                ok(json!({}))
            }
            "hotbar" => {
                let slot = c["slot"].as_u64().unwrap_or(0).min(8) as u8;
                if let Some(v) = &mut self.view {
                    v.select_slot(slot);
                }
                ok(json!({}))
            }
            "tooltip" => {
                // Агент: строки подсказки предмета в слоте инвентаря (без наведения курсора).
                let Some(v) = &self.view else {
                    return Some(err("not in game"));
                };
                let Some(t) = self.text.as_ref() else {
                    return Some(err("no text renderer"));
                };
                let Some(i) = c["slot"].as_u64().map(|s| s as usize) else {
                    return Some(err("no slot"));
                };
                let Some(Some(s)) = v.inv.slots.get(i) else {
                    return Some(ok(json!({"lines": []})));
                };
                let lines: Vec<Value> = crate::tooltip::stack_lines(&self.item_lang, t, s)
                    .into_iter()
                    .map(|(text, color)| json!({"text": text, "color": format!("{color:06X}")}))
                    .collect();
                ok(json!({"lines": lines}))
            }
            "inventory" => {
                let Some(v) = &self.view else {
                    return Some(err("not in game"));
                };
                let slots: Vec<Value> = v
                    .inv
                    .slots
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| {
                        let s = s.as_ref()?;
                        let name = ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize);
                        Some(json!({"slot": i, "item": name, "count": s.count}))
                    })
                    .collect();
                let cursor = v.inv.cursor.as_ref().map(|s| {
                    json!({"item": ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize), "count": s.count})
                });
                ok(json!({"held_slot": v.inv.selected, "cursor": cursor, "slots": slots}))
            }
            "ui_click" => {
                // Агент: клик по экрану меню (координаты в пикселях окна).
                let (Some(x), Some(y)) = (c["x"].as_f64(), c["y"].as_f64()) else {
                    return Some(err("x, y required"));
                };
                let Some(g) = &mut self.gui else {
                    return Some(err("no menu"));
                };
                g.on_cursor(x, y);
                g.on_left_button(true);
                g.on_left_button(false);
                ok(json!({}))
            }
            "join_server" => {
                let Some(addr) = c["addr"].as_str() else {
                    return Some(err("addr required"));
                };
                self.connect = Some(addr.to_string());
                self.menu_active = false;
                self.world_pending = false;
                self.join_pending = true;
                ok(json!({}))
            }
            "wait" => {
                later(self, Wait::Time, Some(reply.clone()), ms);
                return None;
            }
            "wait_joined" => {
                let t = c["timeout_ms"].as_u64().unwrap_or(60_000);
                later(self, Wait::Joined, Some(reply.clone()), t);
                return None;
            }
            "screenshot" => {
                self.want_screenshot = true;
                self.shot_reply = Some(reply.clone());
                return None;
            }
            "quit" => {
                _ = reply.send(ok(json!({})));
                std::process::exit(0);
            }
            other => err(format!("unknown command {other}")),
        })
    }
}
