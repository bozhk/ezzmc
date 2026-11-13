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
use rustc_hash::FxHashMap as HashMap;
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
/// VANILLA: ShulkerRenderer#setupRotations — bodyRot+180 (минус 180° к общему повороту) и
/// rotateAround(attachFace.opposite.rotation, 0, 0.5, 0). Направления: 0 DOWN 1 UP 2 N 3 S 4 W 5 E.
fn shulker_attach(kind: Kind, face: Option<i32>) -> Mat4 {
    if kind != Kind::Shulker {
        return Mat4::IDENTITY;
    }
    use std::f32::consts::{FRAC_PI_2, PI};
    let opp = face.unwrap_or(0) ^ 1;
    let q = match opp {
        0 => glam::Quat::from_rotation_x(PI),
        2 => glam::Quat::from_euler(glam::EulerRot::XYZ, FRAC_PI_2, 0.0, PI),
        3 => glam::Quat::from_rotation_x(FRAC_PI_2),
        4 => glam::Quat::from_euler(glam::EulerRot::XYZ, FRAC_PI_2, 0.0, FRAC_PI_2),
        5 => glam::Quat::from_euler(glam::EulerRot::XYZ, FRAC_PI_2, 0.0, -FRAC_PI_2),
        _ => glam::Quat::IDENTITY,
    };
    Mat4::from_rotation_y(-PI)
        * Mat4::from_translation(Vec3::new(0.0, 0.5, 0.0))
        * Mat4::from_quat(q)
        * Mat4::from_translation(Vec3::new(0.0, -0.5, 0.0))
}

/// VANILLA: VillagerMetadataSection.Hat (из .png.mcmeta ресурсов `villager/type|profession`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum VHat {
    None,
    Partial,
    Full,
}

fn villager_hat(kind: &str, name: &str) -> VHat {
    match (kind, name) {
        ("profession", "butcher") => VHat::Partial,
        ("profession", "farmer" | "fisherman" | "fletcher" | "librarian" | "shepherd")
        | ("type", "desert" | "snow") => VHat::Full,
        _ => VHat::None,
    }
}

