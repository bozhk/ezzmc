//! Рендер мобов: таблица тип → (модель, текстура, масштаб), анимация по движению, отдельный
//! `ParticleDraw` на каждую текстуру (создаётся лениво). Сейчас: гуманоиды (зомби, скелеты).
//! VANILLA: LivingEntityRenderer#submit, ZombieRenderer, SkeletonRenderer, HumanoidModel#setupAnim.

use crate::avatar::AvatarAnim;
use ezzmc_render::animals;
use ezzmc_render::entitymodel::Model;
use ezzmc_render::humanoid::{self, HumanoidParts};
use ezzmc_render::particledraw::ParticleDraw;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::sync::Mutex;

/// Имена записей реестров вариантов (порядок = holder id): `cat_variant`, `frog_variant`.
pub static VARIANT_REGS: Mutex<Vec<(String, Vec<String>)>> = Mutex::new(Vec::new());

fn variant_name(reg: &str, id: i32) -> Option<String> {
    let g = VARIANT_REGS.lock().ok()?;
    let (_, names) = g.iter().find(|(r, _)| r == reg)?;
    let n = names.get(usize::try_from(id).ok()?)?;
    Some(n.strip_prefix("minecraft:").unwrap_or(n).to_owned())
}

/// Путь текстуры с 'static-временем жизни (набор вариантов конечен, строки интернируются).
fn intern(s: String) -> &'static str {
    static POOL: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
    let Ok(mut g) = POOL.lock() else {
        return Box::leak(s.into_boxed_str());
    };
    if let Some(x) = g.iter().find(|x| **x == s) {
        return x;
    }
    let x: &'static str = Box::leak(s.into_boxed_str());
    g.push(x);
    x
}

/// Текстура по варианту/окрасу из метаданных.
/// VANILLA: CatRenderer (CatVariant#assetInfo), HorseRenderer (Variant), LlamaRenderer (Llama.Variant),
/// FrogRenderer (FrogVariant), RabbitRenderer (Rabbit.Variant). Индексы метаданных: Cat#20, Horse#19,
/// Llama#21, Frog#18, Rabbit#18 (счёт defineId по цепочке классов 26.3).
fn variant_tex(name: &str, e: &ezzmc_world::entities::Entity) -> Option<&'static str> {
    Some(match name {
        "cat" => intern(format!(
            "cat/cat_{}",
            variant_name("minecraft:cat_variant", e.int_meta(20)?)?
        )),
        "frog" => intern(format!(
            "frog/frog_{}",
            variant_name("minecraft:frog_variant", e.int_meta(18)?)?
        )),
        "horse" => {
            let v = e.int_meta(19)? & 0xFF;
            let c = [
                "white",
                "creamy",
                "chestnut",
                "brown",
                "black",
                "gray",
                "darkbrown",
            ];
            intern(format!("horse/horse_{}", c.get(usize::try_from(v).ok()?)?))
        }
        "llama" | "trader_llama" => {
            let c = ["creamy", "white", "brown", "gray"];
            intern(format!(
                "llama/llama_{}",
                c.get(usize::try_from(e.int_meta(21)?).ok()?)?
            ))
        }
        "rabbit" => {
            let c = ["brown", "white", "black", "white_splotched", "gold", "salt"];
            let v = e.int_meta(18)?;
            if v == 99 {
                "rabbit/rabbit_caerbannog"
            } else {
                intern(format!(
                    "rabbit/rabbit_{}",
                    c.get(usize::try_from(v).ok()?)?
                ))
            }
        }
        // VANILLA: Cow#DATA_VARIANT_ID (#18, holder cow_variant) → CowVariant#modelAndTexture.
        "cow" => intern(format!(
            "cow/cow_{}",
            variant_name("minecraft:cow_variant", e.int_meta(18)?)?
        )),
        _ => return None,
    })
}

