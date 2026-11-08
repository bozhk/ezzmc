//! Экран инвентаря/контейнера: раскладка слотов, отрисовка, клики.
//! VANILLA: InventoryScreen / ContainerScreen (generic_9xN) / AbstractContainerScreen#mouseClicked.
//! Предсказания нет: клик уходит серверу, он присылает полное состояние (TODO-232: слияние стеков
//! требует реестра предметов 229, хэши компонентов — HashedPatchMap).
use crate::hud;
use crate::inventory::Inventory;
use crate::netthread::NetCmd;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

/// VANILLA: ContainerInput.
pub const PICKUP: i32 = 0;
pub const QUICK_MOVE: i32 = 1;
pub const SWAP: i32 = 2;
pub const THROW: i32 = 4;
/// Слот «вне окна» (бросок курсора).
pub const OUTSIDE: i16 = -999;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Player,
    Chest(usize),
    Other,
}

pub struct Layout {
    kind: Kind,
    pub w: f32,
    pub h: f32,
    /// Позиция слота (None — слот не показывается).
    pub slots: Vec<Option<(f32, f32)>>,
}

fn player_inv_rows(out: &mut Vec<Option<(f32, f32)>>, main_y: f32, hot_y: f32) {
    for i in 0..27 {
        out.push(Some((8.0 + (i % 9) as f32 * 18.0, main_y + (i / 9) as f32 * 18.0)));
    }
    for c in 0..9 {
        out.push(Some((8.0 + c as f32 * 18.0, hot_y)));
    }
}

/// Раскладка для текущего состояния (открытое меню или InventoryMenu).
pub fn layout(inv: &Inventory) -> Layout {
    let Some(o) = &inv.open else {
        // VANILLA: InventoryMenu.
        let mut s = vec![Some((154.0, 28.0))];
        for i in 0..4 {
            s.push(Some((98.0 + (i % 2) as f32 * 18.0, 18.0 + (i / 2) as f32 * 18.0)));
        }
        for i in 0..4 {
            s.push(Some((8.0, 8.0 + i as f32 * 18.0)));
        }
        player_inv_rows(&mut s, 84.0, 142.0);
        s.push(Some((77.0, 62.0)));
        return Layout {
            kind: Kind::Player,
            w: 176.0,
            h: 166.0,
            slots: s,
        };
    };
    let name = ezzmc_protocol::generated::r26_3::MENU
        .get(o.menu as usize)
        .copied()
        .unwrap_or("");
    let rows = match name {
        "generic_9x1" => 1,
        "generic_9x2" => 2,
        "generic_9x3" => 3,
        "generic_9x4" => 4,
        "generic_9x5" => 5,
        "generic_9x6" => 6,
        _ => 0,
    };
    let n = o.slots.len().max(36);
    if rows > 0 && n == rows * 9 + 36 {
        // VANILLA: ContainerScreen / ChestMenu.
        let mut s = Vec::new();
        for i in 0..rows * 9 {
            s.push(Some((8.0 + (i % 9) as f32 * 18.0, 18.0 + (i / 9) as f32 * 18.0)));
        }
        let off = (rows as f32 - 4.0) * 18.0;
        player_inv_rows(&mut s, 103.0 + off, 161.0 + off);
        return Layout {
            kind: Kind::Chest(rows),
            w: 176.0,
            h: 114.0 + rows as f32 * 18.0,
            slots: s,
        };
    }
    // Неподдерживаемое меню: только инвентарь игрока (слоты контейнера скрыты).
    let mut s = vec![None; n - 36];
    player_inv_rows(&mut s, 84.0, 142.0);
    Layout {
        kind: Kind::Other,
        w: 176.0,
        h: 166.0,
        slots: s,
    }
}

impl Layout {
    pub fn left(&self, gw: f32) -> f32 {
        ((gw - self.w) / 2.0).floor()
    }
    pub fn top(&self, gh: f32) -> f32 {
        ((gh - self.h) / 2.0).floor()
    }

    /// Слот под курсором (координаты GUI).
    pub fn slot_at(&self, gw: f32, gh: f32, mx: f32, my: f32) -> Option<usize> {
        let (l, t) = (self.left(gw), self.top(gh));
        self.slots.iter().position(|p| {
            p.is_some_and(|(x, y)| mx >= l + x && mx < l + x + 16.0 && my >= t + y && my < t + y + 16.0)
        })
    }

