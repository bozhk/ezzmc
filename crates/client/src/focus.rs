//! Фокус окна (TODO-039): флаг windowActive, автопауза через 500 мс без фокуса.
//! VANILLA: Minecraft#setWindowActive/isWindowActive, GameRenderer#render (pauseOnLostFocus, 500 мс),
//! MouseHandler (движение игнорируется без фокуса).
#![allow(dead_code)]

/// VANILLA: GameRenderer — задержка перед pauseGame при потере фокуса.
pub const PAUSE_DELAY_MS: u64 = 500;

#[derive(Debug)]
pub struct Focus {
    pub active: bool,
    /// VANILLA: Options#pauseOnLostFocus (по умолчанию true).
    pub pause_on_lost_focus: bool,
    last_active_ms: u64,
}

impl Focus {
    pub fn new(now_ms: u64) -> Self {
        Self {
            active: true,
            pause_on_lost_focus: true,
            last_active_ms: now_ms,
        }
    }

    pub fn set_active(&mut self, a: bool) {
        self.active = a;
    }

    /// Вызывать раз в кадр. true → пора открыть паузу (`pauseGame(false)`): она повторяется
    /// каждый кадр после задержки, как в ванили; потребитель сам игнорирует уже открытую паузу.
    pub fn frame(&mut self, now_ms: u64) -> bool {
        if !self.active && self.pause_on_lost_focus {
            now_ms.saturating_sub(self.last_active_ms) > PAUSE_DELAY_MS
        } else {
            self.last_active_ms = now_ms;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pauses_only_after_delay() {
        let mut f = Focus::new(0);
        f.pause_on_lost_focus = true;
        assert!(!f.frame(1000));
        f.set_active(false);
        assert!(!f.frame(1400));
        assert!(!f.frame(1500));
        assert!(f.frame(1501));
        assert!(f.frame(2000));
    }

    #[test]
    fn regain_focus_resets() {
        let mut f = Focus::new(0);
        f.pause_on_lost_focus = true;
        f.set_active(false);
        assert!(!f.frame(100));
        f.set_active(true);
        assert!(!f.frame(5000));
        f.set_active(false);
        assert!(!f.frame(5300));
    }

    #[test]
    fn default_is_vanilla_pause_on_lost_focus() {
        let mut f = Focus::new(0);
        assert!(f.pause_on_lost_focus);
        f.set_active(false);
        assert!(f.frame(10_000));
    }

    #[test]
    fn option_off_never_pauses() {
        let mut f = Focus::new(0);
        f.pause_on_lost_focus = false;
        f.set_active(false);
        assert!(!f.frame(10_000));
    }
}
