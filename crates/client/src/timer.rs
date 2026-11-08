//! Фиксированный тик 20 TPS + интерполяция рендера (TODO-034).
//! VANILLA: DeltaTracker.Timer (1.21.11), Minecraft#runTick: MAX_TICKS_PER_UPDATE = 10.
use std::time::Instant;

pub const TICKS_PER_SECOND: f32 = 20.0;
/// VANILLA: Minecraft.MAX_TICKS_PER_UPDATE
pub const MAX_TICKS_PER_UPDATE: u32 = 10;

#[derive(Debug)]
pub struct Timer {
    delta_ticks: f32,
    delta_tick_residual: f32,
    realtime_delta_ticks: f32,
    paused_delta_tick_residual: f32,
    last_ms: u64,
    last_ui_ms: u64,
    ms_per_tick: f32,
    paused: bool,
    frozen: bool,
}

impl Timer {
    /// `now_ms` — текущее время в мс (монотонное).
    pub fn new(ticks_per_second: f32, now_ms: u64) -> Self {
        Self {
            delta_ticks: 0.0,
            delta_tick_residual: 0.0,
            realtime_delta_ticks: 0.0,
            paused_delta_tick_residual: 0.0,
            last_ms: now_ms,
            last_ui_ms: now_ms,
            ms_per_tick: 1000.0 / ticks_per_second,
            paused: false,
            frozen: false,
        }
    }

    /// VANILLA: advanceTime(long, boolean) — сколько игровых тиков выполнить в этом кадре.
    /// `advance_game_time=false` (мир не идёт) сдвигает только реальное время.
    pub fn advance_time(&mut self, now_ms: u64, advance_game_time: bool) -> u32 {
        self.realtime_delta_ticks =
            now_ms.saturating_sub(self.last_ui_ms) as f32 / self.ms_per_tick;
        self.last_ui_ms = now_ms;
        if !advance_game_time {
            return 0;
        }
        // targetMsptProvider в ванили — tick-rate сервера (/tick rate); 20 TPS по умолчанию.
        self.delta_ticks = now_ms.saturating_sub(self.last_ms) as f32 / self.ms_per_tick;
        self.last_ms = now_ms;
        self.delta_tick_residual += self.delta_ticks;
        let n = self.delta_tick_residual as u32;
        self.delta_tick_residual -= n as f32;
        n
    }

    #[allow(dead_code)] // используется позже: пауза (039), анимации UI
    pub fn update_pause_state(&mut self, paused: bool) {
        if paused {
            if !self.paused {
                self.paused_delta_tick_residual = self.delta_tick_residual;
            }
        } else if self.paused {
            self.delta_tick_residual = self.paused_delta_tick_residual;
        }
        self.paused = paused;
    }

    #[allow(dead_code)] // используется позже: пауза (039), анимации UI
    pub fn update_frozen_state(&mut self, frozen: bool) {
        self.frozen = frozen;
    }

    #[allow(dead_code)] // используется позже: пауза (039), анимации UI
    pub fn game_time_delta_ticks(&self) -> f32 {
        self.delta_ticks
    }

    /// VANILLA: getGameTimeDeltaPartialTick(ignoreFreeze) — partialTick для интерполяции.
    pub fn partial_tick(&self, ignore_freeze: bool) -> f32 {
        if !ignore_freeze && self.frozen {
            1.0
        } else if self.paused {
            self.paused_delta_tick_residual
        } else {
            self.delta_tick_residual
        }
    }

    #[allow(dead_code)] // используется позже: пауза (039), анимации UI
    pub fn realtime_delta_ticks(&self) -> f32 {
        if self.realtime_delta_ticks > 7.0 {
            0.5
        } else {
            self.realtime_delta_ticks
        }
    }
}

/// Монотонные миллисекунды с момента старта (аналог Util.getMillis).
pub struct Clock(Instant);

impl Clock {
    pub fn start() -> Self {
        Self(Instant::now())
    }
    pub fn now_ms(&self) -> u64 {
        self.0.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_20_tps() {
        let mut t = Timer::new(TICKS_PER_SECOND, 0);
        // 60 FPS ≈ 16–17 мс: за 1000 мс ровно 20 тиков.
        let mut ticks = 0;
        for ms in (0..=1000).step_by(10) {
            ticks += t.advance_time(ms, true);
        }
        assert_eq!(ticks, 20);
    }

    #[test]
    fn residual_is_partial_tick() {
        let mut t = Timer::new(20.0, 0);
        assert_eq!(t.advance_time(75, true), 1); // 1.5 тика
        assert!((t.partial_tick(false) - 0.5).abs() < 1e-6);
        assert!((t.game_time_delta_ticks() - 1.5).abs() < 1e-6);
    }

    #[test]
    fn pause_freezes_partial_tick() {
        let mut t = Timer::new(20.0, 0);
        t.advance_time(25, true); // residual 0.5
        t.update_pause_state(true);
        t.advance_time(1000, true); // идёт, но paused-значение замороженное
        assert!((t.partial_tick(false) - 0.5).abs() < 1e-6);
        t.update_pause_state(false);
        assert!((t.partial_tick(false) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn frozen_gives_one_unless_ignored() {
        let mut t = Timer::new(20.0, 0);
        t.advance_time(25, true);
        t.update_frozen_state(true);
        assert_eq!(t.partial_tick(false), 1.0);
        assert!((t.partial_tick(true) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn realtime_spike_clamped() {
        let mut t = Timer::new(20.0, 0);
        t.advance_time(1000, false);
        assert_eq!(t.realtime_delta_ticks(), 0.5);
    }

    #[test]
    fn world_not_advancing_gives_no_ticks() {
        let mut t = Timer::new(20.0, 0);
        assert_eq!(t.advance_time(500, false), 0);
        // игровое время накопилось с last_ms=0 и выдаст тики позже (как в ванили)
        assert_eq!(t.advance_time(500, true), 10);
    }
}
