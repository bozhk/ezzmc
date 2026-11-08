//! Слои ресурспаков (TODO-091): `pack.mcmeta`, формат пака, overlays, filter, порядок перекрытия.
use crate::source::ResourceSource;
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeSet;

/// Текущий ресурсный формат клиента 26.3 (version.json: resource_major 97, minor 1).
// VANILLA: SharedConstants#getCurrentVersion().packVersion(CLIENT_RESOURCES) (1.21.11)
pub const CURRENT_FORMAT: PackFormat = PackFormat {
    major: 97,
    minor: 1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackFormat {
    pub major: u32,
    pub minor: u32,
}

impl PackFormat {
    pub const fn of(major: u32) -> Self {
        PackFormat { major, minor: 0 }
    }
}

/// Диапазон `[min, max]` включительно.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatRange {
    pub min: PackFormat,
    pub max: PackFormat,
}

impl FormatRange {
    pub fn contains(&self, f: PackFormat) -> bool {
        self.min <= f && f <= self.max
    }
}

// VANILLA: PackFormat#fullCodec — `N` или `[N, M]`; для min без minor=0, для max без minor=MAX.
fn parse_format(v: &Value, default_minor: u32) -> Option<PackFormat> {
    match v {
        Value::Number(n) => Some(PackFormat {
            major: u32::try_from(n.as_u64()?).ok()?,
            minor: default_minor,
        }),
        Value::Array(a) if !a.is_empty() && a.len() <= 256 => {
            let major = u32::try_from(a[0].as_u64()?).ok()?;
            let minor = match a.get(1) {
                Some(m) => u32::try_from(m.as_u64()?).ok()?,
                None => default_minor,
            };
            Some(PackFormat { major, minor })
        }
        _ => None,
    }
}

/// `supported_formats`: число, `[a,b]` или `{"min_inclusive":a,"max_inclusive":b}`.
fn parse_supported(v: &Value) -> Option<(u32, u32)> {
    match v {
        Value::Number(n) => {
            let x = u32::try_from(n.as_u64()?).ok()?;
            Some((x, x))
        }
        Value::Array(a) if a.len() == 2 => Some((
            u32::try_from(a[0].as_u64()?).ok()?,
            u32::try_from(a[1].as_u64()?).ok()?,
        )),
        Value::Object(o) => Some((
            u32::try_from(o.get("min_inclusive")?.as_u64()?).ok()?,
            u32::try_from(o.get("max_inclusive")?.as_u64()?).ok()?,
        )),
        _ => None,
    }
}

/// Формат из секции: новый (`min_format`/`max_format`) в приоритете, затем `supported_formats`,
/// затем одиночный `pack_format` (только для пака), как в `IntermediaryFormat#validate`.
fn parse_range(o: &Value, allow_single: bool, supported_key: &str) -> Option<FormatRange> {
    let min = o.get("min_format").and_then(|v| parse_format(v, 0));
    let max = o.get("max_format").and_then(|v| parse_format(v, u32::MAX));
    if let (Some(min), Some(max)) = (min, max) {
        return (min <= max).then_some(FormatRange { min, max });
    }
    if let Some((a, b)) = o.get(supported_key).and_then(parse_supported) {
        return Some(FormatRange {
            min: PackFormat::of(a),
            max: PackFormat {
                major: b,
                minor: u32::MAX,
            },
        });
    }
    if allow_single {
        let f = u32::try_from(o.get("pack_format")?.as_u64()?).ok()?;
        return Some(FormatRange {
            min: PackFormat::of(f),
            max: PackFormat {
                major: f,
                minor: u32::MAX,
            },
        });
    }
    None
}

#[derive(Debug, Clone)]
pub struct Overlay {
    pub range: FormatRange,
    pub directory: String,
}

#[derive(Debug, Clone, Default)]
pub struct PackMeta {
    /// Описание как есть (строка/JSON-компонент), без разбора.
    pub description: Option<Value>,
    pub range: Option<FormatRange>,
    pub overlays: Vec<Overlay>,
    pub filter_namespaces: Vec<Regex>,
    pub filter_paths: Vec<Regex>,
}

