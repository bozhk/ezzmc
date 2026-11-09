//! Превью персонажа в экране инвентаря (программная растеризация позы модели игрока в текстуру).
//! VANILLA: InventoryScreen#extractEntityInInventoryFollowsMouse (прямоугольник 26,8–75,78, размер 30,
//! смещение 0.0625, поворот тела/головы за курсором), Lighting.ENTITY_IN_UI.
//! VANILLA-UNSURE: ориентация осей «камеры» подобрана так, чтобы персонаж смотрел на зрителя.
use ezzmc_assets::pack::ResourceManager;
use ezzmc_assets::texture::Image;
use ezzmc_render::entitymodel::Model;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::humanoid::{self, HumanoidParts, HumanoidState};
use glam::{Mat4, Vec3};

/// Прямоугольник превью в координатах GUI меню (x0, y0, x1, y1) и размер модели.
pub const RECT: (f32, f32, f32, f32) = (26.0, 8.0, 75.0, 78.0);
pub const SIZE: f32 = 30.0;
const OFFSET_Y: f32 = 0.0625;
/// Высота хитбокса игрока (стоя).
const BOX_HEIGHT: f32 = 1.8;
const TEX: &str = "char_preview";

pub struct CharPreview {
    skin: Option<(Vec<u8>, u32, u32)>,
    model: Model,
    parts: HumanoidParts,
    /// Ключ последней отрисованной текстуры: (масштаб GUI, углы в десятитысячных).
    key: Option<(u32, i32, i32, [i32; 5])>,
}

impl CharPreview {
    pub fn new(rm: &ResourceManager) -> Self {
        // VANILLA: DefaultPlayerSkin — оффлайн всегда Стив.
        let skin = rm
            .read("assets/minecraft/textures/entity/player/wide/steve.png")
            .and_then(|d| Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let (model, parts) = humanoid::player_mesh(false);
        CharPreview {
            skin,
            model,
            parts,
            key: None,
        }
    }

    /// Нарисовать превью; `left`/`top` — угол окна меню, `mx`/`my` — курсор (GUI).
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        p: &mut GuiPainter,
        dev: &wgpu::Device,
        q: &wgpu::Queue,
        scale: u32,
        left: f32,
        top: f32,
        mx: f32,
        my: f32,
        rect: (f32, f32, f32, f32),
        size: f32,
    ) {
        let (x0, y0, x1, y1) = (left + rect.0, top + rect.1, left + rect.2, top + rect.3);
        let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let xa = ((cx - mx) / 40.0).atan();
        let ya = ((cy - my) / 40.0).atan();
        let geom = [
            rect.0 as i32,
            rect.1 as i32,
            rect.2 as i32,
            rect.3 as i32,
            size as i32,
        ];
        let key = (scale, (xa * 1e4) as i32, (ya * 1e4) as i32, geom);
        if self.key != Some(key) || !p.has(TEX) {
            let (w, h) = (
                ((rect.2 - rect.0) as u32) * scale,
                ((rect.3 - rect.1) as u32) * scale,
            );
            let img = self.raster(w, h, scale as f32, size, xa, ya);
            p.add_texture(dev, q, TEX, &img, w, h, false);
            self.key = Some(key);
        }
        p.blit(
            TEX,
            x0,
            y0,
            x1 - x0,
            y1 - y0,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
    }

    fn raster(&mut self, w: u32, h: u32, scale: f32, size: f32, xa: f32, ya: f32) -> Vec<u8> {
        let mut px = vec![0u8; (w * h * 4) as usize];
        let Some((skin, sw, sh)) = &self.skin else {
            return px;
        };
        // VANILLA: bodyRot = 180 + xAngle*20, yRot = xAngle*20, xRot = -yAngle*20 (градусы).
        let st = HumanoidState {
            y_rot: xa * 20.0,
            x_rot: -ya * 20.0,
            ..Default::default()
        };
        humanoid::setup_anim(&mut self.model, &self.parts, &st);
        let body = 180.0 + xa * 20.0;
        let root = Mat4::from_translation(Vec3::new(0.0, BOX_HEIGHT / 2.0 + OFFSET_Y, 0.0))
            * Mat4::from_rotation_z(std::f32::consts::PI)
            * Mat4::from_rotation_x((ya * 20.0).to_radians())
            * Mat4::from_rotation_y((180.0 - body).to_radians())
            * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
            * Mat4::from_scale(Vec3::splat(0.9375))
            * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0));
        let (ox, oy, k) = (w as f32 / 2.0, h as f32 / 2.0, size * scale);
        let mut depth = vec![f32::MIN; (w * h) as usize];
        // VANILLA: Lighting.ENTITY_IN_UI.
        let l0 = Vec3::new(0.2, -1.0, 1.0).normalize();
        let l1 = Vec3::new(-0.2, -1.0, 0.0).normalize();
        for quad in self.model.quads(root) {
            // Ось Z зеркалим: зритель смотрит с +Z, лицо персонажа обращено к нему.
            let n = Vec3::new(quad.normal.x, quad.normal.y, -quad.normal.z);
            let shade = (((l0.dot(n)).max(0.0) + (l1.dot(n)).max(0.0)) * 0.6 + 0.4).min(1.0);
            let v: [Vec3; 4] = quad
                .pos
                .map(|p| Vec3::new(ox + p.x * k, oy + p.y * k, -p.z));
            for tri in [[0, 1, 2], [0, 2, 3]] {
                fill_tri(
                    &mut px,
                    &mut depth,
                    w,
                    h,
                    tri.map(|i| v[i]),
                    tri.map(|i| quad.uv[i]),
                    (skin, *sw, *sh),
                    shade,
                );
            }
        }
        px
    }
}

