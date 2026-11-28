//! 1.6.x (pre-Netty, протокол 78) и 1.5.2 (61) ↔ формат 1.7.10. Дальше пакеты идут путём 1.7
//! (`v17`).
//!
//! VANILLA: `Packet#readPacket`/`writePacket` и read/write пакетов 1.6.4 (Yarn):
//! - кадра нет: байт id, затем поля; конец пакета известен только после разбора всех полей;
//! - строки — short длина в символах + UTF-16BE (в 1.7 — varint длина + UTF-8);
//! - id сущностей и блоков — int/short (в 1.7 часть стала varint);
//! - вход: 0x02 → 0xFD (ключ, всегда, даже offline) → 0xFC ↔ 0xFC → AES/CFB8 → 0xCD 0 → 0x01.
//!
//! `decode` отдаёт пакеты 1.7 (id и тело формата 1.7.10); пакеты входа — с id `RAW + id 1.6`
//! и телом 1.6. `encode` переводит serverbound 1.7 в 1.6; id от `RAW` пишутся как есть.
//!
//! 1.5.2 (VANILLA: read/write пакетов 1.5.2, Yarn) отличается от 1.6.4 только полями: чат —
//! простой текст, здоровье short, способности байтами (/255), статистика byte, у 0x27 нет
//! поводка, у 0x13 нет силы прыжка, нет 0x1B/0x2C/0x85, индексы метаданных живых сущностей
//! другие (`living_index_1_5`).

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::Duration;

use ezzmc_protocol::types::{format_uuid, offline_uuid, GameProfile};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};

use crate::conn::Connection;
use crate::frame::Packet;
use crate::session::PacketIds;
use crate::{NetError, NetResult};

/// Пакеты входа 1.6: id `RAW + id`, тело 1.6 без перевода.
pub const RAW: i32 = 0x100;

/// Сервер 1.5.2 (протокол 61).
fn is_1_5() -> bool {
    ezzmc_protocol::version::current().protocol == ezzmc_protocol::version::V1_5_2.protocol
}

/// Сервер 1.6.1 (протокол 73).
fn is_1_6_1() -> bool {
    ezzmc_protocol::version::current().protocol == ezzmc_protocol::version::V1_6_1.protocol
}

/// Живые сущности 1.5.2 (их метаданные переиндексируются): id → игрок ли.
static LIVING: Mutex<Option<HashMap<i32, bool>>> = Mutex::new(None);

fn living(id: i32) -> Option<bool> {
    LIVING.lock().ok()?.as_ref()?.get(&id).copied()
}

fn set_living(id: i32, player: Option<bool>) {
    if let Ok(mut m) = LIVING.lock() {
        let m = m.get_or_insert_with(HashMap::new);
        match player {
            Some(p) => {
                m.insert(id, p);
            }
            None => {
                m.remove(&id);
            }
        }
    }
}

/// Индекс метаданных живой сущности 1.5.2 → 1.6.4, `None` — поля нет в 1.6.
/// VANILLA: MobEntity#initDataTracker (1.5.2): 5 имя, 6 имя видно, 8 цвет зелий, 9 фон зелий,
/// 10 стрелы; 1.6.4 LivingEntity: 6 здоровье, 7 цвет, 8 фон, 9 стрелы; MobEntity: 10 имя,
/// 11 имя видно. PlayerEntity 17: byte в 1.5.2 (не используется), float поглощение в 1.6.4.
fn living_index_1_5(i: u8, player: bool) -> Option<u8> {
    Some(match i {
        5 => 10,
        6 => 11,
        8 => 7,
        9 => 8,
        10 => 9,
        17 if player => return None,
        i => i,
    })
}

fn err(s: &str) -> ProtocolError {
    ProtocolError::Other(format!("1.6: {s}"))
}

/// VANILLA: Packet#readString (1.6.4): short длина (символы UTF-16), не больше `max`.
fn s16(r: &mut Reader, max: usize) -> ProtocolResult<String> {
    let n = r.i16()?;
    if n < 0 || n as usize > max {
        return Err(err(&format!("длина строки {n} (макс. {max})")));
    }
    let units: Vec<u16> = r
        .bytes(n as usize * 2)?
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();
    Ok(String::from_utf16_lossy(&units))
}

