//! Книга рецептов 1.12–1.21.1 → пакеты канона.
//!
//! Сервер присылает все рецепты (`update_recipes`, id — строки) и отдельно открытые игроку
//! (`recipe`). 1.12.x рецептов не присылает: id числовые, список взят из jar
//! (`data/recipes_1_12.txt`, генератор `tools/gen_recipes_1_12.py`). Канон ждёт `recipe_book_add/remove/settings` с числовыми id и готовыми дисплеями,
//! поэтому рецепты запоминаются здесь, а числовой id — номер в списке сервера.
//! VANILLA: ClientboundUpdateRecipesPacket, *Recipe.Serializer#fromNetwork,
//! ClientboundRecipePacket, ClientboundPlaceGhostRecipePacket, ServerboundPlaceRecipePacket,
//! ServerboundRecipeBookUpdatePacket (1.13.2–1.20.1), RecipeHolder.STREAM_CODEC,
//! Ingredient.CONTENTS_STREAM_CODEC (1.20.6, 1.21.1).

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use ezzmc_protocol::generated::r1_21_1::RECIPE_SERIALIZER;
use ezzmc_protocol::generated::r26_3::ITEM;
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};

use crate::v1201::item_to_canon;

/// Первая версия с книгой рецептов (1.12) и последняя с этими форматами (1.21.1).
const FIRST: i32 = 335;
const LAST: i32 = 767;

#[derive(Default)]
struct Book {
    by_name: HashMap<String, usize>,
    recipes: Vec<Recipe>,
    /// (открыта, только доступные) × крафт, печь, плавильня, коптильня.
    settings: [bool; 8],
}

struct Recipe {
    name: String,
    /// `RecipeDisplay` канона.
    display: Vec<u8>,
    group: Option<i32>,
    category: i32,
    /// `Optional<List<Ingredient>>` канона.
    requirements: Vec<u8>,
    notify: bool,
}

static BOOK: LazyLock<Mutex<Book>> = LazyLock::new(Default::default);

fn book() -> MutexGuard<'static, Book> {
    BOOK.lock().unwrap_or_else(PoisonError::into_inner)
}

fn protocol() -> i32 {
    ezzmc_protocol::version::current().protocol
}

/// Версия соединения переводится этим модулем.
pub fn supported() -> bool {
    (FIRST..=LAST).contains(&protocol())
}

/// Стек в разобранном виде: (id канона, количество, патч компонентов).
#[derive(Clone)]
struct Stack {
    item: i32,
    count: i32,
    patch: Vec<u8>,
}

fn stack(r: &mut Reader) -> ProtocolResult<Option<Stack>> {
    let mut canon = Writer::new();
    if protocol() < 404 {
        // 1.13/1.13.1: стек начинается с short id.
        let mut next = Writer::new();
        crate::v1132::item_to_1_13_2(r, &mut next)?;
        item_to_canon(&mut Reader::new(&next.into_vec()), &mut canon)?;
    } else if protocol() <= 763 {
        item_to_canon(r, &mut canon)?;
    } else {
        crate::legacy::stack(r, &mut canon)?;
    }
    parse_canon(&canon.into_vec())
}

/// Стек 1.13.2 → разобранный стек канона.
fn canon_stack(r: &mut Reader) -> ProtocolResult<Option<Stack>> {
    let mut canon = Writer::new();
    item_to_canon(r, &mut canon)?;
    parse_canon(&canon.into_vec())
}

fn parse_canon(canon: &[u8]) -> ProtocolResult<Option<Stack>> {
    let mut s = Reader::new(canon);
    let count = s.var_int()?;
    if count <= 0 {
        return Ok(None);
    }
    let item = s.var_int()?;
    Ok(Some(Stack {
        item,
        count,
        patch: s.rest().to_vec(),
    }))
}

/// `Ingredient`: список подходящих стеков.
fn ingredient(r: &mut Reader) -> ProtocolResult<Vec<i32>> {
    let n = r.length("ingredient", 4096)?;
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(s) = stack(r)? {
            items.push(s.item);
        }
    }
    Ok(items)
}

fn ingredients(r: &mut Reader, n: usize) -> ProtocolResult<Vec<Vec<i32>>> {
    (0..n).map(|_| ingredient(r)).collect()
}

