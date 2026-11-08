//! `cargo xtask gen-tags <ver>`: датапак `reference/<ver>/data/minecraft/tags/{block,item,entity_type,fluid}`
//! (из `gen-reports`, флаг `--server`) → `crates/world/src/generated/t<ver>.rs`.
//! Теги разворачиваются (`#ref`) в отсортированные списки protocol_id из `reports/registries.json`.
use crate::fetch_reference::Res;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::Path;

type Raw = BTreeMap<String, Vec<String>>;

fn walk(dir: &Path, prefix: &str, out: &mut Raw) -> Res<()> {
    for e in fs::read_dir(dir)? {
        let p = e?.path();
        let name = p.file_name().and_then(|n| n.to_str()).ok_or("имя файла")?;
        if p.is_dir() {
            walk(&p, &format!("{prefix}{name}/"), out)?;
        } else if let Some(stem) = name.strip_suffix(".json") {
            let j: Value = serde_json::from_str(&fs::read_to_string(&p)?)?;
            let vals = j["values"]
                .as_array()
                .ok_or_else(|| format!("{}: нет values", p.display()))?
                .iter()
                .filter_map(|v| v.as_str().or_else(|| v["id"].as_str()).map(String::from))
                .collect();
            out.insert(format!("{prefix}{stem}"), vals);
        }
    }
    Ok(())
}

fn resolve(
    tag: &str,
    raw: &Raw,
    ids: &BTreeMap<String, u64>,
    stack: &mut Vec<String>,
) -> Res<BTreeSet<u64>> {
    if stack.iter().any(|s| s == tag) {
        return Err(format!("цикл тегов: {tag}").into());
    }
    stack.push(tag.into());
    let mut set = BTreeSet::new();
    for v in raw.get(tag).ok_or_else(|| format!("нет тега #{tag}"))? {
        if let Some(r) = v.strip_prefix('#') {
            set.extend(resolve(
                r.strip_prefix("minecraft:").unwrap_or(r),
                raw,
                ids,
                stack,
            )?);
        } else if let Some(id) = ids.get(v) {
            set.insert(*id);
        } else {
            return Err(format!("тег {tag}: неизвестная запись {v}").into());
        }
    }
    stack.pop();
    Ok(set)
}

pub fn run_cmd(ver: &str) -> Res<()> {
    let root = Path::new("reference").join(ver);
    let reg: Value =
        serde_json::from_str(&fs::read_to_string(root.join("reports/registries.json"))?)?;
    let mut out = format!(
        "// @generated `cargo xtask gen-tags {ver}` из датапака. НЕ РЕДАКТИРОВАТЬ.\n\
         // (имя тега без `minecraft:`, значения — protocol_id записей реестра, по возрастанию)\n\n"
    );
    for (kind, regname, konst) in [
        ("block", "minecraft:block", "BLOCK_TAGS"),
        ("item", "minecraft:item", "ITEM_TAGS"),
        ("entity_type", "minecraft:entity_type", "ENTITY_TYPE_TAGS"),
        ("fluid", "minecraft:fluid", "FLUID_TAGS"),
    ] {
        let dir = root.join("data/minecraft/tags").join(kind);
        if !dir.is_dir() {
            continue;
        }
        let ids: BTreeMap<String, u64> = reg[regname]["entries"]
            .as_object()
            .ok_or_else(|| format!("нет реестра {regname}"))?
            .iter()
            .filter_map(|(k, v)| Some((k.clone(), v["protocol_id"].as_u64()?)))
            .collect();
        let mut raw = Raw::new();
        walk(&dir, "", &mut raw)?;
        writeln!(
            out,
            "/// Теги реестра `{regname}` ({} шт.).\npub static {konst}: &[(&str, &[u32])] = &[",
            raw.len()
        )?;
        for tag in raw.keys() {
            let set = resolve(tag, &raw, &ids, &mut Vec::new())?;
            let v: Vec<String> = set.iter().map(ToString::to_string).collect();
            writeln!(out, "    (\"{tag}\", &[{}]),", v.join(", "))?;
        }
        out.push_str("];\n\n");
    }
    let d = Path::new("crates/world/src/generated");
    let modname = format!("t{}", ver.replace(['.', '-'], "_"));
    fs::write(d.join(format!("{modname}.rs")), out)?;
    crate::gen_protocol::write_mod_rs(d)?;
    eprintln!("готово: {modname}.rs");
    Ok(())
}