/// VANILLA: VillagerProfessionLayer — тип (биом), профессия, уровень; у детёныша только слой baby/<тип>.
/// `dir` = «villager»/«zombie_villager», `meta` = индекс VillagerData (Villager #19, ZombieVillager #20).
fn villager_layers(
    data: &mut HashMap<&'static str, Vec<u8>>,
    model: &Model,
    root: Mat4,
    light: (u8, u8),
    e: &ezzmc_world::entities::Entity,
    dir: &str,
    meta: u8,
) {
    use ezzmc_protocol::generated::r26_3::{VILLAGER_PROFESSION, VILLAGER_TYPE};
    // По умолчанию (мета не пришла) — plains/none/1.
    let vd = e.int_meta(meta).unwrap_or(2 | (1 << 16));
    let ty = VILLAGER_TYPE
        .get((vd & 0xFF) as usize)
        .copied()
        .unwrap_or("plains");
    let prof = VILLAGER_PROFESSION
        .get(((vd >> 8) & 0xFF) as usize)
        .copied()
        .unwrap_or("none");
    let level = (vd >> 16).clamp(1, 5);
    let prof_hat = villager_hat("profession", prof);
    let type_hat = villager_hat("type", ty);
    // typeHatVisible: профессия без шляпы, либо частичная и тип не «полная».
    let type_hat_visible =
        prof_hat == VHat::None || (prof_hat == VHat::Partial && type_hat != VHat::Full);
    let no_head = |n: &str| !matches!(n, "head" | "nose" | "hat" | "hat_rim");
    let t = intern(format!(
        "{dir}/{}/{ty}",
        if e.baby { "baby" } else { "type" }
    ));
    let tout = data.entry(t).or_default();
    if type_hat_visible {
        model.bake(root, [255; 4], light, tout);
    } else {
        model.bake_parts(root, [255; 4], light, tout, &no_head);
    }
    if prof != "none" && !e.baby {
        let pt = intern(format!("{dir}/profession/{prof}"));
        model.bake(root, [255; 4], light, data.entry(pt).or_default());
        if prof != "nitwit" {
            let lt = intern(format!(
                "{dir}/profession_level/{}",
                ["stone", "iron", "gold", "emerald", "diamond"][(level - 1) as usize]
            ));
            model.bake(root, [255; 4], light, data.entry(lt).or_default());
        }
    }
}

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
        "shulker" => {
            const DYES: [&str; 16] = [
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
            if !e.byte18_set || e.byte18 > 15 {
                "shulker/shulker"
            } else {
                intern(format!("shulker/shulker_{}", DYES[e.byte18 as usize]))
            }
        }
        "axolotl" => intern(format!(
            "axolotl/axolotl_{}{}",
            ["lucy", "wild", "gold", "cyan", "blue"][e.int_meta(18)?.clamp(0, 4) as usize],
            if e.baby { "_baby" } else { "" }
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
        // MushroomCow#DATA_TYPE (int #18): 0 red, 1 brown.
        "mooshroom" => {
            if e.int_meta(18)? == 1 {
                "cow/mooshroom_brown"
            } else {
                "cow/mooshroom_red"
            }
        }
        // Parrot#DATA_VARIANT_ID (int #20): red_blue, blue, green, yellow_blue, grey.
        "parrot" => {
            let c = ["red_blue", "blue", "green", "yellow_blue", "grey"];
            intern(format!(
                "parrot/parrot_{}",
                c.get(usize::try_from(e.int_meta(20)?).ok()?)?
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
pub(crate) enum Kind {
    Zombie,
    Skeleton,
    Stray,
    Bogged,
    Parched,
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
    CopperGolem,
    Camel,
    Cod,
    TropSmall,
    TropLarge,
    PufferSmall,
    PufferMid,
    PufferBig,
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
    Axolotl,
    Allay,
    Armadillo,
    Breeze,
    Warden,
    Wither,
    EnderDragon,
    Shulker,
    /// Только ключ модели: детёныш броненосца (BabyArmadilloModel); в spec() не встречается.
    ArmadilloBaby,
    VillagerBaby,
    ZVillagerBaby,
    SniffletBaby,
    Nautilus,
    NautilusBaby,
    Creaking,
    HappyGhast,
    HappyGhastBaby,
    /// Только ключ модели: детёныш аксолотля (BabyAxolotlModel).
    AxolotlBaby,
    Sniffer,
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

/// `spec` по id типа: огромный `match` по строке не гоняем на каждого моба каждый кадр.
fn spec_cached(ty: usize) -> Option<Spec> {
    static CACHE: std::sync::OnceLock<Vec<Option<Spec>>> = std::sync::OnceLock::new();
    CACHE
        .get_or_init(|| {
            ezzmc_protocol::generated::r26_3::ENTITY_TYPE
                .iter()
                .map(|n| spec(n))
                .collect()
        })
        .get(ty)
        .copied()
        .flatten()
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
        "stray" => ("skeleton/stray", Kind::Stray, 1.0, 0.5),
        "bogged" => ("skeleton/bogged", Kind::Bogged, 1.0, 0.5),
        "parched" => ("skeleton/parched", Kind::Parched, 1.0, 0.5),
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
        // MooshroomRenderer: модель коровы; вариант (red/brown) — variant_tex, грибы — Mobs::bake_mushrooms.
        "mooshroom" => ("cow/mooshroom_red", Kind::Cow, 1.0, 0.7),
        // EndermanRenderer 0.5; IronGolemRenderer 0.7; BlazeRenderer 0.5; AbstractCubeMobRenderer
        // 0.25×size (размер — в bake); GhastRenderer 1.5 при масштабе модели 4.5.
        "enderman" => ("enderman/enderman", Kind::Enderman, 1.0, 0.5),
        "iron_golem" => ("iron_golem/iron_golem", Kind::IronGolem, 1.0, 0.7),
        "blaze" => ("blaze/blaze", Kind::Blaze, 1.0, 0.5),
        "slime" => ("slime/slime", Kind::Slime, 1.0, 0.25),
        "magma_cube" => ("slime/magmacube", Kind::MagmaCube, 1.0, 0.25),
        "ghast" => ("ghast/ghast", Kind::Ghast, 1.0, 1.5),
        // SilverfishRenderer/EndermiteRenderer 0.3; VillagerRenderer 0.5 (слои типа/профессии/уровня — в bake);
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
        // VANILLA: Llama.Variant.DEFAULT = CREAMY (метаданные не шлются, пока вариант по умолчанию).
        "llama" => ("llama/llama_creamy", Kind::Llama, 1.0, 0.7),
        "trader_llama" => ("llama/llama_creamy", Kind::Llama, 1.0, 0.7),
        // CopperGolemRenderer: shadow 0.5; текстура и глаза по WeatherState (#16).
        "copper_golem" => ("copper_golem/copper_golem", Kind::CopperGolem, 1.0, 0.5),
        "snow_golem" => ("snow_golem/snow_golem", Kind::SnowGolem, 1.0, 0.5),
        "camel" => ("camel/camel", Kind::Camel, 1.0, 0.7),
        "camel_husk" => ("camel/camel_husk", Kind::Camel, 1.0, 0.7),
        "cod" => ("fish/cod", Kind::Cod, 1.0, 0.3),
        "salmon" => ("fish/salmon", Kind::Salmon, 1.0, 0.4),
        // PufferfishRenderer: shadow 0.1+0.1·puffState (kind подменяется по метаданным #17).
        // TropicalFishRenderer: shadow 0.15; модель/текстура/цвета — по метаданным #17.
        "tropical_fish" => ("fish/tropical_a", Kind::TropSmall, 1.0, 0.15),
        "pufferfish" => ("fish/pufferfish", Kind::PufferSmall, 1.0, 0.1),
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
        // AxolotlRenderer: shadow 0.5; текстура по варианту (#18) — variant_tex.
        "axolotl" => ("axolotl/axolotl_lucy", Kind::Axolotl, 1.0, 0.5),
        "tadpole" => ("tadpole/tadpole", Kind::Tadpole, 1.0, 0.14),
        // NautilusRenderer / ZombieNautilusRenderer: shadow 0.7. VANILLA-UNSURE: зомби-наутилус берёт
        // temperate-текстуру (вариант warm = ZombieNautilusCoralModel, не портирован).
        // CreakingRenderer: shadow 0.6; глаза (creaking_eyes) — emissive-слой по части head.
        // HappyGhastRenderer: shadow 2.0; модель ×4 (детёныш ×0.95); упряжь/верёвки не портированы.
        "happy_ghast" => ("ghast/happy_ghast", Kind::HappyGhast, 1.0, 2.0),
        "creaking" => ("creaking/creaking", Kind::Creaking, 1.0, 0.6),
        "nautilus" => ("nautilus/nautilus", Kind::Nautilus, 1.0, 0.7),
        "zombie_nautilus" => ("nautilus/zombie_nautilus", Kind::Nautilus, 1.0, 0.7),
        // AllayRenderer: shadow 0.4, блочный свет 15 (getBlockLightLevel).
        "allay" => ("allay/allay", Kind::Allay, 1.0, 0.4),
        "armadillo" => ("armadillo/armadillo", Kind::Armadillo, 1.0, 0.4),
        // BreezeRenderer: shadow 0.5; слои ветра (breeze_wind) и глаз (breeze_eyes).
        // WardenRenderer: shadow 0.9; слои bioluminescent/spots/tendrils/heart.
        "warden" => ("warden/warden", Kind::Warden, 1.0, 0.9),
        // WitherBossRenderer: shadow 1.0 (= 0.5 × scale 2), блочный свет 15, масштаб 2 (−invul/220·0.5), слой брони (energy swirl).
        "wither" => ("wither/wither", Kind::Wither, 2.0, 0.5),
        // EnderDragonRenderer: shadow 0.5, модель без LivingEntity-масштаба/разворота (свой root).
        "ender_dragon" => ("enderdragon/dragon", Kind::EnderDragon, 1.0, 0.5),
        // ShulkerRenderer: shadow 0; текстура по цвету (#18, 16 = по умолчанию).
        "shulker" => ("shulker/shulker", Kind::Shulker, 1.0, 0.0),
        "breeze" => ("breeze/breeze", Kind::Breeze, 1.0, 0.5),
        "sniffer" => ("sniffer/sniffer", Kind::Sniffer, 1.0, 1.1),
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
        "happy_ghast" => 4.0,
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
        lit: matches!(
            kind,
            Kind::Blaze | Kind::MagmaCube | Kind::Allay | Kind::Wither
        ) || name == "glow_squid",
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
/// Текстура слоя заряда крипера — рисуется аддитивно (RenderTypes#energySwirl).
pub const SWIRL_TEX: &str = "creeper/creeper_armor";

/// Ключ статической сетки GPU-скиннинга: моб или сундук (0 одиночный, 1 левый, 2 правый).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SkinKey {
    Mob(Kind),
    Chest(u8),
}

#[derive(Default)]
pub struct Mobs {
    patch: Option<Model>,
    models: HashMap<Kind, Rig>,
    anims: HashMap<i32, MobAnim>,
    draws: HashMap<&'static str, Option<ParticleDraw>>,
    data: HashMap<&'static str, Vec<u8>>,
    /// GPU-скиннинг толпы: сетка на вид, экземпляры (матрицы частей + свет) и рисовальщики по (вид, текстура).
    skin_meshes: HashMap<SkinKey, ezzmc_render::entitymodel::SkinMesh>,
    skin_inst: HashMap<(SkinKey, &'static str), Vec<u8>>,
    skin_draws: HashMap<(SkinKey, &'static str), ezzmc_render::particledraw::SkinnedDraw>,
    last_tick: Option<i64>,
    /// Частицы дракона, накопленные в bake (забирает worldview).
    pub dragon_fx: Vec<DragonFx>,
    /// Грибы на спине mooshroom, собранные за последний `bake` (матрицы относительно камеры).
    mushrooms: Vec<Mushrooms>,
    /// ItemInHandLayer: (стек, матрица руки до сдвига слоя, свет, левая рука) — рисует worldview.
    pub held_items: Vec<(ezzmc_protocol::item::ItemStack, Mat4, (u8, u8), bool)>,
    /// BlockDecorationLayer (медный голем): (имя блока, матрица головы, свет).
    antenna: Vec<(String, Mat4, (u8, u8))>,
    /// События проигрывателя (1010/1011), ещё не розданные попугаям: (блок, играет).
    record_events: Vec<([i32; 3], bool)>,
}

/// VANILLA: MushroomCowMushroomLayer — три гриба; матрицы корня/головы уже без камеры.
struct Mushrooms {
    root: Mat4,
    head: Mat4,
    brown: bool,
    light: (u8, u8),
}

/// VANILLA: EnergySwirlLayer — RenderTypes.energySwirl(tex, u, v), цвет 0x808080, полная яркость, аддитивно.
/// VANILLA-UNSURE: аддитивного блендинга в пайплайне мобов нет — приближение альфой 128.
fn energy_swirl(model: &Model, root: Mat4, out: &mut Vec<u8>, du: f32, dv: f32) {
    let from = out.len();
    model.bake_emissive_parts(root, 128, out, &|_| true);
    for v in out[from..].chunks_exact_mut(32) {
        let u = f32::from_le_bytes([v[12], v[13], v[14], v[15]]) + du;
        let w = f32::from_le_bytes([v[16], v[17], v[18], v[19]]) + dv;
        v[12..16].copy_from_slice(&u.to_le_bytes());
        v[16..20].copy_from_slice(&w.to_le_bytes());
    }
}

/// Типы толпы (свинья/корова/курица, не mooshroom): быстрый параллельный путь в `Mobs::bake`.
fn crowd_table() -> &'static [bool] {
    static T: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .map(|n| {
                *n != "mooshroom"
                    && spec(n)
                        .is_some_and(|s| matches!(s.kind, Kind::Pig | Kind::Cow | Kind::Chicken))
            })
            .collect()
    })
}

/// Моб идёт быстрым путём: тип толпы, не умирает и не красный от урона (их рисует общий цикл).
fn is_crowd(e: &ezzmc_world::entities::Entity, tbl: &[bool]) -> bool {
    tbl.get(e.ty).copied().unwrap_or(false)
        && e.death_at.is_none()
        && !e
            .hurt_at
            .is_some_and(|t| t.elapsed().as_secs_f32() < e.hurt_dur)
}

/// Общие параметры кадра для `crowd_one`.
struct CrowdCtx<'a> {
    level: &'a Level,
    cam: [f64; 3],
    sky_darken: i32,
    frustum: Option<&'a ezzmc_mesh::frustum::Frustum>,
    ticks: i64,
    net_pt: f32,
}

/// Результат параллельной фазы одного потока: отложенные мобы, тени, сколько нарисовано.
#[derive(Default)]
struct CrowdOut {
    jobs: Vec<ParJob>,
    shadow: Vec<u8>,
    drawn: u32,
}

/// VANILLA: LivingEntityRenderer (свинья/корова/курица) — тик анимации, куллинг, свет, root, тень.
/// Та же логика, что в общем цикле `Mobs::bake`, но без исключений для прочих видов; корень
/// `T·R_y(180−body)·S(−1,−1,1)·S(scale·mscale)·T(0,−1.501,0)` собран в одну матрицу.
fn crowd_one(
    c: &CrowdCtx,
    e: &ezzmc_world::entities::Entity,
    anim: &mut MobAnim,
    out: &mut CrowdOut,
) {
    let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
        return;
    };
    let Some(mut sp) = spec_cached(e.ty) else {
        return;
    };
    if *name == "cow" {
        sp.kind = cow_kind(e);
    }
    let pt = if e.local_ctrl {
        c.level.local_ctrl_pt
    } else {
        c.net_pt
    };
    for _ in 0..c.ticks {
        anim.av.tick(e.pos, e.prev_pos, e.head_yaw, e.pitch);
        if sp.kind == Kind::Chicken {
            anim.tick_chicken(e.on_ground);
        }
    }
    if e.invisible() {
        return;
    }
    let (dx, dy, dz) = (
        e.pos[0] - c.cam[0],
        e.pos[1] - c.cam[1],
        e.pos[2] - c.cam[2],
    );
    let max_d = 64.0 * f64::from(sp.scale.max(1.0));
    if dx * dx + dy * dy + dz * dz > max_d * max_d {
        return;
    }
    if let (Some(fr), Some((lo, hi))) = (c.frustum, e.hitbox()) {
        let lo = [lo[0] - 0.5, lo[1] - 0.5, lo[2] - 0.5];
        let hi = [hi[0] + 0.5, hi[1] + 0.5, hi[2] + 0.5];
        if !fr.is_visible(lo, hi) {
            return;
        }
    }
    let lerp = |a: f64, b: f64| a + (b - a) * f64::from(pt);
    let pos = [
        lerp(e.prev_pos[0], e.pos[0]),
        lerp(e.prev_pos[1], e.pos[1]),
        lerp(e.prev_pos[2], e.pos[2]),
    ];
    let mut light =
        crate::particles::light_at(c.level, pos[0], pos[1] + 1.5 * f64::from(sp.scale), pos[2]);
    if sp.lit {
        light.0 = 15;
    }
    let shadow_r = sp.shadow * sp.scale;
    let state = anim.av.humanoid_state(pt, false);
    let body = anim.av.body_rot(pt);
    let flap = anim.o_flap + (anim.flap - anim.o_flap) * pt;
    let flap_speed = anim.o_flap_speed + (anim.flap_speed - anim.o_flap_speed) * pt;
    let (s, co) = (180.0 - body).to_radians().sin_cos();
    let k = sp.scale * sp.mscale;
    let rel = [
        (pos[0] - c.cam[0]) as f32,
        (pos[1] - c.cam[1]) as f32,
        (pos[2] - c.cam[2]) as f32,
    ];
    let root = Mat4::from_cols(
        Vec3::new(co, 0.0, -s).extend(0.0) * -k,
        Vec3::new(0.0, 1.0, 0.0).extend(0.0) * -k,
        Vec3::new(s, 0.0, co).extend(0.0) * k,
        Vec3::new(rel[0], rel[1] + 1.501 * k, rel[2]).extend(1.0),
    );
    let tex = variant_tex(name, e).unwrap_or(sp.tex);
    out.jobs.push(ParJob {
        kind: sp.kind,
        tex,
        flap: (flap, flap_speed),
        root,
        light,
        pose: (state.y_rot, state.x_rot, state.walk_pos, state.walk_speed),
    });
    crate::entityshadow::bake(
        c.level,
        pos,
        c.cam,
        shadow_r,
        1.0,
        c.sky_darken,
        &mut out.shadow,
    );
    out.drawn += 1;
}

/// Отложенный моб для параллельного запекания (`Mobs::bake_parallel`): свиньи, коровы, курицы.
struct ParJob {
    kind: Kind,
    tex: &'static str,
    /// Курица: (flap, flap_speed) — остальным не нужно.
    flap: (f32, f32),
    root: Mat4,
    light: (u8, u8),
    /// (y_rot, x_rot, walk_pos, walk_speed).
    pose: (f32, f32, f32, f32),
}

enum Rig {
    Human(Model, HumanoidParts),
    /// Крипер: тело + модель заряда (CreeperPowerLayer, деформация 2.0).
    Creeper(Model, Model),
    /// Скелет с внешним слоем (SkeletonClothingLayer): база + слой другой сетки (stray 0.25, bogged 0.2).
    Layered(Model, HumanoidParts, Model, HumanoidParts),
    Plain(Model),
    /// Овца: тело + модель шерсти (SheepFurModel).
    Sheep(Model, Model),
    /// Слайм: ядро + внешний полупрозрачный слой.
    Slime(Model, Model),
    /// Бриз: тело, ветер, глаза (BreezeModel ×3 слоя).
    Breeze(Model, Model, Model),
    Warden(Model),
    /// Визер: тело и броня (WitherBossModel с деформацией 0 и 0.5).
    Wither(Model, Model),
    /// Эндер-дракон (EnderDragonModel; глаза рисуются той же моделью).
    Dragon(Model),
    Enderman(Model, HumanoidParts),
    /// Тропическая рыба: тело + слой узора (TropicalFishPatternLayer, деформация 0.01).
    Tropical(Model, Model),
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
    /// Parrot#jukebox + partyParrot (вечеринка, пока рядом играет проигрыватель).
    jukebox: Option<[i32; 3]>,
    party: bool,
    /// Axolotl: BinaryAnimator (10 тиков) playingDead/inWater/onGround/moving: (старое, новое).
    ax: [(i32, i32); 4],
    /// Armadillo: состояние (#18), inStateTicks, счётчик тиков и старты roll_up/roll_out.
    arm_state: i32,
    arm_ticks: u32,
    arm_clock: u32,
    arm_up: Option<u32>,
    arm_out: Option<u32>,
    /// Sniffer: состояние (#18) и тик его начала (AnimationState#startIfStopped).
    sn_state: i32,
    sn_start: u32,
    /// Детёныш-аксолотль: какое из семи AnimationState запущено и тик его старта.
    bx_state: u8,
    bx_start: u32,
    /// Breeze: поза и AnimationState (старт-тик или None).
    br_pose: i32,
    br_idle: Option<u32>,
    br_shoot: Option<u32>,
    /// Медный голем: состояние (#17), старт idle/interaction (тик), тик следующего idle.
    cg_state: i32,
    cg_idle: Option<u32>,
    cg_inter: Option<u32>,
    cg_next: u32,
    br_inhale: Option<u32>,
    br_slide: Option<u32>,
    br_slide_back: Option<u32>,
    br_jump: Option<u32>,
    /// Warden: поза, старты AnimationState (attack, sonic_boom, dig, emerge, roar, sniff),
    /// виденные счётчики entity_event 4/62/61, tendril/heart (текущее и прошлое, 0..10).
    wd_pose: i32,
    wd: [Option<u32>; 6],
    wd_ev: [u32; 3],
    wd_tendril: [i32; 2],
    wd_heart: [i32; 2],
    /// Shulker: Shulker#currentPeekAmount (прошлое, текущее).
    sh_peek: [f32; 2],
    /// Creaking: старты AnimationState (attack, invulnerable, death) и виденные entity_event 4/66.
    ck: [Option<u32>; 3],
    ck_ev: [u32; 2],
    /// Wither: yRotHeads/xRotHeads (боковые головы, градусы).
    wh_y: [f32; 2],
    wh_x: [f32; 2],
    /// EnderDragon: DragonFlightHistory (64 сэмпла y/yRot), head, flapTime (прошлое, текущее).
    dr_hist: Vec<(f64, f32)>,
    dr_head: i32,
    dr_flap: [f32; 2],
    /// Позиция головы с прошлого тика (EnderDragon#tickPart) и фаза/счётчик для doClientTick.
    dr_head_pos: Option<[f64; 3]>,
    dr_phase: i32,
    dr_flame_ticks: i32,
    /// EnderDragonPart: позиции (ноги) 8 частей с последнего тика (EnderDragon#tickPart).
    dr_parts: Option<[[f64; 3]; 8]>,
}

/// VANILLA: EnderDragon#aiStep — tickPart для head/neck/body/tail1..3/wing1/wing2 (yRotA клиента = 0).
fn dragon_part_positions(
    hist: &[(f64, f32)],
    head: i32,
    pos: [f64; 3],
    yaw: f32,
    sitting: bool,
) -> [[f64; 3]; 8] {
    let at = |n: i32| hist[((head - n) & 63) as usize];
    let tilt = ((at(5).0 - at(10).0) as f32) * 10.0 * (std::f32::consts::PI / 180.0);
    let (cc_t, ss_t) = (tilt.cos(), tilt.sin());
    let rot1 = yaw * (std::f32::consts::PI / 180.0);
    let (ss1, cc1) = (rot1.sin(), rot1.cos());
    let y_off = if sitting {
        -1.0
    } else {
        (at(5).0 - at(0).0) as f32
    };
    let off = |x: f64, y: f64, z: f64| [pos[0] + x, pos[1] + y, pos[2] + z];
    let p1 = at(5);
    let tail = |i: i32| {
        let p0 = at(12 + i * 2);
        let mut d = p0.1 - p1.1;
        while d >= 180.0 {
            d -= 360.0;
        }
        while d < -180.0 {
            d += 360.0;
        }
        let rot = rot1 + d * (std::f32::consts::PI / 180.0);
        let (ss, cc) = (rot.sin(), rot.cos());
        let dd = (i + 1) as f32 * 2.0;
        off(
            f64::from(-(ss1 * 1.5 + ss * dd) * cc_t),
            p0.0 - p1.0 - f64::from((dd + 1.5) * ss_t) + 1.5,
            f64::from((cc1 * 1.5 + cc * dd) * cc_t),
        )
    };
    [
        off(
            f64::from(ss1 * 6.5 * cc_t),
            f64::from(y_off + ss_t * 6.5),
            f64::from(-cc1 * 6.5 * cc_t),
        ),
        off(
            f64::from(ss1 * 5.5 * cc_t),
            f64::from(y_off + ss_t * 5.5),
            f64::from(-cc1 * 5.5 * cc_t),
        ),
        off(f64::from(ss1 * 0.5), 0.0, f64::from(-cc1 * 0.5)),
        tail(0),
        tail(1),
        tail(2),
        off(f64::from(cc1 * 4.5), 2.0, f64::from(ss1 * 4.5)),
        off(f64::from(cc1 * -4.5), 2.0, f64::from(ss1 * -4.5)),
    ]
}

/// EnderDragonPart: (ширина, высота) в порядке head, neck, body, tail×3, wing×2.
const DRAGON_PART_SIZE: [(f64, f64); 8] = [
    (1.0, 1.0),
    (3.0, 3.0),
    (5.0, 3.0),
    (2.0, 2.0),
    (2.0, 2.0),
    (2.0, 2.0),
    (4.0, 2.0),
    (4.0, 2.0),
];

/// Всплеск частиц дыхания дракона за тик (DragonLandingPhase / DragonSittingFlamingPhase#doClientTick).
pub struct DragonFx {
    pub head: [f64; 3],
    pub look: [f64; 3],
    /// Some — посадка (с deltaMovement), None — пламя на земле.
    pub motion: Option<[f64; 3]>,
}

impl MobAnim {
    /// VANILLA: EnderDragon#aiStep (клиентская часть) — flapTime и DragonFlightHistory#record.
    fn tick_dragon(
        &mut self,
        e: &ezzmc_world::entities::Entity,
        phase: i32,
        egg_dist: f64,
        fx: &mut Vec<DragonFx>,
    ) {
        let sitting = (5..=7).contains(&phase);
        self.dr_flap[0] = self.dr_flap[1];
        // VANILLA: deltaMovement приходит set_entity_motion (e.vel) и держится до следующего пакета.
        let horiz = (e.vel[0] * e.vel[0] + e.vel[2] * e.vel[2]).sqrt() as f32;
        let mut speed = 0.2 / (horiz * 10.0 + 1.0);
        speed *= 2.0_f32.powf(e.vel[1] as f32);
        // VANILLA-UNSURE: inWall (flapSpeed·0.5) не учитываем.
        self.dr_flap[1] += if sitting { 0.1 } else { speed };
        let mut yaw = e.yaw % 360.0;
        if yaw >= 180.0 {
            yaw -= 360.0;
        } else if yaw < -180.0 {
            yaw += 360.0;
        }
        if e.mob_flags & 1 != 0 {
            self.dr_flap[1] = 0.5;
        } else {
            if self.dr_hist.is_empty() {
                self.dr_hist = vec![(e.pos[1], yaw); 64];
                self.dr_head = -1;
            }
            self.dr_head = (self.dr_head + 1) & 63;
            self.dr_hist[self.dr_head as usize] = (e.pos[1], yaw);
            // VANILLA: phase.doClientTick() идёт до tickPart(head) → берём голову с прошлого тика.
            if phase != self.dr_phase {
                self.dr_phase = phase;
                self.dr_flame_ticks = 0;
            }
            if let Some(head) = self.dr_head_pos {
                let burst = match phase {
                    3 => true,
                    5 => {
                        self.dr_flame_ticks += 1;
                        self.dr_flame_ticks % 2 == 0 && self.dr_flame_ticks < 10
                    }
                    _ => false,
                };
                if burst {
                    // VANILLA: EnderDragon#getHeadLookVector (xRot подменяется на время расчёта).
                    let x_rot = if phase == 3 || phase == 4 {
                        let dist = (egg_dist / 4.0).max(1.0) as f32;
                        -(6.0 / dist) * 1.5 * 5.0
                    } else if sitting {
                        -45.0
                    } else {
                        e.pitch
                    };
                    let (xr, yr) = (x_rot.to_radians(), (-yaw).to_radians());
                    let v = [
                        f64::from(yr.sin() * xr.cos()),
                        f64::from(-xr.sin()),
                        f64::from(yr.cos() * xr.cos()),
                    ];
                    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    fx.push(DragonFx {
                        head,
                        look: [v[0] / n, v[1] / n, v[2] / n],
                        motion: (phase == 3).then_some(e.vel),
                    });
                }
            }
            self.dr_parts = Some(dragon_part_positions(
                &self.dr_hist,
                self.dr_head,
                e.pos,
                yaw,
                sitting,
            ));
            // VANILLA: EnderDragon#aiStep — tickPart(head) (yRotA клиента = 0).
            let h = |n: i32| self.dr_hist[((self.dr_head - n) & 63) as usize].0;
            let tilt = ((h(5) - h(10)) as f32) * 10.0 * (std::f32::consts::PI / 180.0);
            let (cc_tilt, ss_tilt) = (tilt.cos(), tilt.sin());
            let y_off = if sitting { -1.0 } else { (h(5) - h(0)) as f32 };
            let rot = yaw * (std::f32::consts::PI / 180.0);
            self.dr_head_pos = Some([
                e.pos[0] + f64::from(rot.sin() * 6.5 * cc_tilt),
                e.pos[1] + f64::from(y_off + ss_tilt * 6.5),
                e.pos[2] + f64::from(-rot.cos() * 6.5 * cc_tilt),
            ]);
        }
    }

    /// VANILLA: DragonFlightHistory#get(delay, partialTicks) → (y, yRot).
    fn dragon_sample(&self, delay: i32, pt: f32) -> (f64, f32) {
        if self.dr_hist.is_empty() {
            return (0.0, 0.0);
        }
        let at = |d: i32| self.dr_hist[((self.dr_head - d) & 63) as usize];
        let (cur, old) = (at(delay), at(delay + 1));
        let mut d = (cur.1 - old.1) % 360.0;
        if d >= 180.0 {
            d -= 360.0;
        } else if d < -180.0 {
            d += 360.0;
        }
        (old.0 + (cur.0 - old.0) * f64::from(pt), old.1 + pt * d)
    }

    /// VANILLA: WitherBoss#aiStep (клиентская часть; getScale()=1) — головы следят за целями #17/#18.
    fn tick_wither(&mut self, e: &ezzmc_world::entities::Entity, level: &Level, body_rot: f32) {
        fn rotlerp(a: f32, b: f32, max: f32) -> f32 {
            let mut d = (b - a).rem_euclid(360.0);
            if d >= 180.0 {
                d -= 360.0;
            }
            a + d.clamp(-max, max)
        }
        for i in 0..2 {
            let target = e
                .int_meta(17 + i as u8)
                .filter(|&id| id > 0)
                .and_then(|id| level.entities.map.get(&id));
            if let Some(t) = target {
                let a = ((body_rot + 180.0 * i as f32) * std::f32::consts::PI / 180.0) as f64;
                let (hx, hy, hz) = (
                    e.pos[0] + a.cos() * 1.3,
                    e.pos[1] + 2.2,
                    e.pos[2] + a.sin() * 1.3,
                );
                // VANILLA: Entity#getEyeY цели; высоту глаз оцениваем 1.62 (VANILLA-UNSURE для не-игроков).
                let (xd, yd, zd) = (t.pos[0] - hx, t.pos[1] + 1.62 - hy, t.pos[2] - hz);
                let sd = (xd * xd + zd * zd).sqrt();
                let y_d = (zd.atan2(xd).to_degrees() - 90.0) as f32;
                let x_d = (-yd.atan2(sd).to_degrees()) as f32;
                self.wh_x[i] = rotlerp(self.wh_x[i], x_d, 40.0);
                self.wh_y[i] = rotlerp(self.wh_y[i], y_d, 10.0);
            } else {
                self.wh_y[i] = rotlerp(self.wh_y[i], body_rot, 10.0);
            }
        }
    }

    /// VANILLA: Creeper#tick — swell += swellDir, 0..maxSwell(30).
    fn tick_creeper(&mut self, dir: i32) {
        self.o_swell = self.swell;
        self.swell = (self.swell + dir as f32).clamp(0.0, 30.0);
    }

    /// VANILLA: CopperGolem#setupAnimationStates (+ tick: idleAnimationStartTick = tick + nextInt(200, 240)).
    fn tick_copper_golem(&mut self, state: i32) {
        let c = self.arm_clock;
        if state != self.cg_state {
            self.cg_state = state;
            self.cg_inter = None;
        }
        if state == 0 {
            if self.cg_next == 0 {
                self.g_rng = if self.g_rng == 0 {
                    0x9E37_79B9 ^ c
                } else {
                    self.g_rng
                };
                self.g_rng ^= self.g_rng << 13;
                self.g_rng ^= self.g_rng >> 17;
                self.g_rng ^= self.g_rng << 5;
                self.cg_next = c + 200 + self.g_rng % 40;
            } else if c == self.cg_next {
                self.cg_idle = Some(c);
            } else if c == self.cg_next + 10 {
                self.cg_next = 0;
            }
        } else {
            self.cg_idle = None;
            self.cg_next = 0;
            self.cg_inter.get_or_insert(c);
        }
        self.arm_clock += 1;
    }

    /// VANILLA: Armadillo#setupAnimationStates + inStateTicks (peek не делаем — нужен entity_event 64).
    fn tick_armadillo(&mut self, state: i32) {
        if state != self.arm_state {
            self.arm_state = state;
            self.arm_ticks = 0;
        }
        match state {
            1 => {
                self.arm_out = None;
                self.arm_up.get_or_insert(self.arm_clock);
            }
            3 => {
                self.arm_up = None;
                self.arm_out.get_or_insert(self.arm_clock);
            }
            _ => {
                self.arm_up = None;
                self.arm_out = None;
            }
        }
        self.arm_clock += 1;
        self.arm_ticks += 1;
    }

    /// VANILLA: Warden#tick (клиент) + onSyncedDataUpdated + handleEntityEvent 4/61/62.
    fn tick_warden(&mut self, pose: i32, ev: [u32; 3], tick_count: u32, anger: i32) {
        let c = self.arm_clock;
        if pose != self.wd_pose {
            self.wd_pose = pose;
            match pose {
                13 => self.wd[3] = Some(c),
                14 => self.wd[2] = Some(c),
                11 => self.wd[4] = Some(c),
                12 => self.wd[5] = Some(c),
                _ => {}
            }
        }
        if ev[0] != self.wd_ev[0] {
            self.wd[4] = None;
            self.wd[0] = Some(c);
        }
        if ev[1] != self.wd_ev[1] {
            self.wd[1] = Some(c);
        }
        if ev[2] != self.wd_ev[2] {
            self.wd_tendril[1] = 10;
        }
        self.wd_ev = ev;
        // getHeartBeatDelay: 40 − floor(clamp(anger/80, 0, 1)·30).
        let delay = 40 - ((anger as f32 / 80.0).clamp(0.0, 1.0) * 30.0).floor() as u32;
        if tick_count % delay.max(1) == 0 {
            self.wd_heart[1] = 10;
        }
        self.wd_tendril[0] = self.wd_tendril[1];
        self.wd_tendril[1] = (self.wd_tendril[1] - 1).max(0);
        self.wd_heart[0] = self.wd_heart[1];
        self.wd_heart[1] = (self.wd_heart[1] - 1).max(0);
        self.arm_clock += 1;
    }

    /// VANILLA: Creaking#handleEntityEvent (4 → 15 тиков атаки, 66 → 8 тиков) + setupAnimationStates.
    fn tick_creaking(&mut self, ev: [u32; 2], tearing_down: bool) {
        let c = self.arm_clock;
        for (i, (dur, k)) in [(15u32, 0usize), (8, 1)].into_iter().enumerate() {
            if ev[i] != self.ck_ev[i] {
                self.ck_ev[i] = ev[i];
                // animateWhen: стартует, только если не идёт.
                if self.ck[k].is_none_or(|s| c.saturating_sub(s) >= dur) {
                    self.ck[k] = Some(c);
                }
            }
            if self.ck[k].is_some_and(|s| c.saturating_sub(s) >= dur) {
                self.ck[k] = None;
            }
        }
        if tearing_down {
            self.ck[2].get_or_insert(c);
        } else {
            self.ck[2] = None;
        }
        self.arm_clock += 1;
    }

    /// VANILLA: Shulker#tick — currentPeekAmount ±0.05 к целевому (peek·0.01).
    fn tick_shulker(&mut self, peek_byte: u8) {
        let target = f32::from(peek_byte) * 0.01;
        self.sh_peek[0] = self.sh_peek[1];
        let c = self.sh_peek[1];
        if c != target {
            self.sh_peek[1] = if c > target {
                (c - 0.05).clamp(target, 1.0)
            } else {
                (c + 0.05).clamp(0.0, target)
            };
        }
    }

    /// VANILLA: Breeze#onSyncedDataUpdated + Breeze#tick (AnimationState по позе).
    fn tick_breeze(&mut self, pose: i32) {
        let c = self.arm_clock;
        if pose != self.br_pose {
            self.br_pose = pose;
            self.br_shoot = None;
            self.br_inhale = None;
            self.br_jump = None;
            self.br_idle = None;
            match pose {
                16 => self.br_shoot = Some(c),
                17 => self.br_inhale = Some(c),
                15 => {
                    self.br_slide.get_or_insert(c);
                }
                _ => {}
            }
        }
        if pose == 6 {
            self.br_jump.get_or_insert(c);
        }
        self.br_idle.get_or_insert(c);
        if pose != 15 && self.br_slide.is_some() {
            self.br_slide_back = Some(c);
            self.br_slide = None;
        }
        self.arm_clock += 1;
    }

    /// VANILLA: Sniffer#onSyncedDataUpdated — смена состояния перезапускает анимацию.
    fn tick_sniffer(&mut self, state: i32) {
        if state != self.sn_state {
            self.sn_state = state;
            self.sn_start = self.arm_clock;
        }
        self.arm_clock += 1;
    }

    /// VANILLA: Axolotl#tickBabyAnimations (soloAnimation: смена состояния перезапускает отсчёт).
    fn tick_axolotl_baby(&mut self, dead: bool, in_water: bool, on_ground: bool, moving: bool) {
        let s = if dead {
            6
        } else if moving {
            if in_water && !on_ground {
                0
            } else if !in_water && on_ground {
                1
            } else {
                2
            }
        } else if in_water && !on_ground {
            3
        } else if in_water {
            4
        } else {
            5
        };
        if s != self.bx_state {
            self.bx_state = s;
            self.bx_start = self.arm_clock;
        }
        self.arm_clock += 1;
    }

    /// VANILLA: ArmadilloState#shouldHideInShell.
    fn armadillo_hiding(&self) -> bool {
        match self.arm_state {
            1 => self.arm_ticks > 5,
            2 => true,
            3 => self.arm_ticks < 26,
            _ => false,
        }
    }

    /// VANILLA: BinaryAnimator#tick (IN_OUT_SINE, длина 10).
    fn tick_axolotl(&mut self, active: [bool; 4]) {
        for (a, on) in self.ax.iter_mut().zip(active) {
            a.0 = a.1;
            a.1 = if on {
                (a.1 + 1).min(10)
            } else {
                (a.1 - 1).max(0)
            };
        }
    }

    /// VANILLA: BinaryAnimator#getFactor.
    fn axolotl_factors(&self, pt: f32) -> [f32; 4] {
        self.ax.map(|(o, n)| {
            let x = (o as f32 + (n - o) as f32 * pt) / 10.0;
            -((std::f32::consts::PI * x).cos() - 1.0) / 2.0
        })
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
        // VANILLA: TropicalFishRenderer#setupRotations — виляние; на суше лежит на боку.
        Kind::TropSmall | Kind::TropLarge => {
            let w = Mat4::from_rotation_y((4.3 * (0.6 * age).sin()).to_radians());
            if in_water {
                w
            } else {
                w * Mat4::from_translation(Vec3::new(0.2, 0.1, 0.0))
                    * Mat4::from_rotation_z(90f32.to_radians())
            }
        }
        // VANILLA: PufferfishRenderer#setupRotations — покачивание по Y.
        Kind::PufferSmall | Kind::PufferMid | Kind::PufferBig => {
            Mat4::from_translation(Vec3::new(0.0, (age * 0.05).cos() * 0.08, 0.0))
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
        Kind::ZVillagerBaby => {
            let (m, p) = animals::zombie_villager_baby_mesh();
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
        Kind::Stray => {
            let (m, p) = humanoid::skeleton_mesh(0.0);
            let (o, op) = humanoid::skeleton_overlay_mesh(0.25);
            Rig::Layered(m, p, o, op)
        }
        Kind::Bogged => {
            let (m, p) = humanoid::bogged_mesh();
            let (o, op) = humanoid::skeleton_overlay_mesh(0.2);
            Rig::Layered(m, p, o, op)
        }
        Kind::Parched => {
            let (m, p) = humanoid::parched_mesh();
            Rig::Human(m, p)
        }
        Kind::PolarBear => Rig::Plain(animals::polar_bear_mesh()),
        Kind::Rabbit => Rig::Plain(animals::rabbit_mesh()),
        Kind::Squid => Rig::Plain(animals::squid_mesh()),
        Kind::Fox => Rig::Plain(animals::fox_mesh()),
        Kind::Turtle => Rig::Plain(animals::turtle_mesh()),
        Kind::Panda => Rig::Plain(animals::panda_mesh()),
        Kind::SnowGolem => Rig::Plain(animals::snow_golem_mesh()),
        Kind::CopperGolem => Rig::Plain(animals::copper_golem_mesh()),
        Kind::Camel => Rig::Plain(animals::camel_mesh()),
        Kind::Cod => Rig::Plain(animals::cod_mesh()),
        Kind::Salmon => Rig::Plain(animals::salmon_mesh()),
        Kind::TropSmall => Rig::Tropical(
            animals::tropical_fish_mesh(false, 0.0),
            animals::tropical_fish_mesh(false, 0.01),
        ),
        Kind::TropLarge => Rig::Tropical(
            animals::tropical_fish_mesh(true, 0.0),
            animals::tropical_fish_mesh(true, 0.01),
        ),
        Kind::PufferSmall => Rig::Plain(animals::pufferfish_mesh(0)),
        Kind::PufferMid => Rig::Plain(animals::pufferfish_mesh(1)),
        Kind::PufferBig => Rig::Plain(animals::pufferfish_mesh(2)),
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
        Kind::Allay => Rig::Plain(animals::allay_mesh()),
        Kind::Armadillo => Rig::Plain(animals::armadillo_mesh()),
        Kind::ArmadilloBaby => Rig::Plain(animals::armadillo_baby_mesh()),
        Kind::Sniffer => Rig::Plain(animals::sniffer_mesh()),
        Kind::SniffletBaby => Rig::Plain(animals::sniffer_baby_mesh()),
        Kind::HappyGhast => Rig::Plain(animals::happy_ghast_mesh(false)),
        Kind::HappyGhastBaby => Rig::Plain(animals::happy_ghast_mesh(true)),
        Kind::Creaking => Rig::Plain(animals::creaking_mesh()),
        Kind::Nautilus => Rig::Plain(animals::nautilus_mesh(false)),
        Kind::NautilusBaby => Rig::Plain(animals::nautilus_mesh(true)),
        Kind::Frog => Rig::Plain(animals::frog_mesh()),
        Kind::Axolotl => Rig::Plain(animals::axolotl_mesh()),
        Kind::AxolotlBaby => Rig::Plain(animals::axolotl_baby_mesh()),
        Kind::Tadpole => Rig::Plain(animals::tadpole_mesh()),
        Kind::Goat => Rig::Plain(animals::goat_mesh()),
        Kind::Llama => Rig::Plain(animals::llama_mesh()),
        Kind::Horse => Rig::Plain(animals::equine_mesh(false)),
        Kind::Donkey => Rig::Plain(animals::equine_mesh(true)),
        Kind::Pig => Rig::Plain(animals::pig_mesh()),
        Kind::Cow => Rig::Plain(animals::cow_mesh(0)),
        Kind::CowWarm => Rig::Plain(animals::cow_mesh(1)),
        Kind::CowCold => Rig::Plain(animals::cow_mesh(2)),
        Kind::Creeper => Rig::Creeper(animals::creeper_mesh(0.0), animals::creeper_mesh(2.0)),
        Kind::Spider => Rig::Plain(animals::spider_mesh()),
        Kind::Chicken => Rig::Plain(animals::chicken_mesh()),
        Kind::Enderman => {
            let (m, p) = animals::enderman_mesh();
            Rig::Enderman(m, p)
        }
        Kind::IronGolem => Rig::Plain(animals::iron_golem_mesh()),
        Kind::Blaze => Rig::Plain(animals::blaze_mesh()),
        Kind::Breeze => Rig::Breeze(
            animals::breeze_mesh(0),
            animals::breeze_mesh(1),
            animals::breeze_mesh(2),
        ),
        Kind::Shulker => Rig::Plain(animals::shulker_mesh()),
        Kind::Warden => Rig::Warden(animals::warden_mesh()),
        Kind::EnderDragon => Rig::Dragon(animals::ender_dragon_mesh()),
        Kind::Wither => Rig::Wither(animals::wither_mesh(0.0), animals::wither_mesh(0.5)),
        Kind::Slime => Rig::Slime(animals::slime_inner_mesh(), animals::slime_outer_mesh()),
        Kind::MagmaCube => Rig::Plain(animals::magma_cube_mesh()),
        Kind::Ghast => Rig::Plain(animals::ghast_mesh()),
        Kind::Silverfish => Rig::Plain(animals::silverfish_mesh()),
        Kind::Endermite => Rig::Plain(animals::endermite_mesh()),
        Kind::Villager => Rig::Plain(animals::villager_mesh()),
        Kind::VillagerBaby => Rig::Plain(animals::villager_baby_mesh()),
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
    /// VANILLA: EnderDragon#recreateFromPacket — id части = id дракона + 1 + индекс; хитбоксы частей.
    pub fn dragon_parts(&self) -> Vec<(i32, [f64; 6])> {
        let mut v = Vec::new();
        for (id, a) in &self.anims {
            let Some(ps) = a.dr_parts else { continue };
            for (i, p) in ps.iter().enumerate() {
                let (w, h) = DRAGON_PART_SIZE[i];
                v.push((
                    id + 1 + i as i32,
                    [
                        p[0] - w / 2.0,
                        p[1],
                        p[2] - w / 2.0,
                        p[0] + w / 2.0,
                        p[1] + h,
                        p[2] + w / 2.0,
                    ],
                ));
            }
        }
        v
    }

    /// VANILLA: MushroomCowMushroomLayer#submit — блок гриба в трёх позах; вершины блочного формата.
    pub fn bake_mushrooms(&self, models: &ezzmc_mesh::models::BlockModels, out: &mut Vec<u8>) {
        use ezzmc_mesh::section::ModelProvider;
        use ezzmc_render::particledraw::push_vertex_raw;
        for (brown, name) in [(false, "red_mushroom"), (true, "brown_mushroom")] {
            let Some(st) = crate::pistonview::state_with(name, &[]) else {
                continue;
            };
            let mut ps = Vec::new();
            models.parts(st, 0, 0, 0, &mut ps);
            let t = |x, y, z| Mat4::from_translation(Vec3::new(x, y, z));
            let ry = |d: f32| Mat4::from_rotation_y(d.to_radians());
            let tail = Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0)) * t(-0.5, -0.5, -0.5);
            for m in self.mushrooms.iter().filter(|m| m.brown == brown) {
                let mats = [
                    m.root * t(0.2, -0.35, 0.5) * ry(-48.0) * tail,
                    m.root * t(0.2, -0.35, 0.5) * ry(42.0) * t(0.1, 0.0, -0.6) * ry(-48.0) * tail,
                    m.head * t(0.0, -0.7, -0.2) * ry(-78.0) * tail,
                ];
                let light = (u16::from(m.light.0) * 16, u16::from(m.light.1) * 16);
                for mat in mats {
                    for part in &ps {
                        let qc = &part.quads;
                        for q in qc.unculled.iter().chain(qc.culled.iter().flatten()) {
                            // VANILLA: Lighting.Entry.ENTITY — освещение по нормали квада после матрицы
                            // (S(−1,−1,1) переворачивает «верх» блока), а не блочный shade.
                            // VertexConsumer#putBakedQuad: нормаль = quad.direction(), не геометрия квада.
                            let sd = ezzmc_mesh::cull::offset(q.direction);
                            let n = Vec3::new(sd[0] as f32, sd[1] as f32, sd[2] as f32);
                            let n = mat.transform_vector3(n).normalize_or_zero();
                            let v = (255.0 * ezzmc_render::entitymodel::mix_light(n)) as u8;
                            for i in 0..4 {
                                let p = mat.transform_point3(Vec3::from(q.pos[i]));
                                push_vertex_raw(out, p.to_array(), q.uv[i], [v, v, v, 255], light);
                            }
                        }
                    }
                }
            }
        }
    }

    /// VANILLA: BlockDecorationLayer — блок на антенне медного голема (состояние по умолчанию).
    pub fn bake_antenna(&self, models: &ezzmc_mesh::models::BlockModels, out: &mut Vec<u8>) {
        use ezzmc_mesh::section::ModelProvider;
        use ezzmc_render::particledraw::push_vertex_raw;
        // UNIT_CUBE_BOTTOM_CENTER_TO_ANTENNA_CENTER: translation(−0.5, 0, −0.5) · rotateAround(Z 180°, центр куба).
        let unit = Mat4::from_translation(Vec3::new(-0.5, 0.0, -0.5))
            * Mat4::from_translation(Vec3::splat(0.5))
            * Mat4::from_rotation_z(std::f32::consts::PI)
            * Mat4::from_translation(Vec3::splat(-0.5));
        for (name, head, l) in &self.antenna {
            let Some(st) = crate::pistonview::state_with(name, &[]) else {
                continue;
            };
            let mut ps = Vec::new();
            models.parts(st, 0, 0, 0, &mut ps);
            let mat = *head * unit;
            let light = (u16::from(l.0) * 16, u16::from(l.1) * 16);
            for part in &ps {
                let qc = &part.quads;
                for q in qc.unculled.iter().chain(qc.culled.iter().flatten()) {
                    let sd = ezzmc_mesh::cull::offset(q.direction);
                    let n = Vec3::new(sd[0] as f32, sd[1] as f32, sd[2] as f32);
                    let n = mat.transform_vector3(n).normalize_or_zero();
                    let v = (255.0 * ezzmc_render::entitymodel::mix_light(n)) as u8;
                    for i in 0..4 {
                        let p = mat.transform_point3(Vec3::from(q.pos[i]));
                        push_vertex_raw(out, p.to_array(), q.uv[i], [v, v, v, 255], light);
                    }
                }
            }
        }
    }

    /// Запекает всех известных мобов в вершины по текстурам; тени — в `shadow`.
    /// VANILLA: LevelEventHandler#notifyNearbyEntities (1010 играет / 1011 стоп) → попугаям рядом.
    pub fn record_event(&mut self, pos: [i32; 3], playing: bool) {
        self.record_events.push((pos, playing));
    }

    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        sky_darken: i32,
        frustum: Option<&ezzmc_mesh::frustum::Frustum>,
        shadow: &mut Vec<u8>,
    ) {
        // PROF: EZZMC_PROF=1 — раз в 120 кадров пишет в лог разбивку bake (всего / тени / мобов нарисовано).
        let prof = prof_on();
        let t_all = std::time::Instant::now();
        let mut t_shadow = std::time::Duration::ZERO;
        let mut drawn = 0u32;
        let mut par_jobs: Vec<ParJob> = Vec::new();
        let mut seg = [std::time::Duration::ZERO; 4];
        let mut mark = std::time::Instant::now();
        for v in self.data.values_mut() {
            v.clear();
        }
        for v in self.skin_inst.values_mut() {
            v.clear();
        }
        self.mushrooms.clear();
        self.held_items.clear();
        self.antenna.clear();
        let record_events = std::mem::take(&mut self.record_events);
        // Клиентские тики, а не game_time: set_time двигает его рывками → дубли/пропуски анимации.
        let now = level.client_ticks;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            _ => 0,
        };
        let net_pt = level.partial_tick();
        // Чистка состояния удалённых сущностей — раз за игровой тик, не каждый кадр (хэш-проход по всей толпе).
        // Устаревшая запись безвредна: `work` ниже берёт только живые сущности.
        if ticks > 0 {
            self.anims
                .retain(|id, _| level.entities.map.contains_key(id));
        }
        // Толпа (свиньи/коровы/курицы): параллельно по всем ядрам, общий цикл ниже её пропускает.
        let t_crowd = std::time::Instant::now();
        let tbl = crowd_table();
        for e in level.entities.iter() {
            if tbl.get(e.ty).copied().unwrap_or(false) {
                self.anims.entry(e.id).or_default();
            }
        }
        {
            use rayon::prelude::*;
            let ctx = CrowdCtx {
                level,
                cam,
                sky_darken,
                frustum,
                ticks,
                net_pt,
            };
            let ents = &level.entities.map;
            let mut work: Vec<(&ezzmc_world::entities::Entity, &mut MobAnim)> = self
                .anims
                .iter_mut()
                .filter_map(|(id, anim)| {
                    ents.get(id).filter(|e| is_crowd(e, tbl)).map(|e| (e, anim))
                })
                .collect();
            let outs: Vec<CrowdOut> = work
                .par_chunks_mut(128)
                .map(|chunk| {
                    let mut acc = CrowdOut::default();
                    for (e, anim) in chunk.iter_mut() {
                        crowd_one(&ctx, e, anim, &mut acc);
                    }
                    acc
                })
                .collect();
            for o in outs {
                par_jobs.extend(o.jobs);
                shadow.extend_from_slice(&o.shadow);
                drawn += o.drawn;
            }
        }
        prof_phase(13, "bake: толпа prepass+параллель", t_crowd.elapsed());
        let t_ser = std::time::Instant::now();
        for e in level.entities.iter() {
            if prof {
                mark = std::time::Instant::now();
            }
            if is_crowd(e, tbl) {
                continue;
            }
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                continue;
            };
            let Some(mut sp) = spec_cached(e.ty) else {
                continue;
            };
            if sp.kind == Kind::Wither {
                // VANILLA: WitherBossRenderer#scale — 2.0 − invulnerableTicks/220·0.5 (ticks − partialTicks).
                let inv = e.int_meta(19).unwrap_or(0);
                if inv > 0 {
                    let inv = inv as f32 - net_pt;
                    sp.scale -= inv / 220.0 * 0.5;
                }
            }
            if *name == "cow" {
                sp.kind = cow_kind(e);
            }
            // VANILLA: TropicalFishRenderer — база SMALL/LARGE по packed variant (#17: base | index<<8 | цвета<<16/24).
            if *name == "tropical_fish" {
                let large = e.int_meta(17).unwrap_or(0) & 0xFF == 1;
                sp.kind = if large {
                    Kind::TropLarge
                } else {
                    Kind::TropSmall
                };
                sp.tex = if large {
                    "fish/tropical_b"
                } else {
                    "fish/tropical_a"
                };
            }
            // VANILLA: PufferfishRenderer#submit — модель по puffState (Pufferfish#17), тень 0.1+0.1·state.
            if *name == "pufferfish" {
                let ps = e.int_meta(17).unwrap_or(0).clamp(0, 2);
                sp.kind = [Kind::PufferSmall, Kind::PufferMid, Kind::PufferBig][ps as usize];
                sp.shadow = 0.1 + 0.1 * ps as f32;
            }
            // VANILLA: Entity#updateFluidHeightAndDoFluidPushing(WATER) — вода в AABB (сжат на 0.001),
            // высота жидкости ≥ minY.
            let in_water = matches!(
                sp.kind,
                Kind::Cod
                    | Kind::Salmon
                    | Kind::TropSmall
                    | Kind::TropLarge
                    | Kind::PufferSmall
                    | Kind::PufferMid
                    | Kind::PufferBig
                    | Kind::Guardian
                    | Kind::Squid
                    | Kind::Axolotl
            ) && {
                let (w, h) = ezzmc_world::entity_size::size(name)
                    .map_or((0.5, 1.0), |s| (s.0 as f64, s.1 as f64));
                let (x0, x1) = (e.pos[0] - w / 2.0 + 0.001, e.pos[0] + w / 2.0 - 0.001);
                let (z0, z1) = (e.pos[2] - w / 2.0 + 0.001, e.pos[2] + w / 2.0 - 0.001);
                let (y0, y1) = (e.pos[1] + 0.001, e.pos[1] + h - 0.001);
                let water = |x: i32, y: i32, z: i32| {
                    ezzmc_mesh::blockinfo::fluid(level.chunks.block(x, y, z)).filter(|f| !f.lava)
                };
                let mut hit = false;
                for bx in x0.floor() as i32..=x1.floor() as i32 {
                    for by in y0.floor() as i32..=y1.floor() as i32 {
                        for bz in z0.floor() as i32..=z1.floor() as i32 {
                            if let Some(f) = water(bx, by, bz) {
                                let top = if water(bx, by + 1, bz).is_some() {
                                    1.0
                                } else {
                                    f64::from(f.own_height())
                                };
                                hit |= f64::from(by) + top >= y0;
                            }
                        }
                    }
                }
                hit
            };
            // Управляемая нами лодка тикает на сетке главного потока — рисуем в той же фазе, что камера.
            let pt = if e.local_ctrl {
                level.local_ctrl_pt
            } else {
                net_pt
            };
            let anim = self.anims.entry(e.id).or_default();
            if sp.kind == Kind::Parrot {
                // VANILLA: level.getEntitiesOfClass(LivingEntity, AABB(pos).inflate(3)) → setRecordPlayingNearby.
                // Попугай 0.5×0.9: bbox пересекает [pos-3, pos+4].
                for &(p, playing) in &record_events {
                    let near = |c: f64, b: i32, half: f64, h: f64| {
                        c + half > f64::from(b) - 3.0 && c - h < f64::from(b) + 4.0
                    };
                    if near(e.pos[0], p[0], 0.25, 0.25)
                        && near(e.pos[1], p[1], 0.9, 0.0)
                        && near(e.pos[2], p[2], 0.25, 0.25)
                    {
                        anim.jukebox = Some(p);
                        anim.party = playing;
                    }
                }
                // VANILLA: Parrot#aiStep — вне 3.46 от центра проигрывателя или блок не jukebox → сброс.
                if let Some(p) = anim.jukebox {
                    let d = [
                        e.pos[0] - (f64::from(p[0]) + 0.5),
                        e.pos[1] - (f64::from(p[1]) + 0.5),
                        e.pos[2] - (f64::from(p[2]) + 0.5),
                    ];
                    let is_jb = level
                        .chunks
                        .block(p[0], p[1], p[2])
                        .name()
                        .ends_with("jukebox");
                    if d[0] * d[0] + d[1] * d[1] + d[2] * d[2] >= 3.46 * 3.46 || !is_jb {
                        anim.jukebox = None;
                        anim.party = false;
                    }
                }
            }
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
                if sp.kind == Kind::Shulker {
                    anim.tick_shulker(e.byte17);
                }
                if sp.kind == Kind::Creaking {
                    anim.tick_creaking([e.mob_events[4], e.mob_events[66]], e.bool18);
                }
                if sp.kind == Kind::EnderDragon {
                    // DATA_PHASE (#16): 5..7 — SITTING_*.
                    let phase = e.int_meta(16).unwrap_or(10);
                    let egg_dist = if phase == 3 || phase == 4 {
                        dragon_egg_dist(level, e.pos)
                    } else {
                        0.0
                    };
                    anim.tick_dragon(e, phase, egg_dist, &mut self.dragon_fx);
                }
                if sp.kind == Kind::Wither {
                    anim.tick_wither(e, level, anim.av.body_rot(1.0));
                }
                if sp.kind == Kind::Warden {
                    let ev = [e.mob_events[4], e.mob_events[62], e.mob_events[61]];
                    anim.tick_warden(e.pose, ev, e.age, e.int_meta(16).unwrap_or(0));
                }
                if sp.kind == Kind::Breeze {
                    anim.tick_breeze(e.pose);
                }
                if sp.kind == Kind::Sniffer {
                    anim.tick_sniffer(e.int_meta(18).unwrap_or(0));
                }
                if sp.kind == Kind::CopperGolem {
                    anim.tick_copper_golem(e.int_meta(17).unwrap_or(0));
                }
                if sp.kind == Kind::Armadillo {
                    anim.tick_armadillo(e.int_meta(18).unwrap_or(0));
                }
                if sp.kind == Kind::Axolotl {
                    // VANILLA: Axolotl#tickAdultAnimations — PLAYING_DEAD / IN_WATER / ON_GROUND / иначе.
                    // VANILLA-UNSURE: isMoving = walkAnimation.isMoving() || поворот за тик.
                    let moving = anim.av.walk_speed() > 1.0e-5
                        || e.yaw != e.prev_yaw
                        || e.pitch != e.prev_pitch;
                    let dead = e.bool19;
                    if e.baby {
                        anim.tick_axolotl_baby(dead, in_water, e.on_ground, moving);
                    } else {
                        anim.tick_axolotl([
                            dead,
                            !dead && in_water,
                            !dead && !in_water && e.on_ground,
                            moving,
                        ]);
                    }
                }
                if sp.kind == Kind::Squid {
                    anim.tick_squid(e, in_water);
                }
                if matches!(sp.kind, Kind::Chicken | Kind::Parrot) {
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
            // EnderDragonRenderer#affectedByCulling = false; bbox 16×8 → дальность ≈ 64·12.
            let max_d = if sp.kind == Kind::EnderDragon {
                768.0
            } else {
                max_d
            };
            if dx * dx + dy * dy + dz * dz > max_d * max_d {
                continue;
            }
            // VANILLA: EntityRenderer#shouldRender — AABB.inflate(0.5) против Frustum; дракон: noCulling.
            // Анимация уже протикана выше, пропускаем только запекание вершин.
            if sp.kind != Kind::EnderDragon {
                if let (Some(fr), Some((lo, hi))) = (frustum, e.hitbox()) {
                    let lo = [lo[0] - 0.5, lo[1] - 0.5, lo[2] - 0.5];
                    let hi = [hi[0] + 0.5, hi[1] + 0.5, hi[2] + 0.5];
                    if !fr.is_visible(lo, hi) {
                        continue;
                    }
                }
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
                        a + (p - a) * net_pt.clamp(0.0, 1.0)
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
            if prof {
                seg[0] += mark.elapsed();
                mark = std::time::Instant::now();
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
                * shulker_attach(sp.kind, e.int_meta(16))
                * sq_post
                * death_flip(sp.kind, e.death_at)
                * fish_wiggle(sp.kind, state.age_in_ticks, in_water)
                * phantom_pitch(sp.kind, e.pitch)
                * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
                * Mat4::from_scale(Vec3::splat(sp.scale))
                * squash
                * phantom_pre(sp.kind, e.swell_dir)
                * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0))
                * Mat4::from_scale(Vec3::splat(if sp.kind == Kind::HappyGhast && e.baby {
                    sp.mscale * 0.2375
                } else {
                    sp.mscale
                }));
            // VANILLA: EnderDragonRenderer#submit — свой root: R_y(−yRot[7])·R_x(Δy[5,10]·10)·T(0,0,1)·S(−1,−1,1)·T(0,−1.501,0).
            let root = if sp.kind == Kind::EnderDragon {
                let h = |d: i32| anim.dragon_sample(d, pt);
                let rot2 = (h(5).0 - h(10).0) as f32;
                Mat4::from_translation(Vec3::new(
                    (pos[0] - cam[0]) as f32,
                    (pos[1] - cam[1]) as f32,
                    (pos[2] - cam[2]) as f32,
                )) * Mat4::from_rotation_y((-h(7).1).to_radians())
                    * Mat4::from_rotation_x((rot2 * 10.0).to_radians())
                    * Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0))
                    * Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0))
                    * Mat4::from_translation(Vec3::new(0.0, -1.501, 0.0))
            } else {
                root
            };
            // VANILLA: LivingEntityRenderer#getOverlayCoords — красный оверлей, пока hurtTime>0.
            let hurt = e
                .hurt_at
                .is_some_and(|t| t.elapsed().as_secs_f32() < e.hurt_dur);
            let snap: Vec<(&'static str, usize)> = if hurt {
                self.data.iter().map(|(k, v)| (*k, v.len())).collect()
            } else {
                Vec::new()
            };
            let mk = if sp.kind == Kind::Villager && e.baby {
                Kind::VillagerBaby
            } else if sp.kind == Kind::ZVillager && e.baby {
                Kind::ZVillagerBaby
            } else if sp.kind == Kind::Armadillo && e.baby {
                Kind::ArmadilloBaby
            } else if sp.kind == Kind::Axolotl && e.baby {
                Kind::AxolotlBaby
            } else if sp.kind == Kind::Sniffer && e.baby {
                Kind::SniffletBaby
            } else if sp.kind == Kind::HappyGhast && e.baby {
                Kind::HappyGhastBaby
            } else if sp.kind == Kind::Nautilus && e.baby && *name == "nautilus" {
                Kind::NautilusBaby
            } else {
                sp.kind
            };
            let rig = self.models.entry(mk).or_insert_with(|| build(mk));
            // GhastRenderer: при зарядке (Ghast#DATA_IS_CHARGING, bool #16) текстура ghast_shooting.
            let tex = if sp.kind == Kind::Ghast && e.baby {
                "ghast/ghast_shooting"
            } else if mk == Kind::ZVillagerBaby {
                // VANILLA: ZombieVillagerRenderer#getTextureLocation.
                "zombie_villager/zombie_villager_baby"
            } else if mk == Kind::VillagerBaby {
                // VANILLA: VillagerRenderer#getTextureLocation (villager_baby.png).
                "villager/villager_baby"
            } else if mk == Kind::CopperGolem {
                // VANILLA: CopperGolemOxidationLevels#getOxidationLevel(weathering).texture().
                match e.int_meta(16).unwrap_or(0) {
                    1 => "copper_golem/copper_golem_exposed",
                    2 => "copper_golem/copper_golem_weathered",
                    3 => "copper_golem/copper_golem_oxidized",
                    _ => "copper_golem/copper_golem",
                }
            } else if mk == Kind::ArmadilloBaby {
                "armadillo/armadillo_baby"
            } else if mk == Kind::SniffletBaby {
                "sniffer/snifflet"
            } else if mk == Kind::HappyGhastBaby {
                "ghast/happy_ghast_baby"
            } else if sp.kind == Kind::Wither {
                // VANILLA: WitherBossRenderer#getTextureLocation (invulnerableTicks = floor(ticks − pt)).
                let inv = (e.int_meta(19).unwrap_or(0) as f32 - net_pt).floor() as i32;
                if inv > 0 && (inv > 80 || inv / 5 % 2 != 1) {
                    "wither/wither_invulnerable"
                } else {
                    "wither/wither"
                }
            } else if mk == Kind::NautilusBaby {
                "nautilus/nautilus_baby"
            } else {
                variant_tex(name, e).unwrap_or(sp.tex)
            };
            if prof {
                seg[1] += mark.elapsed();
                mark = std::time::Instant::now();
            }
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
                        Kind::Zombie
                            | Kind::ZPiglin
                            | Kind::Drowned
                            | Kind::ZVillager
                            | Kind::ZVillagerBaby
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
                        // VANILLA: IllusionerRenderer#<init> — model.getHat().visible = true (у остальных иллагеров скрыта).
                        if let Some(h) = model.index("hat") {
                            model.parts[h].visible = sp.tex == "illager/illusioner";
                        }
                    }
                    model.bake(root, [255; 4], light, out);
                    if *name == "zombie_villager" {
                        villager_layers(
                            &mut self.data,
                            model,
                            root,
                            light,
                            e,
                            "zombie_villager",
                            20,
                        );
                    }
                }
                Rig::Layered(model, parts, over, over_parts) => {
                    humanoid::setup_anim(model, parts, &state);
                    humanoid::setup_anim(over, over_parts, &state);
                    // VANILLA: BoggedModel#setupAnim — грибы скрыты у остриженного (Bogged#DATA_SHEARED = #16).
                    if sp.kind == Kind::Bogged {
                        if let Some(m) = model.index("mushrooms") {
                            model.parts[m].visible = !e.baby;
                        }
                    }
                    model.bake(root, [255; 4], light, out);
                    let ot = if sp.kind == Kind::Bogged {
                        "skeleton/bogged_overlay"
                    } else {
                        "skeleton/stray_overlay"
                    };
                    over.bake(root, [255; 4], light, self.data.entry(ot).or_default());
                }
                Rig::Breeze(body, wind, eyes) => {
                    let ms = |s: Option<u32>| {
                        s.map(|s| ((anim.arm_clock.saturating_sub(s)) as f32 + pt) as i64 * 50)
                    };
                    let times = [
                        ms(anim.br_idle),
                        ms(anim.br_shoot),
                        ms(anim.br_slide),
                        ms(anim.br_slide_back),
                        ms(anim.br_inhale),
                        ms(anim.br_jump),
                    ];
                    for m in [&mut *body, &mut *wind, &mut *eyes] {
                        m.reset_pose();
                        animals::breeze_anim(
                            m, times[0], times[1], times[2], times[3], times[4], times[5],
                        );
                    }
                    body.bake(root, [255; 4], light, out);
                    // VANILLA: BreezeEyesLayer (полная яркость) и BreezeWindLayer (u += age·0.02 mod 1).
                    eyes.bake_emissive(root, self.data.entry("breeze/breeze_eyes").or_default());
                    let wout = self.data.entry("breeze/breeze_wind").or_default();
                    let from = wout.len();
                    wind.bake_emissive(root, wout);
                    let du = (state.age_in_ticks * 0.02) % 1.0;
                    for v in wout[from..].chunks_exact_mut(32) {
                        let u = f32::from_le_bytes([v[12], v[13], v[14], v[15]]) + du;
                        v[12..16].copy_from_slice(&u.to_le_bytes());
                    }
                }
                Rig::Dragon(model) => {
                    // VANILLA: EnderDragonRenderer#extractRenderState — фаза (#16), flapTime lerp, история полёта.
                    let phase = e.int_meta(16).unwrap_or(10);
                    let landing_or_takeoff = phase == 3 || phase == 4;
                    let distance_to_egg = if landing_or_takeoff {
                        // Яйцо: Heightmap MOTION_BLOCKING_NO_LEAVES над подиумом (0, 0).
                        let top = (-64..=320)
                            .rev()
                            .find(|&y| {
                                let b = level.chunks.block(0, y, 0);
                                !b.is_air() && !b.name().ends_with("_leaves")
                            })
                            .map_or(64, |y| y + 1);
                        let d = [
                            0.5 - e.pos[0],
                            f64::from(top) + 0.5 - e.pos[1],
                            0.5 - e.pos[2],
                        ];
                        d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
                    } else {
                        0.0
                    };
                    let ph = animals::DragonPhase {
                        landing_or_takeoff,
                        sitting: (5..=7).contains(&phase),
                        distance_to_egg,
                    };
                    let flap = anim.dr_flap[0] + (anim.dr_flap[1] - anim.dr_flap[0]) * pt;
                    let hist = |d: i32| anim.dragon_sample(d, pt);
                    let pose = animals::ender_dragon_anim(model, flap, &hist, ph);
                    let r = root * pose;
                    model.bake(r, [255; 4], light, out);
                    // VANILLA: RenderTypes.eyes(dragon_eyes) — та же модель, полная яркость.
                    model.bake_emissive(r, self.data.entry("enderdragon/dragon_eyes").or_default());
                }
                Rig::Wither(model, armor) => {
                    let (yh, xh) = (anim.wh_y, anim.wh_x);
                    for m in [&mut *model, &mut *armor] {
                        animals::wither_anim(m, state.age_in_ticks, (yr, xr), body, yh, xh);
                    }
                    model.bake(root, [255; 4], light, out);
                    // VANILLA: WitherArmorLayer — isPowered: здоровье ≤ max/2 (max 300; VANILLA-UNSURE: из атрибутов).
                    if e.health >= 0.0 && e.health <= 150.0 {
                        let t = state.age_in_ticks;
                        energy_swirl(
                            armor,
                            root,
                            self.data.entry("wither/wither_armor").or_default(),
                            (t * 0.02).cos() * 3.0 % 1.0,
                            t * 0.01 % 1.0,
                        );
                    }
                }
                Rig::Warden(model) => {
                    let ms = |s: Option<u32>| {
                        s.map(|s| ((anim.arm_clock.saturating_sub(s)) as f32 + pt) as i64 * 50)
                    };
                    let lerp = |a: [i32; 2]| (a[0] as f32 + (a[1] - a[0]) as f32 * pt) / 10.0;
                    let (tendril, heart) = (lerp(anim.wd_tendril), lerp(anim.wd_heart));
                    let times = animals::WardenTimes {
                        attack: ms(anim.wd[0]),
                        sonic_boom: ms(anim.wd[1]),
                        dig: ms(anim.wd[2]),
                        emerge: ms(anim.wd[3]),
                        roar: ms(anim.wd[4]),
                        sniff: ms(anim.wd[5]),
                    };
                    model.reset_pose();
                    animals::warden_anim(
                        model,
                        (yr, xr),
                        wp,
                        ws,
                        state.age_in_ticks,
                        tendril,
                        &times,
                    );
                    model.bake(root, [255; 4], light, out);
                    // VANILLA: WardenRenderer — LivingEntityEmissiveLayer ×5 (entityTranslucentEmissive).
                    let age = state.age_in_ticks;
                    let spot = |ph: f32| (((age * 0.045 + ph).cos() * 0.25).max(0.0) * 255.0) as u8;
                    let layers: [(&'static str, u8, &[&str]); 5] = [
                        (
                            "warden/warden_bioluminescent_layer",
                            255,
                            &["head", "left_arm", "right_arm", "left_leg", "right_leg"],
                        ),
                        (
                            "warden/warden_pulsating_spots_1",
                            spot(0.0),
                            &[
                                "body",
                                "head",
                                "left_arm",
                                "right_arm",
                                "left_leg",
                                "right_leg",
                            ],
                        ),
                        (
                            "warden/warden_pulsating_spots_2",
                            spot(std::f32::consts::PI),
                            &[
                                "body",
                                "head",
                                "left_arm",
                                "right_arm",
                                "left_leg",
                                "right_leg",
                            ],
                        ),
                        (
                            "warden/warden",
                            (tendril.clamp(0.0, 1.0) * 255.0) as u8,
                            &["left_tendril", "right_tendril"],
                        ),
                        (
                            "warden/warden_heart",
                            (heart.clamp(0.0, 1.0) * 255.0) as u8,
                            &["body"],
                        ),
                    ];
                    for (tex, a, names) in layers {
                        if a > 0 {
                            model.bake_emissive_parts(
                                root,
                                a,
                                self.data.entry(tex).or_default(),
                                &|n| names.contains(&n),
                            );
                        }
                    }
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
                    model.bake_emissive(
                        root,
                        self.data.entry("enderman/enderman_eyes").or_default(),
                    );
                }
                Rig::Tropical(model, pat) => {
                    model.reset_pose();
                    pat.reset_pose();
                    animals::tropical_fish_anim(model, state.age_in_ticks, in_water);
                    animals::tropical_fish_anim(pat, state.age_in_ticks, in_water);
                    let packed = e.int_meta(17).unwrap_or(0);
                    let (idx, base_c, pat_c) = (
                        (packed >> 8) & 0xFF,
                        (packed >> 16) as u8,
                        (packed >> 24) as u8,
                    );
                    model.bake(root, animals::dye_diffuse(base_c), light, out);
                    // VANILLA: TropicalFishPatternLayer — tropical_{a|b}_pattern_{index+1}, цвет узора.
                    let ptex = intern(format!(
                        "fish/tropical_{}_pattern_{}",
                        if sp.kind == Kind::TropLarge { 'b' } else { 'a' },
                        idx.clamp(0, 5) + 1
                    ));
                    pat.bake(
                        root,
                        animals::dye_diffuse(pat_c),
                        light,
                        self.data.entry(ptex).or_default(),
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
                Rig::Creeper(model, armor) => {
                    model.reset_pose();
                    armor.reset_pose();
                    animals::creeper_anim(model, yr, xr, wp, ws);
                    animals::creeper_anim(armor, yr, xr, wp, ws);
                    model.bake(root, [255; 4], light, out);
                    // VANILLA: CreeperPowerLayer/EnergySwirlLayer — цвет 0xFF808080, UV += age·0.01 (u и v).
                    if e.bool17 {
                        let wout = self.data.entry(SWIRL_TEX).or_default();
                        let from = wout.len();
                        armor.bake(root, [128, 128, 128, 255], (15, 15), wout);
                        let d = (state.age_in_ticks * 0.01) % 1.0;
                        for v in wout[from..].chunks_exact_mut(32) {
                            let u = f32::from_le_bytes([v[12], v[13], v[14], v[15]]) + d;
                            let w = f32::from_le_bytes([v[16], v[17], v[18], v[19]]) + d;
                            v[12..16].copy_from_slice(&u.to_le_bytes());
                            v[16..20].copy_from_slice(&w.to_le_bytes());
                        }
                    }
                }
                // Свиньи/коровы/курицы (толпа): анимация и запекание вершин откладываются в параллельную фазу ниже.
                // Мууушрум (грибы после bake) и раненые идут обычным путём.
                Rig::Plain(_)
                    if matches!(
                        mk,
                        Kind::Pig | Kind::Cow | Kind::CowWarm | Kind::CowCold | Kind::Chicken
                    ) && !hurt
                        && *name != "mooshroom" =>
                {
                    par_jobs.push(ParJob {
                        kind: mk,
                        tex,
                        flap: (flap, flap_speed),
                        root,
                        light,
                        pose: (yr, xr, wp, ws),
                    });
                }
                Rig::Plain(model) => {
                    model.reset_pose();
                    match sp.kind {
                        Kind::Creeper => animals::creeper_anim(model, yr, xr, wp, ws),
                        Kind::Spider => animals::spider_anim(model, yr, xr, wp, ws),
                        Kind::IronGolem => animals::iron_golem_anim(model, yr, xr, wp, ws),
                        Kind::Blaze => animals::blaze_anim(model, yr, xr, state.age_in_ticks),
                        Kind::MagmaCube => animals::magma_cube_anim(model, squish),
                        Kind::Ghast | Kind::HappyGhast | Kind::HappyGhastBaby => {
                            animals::ghast_anim(model, state.age_in_ticks)
                        }
                        Kind::Silverfish => animals::silverfish_anim(model, state.age_in_ticks),
                        Kind::Endermite => animals::endermite_anim(model, state.age_in_ticks),
                        Kind::Rabbit => animals::rabbit_anim(model, yr, xr),
                        Kind::Turtle => animals::turtle_anim(model, yr, xr, wp, ws, e.on_ground),
                        Kind::Horse | Kind::Donkey => animals::equine_anim(model, yr, xr, wp, ws),
                        Kind::SnowGolem => animals::snow_golem_anim(model, yr, xr),
                        Kind::CopperGolem => {
                            let ms = |s: Option<u32>| {
                                s.map(|s| {
                                    (anim.arm_clock.saturating_sub(s) as f32 + pt) as i64 * 50
                                })
                            };
                            animals::copper_golem_anim(
                                model,
                                yr,
                                xr,
                                wp,
                                ws,
                                e.held.is_some(),
                                ms(anim.cg_idle),
                                ms(anim.cg_inter).map(|t| (anim.cg_state, t)),
                            )
                        }
                        Kind::Camel => animals::camel_anim(model, yr, xr, wp, ws),
                        Kind::Cod => animals::cod_anim(model, state.age_in_ticks),
                        Kind::Salmon => animals::salmon_anim(model, state.age_in_ticks),
                        Kind::PufferSmall => animals::pufferfish_anim(model, 0, state.age_in_ticks),
                        Kind::PufferMid => animals::pufferfish_anim(model, 1, state.age_in_ticks),
                        Kind::PufferBig => animals::pufferfish_anim(model, 2, state.age_in_ticks),
                        Kind::Dolphin => {
                            animals::dolphin_anim(model, yr, xr, state.age_in_ticks, ws > 0.01)
                        }
                        Kind::Hoglin => animals::hoglin_anim(model, yr, wp, ws),
                        Kind::Guardian => {
                            // VANILLA: GuardianModel#setupAnim — глаз смотрит на камеру
                            // (GuardianRenderer#getEntityToLookAt; цели атаки на клиенте нет).
                            let eye_y = e.pos[1] + 0.425 * sp.mscale as f64;
                            let (vy, vp) = (e.yaw.to_radians(), e.pitch.to_radians());
                            let (vx, vz) = (-vy.sin() * vp.cos(), vy.cos() * vp.cos());
                            let (ex, ez) = (e.pos[0] - cam[0], e.pos[2] - cam[2]);
                            let n = (ex * ex + ez * ez).sqrt().max(1e-9);
                            // delta = normalize(eye - target).yRot(PI/2): (x, z) -> (z, -x).
                            let dot = vx as f64 * (ez / n) + vz as f64 * (-ex / n);
                            let eye = (
                                dot.abs().sqrt() as f32 * 2.0 * (dot.signum() as f32),
                                if cam[1] - eye_y > 0.0 { 0.0 } else { 1.0 },
                            );
                            animals::guardian_anim(
                                model,
                                yr,
                                xr,
                                state.age_in_ticks,
                                gd.0,
                                gd.1,
                                eye,
                            )
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
                            flap,
                            flap_speed,
                            !e.on_ground,
                            e.byte18 & 1 != 0, // TamableAnimal#DATA_FLAGS_ID бит 1 = сидит
                            anim.party,
                            state.age_in_ticks,
                        ),
                        Kind::Frog => animals::frog_anim(model, wp, ws, false),
                        Kind::Axolotl if e.baby => animals::axolotl_baby_anim(
                            model,
                            anim.bx_state,
                            (anim.arm_clock.saturating_sub(anim.bx_start) as f32 + pt) as i64 * 50,
                            wp,
                            ws,
                        ),
                        Kind::Axolotl => animals::axolotl_anim(
                            model,
                            yr,
                            xr,
                            state.age_in_ticks,
                            anim.axolotl_factors(pt),
                        ),
                        Kind::Tadpole => animals::tadpole_anim(model, state.age_in_ticks),
                        Kind::Creaking => {
                            let ms = |s: Option<u32>| {
                                s.map(|s| {
                                    (anim.arm_clock.saturating_sub(s) as f32 + pt) as i64 * 50
                                })
                            };
                            animals::creaking_anim(
                                model,
                                (yr, xr),
                                (wp, ws),
                                e.baby,
                                ms(anim.ck[0]),
                                ms(anim.ck[1]),
                                ms(anim.ck[2]),
                            )
                        }
                        Kind::Nautilus | Kind::NautilusBaby => {
                            animals::nautilus_anim(model, yr, xr, wp, ws, state.age_in_ticks)
                        }
                        Kind::Shulker => animals::shulker_anim(
                            model,
                            xr,
                            yr - 180.0,
                            anim.sh_peek[0] + (anim.sh_peek[1] - anim.sh_peek[0]) * pt,
                            state.age_in_ticks,
                        ),
                        Kind::Sniffer | Kind::SniffletBaby => animals::sniffer_anim(
                            model,
                            (yr, xr),
                            wp,
                            ws,
                            anim.sn_state,
                            (anim.arm_clock.saturating_sub(anim.sn_start) as f32 + pt) as i64 * 50,
                            mk == Kind::SniffletBaby,
                        ),
                        Kind::Armadillo => {
                            let ms = |s: Option<u32>| {
                                s.map(|s| {
                                    ((anim.arm_clock.saturating_sub(s)) as f32 + pt) as i64 * 50
                                })
                            };
                            animals::armadillo_anim(
                                model,
                                yr,
                                xr,
                                wp,
                                ws,
                                anim.armadillo_hiding(),
                                ms(anim.arm_up),
                                ms(anim.arm_out),
                                e.baby,
                            )
                        }
                        Kind::Allay => {
                            animals::allay_anim(model, yr, xr, wp, ws, state.age_in_ticks)
                        }
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
                        Kind::Villager | Kind::VillagerBaby => animals::villager_anim(
                            model,
                            yr,
                            xr,
                            wp,
                            ws,
                            e.int_meta(18).unwrap_or(0) > 0,
                            state.age_in_ticks,
                        ),
                        Kind::Wolf => animals::wolf_anim(model, yr, xr, wp, ws),
                        Kind::Feline => {
                            animals::feline_anim(model, yr, xr, wp, ws, e.flags & 8 != 0)
                        }
                        Kind::Chicken => {
                            animals::chicken_anim(model, yr, xr, wp, ws, flap, flap_speed)
                        }
                        _ => animals::quadruped_anim(model, yr, xr, wp, ws),
                    }
                    let tb = prof.then(std::time::Instant::now);
                    model.bake(root, [255; 4], light, out);
                    if let Some(t) = tb {
                        seg[3] += t.elapsed();
                    }
                    if sp.kind == Kind::CopperGolem {
                        // VANILLA: CopperGolemModel#translateToHand + ItemInHandLayer (hand = правая/левая).
                        let idle = e.int_meta(17).unwrap_or(0) == 0;
                        for (st, bone, left) in [
                            (&e.held, "right_arm", false),
                            (&e.held_off, "left_arm", true),
                        ] {
                            let (Some(st), Some(i)) = (st, model.index(bone)) else {
                                continue;
                            };
                            let hand = if idle {
                                Mat4::from_rotation_y(
                                    (if left { 90.0f32 } else { -90.0 }).to_radians(),
                                ) * Mat4::from_translation(Vec3::new(0.0, 0.0, 0.125))
                            } else {
                                Mat4::from_scale(Vec3::splat(0.55))
                                    * Mat4::from_translation(Vec3::new(-0.125, 0.3125, -0.1875))
                            };
                            self.held_items.push((
                                st.clone(),
                                model.part_matrices(root)[i] * hand,
                                light,
                                left,
                            ));
                        }
                        // VANILLA: BlockDecorationLayer — блок на антенне (applyBlockOnAntennaTransform).
                        if let (Some(st), Some(h)) = (&e.saddle, model.index("head")) {
                            if let Some((n, _)) = crate::handitem::stack_ctx(st) {
                                self.antenna.push((
                                    n.to_string(),
                                    model.part_matrices(root)[h]
                                        * Mat4::from_translation(Vec3::new(0.0, -1.75, 0.0)),
                                    light,
                                ));
                            }
                        }
                    }
                    if *name == "villager" {
                        villager_layers(&mut self.data, model, root, light, e, "villager", 19);
                    }
                    if *name == "mooshroom" && !e.baby {
                        if let Some(h) = model.index("head") {
                            self.mushrooms.push(Mushrooms {
                                root,
                                head: model.part_matrices(root)[h],
                                brown: e.int_meta(18) == Some(1),
                                light,
                            });
                        }
                    }
                    // VANILLA: SpiderEyesLayer / PhantomEyesLayer — EyesLayer, полная яркость.
                    let eyes = match sp.kind {
                        Kind::Spider => Some("spider/spider_eyes"),
                        Kind::Phantom => Some("phantom/phantom_eyes"),
                        _ => None,
                    };
                    if let Some(t) = eyes {
                        model.bake_emissive(root, self.data.entry(t).or_default());
                    }
                    // VANILLA: CopperGolemRenderer — LivingEntityEmissiveLayer (eyes по WeatherState), полная яркость.
                    if sp.kind == Kind::CopperGolem {
                        let t = match e.int_meta(16).unwrap_or(0) {
                            1 => "copper_golem/copper_golem_eyes_exposed",
                            2 => "copper_golem/copper_golem_eyes_weathered",
                            3 => "copper_golem/copper_golem_eyes_oxidized",
                            _ => "copper_golem/copper_golem_eyes",
                        };
                        model.bake_emissive(root, self.data.entry(t).or_default());
                    }
                    // VANILLA: CreakingRenderer — LivingEntityEmissiveLayer (creaking_eyes, только head);
                    // активен: isActive (#17), при разборе — hasGlowingEyes (VANILLA-UNSURE: мерцание не портировано).
                    if sp.kind == Kind::Creaking && (e.bool17 || e.bool18) {
                        model.bake_emissive_parts(
                            root,
                            255,
                            self.data.entry("creaking/creaking_eyes").or_default(),
                            &|n| n == "head",
                        );
                    }
                }
            }
            if prof {
                seg[2] += mark.elapsed();
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
            let t = prof.then(std::time::Instant::now);
            crate::entityshadow::bake(level, pos, cam, shadow_r, 1.0, sky_darken, shadow);
            if let Some(t) = t {
                t_shadow += t.elapsed();
            }
            drawn += 1;
        }
        prof_phase(14, "bake: общий цикл", t_ser.elapsed());
        let t_par = std::time::Instant::now();
        self.bake_parallel(&par_jobs);
        prof_phase(15, "bake: bake_parallel", t_par.elapsed());
        let t_par = prof.then_some(t_par);
        if let Some(t) = t_par {
            seg[3] += t.elapsed();
        }
        if prof {
            prof_report(
                t_all.elapsed(),
                t_shadow,
                seg,
                drawn,
                level.entities.iter().count(),
            );
        }
    }

    /// Простые мобы (свинья/корова/курица): анимация на CPU параллельно (rayon), результат — только
    /// матрицы частей `part_matrices(root)` + свет на экземпляр; вершины считает GPU (`skinned.wgsl`).
    /// Модель клонируется на поток (поза мутируется).
    fn bake_parallel(&mut self, jobs: &[ParJob]) {
        use rayon::prelude::*;
        let mut groups: Vec<((Kind, &'static str), Vec<&ParJob>)> = Vec::new();
        for j in jobs {
            match groups.iter_mut().find(|(k, _)| *k == (j.kind, j.tex)) {
                Some((_, v)) => v.push(j),
                None => groups.push(((j.kind, j.tex), vec![j])),
            }
        }
        const CHUNK: usize = 128;
        for ((kind, _), _) in &groups {
            self.models.entry(*kind).or_insert_with(|| build(*kind));
        }
        for ((kind, tex), list) in groups {
            let Some(Rig::Plain(base)) = self.models.get(&kind) else {
                continue;
            };
            let mesh = self
                .skin_meshes
                .entry(SkinKey::Mob(kind))
                .or_insert_with(|| base.skin_mesh());
            // Экземпляр: parts матриц (16 f32) + свет (block, sky, 0, 0).
            let stride = (mesh.parts * 16 + 4) * 4;
            let out = self.skin_inst.entry((SkinKey::Mob(kind), tex)).or_default();
            let start = out.len();
            out.resize(start + list.len() * stride, 0);
            out[start..]
                .par_chunks_mut(CHUNK * stride)
                .zip(list.par_chunks(CHUNK))
                .for_each_init(
                    || (base.clone(), Vec::<Mat4>::with_capacity(base.parts.len())),
                    |(m, mats), (dst, js)| {
                        for (slot, j) in dst.chunks_exact_mut(stride).zip(js) {
                            m.reset_pose();
                            let (yr, xr, wp, ws) = j.pose;
                            if kind == Kind::Chicken {
                                animals::chicken_anim(m, yr, xr, wp, ws, j.flap.0, j.flap.1);
                            } else {
                                animals::quadruped_anim(m, yr, xr, wp, ws);
                            }
                            m.part_matrices_into(j.root, mats);
                            // Матрицы частей (колонками, f32 LE) + свет (block, sky, 0, 0).
                            let mut o = 0;
                            for x in mats.iter() {
                                for f in x.to_cols_array() {
                                    slot[o..o + 4].copy_from_slice(&f.to_le_bytes());
                                    o += 4;
                                }
                            }
                            slot[o..o + 4].copy_from_slice(&f32::from(j.light.0).to_le_bytes());
                            slot[o + 4..o + 8].copy_from_slice(&f32::from(j.light.1).to_le_bytes());
                        }
                    },
                );
        }
    }

    /// Буфер вершин для текстуры (путь под `textures/entity/`): другие модельные рисовальщики
    /// (сундуки) кладут сюда геометрию после `bake`.
    /// Буфер экземпляров GPU-скиннинга `(key, tex)`; сетку строит `mesh` один раз. Возвращает (байт на экземпляр, буфер).
    /// Экземпляр: parts матриц (16 f32, колонками) + свет (block, sky, 0, 0).
    pub fn skin_out(
        &mut self,
        key: SkinKey,
        tex: &'static str,
        mesh: impl FnOnce() -> ezzmc_render::entitymodel::SkinMesh,
    ) -> (usize, &mut Vec<u8>) {
        let parts = self.skin_meshes.entry(key).or_insert_with(mesh).parts;
        (
            (parts * 16 + 4) * 4,
            self.skin_inst.entry((key, tex)).or_default(),
        )
    }

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
        // GPU-скиннинг: экземпляры толпы.
        for (&(key, tex), inst) in &self.skin_inst {
            let Some(Some(d)) = (if inst.is_empty() && !self.skin_draws.contains_key(&(key, tex)) {
                None
            } else {
                Some(self.draws.entry(tex).or_insert_with(|| make(tex)))
            }) else {
                continue;
            };
            let Some(mesh) = self.skin_meshes.get(&key) else {
                continue;
            };
            let sd = self
                .skin_draws
                .entry((key, tex))
                .or_insert_with(|| d.new_skinned(device, mesh));
            sd.prepare(device, queue, inst);
            if !inst.is_empty() {
                d.set_model_view(queue, mv);
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
        // Узоры баннера растут наружу (grow) и пишут глубину: база и заливка обязаны идти раньше,
        // иначе порядок HashMap даёт просветы сквозь узор.
        let rank = |k: &str| match k {
            "banner/banner_base" => 1,
            "banner/base" => 2,
            SWIRL_TEX => 8,
            // Слои жителя лежат в одной плоскости с базой и идут поверх: тип → профессия → уровень.
            _ if k.starts_with("villager/type/") => 4,
            _ if k.starts_with("villager/profession/") => 5,
            _ if k.starts_with("villager/profession_level/") => 6,
            _ if k.starts_with("banner/") => 3,
            _ => 0,
        };
        let mut order: Vec<_> = self.draws.iter().filter(|(k, _)| **k != MASK_TEX).collect();
        order.sort_by_key(|(k, _)| rank(k));
        for (tex, d) in order {
            d.iter().for_each(|d| d.draw(pass));
            if let Some(base) = d {
                for ((_, t), sd) in &self.skin_draws {
                    if t == tex {
                        sd.draw(pass, base);
                    }
                }
            }
        }
        // Заплатка — после моделей: иначе она закрыла бы дно лодки по глубине.
        if let Some(Some(d)) = self.draws.get(MASK_TEX) {
            d.draw(pass);
        }
    }
}

/// VANILLA: egg.distToCenterSqr(position) → sqrt; egg = Heightmap MOTION_BLOCKING_NO_LEAVES над подиумом (0, 0).
fn dragon_egg_dist(level: &Level, pos: [f64; 3]) -> f64 {
    let top = (-64..=320)
        .rev()
        .find(|&y| {
            let b = level.chunks.block(0, y, 0);
            !b.is_air() && !b.name().ends_with("_leaves")
        })
        .map_or(64, |y| y + 1);
    let d = [0.5 - pos[0], f64::from(top) + 0.5 - pos[1], 0.5 - pos[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// PROF: время `Mobs::prepare` (загрузка вершин в GPU), раз в 120 кадров.
pub fn prof_prepare(d: std::time::Duration) {
    if !prof_on() {
        return;
    }
    use std::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    if N.fetch_add(1, Ordering::Relaxed) % 120 == 0 {
        tracing::info!("PROF mobs.prepare: {:.2} мс", d.as_secs_f64() * 1e3);
    }
}

/// PROF: среднее время именованной фазы кадра, раз в 120 вызовов (EZZMC_PROF=1). `slot` 0..8.
pub fn prof_phase(slot: usize, name: &str, d: std::time::Duration) {
    if !prof_on() {
        return;
    }
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
    static ACC: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24];
    static N: [AtomicU64; 24] = [const { AtomicU64::new(0) }; 24];
    ACC[slot].fetch_add(d.as_micros() as u64, Relaxed);
    if N[slot].fetch_add(1, Relaxed) + 1 >= 120 {
        let n = N[slot].swap(0, Relaxed).max(1);
        let a = ACC[slot].swap(0, Relaxed) as f64 / n as f64;
        tracing::info!("PROF phase {name}: {:.2} мс", a / 1000.0);
    }
}

/// PROF: время от начала кадра (метка `prof_frame`) до этой точки, фаза `slot` 5..8 (не более 3 меток).
pub fn prof_at(slot: usize, name: &str) {
    if !prof_on() {
        return;
    }
    let (start, last) = prof_clock();
    let now = start.elapsed().as_micros() as u64;
    prof_phase(
        slot,
        name,
        std::time::Duration::from_micros(now.saturating_sub(last)),
    );
}

fn prof_clock() -> (&'static std::time::Instant, u64) {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    (
        START.get_or_init(std::time::Instant::now),
        PROF_LAST.load(std::sync::atomic::Ordering::Relaxed),
    )
}
static PROF_LAST: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// PROF: период между кадрами (мс), фаза 4.
pub fn prof_frame() {
    if !prof_on() {
        return;
    }
    use std::sync::atomic::Ordering::Relaxed;
    let now = prof_clock().0.elapsed().as_micros() as u64;
    let prev = PROF_LAST.swap(now, Relaxed);
    if prev != 0 {
        prof_phase(
            4,
            "период кадра",
            std::time::Duration::from_micros(now - prev),
        );
    }
}

fn prof_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("EZZMC_PROF").is_some())
}

/// Копит время и раз в 120 вызовов пишет среднее за кадр (мс).
fn prof_report(
    total: std::time::Duration,
    shadow: std::time::Duration,
    seg: [std::time::Duration; 4],
    drawn: u32,
    all: usize,
) {
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
    static ACC: [AtomicU64; 7] = [const { AtomicU64::new(0) }; 7];
    static N: AtomicU64 = AtomicU64::new(0);
    ACC[0].fetch_add(total.as_micros() as u64, Relaxed);
    ACC[1].fetch_add(shadow.as_micros() as u64, Relaxed);
    ACC[2].fetch_add(u64::from(drawn), Relaxed);
    for (i, d) in seg.iter().enumerate() {
        ACC[3 + i].fetch_add(d.as_micros() as u64, Relaxed);
    }
    if N.fetch_add(1, Relaxed) + 1 >= 120 {
        let n = N.swap(0, Relaxed).max(1);
        let a = |i: usize| ACC[i].swap(0, Relaxed) as f64 / n as f64;
        tracing::info!(
            "PROF mobs.bake: total {:.2} мс | тени {:.2} | до анимации(spec/tick/свет) {:.2} | root+текстура {:.2} | setup_anim+bake модели {:.2} (из них только Model::bake {:.2}) | нарисовано {:.0} из {}",
            a(0) / 1000.0,
            a(1) / 1000.0,
            a(3) / 1000.0,
            a(4) / 1000.0,
            a(5) / 1000.0,
            a(6) / 1000.0,
            a(2),
            all
        );
    }
}