/// SlotDisplay: empty (0), item (4) или composite (10).
fn slot(w: &mut Writer, items: &[i32]) {
    match items {
        [] => {
            w.var_int(0);
        }
        [one] => {
            w.var_int(4).var_int(*one);
        }
        many => {
            w.var_int(10).var_int(many.len() as i32);
            for item in many {
                w.var_int(4).var_int(*item);
            }
        }
    }
}

/// SlotDisplay результата: item_stack (ItemStackTemplate).
fn result_slot(w: &mut Writer, s: Option<&Stack>) {
    match s {
        Some(s) => {
            w.var_int(5)
                .var_int(s.item)
                .var_int(s.count)
                .bytes(&s.patch);
        }
        None => {
            w.var_int(0);
        }
    }
}

fn item_id(name: &str) -> Option<i32> {
    ITEM.iter().position(|n| *n == name).map(|i| i as i32)
}

fn station(w: &mut Writer, name: &str) {
    match item_id(name) {
        Some(i) => slot(w, &[i]),
        None => slot(w, &[]),
    }
}

/// До 1.19.3 категорию выводит клиент по результату; берём её из данных рецептов 26.3.
/// VANILLA-UNSURE: ClientRecipeBook#getCategory (≤1.19.2) смотрит на вкладку творческого режима.
fn category_by_result(cooking: bool, result: Option<&Stack>) -> i32 {
    static T: LazyLock<HashMap<(&'static str, &'static str), i32>> = LazyLock::new(|| {
        include_str!("../data/recipe_categories.txt")
            .lines()
            .filter_map(|l| {
                let mut it = l.split(' ');
                Some(((it.next()?, it.next()?), it.next()?.parse().ok()?))
            })
            .collect()
    });
    let name = result.and_then(|s| ITEM.get(usize::try_from(s.item).ok()?));
    let (ns, misc) = if cooking { ("f", 2) } else { ("c", 3) };
    name.and_then(|n| T.get(&(ns, *n)).copied()).unwrap_or(misc)
}

struct Parsed {
    display: Vec<u8>,
    group: String,
    category: i32,
    required: Vec<Vec<i32>>,
    notify: bool,
}

