//! Отличия протокола 1.12.2 (340) от 1.13.2: чанк и предметы приводятся к формату 1.13.2, дальше идут
//! путём `v1132`. Id состояний блоков остаются `id<<4 | meta` (переводит `world::legacy`).

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use ezzmc_protocol::generated::r1_13_2::ITEM;
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};
use ezzmc_world::generated::flattening::ITEMS;

use crate::entity_data_gen::REMAP_1_12;
use crate::v1122_gen::{ENTITIES, SOUND_RENAMES};
use crate::v1201::Pre18;

const SECTION_BLOCKS: usize = 4096;
const LIGHT_BYTES: usize = 2048;
const BIOMES: usize = 256;
const FILLED_MAP: u16 = 358;
const SPAWN_EGG: u16 = 383;

/// Названия картин 1.12.2 в порядке id 1.13.2. VANILLA: PaintingEntity.PaintingMotive (1.12.2),
/// Painting (1.13.2).
const PAINTINGS: [&str; 26] = [
    "Kebab",
    "Aztec",
    "Alban",
    "Aztec2",
    "Bomb",
    "Plant",
    "Wasteland",
    "Pool",
    "Courbet",
    "Sea",
    "Sunset",
    "Creebet",
    "Wanderer",
    "Graham",
    "Match",
    "Bust",
    "Stage",
    "Void",
    "SkullAndRoses",
    "Wither",
    "Fighters",
    "Pointer",
    "Pigscene",
    "BurningSkull",
    "Skeleton",
    "DonkeyKong",
];

/// Числовые id зачарований 1.12.2. VANILLA: Enchantment#register (1.12.2).
const ENCHANTMENTS: &[(i16, &str)] = &[
    (0, "protection"),
    (1, "fire_protection"),
    (2, "feather_falling"),
    (3, "blast_protection"),
    (4, "projectile_protection"),
    (5, "respiration"),
    (6, "aqua_affinity"),
    (7, "thorns"),
    (8, "depth_strider"),
    (9, "frost_walker"),
    (10, "binding_curse"),
    (16, "sharpness"),
    (17, "smite"),
    (18, "bane_of_arthropods"),
    (19, "knockback"),
    (20, "fire_aspect"),
    (21, "looting"),
    (22, "sweeping"),
    (32, "efficiency"),
    (33, "silk_touch"),
    (34, "unbreaking"),
    (35, "fortune"),
    (48, "power"),
    (49, "punch"),
    (50, "flame"),
    (51, "infinity"),
    (61, "luck_of_the_sea"),
    (62, "lure"),
    (70, "mending"),
    (71, "vanishing_curse"),
];

/// Строка с кодами `§` → JSON-компонент 1.13.
fn text(w: &mut Writer, s: &str) {
    w.string(&serde_json::json!({ "text": s }).to_string());
}

/// Последний запрос подсказок: ответ ≤1.12.2 не несёт ни номера, ни диапазона замены.
static SUGGEST_REQUEST: Mutex<(i32, String)> = Mutex::new((0, String::new()));

