//! Сохранённые хотбары креатива: 9 наборов по 9 слотов в `hotbar.nbt`.
//! VANILLA: HotbarManager / Hotbar (26.3). Формат файла: корень без имени, `DataVersion`,
//! ключи "0".."8" — списки из 9 компаундов (`{}` — пустой слот, `{id, count}` — предмет).
//! VANILLA-UNSURE: компоненты предмета ванильный кодек пишет в `components` (NBT); у нас патч
//! хранится сетевыми байтами, поэтому пишем его в свой ключ `ezz_components` ("id:hex"), а
//! ванильный `components` не пишем. Чужой hotbar.nbt читаем (id/count), компоненты теряются.
use ezzmc_protocol::generated::r26_3::ITEM;
use ezzmc_protocol::item::{ComponentPatch, ItemStack};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::Reader;
use std::path::PathBuf;

/// VANILLA: HotbarManager.NUM_HOTBAR_GROUPS.
pub const GROUPS: usize = 9;
/// VANILLA: SharedConstants world_version 26.3 (version.json).
const DATA_VERSION: i32 = 5023;

pub struct Hotbars {
    path: PathBuf,
    sets: Vec<Vec<Option<ItemStack>>>,
    loaded: bool,
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(h: &str) -> Option<Vec<u8>> {
    if h.len() % 2 != 0 {
        return None;
    }
    (0..h.len() / 2)
        .map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).ok())
        .collect()
}

fn encode(s: &Option<ItemStack>) -> Tag {
    let Some(s) = s else {
        return Tag::Compound(Vec::new());
    };
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    let mut c = vec![
        ("id".to_owned(), Tag::String(format!("minecraft:{name}"))),
        ("count".to_owned(), Tag::Int(s.count)),
    ];
    if !s.patch.added.is_empty() {
        let l = s
            .patch
            .added
            .iter()
            .map(|(id, b)| Tag::String(format!("{id}:{}", hex(b))))
            .collect();
        c.push(("ezz_components".to_owned(), Tag::List(l)));
    }
    if !s.patch.removed.is_empty() {
        c.push((
            "ezz_removed".to_owned(),
            Tag::IntArray(s.patch.removed.iter().map(|x| *x as i32).collect()),
        ));
    }
    Tag::Compound(c)
}

fn decode(t: &Tag) -> Option<ItemStack> {
    let id = t.get("id")?.as_str()?;
    let name = id.strip_prefix("minecraft:").unwrap_or(id);
    let item = ITEM.iter().position(|n| *n == name)? as u32;
    if name == "air" {
        return None;
    }
    let count = t.get("count").and_then(Tag::as_i64).unwrap_or(1) as i32;
    if count <= 0 {
        return None;
    }
    let mut patch = ComponentPatch::default();
    if let Some(Tag::List(l)) = t.get("ezz_components") {
        for e in l {
            if let Some((i, h)) = e.as_str().and_then(|s| s.split_once(':')) {
                if let (Ok(i), Some(b)) = (i.parse::<u32>(), unhex(h)) {
                    patch.added.push((i, b.into_boxed_slice()));
                }
            }
        }
    }
    if let Some(Tag::IntArray(a)) = t.get("ezz_removed") {
        patch.removed = a.iter().map(|x| *x as u32).collect();
    }
    Some(ItemStack { item, count, patch })
}

impl Default for Hotbars {
    /// `hotbar.nbt` в рабочем каталоге (как options.txt).
    fn default() -> Self {
        Self::new(PathBuf::from("hotbar.nbt"))
    }
}

impl Hotbars {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            sets: (0..GROUPS).map(|_| vec![None; 9]).collect(),
            loaded: false,
        }
    }

    /// VANILLA: HotbarManager#load — лениво при первом обращении; ошибки только в лог.
    fn load(&mut self) {
        self.loaded = true;
        let Ok(bytes) = std::fs::read(&self.path) else {
            return;
        };
        let mut r = Reader::new(&bytes);
        let Ok(Some((_, root))) = nbt::read_named(&mut r) else {
            tracing::warn!("hotbar.nbt: не разобран");
            return;
        };
        for (i, set) in self.sets.iter_mut().enumerate() {
            let Some(Tag::List(l)) = root.get(&i.to_string()) else {
                continue;
            };
            if l.len() != 9 {
                continue;
            }
            *set = l.iter().map(decode).collect();
        }
    }

    /// VANILLA: HotbarManager#get.
    pub fn get(&mut self, i: usize) -> &[Option<ItemStack>] {
        if !self.loaded {
            self.load();
        }
        &self.sets[i]
    }

    /// VANILLA: Hotbar#isEmpty.
    pub fn is_empty(&mut self, i: usize) -> bool {
        self.get(i).iter().all(Option::is_none)
    }

    /// VANILLA: Hotbar#storeFrom + HotbarManager#save.
    pub fn store(&mut self, i: usize, items: &[Option<ItemStack>]) {
        if !self.loaded {
            self.load();
        }
        self.sets[i] = items.iter().take(9).cloned().collect();
        self.save();
    }

    fn save(&self) {
        let mut root = vec![("DataVersion".to_owned(), Tag::Int(DATA_VERSION))];
        for (i, set) in self.sets.iter().enumerate() {
            root.push((i.to_string(), Tag::List(set.iter().map(encode).collect())));
        }
        let mut out = Vec::new();
        if nbt::write_named(&mut out, "", &Tag::Compound(root)).is_err() {
            return;
        }
        if let Err(e) = std::fs::write(&self.path, out) {
            tracing::error!("hotbar.nbt: не записан: {e}");
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = std::env::temp_dir().join(format!("ezzmc_hb_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("hotbar.nbt");
        let mut h = Hotbars::new(p.clone());
        assert!(h.is_empty(3));
        let mut items = vec![None; 9];
        items[2] = Some(ItemStack {
            item: 1,
            count: 7,
            patch: ComponentPatch {
                added: vec![(5, vec![1, 2, 255].into_boxed_slice())],
                removed: vec![9],
            },
        });
        h.store(3, &items);
        let mut h2 = Hotbars::new(p);
        assert!(!h2.is_empty(3));
        assert!(h2.is_empty(0));
        assert_eq!(h2.get(3)[2], items[2]);
        assert_eq!(h2.get(3)[0], None);
    }
}
