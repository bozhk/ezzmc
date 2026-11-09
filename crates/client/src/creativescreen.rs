//! Креатив-экран: вкладки предметов, поиск, вкладка «Survival Inventory», клики по слотам.
//! OP-вкладка: только при options.operatorItemsTab + креатив + права ≥2 (entity event 24..28).
//! VANILLA: CreativeModeInventoryScreen / ItemPickerMenu / SlotWrapper (26.3).
use crate::charpreview::CharPreview;
use crate::creative_data::{self as cd, Entry};
use crate::hotbars::Hotbars;
use crate::hud;
use crate::inventory::{Inventory, HOTBAR_START, PLAYER_SLOTS};
use crate::invscreen::{PICKUP, PICKUP_ALL, QUICK_MOVE, SWAP, THROW};
use crate::itemdata::max_stack;
use crate::netthread::NetCmd;
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::{ComponentPatch, ItemStack};
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

/// VANILLA: ContainerInput.CLONE.
pub const CLONE: i32 = 3;

const W: f32 = 195.0;
const H: f32 = 136.0;
const COLS: usize = 9;
const ROWS: usize = 5;
const SCROLL_Y: f32 = 18.0;
const SCROLL_X: f32 = 175.0;
const TRACK_H: f32 = 112.0;
/// Слот «Destroy Item» (координаты предмета, как у Mojang).
const TRASH: (f32, f32) = (173.0, 112.0);
/// Окно превью персонажа на вкладке инвентаря и размер модели.
pub const PREVIEW_RECT: (f32, f32, f32, f32) = (73.0, 6.0, 105.0, 49.0);
pub const PREVIEW_SIZE: f32 = 20.0;
const MAX_SEARCH: usize = 50;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Category,
    Search,
    Inventory,
    /// «Saved Hotbars»: 9 наборов по 9 слотов.
    Hotbar,
}

struct Tab {
    top: bool,
    /// Номер спрайта вкладки (1-based = колонка + 1).
    col: u8,
    right: bool,
    icon: &'static str,
    key: &'static str,
    data: &'static str,
    kind: Kind,
}

const fn cat(top: bool, col: u8, icon: &'static str, key: &'static str, data: &'static str) -> Tab {
    Tab {
        top,
        col,
        right: false,
        icon,
        key,
        data,
        kind: Kind::Category,
    }
}

pub const SEARCH_TAB: usize = 10;
#[allow(dead_code)]
pub const INVENTORY_TAB: usize = 11;
/// «Saved Hotbars» (VANILLA: CreativeModeTabs.HOTBAR — верхний ряд, колонка 5, справа).
#[allow(dead_code)]
pub const HOTBAR_TAB: usize = 12;
/// VANILLA: CreativeModeTabs.OP_BLOCKS — видна при `Options#operatorItemsTab` и canUseGameMasterBlocks.
pub const OP_TAB: usize = 13;

static TABS: &[Tab] = &[
    cat(
        true,
        1,
        "bricks",
        "itemGroup.buildingBlocks",
        "building_blocks",
    ),
    cat(
        true,
        2,
        "cyan_wool",
        "itemGroup.coloredBlocks",
        "colored_blocks",
    ),
    cat(
        true,
        3,
        "grass_block",
        "itemGroup.natural",
        "natural_blocks",
    ),
    cat(
        true,
        4,
        "oak_sign",
        "itemGroup.functional",
        "functional_blocks",
    ),
    cat(true, 5, "redstone", "itemGroup.redstone", "redstone_blocks"),
    cat(
        false,
        1,
        "diamond_pickaxe",
        "itemGroup.tools",
        "tools_and_utilities",
    ),
    cat(false, 2, "netherite_sword", "itemGroup.combat", "combat"),
    cat(
        false,
        3,
        "golden_apple",
        "itemGroup.foodAndDrink",
        "food_and_drinks",
    ),
    cat(
        false,
        4,
        "iron_ingot",
        "itemGroup.ingredients",
        "ingredients",
    ),
    cat(
        false,
        5,
        "creeper_spawn_egg",
        "itemGroup.spawnEggs",
        "spawn_eggs",
    ),
    Tab {
        top: true,
        col: 7,
        right: true,
        icon: "compass",
        key: "itemGroup.search",
        data: "",
        kind: Kind::Search,
    },
    Tab {
        top: false,
        col: 7,
        right: true,
        icon: "chest",
        key: "itemGroup.inventory",
        data: "",
        kind: Kind::Inventory,
    },
    Tab {
        top: true,
        col: 6,
        right: true,
        icon: "bookshelf",
        key: "itemGroup.hotbar",
        data: "",
        kind: Kind::Hotbar,
    },
    Tab {
        top: false,
        col: 6,
        right: true,
        icon: "command_block",
        key: "itemGroup.op",
        data: "op_blocks",
        kind: Kind::Category,
    },
];

/// Слот экрана.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Slot {
    /// Ячейка сетки предметов (0..45, позиция на экране).
    Cell(usize),
    /// Слот InventoryMenu (индекс меню 1..46).
    Inv(usize),
    /// «Destroy Item».
    Trash,
}

/// Ячейка вкладки «Saved Hotbars».
#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Stack(ItemStack),
    /// Бумага-подсказка пустого набора №n (CREATIVE_SLOT_LOCK — взять нельзя).
    Hint(usize),
}

#[derive(Default)]
pub struct State {
    pub tab: usize,
    pub row: usize,
    pub search: String,
    /// Курсор и конец выделения в поле поиска (индексы символов).
    pub cursor: usize,
    pub sel: usize,
    /// Полоса прокрутки захвачена мышью.
    pub scrolling: bool,
    /// Все предметы вкладок (для поиска), без повторов.
    all: Vec<&'static Entry>,
    /// Текущий список поиска.
    list: Vec<&'static Entry>,
    /// Видимые теги при поиске `#…` (для строк тултипа).
    visible_tags: Vec<&'static str>,
    /// Содержимое вкладки «Saved Hotbars» (9×9 = 81).
    hot: Vec<Cell>,
    pub hotbars: Hotbars,
    /// Показывать вкладку операторских предметов.
    pub op_tab: bool,
}

/// Теги предметов 26.3 (плоские): (тег, предметы). VANILLA: data/minecraft/tags/item.
fn item_tags() -> &'static [(&'static str, Vec<&'static str>)] {
    static T: std::sync::OnceLock<Vec<(&'static str, Vec<&'static str>)>> =
        std::sync::OnceLock::new();
    T.get_or_init(|| {
        include_str!("../data/item_tags.txt")
            .lines()
            .filter_map(|l| {
                let mut it = l.split(' ');
                Some((it.next()?, it.collect()))
            })
            .collect()
    })
}

/// Список предметов текущей вкладки.
enum Items<'a> {
    Tab(&'static [Entry]),
    List(&'a [&'static Entry]),
}

impl Items<'_> {
    fn len(&self) -> usize {
        match self {
            Items::Tab(t) => t.len(),
            Items::List(l) => l.len(),
        }
    }

    fn get(&self, i: usize) -> Option<&'static Entry> {
        match self {
            Items::Tab(t) => t.get(i),
            Items::List(l) => l.get(i).copied(),
        }
    }
}

impl State {
    fn kind(&self) -> Kind {
        TABS[self.tab].kind
    }

    pub fn is_search(&self) -> bool {
        self.kind() == Kind::Search
    }

    pub fn is_inventory(&self) -> bool {
        self.kind() == Kind::Inventory
    }