/// Модель коровы по варианту (VANILLA: CowRenderer#submit, CowVariant.ModelType).
fn cow_kind(e: &ezzmc_world::entities::Entity) -> Kind {
    match e
        .int_meta(18)
        .and_then(|v| variant_name("minecraft:cow_variant", v))
        .as_deref()
    {
        Some("warm") => Kind::CowWarm,
        Some("cold") => Kind::CowCold,
        _ => Kind::Cow,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Kind {
    Zombie,
    Skeleton,
    Pig,
    Cow,
    CowWarm,
    CowCold,
    Creeper,
    Spider,
    Chicken,
    Sheep,
    Enderman,
    IronGolem,
    Blaze,
    Slime,
    MagmaCube,
    Ghast,
    Silverfish,
    Endermite,
    Villager,
    Wolf,
    Feline,
    Piglin,
    ZPiglin,
    Drowned,
    ZVillager,
    Witch,
    Illager,
    PolarBear,
    Rabbit,
    Squid,
    Fox,
    Turtle,
    Panda,
    Horse,
    Donkey,
    Goat,
    Llama,
    SnowGolem,
    Camel,
    Cod,
    Salmon,
    Dolphin,
    Hoglin,
    Guardian,
    Phantom,
    Ravager,
    Strider,
    Vex,
    Bee,
    Parrot,
    Bat,
    Frog,
    Tadpole,
    Boat,
    ChestBoat,
    Raft,
    ChestRaft,
}

#[derive(Clone, Copy)]
struct Spec {
    tex: &'static str,
    kind: Kind,
    scale: f32,
    /// Радиус тени (до масштаба). VANILLA: EntityRenderer#shadowRadius.
    shadow: f32,
    /// Масштаб самой модели (MeshTransformer.scaling), без сдвига −1.501.
    mscale: f32,
    /// Блочный свет 15 (BlazeRenderer/MagmaCubeRenderer#getBlockLightLevel).
    lit: bool,
}

/// Тип сущности → как рисовать. VANILLA: EntityRenderers (+ масштабы рендереров).
fn spec(name: &str) -> Option<Spec> {
    let (tex, kind, scale, shadow) = match name {
        // ZombieRenderer; HuskRenderer: scale 1.0625; GiantMobRenderer: scale 6.
        "zombie" => ("zombie/zombie", Kind::Zombie, 1.0, 0.5),
        "husk" => ("zombie/husk", Kind::Zombie, 1.0625, 0.5),
        "giant" => ("zombie/zombie", Kind::Zombie, 6.0, 0.5),
        // SkeletonRenderer; WitherSkeletonRenderer: scale 1.2.
        "skeleton" => ("skeleton/skeleton", Kind::Skeleton, 1.0, 0.5),
        "stray" => ("skeleton/stray", Kind::Skeleton, 1.0, 0.5),
        "wither_skeleton" => ("skeleton/wither_skeleton", Kind::Skeleton, 1.2, 0.5),
        // PigRenderer 0.7; CowRenderer 0.7; CreeperRenderer 0.5; SpiderRenderer 0.8
        // (CaveSpiderRenderer: scale 0.7, shadow 0.56); ChickenRenderer 0.3.
        // VANILLA-UNSURE: вариант (temperate/cold/warm) берётся из метаданных — пока всегда temperate.
        "pig" => ("pig/pig_temperate", Kind::Pig, 1.0, 0.7),
        "cow" => ("cow/cow_temperate", Kind::Cow, 1.0, 0.7),
        "creeper" => ("creeper/creeper", Kind::Creeper, 1.0, 0.5),
        "spider" => ("spider/spider", Kind::Spider, 1.0, 0.8),
        "cave_spider" => ("spider/cave_spider", Kind::Spider, 0.7, 0.8),
        // SheepRenderer 0.7; слои шерсти — в bake().
        "sheep" => ("sheep/sheep", Kind::Sheep, 1.0, 0.7),
        "chicken" => ("chicken/chicken_temperate", Kind::Chicken, 1.0, 0.3),
        // MooshroomRenderer: модель коровы (грибы на спине — пока нет; вариант red по умолчанию).
        "mooshroom" => ("cow/mooshroom_red", Kind::Cow, 1.0, 0.7),
        // EndermanRenderer 0.5; IronGolemRenderer 0.7; BlazeRenderer 0.5; AbstractCubeMobRenderer
        // 0.25×size (размер — в bake); GhastRenderer 1.5 при масштабе модели 4.5.
        "enderman" => ("enderman/enderman", Kind::Enderman, 1.0, 0.5),
        "iron_golem" => ("iron_golem/iron_golem", Kind::IronGolem, 1.0, 0.7),
        "blaze" => ("blaze/blaze", Kind::Blaze, 1.0, 0.5),
        "slime" => ("slime/slime", Kind::Slime, 1.0, 0.25),
        "magma_cube" => ("slime/magmacube", Kind::MagmaCube, 1.0, 0.25),
        "ghast" => ("ghast/ghast", Kind::Ghast, 1.0, 1.5),
        // SilverfishRenderer/EndermiteRenderer 0.3; VillagerRenderer 0.5 (профессия/биом — слои, пока нет);
        // WolfRenderer 0.5 (VANILLA-UNSURE: вариант/приручение/гнев не читаются — базовая текстура).
        "silverfish" => ("silverfish/silverfish", Kind::Silverfish, 1.0, 0.3),
        "endermite" => ("endermite/endermite", Kind::Endermite, 1.0, 0.3),
        "villager" => ("villager/villager", Kind::Villager, 1.0, 0.5),
        "wolf" => ("wolf/wolf", Kind::Wolf, 1.0, 0.5),
        // OcelotRenderer/CatRenderer 0.4; кот — модель × 0.8 (CAT_TRANSFORMER), вариант пока tabby.
        "ocelot" => ("cat/ocelot", Kind::Feline, 1.0, 0.4),
        "cat" => ("cat/cat_tabby", Kind::Feline, 1.0, 0.4),
        // PiglinRenderer/ZombifiedPiglinRenderer 0.5.
        "drowned" => ("zombie/drowned", Kind::Drowned, 1.0, 0.5),
        "zombie_villager" => ("zombie_villager/zombie_villager", Kind::ZVillager, 1.0, 0.5),
        "witch" => ("witch/witch", Kind::Witch, 1.0, 0.5),
        "vindicator" => ("illager/vindicator", Kind::Illager, 1.0, 0.5),
        "pillager" => ("illager/pillager", Kind::Illager, 1.0, 0.5),
        "evoker" => ("illager/evoker", Kind::Illager, 1.0, 0.5),
        "illusioner" => ("illager/illusioner", Kind::Illager, 1.0, 0.5),
        "polar_bear" => ("bear/polarbear", Kind::PolarBear, 1.0, 0.9),
        // VANILLA-UNSURE: вариант кролика (Rabbit.Variant) — пока всегда brown.
        "rabbit" => ("rabbit/rabbit_brown", Kind::Rabbit, 1.0, 0.3),
        "squid" => ("squid/squid", Kind::Squid, 1.0, 0.7),
        "glow_squid" => ("squid/glow_squid", Kind::Squid, 1.0, 0.7),
        // VANILLA-UNSURE: вариант лисы (red/snow) и сон — пока всегда red.
        "fox" => ("fox/fox", Kind::Fox, 1.0, 0.4),
        "turtle" => ("turtle/turtle", Kind::Turtle, 1.0, 0.7),
        // VANILLA-UNSURE: ген панды (текстуры brown/lazy/…) — пока обычная.
        "panda" => ("panda/panda", Kind::Panda, 1.0, 0.9),
        // VANILLA-UNSURE: окрас лошади (Variant) и маркировки — пока всегда brown без маркировок.
        "horse" => ("horse/horse_brown", Kind::Horse, 1.0, 0.75),
        "skeleton_horse" => ("horse/horse_skeleton", Kind::Horse, 1.0, 0.75),
        "zombie_horse" => ("horse/horse_zombie", Kind::Horse, 1.0, 0.75),
        "donkey" => ("horse/donkey", Kind::Donkey, 1.0, 0.75),
        "mule" => ("horse/mule", Kind::Donkey, 1.0, 0.75),
        "goat" => ("goat/goat", Kind::Goat, 1.0, 0.7),
        // VANILLA-UNSURE: окрас ламы (Llama.Variant) — brown; у торговой — creamy.
        "llama" => ("llama/llama_brown", Kind::Llama, 1.0, 0.7),
        "trader_llama" => ("llama/llama_creamy", Kind::Llama, 1.0, 0.7),
        "snow_golem" => ("snow_golem/snow_golem", Kind::SnowGolem, 1.0, 0.5),
        "camel" => ("camel/camel", Kind::Camel, 1.0, 0.7),
        "camel_husk" => ("camel/camel_husk", Kind::Camel, 1.0, 0.7),
        "cod" => ("fish/cod", Kind::Cod, 1.0, 0.3),
        "salmon" => ("fish/salmon", Kind::Salmon, 1.0, 0.4),
        "dolphin" => ("dolphin/dolphin", Kind::Dolphin, 1.0, 0.3),
        "hoglin" => ("hoglin/hoglin", Kind::Hoglin, 1.0, 0.7),
        "zoglin" => ("hoglin/zoglin", Kind::Hoglin, 1.0, 0.7),
        "guardian" => ("guardian/guardian", Kind::Guardian, 1.0, 0.5),
        "elder_guardian" => ("guardian/guardian_elder", Kind::Guardian, 1.0, 1.2),
        "phantom" => ("phantom/phantom", Kind::Phantom, 1.0, 0.75),
        "ravager" => ("illager/ravager", Kind::Ravager, 1.0, 1.1),
        "strider" => ("strider/strider", Kind::Strider, 1.0, 0.5),
        "vex" => ("illager/vex", Kind::Vex, 1.0, 0.3),
        "bee" => ("bee/bee", Kind::Bee, 1.0, 0.4),
        "parrot" => ("parrot/parrot_red_blue", Kind::Parrot, 1.0, 0.3),
        // VANILLA-UNSURE: вариант лягушки (Frog#variant) — пока temperate; прыжок/кваканье/язык и isInWater не читаются.
        "frog" => ("frog/frog_temperate", Kind::Frog, 1.0, 0.3),
        "tadpole" => ("tadpole/tadpole", Kind::Tadpole, 1.0, 0.14),
        "bat" => ("bat/bat", Kind::Bat, 1.0, 0.25),
        "wandering_trader" => (
            "wandering_trader/wandering_trader",
            Kind::Villager,
            1.0,
            0.5,
        ),
        "piglin" => ("piglin/piglin", Kind::Piglin, 1.0, 0.5),
        "piglin_brute" => ("piglin/piglin_brute", Kind::Piglin, 1.0, 0.5),
        "zombified_piglin" => ("piglin/zombified_piglin", Kind::ZPiglin, 1.0, 0.5),
        _ => {
            let (tex, kind) = boat_info(name)?;
            (tex, kind, 1.0, 0.8)
        }
    };
    let mscale = match name {
        "ghast" => 4.5,
        "cat" => 0.8,
        "polar_bear" => 1.2,
        "donkey" => 0.87,
        "mule" => 0.92,
        "elder_guardian" => 2.35,
        _ => 1.0,
    };
    Some(Spec {
        tex,
        kind,
        scale,
        shadow,
        mscale,
        // GlowSquidRenderer#getBlockLightLevel: 15 пока darkTicksRemaining = 0.
        lit: matches!(kind, Kind::Blaze | Kind::MagmaCube) || name == "glow_squid",
    })
}

/// Тип лодки → (текстура, модель). VANILLA: EntityRenderers (BoatRenderer/RaftRenderer),
/// ModelLayers.*_BOAT/*_CHEST_BOAT/BAMBOO_RAFT; текстуры `entity/boat|chest_boat/<дерево>`.
fn boat_info(name: &str) -> Option<(&'static str, Kind)> {
    macro_rules! t {
        ($($w:literal),*) => {
            match name {
                $(concat!($w, "_boat") => (concat!("boat/", $w), Kind::Boat),
                  concat!($w, "_chest_boat") => (concat!("chest_boat/", $w), Kind::ChestBoat),)*
                "bamboo_raft" => ("boat/bamboo", Kind::Raft),
                "bamboo_chest_raft" => ("chest_boat/bamboo", Kind::ChestRaft),
                _ => return None,
            }
        };
    }
    Some(t!(
        "oak", "spruce", "birch", "jungle", "acacia", "dark_oak", "cherry", "pale_oak", "mangrove",
        "poplar"
    ))
}

/// Ключ буфера водяной заплатки (только глубина); рисуется после всех моделей.
pub const MASK_TEX: &str = "\0water_mask";

#[derive(Default)]
pub struct Mobs {
    patch: Option<Model>,
    models: HashMap<Kind, Rig>,
    anims: HashMap<i32, MobAnim>,
    draws: HashMap<&'static str, Option<ParticleDraw>>,
    data: HashMap<&'static str, Vec<u8>>,
    last_tick: Option<i64>,
}

enum Rig {
    Human(Model, HumanoidParts),
    Plain(Model),
    /// Овца: тело + модель шерсти (SheepFurModel).
    Sheep(Model, Model),
    /// Слайм: ядро + внешний полупрозрачный слой.
    Slime(Model, Model),
    Enderman(Model, HumanoidParts),
}

/// Состояние анимации одного моба: ходьба/поворот + взмахи курицы.
#[derive(Default)]
struct MobAnim {
    av: AvatarAnim,
    flap: f32,
    o_flap: f32,
    flap_speed: f32,
    o_flap_speed: f32,
    flapping: f32,
    chicken_init: bool,
    swell: f32,
    o_swell: f32,
    squish: f32,
    o_squish: f32,
    target_squish: f32,
    was_on_ground: bool,
    g_tail: f32,
    g_tail_o: f32,
    g_tail_speed: f32,
    g_spikes: f32,
    g_spikes_o: f32,
    g_rng: u32,
    // Squid#aiStep
    sq_init: bool,
    sq_reset: u32,
    sq_move: f32,
    sq_speed: f32,
    sq_angle: f32,
    sq_angle_o: f32,
    sq_rot_speed: f32,
    sq_x: f32,
    sq_x_o: f32,
    sq_z: f32,
    sq_z_o: f32,
    sq_y: f32,
    sq_y_o: f32,
    /// AbstractBoat#paddlePositions (левое/правое весло).
    paddle: [f32; 2],
}

impl MobAnim {
    /// VANILLA: Creeper#tick — swell += swellDir, 0..maxSwell(30).
    fn tick_creeper(&mut self, dir: i32) {
        self.o_swell = self.swell;
        self.swell = (self.swell + dir as f32).clamp(0.0, 30.0);
    }

    /// VANILLA: Guardian#aiStep (клиентская часть: хвост и шипы).
    fn tick_guardian(&mut self, in_water: bool, moving: bool) {
        if self.g_rng == 0 {
            self.g_rng = 0x9E37_79B9;
        }
        self.g_tail_o = self.g_tail;
        if !in_water {
            self.g_tail_speed = 2.0;
        } else if moving {
            if self.g_tail_speed < 0.5 {
                self.g_tail_speed = 4.0;
            } else {
                self.g_tail_speed += (0.5 - self.g_tail_speed) * 0.1;
            }
        } else {
            self.g_tail_speed += (0.125 - self.g_tail_speed) * 0.2;
        }
        self.g_tail += self.g_tail_speed;
        self.g_spikes_o = self.g_spikes;
        if !in_water {
            // random.nextFloat(): xorshift32.
            self.g_rng ^= self.g_rng << 13;
            self.g_rng ^= self.g_rng >> 17;
            self.g_rng ^= self.g_rng << 5;
            self.g_spikes = (self.g_rng >> 8) as f32 / (1u32 << 24) as f32;
        } else if moving {
            self.g_spikes += (0.0 - self.g_spikes) * 0.25;
        } else {
            self.g_spikes += (1.0 - self.g_spikes) * 0.06;
        }
    }

    /// VANILLA: Squid#aiStep (клиентская часть: deltaMovement = e.vel из пакетов).
    // VANILLA-UNSURE: tentacleSpeed на клиенте не меняется (рандом сервера) — фиксируем средним 0.133.
    fn tick_squid(&mut self, e: &ezzmc_world::entities::Entity, in_water: bool) {
        use std::f32::consts::PI;
        if !self.sq_init {
            self.sq_init = true;
            self.sq_y = e.yaw;
            self.sq_y_o = e.yaw;
            self.sq_speed = 1.0 / (0.5 + 1.0) * 0.2;
            self.sq_reset = e.squid_reset;
        }
        if self.sq_reset != e.squid_reset {
            self.sq_reset = e.squid_reset;
            self.sq_move = 0.0;
        }
        self.sq_x_o = self.sq_x;
        self.sq_z_o = self.sq_z;
        self.sq_y_o = self.sq_y;
        self.sq_angle_o = self.sq_angle;
        self.sq_move += self.sq_speed;
        if self.sq_move > PI * 2.0 {
            self.sq_move = PI * 2.0;
        }
        if in_water {
            if self.sq_move < PI {
                let s = self.sq_move / PI;
                self.sq_angle = (s * s * PI).sin() * PI * 0.25;
                if s > 0.75 {
                    self.sq_rot_speed = 1.0;
                } else {
                    self.sq_rot_speed *= 0.8;
                }
            } else {
                self.sq_angle = 0.0;
                self.sq_rot_speed *= 0.99;
            }
            // VANILLA: deltaMovement приходит set_entity_motion и держится до следующего пакета.
            let m = [e.vel[0] as f32, e.vel[1] as f32, e.vel[2] as f32];
            let hz = (m[0] * m[0] + m[2] * m[2]).sqrt();
            let deg = 180.0 / PI;
            // Без движения atan2(0,0)=0 — как в ванили (Mth.atan2 даёт 0).
            // Только f64::atan2: f32::atan2 → atan2f зацикливается из-за заглушки glibc_compat.rs (см. STATUS → Баги).
            let a_y = f64::from(m[0]).atan2(f64::from(m[2])) as f32;
            let a_x = f64::from(hz).atan2(f64::from(m[1])) as f32;
            self.sq_y += (-a_y * deg - self.sq_y) * 0.1;
            self.sq_z += PI * self.sq_rot_speed * 1.5;
            self.sq_x += (-a_x * deg - self.sq_x) * 0.1;
        } else {
            self.sq_angle = self.sq_move.sin().abs() * PI * 0.25;
            self.sq_x += (-90.0 - self.sq_x) * 0.02;
        }
    }

    /// VANILLA: AbstractCubeMob#tick (сжатие при прыжке/приземлении).
    fn tick_cube(&mut self, on_ground: bool) {
        self.o_squish = self.squish;
        self.squish += (self.target_squish - self.squish) * 0.5;
        if on_ground && !self.was_on_ground {
            self.target_squish = -0.5;
        } else if !on_ground && self.was_on_ground {
            self.target_squish = 1.0;
        }
        self.was_on_ground = on_ground;
        self.target_squish *= 0.6;
    }

    /// VANILLA: Chicken#aiStep (flap/flapSpeed/flapping).
    fn tick_chicken(&mut self, on_ground: bool) {
        if !self.chicken_init {
            self.chicken_init = true;
            self.flapping = 1.0;
        }
        self.o_flap = self.flap;
        self.o_flap_speed = self.flap_speed;
        self.flap_speed += (if on_ground { -1.0 } else { 4.0 }) * 0.3;
        self.flap_speed = self.flap_speed.clamp(0.0, 1.0);
        if !on_ground && self.flapping < 1.0 {
            self.flapping = 1.0;
        }
        self.flapping *= 0.9;
        self.flap += self.flapping * 2.0;
    }
}

/// VANILLA: CodRenderer/SalmonRenderer#setupRotations — виляние корпуса вокруг Y (в воде).
// VANILLA-UNSURE: isInWater не отслеживается — всегда «в воде» (на суше рыба не ложится на бок).
fn fish_wiggle(kind: Kind, age: f32, in_water: bool) -> Mat4 {
    // VANILLA: CodRenderer/SalmonRenderer#setupRotations — на суше рыба ложится на бок (Z 90°) и бьётся.
    match kind {
        Kind::Cod => {
            let w = Mat4::from_rotation_y((4.3 * (0.6 * age).sin()).to_radians());
            if in_water {
                w
            } else {
                w * Mat4::from_translation(Vec3::new(0.1, 0.1, -0.1))
                    * Mat4::from_rotation_z(90f32.to_radians())
            }
        }
        Kind::Salmon => {
            let (amp, ang) = if in_water { (1.0, 1.0) } else { (1.3, 1.7) };
            let w = Mat4::from_rotation_y((amp * 4.3 * (ang * 0.6 * age).sin()).to_radians());
            if in_water {
                w
            } else {
                w * Mat4::from_translation(Vec3::new(0.2, 0.1, 0.0))
                    * Mat4::from_rotation_z(90f32.to_radians())
            }
        }
        _ => Mat4::IDENTITY,
    }
}

/// VANILLA: PhantomRenderer#scale — размер и сдвиг; size = Phantom#16 (INT, в swell_dir).
/// VANILLA: LivingEntityRenderer#setupRotations — deathTime>0: ZP на sqrt((t−1)/20·1.6)·flip (90°, паук 180°).
fn death_flip(kind: Kind, death_at: Option<std::time::Instant>) -> Mat4 {
    let Some(t) = death_at else {
        return Mat4::IDENTITY;
    };
    let ticks = t.elapsed().as_secs_f32() * 20.0;
    let f = ((ticks - 1.0).max(0.0) / 20.0 * 1.6).sqrt().min(1.0);
    let flip = if matches!(kind, Kind::Spider) {
        180.0
    } else {
        90.0
    };
    Mat4::from_rotation_z((f * flip).to_radians())
}

fn is_boat(k: Kind) -> bool {
    matches!(
        k,
        Kind::Boat | Kind::ChestBoat | Kind::Raft | Kind::ChestRaft
    )
}

fn phantom_pre(kind: Kind, size: i32) -> Mat4 {
    if kind != Kind::Phantom {
        return Mat4::IDENTITY;
    }
    let s = 1.0 + 0.15 * size.max(0) as f32;
    Mat4::from_scale(Vec3::splat(s)) * Mat4::from_translation(Vec3::new(0.0, 1.3125, 0.1875))
}

/// VANILLA: PhantomRenderer#setupRotations — наклон по pitch.
fn phantom_pitch(kind: Kind, pitch: f32) -> Mat4 {
    if kind == Kind::Phantom {
        Mat4::from_rotation_x(pitch.to_radians())
    } else {
        Mat4::IDENTITY
    }
}

fn build(kind: Kind) -> Rig {
    match kind {
        Kind::Zombie => {
            let (m, p) = humanoid::humanoid_mesh();
            Rig::Human(m, p)
        }
        Kind::Piglin | Kind::ZPiglin => {
            let (m, p) = animals::piglin_mesh();
            Rig::Human(m, p)
        }
        Kind::Drowned => {
            let (m, p) = animals::drowned_mesh();
            Rig::Human(m, p)
        }
        Kind::ZVillager => {
            let (m, p) = animals::zombie_villager_mesh();
            Rig::Human(m, p)
        }
        Kind::Illager => {
            let (m, p) = animals::illager_mesh();
            Rig::Human(m, p)
        }
        Kind::Witch => Rig::Plain(animals::witch_mesh()),
        Kind::Skeleton => {
            let (m, p) = humanoid::skeleton_mesh(0.0);
            Rig::Human(m, p)
        }
        Kind::PolarBear => Rig::Plain(animals::polar_bear_mesh()),
        Kind::Rabbit => Rig::Plain(animals::rabbit_mesh()),
        Kind::Squid => Rig::Plain(animals::squid_mesh()),
        Kind::Fox => Rig::Plain(animals::fox_mesh()),
        Kind::Turtle => Rig::Plain(animals::turtle_mesh()),
        Kind::Panda => Rig::Plain(animals::panda_mesh()),
        Kind::SnowGolem => Rig::Plain(animals::snow_golem_mesh()),
        Kind::Camel => Rig::Plain(animals::camel_mesh()),
        Kind::Cod => Rig::Plain(animals::cod_mesh()),
        Kind::Salmon => Rig::Plain(animals::salmon_mesh()),
        Kind::Dolphin => Rig::Plain(animals::dolphin_mesh()),
        Kind::Hoglin => Rig::Plain(animals::hoglin_mesh()),
        Kind::Guardian => Rig::Plain(animals::guardian_mesh()),
        Kind::Phantom => Rig::Plain(animals::phantom_mesh()),
        Kind::Ravager => Rig::Plain(animals::ravager_mesh()),
        Kind::Strider => Rig::Plain(animals::strider_mesh()),
        Kind::Vex => Rig::Plain(animals::vex_mesh()),
        Kind::Bee => Rig::Plain(animals::bee_mesh()),
        Kind::Parrot => Rig::Plain(animals::parrot_mesh()),
        Kind::Bat => Rig::Plain(animals::bat_mesh()),
        Kind::Frog => Rig::Plain(animals::frog_mesh()),
        Kind::Tadpole => Rig::Plain(animals::tadpole_mesh()),
        Kind::Goat => Rig::Plain(animals::goat_mesh()),
        Kind::Llama => Rig::Plain(animals::llama_mesh()),
        Kind::Horse => Rig::Plain(animals::equine_mesh(false)),
        Kind::Donkey => Rig::Plain(animals::equine_mesh(true)),
        Kind::Pig => Rig::Plain(animals::pig_mesh()),
        Kind::Cow => Rig::Plain(animals::cow_mesh(0)),
        Kind::CowWarm => Rig::Plain(animals::cow_mesh(1)),
        Kind::CowCold => Rig::Plain(animals::cow_mesh(2)),
        Kind::Creeper => Rig::Plain(animals::creeper_mesh()),
        Kind::Spider => Rig::Plain(animals::spider_mesh()),
        Kind::Chicken => Rig::Plain(animals::chicken_mesh()),
        Kind::Enderman => {
            let (m, p) = animals::enderman_mesh();
            Rig::Enderman(m, p)
        }
        Kind::IronGolem => Rig::Plain(animals::iron_golem_mesh()),
        Kind::Blaze => Rig::Plain(animals::blaze_mesh()),
        Kind::Slime => Rig::Slime(animals::slime_inner_mesh(), animals::slime_outer_mesh()),
        Kind::MagmaCube => Rig::Plain(animals::magma_cube_mesh()),
        Kind::Ghast => Rig::Plain(animals::ghast_mesh()),
        Kind::Silverfish => Rig::Plain(animals::silverfish_mesh()),
        Kind::Endermite => Rig::Plain(animals::endermite_mesh()),
        Kind::Villager => Rig::Plain(animals::villager_mesh()),
        Kind::Wolf => Rig::Plain(animals::wolf_mesh()),
        Kind::Feline => Rig::Plain(animals::feline_mesh()),
        Kind::Sheep => Rig::Sheep(animals::sheep_mesh(), animals::sheep_fur_mesh()),
        Kind::Boat => Rig::Plain(animals::boat_mesh(false, false)),
        Kind::ChestBoat => Rig::Plain(animals::boat_mesh(false, true)),
        Kind::Raft => Rig::Plain(animals::boat_mesh(true, false)),
        Kind::ChestRaft => Rig::Plain(animals::boat_mesh(true, true)),
    }
}

impl Mobs {
    /// Запекает всех известных мобов в вершины по текстурам; тени — в `shadow`.
    pub fn bake(&mut self, level: &Level, cam: [f64; 3], sky_darken: i32, shadow: &mut Vec<u8>) {
        for v in self.data.values_mut() {
            v.clear();
        }
        let now = level.game_time;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            _ => 0,
        };
        let pt = level.partial_tick();
        self.anims
            .retain(|id, _| level.entities.map.contains_key(id));
        for e in level.entities.iter() {
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                continue;
            };
            let Some(mut sp) = spec(name) else {
                continue;
            };
            if *name == "cow" {
                sp.kind = cow_kind(e);
            }
            // VANILLA: Entity#isInWater ≈ вода в блоке на середине высоты хитбокса.
            let in_water = matches!(
                sp.kind,
                Kind::Cod | Kind::Salmon | Kind::Guardian | Kind::Squid
            ) && {
                let h = ezzmc_world::entity_size::size(name).map_or(0.5, |s| s.1 as f64 * 0.5);
                ezzmc_mesh::blockinfo::fluid(level.chunks.block(
                    e.pos[0].floor() as i32,
                    (e.pos[1] + h).floor() as i32,
                    e.pos[2].floor() as i32,
                ))
                .is_some_and(|f| !f.lava)
            };
            let anim = self.anims.entry(e.id).or_default();
            for _ in 0..ticks {
                anim.av.tick(e.pos, e.prev_pos, e.head_yaw, e.pitch);
                if is_boat(sp.kind) {
                    // VANILLA: AbstractBoat#tick — гребут, только пока есть управляющий пассажир.
                    let driven = level.riders.vehicle_of.values().any(|v| *v == e.id);
                    for i in 0..2 {
                        anim.paddle[i] = if e.paddle[i] && driven {
                            anim.paddle[i] + std::f32::consts::PI / 8.0
                        } else {
                            0.0
                        };
                    }
                }
                if sp.kind == Kind::Guardian {
                    // #16 BOOLEAN = DATA_ID_MOVING (в поле baby).
                    anim.tick_guardian(in_water, e.baby);
                }
                if sp.kind == Kind::Squid {
                    anim.tick_squid(e, in_water);
                }
                if sp.kind == Kind::Chicken {
                    anim.tick_chicken(e.on_ground);
                }
                if sp.kind == Kind::Creeper {
                    anim.tick_creeper(e.swell_dir);
                }
                if matches!(sp.kind, Kind::Slime | Kind::MagmaCube) {
                    anim.tick_cube(e.on_ground);
                }
            }
            if e.invisible() {
                continue;
            }
            // VANILLA-UNSURE: Entity#shouldRenderAtSqrDistance — 64 × средний размер bbox; размер
            // оцениваем масштабом модели (≥1), чтобы не отсекать крупных мобов раньше ванили.
            let (dx, dy, dz) = (e.pos[0] - cam[0], e.pos[1] - cam[1], e.pos[2] - cam[2]);
            let max_d = 64.0 * f64::from(sp.scale.max(1.0));
            if dx * dx + dy * dy + dz * dz > max_d * max_d {
                continue;
            }
            let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
            let pos = [
                lerp(e.prev_pos[0], e.pos[0]),
                lerp(e.prev_pos[1], e.pos[1]),
                lerp(e.prev_pos[2], e.pos[2]),
            ];
            // Slime#ID_SIZE (INT #16, хранится в swell_dir) — размер куба-моба.
            let cube = matches!(sp.kind, Kind::Slime | Kind::MagmaCube);
            let size = e.swell_dir.max(1) as f32;
            let (mut light, shadow_r) = (
                crate::particles::light_at(
                    level,
                    pos[0],
                    pos[1] + 1.5 * f64::from(sp.scale),
                    pos[2],
                ),
                if cube {
                    size * 0.25
                } else {
                    sp.shadow * sp.scale
                },
            );
            if sp.lit {
                light.0 = 15;
            }
            if is_boat(sp.kind) {
                // VANILLA: AbstractBoatRenderer#submit (+ AbstractBoatModel#setupAnim).
                let yaw = e.prev_yaw + (e.yaw - e.prev_yaw) * pt;
                let root = Mat4::from_translation(Vec3::new(
                    (pos[0] - cam[0]) as f32,
                    (pos[1] - cam[1]) as f32,
                    (pos[2] - cam[2]) as f32,
                )) * Mat4::from_translation(Vec3::new(0.0, 0.375, 0.0))
                    * Mat4::from_rotation_y((180.0 - yaw).to_radians())
                    * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
                    * Mat4::from_rotation_y(90.0_f32.to_radians());
                let driven = level.riders.vehicle_of.values().any(|v| *v == e.id);
                // VANILLA: AbstractBoat#getRowingTime.
                let row = |i: usize| {
                    if e.paddle[i] && driven {
                        let p = anim.paddle[i];
                        let a = p - std::f32::consts::PI / 8.0;
                        a + (p - a) * pt.clamp(0.0, 1.0)
                    } else {
                        0.0
                    }
                };
                let boat_light = crate::particles::light_at(level, pos[0], pos[1] + 0.45, pos[2]);
                let rig = self.models.entry(sp.kind).or_insert_with(|| build(sp.kind));
                if let Rig::Plain(model) = rig {
                    model.reset_pose();
                    animals::boat_anim(model, row(0), row(1));
                    model.bake(
                        root,
                        [255; 4],
                        boat_light,
                        self.data.entry(sp.tex).or_default(),
                    );
                }
                // VANILLA: BoatRenderer#submitTypeAdditions — заплатка, если лодка не под водой.
                let h = 0.5;
                let under = ezzmc_mesh::blockinfo::fluid(level.chunks.block(
                    pos[0].floor() as i32,
                    (pos[1] + h).floor() as i32,
                    pos[2].floor() as i32,
                ))
                .is_some_and(|f| !f.lava);
                if !under && matches!(sp.kind, Kind::Boat | Kind::ChestBoat) {
                    let patch = self.patch.get_or_insert_with(animals::boat_water_patch);
                    patch.bake(
                        root,
                        [255; 4],
                        boat_light,
                        self.data.entry(MASK_TEX).or_default(),
                    );
                }
                crate::entityshadow::bake(level, pos, cam, shadow_r, 1.0, sky_darken, shadow);
                continue;
            }
            let state = anim.av.humanoid_state(pt, false);
            // Guardian#getTailAnimation/getSpikesAnimation = lerp(pt, old, new).
            let gd = (
                anim.g_tail_o + (anim.g_tail - anim.g_tail_o) * pt,
                anim.g_spikes_o + (anim.g_spikes - anim.g_spikes_o) * pt,
            );
            let body = if sp.kind == Kind::Squid {
                let d = (anim.sq_y - anim.sq_y_o + 180.0).rem_euclid(360.0) - 180.0;
                anim.sq_y_o + pt * d
            } else {
                anim.av.body_rot(pt)
            };
            let sq_lerp = |a: f32, b: f32| a + (b - a) * pt;
            let (sq_pre, sq_post) = if sp.kind == Kind::Squid {
                // VANILLA: SquidRenderer#setupRotations.
                let (up, down) = if e.baby { (0.25, -0.6) } else { (0.5, -1.2) };
                (
                    Mat4::from_translation(Vec3::new(0.0, up, 0.0)),
                    Mat4::from_rotation_x(sq_lerp(anim.sq_x_o, anim.sq_x).to_radians())
                        * Mat4::from_rotation_y(sq_lerp(anim.sq_z_o, anim.sq_z).to_radians())
                        * Mat4::from_translation(Vec3::new(0.0, down, 0.0)),
                )
            } else {
                (Mat4::IDENTITY, Mat4::IDENTITY)
            };
            let flap = anim.o_flap + (anim.flap - anim.o_flap) * pt;
            let flap_speed = anim.o_flap_speed + (anim.flap_speed - anim.o_flap_speed) * pt;
            // VANILLA: CreeperRenderer#scale — раздувание перед взрывом.
            let squish = anim.o_squish + (anim.squish - anim.o_squish) * pt;
            let squash = if cube {
                // VANILLA: AbstractCubeMobRenderer#applySizeAndSquish (+ SlimeRenderer#downscaleSlightly).
                let w = 1.0 / (squish / (size * 0.5 + 1.0) + 1.0);
                let down = if sp.kind == Kind::Slime {
                    Mat4::from_scale(Vec3::splat(0.999))
                        * Mat4::from_translation(Vec3::new(0.0, 0.001, 0.0))
                } else {
                    Mat4::IDENTITY
                };
                down * Mat4::from_scale(Vec3::new(w * size, size / w, w * size))
            } else if sp.kind == Kind::Creeper {
                let mut g = (anim.o_swell + (anim.swell - anim.o_swell) * pt) / 28.0;
                let wobble = 1.0 + (g * 100.0).sin() * g * 0.01;
                g = g.clamp(0.0, 1.0);
                g *= g;
                g *= g;
                Mat4::from_scale(Vec3::new(
                    (1.0 + g * 0.4) * wobble,
                    (1.0 + g * 0.1) / wobble,
                    (1.0 + g * 0.4) * wobble,
                ))
            } else {
                Mat4::IDENTITY
            };
            // VANILLA: LivingEntityRenderer — R_y(180−bodyRot)·S(−1,−1,1)·S(scale)·T(0,−1.501,0).
            let root = Mat4::from_translation(Vec3::new(
                (pos[0] - cam[0]) as f32,
                (pos[1] - cam[1]) as f32,
                (pos[2] - cam[2]) as f32,
            )) * sq_pre
                * Mat4::from_rotation_y((180.0 - body).to_radians())
                * sq_post
                * death_flip(sp.kind, e.death_at)
                * fish_wiggle(sp.kind, state.age_in_ticks, in_water)
                * phantom_pitch(sp.kind, e.pitch)
                * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
                * Mat4::from_scale(Vec3::splat(sp.scale))
                * squash
                * phantom_pre(sp.kind, e.swell_dir)
                * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0))
                * Mat4::from_scale(Vec3::splat(sp.mscale));
            // VANILLA: LivingEntityRenderer#getOverlayCoords — красный оверлей, пока hurtTime>0.
            let hurt = e
                .hurt_at
                .is_some_and(|t| t.elapsed().as_secs_f32() < e.hurt_dur);
            let snap: Vec<(&'static str, usize)> = if hurt {
                self.data.iter().map(|(k, v)| (*k, v.len())).collect()
            } else {
                Vec::new()
            };
            let rig = self.models.entry(sp.kind).or_insert_with(|| build(sp.kind));
            // GhastRenderer: при зарядке (Ghast#DATA_IS_CHARGING, bool #16) текстура ghast_shooting.
            let tex = if sp.kind == Kind::Ghast && e.baby {
                "ghast/ghast_shooting"
            } else {
                variant_tex(name, e).unwrap_or(sp.tex)
            };
            let out = self.data.entry(tex).or_default();
            let (yr, xr, wp, ws) = (state.y_rot, state.x_rot, state.walk_pos, state.walk_speed);
            match rig {
                Rig::Human(model, parts) => {
                    humanoid::setup_anim(model, parts, &state);
                    if matches!(sp.kind, Kind::Piglin | Kind::ZPiglin) {
                        animals::piglin_ears(model, state.age_in_ticks, wp, ws);
                    }
                    if matches!(
                        sp.kind,
                        Kind::Zombie | Kind::ZPiglin | Kind::Drowned | Kind::ZVillager
                    ) {
                        // VANILLA: ZombieModel#setupAttackAnimation — руки вперёд (isAggressive = Mob#15 бит 4).
                        humanoid::zombie_arms(model, parts, e.mob_flags & 4 != 0, state.swing);
                    }
                    if sp.kind == Kind::Illager {
                        animals::illager_pose(
                            model,
                            parts,
                            e.mob_flags & 4 != 0,
                            state.swing,
                            state.age_in_ticks,
                            sp.tex == "illager/pillager",
                        );
                    }
                    model.bake(root, [255; 4], light, out);
                }
                Rig::Slime(inner, outer) => {
                    // VANILLA: SlimeRenderer — ядро, затем SlimeOuterLayer (translucent).
                    inner.reset_pose();
                    inner.bake(root, [255; 4], light, out);
                    outer.reset_pose();
                    outer.bake(root, [255; 4], light, out);
                }
                Rig::Enderman(model, parts) => {
                    humanoid::setup_anim(model, parts, &state);
                    animals::enderman_tweak(model, parts, e.bool17);
                    model.bake(root, [255; 4], light, out);
                    // VANILLA: EnderEyesLayer — глаза, полная яркость.
                    model.bake(
                        root,
                        [255; 4],
                        (15, 15),
                        self.data.entry("enderman/enderman_eyes").or_default(),
                    );
                }
                Rig::Sheep(model, fur) => {
                    model.reset_pose();
                    fur.reset_pose();
                    animals::quadruped_anim(model, yr, xr, wp, ws);
                    animals::quadruped_anim(fur, yr, xr, wp, ws);
                    model.bake(root, [255; 4], light, out);
                    let dye = e.byte18 & 15;
                    let col = animals::sheep_color(dye);
                    // VANILLA: SheepWoolUndercoatLayer — не белая и не детёныш; SheepWoolLayer — не острижена.
                    if dye != 0 && !e.baby {
                        fur.bake(
                            root,
                            col,
                            light,
                            self.data.entry("sheep/sheep_wool_undercoat").or_default(),
                        );
                    }
                    if e.byte18 & 16 == 0 {
                        fur.bake(
                            root,
                            col,
                            light,
                            self.data.entry("sheep/sheep_wool").or_default(),
                        );
                    }
                }
                Rig::Plain(model) => {
                    model.reset_pose();
                    match sp.kind {
                        Kind::Creeper => animals::creeper_anim(model, yr, xr, wp, ws),
                        Kind::Spider => animals::spider_anim(model, yr, xr, wp, ws),
                        Kind::IronGolem => animals::iron_golem_anim(model, yr, xr, wp, ws),
                        Kind::Blaze => animals::blaze_anim(model, yr, xr, state.age_in_ticks),
                        Kind::MagmaCube => animals::magma_cube_anim(model, squish),
                        Kind::Ghast => animals::ghast_anim(model, state.age_in_ticks),
                        Kind::Silverfish => animals::silverfish_anim(model, state.age_in_ticks),
                        Kind::Endermite => animals::endermite_anim(model, state.age_in_ticks),
                        Kind::Rabbit => animals::rabbit_anim(model, yr, xr),
                        Kind::Turtle => animals::turtle_anim(model, yr, xr, wp, ws, e.on_ground),
                        Kind::Horse | Kind::Donkey => animals::equine_anim(model, yr, xr, wp, ws),
                        Kind::SnowGolem => animals::snow_golem_anim(model, yr, xr),
                        Kind::Camel => animals::camel_anim(model, yr, xr, wp, ws),
                        Kind::Cod => animals::cod_anim(model, state.age_in_ticks),
                        Kind::Salmon => animals::salmon_anim(model, state.age_in_ticks),
                        Kind::Dolphin => {
                            animals::dolphin_anim(model, yr, xr, state.age_in_ticks, ws > 0.01)
                        }
                        Kind::Hoglin => animals::hoglin_anim(model, yr, wp, ws),
                        Kind::Guardian => {
                            animals::guardian_anim(model, yr, xr, state.age_in_ticks, gd.0, gd.1)
                        }
                        Kind::Phantom => {
                            animals::phantom_anim(model, state.age_in_ticks + e.id as f32 * 3.0)
                        }
                        Kind::Ravager => animals::ravager_anim(model, yr, xr, wp, ws),
                        Kind::Strider => {
                            animals::strider_anim(model, yr, xr, wp, ws, state.age_in_ticks)
                        }
                        Kind::Vex => animals::vex_anim(model, yr, xr, state.age_in_ticks),
                        Kind::Bee => animals::bee_anim(model, state.age_in_ticks, e.on_ground),
                        Kind::Parrot => animals::parrot_anim(
                            model,
                            yr,
                            xr,
                            wp,
                            ws,
                            state.age_in_ticks,
                            !e.on_ground,
                        ),
                        Kind::Frog => animals::frog_anim(model, wp, ws, false),
                        Kind::Tadpole => animals::tadpole_anim(model, state.age_in_ticks),
                        Kind::Bat => {
                            animals::bat_anim(model, yr, state.age_in_ticks, e.byte16 & 1 != 0)
                        }
                        Kind::Fox => animals::fox_anim(model, yr, xr, wp, ws),
                        Kind::Squid => animals::squid_anim(
                            model,
                            anim.sq_angle_o + (anim.sq_angle - anim.sq_angle_o) * pt,
                        ),
                        Kind::Witch => {
                            animals::witch_anim(model, yr, xr, wp, ws, state.age_in_ticks, e.id)
                        }
                        Kind::Villager => animals::villager_anim(model, yr, xr, wp, ws),
                        Kind::Wolf => animals::wolf_anim(model, yr, xr, wp, ws),
                        Kind::Feline => {
                            animals::feline_anim(model, yr, xr, wp, ws, e.flags & 8 != 0)
                        }
                        Kind::Chicken => {
                            animals::chicken_anim(model, yr, xr, wp, ws, flap, flap_speed)
                        }
                        _ => animals::quadruped_anim(model, yr, xr, wp, ws),
                    }
                    model.bake(root, [255; 4], light, out);
                    // VANILLA: SpiderEyesLayer / PhantomEyesLayer — EyesLayer, полная яркость.
                    let eyes = match sp.kind {
                        Kind::Spider => Some("spider/spider_eyes"),
                        Kind::Phantom => Some("phantom/phantom_eyes"),
                        _ => None,
                    };
                    if let Some(t) = eyes {
                        model.bake(root, [255; 4], (15, 15), self.data.entry(t).or_default());
                    }
                }
            }
            if hurt {
                // Флаг оверлея: uv2.x += 4096 у вершин (32 байта, uv2.x по смещению 24), добавленных этим мобом.
                for (k, v) in &mut self.data {
                    let from = snap.iter().find(|(n, _)| n == k).map_or(0, |(_, l)| *l);
                    for vtx in v[from..].chunks_exact_mut(32) {
                        let x = i32::from_le_bytes([vtx[24], vtx[25], vtx[26], vtx[27]]) + 4096;
                        vtx[24..28].copy_from_slice(&x.to_le_bytes());
                    }
                }
            }
            crate::entityshadow::bake(level, pos, cam, shadow_r, 1.0, sky_darken, shadow);
        }
    }

    /// Буфер вершин для текстуры (путь под `textures/entity/`): другие модельные рисовальщики
    /// (сундуки) кладут сюда геометрию после `bake`.
    pub fn out(&mut self, tex: &'static str) -> &mut Vec<u8> {
        self.data.entry(tex).or_default()
    }

    /// Загружает данные в GPU; `make` создаёт `ParticleDraw` по PNG текстуры (путь без префикса/расширения).
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mv: Mat4,
        make: &mut dyn FnMut(&str) -> Option<ParticleDraw>,
    ) {
        for (tex, data) in &self.data {
            if data.is_empty() {
                continue;
            }
            let d = self.draws.entry(tex).or_insert_with(|| make(tex));
            if let Some(d) = d {
                d.prepare_raw(device, queue, mv, data);
            }
        }
        // Текстуры без мобов в этом кадре — очистить, чтобы не рисовать старое.
        for (tex, d) in &mut self.draws {
            if self.data.get(tex).is_none_or(Vec::is_empty) {
                if let Some(d) = d {
                    d.prepare_raw(device, queue, mv, &[]);
                }
            }
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        for (k, d) in &self.draws {
            if *k != MASK_TEX {
                d.iter().for_each(|d| d.draw(pass));
            }
        }
        // Заплатка — после моделей: иначе она закрыла бы дно лодки по глубине.
        if let Some(Some(d)) = self.draws.get(MASK_TEX) {
            d.draw(pass);
        }
    }
}
