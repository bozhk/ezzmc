//! 1.7.6–1.7.10 (протокол 5) ↔ формат 1.8 (47). Дальше пакеты идут путём 1.8 (`v18`).
//!
//! Порядок пакетов 1.7 совпадает с первыми пакетами 1.8 (clientbound 0x00–0x40, serverbound
//! 0x00–0x17), поэтому используется таблица 1.8 (`generated/v1_8.rs`).
//!
//! Отличия тел (VANILLA: read/write пакетов 1.7.10 и 1.8.9, Yarn):
//! - id сущностей во многих пакетах — int (в 1.8 varint), координаты блоков — int/byte/short
//!   по отдельности (в 1.8 упакованный long);
//! - предмет: NBT сжат gzip, перед ним short-длина (-1 — нет);
//! - чанк: zlib, блоки — byte id + nibble meta + nibble add (в 1.8 char `id<<4|meta`);
//! - метаданные: имя моба в индексах 10/11 MobEntity (в 1.8 — 2/3 Entity), возраст — int;
//! - список игроков — имя/онлайн/пинг (в 1.8 — действия с UUID);
//! - позиция игрока — y глаз (в 1.8 — ноги), в пакетах движения игрока ещё и stance.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, PoisonError};

use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};

use crate::v18::Kind;

/// VANILLA: ServerPlayNetworkHandler#requestTeleport (1.7.10): y + 1.62F.
const EYE: f64 = 1.62f32 as f64;

#[derive(Default)]
struct State {
    /// id своей сущности (из `login`): нужен для HandSwingC2SPacket 1.7.
    me: i32,
    /// Ники в списке игроков (1.7 шлёт «онлайн» повторно для обновления пинга).
    tab: HashSet<String>,
    /// Масштаб и метки карт: в 1.8 их несёт каждый MapUpdateS2CPacket.
    maps: HashMap<i32, (u8, Vec<[u8; 3]>)>,
}

fn state() -> std::sync::MutexGuard<'static, State> {
    static S: OnceLock<Mutex<State>> = OnceLock::new();
    S.get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// id своей сущности (из `login`).
pub(crate) fn me() -> i32 {
    state().me
}

pub(crate) fn is_1_7() -> bool {
    ezzmc_protocol::version::current().protocol < 47
}

fn err(s: &str) -> ProtocolError {
    ProtocolError::Other(format!("1.7: {s}"))
}

/// VANILLA: BlockPos#asLong (1.8.9): x 26 бит, y 12 бит, z 26 бит.
fn pos(w: &mut Writer, x: i32, y: i32, z: i32) {
    w.i64(
        ((i64::from(x) & 0x3FF_FFFF) << 38)
            | ((i64::from(y) & 0xFFF) << 26)
            | (i64::from(z) & 0x3FF_FFFF),
    );
}

fn unpos(v: i64) -> (i32, i32, i32) {
    (
        (v >> 38) as i32,
        (v << 26 >> 52) as i32,
        (v << 38 >> 38) as i32,
    )
}

fn gunzip(data: &[u8]) -> ProtocolResult<Vec<u8>> {
    // gzip: последние 4 байта — размер распакованных данных (LE).
    let size = match data.len().checked_sub(4) {
        Some(at) => u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]]),
        None => return Err(err("короткий gzip")),
    } as usize;
    if size > 0x20_0000 {
        return Err(ProtocolError::TooLarge {
            what: "NBT 1.7",
            len: size,
            limit: 0x20_0000,
        });
    }
    let mut out = vec![0; size];
    let n = libdeflater::Decompressor::new()
        .gzip_decompress(data, &mut out)
        .map_err(|e| err(&format!("gzip: {e}")))?;
    out.truncate(n);
    Ok(out)
}

fn gzip(data: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut c = libdeflater::Compressor::new(libdeflater::CompressionLvl::default());
    let mut out = vec![0; c.gzip_compress_bound(data.len())];
    let n = c
        .gzip_compress(data, &mut out)
        .map_err(|e| err(&format!("gzip: {e}")))?;
    out.truncate(n);
    Ok(out)
}

/// VANILLA: PacketByteBuf#readNbtCompound (1.7.10): short длина, gzip; -1 — нет.
/// 1.8: NbtIo#write без сжатия, байт 0 — нет.
fn nbt(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let len = r.i16()?;
    if len < 0 {
        w.u8(0);
    } else {
        w.bytes(&gunzip(r.bytes(len as usize)?)?);
    }
    Ok(())
}