fn suggest_request() -> MutexGuard<'static, (i32, String)> {
    SUGGEST_REQUEST
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Serverbound `command_suggestion` 1.13 (номер, строка) → 1.12.2 (строка, assumeCommand,
/// позиция блока под прицелом). VANILLA: RequestCommandCompletionsC2SPacket#write (1.12.2).
/// В чате assumeCommand = false; позиция не передаётся.
pub(crate) fn suggestion_request_to_1_12(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    let text = r.string(32767)?;
    let mut w = Writer::with_capacity(text.len() + 5);
    w.string(&text).bool(false).bool(false);
    *suggest_request() = (id, text);
    Ok(w.into_vec())
}

/// Ответ 1.12.2 (список строк) → 1.13 (номер, начало, длина, строки без подсказок).
/// VANILLA: ClientPlayNetworkHandler#onCommandSuggestions (1.12.2) сортирует список;
/// заменяется слово перед курсором (после последнего пробела).
fn suggestions_to_1_13(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let n = r.length("suggestions", 1 << 16)?;
    let mut items = Vec::new();
    for _ in 0..n {
        let s = r.string(32767)?;
        if !s.is_empty() {
            items.push(s);
        }
    }
    items.sort();
    let (id, text) = suggest_request().clone();
    let total = text.chars().count();
    let start = text.rfind(' ').map_or(0, |i| text[..i].chars().count() + 1);
    w.var_int(id)
        .var_int(start as i32)
        .var_int((total - start) as i32)
        .var_int(items.len() as i32);
    for s in &items {
        w.string(s).bool(false);
    }
    Ok(())
}

/// Пакеты, формат которых отличается и пока не переводится.
pub(crate) fn is_untranslated(name: &str) -> bool {
    matches!(name, "custom_payload")
}

/// id сущности → тип 1.13.2 (из спавна) — для индексов метаданных.
fn kinds() -> MutexGuard<'static, HashMap<i32, &'static str>> {
    static M: OnceLock<Mutex<HashMap<i32, &'static str>>> = OnceLock::new();
    M.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// Состояние блока в пакетах сущностей и событий (`id | meta << 12`) → `id << 4 | meta`.
/// VANILLA: Block#getByBlockState (1.12.2).
fn state(v: i32) -> i32 {
    ((v & 0xFFF) << 4) | ((v >> 12) & 15)
}

fn index_of(list: &[&str], name: &str) -> Option<i32> {
    let at = list
        .iter()
        .position(|n| n.trim_start_matches("minecraft:") == name)?;
    i32::try_from(at).ok()
}

/// Метаданные 1.12.2 → 1.13.2: индексы по типу сущности, сериализаторы с id 5 сдвинуты на 1
/// (новый OptionalText), имя строкой → компонент.
/// VANILLA: DataTracker#writeEntryToPacket, TrackedDataHandlerRegistry (1.12.2).
fn metadata(kind: &str, r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let map = REMAP_1_12
        .binary_search_by_key(&kind, |(n, _)| *n)
        .ok()
        .and_then(|at| REMAP_1_12.get(at))
        .map(|(_, m)| *m)
        .unwrap_or_default();
    // VANILLA: EvocationIllagerEntity (1.11.2) — заклинание байтом в индексе 12; в 1.12 индекс 12
    // занял общий флаг иллагеров (class_3162), заклинание — 13.
    let evoker_1_11 = kind == "evoker" && ezzmc_protocol::version::current().protocol < 335;
    loop {
        let mut idx = r.u8()?;
        if idx == 0xFF {
            break;
        }
        if evoker_1_11 && idx == 12 {
            idx = 13;
        }
        let ser = r.var_int()?;
        let mut v = Writer::new();
        let mut new_ser = if ser >= 5 { ser + 1 } else { ser };
        match ser {
            0 | 6 => {
                v.u8(r.u8()?);
            }
            1 | 10 => {
                v.var_int(r.var_int()?);
            }
            2 => {
                v.bytes(r.bytes(4)?);
            }
            3 if idx == 2 => {
                new_ser = 5;
                let name = r.string(32767)?;
                v.bool(!name.is_empty());
                if !name.is_empty() {
                    v.string(&serde_json::json!({ "text": name }).to_string());
                }
            }
            3 | 4 => {
                v.string(&r.string(262144)?);
            }
            // предмет остаётся в формате 1.12.2: его переводит общий путь по версии сервера
            5 => {
                let at = r.pos();
                if r.i16()? >= 0 {
                    r.bytes(3)?;
                    nbt::read_named(r)?;
                }
                v.bytes(r.since(at));
            }
            7 => {
                v.bytes(r.bytes(12)?);
            }
            8 => {
                v.bytes(r.bytes(8)?);
            }
            9 | 11 => {
                let some = r.bool()?;
                v.bool(some);
                if some {
                    v.bytes(r.bytes(if ser == 9 { 8 } else { 16 })?);
                }
            }
            12 => {
                let s = r.var_int()?;
                v.var_int(if s == 0 { 0 } else { state(s) });
            }
            13 => {
                let at = r.pos();
                nbt::read_named(r)?;
                v.bytes(r.since(at));
            }
            _ => {
                return Err(ProtocolError::InvalidValue {
                    what: "1.12 entity data serializer",
                    value: ser.into(),
                })
            }
        }
        // поле без пары в 1.13 прочитано и отброшено
        match map.get(usize::from(idx)) {
            Some(&new) if new != 0xFF => {
                w.u8(new).var_int(new_ser).bytes(&v.into_vec());
            }
            _ => {}
        }
    }
    w.u8(0xFF);
    Ok(())
}

/// Частица 1.12.2 по числовому id → имя 1.13.2 (`None` — в 1.13 её нет).
/// VANILLA: ParticleType (1.12.2), ParticleTypes (1.13.2).
fn particle_name(id: i32) -> Option<&'static str> {
    const NAMES: [&str; 49] = [
        "poof",
        "explosion",
        "explosion_emitter",
        "firework",
        "bubble",
        "splash",
        "fishing",
        "underwater",
        "",
        "crit",
        "enchanted_hit",
        "smoke",
        "large_smoke",
        "effect",
        "instant_effect",
        "entity_effect",
        "ambient_entity_effect",
        "witch",
        "dripping_water",
        "dripping_lava",
        "angry_villager",
        "happy_villager",
        "mycelium",
        "note",
        "portal",
        "enchant",
        "flame",
        "lava",
        "",
        "cloud",
        "dust",
        "item_snowball",
        "poof",
        "item_slime",
        "heart",
        "barrier",
        "item",
        "block",
        "block",
        "rain",
        "",
        "elder_guardian",
        "dragon_breath",
        "end_rod",
        "damage_indicator",
        "sweep_attack",
        "falling_dust",
        "totem_of_undying",
        "spit",
    ];
    let name = NAMES.get(usize::try_from(id).ok()?)?;
    (!name.is_empty()).then_some(*name)
}

/// VANILLA: ParticleS2CPacket#read (1.12.2 / 1.13.2). Цвет `dust` в 1.12.2 — смещения при
/// нулевом счётчике (RedstoneParticle), иначе красный.
fn particles(r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
    let id = r.i32()?;
    let long = r.u8()?;
    let mut floats = [0.0f32; 7]; // x, y, z, смещения, скорость
    for f in &mut floats {
        *f = r.f32()?;
    }
    let count = r.i32()?;
    let version = ezzmc_protocol::version::current();
    let Some(name) = particle_name(id) else {
        return Ok(false);
    };
    let Some(new) = index_of(version.particle_type, name) else {
        return Ok(false);
    };
    w.i32(new).u8(long);
    for f in floats {
        w.f32(f);
    }
    w.i32(count);
    match name {
        "block" | "falling_dust" => {
            w.var_int(state(r.var_int()?));
        }
        "item" => {
            let (item, damage) = (r.var_int()?, r.var_int()?);
            let mut old = Writer::new();
            old.i16(i16::try_from(item).unwrap_or(-1));
            old.u8(1).i16(i16::try_from(damage).unwrap_or(0)).u8(0);
            item_to_1_13_2(&mut Reader::new(&old.into_vec()), w)?;
        }
        "dust" => {
            let [.., ox, oy, oz, speed] = floats;
            let mut color = [1.0, 0.0, 0.0];
            if count == 0 {
                color = [ox * speed, oy * speed, oz * speed];
                if color[0] == 0.0 {
                    color[0] = 1.0;
                }
            }
            for c in color {
                w.f32(c);
            }
            w.f32(1.0);
        }
        _ => {}
    }
    Ok(true)
}

/// VANILLA: WorldRenderer#processWorldEvent 2000 (1.12.2): data = (dx+1) + (dz+1)*3 →
/// Direction 3D (1.13+). VANILLA-UNSURE: диагонали сводятся к оси x, центр — вверх.
fn smoke_direction(data: i32) -> i32 {
    match (data.rem_euclid(3) - 1, (data / 3).rem_euclid(3) - 1) {
        (-1, _) => 4,
        (1, _) => 5,
        (_, -1) => 2,
        (_, 1) => 3,
        _ => 1,
    }
}

/// Ответ keep_alive: ниже 340 сервер ждёт varint вместо long.
pub(crate) fn keep_alive_reply(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    if ezzmc_protocol::version::current().protocol >= 340 {
        return Ok(body.to_vec());
    }
    let mut w = Writer::new();
    w.var_int(Reader::new(body).i64()? as i32);
    Ok(w.into_vec())
}

/// Тело клиентского пакета 1.12.2 → формат 1.13.2.
pub(crate) fn to_1_13_2(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 16);
    let version = ezzmc_protocol::version::current();
    match name {
        "command_suggestions" => suggestions_to_1_13(&mut r, &mut w)?,
        // VANILLA: MapUpdateS2CPacket: значок 1.12.2 — байт `тип<<4|поворот`, x, z; 1.13.2 —
        // тип, x, z, поворот, необязательное имя.
        "map_item_data" => {
            w.var_int(r.var_int()?).u8(r.u8()?).bool(r.bool()?);
            let n = r.length("map icons", 1 << 16)?;
            w.var_int(n as i32);
            for _ in 0..n {
                let packed = r.u8()?;
                w.var_int(i32::from(packed >> 4))
                    .bytes(r.bytes(2)?)
                    .u8(packed & 15)
                    .bool(false);
            }
            w.bytes(r.rest());
        }
        // VANILLA: ScoreboardObjectiveUpdateS2CPacket: 1.12.2 — строки заголовка и типа;
        // 1.13.2 — Component и enum (INTEGER, HEARTS).
        "set_objective" => {
            w.string(&r.string(16)?);
            let mode = r.u8()?;
            w.u8(mode);
            if mode == 0 || mode == 2 {
                text(&mut w, &r.string(32)?);
                w.var_int(i32::from(r.string(16)? == "hearts"));
            }
        }
        // VANILLA: TeamS2CPacket: 1.13.2 — заголовок-Component, флаги, правила, цвет (varint,
        // 21 = RESET), приставка и суффикс-Component; 1.12.2 — три строки перед флагами.
        "set_player_team" => {
            w.string(&r.string(16)?);
            let mode = r.u8()?;
            w.u8(mode);
            if mode == 0 || mode == 2 {
                let (title, prefix, suffix) = (r.string(32)?, r.string(16)?, r.string(16)?);
                text(&mut w, &title);
                w.u8(r.u8()?).string(&r.string(32)?).string(&r.string(32)?);
                let color = r.u8()?;
                w.var_int(if color < 16 { i32::from(color) } else { 21 });
                text(&mut w, &prefix);
                text(&mut w, &suffix);
            }
            w.bytes(r.rest());
        }
        // VANILLA: KeepAlivePacket (до 1.12.2 — varint, байткод jar 1.12/1.12.1).
        "keep_alive" if version.protocol < 340 => {
            w.i64(i64::from(r.var_int()?));
        }
        // VANILLA: MobSpawnS2CPacket (1.12.2): id, uuid, varint тип, xyz, 3 угла, скорость,
        // метаданные. Хвост метаданных к этому месту уже отделён в `set_entity_data`
        // (`v1152::expand_1_14`).
        "add_mob" => {
            let id = r.var_int()?;
            w.var_int(id).bytes(r.bytes(16)?);
            let old = r.var_int()?;
            let kind = ENTITIES
                .binary_search_by_key(&old, |(i, _)| *i)
                .ok()
                .and_then(|at| ENTITIES.get(at))
                .map(|(_, n)| *n);
            let Some((kind, new)) = kind.and_then(|k| Some((k, index_of(version.entity_type, k)?)))
            else {
                return Ok(Pre18::Drop);
            };
            kinds().insert(id, kind);
            w.var_int(new).bytes(r.bytes(24 + 3 + 6)?);
            if r.remaining() > 0 {
                metadata(kind, &mut r, &mut w)?;
            }
        }
        "add_player" => {
            let id = r.var_int()?;
            kinds().insert(id, "player");
            w.var_int(id).bytes(r.bytes(16 + 24 + 2)?);
            if r.remaining() > 0 {
                metadata("player", &mut r, &mut w)?;
            }
        }
        // VANILLA: ClientPlayNetworkHandler#onEntitySpawn (1.12.2): data падающего блока —
        // состояние, рамки — горизонтальное направление.
        "add_entity" => {
            let id = r.var_int()?;
            w.var_int(id).bytes(r.bytes(16)?);
            let object = r.u8()?;
            w.u8(object).bytes(r.bytes(24 + 2)?);
            let data = r.i32()?;
            if let Some(kind) = crate::v1132::object_name(object, data) {
                kinds().insert(id, kind);
            }
            let data = match object {
                70 => state(data & 0xFFFF),
                // Direction.fromHorizontal (юг, запад, север, восток) → Direction.getById
                71 => [3, 4, 2, 5][(data & 3) as usize],
                _ => data,
            };
            w.i32(data).bytes(r.rest());
        }
        // VANILLA: PaintingSpawnS2CPacket (1.12.2): название строкой → id картины.
        "add_painting" => {
            w.var_int(r.var_int()?).bytes(r.bytes(16)?);
            let title = r.string(13)?;
            let Some(motive) = PAINTINGS.iter().position(|p| *p == title) else {
                return Ok(Pre18::Drop);
            };
            w.var_int(motive as i32).bytes(r.rest());
        }
        "set_entity_data" => {
            let id = r.var_int()?;
            let Some(kind) = kinds().get(&id).copied() else {
                return Ok(Pre18::Drop);
            };
            w.var_int(id);
            metadata(kind, &mut r, &mut w)?;
        }
        // VANILLA: WorldEventS2CPacket (1.12.2): 2001 — состояние блока, 1010 — id пластинки.
        "level_event" => {
            let event = r.i32()?;
            w.i32(event).bytes(r.bytes(8)?);
            let mut data = r.i32()?;
            if event == 2000 {
                data = smoke_direction(data);
            } else if event == 2001 {
                data = state(data);
            } else if event == 1010 && data != 0 {
                data = u16::try_from(data)
                    .ok()
                    .and_then(|id| ITEMS.binary_search_by_key(&(id, 0), |e| (e.0, e.1)).ok())
                    .map_or(0, |i| i32::from(ITEMS[i].2));
            }
            w.i32(data).bytes(r.rest());
        }
        // VANILLA: BlockActionS2CPacket (1.12.2): числовой id блока → id реестра.
        "block_event" => {
            w.bytes(r.bytes(8 + 2)?);
            let old = r.var_int()?;
            let canon = ezzmc_world::legacy::state_to_canon(state(old & 0xFFF) as u32);
            let block = ezzmc_world::block::BlockState(canon).name();
            let Some(new) = index_of(version.block, block.trim_start_matches("minecraft:")) else {
                return Ok(Pre18::Drop);
            };
            w.var_int(new);
        }
        "level_particles" => {
            if !particles(&mut r, &mut w)? {
                return Ok(Pre18::Drop);
            }
        }
        // VANILLA: PlaySoundNameS2CPacket: имя звука строкой — переименования 1.13.
        "custom_sound" => {
            let old = r.string(256)?;
            let path = old.trim_start_matches("minecraft:");
            let new = SOUND_RENAMES
                .binary_search_by_key(&path, |(a, _)| *a)
                .ok()
                .and_then(|at| SOUND_RENAMES.get(at))
                .map_or(old.as_str(), |(_, b)| *b);
            w.string(new).bytes(r.rest());
        }
        _ => return Ok(Pre18::Keep),
    }
    Ok(Pre18::Replace(w.into_vec()))
}

