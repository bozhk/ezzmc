//! Вкладки креатива 26.3: состав и порядок выгружены из ванильных `CreativeModeTabs`
//! (`tools/DumpCreativeTabs.java` запускает классы server.jar → `data/creative_tabs.txt`;
//! строки: `#TAB имя` или `предмет [hex патча компонентов в виде OPTIONAL_UNTRUSTED]`).
use ezzmc_protocol::item::ComponentPatch;
use ezzmc_protocol::wire::Reader;
use std::sync::OnceLock;

/// Предмет вкладки: имя + патч компонентов (зелья, книги, стрелы…).
pub struct Entry {
    pub name: &'static str,
    pub patch: ComponentPatch,
}

static RAW: &str = include_str!("../data/creative_tabs.txt");

fn unhex(h: &str) -> Vec<u8> {
    (0..h.len() / 2)
        .filter_map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).ok())
        .collect()
}

/// Патч из байтов untrusted-формы: added, removed, (id, длина, байты)×added, id×removed.
fn parse_patch(b: &[u8]) -> Option<ComponentPatch> {
    let mut r = Reader::new(b);
    let added = r.var_int().ok()?;
    let removed = r.var_int().ok()?;
    let mut p = ComponentPatch::default();
    for _ in 0..added {
        let id = r.var_int().ok()? as u32;
        let n = r.var_int().ok()? as usize;
        p.added.push((id, r.bytes(n).ok()?.into()));
    }
    for _ in 0..removed {
        p.removed.push(r.var_int().ok()? as u32);
    }
    Some(p)
}

fn parse() -> Vec<(&'static str, Vec<Entry>)> {
    let mut tabs: Vec<(&'static str, Vec<Entry>)> = Vec::new();
    for line in RAW.lines() {
        if let Some(n) = line.strip_prefix("#TAB ") {
            let name = n.split(' ').next().unwrap_or("");
            tabs.push((name, Vec::new()));
        } else if let Some((_, items)) = tabs.last_mut() {
            let (name, hex) = line.split_once(' ').unwrap_or((line, ""));
            let patch = parse_patch(&unhex(hex)).unwrap_or_default();
            items.push(Entry { name, patch });
        }
    }
    tabs
}

/// Содержимое вкладки по ключу ванили (`building_blocks`, `combat`, `search`…).
pub fn tab(key: &str) -> &'static [Entry] {
    static T: OnceLock<Vec<(&'static str, Vec<Entry>)>> = OnceLock::new();
    T.get_or_init(parse)
        .iter()
        .find(|(k, _)| *k == key)
        .map_or(&[], |(_, v)| v.as_slice())
}
