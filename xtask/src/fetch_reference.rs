//! `cargo xtask fetch-reference <ver>`: официальный client.jar → деобфускация (Mojmap) →
//! декомпиляция Vineflower в `reference/<ver>/client/`. Всё в `reference/` (вне git).
//! Версии до 1.14.4 без официальных маппингов — TODO-016.
use crate::mappings::proguard_to_tiny;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const VINEFLOWER: &str =
    "https://repo1.maven.org/maven2/org/vineflower/vineflower/1.12.0/vineflower-1.12.0.jar";
const TINY_REMAPPER: &str =
    "https://maven.fabricmc.net/net/fabricmc/tiny-remapper/0.15.0/tiny-remapper-0.15.0-fat.jar";

fn get_bytes(url: &str) -> Res<Vec<u8>> {
    let mut buf = Vec::new();
    ureq::get(url).call()?.into_reader().read_to_end(&mut buf)?;
    Ok(buf)
}
use std::io::Read;

fn download(url: &str, dst: &Path, sha1: Option<&str>) -> Res<()> {
    if dst.exists() {
        return Ok(());
    }
    eprintln!("скачиваю {url}");
    let data = get_bytes(url)?;
    if let Some(want) = sha1 {
        let got: String = Sha1::digest(&data)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        if got != want {
            return Err(format!("sha1 не совпал для {url}: {got} != {want}").into());
        }
    }
    if let Some(p) = dst.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(dst, data)?;
    Ok(())
}

fn run(cmd: &mut Command) -> Res<()> {
    let st = cmd.status()?;
    if st.success() {
        Ok(())
    } else {
        Err(format!("команда завершилась с {st}").into())
    }
}

pub fn run_cmd(ver: &str) -> Res<()> {
    let root = PathBuf::from("reference");
    let tools = root.join("_tools");
    let cache = root.join("_cache").join(ver);
    let out = root.join(ver).join("client");
    if out.join(".done").exists() {
        eprintln!("reference/{ver} уже получен");
        return Ok(());
    }
    download(VINEFLOWER, &tools.join("vineflower.jar"), None)?;
    download(TINY_REMAPPER, &tools.join("tiny-remapper.jar"), None)?;

    let manifest: Value = ureq::get(MANIFEST).call()?.into_json()?;
    let url = manifest["versions"]
        .as_array()
        .and_then(|a| a.iter().find(|v| v["id"] == ver))
        .and_then(|v| v["url"].as_str())
        .ok_or_else(|| format!("версия {ver} не найдена в манифесте"))?;
    let meta: Value = ureq::get(url).call()?.into_json()?;
    let dl = &meta["downloads"];
    let file = |key: &str| -> Option<(String, String)> {
        Some((
            dl[key]["url"].as_str()?.into(),
            dl[key]["sha1"].as_str()?.into(),
        ))
    };
    let (jar_url, jar_sha) = file("client").ok_or("нет downloads.client")?;
    let jar = cache.join("client.jar");
    download(&jar_url, &jar, Some(&jar_sha))?;

    let mapped = match file("client_mappings") {
        Some((mu, ms)) => {
            let txt = cache.join("client_mappings.txt");
            download(&mu, &txt, Some(&ms))?;
            let tiny = cache.join("client.tiny");
            fs::write(&tiny, proguard_to_tiny(&fs::read_to_string(&txt)?)?)?;
            let mapped = cache.join("client-named.jar");
            if !mapped.exists() {
                run(Command::new("java")
                    .arg("-jar")
                    .arg(tools.join("tiny-remapper.jar"))
                    .args([&jar, &mapped, &tiny])
                    .args(["official", "named"]))?;
            }
            mapped
        }
        None if ver_is_unobfuscated(&meta) => jar, // 26.x+: Mojang отдаёт jar без обфускации
        None => {
            return Err(
                format!("для {ver} нет официальных маппингов (до 1.14.4) — см. TODO-016").into(),
            )
        }
    };

    fs::create_dir_all(&out)?;
    run(Command::new("java")
        .arg("-jar")
        .arg(tools.join("vineflower.jar"))
        .args(["-dgs=1", "-asc=1", "-rsy=1", "-log=WARN"])
        .arg(&mapped)
        .arg(&out))?;
    fs::write(out.join(".done"), "")?;
    eprintln!("готово: {}", out.display());
    Ok(())
}

/// У версий без `client_mappings` и с новым major Java (≥25) jar не обфусцирован.
fn ver_is_unobfuscated(meta: &Value) -> bool {
    meta["javaVersion"]["majorVersion"].as_u64().unwrap_or(0) >= 25
}