/// Блок-энтити 1.12.2 из `level_chunk` или `block_entity_data`.
#[derive(Debug, Default)]
pub struct LegacyBlockEntities {
    /// Пришла полная колонка (x, z): прежние записи о ней больше не действуют.
    pub full_column: Option<(i32, i32)>,
    pub entries: Vec<([i32; 3], Tag)>,
}

/// Разбор блок-энтити из сырого пакета 1.12.2; `None` — в этом пакете их не бывает.
/// VANILLA: ChunkDataS2CPacket#read, BlockEntityUpdateS2CPacket#read,
/// PacketByteBuf#readBlockPos, BlockPos#fromLong (1.12.2).
pub fn block_entities(name: &str, body: &[u8]) -> ProtocolResult<Option<LegacyBlockEntities>> {
    let mut r = Reader::new(body);
    let mut out = LegacyBlockEntities::default();
    match name {
        "level_chunk" => {
            let column = (r.i32()?, r.i32()?);
            out.full_column = r.bool()?.then_some(column);
            r.var_int()?;
            let len = r.length("chunk data", 1 << 21)?;
            r.bytes(len)?;
            for _ in 0..r.length("block entities", 1 << 16)? {
                let Some((_, tag)) = nbt::read_named(&mut r)? else {
                    continue;
                };
                let at = |k: &str| tag.get(k).and_then(Tag::as_i64).map(|v| v as i32);
                if let (Some(x), Some(y), Some(z)) = (at("x"), at("y"), at("z")) {
                    out.entries.push(([x, y, z], tag));
                }
            }
        }
        "block_entity_data" => {
            let v = r.i64()?;
            let pos = [
                (v >> 38) as i32,
                ((v >> 26) & 0xFFF) as i32,
                ((v << 38) >> 38) as i32,
            ];
            r.u8()?;
            if let Some((_, tag)) = nbt::read_named(&mut r)? {
                out.entries.push((pos, tag));
            }
        }
        _ => return Ok(None),
    }
    Ok(Some(out))
}

