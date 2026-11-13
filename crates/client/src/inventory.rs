//! Инвентарь игрока и открытые контейнеры: состояние слотов, разбор пакетов 26.3.
#![allow(dead_code)] // часть API ждёт экранов инвентаря (232/236)
use ezzmc_protocol::item::{self, ItemStack};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// Раскладка InventoryMenu: 0 результат крафта, 1–4 сетка 2×2, 5–8 броня (голова→ботинки),
/// 9–35 основной, 36–44 хотбар, 45 вторая рука.
pub const PLAYER_SLOTS: usize = 46;
pub const CRAFT_OUTPUT: usize = 0;
pub const ARMOR_START: usize = 5;
pub const MAIN_START: usize = 9;
pub const HOTBAR_START: usize = 36;
pub const OFFHAND: usize = 45;

pub type Slot = Option<ItemStack>;

/// Индекс Inventory (хотбар 0–8, основной 9–35, броня ботинки→голова 36–39, offhand 40) → слот меню.
pub fn menu_slot_for_inventory_index(i: i32) -> Option<usize> {
    Some(match i {
        0..=8 => HOTBAR_START + i as usize,
        9..=35 => i as usize,
        36..=39 => ARMOR_START + (39 - i) as usize,
        40 => OFFHAND,
        _ => return None,
    })
}

/// Событие от сервера, относящееся к инвентарю.
#[derive(Clone, Debug, PartialEq)]
pub enum InvEvent {
    Content {
        id: i32,
        state: i32,
        items: Vec<Slot>,
        carried: Slot,
    },
    SetSlot {
        id: i32,
        state: i32,
        slot: i16,
        item: Slot,
    },
    Cursor(Slot),
    PlayerSlot {
        index: i32,
        item: Slot,
    },
    Held(u8),
    Open {
        id: i32,
        menu: i32,
        title: String,
    },
    Close(i32),
    /// ClientboundMerchantOffersPacket (житель/странствующий торговец).
    Trades {
        id: i32,
        trades: Box<crate::merchantscreen::Trades>,
    },
    /// ContainerSetData: свойство меню (печь: 0 горение, 1 длит., 2 готовка, 3 всего).
    Data {
        id: i32,
        prop: i16,
        value: i16,
    },
}

/// Разбор пакета по имени; `None` — пакет не про инвентарь.
pub fn parse_packet(name: &str, body: &[u8]) -> ProtocolResult<Option<InvEvent>> {
    let mut r = Reader::new(body);
    Ok(Some(match name {
        "container_set_content" => {
            let id = r.var_int()?;
            let state = r.var_int()?;
            let n = r.length("container items", 256)?;
            let mut items = Vec::with_capacity(n);
            for _ in 0..n {
                items.push(item::read_stack(&mut r)?);
            }
            let carried = item::read_stack(&mut r)?;
            InvEvent::Content {
                id,
                state,
                items,
                carried,
            }
        }
        "container_set_slot" => InvEvent::SetSlot {
            id: r.var_int()?,
            state: r.var_int()?,
            slot: r.i16()?,
            item: item::read_stack(&mut r)?,
        },
        "set_cursor_item" => InvEvent::Cursor(item::read_stack(&mut r)?),
        "set_player_inventory" => InvEvent::PlayerSlot {
            index: r.var_int()?,
            item: item::read_stack(&mut r)?,
        },
        "set_held_slot" => InvEvent::Held(r.var_int()?.clamp(0, 8) as u8),
        "open_screen" => {
            let id = r.var_int()?;
            let menu = r.var_int()?;
            let title = match ezzmc_protocol::nbt::read_network(&mut r)? {
                Some(t) => ezzmc_protocol::text::Component::from_nbt(&t)
                    .map(|c| c.to_plain(&|_| None))
                    .unwrap_or_default(),
                None => String::new(),
            };
            InvEvent::Open { id, menu, title }
        }
        "container_close" => InvEvent::Close(r.var_int()?),
        "merchant_offers" => {
            let (id, trades) = crate::merchantscreen::parse(body)?;
            InvEvent::Trades {
                id,
                trades: Box::new(trades),
            }
        }
        "container_set_data" => InvEvent::Data {
            id: r.var_int()?,
            prop: r.i16()?,
            value: r.i16()?,
        },
        _ => return Ok(None),
    }))
}

