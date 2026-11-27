//! Перевод пакетов 1.20.2 (764) → канон: текст JSON-строкой, старые set_score/explode,
//! сдвиг MenuType. Отдельный бинарь: версия соединения — глобальная.
#![allow(clippy::unwrap_used)]
use ezzmc_net::legacy::{canon_name_1_20_2, to_canon};
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::{Reader, Writer};

fn set() {
    ezzmc_protocol::version::set_current(ezzmc_protocol::version::V1_20_2);
}

fn run(name: &str, w: Writer) -> Vec<u8> {
    let body = w.into_vec();
    to_canon(Some(name), body).unwrap().unwrap()
}

fn text(r: &mut Reader) -> String {
    let t = ezzmc_protocol::nbt::read_network(r).unwrap().unwrap();
    Component::from_nbt(&t).unwrap().to_plain(&|_| None)
}

#[test]
fn json_text_1_20_2() {
    set();
    let mut w = Writer::new();
    w.string(r#"{"text":"hello ","extra":[{"text":"json","color":"red"}]}"#)
        .bool(false);
    let out = run("system_chat", w);
    let mut r = Reader::new(&out);
    assert_eq!(text(&mut r), "hello json");
    assert!(!r.bool().unwrap());
    assert_eq!(r.remaining(), 0);

    // open_screen: MenuType после generic_3x3 сдвинут на crafter_3x3.
    for (old, new) in [(6, 6), (7, 8), (12, 13)] {
        let mut w = Writer::new();
        w.var_int(3).var_int(old).string(r#""Box""#);
        let out = run("open_screen", w);
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 3);
        assert_eq!(r.var_int().unwrap(), new);
        assert_eq!(text(&mut r), "Box");
    }
}

#[test]
fn set_score_1_20_2() {
    set();
    let mut w = Writer::new();
    w.string("Fake").var_int(1).string("sc");
    let body = w.into_vec();
    assert_eq!(canon_name_1_20_2("set_score", &body), "reset_score");
    let out = to_canon(Some("set_score"), body).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.string(40).unwrap(), "Fake");
    assert!(r.bool().unwrap());
    assert_eq!(r.string(40).unwrap(), "sc");
    assert_eq!(r.remaining(), 0);

    let mut w = Writer::new();
    w.string("EzzBot").var_int(0).string("sc").var_int(42);
    let body = w.into_vec();
    assert_eq!(canon_name_1_20_2("set_score", &body), "set_score");
    let out = to_canon(Some("set_score"), body).unwrap().unwrap();
    let mut r = Reader::new(&out);
    assert_eq!(r.string(40).unwrap(), "EzzBot");
    assert_eq!(r.string(40).unwrap(), "sc");
    assert_eq!(r.var_int().unwrap(), 42);
}

#[test]
fn explode_and_info_1_20_2() {
    set();
    let mut w = Writer::new();
    w.f64(1.0).f64(64.0).f64(2.0).f32(4.0);
    w.var_int(1).bytes(&[0, 0, 1]);
    w.f32(0.1).f32(0.2).f32(0.3);
    let out = run("explode", w);
    assert!(!out.is_empty());

    // player_info_update: ADD_PLAYER | UPDATE_LISTED | UPDATE_DISPLAY_NAME.
    let mut w = Writer::new();
    w.u8(1 | 8 | 32).var_int(1).bytes(&[7; 16]);
    w.string("Bob").var_int(0).bool(true).bool(true);
    w.string(r#"{"text":"Bobby"}"#);
    let out = run("player_info_update", w);
    let mut r = Reader::new(&out);
    r.bytes(1 + 1 + 16).unwrap();
    assert_eq!(r.string(16).unwrap(), "Bob");
    assert_eq!(r.var_int().unwrap(), 0);
    assert!(r.bool().unwrap());
    assert!(r.bool().unwrap());
    assert_eq!(text(&mut r), "Bobby");
}