/// `level_chunk` 1.12.2 → 1.13.2: у прямой палитры убирается пустой счётчик, биомы byte → int.
/// VANILLA: ChunkDataS2CPacket#writeData, PaletteContainer#write, RegistryPalette#write (1.12.2).
pub(crate) fn chunk_to_1_13(body: &[u8], sky: bool) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let (x, z) = (r.i32()?, r.i32()?);
    let full = r.bool()?;
    let mask = r.var_int()?;
    let len = r.length("chunk data", 1 << 21)?;
    let mut data = Reader::new(r.bytes(len)?);
    let mut out = Writer::with_capacity(len + BIOMES * 4);
    for _ in 0..(mask & 0xFFFF).count_ones() {
        let bits = data.u8()?;
        out.u8(bits);
        let palette = data.length("chunk palette", 256)?;
        // VANILLA: PaletteContainer#setPaletteSize (1.12.2): палитра при bits <= 8.
        if bits <= 8 {
            out.var_int(palette as i32);
            for _ in 0..palette {
                // VANILLA: PaletteContainer#get (1.9.4) — состояние без id (сервер 1.9 шлёт -1)
                // читается как воздух.
                out.var_int(data.var_int()?.max(0));
            }
        }
        let n = data.length("section longs", SECTION_BLOCKS)?;
        out.var_int(n as i32).bytes(data.bytes(n * 8)?);
        let light = if sky { 2 } else { 1 } * LIGHT_BYTES;
        out.bytes(data.bytes(light)?);
    }
    if full {
        for biome in data.bytes(BIOMES)? {
            out.i32(i32::from(*biome));
        }
    }
    let out = out.into_vec();
    let mut w = Writer::with_capacity(out.len() + r.remaining() + 16);
    w.i32(x).i32(z).bool(full).var_int(mask);
    w.var_int(out.len() as i32).bytes(&out).bytes(r.rest());
    Ok(w.into_vec())
}

