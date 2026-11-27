//! Версии протокола, которые умеет клиент (глава 18, TODO-369/371).
//! Каноническая модель — последняя версия (26.3); сервер старой версии получает
//! свою таблицу id пакетов, а id реестров транслируются по именам (`remap`).
use crate::generated;
use crate::PacketInfo;

/// Описание поддерживаемой версии.
#[derive(Clone, Copy, Debug)]
pub struct Version {
    pub name: &'static str,
    pub protocol: i32,
    pub packets: &'static [PacketInfo],
    /// Реестры версии (имя записи по `protocol_id`).
    pub item: &'static [&'static str],
    pub block: &'static [&'static str],
    pub entity_type: &'static [&'static str],
    pub sound_event: &'static [&'static str],
    pub particle_type: &'static [&'static str],
}

pub const V26_3: Version = Version {
    name: "26.3",
    protocol: 777,
    packets: generated::v26_3::PACKETS,
    item: generated::r26_3::ITEM,
    block: generated::r26_3::BLOCK,
    entity_type: generated::r26_3::ENTITY_TYPE,
    sound_event: generated::r26_3::SOUND_EVENT,
    particle_type: generated::r26_3::PARTICLE_TYPE,
};

pub const V1_21_11: Version = Version {
    name: "1.21.11",
    protocol: 774,
    packets: generated::v1_21_11::PACKETS,
    item: generated::r1_21_11::ITEM,
    block: generated::r1_21_11::BLOCK,
    entity_type: generated::r1_21_11::ENTITY_TYPE,
    sound_event: generated::r1_21_11::SOUND_EVENT,
    particle_type: generated::r1_21_11::PARTICLE_TYPE,
};

/// 1.21.9/1.21.10 (протокол 773): пакеты и состояния блоков как у 1.21.11, реестры уже;
/// переводится тем же путём, что 1.21.11, со своими таблицами (TODO-373a).
pub const V1_21_10: Version = Version {
    name: "1.21.10",
    protocol: 773,
    packets: generated::v1_21_10::PACKETS,
    item: generated::r1_21_10::ITEM,
    block: generated::r1_21_10::BLOCK,
    entity_type: generated::r1_21_10::ENTITY_TYPE,
    sound_event: generated::r1_21_10::SOUND_EVENT,
    particle_type: generated::r1_21_10::PARTICLE_TYPE,
};

/// 1.21.7/1.21.8 (протокол 772).
pub const V1_21_8: Version = Version {
    name: "1.21.8",
    protocol: 772,
    packets: generated::v1_21_8::PACKETS,
    item: generated::r1_21_8::ITEM,
    block: generated::r1_21_8::BLOCK,
    entity_type: generated::r1_21_8::ENTITY_TYPE,
    sound_event: generated::r1_21_8::SOUND_EVENT,
    particle_type: generated::r1_21_8::PARTICLE_TYPE,
};

/// 1.21.6 (протокол 771): пакеты и блоки как у 1.21.8; в реестрах item/sound_event нет
/// пластинки lava_chicken (добавлена в 1.21.7).
pub const V1_21_6: Version = Version {
    name: "1.21.6",
    protocol: 771,
    packets: generated::v1_21_8::PACKETS,
    item: generated::r1_21_6::ITEM,
    block: generated::r1_21_6::BLOCK,
    entity_type: generated::r1_21_6::ENTITY_TYPE,
    sound_event: generated::r1_21_6::SOUND_EVENT,
    particle_type: generated::r1_21_6::PARTICLE_TYPE,
};

/// 1.21.5 (протокол 770): нет диалогов/путевых точек 1.21.6 (сдвиг id пакетов), нет
/// dried_ghast/happy_ghast и предметов 1.21.6 в реестрах.
pub const V1_21_5: Version = Version {
    name: "1.21.5",
    protocol: 770,
    packets: generated::v1_21_5::PACKETS,
    item: generated::r1_21_5::ITEM,
    block: generated::r1_21_5::BLOCK,
    entity_type: generated::r1_21_5::ENTITY_TYPE,
    sound_event: generated::r1_21_5::SOUND_EVENT,
    particle_type: generated::r1_21_5::PARTICLE_TYPE,
};