    fn items(&self) -> Items<'_> {
        match self.kind() {
            Kind::Category => Items::Tab(cd::tab(TABS[self.tab].data)),
            Kind::Search => Items::List(&self.list),
            Kind::Inventory | Kind::Hotbar => Items::Tab(&[]),
        }
    }

    fn can_scroll(&self) -> bool {
        self.kind() != Kind::Inventory
    }

    fn count(&self) -> usize {
        if self.kind() == Kind::Hotbar {
            self.hot.len()
        } else {
            self.items().len()
        }
    }

    fn max_row(&self) -> usize {
        self.count().div_ceil(COLS).saturating_sub(ROWS)
    }

    /// Содержимое ячейки сетки по общему индексу (с учётом прокрутки).
    fn cell(&self, idx: usize) -> Cell {
        if self.kind() == Kind::Hotbar {
            return self.hot.get(idx).cloned().unwrap_or(Cell::Empty);
        }
        let Some(e) = self.items().get(idx) else {
            return Cell::Empty;
        };
        let Some(mut s) = stack_of(e.name, 1) else {
            return Cell::Empty;
        };
        s.patch = e.patch.clone();
        Cell::Stack(s)
    }

    /// VANILLA: CreativeModeInventoryScreen#selectTab (ветка HOTBAR).
    fn fill_hotbar_tab(&mut self) {
        self.hot.clear();
        for g in 0..crate::hotbars::GROUPS {
            if self.hotbars.is_empty(g) {
                for i in 0..9 {
                    self.hot
                        .push(if i == g { Cell::Hint(g) } else { Cell::Empty });
                }
            } else {
                let items: Vec<_> = self.hotbars.get(g).to_vec();
                self.hot.extend(
                    items
                        .into_iter()
                        .map(|s| s.map_or(Cell::Empty, Cell::Stack)),
                );
            }
        }
    }

    pub fn scroll(&mut self, dy: f64) {
        if !self.can_scroll() {
            return;
        }
        let r = self.row as i64 - dy.signum() as i64;
        self.row = r.clamp(0, self.max_row() as i64) as usize;
    }

    /// Перетаскивание бегунка. VANILLA: mouseDragged — (my - yscr - 7.5) / (112 - 15).
    pub fn drag_scroll(&mut self, gh: f32, my: f32) {
        let top = ((gh - H) / 2.0).floor();
        let f = ((my - top - SCROLL_Y - 7.5) / (TRACK_H - 15.0)).clamp(0.0, 1.0);
        self.row = (f * self.max_row() as f32 + 0.5) as usize;
    }

    /// VANILLA: selectTab — смена вкладки сбрасывает прокрутку и текст поиска.
    /// VANILLA: CreativeModeInventoryScreen#canUseGameMasterBlocks && displayOperatorCreativeTab; пропала вкладка → первая.
    pub fn set_op_tab(&mut self, on: bool, lang: &Lang) {
        self.op_tab = on;
        if !on && self.tab == OP_TAB {
            self.select_tab(0, lang);
        }
    }

    pub fn select_tab(&mut self, i: usize, lang: &Lang) {
        let old = self.tab;
        self.tab = i;
        self.row = 0;
        self.scrolling = false;
        if self.kind() == Kind::Hotbar {
            self.fill_hotbar_tab();
        }
        if self.is_search() {
            if self.all.is_empty() {
                self.all = cd::tab("search").iter().collect();
            }
            if old != i {
                {
                    self.search.clear();
                    self.cursor = 0;
                    self.sel = 0;
                }
            }
            self.refresh(lang);
        } else {
            {
                self.search.clear();
                self.cursor = 0;
                self.sel = 0;
            }
        }
    }

    /// VANILLA: refreshSearchResults (поиск по подстроке имени и id).
    /// VANILLA-UNSURE: порядок результатов — порядок вкладок.
    pub fn refresh(&mut self, lang: &Lang) {
        self.row = 0;
        self.visible_tags.clear();
        let term = self.search.to_lowercase();
        if term.is_empty() {
            self.list = self.all.clone();
        } else if let Some(t) = term.strip_prefix('#') {
            // VANILLA: updateVisibleTags — `путь` или `пространство:путь` (подстроки).
            let (ns, path) = t
                .split_once(':')
                .map_or((None, t), |(a, b)| (Some(a.trim()), b.trim()));
            self.visible_tags = item_tags()
                .iter()
                .filter(|(tag, _)| ns.is_none_or(|n| "minecraft".contains(n)) && tag.contains(path))
                .map(|(tag, _)| *tag)
                .collect();
            let tags = &self.visible_tags;
            self.list = self
                .all
                .iter()
                .copied()
                .filter(|e| {
                    item_tags()
                        .iter()
                        .any(|(tag, items)| tags.contains(tag) && items.contains(&e.name))
                })
                .collect();
        } else {
            let by_id = term.contains(':');
            self.list = self
                .all
                .iter()
                .copied()
                .filter(|e| {
                    let n = e.name;
                    if by_id {
                        format!("minecraft:{n}").contains(&term)
                    } else {
                        n.contains(&term)
                            || crate::tooltip::item_title(lang, n)
                                .to_lowercase()
                                .contains(&term)
                    }
                })
                .collect();
        }
    }

    fn byte(&self, i: usize) -> usize {
        self.search
            .char_indices()
            .nth(i)
            .map_or(self.search.len(), |(b, _)| b)
    }

    fn len(&self) -> usize {
        self.search.chars().count()
    }

    /// Диапазон выделения (по символам).
    pub fn selection(&self) -> (usize, usize) {
        (self.cursor.min(self.sel), self.cursor.max(self.sel))
    }

    pub fn selected_text(&self) -> String {
        let (a, b) = self.selection();
        self.search[self.byte(a)..self.byte(b)].to_owned()
    }

    fn delete_selection(&mut self) -> bool {
        let (a, b) = self.selection();
        if a == b {
            return false;
        }
        let (ba, bb) = (self.byte(a), self.byte(b));
        self.search.replace_range(ba..bb, "");
        self.cursor = a;
        self.sel = a;
        true
    }

    /// VANILLA: TextFieldHelper#insertText — заменяет выделение, лимит длины.
    pub fn type_text(&mut self, text: &str, lang: &Lang) {
        let mut changed = self.delete_selection();
        for ch in text.chars().filter(|c| !c.is_control()) {
            if self.len() < MAX_SEARCH {
                let b = self.byte(self.cursor);
                self.search.insert(b, ch);
                self.cursor += 1;
                changed = true;
            }
        }
        self.sel = self.cursor;
        if changed {
            self.refresh(lang);
        }
    }

    /// VANILLA: TextFieldHelper#getWordPosition — граница слова от курсора.
    fn word_pos(&self, dir: i32) -> usize {
        let cs: Vec<char> = self.search.chars().collect();
        let mut i = self.cursor as i32;
        let n = cs.len() as i32;
        if dir < 0 {
            while i > 0 && cs[i as usize - 1] == ' ' {
                i -= 1;
            }
            while i > 0 && cs[i as usize - 1] != ' ' {
                i -= 1;
            }
        } else {
            while i < n && cs[i as usize] != ' ' {
                i += 1;
            }
            while i < n && cs[i as usize] == ' ' {
                i += 1;
            }
        }
        i as usize
    }

    /// Backspace/Delete (`dir` = -1/+1); `word` — Ctrl.
    pub fn erase(&mut self, dir: i32, word: bool, lang: &Lang) {
        let mut changed = self.delete_selection();
        if !changed {
            let target = if word {
                self.word_pos(dir)
            } else {
                (self.cursor as i32 + dir).clamp(0, self.len() as i32) as usize
            };
            self.sel = target;
            changed = self.delete_selection();
        }
        if changed {
            self.refresh(lang);
        }
    }

    /// Стрелки/Home/End; `shift` — расширять выделение.
    pub fn move_cursor(&mut self, to: Move, word: bool, shift: bool) {
        let (a, b) = self.selection();
        let pos = match (to, word) {
            (Move::Left, _) if !shift && a != b => a,
            (Move::Right, _) if !shift && a != b => b,
            (Move::Left, true) => self.word_pos(-1),
            (Move::Right, true) => self.word_pos(1),
            (Move::Left, false) => self.cursor.saturating_sub(1),
            (Move::Right, false) => (self.cursor + 1).min(self.len()),
            (Move::Home, _) => 0,
            (Move::End, _) => self.len(),
        };
        self.cursor = pos;
        if !shift {
            self.sel = pos;
        }
    }

    pub fn select_all(&mut self) {
        self.sel = 0;
        self.cursor = self.len();
    }

    /// Ctrl+X: вернуть вырезанный текст.
    pub fn cut(&mut self, lang: &Lang) -> String {
        let t = self.selected_text();
        if self.delete_selection() {
            self.refresh(lang);
        }
        t
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Home,
    End,
}

