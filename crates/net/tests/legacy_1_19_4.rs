//! Перевод пакетов 1.19.4 (762) в формат 1.20.1: portalCooldown, trustEdges, suppressLightUpdates,
//! serverbound sign_update без isFrontText. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{from_1_19_4, play_from_canon};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_19_4);
}

#[test]
fn respawn_gets_portal_cooldown() {
    set();
    let mut w = Writer::new();
    w.string("minecraft:overworld")
        .string("minecraft:overworld")
        .i64(7)
        .u8(1)
        .u8(255)
        .bool(false)
        .bool(false)
        .u8(3)
        .bool(false);
    let src = w.into_vec();
    let out = from_1_19_4("respawn", &src).unwrap().unwrap();
    assert_eq!(&out[..src.len()], &src[..]);
    assert_eq!(&out[src.len()..], &[0]);
}

#[test]
fn light_update_drops_trust_edges() {
    set();
    let mut w = Writer::new();
    w.var_int(3).var_int(-2).bool(true).var_int(1).i64(5);
    let out = from_1_19_4("light_update", &w.into_vec()).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.var_int().unwrap(), 3);
    assert_eq!(r.var_int().unwrap(), -2);
    assert_eq!(r.bitset().unwrap(), vec![5]);
    assert!(r.is_empty());
}

#[test]
fn chunk_light_drops_trust_edges() {
    set();
    let mut w = Writer::new();
    w.i32(1).i32(2);
    let mut hm = Vec::new();
    nbt::write_named(&mut hm, "", &Tag::Compound(vec![])).unwrap();
    w.bytes(&hm).var_int(3).bytes(&[9, 9, 9]);
    // один блок-энтити: xz, y, тип, NBT
    w.var_int(1).u8(0x12).i16(64).var_int(7);
    w.bytes(&hm);
    let head = w.into_vec();
    let mut w = Writer::new();
    w.bytes(&head).bool(true).var_int(1).i64(42);
    let out = from_1_19_4("level_chunk_with_light", &w.into_vec())
        .unwrap()
        .unwrap();
    assert_eq!(&out[..head.len()], &head[..]);
    let mut r = Reader::new(&out[head.len()..]);
    assert_eq!(r.bitset().unwrap(), vec![42]);
    assert!(r.is_empty());
}

#[test]
fn section_blocks_drops_suppress_flag() {
    set();
    let mut w = Writer::new();
    w.i64(0x1234).bool(false).var_int(1).var_long(1 << 12);
    let out = from_1_19_4("section_blocks_update", &w.into_vec())
        .unwrap()
        .unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.i64().unwrap(), 0x1234);
    assert_eq!(r.var_int().unwrap(), 1);
    assert_eq!(r.var_long().unwrap(), 1 << 12);
    assert!(r.is_empty());
}

#[test]
fn unchanged_packets_pass() {
    set();
    assert!(from_1_19_4("keep_alive", &[0; 8]).unwrap().is_none());
}

#[test]
fn sign_update_without_front_flag() {
    set();
    let id = ezzmc_protocol::generated::v1_20_1::PACKETS
        .iter()
        .find(|p| {
            p.state == ezzmc_protocol::State::Play
                && p.dir == ezzmc_protocol::Direction::Serverbound
                && p.name == "sign_update"
        })
        .unwrap()
        .id as i32;
    let mut w = Writer::new();
    w.i64(99).bool(true);
    for s in ["a", "b", "c", "d"] {
        w.string(s);
    }
    let out = play_from_canon(id, &w.into_vec()).unwrap();
    assert_eq!(out.len(), 1);
    let mut r = Reader::new(&out[0].1);
    assert_eq!(r.i64().unwrap(), 99);
    for s in ["a", "b", "c", "d"] {
        assert_eq!(r.string(384).unwrap(), s);
    }
    assert!(r.is_empty());
}
