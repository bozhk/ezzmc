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

/// VANILLA: Musics.CREDITS (WinScreen#getBackgroundMusic).
pub fn credits() -> Music {
    Music {
        sound: "music.credits".into(),
        min_delay: 0,
        max_delay: 0,
        replace: true,
    }
}

/// VANILLA: MusicManager.MusicFrequency (опция musicFrequency): макс. пауза в тиках.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Frequency {
    Default,
    Frequent,
    Constant,
}

impl Frequency {
    pub fn parse(s: &str) -> Self {
        match s.trim_matches('"') {
            "FREQUENT" => Self::Frequent,
            "CONSTANT" => Self::Constant,
            _ => Self::Default,
        }
    }

    fn max_ticks(self) -> i32 {
        match self {
            Self::Default => 20 * 1200,
            Self::Frequent => 10 * 1200,
            Self::Constant => 0,
        }
    }
}

pub struct MusicManager {
    freq: Frequency,
    next_delay: i32,
    current: Option<String>,
    rng: u64,
    last: Instant,
    acc: f64,
    /// Трек только что запущен (для NowPlayingToast).
    started: bool,
    hide_toast: bool,
}

impl MusicManager {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |d| d.as_nanos() as u64)
            | 1;
        MusicManager {
            freq: Frequency::Default,
            next_delay: 100,
            current: None,
            rng: seed,
            last: Instant::now(),
            acc: 0.0,
            started: false,
            hide_toast: false,
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

    /// VANILLA: MusicFrequency#getNextSongDelay.
    fn freq_delay(&mut self, m: Option<&Music>) -> i32 {
        let Some(m) = m else {
            return self.freq.max_ticks();
        };
        if self.freq == Frequency::Constant {
            return 100;
        }
        let max = self.freq.max_ticks();
        self.next_int(m.min_delay.min(max), m.max_delay.min(max))
    }

    /// VANILLA: MusicManager#setMinutesBetweenSongs — смена частоты пересчитывает паузу.
    pub fn set_frequency(&mut self, f: Frequency, music: Option<&Music>) {
        if self.freq != f {
            self.freq = f;
            self.next_delay = self.freq_delay(music);
        }
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
                self.hide_toast = true;
                self.next_delay = self.next_delay_range(0, m.min_delay / 2);
                self.current = None;
            } else if audio::MUSIC_ACTIVE.load(Ordering::Relaxed) == 0 {
                // VANILLA: трек закончился → следующая пауза (частота DEFAULT = 20 мин).
                self.current = None;
                let d = self.freq_delay(Some(m));
                self.next_delay = self.next_delay.min(d);
            }
        }
        self.next_delay = self.next_delay.min(m.max_delay);
        if self.current.is_none() && !loading {
            self.next_delay -= 1;
            if self.next_delay <= 0 {
                if let Ok(mut k) = audio::MUSIC_KEY.lock() {
                    *k = None;
                }
                audio::play_ui(&m.sound, Cat::Music, 1.0, 1.0);
                self.started = true;
                self.current = Some(m.sound.clone());
                self.next_delay = i32::MAX;
            }
        }
    }

    /// VANILLA: MusicManager#stopPlaying(Music) — только если играет именно `sound`;
    /// следующая пауза = getNextSongDelay(situational) + 100.
    pub fn stop_playing(&mut self, sound: &str, situational: Option<&Music>) {
        if self.current.as_deref() != Some(sound) {
            return;
        }
        audio::stop(Some(sound), Some(Cat::Music));
        self.current = None;
        self.hide_toast = true;
        self.next_delay = self.freq_delay(situational).saturating_add(100);
    }

    /// VANILLA: MusicManager#getCurrentMusicTranslationKey.
    pub fn track_key(&self) -> Option<String> {
        self.current.as_ref()?;
        audio::MUSIC_KEY.lock().ok()?.clone()
    }

    /// VANILLA: startPlaying → showNowPlayingToast (один раз на трек).
    /// VANILLA: stopPlaying → hideNowPlayingToast.
    pub fn take_hide(&mut self) -> bool {
        std::mem::take(&mut self.hide_toast)
    }

    pub fn take_started(&mut self) -> bool {
        std::mem::take(&mut self.started)
    }

    fn next_delay_range(&mut self, lo: i32, hi: i32) -> i32 {
        self.next_int(lo, hi)
    }
}
