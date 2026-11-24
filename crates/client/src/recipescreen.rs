//! VANILLA: RecipeBookComponent / RecipeBookPage / RecipeButton (книга крафта, TODO-323c).
//! Срез 1: окно инвентаря и верстак, одна вкладка «все рецепты», сетка 5×4, страницы,
//! фильтр «только доступные», клик → ServerboundPlaceRecipe. Вкладки, поиск, оверлей
//! вариантов и ghost-рецепт — следующими срезами.

use std::collections::{BTreeMap, HashMap};

use crate::inventory::Inventory;
use crate::invscreen::Layout;
use crate::recipebook::{BookKind, Kind, RecipeBook, RecipeEntry};

pub const SPRITES: &[(&str, &str)] = &[
    ("rb_bg", "recipe_book"),
    ("rb_ov_bg", "sprites/recipe_book/overlay_recipe"),
    ("rb_ov", "sprites/recipe_book/crafting_overlay"),
    (
        "rb_ov_hl",
        "sprites/recipe_book/crafting_overlay_highlighted",
    ),
    ("rb_ov_x", "sprites/recipe_book/crafting_overlay_disabled"),
    (
        "rb_ov_x_hl",
        "sprites/recipe_book/crafting_overlay_disabled_highlighted",
    ),
    ("rb_fov", "sprites/recipe_book/furnace_overlay"),
    (
        "rb_fov_hl",
        "sprites/recipe_book/furnace_overlay_highlighted",
    ),
    ("rb_fov_x", "sprites/recipe_book/furnace_overlay_disabled"),
    (
        "rb_fov_x_hl",
        "sprites/recipe_book/furnace_overlay_disabled_highlighted",
    ),
    ("rb_ffilt_on", "sprites/recipe_book/furnace_filter_enabled"),
    (
        "rb_ffilt_on_hl",
        "sprites/recipe_book/furnace_filter_enabled_highlighted",
    ),
    (
        "rb_ffilt_off",
        "sprites/recipe_book/furnace_filter_disabled",
    ),
    (
        "rb_ffilt_off_hl",
        "sprites/recipe_book/furnace_filter_disabled_highlighted",
    ),
    ("rb_btn", "sprites/recipe_book/button"),
    ("rb_btn_hl", "sprites/recipe_book/button_highlighted"),
    ("rb_slot", "sprites/recipe_book/slot_craftable"),
    ("rb_slot_x", "sprites/recipe_book/slot_uncraftable"),
    ("rb_slot_many", "sprites/recipe_book/slot_many_craftable"),
    (
        "rb_slot_many_x",
        "sprites/recipe_book/slot_many_uncraftable",
    ),
    ("rb_fwd", "sprites/recipe_book/page_forward"),
    ("rb_fwd_hl", "sprites/recipe_book/page_forward_highlighted"),
    ("rb_back", "sprites/recipe_book/page_backward"),
    (
        "rb_back_hl",
        "sprites/recipe_book/page_backward_highlighted",
    ),
    ("rb_tab", "sprites/recipe_book/tab"),
    ("rb_tab_sel", "sprites/recipe_book/tab_selected"),
    ("rb_filt_on", "sprites/recipe_book/filter_enabled"),
    (
        "rb_filt_on_hl",
        "sprites/recipe_book/filter_enabled_highlighted",
    ),
    ("rb_filt_off", "sprites/recipe_book/filter_disabled"),
    (
        "rb_filt_off_hl",
        "sprites/recipe_book/filter_disabled_highlighted",
    ),
];

const ITEMS_PER_PAGE: usize = 20;
type Tab = (Option<i32>, &'static str, Option<&'static str>);

/// VANILLA: *Screen#TABS — (категория или None = «поиск», иконки). Id реестра `recipe_book_category`:
/// 0..3 крафт (блоки, механизмы, снаряжение, разное), 4..6 печь (еда, блоки, разное),
/// 7..8 плавильня (блоки, разное), 9 коптильня (еда).
fn tab_defs(k: BookKind) -> &'static [Tab] {
    match k {
        BookKind::Crafting => &[
            (None, "compass", None),
            (Some(2), "iron_axe", Some("golden_sword")),
            (Some(0), "bricks", None),
            (Some(3), "lava_bucket", Some("apple")),
            (Some(1), "redstone", None),
        ],
        BookKind::Furnace => &[
            (None, "compass", None),
            (Some(4), "porkchop", None),
            (Some(5), "stone", None),
            (Some(6), "lava_bucket", Some("emerald")),
        ],
        BookKind::Blast => &[
            (None, "compass", None),
            (Some(7), "redstone_ore", None),
            (Some(8), "iron_shovel", Some("golden_leggings")),
        ],
        BookKind::Smoker => &[(None, "compass", None), (Some(9), "porkchop", None)],
    }
}

