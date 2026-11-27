//! 1.19.1/1.19.2 (760): подписанный чат v2, `player_info` одним пакетом, respawn с bool
//! keepAllPlayerData → формат 1.19.3. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{canon_name, from_1_19_2, play_from_canon, play_to_canon};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, State};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_19_2);
}

fn id(dir: Direction, name: &str) -> i32 {
    ezzmc_protocol::version::V1_19_2
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == dir && p.name == name)
        .unwrap()
        .id as i32
}

#[test]
fn packet_ids_1_19_2() {
    // VANILLA: ConnectionProtocol (1.19.2): login 0x25, player_chat 0x33, player_info 0x37,
    // system_chat 0x62; serverbound chat 0x05, chat_command 0x04.
    assert_eq!(id(Direction::Clientbound, "login"), 0x25);
    assert_eq!(id(Direction::Clientbound, "player_chat"), 0x33);
    assert_eq!(id(Direction::Clientbound, "player_info"), 0x37);
    assert_eq!(id(Direction::Clientbound, "system_chat"), 0x62);
    assert_eq!(id(Direction::Serverbound, "chat_command"), 0x04);
    assert_eq!(id(Direction::Serverbound, "chat"), 0x05);
}

#[test]
fn respawn_keep_flag_to_byte() {
    set();
    for (keep, want) in [(false, 2u8), (true, 3u8)] {
        let mut w = Writer::new();
        w.string("minecraft:overworld")
            .string("minecraft:overworld")
            .i64(7)
            .u8(0)
            .u8(255)
            .bool(false)
            .bool(false);
        let head = w.into_vec();
        let mut w = Writer::new();
        w.bytes(&head).bool(keep).bool(false);
        let out = from_1_19_2("respawn", &w.into_vec()).unwrap().unwrap();
        assert_eq!(&out[..head.len()], &head[..]);
        assert_eq!(&out[head.len()..], &[want, 0]);
    }
}

#[test]
fn player_info_add_and_remove() {
    set();
    let mut w = Writer::new();
    w.var_int(0).var_int(1).bytes(&[7; 16]).string("Bob");
    w.var_int(1).string("textures").string("v").bool(false);
    w.var_int(1).var_int(42).bool(false);
    // ключ чата: instant, byte[] ключ, byte[] подпись
    w.bool(true)
        .i64(1)
        .var_int(2)
        .bytes(&[1, 2])
        .var_int(1)
        .bytes(&[3]);
    let body = w.into_vec();
    assert_eq!(canon_name("player_info", &body), "player_info_update");
    let out = from_1_19_2("player_info", &body).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.u8().unwrap(), 0b11_1101);
    assert_eq!(r.var_int().unwrap(), 1);
    assert_eq!(r.bytes(16).unwrap(), &[7; 16]);
    assert_eq!(r.string(16).unwrap(), "Bob");
    assert_eq!(r.var_int().unwrap(), 1);
    assert_eq!(r.string(64).unwrap(), "textures");
    assert_eq!(r.string(64).unwrap(), "v");
    assert!(!r.bool().unwrap());
    assert_eq!(r.var_int().unwrap(), 1); // game mode
    assert!(r.bool().unwrap()); // listed
    assert_eq!(r.var_int().unwrap(), 42);
    assert!(!r.bool().unwrap());
    assert!(r.is_empty());
    // весь путь до канона разбирается
    assert!(play_to_canon("player_info", &body).unwrap().is_some());

    let mut w = Writer::new();
    w.var_int(4).var_int(1).bytes(&[9; 16]);
    let body = w.into_vec();
    assert_eq!(canon_name("player_info", &body), "player_info_remove");
    let out = from_1_19_2("player_info", &body).unwrap().unwrap();
    let mut want = vec![1];
    want.extend([9; 16]);
    assert_eq!(out, want);
}

#[test]
fn player_chat_to_system_text() {
    set();
    let mut w = Writer::new();
    w.bool(true).var_int(2).bytes(&[0, 0]); // prev подпись
    w.bytes(&[5; 16]); // sender
    w.var_int(3).bytes(&[1, 1, 1]); // подпись заголовка
    w.string("hi").bool(false).i64(10).i64(20);
    w.var_int(1).bytes(&[6; 16]).var_int(1).bytes(&[0]); // last seen
    w.bool(false); // unsigned
    w.var_int(0); // FilterMask PASS_THROUGH
    w.var_int(0).string(r#"{"text":"Bob"}"#).bool(false);
    let body = w.into_vec();
    assert_eq!(canon_name("player_chat", &body), "system_chat");
    let out = play_to_canon("player_chat", &body).unwrap().unwrap();
    assert!(!out.is_empty());
}

#[test]
fn chat_without_signature() {
    set();
    let sid = id(Direction::Serverbound, "chat");
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
    assert_eq!(r.var_int().unwrap(), 0); // пустая подпись
    assert!(!r.bool().unwrap()); // signedPreview
    assert_eq!(r.var_int().unwrap(), 0); // last seen
    assert!(!r.bool().unwrap()); // lastReceived
    assert!(r.is_empty());
}
