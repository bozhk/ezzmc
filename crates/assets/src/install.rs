//! Поиск установленной игры (TODO-089): jar клиента + каталог assets (индекс и объекты).
//! Порядок: env `EZZMC_CLIENT_JAR` / `EZZMC_MINECRAFT_DIR`, затем стандартные `.minecraft`,
//! затем dev-запасной `reference/_cache/<v>/client.jar` (официальный клиент, только ресурсы).
use crate::error::{io, AssetError, AssetResult};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GameInstall {
    pub version: String,
    pub jar: PathBuf,
    /// `<minecraft>/assets` (с `indexes/` и `objects/`), если найден.
    pub assets_dir: Option<PathBuf>,
    /// id индекса ассетов из version json (например `"29"`), если известен.
    pub asset_index: Option<String>,
}

/// Кандидаты корней `.minecraft` в порядке приоритета.
pub fn minecraft_dirs() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(d) = std::env::var_os("EZZMC_MINECRAFT_DIR") {
        v.push(PathBuf::from(d));
    }
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        v.push(home.join(".minecraft"));
        v.push(home.join(".var/app/com.mojang.Minecraft/data/minecraft"));
        v.push(home.join("Library/Application Support/minecraft"));
    }
    if let Some(a) = std::env::var_os("APPDATA").map(PathBuf::from) {
        v.push(a.join(".minecraft"));
    }
    v
}

fn asset_index_id(version_json: &Path) -> Option<String> {
    let data = std::fs::read(version_json).ok()?;
    let j: serde_json::Value = serde_json::from_slice(&data).ok()?;
    j.get("assetIndex")?.get("id")?.as_str().map(str::to_owned)
}

impl GameInstall {
    /// Найти установку версии `version` (например `1.21.11`).
    pub fn locate(version: &str) -> AssetResult<Self> {
        let mut tried = Vec::new();
        let dirs = minecraft_dirs();
        let assets_of = |dirs: &[PathBuf]| {
            dirs.iter()
                .map(|d| d.join("assets"))
                .find(|a| a.join("indexes").is_dir() && a.join("objects").is_dir())
        };
        if let Some(jar) = std::env::var_os("EZZMC_CLIENT_JAR").map(PathBuf::from) {
            if jar.is_file() {
                return Ok(GameInstall {
                    version: version.to_owned(),
                    jar,
                    assets_dir: assets_of(&dirs),
                    asset_index: None,
                });
            }
            tried.push(jar.display().to_string());
        }
        for d in &dirs {
            let vdir = d.join("versions").join(version);
            let jar = vdir.join(format!("{version}.jar"));
            if jar.is_file() {
                let asset_index = asset_index_id(&vdir.join(format!("{version}.json")));
                return Ok(GameInstall {
                    version: version.to_owned(),
                    jar,
                    assets_dir: Some(d.join("assets")).filter(|a| a.is_dir()),
                    asset_index,
                });
            }
            tried.push(jar.display().to_string());
        }
        // dev-запасной: кэш fetch-reference (ищем вверх от cwd, затем от exe).
        let starts = [std::env::current_dir().ok(), std::env::current_exe().ok()];
        for s in starts.into_iter().flatten() {
            for a in s.ancestors() {
                let jar = a.join("reference/_cache").join(version).join("client.jar");
                if jar.is_file() {
                    return Ok(GameInstall {
                        version: version.to_owned(),
                        jar,
                        assets_dir: assets_of(&dirs),
                        asset_index: None,
                    });
                }
            }
        }
        Err(AssetError::NotFound(format!(
            "версия {version}; искали: {}",
            tried.join(", ")
        )))
    }

    /// Менеджер ресурсов с ванильным jar как единственным паком (ресурспаки добавляются сверху).
    pub fn resource_manager(&self) -> AssetResult<crate::pack::ResourceManager> {
        let z = crate::source::ZipSource::open(&self.jar)?;
        Ok(crate::pack::ResourceManager::new(vec![
            crate::pack::Pack::new("vanilla", Box::new(z)),
        ]))
    }

    /// Путь индекса: известный id, иначе самый свежий по номеру в `indexes/`.
    pub fn index_path(&self) -> AssetResult<Option<PathBuf>> {
        let Some(a) = &self.assets_dir else {
            return Ok(None);
        };
        let idx = a.join("indexes");
        if let Some(id) = &self.asset_index {
            let p = idx.join(format!("{id}.json"));
            if p.is_file() {
                return Ok(Some(p));
            }
        }
        let rd = match std::fs::read_dir(&idx) {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };
        let mut best: Option<(u64, PathBuf)> = None;
        for e in rd {
            let e = e.map_err(|e| io(&idx, e))?;
            let p = e.path();
            let n = p
                .file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.parse::<u64>().ok());
            if let (Some(n), Some("json")) = (n, p.extension().and_then(|s| s.to_str())) {
                if best.as_ref().is_none_or(|(b, _)| n > *b) {
                    best = Some((n, p));
                }
            }
        }
        Ok(best.map(|(_, p)| p))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn index_picks_known_then_latest() {
        let d = tempfile::tempdir().unwrap();
        let idx = d.path().join("assets/indexes");
        std::fs::create_dir_all(&idx).unwrap();
        for n in ["5", "19", "34"] {
            std::fs::write(idx.join(format!("{n}.json")), "{}").unwrap();
        }
        std::fs::write(idx.join("1.12.json"), "{}").unwrap();
        let mut g = GameInstall {
            version: "x".into(),
            jar: PathBuf::new(),
            assets_dir: Some(d.path().join("assets")),
            asset_index: None,
        };
        assert!(g.index_path().unwrap().unwrap().ends_with("34.json"));
        g.asset_index = Some("19".into());
        assert!(g.index_path().unwrap().unwrap().ends_with("19.json"));
    }

    #[test]
    fn version_json_asset_index() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("v.json");
        std::fs::write(&p, r#"{"assetIndex":{"id":"29","sha1":"x"}}"#).unwrap();
        assert_eq!(asset_index_id(&p).as_deref(), Some("29"));
    }

    #[test]
    fn locate_dev_fallback_jar_readable() {
        // Живая проверка на кэше fetch-reference, если он есть.
        let Ok(g) = GameInstall::locate("26.3") else {
            return;
        };
        let z = crate::source::ZipSource::open(&g.jar).unwrap();
        use crate::source::ResourceSource;
        let png = z
            .read("assets/minecraft/textures/colormap/grass.png")
            .unwrap();
        assert_eq!(&png[1..4], b"PNG");
        assert!(z.exists("assets/minecraft/lang/en_us.json"));
    }
}
