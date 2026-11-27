//! Отличия протоколов 1.16–1.16.1 (735/736) от 1.16.2 и 1.16.2–1.16.5 (751–754) от 1.17: тела приводятся к формату 1.17/1.17.1, дальше идут
//! общим путём `v1201`. Клики по контейнерам и подтверждения транзакций — свои.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI16, Ordering};
use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, ProtocolResult, State};

use crate::v1201::Pre18;

/// Число секций освещения мира 0..256 (по одной сверху и снизу).
const LIGHT_SECTIONS: u32 = 18;

/// ItemStack 1.16.5 «пусто».
const EMPTY: &[u8] = &[0];

/// (окно, слот) → стек в формате сервера.
type Slots = HashMap<(u8, i16), Vec<u8>>;

/// Предметы слотов: клик в 1.16.5 несёт предмет слота до клика.
static SLOTS: LazyLock<Mutex<Slots>> = LazyLock::new(Default::default);

/// VANILLA: AbstractContainerMenu#backup (1.16.5) — номер транзакции клика.
static ACTION: AtomicI16 = AtomicI16::new(0);

fn slots() -> MutexGuard<'static, Slots> {
    SLOTS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Один ItemStack 1.16.5 (bool, varint id, byte count, NBT) как срез тела.
fn item<'a>(r: &mut Reader<'a>) -> ProtocolResult<&'a [u8]> {
    let at = r.pos();
    if r.bool()? {
        r.var_int()?;
        r.i8()?;
        nbt::read_named(r)?;
    }
    Ok(r.since(at))
}

/// Типы измерений из `login` 1.16.1: `respawn` ссылается на них по имени.
static DIM_TYPES_1_16: LazyLock<Mutex<Vec<(String, Tag)>>> = LazyLock::new(Default::default);

/// Биомы 1.16/1.16.1 `[id, имя, элемент]`: сервер их не присылает (реестр встроен в клиент).
/// VANILLA: Biomes (1.16.1) — id и имена; параметры — отчёт biomes 1.16.2.
const BIOMES_1_16: &str = include_str!("data/biomes_1_16.json");

fn string_tag(s: &str) -> Tag {
    Tag::String(s.to_owned())
}

fn registry(name: &str, values: Vec<(i32, String, Tag)>) -> (String, Tag) {
    let list = values
        .into_iter()
        .map(|(id, n, element)| {
            Tag::Compound(vec![
                ("name".into(), Tag::String(n)),
                ("id".into(), Tag::Int(id)),
                ("element".into(), element),
            ])
        })
        .collect();
    let reg = Tag::Compound(vec![
        ("type".into(), string_tag(name)),
        ("value".into(), Tag::List(list)),
    ]);
    (name.to_owned(), reg)
}

fn biomes_1_16() -> Vec<(i32, String, Tag)> {
    let rows: Vec<(i32, String, serde_json::Value)> =
        serde_json::from_str(BIOMES_1_16).unwrap_or_default();
    rows.into_iter()
        .map(|(id, name, e)| {
            (
                id,
                format!("minecraft:{name}"),
                crate::v1201::json_to_tag(&e),
            )
        })
        .collect()
}

/// VANILLA: RegistryAccess.RegistryHolder#CODEC (1.16.1) — `{dimension: [{name, …поля типа}]}`
/// → кодек 1.16.2 с `dimension_type` и `worldgen/biome`.
fn codec_1_16_1(codec: &Tag) -> (Tag, Vec<(String, Tag)>) {
    let list = match codec.get("dimension") {
        Some(Tag::List(l)) => l.as_slice(),
        _ => &[],
    };
    let dims: Vec<(String, Tag)> = list
        .iter()
        .filter_map(|d| {
            let name = d.get("name")?.as_str()?.to_owned();
            let Tag::Compound(fields) = d else {
                return None;
            };
            let mut el: Vec<_> = fields.iter().filter(|f| f.0 != "name").cloned().collect();
            // 1.16.2 добавил `effects` (по имени типа) вместо жёсткой связи с измерением.
            el.push(("effects".into(), string_tag(&name)));
            // VANILLA: DimensionType#coordinateScale (1.16.2) вместо флага `shrunk` (1.16.1).
            let shrunk = d.get("shrunk").and_then(Tag::as_i64) == Some(1);
            let scale = if shrunk { 8.0 } else { 1.0 };
            el.push(("coordinate_scale".into(), Tag::Double(scale)));
            Some((name, Tag::Compound(el)))
        })
        .collect();
    let values = dims
        .iter()
        .enumerate()
        .map(|(i, (n, t))| (i as i32, n.clone(), t.clone()))
        .collect();
    let out = Tag::Compound(vec![
        registry("minecraft:dimension_type", values),
        registry("minecraft:worldgen/biome", biomes_1_16()),
    ]);
    (out, dims)
}

