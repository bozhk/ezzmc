//! Трансляция тел пакетов сервера старой версии в формат канона (26.3) — глава 18, TODO-372.
//! Весь клиент разбирает только канонический формат; здесь исправляются расхождения
//! между версиями. Реестровые id (предметы, сущности, звуки, частицы) — позже в этом же месте.
use ezzmc_protocol::item::{ClientRegistry, Shape, PARTICLE, PARTICLE_1_21_11, SOUND};
use ezzmc_protocol::translate::Translator;
use ezzmc_protocol::version::{build_remap, CANON, V1_21_10, V1_21_11};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, ProtocolResult, State};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Соединение с сервером 1.21.9/1.21.10 (773)? Иначе 1.21.11 (774).
fn is_1_21_10() -> bool {
    ezzmc_protocol::version::current().protocol == V1_21_10.protocol
}

fn entity_type_map() -> &'static [Option<u32>] {
    static T10: OnceLock<Vec<Option<u32>>> = OnceLock::new();
    static T11: OnceLock<Vec<Option<u32>>> = OnceLock::new();
    if is_1_21_10() {
        T10.get_or_init(|| build_remap(V1_21_10.entity_type, CANON.entity_type))
    } else {
        T11.get_or_init(|| build_remap(V1_21_11.entity_type, CANON.entity_type))
    }
}