/// 1.6 chat JSON → 1.7 form: `using` → `with`, a `text` array → `extra`
/// (VANILLA: ChatMessageSerializer#deserialize, 1.6.4). Not JSON → plain text.
fn chat_json(s: &str) -> String {
    fn conv(v: serde_json::Value) -> serde_json::Value {
        use serde_json::Value;
        match v {
            Value::Object(mut m) => {
                if let Some(u) = m.remove("using") {
                    let with = match u {
                        Value::Array(a) => a.into_iter().map(conv).collect(),
                        other => vec![other],
                    };
                    m.insert("with".into(), Value::Array(with));
                }
                if let Some(Value::Array(a)) = m.remove("text") {
                    m.insert("text".into(), Value::String(String::new()));
                    m.insert(
                        "extra".into(),
                        Value::Array(a.into_iter().map(conv).collect()),
                    );
                }
                Value::Object(m)
            }
            other => other,
        }
    }
    match serde_json::from_str::<serde_json::Value>(s) {
        Ok(v @ serde_json::Value::Object(_)) => conv(v).to_string(),
        _ => serde_json::json!({ "text": s }).to_string(),
    }
}

fn w16(w: &mut Writer, s: &str) {
    let units: Vec<u16> = s.encode_utf16().collect();
    w.i16(units.len() as i16);
    for u in units {
        w.u16(u);
    }
}

/// VANILLA: Packet#readBytes (1.6.4): short длина + байты.
fn short_bytes<'a>(r: &mut Reader<'a>) -> ProtocolResult<&'a [u8]> {
    let n = r.i16()?;
    if n < 0 {
        return Err(err(&format!("длина массива {n}")));
    }
    r.bytes(n as usize)
}

/// VANILLA: Packet#readStack (1.6.4): short id (-1 — пусто), byte число, short урон,
/// short длина gzip NBT (-1 — нет). Формат 1.7 тот же — копируется как есть.
fn item(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let at = r.pos();
    if r.i16()? >= 0 {
        r.bytes(3)?;
        let n = r.i16()?;
        if n > 0 {
            r.bytes(n as usize)?;
        }
    }
    w.bytes(r.since(at));
    Ok(())
}

/// VANILLA: DataTracker#method_6176 (1.6.4): байт `тип<<5|индекс`, 0x7F — конец; тип 4 —
/// строка (UTF-16 → UTF-8 для 1.7), 5 — предмет, 6 — три int. `entity` — id сущности для
/// переиндексации 1.5.2.
fn metadata(r: &mut Reader, w: &mut Writer, entity: i32) -> ProtocolResult<()> {
    let player = if is_1_5() { living(entity) } else { None };
    loop {
        let b = r.u8()?;
        if b == 0x7F {
            w.u8(b);
            return Ok(());
        }
        let mut v = Writer::new();
        match b >> 5 {
            0 => {
                v.bytes(r.bytes(1)?);
            }
            1 => {
                v.bytes(r.bytes(2)?);
            }
            2 | 3 => {
                v.bytes(r.bytes(4)?);
            }
            4 => {
                v.string(&s16(r, 64)?);
            }
            5 => item(r, &mut v)?,
            6 => {
                v.bytes(r.bytes(12)?);
            }
            t => return Err(err(&format!("тип метаданных {t}"))),
        }
        let index = match player {
            Some(p) => living_index_1_5(b & 0x1F, p),
            None => Some(b & 0x1F),
        };
        if let Some(i) = index {
            w.u8(b & 0xE0 | i).bytes(&v.into_vec());
        }
    }
}

/// Один пакет 1.6 из начала `buf`: `Ok(None)` — данных ещё мало; иначе (съедено байт,
/// пакет 1.7 или `None` — пропустить).
pub(crate) fn decode(buf: &[u8]) -> ProtocolResult<Option<(usize, Option<Packet>)>> {
    let mut r = Reader::new(buf);
    match clientbound(&mut r) {
        Ok(p) => Ok(Some((r.pos(), p))),
        Err(ProtocolError::Eof { .. }) => Ok(None),
        Err(e) => Err(e),
    }
}