/// VANILLA: PacketByteBuf#readItemStack (1.7.10): short id (-1 — пусто), byte, short, NBT.
fn item(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let id = r.i16()?;
    w.i16(id);
    if id >= 0 {
        w.bytes(r.bytes(3)?);
        nbt(r, w)?;
    }
    Ok(())
}

// ---------- метаданные ----------

/// Типы предметов-объектов, у которых индексы полей сместились (ItemFrame).
const ITEM_FRAME: u8 = 71;
/// Мобы с возрастом (PassiveEntity): индекс 12 — int в 1.7, byte в 1.8.
fn ageable(ty: u8) -> bool {
    matches!(ty, 90..=93 | 95 | 96 | 98 | 100 | 101 | 120)
}

/// VANILLA: DataTracker#deserializePacket (1.7.10 и 1.8.9: формат одинаковый, `тип<<5 | индекс`,
/// конец 0x7F). Поменялись индексы: имя моба (MobEntity 10/11 → Entity 2/3), возраст
/// (PassiveEntity 12: int → byte), блок эндермена (16: byte → short), рамка (2/3 → 8/9).
/// Смещение от блока опоры к блоку висящего предмета (0 юг, 1 запад, 2 север, 3 восток).
/// VANILLA: AbstractDecorationEntity#method_4460 (1.7.10) — 1.7 шлёт блок опоры, 1.8 — свой.
fn hanging_step(dir: i32) -> (i32, i32) {
    match dir & 3 {
        0 => (0, 1),
        1 => (-1, 0),
        2 => (0, -1),
        _ => (1, 0),
    }
}

fn metadata(r: &mut Reader, w: &mut Writer, kind: Option<Kind>) -> ProtocolResult<()> {
    loop {
        let head = r.u8()?;
        if head == 0x7F {
            break;
        }
        let (ty, idx) = (head >> 5, head & 0x1F);
        let (mut nty, mut nidx) = (ty, idx);
        match (kind, idx) {
            (Some(Kind::Mob(_)), 10) => nidx = 2,
            (Some(Kind::Mob(_)), 11) => nidx = 3,
            (Some(Kind::Mob(m)), 12) if ageable(m) && ty == 2 => nty = 0,
            (Some(Kind::Mob(58)), 16) if ty == 0 => nty = 1,
            (Some(Kind::Object(ITEM_FRAME, _)), 2 | 3) => nidx = idx + 6,
            _ => {}
        }
        w.u8(nty << 5 | nidx);
        match (ty, nty) {
            (0, 0) => {
                w.u8(r.u8()?);
            }
            (0, 1) => {
                w.i16(i16::from(r.u8()?));
            }
            (1, _) => {
                w.i16(r.i16()?);
            }
            (2, 0) => {
                w.u8(r.i32()?.clamp(-1, 1) as u8);
            }
            (2, _) => {
                w.i32(r.i32()?);
            }
            (3, _) => {
                w.f32(r.f32()?);
            }
            (4, _) => {
                w.string(&r.string(32767)?);
            }
            (5, _) => item(r, w)?,
            (6, _) => {
                w.bytes(r.bytes(12)?);
            }
            (t, _) => {
                return Err(ProtocolError::InvalidValue {
                    what: "тип метаданных 1.7",
                    value: i64::from(t),
                })
            }
        }
    }
    w.u8(0x7F);
    Ok(())
}

// ---------- частицы ----------

/// VANILLA: EnumParticleTypes (1.8.9): имя → id (порядок = id).
const PARTICLES: &[&str] = &[
    "explode",
    "largeexplode",
    "hugeexplosion",
    "fireworksSpark",
    "bubble",
    "splash",
    "wake",
    "suspended",
    "depthsuspend",
    "crit",
    "magicCrit",
    "smoke",
    "largesmoke",
    "spell",
    "instantSpell",
    "mobSpell",
    "mobSpellAmbient",
    "witchMagic",
    "dripWater",
    "dripLava",
    "angryVillager",
    "happyVillager",
    "townaura",
    "note",
    "portal",
    "enchantmenttable",
    "flame",
    "lava",
    "footstep",
    "cloud",
    "reddust",
    "snowballpoof",
    "snowshovel",
    "slime",
    "heart",
    "barrier",
    "iconcrack_",
    "blockcrack_",
    "blockdust_",
    "droplet",
    "take",
    "mobappearance",
];

