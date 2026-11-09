//! Ambient-звуки окружения. VANILLA: BiomeAmbientSoundsHandler#tick (26.3):
//! петля биома с кроссфейдом 40 тиков, additions (шанс за тик), mood (пещеры).
use crate::audio::{self, Cat};
use ezzmc_world::audio_attr::{AmbientSounds, AudioAttrs};
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::level::Level;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

struct Loop {
    event: String,
    gain: Arc<AtomicU32>,
    fade: i32,
    dir: i32,
}

pub struct Ambient {
    loops: Vec<Loop>,
    current: Option<String>,
    moodiness: f32,
    rng: u64,
}

impl Ambient {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(7, |d| d.as_nanos() as u64)
            | 1;
        Ambient {
            loops: Vec::new(),
            current: None,
            moodiness: 0.0,
            rng: seed,
        }
    }

    fn next(&mut self) -> u64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        self.rng
    }
    fn double(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn int(&mut self, bound: i32) -> i32 {
        (self.next() % bound.max(1) as u64) as i32
    }

    /// Атрибуты в точке: биом переопределяет dimension (EnvironmentAttributeSystem).
    pub fn attrs_at<'a>(
        level: &'a Level,
        biomes: &'a [Option<BiomeColors>],
        p: [f64; 3],
    ) -> &'a AudioAttrs {
        let (x, y, z) = (
            p[0].floor() as i32,
            p[1].floor() as i32,
            p[2].floor() as i32,
        );
        level
            .chunks
            .get(x >> 4, z >> 4)
            .and_then(|c| {
                biomes.get(c.chunk.biome((x & 15) as usize, y, (z & 15) as usize) as usize)
            })
            .and_then(Option::as_ref)
            .map(|b| &b.audio)
            .unwrap_or(&level.audio)
    }

    /// Музыка в точке с учётом override биомом (BACKGROUND_MUSIC).
    pub fn ambient_of<'a>(
        level: &'a Level,
        biomes: &'a [Option<BiomeColors>],
        p: [f64; 3],
    ) -> Option<&'a AmbientSounds> {
        let b = Self::attrs_at(level, biomes, p);
        // Биом без своего значения → значение dimension.
        b.ambient.as_ref().or(level.audio.ambient.as_ref())
    }

    /// Один клиентский тик. `eye` — глаза игрока.
    pub fn tick(&mut self, level: &Level, biomes: &[Option<BiomeColors>], eye: [f64; 3]) {
        // Затухающие петли.
        self.loops.retain_mut(|l| {
            if l.fade < 0 {
                audio::stop(Some(&l.event), Some(Cat::Ambient));
                return false;
            }
            l.fade += l.dir;
            l.gain.store(
                ((l.fade as f32) / 40.0).clamp(0.0, 1.0).to_bits(),
                Ordering::Relaxed,
            );
            true
        });
        let snd = Self::ambient_of(level, biomes, eye).cloned();
        let cur = snd.as_ref().and_then(|s| s.loop_sound.clone());
        if cur != self.current {
            for l in &mut self.loops {
                l.fade = l.fade.min(40);
                l.dir = -1;
            }
            if let Some(ev) = &cur {
                if let Some(l) = self.loops.iter_mut().find(|l| l.event == *ev) {
                    l.fade = l.fade.max(0);
                    l.dir = 1;
                } else {
                    let gain = Arc::new(AtomicU32::new(0f32.to_bits()));
                    audio::play_loop(ev, Cat::Ambient, gain.clone());
                    self.loops.push(Loop {
                        event: ev.clone(),
                        gain,
                        fade: 0,
                        dir: 1,
                    });
                }
            }
            self.current = cur;
        }
        let Some(snd) = snd else { return };
        for a in &snd.additions {
            if self.double() < a.tick_chance {
                // VANILLA: SimpleSoundInstance.forAmbientAddition (UI-подобный, Ambient, 1.0/1.0).
                audio::play_ui(&a.sound, Cat::Ambient, 1.0, 1.0);
            }
        }
        if let Some(m) = &snd.mood {
            let span = m.search_extent * 2 + 1;
            let b = [
                (eye[0] + f64::from(self.int(span) - m.search_extent)).floor() as i32,
                (eye[1] + f64::from(self.int(span) - m.search_extent)).floor() as i32,
                (eye[2] + f64::from(self.int(span) - m.search_extent)).floor() as i32,
            ];
            let (sky, blk) = level
                .chunks
                .get(b[0] >> 4, b[2] >> 4)
                .map(|c| {
                    let (lx, lz) = ((b[0] & 15) as usize, (b[2] & 15) as usize);
                    (
                        c.sky_light(lx, b[1], lz).unwrap_or(0),
                        c.block_light(lx, b[1], lz).unwrap_or(0),
                    )
                })
                .unwrap_or((0, 0));
            if sky > 0 {
                self.moodiness -= f32::from(sky) / 15.0 * 0.001;
            } else {
                self.moodiness -= (f32::from(blk) - 1.0) / m.tick_delay as f32;
            }
            if self.moodiness >= 1.0 {
                let d = [
                    f64::from(b[0]) + 0.5 - eye[0],
                    f64::from(b[1]) + 0.5 - eye[1],
                    f64::from(b[2]) + 0.5 - eye[2],
                ];
                let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let at = dist + m.offset;
                // VANILLA: SimpleSoundInstance.forAmbientMood: volume 1, pitch 1, дальность по позиции.
                audio::play_at(
                    &m.sound,
                    Cat::Ambient,
                    eye[0] + d[0] / dist * at,
                    eye[1] + d[1] / dist * at,
                    eye[2] + d[2] / dist * at,
                    1.0,
                    1.0,
                );
                self.moodiness = 0.0;
            } else {
                self.moodiness = self.moodiness.max(0.0);
            }
        }
    }
}
