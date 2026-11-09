//! Репозиторий ресурспаков: `./resourcepacks` (каталоги и .zip с `pack.mcmeta`), список выбранных
//! в options.txt (`resourcePacks:["vanilla","file/x.zip"]`, порядок снизу вверх).
//! VANILLA: PackRepository / FolderRepositorySource / Options#resourcePacks (26.3).
use ezzmc_assets::install::GameInstall;
use ezzmc_assets::pack::{Compat, Pack, PackMeta, ResourceManager};
use ezzmc_assets::source::{DirSource, ResourceSource, ZipSource};
use ezzmc_assets::AssetResult;
use std::path::{Path, PathBuf};

pub const VANILLA: &str = "vanilla";

#[derive(Clone, Debug)]
pub struct PackInfo {
    /// `file/<имя>` (или `vanilla`).
    pub id: String,
    pub title: String,
    pub desc: String,
    pub compat: Option<Compat>,
    /// `pack.png` (RGBA, ширина, высота).
    pub icon: Option<(Vec<u8>, u32, u32)>,
}

pub fn dir() -> PathBuf {
    PathBuf::from("resourcepacks")
}

fn open(id: &str) -> Option<Box<dyn ResourceSource>> {
    let p = dir().join(id.strip_prefix("file/")?);
    if p.is_dir() {
        Some(Box::new(DirSource::new(&p)))
    } else {
        ZipSource::open(&p)
            .ok()
            .map(|z| Box::new(z) as Box<dyn ResourceSource>)
    }
}

/// Описание из pack.mcmeta: строка или компонент `{"text":..}` (без разбора стилей).
fn desc_text(m: &PackMeta) -> String {
    fn walk(v: &serde_json::Value, out: &mut String) {
        match v {
            serde_json::Value::String(s) => out.push_str(s),
            serde_json::Value::Object(o) => {
                if let Some(t) = o.get("text") {
                    walk(t, out);
                }
                if let Some(serde_json::Value::Array(a)) = o.get("extra") {
                    a.iter().for_each(|e| walk(e, out));
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|e| walk(e, out)),
            _ => {}
        }
    }
    let mut s = String::new();
    if let Some(d) = &m.description {
        walk(d, &mut s);
    }
    s
}

/// Все валидные паки в `resourcepacks/` (сортировка по имени без регистра, как в ванили).
pub fn scan() -> Vec<PackInfo> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir()) else {
        return out;
    };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let p: &Path = &e.path();
        if !p.is_dir() && !name.to_lowercase().ends_with(".zip") {
            continue;
        }
        let id = format!("file/{name}");
        let Some(src) = open(&id) else { continue };
        // Пак без pack.mcmeta не считается паком.
        let Some(meta) = src.read("pack.mcmeta").and_then(|d| PackMeta::parse(&d)) else {
            continue;
        };
        let icon = src
            .read("pack.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            .map(|i| (i.to_rgba8(), i.width, i.height));
        let pack = Pack::new(&id, src);
        out.push(PackInfo {
            title: name,
            desc: desc_text(&meta),
            compat: pack.compat(),
            icon,
            id,
        });
    }
    out.sort_by_key(|p| p.title.to_lowercase());
    out
}

/// `resourcePacks` из options.txt → идентификаторы снизу вверх; `vanilla` всегда первый.
pub fn parse_selected(raw: &str) -> Vec<String> {
    let mut v: Vec<String> = serde_json::from_str(raw).unwrap_or_default();
    v.retain(|s| s != VANILLA);
    v.insert(0, VANILLA.to_owned());
    v
}

pub fn encode_selected(sel: &[String]) -> String {
    serde_json::to_string(sel).unwrap_or_else(|_| "[\"vanilla\"]".into())
}

/// Ресурсы игры + выбранные паки сверху (VANILLA: PackRepository#openAllSelected). Битые/пропавшие — пропускаются.
pub fn build_resource_manager(install: &GameInstall) -> AssetResult<ResourceManager> {
    let mut rm = install.resource_manager()?;
    let opts = crate::ui::options::Options::load(Path::new("./options.txt"));
    let raw = opts.raw("resourcePacks").unwrap_or("[\"vanilla\"]");
    for id in parse_selected(raw).iter().filter(|i| *i != VANILLA) {
        match open(id) {
            Some(src) => rm.push(Pack::new(id, src)),
            None => tracing::warn!("ресурспак {id} не найден — пропущен"),
        }
    }
    Ok(rm)
}

/// Drag-and-drop: копирует zip или каталог пака в `resourcepacks/`.
/// VANILLA: PackSelectionScreen#onFilesDrop / copyPacks. Возвращает, добавлен ли пак.
pub fn import(src: &Path) -> bool {
    let Some(name) = src.file_name() else {
        return false;
    };
    let _ = std::fs::create_dir_all(dir());
    let dst = dir().join(name);
    if dst.exists() {
        return false;
    }
    if src.is_dir() {
        fn copy(a: &Path, b: &Path) -> std::io::Result<()> {
            std::fs::create_dir_all(b)?;
            for e in std::fs::read_dir(a)? {
                let e = e?;
                let t = b.join(e.file_name());
                if e.path().is_dir() {
                    copy(&e.path(), &t)?;
                } else {
                    std::fs::copy(e.path(), &t)?;
                }
            }
            Ok(())
        }
        copy(src, &dst).is_ok()
    } else if src
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zip"))
    {
        std::fs::copy(src, &dst).is_ok()
    } else {
        false
    }
}
