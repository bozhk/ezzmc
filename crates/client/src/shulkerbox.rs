//! Шалкер-коробки: блок-сущность-рендер (мир), модель для предмета (хотбар/рука/земля).
//! VANILLA: ShulkerBoxRenderer#submit/extractRenderState (TRANSFORMATIONS, ShulkerBoxModel),
//! ShulkerBoxBlockEntity#updateAnimation/triggerEvent (прогресс ±0.1/тик, событие 1),
//! ShulkerBoxSpecialRenderer (предмет, openness 0), ShulkerModel#createBoxLayer (lid+base).

use crate::berscan::{in_view, Scanner};
use ezzmc_render::entitymodel::Model;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{EulerRot, Mat4, Quat, Vec3};
use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, PI};

const COLORS: [&str; 16] = [
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

/// Текстура под `textures/entity/` по имени блока/предмета (`shulker_box`, `red_shulker_box`).
pub fn texture_of(name: &str) -> Option<&'static str> {
    if name == "shulker_box" {
        return Some("shulker/shulker");
    }
    let c = name.strip_suffix("_shulker_box")?;
    let i = COLORS.iter().position(|&x| x == c)?;
    Some(TEXTURES[i])
}

const TEXTURES: [&str; 16] = [
    "shulker/shulker_white",
    "shulker/shulker_orange",
    "shulker/shulker_magenta",
    "shulker/shulker_light_blue",
    "shulker/shulker_yellow",
    "shulker/shulker_lime",
    "shulker/shulker_pink",
    "shulker/shulker_gray",
    "shulker/shulker_light_gray",
    "shulker/shulker_cyan",
    "shulker/shulker_purple",
    "shulker/shulker_blue",
    "shulker/shulker_brown",
    "shulker/shulker_green",
    "shulker/shulker_red",
    "shulker/shulker_black",
];

/// Все текстуры коробок (для атласа предметов).
pub fn item_textures() -> impl Iterator<Item = &'static str> {
    std::iter::once("shulker/shulker").chain(TEXTURES.iter().copied())
}

/// VANILLA: ShulkerModel#createBoxLayer — только крышка и основание (голова скрыта).
fn box_model() -> Model {
    let mut m = ezzmc_render::animals::shulker_mesh();
    if let Some(i) = m.index("head") {
        m.parts[i].visible = false;
    }
    m
}

/// VANILLA: ShulkerBoxModel#setupAnim.
fn setup(m: &mut Model, progress: f32) {
    m.reset_pose();
    if let Some(i) = m.index("lid") {
        m.parts[i].pose.y = 24.0 - progress * 0.5 * 16.0;
        m.parts[i].pose.y_rot = (270.0 * progress).to_radians();
    }
}

/// Модель предмета (openness 0) и текстура.
pub fn item_model(name: &str) -> Option<(Model, &'static str)> {
    let tex = texture_of(name)?;
    let mut m = box_model();
    setup(&mut m, 0.0);
    Some((m, tex))
}

/// Корень модели предмета (`transformation` из items/*.shulker_box.json).
pub fn item_root() -> Mat4 {
    Mat4::from_translation(Vec3::new(0.5, 1.4995, 0.5))
        * Mat4::from_rotation_x(PI)
        * Mat4::from_scale(Vec3::splat(0.9995))
}

/// VANILLA: Direction#getRotation.
fn dir_rotation(f: &str) -> Quat {
    let r = |x: f32, y: f32, z: f32| Quat::from_euler(EulerRot::ZYX, z, y, x);
    match f {
        "down" => Quat::from_rotation_x(PI),
        "north" => r(FRAC_PI_2, 0.0, PI),
        "south" => Quat::from_rotation_x(FRAC_PI_2),
        "west" => r(FRAC_PI_2, 0.0, FRAC_PI_2),
        "east" => r(FRAC_PI_2, 0.0, -FRAC_PI_2),
        _ => Quat::IDENTITY,
    }
}

/// VANILLA: ShulkerBoxRenderer#createModelTransform.
fn block_transform(f: &str) -> Mat4 {
    Mat4::from_translation(Vec3::splat(0.5))
        * Mat4::from_scale(Vec3::splat(0.9995))
        * Mat4::from_quat(dir_rotation(f))
        * Mat4::from_scale(Vec3::new(1.0, -1.0, -1.0))
        * Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0))
}

fn is_box_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| texture_of(s.name()).is_some()))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Status {
    #[default]
    Closed,
    Opening,
    Opened,
    Closing,
}

/// VANILLA: ShulkerBoxBlockEntity (animationStatus, progress, progressOld).
#[derive(Clone, Copy, Default)]
struct Lid {
    status: Status,
    progress: f32,
    old: f32,
}

impl Lid {
    fn tick(&mut self) {
        self.old = self.progress;
        match self.status {
            Status::Closed => self.progress = 0.0,
            Status::Opening => {
                self.progress += 0.1;
                if self.progress >= 1.0 {
                    self.status = Status::Opened;
                    self.progress = 1.0;
                }
            }
            Status::Opened => self.progress = 1.0,
            Status::Closing => {
                self.progress -= 0.1;
                if self.progress <= 0.0 {
                    self.status = Status::Closed;
                    self.progress = 0.0;
                }
            }
        }
    }
    fn get(&self, a: f32) -> f32 {
        self.old + (self.progress - self.old) * a
    }
}

pub struct ShulkerBoxes {
    scan: Scanner,
    model: Model,
    lids: HashMap<[i32; 3], Lid>,
    last_tick: Option<i64>,
}

impl Default for ShulkerBoxes {
    fn default() -> Self {
        ShulkerBoxes {
            scan: Scanner::default(),
            model: box_model(),
            lids: HashMap::new(),
            last_tick: None,
        }
    }
}

impl ShulkerBoxes {
    /// VANILLA: ShulkerBoxBlockEntity#triggerEvent (b0 == 1: b1 == 0 закрывается, b1 == 1 открывается).
    pub fn block_event(&mut self, pos: [i32; 3], b0: i32, b1: i32) {
        if b0 != 1 {
            return;
        }
        let l = self.lids.entry(pos).or_default();
        if b1 == 0 {
            l.status = Status::Closing;
        }
        if b1 == 1 {
            l.status = Status::Opening;
        }
    }

    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        mobs: &mut crate::mobs::Mobs,
    ) {
        self.scan.update(level, cam, versions, is_box_id);
        let now = level.game_time;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            Some(_) => 0,
            None => 1,
        };
        for _ in 0..ticks {
            for l in self.lids.values_mut() {
                l.tick();
            }
        }
        self.lids
            .retain(|_, l| l.status != Status::Closed || l.progress > 0.0);
        let pt = level.partial_tick();
        for &(p, s) in &self.scan.list {
            if !in_view(p, cam) {
                continue;
            }
            let Some(tex) = texture_of(s.name()) else {
                continue;
            };
            let progress = self.lids.get(&p).map_or(0.0, |l| l.get(pt));
            setup(&mut self.model, progress);
            let origin = Vec3::new(
                (f64::from(p[0]) - cam[0]) as f32,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32,
            );
            let root = Mat4::from_translation(origin)
                * block_transform(s.property("facing").unwrap_or("up"));
            let light = crate::particles::light_at(
                level,
                f64::from(p[0]),
                f64::from(p[1]),
                f64::from(p[2]),
            );
            self.model.bake(root, [255; 4], light, mobs.out(tex));
        }
    }
}
