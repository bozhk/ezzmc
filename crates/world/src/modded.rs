//! Модовые состояния блоков Forge 1.20.1 (DEC-018, TODO-505).
//!
//! Канонические id состояний: `0..STATE_COUNT` — ваниль 26.3; `STATE_COUNT + i` — i-е состояние
//! дампа `.ezbake` (`ModTable`). Сервер нумерует свои состояния иначе: блоки в порядке его реестра
//! (снимок FML `S2CRegistry minecraft:block`), внутри блока — порядок `StateDefinition`
//! (VANILLA: Forge GameData$BlockCallbacks#onBake). Карта «id сервера → канон» строится при входе.
//!
//! Для игровой логики (физика, свет, отсечение) у каждого модового состояния есть канонический
//! «прокси» (`BlockState::canon`): пока грубый, по флагам дампа (TODO-507 — точные формы/свет).
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::generated::b26_3::{BLOCKS, STATE_COUNT};

/// Блок дампа (то, что нужно миру; квады/формы читает клиент из `ModBake`).
#[derive(Clone, Debug)]
pub struct ModBlock {
    pub name: String,
    /// Первое состояние блока в нумерации дампа.
    pub first: u32,
    /// Число состояний (произведение числа значений свойств).
    pub count: u32,
}

/// Грубая классификация состояния для прокси.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyKind {
    /// Без коллизии и без заслонения (трава-цветы, провода) → `structure_void`.
    Passable,
    /// Твёрдый непрозрачный куб → `black_concrete`.
    Solid,
    /// Коллизия есть, но свет проходит/форма неполная → `barrier` (не стекло: стекло
    /// прячет грани соседнего стекла, а у разных модовых блоков это дыры).
    SeeThrough,
    /// Воздух.
    Air,
}

/// Данные дампа, нужные миру. Ставит клиент один раз при загрузке `.ezbake`.
pub struct ModTable {
    pub blocks: Vec<ModBlock>,
    /// Индекс блока для каждого состояния дампа.
    pub state_block: Vec<u32>,
    /// Канонический прокси для каждого состояния дампа.
    pub proxy: Vec<u32>,
    by_name: HashMap<String, u32>,
}

impl ModTable {
    /// `blocks` — в порядке реестра дампа; `kinds` — по состояниям дампа.
    pub fn new(blocks: Vec<ModBlock>, kinds: &[ProxyKind]) -> Self {
        let mut state_block = vec![u32::MAX; kinds.len()];
        for (i, b) in blocks.iter().enumerate() {
            for s in b.first..(b.first + b.count).min(kinds.len() as u32) {
                state_block[s as usize] = i as u32;
            }
        }
        let find = |n: &str| {
            BLOCKS
                .iter()
                .find(|b| b.name == n)
                .map_or(0, |b| b.default_state)
        };
        let (passable, solid, see) = (
            find("structure_void"),
            find("black_concrete"),
            find("barrier"),
        );
        let proxy = kinds
            .iter()
            .map(|k| match k {
                ProxyKind::Passable => passable,
                ProxyKind::Solid => solid,
                ProxyKind::SeeThrough => see,
                ProxyKind::Air => 0,
            })
            .collect();
        let by_name = blocks
            .iter()
            .enumerate()
            .map(|(i, b)| (b.name.clone(), i as u32))
            .collect();
        ModTable {
            blocks,
            state_block,
            proxy,
            by_name,
        }
    }

    pub fn state_count(&self) -> u32 {
        self.proxy.len() as u32
    }

    pub fn block_by_name(&self, name: &str) -> Option<&ModBlock> {
        self.by_name.get(name).map(|&i| &self.blocks[i as usize])
    }
}

static TABLE: OnceLock<ModTable> = OnceLock::new();
/// Карта «модовое состояние сервера (id − первое модовое) → канон» текущего соединения.
static SERVER: RwLock<Vec<u32>> = RwLock::new(Vec::new());

/// Поставить таблицу дампа (один раз за процесс; повтор игнорируется).
pub fn install(t: ModTable) {
    let n = t.state_count();
    if TABLE.set(t).is_ok() {
        tracing::info!("модовые состояния: {n} (канон {STATE_COUNT}..)");
    }
}

