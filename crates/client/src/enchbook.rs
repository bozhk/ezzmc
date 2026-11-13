//! Книга в окне стола зачарований (программная растеризация модели книги в текстуру).
//! VANILLA: EnchantmentScreen#tickBook/#extractBook, GuiBookModelRenderer, BookModel (26.3).
//! VANILLA-UNSURE: ориентация осей «камеры» подобрана как в превью персонажа.
use crate::charpreview::fill_tri;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_assets::texture::Image;
use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_render::gui::GuiPainter;
use glam::{Mat4, Vec3};
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Instant;

const TEX: &str = "ench_book";
/// Прямоугольник книги в окне (x0, y0, x1, y1) и масштаб. VANILLA: extractBook (14,14 → +38×31, scale 40).
const RECT: (f32, f32, f32, f32) = (14.0, 14.0, 52.0, 45.0);
const SIZE: f32 = 40.0;
/// VANILLA: PictureInPictureRenderer#getTranslateY для книги (17 · guiScale).
const TRANSLATE_Y: f32 = 17.0;

/// VANILLA: BookModel#createBodyLayer (64×32).
pub fn book_model() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    let lid = |x: f32| {
        BoxDef::new(
            [if x < 0.0 { 0.0 } else { 16.0 }, 0.0],
            [x, -5.0, -0.005],
            [6.0, 10.0, 0.005],
        )
    };
    m.add("left_lid", None, Pose::offset(0.0, 0.0, -1.0), &[lid(-6.0)]);
    m.add("right_lid", None, Pose::offset(0.0, 0.0, 1.0), &[lid(0.0)]);
    m.add(
        "seam",
        None,
        Pose {
            y_rot: FRAC_PI_2,
            ..Pose::default()
        },
        &[BoxDef::new(
            [12.0, 0.0],
            [-1.0, -5.0, 0.0],
            [2.0, 10.0, 0.005],
        )],
    );
    m.add(
        "left_pages",
        None,
        Pose::default(),
        &[BoxDef::new(
            [0.0, 10.0],
            [0.0, -4.0, -0.99],
            [5.0, 8.0, 1.0],
        )],
    );
    m.add(
        "right_pages",
        None,
        Pose::default(),
        &[BoxDef::new(
            [12.0, 10.0],
            [0.0, -4.0, -0.01],
            [5.0, 8.0, 1.0],
        )],
    );
    let page = BoxDef::new([24.0, 10.0], [0.0, -4.0, 0.0], [5.0, 8.0, 0.005]);
    m.add("flip_page1", None, Pose::default(), &[page]);
    m.add("flip_page2", None, Pose::default(), &[page]);
    m
}

/// VANILLA: BookModel#setupAnim.
pub fn setup_anim(m: &mut Model, openness: f32, flip1: f32, flip2: f32) {
    let set = |m: &mut Model, n: &str, y_rot: Option<f32>, x: Option<f32>| {
        if let Some(i) = m.index(n) {
            if let Some(r) = y_rot {
                m.parts[i].pose.y_rot = r;
            }
            if let Some(x) = x {
                m.parts[i].pose.x = x;
            }
        }
    };
    let s = openness.sin();
    set(m, "left_lid", Some(PI + openness), None);
    set(m, "right_lid", Some(-openness), None);
    set(m, "left_pages", Some(openness), Some(s));
    set(m, "right_pages", Some(-openness), Some(s));
    set(
        m,
        "flip_page1",
        Some(openness - openness * 2.0 * flip1),
        Some(s),
    );
    set(
        m,
        "flip_page2",
        Some(openness - openness * 2.0 * flip2),
        Some(s),
    );
}

/// Отпечаток предмета слота 0 (VANILLA: ItemStack#matches — тип, число, компоненты).
pub fn fingerprint(s: Option<&Option<ezzmc_protocol::item::ItemStack>>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    if let Some(Some(s)) = s {
        (s.item, s.count, &s.patch.added, &s.patch.removed).hash(&mut h);
    }
    h.finish()
}

pub struct EnchBook {
    model: Model,
    tex: Option<(Vec<u8>, u32, u32)>,
    flip: f32,
    o_flip: f32,
    flip_t: f32,
    flip_a: f32,
    open: f32,
    o_open: f32,
    /// Отпечаток предмета в слоте 0 (VANILLA: `last` + ItemStack#matches).
    last: u64,
    rng: u64,
    acc: f32,
    at: Option<Instant>,
    key: Option<(u32, i32, i32)>,
}

