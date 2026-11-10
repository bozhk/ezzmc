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

/// Состояние игрока для превью (собирает `WorldView::preview_snapshot`).
#[derive(Clone)]
pub struct Snap {
    pub state: HumanoidState,
    pub crouching: bool,
    pub swim: crate::worldview::SwimPose,
    pub right: Option<std::rc::Rc<crate::handitem::HandMesh>>,
    pub left: Option<std::rc::Rc<crate::handitem::HandMesh>>,
    pub blocks: std::rc::Rc<(Vec<u8>, u32, u32)>,
    pub items: Option<std::rc::Rc<(Vec<u8>, u32, u32)>>,
    /// ItemStack#hasFoil предметов в правой/левой руке.
    pub foil: (bool, bool),
}

/// Текстура блеска и матрица её сдвига (VANILLA: TextureTransform#setupGlintTexturing).
struct Glint<'a> {
    tex: (&'a [u8], u32, u32),
    mat: Mat4,
}

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
    /// Броня на превью (VANILLA: InventoryScreen рисует LocalPlayer целиком, с экипировкой).
    armor: crate::armor::ArmorKit,
    looks: [Option<crate::armor::Look>; 4],
    /// Блеск брони / предметов (`enchanted_glint_armor/item.png`).
    glint_armor: Option<(Vec<u8>, u32, u32)>,
    glint_item: Option<(Vec<u8>, u32, u32)>,
    snap: Option<Snap>,
    /// Отпечаток снимка состояния (поза, предметы).
    snap_hash: u64,
    /// Ключ последней отрисованной текстуры: (масштаб GUI, углы в десятитысячных).
    key: Option<(u32, i32, i32, [i32; 5])>,
}

