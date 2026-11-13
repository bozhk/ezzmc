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

/// Каноническая версия (на ней написан весь клиент).
pub const CANON: Version = V26_3;

pub const SUPPORTED: &[Version] = &[V26_3, V1_21_11];

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
}

/// Таблица «id реестра версии сервера → id канона» по именам записей;
/// отсутствующие в каноне записи → `None` (вызывающий подставляет замену).
pub fn build_remap(from: &[&str], to: &[&str]) -> Vec<Option<u32>> {
    let idx: std::collections::HashMap<&str, u32> =
        to.iter().enumerate().map(|(i, n)| (*n, i as u32)).collect();
    from.iter().map(|n| idx.get(n).copied()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_versions() {
        assert_eq!(by_protocol(777).map(|v| v.name), Some("26.3"));
        assert_eq!(by_protocol(774).map(|v| v.name), Some("1.21.11"));
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
