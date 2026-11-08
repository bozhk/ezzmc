//! Отсечение по пирамиде видимости (TODO-143).
//! VANILLA: Frustum + JOML FrustumIntersection#intersectAab (1.21.11) — AABB отбрасывается, если целиком за любой плоскостью.
use glam::{DVec3, Mat4, Vec4};

#[derive(Clone, Debug)]
pub struct Frustum {
    /// left, right, bottom, top, near, far; `(nx, ny, nz, w)`.
    planes: [Vec4; 6],
    cam: DVec3,
    view_vector: glam::Vec3,
}

impl Frustum {
    /// `projection * modelview`; `zero_to_one` — диапазон глубины клипа 0..1 (wgpu/Vulkan) вместо −1..1.
    // VANILLA: Frustum#calculateFrustum — `projection.mul(modelView)`; FrustumIntersection#set(matrix)
    pub fn new(projection: Mat4, model_view: Mat4, zero_to_one: bool) -> Frustum {
        let m = projection * model_view;
        let r = [m.row(0), m.row(1), m.row(2), m.row(3)];
        let near = if zero_to_one { r[2] } else { r[3] + r[2] };
        // viewVector = matrix.transformTranspose((0,0,1,0)) = row 2 (xyz)
        let view_vector = r[2].truncate();
        Frustum {
            planes: [
                r[3] + r[0],
                r[3] - r[0],
                r[3] + r[1],
                r[3] - r[1],
                near,
                r[3] - r[2],
            ],
            cam: DVec3::ZERO,
            view_vector,
        }
    }

    /// Плоскости `(nx, ny, nz, w)` в системе координат относительно камеры: left, right, bottom, top, near, far.
    pub fn planes(&self) -> [Vec4; 6] {
        self.planes
    }

    /// Позиция камеры (после `prepare`/`offset`).
    pub fn camera(&self) -> DVec3 {
        self.cam
    }

    pub fn prepare(&mut self, x: f64, y: f64, z: f64) {
        self.cam = DVec3::new(x, y, z);
    }

    /// VANILLA: Frustum#offset
    pub fn offset(&mut self, d: f32) -> &mut Self {
        self.cam += DVec3::from(self.view_vector) * f64::from(d);
        self
    }

    /// VANILLA: Frustum#isVisible(AABB)
    pub fn is_visible(&self, min: [f64; 3], max: [f64; 3]) -> bool {
        let lo = (DVec3::from(min) - self.cam).as_vec3();
        let hi = (DVec3::from(max) - self.cam).as_vec3();
        for p in &self.planes {
            let x = if p.x < 0.0 { lo.x } else { hi.x };
            let y = if p.y < 0.0 { lo.y } else { hi.y };
            let z = if p.z < 0.0 { lo.z } else { hi.z };
            if p.x * x + p.y * y + p.z * z + p.w < 0.0 {
                return false;
            }
        }
        true
    }

    /// Секция 16³ (`section.getBoundingBox()`).
    pub fn section_visible(&self, sx: i32, sy: i32, sz: i32) -> bool {
        let (x, y, z) = (f64::from(sx * 16), f64::from(sy * 16), f64::from(sz * 16));
        self.is_visible([x, y, z], [x + 16.0, y + 16.0, z + 16.0])
    }
}

/// Для отладки/тестов: проекция камеры в двойной точности не нужна, но матрица модели строится из позиции.
pub fn look_matrix(yaw_deg: f32, pitch_deg: f32) -> Mat4 {
    // VANILLA: Camera rotation — pitch вокруг X, затем yaw+180 вокруг Y
    Mat4::from_rotation_x(pitch_deg.to_radians())
        * Mat4::from_rotation_y((yaw_deg + 180.0).to_radians())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frustum(yaw: f32, pitch: f32) -> Frustum {
        let proj = Mat4::perspective_rh(70f32.to_radians(), 16.0 / 9.0, 0.05, 512.0);
        // perspective_rh = z 0..1
        let mut f = Frustum::new(proj, look_matrix(yaw, pitch), true);
        f.prepare(0.0, 64.0, 0.0);
        f
    }

    #[test]
    fn in_front_visible_behind_hidden() {
        // yaw 0 → камера смотрит на +Z (юг)
        let f = frustum(0.0, 0.0);
        assert!(f.section_visible(0, 4, 3), "впереди");
        assert!(!f.section_visible(0, 4, -4), "позади");
        assert!(!f.section_visible(30, 4, 1), "далеко сбоку");
        // секция с камерой всегда пересекается
        assert!(f.section_visible(0, 4, 0));
        // взгляд вверх: секция под ногами скрыта, над головой видна
        let up = frustum(0.0, -80.0);
        assert!(up.section_visible(0, 8, 0));
        assert!(!up.section_visible(0, 0, 3));
    }
}
