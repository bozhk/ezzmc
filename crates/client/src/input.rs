//! Ввод (TODO-036): сырая мышь, захват курсора, ванильная кривая чувствительности, keymap.
//! VANILLA: MouseHandler#turnPlayer / #grabMouse / #releaseMouse, KeyboardHandler, InputConstants.
#![allow(dead_code)]
use std::collections::HashSet;
use winit::{
    event::{ElementState, MouseButton},
    keyboard::KeyCode,
    window::{CursorGrabMode, Window},
};

/// VANILLA: Options#sensitivity по умолчанию (0.5 → «100%»).
pub const DEFAULT_SENSITIVITY: f64 = 0.5;
/// VANILLA: Entity#turn — градусов на единицу после кривой.
const TURN_SCALE: f64 = 0.15;

/// VANILLA: MouseHandler#turnPlayer — `f = s*0.6+0.2; f^3 * 8`.
pub fn sensitivity_factor(sensitivity: f64) -> f64 {
    let f = sensitivity.clamp(0.0, 1.0) * 0.6 + 0.2;
    f * f * f * 8.0
}

/// Результат поворота: приращение (yaw, pitch) в градусах.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Turn {
    pub yaw: f64,
    pub pitch: f64,
}

/// VANILLA: Entity#turn — pitch ограничен ±90.
pub fn apply_turn(yaw: f32, pitch: f32, t: Turn) -> (f32, f32) {
    (
        yaw + t.yaw as f32,
        (pitch + t.pitch as f32).clamp(-90.0, 90.0),
    )
}

/// Накопитель сырого движения мыши + состояние захвата.
#[derive(Debug)]
pub struct Mouse {
    dx: f64,
    dy: f64,
    pub grabbed: bool,
    pub sensitivity: f64,
    pub invert_y: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            dx: 0.0,
            dy: 0.0,
            grabbed: false,
            sensitivity: DEFAULT_SENSITIVITY,
            invert_y: false,
        }
    }
}

impl Mouse {
    /// Сырая дельта (DeviceEvent::MouseMotion). Вне захвата игнорируется, как в ванили.
    pub fn on_motion(&mut self, dx: f64, dy: f64) {
        if self.grabbed {
            self.dx += dx;
            self.dy += dy;
        }
    }

    /// Забрать накопленное движение за кадр как поворот.
    pub fn take_turn(&mut self) -> Turn {
        let k = sensitivity_factor(self.sensitivity) * TURN_SCALE;
        let sy = if self.invert_y { -1.0 } else { 1.0 };
        let t = Turn {
            yaw: self.dx * k,
            pitch: self.dy * k * sy,
        };
        self.dx = 0.0;
        self.dy = 0.0;
        t
    }

    /// VANILLA: grabMouse. Confined→Locked с откатом (Wayland: только Locked, X11: Confined).
    pub fn grab(&mut self, w: &Window) {
        if self.grabbed {
            return;
        }
        let ok = w.set_cursor_grab(CursorGrabMode::Locked).is_ok()
            || w.set_cursor_grab(CursorGrabMode::Confined).is_ok();
        if ok {
            w.set_cursor_visible(false);
            self.grabbed = true;
            self.dx = 0.0;
            self.dy = 0.0;
        }
    }

    /// VANILLA: releaseMouse.
    pub fn release(&mut self, w: &Window) {
        if !self.grabbed {
            return;
        }
        let _ = w.set_cursor_grab(CursorGrabMode::None);
        w.set_cursor_visible(true);
        self.grabbed = false;
        self.dx = 0.0;
        self.dy = 0.0;
    }
}

/// Нажатые клавиши/кнопки.
#[derive(Debug, Default)]
pub struct InputState {
    keys: HashSet<KeyCode>,
    buttons: HashSet<MouseButton>,
}

impl InputState {
    pub fn on_key(&mut self, k: KeyCode, s: ElementState) {
        match s {
            ElementState::Pressed => self.keys.insert(k),
            ElementState::Released => self.keys.remove(&k),
        };
    }
    pub fn on_button(&mut self, b: MouseButton, s: ElementState) {
        match s {
            ElementState::Pressed => self.buttons.insert(b),
            ElementState::Released => self.buttons.remove(&b),
        };
    }
    pub fn key_down(&self, k: KeyCode) -> bool {
        self.keys.contains(&k)
    }
    pub fn button_down(&self, b: MouseButton) -> bool {
        self.buttons.contains(&b)
    }
    /// Потеря фокуса: отпустить всё (VANILLA: KeyMapping.releaseAll).
    pub fn release_all(&mut self) {
        self.keys.clear();
        self.buttons.clear();
    }
}