/// 1.21.4 (протокол 769): add_experience_orb, флаги showInTooltip в компонентах,
/// карты высот чанка в NBT, строковые правила в set_player_team.
pub const V1_21_4: Version = Version {
    name: "1.21.4",
    protocol: 769,
    packets: generated::v1_21_4::PACKETS,
    item: generated::r1_21_4::ITEM,
    block: generated::r1_21_4::BLOCK,
    entity_type: generated::r1_21_4::ENTITY_TYPE,
    sound_event: generated::r1_21_4::SOUND_EVENT,
    particle_type: generated::r1_21_4::PARTICLE_TYPE,
};

/// 1.21.2/1.21.3 (протокол 768): pick_item одним пакетом, частица trail без длительности,
/// custom_model_data — одно число, вариант лосося строкой.
pub const V1_21_3: Version = Version {
    name: "1.21.3",
    protocol: 768,
    packets: generated::v1_21_3::PACKETS,
    item: generated::r1_21_3::ITEM,
    block: generated::r1_21_3::BLOCK,
    entity_type: generated::r1_21_3::ENTITY_TYPE,
    sound_event: generated::r1_21_3::SOUND_EVENT,
    particle_type: generated::r1_21_3::PARTICLE_TYPE,
};

/// 1.21/1.21.1 (протокол 767): рецепты одним пакетом, id контейнеров байтом, старые
/// позиция игрока/телепорт/взрыв, еда с эффектами, лодки одним типом.
pub const V1_21_1: Version = Version {
    name: "1.21.1",
    protocol: 767,
    packets: generated::v1_21_1::PACKETS,
    item: generated::r1_21_1::ITEM,
    block: generated::r1_21_1::BLOCK,
    entity_type: generated::r1_21_1::ENTITY_TYPE,
    sound_event: generated::r1_21_1::SOUND_EVENT,
    particle_type: generated::r1_21_1::PARTICLE_TYPE,
};

/// 1.20.5/1.20.6 (протокол 766): как 1.21.1, но без синхронизации реестров enchantment/
/// painting_variant/jukebox_song, модификаторы атрибутов с UUID, use_item без поворота.
pub const V1_20_6: Version = Version {
    name: "1.20.6",
    protocol: 766,
    packets: generated::v1_20_6::PACKETS,
    item: generated::r1_20_6::ITEM,
    block: generated::r1_20_6::BLOCK,
    entity_type: generated::r1_20_6::ENTITY_TYPE,
    sound_event: generated::r1_20_6::SOUND_EVENT,
    particle_type: generated::r1_20_6::PARTICLE_TYPE,
};

/// 1.20.3/1.20.4 (протокол 765): Configuration-фаза без known_packs/cookie/transfer, предметы
/// со стековым NBT вместо компонентов, registry_data одним NBT-деревом.
pub const V1_20_4: Version = Version {
    name: "1.20.4",
    protocol: 765,
    packets: generated::v1_20_4::PACKETS,
    item: generated::r1_20_4::ITEM,
    block: generated::r1_20_4::BLOCK,
    entity_type: generated::r1_20_4::ENTITY_TYPE,
    sound_event: generated::r1_20_4::SOUND_EVENT,
    particle_type: generated::r1_20_4::PARTICLE_TYPE,
};

/// 1.20.2 (протокол 764): как 1.20.4, но текст в пакетах JSON-строкой, старые set_score/explode,
/// пакеты ресурсов без UUID; реестры без экспериментальных блоков/предметов 1.20.3.
pub const V1_20_2: Version = Version {
    name: "1.20.2",
    protocol: 764,
    packets: generated::v1_20_2::PACKETS,
    item: generated::r1_20_2::ITEM,
    block: generated::r1_20_2::BLOCK,
    entity_type: generated::r1_20_2::ENTITY_TYPE,
    sound_event: generated::r1_20_2::SOUND_EVENT,
    particle_type: generated::r1_20_2::PARTICLE_TYPE,
};

