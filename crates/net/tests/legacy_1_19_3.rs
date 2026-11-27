//! 1.19.3 (761): свои id пакетов (нет bundle/chunks_biomes/damage_event/hurt_animation,
//! chat_session_update в конце serverbound-списка), тела — как у 1.19.4. Отдельный бинарь.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{from_1_19_4, play_from_canon};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, State};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_19_3);
}

fn id(dir: Direction, name: &str) -> i32 {
    ezzmc_protocol::version::V1_19_3
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == dir && p.name == name)
        .unwrap()
        .id as i32
}

#[test]
fn packet_ids_1_19_3() {
    // VANILLA: ConnectionProtocol (1.19.3): add_entity — первый клиентский пакет (bundle нет),
    // login 0x24, chat_session_update — 0x20 serverbound.
    assert_eq!(id(Direction::Clientbound, "add_entity"), 0x00);
    assert_eq!(id(Direction::Clientbound, "login"), 0x24);
    assert_eq!(id(Direction::Serverbound, "chat_session_update"), 0x20);
    assert_eq!(id(Direction::Serverbound, "sign_update"), 0x2E);
    let names: Vec<_> = ezzmc_protocol::version::V1_19_3
        .packets
        .iter()
        .map(|p| p.name)
        .collect();
    for n in [
        "bundle_delimiter",
        "chunks_biomes",
        "damage_event",
        "hurt_animation",
    ] {
        assert!(!names.contains(&n), "{n}");
    }
}

#[test]
fn sign_update_uses_1_19_3_id() {
    set();
    let sid = id(Direction::Serverbound, "sign_update");
    let mut w = Writer::new();
    w.i64(5).bool(true);
    for s in ["1", "2", "3", "4"] {
        w.string(s);
    }
    let out = play_from_canon(sid, &w.into_vec()).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].0, sid);
    let mut r = Reader::new(&out[0].1);
    assert_eq!(r.i64().unwrap(), 5);
    assert_eq!(r.string(384).unwrap(), "1");
}

#[test]
fn respawn_gets_portal_cooldown() {
    set();
    let out = from_1_19_4("respawn", &[1, 2, 3]).unwrap().unwrap();
    assert_eq!(out, vec![1, 2, 3, 0]);
}
