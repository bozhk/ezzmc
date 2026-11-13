//! Головы (скелет, иссушитель, игрок, зомби, крипер, дракон, пиглин; стоячие и настенные).
//! VANILLA: SkullBlockRenderer#extractRenderState/#submit (TRANSFORMATIONS), SkullModel
//! (createHumanoidHeadLayer/createMobHeadLayer), DragonHeadModel, PiglinHeadModel,
//! SkullBlockEntity#animation/getAnimation (анимация при `powered`).
//! VANILLA-UNSURE: голова игрока всегда со скином Steve (профиль владельца не подгружается).

use crate::berscan::{in_view, Scanner};
use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Skeleton,
    WitherSkeleton,
    Player,
    Zombie,
    Creeper,
    Dragon,
    Piglin,
}

/// (тип, стоячий блок, настенный блок, текстура под `textures/entity/`).
const KINDS: [(Kind, &str, &str, &str); 7] = [
    (
        Kind::Skeleton,
        "skeleton_skull",
        "skeleton_wall_skull",
        "skeleton/skeleton",
    ),
    (
        Kind::WitherSkeleton,
        "wither_skeleton_skull",
        "wither_skeleton_wall_skull",
        "skeleton/wither_skeleton",
    ),
    (
        Kind::Player,
        "player_head",
        "player_wall_head",
        "player/wide/steve",
    ),
    (
        Kind::Zombie,
        "zombie_head",
        "zombie_wall_head",
        "zombie/zombie",
    ),
    (
        Kind::Creeper,
        "creeper_head",
        "creeper_wall_head",
        "creeper/creeper",
    ),
    (
        Kind::Dragon,
        "dragon_head",
        "dragon_wall_head",
        "enderdragon/dragon",
    ),
    (
        Kind::Piglin,
        "piglin_head",
        "piglin_wall_head",
        "piglin/piglin",
    ),
];

fn kind_of(name: &str) -> Option<(Kind, bool, &'static str)> {
    KINDS.iter().find_map(|&(k, a, w, t)| {
        if name == a {
            Some((k, false, t))
        } else if name == w {
            Some((k, true, t))
        } else {
            None
        }
    })
}

/// Модель и текстура головы по имени предмета (`skeleton_skull`, `player_head`, …); поза — как у
/// стоящей головы без анимации. VANILLA: SkullBlockRenderer#submit для предмета (ground, rot 0).
pub fn item_model(name: &str) -> Option<(Model, &'static str)> {
    let (kind, _, tex) = kind_of(name)?;
    let mut m = match kind {
        Kind::Skeleton | Kind::WitherSkeleton | Kind::Creeper => mob_head(),
        Kind::Player | Kind::Zombie => humanoid_head(),
        Kind::Dragon => dragon_head(),
        Kind::Piglin => piglin_head(),
    };
    m.reset_pose();
    match kind {
        Kind::Dragon => m.parts[0].scale = Vec3::splat(0.75),
        Kind::Piglin => {
            m.parts[1].pose.z_rot = -(2.5 + 1.0) * 0.2;
            m.parts[2].pose.z_rot = (2.5 + 1.0) * 0.2;
        }
        _ => {}
    }
    Some((m, tex))
}

/// Имя предмета-головы, если стек — голова (для CustomHeadLayer).
pub fn worn_name(st: &ezzmc_protocol::item::ItemStack) -> Option<&'static str> {
    let name = *ezzmc_protocol::generated::r26_3::ITEM.get(st.item as usize)?;
    kind_of(name).map(|_| name)
}

/// VANILLA: CustomHeadLayer — голова на голове: scale 1.1875 в системе части головы (без доп. зеркала).
pub fn worn_root(head: Mat4) -> Mat4 {
    head * Mat4::from_scale(Vec3::splat(1.1875))
}

/// Текстуры голов (под `textures/entity/`) для атласа предметов в руке.
pub fn item_textures() -> impl Iterator<Item = &'static str> {
    KINDS.iter().map(|k| k.3)
}