/// Открытый (не-инвентарный) контейнер.
#[derive(Clone, Debug)]
pub struct OpenMenu {
    pub id: i32,
    pub menu: i32,
    pub title: String,
    pub slots: Vec<Slot>,
    /// Свойства меню (ContainerData).
    pub data: [i16; 10],
    /// Выбор эффектов маяка в экране (primary, secondary; id реестра +1, 0 — нет).
    /// VANILLA: BeaconScreen#primary/secondary, обновляются в dataChanged и кнопками.
    pub sel: [i16; 2],
    /// Скроллбар списка (ткацкий станок): scrollOffs и захват мышью.
    pub scroll: f32,
    pub scrolling: bool,
    /// Предложения торговца (меню merchant).
    pub trades: Option<crate::merchantscreen::Trades>,
}

/// Состояние инвентаря клиента.
#[derive(Clone, Debug)]
pub struct Inventory {
    /// Слоты InventoryMenu (контейнер 0).
    pub slots: Vec<Slot>,
    pub cursor: Slot,
    pub selected: u8,
    pub state_id: i32,
    pub open: Option<OpenMenu>,
    /// VANILLA: LocalPlayer#dropSpamThrottler (TickThrottler(20, 1280)): +20 за бросок, −1 за тик.
    pub drop_spam: i32,
    /// Уровень опыта и «бесконечные материалы» (креатив/наблюдатель) — для экранов контейнеров.
    pub xp_level: i32,
    pub infinite_materials: bool,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![None; PLAYER_SLOTS],
            cursor: None,
            selected: 0,
            state_id: 0,
            open: None,
            drop_spam: 0,
            xp_level: 0,
            infinite_materials: false,
        }
    }
}

impl Inventory {
    /// VANILLA: TickThrottler#isUnderThreshold.
    pub fn can_drop(&self) -> bool {
        self.drop_spam < 1280
    }

    /// Выбранный предмет хотбара.
    pub fn held(&self) -> Option<&ItemStack> {
        self.slots
            .get(HOTBAR_START + usize::from(self.selected))?
            .as_ref()
    }

    pub fn hotbar(&self) -> &[Slot] {
        &self.slots[HOTBAR_START..HOTBAR_START + 9]
    }

    /// Последние 36 слотов открытого меню — инвентарь игрока (основной 27 + хотбар 9).
    fn mirror_player_slots(&mut self) {
        let Some(o) = self.open.as_ref() else { return };
        let n = o.slots.len();
        if n >= 36 {
            for j in 0..36 {
                self.slots[MAIN_START + j] = o.slots[n - 36 + j].clone();
            }
        }
    }

    /// Применить пакет сервера.
    pub fn apply(&mut self, ev: InvEvent) {
        match ev {
            InvEvent::Content {
                id,
                state,
                items,
                carried,
            } => {
                if id == 0 {
                    self.state_id = state;
                    for (i, s) in items.into_iter().enumerate().take(PLAYER_SLOTS) {
                        self.slots[i] = s;
                    }
                    self.cursor = carried;
                } else if let Some(o) = self.open.as_mut().filter(|o| o.id == id) {
                    self.state_id = state;
                    o.slots = items;
                    self.cursor = carried;
                    self.mirror_player_slots();
                }
            }
            InvEvent::SetSlot {
                id,
                state,
                slot,
                item,
            } => {
                let Ok(i) = usize::try_from(slot) else { return };
                if id == 0 {
                    self.state_id = state;
                    if let Some(s) = self.slots.get_mut(i) {
                        *s = item;
                    }
                } else if let Some(o) = self.open.as_mut().filter(|o| o.id == id) {
                    self.state_id = state;
                    if let Some(s) = o.slots.get_mut(i) {
                        *s = item;
                    }
                    self.mirror_player_slots();
                }
            }
            InvEvent::Cursor(c) => self.cursor = c,
            InvEvent::PlayerSlot { index, item } => {
                if let Some(i) = menu_slot_for_inventory_index(index) {
                    self.slots[i] = item;
                }
            }
            InvEvent::Held(s) => self.selected = s,
            InvEvent::Open { id, menu, title } => {
                self.open = Some(OpenMenu {
                    id,
                    menu,
                    title,
                    slots: Vec::new(),
                    data: [0; 10],
                    sel: [0; 2],
                    scroll: 0.0,
                    scrolling: false,
                    trades: None,
                });
            }
            InvEvent::Trades { id, trades } => {
                // VANILLA: ClientPacketListener#handleMerchantOffers — только для открытого меню с этим id;
                // выбранная сделка и прокрутка сбрасываются вместе с экраном (новое меню), при обновлении сохраняются.
                if let Some(o) = self.open.as_mut().filter(|o| o.id == id) {
                    let mut t = *trades;
                    if let Some(old) = o.trades.as_ref() {
                        t.shop_item = old.shop_item.min(t.offers.len().saturating_sub(1));
                        t.scroll_off = old.scroll_off.min(t.offers.len().saturating_sub(7));
                        t.dragging = old.dragging;
                    }
                    o.trades = Some(t);
                }
            }
            InvEvent::Data { id, prop, value } => {
                if let Some(o) = self.open.as_mut().filter(|o| o.id == id) {
                    if let Some(d) = usize::try_from(prop).ok().and_then(|i| o.data.get_mut(i)) {
                        *d = value;
                    }
                    // VANILLA: BeaconScreen dataChanged → primary/secondary из menu.
                    if let 1 | 2 = prop {
                        o.sel = [o.data[1], o.data[2]];
                    }
                }
            }
            InvEvent::Close(id) => {
                if self.open.as_ref().is_some_and(|o| o.id == id) || id == 0 {
                    self.open = None;
                }
            }
        }
    }
}

