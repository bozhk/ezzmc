//! `set_entity_data` протоколов 1.14–1.20.1 → формат 26.3: индексы и сериализаторы по таблицам
//! `entity_data_gen`, значения — в формат канона.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use ezzmc_protocol::generated::r26_3;
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};

use crate::entity_data_gen::{Fields, REMAP_1_13, SERIALIZERS, TYPES};

/// id сущности → тип 26.3 (из `add_entity`/`add_player`/входа).
fn kinds() -> MutexGuard<'static, HashMap<i32, &'static str>> {
    static M: OnceLock<Mutex<HashMap<i32, &'static str>>> = OnceLock::new();
    M.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

pub(crate) fn track(id: i32, canon_type: u32) {
    if let Some(name) = r26_3::ENTITY_TYPE.get(canon_type as usize) {
        kinds().insert(id, name);
    }
}

pub(crate) fn track_player(id: i32) {
    kinds().insert(id, "player");
}

fn fields(era: usize, id: i32) -> Option<Fields> {
    let name = *kinds().get(&id)?;
    // 1.13: индексы сначала переводятся в 1.14.4 (`remap_1_13`), дальше таблицы 1.14.4.
    let types = TYPES.get(era)?;
    let at = types.binary_search_by_key(&name, |(n, _)| *n).ok()?;
    types.get(at).map(|(_, f)| *f)
}

fn remap_1_13(id: i32) -> Option<&'static [u8]> {
    let name = *kinds().get(&id)?;
    let at = REMAP_1_13.binary_search_by_key(&name, |(n, _)| *n).ok()?;
    REMAP_1_13.get(at).map(|(_, m)| *m)
}

/// VANILLA: ClientboundSetEntityDataPacket: id, записи (u8 индекс, varint сериализатор,
/// значение), конец 0xFF. `None` — сущность неизвестна.
pub(crate) fn to_canon(era: usize, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    let old = crate::v1201::is_1_13();
    // Эры 1.13 в таблицах нет: сериализаторы — начало списка 1.14.4.
    let table = if old { era.saturating_sub(1) } else { era };
    let (Some(fields), Some(serializers)) = (fields(table, id), SERIALIZERS.get(table)) else {
        return Ok(None);
    };
    let remap = old.then(|| remap_1_13(id).unwrap_or_default());
    let mut w = Writer::with_capacity(body.len() + 16);
    w.var_int(id);
    loop {
        let idx = r.u8()?;
        if idx == 0xFF {
            break;
        }
        let src = r.var_int()?;
        let ser = usize::try_from(src)
            .ok()
            .and_then(|i| serializers.get(i))
            .ok_or(ProtocolError::InvalidValue {
                what: "entity data serializer",
                value: src.into(),
            })?;
        let mut v = Writer::new();
        let keep = value(era, ser, &mut r, &mut v)?;
        let idx = match remap {
            Some(map) => map.get(usize::from(idx)).copied().unwrap_or(0xFF),
            None => idx,
        };
        // Мод или неизвестное поле: значение прочитано и отброшено.
        let target = fields.get(usize::from(idx)).filter(|f| f.0 != 0xFF);
        if let (true, Some(&(canon_idx, canon_ser))) = (keep, target) {
            w.u8(canon_idx)
                .var_int(i32::from(canon_ser))
                .bytes(&v.into_vec());
        }
    }
    w.u8(0xFF);
    Ok(Some(w.into_vec()))
}