/// Forge/ваниль 1.20.1 (протокол 763): нет Configuration-фазы, реестры внутри Play `login`.
pub const V1_20_1: Version = Version {
    name: "1.20.1",
    protocol: 763,
    packets: generated::v1_20_1::PACKETS,
    item: generated::r1_20_1::ITEM,
    block: generated::r1_20_1::BLOCK,
    entity_type: generated::r1_20_1::ENTITY_TYPE,
    sound_event: generated::r1_20_1::SOUND_EVENT,
    particle_type: generated::r1_20_1::PARTICLE_TYPE,
};

/// 1.19.4 (протокол 762): тот же набор и порядок пакетов, что у 1.20.1; тела переводятся
/// в формат 1.20.1 (`legacy::from_1_19_4`) и дальше идут путём 1.20.1.
pub const V1_19_4: Version = Version {
    name: "1.19.4",
    protocol: 762,
    packets: generated::v1_20_1::PACKETS,
    item: generated::r1_19_4::ITEM,
    block: generated::r1_19_4::BLOCK,
    entity_type: generated::r1_19_4::ENTITY_TYPE,
    sound_event: generated::r1_19_4::SOUND_EVENT,
    particle_type: generated::r1_19_4::PARTICLE_TYPE,
};

/// 1.18.2 (протокол 758): чат без подписей, DimensionType NBT в login/respawn, add_mob/add_painting;
/// тела → 1.19 (`v1201::from_1_18_2`) → 1.19.2 → … → 1.20.1.
pub const V1_18_2: Version = Version {
    name: "1.18.2",
    protocol: 758,
    packets: generated::v1_18_2::PACKETS,
    item: generated::r1_18_2::ITEM,
    block: generated::r1_18_2::BLOCK,
    entity_type: generated::r1_18_2::ENTITY_TYPE,
    sound_event: generated::r1_18_2::SOUND_EVENT,
    particle_type: generated::r1_18_2::PARTICLE_TYPE,
};

/// 1.17.1 (протокол 756).
pub const V1_17_1: Version = Version {
    name: "1.17.1",
    protocol: 756,
    packets: generated::v1_17_1::PACKETS,
    item: generated::r1_17_1::ITEM,
    block: generated::r1_17_1::BLOCK,
    entity_type: generated::r1_17_1::ENTITY_TYPE,
    sound_event: generated::r1_17_1::SOUND_EVENT,
    particle_type: generated::r1_17_1::PARTICLE_TYPE,
};

/// 1.16.5 / 1.16.4 (протокол 754).
pub const V1_16_5: Version = Version {
    name: "1.16.5",
    protocol: 754,
    packets: generated::v1_16_5::PACKETS,
    item: generated::r1_16_5::ITEM,
    block: generated::r1_16_5::BLOCK,
    entity_type: generated::r1_16_5::ENTITY_TYPE,
    sound_event: generated::r1_16_5::SOUND_EVENT,
    particle_type: generated::r1_16_5::PARTICLE_TYPE,
};

/// 1.16.3 (753) и 1.16.2 (751): регистры, блоки и порядок пакетов совпадают с 1.16.5
/// (отличается только `edit_book`, клиент его старым серверам не шлёт).
pub const V1_16_3: Version = Version {
    name: "1.16.3",
    protocol: 753,
    ..V1_16_5
};

pub const V1_16_2: Version = Version {
    name: "1.16.2",
    protocol: 751,
    ..V1_16_5
};

/// 1.16.1 (736) и 1.16 (735): таблицы совпадают между собой; тела → 1.16.2 (`v1165::from_1_16_1`).
pub const V1_16_1: Version = Version {
    name: "1.16.1",
    protocol: 736,
    packets: generated::v1_16_1::PACKETS,
    item: generated::r1_16_1::ITEM,
    block: generated::r1_16_1::BLOCK,
    entity_type: generated::r1_16_1::ENTITY_TYPE,
    sound_event: generated::r1_16_1::SOUND_EVENT,
    particle_type: generated::r1_16_1::PARTICLE_TYPE,
};

