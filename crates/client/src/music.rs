//! Фоновая музыка. VANILLA: MusicManager#tick (26.3), Musics.
use crate::audio::{self, Cat};
use std::sync::atomic::Ordering;
use std::time::Instant;

pub use ezzmc_world::audio_attr::MusicDef as Music;

/// VANILLA: Musics.MENU.
pub fn menu() -> Music {
    Music {
        sound: "music.menu".into(),
        min_delay: 20,
        max_delay: 600,
        replace: true,
    }
}

pub struct MusicManager {
    next_delay: i32,
    current: Option<String>,
    rng: u64,
    last: Instant,
    acc: f64,
}

impl MusicManager {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |d| d.as_nanos() as u64)
            | 1;
        MusicManager {
            next_delay: 100,
            current: None,
            rng: seed,
            last: Instant::now(),
            acc: 0.0,
        }
    }

    fn next_int(&mut self, lo: i32, hi: i32) -> i32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        if hi <= lo {
            return lo;
        }
        lo + (self.rng % (hi - lo + 1) as u64) as i32
    }

    /// Вызывать каждый кадр: сам делит время на тики по 50 мс.
    pub fn update(&mut self, music: Option<&Music>, loading: bool) {
        let now = Instant::now();
        self.acc += now.duration_since(self.last).as_secs_f64();
        self.last = now;
        let ticks = (self.acc / 0.05) as i32;
        self.acc -= f64::from(ticks) * 0.05;
        for _ in 0..ticks.min(5) {
            self.tick(music, loading);
        }
    }

    fn tick(&mut self, music: Option<&Music>, loading: bool) {
        let Some(m) = music else {
            self.next_delay = self.next_delay.max(100);
            return;
        };
        if let Some(cur) = self.current.clone() {
            if m.replace && cur != m.sound {
                audio::stop(Some(&cur), Some(Cat::Music));
                self.next_delay = self.next_delay_range(0, m.min_delay / 2);
                self.current = None;
            } else if audio::MUSIC_ACTIVE.load(Ordering::Relaxed) == 0 {
                // VANILLA: трек закончился → следующая пауза (частота DEFAULT = 20 мин).
                self.current = None;
                let d = self.next_delay_range(m.min_delay.min(24000), m.max_delay.min(24000));
                self.next_delay = self.next_delay.min(d);
            }
        }
        self.next_delay = self.next_delay.min(m.max_delay);
        if self.current.is_none() && !loading {
            self.next_delay -= 1;
            if self.next_delay <= 0 {
                audio::play_ui(&m.sound, Cat::Music, 1.0, 1.0);
                self.current = Some(m.sound.clone());
                self.next_delay = i32::MAX;
            }
        }
    }

    fn next_delay_range(&mut self, lo: i32, hi: i32) -> i32 {
        self.next_int(lo, hi)
    }
}
