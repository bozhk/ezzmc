//! Отличия протоколов 1.12.2–1.13.2 (335–404) от 1.14.4: тела приводятся к формату 1.14.4, дальше идут
//! общим путём `v1152`/`v1165`/`v1201`. Пакеты, которых общий путь не разбирает, не переводятся.

use std::sync::atomic::{AtomicBool, Ordering};

use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;

use crate::legacy::Packet;
use crate::v1201::Pre18;

/// Внутренний id `level_chunk` 1.13 после выделения света в отдельный пакет.
pub const CHUNK_ID: i32 = 0x1004;
/// Внутренний id синтезированного `light_update` (формат 1.14).
pub const LIGHT_ID: i32 = 0x1005;
/// Имя пакета [`CHUNK_ID`] на входе [`expand`] (чтобы не разворачивать повторно).
const CHUNK_NAME: &str = "level_chunk_113";

/// Дальность в `login`: 1.13 её не присылает, берётся максимум, чтобы не терять чанки сервера.
/// VANILLA-UNSURE: ваниль 1.13 держит чанки по своей настройке дальности.
const VIEW_DISTANCE: i32 = 32;

const SECTION_BLOCKS: usize = 4096;
const LIGHT_BYTES: usize = 2048;
const BIOME_BYTES: usize = 256 * 4;

/// Есть ли небо в текущем измерении (в чанке 1.13 свет неба идёт только в верхнем мире).
static SKY_LIGHT: AtomicBool = AtomicBool::new(true);

fn current() -> i32 {
    ezzmc_protocol::version::current().protocol
}

/// Имя пакета после разворота.
pub fn after_expand(name: &'static str) -> &'static str {
    if name == CHUNK_NAME {
        "level_chunk"
    } else {
        name
    }
}

/// Имя пакета с внутренним id.
pub fn internal_name(id: i32) -> Option<&'static str> {
    match id {
        CHUNK_ID => Some(CHUNK_NAME),
        LIGHT_ID => Some("light_update"),
        _ => None,
    }
}

/// `x<<38 | y<<26 | z` (1.13) → `x<<38 | z<<12 | y` (1.14).
pub(crate) fn pos_to_1_14(v: i64) -> i64 {
    let v = v as u64;
    let (x, y, z) = (v >> 38, (v >> 26) & 0xFFF, v & 0x3FF_FFFF);
    ((x << 38) | (z << 12) | y) as i64
}

fn pos_to_1_13(v: i64) -> i64 {
    let v = v as u64;
    let (x, z, y) = (v >> 38, (v >> 12) & 0x3FF_FFFF, v & 0xFFF);
    ((x << 38) | (y << 26) | z) as i64
}

/// Id предмета сервера → id таблицы 1.13.2 (у 393 свои id, дальше всё считается по 1.13.2).
fn item_id_in(id: i32) -> Option<i32> {
    if current() != 393 {
        return Some(id);
    }
    static T: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    let map = T.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::version::V1_13.item,
            ezzmc_protocol::version::V1_13_2.item,
        )
    });
    map.get(usize::try_from(id).ok()?)
        .copied()
        .flatten()
        .map(|c| c as i32)
}

fn item_id_out(id: i32) -> Option<i32> {
    if current() != 393 {
        return Some(id);
    }
    static T: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    let map = T.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::version::V1_13_2.item,
            ezzmc_protocol::version::V1_13.item,
        )
    });
    map.get(usize::try_from(id).ok()?)
        .copied()
        .flatten()
        .map(|c| c as i32)
}

/// Предмет 1.13/1.13.1 (`short id`, -1 — пусто, byte count, NBT) → формат 1.13.2.
/// VANILLA: PacketByteBuf#readItemStack (1.13.2: bool, varint id).
pub(crate) fn item_to_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if current() <= 340 {
        return crate::v1122::item_to_1_13_2(r, w);
    }
    let id = r.i16()?;
    if id < 0 {
        w.bool(false);
        return Ok(());
    }
    let count = r.u8()?;
    let at = r.pos();
    ezzmc_protocol::nbt::read_named(r)?;
    let tag = r.since(at);
    match item_id_in(i32::from(id)) {
        Some(id) => {
            w.bool(true).var_int(id).u8(count).bytes(tag);
        }
        None => {
            w.bool(false);
        }
    }
    Ok(())
}

