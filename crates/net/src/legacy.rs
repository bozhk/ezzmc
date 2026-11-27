//! Трансляция тел пакетов сервера старой версии в формат канона (26.3) — глава 18, TODO-372.
//! Весь клиент разбирает только канонический формат; здесь исправляются расхождения
//! между версиями. Реестровые id (предметы, сущности, звуки, частицы) — позже в этом же месте.
use ezzmc_protocol::item::{
    modifier_id_1_21, ClientRegistry, Shape, PARTICLE, PARTICLE_1_21_11, PARTICLE_1_21_3, SOUND,
};
use ezzmc_protocol::translate::Translator;
use ezzmc_protocol::version::{
    build_remap, Version, CANON, V1_20_4, V1_20_6, V1_21_1, V1_21_10, V1_21_11, V1_21_3, V1_21_4,
    V1_21_5, V1_21_8,
};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, ProtocolResult, State};
use std::collections::HashMap;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::{Mutex, OnceLock};

/// Версия соединения среди 1.21.5–1.21.11: 0 — 1.21.6–1.21.8 (771/772: пакеты, сущности и
/// block_entity_type совпадают; предметы/звуки — в `Translator`), 1 — 1.21.9/1.21.10 (773),
/// 2 — 1.21.11 (774), 3 — 1.21.5 (770: свои id пакетов и сущностей, форматы как у 1.21.8),
/// 4 — 1.21.4 (769: форматы 1.21.5 плюс отличия ниже), 5 — 1.21.2/1.21.3 (768: как 1.21.4
/// плюс свои отличия), 6 — 1.21/1.21.1 (767: как 1.21.3 плюс отличия до 1.21.2), 7 — 1.20.5/1.20.6
/// (766: как 1.21.1 плюс отличия до 1.21), 8 — 1.20.3/1.20.4 (765: как 1.20.6 плюс отличия до
/// 1.20.5: предметы с NBT, нет компонентов).
fn ver() -> usize {
    match ezzmc_protocol::version::current().protocol {
        765 => 8,
        766 => 7,
        767 => 6,
        768 => 5,
        769 => 4,
        770 => 3,
        771 | 772 => 0,
        773 => 1,
        _ => 2,
    }
}
const VERSIONS: [&Version; 9] = [
    &V1_21_8, &V1_21_10, &V1_21_11, &V1_21_5, &V1_21_4, &V1_21_3, &V1_21_1, &V1_20_6, &V1_20_4,
];

/// Форматы до 1.21.9 (1.21.5–1.21.8): скорости short, BlockPos спавна без измерения и т. п.
fn pre_1_21_9(k: usize) -> bool {
    matches!(k, 0 | 3..=8)
}

/// Форматы до 1.21.6 (1.21–1.21.5): PRESS/RELEASE_SHIFT_KEY, нет HangingEntity#DATA_DIRECTION.
fn pre_1_21_6(k: usize) -> bool {
    matches!(k, 3..=8)
}

/// Форматы до 1.21.5 (1.21–1.21.4): add_experience_orb, showInTooltip, NBT-карты высот и т. п.
fn pre_1_21_5(k: usize) -> bool {
    matches!(k, 4..=8)
}

/// Форматы до 1.21.2 (1.20.5–1.21.1): id контейнеров байтом, старые позиция/телепорт/взрыв.
fn pre_1_21_2(k: usize) -> bool {
    matches!(k, 6..=8)
}

/// Форматы до 1.21 (1.20.5/1.20.6): модификаторы атрибутов с UUID, use_item без поворота.
fn pre_1_21(k: usize) -> bool {
    k >= 7
}

/// 1.20.3/1.20.4: до компонентов предметов (1.20.5).
fn pre_1_20_5(k: usize) -> bool {
    k == 8
}

/// Стек предмета сервера → канон: 1.20.4 — NBT, новее — компоненты через `Translator`.
fn stack(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if pre_1_20_5(ver()) {
        crate::v1201::item_to_canon_1_20_4(r, w)
    } else {
        tr().stack(r, w)
    }
}

/// Форма частиц версии соединения: 1.21–1.21.3 — `trail` без длительности (в 1.21.1 его нет).
fn particle_src() -> &'static Shape {
    if matches!(ver(), 5..=8) {
        &PARTICLE_1_21_3
    } else {
        &PARTICLE_1_21_11
    }
}

fn entity_type_map() -> &'static [Option<u32>] {
    static T: [OnceLock<Vec<Option<u32>>>; 9] = [const { OnceLock::new() }; 9];
    let k = ver();
    T[k].get_or_init(|| {
        let mut t = build_remap(VERSIONS[k].entity_type, CANON.entity_type);
        // VANILLA: EntityType (1.21.2) — лодка/лодка с сундуком разделены по породе дерева.
        // VANILLA-UNSURE: порода из Boat#DATA_ID_TYPE (1.21.1) не учитывается — всегда дуб.
        let pos = |name: &str| CANON.entity_type.iter().position(|n| *n == name);
        for (i, n) in VERSIONS[k].entity_type.iter().enumerate() {
            match *n {
                "boat" => t[i] = pos("oak_boat").map(|c| c as u32),
                "chest_boat" => t[i] = pos("oak_chest_boat").map(|c| c as u32),
                _ => {}
            }
        }
        // VANILLA: CreakingTransient (1.21.2/1.21.3) — скрипун сердца; в 1.21.4 слит в creaking.
        let creaking = CANON.entity_type.iter().position(|n| *n == "creaking");
        for (i, n) in VERSIONS[k].entity_type.iter().enumerate() {
            if *n == "creaking_transient" {
                t[i] = creaking.map(|c| c as u32);
            }
        }
        t
    })
}

/// Реестр block_entity_type версии соединения → канон (в 26.x убран `bed`, добавлен `potent_sulfur`).
fn block_entity_type(id: i32) -> Option<i32> {
    use ezzmc_protocol::generated::{
        r1_20_4, r1_20_6, r1_21_1, r1_21_10, r1_21_11, r1_21_3, r1_21_4, r1_21_5, r1_21_8, r26_3,
    };
    const SRC: [&[&str]; 9] = [
        r1_21_8::BLOCK_ENTITY_TYPE,
        r1_21_10::BLOCK_ENTITY_TYPE,
        r1_21_11::BLOCK_ENTITY_TYPE,
        r1_21_5::BLOCK_ENTITY_TYPE,
        r1_21_4::BLOCK_ENTITY_TYPE,
        r1_21_3::BLOCK_ENTITY_TYPE,
        r1_21_1::BLOCK_ENTITY_TYPE,
        r1_20_6::BLOCK_ENTITY_TYPE,
        r1_20_4::BLOCK_ENTITY_TYPE,
    ];
    static T: [OnceLock<Vec<Option<u32>>>; 9] = [const { OnceLock::new() }; 9];
    let k = ver();
    T[k].get_or_init(|| build_remap(SRC[k], r26_3::BLOCK_ENTITY_TYPE))
        .get(usize::try_from(id).ok()?)
        .copied()
        .flatten()
        .map(|v| v as i32)
}

/// Измерение игрока (из `login`/`respawn`) — нужно для `set_default_spawn_position` 1.21.8,
/// где GlobalPos ещё нет.
fn dimension() -> std::sync::MutexGuard<'static, String> {
    static D: Mutex<String> = Mutex::new(String::new());
    D.lock().unwrap_or_else(|e| e.into_inner())
}

/// VANILLA: CommonPlayerSpawnInfo (1.21.8 = 1.21.10): holder dimension_type, ключ измерения, …
fn remember_dimension(r: &mut Reader) -> ProtocolResult<()> {
    r.var_int()?;
    *dimension() = r.string(32767)?;
    Ok(())
}

/// Сдвиг индексов полей SynchedEntityData 1.21.11 → 26.3 по классу сущности.
#[derive(Clone, Copy, PartialEq, Debug)]
enum DataLayout {
    Same,
    /// VANILLA: AgeableMob#defineSynchedData (26.3): после DATA_BABY_ID (#16) добавлен AGE_LOCKED (#17).
    Ageable,
    /// Cat: AgeableMob + DATA_SOUND_VARIANT_ID после DATA_VARIANT_ID (#20 в 26.3).
    Cat,
    /// Slime/MagmaCube: в 26.3 AbstractCubeMob extends AgeableMob (+DATA_BABY_ID, AGE_LOCKED перед ID_SIZE).
    CubeMob,
}

impl DataLayout {
    fn of(canon_type: &str) -> Self {
        // Наследники AgeableMob и в 1.21.11, и в 26.3 (по EntityType/EntityTypes обеих версий).
        const AGEABLE: &[&str] = &[
            "armadillo",
            "axolotl",
            "bee",
            "camel",
            "camel_husk",
            "chicken",
            "cow",
            "dolphin",
            "donkey",
            "fox",
            "frog",
            "glow_squid",
            "goat",
            "happy_ghast",
            "hoglin",
            "horse",
            "llama",
            "mooshroom",
            "mule",
            "nautilus",
            "ocelot",
            "panda",
            "parrot",
            "pig",
            "polar_bear",
            "rabbit",
            "sheep",
            "skeleton_horse",
            "sniffer",
            "squid",
            "strider",
            "trader_llama",
            "turtle",
            "villager",
            "wandering_trader",
            "wolf",
            "zombie_horse",
            "zombie_nautilus",
        ];
        match canon_type {
            "cat" => Self::Cat,
            "slime" | "magma_cube" => Self::CubeMob,
            t if AGEABLE.contains(&t) => Self::Ageable,
            _ => Self::Same,
        }
    }

    fn index(self, i: u8) -> u8 {
        let shift = match self {
            Self::Same => 0,
            Self::Ageable => u8::from(i >= 17),
            Self::Cat => u8::from(i >= 17) + u8::from(i >= 20),
            Self::CubeMob => 2 * u8::from(i >= 16),
        };
        i.saturating_add(shift)
    }
}

/// Классы сущностей текущего соединения (id → раскладка, канонический тип), заполняется по add_entity.
fn layouts() -> std::sync::MutexGuard<'static, HashMap<i32, (DataLayout, &'static str)>> {
    static M: OnceLock<Mutex<HashMap<i32, (DataLayout, &'static str)>>> = OnceLock::new();
    M.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// VANILLA: EntityDataSerializers — порядок регистрации 1.21.11 (id = индекс).
const SERIALIZERS_1_21_11: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "PARTICLE",
    "PARTICLES",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "COW_VARIANT",
    "WOLF_VARIANT",
    "WOLF_SOUND_VARIANT",
    "FROG_VARIANT",
    "PIG_VARIANT",
    "CHICKEN_VARIANT",
    "ZOMBIE_NAUTILUS_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "ARMADILLO_STATE",
    "COPPER_GOLEM_STATE",
    "WEATHERING_COPPER_STATE",
    "VECTOR3",
    "QUATERNION",
    "RESOLVABLE_PROFILE",
    "HUMANOID_ARM",
];
/// VANILLA: EntityDataSerializers — порядок регистрации 1.21.10 (нет ZOMBIE_NAUTILUS_VARIANT, HUMANOID_ARM).
const SERIALIZERS_1_21_10: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "PARTICLE",
    "PARTICLES",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "COW_VARIANT",
    "WOLF_VARIANT",
    "WOLF_SOUND_VARIANT",
    "FROG_VARIANT",
    "PIG_VARIANT",
    "CHICKEN_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "ARMADILLO_STATE",
    "COPPER_GOLEM_STATE",
    "WEATHERING_COPPER_STATE",
    "VECTOR3",
    "QUATERNION",
    "RESOLVABLE_PROFILE",
];