fn item_name(id: i32) -> Option<&'static str> {
    let name = ITEM.get(usize::try_from(id).ok()?)?;
    Some(name.trim_start_matches("minecraft:"))
}

fn item_by_name(name: &str) -> Option<i32> {
    ITEM.iter()
        .position(|n| n.trim_start_matches("minecraft:") == name)
        .map(|i| i as i32)
}

fn take(entries: &mut Vec<(String, Tag)>, key: &str) -> Option<Tag> {
    let i = entries.iter().position(|(k, _)| k == key)?;
    Some(entries.remove(i).1)
}

fn rename_display(entries: &mut [(String, Tag)], f: impl Fn(&str) -> String) {
    let name = entries
        .iter_mut()
        .find(|(k, _)| k == "display")
        .and_then(|(_, t)| match t {
            Tag::Compound(d) => d.iter_mut().find(|(k, _)| k == "Name"),
            _ => None,
        });
    if let Some((_, Tag::String(s))) = name {
        *s = f(s);
    }
}

fn map_enchantments(list: &mut Tag, f: impl Fn(&Tag) -> Option<Tag>) {
    if let Tag::List(items) = list {
        for item in items {
            if let Tag::Compound(fields) = item {
                for (k, v) in fields {
                    if k == "id" {
                        if let Some(new) = f(v) {
                            *v = new;
                        }
                    }
                }
            }
        }
    }
}

