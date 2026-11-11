//! Запуск игры: обновление Microsoft-сессии, ezzmc (Rust) или ванильная Java.
use crate::config::{Account, Config, Kind};
use crate::java;
use ezzmc_auth::microsoft;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Session {
    pub name: String,
    pub uuid: String,
    pub token: String,
    pub online: bool,
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Offline → без токена. Microsoft → при необходимости обновить токены (меняет `acc`).
pub fn resolve_session(acc: &mut Account, client_id: &str) -> Result<Session, String> {
    if acc.kind == Kind::Offline {
        return Ok(Session {
            name: acc.name.clone(),
            uuid: acc.uuid.clone(),
            token: "0".into(),
            online: false,
        });
    }
    if !acc.mc_token.is_empty() && acc.mc_expires > now() + 300 {
        return Ok(Session {
            name: acc.name.clone(),
            uuid: acc.uuid.clone(),
            token: acc.mc_token.clone(),
            online: true,
        });
    }
    if client_id.is_empty() {
        return Err("не задан Azure client_id (Настройки → Microsoft)".into());
    }
    let t = microsoft::refresh(client_id, &acc.refresh_token).map_err(|e| e.to_string())?;
    let mc = microsoft::minecraft_login(&t.access_token).map_err(|e| e.to_string())?;
    if !t.refresh_token.is_empty() {
        acc.refresh_token = t.refresh_token;
    }
    acc.name = mc.name.clone();
    acc.uuid = mc.uuid.clone();
    acc.mc_token = mc.access_token.clone();
    acc.mc_expires = mc.expires_at;
    Ok(Session {
        name: mc.name,
        uuid: mc.uuid,
        token: mc.access_token,
        online: true,
    })
}

pub fn find_client(cfg: &Config) -> Result<PathBuf, String> {
    let s = cfg.settings.client_path.trim();
    if !s.is_empty() {
        let p = PathBuf::from(s);
        return p
            .is_file()
            .then_some(p)
            .ok_or_else(|| format!("нет файла {s}"));
    }
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf));
    let mut c = Vec::new();
    if let Some(d) = &exe_dir {
        c.push(d.join("ezzmc-client"));
        // target/<профиль>/ezzmc-launcher → соседние профили
        if let Some(t) = d.parent() {
            for prof in ["fast", "release", "debug"] {
                c.push(t.join(prof).join("ezzmc-client"));
            }
        }
    }
    // самый свежий по времени изменения (в target/ часто лежат старые профили)
    c.into_iter()
        .filter_map(|p| {
            let t = std::fs::metadata(&p).and_then(|m| m.modified()).ok()?;
            Some((t, p))
        })
        .max_by_key(|(t, _)| *t)
        .map(|(_, p)| p)
        .ok_or_else(|| "ezzmc-client не найден: соберите его или укажите путь в настройках".into())
}

/// Готовая к запуску команда и описание для лога.
pub fn build_ezzmc(cfg: &Config, s: &Session) -> Result<Command, String> {
    let bin = find_client(cfg)?;
    let st = &cfg.settings;
    let dir = PathBuf::from(&st.client_dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut cmd = Command::new(bin);
    cmd.current_dir(&dir)
        .env("EZZMC_MINECRAFT_DIR", &st.game_dir)
        .arg("--name")
        .arg(&s.name);
    if s.online {
        cmd.env(ezzmc_auth::ENV_NAME, &s.name)
            .env(ezzmc_auth::ENV_UUID, &s.uuid)
            .env(ezzmc_auth::ENV_TOKEN, &s.token);
    }
    if !st.server.trim().is_empty() {
        cmd.arg("--connect").arg(st.server.trim());
    }
    cmd.args(st.game_args.split_whitespace());
    Ok(cmd)
}

pub fn build_java(cfg: &Config, s: &Session, progress: java::Progress) -> Result<Command, String> {
    let st = &cfg.settings;
    let game = PathBuf::from(&st.game_dir);
    std::fs::create_dir_all(&game).map_err(|e| e.to_string())?;
    let version = st.java_version.trim();
    if version.is_empty() {
        return Err("не выбрана версия игры".into());
    }
    let pr = java::prepare(&game, version, progress)?;
    let jpath = if st.java_path.trim().is_empty() {
        java::find_javas()
            .into_iter()
            .filter(|(_, m)| *m >= pr.java_major)
            .max_by_key(|(_, m)| *m)
            .map(|(p, _)| p)
            .ok_or_else(|| {
                format!(
                    "нужна Java {}+, не найдена (укажите в настройках)",
                    pr.java_major
                )
            })?
    } else {
        st.java_path.trim().to_string()
    };
    if let Some(m) = java::java_major(&jpath) {
        if m < pr.java_major {
            return Err(format!(
                "{version} требует Java {}+, а {jpath} — Java {m}",
                pr.java_major
            ));
        }
    }
    let args = java::build_args(
        &pr,
        &java::LaunchOpts {
            game_dir: &game,
            version,
            name: &s.name,
            uuid: &s.uuid,
            access_token: &s.token,
            user_type: if s.online { "msa" } else { "legacy" },
            server: st.server.trim(),
            fullscreen: st.fullscreen,
            ram_min: st.ram_min_mb,
            ram_max: st.ram_max_mb,
            jvm_args: &st.jvm_args,
            game_args: &st.game_args,
        },
    );
    let mut cmd = Command::new(jpath);
    cmd.current_dir(&game).args(args);
    Ok(cmd)
}

pub fn spawn(mut cmd: Command, capture: bool) -> Result<std::process::Child, String> {
    let io = || {
        if capture {
            Stdio::piped()
        } else {
            Stdio::null()
        }
    };
    cmd.stdout(io())
        .stderr(io())
        .stdin(Stdio::null())
        .spawn()
        .map_err(|e| format!("не удалось запустить: {e}"))
}
