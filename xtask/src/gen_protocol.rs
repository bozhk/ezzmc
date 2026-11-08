//! `cargo xtask gen-protocol <ver>`: `reference/<ver>/reports/packets.json` →
//! `crates/protocol/src/generated/v<ver>.rs` (таблица id пакетов: состояние, направление, id, имя).
//! Файл `generated/mod.rs` пересобирается по списку `v*.rs`.
use crate::fetch_reference::Res;
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::path::Path;

const STATES: [(&str, &str); 5] = [
    ("handshake", "Handshake"),
    ("status", "Status"),
    ("login", "Login"),
    ("configuration", "Configuration"),
    ("play", "Play"),
];

pub fn run_cmd(ver: &str) -> Res<()> {
    let src = Path::new("reference")
        .join(ver)
        .join("reports/packets.json");
    let json: Value = serde_json::from_str(&fs::read_to_string(&src).map_err(|e| {
        format!(
            "{}: {e} (сначала `cargo xtask gen-reports {ver}`; packets.json есть не у всех версий)",
            src.display()
        )
    })?)?;
    let modname = format!("v{}", ver.replace(['.', '-'], "_"));
    let mut out = format!(
        "// @generated `cargo xtask gen-protocol {ver}` из reports/packets.json. НЕ РЕДАКТИРОВАТЬ.\n\
         use crate::{{Direction, PacketInfo, State}};\n\n\
         pub const VERSION: &str = \"{ver}\";\n\n\
         /// Отсортировано по (состояние, направление, id).\n\
         pub static PACKETS: &[PacketInfo] = &[\n"
    );
    let mut count = 0;
    for (key, state) in STATES {
        for (dkey, dir) in [
            ("clientbound", "Clientbound"),
            ("serverbound", "Serverbound"),
        ] {
            let Some(map) = json[key][dkey].as_object() else {
                continue;
            };
            let mut v: Vec<(u64, &str)> = map
                .iter()
                .map(|(n, p)| {
                    p["protocol_id"]
                        .as_u64()
                        .map(|id| (id, n.strip_prefix("minecraft:").unwrap_or(n)))
                        .ok_or_else(|| format!("нет protocol_id у {n}"))
                })
                .collect::<Result<_, _>>()?;
            v.sort();
            for (id, name) in v {
                writeln!(out, "    PacketInfo {{ state: State::{state}, dir: Direction::{dir}, id: {id}, name: \"{name}\" }},")?;
                count += 1;
            }
        }
    }
    out.push_str("];\n");

    let dir = Path::new("crates/protocol/src/generated");
    fs::create_dir_all(dir)?;
    fs::write(dir.join(format!("{modname}.rs")), out)?;
    write_mod_rs(dir)?;
    eprintln!("готово: {modname}.rs ({count} пакетов)");
    Ok(())
}

/// Пересобирает `generated/mod.rs` по списку `*.rs` в каталоге.
pub fn write_mod_rs(dir: &Path) -> Res<()> {
    let mut mods: Vec<String> = fs::read_dir(dir)?
        .filter_map(|e| {
            e.ok()?
                .file_name()
                .to_str()?
                .strip_suffix(".rs")
                .map(String::from)
        })
        .filter(|n| n != "mod")
        .collect();
    mods.sort();
    let mut m = String::from("// @generated `cargo xtask gen-protocol`. НЕ РЕДАКТИРОВАТЬ.\n");
    for n in &mods {
        writeln!(m, "pub mod {n};")?;
    }
    fs::write(dir.join("mod.rs"), m)?;
    Ok(())
}
