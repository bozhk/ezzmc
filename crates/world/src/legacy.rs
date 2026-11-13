//! Трансляция id состояний блоков старых версий в канон (26.3) — глава 18, TODO-387.
//! Совпадение по имени блока и значениям свойств; нет блока/значения → состояние по умолчанию
//! канонического блока, нет блока вовсе → воздух (VANILLA-UNSURE: ViaVersion подставляет замены).
use crate::block::{decode_state, BlockInfo};
use crate::generated::{b1_21_11, b26_3};
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

/// Состояние блока текущей версии соединения → канон.
#[inline]
pub fn state_to_canon(id: u32) -> u32 {
    let v = ezzmc_protocol::version::current();
    if v.is_canon() {
        return id;
    }
    table_1_21_11().get(id as usize).copied().unwrap_or(0)
}

/// Версия соединения не каноническая?
#[inline]
pub fn is_legacy() -> bool {
    !ezzmc_protocol::version::current().is_canon()
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
}
