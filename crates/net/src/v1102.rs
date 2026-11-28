//! Отличия протокола 1.10–1.10.2 (210) от 1.11: тела clientbound пакетов приводятся к формату 1.11,
//! дальше идут общим путём 1.11/1.12 (`v1122`). Применяется до разворота `add_mob` на спавн и
//! метаданные: тип сущностей, разделённых в 1.11, определяется по метаданным спавна.
//!
//! VANILLA: 1.11 разделил варианты на отдельные сущности (EntityType 1.11.2):
//! скелет (тип в метаданных) → skeleton/wither_skeleton/stray, зомби → zombie/zombie_villager/husk,
//! лошадь → horse/donkey/mule/zombie_horse/skeleton_horse, страж → guardian/elder_guardian.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{nbt, ProtocolError, ProtocolResult};

/// Сущность 1.10, у которой в 1.11 свой тип: вариант запоминается со спавна.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Split {
    /// VANILLA: ZombieType (1.10.2): 0 обычный, 1–5 житель (профессия + 1), 6 кадавр.
    Zombie(i32),
    Skeleton,
    /// VANILLA: HorseType (1.10.2): 0 лошадь, 1 осёл, 2 мул, 3 зомби, 4 скелет.
    Horse(i32),
    Guardian,
}

fn splits() -> MutexGuard<'static, HashMap<i32, Split>> {
    static M: OnceLock<Mutex<HashMap<i32, Split>>> = OnceLock::new();
    M.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// Запись метаданных: индекс, сериализатор, значение (байты).
pub(crate) type Entry = (u8, i32, Vec<u8>);

/// VANILLA: DataTracker#deserializePacket, TrackedDataHandlerRegistry (1.10.2): 0 byte, 1 varint,
/// 2 float, 3 string, 4 text, 5 item, 6 bool, 7 rotation, 8 pos, 9 opt pos, 10 facing,
/// 11 opt uuid, 12 opt block state.
pub(crate) fn entries(r: &mut Reader) -> ProtocolResult<Vec<Entry>> {
    let mut out = Vec::new();
    loop {
        let idx = r.u8()?;
        if idx == 0xFF {
            return Ok(out);
        }
        let ser = r.var_int()?;
        let at = r.pos();
        match ser {
            0 | 6 => {
                r.u8()?;
            }
            1 | 10 | 12 => {
                r.var_int()?;
            }
            2 => {
                r.bytes(4)?;
            }
            3 | 4 => {
                r.string(262144)?;
            }
            5 => {
                if r.i16()? >= 0 {
                    r.bytes(3)?;
                    nbt::read_named(r)?;
                }
            }
            7 => {
                r.bytes(12)?;
            }
            8 => {
                r.bytes(8)?;
            }
            9 | 11 => {
                if r.bool()? {
                    r.bytes(if ser == 9 { 8 } else { 16 })?;
                }
            }
            _ => {
                return Err(ProtocolError::InvalidValue {
                    what: "1.10 entity data serializer",
                    value: ser.into(),
                })
            }
        }
        out.push((idx, ser, r.since(at).to_vec()));
    }
}

fn int(e: &[Entry], idx: u8) -> Option<i32> {
    let (_, _, v) = e.iter().find(|(i, s, _)| *i == idx && *s == 1)?;
    Reader::new(v).var_int().ok()
}

fn byte(e: &[Entry], idx: u8) -> Option<u8> {
    let (_, _, v) = e.iter().find(|(i, s, _)| *i == idx && *s == 0)?;
    v.first().copied()
}

/// Тип 1.10 + метаданные спавна → вариант и id типа 1.11.
fn split(old: u8, e: &[Entry]) -> Option<(Split, i32)> {
    Some(match old {
        51 => (
            Split::Skeleton,
            match int(e, 12).unwrap_or(0) {
                1 => 5,
                2 => 6,
                _ => 51,
            },
        ),
        54 => {
            let t = int(e, 13).unwrap_or(0);
            let ty = match t {
                1..=5 => 27,
                6 => 23,
                _ => 54,
            };
            (Split::Zombie(t), ty)
        }
        68 => {
            // VANILLA: GuardianEntity#isElder (1.10.2) — бит 4 флагов
            let elder = byte(e, 12).unwrap_or(0) & 4 != 0;
            (Split::Guardian, if elder { 4 } else { 68 })
        }
        100 => {
            let t = int(e, 14).unwrap_or(0);
            let ty = match t {
                1 => 31,
                2 => 32,
                3 => 29,
                4 => 28,
                _ => 100,
            };
            (Split::Horse(t), ty)
        }
        _ => return None,
    })
}