fn clientbound(r: &mut Reader) -> ProtocolResult<Option<Packet>> {
    let id = r.u8()?;
    tracing::trace!("pre-Netty ← 0x{id:02X}");
    let mut w = Writer::new();
    let out = match id {
        0x00 => {
            w.i32(r.i32()?);
            0x00
        }
        // VANILLA: GameJoinS2CPacket 1.6.4: int id, тип мира, byte режим|8, byte измерение,
        // byte сложность, byte высота мира, byte макс. игроков. 1.7: тип мира в конце, без высоты.
        0x01 => {
            let me = r.i32()?;
            if let Ok(mut m) = LIVING.lock() {
                *m = Some(HashMap::from([(me, true)]));
            }
            let level = s16(r, 16)?;
            let (mode, dim, diff) = (r.u8()?, r.u8()?, r.u8()?);
            r.u8()?;
            w.i32(me)
                .u8(mode)
                .u8(dim)
                .u8(diff)
                .u8(r.u8()?)
                .string(&level);
            0x01
        }
        // VANILLA: ChatMessageS2CPacket 1.6.4: JSON (ChatMessage#fromJson) строкой UTF-16.
        // VANILLA: ChatMessageS2CPacket 1.5.2: простой текст с кодами `§`.
        0x03 => {
            let s = s16(r, 32767)?;
            if is_1_5() {
                w.string(&serde_json::json!({ "text": s }).to_string());
            } else {
                w.string(&chat_json(&s));
            }
            0x02
        }
        0x04 => {
            w.bytes(r.bytes(16)?);
            0x03
        }
        0x05 => {
            w.i32(r.i32()?).i16(r.i16()?);
            item(r, &mut w)?;
            0x04
        }
        0x06 => {
            w.bytes(r.bytes(12)?);
            0x05
        }
        // VANILLA: HealthUpdateS2CPacket 1.5.2: short здоровье; 1.6.4: float.
        0x08 => {
            if is_1_5() {
                w.f32(f32::from(r.i16()?)).bytes(r.bytes(6)?);
            } else {
                w.bytes(r.bytes(10)?);
            }
            0x06
        }
        // VANILLA: PlayerRespawnS2CPacket 1.6.4: int измерение, byte сложность, byte режим,
        // short высота мира, тип мира. 1.7: без высоты.
        0x09 => {
            w.i32(r.i32()?).u8(r.u8()?).u8(r.u8()?);
            r.i16()?;
            w.string(&s16(r, 16)?);
            0x07
        }
        0x0A => {
            r.u8()?;
            return Ok(None);
        }
        0x0B => {
            r.bytes(33)?;
            return Ok(None);
        }
        0x0C => {
            r.bytes(9)?;
            return Ok(None);
        }
        // VANILLA: ClientPlayNetworkHandler#onPlayerMove (1.6.4): позиция игрока — второе поле
        // (`collisionY`, глаза), третье (`y`) не читается. 1.7: x, y глаз, z.
        0x0D => {
            let (x, y) = (r.f64()?, r.f64()?);
            r.f64()?;
            w.f64(x).f64(y).f64(r.f64()?).bytes(r.bytes(9)?);
            0x08
        }
        0x10 => {
            w.u8(r.i16()? as u8);
            0x09
        }
        // VANILLA: class_665 (сон, 1.6.4): int id, byte (не используется), int x, byte y, int z.
        0x11 => {
            w.i32(r.i32()?);
            r.u8()?;
            w.i32(r.i32()?).u8(r.u8()?).i32(r.i32()?);
            0x0A
        }
        // VANILLA: ClientPlayNetworkHandler#onEntityAnimation (1.6.4): 1 взмах, 2 урон, 3 встать
        // с кровати, 5 еда, 6 крит, 7 магический крит. 1.7.10: 0–5 в том же порядке.
        0x12 => {
            let e = r.i32()?;
            let a = match r.u8()? {
                1 => 0,
                2 => 1,
                3 => 2,
                5 => 3,
                6 => 4,
                7 => 5,
                _ => return Ok(None),
            };
            w.var_int(e).u8(a);
            0x0B
        }
        // VANILLA: PlayerSpawnS2CPacket 1.6.4: int id, имя, … 1.7.10: varint id, uuid строкой,
        // имя, свойства профиля (offline — пусто).
        0x14 => {
            let e = r.i32()?;
            set_living(e, Some(true));
            let name = s16(r, 16)?;
            let uuid = format_uuid(offline_uuid(&name));
            w.var_int(e).string(&uuid).string(&name).var_int(0);
            w.bytes(r.bytes(16)?);
            metadata(r, &mut w, e)?;
            0x0C
        }
        0x16 => {
            w.bytes(r.bytes(8)?);
            0x0D
        }
        0x17 => {
            w.var_int(r.i32()?).bytes(r.bytes(1 + 12 + 2)?);
            let data = r.i32()?;
            w.i32(data);
            if data > 0 {
                w.bytes(r.bytes(6)?);
            }
            0x0E
        }
        0x18 => {
            let e = r.i32()?;
            set_living(e, Some(false));
            w.var_int(e).bytes(r.bytes(1 + 12 + 3 + 6)?);
            metadata(r, &mut w, e)?;
            0x0F
        }
        0x19 => {
            w.var_int(r.i32()?).string(&s16(r, 13)?).bytes(r.bytes(16)?);
            0x10
        }
        0x1A => {
            w.var_int(r.i32()?).bytes(r.bytes(14)?);
            0x11
        }
        0x1C => {
            w.bytes(r.bytes(10)?);
            0x12
        }
        0x1D => {
            let n = r.u8()?;
            w.u8(n);
            for _ in 0..n {
                let e = r.i32()?;
                set_living(e, None);
                w.i32(e);
            }
            0x13
        }
        0x1E => {
            w.bytes(r.bytes(4)?);
            0x14
        }
        0x1F => {
            w.bytes(r.bytes(7)?);
            0x15
        }
        0x20 => {
            w.bytes(r.bytes(6)?);
            0x16
        }
        0x21 => {
            w.bytes(r.bytes(9)?);
            0x17
        }
        0x22 => {
            w.bytes(r.bytes(18)?);
            0x18
        }
        0x23 => {
            w.bytes(r.bytes(5)?);
            0x19
        }
        0x26 => {
            w.bytes(r.bytes(5)?);
            0x1A
        }
        // VANILLA: EntityAttachS2CPacket 1.5.2: без байта поводка (только посадка).
        0x27 => {
            w.bytes(r.bytes(8)?);
            w.u8(if is_1_5() { 0 } else { r.u8()? });
            0x1B
        }
        0x28 => {
            let e = r.i32()?;
            w.i32(e);
            metadata(r, &mut w, e)?;
            0x1C
        }
        0x29 => {
            w.bytes(r.bytes(8)?);
            0x1D
        }
        0x2A => {
            w.bytes(r.bytes(5)?);
            0x1E
        }
        0x2B => {
            w.bytes(r.bytes(8)?);
            0x1F
        }
        // VANILLA: EntityAttributesS2CPacket 1.6.1: только ключ и значение; с 1.6.2 — ещё
        // список модификаторов.
        0x2C => {
            w.i32(r.i32()?);
            let n = r.i32()?;
            w.i32(n);
            for _ in 0..n.max(0) {
                w.string(&s16(r, 64)?).f64(r.f64()?);
                let m = if is_1_6_1() { 0 } else { r.i16()? };
                w.i16(m).bytes(r.bytes(m.max(0) as usize * 25)?);
            }
            0x20
        }
        // Чанки, мультиблок и пачка чанков: формат 1.7 тот же.
        0x33 => {
            w.bytes(r.bytes(13)?);
            let n = r.i32()?;
            w.i32(n).bytes(r.bytes(n.max(0) as usize)?);
            0x21
        }
        0x34 => {
            w.bytes(r.bytes(10)?);
            let n = r.i32()?;
            w.i32(n).bytes(r.bytes(n.max(0) as usize)?);
            0x22
        }
        0x35 => {
            w.bytes(r.bytes(9)?)
                .var_int(i32::from(r.i16()?))
                .u8(r.u8()?);
            0x23
        }
        0x36 => {
            w.bytes(r.bytes(12)?).var_int(i32::from(r.i16()? & 4095));
            0x24
        }
        0x37 => {
            w.var_int(r.i32()?).bytes(r.bytes(13)?);
            0x25
        }
        0x38 => {
            let n = r.i16()?;
            let len = r.i32()?;
            w.i16(n).i32(len).bytes(r.bytes(1 + len.max(0) as usize)?);
            w.bytes(r.bytes(n.max(0) as usize * 12)?);
            0x26
        }
        // VANILLA: ExplosionS2CPacket 1.6.4: double центр; 1.7.10: float.
        0x3C => {
            for _ in 0..3 {
                w.f32(r.f64()? as f32);
            }
            w.f32(r.f32()?);
            let n = r.i32()?;
            w.i32(n).bytes(r.bytes(n.max(0) as usize * 3)?);
            w.bytes(r.bytes(12)?);
            0x27
        }
        0x3D => {
            w.bytes(r.bytes(18)?);
            0x28
        }
        0x3E => {
            w.string(&s16(r, 256)?).bytes(r.bytes(17)?);
            0x29
        }
        0x3F => {
            w.string(&s16(r, 64)?).bytes(r.bytes(32)?);
            0x2A
        }
        // VANILLA: GameStateChangeS2CPacket 1.6.4: byte причина, byte значение; 1.7.10: float.
        0x46 => {
            w.u8(r.u8()?).f32(f32::from(r.i8()?));
            0x2B
        }
        0x47 => {
            w.var_int(r.i32()?).bytes(r.bytes(13)?);
            0x2C
        }
        0x64 => {
            let (sync, ty) = (r.u8()?, r.u8()?);
            w.u8(sync).u8(ty).string(&s16(r, 32)?).bytes(r.bytes(2)?);
            if ty == 11 && !is_1_5() {
                w.i32(r.i32()?);
            }
            0x2D
        }
        0x65 => {
            w.bytes(r.bytes(1)?);
            0x2E
        }
        0x67 => {
            w.bytes(r.bytes(3)?);
            item(r, &mut w)?;
            0x2F
        }
        0x68 => {
            w.u8(r.u8()?);
            let n = r.i16()?;
            w.i16(n);
            for _ in 0..n.max(0) {
                item(r, &mut w)?;
            }
            0x30
        }
        0x69 => {
            w.bytes(r.bytes(5)?);
            0x31
        }
        0x6A => {
            w.bytes(r.bytes(4)?);
            0x32
        }
        0x6B => {
            r.i16()?;
            item(r, &mut w)?;
            return Ok(None);
        }
        0x82 => {
            w.bytes(r.bytes(10)?);
            for _ in 0..4 {
                w.string(&s16(r, 15)?);
            }
            0x33
        }
        // VANILLA: MapUpdateS2CPacket 1.6.4: short предмет, short id карты, ushort длина;
        // 1.7.10: varint id карты, short длина.
        0x83 => {
            r.i16()?;
            w.var_int(i32::from(r.i16()?));
            let n = r.u16()?;
            w.u16(n).bytes(r.bytes(usize::from(n))?);
            0x34
        }
        0x84 => {
            w.bytes(r.bytes(11)?);
            let n = r.i16()?;
            w.i16(n);
            if n > 0 {
                w.bytes(r.bytes(n as usize)?);
            }
            0x35
        }
        0x85 => {
            r.u8()?;
            w.bytes(r.bytes(12)?);
            0x36
        }
        // VANILLA: StatisticsS2CPacket 1.5.2: int id, byte значение; 1.6.4: int, int.
        0xC8 => {
            let id = r.i32()?;
            let amount = if is_1_5() {
                i32::from(r.u8()? as i8)
            } else {
                r.i32()?
            };
            // 1.7.10: varint n, (строка, varint сумма).
            let Some((key, total)) = crate::stats::add_increment(id, amount) else {
                return Ok(None);
            };
            w.var_int(1).string(&key).var_int(total);
            0x37
        }
        0xC9 => {
            w.string(&s16(r, 16)?).bytes(r.bytes(3)?);
            0x38
        }
        // VANILLA: PlayerAbilitiesS2CPacket 1.5.2: скорости полёта/ходьбы байтом ×255; 1.6.4: float.
        0xCA => {
            if is_1_5() {
                w.u8(r.u8()?)
                    .f32(f32::from(r.u8()?) / 255.0)
                    .f32(f32::from(r.u8()?) / 255.0);
            } else {
                w.bytes(r.bytes(9)?);
            }
            0x39
        }
        // VANILLA: ChatSuggestionS2CPacket 1.6.4: варианты одной строкой через `\0`.
        0xCB => {
            let s = s16(r, 32767)?;
            let all: Vec<&str> = s.split('\0').collect();
            w.var_int(all.len() as i32);
            for a in all {
                w.string(a);
            }
            0x3A
        }
        0xCE => {
            w.string(&s16(r, 16)?).string(&s16(r, 32)?).u8(r.u8()?);
            0x3B
        }
        0xCF => {
            w.string(&s16(r, 16)?);
            let action = r.u8()?;
            w.u8(action);
            if action != 1 {
                w.string(&s16(r, 16)?).i32(r.i32()?);
            }
            0x3C
        }
        0xD0 => {
            w.u8(r.u8()?).string(&s16(r, 16)?);
            0x3D
        }
        0xD1 => {
            w.string(&s16(r, 16)?);
            let mode = r.u8()?;
            w.u8(mode);
            if mode == 0 || mode == 2 {
                w.string(&s16(r, 32)?)
                    .string(&s16(r, 16)?)
                    .string(&s16(r, 16)?)
                    .u8(r.u8()?);
            }
            if mode == 0 || mode == 3 || mode == 4 {
                let n = r.i16()?;
                w.i16(n);
                for _ in 0..n.max(0) {
                    w.string(&s16(r, 16)?);
                }
            }
            0x3E
        }
        // VANILLA: CustomPayloadC2SPacket#read (1.6.4): данные только при 0 < длина < 32767.
        0xFA => {
            w.string(&s16(r, 20)?);
            let n = r.i16()?;
            if n > 0 && n < 32767 {
                w.i16(n).bytes(r.bytes(n as usize)?);
            } else {
                w.i16(0);
            }
            0x3F
        }
        0xFC => {
            let at = r.pos();
            short_bytes(r)?;
            short_bytes(r)?;
            return Ok(Some(Packet {
                id: RAW + 0xFC,
                body: r.since(at).to_vec(),
            }));
        }
        0xFD => {
            let at = r.pos();
            s16(r, 20)?;
            short_bytes(r)?;
            short_bytes(r)?;
            return Ok(Some(Packet {
                id: RAW + 0xFD,
                body: r.since(at).to_vec(),
            }));
        }
        // VANILLA: DisconnectS2CPacket 1.6.4: причина простой строкой; 1.7 — JSON.
        0xFF => {
            let reason = s16(r, 256)?;
            tracing::debug!("pre-Netty кик: {reason}");
            w.string(&serde_json::json!({ "text": reason }).to_string());
            0x40
        }
        _ => return Err(err(&format!("неизвестный пакет 0x{id:02X}"))),
    };
    Ok(Some(Packet {
        id: out,
        body: w.into_vec(),
    }))
}