/// Имя частицы 1.7 → (id 1.8, аргументы). VANILLA: WorldRenderer#spawnParticle (1.7.10):
/// `iconcrack_<id>[_<meta>]`, `blockcrack_<id>_<meta>`, `blockdust_<id>_<meta>`.
/// 1.8: iconcrack — 2 аргумента (id, meta), blockcrack/blockdust — 1 (`id | meta<<12`).
fn particle(name: &str) -> Option<(i32, Vec<i32>)> {
    for (i, p) in PARTICLES.iter().enumerate() {
        let Some(tail) = name.strip_prefix(p) else {
            continue;
        };
        if !p.ends_with('_') {
            if tail.is_empty() {
                return Some((i as i32, Vec::new()));
            }
            continue;
        }
        let mut nums = tail.split('_').map(|s| s.parse::<i32>().unwrap_or(0));
        let id = nums.next().unwrap_or(0);
        let meta = nums.next().unwrap_or(0);
        let args = if *p == "iconcrack_" {
            vec![id, meta]
        } else {
            vec![id | meta << 12]
        };
        return Some((i as i32, args));
    }
    None
}

// ---------- чанки ----------

/// Данные чанка 1.7 (после zlib) → данные 1.8 (`level_chunk_1_8`).
/// VANILLA: Chunk#method_3895 (1.7.10): блоки (byte) всех секций, meta, свет блоков, свет неба,
/// add (только секции из маски add), биомы. 1.8.9: char `(add<<8|id)<<4 | meta` LE, свет, небо,
/// биомы.
fn chunk_data(
    data: &[u8],
    mask: u16,
    add: u16,
    full: bool,
    sky: bool,
    w: &mut Writer,
) -> ProtocolResult<usize> {
    let n = mask.count_ones() as usize;
    let na = add.count_ones() as usize;
    let mut r = Reader::new(data);
    let blocks = r.bytes(n * 4096)?;
    let meta = r.bytes(n * 2048)?;
    let light = r.bytes(n * 2048)?;
    let sky_light = if sky { Some(r.bytes(n * 2048)?) } else { None };
    let adds = r.bytes(na * 2048)?;
    let nib = |a: &[u8], i: usize| (a[i >> 1] >> ((i & 1) << 2)) & 15;
    let (mut s, mut a) = (0, 0);
    for sec in 0..16 {
        let present = mask & (1 << sec) != 0;
        let has_add = add & (1 << sec) != 0;
        if present {
            let b = &blocks[s * 4096..(s + 1) * 4096];
            let m = &meta[s * 2048..(s + 1) * 2048];
            let ad = has_add.then(|| &adds[a * 2048..(a + 1) * 2048]);
            for (i, &lo) in b.iter().enumerate() {
                let hi = ad.map_or(0, |ad| u16::from(nib(ad, i)));
                let id = (hi << 8 | u16::from(lo)) << 4 | u16::from(nib(m, i));
                w.bytes(&id.to_le_bytes());
            }
            s += 1;
        }
        if has_add {
            a += 1;
        }
    }
    w.bytes(light);
    if let Some(sl) = sky_light {
        w.bytes(sl);
    }
    if full {
        w.bytes(r.bytes(256)?);
    }
    Ok(r.pos())
}

/// Есть ли свет неба: по размеру распакованных данных (в пакете 1.7 флага нет).
fn chunk_has_sky(len: usize, mask: u16, add: u16, full: bool) -> bool {
    let n = mask.count_ones() as usize;
    let base = n * 8192 + add.count_ones() as usize * 2048 + if full { 256 } else { 0 };
    n > 0 && len >= base + n * 2048
}

fn inflate(data: &[u8], cap: usize) -> ProtocolResult<Vec<u8>> {
    let mut out = vec![0; cap];
    let n = libdeflater::Decompressor::new()
        .zlib_decompress(data, &mut out)
        .map_err(|e| err(&format!("zlib чанка: {e}")))?;
    out.truncate(n);
    Ok(out)
}

// ---------- clientbound ----------