    pub fn outside(&self, gw: f32, gh: f32, mx: f32, my: f32) -> bool {
        let (l, t) = (self.left(gw), self.top(gh));
        mx < l || my < t || mx >= l + self.w || my >= t + self.h
    }
}

/// Содержимое слота `i` активного меню.
fn slot_stack(inv: &Inventory, i: usize) -> Option<&ezzmc_protocol::item::ItemStack> {
    match &inv.open {
        Some(o) => o.slots.get(i)?.as_ref(),
        None => inv.slots.get(i)?.as_ref(),
    }
}

/// Команда клика (id контейнера и state_id берутся из инвентаря).
pub fn click_cmd(inv: &Inventory, slot: i16, button: i8, input: i32) -> NetCmd {
    NetCmd::ContainerClick {
        container: inv.open.as_ref().map_or(0, |o| o.id),
        state_id: inv.state_id,
        slot,
        button,
        input,
    }
}

/// VANILLA: AbstractContainerScreen#mouseClicked (без перетаскивания и двойного клика).
#[allow(clippy::too_many_arguments)]
pub fn on_click(
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
    right: bool,
    shift: bool,
) -> Option<NetCmd> {
    let button = i8::from(right);
    if let Some(i) = lay.slot_at(gw, gh, mx, my) {
        let input = if shift { QUICK_MOVE } else { PICKUP };
        return Some(click_cmd(inv, i as i16, button, input));
    }
    if lay.outside(gw, gh, mx, my) && inv.cursor.is_some() {
        return Some(click_cmd(inv, OUTSIDE, button, PICKUP));
    }
    None
}

/// Фон и иконки (проход painter). Курсорный стек рисуется последним.
#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) {
    let (l, t) = (lay.left(gw), lay.top(gh));
    let k = 1.0 / 256.0;
    match lay.kind {
        Kind::Chest(rows) => {
            let top_h = rows as f32 * 18.0 + 17.0;
            p.blit("container_54", l, t, 176.0, top_h, [0.0, 0.0, 176.0 * k, top_h * k], 0xFFFF_FFFF);
            p.blit(
                "container_54",
                l,
                t + top_h,
                176.0,
                96.0,
                [0.0, 126.0 * k, 176.0 * k, 222.0 * k],
                0xFFFF_FFFF,
            );
        }
        _ => p.blit("container_inv", l, t, 176.0, 166.0, [0.0, 0.0, 176.0 * k, 166.0 * k], 0xFFFF_FFFF),
    }
    for (i, pos) in lay.slots.iter().enumerate() {
        if let (Some((x, y)), Some(s)) = (pos, slot_stack(inv, i)) {
            hud::stack_icon(p, dev, q, rm, s, l + x, t + y);
        }
    }
    if let Some(s) = &inv.cursor {
        hud::stack_icon(p, dev, q, rm, s, mx - 8.0, my - 8.0);
    }
}

/// Затемнение мира под экраном (VANILLA: Screen#renderBackground → gradient 0xC0101010→0xD0101010).
pub fn draw_dim(t: &mut TextRenderer, gw: f32, gh: f32) {
    t.fill(0.0, 0.0, gw, gh, 0xC010_1010);
}

/// Подсветка, заголовок и количества (проход текста).
#[allow(clippy::too_many_arguments)]
pub fn draw_text(
    t: &mut TextRenderer,
    inv: &Inventory,
    lay: &Layout,
    lang_title: &str,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) {
    let (l, top) = (lay.left(gw), lay.top(gh));
    let title = match (&inv.open, lay.kind) {
        (None, _) => "Crafting",
        (Some(_), _) => lang_title,
    };
    // VANILLA: titleLabelX 8 / 97 для InventoryScreen, titleLabelY 6 / 8.
    let (tx, ty) = if lay.kind == Kind::Player { (97.0, 8.0) } else { (8.0, 6.0) };
    t.draw(title, l + tx, top + ty, 0x404040, false);
    for (i, pos) in lay.slots.iter().enumerate() {
        if let (Some((x, y)), Some(s)) = (pos, slot_stack(inv, i)) {
            hud::stack_count(t, s, l + x, top + y);
        }
    }
    if let Some(i) = lay.slot_at(gw, gh, mx, my) {
        if let Some((x, y)) = lay.slots[i] {
            t.fill(l + x, top + y, l + x + 16.0, top + y + 16.0, 0x80FF_FFFF);
        }
    }
    if let Some(s) = &inv.cursor {
        hud::stack_count(t, s, mx - 8.0, my - 8.0);
    }
}