/// VANILLA: ItemInstanceTheFlatteningFix, ItemStackEnchantmentFix, ItemCustomNameToJson (1.13.2).
fn tag_to_1_13(entries: &mut [(String, Tag)]) {
    rename_display(entries, |s| serde_json::json!({ "text": s }).to_string());
    for (k, v) in entries.iter_mut() {
        if k == "ench" || k == "StoredEnchantments" {
            map_enchantments(v, |id| {
                let id = id.as_i64()?;
                let (_, name) = ENCHANTMENTS.iter().find(|(n, _)| i64::from(*n) == id)?;
                Some(Tag::String(format!("minecraft:{name}")))
            });
        }
        if k == "ench" {
            *k = "Enchantments".into();
        }
    }
}

fn tag_to_1_12(entries: &mut [(String, Tag)]) {
    rename_display(entries, |s| {
        serde_json::from_str::<serde_json::Value>(s)
            .ok()
            .and_then(|v| v.get("text")?.as_str().map(str::to_owned))
            .unwrap_or_else(|| s.to_owned())
    });
    for (k, v) in entries.iter_mut() {
        if k == "Enchantments" || k == "StoredEnchantments" {
            map_enchantments(v, |id| {
                let name = id.as_str()?.trim_start_matches("minecraft:");
                let (n, _) = ENCHANTMENTS.iter().find(|(_, e)| *e == name)?;
                Some(Tag::Short(*n))
            });
        }
        if k == "Enchantments" {
            *k = "ench".into();
        }
    }
}

fn write_tag(w: &mut Writer, entries: Vec<(String, Tag)>) -> ProtocolResult<()> {
    if entries.is_empty() {
        w.u8(0);
        return Ok(());
    }
    let mut out = Vec::new();
    nbt::write_named(&mut out, "", &Tag::Compound(entries))?;
    w.bytes(&out);
    Ok(())
}

