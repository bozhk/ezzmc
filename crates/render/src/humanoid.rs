//! Модель игрока и гуманоидная анимация. VANILLA: HumanoidModel#createMesh / PlayerModel#createMesh
//! и #setupAnim / #setupAttackAnimation (26.3), AnimationUtils#bobModelPart.
//! База для будущих гуманоидных мобов (зомби, скелет, житель): `humanoid_mesh` + `setup_anim`.

use crate::entitymodel::{BoxDef, Model, Pose};
use std::f32::consts::PI;

/// Какой рукой бьём/машем.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Arm {
    Left,
    #[default]
    Right,
}

/// Что нужно модели для позы. VANILLA: HumanoidRenderState (нужное подмножество).
#[derive(Clone, Copy, Debug, Default)]
pub struct HumanoidState {
    /// Поворот головы относительно тела (градусы) и наклон (pitch).
    pub y_rot: f32,
    pub x_rot: f32,
    pub walk_pos: f32,
    pub walk_speed: f32,
    pub age_in_ticks: f32,
    pub crouching: bool,
    /// Прогресс взмаха 0..1 (`swingAnimation`); 0 — не машет.
    pub swing: f32,
    pub swing_arm: Arm,
    /// Прогресс позы плавания 0..1 (`swimAmount`).
    pub swim_amount: f32,
    /// Предмет в правой руке (ArmPose.ITEM).
    pub item_right: bool,
    /// Предмет в левой руке.
    pub item_left: bool,
}

/// Индексы основных частей гуманоида.
#[derive(Clone, Copy, Debug)]
pub struct HumanoidParts {
    pub head: usize,
    pub body: usize,
    pub right_arm: usize,
    pub left_arm: usize,
    pub right_leg: usize,
    pub left_leg: usize,
}