/// Категории, входящие в вкладку «поиск».
fn book_categories(k: BookKind) -> std::ops::RangeInclusive<i32> {
    match k {
        BookKind::Crafting => 0..=3,
        BookKind::Furnace => 4..=6,
        BookKind::Blast => 7..=8,
        BookKind::Smoker => 9..=9,
    }
}

/// Тип книги окна и размер сетки крафта (для печей не используется).
#[derive(Clone, Copy)]
pub struct Ctx {
    kind: BookKind,
    grid: u32,
}

fn ctx(lay: &Layout) -> Ctx {
    Ctx {
        kind: lay.book_kind().unwrap_or(BookKind::Crafting),
        grid: if lay.is_player() { 2 } else { 3 },
    }
}

fn named(name: &str) -> Option<ezzmc_protocol::item::ItemStack> {
    let id = ezzmc_protocol::generated::r26_3::ITEM
        .iter()
        .position(|n| *n == name)?;
    Some(ezzmc_protocol::item::ItemStack {
        item: id as _,
        count: 1,
        patch: Default::default(),
    })
}

/// VANILLA: CraftingRecipeBookComponent#canDisplay — рецепт влезает в сетку меню; печи — любой Furnace.
fn can_display(e: &RecipeEntry, cx: Ctx) -> bool {
    let grid = cx.grid;
    match e.kind {
        Kind::Shaped { w, h } => cx.kind == BookKind::Crafting && grid >= w && grid >= h,
        Kind::Shapeless => {
            cx.kind == BookKind::Crafting && (grid * grid) as usize >= e.ingredients.len()
        }
        Kind::Furnace { .. } => cx.kind != BookKind::Crafting,
        _ => false,
    }
}

/// VANILLA: RecipeBookTabButton#updateVisibility — «поиск» всегда, остальные при наличии рецептов.
fn visible_tabs(known: &BTreeMap<i32, RecipeEntry>, cx: Ctx) -> Vec<usize> {
    let t = tab_defs(cx.kind);
    (0..t.len())
        .filter(|&i| match t[i].0 {
            None => true,
            Some(c) => known
                .values()
                .any(|e| e.category == c && can_display(e, cx)),
        })
        .collect()
}

/// Позиция i-й видимой вкладки. VANILLA: RecipeBookComponent#updateTabs.
fn tab_pos(g: &Geo, idx: usize) -> (f32, f32) {
    (g.xo - 30.0, g.yo + 3.0 + 27.0 * idx as f32)
}

/// VANILLA: RecipeCollection — рецепты одной группы (или один рецепт без группы).
pub struct Collection<'a> {
    pub recipes: Vec<&'a RecipeEntry>,
    pub craftable: Vec<bool>,
}

impl Collection<'_> {
    pub fn has_craftable(&self) -> bool {
        self.craftable.iter().any(|c| *c)
    }
    /// VANILLA: RecipeButton#getCurrentRecipe (без цикла по времени — первый подходящий).
    pub fn current(&self, filtering: bool) -> Option<&RecipeEntry> {
        self.recipes
            .iter()
            .zip(&self.craftable)
            .find(|(_, c)| **c || !filtering)
            .map(|(r, _)| *r)
    }
}

/// Позиция и размеры элементов книги (GUI-px).
pub struct Geo {
    pub xo: f32,
    pub yo: f32,
}

impl Geo {
    /// VANILLA: RecipeBookComponent#getXOrigin/getYOrigin.
    pub fn new(gw: f32, gh: f32) -> Self {
        let off = if gw < 379.0 { 0.0 } else { 86.0 };
        Self {
            xo: ((gw - 147.0) / 2.0).floor() - off,
            yo: ((gh - 166.0) / 2.0).floor(),
        }
    }
    fn cell(&self, i: usize) -> (f32, f32) {
        (
            self.xo + 11.0 + 25.0 * (i % 5) as f32,
            self.yo + 31.0 + 25.0 * (i / 5) as f32,
        )
    }
    fn fwd(&self) -> (f32, f32) {
        (self.xo + 93.0, self.yo + 137.0)
    }
    fn back(&self) -> (f32, f32) {
        (self.xo + 38.0, self.yo + 137.0)
    }
    fn filter(&self) -> (f32, f32) {
        (self.xo + 110.0, self.yo + 12.0)
    }
}

fn inside(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && my >= y && mx < x + w && my < y + h
}

