//! Golden-тесты: значения, снятые с **настоящей ванили**, лежат данными в `tests-integration/golden/<ver>/<name>.json`.
//!
//! Правила:
//! - Файл создаёт только `cargo xtask capture-golden <ver> <tool>` — запуск `golden/tools/<tool>.java`
//!   (Java-класс с `main`, печатает JSON в stdout) против деобфусцированного клиента Mojang
//!   (`reference/_cache/<ver>/client-named.jar`, нужен `cargo xtask fetch-reference <ver>`).
//! - Автообновления («bless») нет: результат Rust-кода никогда не пишется в golden.
//! - `float` хранится как `i32`/`u32` — биты (`Float.floatToRawIntBits`), сравнение побитовое.
//! - Формат — любой JSON; общий вид `{"fn": "...", "cases": [ {вход..., выход...} ]}`.
use serde_json::Value;
use std::path::PathBuf;

/// Путь к `golden/<ver>/<name>.json`.
pub fn golden_path(ver: &str, name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("golden")
        .join(ver)
        .join(format!("{name}.json"))
}

/// Загружает golden-файл. Ошибка содержит команду, которой его снять.
pub fn load(ver: &str, name: &str) -> Result<Value, String> {
    let p = golden_path(ver, name);
    let text = std::fs::read_to_string(&p).map_err(|e| {
        format!(
            "{}: {e}; снять: cargo xtask capture-golden {ver} {name}",
            p.display()
        )
    })?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", p.display()))
}

/// Массив `cases` из golden-файла.
pub fn cases(v: &Value) -> Result<&Vec<Value>, String> {
    v["cases"].as_array().ok_or_else(|| "нет `cases`".into())
}

/// Побитовое сравнение float с эталоном, сохранённым как биты (`i32` из Java).
pub fn same_f32(got: f32, want_bits: i64) -> bool {
    got.to_bits() == want_bits as i32 as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_f32_is_bitwise() {
        assert!(same_f32(1.0, 0x3f80_0000));
        assert!(same_f32(-0.0, i64::from(i32::MIN))); // -0.0 ≠ 0.0
        assert!(!same_f32(0.0, i64::from(i32::MIN)));
    }
}