/// Тело рецепта после типа и id. `None` — особый рецепт без дисплея (в книге не показывается).
fn recipe(kind: &str, r: &mut Reader) -> ProtocolResult<Option<Parsed>> {
    let v = protocol();
    // 1.19.3+: категория приходит с сервера.
    let categories = v >= 761;
    let mut w = Writer::new();
    let mut notify = true;
    let (group, category, required) = match kind {
        "crafting_shaped" => {
            // 1.20.3 перенёс размер сетки после группы и категории.
            let size_first = v < 765;
            let mut size = (0, 0);
            if size_first {
                size = (r.var_int()?, r.var_int()?);
            }
            let group = r.string(32767)?.to_string();
            let sent = if categories { Some(r.var_int()?) } else { None };
            if !size_first {
                size = (r.var_int()?, r.var_int()?);
            }
            let (width, height) = size;
            let cells = usize::try_from(width.saturating_mul(height))
                .ok()
                .filter(|n| *n <= 4096)
                .ok_or(ProtocolError::InvalidValue {
                    what: "shaped recipe size",
                    value: i64::from(width),
                })?;
            let grid = ingredients(r, cells)?;
            let out = stack(r)?;
            if v >= 762 {
                notify = r.bool()?;
            }
            w.var_int(1).var_int(width).var_int(height);
            w.var_int(grid.len() as i32);
            for i in &grid {
                slot(&mut w, i);
            }
            result_slot(&mut w, out.as_ref());
            station(&mut w, "crafting_table");
            let category = sent.unwrap_or_else(|| category_by_result(false, out.as_ref()));
            (group, category, grid)
        }
        "crafting_shapeless" => {
            let group = r.string(32767)?.to_string();
            let sent = if categories { Some(r.var_int()?) } else { None };
            let n = r.length("shapeless recipe", 4096)?;
            let list = ingredients(r, n)?;
            let out = stack(r)?;
            w.var_int(0).var_int(list.len() as i32);
            for i in &list {
                slot(&mut w, i);
            }
            result_slot(&mut w, out.as_ref());
            station(&mut w, "crafting_table");
            let category = sent.unwrap_or_else(|| category_by_result(false, out.as_ref()));
            (group, category, list)
        }
        "smelting" | "blasting" | "smoking" | "campfire_cooking" => {
            let group = r.string(32767)?.to_string();
            let sent = if categories { Some(r.var_int()?) } else { None };
            let input = ingredient(r)?;
            let out = stack(r)?;
            let experience = r.f32()?;
            let time = r.var_int()?;
            w.var_int(2);
            slot(&mut w, &input);
            w.var_int(1); // топливо: any_fuel
            result_slot(&mut w, out.as_ref());
            // CookingBookCategory: еда, блоки, разное.
            let cooking = sent.unwrap_or_else(|| category_by_result(true, out.as_ref()));
            let (at, category) = match kind {
                "smelting" => ("furnace", 4 + cooking.clamp(0, 2)),
                "blasting" => ("blast_furnace", if cooking == 1 { 7 } else { 8 }),
                "smoking" => ("smoker", 9),
                _ => ("campfire", 12),
            };
            station(&mut w, at);
            w.var_int(time).f32(experience);
            (group, category, vec![input])
        }
        "stonecutting" => {
            let group = r.string(32767)?.to_string();
            let input = ingredient(r)?;
            let out = stack(r)?;
            w.var_int(3);
            slot(&mut w, &input);
            result_slot(&mut w, out.as_ref());
            station(&mut w, "stonecutter");
            (group, 10, vec![input])
        }
        "smithing" | "smithing_transform" | "smithing_trim" => {
            let template = if kind == "smithing" {
                Vec::new()
            } else {
                ingredient(r)?
            };
            let base = ingredient(r)?;
            let addition = ingredient(r)?;
            let out = if kind == "smithing_trim" {
                None
            } else {
                stack(r)?
            };
            w.var_int(4);
            for i in [&template, &base, &addition] {
                slot(&mut w, i);
            }
            match &out {
                Some(s) => result_slot(&mut w, Some(s)),
                None => slot(&mut w, &base),
            }
            station(&mut w, "smithing_table");
            (String::new(), 11, vec![template, base, addition])
        }
        k if k.starts_with("crafting_special_") || k == "crafting_decorated_pot" => {
            if categories {
                r.var_int()?;
            }
            return Ok(None);
        }
        _ => {
            return Err(ProtocolError::InvalidValue {
                what: "recipe type",
                value: 0,
            })
        }
    };
    Ok(Some(Parsed {
        display: w.into_vec(),
        group,
        category,
        required,
        notify,
    }))
}

/// Имя и тип рецепта. 1.13.x: id, тип; 1.14–1.20.4: тип, id; с 1.20.5: id и номер сериализатора.
fn header(r: &mut Reader) -> ProtocolResult<(String, String)> {
    let first = r.string(32767)?.to_string();
    if protocol() >= 766 {
        let at = r.var_int()?;
        // Реестр recipe_serializer у 1.20.6 и 1.21.1 одинаков.
        let kind = usize::try_from(at)
            .ok()
            .and_then(|i| RECIPE_SERIALIZER.get(i))
            .ok_or(ProtocolError::InvalidValue {
                what: "recipe serializer",
                value: i64::from(at),
            })?;
        return Ok((first, (*kind).to_string()));
    }
    let second = r.string(32767)?.to_string();
    Ok(if protocol() < 477 {
        (first, second)
    } else {
        (second, first)
    })
}

/// `update_recipes`: запоминает рецепты сервера. Тело неизвестного (модового) типа разобрать
/// нельзя — остаются рецепты, прочитанные до него.
pub fn update(body: &[u8]) -> ProtocolResult<()> {
    let mut r = Reader::new(body);
    let mut b = book();
    b.by_name.clear();
    b.recipes.clear();
    let mut groups: HashMap<String, i32> = HashMap::new();
    for _ in 0..r.length("recipes", 1 << 20)? {
        let (name, kind) = header(&mut r)?;
        let kind = kind.strip_prefix("minecraft:").unwrap_or(&kind);
        let Some(p) = recipe(kind, &mut r)? else {
            continue;
        };
        add(&mut b, &mut groups, name, p);
    }
    Ok(())
}