/// VANILLA: AdvancementDisplay#writeTo (1.12.2/1.13): переписывает только иконки.
fn advancement_icons(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    w.bool(r.bool()?);
    let n = r.length("advancements", 1 << 16)?;
    w.var_int(n as i32);
    for _ in 0..n {
        w.string(&r.string(32767)?);
        let parent = r.bool()?;
        w.bool(parent);
        if parent {
            w.string(&r.string(32767)?);
        }
        let display = r.bool()?;
        w.bool(display);
        if display {
            w.string(&r.string(262_144)?).string(&r.string(262_144)?);
            item_to_1_13_2(r, w)?;
            w.var_int(r.var_int()?);
            let flags = r.i32()?;
            w.i32(flags);
            if flags & 1 != 0 {
                w.string(&r.string(32767)?);
            }
            w.bytes(r.bytes(8)?);
        }
        // критерии и требования
        let at = r.pos();
        for _ in 0..r.length("criteria", 1 << 16)? {
            r.string(32767)?;
        }
        for _ in 0..r.length("requirements", 1 << 16)? {
            for _ in 0..r.length("requirement", 1 << 16)? {
                r.string(32767)?;
            }
        }
        w.bytes(r.since(at));
    }
    w.bytes(r.rest());
    Ok(())
}

/// Обратный перевод для serverbound.
fn item_from_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if current() <= 340 {
        return crate::v1122::item_from_1_13_2(r, w);
    }
    if !r.bool()? {
        w.i16(-1);
        return Ok(());
    }
    let id = r.var_int()?;
    let count = r.u8()?;
    let tag = r.rest();
    match item_id_out(id).and_then(|id| i16::try_from(id).ok()) {
        Some(id) => {
            w.i16(id).u8(count).bytes(tag);
        }
        None => {
            w.i16(-1);
        }
    }
    Ok(())
}

/// Вид объекта в `add_entity` 1.13 → имя типа сущности (у вагонеток вид уточняет `data`).
/// VANILLA: ClientPlayNetworkHandler#onEntitySpawn (1.13.2).
pub(crate) fn object_name(object: u8, data: i32) -> Option<&'static str> {
    Some(match object {
        1 => "boat",
        2 => "item",
        3 => "area_effect_cloud",
        10 => match data {
            1 => "chest_minecart",
            2 => "furnace_minecart",
            3 => "tnt_minecart",
            4 => "spawner_minecart",
            5 => "hopper_minecart",
            6 => "command_block_minecart",
            _ => "minecart",
        },
        50 => "tnt",
        51 => "end_crystal",
        60 => "arrow",
        61 => "snowball",
        62 => "egg",
        63 => "fireball",
        64 => "small_fireball",
        65 => "ender_pearl",
        66 => "wither_skull",
        67 => "shulker_bullet",
        68 => "llama_spit",
        70 => "falling_block",
        71 => "item_frame",
        72 => "eye_of_ender",
        73 => "potion",
        75 => "experience_bottle",
        76 => "firework_rocket",
        77 => "leash_knot",
        78 => "armor_stand",
        79 => "evoker_fangs",
        90 => "fishing_bobber",
        91 => "spectral_arrow",
        93 => "dragon_fireball",
        94 => "trident",
        _ => return None,
    })
}

/// Тип окна 1.13 (строка) → имя меню 1.14.
/// VANILLA: ClientPlayNetworkHandler#onOpenScreen (1.13.2).
fn menu_name(kind: &str, slots: u8) -> Option<&'static str> {
    const ROWS: [&str; 6] = [
        "generic_9x1",
        "generic_9x2",
        "generic_9x3",
        "generic_9x4",
        "generic_9x5",
        "generic_9x6",
    ];
    Some(match kind.trim_start_matches("minecraft:") {
        "chest" | "container" => ROWS.get(usize::from(slots / 9).checked_sub(1)?).copied()?,
        "crafting_table" => "crafting",
        "furnace" => "furnace",
        "dispenser" | "dropper" => "generic_3x3",
        "enchanting_table" => "enchantment",
        "brewing_stand" => "brewing_stand",
        "villager" => "merchant",
        "beacon" => "beacon",
        "anvil" => "anvil",
        "hopper" => "hopper",
        "shulker_box" => "shulker_box",
        _ => return None,
    })
}

