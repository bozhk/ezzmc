//! Окно (TODO-031): winit 0.30, размер/DPI, оконный/borderless-режим, иконка, заголовок.
//! VANILLA: com.mojang.blaze3d.platform.Window (854×480 по умолчанию, заголовок «Minecraft <ver>»).
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Icon, Window, WindowAttributes},
};

/// VANILLA: Window — стартовые размеры по умолчанию.
pub const DEFAULT_WIDTH: u32 = 854;
pub const DEFAULT_HEIGHT: u32 = 480;
/// Идентификатор приложения для панели задач (совпадает с `ezzmc.desktop`).
const APP_ID: &str = "ezzmc";
const MIN_SIZE: (u32, u32) = (320, 240);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullscreenMode {
    Windowed,
    /// Borderless на текущем мониторе (без смены видеорежима).
    Borderless,
}

#[derive(Clone, Debug)]
pub struct WindowConfig {
    pub title: String,
    /// Логический размер (умножается на scale factor — HiDPI).
    pub width: u32,
    pub height: u32,
    pub fullscreen: FullscreenMode,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("ezzmc"),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            fullscreen: FullscreenMode::Windowed,
        }
    }
}

impl WindowConfig {
    pub fn attributes(&self) -> WindowAttributes {
        let mut a = Window::default_attributes()
            .with_title(self.title.clone())
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_min_inner_size(LogicalSize::new(MIN_SIZE.0, MIN_SIZE.1))
            .with_window_icon(default_icon());
        if self.fullscreen == FullscreenMode::Borderless {
            a = a.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        a = a.with_decorations(true).with_visible(true);
        // Без app_id (Wayland) / WM_CLASS (X11) панель задач не создаёт запись об окне.
        #[cfg(target_os = "linux")]
        {
            use winit::platform::{
                wayland::WindowAttributesExtWayland, x11::WindowAttributesExtX11,
            };
            a = WindowAttributesExtWayland::with_name(a, APP_ID, APP_ID);
            a = WindowAttributesExtX11::with_name(a, APP_ID, APP_ID);
        }
        a
    }
}

/// Запоминаемое состояние окна. VANILLA: Options `fullscreen`, `overrideWidth`, `overrideHeight`
/// (0 = не задано → 854×480).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowState {
    pub fullscreen: bool,
    /// Логический размер оконного (не полноэкранного) режима.
    pub width: u32,
    pub height: u32,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            fullscreen: false,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

#[allow(dead_code)] // потребитель — чтение/запись options (TODO-026)
impl WindowState {
    pub fn config(&self, title: &str) -> WindowConfig {
        WindowConfig {
            title: title.to_string(),
            width: self.width.max(MIN_SIZE.0),
            height: self.height.max(MIN_SIZE.1),
            fullscreen: if self.fullscreen {
                FullscreenMode::Borderless
            } else {
                FullscreenMode::Windowed
            },
        }
    }

    /// Обновить размер по ресайзу; в полноэкранном режиме оконный размер не трогаем.
    pub fn on_resized(&mut self, logical_w: u32, logical_h: u32) {
        if !self.fullscreen && logical_w > 0 && logical_h > 0 {
            self.width = logical_w;
            self.height = logical_h;
        }
    }

    pub fn to_options_lines(self) -> Vec<String> {
        vec![
            format!("fullscreen:{}", self.fullscreen),
            format!("overrideWidth:{}", self.width),
            format!("overrideHeight:{}", self.height),
        ]
    }

    pub fn load_options<'a>(&mut self, pairs: impl IntoIterator<Item = (&'a str, &'a str)>) {
        let (mut w, mut h) = (0u32, 0u32);
        for (k, v) in pairs {
            match k {
                "fullscreen" => {
                    if let Ok(b) = v.parse() {
                        self.fullscreen = b;
                    }
                }
                "overrideWidth" => w = v.parse().unwrap_or(0),
                "overrideHeight" => h = v.parse().unwrap_or(0),
                _ => {}
            }
        }
        if w > 0 && h > 0 {
            self.width = w;
            self.height = h;
        }
    }
}

/// VANILLA: Window#toggleFullScreen. Borderless на текущем мониторе (без смены видеорежима);
/// winit сам возвращает прежний размер/позицию при выходе.
pub fn toggle_fullscreen(w: &Window, state: &mut WindowState) {
    state.fullscreen = w.fullscreen().is_none();
    w.set_fullscreen(state.fullscreen.then_some(Fullscreen::Borderless(None)));
}

pub fn create(event_loop: &ActiveEventLoop, cfg: &WindowConfig) -> Result<Window, WindowError> {
    event_loop
        .create_window(cfg.attributes())
        .map_err(|e| WindowError::Create(e.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("не удалось создать окно: {0}")]
    Create(String),
}

/// Физический размер клиентской области, не меньше 1×1 (для swapchain).
pub fn framebuffer_size(w: &Window) -> PhysicalSize<u32> {
    let s = w.inner_size();
    PhysicalSize::new(s.width.max(1), s.height.max(1))
}

/// Встроенная иконка 32×32: бордовая плитка с фиолетовой рамкой (без файлов-ассетов).
pub fn default_icon() -> Option<Icon> {
    let (rgba, n) = icon_rgba();
    Icon::from_rgba(rgba, n, n).ok()
}

fn icon_rgba() -> (Vec<u8>, u32) {
    const N: u32 = 32;
    let mut v = Vec::with_capacity((N * N * 4) as usize);
    for y in 0..N {
        for x in 0..N {
            let edge = x < 3 || y < 3 || x >= N - 3 || y >= N - 3;
            let px: [u8; 4] = if edge {
                [0x6a, 0x2c, 0x91, 0xff]
            } else if (x / 4 + y / 4) % 2 == 0 {
                [0x7b, 0x14, 0x2c, 0xff]
            } else {
                [0x5e, 0x0f, 0x22, 0xff]
            };
            v.extend_from_slice(&px);
        }
    }
    (v, N)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_remembers_windowed_size_only() {
        let mut s = WindowState::default();
        s.on_resized(1000, 700);
        assert_eq!((s.width, s.height), (1000, 700));
        s.fullscreen = true;
        s.on_resized(2560, 1440);
        assert_eq!((s.width, s.height), (1000, 700));
        let c = s.config("t");
        assert_eq!(c.fullscreen, FullscreenMode::Borderless);
        assert_eq!((c.width, c.height), (1000, 700));
    }

    #[test]
    fn state_options_roundtrip() {
        let s = WindowState {
            fullscreen: true,
            width: 1280,
            height: 720,
        };
        let lines = s.to_options_lines();
        let mut t = WindowState::default();
        t.load_options(lines.iter().filter_map(|l| l.split_once(':')));
        assert_eq!(t, s);
        // битые значения / нулевой размер — умолчания остаются
        let mut u = WindowState::default();
        u.load_options([
            ("fullscreen", "x"),
            ("overrideWidth", "0"),
            ("overrideHeight", "500"),
        ]);
        assert_eq!(u, WindowState::default());
    }

    #[test]
    fn icon_is_valid() {
        assert!(default_icon().is_some());
    }

    #[test]
    fn default_config_is_vanilla_size() {
        let c = WindowConfig::default();
        assert_eq!((c.width, c.height), (854, 480));
        let a = c.attributes();
        assert!(a.fullscreen.is_none());
        let b = WindowConfig {
            fullscreen: FullscreenMode::Borderless,
            ..c
        }
        .attributes();
        assert!(b.fullscreen.is_some());
    }
}