pub const V1_16: Version = Version {
    name: "1.16",
    protocol: 735,
    ..V1_16_1
};

/// 1.15.2 (578): тела 1.15–1.15.2 → 1.16.1 (`v1152::from_1_15_2`).
pub const V1_15_2: Version = Version {
    name: "1.15.2",
    protocol: 578,
    packets: generated::v1_15_2::PACKETS,
    item: generated::r1_15_2::ITEM,
    block: generated::r1_15_2::BLOCK,
    entity_type: generated::r1_15_2::ENTITY_TYPE,
    sound_event: generated::r1_15_2::SOUND_EVENT,
    particle_type: generated::r1_15_2::PARTICLE_TYPE,
};

/// 1.15.1 (575) и 1.15 (573): таблицы совпадают с 1.15.2.
pub const V1_15_1: Version = Version {
    name: "1.15.1",
    protocol: 575,
    ..V1_15_2
};

pub const V1_15: Version = Version {
    name: "1.15",
    protocol: 573,
    ..V1_15_2
};

/// 1.17 (протокол 755): регистры и состояния блоков совпадают с 1.17.1.
pub const V1_17: Version = Version {
    name: "1.17",
    protocol: 755,
    packets: generated::v1_17::PACKETS,
    item: generated::r1_17_1::ITEM,
    block: generated::r1_17_1::BLOCK,
    entity_type: generated::r1_17_1::ENTITY_TYPE,
    sound_event: generated::r1_17_1::SOUND_EVENT,
    particle_type: generated::r1_17_1::PARTICLE_TYPE,
};

/// 1.18/1.18.1 (протокол 757): провод совпадает с 1.18.2 (порядок пакетов, blocks.json, регистры
/// клиента; id эффектов — byte вместо varint, для id < 128 байты те же), путь тот же, что у 1.18.2.
pub const V1_18_1: Version = Version {
    name: "1.18.1",
    protocol: 757,
    ..V1_18_2
};

/// 1.19 (протокол 759): подписанный чат v1, system_chat с typeId; тела → 1.19.2 (`v1201::from_1_19`)
/// → 1.19.3 → 1.19.4 → 1.20.1.
pub const V1_19: Version = Version {
    name: "1.19",
    protocol: 759,
    packets: generated::v1_19::PACKETS,
    item: generated::r1_19::ITEM,
    block: generated::r1_19::BLOCK,
    entity_type: generated::r1_19::ENTITY_TYPE,
    sound_event: generated::r1_19::SOUND_EVENT,
    particle_type: generated::r1_19::PARTICLE_TYPE,
};

/// 1.19.1/1.19.2 (протокол 760): подписанный чат v2, `player_info` одним пакетом; тела → 1.19.3
/// (`v1201::from_1_19_2`) → 1.19.4 → 1.20.1.
pub const V1_19_2: Version = Version {
    name: "1.19.2",
    protocol: 760,
    packets: generated::v1_19_2::PACKETS,
    item: generated::r1_19_2::ITEM,
    block: generated::r1_19_2::BLOCK,
    entity_type: generated::r1_19_2::ENTITY_TYPE,
    sound_event: generated::r1_19_2::SOUND_EVENT,
    particle_type: generated::r1_19_2::PARTICLE_TYPE,
};

/// 1.19.3 (протокол 761): без bundle/chunks_biomes/damage_event/hurt_animation; тела → 1.19.4 →
/// 1.20.1 (`v1201::from_1_19_4`), дальше путём 1.20.1.
pub const V1_19_3: Version = Version {
    name: "1.19.3",
    protocol: 761,
    packets: generated::v1_19_3::PACKETS,
    item: generated::r1_19_3::ITEM,
    block: generated::r1_19_3::BLOCK,
    entity_type: generated::r1_19_3::ENTITY_TYPE,
    sound_event: generated::r1_19_3::SOUND_EVENT,
    particle_type: generated::r1_19_3::PARTICLE_TYPE,
};