/// VANILLA: HumanoidModel#createMesh (64×64) — без внешних слоёв.
pub fn humanoid_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 0.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(0.5)],
    );
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 0.0, 0.0),
        &[BoxDef::new(
            [16.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    let right_arm = m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [40.0, 16.0],
            [-3.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let left_arm = m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new([40.0, 16.0], [-1.0, -2.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    let right_leg = m.add(
        "right_leg",
        None,
        Pose::offset(-1.9, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 16.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let left_leg = m.add(
        "left_leg",
        None,
        Pose::offset(1.9, 12.0, 0.0),
        &[BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    (
        m,
        HumanoidParts {
            head,
            body,
            right_arm,
            left_arm,
            right_leg,
            left_leg,
        },
    )
}

/// VANILLA: PlayerModel#createMesh(slim) — руки/ноги заменяются, добавляются слои одежды.
pub fn player_mesh(slim: bool) -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(0.5)],
    );
    let body = m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new(
            [16.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    m.add(
        "jacket",
        Some(body),
        Pose::default(),
        &[BoxDef::new([16.0, 32.0], [-4.0, 0.0, -2.0], [8.0, 12.0, 4.0]).grow(0.25)],
    );
    let aw = if slim { 3.0 } else { 4.0 };
    let right_arm = m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [40.0, 16.0],
            [if slim { -2.0 } else { -3.0 }, -2.0, -2.0],
            [aw, 12.0, 4.0],
        )],
    );
    m.add(
        "right_sleeve",
        Some(right_arm),
        Pose::default(),
        &[BoxDef::new(
            [40.0, 32.0],
            [if slim { -2.0 } else { -3.0 }, -2.0, -2.0],
            [aw, 12.0, 4.0],
        )
        .grow(0.25)],
    );
    let left_arm = m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new(
            [32.0, 48.0],
            [-1.0, -2.0, -2.0],
            [aw, 12.0, 4.0],
        )],
    );
    m.add(
        "left_sleeve",
        Some(left_arm),
        Pose::default(),
        &[BoxDef::new([48.0, 48.0], [-1.0, -2.0, -2.0], [aw, 12.0, 4.0]).grow(0.25)],
    );
    let right_leg = m.add(
        "right_leg",
        None,
        Pose::offset(-1.9, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 16.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "right_pants",
        Some(right_leg),
        Pose::default(),
        &[BoxDef::new([0.0, 32.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).grow(0.25)],
    );
    let left_leg = m.add(
        "left_leg",
        None,
        Pose::offset(1.9, 12.0, 0.0),
        &[BoxDef::new(
            [16.0, 48.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_pants",
        Some(left_leg),
        Pose::default(),
        &[BoxDef::new([0.0, 48.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).grow(0.25)],
    );
    (
        m,
        HumanoidParts {
            head,
            body,
            right_arm,
            left_arm,
            right_leg,
            left_leg,
        },
    )
}

/// VANILLA: Ease#outQuart.
fn out_quart(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(4)
}

/// VANILLA: HumanoidModel#setupAnim для пустых рук (поза EMPTY), без плавания/полёта/верховой езды.
pub fn setup_anim(m: &mut Model, p: &HumanoidParts, s: &HumanoidState) {
    m.reset_pose();
    let deg = PI / 180.0;
    m.parts[p.head].pose.x_rot = s.x_rot * deg;
    m.parts[p.head].pose.y_rot = s.y_rot * deg;
    let (pos, spd) = (s.walk_pos, s.walk_speed);
    // speedValue = 1 для игрока.
    m.parts[p.right_arm].pose.x_rot = (pos * 0.6662 + PI).cos() * 2.0 * spd * 0.5;
    m.parts[p.left_arm].pose.x_rot = (pos * 0.6662).cos() * 2.0 * spd * 0.5;
    // VANILLA: HumanoidModel#poseRightArm (ITEM): xRot = xRot·0.5 − π/10.
    if s.item_left {
        m.parts[p.left_arm].pose.x_rot = m.parts[p.left_arm].pose.x_rot * 0.5 - PI / 10.0;
    }
    if s.item_right {
        m.parts[p.right_arm].pose.x_rot = m.parts[p.right_arm].pose.x_rot * 0.5 - PI / 10.0;
    }
    m.parts[p.right_leg].pose.x_rot = (pos * 0.6662).cos() * 1.4 * spd;
    m.parts[p.left_leg].pose.x_rot = (pos * 0.6662 + PI).cos() * 1.4 * spd;
    m.parts[p.right_leg].pose.y_rot = 0.005;
    m.parts[p.left_leg].pose.y_rot = -0.005;
    m.parts[p.right_leg].pose.z_rot = 0.005;
    m.parts[p.left_leg].pose.z_rot = -0.005;
    // poseRightArm/poseLeftArm EMPTY: yRot = 0 (уже).

    // setupAttackAnimation (WHACK).
    if s.swing > 0.0 {
        let a = s.swing;
        let mut by = (a.sqrt() * (PI * 2.0)).sin() * 0.2;
        if s.swing_arm == Arm::Left {
            by *= -1.0;
        }
        m.parts[p.body].pose.y_rot = by;
        m.parts[p.right_arm].pose.z = by.sin() * 5.0;
        m.parts[p.right_arm].pose.x = -by.cos() * 5.0;
        m.parts[p.left_arm].pose.z = -by.sin() * 5.0;
        m.parts[p.left_arm].pose.x = by.cos() * 5.0;
        m.parts[p.right_arm].pose.y_rot += by;
        m.parts[p.left_arm].pose.y_rot += by;
        m.parts[p.left_arm].pose.x_rot += by;
        let aa = (out_quart(a) * PI).sin();
        let head_x = m.parts[p.head].pose.x_rot;
        let bb = (a * PI).sin() * -(head_x - 0.7) * 0.75;
        let arm = if s.swing_arm == Arm::Left {
            p.left_arm
        } else {
            p.right_arm
        };
        let pose = &mut m.parts[arm].pose;
        pose.x_rot -= aa * 1.2 + bb;
        pose.y_rot += by * 2.0;
        pose.z_rot += (a * PI).sin() * -0.4;
    }
    if s.crouching {
        m.parts[p.body].pose.x_rot = 0.5;
        m.parts[p.right_arm].pose.x_rot += 0.4;
        m.parts[p.left_arm].pose.x_rot += 0.4;
        m.parts[p.right_leg].pose.z += 4.0;
        m.parts[p.left_leg].pose.z += 4.0;
        m.parts[p.head].pose.y += 4.2;
        m.parts[p.body].pose.y += 3.2;
        m.parts[p.left_arm].pose.y += 3.2;
        m.parts[p.right_arm].pose.y += 3.2;
    }
    // AnimationUtils#bobModelPart: лёгкое покачивание рук в покое.
    let bob = |pose: &mut Pose, sign: f32| {
        pose.z_rot += sign * ((s.age_in_ticks * 0.09).cos() * 0.05 + 0.05);
        pose.x_rot += sign * (s.age_in_ticks * 0.067).sin() * 0.05;
    };
    bob(&mut m.parts[p.right_arm].pose, 1.0);
    bob(&mut m.parts[p.left_arm].pose, -1.0);
    if s.swim_amount > 0.0 {
        setup_swim_animation(m, p, s);
    }
}

/// VANILLA: Mth.rotLerpRad.
fn rot_lerp_rad(t: f32, a: f32, b: f32) -> f32 {
    let mut d = (b - a) % (2.0 * PI);
    if d >= PI {
        d -= 2.0 * PI;
    }
    if d < -PI {
        d += 2.0 * PI;
    }
    a + t * d
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

/// VANILLA: HumanoidModel#setupSwimAnimation (без предметов в руках).
fn setup_swim_animation(m: &mut Model, p: &HumanoidParts, s: &HumanoidState) {
    let sa = s.swim_amount;
    let hx = m.parts[p.head].pose.x_rot;
    m.parts[p.head].pose.x_rot = rot_lerp_rad(sa, hx, -PI / 4.0);
    let swim_pos = s.walk_pos % 26.0;
    // Рука, которой идёт взмах, в плавании не участвует.
    let swinging = s.swing > 0.0;
    let ra = if swinging && s.swing_arm == Arm::Right {
        0.0
    } else {
        sa
    };
    let la = if swinging && s.swing_arm == Arm::Left {
        0.0
    } else {
        sa
    };
    let q = |f: f32| -65.0 * f + f * f;
    let (l_x, r_x, l_z, r_z);
    if swim_pos < 14.0 {
        let k = 1.870_796_4 * q(swim_pos) / q(14.0);
        (l_x, r_x) = (0.0, 0.0);
        (l_z, r_z) = (PI + k, PI - k);
    } else if swim_pos < 22.0 {
        let i = (swim_pos - 14.0) / 8.0;
        (l_x, r_x) = (PI / 2.0 * i, PI / 2.0 * i);
        (l_z, r_z) = (5.012_389 - 1.870_796_4 * i, 1.270_796_3 + 1.870_796_4 * i);
    } else {
        let i = (swim_pos - 22.0) / 4.0;
        (l_x, r_x) = (PI / 2.0 - PI / 2.0 * i, PI / 2.0 - PI / 2.0 * i);
        (l_z, r_z) = (PI, PI);
    }
    let (lp, rp) = (p.left_arm, p.right_arm);
    let (a, b) = (m.parts[lp].pose, m.parts[rp].pose);
    m.parts[lp].pose.x_rot = rot_lerp_rad(la, a.x_rot, l_x);
    m.parts[rp].pose.x_rot = lerp(ra, b.x_rot, r_x);
    m.parts[lp].pose.y_rot = rot_lerp_rad(la, a.y_rot, PI);
    m.parts[rp].pose.y_rot = lerp(ra, b.y_rot, PI);
    m.parts[lp].pose.z_rot = rot_lerp_rad(la, a.z_rot, l_z);
    m.parts[rp].pose.z_rot = lerp(ra, b.z_rot, r_z);
    let pos = s.walk_pos;
    let (ll, rl) = (p.left_leg, p.right_leg);
    let (lx, rx) = (m.parts[ll].pose.x_rot, m.parts[rl].pose.x_rot);
    m.parts[ll].pose.x_rot = lerp(sa, lx, 0.3 * (pos * 0.333_333_34 + PI).cos());
    m.parts[rl].pose.x_rot = lerp(sa, rx, 0.3 * (pos * 0.333_333_34).cos());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_has_all_layers() {
        let (m, _) = player_mesh(false);
        for n in [
            "hat",
            "jacket",
            "right_sleeve",
            "left_sleeve",
            "right_pants",
            "left_pants",
        ] {
            assert!(m.index(n).is_some(), "{n}");
        }
    }

    #[test]
    fn walking_swings_limbs_oppositely() {
        let (mut m, p) = player_mesh(false);
        let s = HumanoidState {
            walk_pos: 1.0,
            walk_speed: 1.0,
            ..Default::default()
        };
        setup_anim(&mut m, &p, &s);
        let (ra, la) = (
            m.parts[p.right_arm].pose.x_rot,
            m.parts[p.left_arm].pose.x_rot,
        );
        let (rl, ll) = (
            m.parts[p.right_leg].pose.x_rot,
            m.parts[p.left_leg].pose.x_rot,
        );
        assert!((ra + la).abs() < 0.01 && (rl + ll).abs() < 0.01);
        assert!(ra * rl < 0.0, "рука и нога одной стороны — в противофазе");
    }

    #[test]
    fn crouch_shifts_body_down_and_tilts() {
        let (mut m, p) = player_mesh(false);
        setup_anim(
            &mut m,
            &p,
            &HumanoidState {
                crouching: true,
                ..Default::default()
            },
        );
        assert_eq!(m.parts[p.body].pose.x_rot, 0.5);
        assert!((m.parts[p.head].pose.y - 4.2).abs() < 1e-6);
    }

    #[test]
    fn swing_moves_attack_arm() {
        let (mut m, p) = player_mesh(false);
        setup_anim(
            &mut m,
            &p,
            &HumanoidState {
                swing: 0.5,
                ..Default::default()
            },
        );
        assert!(m.parts[p.right_arm].pose.x_rot < -0.5);
        assert!(m.parts[p.body].pose.y_rot.abs() > 0.01);
    }
}
