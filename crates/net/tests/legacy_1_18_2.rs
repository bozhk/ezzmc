//! 1.18.2 (758): чат без подписей, DimensionType NBT в login/respawn, add_mob/add_painting,
//! block_break_ack → формат 1.19. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{canon_name, from_1_18_2, play_from_canon};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, State};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_18_2);
}

fn sid(name: &str) -> i32 {
    ezzmc_protocol::version::V1_18_2
        .packets
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.name == name)
        .unwrap()
        .id as i32
}

fn dim(height: i32) -> Tag {
    Tag::Compound(vec![("height".into(), Tag::Int(height))])
}

#[test]
fn login_and_respawn_dimension_key() {
    set();
    let entry = |n: &str, h| {
        Tag::Compound(vec![
            ("name".into(), Tag::String(n.into())),
            ("id".into(), Tag::Int(0)),
            ("element".into(), dim(h)),
        ])
    };
    let codec = Tag::Compound(vec![(
        "minecraft:dimension_type".into(),
        Tag::Compound(vec![(
            "value".into(),
            Tag::List(vec![
                entry("minecraft:overworld", 384),
                entry("minecraft:the_nether", 256),
            ]),
        )]),
    )]);
    let mut w = Writer::new();
    w.i32(7)
        .bool(false)
        .u8(1)
        .u8(255)
        .var_int(1)
        .string("minecraft:overworld");
    let mut b = Vec::new();
    nbt::write_named(&mut b, "", &codec).unwrap();
    w.bytes(&b);
    let mut d = Vec::new();
    nbt::write_named(&mut d, "", &dim(384)).unwrap();
    w.bytes(&d).string("minecraft:overworld");
    let tail = [1u8, 2, 3];
    w.bytes(&tail);
    let out = from_1_18_2("login", &w.into_vec()).unwrap().unwrap();
    assert!(out.ends_with(&[1, 2, 3, 0]));
    let mut r = Reader::new(&out);
    r.bytes(4 + 1 + 2).unwrap();
    r.var_int().unwrap();
    r.string(64).unwrap();
    nbt::read_named(&mut r).unwrap();
    assert_eq!(r.string(64).unwrap(), "minecraft:overworld");

    // respawn в Незер: тип ищется по элементу из login
    let mut w = Writer::new();
    let mut d = Vec::new();
    nbt::write_named(&mut d, "", &dim(256)).unwrap();
    w.bytes(&d).string("minecraft:the_nether").i64(5);
    let out = from_1_18_2("respawn", &w.into_vec()).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.string(64).unwrap(), "minecraft:the_nether");
    assert_eq!(r.string(64).unwrap(), "minecraft:the_nether");
    assert_eq!(r.i64().unwrap(), 5);
    assert!(!r.bool().unwrap());
    assert!(r.is_empty());
}

#[test]
fn add_mob_becomes_add_entity() {
    set();
    assert_eq!(canon_name("add_mob", &[]), "add_entity");
    let mut w = Writer::new();
    w.var_int(3)
        .bytes(&[9; 16])
        .var_int(5)
        .bytes(&[0; 24])
        .u8(10)
        .u8(20)
        .u8(30);
    w.i16(1).i16(2).i16(3);
    let out = from_1_18_2("add_mob", &w.into_vec()).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.var_int().unwrap(), 3);
    r.bytes(16).unwrap();
    assert_eq!(r.var_int().unwrap(), 5);
    r.bytes(24).unwrap();
    assert_eq!(
        (r.u8().unwrap(), r.u8().unwrap(), r.u8().unwrap()),
        (20, 10, 30)
    );
    assert_eq!(r.var_int().unwrap(), 0);
    assert_eq!(
        (r.i16().unwrap(), r.i16().unwrap(), r.i16().unwrap()),
        (1, 2, 3)
    );
    assert!(r.is_empty());
}

#[test]
fn add_entity_head_and_varint_data() {
    set();
    let mut w = Writer::new();
    w.var_int(3)
        .bytes(&[9; 16])
        .var_int(5)
        .bytes(&[0; 24])
        .u8(10)
        .u8(20)
        .i32(300);
    w.i16(0).i16(0).i16(0);
    let out = from_1_18_2("add_entity", &w.into_vec()).unwrap().unwrap();
    let mut r = Reader::new(&out);
    r.var_int().unwrap();
    r.bytes(16).unwrap();
    r.var_int().unwrap();
    r.bytes(24).unwrap();
    assert_eq!(
        (r.u8().unwrap(), r.u8().unwrap(), r.u8().unwrap()),
        (10, 20, 20)
    );
    assert_eq!(r.var_int().unwrap(), 300);
    r.bytes(6).unwrap();
    assert!(r.is_empty());
}

#[test]
fn chat_becomes_system_chat() {
    set();
    assert_eq!(canon_name("chat", &[]), "system_chat");
    let mut w = Writer::new();
    w.string(r#"{"text":"a"}"#).u8(2).bytes(&[0; 16]);
    let out = from_1_18_2("chat", &w.into_vec()).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.string(64).unwrap(), r#"{"text":"a"}"#);
    assert_eq!(r.var_int().unwrap(), 2);
    assert!(r.is_empty());
}

#[test]
fn block_break_ack_becomes_block_update() {
    set();
    assert_eq!(canon_name("block_break_ack", &[]), "block_update");
    let mut w = Writer::new();
    w.i64(77).var_int(1234).var_int(2).bool(true);
    let out = from_1_18_2("block_break_ack", &w.into_vec())
        .unwrap()
        .unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.i64().unwrap(), 77);
    assert_eq!(r.var_int().unwrap(), 1234);
    assert!(r.is_empty());
}

#[test]
fn serverbound_chat_and_sequences() {
    set();
    let mut w = Writer::new();
    w.string("/kill")
        .i64(1)
        .i64(2)
        .bool(false)
        .var_int(0)
        .bytes(&[0, 0, 0])
        .u8(0);
    let out = play_from_canon(sid("chat"), &w.into_vec()).unwrap();
    let mut r = Reader::new(&out[0].1);
    assert_eq!(r.string(256).unwrap(), "/kill");
    assert!(r.is_empty());

    let mut w = Writer::new();
    w.var_int(0).i64(5).u8(1).var_int(42);
    let out = play_from_canon(sid("player_action"), &w.into_vec()).unwrap();
    assert_eq!(out[0].1.len(), 1 + 8 + 1);

    let mut w = Writer::new();
    w.var_int(1).var_int(42).f32(0.0).f32(0.0);
    let out = play_from_canon(sid("use_item"), &w.into_vec()).unwrap();
    assert_eq!(out[0].1, vec![1]);
}

#[test]
fn mob_effect_to_canon_holder() {
    set();
    // 1.18.2: varint entity, byte id (speed = 1), byte amplifier, varint duration, byte flags.
    let mut w = Writer::new();
    w.var_int(7).u8(1).u8(2).var_int(600).u8(6);
    let out = ezzmc_net::v1201::play_to_canon("update_mob_effect", &w.into_vec())
        .unwrap()
        .unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.var_int().unwrap(), 7);
    // канон 26.3: speed = 0
    assert_eq!(r.var_int().unwrap(), 0);
    assert_eq!(r.var_int().unwrap(), 2);
    assert_eq!(r.var_int().unwrap(), 600);
    assert_eq!(r.u8().unwrap(), 6);
    assert!(r.is_empty());
    let mut w = Writer::new();
    w.var_int(7).u8(2);
    let out = ezzmc_net::v1201::play_to_canon("remove_mob_effect", &w.into_vec())
        .unwrap()
        .unwrap();
    assert_eq!(out, vec![7, 1]);
}
