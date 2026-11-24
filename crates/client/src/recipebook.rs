//! Книга рецептов: разбор recipe_book_add/remove/settings и хранилище `RecipeBook`.
//! VANILLA: ClientboundRecipeBookAddPacket, RecipeDisplayEntry, RecipeDisplay, SlotDisplay (26.3).
//! Экран рецептов (RecipeBookComponent) — TODO-323c.
use ezzmc_protocol::item::{self, ItemStack};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::{ProtocolError, ProtocolResult};

fn bad(what: &'static str, value: i64) -> ProtocolError {
    ProtocolError::InvalidValue { what, value }
}

const MAX_DEPTH: u32 = 16;

fn one(item: u32) -> ItemStack {
    ItemStack {
        item,
        count: 1,
        patch: Default::default(),
    }
}

/// Раскрытый SlotDisplay: первый стек (для иконки) и все подходящие предметы (для canCraft/ghost).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sd {
    pub stack: Option<ItemStack>,
    /// Пусто — слот пуст или «любое топливо» (не раскрывается: VANILLA-UNSURE).
    pub alts: Vec<u32>,
}

fn sd_one(stack: ItemStack) -> Sd {
    Sd {
        alts: vec![stack.item],
        stack: Some(stack),
    }
}

/// Теги предметов 26.3: имя тега (без `minecraft:`) → id предметов. VANILLA: data/tags/item.
fn expand_tag(name: &str) -> Vec<u32> {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static T: OnceLock<HashMap<&'static str, Vec<&'static str>>> = OnceLock::new();
    static IDS: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();
    let tags = T.get_or_init(|| {
        include_str!("../data/item_tags.txt")
            .lines()
            .filter_map(|l| {
                let mut it = l.split(' ');
                Some((it.next()?, it.collect()))
            })
            .collect()
    });
    let ids = IDS.get_or_init(|| {
        ezzmc_protocol::generated::r26_3::ITEM
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i as u32))
            .collect()
    });
    let key = name.strip_prefix("minecraft:").unwrap_or(name);
    tags.get(key)
        .map(|v| v.iter().filter_map(|n| ids.get(n).copied()).collect())
        .unwrap_or_default()
}

/// VANILLA: SlotDisplay#resolveForStacks (приближённо: контекстные дисплеи без уровня дают первый
/// стек/пусто; VANILLA-UNSURE).
fn slot(r: &mut Reader, depth: u32) -> ProtocolResult<Sd> {
    if depth > MAX_DEPTH {
        return Err(bad("slot display depth", depth as i64));
    }
    let d = depth + 1;
    Ok(match r.var_int()? {
        // empty, any_fuel
        0 | 1 => Sd::default(),
        // with_any_potion(display)
        2 => slot(r, d)?,
        // only_with_component(source, component type)
        3 => {
            let s = slot(r, d)?;
            r.var_int()?;
            s
        }
        // item
        4 => sd_one(one(item::read_item(r)?)),
        // item_stack (ItemStackTemplate)
        5 => sd_one(item::read_template(r)?),
        // tag (HolderSet): имя тега или прямые id
        6 => match r.var_int()? {
            0 => {
                let name = r.string(32767)?.to_string();
                let alts = expand_tag(&name);
                Sd {
                    stack: alts.first().copied().map(one),
                    alts,
                }
            }
            n => {
                let n = (n - 1).max(0) as usize;
                if n > r.remaining() {
                    return Err(bad("holder set length", n as i64));
                }
                let mut alts = Vec::with_capacity(n);
                for _ in 0..n {
                    alts.push(r.var_int()? as u32);
                }
                Sd {
                    stack: alts.first().copied().map(one),
                    alts,
                }
            }
        },
        // dyed(dye, target)
        7 => {
            slot(r, d)?;
            slot(r, d)?
        }
        // smithing_trim(base, material, pattern holder)
        8 => {
            let base = slot(r, d)?;
            slot(r, d)?;
            // Holder<TrimPattern>: 0 — встроенный объект (не разбираем), иначе id+1.
            if r.var_int()? == 0 {
                return Err(bad("inline trim pattern", 0));
            }
            base
        }
        // with_remainder(input, remainder)
        9 => {
            let s = slot(r, d)?;
            slot(r, d)?;
            s
        }
        // composite(list)
        10 => {
            let n = r.length("composite slot", 4096)?;
            let mut out = Sd::default();
            for _ in 0..n {
                let s = slot(r, d)?;
                if out.stack.is_none() {
                    out.stack = s.stack;
                }
                out.alts.extend(s.alts);
            }
            out
        }
        t => return Err(bad("slot display type", t as i64)),
    })
}