pub fn table() -> Option<&'static ModTable> {
    TABLE.get()
}

/// Число модовых канонических состояний (0 без дампа).
pub fn count() -> u32 {
    TABLE.get().map_or(0, ModTable::state_count)
}

/// Индекс состояния дампа для канонического id, если оно модовое.
#[inline]
pub fn bake_index(canon: u32) -> Option<u32> {
    let i = canon.checked_sub(STATE_COUNT)?;
    (i < count()).then_some(i)
}

/// Канонический прокси модового состояния (ванильное состояние для логики).
#[inline]
pub fn proxy(canon: u32) -> Option<u32> {
    let t = TABLE.get()?;
    let i = canon.checked_sub(STATE_COUNT)?;
    t.proxy.get(i as usize).copied()
}

/// Бит на id состояния в хранилище секций клиента (ваниль + модовые).
pub fn block_global_bits() -> u32 {
    crate::palette::ceil_log2(STATE_COUNT + count())
}

/// Построить карту сервера: `server_blocks` — (id, имя) из снимка FML, `first_modded` — первый
/// id модового состояния сервера (= число ванильных состояний 1.20.1). Ванильные блоки (id
/// которых меньше) пропускаются по числу состояний из `vanilla_counts` (id блока → состояний).
/// Возвращает (сопоставлено состояний, блоков без дампа).
pub fn set_server_blocks(
    server_blocks: &mut [(u32, String)],
    vanilla_counts: &dyn Fn(&str) -> Option<u32>,
    first_modded: u32,
) -> (u32, u32) {
    server_blocks.sort_by_key(|b| b.0);
    let table = TABLE.get();
    let mut map = Vec::new();
    let mut next = 0u32; // id сервера следующего состояния
    let mut missing = 0;
    let mut mapped = 0;
    for (_, name) in server_blocks.iter() {
        let bare = name.strip_prefix("minecraft:").unwrap_or(name);
        if name.starts_with("minecraft:") {
            if let Some(n) = vanilla_counts(bare) {
                next += n;
                continue;
            }
        }
        let Some(mb) = table.and_then(|t| t.block_by_name(name)) else {
            // VANILLA-UNSURE: блока нет в дампе — число его состояний неизвестно, дальше id съедут.
            missing += 1;
            if missing <= 5 {
                tracing::warn!("модовый блок сервера не найден в дампе: {name}");
            }
            next += 1;
            continue;
        };
        for k in 0..mb.count {
            let sid = next + k;
            if sid >= first_modded {
                let i = (sid - first_modded) as usize;
                if map.len() <= i {
                    map.resize(i + 1, u32::MAX);
                }
                map[i] = STATE_COUNT + mb.first + k;
                mapped += 1;
            }
        }
        next += mb.count;
    }
    if let Ok(mut g) = SERVER.write() {
        *g = map;
    }
    (mapped, missing)
}

/// Модовое состояние сервера (`id − first_modded`) → канон, `None` — неизвестно.
#[inline]
pub fn server_to_canon(offset: u32) -> Option<u32> {
    let g = SERVER.read().ok()?;
    g.get(offset as usize).copied().filter(|&c| c != u32::MAX)
}

/// Сбросить карту сервера (новое соединение).
pub fn clear_server() {
    if let Ok(mut g) = SERVER.write() {
        g.clear();
    }
    if let Ok(mut g) = SERVER_REGS.write() {
        g.clear();
    }
}

/// Канонический id модового меню: `MOD_MENU_BASE + id в реестре minecraft:menu сервера`
/// (ванильные меню 26.3 — `0..MENU.len()`). Имя — `server_name("minecraft:menu", id)`.
pub const MOD_MENU_BASE: i32 = 1 << 20;

/// Канонический id модового предмета: `MOD_ITEM_BASE + id в реестре minecraft:item сервера`
/// (ванильные предметы 26.3 — `0..ITEM.len()`). Имя — `server_name("minecraft:item", id)`.
pub const MOD_ITEM_BASE: u32 = ezzmc_protocol::item::MOD_ITEM_BASE;