fn stack_max(s: &ItemStack) -> i32 {
    ezzmc_protocol::generated::r26_3::ITEM
        .get(s.item as usize)
        .map_or(64, |n| crate::itemdata::max_stack(n))
}

/// Предмет, который quick-move может отправить в слот брони/второй руки (по имени; запасной путь — сервер).
fn is_equippable(s: &ItemStack) -> bool {
    let name = ezzmc_protocol::generated::r26_3::ITEM
        .get(s.item as usize)
        .copied()
        .unwrap_or("");
    [
        "_helmet",
        "_chestplate",
        "_leggings",
        "_boots",
        "_head",
        "_skull",
        "_banner",
    ]
    .iter()
    .any(|p| name.ends_with(p))
        || matches!(
            name,
            "elytra" | "shield" | "carved_pumpkin" | "totem_of_undying" | "saddle" | "wolf_armor"
        )
        || name.ends_with("_harness")
        || name.ends_with("_horse_armor")
        || name.ends_with("_nautilus_armor")
}

fn same_item(a: &ItemStack, b: &ItemStack) -> bool {
    a.item == b.item && a.patch == b.patch
}

/// Слоты-результаты (клик по ним предсказывать нельзя): результат крафта InventoryMenu.
fn is_output(open: bool, slot: usize) -> bool {
    !open && slot == CRAFT_OUTPUT
}

impl Inventory {
    pub fn menu_slots(&self) -> &[Slot] {
        match &self.open {
            Some(o) => &o.slots,
            None => &self.slots,
        }
    }

    fn menu_slots_mut(&mut self) -> &mut Vec<Slot> {
        match self.open.as_mut() {
            Some(o) => &mut o.slots,
            None => &mut self.slots,
        }
    }

    /// Предсказание клика (VANILLA: AbstractContainerMenu#doClick для PICKUP / PICKUP_ALL).
    /// Сервер остаётся авторитетным и поправит расхождение (мы шлём пустые хэши).
    pub fn predict_click(&mut self, slot: i16, button: i8, mode: i32) {
        let open = self.open.is_some();
        if slot == -999 {
            if mode == 0 {
                // Бросок курсора наружу: левая — всё, правая — один.
                if let Some(c) = self.cursor.as_mut() {
                    if button == 0 || c.count <= 1 {
                        self.cursor = None;
                    } else {
                        c.count -= 1;
                    }
                }
            }
            return;
        }
        let Ok(i) = usize::try_from(slot) else { return };
        if is_output(open, i) {
            return;
        }
        match mode {
            0 => self.predict_pickup(i, button),
            1 => self.predict_quick_move(i),
            2 => self.predict_swap(i, button),
            4 => self.predict_throw(i, button),
            6 => self.predict_pickup_all(),
            _ => {}
        }
        if open {
            self.mirror_player_slots();
        }
    }