/// Индексы метаданных 1.10 → 1.11 для разделённых сущностей (запись без пары отбрасывается).
fn remap(kind: Split, spawn: bool, e: Vec<Entry>) -> Vec<Entry> {
    let mut out = Vec::with_capacity(e.len() + 1);
    for (idx, ser, v) in e {
        let new = match (kind, idx) {
            (_, 0..=11) => Some((idx, ser, v)),
            // VANILLA: ZombieEntity (1.10.2): 12 ребёнок, 13 тип, 14 превращение, 15 руки;
            // 1.11: 12 ребёнок, 13 (не используется), 14 руки; ZombieVillagerEntity: 15 превращение,
            // 16 профессия.
            (Split::Zombie(_), 12) => Some((12, ser, v)),
            (Split::Zombie(t), 14) if (1..=5).contains(&t) => Some((15, ser, v)),
            (Split::Zombie(_), 15) => Some((14, ser, v)),
            // VANILLA: SkeletonEntity (1.10.2): 12 тип, 13 руки; AbstractSkeletonEntity (1.11): 12 руки.
            (Split::Skeleton, 13) => Some((12, ser, v)),
            // VANILLA: GuardianEntity (1.10.2): 12 флаги (бит 2 — шипы убраны); 1.11: 12 bool.
            (Split::Guardian, 12) => {
                Some((12, 6, vec![u8::from(v.first().is_some_and(|b| b & 2 != 0))]))
            }
            (Split::Guardian, 13) => Some((13, ser, v)),
            // VANILLA: HorseBaseEntity (1.10.2): 12 ребёнок, 13 флаги (бит 8 — сундук), 14 тип,
            // 15 окрас, 16 владелец, 17 броня. 1.11 AbstractHorseEntity: 13 флаги, 14 владелец;
            // HorseBaseEntity: 15 окрас, 16 броня; AbstractDonkeyEntity: 15 сундук.
            (Split::Horse(t), 13) => {
                let flags = v.first().copied().unwrap_or(0);
                if matches!(t, 1 | 2) {
                    out.push((15, 6, vec![u8::from(flags & 8 != 0)]));
                }
                Some((13, ser, vec![flags & !8]))
            }
            (Split::Horse(_), 12) => Some((12, ser, v)),
            (Split::Horse(_), 16) => Some((14, ser, v)),
            (Split::Horse(0), 15) => Some((15, ser, v)),
            (Split::Horse(0), 17) => Some((16, ser, v)),
            _ => None,
        };
        out.extend(new);
    }
    if let (Split::Zombie(t @ 1..=5), true) = (kind, spawn) {
        let mut v = Writer::new();
        v.var_int(t - 1);
        out.push((16, 1, v.into_vec()));
    }
    out
}

fn write_entries(w: &mut Writer, e: &[Entry]) {
    for (idx, ser, v) in e {
        w.u8(*idx).var_int(*ser).bytes(v);
    }
    w.u8(0xFF);
}