/// Корень модели головы-предмета в блочных координатах (translation 0.5,0,0.5 из items/*.json).
pub fn item_root() -> Mat4 {
    // left_rotation [1,0,0,0] = поворот на 180° вокруг X (не зеркало блока).
    Mat4::from_translation(Vec3::new(0.5, 0.0, 0.5)) * Mat4::from_rotation_x(PI)
}

fn is_skull_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| kind_of(s.name()).is_some()))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

fn head_box() -> BoxDef {
    BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])
}

/// VANILLA: SkullModel#createMobHeadLayer (64×32).
fn mob_head() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add("head", None, Pose::default(), &[head_box()]);
    m
}

/// VANILLA: SkullModel#createHumanoidHeadLayer (64×64, слой шапки).
fn humanoid_head() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add("head", None, Pose::default(), &[head_box()]);
    m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(0.25)],
    );
    m
}

/// VANILLA: DragonHeadModel#createHeadLayer (256×256); голова масштабом 0.75.
fn dragon_head() -> Model {
    let mut m = Model::new([256.0, 256.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, -7.986_666 * 0.75, 0.0),
        &[
            BoxDef::new([176.0, 44.0], [-6.0, -1.0, -24.0], [12.0, 5.0, 16.0]),
            BoxDef::new([112.0, 30.0], [-8.0, -8.0, -10.0], [16.0, 16.0, 16.0]),
            BoxDef::new([0.0, 0.0], [-5.0, -12.0, -4.0], [2.0, 4.0, 6.0]).mirror(),
            BoxDef::new([112.0, 0.0], [-5.0, -3.0, -22.0], [2.0, 2.0, 4.0]).mirror(),
            BoxDef::new([0.0, 0.0], [3.0, -12.0, -4.0], [2.0, 4.0, 6.0]),
            BoxDef::new([112.0, 0.0], [3.0, -3.0, -22.0], [2.0, 2.0, 4.0]),
        ],
    );
    m.add(
        "jaw",
        Some(head),
        Pose::offset(0.0, 4.0, -8.0),
        &[BoxDef::new(
            [176.0, 65.0],
            [-6.0, 0.0, -16.0],
            [12.0, 4.0, 16.0],
        )],
    );
    m
}

/// VANILLA: PiglinHeadModel (AbstractPiglinModel#addHead, 64×64).
fn piglin_head() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-5.0, -8.0, -4.0], [10.0, 8.0, 8.0]),
            BoxDef::new([31.0, 1.0], [-2.0, -4.0, -5.0], [4.0, 4.0, 1.0]),
            BoxDef::new([2.0, 4.0], [2.0, -2.0, -5.0], [1.0, 2.0, 1.0]),
            BoxDef::new([2.0, 0.0], [-3.0, -2.0, -5.0], [1.0, 2.0, 1.0]),
        ],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(4.5, -6.0, 0.0),
        &[BoxDef::new([51.0, 6.0], [0.0, 0.0, -2.0], [1.0, 5.0, 4.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-4.5, -6.0, 0.0),
        &[BoxDef::new([39.0, 6.0], [-1.0, 0.0, -2.0], [1.0, 5.0, 4.0])],
    );
    m
}

/// SkullBlockEntity: счётчик анимации (тикает, пока `powered`).
#[derive(Clone, Copy, Default)]
struct Anim {
    count: i32,
    animating: bool,
}

pub struct Skulls {
    scan: Scanner,
    /// mob, humanoid, dragon, piglin.
    models: [Model; 4],
    anims: HashMap<[i32; 3], Anim>,
    last_tick: Option<i64>,
}

impl Default for Skulls {
    fn default() -> Self {
        Skulls {
            scan: Scanner::default(),
            models: [mob_head(), humanoid_head(), dragon_head(), piglin_head()],
            anims: HashMap::new(),
            last_tick: None,
        }
    }
}