/// Кнопка-переключатель книги: (x, y, w, h).
/// VANILLA: InventoryScreen (leftPos+104, height/2−22) и CraftingScreen (leftPos+5, height/2−49), 20×18.
pub fn toggle_rect(lay: &Layout, gw: f32, gh: f32) -> Option<(f32, f32, f32, f32)> {
    if !lay.has_crafting_book() {
        return None;
    }
    let l = lay.left(gw);
    let c = (gh / 2.0).floor();
    Some(if lay.is_player() {
        (l + 104.0, c - 22.0, 20.0, 18.0)
    } else if ctx(lay).kind != BookKind::Crafting {
        // VANILLA: AbstractFurnaceScreen#getRecipeBookButtonPosition
        (l + 20.0, c - 49.0, 20.0, 18.0)
    } else {
        (l + 5.0, c - 49.0, 20.0, 18.0)
    })
}

/// VANILLA: Inventory#fillStackedContents + menu.fillCraftSlotsStackedContents.
fn have(inv: &Inventory, kind: BookKind) -> HashMap<u32, i32> {
    let mut m = HashMap::new();
    let mut add = |s: &Option<ezzmc_protocol::item::ItemStack>| {
        if let Some(s) = s {
            *m.entry(s.item).or_insert(0) += s.count;
        }
    };
    match &inv.open {
        // печи: 0 вход, 1 топливо, 2 результат, 3..=38 инвентарь (fillCraftSlotsStackedContents — вход)
        Some(o) if kind != BookKind::Crafting => {
            o.slots.iter().take(1).for_each(&mut add);
            o.slots.iter().skip(3).take(36).for_each(&mut add);
        }
        // верстак: 0 результат, 1..=9 сетка, 10..=45 инвентарь
        Some(o) => o.slots.iter().skip(1).take(45).for_each(&mut add),
        // InventoryMenu: 1..=4 сетка, 9..=44 инвентарь (без брони 5..=8 и второй руки)
        None => {
            inv.slots.iter().skip(1).take(4).for_each(&mut add);
            inv.slots.iter().skip(9).take(36).for_each(&mut add);
        }
    }
    m
}

/// Коллекции выбранной вкладки (0 — «поиск», все категории крафта) с рецептами, влезающими в сетку.
pub fn collections<'a>(
    known: &'a BTreeMap<i32, RecipeEntry>,
    inv: &Inventory,
    cx: Ctx,
    tab: usize,
    names: &HashMap<u32, String>,
    term: &str,
) -> Vec<Collection<'a>> {
    let want = tab_defs(cx.kind).get(tab).and_then(|t| t.0);
    let have = have(inv, cx.kind);
    let mut out: Vec<Collection> = Vec::new();
    let mut by_group: HashMap<i32, usize> = HashMap::new();
    for e in known.values() {
        if !book_categories(cx.kind).contains(&e.category)
            || want.is_some_and(|c| c != e.category)
            || !can_display(e, cx)
        {
            continue;
        }
        let ok = e.can_craft(&have);
        match e.group.and_then(|g| by_group.get(&g).copied()) {
            Some(i) => {
                out[i].recipes.push(e);
                out[i].craftable.push(ok);
            }
            None => {
                if let Some(g) = e.group {
                    by_group.insert(g, out.len());
                }
                out.push(Collection {
                    recipes: vec![e],
                    craftable: vec![ok],
                });
            }
        }
    }
    // VANILLA: SearchTree по имени результата (здесь подстрока без учёта регистра).
    let term = term.to_lowercase();
    if !term.is_empty() {
        out.retain(|c| {
            c.recipes.iter().any(|r| {
                r.result
                    .stack
                    .as_ref()
                    .and_then(|s| names.get(&s.item))
                    .is_some_and(|n| n.contains(&term))
            })
        });
    }
    out
}

/// Имена результатов для поиска (VANILLA: SessionSearchTrees#recipes).
pub fn sync_names(book: &mut RecipeBook, lang: &crate::ui::lang::Lang) {
    if book.names_n == book.known.len() {
        return;
    }
    book.names_n = book.known.len();
    for e in book.known.values() {
        if let Some(s) = &e.result.stack {
            book.names
                .entry(s.item)
                .or_insert_with(|| crate::tooltip::hover_name(lang, s).to_lowercase());
        }
    }
}

fn search_rect(g: &Geo) -> (f32, f32, f32, f32) {
    (g.xo + 25.0, g.yo + 13.0, 81.0, 14.0)
}

fn visible<'a>(cols: Vec<Collection<'a>>, filtering: bool) -> Vec<Collection<'a>> {
    if filtering {
        cols.into_iter().filter(Collection::has_craftable).collect()
    } else {
        cols
    }
}