/// Список слотов-дисплеев (ингредиенты).
fn slots(r: &mut Reader) -> ProtocolResult<Vec<Sd>> {
    let n = r.length("slot list", 4096)?;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(slot(r, 0)?);
    }
    Ok(v)
}

/// VANILLA: RecipeDisplay (crafting_shapeless/shaped, furnace, stonecutter, smithing).
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Shapeless,
    Shaped { w: u32, h: u32 },
    Furnace { duration: i32, experience: f32 },
    Stonecutter,
    Smithing,
}

/// Ингредиент `craftingRequirements`: прямые id предметов или тег (не раскрыт).
#[derive(Clone, Debug, PartialEq)]
pub enum Ingredient {
    Items(Vec<u32>),
    Tag(String),
}

/// VANILLA: RecipeDisplayEntry (+ флаги Entry из ClientboundRecipeBookAddPacket).
#[derive(Clone, Debug, PartialEq)]
pub struct RecipeEntry {
    pub id: i32,
    pub kind: Kind,
    /// Ингредиенты для сетки/ghost-рецепта (для печи — один вход, для резака — один).
    pub ingredients: Vec<Sd>,
    pub result: Sd,
    pub station: Sd,
    pub group: Option<i32>,
    /// Id реестра `recipe_book_category`.
    pub category: i32,
    /// `None` — сервер не прислал требований (canCraft всегда false).
    pub requirements: Option<Vec<Ingredient>>,
    pub notification: bool,
    pub highlight: bool,
}

fn holder_set(r: &mut Reader) -> ProtocolResult<Ingredient> {
    Ok(match r.var_int()? {
        0 => Ingredient::Tag(r.string(32767)?.to_string()),
        n => {
            let n = (n - 1).max(0) as usize;
            if n > r.remaining() {
                return Err(bad("holder set length", n as i64));
            }
            let mut ids = Vec::with_capacity(n);
            for _ in 0..n {
                ids.push(r.var_int()? as u32);
            }
            Ingredient::Items(ids)
        }
    })
}

/// VANILLA: RecipeDisplay (тип + поля, без id/группы/категории).
fn display(r: &mut Reader) -> ProtocolResult<(Kind, Vec<Sd>, Sd, Sd)> {
    Ok(match r.var_int()? {
        // crafting_shapeless: ingredients, result, station
        0 => {
            let ing = slots(r)?;
            let res = slot(r, 0)?;
            (Kind::Shapeless, ing, res, slot(r, 0)?)
        }
        // crafting_shaped: w, h, ingredients, result, station
        1 => {
            let w = r.var_int()?.max(0) as u32;
            let h = r.var_int()?.max(0) as u32;
            let ing = slots(r)?;
            let res = slot(r, 0)?;
            (Kind::Shaped { w, h }, ing, res, slot(r, 0)?)
        }
        // furnace: ingredient, fuel, result, station, duration, experience
        2 => {
            let ing = slot(r, 0)?;
            let fuel = slot(r, 0)?;
            let res = slot(r, 0)?;
            let st = slot(r, 0)?;
            let duration = r.var_int()?;
            let experience = r.f32()?;
            (
                Kind::Furnace {
                    duration,
                    experience,
                },
                // [0] — вход, [1] — топливо (для ghost-рецепта).
                vec![ing, fuel],
                res,
                st,
            )
        }
        // stonecutter: input, result, station
        3 => {
            let ing = slot(r, 0)?;
            let res = slot(r, 0)?;
            (Kind::Stonecutter, vec![ing], res, slot(r, 0)?)
        }
        // smithing: template, base, addition, result, station
        4 => {
            let ing = vec![slot(r, 0)?, slot(r, 0)?, slot(r, 0)?];
            let res = slot(r, 0)?;
            (Kind::Smithing, ing, res, slot(r, 0)?)
        }
        t => return Err(bad("recipe display type", t as i64)),
    })
}

