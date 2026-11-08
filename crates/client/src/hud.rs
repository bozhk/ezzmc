//! HUD: хотбар, иконки предметов, количество.
//! VANILLA: Gui#renderItemHotbar (hud/hotbar, hud/hotbar_selection), GuiGraphics#renderItem/renderItemDecorations.
use crate::inventory::Inventory;
use ezzmc_protocol::item::ItemStack;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const SPRITES: &[(&str, &str)] = &[
    ("hotbar", "sprites/hud/hotbar"),
    ("hotbar_sel", "sprites/hud/hotbar_selection"),
    ("container_inv", "container/inventory"),
    ("container_54", "container/generic_54"),
];

/// Имя текстуры иконки в рисовалке.
fn icon_key(name: &str) -> String {
    format!("item:{name}")
}

/// Загрузить иконку предмета, если ещё нет.
// TODO(234): блоки — 3D-куб из модели (gui light), пока плоская текстура block/<name>.
fn ensure_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    name: &str,
) -> bool {
    let key = icon_key(name);
    if p.has(&key) {
        return true;
    }
    for dir in ["item", "block"] {
        let path = format!("assets/minecraft/textures/{dir}/{name}.png");
        if let Some(img) = rm
            .read(&path)
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        {
            // Анимированные полосы: берём первый кадр (квадрат сверху).
            let side = img.width;
            let rgba = img.to_rgba8();
            let h = side.min(img.height);
            p.add_texture(dev, q, &key, &rgba[..(side * h * 4) as usize], side, h, false);
            return true;
        }
    }
    false
}

/// Нарисовать хотбар в нижней части экрана. `gw/gh` — размер в GUI-пикселях.
#[allow(clippy::too_many_arguments)]
pub fn draw_hotbar(
    p: &mut GuiPainter,
    t: &mut TextRenderer,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    inv: &Inventory,
    gw: f32,
    gh: f32,
) {
    let x0 = (gw / 2.0 - 91.0).floor();
    let y0 = gh - 22.0;
    p.blit("hotbar", x0, y0, 182.0, 22.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
    p.blit(
        "hotbar_sel",
        x0 - 1.0 + f32::from(inv.selected) * 20.0,
        y0 - 1.0,
        24.0,
        23.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFF_FFFF,
    );
    for (i, s) in inv.hotbar().iter().enumerate() {
        let Some(s) = s else { continue };
        let x = x0 + 3.0 + i as f32 * 20.0;
        let y = y0 + 3.0;
        stack_icon(p, dev, q, rm, s, x, y);
        stack_count(t, s, x, y);
    }
}

/// Иконка стека в (x, y) — левый верхний угол 16×16.
pub fn stack_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    s: &ItemStack,
    x: f32,
    y: f32,
) {
    let Some(name) = ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize) else {
        return;
    };
    if ensure_icon(p, dev, q, rm, name) {
        p.blit(&icon_key(name), x, y, 16.0, 16.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
    }
}

/// VANILLA: GuiGraphics#renderItemDecorations — количество справа внизу, с тенью.
pub fn stack_count(t: &mut TextRenderer, s: &ItemStack, x: f32, y: f32) {
    if s.count != 1 {
        let txt = s.count.to_string();
        t.draw(&txt, x + 17.0 - t.text_width(&txt), y + 9.0, 0xFFFFFF, true);
    }
}