struct Section<'a> {
    /// bits, палитра и long[] — формат совпадает с 1.14.
    blocks: &'a [u8],
    non_air: i16,
    block_light: &'a [u8],
    sky_light: Option<&'a [u8]>,
}

struct Chunk<'a> {
    x: i32,
    z: i32,
    full: bool,
    mask: i32,
    sections: Vec<Section<'a>>,
    biomes: &'a [u8],
    block_entities: &'a [u8],
}

fn count_non_air(palette: Option<&[i32]>, longs: &[u64], bits: u32) -> i16 {
    use ezzmc_world::block::BlockState;
    let is_air =
        |state: i32| BlockState(ezzmc_world::legacy::state_to_canon(state.max(0) as u32)).is_air();
    let air: Option<Vec<bool>> = palette.map(|p| p.iter().map(|s| is_air(*s)).collect());
    let mut cache = std::collections::HashMap::new();
    let n = crate::v1165::unpack(longs, bits, SECTION_BLOCKS, true)
        .into_iter()
        .filter(|v| match &air {
            Some(a) => !a.get(*v as usize).copied().unwrap_or(false),
            None => !*cache.entry(*v).or_insert_with(|| is_air(*v as i32)),
        })
        .count();
    n as i16
}

/// VANILLA: ChunkDataS2CPacket#writeData (1.13.2): на секцию контейнер палитры, свет блоков,
/// свет неба (только верхний мир); в конце полного чанка — 256 int биомов.
fn parse_chunk(body: &[u8], sky: bool) -> ProtocolResult<Chunk<'_>> {
    let mut r = Reader::new(body);
    let (x, z) = (r.i32()?, r.i32()?);
    let full = r.bool()?;
    let mask = r.var_int()?;
    let len = r.length("chunk data", 1 << 21)?;
    let mut data = Reader::new(r.bytes(len)?);
    let count = (mask & 0xFFFF).count_ones();
    let mut sections = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let start = data.pos();
        let raw = data.u8()?;
        let bits = u32::from(raw).clamp(4, 31);
        // VANILLA: PaletteContainer (1.13.2): палитра при bits <= 8, дальше глобальные id.
        let palette = if bits <= 8 {
            let n = data.length("chunk palette", 256)?;
            let mut p = Vec::with_capacity(n);
            for _ in 0..n {
                p.push(data.var_int()?);
            }
            Some(p)
        } else {
            None
        };
        let n = data.length("section longs", SECTION_BLOCKS)?;
        let mut longs = Vec::with_capacity(n);
        for _ in 0..n {
            longs.push(data.i64()? as u64);
        }
        let blocks = data.since(start);
        let non_air = count_non_air(palette.as_deref(), &longs, bits);
        let block_light = data.bytes(LIGHT_BYTES)?;
        let sky_light = if sky {
            Some(data.bytes(LIGHT_BYTES)?)
        } else {
            None
        };
        sections.push(Section {
            blocks,
            non_air,
            block_light,
            sky_light,
        });
    }
    let biomes = if full { data.bytes(BIOME_BYTES)? } else { &[] };
    Ok(Chunk {
        x,
        z,
        full,
        mask,
        sections,
        biomes,
        block_entities: r.rest(),
    })
}

/// `level_chunk` 1.13 → 1.14.4: счётчик не-воздуха в секциях, без света, пустые карты высот.
/// VANILLA-UNSURE: 1.13 карты высот не присылает (считает сам) — отдаются пустыми.
fn chunk_to_1_14(c: &Chunk) -> ProtocolResult<Vec<u8>> {
    let mut data = Writer::with_capacity(c.sections.len() * 8192 + BIOME_BYTES);
    for s in &c.sections {
        data.i16(s.non_air).bytes(s.blocks);
    }
    data.bytes(c.biomes);
    let data = data.into_vec();
    let mut w = Writer::with_capacity(data.len() + c.block_entities.len() + 32);
    w.i32(c.x).i32(c.z).bool(c.full).var_int(c.mask);
    let mut maps = Vec::new();
    ezzmc_protocol::nbt::write_named(&mut maps, "", &ezzmc_protocol::nbt::Tag::Compound(vec![]))?;
    w.bytes(&maps)
        .var_int(data.len() as i32)
        .bytes(&data)
        .bytes(c.block_entities);
    Ok(w.into_vec())
}

