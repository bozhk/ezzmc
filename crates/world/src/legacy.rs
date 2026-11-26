//! Трансляция id состояний блоков старых версий в канон (26.3) — глава 18, TODO-387.
//! Совпадение по имени блока и значениям свойств; нет блока/значения → состояние по умолчанию
//! канонического блока, нет блока вовсе → воздух (VANILLA-UNSURE: ViaVersion подставляет замены).
use crate::block::{decode_state, BlockInfo};
use crate::generated::{b1_20_1, b1_21_11, b1_21_8, b26_3};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Таблица `старый id состояния → канонический`.
pub fn build_state_map(from: &'static [BlockInfo], to: &'static [BlockInfo]) -> Vec<u32> {
    let by_name: HashMap<&str, &BlockInfo> = to.iter().map(|b| (b.name, b)).collect();
    let total = from.last().map_or(0, |b| b.first_state + b.state_count);
    let mut out = Vec::with_capacity(total as usize);
    for state in 0..total {
        let Some((bi, idx)) = decode_state(from, state) else {
            out.push(0);
            continue;
        };
        let old = &from[bi];
        let Some(new) = by_name.get(old.name) else {
            out.push(0);
            continue;
        };
        let mut off = 0u32;
        let mut exact = true;
        for np in new.props {
            let v = old
                .props
                .iter()
                .position(|p| p.name == np.name)
                .and_then(|k| {
                    let val = old.props[k].values[idx[k] as usize];
                    np.values.iter().position(|x| *x == val)
                });
            let v = match v {
                Some(v) => v as u32,
                None => {
                    exact = false;
                    break;
                }
            };
            off = off * np.values.len() as u32 + v;
        }
        out.push(if exact {
            new.first_state + off
        } else {
            new.default_state
        });
    }
    out
}

fn table_1_21_11() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_11::BLOCKS, b26_3::BLOCKS))
}

fn table_1_21_8() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_8::BLOCKS, b26_3::BLOCKS))
}

fn table_1_20_1() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_20_1::BLOCKS, b26_3::BLOCKS))
}

/// Каноническое состояние «чёрного» блока для чужих (модовых) состояний (Forge 1.20.1).
fn modded_state() -> u32 {
    static T: OnceLock<u32> = OnceLock::new();
    *T.get_or_init(|| {
        b26_3::BLOCKS
            .iter()
            .find(|b| b.name == "black_concrete")
            .map_or(0, |b| b.default_state)
    })
}

/// Состояние блока текущей версии соединения → канон.
/// 1.20.1 (Forge): id за пределами ванильной таблицы — модовые блоки → `crate::modded`
/// (DEC-018); без дампа/неизвестные → чёрный бетон (VANILLA-UNSURE: решение владельца).
#[inline]
pub fn state_to_canon(id: u32) -> u32 {
    let v = ezzmc_protocol::version::current();
    if v.is_canon() {
        return id;
    }
    if v.protocol == 763 {
        let t = table_1_20_1();
        return match t.get(id as usize) {
            Some(&c) => c,
            // модовые: по снимку реестра сервера и дампу (crate::modded), иначе чёрный бетон
            None => {
                crate::modded::server_to_canon(id - t.len() as u32).unwrap_or_else(modded_state)
            }
        };
    }
    // 1.21.9/1.21.10 и 1.21.11 — одинаковые состояния; 1.21.7/1.21.8 — без медных блоков 1.21.9.
    let t = if v.protocol == 772 {
        table_1_21_8()
    } else {
        table_1_21_11()
    };
    t.get(id as usize).copied().unwrap_or(0)
}

/// id состояния канона → версия соединения 1.21.x (для пакетов клиента; несопоставимое —
/// состояние по умолчанию блока).
pub fn state_from_canon(id: u32) -> u32 {
    static T8: OnceLock<Vec<u32>> = OnceLock::new();
    static T11: OnceLock<Vec<u32>> = OnceLock::new();
    let t = if ezzmc_protocol::version::current().protocol == 772 {
        T8.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_8::BLOCKS))
    } else {
        T11.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_11::BLOCKS))
    };
    t.get(id as usize).copied().unwrap_or(0)
}

/// Версия соединения не каноническая?
#[inline]
pub fn is_legacy() -> bool {
    !ezzmc_protocol::version::current().is_canon()
}

/// Число состояний блока 1.20.1 по имени (без `minecraft:`) — для карты модовых id сервера.
pub fn vanilla_1_20_1_state_count(name: &str) -> Option<u32> {
    static T: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();
    T.get_or_init(|| {
        b1_20_1::BLOCKS
            .iter()
            .map(|b| (b.name, b.state_count))
            .collect()
    })
    .get(name)
    .copied()
}

/// Число ванильных состояний 1.20.1 (первый модовый id сервера Forge).
pub fn vanilla_1_20_1_states() -> u32 {
    table_1_20_1().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_for_unchanged_blocks() {
        let m = table_1_21_11();
        let name = |blocks: &'static [BlockInfo], s: u32| {
            let (i, _) = decode_state(blocks, s).unwrap();
            blocks[i].name
        };
        let mut bad = 0;
        for (s, &c) in m.iter().enumerate() {
            if name(b1_21_11::BLOCKS, s as u32) != name(b26_3::BLOCKS, c) {
                bad += 1;
            }
        }
        // блоки, которых нет в 26.3, — единицы
        assert!(bad < 200, "потеряно состояний: {bad}");
        assert_eq!(m[0], 0);
    }

    #[test]
    fn v1_20_1_table_and_modded() {
        let m = table_1_20_1();
        assert_eq!(m.len(), 24135);
        assert_eq!(m[0], 0);
        let name = |blocks: &'static [BlockInfo], s: u32| {
            let (i, _) = decode_state(blocks, s).unwrap();
            blocks[i].name
        };
        let bad = m
            .iter()
            .enumerate()
            .filter(|(s, &c)| name(b1_20_1::BLOCKS, *s as u32) != name(b26_3::BLOCKS, c))
            .count();
        assert!(bad < 400, "потеряно состояний: {bad}");
        assert_eq!(name(b26_3::BLOCKS, modded_state()), "black_concrete");
    }
}