/// Serverbound 1.7.10 → 1.6.4 (байт id + поля), дописать в `out`. id от `RAW` — тело уже 1.6.
pub(crate) fn encode(id: i32, body: &[u8], out: &mut Vec<u8>) -> ProtocolResult<()> {
    encode_inner(id, body, out).inspect_err(|e| tracing::warn!("pre-Netty → 0x{id:02X}: {e}"))
}

fn encode_inner(id: i32, body: &[u8], out: &mut Vec<u8>) -> ProtocolResult<()> {
    if id >= RAW {
        out.push((id - RAW) as u8);
        out.extend_from_slice(body);
        return Ok(());
    }
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 16);
    let new_id: u8 = match id {
        0x00 => {
            w.bytes(body);
            0x00
        }
        0x01 => {
            let msg = r.string(100)?;
            tracing::trace!("pre-Netty чат → {msg:?}");
            w16(&mut w, &msg);
            0x03
        }
        // VANILLA: PlayerInteractEntityC2SPacket 1.6.4: int id игрока, int цель, byte (0/1 — как
        // в 1.7).
        0x02 => {
            w.i32(crate::v17::me()).bytes(body);
            0x07
        }
        0x03..=0x06 => {
            w.bytes(body);
            id as u8 + 7
        }
        0x07 => {
            w.bytes(body);
            0x0E
        }
        0x08 => {
            w.bytes(body);
            0x0F
        }
        0x09 => {
            w.bytes(body);
            0x10
        }
        0x0A => {
            w.bytes(body);
            0x12
        }
        // VANILLA: ClientCommandC2SPacket 1.5.2: int id, byte действие (1–5), без силы прыжка.
        0x0B if is_1_5() => {
            w.bytes(r.bytes(4)?);
            let a = r.u8()?;
            if a > 5 {
                return Ok(());
            }
            w.u8(a);
            0x13
        }
        0x0B => {
            w.bytes(body);
            0x13
        }
        0x0C if is_1_5() => return Ok(()),
        0x0C => {
            w.bytes(body);
            0x1B
        }
        0x0D => {
            w.bytes(body);
            0x65
        }
        0x0E => {
            w.bytes(body);
            0x66
        }
        0x0F => {
            w.bytes(body);
            0x6A
        }
        0x10 => {
            w.bytes(body);
            0x6B
        }
        0x11 => {
            w.bytes(body);
            0x6C
        }
        0x12 => {
            w.bytes(r.bytes(10)?);
            for _ in 0..4 {
                w16(&mut w, &r.string(15)?);
            }
            0x82
        }
        0x13 if is_1_5() => {
            w.u8(r.u8()?)
                .u8((r.f32()? * 255.0) as u8)
                .u8((r.f32()? * 255.0) as u8);
            0xCA
        }
        0x13 => {
            w.bytes(body);
            0xCA
        }
        0x14 => {
            w16(&mut w, &r.string(32767)?);
            0xCB
        }
        // VANILLA: class_651 (настройки, 1.6.4): язык (7), дальность, `чат | цвета<<3`,
        // сложность, плащ.
        0x15 => {
            let lang: String = r.string(16)?.chars().take(7).collect();
            w16(&mut w, &lang);
            let view = r.u8()?;
            let chat = r.u8()?;
            let colors = r.bool()?;
            w.u8(view)
                .u8(chat & 7 | u8::from(colors) << 3)
                .bytes(r.rest());
            0xCC
        }
        // VANILLA: ClientStatusC2SPacket 1.6.4: 0 — первый вход, 1 — возрождение.
        0x16 => {
            if r.u8()? != 0 {
                return Ok(());
            }
            w.u8(1);
            0xCD
        }
        0x17 => {
            w16(&mut w, &r.string(20)?);
            w.bytes(r.rest());
            0xFA
        }
        _ => return Ok(()),
    };
    tracing::trace!("pre-Netty → 0x{new_id:02X}");
    out.push(new_id);
    out.extend_from_slice(&w.into_vec());
    Ok(())
}

