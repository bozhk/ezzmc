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
    pub data: [i16; 8],
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
                    data: [0; 8],
                });
            }
            InvEvent::Data { id, prop, value } => {
                if let Some(o) = self.open.as_mut().filter(|o| o.id == id) {
                    if let Some(d) = usize::try_from(prop).ok().and_then(|i| o.data.get_mut(i)) {
                        *d = value;
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
            6 => self.predict_pickup_all(),
            _ => {}
        }
        if open {
            self.mirror_player_slots();
        }
    }

    fn predict_pickup(&mut self, i: usize, button: i8) {
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
            if is_output(self.open.is_some(), i) {
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
}