pub enum Act {
    None,
    /// Клик поглощён книгой (не пускать в слоты).
    Handled,
    Place(i32, bool),
    Toggle,
    Filter,
}

/// VANILLA: RecipeBookComponent#mouseClicked + кнопка книги. Возвращает действие; состояние страницы
/// меняет сама.
#[allow(clippy::too_many_arguments)]
pub fn click(
    book: &mut RecipeBook,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
    left: bool,
    right: bool,
    shift: bool,
) -> Act {
    if !left && !right {
        return Act::None;
    }
    if let Some((x, y, w, h)) = toggle_rect(lay, gw, gh) {
        if inside(mx, my, x, y, w, h) {
            return Act::Toggle;
        }
    }
    if !lay.book {
        return Act::None;
    }
    let g = Geo::new(gw, gh);
    let filtering = book.settings.of(ctx(lay).kind).filtering;
    // VANILLA: RecipeBookPage#mouseClicked — открытый оверлей забирает клик; ЛКМ по варианту
    // ставит рецепт, любой другой клик его закрывает.
    if let Some(o) = book.overlay.take() {
        if left {
            if let Some(b) = o
                .buttons
                .iter()
                .find(|b| inside(mx, my, b.x, b.y, 24.0, 24.0))
            {
                return Act::Place(b.id, shift);
            }
        }
        return Act::Handled;
    }
    // VANILLA: RecipeBookComponent#mouseClicked — клик по полю или лупе фокусирует поиск, иначе снимает.
    let (sx, sy, sw, sh) = search_rect(&g);
    if inside(mx, my, g.xo + 8.0, sy, sx - g.xo - 8.0, sh) || inside(mx, my, sx, sy, sw, sh) {
        book.search_focus = true;
        book.search.max = 50;
        return Act::Handled;
    }
    book.search_focus = false;
    let tabs = visible_tabs(&book.known, ctx(lay));
    if !tabs.contains(&book.tab) {
        book.tab = 0;
    }
    for (idx, &t) in tabs.iter().enumerate() {
        let (x, y) = tab_pos(&g, idx);
        if inside(mx, my, x, y, 35.0, 27.0) {
            if book.tab != t {
                book.tab = t;
                book.page = 0;
            }
            return Act::Handled;
        }
    }
    let cols = visible(
        collections(
            &book.known,
            inv,
            ctx(lay),
            book.tab,
            &book.names,
            &book.search.text,
        ),
        filtering,
    );
    let pages = cols.len().div_ceil(ITEMS_PER_PAGE);
    book.page = book.page.min(pages.saturating_sub(1));
    let (fx, fy) = g.filter();
    if inside(mx, my, fx, fy, 26.0, 16.0) {
        return Act::Filter;
    }
    let (bx, by) = g.fwd();
    if pages > 1 && book.page + 1 < pages && inside(mx, my, bx, by, 12.0, 17.0) {
        book.page += 1;
        return Act::Handled;
    }
    let (bx, by) = g.back();
    if book.page > 0 && inside(mx, my, bx, by, 12.0, 17.0) {
        book.page -= 1;
        return Act::Handled;
    }
    for i in 0..ITEMS_PER_PAGE {
        let (x, y) = g.cell(i);
        if inside(mx, my, x, y, 25.0, 25.0) {
            if let Some(c) = cols.get(book.page * ITEMS_PER_PAGE + i) {
                if right {
                    // VANILLA: RecipeBookPage#mouseClicked — ПКМ: оверлей, если вариантов больше одного.
                    let n = if filtering {
                        c.craftable.iter().filter(|c| **c).count()
                    } else {
                        c.recipes.len()
                    };
                    if n > 1 {
                        book.overlay = Some(Overlay::new(c, filtering, x, y, &g, ctx(lay).kind));
                    }
                    return Act::Handled;
                }
                // VANILLA: RecipeBookComponent#tryPlaceRecipe — недоступный рецепт шлём один раз; здесь
                // сервер сам отвечает ghost-рецептом.
                if let Some(r) = c.current(filtering) {
                    return Act::Place(r.id, shift);
                }
            }
            return Act::Handled;
        }
    }
    // клик по панели книги не должен считаться кликом «вне окна»
    if inside(mx, my, g.xo, g.yo, 147.0, 166.0) {
        return Act::Handled;
    }
    Act::None
}