    /// Меню контейнера, где все слоты контейнера принимают любые предметы (ChestMenu-подобные).
    fn plain_container(&self) -> bool {
        self.open.as_ref().is_some_and(|o| {
            let name = ezzmc_protocol::generated::r26_3::MENU
                .get(o.menu as usize)
                .copied()
                .unwrap_or("");
            name.starts_with("generic_9x") || name == "generic_3x3" || name == "hopper"
        })
    }

    /// Слот, из которого/в который можно предсказывать без знания правил меню (mayPlace/mayPickup).
    fn free_slot(&self, i: usize) -> bool {
        match &self.open {
            None => (1..=4).contains(&i) || (MAIN_START..=OFFHAND).contains(&i),
            Some(o) => {
                let n = o.slots.len();
                i < n && (self.plain_container() || i + 36 >= n)
            }
        }
    }

    /// VANILLA: AbstractContainerMenu#doClick SWAP (кнопка 0–8 хотбар, 40 вторая рука).
    fn predict_swap(&mut self, i: usize, button: i8) {
        let open = self.open.is_some();
        let n = self.menu_slots().len();
        let other = match button {
            0..=8 if open => n.saturating_sub(9) + button as usize,
            0..=8 => HOTBAR_START + button as usize,
            40 if !open => OFFHAND,
            _ => return,
        };
        if other == i || other >= n || !self.free_slot(i) || !self.free_slot(other) {
            return;
        }
        let slots = self.menu_slots_mut();
        // Слот-источник с переполненным (больше max) стеком или чужим maxStackSize — на сервер.
        slots.swap(i, other);
    }

    /// VANILLA: AbstractContainerMenu#doClick THROW (0 — один предмет, 1 — весь стек, Ctrl+Q).
    fn predict_throw(&mut self, i: usize, button: i8) {
        if self.cursor.is_some() || !self.free_slot(i) {
            return;
        }
        let Some(cell) = self.menu_slots_mut().get_mut(i) else {
            return;
        };
        let Some(s) = cell.as_mut() else { return };
        if button == 0 && s.count > 1 {
            s.count -= 1;
        } else {
            *cell = None;
        }
    }

    /// VANILLA: AbstractContainerMenu#moveItemStackTo (сперва слияние, затем пустые слоты).
    fn move_stack_to(&mut self, stack: &mut ItemStack, start: usize, end: usize, reverse: bool) {
        let max = stack_max(stack);
        let slots = self.menu_slots_mut();
        let end = end.min(slots.len());
        let order: Vec<usize> = if reverse {
            (start..end).rev().collect()
        } else {
            (start..end).collect()
        };
        if max > 1 {
            for &k in &order {
                if stack.count <= 0 {
                    return;
                }
                if let Some(t) = slots[k].as_mut() {
                    if same_item(t, stack) {
                        let mv = (max - t.count).min(stack.count).max(0);
                        t.count += mv;
                        stack.count -= mv;
                    }
                }
            }
        }
        for &k in &order {
            if stack.count <= 0 {
                return;
            }
            if slots[k].is_none() {
                let mut t = stack.clone();
                t.count = stack.count.min(max);
                stack.count -= t.count;
                slots[k] = Some(t);
            }
        }
    }

    /// VANILLA: AbstractContainerMenu#quickMoveStack для ChestMenu и InventoryMenu (основной↔хотбар).
    /// Меню с особыми правилами (печь, верстак, броня…) — не предсказываем, ответит сервер.
    fn predict_quick_move(&mut self, i: usize) {
        let n = self.menu_slots().len();
        let Some(Some(src)) = self.menu_slots().get(i).cloned() else {
            return;
        };
        if self.open.is_some() {
            if !self.plain_container() || n < 36 {
                return;
            }
            let mut rest = src;
            if i < n - 36 {
                self.move_stack_to(&mut rest, n - 36, n, true);
            } else {
                self.move_stack_to(&mut rest, 0, n - 36, false);
            }
            self.menu_slots_mut()[i] = (rest.count > 0).then_some(rest);
            return;
        }
        // InventoryMenu: броня/щит/результат/сетка и экипируемое — на сервер.
        if !(MAIN_START..OFFHAND).contains(&i) || is_equippable(&src) {
            return;
        }
        let mut rest = src;
        if i < HOTBAR_START {
            self.move_stack_to(&mut rest, HOTBAR_START, OFFHAND, false);
        } else {
            self.move_stack_to(&mut rest, MAIN_START, HOTBAR_START, false);
        }
        self.slots[i] = (rest.count > 0).then_some(rest);
    }