/// Заливка треугольника: z-буфер (больше — ближе), nearest-выборка, alpha-test.
#[allow(clippy::too_many_arguments)]
fn fill_tri(
    px: &mut [u8],
    depth: &mut [f32],
    w: u32,
    h: u32,
    v: [Vec3; 3],
    uv: [[f32; 2]; 3],
    skin: (&[u8], u32, u32),
    shade: f32,
) {
    let area = (v[1].x - v[0].x) * (v[2].y - v[0].y) - (v[2].x - v[0].x) * (v[1].y - v[0].y);
    if area.abs() < 1e-6 {
        return;
    }
    let min_x = v
        .iter()
        .map(|p| p.x)
        .fold(f32::MAX, f32::min)
        .floor()
        .max(0.0) as u32;
    let max_x = (v
        .iter()
        .map(|p| p.x)
        .fold(f32::MIN, f32::max)
        .ceil()
        .max(0.0) as u32)
        .min(w);
    let min_y = v
        .iter()
        .map(|p| p.y)
        .fold(f32::MAX, f32::min)
        .floor()
        .max(0.0) as u32;
    let max_y = (v
        .iter()
        .map(|p| p.y)
        .fold(f32::MIN, f32::max)
        .ceil()
        .max(0.0) as u32)
        .min(h);
    for y in min_y..max_y {
        for x in min_x..max_x {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = ((v[1].x - fx) * (v[2].y - fy) - (v[2].x - fx) * (v[1].y - fy)) / area;
            let w1 = ((v[2].x - fx) * (v[0].y - fy) - (v[0].x - fx) * (v[2].y - fy)) / area;
            let w2 = 1.0 - w0 - w1;
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }
            let z = w0 * v[0].z + w1 * v[1].z + w2 * v[2].z;
            let di = (y * w + x) as usize;
            if z <= depth[di] {
                continue;
            }
            let u = w0 * uv[0][0] + w1 * uv[1][0] + w2 * uv[2][0];
            let t = w0 * uv[0][1] + w1 * uv[1][1] + w2 * uv[2][1];
            let tx = ((u * skin.1 as f32) as i64).clamp(0, i64::from(skin.1) - 1) as u32;
            let ty = ((t * skin.2 as f32) as i64).clamp(0, i64::from(skin.2) - 1) as u32;
            let s = ((ty * skin.1 + tx) * 4) as usize;
            if skin.0[s + 3] < 26 {
                continue;
            }
            depth[di] = z;
            let o = di * 4;
            for c in 0..3 {
                px[o + c] = (f32::from(skin.0[s + c]) * shade) as u8;
            }
            px[o + 3] = 255;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Синтетический скин: непрозрачный серый; персонаж занимает центр и не вылезает за рамку.
    #[test]
    fn raster_fills_center_not_edges() {
        let (model, parts) = humanoid::player_mesh(false);
        let mut cp = CharPreview {
            skin: Some((vec![200; 64 * 64 * 4], 64, 64)),
            model,
            parts,
            key: None,
        };
        if let Ok(f) = std::env::var("EZZMC_PREVIEW_SKIN") {
            let i = Image::decode_png(&std::fs::read(f).unwrap()).unwrap();
            cp.skin = Some((i.to_rgba8(), i.width, i.height));
        }
        let (w, h) = (49u32 * 4, 70u32 * 4);
        let img = cp.raster(w, h, 4.0, 30.0, 0.3, 0.2);
        let a = |x: u32, y: u32| img[((y * w + x) * 4 + 3) as usize];
        assert_eq!(a(98, 140), 255);
        assert_eq!(a(0, 0), 0);
        assert_eq!(a(195, 279), 0);
        if let Ok(dir) = std::env::var("EZZMC_DUMP_PREVIEW") {
            let mut s = format!("P6\n{w} {h}\n255\n").into_bytes();
            for p in img.chunks(4) {
                s.extend_from_slice(&p[..3]);
            }
            std::fs::write(dir, s).unwrap();
        }
    }
}