/// VANILLA: EntityDataSerializers — порядок регистрации 1.21.8 (есть COMPOUND_TAG; нет
/// COPPER_GOLEM_STATE, WEATHERING_COPPER_STATE, RESOLVABLE_PROFILE).
const SERIALIZERS_1_21_8: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "COMPOUND_TAG",
    "PARTICLE",
    "PARTICLES",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "COW_VARIANT",
    "WOLF_VARIANT",
    "WOLF_SOUND_VARIANT",
    "FROG_VARIANT",
    "PIG_VARIANT",
    "CHICKEN_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "ARMADILLO_STATE",
    "VECTOR3",
    "QUATERNION",
];

/// VANILLA: EntityDataSerializers — порядок регистрации 1.21.4 (нет COW/WOLF_SOUND/PIG/
/// CHICKEN_VARIANT; OPTIONAL_UUID с тем же форматом, что OPTIONAL_LIVING_ENTITY_REFERENCE).
const SERIALIZERS_1_21_4: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "COMPOUND_TAG",
    "PARTICLE",
    "PARTICLES",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "WOLF_VARIANT",
    "FROG_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "ARMADILLO_STATE",
    "VECTOR3",
    "QUATERNION",
];

/// VANILLA: EntityDataSerializers — порядок регистрации 1.20.4 (нет PARTICLES, WOLF_VARIANT,
/// ARMADILLO_STATE — добавлены в 1.20.5).
const SERIALIZERS_1_20_4: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "COMPOUND_TAG",
    "PARTICLE",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "FROG_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "VECTOR3",
    "QUATERNION",
];

/// Поле сущности 1.21.4 → индекс 1.21.5 (`None` — поля в 1.21.5 нет, значение пропускается).
/// VANILLA: 1.21.4 → 1.21.5 — седло свиньи/страйдера ушло в снаряжение, у дельфина и черепахи
/// убраны позиции (TREASURE_POS, HOME_POS, TRAVEL_POS) и флаги пути, у вагонеток блок-витрина
/// стал OPTIONAL_BLOCK_STATE без флага CUSTOM_DISPLAY. Базовые поля Animal — с 17.
/// Индекс поля 1.21.1 → 1.21.3 (канонический тип сущности). VANILLA (1.21.2): AbstractArrow
/// +IN_GROUND (#10); Squid/GlowSquid/Dolphin — AgeableWaterCreature (+DATA_BABY_ID #16);
/// AbstractBoat без DATA_ID_TYPE (#11, порода стала типом сущности).
fn index_1_21_1(ty: &str, idx: u8) -> Option<u8> {
    match (ty, idx) {
        ("arrow", 10) | ("trident", 10 | 11) => Some(idx + 1),
        ("glow_squid" | "dolphin", 16..) => Some(idx + 1),
        (t, 11) if t.ends_with("boat") || t.ends_with("raft") => None,
        (t, 12..=14) if t.ends_with("boat") || t.ends_with("raft") => Some(idx - 1),
        _ => Some(idx),
    }
}

fn index_1_21_4(ty: &str, idx: u8) -> Option<u8> {
    match (ty, idx) {
        ("pig", 17) => None,
        ("pig", 18) => Some(17),
        ("strider", 19) => None,
        ("dolphin", 17) => None,
        ("dolphin", 18 | 19) => Some(idx - 1),
        ("turtle", 17 | 20..=22) => None,
        ("turtle", 18 | 19) => Some(idx - 1),
        (t, 13) if t.ends_with("minecart") => None,
        _ => Some(idx),
    }
}

/// Индекс поля игрока 1.21.8 → 1.21.10. VANILLA: Player (1.21.8): 15 absorption, 16 score,
/// 17 customisation, 18 main_hand, 19/20 shoulder (COMPOUND_TAG); в 1.21.9 Avatar
/// (15 main_hand, 16 customisation), Player (17 absorption, 18 score, 19/20 shoulder parrot).
fn player_index_1_21_8(idx: u8) -> u8 {
    match idx {
        15 => 17,
        16 => 18,
        17 => 16,
        18 => 15,
        i => i,
    }
}

/// VANILLA: Player#convertParrotVariant (1.21.9) — плечевой compound 1.21.8 (`id` попугая,
/// `Variant` int) → OptionalInt; OPTIONAL_UNSIGNED_INT: 0 — пусто, иначе вариант + 1.
fn shoulder_parrot(tag: Option<&ezzmc_protocol::nbt::Tag>) -> i32 {
    use ezzmc_protocol::nbt::Tag;
    let Some(t) = tag else { return 0 };
    let is_parrot = matches!(
        t.get("id").and_then(Tag::as_str),
        Some("minecraft:parrot" | "parrot")
    );
    if !is_parrot {
        return 0;
    }
    let v = t.get("Variant").and_then(Tag::as_i64).unwrap_or(0);
    i32::try_from(v).unwrap_or(0).clamp(0, 4) + 1
}

/// Поля 1.21.10, сменившие сериализатор в 1.21.11: (канонический тип сущности, индекс 1.21.10).
/// VANILLA: Avatar#DATA_PLAYER_MAIN_HAND (1.21.10: BYTE 0 = левая, 1 = правая; 1.21.11: HUMANOID_ARM).
/// VANILLA: Wolf#DATA_REMAINING_ANGER_TIME (#21), Bee (#18) (1.21.10: INT тиков; 1.21.11: LONG
/// DATA_ANGER_END_TIME, −1 = не зол).
fn retyped_1_21_10(ty: &str, idx: u8) -> Option<&'static str> {
    match (ty, idx) {
        ("player" | "mannequin", 15) => Some("HUMANOID_ARM"),
        ("wolf", 21) | ("bee", 18) => Some("ANGER_END_TIME"),
        _ => None,
    }
}

/// VANILLA: EntityDataSerializers (26.3).
const SERIALIZERS_26_3: &[&str] = &[
    "BYTE",
    "INT",
    "LONG",
    "FLOAT",
    "STRING",
    "COMPONENT",
    "OPTIONAL_COMPONENT",
    "ITEM_STACK",
    "BOOLEAN",
    "ROTATIONS",
    "BLOCK_POS",
    "OPTIONAL_BLOCK_POS",
    "DIRECTION",
    "OPTIONAL_LIVING_ENTITY_REFERENCE",
    "BLOCK_STATE",
    "OPTIONAL_BLOCK_STATE",
    "PARTICLE",
    "PARTICLES",
    "VILLAGER_DATA",
    "OPTIONAL_UNSIGNED_INT",
    "POSE",
    "CAT_VARIANT",
    "CAT_SOUND_VARIANT",
    "COW_VARIANT",
    "COW_SOUND_VARIANT",
    "WOLF_VARIANT",
    "WOLF_SOUND_VARIANT",
    "FROG_VARIANT",
    "PIG_VARIANT",
    "PIG_SOUND_VARIANT",
    "CHICKEN_VARIANT",
    "CHICKEN_SOUND_VARIANT",
    "ZOMBIE_NAUTILUS_VARIANT",
    "OPTIONAL_GLOBAL_POS",
    "PAINTING_VARIANT",
    "SNIFFER_STATE",
    "ARMADILLO_STATE",
    "COPPER_GOLEM_STATE",
    "WEATHERING_COPPER_STATE",
    "VECTOR3",
    "QUATERNION",
    "RESOLVABLE_PROFILE",
    "HUMANOID_ARM",
    "DYE_COLOR",
];

/// VANILLA: PaintingVariant.STREAM_CODEC — holder (id+1 | 0 + DIRECT: w, h, assetId, title?, author?).
static PAINTING: Shape = Shape::Holder(&Shape::Struct(&[
    Shape::VarInt,
    Shape::VarInt,
    Shape::Utf(32767),
    Shape::Optional(&Shape::Nbt),
    Shape::Optional(&Shape::Nbt),
]));

/// Значение SynchedEntityData 1.21.11 → канон. `false` — значение не переводится (запись выбросить).
fn entity_data_value(r: &mut Reader, w: &mut Writer, ser: &str) -> ProtocolResult<bool> {
    let state = ezzmc_world::legacy::state_to_canon;
    let same = |r: &mut Reader, w: &mut Writer, s: &'static Shape| tr().value(r, w, s, s);
    match ser {
        "BYTE" | "BOOLEAN" => {
            w.u8(r.u8()?);
        }
        "INT"
        | "DIRECTION"
        | "OPTIONAL_UNSIGNED_INT"
        | "POSE"
        | "SNIFFER_STATE"
        | "ARMADILLO_STATE"
        | "COPPER_GOLEM_STATE"
        | "WEATHERING_COPPER_STATE"
        | "HUMANOID_ARM"
        | "CAT_VARIANT"
        | "COW_VARIANT"
        | "WOLF_VARIANT"
        | "WOLF_SOUND_VARIANT"
        | "FROG_VARIANT"
        | "PIG_VARIANT"
        | "CHICKEN_VARIANT"
        | "ZOMBIE_NAUTILUS_VARIANT" => {
            w.var_int(r.var_int()?);
        }
        "LONG" => {
            w.var_long(r.var_long()?);
        }
        "FLOAT" => {
            w.f32(r.f32()?);
        }
        "STRING" => {
            w.string(&r.string(32767)?);
        }
        "COMPONENT" => return same(r, w, &Shape::Nbt),
        "OPTIONAL_COMPONENT" => return same(r, w, &Shape::Optional(&Shape::Nbt)),
        "ITEM_STACK" => stack(r, w)?,
        "ROTATIONS" | "VECTOR3" => {
            w.bytes(r.bytes(12)?);
        }
        "QUATERNION" => {
            w.bytes(r.bytes(16)?);
        }
        "BLOCK_POS" => {
            w.bytes(r.bytes(8)?);
        }
        "OPTIONAL_BLOCK_POS" => {
            let b = r.bool()?;
            w.bool(b);
            if b {
                w.bytes(r.bytes(8)?);
            }
        }
        "OPTIONAL_LIVING_ENTITY_REFERENCE" => {
            let b = r.bool()?;
            w.bool(b);
            if b {
                w.bytes(r.bytes(16)?);
            }
        }
        // VANILLA: OPTIONAL_BLOCK_STATE_CODEC — 0 = пусто, иначе id состояния.
        "BLOCK_STATE" | "OPTIONAL_BLOCK_STATE" => {
            let v = r.var_int()?;
            let empty = ser == "OPTIONAL_BLOCK_STATE" && v == 0;
            w.var_int(if empty { 0 } else { state(v as u32) as i32 });
        }
        "PARTICLE" => return tr().value(r, w, particle_src(), &PARTICLE),
        "PARTICLES" => {
            let n = r.length("particles", 1 << 12)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                if tr().value(r, &mut list, particle_src(), &PARTICLE)? {
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec());
        }
        // VANILLA: VillagerData.STREAM_CODEC — type, profession (реестры совпадают), level.
        "VILLAGER_DATA" => {
            w.var_int(r.var_int()?)
                .var_int(r.var_int()?)
                .var_int(r.var_int()?);
        }
        "OPTIONAL_GLOBAL_POS" => {
            let b = r.bool()?;
            w.bool(b);
            if b {
                w.string(&r.string(32767)?).bytes(r.bytes(8)?);
            }
        }
        "PAINTING_VARIANT" => return same(r, w, &PAINTING),
        "RESOLVABLE_PROFILE" => return same(r, w, &ezzmc_protocol::item::RESOLVABLE_PROFILE),
        _ => {
            return Err(ezzmc_protocol::ProtocolError::Other(format!(
                "serializer {ser}"
            )))
        }
    }
    Ok(true)
}

const MAX_SLOT_DEPTH: u32 = 16;

fn str_id(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    w.string(&r.string(32767)?);
    Ok(())
}