/// Реестр block_entity_type 1.21.10/1.21.11 → канон (в 26.x убран `bed`, добавлен `potent_sulfur`).
fn block_entity_type(id: i32) -> Option<i32> {
    use ezzmc_protocol::generated::{r1_21_10, r1_21_11, r26_3};
    static T10: OnceLock<Vec<Option<u32>>> = OnceLock::new();
    static T11: OnceLock<Vec<Option<u32>>> = OnceLock::new();
    let t = if is_1_21_10() {
        T10.get_or_init(|| build_remap(r1_21_10::BLOCK_ENTITY_TYPE, r26_3::BLOCK_ENTITY_TYPE))
    } else {
        T11.get_or_init(|| build_remap(r1_21_11::BLOCK_ENTITY_TYPE, r26_3::BLOCK_ENTITY_TYPE))
    };
    t.get(usize::try_from(id).ok()?)
        .copied()
        .flatten()
        .map(|v| v as i32)
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
        "ITEM_STACK" => tr().stack(r, w)?,
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
        "PARTICLE" => return tr().value(r, w, &PARTICLE_1_21_11, &PARTICLE),
        "PARTICLES" => {
            let n = r.length("particles", 1 << 12)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                if tr().value(r, &mut list, &PARTICLE_1_21_11, &PARTICLE)? {
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
    use ezzmc_protocol::translate::{v1_21_10, v1_21_11};
    if is_1_21_10() {
        v1_21_10(ezzmc_world::legacy::state_to_canon)
    } else {
        v1_21_11(ezzmc_world::legacy::state_to_canon)
    }
}

/// Канон → версия соединения; состояния блоков 1.21.10 и 1.21.11 совпадают.
fn tr_back() -> &'static Translator {
    use ezzmc_protocol::translate::{to_1_21_10, to_1_21_11};
    let back = ezzmc_world::legacy::state_from_canon_1_21_11;
    if is_1_21_10() {
        to_1_21_10(back)
    } else {
        to_1_21_11(back)
    }
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

/// VANILLA: ClientboundSetEntityDataPacket (1.21.10/1.21.11) → 26.3; `v10` — сервер 1.21.10.
fn set_entity_data(r: &mut Reader, w: &mut Writer, v10: bool) -> ProtocolResult<()> {
    let id = r.var_int()?;
    let (layout, ty) = layouts()
        .get(&id)
        .copied()
        .unwrap_or((DataLayout::Same, ""));
    let table = if v10 {
        SERIALIZERS_1_21_10
    } else {
        SERIALIZERS_1_21_11
    };
    let canon_of = |n: &str| SERIALIZERS_26_3.iter().position(|x| *x == n).unwrap_or(0);
    w.var_int(id);
    loop {
        let idx = r.u8()?;
        if idx == 0xFF {
            break;
        }
        let src = r.var_int()?;
        let ser = usize::try_from(src).ok().and_then(|i| table.get(i)).ok_or(
            ezzmc_protocol::ProtocolError::InvalidValue {
                what: "entity data serializer",
                value: src.into(),
            },
        )?;
        let mut one = Writer::new();
        match v10.then(|| retyped_1_21_10(ty, idx)).flatten() {
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

/// Пакет клиента (собран в каноне, id из таблицы версии) → тело для сервера 1.21.10/1.21.11.
/// `None` — тело без изменений.
pub fn serverbound_1_21_11(id: i32, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let table = if is_1_21_10() {
        V1_21_10.packets
    } else {
        V1_21_11.packets
    };
    let Some(name) = table
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.id as i32 == id)
        .map(|p| p.name)
    else {
        return Ok(None);
    };
    let mut r = Reader::new(body);
    let mut w = Writer::new();
    match name {
        // VANILLA: ServerboundContainerClickPacket — containerId, stateId, slot, button,
        // ClickType (1.21.11) = ContainerInput (26.3), changedSlots, carried.
        "container_click" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            w.bytes(r.bytes(3)?).var_int(r.var_int()?);
            let n = r.length("changed slots", 128)?;
            w.var_int(n as i32);
            for _ in 0..n {
                w.i16(r.i16()?);
                hashed_stack_back(&mut r, &mut w)?;
            }
            hashed_stack_back(&mut r, &mut w)?;
        }
        // VANILLA: ServerboundSetCreativeModeSlotPacket — short slot, OPTIONAL_UNTRUSTED stack.
        "set_creative_mode_slot" => {
            w.i16(r.i16()?);
            tr_back().stack_untrusted(&mut r, &mut w)?;
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
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
            let start = r.pos();
            r.bytes(8)?;
            for _ in 0..r.length("heightmaps", 16)? {
                r.var_int()?;
                let n = r.length("heightmap longs", 1 << 12)?;
                r.bytes(n * 8)?;
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
            w.var_int(kept).bytes(&list.into_vec()).bytes(r.rest());
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
        "place_ghost_recipe" => {
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
                    if !tr().value(&mut r, &mut icon, &Shape::Stack, &Shape::ItemTemplate)? {
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
        }
        "login" => {
            layouts().clear();
            return Ok(None);
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
        "set_entity_data" => set_entity_data(&mut r, &mut w, is_1_21_10())?,
        // Стеки: ItemStack.OPTIONAL_STREAM_CODEC (id предметов и компонентов, формы компонентов).
        "container_set_content" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            let n = r.length("container items", 1 << 16)?;
            w.var_int(n as i32);
            for _ in 0..=n {
                tr().stack(&mut r, &mut w)?;
            }
        }
        "container_set_slot" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?).i16(r.i16()?);
            tr().stack(&mut r, &mut w)?;
        }
        "set_cursor_item" => tr().stack(&mut r, &mut w)?,
        "set_player_inventory" => {
            w.var_int(r.var_int()?);
            tr().stack(&mut r, &mut w)?;
        }
        // VANILLA: ClientboundSetEquipmentPacket — слот с битом 0x80 «дальше ещё».
        "set_equipment" => {
            w.var_int(r.var_int()?);
            loop {
                let slot = r.u8()?;
                w.u8(slot);
                tr().stack(&mut r, &mut w)?;
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
                tr().stack(&mut r, &mut w)?;
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
            let flags = r.bytes(2)?;
            let pos = r.bytes(24)?;
            let dist = r.bytes(12)?;
            let speed = r.f32()?;
            let count = r.i32()?;
            if !tr().value(&mut r, &mut w, &PARTICLE_1_21_11, &PARTICLE)? {
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
        // VANILLA: ClientboundExplodePacket (26.3) добавил playSound в конце; в 1.21.11 звук всегда.
        "explode" => {
            w.bytes(r.bytes(8 * 3 + 4 + 4)?);
            let kb = r.bool()?;
            w.bool(kb);
            if kb {
                w.bytes(r.bytes(24)?);
            }
            if !tr().value(&mut r, &mut w, &PARTICLE_1_21_11, &PARTICLE)?
                || !tr().value(&mut r, &mut w, &SOUND, &SOUND)?
            {
                return Ok(Some(Vec::new()));
            }
            let n = r.length("explosion particles", 1 << 12)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                let mut one = Writer::new();
                let ok = tr().value(&mut r, &mut one, &PARTICLE_1_21_11, &PARTICLE)?;
                one.bytes(r.bytes(8)?).var_int(r.var_int()?);
                if ok {
                    list.bytes(&one.into_vec());
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec()).bool(true);
        }
        // VANILLA: AttributeSnapshot — Attribute holder (реестр attribute), base, модификаторы.
        "update_attributes" => {
            w.var_int(r.var_int()?);
            let n = r.length("attributes", 1 << 10)?;
            let mut list = Writer::new();
            let mut kept = 0;
            for _ in 0..n {
                let id = tr().id(ClientRegistry::Attribute, r.var_int()?);
                let start = r.pos();
                r.f64()?;
                for _ in 0..r.length("modifiers", 1 << 12)? {
                    r.string(32767)?;
                    r.f64()?;
                    r.var_int()?;
                }
                if let Some(id) = id {
                    list.var_int(id).bytes(r.since(start));
                    kept += 1;
                }
            }
            w.var_int(kept).bytes(&list.into_vec());
        }
        // 1.21.11: gameTime, dayTime, tickDayTime → 26.3: gameTime, {часы overworld: total, 0, rate}.
        "set_time" => {
            let (game, day, tick) = (r.i64()?, r.i64()?, r.bool()?);
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
        let out = serverbound_1_21_11(id, &w.into_vec()).unwrap().unwrap();
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
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, true).unwrap();
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
        set_entity_data(&mut Reader::new(&w.into_vec()), &mut out, true).unwrap();
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