/// Свет чанка 1.13 → `light_update` 1.14 (бит 0 маски — секция под миром).
/// VANILLA: Chunk#getLightLevel (1.13.2): в отсутствующих секциях над рельефом свет неба 15.
/// VANILLA-UNSURE: отсутствующие секции под верхней присланной считаются тёмными.
fn light_to_1_14(c: &Chunk, sky: bool) -> Vec<u8> {
    const FULL: [u8; LIGHT_BYTES] = [0xFF; LIGHT_BYTES];
    let (mut sky_mask, mut block_mask) = (0i32, 0i32);
    let (mut empty_sky, mut empty_block) = (0i32, 0i32);
    let mut sky_arrays: Vec<&[u8]> = Vec::new();
    let mut block_arrays: Vec<&[u8]> = Vec::new();
    let mut present = c.sections.iter();
    let top = 32 - (c.mask & 0xFFFF).leading_zeros(); // число секций до верхней присланной
    for y in 0..17u32 {
        let bit = 1i32 << (y + 1);
        let section = if y < 16 && c.mask & (1 << y) != 0 {
            present.next()
        } else {
            None
        };
        match section {
            Some(s) => {
                block_mask |= bit;
                block_arrays.push(s.block_light);
                if let Some(l) = s.sky_light {
                    sky_mask |= bit;
                    sky_arrays.push(l);
                }
            }
            // частичное обновление не трогает остальные секции
            None if !c.full => {}
            None => {
                empty_block |= bit;
                if sky && y >= top {
                    sky_mask |= bit;
                    sky_arrays.push(&FULL);
                } else if sky {
                    empty_sky |= bit;
                }
            }
        }
    }
    let mut w = Writer::with_capacity((sky_arrays.len() + block_arrays.len()) * 2050 + 32);
    w.var_int(c.x)
        .var_int(c.z)
        .var_int(sky_mask)
        .var_int(block_mask)
        .var_int(empty_sky)
        .var_int(empty_block);
    for a in sky_arrays.into_iter().chain(block_arrays) {
        w.var_int(LIGHT_BYTES as i32).bytes(a);
    }
    w.into_vec()
}