/// VANILLA: SlotDisplay.STREAM_CODEC 1.21.11 → 26.3. Типы реестра сдвинуты (в 26.3 добавлены
/// with_any_potion, only_with_component, dyed); `item_stack` — ItemStack → ItemStackTemplate;
/// `tag` — TagKey → HolderSet. Непереводимый предмет становится `empty`.
fn slot_display(r: &mut Reader, w: &mut Writer, depth: u32) -> ProtocolResult<()> {
    if depth > MAX_SLOT_DEPTH {
        return Err(ezzmc_protocol::ProtocolError::InvalidValue {
            what: "slot display depth",
            value: depth as i64,
        });
    }
    let d = depth + 1;
    match r.var_int()? {
        t @ (0 | 1) => {
            w.var_int(t);
        }
        2 => match tr().id(ClientRegistry::Item, r.var_int()?) {
            Some(i) => {
                w.var_int(4).var_int(i);
            }
            None => {
                w.var_int(0);
            }
        },
        3 => {
            let mut t = Writer::new();
            if tr().value(r, &mut t, &Shape::Stack, &Shape::ItemTemplate)? {
                w.var_int(5).bytes(&t.into_vec());
            } else {
                w.var_int(0);
            }
        }
        // HolderSet: 0 — имя тега.
        4 => {
            w.var_int(6).var_int(0);
            str_id(r, w)?;
        }
        5 => {
            w.var_int(8);
            slot_display(r, w, d)?;
            slot_display(r, w, d)?;
            // Holder<TrimPattern>: id+1; 0 — встроенный объект (сервер так не шлёт).
            let h = r.var_int()?;
            if h == 0 {
                return Err(ezzmc_protocol::ProtocolError::InvalidValue {
                    what: "inline trim pattern",
                    value: 0,
                });
            }
            w.var_int(h);
        }
        6 => {
            w.var_int(9);
            slot_display(r, w, d)?;
            slot_display(r, w, d)?;
        }
        7 => {
            let n = r.length("composite slot", 4096)?;
            w.var_int(10).var_int(n as i32);
            for _ in 0..n {
                slot_display(r, w, d)?;
            }
        }
        t => {
            return Err(ezzmc_protocol::ProtocolError::InvalidValue {
                what: "slot display type",
                value: t as i64,
            })
        }
    }
    Ok(())
}

fn slot_displays(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let n = r.length("slot list", 4096)?;
    w.var_int(n as i32);
    for _ in 0..n {
        slot_display(r, w, 0)?;
    }
    Ok(())
}

/// VANILLA: RecipeDisplay.STREAM_CODEC — типы и поля совпадают, переводятся SlotDisplay.
fn recipe_display(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let t = r.var_int()?;
    w.var_int(t);
    let slots = match t {
        // crafting_shapeless: ingredients, result, station
        0 => {
            slot_displays(r, w)?;
            2
        }
        // crafting_shaped: w, h, ingredients, result, station
        1 => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            slot_displays(r, w)?;
            2
        }
        // furnace: ingredient, fuel, result, station, duration, experience
        2 => {
            for _ in 0..4 {
                slot_display(r, w, 0)?;
            }
            w.var_int(r.var_int()?).f32(r.f32()?);
            0
        }
        // stonecutter: input, result, station
        3 => 3,
        // smithing: template, base, addition, result, station
        4 => 5,
        t => {
            return Err(ezzmc_protocol::ProtocolError::InvalidValue {
                what: "recipe display type",
                value: t as i64,
            })
        }
    };
    for _ in 0..slots {
        slot_display(r, w, 0)?;
    }
    Ok(())
}

/// VANILLA: RecipeDisplayEntry.STREAM_CODEC — id, display, OPTIONAL_VAR_INT group, category,
/// Optional<List<Ingredient>> (HolderSet предметов).
fn recipe_entry(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    w.var_int(r.var_int()?);
    recipe_display(r, w)?;
    w.var_int(r.var_int()?).var_int(r.var_int()?);
    let some = r.bool()?;
    w.bool(some);
    if some {
        let n = r.length("ingredients", 4096)?;
        w.var_int(n as i32);
        for _ in 0..n {
            match r.var_int()? {
                0 => {
                    w.var_int(0);
                    str_id(r, w)?;
                }
                k => {
                    let m = (k - 1).max(0) as usize;
                    if m > r.remaining() {
                        return Err(ezzmc_protocol::ProtocolError::InvalidValue {
                            what: "holder set length",
                            value: m as i64,
                        });
                    }
                    let mut v = Vec::with_capacity(m.min(r.remaining()));
                    for _ in 0..m {
                        if let Some(i) = tr().id(ClientRegistry::Item, r.var_int()?) {
                            v.push(i);
                        }
                    }
                    w.var_int(v.len() as i32 + 1);
                    for i in v {
                        w.var_int(i);
                    }
                }
            }
        }
    }
    Ok(())
}

fn state_id(id: i32) -> i32 {
    ezzmc_world::legacy::state_to_canon(id.max(0) as u32) as i32
}

fn tr() -> &'static Translator {
    ezzmc_protocol::translate::from_server(ezzmc_world::legacy::state_to_canon)
}

fn tr_back() -> &'static Translator {
    ezzmc_protocol::translate::to_server(ezzmc_world::legacy::state_from_canon)
}

/// VANILLA: HashedStack.STREAM_CODEC — Optional(item, count, HashedPatchMap(added: id→int, removed: ids)),
/// канон → 1.21.11. Непереводимый предмет — пустой стек (сервер пришлёт верное состояние).
fn hashed_stack_back(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.bool(false);
        return Ok(());
    }
    let item = tr_back().id(ClientRegistry::Item, r.var_int()?);
    let count = r.var_int()?;
    let n = r.length("hashed components", 256)?;
    let mut added = Vec::with_capacity(n);
    for _ in 0..n {
        let (id, hash) = (r.var_int()?, r.i32()?);
        if let Some(c) = tr_back().component(id) {
            added.push((c, hash));
        }
    }
    let n = r.length("removed components", 256)?;
    let mut removed = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(c) = tr_back().component(r.var_int()?) {
            removed.push(c);
        }
    }
    let Some(item) = item else {
        w.bool(false);
        return Ok(());
    };
    w.bool(true).var_int(item).var_int(count);
    w.var_int(added.len() as i32);
    for (c, h) in added {
        w.var_int(c).i32(h);
    }
    w.var_int(removed.len() as i32);
    for c in removed {
        w.var_int(c);
    }
    Ok(())
}

/// VANILLA: ClientboundSetEntityDataPacket (1.21.8–1.21.11) → 26.3; `k` — индекс [`ver`].
fn set_entity_data(r: &mut Reader, w: &mut Writer, k: usize) -> ProtocolResult<()> {
    let id = r.var_int()?;
    let (layout, ty) = layouts()
        .get(&id)
        .copied()
        .unwrap_or((DataLayout::Same, ""));
    let table = [
        SERIALIZERS_1_21_8,
        SERIALIZERS_1_21_10,
        SERIALIZERS_1_21_11,
        SERIALIZERS_1_21_8,
        SERIALIZERS_1_21_4,
        SERIALIZERS_1_21_4,
        SERIALIZERS_1_21_4,
        SERIALIZERS_1_21_4,
        SERIALIZERS_1_20_4,
    ][k];
    // VANILLA: HangingEntity#DATA_DIRECTION (#8) появился в 1.21.6 — поля рамок/картин 1.21.5
    // с #8 сдвигаются на 1 (направление приходит в add_entity, как и в 1.21.6).
    let hanging = pre_1_21_6(k) && matches!(ty, "item_frame" | "glow_item_frame" | "painting");
    let player = ty == "player";
    let canon_of = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap_or(0);
    w.var_int(id);
    loop {
        let mut idx = r.u8()?;
        if idx == 0xFF {
            break;
        }
        if pre_1_21_9(k) && player {
            idx = player_index_1_21_8(idx);
        }
        let src = r.var_int()?;
        let ser = usize::try_from(src).ok().and_then(|i| table.get(i)).ok_or(
            ezzmc_protocol::ProtocolError::InvalidValue {
                what: "entity data serializer",
                value: src.into(),
            },
        )?;
        if pre_1_21_5(k) {
            let i = if pre_1_21_2(k) {
                index_1_21_1(ty, idx)
            } else {
                Some(idx)
            };
            let Some(i) = i.and_then(|i| index_1_21_4(ty, i)) else {
                entity_data_value(r, &mut Writer::new(), ser)?;
                continue;
            };
            idx = i;
            // Вагонетка: INT id состояния (0 — воздух = нет своего блока) → OPTIONAL_BLOCK_STATE;
            // VANILLA-UNSURE: флаг CUSTOM_DISPLAY (#13) не учитывается.
            // Грибная корова: STRING "red"/"brown" → INT (MushroomCow.Variant 1.21.5).
            let conv = match (ty, idx, *ser) {
                (t, 11, "INT") if t.ends_with("minecart") => {
                    Some(("OPTIONAL_BLOCK_STATE", r.var_int()?))
                }
                ("mooshroom", 17, "STRING") => {
                    let v = r.string(32767)?;
                    Some(("INT", i32::from(v == "brown")))
                }
                // VANILLA: Salmon#DATA_TYPE (1.21.4) — INT по Salmon.Variant (small 0,
                // medium 1, large 2); в 1.21.3 — строка.
                ("salmon", _, "STRING") => {
                    let v = r.string(32767)?;
                    let id = match v.as_str() {
                        "small" => 0,
                        "large" => 2,
                        _ => 1,
                    };
                    Some(("INT", id))
                }
                _ => None,
            };
            if let Some((ser, v)) = conv {
                let mut raw = Writer::new();
                raw.var_int(v);
                let raw = raw.into_vec();
                let mut one = Writer::new();
                one.u8(layout.index(idx)).var_int(canon_of(ser) as i32);
                if entity_data_value(&mut Reader::new(&raw), &mut one, ser)? {
                    w.bytes(&one.into_vec());
                }
                continue;
            }
        }
        if hanging && idx >= 8 {
            idx += 1;
        }
        let mut one = Writer::new();
        if *ser == "COMPOUND_TAG" {
            // 1.21.8: только плечи игрока (19/20) → OPTIONAL_UNSIGNED_INT.
            let tag = ezzmc_protocol::nbt::read_network(r)?;
            if player && matches!(idx, 19 | 20) {
                one.u8(layout.index(idx))
                    .var_int(canon_of("OPTIONAL_UNSIGNED_INT") as i32);
                w.bytes(&one.var_int(shoulder_parrot(tag.as_ref())).buf);
            }
            continue;
        }
        match (k != 2).then(|| retyped_1_21_10(ty, idx)).flatten() {
            Some("HUMANOID_ARM") if *ser == "BYTE" => {
                // HumanoidArm: 0 = LEFT, 1 = RIGHT.
                let arm = i32::from(r.u8()? != 0);
                one.u8(layout.index(idx))
                    .var_int(canon_of("HUMANOID_ARM") as i32);
                w.bytes(&one.var_int(arm).buf);
                continue;
            }
            Some("ANGER_END_TIME") if *ser == "INT" => {
                // VANILLA-UNSURE: остаток тиков → момент окончания без игрового времени
                // сервера; «зол» = i64::MAX до прихода 0 (−1, как define в 1.21.11).
                let end = if r.var_int()? > 0 { i64::MAX } else { -1 };
                one.u8(layout.index(idx)).var_int(canon_of("LONG") as i32);
                w.bytes(&one.var_long(end).buf);
                continue;
            }
            _ => {}
        }
        one.u8(layout.index(idx)).var_int(canon_of(ser) as i32);
        if entity_data_value(r, &mut one, ser)? {
            w.bytes(&one.into_vec());
        }
    }
    w.u8(0xFF);
    Ok(())
}

/// Пакет клиента для отправки: (id, тело).
pub type Packet = (i32, Vec<u8>);

/// id игрока из `login` и последнее отправленное состояние шифта (1.21.5: PRESS/RELEASE_SHIFT_KEY).
static PLAYER_ID: AtomicI32 = AtomicI32::new(0);
static SHIFT_DOWN: AtomicBool = AtomicBool::new(false);

