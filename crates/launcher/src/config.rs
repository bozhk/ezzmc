//! Конфиг лаунчера: `~/.config/ezzmc-launcher/config.json` (права 0600: внутри refresh-токены).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Offline,
    Microsoft,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Account {
    pub kind: Kind,
    pub name: String,
    /// UUID без дефисов.
    pub uuid: String,
    #[serde(default)]
    pub refresh_token: String,
    #[serde(default)]
    pub mc_token: String,
    #[serde(default)]
    pub mc_expires: u64,
}

impl Account {
    pub fn offline(name: &str) -> Self {
        let u = ezzmc_protocol::types::offline_uuid(name);
        Self {
            kind: Kind::Offline,
            name: name.to_string(),
            uuid: format!("{u:032x}"),
            refresh_token: String::new(),
            mc_token: String::new(),
            mc_expires: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Settings {
    /// 0 = ezzmc (Rust), 1 = ванильная Java.
    pub target: i32,
    /// Папка игры (.minecraft): версии, библиотеки, ассеты, сохранения Java; ezzmc берёт ресурсы отсюда.
    pub game_dir: String,
    pub java_path: String,
    pub ram_min_mb: i32,
    pub ram_max_mb: i32,
    pub jvm_args: String,
    pub game_args: String,
    pub java_version: String,
    /// Путь к `ezzmc-client` (пусто = искать рядом / в target/).
    pub client_path: String,
    /// Рабочая папка ezzmc (options.txt, servers.dat, скриншоты).
    pub client_dir: String,
    pub server: String,
    pub msa_client_id: String,
    pub close_on_launch: bool,
    pub fullscreen: bool,
}

pub fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target: 0,
            game_dir: home().join(".minecraft").to_string_lossy().into_owned(),
            java_path: String::new(),
            ram_min_mb: 1024,
            ram_max_mb: 4096,
            jvm_args: "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200".into(),
            game_args: String::new(),
            java_version: "26.3".into(),
            client_path: String::new(),
            client_dir: data_dir().join("ezzmc").to_string_lossy().into_owned(),
            server: String::new(),
            msa_client_id: std::env::var("EZZMC_MSA_CLIENT_ID").unwrap_or_default(),
            close_on_launch: false,
            fullscreen: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub accounts: Vec<Account>,
    pub selected: usize,
    pub settings: Settings,
}

pub fn config_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
        .join("ezzmc-launcher")
}

pub fn data_dir() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".local/share"))
        .join("ezzmc-launcher")
}

fn path() -> PathBuf {
    config_dir().join("config.json")
}

impl Config {
    pub fn load() -> Self {
        let mut c: Config = std::fs::read(path())
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        if c.accounts.is_empty() {
            c.accounts.push(Account::offline("EzzBot"));
        }
        c.selected = c.selected.min(c.accounts.len() - 1);
        c
    }

    pub fn save(&self) -> std::io::Result<()> {
        let p = path();
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d)?;
        }
        let tmp = p.with_extension("json.tmp");
        std::fs::write(
            &tmp,
            serde_json::to_vec_pretty(self).map_err(std::io::Error::other)?,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600))?;
        }
        std::fs::rename(tmp, p)
    }

    pub fn msa_client_id(&self) -> String {
        let s = self.settings.msa_client_id.trim();
        if s.is_empty() {
            std::env::var("EZZMC_MSA_CLIENT_ID").unwrap_or_default()
        } else {
            s.to_string()
        }
    }
}

pub fn valid_nick(n: &str) -> bool {
    (3..=16).contains(&n.len()) && n.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nick_rules() {
        assert!(valid_nick("EzzBot"));
        assert!(!valid_nick("ab"));
        assert!(!valid_nick("bad name"));
    }
    #[test]
    fn offline_uuid_is_v3() {
        let a = Account::offline("Notch");
        assert_eq!(a.uuid.len(), 32);
        assert_eq!(&a.uuid[12..13], "3");
    }
}