/// `level_chunk` → тот же пакет под внутренним id и синтезированный `light_update`.
pub fn expand(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<Packet>>> {
    let legacy = current() <= 340;
    if legacy && crate::v1122::is_untranslated(name) {
        return Ok(Some(Vec::new()));
    }
    if name != "level_chunk" {
        return Ok(None);
    }
    let sky = SKY_LIGHT.load(Ordering::Relaxed);
    let converted;
    let body = if legacy {
        converted = crate::v1122::chunk_to_1_13(body, sky)?;
        converted.as_slice()
    } else {
        body
    };
    let chunk = parse_chunk(body, sky)?;
    // VANILLA: ChunkHolder (1.13.2) — от 64 изменений идёт частичный чанк (fullChunk=false):
    // меняются только присланные секции.
    if !chunk.full {
        let bodies = crate::v1165::expand_1_16_1("level_chunk", &chunk_to_1_14(&chunk)?)?;
        let id = crate::v1165::SECTION_UPDATE_ID;
        return Ok(bodies.map(|b| b.into_iter().map(|b| (id, b)).collect()));
    }
    let light = light_to_1_14(&chunk, sky);
    Ok(Some(vec![(CHUNK_ID, body.to_vec()), (LIGHT_ID, light)]))
}

/// Тело клиентского пакета 1.13.x → формат 1.14.4.
pub fn from_1_13(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    if current() > 340 {
        return from_1_13_2(name, body);
    }
    match crate::v1122::to_1_13_2(name, body)? {
        Pre18::Keep => from_1_13_2(name, body),
        Pre18::Replace(new) => Ok(match from_1_13_2(name, &new)? {
            Pre18::Keep => Pre18::Replace(new),
            other => other,
        }),
        Pre18::Drop => Ok(Pre18::Drop),
    }
}

fn from_1_13_2(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 16);
    match name {
        // VANILLA: GameJoinS2CPacket (1.13.2): int id, byte режим, int измерение, byte сложность,
        // byte maxPlayers, строка типа уровня, bool.
        "login" => {
            w.bytes(r.bytes(5)?);
            let dim = r.i32()?;
            SKY_LIGHT.store(dim == 0, Ordering::Relaxed);
            w.i32(dim);
            r.u8()?; // сложность: в 1.14 отдельным change_difficulty
            w.u8(r.u8()?).string(&r.string(16)?).var_int(VIEW_DISTANCE);
            w.bytes(r.rest());
        }
        // VANILLA: PlayerRespawnS2CPacket (1.13.2): int измерение, byte сложность, byte режим, строка.
        "respawn" => {
            let dim = r.i32()?;
            SKY_LIGHT.store(dim == 0, Ordering::Relaxed);
            r.u8()?;
            w.i32(dim).bytes(r.rest());
        }
        "level_chunk" => {
            let chunk = parse_chunk(body, SKY_LIGHT.load(Ordering::Relaxed))?;
            return Ok(Pre18::Replace(chunk_to_1_14(&chunk)?));
        }
        "block_update" | "block_event" => {
            w.i64(pos_to_1_14(r.i64()?)).bytes(r.rest());
        }
        "level_event" => {
            w.i32(r.i32()?).i64(pos_to_1_14(r.i64()?)).bytes(r.rest());
        }
        "block_destruction" => {
            w.var_int(r.var_int()?)
                .i64(pos_to_1_14(r.i64()?))
                .bytes(r.rest());
        }
        // varint id, uuid, varint motive, позиция, byte направление
        "add_painting" => {
            w.var_int(r.var_int()?)
                .bytes(r.bytes(16)?)
                .var_int(r.var_int()?)
                .i64(pos_to_1_14(r.i64()?))
                .bytes(r.rest());
        }
        // VANILLA: OpenScreenS2CPacket (1.13.2): byte окно, строка типа, Text, byte слоты
        // [, int сущность для EntityHorse].
        "open_screen" => {
            let win = r.u8()?;
            let kind = r.string(32)?;
            let title = r.string(262144)?;
            let slots = r.u8()?;
            // VANILLA-UNSURE: окно лошади (в 1.14 — horse_screen_open) пока не переводится.
            let menu = menu_name(&kind, slots).and_then(|n| {
                ezzmc_protocol::generated::r1_14_4::MENU
                    .iter()
                    .position(|m| *m == n)
            });
            let Some(menu) = menu else {
                tracing::warn!("1.13 open_screen: тип окна {kind} не переведён");
                return Ok(Pre18::Drop);
            };
            w.var_int(i32::from(win))
                .var_int(menu as i32)
                .string(&title);
        }
        "container_set_content" if current() < 404 => {
            w.u8(r.u8()?);
            let n = r.i16()?;
            w.i16(n);
            for _ in 0..n.clamp(0, 256) {
                item_to_1_13_2(&mut r, &mut w)?;
            }
        }
        "container_set_slot" if current() < 404 => {
            w.u8(r.u8()?).i16(r.i16()?);
            item_to_1_13_2(&mut r, &mut w)?;
        }
        // Иконки в достижениях ≤1.13.1 — в старом формате предмета.
        "update_advancements" if current() < 404 => advancement_icons(&mut r, &mut w)?,
        "set_equipment" if current() < 404 => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            item_to_1_13_2(&mut r, &mut w)?;
        }
        // VANILLA: EntitySpawnS2CPacket (1.13.2): id, uuid, byte вид объекта, xyz, pitch, yaw,
        // int data, скорость → 1.14: varint тип сущности.
        "add_entity" => {
            w.var_int(r.var_int()?).bytes(r.bytes(16)?);
            let object = r.u8()?;
            let pose = r.bytes(24 + 2)?;
            let data = r.i32()?;
            let kind = object_name(object, data).and_then(|name| {
                ezzmc_protocol::version::current()
                    .entity_type
                    .iter()
                    .position(|n| n.trim_start_matches("minecraft:") == name)
            });
            let Some(kind) = kind else {
                return Ok(Pre18::Drop);
            };
            w.var_int(kind as i32).bytes(pose).i32(data).bytes(r.rest());
        }
        _ => return Ok(Pre18::Keep),
    }
    Ok(Pre18::Replace(w.into_vec()))
}