/// Фон, кнопка, сетка. Иконки результатов рисует `icon` (стек, x, y).
#[allow(clippy::too_many_arguments)]
/// VANILLA: GhostSlots#extractRenderState — призраки рецепта в слотах; вариант меняется раз в 30 тиков.
pub fn draw_ghost(
    p: &mut ezzmc_render::gui::GuiPainter,
    ghost: &[crate::recipebook::GhostSlot],
    lay: &Layout,
    gw: f32,
    gh: f32,
    now_ms: u128,
    icon: &mut dyn FnMut(
        &mut ezzmc_render::gui::GuiPainter,
        &ezzmc_protocol::item::ItemStack,
        f32,
        f32,
    ),
) {
    let (l, t) = (lay.left(gw), lay.top(gh));
    let uv = [0.0, 0.0, 1.0, 1.0];
    let idx = (now_ms / 1500) as usize;
    for g in ghost {
        let Some(Some((x, y))) = lay.slots.get(g.slot) else {
            continue;
        };
        let (x, y) = (l + x, t + y);
        if g.result {
            p.blit("white", x - 4.0, y - 4.0, 24.0, 24.0, uv, 0x30FF_0000);
        } else {
            p.blit("white", x, y, 16.0, 16.0, uv, 0x30FF_0000);
        }
        let item = g.items[idx % g.items.len()];
        let st = ezzmc_protocol::item::ItemStack {
            item,
            count: 1,
            patch: Default::default(),
        };
        icon(p, &st, x, y);
        p.blit("white", x, y, 16.0, 16.0, uv, 0x30FF_FFFF);
    }
}

pub fn draw(
    p: &mut ezzmc_render::gui::GuiPainter,
    book: &RecipeBook,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
    icon: &mut dyn FnMut(
        &mut ezzmc_render::gui::GuiPainter,
        &ezzmc_protocol::item::ItemStack,
        f32,
        f32,
    ),
) {
    let white = 0xFFFF_FFFF;
    if let Some((x, y, w, h)) = toggle_rect(lay, gw, gh) {
        let name = if inside(mx, my, x, y, w, h) {
            "rb_btn_hl"
        } else {
            "rb_btn"
        };
        p.blit(name, x, y, w, h, [0.0, 0.0, 1.0, 1.0], white);
    }
    if !lay.book {
        return;
    }
    let g = Geo::new(gw, gh);
    let k = 1.0 / 256.0;
    p.blit(
        "rb_bg",
        g.xo,
        g.yo,
        147.0,
        166.0,
        [k, k, 148.0 * k, 167.0 * k],
        white,
    );
    let tabs = visible_tabs(&book.known, ctx(lay));
    for (idx, &t) in tabs.iter().enumerate() {
        let sel = t == book.tab || (idx == 0 && !tabs.contains(&book.tab));
        let (tx, ty) = tab_pos(&g, idx);
        let tx = if sel { tx - 2.0 } else { tx };
        p.blit(
            if sel { "rb_tab_sel" } else { "rb_tab" },
            tx,
            ty,
            35.0,
            27.0,
            [0.0, 0.0, 1.0, 1.0],
            white,
        );
        let (a, b) = (tab_defs(ctx(lay).kind)[t].1, tab_defs(ctx(lay).kind)[t].2);
        match b {
            Some(b) => {
                for (n, dx) in [(a, 3.0), (b, 14.0)] {
                    if let Some(st) = named(n) {
                        icon(p, &st, tx + dx, ty + 5.0);
                    }
                }
            }
            None => {
                if let Some(st) = named(a) {
                    icon(p, &st, tx + 9.0, ty + 5.0);
                }
            }
        }
    }
    let filtering = book.settings.of(ctx(lay).kind).filtering;
    let (fx, fy) = g.filter();
    let hov = inside(mx, my, fx, fy, 26.0, 16.0);
    let name = match (filtering, hov, ctx(lay).kind == BookKind::Crafting) {
        (true, false, true) => "rb_filt_on",
        (true, true, true) => "rb_filt_on_hl",
        (false, false, true) => "rb_filt_off",
        (false, true, true) => "rb_filt_off_hl",
        (true, false, false) => "rb_ffilt_on",
        (true, true, false) => "rb_ffilt_on_hl",
        (false, false, false) => "rb_ffilt_off",
        (false, true, false) => "rb_ffilt_off_hl",
    };
    p.blit(name, fx, fy, 26.0, 16.0, [0.0, 0.0, 1.0, 1.0], white);
    let cols = visible(
        collections(
            &book.known,
            inv,
            ctx(lay),
            book.tab,
            &book.names,
            &book.search.text,
        ),
        filtering,
    );
    let pages = cols.len().div_ceil(ITEMS_PER_PAGE);
    let page = book.page.min(pages.saturating_sub(1));
    if pages > 1 && page + 1 < pages {
        let (x, y) = g.fwd();
        let n = if inside(mx, my, x, y, 12.0, 17.0) {
            "rb_fwd_hl"
        } else {
            "rb_fwd"
        };
        p.blit(n, x, y, 12.0, 17.0, [0.0, 0.0, 1.0, 1.0], white);
    }
    if page > 0 {
        let (x, y) = g.back();
        let n = if inside(mx, my, x, y, 12.0, 17.0) {
            "rb_back_hl"
        } else {
            "rb_back"
        };
        p.blit(n, x, y, 12.0, 17.0, [0.0, 0.0, 1.0, 1.0], white);
    }
    for i in 0..ITEMS_PER_PAGE {
        let Some(c) = cols.get(page * ITEMS_PER_PAGE + i) else {
            break;
        };
        let (x, y) = g.cell(i);
        let many = c.recipes.len() > 1;
        let sprite = match (c.has_craftable(), many) {
            (true, false) => "rb_slot",
            (true, true) => "rb_slot_many",
            (false, false) => "rb_slot_x",
            (false, true) => "rb_slot_many_x",
        };
        p.blit(sprite, x, y, 25.0, 25.0, [0.0, 0.0, 1.0, 1.0], white);
        if let Some(r) = c.current(filtering).or(c.recipes.first().copied()) {
            if let Some(s) = &r.result.stack {
                icon(p, s, x + 4.0, y + 4.0);
            }
        }
    }
}