/// Тело пакета 1.7 → 1.8 (имя — по таблице 1.8). `None` — без изменений.
pub(crate) fn to_1_8(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 32);
    match name {
        "keep_alive" => {
            w.var_int(r.i32()?);
        }
        // VANILLA: GameJoinS2CPacket 1.8: в конце bool reducedDebugInfo.
        "login" => {
            let me = r.i32()?;
            *state() = State {
                me,
                ..State::default()
            };
            w.i32(me).bytes(r.rest()).bool(false);
        }
        // VANILLA: ChatMessageS2CPacket 1.8: в конце byte позиция (0 — чат).
        "chat" => {
            w.bytes(body).u8(0);
        }
        "set_equipped_item" => {
            w.var_int(r.i32()?).i16(r.i16()?);
            item(&mut r, &mut w)?;
        }
        "set_spawn_position" | "open_sign_editor" => {
            let (x, y, z) = (r.i32()?, r.i32()?, r.i32()?);
            pos(&mut w, x, y, z);
        }
        // 0x06: float здоровье, short еда → varint, float насыщение.
        "set_health" => {
            w.f32(r.f32()?).var_int(i32::from(r.i16()?)).f32(r.f32()?);
        }
        // 0x1F: float полоска, short уровень, short всего → varint, varint.
        "set_experience" => {
            w.f32(r.f32()?)
                .var_int(i32::from(r.i16()?))
                .var_int(i32::from(r.i16()?));
        }
        // VANILLA: ServerPlayNetworkHandler#requestTeleport (1.7.10): y глаз; флага
        // относительности нет (1.8: y ног, byte флаги).
        "player_position" => {
            let (x, y, z) = (r.f64()?, r.f64()?, r.f64()?);
            w.f64(x)
                .f64(y - EYE)
                .f64(z)
                .f32(r.f32()?)
                .f32(r.f32()?)
                .u8(0);
        }
        "player_sleep" => {
            w.var_int(r.i32()?);
            let (x, y, z) = (r.i32()?, i32::from(r.u8()?), r.i32()?);
            pos(&mut w, x, y, z);
        }
        // VANILLA: PlayerSpawnS2CPacket 1.7.10: uuid строкой, имя, свойства (в 1.7.2 свойств нет);
        // 1.8 — только uuid.
        "add_player" => {
            let id = r.var_int()?;
            let uuid = crate::v1152::parse_uuid(&r.string(36)?);
            r.string(16)?;
            // Свойства профиля нет только в 1.7.2 (протокол 4); `v16` отдаёт формат 1.7.10.
            if has_profile_properties(ezzmc_protocol::version::current().protocol) {
                for _ in 0..r.length("свойства профиля 1.7", 64)? {
                    for _ in 0..3 {
                        r.string(32767)?;
                    }
                }
            }
            w.var_int(id).uuid(uuid).bytes(r.bytes(12 + 2 + 2)?);
            metadata(&mut r, &mut w, Some(Kind::Player))?;
        }
        "take_item_entity" => {
            w.var_int(r.i32()?).var_int(r.i32()?);
        }
        // VANILLA: ClientPlayNetworkHandler#onEntitySpawn: падающий блок 1.7 — `id | meta<<16`,
        // 1.8 — `id | meta<<12`.
        "add_entity" => {
            w.var_int(r.var_int()?);
            let ty = r.u8()?;
            w.u8(ty);
            let mut pos = [r.i32()?, r.i32()?, r.i32()?];
            let angles = r.bytes(2)?;
            let data = r.i32()?;
            if ty == ITEM_FRAME {
                // Позиция в 1/32 блока.
                let (dx, dz) = hanging_step(data);
                pos[0] = pos[0].saturating_add(dx * 32);
                pos[2] = pos[2].saturating_add(dz * 32);
            }
            for v in pos {
                w.i32(v);
            }
            w.bytes(angles);
            w.i32(if ty == 70 {
                (data & 0xFFFF) | (data >> 16) << 12
            } else {
                data
            });
            w.bytes(r.rest());
        }
        "add_mob" => {
            w.var_int(r.var_int()?);
            let ty = r.u8()?;
            w.u8(ty).bytes(r.bytes(12 + 3 + 6)?);
            metadata(&mut r, &mut w, Some(Kind::Mob(ty)))?;
        }
        // VANILLA: PaintingSpawnS2CPacket 1.7.10: имя, int x, y, z, int направление (0 юг, 1 запад,
        // 2 север, 3 восток — как Direction#getHorizontal 1.8).
        "add_painting" => {
            w.var_int(r.var_int()?).string(&r.string(13)?);
            let (x, y, z) = (r.i32()?, r.i32()?, r.i32()?);
            let dir = r.i32()?;
            let (dx, dz) = hanging_step(dir);
            pos(&mut w, x.saturating_add(dx), y, z.saturating_add(dz));
            w.u8(dir as u8);
        }
        "set_entity_motion" | "move_entity" | "rotate_head" | "remove_mob_effect" => {
            w.var_int(r.i32()?).bytes(r.rest());
        }
        // VANILLA: EntityS2CPacket 1.8 и EntityPositionS2CPacket 1.8: в конце bool onGround
        // (в 1.7 его нет, клиент считает сам).
        "move_entity_pos" | "move_entity_rot" | "move_entity_pos_rot" | "teleport_entity" => {
            w.var_int(r.i32()?).bytes(r.rest()).bool(false);
        }
        "remove_entities" => {
            let n = r.u8()?;
            w.var_int(i32::from(n));
            for _ in 0..n {
                w.var_int(r.i32()?);
            }
        }
        "set_entity_data" => {
            let id = r.i32()?;
            w.var_int(id);
            metadata(&mut r, &mut w, crate::v18::kind_of(id))?;
        }
        // VANILLA: EntityStatusEffectS2CPacket 1.8: длительность varint, byte скрыть частицы.
        "update_mob_effect" => {
            w.var_int(r.i32()?).bytes(r.bytes(2)?);
            w.var_int(i32::from(r.i16()?)).u8(0);
        }
        "update_attributes" => {
            w.var_int(r.i32()?);
            let n = r.i32()?;
            w.i32(n);
            for _ in 0..n.max(0) {
                w.string(&r.string(64)?).f64(r.f64()?);
                let m = r.i16()?;
                w.var_int(i32::from(m));
                for _ in 0..m.max(0) {
                    w.bytes(r.bytes(16 + 8 + 1)?);
                }
            }
        }
        // VANILLA: ChunkDataS2CPacket#read (1.7.10): x, z, полный, short маска, short маска add,
        // int длина, zlib.
        "level_chunk_1_8" => {
            let (x, z, full) = (r.i32()?, r.i32()?, r.bool()?);
            let (mask, add) = (r.u16()?, r.u16()?);
            let len = r.i32()?.max(0) as usize;
            let n = mask.count_ones() as usize;
            let data = inflate(r.bytes(len)?, n * 12288 + 256)?;
            let sky = chunk_has_sky(data.len(), mask, add, full);
            let mut d = Writer::with_capacity(n * 12288 + 256);
            if n > 0 || !full {
                chunk_data(&data, mask, add, full, sky, &mut d)?;
            }
            let d = d.into_vec();
            w.i32(x).i32(z).bool(full).u16(mask);
            w.var_int(d.len() as i32).bytes(&d);
        }
        // VANILLA: ChunkMapS2CPacket#read (1.7.10): short n, int длина, bool небо, zlib, затем
        // n × (x, z, short маска, short маска add).
        "level_chunk_bulk_1_8" => {
            let n = r.i16()?.max(0) as usize;
            let len = r.i32()?.max(0) as usize;
            let sky = r.bool()?;
            let packed = r.bytes(len)?;
            let mut heads = Vec::with_capacity(n);
            let mut total = 0;
            for _ in 0..n {
                let h = (r.i32()?, r.i32()?, r.u16()?, r.u16()?);
                let s = h.2.count_ones() as usize;
                total += s * 8192
                    + 256
                    + h.3.count_ones() as usize * 2048
                    + if sky { s * 2048 } else { 0 };
                heads.push(h);
            }
            if total > 0x400_0000 {
                return Err(ProtocolError::TooLarge {
                    what: "chunk bulk 1.7",
                    len: total,
                    limit: 0x400_0000,
                });
            }
            let data = inflate(packed, total)?;
            w.bool(sky).var_int(n as i32);
            for &(x, z, mask, _) in &heads {
                w.i32(x).i32(z).u16(mask);
            }
            let mut at = 0;
            for &(_, _, mask, add) in &heads {
                at += chunk_data(&data[at..], mask, add, true, sky, &mut w)?;
            }
        }
        // VANILLA: ChunkDeltaUpdateS2CPacket 1.7.10: x, z, short n, int длина, n × (short позиция,
        // short `id<<4|meta`). 1.8: varint n, n × (short позиция, varint состояние).
        "chunk_blocks_update" => {
            w.i32(r.i32()?).i32(r.i32()?);
            let n = r.u16()?;
            r.i32()?;
            w.var_int(i32::from(n));
            for _ in 0..n {
                w.i16(r.i16()?).var_int(i32::from(r.u16()?));
            }
        }
        "block_update" => {
            let (x, y, z) = (r.i32()?, i32::from(r.u8()?), r.i32()?);
            pos(&mut w, x, y, z);
            let id = r.var_int()?;
            w.var_int(id << 4 | i32::from(r.u8()? & 15));
        }
        "block_event" => {
            let (x, y, z) = (r.i32()?, i32::from(r.i16()?), r.i32()?);
            pos(&mut w, x, y, z);
            w.bytes(r.rest());
        }
        "block_destruction" => {
            w.var_int(r.var_int()?);
            let (x, y, z) = (r.i32()?, r.i32()?, r.i32()?);
            pos(&mut w, x, y, z);
            w.bytes(r.rest());
        }
        "level_event" => {
            w.i32(r.i32()?);
            let (x, y, z) = (r.i32()?, i32::from(r.u8()?), r.i32()?);
            pos(&mut w, x, y, z);
            w.bytes(r.rest());
        }
        // VANILLA: ParticleS2CPacket 1.7.10: имя частицы; 1.8: int id, bool дальность, …, аргументы.
        "level_particles" => {
            let name = r.string(64)?;
            let Some((id, args)) = particle(&name) else {
                return Err(err(&format!("частица `{name}`")));
            };
            w.i32(id).bool(false).bytes(r.bytes(7 * 4 + 4)?);
            for a in args {
                w.var_int(a);
            }
        }
        // VANILLA: ClientPlayNetworkHandler#onOpenScreen (1.7.10): тип числом, заголовок строкой
        // (ключ перевода, если `useTitle == false`). 1.8: тип строкой, заголовок JSON.
        "open_screen" => {
            w.u8(r.u8()?);
            let ty = r.u8()?;
            let title = r.string(32)?;
            let slots = r.u8()?;
            let custom = r.bool()?;
            const TYPES: [&str; 12] = [
                "minecraft:chest",
                "minecraft:crafting_table",
                "minecraft:furnace",
                "minecraft:dispenser",
                "minecraft:enchanting_table",
                "minecraft:brewing_stand",
                "minecraft:villager",
                "minecraft:beacon",
                "minecraft:anvil",
                "minecraft:hopper",
                "minecraft:dropper",
                "EntityHorse",
            ];
            let Some(kind) = TYPES.get(usize::from(ty)) else {
                return Err(err(&format!("тип окна {ty}")));
            };
            let json = if custom {
                serde_json::json!({ "text": title })
            } else {
                serde_json::json!({ "translate": title })
            };
            w.string(kind).string(&json.to_string()).u8(slots);
            if ty == 11 {
                w.i32(r.i32()?);
            }
        }
        "container_set_slot" => {
            w.bytes(r.bytes(3)?);
            item(&mut r, &mut w)?;
        }
        "container_set_content" => {
            w.u8(r.u8()?);
            let n = r.i16()?;
            w.i16(n);
            for _ in 0..n.max(0) {
                item(&mut r, &mut w)?;
            }
        }
        // VANILLA: UpdateSignS2CPacket 1.8: строки — текстовые компоненты JSON.
        "update_sign" => {
            let (x, y, z) = (r.i32()?, i32::from(r.i16()?), r.i32()?);
            pos(&mut w, x, y, z);
            for _ in 0..4 {
                w.string(&serde_json::json!({ "text": r.string(15)? }).to_string());
            }
        }
        // VANILLA: MapState#method_185 (1.7.10): 0 — столбец (x, y, цвета), 1 — метки
        // (`тип<<4|поворот`, x, z), 2 — масштаб. 1.8: масштаб и метки в каждом пакете.
        "map_item_data" => {
            let id = r.var_int()?;
            let len = r.u16()?;
            let data = r.bytes(usize::from(len))?;
            let mut st = state();
            let map = st.maps.entry(id).or_default();
            let mut column = None;
            match data.first() {
                Some(0) if data.len() >= 3 => column = Some((data[1], data[2], &data[3..])),
                Some(1) => {
                    map.1 = data[1..]
                        .chunks_exact(3)
                        .map(|c| [c[0], c[1], c[2]])
                        .collect();
                }
                Some(2) if data.len() >= 2 => map.0 = data[1],
                _ => {}
            }
            w.var_int(id).u8(map.0).var_int(map.1.len() as i32);
            for icon in &map.1 {
                w.bytes(icon);
            }
            match column {
                Some((x, y, colors)) if !colors.is_empty() => {
                    w.u8(1).u8(colors.len() as u8).u8(x).u8(y);
                    w.var_int(colors.len() as i32).bytes(colors);
                }
                _ => {
                    w.u8(0);
                }
            }
        }
        "block_entity_data" => {
            let (x, y, z) = (r.i32()?, i32::from(r.i16()?), r.i32()?);
            pos(&mut w, x, y, z);
            w.u8(r.u8()?);
            nbt(&mut r, &mut w)?;
        }
        // VANILLA: PlayerListS2CPacket 1.7.10: имя, онлайн, short пинг. 1.8: действие 0 (добавить),
        // 2 (пинг) или 4 (убрать) по UUID. В offline-mode UUID — offline_uuid(имя).
        "player_info" => {
            let nick = r.string(16)?;
            let online = r.bool()?;
            let ping = i32::from(r.i16()?);
            let uuid = ezzmc_protocol::types::offline_uuid(&nick);
            let mut st = state();
            if !online {
                st.tab.remove(&nick);
                w.var_int(4).var_int(1).uuid(uuid);
            } else if st.tab.contains(&nick) {
                w.var_int(2).var_int(1).uuid(uuid).var_int(ping);
            } else {
                st.tab.insert(nick.clone());
                w.var_int(0).var_int(1).uuid(uuid).string(&nick);
                w.var_int(0).var_int(0).var_int(ping).bool(false);
            }
        }
        // VANILLA: ScoreboardObjectiveUpdateS2CPacket 1.8: имя, режим, [заголовок, тип].
        "set_objective" => {
            let n = r.string(16)?;
            let title = r.string(32)?;
            let mode = r.u8()?;
            w.string(&n).u8(mode);
            if mode == 0 || mode == 2 {
                w.string(&title).string("integer");
            }
        }
        // VANILLA: ScoreboardPlayerUpdateS2CPacket 1.8: имя, действие, цель (всегда), varint очки.
        "set_score" => {
            w.string(&r.string(40)?);
            let action = r.u8()?;
            w.u8(action);
            if action != 1 {
                w.string(&r.string(16)?).var_int(r.i32()?);
            } else {
                w.string("");
            }
        }
        // VANILLA: TeamS2CPacket 1.8: после флагов — видимость ника и цвет; список — varint.
        "set_player_team" => {
            w.string(&r.string(16)?);
            let mode = r.u8()?;
            w.u8(mode);
            if mode == 0 || mode == 2 {
                w.string(&r.string(32)?)
                    .string(&r.string(16)?)
                    .string(&r.string(16)?);
                w.u8(r.u8()?).string("always").u8(0xFF);
            }
            if mode == 0 || mode == 3 || mode == 4 {
                let n = r.i16()?;
                w.var_int(i32::from(n));
                for _ in 0..n.max(0) {
                    w.string(&r.string(40)?);
                }
            }
        }
        "custom_payload" => {
            w.string(&r.string(20)?);
            let n = r.u16()?;
            w.bytes(r.bytes(usize::from(n))?);
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

// ---------- serverbound ----------

fn serverbound_name(id: i32) -> Option<&'static str> {
    use ezzmc_protocol::{Direction, State};
    ezzmc_protocol::version::current()
        .packets
        .iter()
        .find(|p| {
            p.state == State::Play
                && p.dir == Direction::Serverbound
                && i64::from(p.id) == i64::from(id)
        })
        .map(|p| p.name)
}

/// Текст JSON-компонента 1.8 → строка 1.7 (только верхний `text`).
fn plain(json: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(json) {
        Ok(serde_json::Value::String(s)) => s,
        Ok(serde_json::Value::Object(o)) => o
            .get("text")
            .and_then(|t| t.as_str())
            .unwrap_or_default()
            .to_owned(),
        _ => json.to_owned(),
    }
}

/// Тело пакета игрока 1.8 → 1.7 (id тот же). `None` — не отправлять.
/// VANILLA: write() пакетов C2S 1.7.10 и 1.8.9.
pub(crate) fn to_1_7(id: i32, body: Vec<u8>) -> ProtocolResult<Option<Vec<u8>>> {
    let Some(name) = serverbound_name(id) else {
        return Ok(Some(body));
    };
    let mut r = Reader::new(&body);
    let mut w = Writer::with_capacity(body.len() + 16);
    match name {
        "teleport_to_entity" | "resource_pack" => return Ok(None),
        "keep_alive" => {
            w.i32(r.var_int()?);
        }
        // VANILLA: PlayerInteractEntityC2SPacket 1.7.10: int цель, byte (0 использовать, 1 ударить);
        // INTERACT_AT (2) появился в 1.8.
        "interact" => {
            let target = r.var_int()?;
            let action = r.var_int()?;
            if action == 2 {
                return Ok(None);
            }
            w.i32(target).u8(action as u8);
        }
        // VANILLA: ClientPlayNetworkHandler#onPlayerPositionLook (1.7.10): x, minY (ноги),
        // y (глаза, stance), z.
        "move_player_pos" | "move_player_pos_rot" => {
            let (x, y, z) = (r.f64()?, r.f64()?, r.f64()?);
            w.f64(x).f64(y).f64(y + EYE).f64(z).bytes(r.rest());
        }
        "player_action" => {
            w.u8(r.u8()?);
            let (x, y, z) = unpos(r.i64()?);
            w.i32(x).u8(y as u8).i32(z).u8(r.u8()?);
        }
        "use_item_on" => {
            let (x, y, z) = unpos(r.i64()?);
            w.i32(x).u8(y as u8).i32(z).u8(r.u8()?);
            item_1_7(&mut r, &mut w)?;
            w.bytes(r.rest());
        }
        // VANILLA: HandSwingC2SPacket 1.7.10: int id игрока, byte 1 (взмах).
        "swing" => {
            w.i32(state().me).u8(1);
        }
        // VANILLA: ClientCommandC2SPacket 1.7.10: int id, byte действие с 1, int сила прыжка.
        "player_command" => {
            w.i32(r.var_int()?)
                .u8(r.var_int()? as u8 + 1)
                .i32(r.var_int()?);
        }
        "player_input" => {
            w.f32(r.f32()?).f32(r.f32()?);
            let f = r.u8()?;
            w.bool(f & 1 != 0).bool(f & 2 != 0);
        }
        "container_click" => {
            w.bytes(r.bytes(7)?);
            item_1_7(&mut r, &mut w)?;
        }
        "set_creative_mode_slot" => {
            w.i16(r.i16()?);
            item_1_7(&mut r, &mut w)?;
        }
        "sign_update" => {
            let (x, y, z) = unpos(r.i64()?);
            w.i32(x).i16(y as i16).i32(z);
            for _ in 0..4 {
                let s: String = plain(&r.string(32767)?).chars().take(15).collect();
                w.string(&s);
            }
        }
        "command_suggestion" => {
            w.string(&r.string(32767)?);
        }
        // VANILLA: ClientSettingsC2SPacket 1.7.10: язык, дальность, чат, цвета, сложность, плащ.
        "client_information" => {
            w.string(&r.string(16)?)
                .u8(r.u8()?)
                .u8(r.u8()?)
                .bool(r.bool()?);
            w.u8(2).bool(true);
        }
        "client_command" => {
            w.u8(r.var_int()? as u8);
        }
        "custom_payload" => {
            w.string(&r.string(20)?);
            let rest = r.rest();
            w.i16(rest.len() as i16).bytes(rest);
        }
        _ => return Ok(Some(body)),
    }
    Ok(Some(w.into_vec()))
}

/// Предмет 1.8 (NBT без сжатия, байт 0 — нет) → 1.7 (short длина + gzip, -1 — нет).
fn item_1_7(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let id = r.i16()?;
    w.i16(id);
    if id < 0 {
        return Ok(());
    }
    w.bytes(r.bytes(3)?);
    let at = r.pos();
    if ezzmc_protocol::nbt::read_named(r)?.is_none() {
        w.i16(-1);
    } else {
        let gz = gzip(r.since(at))?;
        w.i16(gz.len() as i16).bytes(&gz);
    }
    Ok(())
}

fn has_profile_properties(protocol: i32) -> bool {
    protocol != ezzmc_protocol::version::V1_7_2.protocol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_netty_spawn_player_carries_properties() {
        use ezzmc_protocol::version::{V1_5_2, V1_6_4, V1_7_10, V1_7_2};
        for v in [V1_5_2, V1_6_4, V1_7_10] {
            assert!(has_profile_properties(v.protocol), "{}", v.name);
        }
        assert!(!has_profile_properties(V1_7_2.protocol));
    }

    #[test]
    fn particle_names() {
        assert_eq!(particle("heart"), Some((34, vec![])));
        assert_eq!(particle("iconcrack_373_8193"), Some((36, vec![373, 8193])));
        assert_eq!(particle("iconcrack_381"), Some((36, vec![381, 0])));
        assert_eq!(particle("blockcrack_1_2"), Some((37, vec![1 | 2 << 12])));
        assert_eq!(particle("nope"), None);
    }

    #[test]
    fn block_pos_1_8() {
        let mut w = Writer::new();
        pos(&mut w, -5, 64, 123);
        let v = Reader::new(&w.into_vec()).i64().unwrap();
        assert_eq!(unpos(v), (-5, 64, 123));
    }

    #[test]
    fn chunk_section_ids() {
        // Одна секция: блок 0 — id 0x1A3 (add 1, id 0xA3), meta 5.
        let mut d = vec![0u8; 4096 + 2048 * 3 + 2048 + 256];
        d[0] = 0xA3;
        d[4096] = 5;
        d[4096 + 2048 * 3] = 1;
        assert!(chunk_has_sky(d.len(), 1, 1, true));
        let mut w = Writer::new();
        let used = chunk_data(&d, 1, 1, true, true, &mut w).unwrap();
        assert_eq!(used, d.len());
        let out = w.into_vec();
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), 0x1A3 << 4 | 5);
        assert_eq!(out.len(), 8192 + 2048 * 2 + 256);
    }
}