pub enum Hit {
    Tab(usize),
    Slot(Slot),
    Scroll(f32),
    Outside,
    None,
}

pub fn origin(gw: f32, gh: f32) -> (f32, f32) {
    (((gw - W) / 2.0).floor(), ((gh - H) / 2.0).floor())
}

fn tab_x(tb: &Tab) -> f32 {
    let pos = f32::from(tb.col - 1);
    if tb.right {
        W - 27.0 * (7.0 - pos) + 1.0
    } else {
        27.0 * pos
    }
}

/// Спрайт вкладки рисуется на 4 px внутрь окна; область клика — 32 px.
fn tab_y(top: bool) -> f32 {
    if top {
        -28.0
    } else {
        H - 4.0
    }
}

/// Слоты игрока на экране: (индекс меню, x, y) в координатах предмета.
/// VANILLA: SlotWrapper (вкладка инвентаря) / addInventoryHotbarSlots(9, 112).
fn player_slots(st: &State) -> Vec<(usize, f32, f32)> {
    if !st.is_inventory() {
        return (0..9)
            .map(|c| (HOTBAR_START + c, 9.0 + c as f32 * 18.0, 112.0))
            .collect();
    }
    let mut v = Vec::new();
    for i in 5..PLAYER_SLOTS {
        let (x, y) = if i < 9 {
            let pos = (i - 5) as f32;
            (54.0 + (pos / 2.0).floor() * 54.0, 6.0 + (pos % 2.0) * 27.0)
        } else if i == 45 {
            (35.0, 20.0)
        } else {
            let pos = i - 9;
            let x = 9.0 + (pos % 9) as f32 * 18.0;
            let y = if i >= 36 {
                112.0
            } else {
                54.0 + (pos / 9) as f32 * 18.0
            };
            (x, y)
        };
        v.push((i, x, y));
    }
    v
}

fn over(x: f32, y: f32, sx: f32, sy: f32) -> bool {
    // VANILLA: AbstractContainerScreen#isHovering — 16×16 с полем в 1 px.
    x >= sx - 1.0 && x < sx + 17.0 && y >= sy - 1.0 && y < sy + 17.0
}

pub fn hit(st: &State, gw: f32, gh: f32, mx: f32, my: f32) -> Hit {
    let (l, t) = origin(gw, gh);
    let (x, y) = (mx - l, my - t);
    for (i, tb) in TABS.iter().enumerate() {
        if i == OP_TAB && !st.op_tab {
            continue;
        }
        let (tx, ty) = (tab_x(tb), if tb.top { -32.0 } else { H });
        if x >= tx && x <= tx + 26.0 && y >= ty && y <= ty + 32.0 {
            return Hit::Tab(i);
        }
    }
    if !st.is_inventory() {
        for c in 0..COLS * ROWS {
            if over(
                x,
                y,
                9.0 + (c % COLS) as f32 * 18.0,
                18.0 + (c / COLS) as f32 * 18.0,
            ) {
                return Hit::Slot(Slot::Cell(c));
            }
        }
    }
    for (i, sx, sy) in player_slots(st) {
        if over(x, y, sx, sy) {
            return Hit::Slot(Slot::Inv(i));
        }
    }
    if st.is_inventory() && over(x, y, TRASH.0, TRASH.1) {
        return Hit::Slot(Slot::Trash);
    }
    if st.can_scroll()
        && (SCROLL_X..SCROLL_X + 14.0).contains(&x)
        && (SCROLL_Y..SCROLL_Y + TRACK_H).contains(&y)
    {
        return Hit::Scroll(((y - SCROLL_Y - 7.5) / (TRACK_H - 15.0)).clamp(0.0, 1.0));
    }
    if x < 0.0 || y < 0.0 || x >= W || y >= H {
        return Hit::Outside;
    }
    Hit::None
}

fn stack_of(name: &str, count: i32) -> Option<ItemStack> {
    let item = ezzmc_protocol::generated::r26_3::ITEM
        .iter()
        .position(|n| *n == name)? as u32;
    Some(ItemStack {
        item,
        count,
        patch: ComponentPatch::default(),
    })
}

fn max_of(s: &ItemStack) -> i32 {
    ezzmc_protocol::generated::r26_3::ITEM
        .get(s.item as usize)
        .map_or(64, |n| max_stack(n))
}

fn with_count(s: &ItemStack, count: i32) -> ItemStack {
    ItemStack { count, ..s.clone() }
}

/// VANILLA: ItemStack#isSameItemSameComponents.
fn same(a: &ItemStack, b: &ItemStack) -> bool {
    a.item == b.item && a.patch == b.patch
}

/// Стек ячейки сетки; подсказка-бумага — не стек (взять нельзя).
fn cell_stack(st: &State, cell: usize) -> Option<ItemStack> {
    match st.cell(st.row * COLS + cell) {
        Cell::Stack(s) => Some(s),
        _ => None,
    }
}

fn set_slot(inv: &mut Inventory, out: &mut Vec<NetCmd>, i: usize, s: Option<ItemStack>) {
    inv.slots[i] = s.clone();
    out.push(NetCmd::CreativeSlot(i as i16, s));
}

fn drop_stack(out: &mut Vec<NetCmd>, s: ItemStack) {
    // VANILLA: MultiPlayerGameMode#handleCreativeModeItemDrop — слот -1.
    out.push(NetCmd::CreativeSlot(-1, Some(s)));
}

/// Разослать слоты, изменившиеся с `before` (VANILLA: broadcastChanges → handleCreativeModeItemAdd).
fn send_changes(inv: &Inventory, before: &[Option<ItemStack>], out: &mut Vec<NetCmd>) {
    for (i, (now, was)) in inv
        .slots
        .iter()
        .zip(before)
        .enumerate()
        .take(PLAYER_SLOTS)
        .skip(1)
    {
        if now != was {
            out.push(NetCmd::CreativeSlot(i as i16, now.clone()));
        }
    }
}

/// Куда нажатием цифры/F переносится предмет: индекс меню слота-цели.
fn swap_target(button: i8) -> Option<usize> {
    match button {
        0..=8 => Some(HOTBAR_START + button as usize),
        40 => Some(45),
        _ => None,
    }
}

/// Клик; меняет локальный инвентарь/курсор и возвращает команды сети.
/// `button`: 0 ЛКМ, 1 ПКМ, 2 СКМ; для SWAP — номер слота хотбара (40 — вторая рука);
/// для THROW — 1, если зажат Ctrl.
/// VANILLA: CreativeModeInventoryScreen#slotClicked.
pub fn click(
    st: &mut State,
    inv: &mut Inventory,
    h: &Hit,
    button: i8,
    mode: i32,
    lang: &Lang,
) -> Vec<NetCmd> {
    // VANILLA: slotClicked — бросок (и клик вне окна = THROW) пропускается, пока
    // LocalPlayer#canDropItems() ложно (защита от спама бросками).
    if (mode == THROW || (mode == PICKUP && matches!(h, Hit::Outside))) && !inv.can_drop() {
        return Vec::new();
    }
    let mut out = Vec::new();
    match h {
        Hit::Tab(i) => {
            if button == 0 && mode == PICKUP {
                st.select_tab(*i, lang);
            }
        }
        Hit::Scroll(f) => {
            if button == 0 && mode == PICKUP {
                st.scrolling = true;
                st.row = (f * st.max_row() as f32 + 0.5) as usize;
            }
        }
        Hit::Outside => outside_click(st, inv, button, &mut out),
        Hit::Slot(Slot::Cell(c)) => cell_click(st, inv, *c, button, mode, &mut out),
        Hit::Slot(Slot::Inv(i)) => inv_click(st, inv, *i, button, mode, &mut out),
        Hit::Slot(Slot::Trash) => {
            if mode == QUICK_MOVE {
                // VANILLA: shift по «Destroy Item» — очистка всего InventoryMenu.
                for i in 0..PLAYER_SLOTS {
                    inv.slots[i] = None;
                    if i >= 1 {
                        out.push(NetCmd::CreativeSlot(i as i16, None));
                    }
                }
            } else if mode == PICKUP {
                inv.cursor = None;
            }
        }
        Hit::None => {}
    }
    // VANILLA: MultiPlayerGameMode#handleCreativeModeItemDrop — throttler.increment() за бросок.
    for c in &out {
        if matches!(c, NetCmd::CreativeSlot(-1, Some(_))) {
            inv.drop_spam += 20;
        }
    }
    out
}