fn valid_overlay_dir(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

impl PackMeta {
    /// Разбор `pack.mcmeta`. Ошибки отдельных секций → секция пропускается (как в ванили
    /// пак без валидной `pack` секции считается битым — решает вызывающий по `range`).
    // VANILLA: PackMetadataSection / OverlayMetadataSection / ResourceFilterSection (1.21.11)
    pub fn parse(data: &[u8]) -> Option<PackMeta> {
        let root: Value = serde_json::from_slice(data).ok()?;
        let mut m = PackMeta::default();
        if let Some(p) = root.get("pack") {
            m.description = p.get("description").cloned();
            m.range = parse_range(p, true, "supported_formats");
        }
        if let Some(entries) = root
            .get("overlays")
            .and_then(|o| o.get("entries"))
            .and_then(Value::as_array)
        {
            for e in entries {
                let dir = e.get("directory").and_then(Value::as_str);
                let range = parse_range(e, false, "formats");
                if let (Some(d), Some(range)) = (dir, range) {
                    if valid_overlay_dir(d) {
                        m.overlays.push(Overlay {
                            range,
                            directory: d.to_owned(),
                        });
                    }
                }
            }
        }
        if let Some(block) = root
            .get("filter")
            .and_then(|f| f.get("block"))
            .and_then(Value::as_array)
        {
            for b in block {
                // Отсутствующее поле = любое значение (IdentifierPattern: пустой предикат → true).
                let ns = b.get("namespace").and_then(Value::as_str);
                let pa = b.get("path").and_then(Value::as_str);
                #[allow(clippy::regex_creation_in_loops)] // паттерны уникальны для каждой записи
                let mk = |s: Option<&str>| s.map_or_else(|| Regex::new(""), Regex::new).ok();
                if let (Some(n), Some(p)) = (mk(ns), mk(pa)) {
                    m.filter_namespaces.push(n);
                    m.filter_paths.push(p);
                }
            }
        }
        Some(m)
    }

    /// Какие overlays действуют на формат `f` (в порядке объявления; позднее перекрывает).
    pub fn overlays_for(&self, f: PackFormat) -> Vec<&str> {
        self.overlays
            .iter()
            .filter(|o| o.range.contains(f))
            .map(|o| o.directory.as_str())
            .collect()
    }

    /// Блокирует ли фильтр ресурс `namespace`/`path` в паках ниже.
    // VANILLA: FallbackResourceManager.PackEntry#isFiltered
    pub fn filters(&self, namespace: &str, path: &str) -> bool {
        self.filter_namespaces
            .iter()
            .zip(&self.filter_paths)
            .any(|(n, p)| n.is_match(namespace) && p.is_match(path))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compat {
    Compatible,
    TooOld,
    TooNew,
}

pub struct Pack {
    pub id: String,
    pub meta: PackMeta,
    source: Box<dyn ResourceSource>,
    /// Активные overlays (каталоги), применены к `CURRENT_FORMAT`.
    overlays: Vec<String>,
}

impl Pack {
    /// Пак из источника; `pack.mcmeta` читается из корня, если есть.
    pub fn new(id: &str, source: Box<dyn ResourceSource>) -> Pack {
        let meta = source
            .read("pack.mcmeta")
            .and_then(|d| PackMeta::parse(&d))
            .unwrap_or_default();
        let overlays = meta
            .overlays_for(CURRENT_FORMAT)
            .into_iter()
            .map(str::to_owned)
            .collect();
        Pack {
            id: id.to_owned(),
            meta,
            source,
            overlays,
        }
    }

    pub fn compat(&self) -> Option<Compat> {
        let r = self.meta.range?;
        Some(if CURRENT_FORMAT < r.min {
            Compat::TooNew
        } else if CURRENT_FORMAT > r.max {
            Compat::TooOld
        } else {
            Compat::Compatible
        })
    }

    fn read(&self, path: &str) -> Option<Vec<u8>> {
        // Overlays — поверх основного содержимого, последний выигрывает.
        for o in self.overlays.iter().rev() {
            if let Some(d) = self.source.read(&format!("{o}/{path}")) {
                return Some(d);
            }
        }
        self.source.read(path)
    }

    fn exists(&self, path: &str) -> bool {
        self.overlays
            .iter()
            .any(|o| self.source.exists(&format!("{o}/{path}")))
            || self.source.exists(path)
    }

    fn list(&self, prefix: &str) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = self.source.list(prefix).into_iter().collect();
        for o in &self.overlays {
            let p = format!("{o}/");
            for k in self.source.list(&format!("{p}{prefix}")) {
                if let Some(rel) = k.strip_prefix(&p) {
                    out.insert(rel.to_owned());
                }
            }
        }
        out
    }
}

/// Менеджер ресурсов: паки снизу вверх (последний — сверху).
// VANILLA: MultiPackResourceManager / FallbackResourceManager (1.21.11)
#[derive(Default)]
pub struct ResourceManager {
    packs: Vec<Pack>,
}

/// `assets/<ns>/<path>` → `(ns, path)`.
fn split_asset(path: &str) -> Option<(&str, &str)> {
    path.strip_prefix("assets/")?.split_once('/')
}

impl ResourceManager {
    pub fn new(packs: Vec<Pack>) -> Self {
        ResourceManager { packs }
    }

    pub fn push(&mut self, p: Pack) {
        self.packs.push(p);
    }

    pub fn packs(&self) -> &[Pack] {
        &self.packs
    }

    /// Фильтр пака `i` скрывает ресурс в паках ниже него.
    fn filtered_by(&self, i: usize, path: &str) -> bool {
        split_asset(path).is_some_and(|(ns, p)| self.packs[i].meta.filters(ns, p))
    }

    /// Файл по пути `assets/<ns>/<path>`: сверху вниз, фильтр обрывает поиск.
    pub fn read(&self, path: &str) -> Option<Vec<u8>> {
        for i in (0..self.packs.len()).rev() {
            if let Some(d) = self.packs[i].read(path) {
                return Some(d);
            }
            if self.filtered_by(i, path) {
                return None;
            }
        }
        None
    }

    pub fn exists(&self, path: &str) -> bool {
        for i in (0..self.packs.len()).rev() {
            if self.packs[i].exists(path) {
                return true;
            }
            if self.filtered_by(i, path) {
                return false;
            }
        }
        false
    }

    /// Все версии файла, снизу вверх (для атласов/lang, как `getResourceStack`).
    pub fn read_stack(&self, path: &str) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for i in (0..self.packs.len()).rev() {
            if let Some(d) = self.packs[i].read(path) {
                out.push(d);
            }
            if self.filtered_by(i, path) {
                break;
            }
        }
        out.reverse();
        out
    }

    /// Уникальные пути с префиксом, не скрытые фильтрами вышестоящих паков.
    pub fn list(&self, prefix: &str) -> Vec<String> {
        let mut out = BTreeSet::new();
        for (i, p) in self.packs.iter().enumerate() {
            for k in p.list(prefix) {
                let hidden = (i + 1..self.packs.len()).any(|j| self.filtered_by(j, &k));
                if !hidden {
                    out.insert(k);
                }
            }
        }
        out.into_iter().collect()
    }

    pub fn namespaces(&self) -> BTreeSet<String> {
        self.list("assets/")
            .iter()
            .filter_map(|k| split_asset(k).map(|(n, _)| n.to_owned()))
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::source::DirSource;

    fn write(root: &std::path::Path, rel: &str, data: &str) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, data).unwrap();
    }

    #[test]
    fn parse_formats() {
        let m = PackMeta::parse(
            br#"{"pack":{"description":"x","min_format":[65,2],"max_format":97},
            "overlays":{"entries":[{"min_format":90,"max_format":100,"directory":"o1"},
            {"formats":[1,20],"directory":"old"},{"formats":[1,2],"directory":"bad/dir"}]},
            "filter":{"block":[{"namespace":"minecraft","path":"textures/.*"}]}}"#,
        )
        .unwrap();
        let r = m.range.unwrap();
        assert_eq!(
            r.min,
            PackFormat {
                major: 65,
                minor: 2
            }
        );
        assert_eq!(
            r.max,
            PackFormat {
                major: 97,
                minor: u32::MAX
            }
        );
        assert!(r.contains(CURRENT_FORMAT));
        assert_eq!(m.overlays_for(CURRENT_FORMAT), vec!["o1"]);
        assert!(m.filters("minecraft", "textures/block/a.png"));
        assert!(!m.filters("other", "textures/a.png"));
        let old = PackMeta::parse(br#"{"pack":{"description":"x","pack_format":34}}"#).unwrap();
        assert_eq!(old.range.unwrap().max.major, 34);
        let sup = PackMeta::parse(
            br#"{"pack":{"pack_format":15,"supported_formats":{"min_inclusive":15,"max_inclusive":18}}}"#,
        )
        .unwrap();
        assert_eq!(sup.range.unwrap().max.major, 18);
        assert!(PackMeta::parse(b"{broken").is_none());
    }

    #[test]
    fn compat_and_layering_and_filter() {
        let d = tempfile::tempdir().unwrap();
        let (a, b, c) = (d.path().join("a"), d.path().join("b"), d.path().join("c"));
        write(&a, "assets/minecraft/x.txt", "A");
        write(&a, "assets/minecraft/only_a.txt", "A");
        write(&a, "assets/other/y.txt", "A");
        write(&b, "assets/minecraft/x.txt", "B");
        write(
            &b,
            "pack.mcmeta",
            r#"{"pack":{"description":"b","pack_format":1}}"#,
        );
        write(
            &c,
            "pack.mcmeta",
            r#"{"pack":{"description":"c","min_format":97,"max_format":97},
               "overlays":{"entries":[{"min_format":90,"max_format":100,"directory":"ov"}]},
               "filter":{"block":[{"namespace":"minecraft","path":"only_a.txt"}]}}"#,
        );
        write(&c, "assets/minecraft/x.txt", "C");
        write(&c, "ov/assets/minecraft/x.txt", "OV");
        write(&c, "ov/assets/minecraft/z.txt", "Z");
        let pb = Pack::new("b", Box::new(DirSource::new(&b)));
        assert_eq!(pb.compat(), Some(Compat::TooOld));
        let pc = Pack::new("c", Box::new(DirSource::new(&c)));
        assert_eq!(pc.compat(), Some(Compat::Compatible));
        let rm = ResourceManager::new(vec![Pack::new("a", Box::new(DirSource::new(&a))), pb, pc]);
        // overlay сверху основного содержимого пака и выше нижних паков
        assert_eq!(rm.read("assets/minecraft/x.txt").unwrap(), b"OV");
        assert_eq!(rm.read("assets/minecraft/z.txt").unwrap(), b"Z");
        // фильтр верхнего пака скрывает ресурс нижнего
        assert!(rm.read("assets/minecraft/only_a.txt").is_none());
        assert!(!rm.exists("assets/minecraft/only_a.txt"));
        assert_eq!(rm.read("assets/other/y.txt").unwrap(), b"A");
        // стек: A, B, C (ov не отдельный слой стека пака — он подменяет чтение пака c)
        let st = rm.read_stack("assets/minecraft/x.txt");
        assert_eq!(st, vec![b"A".to_vec(), b"B".to_vec(), b"OV".to_vec()]);
        let l = rm.list("assets/minecraft/");
        assert!(l.contains(&"assets/minecraft/z.txt".to_string()));
        assert!(!l.contains(&"assets/minecraft/only_a.txt".to_string()));
        assert_eq!(
            rm.namespaces().into_iter().collect::<Vec<_>>(),
            vec!["minecraft", "other"]
        );
    }
}
