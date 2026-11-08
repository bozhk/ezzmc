//! Анимационное состояние аватара игрока: поворот тела/головы, ходьба, взмах.
//! VANILLA: LivingEntity#tick (yBodyRot, tickHeadTurn), WalkAnimationState, SwingState,
//! LivingEntityRenderer#extractRenderState (интерполяция головы и тела по partial tick).

use ezzmc_render::humanoid::{Arm, HumanoidState};

/// VANILLA: Mth.wrapDegrees.
fn wrap_degrees(a: f32) -> f32 {
    let mut d = a % 360.0;
    if d >= 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0;
    }
    d
}

/// VANILLA: Mth.rotLerp.
fn rot_lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * wrap_degrees(b - a)
}

/// Длительность взмаха без эффектов спешки/усталости. VANILLA: LivingEntity#getCurrentSwingDuration.
const SWING_DURATION: i32 = 6;

#[derive(Clone, Debug, Default)]
pub struct AvatarAnim {
    inited: bool,
    body_rot: f32,
    body_rot_o: f32,
    yaw: f32,
    yaw_o: f32,
    pitch: f32,
    pitch_o: f32,
    walk_speed: f32,
    walk_speed_o: f32,
    walk_pos: f32,
    /// SwingState: идёт ли взмах, тики, анимация (старая/новая).
    swinging: bool,
    swing_ticks: i32,
    swing_old: f32,
    swing_now: f32,
    age: u32,
    /// Поза плавания (Pose.SWIMMING) и «в воде» — выставляются владельцем перед `tick`.
    pub swimming: bool,
    pub in_water: bool,
    swim_amount: f32,
    swim_amount_o: f32,
}

impl AvatarAnim {
    /// VANILLA: SwingState#startIfAble — нельзя перезапустить в первой половине взмаха.
    pub fn swing(&mut self) {
        if self.swinging && self.swing_ticks <= SWING_DURATION / 2 && self.swing_ticks > 0 {
            return;
        }
        self.swinging = true;
        self.swing_ticks = 0;
    }

    fn tick_swing(&mut self) {
        self.swing_old = self.swing_now;
        if self.swinging {
            self.swing_now = (self.swing_ticks as f32 / SWING_DURATION as f32).min(1.0);
            let over = self.swing_ticks > SWING_DURATION;
            self.swing_ticks += 1;
            if over {
                self.swinging = false;
                self.swing_now = 0.0;
            }
        } else {
            self.swing_now = 0.0;
        }
    }

    /// Один тик клиента после движения. `pos`/`prev_pos` — позиция в этом и прошлом тике.
    pub fn tick(&mut self, pos: [f64; 3], prev_pos: [f64; 3], yaw: f32, pitch: f32) {
        if !self.inited {
            self.inited = true;
            (self.body_rot, self.body_rot_o) = (yaw, yaw);
            (self.yaw, self.yaw_o) = (yaw, yaw);
            (self.pitch, self.pitch_o) = (pitch, pitch);
        }
        self.age = self.age.wrapping_add(1);
        // Entity: yRotO/xRotO ← прошлые значения.
        self.yaw_o = self.yaw;
        self.pitch_o = self.pitch;
        self.yaw = yaw;
        self.pitch = pitch;
        self.body_rot_o = self.body_rot;
        self.tick_swing();
        // VANILLA: LivingEntity#updateSwimAmount.
        self.swim_amount_o = self.swim_amount;
        self.swim_amount = if self.swimming {
            (self.swim_amount + 0.09).min(1.0)
        } else {
            (self.swim_amount - 0.09).max(0.0)
        };

        // LivingEntity#tick: направление ходьбы → целевой поворот тела.
        let (xd, zd) = (pos[0] - prev_pos[0], pos[2] - prev_pos[2]);
        let side_dist = (xd * xd + zd * zd) as f32;
        let mut target = self.body_rot;
        if side_dist > 0.0025 {
            let walk_dir = (zd.atan2(xd) as f32).to_degrees() - 90.0;
            let diff = (wrap_degrees(self.yaw) - walk_dir).abs();
            target = if 95.0 < diff && diff < 265.0 {
                walk_dir - 180.0
            } else {
                walk_dir
            };
        }
        if self.swing_now > 0.0 {
            target = self.yaw;
        }
        // tickHeadTurn: тело догоняет цель, голова не дальше 50° от тела.
        self.body_rot += wrap_degrees(target - self.body_rot) * 0.3;
        let head_diff = wrap_degrees(self.yaw - self.body_rot);
        if head_diff.abs() > 50.0 {
            self.body_rot += head_diff - head_diff.signum() * 50.0;
        }
        // Диапазоны: старые значения не дальше 180° от новых.
        while self.yaw - self.yaw_o < -180.0 {
            self.yaw_o -= 360.0;
        }
        while self.yaw - self.yaw_o >= 180.0 {
            self.yaw_o += 360.0;
        }
        while self.body_rot - self.body_rot_o < -180.0 {
            self.body_rot_o -= 360.0;
        }
        while self.body_rot - self.body_rot_o >= 180.0 {
            self.body_rot_o += 360.0;
        }

        // calculateEntityAnimation(false) + WalkAnimationState#update(target, 0.4, 1).
        let dist = ((xd * xd + zd * zd).sqrt()) as f32;
        let target_speed = (dist * 4.0).min(1.0);
        self.walk_speed_o = self.walk_speed;
        self.walk_speed += (target_speed - self.walk_speed) * 0.4;
        self.walk_pos += self.walk_speed;
    }