/// Номер страницы «n/m» (VANILLA: gui.recipebook.page): текст и позиция центра.
pub fn page_label(
    book: &RecipeBook,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
) -> Option<(String, f32, f32)> {
    if !lay.book {
        return None;
    }
    let filtering = book.settings.of(ctx(lay).kind).filtering;
    let n = visible(
        collections(
            &book.known,
            inv,
            ctx(lay),
            book.tab,
            &book.names,
            &book.search.text,
        ),
        filtering,
    )
    .len();
    let pages = n.div_ceil(ITEMS_PER_PAGE);
    if pages <= 1 {
        return None;
    }
    let g = Geo::new(gw, gh);
    let page = book.page.min(pages - 1);
    Some((format!("{}/{}", page + 1, pages), g.xo + 73.0, g.yo + 141.0))
}

/// Рецепт под курсором (для тултипа): стек результата.
pub fn hover_stack(
    book: &RecipeBook,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) -> Option<ezzmc_protocol::item::ItemStack> {
    if !lay.book {
        return None;
    }
    let g = Geo::new(gw, gh);
    let filtering = book.settings.of(ctx(lay).kind).filtering;
    let cols = visible(
        collections(
            &book.known,
            inv,
            ctx(lay),
            book.tab,
            &book.names,
            &book.search.text,
        ),
        filtering,
    );
    let pages = cols.len().div_ceil(ITEMS_PER_PAGE);
    let page = book.page.min(pages.saturating_sub(1));
    for i in 0..ITEMS_PER_PAGE {
        let (x, y) = g.cell(i);
        if inside(mx, my, x, y, 25.0, 25.0) {
            let c = cols.get(page * ITEMS_PER_PAGE + i)?;
            let r = c.current(filtering).or(c.recipes.first().copied())?;
            return r.result.stack.clone();
        }
    }
    None
}

/// Предмет призрака под курсором. VANILLA: GhostSlots#extractTooltip (вариант по slotSelectTime).
pub fn hover_ghost(
    ghost: &[crate::recipebook::GhostSlot],
    lay: &Layout,
    gw: f32,
    gh: f32,
    now_ms: u128,
    mx: f32,
    my: f32,
) -> Option<ezzmc_protocol::item::ItemStack> {
    let (l, t) = (lay.left(gw), lay.top(gh));
    let idx = (now_ms / 1500) as usize;
    ghost.iter().find_map(|g| {
        let Some((x, y)) = lay.slots.get(g.slot).copied().flatten() else {
            return None;
        };
        if g.items.is_empty() || !inside(mx, my, l + x, t + y, 16.0, 16.0) {
            return None;
        }
        Some(ezzmc_protocol::item::ItemStack {
            item: g.items[idx % g.items.len()],
            count: 1,
            patch: Default::default(),
        })
    })
}