fn write_dim_type(w: &mut Writer, dims: &[(String, Tag)], name: &str) -> ProtocolResult<()> {
    let element = dims
        .iter()
        .find(|d| d.0 == name)
        .map_or_else(|| Tag::Compound(Vec::new()), |d| d.1.clone());
    write_nbt(w, &element)
}

fn write_nbt(w: &mut Writer, tag: &Tag) -> ProtocolResult<()> {
    let mut out = Vec::new();
    nbt::write_named(&mut out, "", tag)?;
    w.bytes(&out);
    Ok(())
}

/// Тело клиентского пакета 1.16/1.16.1 → формат 1.16.2.
pub fn from_1_16_1(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 32);
    match name {
        // VANILLA: ClientboundLoginPacket (1.16.1): hardcore — бит 8 байта режима, тип
        // измерения — имя, maxPlayers — байт.
        "login" => {
            w.i32(r.i32()?);
            let mode = r.u8()?;
            w.bool(mode & 8 != 0).u8(mode & !8).u8(r.u8()?);
            let n = r.length("levels", 1024)?;
            w.var_int(n as i32);
            for _ in 0..n {
                w.string(&r.string(32767)?);
            }
            let codec = nbt::read_named(&mut r)?.map(|(_, t)| t);
            let (codec, dims) = codec_1_16_1(&codec.unwrap_or(Tag::Compound(Vec::new())));
            write_nbt(&mut w, &codec)?;
            write_dim_type(&mut w, &dims, &r.string(32767)?)?;
            *DIM_TYPES_1_16
                .lock()
                .unwrap_or_else(PoisonError::into_inner) = dims;
            w.string(&r.string(32767)?).bytes(r.bytes(8)?);
            w.var_int(i32::from(r.u8()?)).bytes(r.rest());
        }
        // VANILLA: ClientboundRespawnPacket (1.16.1): тип измерения — имя.
        "respawn" => {
            let dim_type = r.string(32767)?;
            let dims = DIM_TYPES_1_16
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            write_dim_type(&mut w, &dims, &dim_type)?;
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundLevelChunkPacket (1.16.1): + bool forgetOldData; биомы — 1024
        // int без длины (1.16.2: массив varint).
        "level_chunk" => {
            w.i32(r.i32()?).i32(r.i32()?);
            let full = r.bool()?;
            r.bool()?;
            w.bool(full).var_int(r.var_int()?);
            let at = r.pos();
            nbt::read_named(&mut r)?;
            w.bytes(r.since(at));
            if full {
                w.var_int(1024);
                for _ in 0..1024 {
                    w.var_int(r.i32()?);
                }
            }
            w.bytes(r.rest());
        }
        _ => return Ok(Pre18::Keep),
    }
    Ok(Pre18::Replace(w.into_vec()))
}

/// Id внутреннего пакета: изменения одной секции в формате 1.16.2 (`section_blocks_update`) —
/// в таблице 1.16.1 такого пакета нет.
pub const SECTION_UPDATE_ID: i32 = 0x1000;

