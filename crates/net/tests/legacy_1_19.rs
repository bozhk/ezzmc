//! 1.19 (759): подписанный чат v1, system_chat с typeId, chat/chat_command с SaltSignaturePair →
//! формат 1.19.2. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{canon_name, from_1_19, play_from_canon, play_to_canon};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, State};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_19);
}

fn id(dir: Direction, name: &str) -> i32 {
    ezzmc_protocol::version::V1_19
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == dir && p.name == name)
        .unwrap()
        .id as i32
}

#[test]
fn packet_ids_1_19() {
    // VANILLA: ConnectionProtocol (1.19): login 0x23, player_chat 0x30, player_info 0x34,
    // system_chat 0x5F; serverbound chat_command 0x03, chat 0x04.
    assert_eq!(id(Direction::Clientbound, "login"), 0x23);
    assert_eq!(id(Direction::Clientbound, "player_chat"), 0x30);
    assert_eq!(id(Direction::Clientbound, "player_info"), 0x34);
    assert_eq!(id(Direction::Clientbound, "system_chat"), 0x5F);
    assert_eq!(id(Direction::Serverbound, "chat_command"), 0x03);
    assert_eq!(id(Direction::Serverbound, "chat"), 0x04);
}

#[test]
fn system_chat_type_to_overlay() {
    set();
    for (kind, overlay) in [(1, false), (2, true)] {
        let mut w = Writer::new();
        w.string(r#"{"text":"x"}"#).var_int(kind);
        let out = from_1_19("system_chat", &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.string(64).unwrap(), r#"{"text":"x"}"#);
        assert_eq!(r.bool().unwrap(), overlay);
        assert!(r.is_empty());
    }
}

#[test]
fn player_chat_v1_reaches_canon() {
    set();
    let mut w = Writer::new();
    w.string(r#"{"text":"hi"}"#).bool(false).var_int(0);
    w.bytes(&[5; 16]).string(r#"{"text":"Bob"}"#).bool(false);
    w.i64(10).i64(20).var_int(2).bytes(&[1, 2]);
    let body = w.into_vec();
    assert_eq!(canon_name("player_chat", &body), "system_chat");
    let out = play_to_canon("player_chat", &body).unwrap().unwrap();
    assert!(!out.is_empty());
}

#[test]
fn chat_and_command_without_signature() {
    set();
    for name in ["chat", "chat_command"] {
        let sid = id(Direction::Serverbound, name);
        let mut w = Writer::new();
        w.string("hey")
            .i64(1)
            .i64(2)
            .bool(false)
            .var_int(0)
            .bytes(&[0, 0, 0])
            .u8(0);
        let out = play_from_canon(sid, &w.into_vec()).unwrap();
        assert_eq!(out.len(), 1);
        let mut r = Reader::new(&out[0].1);
        assert_eq!(r.string(256).unwrap(), "hey");
        assert_eq!(r.i64().unwrap(), 1);
        assert_eq!(r.i64().unwrap(), 2);
        assert_eq!(r.var_int().unwrap(), 0);
        assert!(!r.bool().unwrap());
        assert!(r.is_empty());
    }
}