/// Значение старой версии → формат 26.3. `false` — перевести нечем (прочитано целиком).
fn value(era: usize, ser: &str, r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
    match ser {
        "BYTE" | "BOOLEAN" => {
            w.u8(r.u8()?);
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
        "COMPONENT" => {
            w.bytes(&crate::v1201::json_component(&r.string(262144)?));
        }
        "OPTIONAL_COMPONENT" => {
            let some = r.bool()?;
            w.bool(some);
            if some {
                w.bytes(&crate::v1201::json_component(&r.string(262144)?));
            }
        }
        // 1.13/1.13.1: предмет с коротким id → форма 1.13.2.
        "ITEM_STACK" if ezzmc_protocol::version::current().protocol < 404 => {
            let mut new = Writer::new();
            crate::v1132::item_to_1_13_2(r, &mut new)?;
            let new = new.into_vec();
            crate::v1201::item_to_canon(&mut Reader::new(&new), w)?;
        }
        "ITEM_STACK" => crate::v1201::item_to_canon(r, w)?,
        "ROTATIONS" | "VECTOR3" => {
            w.bytes(r.bytes(12)?);
        }
        "QUATERNION" => {
            w.bytes(r.bytes(16)?);
        }
        "BLOCK_POS" => {
            w.bytes(r.bytes(8)?);
        }
        "OPTIONAL_BLOCK_POS" | "OPTIONAL_LIVING_ENTITY_REFERENCE" => {
            let some = r.bool()?;
            w.bool(some);
            if some {
                let n = if ser == "OPTIONAL_BLOCK_POS" { 8 } else { 16 };
                w.bytes(r.bytes(n)?);
            }
        }
        "BLOCK_STATE" | "OPTIONAL_BLOCK_STATE" => {
            let v = r.var_int()?;
            let state = if v == 0 {
                0
            } else {
                ezzmc_world::legacy::state_to_canon(v as u32) as i32
            };
            w.var_int(state);
        }
        "COMPOUND_TAG" => {
            ezzmc_protocol::nbt::read_named(r)?;
            return Ok(false);
        }
        "PARTICLE" => return particle(r, w),
        "VILLAGER_DATA" => {
            for _ in 0..3 {
                w.var_int(r.var_int()?);
            }
        }
        // VANILLA: Pose — до 1.17 нет LONG_JUMPING (6), DYING был 6.
        "POSE" => {
            let v = r.var_int()?;
            w.var_int(if era >= 7 && v == 6 { 7 } else { v });
        }
        "OPTIONAL_GLOBAL_POS" => {
            let some = r.bool()?;
            w.bool(some);
            if some {
                w.string(&r.string(32767)?).bytes(r.bytes(8)?);
            }
        }
        // INT, DIRECTION, OPTIONAL_UNSIGNED_INT, варианты и состояния — VarInt.
        _ => {
            w.var_int(r.var_int()?);
        }
    }
    Ok(true)
}

/// VANILLA: ParticleTypes (1.14–1.20.1): varint id, данные по типу. Клиенту из метаданных нужен
/// только `dragon_breath` (облако дракона); остальные пропускаются.
fn particle(r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
    let id = r.var_int()?;
    let name = usize::try_from(id)
        .ok()
        .and_then(|i| ezzmc_protocol::version::current().particle_type.get(i))
        .copied()
        .ok_or(ProtocolError::InvalidValue {
            what: "particle type",
            value: id.into(),
        })?;
    match name {
        "block" | "block_marker" | "falling_dust" => {
            r.var_int()?;
        }
        "dust" => {
            r.bytes(16)?;
        }
        "dust_color_transition" => {
            r.bytes(28)?;
        }
        "item" => crate::v1201::item_to_canon(r, &mut Writer::new())?,
        "sculk_charge" => {
            r.f32()?;
        }
        "shriek" => {
            r.var_int()?;
        }
        "vibration" => {
            // источник вибрации нестандартной длины: дальше читать нечем
            return Err(ProtocolError::InvalidValue {
                what: "vibration particle in entity data",
                value: id.into(),
            });
        }
        "dragon_breath" => {
            let canon = r26_3::PARTICLE_TYPE.iter().position(|n| *n == name);
            let Some(canon) = canon else { return Ok(false) };
            w.var_int(canon as i32).f32(1.0);
            return Ok(true);
        }
        _ => {}
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// VANILLA: Entity (+TicksFrozen #7 с 1.17), LivingEntity (+Stinger с 1.15).
    #[test]
    fn living_indices_shift_to_canon() {
        // era 10 = 1.14.x: health #8, pose #6, сон #12
        let player = |era: usize| {
            let types = TYPES[era];
            types[types.binary_search_by_key(&"player", |(n, _)| *n).unwrap()].1
        };
        assert_eq!(player(10)[8].0, 9);
        assert_eq!(player(10)[12].0, 14);
        assert_eq!(player(9)[13].0, 14);
        assert_eq!(player(0)[9].0, 9);
        for era in 0..TYPES.len() {
            assert_eq!(player(era)[6].0, 6);
        }
    }

    #[test]
    fn old_dying_pose_and_unknown_entity() {
        let id = -4001;
        let mut b = Writer::new();
        // 1.14: сериализатор 18 = POSE
        b.var_int(id).u8(6).var_int(18).var_int(6).u8(0xFF);
        assert!(to_canon(10, &b.buf).unwrap().is_none());
        track_player(id);
        let out = to_canon(10, &b.buf).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), id);
        assert_eq!(r.u8().unwrap(), 6);
        assert_eq!(r.var_int().unwrap(), 20);
        assert_eq!(r.var_int().unwrap(), 7);
    }
}
