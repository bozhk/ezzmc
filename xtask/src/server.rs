//! `cargo xtask server <ver> [порт] [--reset] [--animals]`: отладочный ванильный сервер для тестов клиента.
//! Каталог `reference/_server/<ver>/` (вне git): offline-режим (`online-mode=false`), стабильный seed (обычный мир),
//! творческий режим, мир без мобов/погоды, стабильный seed, spawn-protection=0. Запуск — на переднем плане
//! (Ctrl+C / `stop` в консоли). Порт по умолчанию 25565. `--reset` удаляет мир (но не jar и не настройки).
//! `--animals` — `spawn-animals=true` и `spawn-npcs=true`: до 1.21.2 сервер с `false` сразу удаляет
//! и призванных животных/жителей.
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
view-distance=32
simulation-distance=8
max-players=4
use-native-transport=false
motd=ezzmc test server
";

pub fn run_cmd(ver: &str, rest: &[String]) -> Res<()> {
    let reset = rest.iter().any(|a| a == "--reset");
    let animals = rest.iter().any(|a| a == "--animals");
    // Старые серверы генерируют чанки в главном потоке: меньшая дальность ускоряет стенд.
    let view_distance = rest
        .iter()
        .find_map(|a| a.strip_prefix("--view-distance="))
        .map(|v| {
            v.parse::<u8>()
                .map_err(|_| format!("дальность `{v}` не число"))
        })
        .transpose()?;
    // Отдельный мир рядом со стендовым (заготовки для старых версий без команд).
    let level = rest.iter().find_map(|a| a.strip_prefix("--level="));
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
        format!(
            "{}{}server-port={port}\n",
            level
                .map(|l| format!("level-name={l}\n"))
                .unwrap_or_default(),
            {
                let mut p = if animals {
                    PROPERTIES
                        .replace("spawn-animals=false", "spawn-animals=true")
                        .replace("spawn-npcs=false", "spawn-npcs=true")
                } else {
                    PROPERTIES.to_owned()
                };
                if let Some(vd) = view_distance {
                    p = p.replace("view-distance=32", &format!("view-distance={vd}"));
                }
                p
            }
        ),
    )?;
    eprintln!(
        "сервер {ver} на 127.0.0.1:{port} (offline), каталог {}",
        dir.display()
    );
    // Старые серверы форматируют числа в консоли по локали JVM (1.5.2: `1003,50`).
    let st = Command::new("java")
        .current_dir(&dir)
        .arg("-Duser.language=en")
        .arg("-Duser.country=US")
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