/// VANILLA: Direction#toYRot.
fn facing_yrot(f: &str) -> Option<(f32, f32, f32)> {
    // (yrot, stepX, stepZ)
    Some(match f {
        "south" => (0.0, 0.0, 1.0),
        "west" => (90.0, -1.0, 0.0),
        "north" => (180.0, 0.0, -1.0),
        "east" => (270.0, 1.0, 0.0),
        _ => return None,
    })
}

impl Skulls {
    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        mobs: &mut crate::mobs::Mobs,
    ) {
        if self.scan.update(level, cam, versions, is_skull_id) {
            let list = &self.scan.list;
            self.anims.retain(|p, _| list.iter().any(|(q, _)| q == p));
        }
        let now = level.game_time;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            Some(_) => 0,
            None => 1,
        };
        let pt = level.partial_tick();
        for &(p, s) in &self.scan.list {
            let Some((kind, wall, tex)) = kind_of(s.name()) else {
                continue;
            };
            // VANILLA: SkullBlockEntity#animation — тикает только при powered=true.
            let powered = s.property("powered") == Some("true");
            let a = self.anims.entry(p).or_default();
            for _ in 0..ticks {
                a.animating = powered;
                if powered {
                    a.count += 1;
                }
            }
            if ticks == 0 {
                a.animating = powered;
            }
            if !in_view(p, cam) {
                continue;
            }
            let anim = if a.animating {
                a.count as f32 + pt
            } else {
                a.count as f32
            };
            let origin = Vec3::new(
                (f64::from(p[0]) - cam[0]) as f32,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32,
            );
            let flip = Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0));
            let root = if wall {
                let Some((yrot, sx, sz)) = s.property("facing").and_then(facing_yrot) else {
                    continue;
                };
                // WallAndGround: translate(0.5 − stepX·0.25, 0.25, 0.5 − stepZ·0.25), rotY(−opposite.yRot).
                let opp = (yrot + 180.0) % 360.0;
                Mat4::from_translation(origin + Vec3::new(0.5 - sx * 0.25, 0.25, 0.5 - sz * 0.25))
                    * Mat4::from_rotation_y((-opp).to_radians())
                    * flip
            } else {
                let seg: i32 = s
                    .property("rotation")
                    .and_then(|r| r.parse().ok())
                    .unwrap_or(0);
                let d = (seg & 15) as f32 * 22.5;
                let d = if d >= 180.0 { d - 360.0 } else { d };
                Mat4::from_translation(origin + Vec3::new(0.5, 0.0, 0.5))
                    * Mat4::from_rotation_y((-d).to_radians())
                    * flip
            };
            let mi = match kind {
                Kind::Skeleton | Kind::WitherSkeleton | Kind::Creeper => 0,
                Kind::Player | Kind::Zombie => 1,
                Kind::Dragon => 2,
                Kind::Piglin => 3,
            };
            let m = &mut self.models[mi];
            m.reset_pose();
            match kind {
                Kind::Dragon => {
                    // VANILLA: DragonHeadModel#setupAnim; PartPose#scaled(0.75) на голове.
                    m.parts[0].scale = Vec3::splat(0.75);
                    m.parts[1].pose.x_rot = ((anim * PI * 0.2).sin() + 1.0) * 0.2;
                }
                Kind::Piglin => {
                    // VANILLA: PiglinHeadModel#setupAnim (уши).
                    m.parts[1].pose.z_rot = -((anim * PI * 0.2 * 1.2).cos() + 2.5) * 0.2;
                    m.parts[2].pose.z_rot = ((anim * PI * 0.2).cos() + 2.5) * 0.2;
                }
                _ => {}
            }
            let light = crate::particles::light_at(
                level,
                f64::from(p[0]),
                f64::from(p[1]),
                f64::from(p[2]),
            );
            m.bake(root, [255; 4], light, mobs.out(tex));
        }
    }
}