/// Пакет клиента (собран в каноне, id из таблицы версии) → пакеты для сервера 1.21.5–1.21.11
/// (id, тело) в порядке отправки. `None` — пакет без изменений.
pub fn serverbound_1_21_11(id: i32, body: &[u8]) -> ProtocolResult<Option<Vec<Packet>>> {
    // VANILLA: книга рецептов 1.21.1 — id рецептов строками (ResourceLocation), в каноне —
    // RecipeDisplayId; перевести нечем, пакеты не отправляются.
    if pre_1_21_2(ver())
        && matches!(
            packet_name(id),
            Some("place_recipe" | "recipe_book_seen_recipe")
        )
    {
        return Ok(Some(Vec::new()));
    }
    let b = serverbound_body(id, body)?;
    let mut out = Vec::new();
    // VANILLA: LocalPlayer#sendShiftKeyState (1.21.5) — до ServerboundPlayerInputPacket, при
    // смене Input.shift: ServerboundPlayerCommandPacket PRESS_SHIFT_KEY(0)/RELEASE_SHIFT_KEY(1).
    if pre_1_21_6(ver()) && packet_name(id) == Some("player_input") {
        let shift = body.first().is_some_and(|f| f & 32 != 0);
        if SHIFT_DOWN.swap(shift, Relaxed) != shift {
            if let Some(cmd) = packet_id("player_command") {
                let mut w = Writer::new();
                w.var_int(PLAYER_ID.load(Relaxed))
                    .var_int(i32::from(!shift))
                    .var_int(0);
                out.push((cmd, w.into_vec()));
            }
        }
    }
    if b.is_none() && out.is_empty() {
        return Ok(None);
    }
    out.push((id, b.unwrap_or_else(|| body.to_vec())));
    Ok(Some(out))
}

fn packet_name(id: i32) -> Option<&'static str> {
    VERSIONS[ver()]
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.id as i32 == id)
        .map(|p| p.name)
}

fn packet_id(name: &str) -> Option<i32> {
    VERSIONS[ver()]
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.name == name)
        .map(|p| p.id as i32)
}

/// Тело одного пакета клиента в формате версии соединения. `None` — без изменений.
fn serverbound_body(id: i32, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let Some(name) = packet_name(id) else {
        return Ok(None);
    };
    let mut r = Reader::new(body);
    let mut w = Writer::new();
    match name {
        // VANILLA: ServerboundContainerClickPacket — containerId, stateId, slot, button,
        // ClickType (1.21.11) = ContainerInput (26.3), changedSlots, carried.
        "container_click" => {
            container_id_back(&mut r, &mut w)?;
            w.var_int(r.var_int()?);
            w.bytes(r.bytes(3)?).var_int(r.var_int()?);
            let n = r.length("changed slots", 128)?;
            w.var_int(n as i32);
            let stack = if pre_1_20_5(ver()) {
                crate::v1201::hashed_to_1_20_4
            } else if pre_1_21_5(ver()) {
                hashed_to_stack_1_21_4
            } else {
                hashed_stack_back
            };
            for _ in 0..n {
                w.i16(r.i16()?);
                stack(&mut r, &mut w)?;
            }
            stack(&mut r, &mut w)?;
        }
        // VANILLA: LastSeenMessages.Update (1.21.5) дописал байт checksum в конец chat /
        // chat_command_signed; в 1.21.4 его нет.
        "chat" | "chat_command_signed" if pre_1_21_5(ver()) => {
            let Some((_, head)) = body.split_last() else {
                return Ok(None);
            };
            w.bytes(head);
        }
        // VANILLA: ServerboundSetCreativeModeSlotPacket — short slot, OPTIONAL_UNTRUSTED stack.
        "set_creative_mode_slot" => {
            w.i16(r.i16()?);
            if pre_1_20_5(ver()) {
                crate::v1201::untrusted_to_1_20_4(&mut r, &mut w)?;
            } else {
                tr_back().stack_untrusted(&mut r, &mut w)?;
            }
        }
        // VANILLA: ServerboundPlayerCommandPacket.Action (1.21.5): перед STOP_SLEEPING ещё
        // PRESS_SHIFT_KEY, RELEASE_SHIFT_KEY.
        // VANILLA: ServerboundMoveVehiclePacket (1.21.4) дописал onGround; в 1.21.3 его нет.
        "move_vehicle" if matches!(ver(), 5..=8) => {
            let Some((_, head)) = body.split_last() else {
                return Ok(None);
            };
            w.bytes(head);
        }
        "player_command" if pre_1_21_6(ver()) => {
            let (eid, action, data) = (r.var_int()?, r.var_int()?, r.var_int()?);
            w.var_int(eid).var_int(action + 2).var_int(data);
        }
        // VANILLA: ServerboundContainerClosePacket (1.21.1) — id окна байтом.
        "container_close" if pre_1_21_2(ver()) => container_id_back(&mut r, &mut w)?,
        // VANILLA: ServerboundMovePlayerPacket (1.21.1) — bool onGround; в 1.21.2 байт флагов
        // (бит 0 onGround, бит 1 horizontalCollision).
        "move_player_pos"
        | "move_player_pos_rot"
        | "move_player_rot"
        | "move_player_status_only"
            if pre_1_21_2(ver()) =>
        {
            let Some((flags, head)) = body.split_last() else {
                return Ok(None);
            };
            w.bytes(head).u8(flags & 1);
        }
        // VANILLA: ServerboundPlayerInputPacket (1.21.1): float xxa, float zza, байт флагов
        // (1 — прыжок, 2 — шифт); канон — Input (вперёд 1, назад 2, влево 4, вправо 8,
        // прыжок 16, шифт 32, спринт 64).
        // VANILLA-UNSURE: LocalPlayer 1.21.1 шлёт пакет только верхом и с xxa/zza после
        // замедления (шифт 0.3); здесь — импульсы ±1 при каждом изменении ввода.
        "player_input" if pre_1_21_2(ver()) => {
            let i = r.u8()?;
            let axis = |pos: u8, neg: u8| {
                f32::from(u8::from(i & pos != 0)) - f32::from(u8::from(i & neg != 0))
            };
            w.f32(axis(4, 8)).f32(axis(1, 2));
            w.u8(u8::from(i & 16 != 0) | (u8::from(i & 32 != 0) << 1));
        }
        // VANILLA: FriendlyByteBuf#writeBlockHitResult (1.21.1) — без worldBorderHit (1.21.2).
        "use_item_on" if pre_1_21_2(ver()) => {
            w.var_int(r.var_int()?)
                .bytes(r.bytes(8)?)
                .var_int(r.var_int()?);
            w.bytes(r.bytes(12)?).bool(r.bool()?);
            r.bool()?;
            w.var_int(r.var_int()?);
        }
        // VANILLA: ServerboundUseItemPacket (1.20.6) — рука и sequence, без yRot/xRot.
        "use_item" if pre_1_21(ver()) => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
        }
        // VANILLA: ClientInformation (1.21.1) — без particleStatus (появился в 1.21.2).
        "client_information" if pre_1_21_2(ver()) => {
            let Some((_, head)) = body.split_last() else {
                return Ok(None);
            };
            w.bytes(head);
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

/// VarInt ContainerId канона → байт окна 1.21.1 (`writeByte`), для остальных версий — как есть.
fn container_id_back(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let id = r.var_int()?;
    if pre_1_21_2(ver()) {
        w.u8(id as u8);
    } else {
        w.var_int(id);
    }
    Ok(())
}

/// Байт окна 1.21.1 (`readUnsignedByte`) → VarInt ContainerId канона.
fn container_id(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if pre_1_21_2(ver()) {
        w.var_int(i32::from(r.u8()?));
    } else {
        w.var_int(r.var_int()?);
    }
    Ok(())
}

/// Имя пакета сервера 1.21.1 в каноне (тело переводит [`clientbound_1_21_11`]). VANILLA:
/// ClientPacketListener#handleContainerSetSlot (1.21.1) — окно -1 курсор, -2 инвентарь игрока.
pub fn canon_name_1_21_1<'a>(name: &'a str, body: &[u8]) -> &'a str {
    match (name, body.first()) {
        ("set_carried_item", _) => "set_held_slot",
        ("container_set_slot", Some(0xFF)) => "set_cursor_item",
        ("container_set_slot", Some(0xFE)) => "set_player_inventory",
        (n, _) => n,
    }
}

/// CommonPlayerSpawnInfo 1.21.1 после ключа измерения (seed, режимы, флаги, место смерти,
/// portalCooldown) → + seaLevel (1.21.2). Остаток пакета копируется.
/// VANILLA-UNSURE: уровень моря берётся по измерению (NoiseGeneratorSettings: 63 / 32 / 0).
/// CommonPlayerSpawnInfo 1.20.4 → 1.21.1-формат (тип измерения id реестра) → канон.
fn spawn_info_1_20_4(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let ty = r.string(32767)?;
    w.var_int(crate::v1201::dim_index(&ty));
    *dimension() = r.string(32767)?;
    w.string(&dimension());
    spawn_info_1_21_1(r, w)
}

fn spawn_info_1_21_1(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let start = r.pos();
    r.bytes(8 + 2 + 2)?;
    if r.bool()? {
        r.string(32767)?;
        r.bytes(8)?;
    }
    r.var_int()?;
    w.bytes(r.since(start));
    let sea = match dimension().as_str() {
        "minecraft:the_nether" => 32,
        "minecraft:the_end" => 0,
        _ => 63,
    };
    w.var_int(sea).bytes(r.rest());
    Ok(())
}

/// HashedStack канона → ItemStack.OPTIONAL_STREAM_CODEC 1.21.4 (count, item, пустой патч).
/// VANILLA-UNSURE: компоненты (в каноне только хеши) не передаются; сервер сверит слоты и
/// пришлёт исправление, как при рассинхроне.
fn hashed_to_stack_1_21_4(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.var_int(0);
        return Ok(());
    }
    let item = tr_back().id(ClientRegistry::Item, r.var_int()?);
    let count = r.var_int()?;
    for _ in 0..r.length("hashed added", 256)? {
        r.var_int()?;
        r.i32()?;
    }
    for _ in 0..r.length("hashed removed", 256)? {
        r.var_int()?;
    }
    match item {
        Some(item) if count > 0 => w.var_int(count).var_int(item).var_int(0).var_int(0),
        _ => w.var_int(0),
    };
    Ok(())
}

/// Пакет сервера 1.21.4, которого нет в 1.21.5+, → пакеты 1.21.4 (id, тело), которые затем
/// переводятся обычным путём. `None` — пакет не разворачивается.
pub fn expand(name: Option<&str>, body: &[u8]) -> ProtocolResult<Option<Vec<Packet>>> {
    if !pre_1_21_5(ver()) || name != Some("add_experience_orb") {
        return Ok(None);
    }
    // VANILLA: ClientboundAddExperienceOrbPacket (1.21.4): id, x, y, z, short value;
    // ClientPacketListener#handleAddExperienceOrb — сфера с повёрнутостью 0. В 1.21.5 —
    // add_entity(experience_orb) + ExperienceOrb#DATA_VALUE (#8, INT).
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    let pos = r.bytes(24)?;
    let value = i32::from(r.i16()?);
    let (Some(add), Some(data), Some(ty)) = (
        clientbound_id("add_entity"),
        clientbound_id("set_entity_data"),
        VERSIONS[ver()]
            .entity_type
            .iter()
            .position(|n| *n == "experience_orb"),
    ) else {
        return Ok(Some(Vec::new()));
    };
    let mut a = Writer::new();
    a.var_int(id)
        .i64(0)
        .i64(i64::from(id))
        .var_int(ty as i32)
        .bytes(pos)
        .bytes(&[0, 0, 0])
        .var_int(0)
        .i16(0)
        .i16(0)
        .i16(0);
    let mut d = Writer::new();
    d.var_int(id).u8(8).var_int(1).var_int(value).u8(0xFF);
    Ok(Some(vec![(add, a.into_vec()), (data, d.into_vec())]))
}

