//! Кодеки пакетов, типы протокола, трансляторы версий.
pub mod error;
pub mod generated;
pub mod item;
pub mod nbt;
pub mod text;
pub mod types;
pub mod wire;

pub use error::{ProtocolError, ProtocolResult};

/// Состояние соединения.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

/// Направление пакета.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Clientbound,
    Serverbound,
}

/// Запись таблицы id пакетов (генерируется `cargo xtask gen-protocol`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PacketInfo {
    pub state: State,
    pub dir: Direction,
    pub id: u32,
    pub name: &'static str,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn generated_tables_are_sane() {
        let _ = generated::v26_3::PACKETS; // существует
        for p in generated::v26_3::PACKETS {
            let _ = p;
        }
        let keep_alive = generated::v26_3::PACKETS
            .iter()
            .find(|p| {
                p.state == State::Configuration
                    && p.dir == Direction::Clientbound
                    && p.name == "keep_alive"
            })
            .unwrap();
        assert_eq!(keep_alive.id, 4); // reports/packets.json 26.3
        use generated::r26_3 as r;
        assert_eq!(r::BLOCK[1], "stone"); // reports/registries.json 26.3
        assert_eq!(r::ITEM[1012], "diamond");
        assert_eq!(r::ENTITY_TYPE[159], "player");
    }
}
