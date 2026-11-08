//! Проверяет, что снятые с ванили golden-файлы читаются и имеют ожидаемую форму.
//! (Сравнения с Rust-реализацией появятся вместе с портом `Mth`.)
use ezzmc_tests_integration::{cases, load};

#[test]
fn mth_sin_golden_loads() {
    let v = load("1.21.11", "MthSin").expect("golden MthSin");
    let c = cases(&v).expect("cases");
    assert!(c.len() >= 10);
    // sin(0) = 0.0 (биты 0), cos(0) = 1.0 (0x3f800000) — сверка самого эталона.
    assert_eq!(c[0]["sin"], 0);
    assert_eq!(c[0]["cos"], 0x3f80_0000);
}