/// Тултип кнопки фильтра. VANILLA: RecipeBookComponent#filterButton (withTooltip) —
/// при фильтрации имя типа книги (craftable/smeltable/blastable/smokable), иначе `toggleRecipes.all`.
pub fn hover_filter(
    book: &RecipeBook,
    lay: &Layout,
    lang: &crate::ui::lang::Lang,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) -> Option<Vec<(String, u32)>> {
    if !lay.book {
        return None;
    }
    let (fx, fy) = Geo::new(gw, gh).filter();
    if !inside(mx, my, fx, fy, 26.0, 16.0) {
        return None;
    }
    let kind = ctx(lay).kind;
    let key = if book.settings.of(kind).filtering {
        match kind {
            BookKind::Crafting => "gui.recipebook.toggleRecipes.craftable",
            BookKind::Furnace => "gui.recipebook.toggleRecipes.smeltable",
            BookKind::Blast => "gui.recipebook.toggleRecipes.blastable",
            BookKind::Smoker => "gui.recipebook.toggleRecipes.smokable",
        }
    } else {
        "gui.recipebook.toggleRecipes.all"
    };
    Some(vec![(lang.tr(key), 0xFFFF_FFFF)])
}

/// Поле поиска: рамка, текст/подсказка, каретка. VANILLA: EditBox (81×14, подсказка gui.recipebook.search_hint).
#[allow(clippy::too_many_arguments)]
pub fn draw_search(
    t: &mut ezzmc_render::text::TextRenderer,
    book: &mut RecipeBook,
    lang: &crate::ui::lang::Lang,
    lay: &Layout,
    gw: f32,
    gh: f32,
    caret: bool,
) {
    if !lay.book {
        return;
    }
    let (x, y, w, h) = search_rect(&Geo::new(gw, gh));
    let border = if book.search_focus {
        0xFFFF_FFFF
    } else {
        0xFFA0_A0A0
    };
    t.fill(x, y, x + w, y + h, border);
    t.fill(x + 1.0, y + 1.0, x + w - 1.0, y + h - 1.0, 0xFF00_0000);
    let (tx, ty, tw) = (x + 4.0, y + 3.0, w - 8.0);
    let e = &mut book.search;
    if e.text.is_empty() {
        if !book.search_focus {
            let hint = lang.tr("gui.recipebook.search_hint");
            t.draw(&hint, tx, ty, 0xFF90_9090, false);
        }
    } else {
        let cs: Vec<char> = e.text.chars().collect();
        let sub = |a: usize, b: usize| cs[a..b].iter().collect::<String>();
        e.scroll = e.scroll.min(cs.len()).min(e.cursor);
        while e.scroll < e.cursor && t.width_fmt(&sub(e.scroll, e.cursor), false) > tw {
            e.scroll += 1;
        }
        let mut end = e.scroll;
        while end < cs.len() && t.width_fmt(&sub(e.scroll, end + 1), false) <= tw {
            end += 1;
        }
        let (a, b) = e.selection();
        let (a, b) = (a.clamp(e.scroll, end), b.clamp(e.scroll, end));
        if a < b {
            let x1 = tx + t.width_fmt(&sub(e.scroll, a), false);
            let x2 = tx + t.width_fmt(&sub(e.scroll, b), false);
            t.fill(x1, ty - 1.0, x2, ty + 9.0, 0xFF3A_5FD9);
        }
        t.draw(&sub(e.scroll, end), tx, ty, 0xFFFF_FFFF, true);
        if book.search_focus && caret {
            let c = e.cursor.clamp(e.scroll, end);
            let cx = tx + t.width_fmt(&sub(e.scroll, c), false);
            if c == cs.len() {
                t.draw("_", cx, ty, 0xFFFF_FFFF, true);
            } else {
                t.fill(cx, ty - 1.0, cx + 1.0, ty + 9.0, 0xFFD0_D0D0);
            }
        }
        return;
    }
    if book.search_focus && caret {
        t.draw("_", tx, ty, 0xFFFF_FFFF, true);
    }
}

/// Кнопка оверлея: позиция, рецепт, доступность, мини-иконки (смещение, варианты предметов).
pub struct OvButton {
    pub x: f32,
    pub y: f32,
    pub id: i32,
    pub craftable: bool,
    pub items: Vec<(f32, f32, Vec<u32>)>,
}

/// VANILLA: OverlayRecipeComponent (крафт; для печей — отдельный срез).
pub struct Overlay {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub furnace: bool,
    pub buttons: Vec<OvButton>,
}

