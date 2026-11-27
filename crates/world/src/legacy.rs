//! Трансляция id состояний блоков старых версий в канон (26.3) — глава 18, TODO-387.
//! Совпадение по имени блока и значениям свойств; нет блока/значения → состояние по умолчанию
//! канонического блока, нет блока вовсе → воздух (VANILLA-UNSURE: ViaVersion подставляет замены).
use crate::block::{decode_state, BlockInfo};
use crate::generated::{
    b1_13, b1_13_2, b1_14_4, b1_15_2, b1_16_1, b1_16_5, b1_17_1, b1_18_2, b1_19, b1_19_2, b1_19_3,
    b1_19_4, b1_20_1, b1_20_2, b1_20_4, b1_21_1, b1_21_11, b1_21_3, b1_21_4, b1_21_5, b1_21_8,
    b26_3, flattening,
};
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
        // 1.17: grass_path → dirt_path; непустой котёл стал water_cauldron.
        let level = old
            .props
            .iter()
            .position(|p| p.name == "level")
            .map(|k| old.props[k].values[idx[k] as usize]);
        let name = match old.name {
            "grass" => "short_grass",
            "short_grass" if !by_name.contains_key("short_grass") => "grass",
            "sign" => "oak_sign",
            "oak_sign" if !by_name.contains_key("oak_sign") => "sign",
            "wall_sign" => "oak_wall_sign",
            "oak_wall_sign" if !by_name.contains_key("oak_wall_sign") => "wall_sign",
            // 1.14: прежняя `stone_slab` стала `smooth_stone_slab`.
            "stone_slab" if !from.iter().any(|b| b.name == "smooth_stone_slab") => {
                "smooth_stone_slab"
            }
            "smooth_stone_slab" if !by_name.contains_key("smooth_stone_slab") => "stone_slab",
            "grass_path" => "dirt_path",
            "dirt_path" if !by_name.contains_key("dirt_path") => "grass_path",
            "cauldron" if level.is_some_and(|l| l != "0") => "water_cauldron",
            "water_cauldron" if !by_name.contains_key("water_cauldron") => "cauldron",
            n => n,
        };
        let Some(new) = by_name.get(name) else {
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

fn table_1_21_4() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_4::BLOCKS, b26_3::BLOCKS))
}

fn table_1_21_1() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_1::BLOCKS, b26_3::BLOCKS))
}

fn table_1_20_2() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_20_2::BLOCKS, b26_3::BLOCKS))
}

fn table_1_20_4() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_20_4::BLOCKS, b26_3::BLOCKS))
}

fn table_1_21_3() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_3::BLOCKS, b26_3::BLOCKS))
}

fn table_1_21_5() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_21_5::BLOCKS, b26_3::BLOCKS))
}

fn table_1_18_2() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_18_2::BLOCKS, b26_3::BLOCKS))
}

fn table_1_16_5() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_16_5::BLOCKS, b26_3::BLOCKS))
}

fn table_1_13() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_13::BLOCKS, b26_3::BLOCKS))
}

fn table_1_13_2() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_13_2::BLOCKS, b26_3::BLOCKS))
}

/// ≤1.12.2: состояние — `id << 4 | meta`; через таблицу Flattening и состояния 1.13.2.
fn table_1_12() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| {
        let flat = table_1_13_2();
        flattening::BLOCKS
            .iter()
            .map(|&s| flat.get(usize::from(s)).copied().unwrap_or(0))
            .collect()
    })
}

fn table_1_14_4() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_14_4::BLOCKS, b26_3::BLOCKS))
}

fn table_1_15_2() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_15_2::BLOCKS, b26_3::BLOCKS))
}

fn table_1_16_1() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_16_1::BLOCKS, b26_3::BLOCKS))
}

fn table_1_17_1() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_17_1::BLOCKS, b26_3::BLOCKS))
}

fn table_1_19() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_19::BLOCKS, b26_3::BLOCKS))
}

fn table_1_19_2() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_19_2::BLOCKS, b26_3::BLOCKS))
}

fn table_1_19_3() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_19_3::BLOCKS, b26_3::BLOCKS))
}

fn table_1_19_4() -> &'static [u32] {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| build_state_map(b1_19_4::BLOCKS, b26_3::BLOCKS))
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
    // 1.21.9/1.21.10 и 1.21.11 — одинаковые состояния; 1.21.6–1.21.8 — без медных блоков 1.21.9;
    // 1.21.5 — ещё и без dried_ghast.
    let t = match v.protocol {
        757 | 758 => table_1_18_2(),
        335..=340 => table_1_12(),
        393 => table_1_13(),
        401 | 404 => table_1_13_2(),
        477..=498 => table_1_14_4(),
        573..=578 => table_1_15_2(),
        735 | 736 => table_1_16_1(),
        751..=754 => table_1_16_5(),
        755 | 756 => table_1_17_1(),
        759 => table_1_19(),
        760 => table_1_19_2(),
        761 => table_1_19_3(),
        762 => table_1_19_4(),
        // 1.20.5/1.20.6: реестр состояний блоков совпадает с 1.21.1.
        764 => table_1_20_2(),
        765 => table_1_20_4(),
        766 | 767 => table_1_21_1(),
        768 => table_1_21_3(),
        769 => table_1_21_4(),
        770 => table_1_21_5(),
        771 | 772 => table_1_21_8(),
        _ => table_1_21_11(),
    };
    t.get(id as usize).copied().unwrap_or(0)
}