fn clientbound_id(name: &str) -> Option<i32> {
    VERSIONS[ver()]
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Clientbound && p.name == name)
        .map(|p| p.id as i32)
}

/// VANILLA: ItemCost.STREAM_CODEC — предмет, количество, DataComponentExactPredicate.
static ITEM_COST: Shape = Shape::Struct(&[
    Shape::RegistryId(ClientRegistry::Item),
    Shape::VarInt,
    Shape::List(usize::MAX, &Shape::TypedComponent),
]);

/// Тело пакета (имя из таблицы версии соединения) → канон, если версия не каноническая.
/// `None` — пакет надо отбросить (в каноне нет аналога).
pub fn to_canon(name: Option<&str>, body: Vec<u8>) -> ProtocolResult<Option<Vec<u8>>> {
    let v = ezzmc_protocol::version::current();
    if v.is_canon() {
        return Ok(Some(body));
    }
    if v.protocol == 763 {
        return match name {
            Some(n) => clientbound_1_20_1(n, &body),
            None => Ok(None),
        };
    }
    let Some(n) = name else { return Ok(Some(body)) };
    match clientbound_1_21_11(n, &body)? {
        Some(b) if b.is_empty() && !body.is_empty() => Ok(None),
        Some(b) => Ok(Some(b)),
        None => Ok(Some(body)),
    }
}

/// Сервер 1.20.1: БЕЛЫЙ список переведённых пакетов; всё остальное отбрасываем
/// (форматы 1.20.1 и канона расходятся, непереведённый пакет сломал бы разбор).
pub fn clientbound_1_20_1(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    crate::v1201::play_to_canon(name, body)
}