/// Клик вне окна с предметом на курсоре — выбросить.
fn outside_click(st: &State, inv: &mut Inventory, button: i8, out: &mut Vec<NetCmd>) {
    let Some(c) = inv.cursor.clone() else { return };
    if st.is_inventory() || button == 0 {
        // Вкладка инвентаря выбрасывает весь стек при любой кнопке (THROW -999).
        inv.cursor = None;
        drop_stack(out, c);
    } else if button == 1 {
        drop_stack(out, with_count(&c, 1));
        if c.count <= 1 {
            inv.cursor = None;
        } else if let Some(x) = inv.cursor.as_mut() {
            x.count -= 1;
        }
    }
}

/// Ячейка сетки предметов. VANILLA: slotClicked, ветка `slot.container == CONTAINER`.
fn cell_click(
    st: &State,
    inv: &mut Inventory,
    cell: usize,
    button: i8,
    mode: i32,
    out: &mut Vec<NetCmd>,
) {
    // VANILLA: CustomCreativeSlot#mayPickup — бумага-подсказка (CREATIVE_SLOT_LOCK) неактивна.
    if matches!(st.cell(st.row * COLS + cell), Cell::Hint(_)) {
        return;
    }
    let clicked = cell_stack(st, cell);
    match mode {
        SWAP => {
            if let (Some(cl), Some(t)) = (clicked, swap_target(button)) {
                let max = max_of(&cl);
                set_slot(inv, out, t, Some(with_count(&cl, max)));
            }
        }
        CLONE => {
            if let (None, Some(cl)) = (&inv.cursor, clicked) {
                let max = max_of(&cl);
                inv.cursor = Some(with_count(&cl, max));
            }
        }
        THROW => {
            if let Some(cl) = clicked {
                let n = if button == 0 { 1 } else { max_of(&cl) };
                drop_stack(out, with_count(&cl, n));
            }
        }
        _ => {
            let quick = mode == QUICK_MOVE;
            match (inv.cursor.as_mut(), clicked) {
                (Some(c), Some(cl)) if same(c, &cl) => {
                    let max = max_of(c);
                    if button == 0 {
                        if quick {
                            c.count = max;
                        } else if c.count < max {
                            c.count += 1;
                        }
                    } else {
                        c.count -= 1;
                        if c.count <= 0 {
                            inv.cursor = None;
                        }
                    }
                }
                (None, Some(cl)) => {
                    let n = if quick { max_of(&cl) } else { cl.count };
                    inv.cursor = Some(with_count(&cl, n));
                }
                (Some(_), _) if button == 0 => inv.cursor = None,
                (Some(c), _) => {
                    c.count -= 1;
                    if c.count <= 0 {
                        inv.cursor = None;
                    }
                }
                (None, None) => {}
            }
        }
    }
}

/// Слот игрока (хотбар / InventoryMenu). VANILLA: SlotWrapper → inventoryMenu#clicked.
fn inv_click(
    st: &State,
    inv: &mut Inventory,
    i: usize,
    button: i8,
    mode: i32,
    out: &mut Vec<NetCmd>,
) {
    let before = inv.slots.clone();
    match mode {
        QUICK_MOVE => {
            if st.is_inventory() {
                quick_move(inv, i);
            } else if inv.slots[i].is_some() {
                // VANILLA: ItemPickerMenu#quickMoveStack — shift по хотбару удаляет предмет.
                inv.slots[i] = None;
            }
        }
        SWAP => {
            if let Some(t) = swap_target(button) {
                if t != i {
                    inv.slots.swap(i, t);
                }
            }
        }
        CLONE => {
            if let (None, Some(s)) = (&inv.cursor, inv.slots[i].clone()) {
                let max = max_of(&s);
                inv.cursor = Some(with_count(&s, max));
            }
        }
        THROW => {
            if let Some(s) = inv.slots[i].clone() {
                let n = if button == 0 {
                    1
                } else {
                    max_of(&s).min(s.count)
                };
                drop_stack(out, with_count(&s, n));
                inv.slots[i] = (s.count > n).then(|| with_count(&s, s.count - n));
            }
        }
        PICKUP | PICKUP_ALL => {
            if mode == PICKUP_ALL && !st.is_inventory() {
                pickup_all_hotbar(inv, i, button);
            } else {
                inv.predict_click(i as i16, button, mode);
            }
        }
        _ => {}
    }
    send_changes(inv, &before, out);
}

/// Двойной клик по хотбару на вкладке каталога: в ItemPickerMenu только слоты хотбара
/// (`canTakeItemForPickAll` отсекает сетку). VANILLA: AbstractContainerMenu#doClick (PICKUP_ALL).
fn pickup_all_hotbar(inv: &mut Inventory, clicked: usize, button: i8) {
    let Some(mut c) = inv.cursor.take() else {
        return;
    };
    if inv.slots[clicked].is_some() {
        inv.cursor = Some(c);
        return;
    }
    let max = max_of(&c);
    let order: Vec<usize> = if button == 0 {
        (HOTBAR_START..HOTBAR_START + 9).collect()
    } else {
        (HOTBAR_START..HOTBAR_START + 9).rev().collect()
    };
    for pass in 0..2 {
        for &j in &order {
            if c.count >= max {
                break;
            }
            let Some(t) = inv.slots[j].as_mut() else {
                continue;
            };
            if !same(t, &c) || (pass == 0 && t.count == max) {
                continue;
            }
            let take = t.count.min(max - c.count);
            t.count -= take;
            c.count += take;
            if t.count <= 0 {
                inv.slots[j] = None;
            }
        }
    }
    inv.cursor = Some(c);
}

/// VANILLA: InventoryMenu#quickMoveStack (без экипировки брони/второй руки: нет данных о слотах).
fn quick_move(inv: &mut Inventory, i: usize) {
    let Some(mut s) = inv.slots[i].take() else {
        return;
    };
    // VANILLA: InventoryMenu#quickMoveStack — броня/щит в пустой слот экипировки (по имени предмета).
    if (9..45).contains(&i) {
        let name = ezzmc_protocol::generated::r26_3::ITEM
            .get(s.item as usize)
            .copied()
            .unwrap_or("");
        let eq = if name.ends_with("_helmet") || name.ends_with("_head") || name == "carved_pumpkin"
        {
            Some(5)
        } else if name.ends_with("_chestplate") || name == "elytra" {
            Some(6)
        } else if name.ends_with("_leggings") {
            Some(7)
        } else if name.ends_with("_boots") {
            Some(8)
        } else if name == "shield" || name == "totem_of_undying" {
            Some(45)
        } else {
            None
        };
        if let Some(t) = eq.filter(|&t| inv.slots[t].is_none()) {
            inv.slots[t] = Some(with_count(&s, 1));
            s.count -= 1;
            inv.slots[i] = (s.count > 0).then_some(s);
            return;
        }
    }
    let (start, end) = match i {
        9..=35 => (HOTBAR_START, 45),
        36..=44 => (9, 36),
        _ => (9, 45),
    };
    // moveItemStackTo: сначала дополняем стеки, затем пустые слоты.
    let max = max_of(&s);
    for j in start..end {
        if s.count <= 0 {
            break;
        }
        if let Some(t) = inv.slots[j].as_mut() {
            if same(t, &s) && t.count < max {
                let mv = (max - t.count).min(s.count);
                t.count += mv;
                s.count -= mv;
            }
        }
    }
    for j in start..end {
        if s.count <= 0 {
            break;
        }
        if inv.slots[j].is_none() {
            let mv = max.min(s.count);
            inv.slots[j] = Some(with_count(&s, mv));
            s.count -= mv;
        }
    }
    inv.slots[i] = (s.count > 0).then_some(s);
}