fn read_tag(r: &mut Reader) -> ProtocolResult<Vec<(String, Tag)>> {
    Ok(match nbt::read_named(r)? {
        Some((_, Tag::Compound(entries))) => entries,
        _ => Vec::new(),
    })
}

/// Предмет 1.12.2 (short id, byte count, short damage, NBT) → 1.13.2 (bool, varint id, byte, NBT).
/// VANILLA: PacketByteBuf#readItemStack (1.12.2).
pub(crate) fn item_to_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let id = r.i16()?;
    let Ok(id) = u16::try_from(id) else {
        w.bool(false);
        return Ok(());
    };
    let count = r.u8()?;
    let damage = r.i16()?;
    let mut tag = read_tag(r)?;
    let meta = u16::try_from(damage).unwrap_or(0);
    let exact = |d: u16| {
        ITEMS
            .binary_search_by_key(&(id, d), |e| (e.0, e.1))
            .ok()
            .map(|i| i32::from(ITEMS[i].2))
    };
    let new = if id == SPAWN_EGG {
        // Вид яйца — в NBT `EntityTag.id`.
        tag.iter()
            .find(|(k, _)| k == "EntityTag")
            .and_then(|(_, t)| t.get("id")?.as_str())
            .and_then(|e| {
                let e = e.trim_start_matches("minecraft:");
                item_by_name(&format!("{e}_spawn_egg"))
            })
    } else if id == FILLED_MAP {
        tag.push(("map".into(), Tag::Int(i32::from(damage))));
        exact(0)
    } else {
        exact(meta).or_else(|| {
            // Нет варианта с таким damage — это прочность: она переезжает в NBT.
            let base = exact(0)?;
            tag.push(("Damage".into(), Tag::Int(i32::from(damage))));
            Some(base)
        })
    };
    let Some(new) = new else {
        w.bool(false);
        return Ok(());
    };
    tag_to_1_13(&mut tag);
    w.bool(true).var_int(new).u8(count);
    write_tag(w, tag)
}