impl Overlay {
    /// VANILLA: OverlayRecipeComponent#init.
    fn new(c: &Collection, filtering: bool, bx: f32, by: f32, g: &Geo, kind: BookKind) -> Self {
        let mut list: Vec<(&RecipeEntry, bool)> = Vec::new();
        for (r, ok) in c.recipes.iter().zip(&c.craftable) {
            if *ok {
                list.push((r, true));
            }
        }
        if !filtering {
            for (r, ok) in c.recipes.iter().zip(&c.craftable) {
                if !*ok {
                    list.push((r, false));
                }
            }
        }
        let total = list.len();
        let max_row = if total <= 16 { 4 } else { 5 };
        let rows = total.div_ceil(max_row);
        let (mut x, mut y) = (bx, by);
        let right = x + (total.min(max_row) * 25) as f32;
        let max_left = g.xo + 147.0 / 2.0 + 50.0;
        if right > max_left {
            x -= 25.0 * ((right - max_left) / 25.0).trunc();
        }
        let (cy, bottom) = (g.yo + 13.0 + 166.0 / 2.0, y + (rows * 25) as f32);
        if bottom > cy + 50.0 {
            y -= 25.0 * ((bottom - (cy + 50.0)) / 25.0).ceil();
        }
        if y < cy - 100.0 {
            y -= 25.0 * ((y - (cy - 100.0)) / 25.0).ceil();
        }
        let buttons = list
            .iter()
            .enumerate()
            .map(|(i, (r, ok))| {
                let mut items = Vec::new();
                let mut add = |gx: usize, gy: usize, sd: &crate::recipebook::Sd| {
                    if !sd.alts.is_empty() {
                        items.push((
                            3.0 + gx as f32 * 7.0,
                            3.0 + gy as f32 * 7.0,
                            sd.alts.clone(),
                        ));
                    }
                };
                match r.kind {
                    Kind::Shaped { w, h } => {
                        for (gi, ing) in crate::recipebook::place_recipe(
                            3,
                            3,
                            w as usize,
                            h as usize,
                            r.ingredients.len(),
                        ) {
                            add(gi % 3, gi / 3, &r.ingredients[ing]);
                        }
                    }
                    Kind::Shapeless => {
                        for (i, sd) in r.ingredients.iter().enumerate() {
                            add(i % 3, i / 3, sd);
                        }
                    }
                    // VANILLA: OverlaySmeltingRecipeButton — вход в клетке (1, 1).
                    Kind::Furnace { .. } => add(1, 1, &r.ingredients[0]),
                    _ => {}
                }
                OvButton {
                    x: x + 4.0 + 25.0 * (i % max_row) as f32,
                    y: y + 5.0 + 25.0 * (i / max_row) as f32,
                    id: r.id,
                    craftable: *ok,
                    items,
                }
            })
            .collect();
        Self {
            x,
            y,
            w: (total.min(max_row) * 25 + 8) as f32,
            h: (rows * 25 + 8) as f32,
            furnace: kind != BookKind::Crafting,
            buttons,
        }
    }
}

/// Рисует оверлей поверх книги; мини-иконки 6×6 (масштаб 0.375), вариант меняется раз в 1,5 с.
#[allow(clippy::too_many_arguments)]
pub fn draw_overlay(
    p: &mut ezzmc_render::gui::GuiPainter,
    book: &RecipeBook,
    mx: f32,
    my: f32,
    now_ms: u128,
    icon: &mut dyn FnMut(
        &mut ezzmc_render::gui::GuiPainter,
        &ezzmc_protocol::item::ItemStack,
        f32,
        f32,
        f32,
    ),
) {
    let Some(o) = &book.overlay else {
        return;
    };
    let uv = [0.0, 0.0, 1.0, 1.0];
    p.nine_slice(
        "rb_ov_bg",
        o.x,
        o.y,
        o.w,
        o.h,
        [32.0, 32.0],
        [4.0; 4],
        0xFFFF_FFFF,
    );
    let idx = (now_ms / 1500) as usize;
    for b in &o.buttons {
        let hot = inside(mx, my, b.x, b.y, 24.0, 24.0);
        let sp = match (b.craftable, hot, o.furnace) {
            (true, false, false) => "rb_ov",
            (true, true, false) => "rb_ov_hl",
            (false, false, false) => "rb_ov_x",
            (false, true, false) => "rb_ov_x_hl",
            (true, false, true) => "rb_fov",
            (true, true, true) => "rb_fov_hl",
            (false, false, true) => "rb_fov_x",
            (false, true, true) => "rb_fov_x_hl",
        };
        p.blit(sp, b.x, b.y, 24.0, 24.0, uv, 0xFFFF_FFFF);
        for (ix, iy, alts) in &b.items {
            let st = ezzmc_protocol::item::ItemStack {
                item: alts[idx % alts.len()],
                count: 1,
                patch: Default::default(),
            };
            // translate(x+2+ix, y+2+iy) · scale 0.375 · translate(-8,-8) → левый верх на −3
            icon(p, &st, b.x + 2.0 + ix - 3.0, b.y + 2.0 + iy - 3.0, 6.0);
        }
    }
}
