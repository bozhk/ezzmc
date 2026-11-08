//! `cargo xtask server <ver> [порт] [--reset]`: отладочный ванильный сервер для тестов клиента.
//! Каталог `reference/_server/<ver>/` (вне git): offline-режим (`online-mode=false`), стабильный seed (обычный мир),
//! творческий режим, мир без мобов/погоды, стабильный seed, spawn-protection=0. Запуск — на переднем плане
//! (Ctrl+C / `stop` в консоли). Порт по умолчанию 25565. `--reset` удаляет мир (но не jar и не настройки).
//! Принятие EULA Mojang для локального отладочного сервера записывается автоматически (`eula=true`).
use crate::fetch_reference::Res;
use crate::gen_reports::server_jar;
use std::fs;
use std::path::Path;
use std::process::Command;

const PROPERTIES: &str = "\
online-mode=false
white-list=false
enforce-secure-profile=false
level-seed=1337
gamemode=creative
force-gamemode=true
difficulty=peaceful
spawn-monsters=false
spawn-animals=false
spawn-npcs=false
spawn-protection=0
allow-flight=true
enable-command-block=true
view-distance=8
simulation-distance=8
max-players=4
motd=ezzmc test server
";

pub fn run_cmd(ver: &str, rest: &[String]) -> Res<()> {
    let reset = rest.iter().any(|a| a == "--reset");
    let port: u16 = match rest.iter().find(|a| !a.starts_with("--")) {
        Some(p) => p.parse().map_err(|_| format!("порт `{p}` не число"))?,
        None => 25565,
    };
    let jar = server_jar(ver)?;
    let dir = Path::new("reference/_server").join(ver);
    fs::create_dir_all(&dir)?;
    if reset {
        for w in ["world", "world_nether", "world_the_end"] {
            let _ = fs::remove_dir_all(dir.join(w));
        }
    }
    fs::write(dir.join("eula.txt"), "eula=true\n")?;
    // Настройки пишем всегда: стенд должен быть воспроизводим (правки руками затираются).
    fs::write(
        dir.join("server.properties"),
        format!("{PROPERTIES}server-port={port}\n"),
    )?;
    eprintln!(
        "сервер {ver} на 127.0.0.1:{port} (offline), каталог {}",
        dir.display()
    );
    let st = Command::new("java")
        .current_dir(&dir)
        .arg("-jar")
        .arg(&jar)
        .arg("nogui")
        .status()?;
    if st.success() {
        Ok(())
    } else {
        Err(format!("сервер завершился с {st}").into())
    }
}