fn entry(r: &mut Reader) -> ProtocolResult<RecipeEntry> {
    let id = r.var_int()?; // RecipeDisplayId
    let (kind, ingredients, result, station) = display(r)?;
    // OPTIONAL_VAR_INT group: 0 — нет, иначе значение-1.
    let group = match r.var_int()? {
        0 => None,
        g => Some(g - 1),
    };
    let category = r.var_int()?; // RecipeBookCategory
    let requirements = if r.bool()? {
        let n = r.length("ingredients", 4096)?;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(holder_set(r)?);
        }
        Some(v)
    } else {
        None
    };
    Ok(RecipeEntry {
        id,
        kind,
        ingredients,
        result,
        station,
        group,
        category,
        requirements,
        notification: false,
        highlight: false,
    })
}

/// (предмет станции, результат); `None` — пустой стек (рисуется как ничего).
pub type RecipeNote = (Option<ItemStack>, Option<ItemStack>);

/// VANILLA: RecipeBookSettings.TypeSettings (open, filtering).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TypeSettings {
    pub open: bool,
    pub filtering: bool,
}

/// Тип книги рецептов (RecipeBookType); индекс совпадает с пакетом настроек.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookKind {
    Crafting,
    Furnace,
    Blast,
    Smoker,
}

impl BookKind {
    pub fn idx(self) -> usize {
        self as usize
    }
}

/// VANILLA: RecipeBookSettings — crafting, furnace, blast furnace, smoker.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Settings {
    pub crafting: TypeSettings,
    pub furnace: TypeSettings,
    pub blast_furnace: TypeSettings,
    pub smoker: TypeSettings,
}

impl Settings {
    pub fn of(&self, k: BookKind) -> TypeSettings {
        match k {
            BookKind::Crafting => self.crafting,
            BookKind::Furnace => self.furnace,
            BookKind::Blast => self.blast_furnace,
            BookKind::Smoker => self.smoker,
        }
    }
    pub fn of_mut(&mut self, k: BookKind) -> &mut TypeSettings {
        match k {
            BookKind::Crafting => &mut self.crafting,
            BookKind::Furnace => &mut self.furnace,
            BookKind::Blast => &mut self.blast_furnace,
            BookKind::Smoker => &mut self.smoker,
        }
    }
    pub fn open_flags(&self) -> [bool; 4] {
        [
            self.crafting.open,
            self.furnace.open,
            self.blast_furnace.open,
            self.smoker.open,
        ]
    }
}

/// Одно изменение книги от сервера.
#[derive(Clone, Debug, PartialEq)]
pub enum Update {
    Add {
        entries: Vec<RecipeEntry>,
        replace: bool,
    },
    Remove(Vec<i32>),
    Settings(Settings),
}

/// VANILLA: ClientboundPlaceGhostRecipePacket — (containerId, RecipeDisplay).
pub fn parse_ghost(body: &[u8]) -> ProtocolResult<(i32, Ghost)> {
    let mut r = Reader::new(body);
    let container = r.var_int()?;
    let (kind, ingredients, result, _) = display(&mut r)?;
    Ok((
        container,
        Ghost {
            kind,
            ingredients,
            result,
        },
    ))
}

/// Ghost-рецепт, пришедший от сервера (RecipeDisplay без id).
#[derive(Clone, Debug, PartialEq)]
pub struct Ghost {
    pub kind: Kind,
    pub ingredients: Vec<Sd>,
    pub result: Sd,
}

/// Одна клетка призрака: слот меню, варианты предметов, это результат.
#[derive(Clone, Debug, PartialEq)]
pub struct GhostSlot {
    pub slot: usize,
    pub items: Vec<u32>,
    pub result: bool,
}