/// Тело clientbound пакета 1.10 → 1.11. `None` — без изменений.
pub(crate) fn to_1_11(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 8);
    match name {
        // VANILLA: MobSpawnS2CPacket (1.10.2): тип — байт; 1.11 — varint.
        "add_mob" => {
            let id = r.var_int()?;
            let uuid = r.bytes(16)?;
            let old = r.u8()?;
            let fixed = r.bytes(24 + 3 + 6)?;
            let e = if r.remaining() > 0 {
                entries(&mut r)?
            } else {
                Vec::new()
            };
            let (e, ty) = match split(old, &e) {
                Some((kind, ty)) => {
                    splits().insert(id, kind);
                    (remap(kind, true, e), ty)
                }
                None => {
                    splits().remove(&id);
                    (e, i32::from(old))
                }
            };
            w.var_int(id).bytes(uuid).var_int(ty).bytes(fixed);
            write_entries(&mut w, &e);
        }
        "set_entity_data" => {
            let id = r.var_int()?;
            let Some(kind) = splits().get(&id).copied() else {
                return Ok(None);
            };
            let e = remap(kind, false, entries(&mut r)?);
            w.var_int(id);
            write_entries(&mut w, &e);
        }
        // VANILLA: ItemPickupAnimationS2CPacket (1.11): + количество (в 1.10 нет).
        "take_item_entity" => {
            w.bytes(body).var_int(1);
        }
        // VANILLA: TitleS2CPacket.Action (1.11): ACTIONBAR вставлен вторым → TIMES/CLEAR/RESET +1.
        "set_titles" => {
            let action = r.var_int()?;
            w.var_int(if action >= 2 { action + 1 } else { action })
                .bytes(r.rest());
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(e: &[Entry]) -> Vec<u8> {
        let mut w = Writer::new();
        write_entries(&mut w, e);
        w.into_vec()
    }

    fn varint(v: i32) -> Vec<u8> {
        let mut w = Writer::new();
        w.var_int(v);
        w.into_vec()
    }

    fn spawn(id: i32, ty: u8, e: &[Entry]) -> Vec<u8> {
        let mut w = Writer::new();
        w.var_int(id).bytes(&[7; 16]).u8(ty).bytes(&[0; 33]);
        w.bytes(&meta(e));
        w.into_vec()
    }

    fn parse(body: &[u8]) -> (i32, Vec<Entry>) {
        let mut r = Reader::new(body);
        r.var_int().unwrap();
        r.bytes(16).unwrap();
        let ty = r.var_int().unwrap();
        r.bytes(33).unwrap();
        (ty, entries(&mut r).unwrap())
    }

    #[test]
    fn zombie_villager_gets_own_type_and_profession() {
        let body = spawn(
            901,
            54,
            &[
                (12, 6, vec![1]),
                (13, 1, varint(3)),
                (14, 6, vec![1]),
                (15, 6, vec![0]),
            ],
        );
        let (ty, e) = parse(&to_1_11("add_mob", &body).unwrap().unwrap());
        assert_eq!(ty, 27);
        let idx: Vec<_> = e.iter().map(|(i, s, v)| (*i, *s, v.clone())).collect();
        assert_eq!(
            idx,
            vec![
                (12, 6, vec![1]),
                (15, 6, vec![1]),
                (14, 6, vec![0]),
                (16, 1, varint(2))
            ]
        );
        // обновление: руки 15 → 14
        let mut upd = Writer::new();
        upd.var_int(901).bytes(&meta(&[(15, 6, vec![1])]));
        let out = to_1_11("set_entity_data", &upd.into_vec())
            .unwrap()
            .unwrap();
        assert_eq!(&out[2..], &meta(&[(14, 6, vec![1])])[..]);
    }

    #[test]
    fn horses_skeletons_guardians() {
        let (ty, e) = parse(
            &to_1_11(
                "add_mob",
                &spawn(
                    902,
                    100,
                    &[(13, 0, vec![8 | 4]), (14, 1, varint(1)), (17, 1, varint(0))],
                ),
            )
            .unwrap()
            .unwrap(),
        );
        assert_eq!(ty, 31);
        assert_eq!(e, vec![(15, 6, vec![1]), (13, 0, vec![4])]);

        let (ty, e) = parse(
            &to_1_11(
                "add_mob",
                &spawn(903, 51, &[(12, 1, varint(2)), (13, 6, vec![1])]),
            )
            .unwrap()
            .unwrap(),
        );
        assert_eq!((ty, e), (6, vec![(12, 6, vec![1])]));

        let (ty, e) = parse(
            &to_1_11("add_mob", &spawn(904, 68, &[(12, 0, vec![4 | 2])]))
                .unwrap()
                .unwrap(),
        );
        assert_eq!((ty, e), (4, vec![(12, 6, vec![1])]));

        let (ty, _) = parse(&to_1_11("add_mob", &spawn(905, 90, &[])).unwrap().unwrap());
        assert_eq!(ty, 90);
    }

    #[test]
    fn titles_and_pickup() {
        assert_eq!(to_1_11("set_titles", &[2, 0]).unwrap().unwrap(), vec![3, 0]);
        assert_eq!(to_1_11("set_titles", &[1]).unwrap().unwrap(), vec![1]);
        assert_eq!(
            to_1_11("take_item_entity", &[5, 6]).unwrap().unwrap(),
            vec![5, 6, 1]
        );
    }
}
