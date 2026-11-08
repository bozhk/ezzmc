//! Keyframe-анимации моделей. VANILLA: AnimationDefinition / AnimationChannel / Keyframe /
//! KeyframeAnimation / KeyframeAnimations (26.3). Анимация прибавляет смещения к позе частей
//! (`offsetPos/offsetRotation/offsetScale`), поэтому перед применением нужен `Model::reset_pose`.

use crate::entitymodel::Model;
use glam::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Position,
    Rotation,
    Scale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    Linear,
    CatmullRom,
}

/// VANILLA: Keyframe (pre/post — для разрывных ключей).
#[derive(Clone, Copy, Debug)]
pub struct Keyframe {
    pub timestamp: f32,
    pub pre: Vec3,
    pub post: Vec3,
    pub interpolation: Interpolation,
}

impl Keyframe {
    pub fn new(timestamp: f32, target: Vec3, interpolation: Interpolation) -> Self {
        Keyframe {
            timestamp,
            pre: target,
            post: target,
            interpolation,
        }
    }
}

/// VANILLA: KeyframeAnimations#posVec / degreeVec / scaleVec.
pub fn pos_vec(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, -y, z)
}
pub fn degree_vec(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z) * (std::f32::consts::PI / 180.0)
}
pub fn scale_vec(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new((x - 1.0) as f32, (y - 1.0) as f32, (z - 1.0) as f32)
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub bone: &'static str,
    pub target: Target,
    pub keyframes: Vec<Keyframe>,
}

/// VANILLA: AnimationDefinition.
#[derive(Clone, Debug)]
pub struct AnimationDefinition {
    pub length: f32,
    pub looping: bool,
    pub channels: Vec<Channel>,
}

/// VANILLA: Mth#catmullrom.
fn catmullrom(a: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    0.5 * (2.0 * p1
        + (p2 - p0) * a
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * a * a
        + (3.0 * p1 - p0 - 3.0 * p2 + p3) * a * a * a)
}

impl AnimationDefinition {
    /// VANILLA: KeyframeAnimation#applyWalk.
    pub fn apply_walk(
        &self,
        model: &mut Model,
        pos: f32,
        speed: f32,
        speed_factor: f32,
        scale_factor: f32,
    ) {
        let time = (pos * 50.0 * speed_factor) as i64;
        self.apply(model, time, (speed * scale_factor).min(1.0));
    }

    /// VANILLA: KeyframeAnimation#apply(long, float). Кости, которых нет в модели, пропускаются
    /// (в ванили — исключение при bake).
    pub fn apply(&self, model: &mut Model, millis: i64, target_scale: f32) {
        let mut secs = millis as f32 / 1000.0;
        if self.looping {
            secs %= self.length;
        }
        for ch in &self.channels {
            let (Some(idx), false) = (model.index(ch.bone), ch.keyframes.is_empty()) else {
                continue;
            };
            let v = sample(&ch.keyframes, secs, target_scale);
            let part = &mut model.parts[idx];
            match ch.target {
                Target::Position => {
                    part.pose.x += v.x;
                    part.pose.y += v.y;
                    part.pose.z += v.z;
                }
                Target::Rotation => {
                    part.pose.x_rot += v.x;
                    part.pose.y_rot += v.y;
                    part.pose.z_rot += v.z;
                }
                Target::Scale => part.scale += v,
            }
        }
    }
}

/// VANILLA: KeyframeAnimation.Entry#apply (бинарный поиск первого ключа с secs <= timestamp).
fn sample(kf: &[Keyframe], secs: f32, scale: f32) -> Vec3 {
    let first = kf.partition_point(|k| secs > k.timestamp);
    let prev = first.saturating_sub(1);
    let next = (prev + 1).min(kf.len() - 1);
    let (a, b) = (&kf[prev], &kf[next]);
    let alpha = if next != prev {
        ((secs - a.timestamp) / (b.timestamp - a.timestamp)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    match b.interpolation {
        Interpolation::Linear => a.post.lerp(b.pre, alpha) * scale,
        Interpolation::CatmullRom => {
            let p0 = kf[prev.saturating_sub(1)].post;
            let p3 = kf[(next + 1).min(kf.len() - 1)].post;
            let (p1, p2) = (a.post, b.post);
            Vec3::new(
                catmullrom(alpha, p0.x, p1.x, p2.x, p3.x),
                catmullrom(alpha, p0.y, p1.y, p2.y, p3.y),
                catmullrom(alpha, p0.z, p1.z, p2.z, p3.z),
            ) * scale
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entitymodel::Pose;

    fn def(looping: bool) -> AnimationDefinition {
        AnimationDefinition {
            length: 1.0,
            looping,
            channels: vec![Channel {
                bone: "a",
                target: Target::Rotation,
                keyframes: vec![
                    Keyframe::new(0.0, Vec3::ZERO, Interpolation::Linear),
                    Keyframe::new(1.0, Vec3::new(2.0, 0.0, 0.0), Interpolation::Linear),
                ],
            }],
        }
    }

    fn model() -> Model {
        let mut m = Model::new([16.0; 2]);
        m.add("a", None, Pose::default(), &[]);
        m
    }

    #[test]
    fn linear_midpoint_and_clamp() {
        let mut m = model();
        def(false).apply(&mut m, 500, 1.0);
        assert!((m.parts[0].pose.x_rot - 1.0).abs() < 1e-6);
        m.reset_pose();
        def(false).apply(&mut m, 5000, 1.0);
        assert!((m.parts[0].pose.x_rot - 2.0).abs() < 1e-6);
    }

    #[test]
    fn looping_wraps_and_scale_applies() {
        let mut m = model();
        def(true).apply(&mut m, 1500, 0.5);
        assert!((m.parts[0].pose.x_rot - 0.5).abs() < 1e-5);
    }

    #[test]
    fn catmullrom_hits_keyframe_values_and_scale_channel() {
        let kf = vec![
            Keyframe::new(0.0, Vec3::ZERO, Interpolation::CatmullRom),
            Keyframe::new(1.0, Vec3::splat(1.0), Interpolation::CatmullRom),
            Keyframe::new(2.0, Vec3::ZERO, Interpolation::CatmullRom),
        ];
        assert!((sample(&kf, 1.0, 1.0).x - 1.0).abs() < 1e-6);
        let mut m = model();
        AnimationDefinition {
            length: 1.0,
            looping: false,
            channels: vec![Channel {
                bone: "a",
                target: Target::Scale,
                keyframes: vec![Keyframe::new(
                    0.0,
                    scale_vec(2.0, 1.0, 1.0),
                    Interpolation::Linear,
                )],
            }],
        }
        .apply(&mut m, 0, 1.0);
        assert_eq!(m.parts[0].scale, Vec3::new(2.0, 1.0, 1.0));
    }
}