/// VANILLA: PlaceRecipeHelper#placeRecipe — (индекс ячейки сетки, номер ингредиента).
pub fn place_recipe(
    grid_w: usize,
    grid_h: usize,
    rw: usize,
    rh: usize,
    count: usize,
) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let (mut grid_index, mut next) = (0usize, 0usize);
    let mut y = 0usize;
    while y < grid_h {
        let center_y = (rh as f32) < grid_h as f32 / 2.0;
        let start_y = (grid_h as f32 / 2.0 - rh as f32 / 2.0).floor() as usize;
        if center_y && start_y > y {
            grid_index += grid_w;
            y += 1;
        }
        for x in 0..grid_w {
            if next >= count {
                return out;
            }
            let center_x = (rw as f32) < grid_w as f32 / 2.0;
            let start_x = (grid_w as f32 / 2.0 - rw as f32 / 2.0).floor() as usize;
            let (total, add) = if center_x {
                (start_x + rw, start_x <= x && x < start_x + rw)
            } else {
                (rw, x < rw)
            };
            if add {
                out.push((grid_index, next));
                next += 1;
            } else if total == x {
                grid_index += grid_w - x;
                break;
            }
            grid_index += 1;
        }
        y += 1;
    }
    out
}

impl Ghost {
    /// VANILLA: CraftingRecipeBookComponent#fillGhostRecipe. Слоты меню: 0 результат, 1.. сетка.
    pub fn slots(&self, grid: usize, fuel_filled: bool) -> Vec<GhostSlot> {
        let mut v = Vec::new();
        let mut push = |slot: usize, sd: &Sd, result: bool| {
            if !sd.alts.is_empty() {
                v.push(GhostSlot {
                    slot,
                    items: sd.alts.clone(),
                    result,
                });
            }
        };
        let result_slot = if matches!(self.kind, Kind::Furnace { .. }) {
            2
        } else {
            0
        };
        push(result_slot, &self.result, true);
        match self.kind {
            Kind::Furnace { .. } => {
                push(0, &self.ingredients[0], false);
                // VANILLA: FurnaceRecipeBookComponent — топливо только в пустой слот.
                if !fuel_filled {
                    push(1, &self.ingredients[1], false);
                }
            }
            Kind::Shaped { w, h } => {
                for (gi, ing) in
                    place_recipe(grid, grid, w as usize, h as usize, self.ingredients.len())
                {
                    push(1 + gi, &self.ingredients[ing], false);
                }
            }
            Kind::Shapeless => {
                for (i, sd) in self.ingredients.iter().take(grid * grid).enumerate() {
                    push(1 + i, sd, false);
                }
            }
            _ => {}
        }
        v
    }
}

/// VANILLA: ClientboundRecipeBookAddPacket.
pub fn parse_add(body: &[u8]) -> ProtocolResult<Update> {
    let mut r = Reader::new(body);
    let n = r.length("recipe entries", 65536)?;
    let mut entries = Vec::with_capacity(n);
    for _ in 0..n {
        let mut e = entry(&mut r)?;
        let flags = r.u8()?;
        e.notification = flags & 1 != 0;
        e.highlight = flags & 2 != 0;
        entries.push(e);
    }
    let replace = r.bool()?;
    Ok(Update::Add { entries, replace })
}

/// VANILLA: ClientboundRecipeBookRemovePacket.
pub fn parse_remove(body: &[u8]) -> ProtocolResult<Update> {
    let mut r = Reader::new(body);
    let n = r.length("recipe ids", 65536)?;
    let mut ids = Vec::with_capacity(n);
    for _ in 0..n {
        ids.push(r.var_int()?);
    }
    Ok(Update::Remove(ids))
}

/// VANILLA: ClientboundRecipeBookSettingsPacket (4 × (open, filtering)).
pub fn parse_settings(body: &[u8]) -> ProtocolResult<Update> {
    let mut r = Reader::new(body);
    let mut t = || -> ProtocolResult<TypeSettings> {
        Ok(TypeSettings {
            open: r.bool()?,
            filtering: r.bool()?,
        })
    };
    Ok(Update::Settings(Settings {
        crafting: t()?,
        furnace: t()?,
        blast_furnace: t()?,
        smoker: t()?,
    }))
}

