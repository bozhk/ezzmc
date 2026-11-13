//! Баннеры (стоячие и настенные): блок-сущность-рендер со слоями узора.
//! VANILLA: BannerRenderer#extractRenderState/#submit/#submitPatterns, BannerModel/BannerFlagModel
//! (createBodyLayer/createFlagLayer, setupAnim), BannerBlockEntity (patterns из NBT), DyeColor#getTextureDiffuseColor.
//! VANILLA-UNSURE: порядок слоёв в ванили задаётся order(); здесь слои разведены раздутием куба флага
//! (по 0.03 px на слой), т. к. буферы мобов рисуются по текстурам без порядка.

use crate::berscan::{block_entity_tag, in_view, Scanner};
use ezzmc_protocol::nbt::Tag;
use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::f32::consts::PI;

const BASE_TEX: &str = "banner/banner_base";
const MAX_LAYERS: usize = 16;
/// Раздутие флага на слой (px).
const LAYER_STEP: f32 = 0.03;

macro_rules! patterns {
    ($($n:literal),* $(,)?) => {
        /// (имя узора, текстура под `textures/entity/`).
        pub(crate) const PATTERNS: &[(&str, &str)] = &[$(($n, concat!("banner/", $n))),*];
    };
}
patterns!(
    "base",
    "border",
    "bricks",
    "circle",
    "creeper",
    "cross",
    "curly_border",
    "diagonal_left",
    "diagonal_right",
    "diagonal_up_left",
    "diagonal_up_right",
    "flow",
    "flower",
    "globe",
    "gradient",
    "gradient_up",
    "guster",
    "half_horizontal",
    "half_horizontal_bottom",
    "half_vertical",
    "half_vertical_right",
    "mojang",
    "piglin",
    "rhombus",
    "skull",
    "small_stripes",
    "square_bottom_left",
    "square_bottom_right",
    "square_top_left",
    "square_top_right",
    "straight_cross",
    "stripe_bottom",
    "stripe_center",
    "stripe_downleft",
    "stripe_downright",
    "stripe_left",
    "stripe_middle",
    "stripe_right",
    "stripe_top",
    "triangle_bottom",
    "triangle_top",
    "triangles_bottom",
    "triangles_top",
);

/// VANILLA: DyeColor (имя, textureDiffuseColor).
pub(crate) const DYES: [(&str, u32); 16] = [
    ("white", 16383998),
    ("orange", 16351261),
    ("magenta", 13061821),
    ("light_blue", 3847130),
    ("yellow", 16701501),
    ("lime", 8439583),
    ("pink", 15961002),
    ("gray", 4673362),
    ("light_gray", 10329495),
    ("cyan", 1481884),
    ("purple", 8991416),
    ("blue", 3949738),
    ("brown", 8606770),
    ("green", 6192150),
    ("red", 11546150),
    ("black", 1908001),
];

fn dye_rgba(name: &str) -> Option<[u8; 4]> {
    DYES.iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, c)| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255])
}

fn is_banner_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| s.name().ends_with("banner")))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

/// VANILLA: BannerModel#createBodyLayer(standing).
fn body_model(standing: bool) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    if standing {
        m.add(
            "pole",
            None,
            Pose::default(),
            &[BoxDef::new(
                [44.0, 0.0],
                [-1.0, -42.0, -1.0],
                [2.0, 42.0, 2.0],
            )],
        );
    }
    let (y, z) = if standing {
        (-44.0, -1.0)
    } else {
        (-20.5, 9.5)
    };
    m.add(
        "bar",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 42.0], [-10.0, y, z], [20.0, 2.0, 2.0])],
    );
    m
}

/// VANILLA: BannerFlagModel#createFlagLayer(standing); `grow` — раздутие слоя (px).
fn flag_model(standing: bool, grow: f32) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let pose = if standing {
        Pose::offset(0.0, -44.0, 0.0)
    } else {
        Pose::offset(0.0, -20.5, 10.5)
    };
    m.add(
        "flag",
        None,
        pose,
        &[BoxDef::new([0.0, 0.0], [-10.0, 0.0, -2.0], [20.0, 40.0, 1.0]).grow(grow)],
    );
    m
}

/// Цвет баннера по имени блока (`red_banner`, `light_blue_wall_banner`).
fn block_color(name: &str) -> Option<&'static str> {
    let c = name
        .strip_suffix("_wall_banner")
        .or_else(|| name.strip_suffix("_banner"))?;
    DYES.iter().find(|(n, _)| *n == c).map(|(n, _)| *n)
}

