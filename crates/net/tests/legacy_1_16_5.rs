//! 1.16.5 (754): отличия провода от 1.17.
#![allow(clippy::unwrap_used)]

use ezzmc_net::v1165::{container_click, from_1_16_5};
use ezzmc_net::v1201::Pre18;
use ezzmc_protocol::wire::{Reader, Writer};

fn replaced(name: &str, body: &[u8]) -> Vec<u8> {
    match from_1_16_5(name, body).unwrap() {
        Pre18::Replace(b) => b,
        other => panic!("{other:?}"),
    }
}

#[test]
fn chunk_mask_and_light_become_bit_sets() {
    let mut w = Writer::new();
    w.i32(1).i32(-1).bool(true).var_int(0b101).u8(0xAB);
    let b = replaced("level_chunk", &w.into_vec());
    let mut r = Reader::new(&b);
    assert_eq!((r.i32().unwrap(), r.i32().unwrap()), (1, -1));
    assert_eq!((r.var_int().unwrap(), r.i64().unwrap()), (1, 0b101));
    assert_eq!(r.rest(), [0xAB]);

    // свет: sky = секция 1, block = нет
    let mut w = Writer::new();
    w.var_int(0).var_int(0).bool(true);
    w.var_int(0b10).var_int(0).var_int(0).var_int(0);
    w.var_int(2048).bytes(&[7; 2048]);
    let b = replaced("light_update", &w.into_vec());
    let mut r = Reader::new(&b);
    r.bytes(3).unwrap();
    assert_eq!((r.var_int().unwrap(), r.i64().unwrap()), (1, 2));
    for _ in 0..3 {
        assert_eq!(r.var_int().unwrap(), 0);
    }
    assert_eq!(r.var_int().unwrap(), 1);
    assert_eq!(r.var_int().unwrap(), 2048);
    r.bytes(2048).unwrap();
    assert_eq!((r.var_int().unwrap(), r.rest().len()), (0, 0));

    assert!(from_1_16_5("level_chunk", &[0; 3]).is_err());
}

#[test]
fn click_carries_item_from_before_the_click() {
    // окно 3: слот 0 = предмет id 5 ×2 без NBT
    let stack = [1, 5, 2, 0];
    let mut w = Writer::new();
    w.u8(3).i16(1).bytes(&stack);
    from_1_16_5("container_set_content", &w.into_vec()).unwrap();

    let b = container_click(3, 0, 0, 0, vec![(0, vec![0])]);
    assert_eq!(&b[..4], [3, 0, 0, 0]);
    assert_eq!(b[6], 0, "режим PICKUP");
    assert_eq!(&b[7..], stack);
    // слот опустел — следующий клик по нему несёт пустой стек
    let b = container_click(3, 0, 0, 0, vec![]);
    assert_eq!(&b[7..], [0]);
}