    /// VANILLA: Slot#mayPlace для InventoryMenu: результат крафта — ничего; броня — только
    /// предмет с `equippable` на этот слот (ArmorSlot, stack max 1).
    fn may_place(&self, i: usize, st: &ItemStack) -> bool {
        if self.open.is_some() {
            return true;
        }
        if i == CRAFT_OUTPUT {
            return false;
        }
        if !(ARMOR_START..ARMOR_START + 4).contains(&i) {
            return true;
        }
        let name = ezzmc_protocol::generated::r26_3::ITEM
            .get(st.item as usize)
            .copied()
            .unwrap_or("");
        match i - ARMOR_START {
            0 => {
                name.ends_with("_helmet")
                    || name.ends_with("_head")
                    || name.ends_with("_skull")
                    || name == "carved_pumpkin"
            }
            1 => name.ends_with("_chestplate") || name == "elytra",
            2 => name.ends_with("_leggings"),
            _ => name.ends_with("_boots"),
        }
    }

    fn predict_pickup(&mut self, i: usize, button: i8) {
        // Нельзя положить курсор в слот, который не принимает предмет (клик ничего не делает);
        // в слот брони стек >1 кладёт сервер (максимум 1) — не предсказываем.
        if let Some(c) = &self.cursor {
            if !self.may_place(i, c) {
                return;
            }
            if self.open.is_none() && (ARMOR_START..ARMOR_START + 4).contains(&i) && c.count > 1 {
                return;
            }
        }
        let mut cursor = self.cursor.take();
        let slots = self.menu_slots_mut();
        let Some(cell) = slots.get_mut(i) else {
            self.cursor = cursor;
            return;
        };
        match (cell.take(), cursor.take()) {
            (None, None) => {}
            (Some(mut s), None) => {
                let take = if button == 0 {
                    s.count
                } else {
                    (s.count + 1) / 2
                };
                let mut c = s.clone();
                c.count = take;
                s.count -= take;
                *cell = (s.count > 0).then_some(s);
                cursor = Some(c);
            }
            (None, Some(mut c)) => {
                let put = if button == 0 { c.count } else { 1 }.min(stack_max(&c));
                let mut s = c.clone();
                s.count = put;
                c.count -= put;
                *cell = Some(s);
                cursor = (c.count > 0).then_some(c);
            }
            (Some(mut s), Some(mut c)) => {
                if same_item(&s, &c) {
                    let room = (stack_max(&s) - s.count).max(0);
                    let mv = if button == 0 { c.count } else { 1 }.min(room);
                    s.count += mv;
                    c.count -= mv;
                    cursor = (c.count > 0).then_some(c);
                    *cell = Some(s);
                } else if c.count <= stack_max(&c) {
                    *cell = Some(c);
                    cursor = Some(s);
                } else {
                    *cell = Some(s);
                    cursor = Some(c);
                }
            }
        }
        self.cursor = cursor;
    }

    fn predict_pickup_all(&mut self) {
        let Some(mut c) = self.cursor.take() else {
            return;
        };
        let max = stack_max(&c);
        let skip0 = self.open.is_none();
        let slots = self.menu_slots_mut();
        for full_pass in [false, true] {
            for cell in slots
                .iter_mut()
                .enumerate()
                .filter(|(k, _)| !(skip0 && *k == CRAFT_OUTPUT))
                .map(|(_, c)| c)
            {
                let Some(s) = cell.as_mut() else { continue };
                if !same_item(s, &c) || (s.count >= max) != full_pass {
                    continue;
                }
                let mv = s.count.min(max - c.count);
                if mv <= 0 {
                    continue;
                }
                s.count -= mv;
                c.count += mv;
                if s.count == 0 {
                    *cell = None;
                }
            }
        }
        self.cursor = Some(c);
    }

