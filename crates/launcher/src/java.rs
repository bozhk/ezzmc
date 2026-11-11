//! Ванильная Java: манифест версий, скачивание клиента/библиотек/ассетов, сборка командной строки.
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type Progress<'a> = &'a (dyn Fn(&str, f32) + Sync);
type R<T> = Result<T, String>;

const MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const RESOURCES: &str = "https://resources.download.minecraft.net";

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(10))
        .timeout_read(Duration::from_secs(60))
        .build()
}

fn get_json(url: &str) -> R<Value> {
    agent()
        .get(url)
        .call()
        .map_err(|e| format!("{url}: {e}"))?
        .into_json()
        .map_err(|e| format!("{url}: {e}"))
}

/// Версии из манифеста Mojang (id, url) — только релизы и снапшоты по порядку манифеста.
pub fn remote_versions() -> R<Vec<(String, String)>> {
    let m = get_json(MANIFEST)?;
    Ok(m["versions"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| {
                    let t = v["type"].as_str()?;
                    (t == "release").then_some((
                        v["id"].as_str()?.to_string(),
                        v["url"].as_str()?.to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default())
}

/// Установленные версии в `<game>/versions`.
pub fn installed_versions(game: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(game.join("versions"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            e.path().join(format!("{n}.json")).is_file().then_some(n)
        })
        .collect();
    v.sort();
    v.reverse();
    v
}

fn sha1_hex(data: &[u8]) -> String {
    Sha1::digest(data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn file_ok(p: &Path, sha1: Option<&str>) -> bool {
    match (std::fs::read(p), sha1) {
        (Ok(_), None) => true,
        (Ok(d), Some(h)) => sha1_hex(&d).eq_ignore_ascii_case(h),
        _ => false,
    }
}

fn download(url: &str, dest: &Path, sha1: Option<&str>) -> R<()> {
    if file_ok(dest, sha1) {
        return Ok(());
    }
    if let Some(d) = dest.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let mut last = String::new();
    for _ in 0..3 {
        let mut buf = Vec::new();
        match agent().get(url).call() {
            Ok(r) => {
                use std::io::Read;
                if let Err(e) = r.into_reader().read_to_end(&mut buf) {
                    last = e.to_string();
                    continue;
                }
                if let Some(h) = sha1 {
                    if !sha1_hex(&buf).eq_ignore_ascii_case(h) {
                        last = "контрольная сумма не совпала".into();
                        continue;
                    }
                }
                let tmp = dest.with_extension("part");
                std::fs::write(&tmp, &buf).map_err(|e| e.to_string())?;
                return std::fs::rename(tmp, dest).map_err(|e| e.to_string());
            }
            Err(e) => last = e.to_string(),
        }
    }
    Err(format!("{url}: {last}"))
}

// ---------- правила ----------

fn os_name() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "osx"
    } else {
        "linux"
    }
}

fn arch_name() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x86"
    }
}

/// Правила Mojang: последнее подходящее решает; пусто = разрешено.
fn rules_allow(rules: Option<&Value>, features: &dyn Fn(&str) -> bool) -> bool {
    let Some(rules) = rules.and_then(Value::as_array) else {
        return true;
    };
    let mut allowed = false;
    for r in rules {
        let mut m = true;
        if let Some(os) = r.get("os") {
            if let Some(n) = os["name"].as_str() {
                m &= n == os_name();
            }
            if let Some(a) = os["arch"].as_str() {
                m &= a == arch_name();
            }
        }
        if let Some(f) = r.get("features").and_then(Value::as_object) {
            for (k, v) in f {
                m &= features(k) == v.as_bool().unwrap_or(false);
            }
        }
        if m {
            allowed = r["action"].as_str() == Some("allow");
        }
    }
    allowed
}

pub struct Prepared {
    pub version_json: Value,
    pub classpath: Vec<PathBuf>,
    pub natives_dir: PathBuf,
    pub asset_index: String,
    pub main_class: String,
    pub java_major: u32,
}

fn load_version(game: &Path, id: &str, p: Progress) -> R<Value> {
    let local = game.join("versions").join(id).join(format!("{id}.json"));
    if let Ok(b) = std::fs::read(&local) {
        return serde_json::from_slice(&b).map_err(|e| format!("{id}.json: {e}"));
    }
    p("Загрузка манифеста версий", 0.0);
    let url = remote_versions()?
        .into_iter()
        .find(|(v, _)| v == id)
        .map(|(_, u)| u)
        .ok_or_else(|| format!("версия {id} не найдена в манифесте Mojang"))?;
    let v = get_json(&url)?;
    std::fs::create_dir_all(local.parent().ok_or("путь")?).map_err(|e| e.to_string())?;
    std::fs::write(&local, serde_json::to_vec(&v).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(v)
}

/// Скачать недостающее и вернуть всё нужное для запуска.
pub fn prepare(game: &Path, id: &str, p: Progress) -> R<Prepared> {
    let v = load_version(game, id, p)?;
    if v.get("inheritsFrom").is_some() {
        return Err("версии с inheritsFrom (Fabric/Forge) не поддерживаются".into());
    }
    let no_features = |_: &str| false;
    // клиентский jar
    let jar = game.join("versions").join(id).join(format!("{id}.jar"));
    if let Some(c) = v["downloads"]["client"].as_object() {
        p("Клиент Minecraft", 0.02);
        download(
            c["url"].as_str().unwrap_or_default(),
            &jar,
            c["sha1"].as_str(),
        )?;
    } else if !jar.is_file() {
        return Err(format!("нет client.jar для {id}"));
    }

    // библиотеки
    let libs: Vec<Value> = v["libraries"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|l| rules_allow(l.get("rules"), &no_features))
        .collect();
    let natives_dir = game.join("versions").join(id).join("natives");
    let mut classpath = Vec::new();
    let total = libs.len().max(1);
    for (i, l) in libs.iter().enumerate() {
        p(
            &format!("Библиотеки {}/{}", i + 1, total),
            0.05 + 0.25 * i as f32 / total as f32,
        );
        let d = &l["downloads"];
        if let Some(a) = d.get("artifact").filter(|a| a.get("path").is_some()) {
            let path = game
                .join("libraries")
                .join(a["path"].as_str().unwrap_or_default());
            download(
                a["url"].as_str().unwrap_or_default(),
                &path,
                a["sha1"].as_str(),
            )?;
            classpath.push(path);
        }
        // старые версии: natives по классификатору
        if let Some(cls) = l["natives"][os_name()].as_str() {
            let cls = cls.replace(
                "${arch}",
                if cfg!(target_pointer_width = "64") {
                    "64"
                } else {
                    "32"
                },
            );
            if let Some(a) = d["classifiers"].get(&cls) {
                let path = game
                    .join("libraries")
                    .join(a["path"].as_str().unwrap_or_default());
                download(
                    a["url"].as_str().unwrap_or_default(),
                    &path,
                    a["sha1"].as_str(),
                )?;
                extract_natives(&path, &natives_dir)?;
            }
        }
    }
    std::fs::create_dir_all(&natives_dir).map_err(|e| e.to_string())?;
    classpath.push(jar);

    // ассеты
    let ai = &v["assetIndex"];
    let ai_id = ai["id"]
        .as_str()
        .or(v["assets"].as_str())
        .unwrap_or("legacy")
        .to_string();
    let idx_path = game.join("assets/indexes").join(format!("{ai_id}.json"));
    if let Some(url) = ai["url"].as_str() {
        p("Индекс ассетов", 0.3);
        download(url, &idx_path, ai["sha1"].as_str())?;
    }
    if let Ok(b) = std::fs::read(&idx_path) {
        let idx: Value = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
        download_assets(game, &idx, p)?;
    }

    Ok(Prepared {
        classpath,
        natives_dir,
        asset_index: ai_id,
        main_class: v["mainClass"]
            .as_str()
            .unwrap_or("net.minecraft.client.main.Main")
            .into(),
        java_major: v["javaVersion"]["majorVersion"].as_u64().unwrap_or(8) as u32,
        version_json: v,
    })
}

fn extract_natives(jar: &Path, dest: &Path) -> R<()> {
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let f = std::fs::File::open(jar).map_err(|e| e.to_string())?;
    let mut z = zip::ZipArchive::new(f).map_err(|e| e.to_string())?;
    for i in 0..z.len() {
        let mut e = z.by_index(i).map_err(|e| e.to_string())?;
        let name = e.name().to_string();
        if e.is_dir() || name.starts_with("META-INF") || name.contains("..") {
            continue;
        }
        let out = dest.join(&name);
        if let Some(d) = out.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let mut o = std::fs::File::create(out).map_err(|e| e.to_string())?;
        std::io::copy(&mut e, &mut o).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn download_assets(game: &Path, idx: &Value, p: Progress) -> R<()> {
    let objs = idx["objects"]
        .as_object()
        .ok_or("индекс ассетов без objects")?;
    let virtual_ = idx["virtual"].as_bool().unwrap_or(false)
        || idx["map_to_resources"].as_bool().unwrap_or(false);
    let todo: Vec<(String, String, String)> = objs
        .iter()
        .filter_map(|(name, o)| {
            let h = o["hash"].as_str()?.to_string();
            Some((name.clone(), h.clone(), h[..2].to_string()))
        })
        .filter(|(_, h, pre)| {
            let path = game.join("assets/objects").join(pre).join(h);
            !path.is_file()
        })
        .collect();
    let total = todo.len();
    let queue = Arc::new(Mutex::new(todo));
    let done = Arc::new(AtomicUsize::new(0));
    let err: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    std::thread::scope(|s| {
        for _ in 0..12 {
            let (queue, done, err) = (queue.clone(), done.clone(), err.clone());
            s.spawn(move || loop {
                let item = queue.lock().ok().and_then(|mut q| q.pop());
                let Some((_, h, pre)) = item else { break };
                let dest = game.join("assets/objects").join(&pre).join(&h);
                if let Err(e) = download(&format!("{RESOURCES}/{pre}/{h}"), &dest, Some(&h)) {
                    if let Ok(mut g) = err.lock() {
                        g.get_or_insert(e);
                    }
                    break;
                }
                let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                if n % 25 == 0 || n == total {
                    p(
                        &format!("Ассеты {n}/{total}"),
                        0.3 + 0.7 * n as f32 / total.max(1) as f32,
                    );
                }
            });
        }
    });
    if let Some(e) = err.lock().ok().and_then(|mut g| g.take()) {
        return Err(e);
    }
    if virtual_ {
        for (name, o) in objs {
            let h = o["hash"].as_str().unwrap_or_default();
            if h.len() < 2 {
                continue;
            }
            let src = game.join("assets/objects").join(&h[..2]).join(h);
            let dst = game.join("assets/virtual/legacy").join(name);
            if !dst.is_file() && !name.contains("..") {
                if let Some(d) = dst.parent() {
                    let _ = std::fs::create_dir_all(d);
                }
                let _ = std::fs::copy(src, dst);
            }
        }
    }
    Ok(())
}

// ---------- запуск ----------

pub struct LaunchOpts<'a> {
    pub game_dir: &'a Path,
    pub version: &'a str,
    pub name: &'a str,
    /// UUID с дефисами или без.
    pub uuid: &'a str,
    pub access_token: &'a str,
    pub user_type: &'a str,
    pub server: &'a str,
    pub fullscreen: bool,
    pub ram_min: i32,
    pub ram_max: i32,
    pub jvm_args: &'a str,
    pub game_args: &'a str,
}

fn dashed(u: &str) -> String {
    let u = u.replace('-', "");
    if u.len() == 32 {
        format!(
            "{}-{}-{}-{}-{}",
            &u[..8],
            &u[8..12],
            &u[12..16],
            &u[16..20],
            &u[20..]
        )
    } else {
        u
    }
}

/// Аргументы после `java`.
pub fn build_args(pr: &Prepared, o: &LaunchOpts) -> Vec<String> {
    let sep = if cfg!(windows) { ";" } else { ":" };
    let cp = pr
        .classpath
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(sep);
    let (host, port) = match o.server.rsplit_once(':') {
        Some((h, p)) if p.parse::<u16>().is_ok() => (h.to_string(), p.to_string()),
        _ => (o.server.to_string(), "25565".to_string()),
    };
    let subst = |s: &str| -> String {
        let pairs: [(&str, String); 24] = [
            ("auth_player_name", o.name.into()),
            ("version_name", o.version.into()),
            ("game_directory", o.game_dir.to_string_lossy().into()),
            (
                "assets_root",
                o.game_dir.join("assets").to_string_lossy().into(),
            ),
            (
                "game_assets",
                o.game_dir
                    .join("assets/virtual/legacy")
                    .to_string_lossy()
                    .into(),
            ),
            ("assets_index_name", pr.asset_index.clone()),
            ("auth_uuid", dashed(o.uuid)),
            ("auth_access_token", o.access_token.into()),
            (
                "auth_session",
                format!("token:{}:{}", o.access_token, o.uuid.replace('-', "")),
            ),
            ("clientid", String::new()),
            ("auth_xuid", String::new()),
            ("user_type", o.user_type.into()),
            ("user_properties", "{}".into()),
            ("version_type", "release".into()),
            ("natives_directory", pr.natives_dir.to_string_lossy().into()),
            ("launcher_name", "ezzmc-launcher".into()),
            ("launcher_version", "0.1".into()),
            ("classpath", cp.clone()),
            (
                "library_directory",
                o.game_dir.join("libraries").to_string_lossy().into(),
            ),
            ("classpath_separator", sep.into()),
            ("resolution_width", "854".into()),
            ("resolution_height", "480".into()),
            ("quickPlayMultiplayer", o.server.into()),
            ("path", String::new()),
        ];
        let mut out = s.to_string();
        for (k, v) in pairs {
            out = out.replace(&format!("${{{k}}}"), &v);
        }
        out
    };
    let has_server = !o.server.trim().is_empty();
    let features = |f: &str| match f {
        "is_quick_play_multiplayer" => has_server,
        "has_quick_plays_support" => false,
        _ => false,
    };
    let collect = |key: &str| -> Vec<String> {
        let mut out = Vec::new();
        for a in pr.version_json["arguments"][key]
            .as_array()
            .into_iter()
            .flatten()
        {
            match a {
                Value::String(s) => out.push(subst(s)),
                Value::Object(_) if rules_allow(a.get("rules"), &features) => match &a["value"] {
                    Value::String(s) => out.push(subst(s)),
                    Value::Array(v) => out.extend(v.iter().filter_map(|x| x.as_str()).map(&subst)),
                    _ => {}
                },
                _ => {}
            }
        }
        out
    };
    let mut args: Vec<String> = vec![format!("-Xms{}M", o.ram_min), format!("-Xmx{}M", o.ram_max)];
    args.extend(o.jvm_args.split_whitespace().map(str::to_owned));
    let modern = pr.version_json["arguments"].is_object();
    if modern {
        args.extend(collect("jvm"));
    } else {
        args.push(format!("-Djava.library.path={}", pr.natives_dir.display()));
        args.push("-cp".into());
        args.push(cp.clone());
    }
    args.push(pr.main_class.clone());
    if modern {
        args.extend(collect("game"));
    } else if let Some(m) = pr.version_json["minecraftArguments"].as_str() {
        args.extend(m.split_whitespace().map(&subst));
    }
    // быстрый вход на сервер
    if has_server {
        if modern {
            if !args.iter().any(|a| a == "--quickPlayMultiplayer") {
                args.push("--quickPlayMultiplayer".into());
                args.push(o.server.into());
            }
        } else {
            args.extend(["--server".into(), host, "--port".into(), port]);
        }
    }
    if o.fullscreen {
        args.push("--fullscreen".into());
    }
    args.extend(o.game_args.split_whitespace().map(str::to_owned));
    args
}

/// Мажорная версия Java (`java -version`).
pub fn java_major(java: &str) -> Option<u32> {
    let out = std::process::Command::new(java)
        .arg("-version")
        .output()
        .ok()?;
    let text =
        String::from_utf8_lossy(&out.stderr).to_string() + &String::from_utf8_lossy(&out.stdout);
    let q = text.split('"').nth(1)?;
    let mut it = q.split(['.', '-', '_']);
    let first: u32 = it.next()?.parse().ok()?;
    if first == 1 {
        it.next()?.parse().ok()
    } else {
        Some(first)
    }
}

/// Найденные Java: PATH, /usr/lib/jvm, TLauncher/Mojang JRE.
pub fn find_javas() -> Vec<(String, u32)> {
    let mut cands: Vec<String> = vec!["java".into()];
    for d in ["/usr/lib/jvm", "/usr/lib64/jvm"] {
        for e in std::fs::read_dir(d).into_iter().flatten().flatten() {
            cands.push(e.path().join("bin/java").to_string_lossy().into_owned());
        }
    }
    let home = crate::config::home();
    for base in [
        home.join(".tlauncher/mojang_jre"),
        home.join(".minecraft/runtime"),
    ] {
        if let Ok(rd) = std::fs::read_dir(&base) {
            for e in rd.flatten() {
                for sub in walk_java(&e.path(), 4) {
                    cands.push(sub);
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    cands
        .into_iter()
        .filter_map(|c| {
            let canon = std::fs::canonicalize(&c)
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or(c.clone());
            if !seen.insert(canon) {
                return None;
            }
            java_major(&c).map(|m| (c, m))
        })
        .collect()
}

fn walk_java(dir: &Path, depth: u32) -> Vec<String> {
    let mut out = Vec::new();
    let j = dir.join("bin/java");
    if j.is_file() {
        out.push(j.to_string_lossy().into_owned());
        return out;
    }
    if depth == 0 {
        return out;
    }
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        if e.path().is_dir() {
            out.extend(walk_java(&e.path(), depth - 1));
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn rules() {
        let none = |_: &str| false;
        let r: Value =
            serde_json::json!([{"action":"allow"},{"action":"disallow","os":{"name":"windows"}}]);
        assert!(rules_allow(Some(&r), &none));
        let r: Value = serde_json::json!([{"action":"allow","os":{"name":"osx"}}]);
        assert!(!rules_allow(Some(&r), &none));
        let r: Value = serde_json::json!([{"action":"allow","features":{"is_demo_user":true}}]);
        assert!(!rules_allow(Some(&r), &none));
    }

    #[test]
    fn dash() {
        assert_eq!(
            dashed("00000000000000000000000000000001"),
            "00000000-0000-0000-0000-000000000001"
        );
    }
}
