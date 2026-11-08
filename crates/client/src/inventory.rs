//! Инвентарь игрока и открытые контейнеры: состояние слотов, разбор пакетов 26.3.
#![allow(dead_code)] // часть API ждёт экранов инвентаря (232/236)
// FROM: pomme/pomme-client/src/player/inventory.rs (раскладка слотов InventoryMenu).
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
}

/// Состояние инвентаря клиента.
#[derive(Debug)]
pub struct Inventory {
    /// Слоты InventoryMenu (контейнер 0).
    pub slots: Vec<Slot>,
    pub cursor: Slot,
    pub selected: u8,
    pub state_id: i32,
    pub open: Option<OpenMenu>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![None; PLAYER_SLOTS],
            cursor: None,
            selected: 0,
            state_id: 0,
            open: None,
        }
    }
}

impl Inventory {
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
                });
            }
            InvEvent::Close(id) => {
                if self.open.as_ref().is_some_and(|o| o.id == id) || id == 0 {
                    self.open = None;
                }
            }
        }
    }
}

#[cfg(test)]
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