/// Вход на сервер 1.6 (offline): pre-Netty рукопожатие, затем общий путь `v1201::finish_login`.
pub fn login(
    conn: &mut Connection,
    ids: &PacketIds,
    host: &str,
    port: u16,
    name: &str,
) -> NetResult<crate::v1201::Joined> {
    conn.set_pre_netty();
    crate::stats::open_totals(name);
    let wire = ezzmc_protocol::version::current().wire_protocol();
    // VANILLA: HelloC2SPacket 1.6.4: byte протокол, имя, хост, int порт.
    let mut w = Writer::new();
    w.u8(wire as u8);
    w16(&mut w, name);
    w16(&mut w, host);
    w.i32(i32::from(port));
    conn.send(RAW + 0x02, &w.into_vec())?;
    let mut secret = None;
    loop {
        let p = conn.recv()?;
        match p.id {
            // VANILLA: ClientLoginNetworkHandler#onHello (1.6.4): serverId "-" — без сессии;
            // ключ шифруется всегда.
            id if id == RAW + 0xFD => {
                let mut r = Reader::new(&p.body);
                let server_id = s16(&mut r, 20)?;
                let key = short_bytes(&mut r)?;
                let nonce = short_bytes(&mut r)?;
                if server_id != "-" {
                    return Err(NetError::Disconnected(
                        "сервер в online-mode: нужен вход через лаунчер (Microsoft)".into(),
                    ));
                }
                let s = crate::crypt::random_secret();
                let es = crate::crypt::rsa_encrypt(key, &s).map_err(|x| err(&x))?;
                let en = crate::crypt::rsa_encrypt(key, nonce).map_err(|x| err(&x))?;
                let mut w = Writer::new();
                w.i16(es.len() as i16)
                    .bytes(&es)
                    .i16(en.len() as i16)
                    .bytes(&en);
                conn.send(RAW + 0xFC, &w.into_vec())?;
                secret = Some(s);
            }
            // VANILLA: OutboundConnection#readPacket (1.6.4): после ответа 0xFC шифруются оба
            // направления; ClientLoginNetworkHandler#onKey шлёт ClientStatusC2SPacket(0).
            id if id == RAW + 0xFC => {
                let Some(s) = secret else {
                    return Err(err("0xFC до 0xFD").into());
                };
                conn.enable_encryption(&s);
                conn.send(RAW + 0xCD, &[0])?;
                break;
            }
            0x40 => return Err(kicked(&p.body)),
            _ => {}
        }
    }
    // Сервер может кикнуть сразу после шифрования (вместо 0x01): причину — человеку.
    let next = conn.recv()?;
    if next.id == 0x40 {
        return Err(kicked(&next.body));
    }
    conn.push_front(next);
    crate::v1201::finish_login(
        conn,
        ids,
        GameProfile {
            id: offline_uuid(name),
            name: name.to_owned(),
            properties: Vec::new(),
        },
    )
}

