//! `cargo xtask fetch-reference <ver>`: официальный client.jar → деобфускация (Mojmap) →
//! декомпиляция Vineflower в `reference/<ver>/client/`. Всё в `reference/` (вне git).
//! Версии до 1.14.4 — Yarn-маппинги (`legacy_named_jar`).
use crate::mappings::proguard_to_tiny;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) type Res<T> = Result<T, Box<dyn std::error::Error>>;

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

pub(crate) fn download(url: &str, dst: &Path, sha1: Option<&str>) -> Res<()> {
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

pub(crate) fn run(cmd: &mut Command) -> Res<()> {
    let st = cmd.status()?;
    if st.success() {
        Ok(())
    } else {
        Err(format!("команда завершилась с {st}").into())
    }
}

/// JSON-описание версии из манифеста Mojang (`downloads`, `javaVersion`, …).
pub(crate) fn version_meta(ver: &str) -> Res<Value> {
    let manifest: Value = ureq::get(MANIFEST).call()?.into_json()?;
    let url = manifest["versions"]
        .as_array()
        .and_then(|a| a.iter().find(|v| v["id"] == ver))
        .and_then(|v| v["url"].as_str())
        .ok_or_else(|| format!("версия {ver} не найдена в манифесте"))?;
    Ok(ureq::get(url).call()?.into_json()?)
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

    let meta = version_meta(ver)?;
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
        None => legacy_named_jar(ver, &jar, &cache, &tools)?,
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

/// Версии до 1.14.4: официальных маппингов нет → official→intermediary→named (Yarn).
/// Legacy Fabric (1.5.2–1.13.2), для 1.14–1.14.3 — Fabric. Это НЕ имена Mojang: классы/поля
/// называются по Yarn; смысл тот же. (TODO-016)
fn legacy_named_jar(ver: &str, jar: &Path, cache: &Path, tools: &Path) -> Res<PathBuf> {
    let named = cache.join("client-named.jar");
    if named.exists() {
        return Ok(named);
    }
    // (meta, maven-база, group)
    let sources = [
        (
            "https://meta.legacyfabric.net/v2/versions/yarn",
            "https://maven.legacyfabric.net/net/legacyfabric",
            "legacyfabric",
        ),
        (
            "https://meta.fabricmc.net/v2/versions/yarn",
            "https://maven.fabricmc.net/net/fabricmc",
            "fabric",
        ),
    ];
    for (meta, base, tag) in sources {
        let list: Value = ureq::get(&format!("{meta}/{ver}")).call()?.into_json()?;
        let Some(yv) = list.get(0).and_then(|v| v["version"].as_str()) else {
            continue;
        };
        eprintln!("{ver}: Yarn {yv} ({tag})");
        let yarn = cache.join("yarn-v2.jar");
        let inter = cache.join("intermediary-v2.jar");
        download(&format!("{base}/yarn/{yv}/yarn-{yv}-v2.jar"), &yarn, None)?;
        download(
            &format!("{base}/intermediary/{ver}/intermediary-{ver}-v2.jar"),
            &inter,
            None,
        )?;
        let (m1, m2) = (
            cache.join("official-intermediary.tiny"),
            cache.join("intermediary-named.tiny"),
        );
        for (src, dst) in [(&inter, &m1), (&yarn, &m2)] {
            let mut z = zip::ZipArchive::new(fs::File::open(src)?)?;
            let mut f = z.by_name("mappings/mappings.tiny")?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            fs::write(dst, buf)?;
        }
        let mid = cache.join("client-intermediary.jar");
        let remap = |from: &Path, to: &Path, map: &Path, a: &str, b: &str| {
            run(Command::new("java")
                .arg("-jar")
                .arg(tools.join("tiny-remapper.jar"))
                .args([from, to, map])
                .args([a, b]))
        };
        remap(jar, &mid, &m1, "official", "intermediary")?;
        remap(&mid, &named, &m2, "intermediary", "named")?;
        return Ok(named);
    }
    Err(format!("для {ver} нет Yarn-маппингов ни в Legacy Fabric, ни в Fabric").into())
}