/// Тело serverbound пакета формата 1.14.4 → 1.13.x (на месте).
pub(crate) fn to_1_13(name: &str, body: &mut Vec<u8>) -> ProtocolResult<()> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len());
    match name {
        "command_suggestion" if current() <= 340 => {
            *body = crate::v1122::suggestion_request_to_1_12(body)?;
            return Ok(());
        }
        // VANILLA: PlayerInteractBlockC2SPacket (1.13.2): позиция, сторона, рука, 3 float.
        "use_item_on" => {
            let hand = r.var_int()?;
            let pos = r.i64()?;
            let face = r.var_int()?;
            w.i64(pos_to_1_13(pos)).var_int(face).var_int(hand);
            if current() < 315 {
                // VANILLA: PlayerInteractBlockC2SPacket (1.10.2): точка на грани — байты (×16).
                for _ in 0..3 {
                    w.u8((r.f32()? * 16.0) as u8);
                }
            } else {
                w.bytes(r.bytes(12)?);
            }
        }
        // VANILLA: ResourcePackStatusC2SPacket (1.9.4): хеш из запроса, затем статус.
        "resource_pack" if current() < 210 => {
            w.string(&crate::v194::resource_pack_hash()).bytes(r.rest());
        }
        "player_action" => {
            w.var_int(r.var_int()?)
                .i64(pos_to_1_13(r.i64()?))
                .bytes(r.rest());
        }
        "sign_update" => {
            w.i64(pos_to_1_13(r.i64()?)).bytes(r.rest());
        }
        // window, slot, button, номер действия, режим, предмет
        "container_click" if current() < 404 => {
            w.bytes(r.bytes(6)?).var_int(r.var_int()?);
            item_from_1_13_2(&mut r, &mut w)?;
        }
        _ => return Ok(()),
    }
    *body = w.into_vec();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_ids() {
        assert_eq!(object_name(10, 0), Some("minecart"));
        assert_eq!(object_name(10, 5), Some("hopper_minecart"));
        assert_eq!(object_name(78, 0), Some("armor_stand"));
        assert_eq!(object_name(200, 0), None);
    }

    #[test]
    fn position_roundtrip() {
        let (x, y, z) = (-1001i64, 119i64, 2002i64);
        let old = (((x & 0x3FF_FFFF) << 38) | (y << 26) | (z & 0x3FF_FFFF)) as u64 as i64;
        let new = (((x & 0x3FF_FFFF) << 38) | ((z & 0x3FF_FFFF) << 12) | y) as u64 as i64;
        assert_eq!(pos_to_1_14(old), new);
        assert_eq!(pos_to_1_13(new), old);
    }

    #[test]
    fn chunk_and_light() {
        // одна секция y=0, палитра [0, 1], два непустых блока
        let mut data = Writer::new();
        data.u8(4).var_int(2).var_int(0).var_int(1).var_int(256);
        data.i64(0x11);
        for _ in 1..256 {
            data.i64(0);
        }
        data.bytes(&[1; LIGHT_BYTES]).bytes(&[2; LIGHT_BYTES]);
        data.bytes(&[0; BIOME_BYTES]);
        let mut body = Writer::new();
        body.i32(3).i32(-4).bool(true).var_int(1);
        body.var_int(data.buf.len() as i32)
            .bytes(&data.buf)
            .var_int(0);

        let chunk = parse_chunk(&body.buf, true).unwrap();
        assert_eq!(chunk.sections[0].non_air, 2);
        let light = light_to_1_14(&chunk, true);
        let mut r = Reader::new(&light);
        assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (3, -4));
        // свет неба: секция 0 и все выше (биты 1..=17), свет блоков: только бит 1
        assert_eq!(r.var_int().unwrap(), 0x3_FFFE);
        assert_eq!(r.var_int().unwrap(), 0b10);
        assert_eq!(r.var_int().unwrap(), 0);
        assert_eq!(r.var_int().unwrap(), 0x3_FFFC);
        assert_eq!(r.remaining(), 18 * 2050);
        assert!(chunk_to_1_14(&chunk).is_ok());
    }

    #[test]
    fn truncated_chunk_is_error() {
        let mut body = Writer::new();
        body.i32(0).i32(0).bool(true).var_int(1).var_int(1).u8(4);
        assert!(parse_chunk(&body.buf, true).is_err());
    }

    #[test]
    fn windows() {
        assert_eq!(menu_name("minecraft:chest", 54), Some("generic_9x6"));
        assert_eq!(menu_name("minecraft:chest", 0), None);
        assert_eq!(menu_name("EntityHorse", 2), None);
    }
}
