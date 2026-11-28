//! 1.9.4 (110) → формат 1.10 (210). Таблица пакетов та же, тела отличаются:
//! - VANILLA: Entity (1.10.2) — новый `NO_GRAVITY` (индекс 5); все индексы метаданных ≥ 5 у 1.9.4
//!   на единицу меньше. Порядок полей остальных классов совпадает (DataTracker 1.9.4/1.10.2).
//! - VANILLA: PlaySoundIdS2CPacket/PlaySoundNameS2CPacket (1.9.4): высота — u8 (×63), в 1.10 — float.
//! - VANILLA: ResourcePackStatusC2SPacket (1.9.4): перед статусом — хеш набора из запроса сервера.
//!
//! 1.9–1.9.2 (107–109) → 1.9.4 (jar без Yarn, сверено по байткоду пакетов с 1.9.4):
//! - ChunkDataS2CPacket: без списка блок-сущностей (добавляется пустой);
//! - clientbound UpdateSignS2CPacket → BlockEntityUpdateS2CPacket(9) с NBT таблички (1.9.4);
//! - 1.9 (107): GameJoinS2CPacket — измерение byte (с 1.9.1 int).

use std::sync::{Mutex, OnceLock, PoisonError};

use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;

/// Хеш последнего запроса набора ресурсов (нужен в ответе 1.9.4).
fn last_hash() -> &'static Mutex<String> {
    static H: OnceLock<Mutex<String>> = OnceLock::new();
    H.get_or_init(|| Mutex::new(String::new()))
}

/// Метаданные 1.9.4 → 1.10: индексы ≥ 5 сдвигаются на +1 (значения не меняются).
fn metadata(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    for (idx, ser, value) in crate::v1102::entries(r)? {
        w.u8(if idx >= 5 { idx + 1 } else { idx })
            .var_int(ser)
            .bytes(&value);
    }
    w.u8(0xFF);
    Ok(())
}

/// Тело клиентского пакета 1.9.4 → 1.10. `None` — без изменений.
pub(crate) fn to_1_10(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let protocol = ezzmc_protocol::version::current().protocol;
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 8);
    match name {
        // VANILLA: ChunkDataS2CPacket#write (1.9.4) — в конце varint-список NBT блок-сущностей.
        "level_chunk" if protocol < 110 => {
            w.bytes(body).var_int(0);
        }
        // VANILLA: GameJoinS2CPacket (1.9): id, режим, byte измерение, сложность, макс. игроков…
        "login" if protocol < 108 => {
            w.i32(r.i32()?).u8(r.u8()?);
            w.i32(i32::from(r.i8()?)).bytes(r.rest());
        }
        // VANILLA: MobSpawnS2CPacket (1.9.4): id, uuid, u8 тип, xyz, 3 угла, скорость, метаданные.
        "add_mob" => {
            w.var_int(r.var_int()?);
            w.bytes(r.bytes(16 + 1 + 24 + 3 + 6)?);
            metadata(&mut r, &mut w)?;
        }
        // VANILLA: PlayerSpawnS2CPacket (1.9.4): id, uuid, xyz, 2 угла, метаданные.
        "add_player" => {
            w.var_int(r.var_int()?);
            w.bytes(r.bytes(16 + 24 + 2)?);
            metadata(&mut r, &mut w)?;
        }
        "set_entity_data" => {
            w.var_int(r.var_int()?);
            metadata(&mut r, &mut w)?;
        }
        // VANILLA: PlaySoundIdS2CPacket (1.9.4): id, категория, xyz (int ×8), громкость, u8 высота.
        "sound" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            w.bytes(r.bytes(12 + 4)?);
            w.f32(f32::from(r.u8()?) / 63.0);
        }
        // VANILLA: PlaySoundNameS2CPacket (1.9.4): имя, категория, xyz, громкость, u8 высота.
        "custom_sound" => {
            w.string(&r.string(256)?).var_int(r.var_int()?);
            w.bytes(r.bytes(12 + 4)?);
            w.f32(f32::from(r.u8()?) / 63.0);
        }
        // VANILLA: ResourcePackSendS2CPacket (1.9.4): url, хеш — запомнить хеш для ответа.
        "resource_pack" => {
            r.string(32767)?;
            let hash = r.string(40)?;
            *last_hash().lock().unwrap_or_else(PoisonError::into_inner) = hash;
            return Ok(None);
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

/// `update_sign` 1.9–1.9.2 → тело `block_entity_data` 1.9.4.
/// VANILLA: UpdateSignS2CPacket#read (1.9.2): позиция, 4 текста JSON;
/// SignBlockEntity#toNbt, #getUpdatePacket (1.9.4): действие 9, `Sign`, x/y/z, Text1..Text4.
pub(crate) fn update_sign(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let pos = r.i64()?;
    let (x, y, z) = (
        (pos >> 38) as i32,
        ((pos >> 26) & 0xFFF) as i32,
        ((pos << 38) >> 38) as i32,
    );
    let mut tag = vec![
        ("id".to_owned(), Tag::String("Sign".to_owned())),
        ("x".to_owned(), Tag::Int(x)),
        ("y".to_owned(), Tag::Int(y)),
        ("z".to_owned(), Tag::Int(z)),
    ];
    for i in 1..=4 {
        tag.push((format!("Text{i}"), Tag::String(r.string(32767)?)));
    }
    let mut w = Writer::with_capacity(body.len() + 64);
    w.i64(pos).u8(9);
    let mut out = w.into_vec();
    nbt::write_named(&mut out, "", &Tag::Compound(tag))?;
    Ok(out)
}

/// Хеш для `ResourcePackStatusC2SPacket` 1.9.4.
pub(crate) fn resource_pack_hash() -> String {
    last_hash()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_indices_and_sound_pitch() {
        // set_entity_data: id 7, (0 byte 0), (6 float hp) → 6 становится 7.
        let mut w = Writer::new();
        w.var_int(7).u8(0).var_int(0).u8(0);
        w.u8(6).var_int(2).f32(20.0).u8(0xFF);
        let out = to_1_10("set_entity_data", &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 7);
        let e = crate::v1102::entries(&mut r).unwrap();
        assert_eq!(e.iter().map(|x| x.0).collect::<Vec<_>>(), [0, 7]);

        let mut w = Writer::new();
        w.var_int(3)
            .var_int(0)
            .i32(8)
            .i32(16)
            .i32(24)
            .f32(1.0)
            .u8(63);
        let out = to_1_10("sound", &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        r.bytes(2 + 12 + 4).unwrap();
        assert_eq!(r.f32().unwrap(), 1.0);
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn sign_to_block_entity() {
        let mut w = Writer::new();
        let pos = ((-5_i64 & 0x3FF_FFFF) << 38) | (64 << 26) | (7 & 0x3FF_FFFF);
        w.i64(pos);
        for t in ["{\"text\":\"a\"}", "\"\"", "\"\"", "\"\""] {
            w.string(t);
        }
        let out = update_sign(&w.into_vec()).unwrap();
        let be = crate::v1122::block_entities("block_entity_data", &out)
            .unwrap()
            .unwrap();
        let (at, tag) = &be.entries[0];
        assert_eq!(*at, [-5, 64, 7]);
        assert_eq!(
            tag.get("Text1").and_then(Tag::as_str),
            Some("{\"text\":\"a\"}")
        );
    }
}