fn load_png(rm: &ResourceManager, name: &str) -> Option<(Vec<u8>, u32, u32)> {
    rm.read(&format!("assets/minecraft/textures/misc/{name}.png"))
        .and_then(|d| Image::decode_png(&d).ok())
        .map(|i| (i.to_rgba8(), i.width, i.height))
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
            armor: crate::armor::ArmorKit::new(rm),
            looks: [None; 4],
            glint_armor: load_png(rm, "enchanted_glint_armor"),
            glint_item: load_png(rm, "enchanted_glint_item"),
            snap: None,
            snap_hash: 0,
            key: None,
        }
    }

    /// Состояние игрока (присед/плавание/предметы); изменение сбрасывает кэш текстуры.
    pub fn set_snap(&mut self, snap: Snap) {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        let s = &snap.state;
        for f in [
            s.walk_pos,
            s.walk_speed,
            s.swim_amount,
            s.swing,
            snap.swim.amount,
        ] {
            ((f * 1000.0) as i32).hash(&mut h);
        }
        (snap.crouching, snap.swim.in_water, snap.swim.visually).hash(&mut h);
        for m in [&snap.right, &snap.left] {
            m.as_ref()
                .map(|m| std::rc::Rc::as_ptr(m) as usize)
                .hash(&mut h);
        }
        let hash = h.finish();
        if hash != self.snap_hash {
            self.snap_hash = hash;
            self.key = None;
        }
        self.snap = Some(snap);
    }

    /// Надетая броня из слотов InventoryMenu; смена сбрасывает кэш текстуры.
    pub fn set_armor(&mut self, slots: &[Option<ezzmc_protocol::item::ItemStack>]) {
        let looks = crate::armor::looks_of_menu(slots);
        if looks != self.looks {
            self.looks = looks;
            self.key = None;
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
        // Блеск анимирован → перерисовка каждый кадр, пока что-то блестит.
        let foil = self.looks.iter().flatten().any(|l| l.foil)
            || self.snap.as_ref().is_some_and(|s| s.foil.0 || s.foil.1);
        if self.key != Some(key) || !p.has(TEX) || foil {
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
        let snap = self.snap.clone();
        let st = HumanoidState {
            y_rot: xa * 20.0,
            x_rot: -ya * 20.0,
            ..snap.as_ref().map(|s| s.state).unwrap_or_default()
        };
        let crouching = snap.as_ref().is_some_and(|s| s.crouching);
        let mut swim = snap.as_ref().map(|s| s.swim).unwrap_or_default();
        swim.x_rot = st.x_rot;
        let mut st = st;
        st.crouching = crouching;
        humanoid::setup_anim(&mut self.model, &self.parts, &st);
        let body = 180.0 + xa * 20.0;
        // VANILLA: AvatarRenderer#getRenderOffset — приседание сдвигает вниз на 2/16.
        let crouch_off = if crouching { 2.0 / 16.0 } else { 0.0 };
        let root = Mat4::from_translation(Vec3::new(
            0.0,
            BOX_HEIGHT / 2.0 + OFFSET_Y + crouch_off,
            0.0,
        )) * Mat4::from_rotation_z(std::f32::consts::PI)
            * Mat4::from_rotation_x((ya * 20.0).to_radians())
            * Mat4::from_rotation_y((180.0 - body).to_radians())
            * swim.matrix()
            * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
            * Mat4::from_scale(Vec3::splat(0.9375))
            * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0));
        let (ox, oy, k) = (w as f32 / 2.0, h as f32 / 2.0, size * scale);
        let mut depth = vec![f32::MIN; (w * h) as usize];
        // VANILLA: Lighting.ENTITY_IN_UI.
        let l0 = Vec3::new(0.2, -1.0, 1.0).normalize();
        let l1 = Vec3::new(-0.2, -1.0, 0.0).normalize();
        // Слой скина, затем слои брони (атлас: v внутри строки → v всего атласа).
        #[allow(clippy::type_complexity)]
        let mut jobs: Vec<(
            ezzmc_render::entitymodel::RasterQuad,
            Option<(u32, [u8; 3])>,
        )> = self
            .model
            .quads(root)
            .into_iter()
            .map(|q| (q, None))
            .collect();
        let (armor_quads, armor_foil) = self.armor.raster(&self.looks, &st, root);
        jobs.extend(
            armor_quads
                .into_iter()
                .map(|(q, row, tint)| (q, Some((row, tint)))),
        );
        // Предметы в руках. VANILLA: ItemInHandLayer — рука, X−90°, Y180°, translate(±1/16, 2/16, −10/16).
        let mut item_jobs = Vec::new();
        let mut item_foil: Vec<bool> = Vec::new();
        if let Some(sn) = &snap {
            let mats = self.model.part_matrices(root);
            for (mesh, arm, dx, view) in [
                (
                    &sn.right,
                    self.parts.right_arm,
                    1.0,
                    crate::handitem::View::ThirdPerson,
                ),
                (
                    &sn.left,
                    self.parts.left_arm,
                    -1.0,
                    crate::handitem::View::ThirdPersonLeft,
                ),
            ] {
                if let Some(mesh) = mesh {
                    let foil = if dx > 0.0 { sn.foil.0 } else { sn.foil.1 };
                    let m = mats[arm]
                        * Mat4::from_rotation_x((-90.0f32).to_radians())
                        * Mat4::from_rotation_y(180.0f32.to_radians())
                        * Mat4::from_translation(Vec3::new(dx / 16.0, 0.125, -0.625));
                    item_foil.extend(std::iter::repeat_n(foil, mesh.quad_count()));
                    item_jobs.extend(mesh.raster(m, view));
                }
            }
        }
        for (quad, arm) in jobs {
            // Ось Z зеркалим: зритель смотрит с +Z, лицо персонажа обращено к нему.
            let n = Vec3::new(quad.normal.x, quad.normal.y, -quad.normal.z);
            let shade = (((l0.dot(n)).max(0.0) + (l1.dot(n)).max(0.0)) * 0.6 + 0.4).min(1.0);
            let v: [Vec3; 4] = quad
                .pos
                .map(|p| Vec3::new(ox + p.x * k, oy + p.y * k, -p.z));
            let (tex, uv, tint) = match arm {
                None => ((skin.as_slice(), *sw, *sh), quad.uv, [255u8; 3]),
                Some((row, tint)) => (
                    (
                        self.armor.rgba.as_slice(),
                        self.armor.width,
                        self.armor.height,
                    ),
                    quad.uv.map(|uv| self.armor.atlas_uv(row, uv)),
                    tint,
                ),
            };
            for tri in [[0, 1, 2], [0, 2, 3]] {
                fill_tri(
                    &mut px,
                    &mut depth,
                    w,
                    h,
                    tri.map(|i| v[i]),
                    tri.map(|i| uv[i]),
                    tex,
                    shade,
                    tint,
                    None,
                );
            }
        }
        for q in &item_jobs {
            let Some(sn) = &snap else { break };
            let tex = if q.items {
                match &sn.items {
                    Some(t) => t,
                    None => continue,
                }
            } else {
                &sn.blocks
            };
            let v: [Vec3; 4] = q.pos.map(|p| Vec3::new(ox + p.x * k, oy + p.y * k, -p.z));
            for tri in [[0, 1, 2], [0, 2, 3]] {
                fill_tri(
                    &mut px,
                    &mut depth,
                    w,
                    h,
                    tri.map(|i| v[i]),
                    tri.map(|i| q.uv[i]),
                    (tex.0.as_slice(), tex.1, tex.2),
                    1.0,
                    q.color,
                    None,
                );
            }
        }
        // Блеск: повторный проход по той же геометрии, тест глубины «равно», сложение цвета.
        let white = [255u8; 4];
        if let Some(g) = &self.glint_armor {
            let gl = Glint {
                tex: (g.0.as_slice(), g.1, g.2),
                mat: crate::armor::glint_matrix(0.16),
            };
            for quad in &armor_foil {
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
                        (&white, 1, 1),
                        1.0,
                        [255; 3],
                        Some(&gl),
                    );
                }
            }
        }
        if let (Some(g), Some(sn)) = (&self.glint_item, &snap) {
            let gl = Glint {
                tex: (g.0.as_slice(), g.1, g.2),
                mat: crate::armor::glint_matrix(8.0),
            };
            for (q, _) in item_jobs.iter().zip(&item_foil).filter(|(_, f)| **f) {
                let tex = if q.items {
                    match &sn.items {
                        Some(t) => t,
                        None => continue,
                    }
                } else {
                    &sn.blocks
                };
                let v: [Vec3; 4] = q.pos.map(|p| Vec3::new(ox + p.x * k, oy + p.y * k, -p.z));
                for tri in [[0, 1, 2], [0, 2, 3]] {
                    fill_tri(
                        &mut px,
                        &mut depth,
                        w,
                        h,
                        tri.map(|i| v[i]),
                        tri.map(|i| q.uv[i]),
                        (tex.0.as_slice(), tex.1, tex.2),
                        1.0,
                        [255; 3],
                        Some(&gl),
                    );
                }
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
    tint: [u8; 3],
    glint: Option<&Glint>,
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
            // LessEqual, как у GPU: слой трима с той же геометрией ложится поверх брони.
            if z < depth[di] {
                continue;
            }
            if let Some(g) = glint {
                // Блеск: форма по альфе, без записи глубины; dst += (glint·0.75)².
                let u = w0 * uv[0][0] + w1 * uv[1][0] + w2 * uv[2][0];
                let t = w0 * uv[0][1] + w1 * uv[1][1] + w2 * uv[2][1];
                let tx = ((u * skin.1 as f32) as i64).clamp(0, i64::from(skin.1) - 1) as u32;
                let ty = ((t * skin.2 as f32) as i64).clamp(0, i64::from(skin.2) - 1) as u32;
                if skin.0[((ty * skin.1 + tx) * 4) as usize + 3] < 26 || px[di * 4 + 3] == 0 {
                    continue;
                }
                let m = g.mat.transform_point3(Vec3::new(u, t, 0.0));
                let (gw, gh) = (g.tex.1, g.tex.2);
                let gx = ((m.x.rem_euclid(1.0) * gw as f32) as u32).min(gw - 1);
                let gy = ((m.y.rem_euclid(1.0) * gh as f32) as u32).min(gh - 1);
                let s = ((gy * gw + gx) * 4) as usize;
                for c in 0..3 {
                    let v = f32::from(g.tex.0[s + c]) / 255.0 * 0.75;
                    let o = di * 4 + c;
                    px[o] = (f32::from(px[o]) + v * v * 255.0).min(255.0) as u8;
                }
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
                px[o + c] = (f32::from(skin.0[s + c]) * f32::from(tint[c]) / 255.0 * shade) as u8;
            }
            px[o + 3] = 255;
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
            armor: crate::armor::ArmorKit::new(&ResourceManager::new(Vec::new())),
            looks: [None; 4],
            glint_armor: None,
            glint_item: None,
            snap: None,
            snap_hash: 0,
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