/// Код клавиши GLFW (формат options.txt: `key_key.jump:key.keyboard.space`).
pub fn glfw_code(k: KeyCode) -> Option<i32> {
    use KeyCode::*;
    Some(match k {
        Space => 32,
        Quote => 39,
        Comma => 44,
        Minus => 45,
        Period => 46,
        Slash => 47,
        Digit0 => 48,
        Digit1 => 49,
        Digit2 => 50,
        Digit3 => 51,
        Digit4 => 52,
        Digit5 => 53,
        Digit6 => 54,
        Digit7 => 55,
        Digit8 => 56,
        Digit9 => 57,
        Semicolon => 59,
        Equal => 61,
        KeyA => 65,
        KeyB => 66,
        KeyC => 67,
        KeyD => 68,
        KeyE => 69,
        KeyF => 70,
        KeyG => 71,
        KeyH => 72,
        KeyI => 73,
        KeyJ => 74,
        KeyK => 75,
        KeyL => 76,
        KeyM => 77,
        KeyN => 78,
        KeyO => 79,
        KeyP => 80,
        KeyQ => 81,
        KeyR => 82,
        KeyS => 83,
        KeyT => 84,
        KeyU => 85,
        KeyV => 86,
        KeyW => 87,
        KeyX => 88,
        KeyY => 89,
        KeyZ => 90,
        BracketLeft => 91,
        Backslash => 92,
        BracketRight => 93,
        Backquote => 96,
        Escape => 256,
        Enter => 257,
        Tab => 258,
        Backspace => 259,
        Insert => 260,
        Delete => 261,
        ArrowRight => 262,
        ArrowLeft => 263,
        ArrowDown => 264,
        ArrowUp => 265,
        PageUp => 266,
        PageDown => 267,
        Home => 268,
        End => 269,
        CapsLock => 280,
        F1 => 290,
        F2 => 291,
        F3 => 292,
        F4 => 293,
        F5 => 294,
        F6 => 295,
        F7 => 296,
        F8 => 297,
        F9 => 298,
        F10 => 299,
        F11 => 300,
        F12 => 301,
        ShiftLeft => 340,
        ControlLeft => 341,
        AltLeft => 342,
        ShiftRight => 344,
        ControlRight => 345,
        AltRight => 346,
        _ => return None,
    })
}

/// Обратное отображение (для чтения options.txt).
pub fn from_glfw_code(code: i32) -> Option<KeyCode> {
    // Таблица небольшая; перебор по единственной истине — `glfw_code`.
    ALL_KEYS
        .iter()
        .copied()
        .find(|k| glfw_code(*k) == Some(code))
}

pub(crate) const ALL_KEYS: &[KeyCode] = &{
    use KeyCode::*;
    [
        Space,
        Quote,
        Comma,
        Minus,
        Period,
        Slash,
        Digit0,
        Digit1,
        Digit2,
        Digit3,
        Digit4,
        Digit5,
        Digit6,
        Digit7,
        Digit8,
        Digit9,
        Semicolon,
        Equal,
        KeyA,
        KeyB,
        KeyC,
        KeyD,
        KeyE,
        KeyF,
        KeyG,
        KeyH,
        KeyI,
        KeyJ,
        KeyK,
        KeyL,
        KeyM,
        KeyN,
        KeyO,
        KeyP,
        KeyQ,
        KeyR,
        KeyS,
        KeyT,
        KeyU,
        KeyV,
        KeyW,
        KeyX,
        KeyY,
        KeyZ,
        BracketLeft,
        Backslash,
        BracketRight,
        Backquote,
        Escape,
        Enter,
        Tab,
        Backspace,
        Insert,
        Delete,
        ArrowRight,
        ArrowLeft,
        ArrowDown,
        ArrowUp,
        PageUp,
        PageDown,
        Home,
        End,
        CapsLock,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        ShiftLeft,
        ControlLeft,
        AltLeft,
        ShiftRight,
        ControlRight,
        AltRight,
    ]
};