/// Модовый предмет сервера (по снимку FML) → канон. id; `None` — не модовый или снимка нет.
pub fn modded_item(server_id: u32) -> Option<u32> {
    let g = SERVER_REGS.read().ok()?;
    let (_, names) = g.iter().find(|(r, _)| r == "minecraft:item")?;
    let n = names.get(server_id as usize)?;
    (!n.is_empty() && !n.starts_with("minecraft:")).then_some(MOD_ITEM_BASE + server_id)
}

/// Снимки реестров сервера из FML (`minecraft:menu`, `minecraft:item`): реестр → (id → имя).
static SERVER_REGS: RwLock<Vec<(String, Vec<String>)>> = RwLock::new(Vec::new());

/// Запомнить снимок реестра сервера (`ids` — пары «id, полное имя»).
pub fn set_server_registry(registry: &str, ids: &[(u32, String)]) {
    let len = ids.iter().map(|(i, _)| *i as usize + 1).max().unwrap_or(0);
    let mut names = vec![String::new(); len];
    for (i, n) in ids {
        names[*i as usize] = n.clone();
    }
    if let Ok(mut g) = SERVER_REGS.write() {
        g.retain(|(r, _)| r != registry);
        g.push((registry.to_owned(), names));
    }
}

/// Полное имя записи реестра сервера по id; `None` — снимка нет или id вне снимка.
pub fn server_name(registry: &str, id: u32) -> Option<String> {
    let g = SERVER_REGS.read().ok()?;
    let (_, names) = g.iter().find(|(r, _)| r == registry)?;
    names.get(id as usize).filter(|n| !n.is_empty()).cloned()
}

/// Id записи реестра сервера по полному имени (обратный поиск к [`server_name`]).
pub fn server_id(registry: &str, name: &str) -> Option<u32> {
    let g = SERVER_REGS.read().ok()?;
    let (_, names) = g.iter().find(|(r, _)| r == registry)?;
    names.iter().position(|n| n == name).map(|i| i as u32)
}

/// Есть ли снимок реестра сервера.
pub fn has_server_registry(registry: &str) -> bool {
    SERVER_REGS
        .read()
        .is_ok_and(|g| g.iter().any(|(r, _)| r == registry))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn server_mapping_by_name() {
        // дамп: [mod:a (2 сост.), mod:b (3 сост.)]
        let blocks = vec![
            ModBlock {
                name: "mod:a".into(),
                first: 0,
                count: 2,
            },
            ModBlock {
                name: "mod:b".into(),
                first: 2,
                count: 3,
            },
        ];
        let kinds = [ProxyKind::Solid; 5];
        let t = ModTable::new(blocks, &kinds);
        assert_eq!(t.state_block, [0, 0, 1, 1, 1]);
        install(t);
        // сервер: ваниль stone (1 сост.) + air (1), затем mod:b раньше mod:a
        let mut sb = vec![
            (3, "mod:a".to_string()),
            (0, "minecraft:air".to_string()),
            (2, "mod:b".to_string()),
            (1, "minecraft:stone".to_string()),
        ];
        let (mapped, missing) = set_server_blocks(&mut sb, &|_| Some(1), 2);
        assert_eq!((mapped, missing), (5, 0));
        // сервер: 2,3,4 = mod:b; 5,6 = mod:a
        assert_eq!(server_to_canon(0), Some(STATE_COUNT + 2));
        assert_eq!(server_to_canon(2), Some(STATE_COUNT + 4));
        assert_eq!(server_to_canon(3), Some(STATE_COUNT));
        assert_eq!(server_to_canon(5), None);
        assert_eq!(bake_index(STATE_COUNT + 4), Some(4));
        assert_eq!(bake_index(STATE_COUNT + 5), None);
        assert!(proxy(STATE_COUNT).is_some());
        assert_eq!(
            block_global_bits(),
            crate::palette::ceil_log2(STATE_COUNT + 5)
        );
    }
}
