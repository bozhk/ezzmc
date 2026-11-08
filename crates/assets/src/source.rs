//! Источники ресурсов (TODO-090): jar/zip через mmap без блокировок, каталог, объекты по hash.
//! Индекс zip строится один раз; чтение — параллельное, без `&mut`.
use crate::error::{io, AssetError, AssetResult};
use memmap2::Mmap;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Источник файлов по ресурсному пути вида `assets/minecraft/textures/block/stone.png`.
pub trait ResourceSource: Send + Sync {
    fn name(&self) -> &str;
    fn read(&self, path: &str) -> Option<Vec<u8>>;
    fn exists(&self, path: &str) -> bool;
    /// Все пути с данным префиксом (каталогом).
    fn list(&self, prefix: &str) -> Vec<String>;
}

#[derive(Clone, Copy)]
struct Entry {
    method: u16,
    start: usize,
    comp: usize,
    size: usize,
}

/// zip/jar поверх mmap.
pub struct ZipSource {
    name: String,
    map: Mmap,
    index: HashMap<String, Entry>,
}

impl ZipSource {
    pub fn open(path: &Path) -> AssetResult<Self> {
        let file = File::open(path).map_err(|e| io(path, e))?;
        // SAFETY: файл только читается; изменение jar во время работы — вне модели (как и в ванили).
        let map = unsafe { Mmap::map(&file) }.map_err(|e| io(path, e))?;
        let zerr = |e: zip::result::ZipError| AssetError::Zip {
            path: path.display().to_string(),
            msg: e.to_string(),
        };
        let mut index = HashMap::new();
        {
            let mut ar = zip::ZipArchive::new(std::io::Cursor::new(&map[..])).map_err(zerr)?;
            for i in 0..ar.len() {
                let f = ar.by_index_raw(i).map_err(zerr)?;
                if f.is_dir() {
                    continue;
                }
                let method = match f.compression() {
                    zip::CompressionMethod::Stored => 0,
                    zip::CompressionMethod::Deflated => 8,
                    _ => continue, // неподдерживаемый метод — запись недоступна
                };
                // enclosed_name защищает от `..`/абсолютных путей.
                let Some(name) = f.enclosed_name() else {
                    continue;
                };
                let key = name.to_string_lossy().replace('\\', "/");
                index.insert(
                    key,
                    Entry {
                        method,
                        start: f.data_start() as usize,
                        comp: f.compressed_size() as usize,
                        size: f.size() as usize,
                    },
                );
            }
        }
        Ok(ZipSource {
            name: path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default(),
            map,
            index,
        })
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

impl ResourceSource for ZipSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn read(&self, path: &str) -> Option<Vec<u8>> {
        let e = *self.index.get(path)?;
        let raw = self.map.get(e.start..e.start.checked_add(e.comp)?)?;
        match e.method {
            0 => Some(raw.to_vec()),
            _ => {
                let mut out = vec![0u8; e.size];
                let n = libdeflater::Decompressor::new()
                    .deflate_decompress(raw, &mut out)
                    .ok()?;
                (n == e.size).then_some(out)
            }
        }
    }

    fn exists(&self, path: &str) -> bool {
        self.index.contains_key(path)
    }

    fn list(&self, prefix: &str) -> Vec<String> {
        let mut v: Vec<String> = self
            .index
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        v.sort();
        v
    }
}

/// Каталог как источник (распакованный ресурспак).
pub struct DirSource {
    name: String,
    root: PathBuf,
}

impl DirSource {
    pub fn new(root: &Path) -> Self {
        DirSource {
            name: root
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default(),
            root: root.to_path_buf(),
        }
    }

    /// Безопасный путь внутри корня (без `..` и абсолютных).
    fn resolve(&self, path: &str) -> Option<PathBuf> {
        let p = Path::new(path);
        if p.components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return None;
        }
        Some(self.root.join(p))
    }
}

impl ResourceSource for DirSource {
    fn name(&self) -> &str {
        &self.name
    }
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        std::fs::read(self.resolve(path)?).ok()
    }
    fn exists(&self, path: &str) -> bool {
        self.resolve(path).is_some_and(|p| p.is_file())
    }
    fn list(&self, prefix: &str) -> Vec<String> {
        fn walk(dir: &Path, rel: &str, out: &mut Vec<String>) {
            let Ok(rd) = std::fs::read_dir(dir) else {
                return;
            };
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                let r = if rel.is_empty() {
                    name.clone()
                } else {
                    format!("{rel}/{name}")
                };
                match e.file_type() {
                    Ok(t) if t.is_dir() => walk(&e.path(), &r, out),
                    Ok(t) if t.is_file() => out.push(r),
                    _ => {}
                }
            }
        }
        let mut all = Vec::new();
        walk(&self.root, "", &mut all);
        all.retain(|p| p.starts_with(prefix));
        all.sort();
        all
    }
}

#[derive(Deserialize)]
struct IndexFile {
    objects: HashMap<String, IndexObject>,
}

#[derive(Deserialize)]
struct IndexObject {
    hash: String,
}

