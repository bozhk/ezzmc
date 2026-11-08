//! `cargo xtask gen-blockprops <ver>`: `tests-integration/golden/<ver>/BlockProps.json` (снят с ванили
//! через `capture-golden <ver> BlockProps`) → `crates/world/src/generated/p<ver>.rs`.
//! Состояния дедуплицируются: `STATE_PROPS[state_id]` — индекс в `PROPS`; формы — в `SHAPES`.
use crate::fetch_reference::Res;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub fn run_cmd(ver: &str) -> Res<()> {
    let src = Path::new("tests-integration/golden")
        .join(ver)
        .join("BlockProps.json");
    let json: Value = serde_json::from_str(&fs::read_to_string(&src).map_err(|e| {
        format!(
            "{}: {e} (сначала `cargo xtask capture-golden {ver} BlockProps`)",
            src.display()
        )
    })?)?;
    let shapes = json["shapes"].as_array().ok_or("нет shapes")?;
    let states = json["states"].as_array().ok_or("нет states")?;
    let mut out = format!(
        "// @generated `cargo xtask gen-blockprops {ver}` из golden/{ver}/BlockProps.json. НЕ РЕДАКТИРОВАТЬ.\n\
         use crate::block::StateProps;\n\n\
         /// Формы (коллизия/контур): список AABB `[minx,miny,minz,maxx,maxy,maxz]` в долях блока.\n\
         pub static SHAPES: &[&[[f64; 6]]] = &[\n"
    );
    for s in shapes {
        out.push_str("    &[");
        for b in s.as_array().ok_or("форма не массив")? {
            let v: Vec<String> = b
                .as_array()
                .ok_or("AABB не массив")?
                .iter()
                .map(|x| fmt_f(x.as_f64().unwrap_or(0.0)))
                .collect();
            write!(out, "[{}], ", v.join(", "))?;
        }
        out.push_str("],\n");
    }
    out.push_str("];\n\n");
    let mut uniq: HashMap<String, usize> = HashMap::new();
    let mut props = String::new();
    let mut idx: Vec<usize> = Vec::with_capacity(states.len());
    for s in states {
        let a = s
            .as_array()
            .filter(|a| a.len() == 10)
            .ok_or("состояние ≠ 10 полей")?;
        let f = |i: usize| a[i].as_f64().unwrap_or(0.0);
        let u = |i: usize| a[i].as_u64().unwrap_or(0);
        let line = format!(
            "    StateProps {{ destroy_time: {}, light_emission: {}, light_block: {}, flags: {}, collision_shape: {}, outline_shape: {}, explosion_resistance: {}, friction: {}, speed_factor: {}, jump_factor: {} }},\n",
            fmt_f32(f(0)), u(1), u(2), u(3), u(4), u(5), fmt_f32(f(6)), fmt_f32(f(7)), fmt_f32(f(8)), fmt_f32(f(9))
        );
        let n = uniq.len();
        let id = *uniq.entry(line.clone()).or_insert_with(|| {
            props.push_str(&line);
            n
        });
        idx.push(id);
    }
    writeln!(out, "pub static PROPS: &[StateProps] = &[\n{props}];\n")?;
    writeln!(
        out,
        "/// Индекс = id состояния блока.\npub static STATE_PROPS: &[u16] = &["
    )?;
    for ch in idx.chunks(24) {
        let v: Vec<String> = ch.iter().map(ToString::to_string).collect();
        writeln!(out, "    {},", v.join(", "))?;
    }
    out.push_str("];\n");
    let dir = Path::new("crates/world/src/generated");
    let modname = format!("p{}", ver.replace(['.', '-'], "_"));
    fs::write(dir.join(format!("{modname}.rs")), out)?;
    crate::gen_protocol::write_mod_rs(dir)?;
    eprintln!(
        "готово: {modname}.rs ({} состояний, {} уникальных наборов, {} форм)",
        states.len(),
        uniq.len(),
        shapes.len()
    );
    Ok(())
}

fn fmt_f(x: f64) -> String {
    format!("{x:?}")
}
fn fmt_f32(x: f64) -> String {
    format!("{:?}", x as f32)
}
