//! Перевод пакетов 1.20.4 (765) → канон. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::legacy::to_canon;
use ezzmc_protocol::generated::{r1_20_4, r26_3};
use ezzmc_protocol::wire::{Reader, Writer};

fn id(table: &[&str], n: &str) -> i32 {
    table.iter().position(|x| *x == n).unwrap() as i32
}

/// Заголовок level_particles 1.20.4: тип, overrideLimiter, xyz, dist, maxSpeed, count.
fn head(ty: &str, dist: f32, speed: f32, count: i32) -> Writer {
    let mut w = Writer::new();
    w.var_int(id(r1_20_4::PARTICLE_TYPE, ty)).bool(true);
    w.f64(1.0).f64(2.0).f64(3.0);
    w.f32(dist).f32(dist).f32(dist).f32(speed).i32(count);
    w
}

/// Канон: тип, данные (`data`), флаги, xyz, dist, maxSpeed ×3, count, randomization.
fn check(out: &[u8], ty: &str, data: impl FnOnce(&mut Reader)) {
    let mut r = Reader::new(out);
    assert_eq!(r.var_int().unwrap(), id(r26_3::PARTICLE_TYPE, ty));
    data(&mut r);
    assert_eq!((r.bool().unwrap(), r.bool().unwrap()), (true, false));
    assert_eq!(r.f64().unwrap(), 1.0);
    r.bytes(16 + 12 + 12).unwrap();
    r.var_int().unwrap();
    assert_eq!(r.var_int().unwrap(), 0);
    assert_eq!(r.remaining(), 0);
}

#[test]
fn particles_1_20_4() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_20_4);
    let run = |w: Writer| {
        to_canon(Some("level_particles"), w.into_vec())
            .unwrap()
            .unwrap()
    };

    // dust: Vector3f + scale → int RGB + scale.
    let mut w = head("dust", 0.3, 0.0, 30);
    w.f32(0.0).f32(0.0).f32(1.0).f32(2.0);
    check(&run(w), "dust", |r| {
        assert_eq!(r.i32().unwrap(), 0x0000FF);
        assert_eq!(r.f32().unwrap(), 2.0);
    });

    // dust_color_transition 1.20.4: from, scale, to → from, to, scale.
    let mut w = head("dust_color_transition", 0.3, 0.0, 30);
    w.f32(1.0)
        .f32(1.0)
        .f32(0.0)
        .f32(2.0)
        .f32(0.0)
        .f32(1.0)
        .f32(0.0);
    check(&run(w), "dust_color_transition", |r| {
        assert_eq!(r.i32().unwrap(), 0xFFFF00);
        assert_eq!(r.i32().unwrap(), 0x00FF00);
        assert_eq!(r.f32().unwrap(), 2.0);
    });

    // entity_effect без данных: при count == 0 цвет = dist * speed.
    let w = head("entity_effect", 1.0, 1.0, 0);
    check(&run(w), "entity_effect", |r| {
        assert_eq!(r.i32().unwrap() as u32, 0xFFFF_FFFF);
    });

    // ambient_entity_effect (удалён в 1.20.5) → entity_effect с альфой 0x26.
    let w = head("ambient_entity_effect", 0.0, 1.0, 5);
    check(&run(w), "entity_effect", |r| {
        assert_eq!(r.i32().unwrap() as u32, 0x26FF_FFFF);
    });

    // Тип без отличий в данных (flame) проходит общим путём.
    let w = head("flame", 0.0, 0.0, 3);
    check(&run(w), "flame", |_| {});
}

/// Команда: строки правил 1.20.4 → id, порядок полей 26.3, цвет Optional<TeamColor>.
#[test]
fn team_1_20_4() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_20_4);
    let team = |color: i32| {
        let mut w = Writer::new();
        w.string("red").u8(2);
        w.u8(8).u16(3).bytes(b"red");
        w.u8(3).string("never").string("pushOwnTeam").var_int(color);
        w.u8(8).u16(1).bytes(b"[");
        w.u8(8).u16(1).bytes(b"]");
        to_canon(Some("set_player_team"), w.into_vec())
            .unwrap()
            .unwrap()
    };
    let out = team(12);
    let mut r = Reader::new(&out);
    assert_eq!(r.string(16).unwrap(), "red");
    assert_eq!(r.u8().unwrap(), 2);
    assert_eq!(r.bytes(6).unwrap(), [8, 0, 3, b'r', b'e', b'd']);
    assert_eq!(r.bytes(4).unwrap(), [8, 0, 1, b'[']);
    assert_eq!(r.bytes(4).unwrap(), [8, 0, 1, b']']);
    assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (1, 3));
    assert_eq!((r.bool().unwrap(), r.var_int().unwrap()), (true, 12));
    assert_eq!(r.u8().unwrap(), 3);
    assert_eq!(r.remaining(), 0);
    // RESET (21) → без цвета.
    let out = team(21);
    assert_eq!(out[out.len() - 2..], [0, 3]);
}