fn add(b: &mut Book, groups: &mut HashMap<String, i32>, name: String, p: Parsed) {
    let group = (!p.group.is_empty()).then(|| {
        let next = groups.len() as i32;
        *groups.entry(p.group).or_insert(next)
    });
    let required: Vec<&Vec<i32>> = p.required.iter().filter(|i| !i.is_empty()).collect();
    let mut req = Writer::new();
    req.bool(true).var_int(required.len() as i32);
    for items in required {
        req.var_int(items.len() as i32 + 1);
        for i in items {
            req.var_int(*i);
        }
    }
    let at = b.recipes.len();
    b.by_name.insert(name.clone(), at);
    b.recipes.push(Recipe {
        name,
        display: p.display,
        group,
        category: p.category,
        requirements: req.into_vec(),
        notify: p.notify,
    });
}

/// Рецепты крафта 1.12.x по порядку id; первые [`SPECIAL_1_12`] id — особые, без дисплея.
const BUILTIN_1_12: &str = include_str!("../data/recipes_1_12.txt");
const SPECIAL_1_12: usize = 11;

/// `id:damage` 1.12.x → стек канона.
fn legacy_stack(id: i16, damage: i16, count: u8) -> ProtocolResult<Option<Stack>> {
    let mut old = Writer::new();
    old.i16(id).u8(count).i16(damage).u8(0);
    let mut flat = Writer::new();
    crate::v1122::item_to_1_13_2(&mut Reader::new(&old.into_vec()), &mut flat)?;
    canon_stack(&mut Reader::new(&flat.into_vec()))
}

/// Строка `data/recipes_1_12.txt`; `None` — строка испорчена.
fn builtin(line: &str) -> Option<Parsed> {
    let mut it = line.split(' ');
    let kind = it.next()?;
    let group = Some(it.next()?).filter(|g| *g != "-").unwrap_or_default();
    let size = if kind == "s" {
        Some((
            it.next()?.parse::<i32>().ok()?,
            it.next()?.parse::<i32>().ok()?,
        ))
    } else {
        None
    };
    let mut out = it.next()?.split(':').map(|n| n.parse::<i16>().ok());
    let (id, damage, count) = (out.next()??, out.next()??, out.next()??);
    let out = legacy_stack(id, damage, u8::try_from(count).ok()?).ok()?;
    let cells: Vec<Vec<i32>> = it
        .map(|cell| {
            cell.split(',')
                .filter_map(|v| {
                    let (id, damage) = v.split_once(':')?;
                    let s = legacy_stack(id.parse().ok()?, damage.parse().ok()?, 1);
                    Some(s.ok()??.item)
                })
                .collect()
        })
        .collect();
    let mut w = Writer::new();
    match size {
        Some((width, height)) => {
            w.var_int(1).var_int(width).var_int(height);
        }
        None => {
            w.var_int(0);
        }
    }
    w.var_int(cells.len() as i32);
    for c in &cells {
        slot(&mut w, c);
    }
    result_slot(&mut w, out.as_ref());
    station(&mut w, "crafting_table");
    Some(Parsed {
        display: w.into_vec(),
        group: group.to_string(),
        category: category_by_result(false, out.as_ref()),
        required: cells,
        notify: true,
    })
}

/// Заполняет книгу рецептами 1.12.x; имя рецепта — его числовой id.
fn load_1_12(b: &mut Book) {
    b.by_name.clear();
    b.recipes.clear();
    let mut groups = HashMap::new();
    for (i, line) in BUILTIN_1_12.lines().enumerate() {
        if let Some(p) = builtin(line) {
            add(b, &mut groups, (SPECIAL_1_12 + i).to_string(), p);
        }
    }
}

/// Id рецепта в пакете: строка, в 1.12.x — число.
fn recipe_name(r: &mut Reader) -> ProtocolResult<String> {
    if protocol() < 393 {
        Ok(r.var_int()?.to_string())
    } else {
        Ok(r.string(32767)?.to_string())
    }
}

fn names(r: &mut Reader) -> ProtocolResult<Vec<String>> {
    let n = r.length("recipe ids", 1 << 20)?;
    (0..n).map(|_| recipe_name(r)).collect()
}