    /// Поворот тела (градусы) для кадра: VANILLA solveBodyRot без езды.
    pub fn body_rot(&self, pt: f32) -> f32 {
        rot_lerp(pt, self.body_rot_o, self.body_rot)
    }

    /// Состояние позы модели на кадр. Голова/прицел интерполируются по partial tick.
    pub fn humanoid_state(&self, pt: f32, crouching: bool) -> HumanoidState {
        let head_rot = rot_lerp(pt, self.yaw_o, self.yaw);
        let body = self.body_rot(pt);
        let swing = if self.swing_now >= self.swing_old {
            self.swing_old + (self.swing_now - self.swing_old) * pt
        } else {
            self.swing_old + (self.swing_now + 1.0 - self.swing_old) * pt
        };
        HumanoidState {
            y_rot: wrap_degrees(head_rot - body),
            x_rot: self.pitch_o + (self.pitch - self.pitch_o) * pt,
            walk_pos: self.walk_pos - self.walk_speed * (1.0 - pt),
            walk_speed: (self.walk_speed_o + (self.walk_speed - self.walk_speed_o) * pt).min(1.0),
            age_in_ticks: self.age as f32 + pt,
            crouching,
            swing: if self.swinging || self.swing_now > 0.0 {
                swing
            } else {
                0.0
            },
            swing_arm: Arm::Right,
            swim_amount: self.swim_amount_o + (self.swim_amount - self.swim_amount_o) * pt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standing_still_has_no_walk_motion() {
        let mut a = AvatarAnim::default();
        for _ in 0..10 {
            a.tick([0.0; 3], [0.0; 3], 30.0, 0.0);
        }
        let s = a.humanoid_state(0.5, false);
        assert!(s.walk_speed < 1e-6);
        assert_eq!(s.swing, 0.0);
    }

    #[test]
    fn walking_forward_accumulates_walk_position() {
        let mut a = AvatarAnim::default();
        let mut z = 0.0;
        for _ in 0..10 {
            a.tick([0.0, 0.0, z + 0.2], [0.0, 0.0, z], 0.0, 0.0);
            z += 0.2;
        }
        let s = a.humanoid_state(1.0, false);
        assert!(s.walk_speed > 0.5 && s.walk_pos > 3.0);
    }

    #[test]
    fn swing_runs_then_stops_and_blocks_restart_early() {
        let mut a = AvatarAnim::default();
        a.tick([0.0; 3], [0.0; 3], 0.0, 0.0);
        a.swing();
        let mut seen = 0.0f32;
        for _ in 0..4 {
            a.tick([0.0; 3], [0.0; 3], 0.0, 0.0);
            seen = seen.max(a.humanoid_state(1.0, false).swing);
        }
        assert!(seen > 0.3);
        for _ in 0..10 {
            a.tick([0.0; 3], [0.0; 3], 0.0, 0.0);
        }
        assert_eq!(a.humanoid_state(0.5, false).swing, 0.0);
    }

    #[test]
    fn head_turn_is_limited_to_50_degrees_from_body() {
        let mut a = AvatarAnim::default();
        a.tick([0.0; 3], [0.0; 3], 0.0, 0.0);
        a.tick([0.0; 3], [0.0; 3], 120.0, 0.0);
        let s = a.humanoid_state(1.0, false);
        assert!(s.y_rot.abs() <= 50.01, "{}", s.y_rot);
    }
}