impl Ingredient {
    /// Подходящие id предметов (теги раскрываются по `item_tags.txt`).
    pub fn items(&self) -> Vec<u32> {
        match self {
            Ingredient::Items(v) => v.clone(),
            Ingredient::Tag(t) => expand_tag(t),
        }
    }
}

impl RecipeEntry {
    /// VANILLA: RecipeDisplayEntry#canCraft → StackedContents#canCraft: каждому ингредиенту
    /// нужен свой предмет из набора; `have` — сколько каждого предмета у игрока.
    /// Паросочетание с ёмкостями (путь увеличения), а не жадный выбор: доски/палки пересекаются.
    pub fn can_craft(&self, have: &std::collections::HashMap<u32, i32>) -> bool {
        let Some(req) = &self.requirements else {
            return false;
        };
        let opts: Vec<Vec<u32>> = req.iter().map(Ingredient::items).collect();
        let mut used: std::collections::HashMap<u32, Vec<usize>> = Default::default();
        fn place(
            i: usize,
            opts: &[Vec<u32>],
            have: &std::collections::HashMap<u32, i32>,
            used: &mut std::collections::HashMap<u32, Vec<usize>>,
            seen: &mut std::collections::HashSet<u32>,
        ) -> bool {
            for &it in &opts[i] {
                let cap = have.get(&it).copied().unwrap_or(0).max(0) as usize;
                if cap == 0 || !seen.insert(it) {
                    continue;
                }
                let holders = used.get(&it).cloned().unwrap_or_default();
                if holders.len() < cap {
                    used.entry(it).or_default().push(i);
                    return true;
                }
                // вытеснить одного из владельцев на другой предмет
                for &o in &holders {
                    if place(o, opts, have, used, seen) {
                        // o ушёл на другой предмет (и добавлен туда), освобождаем место здесь
                        let h = used.entry(it).or_default();
                        h.retain(|&x| x != o);
                        h.push(i);
                        return true;
                    }
                }
            }
            false
        }
        (0..opts.len()).all(|i| place(i, &opts, have, &mut used, &mut Default::default()))
    }
}

/// VANILLA: ClientRecipeBook (known, highlight, settings). Коллекции/вкладки — в экране (323c).
#[derive(Default)]
pub struct RecipeBook {
    pub known: std::collections::BTreeMap<i32, RecipeEntry>,
    pub highlight: std::collections::HashSet<i32>,
    pub settings: Settings,
    /// Текущая страница сетки (RecipeBookPage#currentPage).
    pub page: usize,
    /// Выбранная вкладка книги крафта (0 — «поиск»).
    pub tab: usize,
    /// Поле поиска и его фокус (VANILLA: RecipeBookComponent#searchBox).
    pub search: crate::editbox::EditBox,
    pub search_focus: bool,
    /// Имена результатов (нижний регистр) для поиска; пересчитываются при смене числа рецептов.
    pub names: std::collections::HashMap<u32, String>,
    pub names_n: usize,
    /// Открытый оверлей вариантов (ПКМ по кнопке с несколькими рецептами).
    pub overlay: Option<crate::recipescreen::Overlay>,
}

