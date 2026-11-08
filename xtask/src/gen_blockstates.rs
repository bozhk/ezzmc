//! `cargo xtask gen-blockstates <ver>`: `reports/blocks.json` → `crates/world/src/generated/b<ver>.rs`.
//! Для каждого блока: имя, первый id состояния, число состояний, состояние по умолчанию, свойства
//! (свойства по алфавиту имён, как StateDefinition; значения — как в отчёте). id состояния = first + Σ idx_i·stride_i (последнее свойство
//! меняется быстрее всех); генератор сверяет это с каждым состоянием из отчёта.
//! Твёрдость/форма/инструмент в отчёте нет — TODO-020b.
use crate::fetch_reference::Res;
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub fn run_cmd(ver: &str) -> Res<()> {
    let src = Path::new("reference").join(ver).join("reports/blocks.json");
    let json: Value = serde_json::from_str(&fs::read_to_string(&src).map_err(|e| {
        format!(
            "{}: {e} (сначала `cargo xtask gen-reports {ver}`)",
            src.display()
        )
    })?)?;
    struct B<'a> {
        name: &'a str,
        first: u64,
        count: u64,
        default: u64,
        props: Vec<(&'a str, Vec<&'a str>)>,
    }
    let mut blocks = Vec::new();
    for (name, b) in json.as_object().ok_or("blocks.json не объект")? {
        // VANILLA: StateDefinition (properties в TreeMap по имени) — id считаются по алфавиту имён,
        // а не по порядку объявления в отчёте.
        let mut props: Vec<(&str, Vec<&str>)> = b["properties"]
            .as_object()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        let vals = v
                            .as_array()
                            .map(|a| a.iter().filter_map(Value::as_str).collect())
                            .unwrap_or_default();
                        (k.as_str(), vals)
                    })
                    .collect()
            })
            .unwrap_or_default();
        props.sort_by_key(|(k, _)| *k);
        let states = b["states"].as_array().ok_or("нет states")?;
        let mut ids: Vec<u64> = states.iter().filter_map(|s| s["id"].as_u64()).collect();
        ids.sort();
        let first = *ids
            .first()
            .ok_or_else(|| format!("{name}: нет состояний"))?;
        let count: u64 = props.iter().map(|(_, v)| v.len() as u64).product();
        if count != ids.len() as u64 || ids.last() != Some(&(first + count - 1)) {
            return Err(format!("{name}: состояния не образуют диапазон {first}+{count}").into());
        }
        let mut default = None;
        for s in states {
            let id = s["id"].as_u64().ok_or("нет id")?;
            let mut off = 0;
            for (p, vals) in &props {
                let v = s["properties"][p].as_str().ok_or("нет значения свойства")?;
                let i = vals
                    .iter()
                    .position(|x| *x == v)
                    .ok_or("значение вне списка")? as u64;
                off = off * vals.len() as u64 + i;
            }
            if first + off != id {
                return Err(format!("{name}: порядок состояний не radix (id {id})").into());
            }
            if s["default"].as_bool() == Some(true) {
                default = Some(id);
            }
        }
        blocks.push(B {
            name: name.strip_prefix("minecraft:").unwrap_or(name),
            first,
            count,
            default: default.ok_or_else(|| format!("{name}: нет default"))?,
            props,
        });
    }
    blocks.sort_by_key(|b| b.first);
    let mut next = 0;
    for b in &blocks {
        if b.first != next {
            return Err(format!("{}: дыра в id состояний ({} != {next})", b.name, b.first).into());
        }
        next += b.count;
    }
    let mut out = format!(
        "// @generated `cargo xtask gen-blockstates {ver}` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.\n\
         use crate::block::{{BlockInfo, PropInfo}};\n\n\
         pub const VERSION: &str = \"{ver}\";\n\
         pub const STATE_COUNT: u32 = {next};\n\n\
         /// Индекс = id блока в реестре `minecraft:block` (порядок по первому id состояния).\n\
         pub static BLOCKS: &[BlockInfo] = &[\n"
    );
    for b in &blocks {
        write!(
            out,
            "    BlockInfo {{ name: \"{}\", first_state: {}, state_count: {}, default_state: {}, props: &[",
            b.name, b.first, b.count, b.default
        )?;
        for (p, vals) in &b.props {
            let v: Vec<String> = vals.iter().map(|x| format!("\"{x}\"")).collect();
            write!(
                out,
                "PropInfo {{ name: \"{p}\", values: &[{}] }}, ",
                v.join(", ")
            )?;
        }
        out.push_str("] },\n");
    }
    out.push_str("];\n");
    let dir = Path::new("crates/world/src/generated");
    fs::create_dir_all(dir)?;
    let modname = format!("b{}", ver.replace(['.', '-'], "_"));
    fs::write(dir.join(format!("{modname}.rs")), out)?;
    crate::gen_protocol::write_mod_rs(dir)?;
    eprintln!(
        "готово: {modname}.rs ({} блоков, {next} состояний)",
        blocks.len()
    );
    Ok(())
}
