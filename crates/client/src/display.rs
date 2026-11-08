//! Ограничение FPS, vsync, GUI-масштаб (TODO-040).
//! VANILLA: FramerateLimitTracker, Window#calculateScale, Options (maxFps=120, enableVsync=true, guiScale=0).
#![allow(dead_code)]
use std::time::{Duration, Instant};

/// VANILLA: Options.UNLIMITED_FRAMERATE_CUTOFF — 260 = «без ограничения».
pub const UNLIMITED_FPS: u32 = 260;
pub const DEFAULT_MAX_FPS: u32 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Throttle {
    None,
    WindowIconified,
    LongAfk,
    ShortAfk,
    OutOfLevelMenu,
}

/// VANILLA: InactivityFpsLimit — AFK-троттлинг включается этой опцией.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InactivityFpsLimit {
    Minimized,
    Afk,
}

#[derive(Debug)]
pub struct FramerateLimit {
    limit: u32,
    pub inactivity: InactivityFpsLimit,
    latest_input_ms: u64,
}

impl FramerateLimit {
    pub fn new(max_fps: u32) -> Self {
        Self {
            limit: max_fps.clamp(10, UNLIMITED_FPS),
            inactivity: InactivityFpsLimit::Afk,
            latest_input_ms: 0,
        }
    }
    pub fn limit(&self) -> u32 {
        self.limit
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v.clamp(10, UNLIMITED_FPS);
    }
    pub fn on_input(&mut self, now_ms: u64) {
        self.latest_input_ms = now_ms;
    }

    /// VANILLA: getThrottleReason. `in_level` — есть мир; `in_menu` — открыт экран/оверлей.
    pub fn throttle(
        &self,
        now_ms: u64,
        iconified: bool,
        in_level: bool,
        in_menu: bool,
    ) -> Throttle {
        if iconified {
            return Throttle::WindowIconified;
        }
        if self.inactivity == InactivityFpsLimit::Afk {
            let idle = now_ms.saturating_sub(self.latest_input_ms);
            if idle > 600_000 {
                return Throttle::LongAfk;
            }
            if idle > 60_000 {
                return Throttle::ShortAfk;
            }
        }
        if in_level || !in_menu {
            Throttle::None
        } else {
            Throttle::OutOfLevelMenu
        }
    }

    /// VANILLA: getFramerateLimit.
    pub fn effective(&self, t: Throttle) -> u32 {
        match t {
            Throttle::None => self.limit,
            Throttle::WindowIconified | Throttle::LongAfk => 10,
            Throttle::ShortAfk => self.limit.min(30),
            Throttle::OutOfLevelMenu => 60,
        }
    }
}

/// Ограничитель кадров. VANILLA: RenderSystem#limitDisplayFPS (лимит ≥260 не применяется).
#[derive(Debug)]
pub struct FrameLimiter {
    last: Instant,
}

impl Default for FrameLimiter {
    fn default() -> Self {
        Self {
            last: Instant::now(),
        }
    }
}

impl FrameLimiter {
    /// Дедлайн следующего кадра; None — без ограничения.
    pub fn deadline(&self, fps: u32) -> Option<Instant> {
        (fps < UNLIMITED_FPS && fps > 0)
            .then(|| self.last + Duration::from_secs_f64(1.0 / fps as f64))
    }
    /// Блокирует до дедлайна и отмечает начало кадра.
    pub fn wait(&mut self, fps: u32) {
        if let Some(d) = self.deadline(fps) {
            let now = Instant::now();
            if d > now {
                std::thread::sleep(d - now);
            }
        }
        self.last = Instant::now();
    }
}

/// VANILLA: Window#calculateScale(maxScale, forceEven). `max_scale` 0 = авто.
pub fn calculate_scale(fb_w: u32, fb_h: u32, max_scale: u32, force_even: bool) -> u32 {
    let mut s = 1u32;
    while s != max_scale && s < fb_w && s < fb_h && fb_w / (s + 1) >= 320 && fb_h / (s + 1) >= 240 {
        s += 1;
    }
    if force_even && s % 2 != 0 {
        s += 1;
    }
    s
}

/// GUI-размер в «масштабированных» пикселях (VANILLA: Window#setGuiScale, ceil).
pub fn scaled_size(fb_w: u32, fb_h: u32, scale: u32) -> (u32, u32) {
    (fb_w.div_ceil(scale.max(1)), fb_h.div_ceil(scale.max(1)))
}

/// VANILLA: enableVsync → Fifo; иначе Immediate (с откатом внутри Swapchain).
pub fn present_mode(vsync: bool) -> ezzmc_render::PresentMode {
    if vsync {
        ezzmc_render::PresentMode::Vsync
    } else {
        ezzmc_render::PresentMode::Immediate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_auto_matches_vanilla() {
        assert_eq!(calculate_scale(854, 480, 0, false), 2); // 854/3=284<320
        assert_eq!(calculate_scale(1920, 1080, 0, false), 4); // 1080/5=216<240
        assert_eq!(calculate_scale(3840, 2160, 0, false), 9); // 2160/10=216<240
    }

    #[test]
    fn scale_respects_max_and_even() {
        assert_eq!(calculate_scale(1920, 1080, 2, false), 2);
        assert_eq!(calculate_scale(1920, 1080, 3, true), 4);
        assert_eq!(calculate_scale(100, 100, 0, false), 1);
        assert_eq!(calculate_scale(100, 100, 0, true), 2);
    }

    #[test]
    fn scaled_size_ceils() {
        assert_eq!(scaled_size(854, 480, 2), (427, 240));
        assert_eq!(scaled_size(855, 481, 2), (428, 241));
    }

    #[test]
    fn throttle_rules() {
        let mut f = FramerateLimit::new(120);
        f.on_input(0);
        assert_eq!(f.throttle(1000, false, true, false), Throttle::None);
        assert_eq!(f.effective(Throttle::None), 120);
        assert_eq!(
            f.throttle(1000, true, true, false),
            Throttle::WindowIconified
        );
        assert_eq!(f.effective(Throttle::WindowIconified), 10);
        assert_eq!(f.throttle(60_001, false, true, false), Throttle::ShortAfk);
        assert_eq!(f.effective(Throttle::ShortAfk), 30);
        assert_eq!(f.throttle(600_001, false, true, false), Throttle::LongAfk);
        assert_eq!(
            f.throttle(1000, false, false, true),
            Throttle::OutOfLevelMenu
        );
        assert_eq!(f.effective(Throttle::OutOfLevelMenu), 60);
        f.inactivity = InactivityFpsLimit::Minimized;
        assert_eq!(f.throttle(900_000, false, true, false), Throttle::None);
    }

    #[test]
    fn short_afk_never_raises_limit() {
        let f = FramerateLimit::new(20);
        assert_eq!(f.effective(Throttle::ShortAfk), 20);
    }

    #[test]
    fn limiter_deadline() {
        let l = FrameLimiter::default();
        assert!(l.deadline(UNLIMITED_FPS).is_none());
        let d = l.deadline(100).expect("deadline");
        assert!(d > Instant::now() || d.elapsed() < Duration::from_millis(50));
    }

    #[test]
    fn vsync_maps_to_present_mode() {
        assert_eq!(present_mode(true), ezzmc_render::PresentMode::Vsync);
        assert_eq!(present_mode(false), ezzmc_render::PresentMode::Immediate);
    }
}
