//! Определения атласов `atlases/*.json` (TODO-093): источники спрайтов и их разрешение.
use crate::pack::ResourceManager;
use crate::texture::Image;
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;

/// `namespace:path`; без пространства — `minecraft`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

impl Identifier {
    pub fn parse(s: &str) -> Identifier {
        match s.split_once(':') {
            Some((n, p)) => Identifier {
                namespace: n.to_owned(),
                path: p.to_owned(),
            },
            None => Identifier {
                namespace: "minecraft".into(),
                path: s.to_owned(),
            },
        }
    }

    pub fn new(ns: &str, path: &str) -> Identifier {
        Identifier {
            namespace: ns.to_owned(),
            path: path.to_owned(),
        }
    }

    /// `assets/<ns>/textures/<path>.png`
    pub fn texture_file(&self) -> String {
        format!("assets/{}/textures/{}.png", self.namespace, self.path)
    }

    pub fn with_prefix(&self, p: &str) -> Identifier {
        Identifier {
            namespace: self.namespace.clone(),
            path: format!("{p}{}", self.path),
        }
    }

    pub fn with_suffix(&self, s: &str) -> Identifier {
        Identifier {
            namespace: self.namespace.clone(),
            path: format!("{}{s}", self.path),
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    namespace: Regex,
    path: Regex,
}

impl Pattern {
    // VANILLA: IdentifierPattern — отсутствующее поле = совпадает с любым; поиск `find`.
    fn from_json(v: &Value) -> Option<Pattern> {
        let mk = |k: &str| match v.get(k).and_then(Value::as_str) {
            Some(s) => Regex::new(s).ok(),
            None => Regex::new("").ok(),
        };
        Some(Pattern {
            namespace: mk("namespace")?,
            path: mk("path")?,
        })
    }

    fn matches(&self, id: &Identifier) -> bool {
        self.namespace.is_match(&id.namespace) && self.path.is_match(&id.path)
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub sprite: Identifier,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub enum SpriteSource {
    Directory {
        source: String,
        prefix: String,
    },
    Single {
        resource: Identifier,
        sprite: Option<Identifier>,
    },
    Filter(Pattern),
    Unstitch {
        resource: Identifier,
        regions: Vec<Region>,
        div_x: f64,
        div_y: f64,
    },
    Paletted {
        textures: Vec<Identifier>,
        palette_key: Identifier,
        permutations: Vec<(String, Identifier)>,
        separator: String,
    },
}

/// Как получить пиксели спрайта.
#[derive(Debug, Clone)]
pub enum SpriteLoad {
    Texture(Identifier),
    Region {
        resource: Identifier,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        from: (f64, f64, f64, f64),
        div: (f64, f64),
    },
    Paletted {
        texture: Identifier,
        palette_key: Identifier,
        palette: Identifier,
    },
    Missing,
}

fn parse_source(v: &Value) -> Option<SpriteSource> {
    let ty = Identifier::parse(v.get("type")?.as_str()?);
    if ty.namespace != "minecraft" {
        return None;
    }
    let id = |k: &str| v.get(k).and_then(Value::as_str).map(Identifier::parse);
    match ty.path.as_str() {
        "directory" => Some(SpriteSource::Directory {
            source: v.get("source")?.as_str()?.to_owned(),
            prefix: v.get("prefix")?.as_str()?.to_owned(),
        }),
        "single" => Some(SpriteSource::Single {
            resource: id("resource")?,
            sprite: id("sprite"),
        }),
        "filter" => Some(SpriteSource::Filter(Pattern::from_json(v.get("pattern")?)?)),
        "unstitch" => {
            let regions: Vec<Region> = v
                .get("regions")?
                .as_array()?
                .iter()
                .map(|r| {
                    Some(Region {
                        sprite: Identifier::parse(r.get("sprite")?.as_str()?),
                        x: r.get("x")?.as_f64()?,
                        y: r.get("y")?.as_f64()?,
                        width: r.get("width")?.as_f64()?,
                        height: r.get("height")?.as_f64()?,
                    })
                })
                .collect::<Option<_>>()?;
            if regions.is_empty() {
                return None;
            }
            Some(SpriteSource::Unstitch {
                resource: id("resource")?,
                regions,
                div_x: v.get("divisor_x").and_then(Value::as_f64).unwrap_or(1.0),
                div_y: v.get("divisor_y").and_then(Value::as_f64).unwrap_or(1.0),
            })
        }
        "paletted_permutations" => Some(SpriteSource::Paletted {
            textures: v
                .get("textures")?
                .as_array()?
                .iter()
                .map(|t| t.as_str().map(Identifier::parse))
                .collect::<Option<_>>()?,
            palette_key: id("palette_key")?,
            permutations: v
                .get("permutations")?
                .as_object()?
                .iter()
                .map(|(k, t)| Some((k.clone(), Identifier::parse(t.as_str()?))))
                .collect::<Option<_>>()?,
            separator: v
                .get("separator")
                .and_then(Value::as_str)
                .unwrap_or("_")
                .to_owned(),
        }),
        _ => None,
    }
}

/// `atlases/<name>.json` из всех паков (стек снизу вверх); битые файлы пропускаются.
// VANILLA: SpriteSourceList#load
pub fn load_sources(rm: &ResourceManager, atlas: &str) -> Vec<SpriteSource> {
    let path = format!("assets/minecraft/atlases/{atlas}.json");
    let mut out = Vec::new();
    for data in rm.read_stack(&path) {
        let Ok(v) = serde_json::from_slice::<Value>(&data) else {
            tracing::error!("битый атлас {path}");
            continue;
        };
        let Some(list) = v.get("sources").and_then(Value::as_array) else {
            tracing::error!("атлас {path}: нет sources");
            continue;
        };
        // как FILE_CODEC: любая ошибка источника отбрасывает весь файл
        let parsed: Option<Vec<SpriteSource>> = list.iter().map(parse_source).collect();
        match parsed {
            Some(p) => out.extend(p),
            None => tracing::error!("атлас {path}: неверный источник"),
        }
    }
    out
}

/// Применить источники по порядку → карта спрайтов (позднее перекрывает, filter удаляет).
// VANILLA: SpriteSourceList#list; `missingno` добавляется отдельно (MissingTextureAtlasSprite)
pub fn resolve(rm: &ResourceManager, sources: &[SpriteSource]) -> BTreeMap<Identifier, SpriteLoad> {
    let mut map: BTreeMap<Identifier, SpriteLoad> = BTreeMap::new();
    for s in sources {
        match s {
            SpriteSource::Directory { source, prefix } => {
                for ns in rm.namespaces() {
                    let base = format!("assets/{ns}/textures/{source}/");
                    for f in rm.list(&base) {
                        // VANILLA: FileToIdConverter("textures/<source>", ".png")#fileToId + prefix
                        let Some(rel) = f.strip_prefix(&base).and_then(|r| r.strip_suffix(".png"))
                        else {
                            continue;
                        };
                        let id = Identifier::new(&ns, rel).with_prefix(prefix);
                        let tex = Identifier::new(&ns, &format!("{source}/{rel}"));
                        map.insert(id, SpriteLoad::Texture(tex));
                    }
                }
            }
            SpriteSource::Single { resource, sprite } => {
                if rm.exists(&resource.texture_file()) {
                    map.insert(
                        sprite.clone().unwrap_or_else(|| resource.clone()),
                        SpriteLoad::Texture(resource.clone()),
                    );
                } else {
                    tracing::warn!("нет спрайта {}", resource.texture_file());
                }
            }
            SpriteSource::Filter(p) => map.retain(|k, _| !p.matches(k)),
            SpriteSource::Unstitch {
                resource,
                regions,
                div_x,
                div_y,
            } => {
                if !rm.exists(&resource.texture_file()) {
                    tracing::warn!("нет спрайта {}", resource.texture_file());
                    continue;
                }
                for r in regions {
                    map.insert(
                        r.sprite.clone(),
                        SpriteLoad::Region {
                            resource: resource.clone(),
                            x: 0,
                            y: 0,
                            w: 0,
                            h: 0,
                            from: (r.x, r.y, r.width, r.height),
                            div: (*div_x, *div_y),
                        },
                    );
                }
            }
            SpriteSource::Paletted {
                textures,
                palette_key,
                permutations,
                separator,
            } => {
                for t in textures {
                    if !rm.exists(&t.texture_file()) {
                        tracing::warn!("нет текстуры {}", t.texture_file());
                        continue;
                    }
                    for (name, pal) in permutations {
                        map.insert(
                            t.with_suffix(&format!("{separator}{name}")),
                            SpriteLoad::Paletted {
                                texture: t.clone(),
                                palette_key: palette_key.clone(),
                                palette: pal.clone(),
                            },
                        );
                    }
                }
            }
        }
    }
    map
}

/// Заглушка отсутствующей текстуры 16×16 (розовый/чёрный).
// VANILLA: MissingTextureAtlasSprite#generateMissingImage
pub fn missing_image() -> Image {
    let mut im = Image::new(16, 16);
    for y in 0..16 {
        for x in 0..16 {
            let pink = (x < 8) == (y < 8);
            im.set(x, y, if pink { 0xFFF8_00F8 } else { 0xFF00_0000 });
        }
    }
    im
}

fn load_png(rm: &ResourceManager, id: &Identifier) -> Option<Image> {
    let data = rm.read(&id.texture_file())?;
    match Image::decode_png(&data) {
        Ok(i) => Some(i),
        Err(e) => {
            tracing::error!("текстура {id}: {e}");
            None
        }
    }
}

impl SpriteLoad {
    /// Загрузить пиксели. `None` — спрайт пропускается (ошибка уже залогирована).
    pub fn load(&self, rm: &ResourceManager) -> Option<Image> {
        match self {
            SpriteLoad::Missing => Some(missing_image()),
            SpriteLoad::Texture(id) => load_png(rm, id),
            SpriteLoad::Region {
                resource,
                from,
                div,
                ..
            } => {
                let img = load_png(rm, resource).or_else(|| Some(missing_image()))?;
                // VANILLA: Unstitcher.RegionInstance#get
                let (fx, fy) = (f64::from(img.width) / div.0, f64::from(img.height) / div.1);
                let x = (from.0 * fx).floor() as i64;
                let y = (from.1 * fy).floor() as i64;
                let w = (from.2 * fx).floor() as i64;
                let h = (from.3 * fy).floor() as i64;
                if x < 0
                    || y < 0
                    || w <= 0
                    || h <= 0
                    || x + w > i64::from(img.width)
                    || y + h > i64::from(img.height)
                {
                    tracing::error!("unstitch {resource}: область вне изображения");
                    return Some(missing_image());
                }
                Some(img.crop(x as u32, y as u32, w as u32, h as u32))
            }
            SpriteLoad::Paletted {
                texture,
                palette_key,
                palette,
            } => {
                let base = load_png(rm, texture)?;
                let key = load_png(rm, palette_key)?;
                let pal = load_png(rm, palette)?;
                if key.px.len() != pal.px.len() {
                    tracing::warn!(
                        "палитры разного размера: {} и {}",
                        key.px.len(),
                        pal.px.len()
                    );
                    return None;
                }
                // VANILLA: PalettedPermutations#createPaletteMapping
                let mut map = std::collections::HashMap::new();
                for (k, p) in key.px.iter().zip(&pal.px) {
                    if k >> 24 != 0 {
                        map.insert(k & 0x00FF_FFFF, *p);
                    }
                }
                let mut out = base;
                for px in &mut out.px {
                    let a = *px >> 24;
                    if a == 0 {
                        continue;
                    }
                    let rgb = *px & 0x00FF_FFFF;
                    let m = map.get(&rgb).copied().unwrap_or(rgb | 0xFF00_0000);
                    *px = (a * (m >> 24) / 255) << 24 | m & 0x00FF_FFFF;
                }
                Some(out)
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::pack::Pack;
    use crate::source::DirSource;

    fn png(px: &[(u32, u32, u32, u32)], w: u32, h: u32) -> Vec<u8> {
        let mut im = image::RgbaImage::new(w, h);
        for (i, &(r, g, b, a)) in px.iter().enumerate() {
            im.put_pixel(
                i as u32 % w,
                i as u32 / w,
                image::Rgba([r as u8, g as u8, b as u8, a as u8]),
            );
        }
        let mut buf = Vec::new();
        image::DynamicImage::ImageRgba8(im)
            .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();
        buf
    }

    fn put(root: &std::path::Path, rel: &str, data: &[u8]) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, data).unwrap();
    }

    fn rm(root: &std::path::Path) -> ResourceManager {
        ResourceManager::new(vec![Pack::new("t", Box::new(DirSource::new(root)))])
    }

    #[test]
    fn identifier_and_sources() {
        let d = tempfile::tempdir().unwrap();
        let r = d.path();
        let one = png(&[(1, 2, 3, 255)], 1, 1);
        put(r, "assets/minecraft/textures/block/stone.png", &one);
        put(r, "assets/minecraft/textures/block/sub/dirt.png", &one);
        put(r, "assets/mod/textures/block/ore.png", &one);
        put(r, "assets/minecraft/textures/entity/bell/body.png", &one);
        put(
            r,
            "assets/minecraft/atlases/blocks.json",
            br#"{"sources":[
              {"type":"minecraft:directory","prefix":"block/","source":"block"},
              {"type":"minecraft:single","resource":"minecraft:entity/bell/body","sprite":"minecraft:bell"},
              {"type":"minecraft:filter","pattern":{"path":"sub/"}},
              {"type":"minecraft:single","resource":"minecraft:nope"}]}"#,
        );
        let rm = rm(r);
        let src = load_sources(&rm, "blocks");
        assert_eq!(src.len(), 4);
        let m = resolve(&rm, &src);
        let keys: Vec<String> = m.keys().map(ToString::to_string).collect();
        assert_eq!(
            keys,
            vec!["minecraft:bell", "minecraft:block/stone", "mod:block/ore"]
        );
        assert_eq!(
            m[&Identifier::parse("block/stone")]
                .load(&rm)
                .unwrap()
                .get(0, 0),
            0xFF01_0203
        );
        assert_eq!(
            Identifier::parse("a:b/c").texture_file(),
            "assets/a/textures/b/c.png"
        );
    }

    #[test]
    fn broken_file_is_dropped_and_unknown_type_rejects_file() {
        let d = tempfile::tempdir().unwrap();
        put(
            d.path(),
            "assets/minecraft/atlases/x.json",
            br#"{"sources":[{"type":"minecraft:bogus"}]}"#,
        );
        put(d.path(), "assets/minecraft/atlases/y.json", b"{oops");
        let rm = rm(d.path());
        assert!(load_sources(&rm, "x").is_empty());
        assert!(load_sources(&rm, "y").is_empty());
    }

    #[test]
    fn unstitch_and_paletted() {
        let d = tempfile::tempdir().unwrap();
        let r = d.path();
        // 4×2 изображение: левая половина красная, правая синяя
        let mut px = Vec::new();
        for _ in 0..2 {
            px.extend([
                (255, 0, 0, 255),
                (255, 0, 0, 255),
                (0, 0, 255, 255),
                (0, 0, 255, 255),
            ]);
        }
        put(r, "assets/minecraft/textures/sheet.png", &png(&px, 4, 2));
        // палитра: ключ — красный и зелёный; пермутация — белый и серый
        put(
            r,
            "assets/minecraft/textures/key.png",
            &png(&[(255, 0, 0, 255), (0, 255, 0, 255)], 2, 1),
        );
        put(
            r,
            "assets/minecraft/textures/pal.png",
            &png(&[(255, 255, 255, 255), (9, 9, 9, 255)], 2, 1),
        );
        put(
            r,
            "assets/minecraft/textures/t.png",
            &png(
                &[
                    (255, 0, 0, 255),
                    (0, 255, 0, 128),
                    (7, 7, 7, 255),
                    (0, 0, 0, 0),
                ],
                2,
                2,
            ),
        );
        put(
            r,
            "assets/minecraft/atlases/a.json",
            br#"{"sources":[
             {"type":"unstitch","resource":"sheet","divisor_x":2,"divisor_y":1,
              "regions":[{"sprite":"right","x":1,"y":0,"width":1,"height":1}]},
             {"type":"paletted_permutations","textures":["t"],"palette_key":"key",
              "permutations":{"w":"pal"}}]}"#,
        );
        let rm = rm(r);
        let m = resolve(&rm, &load_sources(&rm, "a"));
        let right = m[&Identifier::parse("right")].load(&rm).unwrap();
        assert_eq!(
            (right.width, right.height, right.get(0, 0)),
            (2, 2, 0xFF00_00FF)
        );
        let t = m[&Identifier::parse("t_w")].load(&rm).unwrap();
        assert_eq!(t.get(0, 0), 0xFFFF_FFFF);
        assert_eq!(t.get(1, 0), 0x8009_0909); // альфа 128*255/255
        assert_eq!(t.get(0, 1), 0xFF07_0707); // нет в палитре — оставить цвет
        assert_eq!(t.get(1, 1) >> 24, 0);
        assert_eq!(missing_image().get(0, 0), 0xFFF8_00F8);
    }

    #[test]
    fn real_jar_atlases_resolve() {
        let Ok(g) = crate::install::GameInstall::locate("26.3") else {
            return;
        };
        let z = crate::source::ZipSource::open(&g.jar).unwrap();
        let rm = ResourceManager::new(vec![Pack::new("vanilla", Box::new(z))]);
        let blocks = resolve(&rm, &load_sources(&rm, "blocks"));
        assert!(blocks.len() > 1000, "{}", blocks.len());
        assert!(blocks.contains_key(&Identifier::parse("block/stone")));
        assert!(blocks.contains_key(&Identifier::parse("entity/bell/bell_body")));
        // 26.3: атласы armor_trims/signs/beds убраны из ванили.
        let pats = resolve(&rm, &load_sources(&rm, "banner_patterns"));
        let t = pats.values().next().unwrap().load(&rm);
        assert!(t.is_some());
        for a in [
            "items",
            "gui",
            "particles",
            "paintings",
            "chests",
            "celestials",
            "shield_patterns",
            "decorated_pot",
            "shulker_boxes",
            "map_decorations",
        ] {
            assert!(!load_sources(&rm, a).is_empty(), "{a}");
        }
    }
}