/// Кик (0xFF, уже в форме 1.7: JSON строкой) → ошибка с текстом причины.
fn kicked(body: &[u8]) -> NetError {
    let s = match Reader::new(body).string(32767) {
        Ok(s) => s,
        Err(e) => return e.into(),
    };
    let text = ezzmc_protocol::text::Component::from_json_str(&s)
        .map(|c| c.to_plain(&|_| None))
        .unwrap_or(s);
    NetError::Disconnected(text)
}

/// VANILLA: EmptyS2CPacket#write (1.6.4, пинг списка серверов): 0xFE 0x01, MC|PingHost с
/// протоколом, хостом и портом. Ответ — 0xFF `§1\0протокол\0версия\0motd\0онлайн\0макс`.
/// Возвращает протокол сервера (pre-Netty).
pub fn ping(address: &str) -> NetResult<i32> {
    let (host, port) = crate::srv::parse_address(address);
    let (thost, tport) = crate::srv::resolve_target(&host, port);
    let addrs = crate::srv::resolve_addrs(&thost, tport)?;
    let Some(addr) = addrs.first() else {
        return Err(NetError::Closed);
    };
    let mut s = TcpStream::connect_timeout(addr, Duration::from_secs(3))?;
    // Сервер 1.5.2 отвечает на `FE 01` сразу (часть серверов 1.5.2 молчит, получив хвост 1.6);
    // сервер 1.6.4 ждёт хвост `MC|PingHost` (EmptyS2CPacket#read, 1.6.4) — досылаем его,
    // если ответа нет. Сервер 1.6.1 принимает хвост лишь около секунды (проверено на стенде:
    // через 0,9 с отвечает с протоколом, через 1,5 с — старым форматом без него).
    s.write_all(&[0xFE, 0x01])?;
    s.set_read_timeout(Some(Duration::from_millis(500)))?;
    let mut buf = Vec::new();
    let _ = s.read_to_end(&mut buf);
    if buf.is_empty() {
        let mut w = Writer::new();
        w.u8(0xFA);
        w16(&mut w, "MC|PingHost");
        w.i16(7 + 2 * host.encode_utf16().count() as i16).u8(78);
        w16(&mut w, &host);
        w.i32(i32::from(port));
        s.write_all(&w.into_vec())?;
        s.set_read_timeout(Some(Duration::from_secs(3)))?;
        let _ = s.read_to_end(&mut buf);
    }
    let mut r = Reader::new(&buf);
    if r.u8()? != 0xFF {
        return Err(NetError::Closed);
    }
    let reply = s16(&mut r, 256)?;
    reply
        .split('\0')
        .nth(1)
        .and_then(|p| p.parse().ok())
        .ok_or(NetError::Closed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_using_to_with() {
        let j = chat_json(r#"{"translate":"chat.type.announcement","using":["Server","hi"]}"#);
        let v: serde_json::Value = serde_json::from_str(&j).unwrap();
        assert_eq!(v["with"][1], "hi");
        assert!(v.get("using").is_none());
        let v: serde_json::Value =
            serde_json::from_str(&chat_json(r#"{"text":["a",{"text":"b"}]}"#)).unwrap();
        assert_eq!(v["extra"][0], "a");
        assert_eq!(v["text"], "");
        assert_eq!(chat_json("plain"), r#"{"text":"plain"}"#);
    }

    #[test]
    fn utf16_strings() {
        let mut w = Writer::new();
        w16(&mut w, "Привет");
        let b = w.into_vec();
        assert_eq!(b.len(), 2 + 12);
        assert_eq!(s16(&mut Reader::new(&b), 16).unwrap(), "Привет");
    }

    #[test]
    fn partial_packet_waits() {
        // Чат 0x03: строка "hi"; без последнего байта — ждать.
        let full = [0x03, 0, 2, 0, b'h', 0, b'i'];
        assert!(decode(&full[..6]).unwrap().is_none());
        let (n, p) = decode(&full).unwrap().unwrap();
        assert_eq!(n, full.len());
        let p = p.unwrap();
        assert_eq!(p.id, 0x02);
        assert_eq!(Reader::new(&p.body).string(64).unwrap(), r#"{"text":"hi"}"#);
    }

    #[test]
    fn join_to_1_7() {
        let mut w = Writer::new();
        w.u8(0x01).i32(7);
        w16(&mut w, "default");
        w.u8(1).u8(0).u8(2).u8(0).u8(20);
        let (_, p) = decode(&w.into_vec()).unwrap().unwrap();
        let p = p.unwrap();
        let mut r = Reader::new(&p.body);
        assert_eq!(r.i32().unwrap(), 7);
        assert_eq!(
            (r.u8().unwrap(), r.u8().unwrap(), r.u8().unwrap()),
            (1, 0, 2)
        );
        assert_eq!(r.u8().unwrap(), 20);
        assert_eq!(r.string(16).unwrap(), "default");
    }

    #[test]
    fn metadata_string() {
        let mut w = Writer::new();
        w.u8(0x28).i32(5).u8(4 << 5 | 10);
        w16(&mut w, "Bob");
        w.u8(0x7F);
        let (_, p) = decode(&w.into_vec()).unwrap().unwrap();
        let p = p.unwrap();
        assert_eq!(p.id, 0x1C);
        let mut r = Reader::new(&p.body);
        r.i32().unwrap();
        assert_eq!(r.u8().unwrap(), 4 << 5 | 10);
        assert_eq!(r.string(64).unwrap(), "Bob");
        assert_eq!(r.u8().unwrap(), 0x7F);
    }

    #[test]
    fn settings_to_1_6() {
        let mut w = Writer::new();
        w.string("en_US").u8(6).u8(0).bool(true).u8(2).bool(true);
        let mut out = Vec::new();
        encode(0x15, &w.into_vec(), &mut out).unwrap();
        assert_eq!(out[0], 0xCC);
        let mut r = Reader::new(&out[1..]);
        assert_eq!(s16(&mut r, 7).unwrap(), "en_US");
        assert_eq!((r.u8().unwrap(), r.u8().unwrap()), (6, 8));
    }

    #[test]
    fn living_indexes_1_5() {
        assert_eq!(living_index_1_5(5, false), Some(10));
        assert_eq!(living_index_1_5(8, false), Some(7));
        assert_eq!(living_index_1_5(16, true), Some(16));
        assert_eq!(living_index_1_5(17, true), None);
        assert_eq!(living_index_1_5(17, false), Some(17));
    }
}