impl EnchBook {
    pub fn new(rm: &ResourceManager) -> Self {
        let tex = [
            "assets/minecraft/textures/entity/enchantment/enchanting_table_book.png",
            "assets/minecraft/textures/entity/enchanting_table_book.png",
        ]
        .iter()
        .find_map(|p| rm.read(p))
        .and_then(|d| Image::decode_png(&d).ok())
        .map(|i| (i.to_rgba8(), i.width, i.height));
        EnchBook {
            model: book_model(),
            tex,
            flip: 0.0,
            o_flip: 0.0,
            flip_t: 0.0,
            flip_a: 0.0,
            open: 0.0,
            o_open: 0.0,
            last: 0,
            rng: 0x2545_F491_4F6C_DD1D,
            acc: 0.0,
            at: None,
            key: None,
        }
    }

    fn rand4(&mut self) -> i32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng >> 33) as i32 & 3
    }

    /// VANILLA: EnchantmentScreen#tickBook.
    fn tick(&mut self, item: u64, any_cost: bool) {
        if item != self.last {
            self.last = item;
            loop {
                self.flip_t += (self.rand4() - self.rand4()) as f32;
                if !(self.flip <= self.flip_t + 1.0 && self.flip >= self.flip_t - 1.0) {
                    break;
                }
            }
        }
        self.o_flip = self.flip;
        self.o_open = self.open;
        self.open += if any_cost { 0.2 } else { -0.2 };
        self.open = self.open.clamp(0.0, 1.0);
        let diff = ((self.flip_t - self.flip) * 0.4).clamp(-0.2, 0.2);
        self.flip_a += (diff - self.flip_a) * 0.9;
        self.flip += self.flip_a;
    }

    /// `item` — отпечаток слота 0, `any_cost` — есть ли цена хоть в одной кнопке.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        p: &mut GuiPainter,
        dev: &wgpu::Device,
        q: &wgpu::Queue,
        scale: u32,
        left: f32,
        top: f32,
        item: u64,
        any_cost: bool,
    ) {
        let now = Instant::now();
        let dt = self
            .at
            .replace(now)
            .map_or(0.0, |t| (now - t).as_secs_f32());
        self.acc += dt.min(0.25) * 20.0;
        while self.acc >= 1.0 {
            self.acc -= 1.0;
            self.tick(item, any_cost);
        }
        let a = self.acc;
        let open = self.o_open + (self.open - self.o_open) * a;
        let flip = self.o_flip + (self.flip - self.o_flip) * a;
        let key = (scale, (open * 1e4) as i32, (flip * 1e4) as i32);
        let (w, h) = (
            ((RECT.2 - RECT.0) as u32) * scale,
            ((RECT.3 - RECT.1) as u32) * scale,
        );
        if self.key != Some(key) || !p.has(TEX) {
            let img = self.raster(w, h, scale as f32, open, flip);
            p.add_texture(dev, q, TEX, &img, w, h, false);
            self.key = Some(key);
        }
        p.blit(
            TEX,
            left + RECT.0,
            top + RECT.1,
            RECT.2 - RECT.0,
            RECT.3 - RECT.1,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
    }

    /// VANILLA: GuiBookModelRenderer#renderToTexture.
    fn raster(&mut self, w: u32, h: u32, scale: f32, open: f32, flip: f32) -> Vec<u8> {
        let mut px = vec![0u8; (w * h * 4) as usize];
        let Some((tex, tw, th)) = &self.tex else {
            return px;
        };
        let frac = |x: f32| x - x.floor();
        let f1 = (frac(flip + 0.25) * 1.6 - 0.3).clamp(0.0, 1.0);
        let f2 = (frac(flip + 0.75) * 1.6 - 0.3).clamp(0.0, 1.0);
        // BookModel.State#forAnimation(progress = 0, ...)
        setup_anim(&mut self.model, 1.25 * open, f1, f2);
        let o = 1.0 - open;
        let root = Mat4::from_translation(Vec3::new(0.0, TRANSLATE_Y / SIZE, 0.0))
            * Mat4::from_rotation_y(PI)
            * Mat4::from_rotation_x(25.0f32.to_radians())
            * Mat4::from_translation(Vec3::new(o * 0.2, o * 0.1, o * 0.25))
            * Mat4::from_rotation_y((-o * 90.0 - 90.0).to_radians())
            * Mat4::from_rotation_x(PI);
        let (ox, oy, k) = (w as f32 / 2.0, 0.0, SIZE * scale);
        let mut depth = vec![f32::MIN; (w * h) as usize];
        // VANILLA: Lighting.ENTITY_IN_UI.
        let l0 = Vec3::new(0.2, -1.0, 1.0).normalize();
        let l1 = Vec3::new(-0.2, -1.0, 0.0).normalize();
        for quad in self.model.quads(root) {
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
                    (tex.as_slice(), *tw, *th),
                    shade,
                    [255; 3],
                    None,
                );
            }
        }
        px
    }
}
