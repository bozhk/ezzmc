//! `cargo xtask gen-reports <ver>`: официальный server.jar → ванильный data generator
//! (`net.minecraft.data.Main --reports --server`) → `reference/<ver>/reports/` (packets, blocks, registries…) и `reference/<ver>/data/` (датапак: теги…).
//! Версии до 1.14 не имеют data generator — там отчётов нет.
use crate::fetch_reference::{download, run, version_meta, Res};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Скачивает (с проверкой sha1) официальный server.jar в кэш и возвращает абсолютный путь.
pub(crate) fn server_jar(ver: &str) -> Res<PathBuf> {
    let meta = version_meta(ver)?;
    let dl = &meta["downloads"]["server"];
    let (url, sha) = match (dl["url"].as_str(), dl["sha1"].as_str()) {
        (Some(u), Some(s)) => (u, s),
        _ => return Err(format!("у {ver} нет server.jar (до 1.2.5)").into()),
    };
    let jar = PathBuf::from("reference/_cache")
        .join(ver)
        .join("server.jar");
    download(url, &jar, Some(sha))?;
    Ok(fs::canonicalize(&jar)?)
}

pub fn run_cmd(ver: &str) -> Res<()> {
    let root = PathBuf::from("reference");
    let out = root.join(ver);
    let reports = out.join("reports");
    let data = out.join("data");
    if reports.join(".done").exists() && data.join(".done").exists() {
        eprintln!("reference/{ver}/reports и data уже получены");
        return Ok(());
    }
    let jar = server_jar(ver)?;

    // С 1.18 server.jar — bundler (внутри META-INF/versions.list); имя файла лежит в central directory открытым текстом.
    let bundled = fs::read(&jar)?
        .windows(22)
        .any(|w| w == b"META-INF/versions.list");
    let mut cmd = Command::new("java");
    if bundled {
        cmd.arg("-DbundlerMainClass=net.minecraft.data.Main")
            .arg("-jar")
            .arg(&jar);
    } else {
        cmd.arg("-cp").arg(&jar).arg("net.minecraft.data.Main");
    }
    // Output — отдельный каталог: генератор удаляет «устаревшие» файлы в нём (стёр бы client/).
    let gen = root.join("_cache").join(ver).join("gen");
    fs::create_dir_all(&gen)?;
    // bundler распаковывается в cwd (libraries/, versions/, logs/) — уводим в кэш.
    let gen = fs::canonicalize(&gen)?;
    fs::create_dir_all(gen.join("work"))?;
    cmd.current_dir(gen.join("work"));
    cmd.args(["--reports", "--server", "--output"]).arg(&gen);
    run(&mut cmd)?;
    if gen.join("reports").is_dir() {
        let _ = fs::remove_dir_all(&reports);
        fs::create_dir_all(&out)?;
        fs::rename(gen.join("reports"), &reports)?;
    }
    // --server: датапак (теги, лут, рецепты) → reference/<ver>/data (теги блоков для TODO-020c).
    if gen.join("data").is_dir() {
        let _ = fs::remove_dir_all(&data);
        fs::rename(gen.join("data"), &data)?;
    } else {
        fs::create_dir_all(&data)?;
    }
    fs::write(data.join(".done"), "")?;
    if !reports.is_dir() {
        return Err(format!(
            "генератор не создал {} (версия без --reports?)",
            reports.display()
        )
        .into());
    }
    fs::write(reports.join(".done"), "")?;
    eprintln!("готово: {}", reports.display());
    Ok(())
}