/// Каноническая версия (на ней написан весь клиент).
pub const CANON: Version = V26_3;

pub const SUPPORTED: &[Version] = &[
    V26_3, V1_21_11, V1_21_10, V1_21_8, V1_21_6, V1_21_5, V1_21_4, V1_21_3, V1_21_1, V1_20_6,
    V1_20_4, V1_20_2, V1_20_1, V1_19_4, V1_19_3, V1_19_2, V1_19, V1_18_2, V1_18_1, V1_17_1, V1_17,
    V1_16_5, V1_16_3, V1_16_2, V1_16_1, V1_16, V1_15_2, V1_15_1, V1_15,
];

/// Версия по protocol number из Status-ответа.
pub fn by_protocol(protocol: i32) -> Option<Version> {
    SUPPORTED.iter().copied().find(|v| v.protocol == protocol)
}

static CURRENT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(777);

/// Версия текущего соединения (одно соединение на клиент).
pub fn current() -> Version {
    by_protocol(CURRENT.load(std::sync::atomic::Ordering::Relaxed)).unwrap_or(CANON)
}

pub fn set_current(v: Version) {
    CURRENT.store(v.protocol, std::sync::atomic::Ordering::Relaxed);
}

impl Version {
    pub fn is_canon(&self) -> bool {
        self.protocol == CANON.protocol
    }

    /// 1.18–1.20.1: без Configuration-фазы, реестры в Play `login`; один путь (`v1201`).
    pub fn is_pre_config(&self) -> bool {
        matches!(self.protocol, 573..=763)
    }

    /// 1.20.2–1.21.11: переводятся одним путём (`legacy::clientbound_1_21_11`).
    pub fn is_1_21_late(&self) -> bool {
        matches!(self.protocol, 764..=774)
    }
}

/// Переименования записей реестров между версиями (в обе стороны).
fn renamed(name: &str) -> &str {
    match name {
        // 1.17: grass_path → dirt_path
        "grass_path" => "dirt_path",
        "dirt_path" => "grass_path",
        n => n,
    }
}

/// Таблица «id реестра версии сервера → id канона» по именам записей;
/// отсутствующие в каноне записи → `None` (вызывающий подставляет замену).
pub fn build_remap(from: &[&str], to: &[&str]) -> Vec<Option<u32>> {
    let idx: std::collections::HashMap<&str, u32> =
        to.iter().enumerate().map(|(i, n)| (*n, i as u32)).collect();
    from.iter()
        .map(|n| idx.get(n).or_else(|| idx.get(renamed(n))).copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_versions() {
        assert_eq!(by_protocol(777).map(|v| v.name), Some("26.3"));
        assert_eq!(by_protocol(774).map(|v| v.name), Some("1.21.11"));
        assert_eq!(by_protocol(773).map(|v| v.name), Some("1.21.10"));
        assert_eq!(by_protocol(772).map(|v| v.name), Some("1.21.8"));
        assert_eq!(by_protocol(771).map(|v| v.name), Some("1.21.6"));
        assert_eq!(by_protocol(770).map(|v| v.name), Some("1.21.5"));
        assert_eq!(by_protocol(769).map(|v| v.name), Some("1.21.4"));
        assert_eq!(by_protocol(768).map(|v| v.name), Some("1.21.3"));
        assert_eq!(by_protocol(767).map(|v| v.name), Some("1.21.1"));
        assert_eq!(by_protocol(766).map(|v| v.name), Some("1.20.6"));
        assert!(by_protocol(1).is_none());
    }

    #[test]
    fn remap_items_mostly_resolves() {
        let m = build_remap(V1_21_11.item, CANON.item);
        let miss = m.iter().filter(|x| x.is_none()).count();
        assert!(miss * 50 < m.len(), "слишком много потерянных: {miss}");
        let stone_old = V1_21_11.item.iter().position(|n| *n == "diamond").unwrap();
        let stone_new = CANON.item.iter().position(|n| *n == "diamond").unwrap() as u32;
        assert_eq!(m[stone_old], Some(stone_new));
    }
}