/// `assets/indexes/<id>.json` + `assets/objects/<2 hex>/<hash>`.
/// Ключи индекса вида `minecraft/sounds/ambient/cave/cave1.ogg` или `icons/icon_16x16.png`.
pub struct ObjectStore {
    objects_dir: PathBuf,
    map: HashMap<String, String>,
}

impl ObjectStore {
    pub fn open(index: &Path, objects_dir: &Path) -> AssetResult<Self> {
        let data = std::fs::read(index).map_err(|e| io(index, e))?;
        let idx: IndexFile = serde_json::from_slice(&data).map_err(|e| AssetError::Json {
            path: index.display().to_string(),
            msg: e.to_string(),
        })?;
        Ok(ObjectStore {
            objects_dir: objects_dir.to_path_buf(),
            map: idx.objects.into_iter().map(|(k, v)| (k, v.hash)).collect(),
        })
    }

    fn object_path(&self, key: &str) -> Option<PathBuf> {
        let h = self.map.get(key)?;
        // hash — только hex (защита от выхода из каталога через подделанный индекс).
        if h.len() < 2 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        Some(self.objects_dir.join(&h[..2]).join(h))
    }
}

impl ResourceSource for ObjectStore {
    fn name(&self) -> &str {
        "assets/objects"
    }
    /// Путь вида `assets/minecraft/sounds/x.ogg` → ключ индекса `minecraft/sounds/x.ogg`.
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        let key = path.strip_prefix("assets/")?;
        std::fs::read(self.object_path(key)?).ok()
    }
    fn exists(&self, path: &str) -> bool {
        path.strip_prefix("assets/")
            .and_then(|k| self.object_path(k))
            .is_some_and(|p| p.is_file())
    }
    fn list(&self, prefix: &str) -> Vec<String> {
        let Some(p) = prefix.strip_prefix("assets/") else {
            return Vec::new();
        };
        let mut v: Vec<String> = self
            .map
            .keys()
            .filter(|k| k.starts_with(p))
            .map(|k| format!("assets/{k}"))
            .collect();
        v.sort();
        v
    }
}

/// Проверка sha1 объекта (для диагностики/кэша).
pub fn sha1_hex(data: &[u8]) -> String {
    use sha1::{Digest, Sha1};
    let mut h = Sha1::new();
    h.update(data);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_zip(path: &Path) {
        let f = File::create(path).unwrap();
        let mut z = zip::ZipWriter::new(f);
        let o = zip::write::SimpleFileOptions::default();
        z.start_file(
            "assets/minecraft/a.txt",
            o.compression_method(zip::CompressionMethod::Deflated),
        )
        .unwrap();
        z.write_all(&b"hello hello hello hello hello".repeat(50))
            .unwrap();
        z.start_file(
            "assets/minecraft/b.bin",
            o.compression_method(zip::CompressionMethod::Stored),
        )
        .unwrap();
        z.write_all(&[1, 2, 3]).unwrap();
        z.start_file("assets/other/c.txt", o).unwrap();
        z.write_all(b"c").unwrap();
        z.finish().unwrap();
    }

    #[test]
    fn zip_read_list_exists() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("t.jar");
        make_zip(&p);
        let z = ZipSource::open(&p).unwrap();
        assert_eq!(z.len(), 3);
        assert_eq!(z.read("assets/minecraft/a.txt").unwrap().len(), 29 * 50);
        assert_eq!(z.read("assets/minecraft/b.bin").unwrap(), vec![1, 2, 3]);
        assert!(z.read("nope").is_none());
        assert!(z.exists("assets/other/c.txt"));
        assert_eq!(z.list("assets/minecraft/").len(), 2);
    }

    #[test]
    fn dir_source_blocks_traversal() {
        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(d.path().join("assets/minecraft")).unwrap();
        std::fs::write(d.path().join("assets/minecraft/x.txt"), b"x").unwrap();
        std::fs::write(d.path().join("secret.txt"), b"s").unwrap();
        let s = DirSource::new(&d.path().join("assets"));
        assert!(s.read("minecraft/x.txt").is_some());
        assert!(s.read("../secret.txt").is_none());
        assert!(s.read("/etc/passwd").is_none());
        assert_eq!(s.list("minecraft"), vec!["minecraft/x.txt".to_string()]);
    }

    #[test]
    fn object_store_by_hash() {
        let d = tempfile::tempdir().unwrap();
        let data = b"ogg";
        let h = sha1_hex(data);
        std::fs::create_dir_all(d.path().join("objects").join(&h[..2])).unwrap();
        std::fs::write(d.path().join("objects").join(&h[..2]).join(&h), data).unwrap();
        let idx = d.path().join("idx.json");
        std::fs::write(
            &idx,
            format!(r#"{{"objects":{{"minecraft/sounds/a.ogg":{{"hash":"{h}","size":3}},"evil":{{"hash":"../../x","size":1}}}}}}"#),
        )
        .unwrap();
        let s = ObjectStore::open(&idx, &d.path().join("objects")).unwrap();
        assert_eq!(s.read("assets/minecraft/sounds/a.ogg").unwrap(), data);
        assert!(s.read("assets/evil").is_none());
        assert_eq!(sha1_hex(b""), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }
}