/// Конец перетаскивания стека по слотам игрока. VANILLA: QUICK_CRAFT + handleCreativeModeItemAdd.
pub fn finish_drag(inv: &mut Inventory, slots: &[usize], ty: u8) -> Vec<NetCmd> {
    let mut out = Vec::new();
    let before = inv.slots.clone();
    inv.predict_quick_craft(slots, ty);
    send_changes(inv, &before, &mut out);
    out
}

/// Названия вкладок, где лежит предмет (синие строки тултипа).
/// VANILLA: CreativeModeInventoryScreen#getTooltipFromContainerItem — CreativeModeTab#contains.
fn tab_lines(lang: &Lang, s: &ItemStack) -> Vec<String> {
    let name = ezzmc_protocol::generated::r26_3::ITEM
        .get(s.item as usize)
        .copied()
        .unwrap_or("air");
    TABS.iter()
        .filter(|t| t.kind == Kind::Category)
        .filter(|t| {
            cd::tab(t.data)
                .iter()
                .any(|e| e.name == name && e.patch == s.patch)
        })
        .map(|t| lang.tr(t.key))
        .collect()
}

/// Предмет под курсором и его тултип; иначе подсказка вкладки/корзины.
pub fn hover_text(
    t: &TextRenderer,
    st: &State,
    inv: &Inventory,
    lang: &Lang,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) -> Option<Vec<(String, u32)>> {
    const BLUE: u32 = 0x5555FF;
    const PURPLE: u32 = 0xAA00AA;
    let with_tabs = |s: &ItemStack, creative_cell: bool| {
        let mut lines = crate::tooltip::stack_lines(lang, t, s);
        // VANILLA: в категории строки с вкладками у ячеек каталога не добавляются.
        if lines.is_empty() || (creative_cell && st.kind() == Kind::Category) {
            return lines;
        }
        if creative_cell && st.is_search() {
            let name = ezzmc_protocol::generated::r26_3::ITEM
                .get(s.item as usize)
                .copied()
                .unwrap_or("air");
            let mut at = 1;
            for tag in &st.visible_tags {
                if item_tags()
                    .iter()
                    .any(|(t, items)| t == tag && items.contains(&name))
                {
                    lines.insert(at.min(lines.len()), (format!("#minecraft:{tag}"), PURPLE));
                    at += 1;
                }
            }
        }
        let mut i = 1;
        for t in tab_lines(lang, s) {
            lines.insert(i.min(lines.len()), (t, BLUE));
            i += 1;
        }
        lines
    };
    match hit(st, gw, gh, mx, my) {
        Hit::Tab(i) => Some(vec![(lang.tr(TABS[i].key), 0xFFFFFF)]),
        Hit::Slot(Slot::Cell(c)) => match st.cell(st.row * COLS + c) {
            Cell::Stack(s) => Some(with_tabs(&s, true)),
            Cell::Hint(g) => Some(vec![(
                lang.tr("inventory.hotbarInfo")
                    .replacen("%1$s", "C", 1)
                    .replacen("%2$s", &(g + 1).to_string(), 1),
                0xFFFFFF,
            )]),
            Cell::Empty => None,
        },
        Hit::Slot(Slot::Inv(i)) => {
            let s = inv.slots[i].as_ref()?;
            Some(with_tabs(s, false))
        }
        Hit::Slot(Slot::Trash) => Some(vec![(lang.tr("inventory.binSlot"), 0xFFFFFF)]),
        _ => None,
    }
}

/// Предмет под курсором для проверки «ячейка не пуста» (клавиши цифр на вкладке поиска).
pub fn cell_has_item(st: &State, cell: usize) -> bool {
    st.cell(st.row * COLS + cell) != Cell::Empty
}

