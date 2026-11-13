//! Трансляция тел пакетов сервера старой версии в формат канона (26.3) — глава 18, TODO-372.
//! Весь клиент разбирает только канонический формат; здесь исправляются расхождения
//! между версиями. Реестровые id (предметы, сущности, звуки, частицы) — позже в этом же месте.
use ezzmc_protocol::version::{build_remap, CANON, V1_21_11};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;
use std::sync::OnceLock;

fn entity_type_map() -> &'static [Option<u32>] {
    static T: OnceLock<Vec<Option<u32>>> = OnceLock::new();
    T.get_or_init(|| build_remap(V1_21_11.entity_type, CANON.entity_type))
}

/// Тело пакета (имя из таблицы версии соединения) → канон, если версия не каноническая.
/// `None` — пакет надо отбросить (в каноне нет аналога).
pub fn to_canon(name: Option<&str>, body: Vec<u8>) -> ProtocolResult<Option<Vec<u8>>> {
    let Some(n) = name else { return Ok(Some(body)) };
    if ezzmc_protocol::version::current().is_canon() {
        return Ok(Some(body));
    }
    match clientbound_1_21_11(n, &body)? {
        Some(b) if b.is_empty() && !body.is_empty() => Ok(None),
        Some(b) => Ok(Some(b)),
        None => Ok(Some(body)),
    }
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
            w.var_int(id).bytes(uuid).var_int(canon as i32);
            w.bytes(r.rest());
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