/// Обратный перевод предмета для serverbound.
pub(crate) fn item_from_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.i16(-1);
        return Ok(());
    }
    let id = r.var_int()?;
    let count = r.u8()?;
    let mut tag = read_tag(r)?;
    let found = ITEMS
        .iter()
        .find(|e| i32::from(e.2) == id)
        .map(|e| (e.0, e.1))
        .or_else(|| {
            item_name(id)?
                .ends_with("_spawn_egg")
                .then_some((SPAWN_EGG, 0))
        });
    let Some((old, mut damage)) = found else {
        w.i16(-1);
        return Ok(());
    };
    let moved = if old == FILLED_MAP {
        take(&mut tag, "map")
    } else if damage == 0 {
        take(&mut tag, "Damage")
    } else {
        None
    };
    if let Some(v) = moved.and_then(|t| t.as_i64()) {
        damage = u16::try_from(v).unwrap_or(0);
    }
    tag_to_1_12(&mut tag);
    w.i16(old as i16).u8(count).i16(damage as i16);
    write_tag(w, tag)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn roundtrip(id: i16, damage: i16, tag: Vec<(String, Tag)>) -> (String, Vec<(String, Tag)>) {
        let mut old = Writer::new();
        old.i16(id).u8(3).i16(damage);
        write_tag(&mut old, tag).unwrap();
        let old = old.into_vec();
        let mut new = Writer::new();
        item_to_1_13_2(&mut Reader::new(&old), &mut new).unwrap();
        let new = new.into_vec();
        let mut back = Writer::new();
        item_from_1_13_2(&mut Reader::new(&new), &mut back).unwrap();
        assert_eq!(back.into_vec(), old);
        let mut r = Reader::new(&new);
        assert!(r.bool().unwrap());
        let name = item_name(r.var_int().unwrap()).unwrap().to_owned();
        r.u8().unwrap();
        (name, read_tag(&mut r).unwrap())
    }

    /// Все имена, в которые переводятся типы и частицы, есть в регистрах 1.13.2.
    #[test]
    fn names_exist_in_1_13_2() {
        use ezzmc_protocol::generated::r1_13_2::{ENTITY_TYPE, PARTICLE_TYPE};
        for (_, name) in ENTITIES {
            assert!(index_of(ENTITY_TYPE, name).is_some(), "{name}");
        }
        for id in 0..49 {
            if let Some(name) = particle_name(id) {
                assert!(index_of(PARTICLE_TYPE, name).is_some(), "{name}");
            }
        }
        assert!(REMAP_1_12.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(SOUND_RENAMES.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn zombie_metadata_name_and_block_state() {
        // имя (строка, #2), состояние в руке эндермена и неизвестный тип поля
        let mut b = Writer::new();
        b.u8(2).var_int(3).string("Bob");
        b.u8(12).var_int(6).u8(1).u8(0xFF);
        let mut out = Writer::new();
        metadata("zombie", &mut Reader::new(&b.buf), &mut out).unwrap();
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        assert_eq!((r.u8().unwrap(), r.var_int().unwrap()), (2, 5));
        assert!(r.bool().unwrap());
        assert_eq!(r.string(100).unwrap(), r#"{"text":"Bob"}"#);
        assert_eq!((r.u8().unwrap(), r.var_int().unwrap()), (12, 7));
        // трава (id 2) с метой 0
        assert_eq!(state(2), 32);
        assert_eq!(state(35 | (14 << 12)), (35 << 4) | 14);
        let bad = [0u8, 20, 0xFF];
        assert!(metadata("zombie", &mut Reader::new(&bad), &mut Writer::new()).is_err());
    }

    #[test]
    fn items() {
        assert_eq!(roundtrip(1, 1, Vec::new()).0, "granite");
        assert_eq!(roundtrip(35, 14, Vec::new()).0, "red_wool");
        let (name, tag) = roundtrip(276, 7, Vec::new());
        assert_eq!(name, "diamond_sword");
        assert_eq!(tag, [("Damage".to_owned(), Tag::Int(7))]);
        assert_eq!(roundtrip(358, 12, Vec::new()).0, "filled_map");
        let egg = vec![(
            "EntityTag".to_owned(),
            Tag::Compound(vec![("id".into(), Tag::String("minecraft:zombie".into()))]),
        )];
        assert_eq!(roundtrip(383, 0, egg).0, "zombie_spawn_egg");
        let ench = vec![(
            "ench".to_owned(),
            Tag::List(vec![Tag::Compound(vec![
                ("id".into(), Tag::Short(16)),
                ("lvl".into(), Tag::Short(5)),
            ])]),
        )];
        assert_eq!(roundtrip(276, 0, ench).1[0].0, "Enchantments");
    }

    #[test]
    fn chunk_direct_palette_and_biomes() {
        let mut data = Writer::new();
        data.u8(13).var_int(0).var_int(832);
        data.bytes(&[0; 832 * 8]);
        data.bytes(&[0; LIGHT_BYTES * 2]).bytes(&[7; BIOMES]);
        let mut body = Writer::new();
        body.i32(1).i32(2).bool(true).var_int(1);
        body.var_int(data.buf.len() as i32)
            .bytes(&data.buf)
            .var_int(0);
        let out = chunk_to_1_13(&body.buf, true).unwrap();
        let mut r = Reader::new(&out);
        r.bytes(4 + 4 + 1 + 1).unwrap();
        let len = r.var_int().unwrap() as usize;
        assert_eq!(len, 1 + 2 + 832 * 8 + LIGHT_BYTES * 2 + BIOMES * 4);
        assert!(chunk_to_1_13(&body.buf[..40], true).is_err());
    }

    #[test]
    fn command_suggestions_roundtrip() {
        let mut w = Writer::new();
        w.var_int(7).string("/give Bot sto");
        let out = suggestion_request_to_1_12(&w.buf).unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.string(32767).unwrap(), "/give Bot sto");
        assert!(!r.bool().unwrap());
        assert!(!r.bool().unwrap());

        let mut w = Writer::new();
        w.var_int(2).string("stone_bricks").string("stone");
        let Pre18::Replace(out) = to_1_13_2("command_suggestions", &w.buf).unwrap() else {
            panic!("not replaced");
        };
        let mut r = Reader::new(&out);
        for expected in [7, 10, 3, 2] {
            assert_eq!(r.var_int().unwrap(), expected);
        }
        assert_eq!(r.string(32767).unwrap(), "stone");
        assert!(!r.bool().unwrap());
        assert_eq!(r.string(32767).unwrap(), "stone_bricks");
    }

    #[test]
    fn map_icons_to_1_13() {
        let mut w = Writer::new();
        w.var_int(3).u8(1).bool(true).var_int(1);
        w.u8(0x42).u8(10).u8(20).u8(0);
        let Pre18::Replace(out) = to_1_13_2("map_item_data", &w.buf).unwrap() else {
            panic!("not replaced");
        };
        assert_eq!(out, [3, 1, 1, 1, 4, 10, 20, 2, 0, 0]);
    }
}