/// `recipe` → [8 байт `recipe_book_settings`][u8: 1 — добавление, 2 — удаление][тело
/// `recipe_book_add` / `recipe_book_remove`]. Разбирает [`split`].
/// VANILLA: ClientPacketListener#handleAddOrRemoveRecipes (INIT, ADD, REMOVE).
pub fn recipe_to_canon(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let state = r.var_int()?;
    let mut b = book();
    if protocol() < 393 && state == 0 {
        load_1_12(&mut b);
    }
    // 1.12.x: настройки только у крафта; до 1.16.2 — у крафта и печи.
    let sent = settings_sent();
    b.settings = [false; 8];
    for i in 0..sent {
        b.settings[i] = r.bool()?;
    }
    let ids = names(&mut r)?;
    let highlight: std::collections::HashSet<String> = if state == 0 {
        names(&mut r)?.into_iter().collect()
    } else {
        Default::default()
    };
    let mut w = Writer::new();
    for s in b.settings {
        w.bool(s);
    }
    let known: Vec<usize> = ids
        .iter()
        .filter_map(|n| b.by_name.get(n).copied())
        .collect();
    if state == 2 {
        w.u8(2).var_int(known.len() as i32);
        for at in known {
            w.var_int(at as i32);
        }
        return Ok(w.into_vec());
    }
    w.u8(1).var_int(known.len() as i32);
    for at in known {
        let Some(x) = b.recipes.get(at) else { continue };
        w.var_int(at as i32).bytes(&x.display);
        w.var_int(x.group.map_or(0, |g| g + 1)).var_int(x.category);
        w.bytes(&x.requirements);
        // Флаги: 1 — уведомление, 2 — подсветка.
        let flags = if state == 1 {
            2 | u8::from(x.notify)
        } else if highlight.contains(&x.name) {
            2
        } else {
            0
        };
        w.u8(flags);
    }
    w.bool(state == 0);
    Ok(w.into_vec())
}

/// Число bool настроек книги в пакетах версии.
fn settings_sent() -> usize {
    match protocol() {
        ..393 => 2,
        393..751 => 4,
        _ => 8,
    }
}

/// Тело из [`recipe_to_canon`] → (настройки, (добавление?, тело)).
pub fn split(body: &[u8]) -> Option<(&[u8], bool, &[u8])> {
    let settings = body.get(..8)?;
    let add = *body.get(8)? == 1;
    Some((settings, add, body.get(9..)?))
}

/// `place_ghost_recipe`: (байт окна, имя) → (окно, RecipeDisplay). `None` — рецепт неизвестен.
pub fn ghost_to_canon(body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let container = i32::from(r.u8()?);
    let name = recipe_name(&mut r)?;
    let b = book();
    Ok(b.by_name
        .get(&name)
        .and_then(|at| b.recipes.get(*at))
        .map(|x| {
            let mut w = Writer::new();
            w.var_int(container).bytes(&x.display);
            w.into_vec()
        }))
}

/// `place_recipe` канона (окно, номер, всё) → (байт окна, имя, всё). `false` — рецепт неизвестен.
pub fn place_from_canon(r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
    let container = r.var_int()?;
    let at = usize::try_from(r.var_int()?).unwrap_or(usize::MAX);
    let use_max = r.bool()?;
    let b = book();
    let Some(x) = b.recipes.get(at) else {
        return Ok(false);
    };
    w.u8(container as u8);
    if protocol() < 393 {
        let Ok(id) = x.name.parse() else {
            return Ok(false);
        };
        w.var_int(id);
    } else {
        w.string(&x.name);
    }
    w.bool(use_max);
    Ok(true)
}

/// `recipe_book_seen_recipe` канона (номер) → имя. `false` — рецепт неизвестен.
pub fn seen_from_canon(r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
    let at = usize::try_from(r.var_int()?).unwrap_or(usize::MAX);
    let b = book();
    let Some(x) = b.recipes.get(at) else {
        return Ok(false);
    };
    w.string(&x.name);
    Ok(true)
}

/// Тело `recipe_book_update` (SETTINGS) для 1.13–1.16.1: сервер ждёт все настройки разом.
/// VANILLA: ServerboundRecipeBookUpdatePacket#write — 1.12.x два bool, 1.13.x четыре,
/// с 1.14 восемь.
pub fn settings_update(book_type: i32, open: bool, filtering: bool) -> Vec<u8> {
    let mut b = book();
    if let Ok(t @ 0..=3) = usize::try_from(book_type) {
        b.settings[t * 2] = open;
        b.settings[t * 2 + 1] = filtering;
    }
    let mut w = Writer::new();
    w.var_int(1);
    // 1.14–1.16.1 уже шлют восемь.
    let n = match protocol() {
        ..393 => 2,
        393..477 => 4,
        _ => 8,
    };
    for s in &b.settings[..n] {
        w.bool(*s);
    }
    w.into_vec()
}