/// Имя ввода в options.txt: `key.keyboard.w`, `key.mouse.left`.
pub fn key_name(k: KeyCode) -> Option<String> {
    use KeyCode::*;
    let s = match k {
        Space => "space",
        Quote => "apostrophe",
        Comma => "comma",
        Minus => "minus",
        Period => "period",
        Slash => "slash",
        Semicolon => "semicolon",
        Equal => "equal",
        BracketLeft => "left.bracket",
        Backslash => "backslash",
        BracketRight => "right.bracket",
        Backquote => "grave.accent",
        Escape => "escape",
        Enter => "enter",
        Tab => "tab",
        Backspace => "backspace",
        Insert => "insert",
        Delete => "delete",
        ArrowRight => "right",
        ArrowLeft => "left",
        ArrowDown => "down",
        ArrowUp => "up",
        PageUp => "page.up",
        PageDown => "page.down",
        Home => "home",
        End => "end",
        CapsLock => "caps.lock",
        ShiftLeft => "left.shift",
        ControlLeft => "left.control",
        AltLeft => "left.alt",
        ShiftRight => "right.shift",
        ControlRight => "right.control",
        AltRight => "right.alt",
        _ => {
            let c = glfw_code(k)?;
            return Some(match c {
                48..=57 | 65..=90 => {
                    format!("key.keyboard.{}", (c as u8 as char).to_ascii_lowercase())
                }
                290..=301 => format!("key.keyboard.f{}", c - 289),
                _ => return None,
            });
        }
    };
    Some(format!("key.keyboard.{s}"))
}

pub fn mouse_button_name(b: MouseButton) -> Option<&'static str> {
    Some(match b {
        MouseButton::Left => "key.mouse.left",
        MouseButton::Right => "key.mouse.right",
        MouseButton::Middle => "key.mouse.middle",
        MouseButton::Back => "key.mouse.4",
        MouseButton::Forward => "key.mouse.5",
        MouseButton::Other(_) => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_sensitivity_curve() {
        // 0.5 → (0.5)^3*8 = 1.0 (ваниль: 100% = коэффициент 1.0)
        assert!((sensitivity_factor(0.5) - 1.0).abs() < 1e-12);
        // 0 → 0.2^3*8 = 0.064; 1 → 0.8^3*8 = 4.096
        assert!((sensitivity_factor(0.0) - 0.064).abs() < 1e-12);
        assert!((sensitivity_factor(1.0) - 4.096).abs() < 1e-12);
        assert_eq!(sensitivity_factor(7.0), sensitivity_factor(1.0));
    }

    #[test]
    fn motion_ignored_when_not_grabbed() {
        let mut m = Mouse::default();
        m.on_motion(10.0, 10.0);
        assert_eq!(m.take_turn(), Turn::default());
    }

    #[test]
    fn turn_scales_and_inverts_and_resets() {
        let mut m = Mouse {
            grabbed: true,
            ..Default::default()
        };
        m.on_motion(10.0, 4.0);
        m.on_motion(10.0, 0.0);
        let t = m.take_turn();
        assert!((t.yaw - 20.0 * 0.15).abs() < 1e-12);
        assert!((t.pitch - 4.0 * 0.15).abs() < 1e-12);
        assert_eq!(m.take_turn(), Turn::default());
        m.invert_y = true;
        m.on_motion(0.0, 4.0);
        assert!((m.take_turn().pitch + 0.6).abs() < 1e-12);
    }

    #[test]
    fn pitch_clamped() {
        let (_, p) = apply_turn(
            0.0,
            80.0,
            Turn {
                yaw: 0.0,
                pitch: 50.0,
            },
        );
        assert_eq!(p, 90.0);
        let (y, p) = apply_turn(
            10.0,
            -80.0,
            Turn {
                yaw: 5.0,
                pitch: -50.0,
            },
        );
        assert_eq!((y, p), (15.0, -90.0));
    }

    #[test]
    fn state_tracks_and_releases() {
        let mut s = InputState::default();
        s.on_key(KeyCode::KeyW, ElementState::Pressed);
        s.on_button(MouseButton::Left, ElementState::Pressed);
        assert!(s.key_down(KeyCode::KeyW) && s.button_down(MouseButton::Left));
        s.release_all();
        assert!(!s.key_down(KeyCode::KeyW) && !s.button_down(MouseButton::Left));
    }

    #[test]
    fn keymap_roundtrip_and_names() {
        for k in ALL_KEYS {
            let c = glfw_code(*k).expect("glfw");
            assert_eq!(from_glfw_code(c), Some(*k));
            assert!(key_name(*k).is_some(), "{k:?}");
        }
        assert_eq!(key_name(KeyCode::KeyW).expect("n"), "key.keyboard.w");
        assert_eq!(key_name(KeyCode::F11).expect("n"), "key.keyboard.f11");
        assert_eq!(key_name(KeyCode::Space).expect("n"), "key.keyboard.space");
        assert_eq!(key_name(KeyCode::Digit5).expect("n"), "key.keyboard.5");
        assert_eq!(mouse_button_name(MouseButton::Back), Some("key.mouse.4"));
    }
}