    /// Раздача стека при перетаскивании: (слот, сколько стало) для каждого слота.
    /// VANILLA: AbstractContainerMenu#doClick QUICK_CRAFT (тип 0 — поровну, 1 — по одному).
    pub fn quick_craft_result(&self, slots: &[usize], ty: u8) -> Vec<(usize, ItemStack)> {
        let Some(c) = self.cursor.as_ref() else {
            return Vec::new();
        };
        let cells = match &self.open {
            Some(o) => &o.slots,
            None => &self.slots,
        };
        // VANILLA: AbstractContainerMenu#getQuickCraftPlaceCount (0 поровну, 1 по одному, 2 максимум).
        let each = match ty {
            1 => 1,
            2 => stack_max(c),
            _ => c.count / slots.len().max(1) as i32,
        };
        let mut left = c.count;
        let mut out = Vec::new();
        for &i in slots {
            // VANILLA: AbstractContainerMenu#canItemQuickReplace + Slot#mayPlace (броня — max 1).
            if is_output(self.open.is_some(), i)
                || !self.may_place(i, c)
                || (self.open.is_none() && (ARMOR_START..ARMOR_START + 4).contains(&i))
            {
                continue;
            }
            let Some(cell) = cells.get(i) else { continue };
            let have = match cell {
                None => 0,
                Some(s) if same_item(s, c) => s.count,
                Some(_) => continue,
            };
            let put = if ty == 2 {
                each.min(stack_max(c) - have)
            } else {
                each.min(stack_max(c) - have).min(left)
            };
            if put <= 0 {
                continue;
            }
            left = (left - put).max(0);
            let mut n = c.clone();
            n.count = have + put;
            out.push((i, n));
        }
        out
    }

    /// Применить раздачу перетаскивания (после отправки пакетов).
    pub fn predict_quick_craft(&mut self, slots: &[usize], ty: u8) {
        let res = self.quick_craft_result(slots, ty);
        let placed: i32 = {
            let cells = match &self.open {
                Some(o) => &o.slots,
                None => &self.slots,
            };
            res.iter()
                .map(|(i, n)| n.count - cells[*i].as_ref().map_or(0, |s| s.count))
                .sum()
        };
        let cells = self.menu_slots_mut();
        for (i, n) in res {
            cells[i] = Some(n);
        }
        if let Some(c) = self.cursor.as_mut() {
            c.count -= placed;
            if c.count <= 0 {
                self.cursor = None;
            }
        }
        if self.open.is_some() {
            self.mirror_player_slots();
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    #[test]
    fn content_then_slot() {
        let mut w = Writer::new();
        w.var_int(0).var_int(5).var_int(46);
        for i in 0..46 {
            if i == 36 {
                w.var_int(3).var_int(1).var_int(0).var_int(0);
            } else {
                w.var_int(0);
            }
        }
        w.var_int(0);
        let ev = parse_packet("container_set_content", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut inv = Inventory::default();
        inv.apply(ev);
        assert_eq!(inv.state_id, 5);
        assert_eq!(inv.held().map(|s| s.count), Some(3));
        let mut w = Writer::new();
        w.var_int(0).var_int(6).i16(36).var_int(0);
        inv.apply(
            parse_packet("container_set_slot", &w.into_vec())
                .unwrap()
                .unwrap(),
        );
        assert!(inv.held().is_none());
        assert_eq!(menu_slot_for_inventory_index(39), Some(5));
    }

    fn st(item: u32, count: i32) -> Slot {
        Some(ItemStack {
            item,
            count,
            patch: Default::default(),
        })
    }

    fn player_inv() -> Inventory {
        Inventory::default()
    }

    #[test]
    fn swap_hotbar_and_throw() {
        let mut inv = player_inv();
        inv.slots[10] = st(1, 5);
        inv.slots[HOTBAR_START + 2] = st(2, 3);
        inv.predict_click(10, 2, 2);
        assert_eq!(inv.slots[10], st(2, 3));
        assert_eq!(inv.slots[HOTBAR_START + 2], st(1, 5));
        inv.predict_click(10, 0, 4);
        assert_eq!(inv.slots[10], st(2, 2));
        inv.predict_click(10, 1, 4);
        assert!(inv.slots[10].is_none());
    }

    #[test]
    fn quick_move_main_to_hotbar_merges() {
        let mut inv = player_inv();
        inv.slots[12] = st(1, 40);
        inv.slots[HOTBAR_START] = st(1, 40);
        inv.predict_click(12, 0, 1);
        let max = stack_max(inv.slots[HOTBAR_START].as_ref().unwrap());
        let total = inv.slots[12].as_ref().map_or(0, |s| s.count)
            + inv.slots[HOTBAR_START..OFFHAND]
                .iter()
                .flatten()
                .map(|s| s.count)
                .sum::<i32>();
        assert_eq!(total, 80);
        assert_eq!(inv.slots[HOTBAR_START].as_ref().unwrap().count, max.min(80));
    }
}