/// Клиентский пакет сервера 1.21.11 → тело в каноне. Неизвестные расхождения не трогаем.
pub fn clientbound_1_21_11(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::new();
    match name {
        // VANILLA: ClientboundMoveEntityPacket.Pos (1.21.11): id, short×3, onGround
        // → 26.3: id, varint(props = steps<<1 | onGround), VecDelta
        "move_entity_pos" => {
            let id = r.var_int()?;
            let d = [r.i16()?, r.i16()?, r.i16()?];
            let g = r.bool()?;
            w.var_int(id).var_int(i32::from(g));
            d.iter().for_each(|v| {
                w.i16(*v);
            });
        }
        "move_entity_pos_rot" => {
            let id = r.var_int()?;
            let d = [r.i16()?, r.i16()?, r.i16()?];
            let (y, x, g) = (r.i8()?, r.i8()?, r.bool()?);
            w.var_int(id).var_int(i32::from(g));
            d.iter().for_each(|v| {
                w.i16(*v);
            });
            w.u8(y as u8).u8(x as u8);
        }
        // 26.3: id, onGround, yRot, xRot
        "move_entity_rot" => {
            let id = r.var_int()?;
            let (y, x, g) = (r.i8()?, r.i8()?, r.bool()?);
            w.var_int(id).u8(u8::from(g)).u8(y as u8).u8(x as u8);
        }
        // 1.21.11: id, PositionMoveRotation(pos, delta, yRot, xRot), onGround
        // → 26.3: id, PositionPath::LINEAR(0, pos), yRot, xRot, onGround
        "entity_position_sync" => {
            let id = r.var_int()?;
            let pos = [r.f64()?, r.f64()?, r.f64()?];
            let _delta = [r.f64()?, r.f64()?, r.f64()?];
            let (y, x, g) = (r.f32()?, r.f32()?, r.bool()?);
            w.var_int(id).var_int(0);
            pos.iter().for_each(|v| {
                w.f64(*v);
            });
            w.f32(y).f32(x).u8(u8::from(g));
        }
        // Тип сущности — id реестра entity_type (у версий разный порядок) → канон.
        "add_entity" => {
            let id = r.var_int()?;
            let uuid = r.bytes(16)?;
            let ty = r.var_int()?;
            let canon = entity_type_map()
                .get(usize::try_from(ty).unwrap_or(usize::MAX))
                .copied()
                .flatten();
            let Some(canon) = canon else {
                // Нет такого типа в каноне: пакет пропускаем целиком (сущность не появится).
                return Ok(Some(Vec::new()));
            };
            let name = CANON.entity_type.get(canon as usize).copied().unwrap_or("");
            layouts().insert(id, (DataLayout::of(name), name));
            w.var_int(id).bytes(uuid).var_int(canon as i32);
            if pre_1_21_9(ver()) {
                // VANILLA: ClientboundAddEntityPacket (1.21.8): x, y, z, xRot, yRot, yHeadRot,
                // data, short×3 скорости → (1.21.9) x, y, z, LpVec3, xRot, yRot, yHeadRot, data.
                let pos = r.bytes(24)?;
                let rot = r.bytes(3)?;
                let data = r.var_int()?;
                let v = [r.i16()?, r.i16()?, r.i16()?].map(|v| f64::from(v) / 8000.0);
                w.bytes(pos);
                crate::play::write_lp_vec3(&mut w, v);
                w.bytes(rot);
                let data = if name == "falling_block" {
                    state_id(data)
                } else {
                    data
                };
                w.var_int(data);
                return Ok(Some(w.into_vec()));
            }
            if name != "falling_block" {
                w.bytes(r.rest());
                return Ok(Some(w.into_vec()));
            }
            // x, y, z; LpVec3 скорости (0 — один байт; иначе 2+4 байта и VarInt при флаге 4);
            // xRot, yRot, yHeadRot; data — id состояния (FallingBlockEntity).
            w.bytes(r.bytes(24)?);
            let m = r.u8()?;
            w.u8(m);
            if m != 0 {
                w.bytes(r.bytes(5)?);
                if m & 4 != 0 {
                    w.var_int(r.var_int()?);
                }
            }
            w.bytes(r.bytes(3)?);
            w.var_int(state_id(r.var_int()?));
        }
        // VANILLA: ClientboundBlockUpdatePacket — BlockPos, id состояния.
        "block_update" => {
            w.bytes(r.bytes(8)?).var_int(state_id(r.var_int()?));
        }
        // VANILLA: ClientboundSectionBlocksUpdatePacket — SectionPos, VarLong (state << 12 | pos).
        "section_blocks_update" => {
            w.bytes(r.bytes(8)?);
            let n = r.length("section updates", 4096)?;
            w.var_int(n as i32);
            for _ in 0..n {
                let e = r.var_long()?;
                let st = ezzmc_world::legacy::state_to_canon((e >> 12) as u32);
                w.var_long((i64::from(st) << 12) | (e & 4095));
            }
        }
        // VANILLA: ClientboundBlockEventPacket — BlockPos, b0, b1, registry block.
        "block_event" => {
            w.bytes(r.bytes(10)?);
            let Some(b) = tr().id(ClientRegistry::Block, r.var_int()?) else {
                return Ok(Some(Vec::new()));
            };
            w.var_int(b);
        }
        // VANILLA: ClientboundLevelEventPacket — int type, BlockPos, int data, bool global.
        // LevelEventHandler: 2001 (разрушение) и 3008 (кисть) — data = id состояния.
        "level_event" => {
            let t = r.i32()?;
            w.i32(t).bytes(r.bytes(8)?);
            let data = r.i32()?;
            w.i32(if matches!(t, 2001 | 3008) {
                state_id(data)
            } else {
                data
            });
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundLevelChunkPacketData (1.21.11 = 26.3): heightmaps, byteArray секций,
        // list<BlockEntityInfo{byte xz, short y, registry block_entity_type, OPTIONAL_COMPOUND_TAG}>,
        // затем свет. Переводится только тип блок-сущности; без пары в каноне (bed) — запись выбрасывается.
        "level_chunk_with_light" => {
            if pre_1_21_5(ver()) {
                // VANILLA: ClientboundLevelChunkPacketData (1.21.4) — карты высот NBT-compound
                // (ключ Heightmap.Types#getSerializationKey → LongArray); 1.21.5 — Map(id, long[]).
                w.bytes(r.bytes(8)?);
                let mut maps = Vec::new();
                if let Some(ezzmc_protocol::nbt::Tag::Compound(items)) =
                    ezzmc_protocol::nbt::read_network(&mut r)?
                {
                    for (k, v) in items {
                        if let (Some(id), ezzmc_protocol::nbt::Tag::LongArray(l)) =
                            (crate::v1201::heightmap_id(&k), v)
                        {
                            maps.push((id, l));
                        }
                    }
                }
                w.var_int(maps.len() as i32);
                for (id, l) in maps {
                    w.var_int(id).var_int(l.len() as i32);
                    for v in l {
                        w.i64(v);
                    }
                }
            }
            let start = r.pos();
            if !pre_1_21_5(ver()) {
                r.bytes(8)?;
                for _ in 0..r.length("heightmaps", 16)? {
                    r.var_int()?;
                    let n = r.length("heightmap longs", 1 << 12)?;
                    r.bytes(n * 8)?;
                }
            }
            let n = r.length("chunk buffer", 2 * 1024 * 1024)?;
            r.bytes(n)?;
            w.bytes(r.since(start));
            let n = r.length("block entities", 1 << 16)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                let head = r.bytes(3)?;
                let ty = block_entity_type(r.var_int()?);
                let tag = r.pos();
                ezzmc_protocol::nbt::read_network(&mut r)?;
                if let Some(ty) = ty {
                    list.bytes(head).var_int(ty).bytes(r.since(tag));
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec());
            crate::v1201::light_to_canon(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundLightUpdatePacket (1.21.11 и ниже): маски света long[] (writeBitSet).
        "light_update" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            crate::v1201::light_to_canon(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundBlockEntityDataPacket: BlockPos, registry block_entity_type, tag.
        "block_entity_data" => {
            w.bytes(r.bytes(8)?);
            let Some(ty) = block_entity_type(r.var_int()?) else {
                return Ok(Some(Vec::new()));
            };
            w.var_int(ty).bytes(r.rest());
        }
        // VANILLA: ClientboundRecipeBookAddPacket — List<Entry(RecipeDisplayEntry, u8 flags)>, replace.
        "recipe_book_add" => {
            let n = r.length("recipe entries", 1 << 16)?;
            w.var_int(n as i32);
            for _ in 0..n {
                recipe_entry(&mut r, &mut w)?;
                w.u8(r.u8()?);
            }
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundPlaceGhostRecipePacket — containerId, RecipeDisplay.
        "place_ghost_recipe" if !pre_1_21_2(ver()) => {
            w.var_int(r.var_int()?);
            recipe_display(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundUpdateAdvancementsPacket. 1.21.11: x/y в DisplayInfo, иконка —
        // ItemStack; 26.3: PositionedAdvancement(holder, x, y), иконка — ItemStackTemplate.
        // Без DisplayInfo x = y = 0 (VANILLA-UNSURE: в 26.3 сервер шлёт позицию узла дерева).
        "update_advancements" => {
            w.bool(r.bool()?);
            let n = r.length("advancements", 1 << 16)?;
            w.var_int(n as i32);
            for _ in 0..n {
                str_id(&mut r, &mut w)?;
                let parent = r.bool()?;
                w.bool(parent);
                if parent {
                    str_id(&mut r, &mut w)?;
                }
                let display = r.bool()?;
                w.bool(display);
                let mut xy = (0.0, 0.0);
                if display {
                    tr().value(&mut r, &mut w, &Shape::Nbt, &Shape::Nbt)?;
                    tr().value(&mut r, &mut w, &Shape::Nbt, &Shape::Nbt)?;
                    let mut icon = Writer::new();
                    let ok = if pre_1_20_5(ver()) {
                        // VANILLA: DisplayInfo#serializeToNetwork (1.20.4) — writeItem (NBT);
                        // канонический стек (count, item, патч) → ItemTemplate (item, count, патч).
                        let mut st = Writer::new();
                        stack(&mut r, &mut st)?;
                        let st = st.into_vec();
                        let mut sr = Reader::new(&st);
                        let count = sr.var_int()?;
                        if count > 0 {
                            icon.var_int(sr.var_int()?).var_int(count).bytes(sr.rest());
                        }
                        count > 0
                    } else {
                        tr().value(&mut r, &mut icon, &Shape::Stack, &Shape::ItemTemplate)?
                    };
                    if !ok {
                        icon = Writer::new();
                        icon.var_int(0).var_int(1).var_int(0).var_int(0);
                    }
                    w.bytes(&icon.into_vec());
                    w.var_int(r.var_int()?);
                    let flags = r.i32()?;
                    w.i32(flags);
                    if flags & 1 != 0 {
                        str_id(&mut r, &mut w)?;
                    }
                    xy = (r.f32()?, r.f32()?);
                }
                // AdvancementRequirements: List<List<String>>, затем sendsTelemetryEvent.
                let groups = r.length("requirements", 1 << 16)?;
                w.var_int(groups as i32);
                for _ in 0..groups {
                    let k = r.length("requirement", 1 << 16)?;
                    w.var_int(k as i32);
                    for _ in 0..k {
                        str_id(&mut r, &mut w)?;
                    }
                }
                w.bool(r.bool()?).f32(xy.0).f32(xy.1);
            }
            // removed, progress, showAdvancements — форматы совпадают.
            w.bytes(r.rest());
            // VANILLA: showAdvancements (1.21.5) — в 1.21.4 нет, по умолчанию true.
            if pre_1_21_5(ver()) {
                w.bool(true);
            }
        }
        // VANILLA: ClientboundPlayerChatPacket (1.21.5) — первым globalIndex (VarInt), в 1.21.4 нет.
        "player_chat" if pre_1_21_5(ver()) => {
            w.var_int(0).bytes(r.rest());
        }
        // VANILLA: ClientboundSetPlayerTeamPacket.Parameters (1.21.4): nametagVisibility и
        // collisionRule — строки (readUtf(40)); 1.21.5 — Team.Visibility/CollisionRule по id.
        "set_player_team" if pre_1_21_5(ver()) => {
            w.string(&r.string(32767)?);
            let method = r.u8()?;
            w.u8(method);
            if matches!(method, 0 | 2) {
                let text = r.pos();
                ezzmc_protocol::nbt::read_network(&mut r)?;
                w.bytes(r.since(text)).u8(r.u8()?);
                let vis = r.string(40)?;
                let col = r.string(40)?;
                let vis = ["always", "never", "hideForOtherTeams", "hideForOwnTeam"]
                    .iter()
                    .position(|n| *n == vis);
                let col = ["always", "never", "pushOtherTeams", "pushOwnTeam"]
                    .iter()
                    .position(|n| *n == col);
                w.var_int(vis.unwrap_or(0) as i32)
                    .var_int(col.unwrap_or(0) as i32);
            }
            w.bytes(r.rest());
        }
        "login" => {
            layouts().clear();
            SHIFT_DOWN.store(false, Relaxed);
            PLAYER_ID.store(r.i32()?, Relaxed);
            // int id (выше), bool hardcore, список измерений, 3×VarInt, 3×bool, CommonPlayerSpawnInfo.
            r.bool()?;
            for _ in 0..r.length("levels", 1 << 10)? {
                r.string(32767)?;
            }
            for _ in 0..3 {
                r.var_int()?;
            }
            r.bytes(3)?;
            if pre_1_20_5(ver()) {
                // VANILLA: CommonPlayerSpawnInfo (1.20.4) — тип измерения ResourceKey (строка),
                // в 1.20.5 стал Holder (id реестра); enforcesSecureChat в login появился в 1.20.5.
                w.bytes(r.since(0));
                spawn_info_1_20_4(&mut r, &mut w)?;
                w.bool(false);
                return Ok(Some(w.into_vec()));
            }
            remember_dimension(&mut r)?;
            if !pre_1_21_2(ver()) {
                return Ok(None);
            }
            w.bytes(r.since(0));
            spawn_info_1_21_1(&mut r, &mut w)?;
        }
        "respawn" if pre_1_20_5(ver()) => {
            spawn_info_1_20_4(&mut r, &mut w)?;
            w.bytes(r.rest());
        }
        "respawn" => {
            remember_dimension(&mut r)?;
            if !pre_1_21_2(ver()) {
                return Ok(None);
            }
            w.bytes(r.since(0));
            spawn_info_1_21_1(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundSetDefaultSpawnPositionPacket (1.21.8): BlockPos, float angle →
        // (1.21.9) RespawnData: GlobalPos (измерение, BlockPos), yaw, pitch.
        "set_default_spawn_position" if pre_1_21_9(ver()) => {
            let pos = r.bytes(8)?;
            let yaw = r.f32()?;
            let dim = dimension().clone();
            w.string(&dim).bytes(pos).f32(yaw).f32(0.0);
        }
        // VANILLA: ClientboundPlayerRotationPacket (1.21.8): yRot, xRot → (1.21.9) с флагами
        // relativeY/relativeX (абсолютный поворот).
        "player_rotation" if pre_1_21_9(ver()) => {
            let (y, x) = (r.f32()?, r.f32()?);
            w.f32(y).bool(false).f32(x).bool(false);
        }
        // VANILLA: ClientboundSetEntityMotionPacket (1.21.8): short×3 (скорость × 8000) →
        // (1.21.9) LpVec3.
        "set_entity_motion" if pre_1_21_9(ver()) => {
            w.var_int(r.var_int()?);
            let v = [r.i16()?, r.i16()?, r.i16()?].map(|v| f64::from(v) / 8000.0);
            crate::play::write_lp_vec3(&mut w, v);
        }
        "remove_entities" => {
            let mut l = layouts();
            for _ in 0..r.length("entities", 1 << 16)? {
                l.remove(&r.var_int()?);
            }
            return Ok(None);
        }
        // VANILLA: ClientboundSetEntityDataPacket — id, записи (u8 индекс, VarInt сериализатор,
        // значение), 0xFF. Сериализаторы и индексы полей переводятся в 26.3.
        "set_entity_data" => set_entity_data(&mut r, &mut w, ver())?,
        // Стеки: ItemStack.OPTIONAL_STREAM_CODEC (id предметов и компонентов, формы компонентов).
        "container_set_content" => {
            container_id(&mut r, &mut w)?;
            w.var_int(r.var_int()?);
            let n = r.length("container items", 1 << 16)?;
            w.var_int(n as i32);
            for _ in 0..=n {
                stack(&mut r, &mut w)?;
            }
        }
        "container_set_slot" if pre_1_21_2(ver()) => {
            let win = r.i8()?;
            let state = r.var_int()?;
            let slot = r.i16()?;
            match win {
                -1 => {}
                -2 => {
                    w.var_int(i32::from(slot));
                }
                _ => {
                    w.var_int(i32::from(win as u8)).var_int(state).i16(slot);
                }
            }
            stack(&mut r, &mut w)?;
        }
        "container_set_slot" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?).i16(r.i16()?);
            stack(&mut r, &mut w)?;
        }
        // VANILLA: id окна 1.21.1 байтом (ContainerSetData, ContainerClose, HorseScreenOpen).
        // VANILLA: ClientboundHorseScreenOpenPacket (1.20.6) — размер инвентаря вместо колонок:
        // лошадь 2, осёл/мул с сундуком 16, лама 1 + 3 × колонки (AbstractHorse#getInventorySize).
        "horse_screen_open" if pre_1_21(ver()) => {
            container_id(&mut r, &mut w)?;
            let size = r.var_int()?;
            w.var_int((size.max(1) - 1) / 3).bytes(r.rest());
        }
        "container_set_data" | "container_close" | "horse_screen_open" if pre_1_21_2(ver()) => {
            container_id(&mut r, &mut w)?;
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundSetCarriedItemPacket (1.21.1) — байт слота; set_held_slot — VarInt.
        "set_carried_item" => {
            w.var_int(i32::from(r.u8()?));
        }
        // VANILLA: книга рецептов и рецепты 1.21.1 (ClientboundRecipePacket, полные Recipe в
        // update_recipes, id рецепта в place_ghost_recipe) несовместимы с RecipeDisplay 1.21.2.
        "recipe" | "update_recipes" | "place_ghost_recipe" if pre_1_21_2(ver()) => {
            return Ok(Some(Vec::new()));
        }
        // VANILLA: ClientboundCooldownPacket (1.21.1) — id предмета; в 1.21.2 группа (Identifier,
        // по умолчанию id предмета).
        "cooldown" if pre_1_21_2(ver()) => {
            let item = r.var_int()?;
            let Some(name) = usize::try_from(item)
                .ok()
                .and_then(|i| VERSIONS[ver()].item.get(i))
            else {
                return Ok(Some(Vec::new()));
            };
            w.string(&format!("minecraft:{name}")).bytes(r.rest());
        }
        // VANILLA: ClientboundTeleportEntityPacket (1.21.1): id, xyz, byte yRot/xRot, onGround →
        // (1.21.2) id, PositionMoveRotation (xyz, delta 0, float повороты), Relative (пусто), onGround.
        "teleport_entity" if pre_1_21_2(ver()) => {
            w.var_int(r.var_int()?).bytes(r.bytes(24)?);
            w.f64(0.0).f64(0.0).f64(0.0);
            let (y, x) = (r.i8()?, r.i8()?);
            w.f32(f32::from(y) * 360.0 / 256.0)
                .f32(f32::from(x) * 360.0 / 256.0)
                .i32(0)
                .bool(r.bool()?);
        }
        // VANILLA: ClientboundPlayerPositionPacket (1.21.1): xyz, yRot, xRot, байт RelativeMovement,
        // id → (1.21.2) id, PositionMoveRotation, Relative. handleMovePlayer 1.21.1 сохраняет
        // скорость по относительной оси — в 1.21.2 это DELTA_X/Y/Z (биты 5–7).
        "player_position" if pre_1_21_2(ver()) => {
            let pos = r.bytes(24)?;
            let rot = r.bytes(8)?;
            let fl = i32::from(r.u8()? & 0x1F);
            w.var_int(r.var_int()?).bytes(pos);
            w.f64(0.0).f64(0.0).f64(0.0);
            w.bytes(rot).i32(fl | ((fl & 7) << 5));
        }
        "set_cursor_item" => stack(&mut r, &mut w)?,
        "set_player_inventory" => {
            w.var_int(r.var_int()?);
            stack(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundSetEquipmentPacket — слот с битом 0x80 «дальше ещё».
        "set_equipment" => {
            w.var_int(r.var_int()?);
            loop {
                let slot = r.u8()?;
                w.u8(slot);
                stack(&mut r, &mut w)?;
                if slot & 0x80 == 0 {
                    break;
                }
            }
        }
        // VANILLA: MerchantOffer#writeToStream (1.21.11 = 26.3): costA, result, Optional costB, …
        "merchant_offers" => {
            w.var_int(r.var_int()?);
            let n = r.length("merchant offers", 1 << 10)?;
            w.var_int(n as i32);
            for _ in 0..n {
                if !tr().value(&mut r, &mut w, &ITEM_COST, &ITEM_COST)? {
                    return Ok(Some(Vec::new()));
                }
                stack(&mut r, &mut w)?;
                let b = r.bool()?;
                w.bool(b);
                if b && !tr().value(&mut r, &mut w, &ITEM_COST, &ITEM_COST)? {
                    return Ok(Some(Vec::new()));
                }
                // outOfStock, uses, maxUses, xp, specialPriceDiff, priceMultiplier, demand
                w.bytes(r.bytes(1 + 4 * 6)?);
            }
            w.bytes(r.rest());
        }
        // 1.21.11: overrideLimiter, alwaysShow, xyz, dist xyz, maxSpeed, int count, particle
        // → 26.3: particle, overrideLimiter, alwaysShow, xyz, dist xyz, maxSpeed xyz, varint count,
        // randomizationType DEFAULT (поведение 1.21.11: гаусс по всем осям с одной скоростью).
        "level_particles" => {
            // VANILLA: ClientboundLevelParticlesPacket (1.21.4) — overrideLimiter, alwaysShow;
            // в 1.21.3 только overrideLimiter (LevelRenderer#addParticle с alwaysShow = false).
            let flags: &[u8] = if matches!(ver(), 5..=8) {
                &[r.u8()?, 0]
            } else {
                r.bytes(2)?
            };
            let pos = r.bytes(24)?;
            let dist = r.bytes(12)?;
            let speed = r.f32()?;
            let count = r.i32()?;
            if !tr().value(&mut r, &mut w, particle_src(), &PARTICLE)? {
                return Ok(Some(Vec::new()));
            }
            w.bytes(flags).bytes(pos).bytes(dist);
            w.f32(speed).f32(speed).f32(speed).var_int(count).var_int(0);
        }
        // SoundEvent.STREAM_CODEC — holder реестра звуков.
        "sound" | "sound_entity" => {
            if !tr().value(&mut r, &mut w, &SOUND, &SOUND)? {
                return Ok(Some(Vec::new()));
            }
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundExplodePacket (1.21.1): xyz, power, toBlow (байтовые смещения),
        // отброс float×3, BlockInteraction, малая и большая частицы, звук. Explosion#finalizeExplosion
        // (клиент): большая частица при power ≥ 2 и BlockInteraction != KEEP, иначе малая.
        "explode" if pre_1_21_2(ver()) => {
            w.bytes(r.bytes(24)?);
            let power = r.f32()?;
            let n = r.length("explosion blocks", 1 << 16)?;
            r.bytes(n * 3)?;
            let kb = [r.f32()?, r.f32()?, r.f32()?];
            let interacts = r.var_int()? != 0;
            w.f32(power).i32(n as i32);
            let has_kb = kb.iter().any(|v| *v != 0.0);
            w.bool(has_kb);
            if has_kb {
                kb.iter().for_each(|v| {
                    w.f64(f64::from(*v));
                });
            }
            let mut small = Writer::new();
            let small_ok = tr().value(&mut r, &mut small, particle_src(), &PARTICLE)?;
            let mut large = Writer::new();
            let large_ok = tr().value(&mut r, &mut large, particle_src(), &PARTICLE)?;
            let (ok, part) = if power >= 2.0 && interacts {
                (large_ok, large)
            } else {
                (small_ok, small)
            };
            if !ok {
                return Ok(Some(Vec::new()));
            }
            w.bytes(&part.into_vec());
            if !tr().value(&mut r, &mut w, &SOUND, &SOUND)? {
                return Ok(Some(Vec::new()));
            }
            w.var_int(0).bool(true);
        }
        // VANILLA: ClientboundExplodePacket (26.3) добавил playSound в конце; в 1.21.11 звук всегда.
        "explode" => {
            w.bytes(r.bytes(8 * 3)?);
            let old = pre_1_21_9(ver());
            if old {
                // VANILLA-UNSURE: radius и blockCount появились в 1.21.9; при blockCount = 0
                // и пустом blockParticles клиент 1.21.9+ не порождает частиц блоков.
                w.f32(0.0).i32(0);
            } else {
                w.bytes(r.bytes(4 + 4)?);
            }
            let kb = r.bool()?;
            w.bool(kb);
            if kb {
                w.bytes(r.bytes(24)?);
            }
            if !tr().value(&mut r, &mut w, particle_src(), &PARTICLE)?
                || !tr().value(&mut r, &mut w, &SOUND, &SOUND)?
            {
                return Ok(Some(Vec::new()));
            }
            if old {
                w.var_int(0).bool(true);
                return Ok(Some(w.into_vec()));
            }
            let n = r.length("explosion particles", 1 << 12)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                let mut one = Writer::new();
                let ok = tr().value(&mut r, &mut one, particle_src(), &PARTICLE)?;
                one.bytes(r.bytes(8)?).var_int(r.var_int()?);
                if ok {
                    list.bytes(&one.into_vec());
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec()).bool(true);
        }
        // VANILLA: ClientboundProjectilePowerPacket (1.20.6) — вектор xPower/yPower/zPower; в 1.21
        // одно accelerationPower по направлению полёта.
        // VANILLA-UNSURE: берётся длина вектора (AbstractHurtingProjectile 1.20.6 → 1.21).
        "projectile_power" if pre_1_21(ver()) => {
            w.var_int(r.var_int()?);
            let (x, y, z) = (r.f64()?, r.f64()?, r.f64()?);
            w.f64((x * x + y * y + z * z).sqrt());
        }
        // VANILLA: AttributeSnapshot — Attribute holder (реестр attribute), base, модификаторы.
        "update_attributes" => {
            w.var_int(r.var_int()?);
            let n = r.length("attributes", 1 << 10)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                // VANILLA: ClientboundUpdateAttributesPacket#write (1.20.4) — атрибут как
                // ResourceLocation, операция модификатора — байт (в 1.20.5 id реестра и varint).
                let raw = if pre_1_20_5(ver()) {
                    let name = r.string(32767)?;
                    let name = name.strip_prefix("minecraft:").unwrap_or(&name);
                    ezzmc_protocol::generated::r1_20_4::ATTRIBUTE
                        .iter()
                        .position(|n| *n == name)
                        .map_or(-1, |i| i as i32)
                } else {
                    r.var_int()?
                };
                let id = tr().id(ClientRegistry::Attribute, raw);
                let start = r.pos();
                let modifiers = if pre_1_21(ver()) {
                    // VANILLA: AttributeSnapshot#MODIFIER_STREAM_CODEC (1.20.6) — UUID вместо
                    // id; имя на клиенте "Unknown synced attribute modifier".
                    let mut m = Writer::new();
                    let base = r.f64()?;
                    let n = r.length("modifiers", 1 << 12)?;
                    m.f64(base).var_int(n as i32);
                    for _ in 0..n {
                        let uuid = r.uuid()?;
                        let id = modifier_id_1_21(uuid, "Unknown synced attribute modifier");
                        let amount = r.f64()?;
                        let op = if pre_1_20_5(ver()) {
                            i32::from(r.u8()?)
                        } else {
                            r.var_int()?
                        };
                        m.string(&id).f64(amount).var_int(op);
                    }
                    m.into_vec()
                } else {
                    r.f64()?;
                    for _ in 0..r.length("modifiers", 1 << 12)? {
                        r.string(32767)?;
                        r.f64()?;
                        r.var_int()?;
                    }
                    r.since(start).to_vec()
                };
                if let Some(id) = id {
                    list.var_int(id).bytes(&modifiers);
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec());
        }
        // 1.21.11: gameTime, dayTime, tickDayTime → 26.3: gameTime, {часы overworld: total, 0, rate}.
        "set_time" => {
            let (game, day) = (r.i64()?, r.i64()?);
            // VANILLA: ClientLevel#setDayTime (1.21.1) — отрицательное время = doDaylightCycle false;
            // в 1.21.2 отдельный bool tickDayTime.
            let (day, tick) = if pre_1_21_2(ver()) {
                (day.abs(), day >= 0)
            } else {
                (day, r.bool()?)
            };
            w.i64(game).var_int(1).var_int(0).var_long(day).f32(0.0);
            w.f32(if tick { 1.0 } else { 0.0 });
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// До 26.1 маски света — long[] (writeBitSet); канон читает byte[] (ByteBufCodecs.BIT_SET).
    #[test]
    fn light_update_masks_to_byte_bitsets() {
        let mut w = Writer::new();
        w.var_int(3).var_int(-2);
        w.bitset(&[0b10]).bitset(&[]).bitset(&[0b100]).bitset(&[]);
        w.var_int(1).var_int(2048).bytes(&[0x11; 2048]).var_int(0);
        let out = clientbound_1_21_11("light_update", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (3, -2));
        assert_eq!(r.bitset_bytes().unwrap(), vec![0b10]);
        assert_eq!(r.bitset_bytes().unwrap(), Vec::<u64>::new());
        assert_eq!(r.bitset_bytes().unwrap(), vec![0b100]);
        assert_eq!(r.bitset_bytes().unwrap(), Vec::<u64>::new());
        assert_eq!(r.var_int().unwrap(), 1);
        assert_eq!(r.var_int().unwrap(), 2048);
    }

    #[test]
    fn block_entity_type_drops_bed() {
        let src = |n: &str| {
            ezzmc_protocol::generated::r1_21_11::BLOCK_ENTITY_TYPE
                .iter()
                .position(|x| *x == n)
                .unwrap() as i32
        };
        let canon = ezzmc_protocol::generated::r26_3::BLOCK_ENTITY_TYPE
            .iter()
            .position(|x| *x == "sign")
            .unwrap() as i32;
        let mut w = Writer::new();
        w.i64(7).var_int(src("bed")).u8(0);
        let out = clientbound_1_21_11("block_entity_data", &w.into_vec()).unwrap();
        assert_eq!(out, Some(Vec::new()));
        let mut w = Writer::new();
        w.i64(7).var_int(src("sign")).u8(0);
        let out = clientbound_1_21_11("block_entity_data", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!((r.i64().unwrap(), r.var_int().unwrap()), (7, canon));
    }

    #[test]
    fn container_click_item_ids_back_to_1_21_11() {
        let id = ezzmc_protocol::generated::v1_21_11::PACKETS
            .iter()
            .find(|p| {
                p.state == State::Play
                    && p.dir == Direction::Serverbound
                    && p.name == "container_click"
            })
            .unwrap()
            .id as i32;
        let pos = |t: &[&str]| t.iter().position(|x| *x == "cherry_log").unwrap() as i32;
        let mut w = Writer::new();
        w.var_int(0)
            .var_int(5)
            .i16(36)
            .u8(0)
            .var_int(0)
            .var_int(1)
            .i16(36);
        w.bool(true)
            .var_int(pos(ezzmc_protocol::generated::r26_3::ITEM))
            .var_int(3)
            .var_int(0)
            .var_int(0);
        w.bool(false);
        let out = serverbound_1_21_11(id, &w.into_vec())
            .unwrap()
            .unwrap()
            .remove(0)
            .1;
        let mut e = Writer::new();
        e.var_int(0)
            .var_int(5)
            .i16(36)
            .u8(0)
            .var_int(0)
            .var_int(1)
            .i16(36);
        e.bool(true)
            .var_int(pos(ezzmc_protocol::generated::r1_21_11::ITEM))
            .var_int(3)
            .var_int(0)
            .var_int(0);
        e.bool(false);
        assert_eq!(out, e.into_vec());
    }

    /// 1.21.8: поля игрока переставлены (18 рука → 15, 15 поглощение → 17), плечевой
    /// COMPOUND_TAG → вариант попугая (OPTIONAL_UNSIGNED_INT, +1), пустое плечо → 0.
    #[test]
    fn entity_data_1_21_8_player_fields() {
        use ezzmc_protocol::nbt::{write_network, Tag};
        let ser8 = |n: &str| SERIALIZERS_1_21_8.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        let nbt = |t: Tag| {
            let mut b = Vec::new();
            write_network(&mut b, Some(&t)).unwrap();
            b
        };
        layouts().insert(-9, (DataLayout::Same, "player"));
        let mut w = Writer::new();
        w.var_int(-9).u8(18).var_int(ser8("BYTE")).u8(1);
        w.u8(15).var_int(ser8("FLOAT")).f32(4.0);
        w.u8(19)
            .var_int(ser8("COMPOUND_TAG"))
            .bytes(&nbt(Tag::Compound(vec![
                ("id".into(), Tag::String("minecraft:parrot".into())),
                ("Variant".into(), Tag::Int(2)),
            ])));
        w.u8(20)
            .var_int(ser8("COMPOUND_TAG"))
            .bytes(&nbt(Tag::Compound(vec![])));
        w.u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 0).unwrap();
        let mut e = Writer::new();
        e.var_int(-9)
            .u8(15)
            .var_int(canon("HUMANOID_ARM"))
            .var_int(1);
        e.u8(17).var_int(canon("FLOAT")).f32(4.0);
        e.u8(19).var_int(canon("OPTIONAL_UNSIGNED_INT")).var_int(3);
        e.u8(20).var_int(canon("OPTIONAL_UNSIGNED_INT")).var_int(0);
        e.u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    /// 1.21.5: у рамки нет HangingEntity#DATA_DIRECTION — предмет (#8) и поворот (#9) → #9/#10.
    #[test]
    fn entity_data_1_21_5_item_frame_shift() {
        let ser = |n: &str| SERIALIZERS_1_21_8.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        layouts().insert(-11, (DataLayout::Same, "item_frame"));
        let mut w = Writer::new();
        w.var_int(-11).u8(9).var_int(ser("INT")).var_int(3);
        w.u8(0).var_int(ser("BYTE")).u8(0);
        w.u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 3).unwrap();
        let mut e = Writer::new();
        e.var_int(-11).u8(10).var_int(canon("INT")).var_int(3);
        e.u8(0).var_int(canon("BYTE")).u8(0);
        e.u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    /// 1.21.4: седло свиньи (#17) пропускается, разгон #18 → #17; вариант грибной коровы
    /// STRING → INT; WOLF_VARIANT по таблице сериализаторов 1.21.4.
    #[test]
    fn entity_data_1_21_4_fields() {
        let ser = |n: &str| SERIALIZERS_1_21_4.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        layouts().insert(-21, (DataLayout::Same, "pig"));
        layouts().insert(-22, (DataLayout::Same, "mooshroom"));
        layouts().insert(-23, (DataLayout::Same, "wolf"));
        let run = |w: Writer| {
            let mut out = Writer::new();
            set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 4).unwrap();
            out.into_vec()
        };
        let mut w = Writer::new();
        w.var_int(-21).u8(17).var_int(ser("BOOLEAN")).u8(1);
        w.u8(18).var_int(ser("INT")).var_int(40).u8(0xFF);
        let mut e = Writer::new();
        e.var_int(-21)
            .u8(17)
            .var_int(canon("INT"))
            .var_int(40)
            .u8(0xFF);
        assert_eq!(run(w), e.into_vec());

        let mut w = Writer::new();
        w.var_int(-22)
            .u8(17)
            .var_int(ser("STRING"))
            .string("brown")
            .u8(0xFF);
        let mut e = Writer::new();
        e.var_int(-22)
            .u8(17)
            .var_int(canon("INT"))
            .var_int(1)
            .u8(0xFF);
        assert_eq!(run(w), e.into_vec());

        let mut w = Writer::new();
        w.var_int(-23)
            .u8(22)
            .var_int(ser("WOLF_VARIANT"))
            .var_int(2)
            .u8(0xFF);
        let mut e = Writer::new();
        e.var_int(-23)
            .u8(22)
            .var_int(canon("WOLF_VARIANT"))
            .var_int(2)
            .u8(0xFF);
        assert_eq!(run(w), e.into_vec());
    }

    /// 1.21.3: вариант лосося строкой → INT 1.21.4+ (small 0, medium 1, large 2).
    #[test]
    fn entity_data_1_21_3_salmon_variant() {
        let ser = |n: &str| SERIALIZERS_1_21_4.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        layouts().insert(-31, (DataLayout::Same, "salmon"));
        let mut w = Writer::new();
        w.var_int(-31)
            .u8(17)
            .var_int(ser("STRING"))
            .string("large")
            .u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 5).unwrap();
        let mut e = Writer::new();
        e.var_int(-31)
            .u8(17)
            .var_int(canon("INT"))
            .var_int(2)
            .u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    /// 1.21.1: у лодки нет порода-поля (#11 TYPE выбрасывается, весла/пузыри −1), у стрелы
    /// цвет эффекта #10 → #11 (AbstractArrow#IN_GROUND в 1.21.2).
    #[test]
    fn entity_data_1_21_1_boat_and_arrow() {
        let ser = |n: &str| SERIALIZERS_1_21_4.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        layouts().insert(-41, (DataLayout::Same, "oak_boat"));
        let mut w = Writer::new();
        w.var_int(-41).u8(11).var_int(ser("INT")).var_int(3);
        w.u8(12).var_int(ser("BOOLEAN")).u8(1).u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 6).unwrap();
        let mut e = Writer::new();
        e.var_int(-41)
            .u8(11)
            .var_int(canon("BOOLEAN"))
            .u8(1)
            .u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
        layouts().insert(-42, (DataLayout::Same, "arrow"));
        let mut w = Writer::new();
        w.var_int(-42)
            .u8(10)
            .var_int(ser("INT"))
            .var_int(-1)
            .u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 6).unwrap();
        let mut e = Writer::new();
        e.var_int(-42)
            .u8(11)
            .var_int(canon("INT"))
            .var_int(-1)
            .u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    /// 1.21.10: рука игрока BYTE → HUMANOID_ARM, гнев волка INT → LONG, номера сериализаторов
    /// по таблице 1.21.10 (FLOAT = 3 в обеих).
    #[test]
    fn entity_data_1_21_10_retyped_fields() {
        let ser10 = |n: &str| SERIALIZERS_1_21_10.iter().position(|x| *x == n).unwrap() as i32;
        let canon = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap() as i32;
        layouts().insert(-7, (DataLayout::Same, "player"));
        let mut w = Writer::new();
        w.var_int(-7).u8(15).var_int(ser10("BYTE")).u8(0);
        w.u8(9).var_int(ser10("FLOAT")).f32(20.0);
        w.u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 1).unwrap();
        let mut e = Writer::new();
        e.var_int(-7)
            .u8(15)
            .var_int(canon("HUMANOID_ARM"))
            .var_int(0);
        e.u8(9).var_int(canon("FLOAT")).f32(20.0);
        e.u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());

        layouts().insert(-8, (DataLayout::Ageable, "wolf"));
        let mut w = Writer::new();
        w.var_int(-8)
            .u8(21)
            .var_int(ser10("INT"))
            .var_int(400)
            .u8(0xFF);
        let mut out = Writer::new();
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, 1).unwrap();
        let mut e = Writer::new();
        e.var_int(-8)
            .u8(22)
            .var_int(canon("LONG"))
            .var_long(i64::MAX)
            .u8(0xFF);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    #[test]
    fn block_event_uses_canon_block_id() {
        let src = ezzmc_protocol::generated::r1_21_11::BLOCK
            .iter()
            .position(|x| *x == "cherry_log")
            .unwrap() as i32;
        let dst = ezzmc_protocol::generated::r26_3::BLOCK
            .iter()
            .position(|x| *x == "cherry_log")
            .unwrap() as i32;
        let mut w = Writer::new();
        w.i64(0).u8(1).u8(2).var_int(src);
        let out = clientbound_1_21_11("block_event", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut e = Writer::new();
        e.i64(0).u8(1).u8(2).var_int(dst);
        assert_eq!(out, e.into_vec());
    }

    #[test]
    fn ghost_recipe_slot_displays_renumbered() {
        let stick = ezzmc_protocol::generated::r1_21_11::ITEM
            .iter()
            .position(|x| *x == "stick")
            .unwrap() as i32;
        let canon = ezzmc_protocol::generated::r26_3::ITEM
            .iter()
            .position(|x| *x == "stick")
            .unwrap() as i32;
        let mut w = Writer::new();
        // stonecutter: input = tag, result = item_stack(4 stick), station = composite[any_fuel].
        w.var_int(7).var_int(3);
        w.var_int(4).string("minecraft:planks");
        w.var_int(3).var_int(4).var_int(stick).var_int(0).var_int(0);
        w.var_int(7).var_int(1).var_int(1);
        let out = clientbound_1_21_11("place_ghost_recipe", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut e = Writer::new();
        e.var_int(7).var_int(3);
        e.var_int(6).var_int(0).string("minecraft:planks");
        e.var_int(5).var_int(canon).var_int(4).var_int(0).var_int(0);
        e.var_int(10).var_int(1).var_int(1);
        assert_eq!(out, e.into_vec());
    }

    #[test]
    fn entity_data_shifts_cat_fields() {
        let ty = ezzmc_protocol::generated::r1_21_11::ENTITY_TYPE
            .iter()
            .position(|x| *x == "cat")
            .unwrap() as i32;
        let mut w = Writer::new();
        w.var_int(4242).bytes(&[0; 16]).var_int(ty);
        clientbound_1_21_11("add_entity", &w.into_vec()).unwrap();
        let mut w = Writer::new();
        // #0 BYTE, #19 CAT_VARIANT (21), #20 BOOLEAN IS_LYING, #22 INT collar.
        w.var_int(4242).u8(0).var_int(0).u8(3);
        w.u8(19).var_int(21).var_int(5);
        w.u8(20).var_int(8).u8(1);
        w.u8(22).var_int(1).var_int(14).u8(0xFF);
        let out = clientbound_1_21_11("set_entity_data", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 4242);
        let mut got = Vec::new();
        loop {
            let i = r.u8().unwrap();
            if i == 0xFF {
                break;
            }
            let s = r.var_int().unwrap();
            let v = if s == 8 || s == 0 {
                r.u8().unwrap() as i32
            } else {
                r.var_int().unwrap()
            };
            got.push((i, s, v));
        }
        assert_eq!(got, [(0, 0, 3), (20, 21, 5), (22, 8, 1), (24, 1, 14)]);
        assert_eq!(DataLayout::of("slime").index(16), 18);
        assert_eq!(DataLayout::of("zombie").index(17), 17);
    }

    #[test]
    fn set_time_becomes_clock_map() {
        let mut w = Writer::new();
        w.i64(100).i64(6000).bool(false);
        let out = clientbound_1_21_11("set_time", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!((r.i64().unwrap(), r.var_int().unwrap()), (100, 1));
        assert_eq!((r.var_int().unwrap(), r.var_long().unwrap()), (0, 6000));
        assert_eq!((r.f32().unwrap(), r.f32().unwrap()), (0.0, 0.0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn level_particles_reordered_with_block_state() {
        let src = |n: &str| {
            ezzmc_protocol::generated::r1_21_11::PARTICLE_TYPE
                .iter()
                .position(|x| *x == n)
                .unwrap() as i32
        };
        let canon = |n: &str| {
            ezzmc_protocol::generated::r26_3::PARTICLE_TYPE
                .iter()
                .position(|x| *x == n)
                .unwrap() as i32
        };
        let mut w = Writer::new();
        w.bool(true).bool(false).f64(1.0).f64(2.0).f64(3.0);
        w.f32(0.5).f32(0.5).f32(0.5).f32(0.25).i32(7);
        w.var_int(src("block")).var_int(1);
        let out = clientbound_1_21_11("level_particles", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), canon("block"));
        r.var_int().unwrap(); // состояние блока (перевод — ezzmc-world)
        assert_eq!((r.bool().unwrap(), r.bool().unwrap()), (true, false));
        assert_eq!(r.f64().unwrap(), 1.0);
        r.bytes(16 + 12).unwrap();
        assert_eq!(
            [r.f32().unwrap(), r.f32().unwrap(), r.f32().unwrap()],
            [0.25; 3]
        );
        assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (7, 0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn move_pos_rot_roundtrip_layout() {
        let mut w = Writer::new();
        w.var_int(7).i16(1).i16(-2).i16(3).u8(10).u8(20).u8(1);
        let out = clientbound_1_21_11("move_entity_pos_rot", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 7);
        assert_eq!(r.var_int().unwrap(), 1);
        assert_eq!(
            (r.i16().unwrap(), r.i16().unwrap(), r.i16().unwrap()),
            (1, -2, 3)
        );
        assert_eq!((r.i8().unwrap(), r.i8().unwrap()), (10, 20));
    }
}