/// VANILLA: Direction#toYRot по `facing`.
fn facing_yrot(f: &str) -> Option<f32> {
    Some(match f {
        "south" => 0.0,
        "west" => 90.0,
        "north" => 180.0,
        "east" => 270.0,
        _ => return None,
    })
}

/// Слои узора из блок-сущности: (текстура, цвет). VANILLA: BannerPatternLayers#CODEC.
fn read_layers(tag: Option<&Tag>) -> Vec<(&'static str, [u8; 4])> {
    let Some(Tag::List(list)) = tag.and_then(|t| t.get("patterns")) else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|l| {
            let id = l.get("pattern")?.as_str()?;
            let id = id.strip_prefix("minecraft:").unwrap_or(id);
            let color = dye_rgba(l.get("color")?.as_str()?)?;
            let tex = PATTERNS.iter().find(|(n, _)| *n == id)?.1;
            Some((tex, color))
        })
        .take(MAX_LAYERS)
        .collect()
}

pub struct Banners {
    scan: Scanner,
    body: [Model; 2],
    /// flags[standing][слой]: 0 — базовая заливка, 1.. — узоры; слой 0 тоже раздут на шаг.
    flags: [Vec<Model>; 2],
    /// Флаг-основа самой текстуры banner_base (без раздутия).
    flag_plain: [Model; 2],
}

impl Default for Banners {
    fn default() -> Self {
        let mk = |s| {
            (0..=MAX_LAYERS + 1)
                .map(|k| flag_model(s, LAYER_STEP * k as f32))
                .collect::<Vec<_>>()
        };
        Banners {
            scan: Scanner::default(),
            body: [body_model(false), body_model(true)],
            flags: [mk(false), mk(true)],
            flag_plain: [flag_model(false, 0.0), flag_model(true, 0.0)],
        }
    }
}

impl Banners {
    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        mobs: &mut crate::mobs::Mobs,
    ) {
        self.scan.update(level, cam, versions, is_banner_id);
        let pt = level.partial_tick();
        for &(p, s) in &self.scan.list {
            if !in_view(p, cam) {
                continue;
            }
            let name = s.name();
            let Some(base) = block_color(name).and_then(dye_rgba) else {
                continue;
            };
            let standing = !name.ends_with("_wall_banner");
            let yrot = if standing {
                // VANILLA: RotationSegment#convertToDegrees (16 сегментов, в [-180, 180)).
                let seg: i32 = s
                    .property("rotation")
                    .and_then(|r| r.parse().ok())
                    .unwrap_or(0);
                let d = (seg & 15) as f32 * 22.5;
                if d >= 180.0 {
                    d - 360.0
                } else {
                    d
                }
            } else {
                match s.property("facing").and_then(facing_yrot) {
                    Some(y) => y,
                    None => continue,
                }
            };
            // VANILLA: BannerRenderer#extractRenderState — фаза колыхания.
            let phase = (((i64::from(p[0]) * 7
                + i64::from(p[1]) * 9
                + i64::from(p[2]) * 13
                + level.game_time)
                .rem_euclid(100)) as f32
                + pt)
                / 100.0;
            let si = usize::from(standing);
            let flag_x = (-0.0125 + 0.01 * (2.0 * PI * phase).cos()) * PI;
            // BannerRenderer: Transformation(translation(0.5,0,0.5), rotY(-angle), scale(2/3,-2/3,-2/3)).
            let k = 0.666_666_7;
            let root = Mat4::from_translation(Vec3::new(
                (f64::from(p[0]) - cam[0]) as f32 + 0.5,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32 + 0.5,
            )) * Mat4::from_rotation_y((-yrot).to_radians())
                * Mat4::from_scale(Vec3::new(k, -k, -k));
            let light = crate::particles::light_at(
                level,
                f64::from(p[0]),
                f64::from(p[1]),
                f64::from(p[2]),
            );
            self.body[si].bake(root, [255; 4], light, mobs.out(BASE_TEX));
            let tilt = |m: &mut Model| {
                m.reset_pose();
                m.parts[0].pose.x_rot = flag_x;
            };
            tilt(&mut self.flag_plain[si]);
            self.flag_plain[si].bake(root, [255; 4], light, mobs.out(BASE_TEX));
            // Слой «base» (заливка цветом баннера), затем узоры.
            let layers = read_layers(block_entity_tag(level, p));
            let fill = std::iter::once(("banner/base", base)).chain(layers);
            for (i, (tex, color)) in fill.enumerate() {
                let m = &mut self.flags[si][i + 1];
                tilt(m);
                m.bake(root, color, light, mobs.out(tex));
            }
        }
    }
}
