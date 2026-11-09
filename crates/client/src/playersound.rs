//! Звуки локального игрока (сервер их самому игроку не шлёт). Источник — PLAYERS.
//! VANILLA: Entity#applyMovementEmissionAndPlaySound / playStepSound, Player#playStepSound,
//! Entity#doWaterSplashEffect, LivingEntity#causeFallDamage.
use crate::audio::{self, Cat};
use crate::blocksound;
use crate::physics::Player;
use ezzmc_world::level::Level;

#[derive(Default)]
pub struct PlayerSound {
    move_dist: f32,
    next_step: f32,
    was_in_water: bool,
    was_under_water: bool,
    /// UnderLiquidAmbientSoundInstance: fade 0..40 и громкость петли; None — петля не играет.
    loop_gain: Option<std::sync::Arc<std::sync::atomic::AtomicU32>>,
    loop_fade: i32,
    /// Вход в воду на этом тике: (позиция, скорость) для частиц doWaterSplashEffect.
    pub splash: Option<([f64; 3], [f64; 3])>,
    prev_pos: Option<[f64; 3]>,
    fall_before: f64,
    rng: u64,
}

impl PlayerSound {
    fn rand(&mut self) -> f32 {
        // xorshift: источник случайности только для питча/громкости (ваниль: RandomSource клиента).
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Вызывать перед `Player::tick` (запоминает падение до обнуления).
    pub fn before_tick(&mut self, p: &Player) {
        if self.rng == 0 {
            self.rng = 0x9E37_79B9_7F4A_7C15;
        }
        self.fall_before = p.fall_distance;
        self.prev_pos.get_or_insert(p.pos);
    }

    /// Вызывать после `Player::tick`. `was_on_ground` — флаг до тика.
    pub fn after_tick(&mut self, p: &Player, l: &Level, was_on_ground: bool) {
        let prev = self.prev_pos.replace(p.pos).unwrap_or(p.pos);
        let d = [p.pos[0] - prev[0], p.pos[1] - prev[1], p.pos[2] - prev[2]];
        let name_at = |x: i32, y: i32, z: i32| l.chunks.block(x, y, z);
        let (bx, bz) = (p.pos[0].floor() as i32, p.pos[2].floor() as i32);
        let feet = [p.pos[0], p.pos[1], p.pos[2]];

        // Приземление: LivingEntity#causeFallDamage → small_fall/big_fall + звук падения блока.
        // VANILLA: Player#causeFallDamage — при abilities.mayfly (креатив/наблюдатель) сразу false, без звуков.
        if p.on_ground
            && !was_on_ground
            && self.fall_before > 0.0
            && !p.in_water
            && !p.flying
            && !p.mayfly
        {
            let dmg = (self.fall_before - 3.0).ceil() as i32;
            if dmg > 0 {
                let ev = if dmg > 4 {
                    "entity.player.big_fall"
                } else {
                    "entity.player.small_fall"
                };
                audio::play_at(ev, Cat::Players, feet[0], feet[1], feet[2], 1.0, 1.0);
                let st = name_at(bx, (p.pos[1] - 0.2).floor() as i32, bz);
                if !st.is_air() {
                    blocksound::fall(st.name(), dmg > 4, feet, Cat::Players);
                }
            }
        }

        // Всплеск при входе в воду: Entity#doWaterSplashEffect.
        if p.in_water && !self.was_in_water {
            let v = p.vel;
            let vol = ((v[0] * v[0] * 0.2 + v[1] * v[1] + v[2] * v[2] * 0.2).sqrt() * 0.2).min(1.0)
                as f32;
            let ev = if vol < 0.25 {
                "entity.player.splash"
            } else {
                "entity.player.splash.high_speed"
            };
            let pitch = 1.0 + (self.rand() - self.rand()) * 0.4;
            audio::play_at(ev, Cat::Players, feet[0], feet[1], feet[2], vol, pitch);
            self.splash = Some((feet, v));
        }
        self.was_in_water = p.in_water;
        // VANILLA: LocalPlayer#updateIsUnderwater — ambient.underwater.enter/exit (1.0, 1.0).
        if p.under_water != self.was_under_water {
            let ev = if p.under_water {
                "ambient.underwater.enter"
            } else {
                "ambient.underwater.exit"
            };
            audio::play_at(ev, Cat::Ambient, feet[0], feet[1], feet[2], 1.0, 1.0);
            self.was_under_water = p.under_water;
            if p.under_water && self.loop_gain.is_none() {
                let g = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0f32.to_bits()));
                audio::play_loop("ambient.underwater.loop", Cat::Ambient, g.clone());
                self.loop_gain = Some(g);
                self.loop_fade = 0;
            }
        }
        // VANILLA: UnderLiquidAmbientSoundInstance#tick — fade +1 под водой, −2 вне; <0 → stop.
        if let Some(g) = &self.loop_gain {
            self.loop_fade += if p.under_water { 1 } else { -2 };
            self.loop_fade = self.loop_fade.min(40);
            if self.loop_fade < 0 {
                audio::stop(Some("ambient.underwater.loop"), Some(Cat::Ambient));
                self.loop_gain = None;
            } else {
                let v = (self.loop_fade as f32 / 40.0).clamp(0.0, 1.0);
                g.store(v.to_bits(), std::sync::atomic::Ordering::Relaxed);
            }
        }

        // Шаги: moveDist += горизонтальный путь · 0.6.
        if p.flying {
            return;
        }
        self.move_dist += (d[0].hypot(d[2]) * 0.6) as f32;
        if self.move_dist > self.next_step {
            let under = name_at(bx, (p.pos[1] - 0.2).floor() as i32, bz);
            if !under.is_air() {
                // VANILLA: emission NONE при присяде на земле → без шага (но moveDist растёт).
                let silent = p.crouching && p.on_ground;
                if p.on_ground && !p.in_water && !silent {
                    self.next_step = self.move_dist.floor() + 1.0;
                    let above = name_at(bx, (p.pos[1] - 0.2).floor() as i32 + 1, bz);
                    let top = !above.is_air() && matches!(above.name(), "snow" | "powder_snow");
                    let src = if top { above } else { under };
                    blocksound::step(src.name(), feet, Cat::Players);
                } else if p.in_water {
                    self.next_step = self.move_dist.floor() + 1.0;
                    self.swim(p, feet);
                    // Приглушённый шаг: v·0.05, p·0.8.
                    let st = blocksound::of(under.name());
                    if !st.step.is_empty() {
                        audio::play_at(
                            &st.step,
                            Cat::Players,
                            feet[0],
                            feet[1],
                            feet[2],
                            st.volume * 0.05,
                            st.pitch * 0.8,
                        );
                    }
                } else {
                    self.next_step = self.move_dist.floor() + 1.0;
                }
            }
        }
    }

    fn swim(&mut self, p: &Player, feet: [f64; 3]) {
        let v = p.vel;
        let vol =
            ((v[0] * v[0] * 0.2 + v[1] * v[1] + v[2] * v[2] * 0.2).sqrt() * 0.35).min(1.0) as f32;
        let pitch = 1.0 + (self.rand() - self.rand()) * 0.4;
        audio::play_at(
            "entity.player.swim",
            Cat::Players,
            feet[0],
            feet[1],
            feet[2],
            vol,
            pitch,
        );
    }
}
