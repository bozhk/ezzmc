//! `cargo xtask capture-golden <ver> <tool>`: запускает `tests-integration/golden/tools/<tool>.java`
//! против деобфусцированного клиента Mojang и пишет stdout в `tests-integration/golden/<ver>/<tool>.json`.
//! Нужен `cargo xtask fetch-reference <ver>` (jar в `reference/_cache/<ver>/client-named.jar`;
//! для версий без обфускации — `client.jar`).
use crate::fetch_reference::{download, version_meta, Res};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run_cmd(ver: &str, tool: &str) -> Res<()> {
    let cache = Path::new("reference/_cache").join(ver);
    let jar = ["client-named.jar", "client.jar"]
        .iter()
        .map(|n| cache.join(n))
        .find(|p| p.exists())
        .ok_or_else(|| {
            format!(
                "нет jar в {} (сначала fetch-reference {ver})",
                cache.display()
            )
        })?;
    let src = Path::new("tests-integration/golden/tools").join(format!("{tool}.java"));
    if !src.exists() {
        return Err(format!("нет {}", src.display()).into());
    }
    // Классы клиента ссылаются на библиотеки (joml, fastutil, …) — докачиваем из манифеста версии.
    let sep = if cfg!(windows) { ";" } else { ":" };
    let mut cp = jar.display().to_string();
    let meta = version_meta(ver)?;
    for lib in meta["libraries"].as_array().into_iter().flatten() {
        let art = &lib["downloads"]["artifact"];
        let (Some(url), Some(sha), Some(path)) = (
            art["url"].as_str(),
            art["sha1"].as_str(),
            art["path"].as_str(),
        ) else {
            continue;
        };
        if path.contains("natives") {
            continue;
        }
        let dst = cache.join("libs").join(path);
        download(url, &dst, Some(sha))?;
        cp.push_str(sep);
        cp.push_str(&dst.display().to_string());
    }
    // Java пишет `logs/` в cwd — запускаем из кэша; пути к jar/исходнику делаем абсолютными.
    let work = cache.join("work");
    fs::create_dir_all(&work)?;
    let abs = |p: &str| -> Res<String> {
        Ok(p.split(sep)
            .map(|x| fs::canonicalize(x).map(|a| a.display().to_string()))
            .collect::<Result<Vec<_>, _>>()?
            .join(sep))
    };
    let out = Command::new("java")
        .current_dir(&work)
        .arg("-cp")
        .arg(abs(&cp)?)
        .arg(fs::canonicalize(&src)?)
        .output()?;
    if !out.status.success() {
        return Err(format!("java: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    let dst = Path::new("tests-integration/golden")
        .join(ver)
        .join(format!("{tool}.json"));
    fs::create_dir_all(dst.parent().ok_or("путь без родителя")?)?;
    fs::write(&dst, out.stdout)?;
    eprintln!("готово: {}", dst.display());
    Ok(())
}
