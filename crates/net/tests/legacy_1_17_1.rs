//! 1.17.1 (756): свет отдельным light_update перед level_chunk, чанк с BitSet секций и биомами в
//! пакете, login без simulationDistance. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::v1201::{canon_name, from_1_17_1, play_from_canon, play_to_canon, Pre18};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, State};
use ezzmc_world::palette::{Kind, PalettedContainer};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_17_1);
}

fn replaced(name: &str, body: &[u8]) -> Vec<u8> {
    match from_1_17_1(name, body).unwrap() {
        Pre18::Replace(b) => b,
        other => panic!("{name}: {other:?}"),
    }
}

/// Чанк в 2 секции: отмечена только вторая; биомы первой одинаковые, второй — 2 и 10 значений.
fn chunk(x: i32, biomes: &[i32]) -> Vec<u8> {
    let mut data = Writer::new();
    data.i16(1).u8(4).var_int(2).var_int(0).var_int(1);
    data.var_int(256);
    for _ in 0..256 {
        data.i64(0x1111_1111_1111_1111);
    }
    let data = data.into_vec();
    let mut w = Writer::new();
    w.i32(x).i32(0).var_int(1).i64(0b10);
    let mut h = Vec::new();
    nbt::write_named(&mut h, "", &Tag::Compound(Vec::new())).unwrap();
    w.bytes(&h).var_int(biomes.len() as i32);
    for &b in biomes {
        w.var_int(b);
    }
    w.var_int(data.len() as i32).bytes(&data).var_int(0);
    w.into_vec()
}

fn light(x: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.var_int(x).var_int(0).bool(true);
    w.var_int(1).i64(3).var_int(0).var_int(0).var_int(0);
    w.var_int(0).var_int(0);
    w.into_vec()
}

#[test]
fn light_waits_for_chunk_and_sections_expand() {
    set();
    let mut biomes = vec![1; 64];
    biomes.extend((0..64).map(|i| i % 10 + 40));
    let l = light(5);
    assert!(matches!(
        from_1_17_1("light_update", &l).unwrap(),
        Pre18::Drop
    ));
    let out = replaced("level_chunk", &chunk(5, &biomes));
    assert_eq!(canon_name("level_chunk", &[]), "level_chunk_with_light");
    let mut r = Reader::new(&out);
    assert_eq!((r.i32().unwrap(), r.i32().unwrap()), (5, 0));
    nbt::read_named(&mut r).unwrap();
    let n = r.var_int().unwrap() as usize;
    let mut s = Reader::new(r.bytes(n).unwrap());
    for (section, want) in biomes.chunks(64).enumerate() {
        assert_eq!(s.i16().unwrap(), section as i16);
        let blocks = PalettedContainer::read(Kind::BlockStates, 15, &mut s).unwrap();
        assert_eq!(blocks.get(3, 0, 0), section as u32);
        let b = PalettedContainer::read(Kind::Biomes, 6, &mut s).unwrap();
        for (i, &v) in want.iter().enumerate() {
            assert_eq!(b.get(i & 3, i >> 4, (i >> 2) & 3), v as u32);
        }
    }
    assert!(s.is_empty());
    assert_eq!(r.var_int().unwrap(), 0);
    // свет — тело light_update после координат
    assert_eq!(r.rest(), &l[2..]);
    // загруженный чанк: свет идёт как есть; после forget — снова откладывается
    assert!(matches!(
        from_1_17_1("light_update", &l).unwrap(),
        Pre18::Keep
    ));
    let mut f = Writer::new();
    f.i32(5).i32(0);
    from_1_17_1("forget_level_chunk", &f.into_vec()).unwrap();
    assert!(matches!(
        from_1_17_1("light_update", &l).unwrap(),
        Pre18::Drop
    ));
    // без отложенного света — пустые маски
    let out = replaced("level_chunk", &chunk(9, &biomes));
    assert!(out.ends_with(&[0, 1, 0, 0, 0, 0, 0, 0]));
    // обрезанные данные — ошибка, а не паника
    let c = chunk(9, &biomes);
    assert!(from_1_17_1("level_chunk", &c[..c.len() / 2]).is_err());
    login_gets_simulation_distance();
}

/// Вызывается из теста чанка: `login` сбрасывает общий кэш света.
fn login_gets_simulation_distance() {
    let mut w = Writer::new();
    w.i32(7).bool(false).u8(1).u8(255).var_int(1).string("a:b");
    let mut t = Vec::new();
    nbt::write_named(&mut t, "", &Tag::Compound(Vec::new())).unwrap();
    w.bytes(&t)
        .bytes(&t)
        .string("a:b")
        .i64(5)
        .var_int(20)
        .var_int(8);
    let head = w.into_vec();
    let mut body = head.clone();
    body.extend_from_slice(&[0, 1, 0, 0]);
    let out = replaced("login", &body);
    assert_eq!(&out[..head.len()], &head[..]);
    assert_eq!(&out[head.len()..], &[8, 0, 1, 0, 0]);
}

#[test]
fn client_information_without_allows_listing() {
    set();
    let id = ezzmc_protocol::version::V1_17_1
        .packets
        .iter()
        .find(|p| {
            p.state == State::Play
                && p.dir == Direction::Serverbound
                && p.name == "client_information"
        })
        .unwrap()
        .id as i32;
    let mut w = Writer::new();
    w.string("ru_ru")
        .u8(8)
        .var_int(0)
        .bool(true)
        .u8(127)
        .var_int(1)
        .bool(false)
        .bool(true)
        .var_int(0);
    let out = play_from_canon(id, &w.into_vec()).unwrap();
    assert_eq!(out[0].1.len(), 6 + 6);
}

#[test]
fn light_update_dropped_by_play_to_canon() {
    set();
    assert!(play_to_canon("light_update", &light(77)).unwrap().is_none());
}