#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    st: &State,
    inv: &Inventory,
    scale: u32,
    preview: Option<&mut CharPreview>,
    drag: &[usize],
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) {
    let (l, t) = origin(gw, gh);
    // VANILLA: CreativeModeInventoryScreen#extractTabButton (невыбранная вкладка → рука) и
    // #extractBackground (полоса прокрутки: рука / стрелки ↕ при перетаскивании / «нельзя»).
    {
        use crate::cursor::{request, Kind};
        let (x, y) = (mx - l, my - t);
        for (i, tb) in TABS.iter().enumerate() {
            if (i == OP_TAB && !st.op_tab) || i == st.tab {
                continue;
            }
            let (tx, ty) = (tab_x(tb), tab_y(tb.top));
            if x > tx && y > ty && x < tx + 26.0 && y < ty + 32.0 {
                request(Kind::PointingHand);
            }
        }
        if (SCROLL_X..SCROLL_X + 14.0).contains(&x) && (SCROLL_Y..SCROLL_Y + TRACK_H).contains(&y) {
            request(if !st.can_scroll() {
                Kind::NotAllowed
            } else if st.scrolling {
                Kind::ResizeNs
            } else {
                Kind::PointingHand
            });
        }
    }
    let w = 0xFFFF_FFFF;
    let tab_sprite = |tb: &Tab, sel: bool| {
        format!(
            "ct_{}{}_{}",
            if tb.top { "top" } else { "bot" },
            if sel { "_sel" } else { "" },
            tb.col
        )
    };
    // VANILLA: невыбранные вкладки → фон → полоса → выбранная вкладка.
    let tab_btn = |p: &mut GuiPainter, i: usize, sel: bool| {
        let tb = &TABS[i];
        let (x, y) = (l + tab_x(tb), t + tab_y(tb.top));
        p.blit(
            &tab_sprite(tb, sel),
            x,
            y,
            26.0,
            32.0,
            [0.0, 0.0, 1.0, 1.0],
            w,
        );
        let iy = if tb.top { 9.0 } else { 7.0 };
        hud::name_icon(p, dev, q, rm, tb.icon, x + 5.0, y + iy);
    };
    for i in 0..TABS.len() {
        if i != st.tab && (i != OP_TAB || st.op_tab) {
            tab_btn(p, i, false);
        }
    }
    let k = 1.0 / 256.0;
    let bg = match st.kind() {
        Kind::Category => "ct_items",
        Kind::Search => "ct_search",
        Kind::Inventory => "ct_inventory",
        Kind::Hotbar => "ct_items",
    };
    p.blit(bg, l, t, W, H, [0.0, 0.0, W * k, H * k], w);
    if st.can_scroll() {
        let can = st.max_row() > 0;
        let frac = if can {
            st.row as f32 / st.max_row() as f32
        } else {
            0.0
        };
        p.blit(
            if can {
                "ct_scroller"
            } else {
                "ct_scroller_off"
            },
            l + SCROLL_X,
            t + SCROLL_Y + frac * (TRACK_H - 15.0),
            12.0,
            15.0,
            [0.0, 0.0, 1.0, 1.0],
            w,
        );
    }
    tab_btn(p, st.tab, true);
    if st.is_inventory() {
        if let Some(cp) = preview {
            cp.set_armor(&inv.slots);
            cp.draw(p, dev, q, scale, l, t, mx, my, PREVIEW_RECT, PREVIEW_SIZE);
        }
    } else if st.kind() == Kind::Hotbar {
        for c in 0..COLS * ROWS {
            let (x, y) = (
                l + 9.0 + (c % COLS) as f32 * 18.0,
                t + 18.0 + (c / COLS) as f32 * 18.0,
            );
            match st.cell(st.row * COLS + c) {
                Cell::Stack(s) => hud::stack_icon(p, dev, q, rm, &s, x, y),
                Cell::Hint(_) => hud::name_icon(p, dev, q, rm, "paper", x, y),
                Cell::Empty => {}
            }
        }
    } else {
        let items = st.items();
        for c in 0..COLS * ROWS {
            if let Some(e) = items.get(st.row * COLS + c) {
                let name = e.name;
                hud::patch_icon(
                    p,
                    dev,
                    q,
                    rm,
                    name,
                    &e.patch,
                    l + 9.0 + (c % COLS) as f32 * 18.0,
                    t + 18.0 + (c / COLS) as f32 * 18.0,
                );
            }
        }
    }
    for (i, x, y) in player_slots(st) {
        // VANILLA: AbstractContainerScreen#extractSlot — подсветка раздачи (0x80FFFFFF) под предметом.
        if drag.contains(&i) {
            p.blit(
                "white",
                l + x,
                t + y,
                16.0,
                16.0,
                [0.0, 0.0, 1.0, 1.0],
                0x80FF_FFFF,
            );
        }
        if let Some(s) = &inv.slots[i] {
            hud::stack_icon(p, dev, q, rm, s, l + x, t + y);
        } else {
            hud::empty_slot_icon(p, i, l + x, t + y);
        }
    }
    let cell = match hit(st, gw, gh, mx, my) {
        Hit::Slot(Slot::Cell(c)) => Some((
            9.0 + (c % COLS) as f32 * 18.0,
            18.0 + (c / COLS) as f32 * 18.0,
        )),
        Hit::Slot(Slot::Inv(i)) => player_slots(st)
            .into_iter()
            .find(|s| s.0 == i)
            .map(|s| (s.1, s.2)),
        Hit::Slot(Slot::Trash) => Some(TRASH),
        _ => None,
    };
    if let Some((x, y)) = cell {
        p.blit(
            "white",
            l + x,
            t + y,
            16.0,
            16.0,
            [0.0, 0.0, 1.0, 1.0],
            0x80FF_FFFF,
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_text(t: &mut TextRenderer, st: &State, inv: &Inventory, lang: &Lang, gw: f32, gh: f32) {
    let (l, top) = origin(gw, gh);
    if !st.is_inventory() {
        t.draw(
            &lang.tr(TABS[st.tab].key),
            l + 8.0,
            top + 6.0,
            0x404040,
            false,
        );
    }
    if st.is_search() {
        // VANILLA: EditBox (82,6) 80×9, белый текст с тенью; показываем хвост, если не влезает.
        let chars: Vec<char> = st.search.chars().collect();
        let mut first = 0;
        while first < chars.len().min(st.cursor)
            && t.text_width(
                &chars[first..st.cursor.max(first)]
                    .iter()
                    .collect::<String>(),
            ) > 79.0
        {
            first += 1;
        }
        let mut last = chars.len();
        while last > first && t.text_width(&chars[first..last].iter().collect::<String>()) > 79.0 {
            last -= 1;
        }
        fn w(t: &TextRenderer, chars: &[char], a: usize, b: usize) -> f32 {
            t.text_width(&chars[a.min(b)..b].iter().collect::<String>())
        }
        let (sa, sb) = st.selection();
        if sa != sb {
            let (x0, x1) = (
                w(t, &chars, first, sa.clamp(first, last)),
                w(t, &chars, first, sb.clamp(first, last)),
            );
            t.fill(
                l + 82.0 + x0,
                top + 5.0,
                l + 82.0 + x1,
                top + 15.0,
                0xFF3A6FD8,
            );
        }
        let txt: String = chars[first..last].iter().collect();
        t.draw(&txt, l + 82.0, top + 6.0, 0xFFFFFF, true);
        let blink = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() / 300);
        if blink % 2 == 0 && st.cursor >= first && st.cursor <= last {
            let cx = l + 82.0 + w(t, &chars, first, st.cursor);
            if st.cursor == chars.len() {
                t.draw("_", cx, top + 6.0, 0xFFFFFF, true);
            } else {
                t.fill(cx, top + 5.0, cx + 1.0, top + 14.0, 0xFFFFFFFF);
            }
        }
    }
    if st.kind() == Kind::Hotbar {
        for c in 0..COLS * ROWS {
            if let Cell::Stack(s) = st.cell(st.row * COLS + c) {
                let (x, y) = (
                    l + 9.0 + (c % COLS) as f32 * 18.0,
                    top + 18.0 + (c / COLS) as f32 * 18.0,
                );
                hud::stack_count(t, &s, x, y);
            }
        }
    }
    for (i, x, y) in player_slots(st) {
        if let Some(s) = &inv.slots[i] {
            hud::stack_count(t, s, l + x, top + y);
        }
    }
}

/// Центр слота на экране в GUI-пикселях (для управления агентом).
pub fn slot_center(st: &State, gw: f32, gh: f32, s: Slot) -> Option<(f32, f32)> {
    let (l, t) = origin(gw, gh);
    let (x, y) = match s {
        Slot::Cell(c) => (
            9.0 + (c % COLS) as f32 * 18.0,
            18.0 + (c / COLS) as f32 * 18.0,
        ),
        Slot::Inv(i) => player_slots(st)
            .into_iter()
            .find(|p| p.0 == i)
            .map(|p| (p.1, p.2))?,
        Slot::Trash => TRASH,
    };
    Some((l + x + 8.0, t + y + 8.0))
}

/// Центр вкладки (для управления агентом).
pub fn tab_center(gw: f32, gh: f32, i: usize) -> Option<(f32, f32)> {
    let (l, t) = origin(gw, gh);
    let tb = TABS.get(i)?;
    let y = if tb.top { -16.0 } else { H + 16.0 };
    Some((l + tab_x(tb) + 13.0, t + y))
}

/// Headless-проверка креатив-инвентаря по спецификации: `cargo test -p ezzmc-client spec -- --nocapture`.
#[cfg(test)]
mod spec {
    use super::*;

    fn st(n: &str, c: i32) -> ItemStack {
        stack_of(n, c).unwrap()
    }
    /// Состояние + индекс ячейки сетки с предметом.
    fn grid(name: &str) -> (State, usize) {
        let l = Lang::default();
        let mut s = State::default();
        for t in 0..10 {
            let e = cd::tab(TABS[t].data);
            if let Some(i) = e
                .iter()
                .position(|e| e.name == name && e.patch == ComponentPatch::default())
            {
                s.select_tab(t, &l);
                s.row = (i / COLS).min(s.max_row());
                let c = i - s.row * COLS;
                return (s, c);
            }
        }
        panic!("нет {name}");
    }
    #[test]
    fn search_editing() {
        let l = Lang::default();
        let mut s = State::default();
        s.select_tab(SEARCH_TAB, &l);
        s.type_text("привет мир", &l);
        s.move_cursor(Move::Left, true, true);
        assert_eq!(s.selected_text(), "мир");
        assert_eq!(s.cut(&l), "мир");
        assert_eq!(s.search, "привет ");
        s.type_text("Ё", &l);
        s.move_cursor(Move::Home, false, false);
        s.erase(1, false, &l);
        assert_eq!(s.search, "ривет Ё");
        s.select_all();
        s.type_text("a", &l);
        assert_eq!(s.search, "a");
    }

    fn empty_cell() -> (State, usize) {
        let l = Lang::default();
        let mut s = State::default();
        s.select_tab(SEARCH_TAB, &l);
        s.type_text("zzzzqq", &l);
        (s, 0)
    }
    fn go(s: &mut State, inv: &mut Inventory, h: Hit, b: i8, m: i32) -> Vec<NetCmd> {
        click(s, inv, &h, b, m, &Lang::default())
    }
    fn cell(c: usize) -> Hit {
        Hit::Slot(Slot::Cell(c))
    }
    fn hb(i: usize) -> Hit {
        Hit::Slot(Slot::Inv(HOTBAR_START + i))
    }
    fn cur(inv: &Inventory) -> Option<(u32, i32)> {
        inv.cursor.as_ref().map(|c| (c.item, c.count))
    }
    fn id(n: &str) -> u32 {
        st(n, 1).item
    }
    fn k(n: &str, c: i32) -> Option<(u32, i32)> {
        Some((id(n), c))
    }
    fn slot(inv: &Inventory, i: usize) -> Option<(u32, i32)> {
        inv.slots[HOTBAR_START + i]
            .as_ref()
            .map(|c| (c.item, c.count))
    }

    #[test]
    fn spec() {
        let mut rep: Vec<(&str, String, String, &str)> = Vec::new();
        macro_rules! chk {
            ($id:expr, $exp:expr, $act:expr) => {{
                let (e, a) = (format!("{:?}", $exp), format!("{:?}", $act));
                rep.push((
                    $id,
                    e.clone(),
                    a.clone(),
                    if e == a { "PASS" } else { "FAIL" },
                ));
            }};
        }
        macro_rules! unv {
            ($id:expr, $act:expr) => {
                rep.push(($id, "[VERIFY]".into(), format!("{:?}", $act), "UNVERIFIED"))
            };
        }
        // Прогон: (курсор, ввод) по ячейке stone/dirt/пустой.
        let run = |name: Option<&str>, c: Option<(&str, i32)>, b: i8, m: i32| {
            let (mut s, ci) = name.map_or_else(empty_cell, grid);
            let mut inv = Inventory::default();
            inv.cursor = c.map(|(n, x)| st(n, x));
            let out = go(&mut s, &mut inv, cell(ci), b, m);
            (cur(&inv), out.len(), inv)
        };
        let (a, ..) = run(Some("stone"), None, 0, PICKUP);
        chk!("G1", k("stone", 1), a);
        chk!("G2", k("stone", 1), run(Some("stone"), None, 1, PICKUP).0);
        chk!("G3", k("stone", 64), run(Some("stone"), None, 2, CLONE).0);
        chk!(
            "G4",
            k("stone", 64),
            run(Some("stone"), None, 0, QUICK_MOVE).0
        );
        chk!(
            "G5",
            k("stone", 64),
            run(Some("stone"), None, 1, QUICK_MOVE).0
        );
        chk!(
            "G6",
            k("stone", 6),
            run(Some("stone"), Some(("stone", 5)), 0, PICKUP).0
        );
        chk!(
            "G7",
            k("stone", 64),
            run(Some("stone"), Some(("stone", 64)), 0, PICKUP).0
        );
        chk!(
            "G8",
            k("stone", 4),
            run(Some("stone"), Some(("stone", 5)), 1, PICKUP).0
        );
        chk!(
            "G9",
            None::<(u32, i32)>,
            run(Some("stone"), Some(("stone", 1)), 1, PICKUP).0
        );
        chk!(
            "G10",
            k("stone", 64),
            run(Some("stone"), Some(("stone", 5)), 0, QUICK_MOVE).0
        );
        chk!(
            "G11",
            k("stone", 4),
            run(Some("stone"), Some(("stone", 5)), 1, QUICK_MOVE).0
        );
        chk!(
            "G12",
            None::<(u32, i32)>,
            run(Some("dirt"), Some(("stone", 5)), 0, PICKUP).0
        );
        chk!(
            "G13",
            k("stone", 4),
            run(Some("dirt"), Some(("stone", 5)), 1, PICKUP).0
        );
        chk!(
            "G14",
            None::<(u32, i32)>,
            run(None, Some(("stone", 5)), 0, PICKUP).0
        );
        chk!(
            "G15",
            k("stone", 4),
            run(None, Some(("stone", 5)), 1, PICKUP).0
        );
        chk!(
            "G16",
            (None::<(u32, i32)>, None::<(u32, i32)>),
            (run(None, None, 0, PICKUP).0, run(None, None, 1, PICKUP).0)
        );
        // G17: кастомный компонент → другой предмет (курсор сбрасывается).
        {
            let (mut s, ci) = grid("stone");
            let mut inv = Inventory::default();
            let mut c = st("stone", 5);
            c.patch.added.push((1, vec![1u8].into()));
            inv.cursor = Some(c);
            go(&mut s, &mut inv, cell(ci), 0, PICKUP);
            chk!("G17", None::<(u32, i32)>, cur(&inv));
        }
        // G18–G22: цифры (SWAP).
        {
            let (mut s, ci) = grid("stone");
            let mut inv = Inventory::default();
            inv.slots[HOTBAR_START + 2] = Some(st("diamond_sword", 1));
            let o = go(&mut s, &mut inv, cell(ci), 2, SWAP);
            chk!(
                "G18",
                (k("stone", 64), None::<(u32, i32)>, 1),
                (slot(&inv, 2), cur(&inv), o.len())
            );
            let (mut s, ci) = grid("egg");
            let mut inv = Inventory::default();
            go(&mut s, &mut inv, cell(ci), 0, SWAP);
            chk!("G19", k("egg", 16), slot(&inv, 0));
            let (mut s, ci) = grid("diamond_sword");
            let mut inv = Inventory::default();
            go(&mut s, &mut inv, cell(ci), 0, SWAP);
            chk!("G20", k("diamond_sword", 1), slot(&inv, 0));
            // vanilla: SWAP при непустом курсоре не вызывается экраном (main.rs: cursor_empty).
            let (mut s, ci) = empty_cell();
            let mut inv = Inventory::default();
            let o = go(&mut s, &mut inv, cell(ci), 2, SWAP);
            chk!("G22", (None::<(u32, i32)>, 0), (slot(&inv, 2), o.len()));
        }
        unv!("G21", "условие cursor_empty в main.rs (creative_search_input), только Search; в click() не проверяется — не покрыто headless");
        // G23–G25: выброс.
        for (id_, n, q, qc) in [
            ("G23/24", "stone", 1, 64),
            ("G25a", "egg", 1, 16),
            ("G25b", "diamond_sword", 1, 1),
        ] {
            let (mut s, ci) = grid(n);
            let mut inv = Inventory::default();
            let a = go(&mut s, &mut inv, cell(ci), 0, THROW);
            let b = go(&mut s, &mut inv, cell(ci), 1, THROW);
            let d = |o: &Vec<NetCmd>| match o.first() {
                Some(NetCmd::CreativeSlot(-1, Some(x))) => Some(x.count),
                _ => None,
            };
            chk!(id_, (Some(q), Some(qc)), (d(&a), d(&b)));
        }
        // G26/G27: подряд клики по сетке.
        {
            let (mut s, ci) = grid("stone");
            let mut inv = Inventory::default();
            for _ in 0..3 {
                go(&mut s, &mut inv, cell(ci), 0, PICKUP);
            }
            chk!("G26/27", k("stone", 3), cur(&inv));
        }
        unv!("G28", "drag по сетке: creative_mouse стартует drag только для Slot::Inv; для сетки — обычный клик на нажатии");
        // Q при курсоре над stone: click() выбрасывает независимо от курсора.
        {
            let (mut s, ci) = grid("stone");
            let mut inv = Inventory::default();
            inv.cursor = Some(st("stone", 5));
            let o = go(&mut s, &mut inv, cell(ci), 0, THROW);
            unv!("G29", (o.len(), cur(&inv)));
        }
        // Хотбар.
        let hbrun = |h3: Option<(&str, i32)>, c: Option<(&str, i32)>, b: i8, m: i32| {
            let (mut s, _) = grid("stone");
            let mut inv = Inventory::default();
            inv.slots[HOTBAR_START + 3] = h3.map(|(n, x)| st(n, x));
            inv.cursor = c.map(|(n, x)| st(n, x));
            let o = go(&mut s, &mut inv, hb(3), b, m);
            (slot(&inv, 3), cur(&inv), o)
        };
        let e = None::<(u32, i32)>;
        for (i, b) in [("H1", 0), ("H2", 1)] {
            let (h, c, o) = hbrun(Some(("stone", 20)), None, b, QUICK_MOVE);
            chk!(
                i,
                (e, e, true),
                (
                    h,
                    c,
                    matches!(o.first(), Some(NetCmd::CreativeSlot(_, None)))
                )
            );
        }
        chk!("H3", 0, hbrun(None, None, 0, QUICK_MOVE).2.len());
        unv!("H4", {
            let (h, c, _) = hbrun(Some(("dirt", 10)), Some(("stone", 5)), 0, QUICK_MOVE);
            (h, c)
        });
        let (h, c, _) = hbrun(Some(("stone", 20)), None, 0, PICKUP);
        chk!("H5", (e, k("stone", 20)), (h, c));
        let (h, c, _) = hbrun(None, Some(("stone", 5)), 0, PICKUP);
        chk!("H6", (k("stone", 5), e), (h, c));
        let (h, c, _) = hbrun(Some(("stone", 5)), None, 1, PICKUP);
        chk!("H7", (k("stone", 2), k("stone", 3)), (h, c));
        let (h, c, _) = hbrun(Some(("stone", 10)), Some(("stone", 5)), 1, PICKUP);
        chk!("H8", (k("stone", 11), k("stone", 4)), (h, c));
        let (h, c, _) = hbrun(Some(("stone", 10)), Some(("diamond_sword", 1)), 0, PICKUP);
        chk!("H9", (k("diamond_sword", 1), k("stone", 10)), (h, c));
        let (h, c, _) = hbrun(Some(("stone", 20)), None, 2, CLONE);
        chk!("H10", (k("stone", 20), k("stone", 64)), (h, c));
        {
            let (mut s, _) = grid("stone");
            let mut inv = Inventory::default();
            inv.slots[HOTBAR_START + 3] = Some(st("stone", 20));
            inv.slots[HOTBAR_START] = Some(st("dirt", 1));
            go(&mut s, &mut inv, hb(3), 0, SWAP);
            chk!(
                "H11",
                (k("dirt", 1), k("stone", 20)),
                (slot(&inv, 3), slot(&inv, 0))
            );
            // H12: двойной клик по пустому h2 с курсором stone×10 при h0,h4 = 30.
            let mut inv = Inventory::default();
            inv.cursor = Some(st("stone", 10));
            inv.slots[HOTBAR_START] = Some(st("stone", 30));
            inv.slots[HOTBAR_START + 4] = Some(st("stone", 30));
            go(&mut s, &mut inv, hb(1), 0, PICKUP_ALL);
            chk!("H12", k("stone", 64), cur(&inv));
        }
        for (i, ty, per, left) in [
            ("H13", 0u8, 3, None),
            ("H14", 1, 1, k("stone", 6)),
            ("H15", 2, 64, None),
        ] {
            let mut inv = Inventory::default();
            inv.cursor = Some(st(
                "stone",
                if ty == 0 {
                    9
                } else if ty == 1 {
                    9
                } else {
                    1
                },
            ));
            let sl = [HOTBAR_START, HOTBAR_START + 1, HOTBAR_START + 2];
            let o = finish_drag(&mut inv, &sl, ty);
            // VANILLA: AbstractContainerMenu#doClick QUICK_CRAFT — remaining уходит в минус, курсор пуст (спец. ошибалась).
            let want = left;
            chk!(
                i,
                (k("stone", per), k("stone", per), k("stone", per), want, 3),
                (
                    slot(&inv, 0),
                    slot(&inv, 1),
                    slot(&inv, 2),
                    cur(&inv),
                    o.len()
                )
            );
        }
        unv!("H15-sync", "после СКМ-раздачи должны уходить пакеты всех 9 слотов хотбара; сейчас только изменённые слоты (send_changes)");
        {
            let (h, _, o) = hbrun(Some(("stone", 20)), None, 0, THROW);
            let (h2, _, o2) = hbrun(Some(("stone", 20)), None, 1, THROW);
            let dr = |o: &Vec<NetCmd>| matches!(o.first(), Some(NetCmd::CreativeSlot(-1, Some(_))));
            chk!(
                "H16",
                (k("stone", 19), e, true, true),
                (h, h2, dr(&o), dr(&o2))
            );
        }
        // Survival Inventory.
        {
            let l = Lang::default();
            let mut s = State::default();
            s.select_tab(INVENTORY_TAB, &l);
            let mut inv = Inventory::default();
            inv.slots[20] = Some(st("stone", 10));
            go(&mut s, &mut inv, Hit::Slot(Slot::Inv(20)), 0, QUICK_MOVE);
            chk!(
                "I1",
                (None::<(u32, i32)>, k("stone", 10)),
                (
                    inv.slots[20].as_ref().map(|c| (c.item, c.count)),
                    slot(&inv, 0)
                )
            );
            let mut inv = Inventory::default();
            inv.slots[20] = Some(st("diamond_sword", 1));
            inv.slots[5] = None;
            // броня: нужен предмет-броня
            inv.slots[21] = stack_of("iron_helmet", 1);
            go(&mut s, &mut inv, Hit::Slot(Slot::Inv(21)), 0, QUICK_MOVE);
            chk!("I2", true, inv.slots[5].is_some());
            let mut inv = Inventory::default();
            inv.slots[20] = Some(st("stone", 10));
            go(&mut s, &mut inv, Hit::Slot(Slot::Inv(20)), 2, CLONE);
            chk!("I3", k("stone", 64), cur(&inv));
            let mut inv = Inventory::default();
            inv.cursor = Some(st("stone", 5));
            go(&mut s, &mut inv, Hit::Slot(Slot::Trash), 0, PICKUP);
            chk!("I5", e, cur(&inv));
            let mut inv = Inventory::default();
            for i in 1..PLAYER_SLOTS {
                inv.slots[i] = Some(st("stone", 1));
            }
            let o = go(&mut s, &mut inv, Hit::Slot(Slot::Trash), 0, QUICK_MOVE);
            chk!(
                "I6",
                (true, o.len()),
                (inv.slots.iter().all(Option::is_none), PLAYER_SLOTS - 1)
            );
            let mut inv = Inventory::default();
            inv.slots[20] = Some(st("stone", 100));
            let o = go(&mut s, &mut inv, Hit::Slot(Slot::Inv(20)), 1, THROW);
            chk!(
                "I7",
                true,
                matches!(o.first(), Some(NetCmd::CreativeSlot(-1, Some(x))) if x.count == 64)
            );
        }
        unv!("I4/I8", "I4 drag Inv-слотов через main.rs (не покрыто); I8 — раскладка Slot::Inv/hit() без крафта: сверить глазами");
        // Курсор / вне окна / скролл.
        {
            let l = Lang::default();
            let (mut s, _) = grid("stone");
            let mut inv = Inventory::default();
            inv.cursor = Some(st("stone", 5));
            s.select_tab(2, &l);
            chk!("C1", k("stone", 5), cur(&inv));
            let o = go(&mut s, &mut inv, Hit::Outside, 0, PICKUP);
            chk!(
                "C3",
                (e, true),
                (
                    cur(&inv),
                    matches!(o.first(), Some(NetCmd::CreativeSlot(-1, Some(x))) if x.count == 5)
                )
            );
            inv.cursor = Some(st("stone", 5));
            let o = go(&mut s, &mut inv, Hit::Outside, 1, PICKUP);
            chk!(
                "C4",
                (k("stone", 4), true),
                (
                    cur(&inv),
                    matches!(o.first(), Some(NetCmd::CreativeSlot(-1, Some(x))) if x.count == 1)
                )
            );
            s.scroll(-1.0);
            let r1 = s.row;
            for _ in 0..1000 {
                s.scroll(-1.0);
            }
            chk!("C6", (1, s.max_row()), (r1, s.row));
            s.select_tab(3, &l);
            chk!("C8", 0, s.row);
            s.select_tab(SEARCH_TAB, &l);
            s.type_text("diamond_sword", &l);
            let small = s.count() <= 45;
            s.scroll(-1.0);
            chk!("C7", (true, 0), (small, if small { s.row } else { 0 }));
            s.select_tab(SEARCH_TAB, &l);
            s.type_text("a", &l);
            s.row = 3;
            s.type_text("b", &l);
            chk!("S5", 0, s.row);
        }
        unv!("C2/C5/S1-S4/S6/S7/SH*/OP1", "UI-ввод в main.rs, поиск по тегам/тултипы, сохранённые хотбары, оп-вкладка: не покрыто headless (OP-вкладки нет, см. шапку модуля)");

        println!("\nID | ожидалось | фактически | итог");
        for (i, ex, ac, r) in &rep {
            println!("{i} | {ex} | {ac} | {r}");
        }
        let fails: Vec<_> = rep.iter().filter(|r| r.3 == "FAIL").map(|r| r.0).collect();
        println!("FAIL: {fails:?}");
    }
}
