//! `cargo xtask gen-registries <ver>`: `reference/<ver>/reports/registries.json` →
//! `crates/protocol/src/generated/r<ver>.rs`. На каждый реестр — `pub static NAME: &[&str]`,
//! индекс = сетевой protocol_id, значение = имя без `minecraft:` (прочие namespace сохраняются).
use crate::fetch_reference::Res;
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub fn run_cmd(ver: &str) -> Res<()> {
    let src = Path::new("reference")
        .join(ver)
        .join("reports/registries.json");
    let json: Value = serde_json::from_str(&fs::read_to_string(&src).map_err(|e| {
        format!(
            "{}: {e} (сначала `cargo xtask gen-reports {ver}`)",
            src.display()
        )
    })?)?;
    let mut out = format!(
        "// @generated `cargo xtask gen-registries {ver}` из reports/registries.json. НЕ РЕДАКТИРОВАТЬ.\n\
         // Индекс в массиве = protocol_id записи реестра.\n\n\
         pub const VERSION: &str = \"{ver}\";\n"
    );
    let mut names: Vec<&String> = json
        .as_object()
        .ok_or("registries.json не объект")?
        .keys()
        .collect();
    names.sort();
    let mut total = 0;
    for reg in names {
        let entries = json[reg]["entries"].as_object().ok_or("нет entries")?;
        let mut v: Vec<(u64, &str)> = entries
            .iter()
            .map(|(n, e)| {
                e["protocol_id"]
                    .as_u64()
                    .map(|id| (id, n.strip_prefix("minecraft:").unwrap_or(n)))
                    .ok_or_else(|| format!("нет protocol_id у {reg}/{n}"))
            })
            .collect::<Result<_, _>>()?;
        v.sort();
        for (i, (id, n)) in v.iter().enumerate() {
            if *id != i as u64 {
                return Err(format!("{reg}: id не плотные ({n}: {id} != {i})").into());
            }
        }
        let ident: String = reg
            .strip_prefix("minecraft:")
            .unwrap_or(reg)
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect();
        writeln!(
            out,
            "\n/// `{reg}` ({} записей)\npub static {ident}: &[&str] = &[",
            v.len()
        )?;
        for (_, n) in &v {
            writeln!(out, "    \"{n}\",")?;
        }
        out.push_str("];\n");
        total += v.len();
    }
    let dir = Path::new("crates/protocol/src/generated");
    fs::create_dir_all(dir)?;
    let modname = format!("r{}", ver.replace(['.', '-'], "_"));
    fs::write(dir.join(format!("{modname}.rs")), out)?;
    crate::gen_protocol::write_mod_rs(dir)?;
    eprintln!("готово: {modname}.rs ({total} записей)");
    Ok(())
}