/// Тело `section_blocks_update` 1.16.2: long SectionPos, bool suppressLight, varint n,
/// varlong (состояние << 12 | x<<8 | z<<4 | y).
fn section_update(cx: i32, sy: i32, cz: i32, entries: &[(u16, i32)]) -> Vec<u8> {
    // VANILLA: SectionPos#asLong (1.16.2)
    let pos = ((i64::from(cx) & 0x3F_FFFF) << 42)
        | ((i64::from(cz) & 0x3F_FFFF) << 20)
        | (i64::from(sy) & 0xF_FFFF);
    let mut w = Writer::with_capacity(16 + entries.len() * 4);
    w.i64(pos).bool(false).var_int(entries.len() as i32);
    for &(at, state) in entries {
        w.var_long((i64::from(state) << 12) | i64::from(at));
    }
    w.into_vec()
}

/// Пакеты 1.16.1 с изменениями блоков → тела по секциям (`None` — пакет не из них).
///
/// VANILLA: ChunkHolder#broadcastChanges (1.16.1) — 2..63 изменений идут пакетом
/// ChunkBlocksUpdate, от 64 — частичным чанком (fullChunk=false).
pub fn expand_1_16_1(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<Vec<u8>>>> {
    match name {
        "chunk_blocks_update" => chunk_blocks_update(body).map(Some),
        // байт 8 — fullChunk
        "level_chunk" if body.get(8) == Some(&0) => partial_chunk(body).map(Some),
        _ => Ok(None),
    }
}

/// VANILLA: ClientboundChunkBlocksUpdatePacket (1.16.1): int x, int z чанка, записи short
/// (x<<12 | z<<8 | y) + varint состояние.
fn chunk_blocks_update(body: &[u8]) -> ProtocolResult<Vec<Vec<u8>>> {
    let mut r = Reader::new(body);
    let (cx, cz) = (r.i32()?, r.i32()?);
    let n = r.length("block updates", 65536)?;
    let mut sections: [Vec<(u16, i32)>; 16] = Default::default();
    for _ in 0..n {
        let off = r.i16()? as u16;
        let state = r.var_int()?;
        let (x, z, y) = (off >> 12, (off >> 8) & 15, off & 255);
        sections[usize::from(y >> 4)].push(((x << 8) | (z << 4) | (y & 15), state));
    }
    Ok(sections
        .iter()
        .enumerate()
        .filter(|(_, e)| !e.is_empty())
        .map(|(sy, e)| section_update(cx, sy as i32, cz, e))
        .collect())
}

/// Распаковка long[] секции: 1.16 — без переноса через границу long, до 1.16 — сплошным потоком.
/// VANILLA: SimpleBitStorage (1.16.1), BitStorage (1.15.2).
pub(crate) fn unpack(longs: &[u64], bits: u32, n: usize, spanning: bool) -> Vec<u64> {
    let at = |i: usize| longs.get(i).copied().unwrap_or(0);
    let per_long = (64 / bits) as usize;
    (0..n)
        .map(|i| {
            let v = if spanning {
                let bit = i * bits as usize;
                let (k, off) = (bit / 64, (bit % 64) as u32);
                let mut v = at(k) >> off;
                if off + bits > 64 {
                    v |= at(k + 1) << (64 - off);
                }
                v
            } else {
                at(i / per_long) >> ((i % per_long) as u32 * bits)
            };
            v & ((1 << bits) - 1)
        })
        .collect()
}

/// Секции частичного чанка → полные обновления секций.
/// VANILLA: LevelChunk#replaceWithPacketData, LevelChunkSection#read, SimpleBitStorage (1.16.1):
/// short, byte bits (минимум 4), палитра при bits < 9, long[] без переноса через границу long;
/// индекс y<<8 | z<<4 | x.
/// VANILLA-UNSURE: блок-энтити частичного чанка не переносятся.
fn partial_chunk(body: &[u8]) -> ProtocolResult<Vec<Vec<u8>>> {
    let mut r = Reader::new(body);
    let (cx, cz) = (r.i32()?, r.i32()?);
    r.bool()?; // fullChunk
    let spanning = crate::v1201::is_1_15();
    if !spanning {
        r.bool()?; // forgetOldData
    }
    let mask = r.var_int()?;
    nbt::read_named(&mut r)?;
    let len = r.length("chunk data", 1 << 21)?;
    let mut data = Reader::new(r.bytes(len)?);
    let mut out = Vec::new();
    for sy in (0..16).filter(|i| mask & (1 << i) != 0) {
        data.bytes(2)?;
        let bits = u32::from(data.u8()?).clamp(4, 31);
        let mut palette = Vec::new();
        if bits < 9 {
            for _ in 0..data.length("chunk palette", 256)? {
                palette.push(data.var_int()?);
            }
        }
        let n = data.length("chunk longs", 4096)?;
        let mut longs = Vec::with_capacity(n);
        for _ in 0..n {
            longs.push(data.i64()? as u64);
        }
        let mut entries = Vec::with_capacity(4096);
        for (i, raw) in (0u16..).zip(unpack(&longs, bits, 4096, spanning)) {
            let state = if bits < 9 {
                palette.get(raw as usize).copied().unwrap_or(0)
            } else {
                raw as i32
            };
            let (x, z, y) = (i & 15, (i >> 4) & 15, i >> 8);
            entries.push(((x << 8) | (z << 4) | y, state));
        }
        out.push(section_update(cx, sy, cz, &entries));
    }
    Ok(out)
}

/// Тело клиентского пакета 1.16.5 → формат 1.17.
pub fn from_1_16_5(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 32);
    match name {
        // VANILLA: ClientboundLevelChunkPacket (1.16.5): x, z, bool fullChunk, varint маска
        // секций → 1.17: x, z, BitSet маска (без fullChunk); остальное совпадает.
        "level_chunk" => {
            w.i32(r.i32()?).i32(r.i32()?);
            // VANILLA-UNSURE: частичные чанки (fullChunk=false) ванильный сервер не шлёт.
            if !r.bool()? {
                return Ok(Pre18::Drop);
            }
            bit_set(&mut w, r.var_int()?);
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundLightUpdatePacket (1.16.5): четыре varint-маски и массивы по битам
        // → 1.17: четыре BitSet и два списка массивов с длинами.
        "light_update" => {
            w.var_int(r.var_int()?)
                .var_int(r.var_int()?)
                .bool(r.bool()?);
            let masks = [r.var_int()?, r.var_int()?, r.var_int()?, r.var_int()?];
            masks.iter().for_each(|m| bit_set(&mut w, *m));
            for mask in &masks[..2] {
                let n = (0..LIGHT_SECTIONS).filter(|i| mask & (1 << i) != 0).count();
                w.var_int(n as i32);
                for _ in 0..n {
                    let len = r.length("light array", 2048)?;
                    w.var_int(len as i32).bytes(r.bytes(len)?);
                }
            }
        }
        "container_set_content" => {
            let win = r.u8()?;
            let n = r.i16()?;
            let mut map = slots();
            map.retain(|k, _| k.0 != win);
            for slot in 0..n.max(0) {
                map.insert((win, slot), item(&mut r)?.to_vec());
            }
            return Ok(Pre18::Keep);
        }
        "container_set_slot" => {
            let (win, slot) = (r.u8()?, r.i16()?);
            let stack = item(&mut r)?.to_vec();
            if slot >= 0 {
                slots().insert((win, slot), stack);
            }
            return Ok(Pre18::Keep);
        }
        // Подтверждение уже отправлено соединением (`ack_reply`).
        "container_ack" => return Ok(Pre18::Drop),
        _ => return Ok(Pre18::Keep),
    }
    Ok(Pre18::Replace(w.into_vec()))
}

/// BitSet из одной int-маски: varint число long, long[].
fn bit_set(w: &mut Writer, mask: i32) {
    if mask == 0 {
        w.var_int(0);
    } else {
        w.var_int(1).i64(i64::from(mask as u32));
    }
}

/// VANILLA: ServerboundContainerClickPacket (1.16.5): byte окно, short слот, byte кнопка, short
/// номер транзакции, enum режим, предмет — результат `AbstractContainerMenu#clicked`
/// (PICKUP/QUICK_MOVE по слоту — предмет до клика, иначе пусто).
/// `changed` — новые стеки затронутых слотов в формате сервера (только для кэша).
pub fn container_click(
    window: u8,
    slot: i16,
    button: u8,
    mode: i32,
    changed: Vec<(i16, Vec<u8>)>,
) -> Vec<u8> {
    let mut map = slots();
    let clicked = match mode {
        0 | 1 if slot >= 0 => map.get(&(window, slot)).cloned(),
        _ => None,
    };
    for (s, stack) in changed {
        map.insert((window, s), stack);
    }
    let uid = ACTION.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    let mut w = Writer::new();
    w.u8(window)
        .i16(slot)
        .u8(button)
        .i16(uid)
        .var_int(mode)
        .bytes(clicked.as_deref().unwrap_or(EMPTY));
    w.into_vec()
}

/// VANILLA: ClientPacketListener#handleContainerAck (1.16.5): на отклонённую транзакцию клиент
/// отвечает тем же окном и номером. Возвращает `(id, тело)` ответа на пакет `id`.
/// VANILLA-UNSURE: ваниль отвечает только для открытого окна и инвентаря — здесь для любого.
pub fn ack_reply(id: i32, body: &[u8]) -> Option<(i32, Vec<u8>)> {
    let table = ezzmc_protocol::version::current().packets;
    let find = |dir, name| {
        table
            .iter()
            .find(|p| p.state == State::Play && p.dir == dir && p.name == name)
            .map(|p| p.id as i32)
    };
    if find(Direction::Clientbound, "container_ack") != Some(id) {
        return None;
    }
    let mut r = Reader::new(body);
    let (window, uid) = (r.u8().ok()?, r.i16().ok()?);
    if r.bool().ok()? {
        return None;
    }
    let mut w = Writer::new();
    w.u8(window).i16(uid).u8(1);
    Some((find(Direction::Serverbound, "container_ack")?, w.into_vec()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn block_updates_split() {
        let mut w = Writer::new();
        // чанк (-1, 2): x=3 z=4 y=70 и x=0 z=0 y=1 — две секции
        w.i32(-1).i32(2).var_int(2);
        w.i16(0x3446).var_int(9).i16(1).var_int(5);
        let out = chunk_blocks_update(&w.into_vec()).unwrap();
        assert_eq!(out.len(), 2);
        let mut r = Reader::new(&out[1]);
        let p = r.i64().unwrap();
        assert_eq!((p >> 42, p << 44 >> 44, p << 22 >> 42), (-1, 4, 2));
        assert!(!r.bool().unwrap());
        assert_eq!(r.var_int().unwrap(), 1);
        assert_eq!(r.var_long().unwrap(), (9 << 12) | (3 << 8) | (4 << 4) | 6);
    }

    #[test]
    fn partial_chunk_to_section() {
        let mut data = Writer::new();
        data.i16(1)
            .u8(4)
            .var_int(2)
            .var_int(0)
            .var_int(7)
            .var_int(256);
        // блок x=1 y=0 z=0 — индекс палитры 1
        data.i64(0x10);
        for _ in 1..256 {
            data.i64(0);
        }
        let data = data.into_vec();
        let mut w = Writer::new();
        w.i32(3).i32(4).bool(false).bool(false).var_int(1 << 2);
        w.bytes(&[10, 0, 0, 0])
            .var_int(data.len() as i32)
            .bytes(&data);
        let out = expand_1_16_1("level_chunk", &w.into_vec())
            .unwrap()
            .unwrap();
        assert_eq!(out.len(), 1);
        let mut r = Reader::new(&out[0]);
        r.bytes(9).unwrap();
        assert_eq!(r.var_int().unwrap(), 4096);
        assert_eq!(r.var_long().unwrap(), 0);
        assert_eq!(r.var_long().unwrap(), (7 << 12) | (1 << 8));
    }

    #[test]
    fn biomes_table() {
        let b = biomes_1_16();
        assert_eq!(b.len(), 79);
        assert!(b
            .iter()
            .any(|e| e.0 == 173 && e.1 == "minecraft:basalt_deltas"));
    }
}