impl RecipeBook {
    /// VANILLA: ClientPacketListener#handleRecipeBookAdd/Remove/Settings.
    /// Возвращает (станция, результат) для тостов.
    pub fn apply(&mut self, u: Update) -> Vec<RecipeNote> {
        let mut notes = Vec::new();
        match u {
            Update::Add { entries, replace } => {
                if replace {
                    self.known.clear();
                    self.highlight.clear();
                }
                for e in entries {
                    if e.highlight {
                        self.highlight.insert(e.id);
                    }
                    if e.notification {
                        notes.push((e.station.stack.clone(), e.result.stack.clone()));
                    }
                    self.known.insert(e.id, e);
                }
            }
            Update::Remove(ids) => {
                for id in ids {
                    self.known.remove(&id);
                    self.highlight.remove(&id);
                }
            }
            Update::Settings(s) => self.settings = s,
        }
        notes
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn place_recipe_centers_like_vanilla() {
        use super::place_recipe;
        // 2×2 рецепт в сетке 3×3: ячейки 0,1,3,4
        let v = place_recipe(3, 3, 2, 2, 4);
        assert_eq!(v, vec![(0, 0), (1, 1), (3, 2), (4, 3)]);
        // 1×1 в 3×3: центр (ячейка 4)
        assert_eq!(place_recipe(3, 3, 1, 1, 1), vec![(4, 0)]);
        // 3×3 целиком
        assert_eq!(place_recipe(3, 3, 3, 3, 9).len(), 9);
    }

    use super::*;

    // 1 запись: id=0, shapeless, 1 ингредиент item(1), результат item(2), станция item(3),
    // group=0, category=3, requirements=нет, flags=notification; replace=false.
    const B: [u8; 15] = [1, 0, 0, 1, 4, 1, 4, 2, 4, 3, 0, 3, 0, 1, 0];

    #[test]
    fn shapeless_with_notification() {
        let mut book = RecipeBook::default();
        let notes = book.apply(parse_add(&B).unwrap());
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].0.as_ref().unwrap().item, 3);
        assert_eq!(notes[0].1.as_ref().unwrap().item, 2);
        let e = &book.known[&0];
        assert_eq!(e.kind, Kind::Shapeless);
        assert_eq!(e.category, 3);
        assert_eq!(e.ingredients[0].stack.as_ref().unwrap().item, 1);
        // без notification — тоста нет
        let mut c = B;
        c[13] = 2;
        let mut book = RecipeBook::default();
        assert!(book.apply(parse_add(&c).unwrap()).is_empty());
        assert!(book.highlight.contains(&0));
    }

    #[test]
    fn remove_and_replace() {
        let mut book = RecipeBook::default();
        book.apply(parse_add(&B).unwrap());
        book.apply(parse_remove(&[1, 0]).unwrap());
        assert!(book.known.is_empty());
        book.apply(parse_add(&B).unwrap());
        let mut rep = B;
        rep[14] = 1;
        rep[1] = 5; // другой id
        book.apply(parse_add(&rep).unwrap());
        assert_eq!(book.known.keys().copied().collect::<Vec<_>>(), vec![5]);
    }

    #[test]
    fn settings() {
        let u = parse_settings(&[1, 0, 0, 1, 1, 1, 0, 0]).unwrap();
        let Update::Settings(s) = u else { panic!() };
        assert!(s.crafting.open && !s.crafting.filtering);
        assert!(s.furnace.filtering && s.blast_furnace.open && s.blast_furnace.filtering);
        assert!(!s.smoker.open);
    }

    fn req(r: Vec<Ingredient>) -> RecipeEntry {
        RecipeEntry {
            id: 0,
            kind: Kind::Shapeless,
            ingredients: vec![],
            result: Sd::default(),
            station: Sd::default(),
            group: None,
            category: 0,
            requirements: Some(r),
            notification: false,
            highlight: false,
        }
    }

    #[test]
    fn can_craft_matching() {
        use std::collections::HashMap;
        let have: HashMap<u32, i32> = [(1, 1), (2, 1)].into();
        // A:{1,2}, B:{1} — жадность A→1 провалила бы B; паросочетание: A→2, B→1.
        let e = req(vec![
            Ingredient::Items(vec![1, 2]),
            Ingredient::Items(vec![1]),
        ]);
        assert!(e.can_craft(&have));
        // три ингредиента на два предмета
        let e = req(vec![Ingredient::Items(vec![1, 2]); 3]);
        assert!(!e.can_craft(&have));
        // количество
        let have: HashMap<u32, i32> = [(1, 3)].into();
        assert!(req(vec![Ingredient::Items(vec![1]); 3]).can_craft(&have));
        assert!(!req(vec![Ingredient::Items(vec![1]); 4]).can_craft(&have));
        // без требований — нельзя
        let mut e = req(vec![]);
        e.requirements = None;
        assert!(!e.can_craft(&have));
    }

    #[test]
    fn tag_planks_expands() {
        let ids = Ingredient::Tag("minecraft:planks".into()).items();
        assert!(ids.len() > 5, "{ids:?}");
    }
}