/// id состояния канона → версия соединения 1.21.x (для пакетов клиента; несопоставимое —
/// состояние по умолчанию блока).
pub fn state_from_canon(id: u32) -> u32 {
    static T8: OnceLock<Vec<u32>> = OnceLock::new();
    static T11: OnceLock<Vec<u32>> = OnceLock::new();
    static T5: OnceLock<Vec<u32>> = OnceLock::new();
    static T4: OnceLock<Vec<u32>> = OnceLock::new();
    static T3: OnceLock<Vec<u32>> = OnceLock::new();
    static T1: OnceLock<Vec<u32>> = OnceLock::new();
    static T04: OnceLock<Vec<u32>> = OnceLock::new();
    static T02: OnceLock<Vec<u32>> = OnceLock::new();
    static T01: OnceLock<Vec<u32>> = OnceLock::new();
    static T194: OnceLock<Vec<u32>> = OnceLock::new();
    static T193: OnceLock<Vec<u32>> = OnceLock::new();
    static T192: OnceLock<Vec<u32>> = OnceLock::new();
    static T19: OnceLock<Vec<u32>> = OnceLock::new();
    static T1182: OnceLock<Vec<u32>> = OnceLock::new();
    static T1171: OnceLock<Vec<u32>> = OnceLock::new();
    static T1165: OnceLock<Vec<u32>> = OnceLock::new();
    static T1144: OnceLock<Vec<u32>> = OnceLock::new();
    static T113: OnceLock<Vec<u32>> = OnceLock::new();
    static T1132: OnceLock<Vec<u32>> = OnceLock::new();
    static T1152: OnceLock<Vec<u32>> = OnceLock::new();
    static T1161: OnceLock<Vec<u32>> = OnceLock::new();
    let t = match ezzmc_protocol::version::current().protocol {
        393 => T113.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_13::BLOCKS)),
        401 | 404 => T1132.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_13_2::BLOCKS)),
        477..=498 => T1144.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_14_4::BLOCKS)),
        573..=578 => T1152.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_15_2::BLOCKS)),
        735 | 736 => T1161.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_16_1::BLOCKS)),
        751..=754 => T1165.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_16_5::BLOCKS)),
        755 | 756 => T1171.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_17_1::BLOCKS)),
        759 => T19.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_19::BLOCKS)),
        757 | 758 => T1182.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_18_2::BLOCKS)),
        760 => T192.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_19_2::BLOCKS)),
        761 => T193.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_19_3::BLOCKS)),
        762 => T194.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_19_4::BLOCKS)),
        763 => T01.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_20_1::BLOCKS)),
        764 => T02.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_20_2::BLOCKS)),
        765 => T04.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_20_4::BLOCKS)),
        766 | 767 => T1.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_1::BLOCKS)),
        768 => T3.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_3::BLOCKS)),
        769 => T4.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_4::BLOCKS)),
        770 => T5.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_5::BLOCKS)),
        771 | 772 => T8.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_8::BLOCKS)),
        _ => T11.get_or_init(|| build_state_map(b26_3::BLOCKS, b1_21_11::BLOCKS)),
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

#[cfg(test)]
mod rename_tests {
    use super::*;

    fn map_name(from: &'static [BlockInfo], name: &str) -> &'static str {
        let b = from.iter().find(|b| b.name == name).expect("block");
        let canon = build_state_map(from, b26_3::BLOCKS)[b.default_state as usize];
        let (i, _) = decode_state(b26_3::BLOCKS, canon).expect("state");
        b26_3::BLOCKS[i].name
    }

    #[test]
    fn old_block_names_resolve() {
        assert_eq!(map_name(b1_20_1::BLOCKS, "grass"), "short_grass");
        assert_eq!(map_name(b1_13_2::BLOCKS, "sign"), "oak_sign");
        assert_eq!(map_name(b1_13_2::BLOCKS, "wall_sign"), "oak_wall_sign");
        assert_eq!(map_name(b1_13_2::BLOCKS, "grass_path"), "dirt_path");
        assert_eq!(map_name(b1_13_2::BLOCKS, "stone_slab"), "smooth_stone_slab");
        assert_eq!(map_name(b1_14_4::BLOCKS, "stone_slab"), "stone_slab");
    }

    #[test]
    fn all_1_13_blocks_exist_in_canon() {
        for blocks in [b1_13::BLOCKS, b1_13_2::BLOCKS] {
            for b in blocks.iter().filter(|b| !b.name.ends_with("air")) {
                assert_ne!(map_name(blocks, b.name), "air", "{}", b.name);
            }
        }
    }

    #[test]
    fn flattening_table() {
        use crate::block::BlockState;
        let name = |old: usize| BlockState(super::table_1_12()[old]).name();
        assert_eq!(name(0), "air");
        assert_eq!(name(1 << 4 | 1), "granite");
        assert_eq!(name(2 << 4), "grass_block");
        // доски: мета 3 — тропические
        assert_eq!(name(5 << 4 | 3), "jungle_planks");
        assert_eq!(name(35 << 4 | 14), "red_wool");
        // мета без записи берёт мету 0
        assert_eq!(name(1 << 4 | 15), "stone");
    }
}
