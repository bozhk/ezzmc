//! HUD: хотбар, иконки предметов, количество.
//! VANILLA: Gui#renderItemHotbar (hud/hotbar, hud/hotbar_selection), GuiGraphics#renderItem/renderItemDecorations.
use crate::inventory::Inventory;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const SPRITES: &[(&str, &str)] = &[
    ("slot_helmet", "sprites/container/slot/helmet"),
    ("slot_chestplate", "sprites/container/slot/chestplate"),
    ("slot_leggings", "sprites/container/slot/leggings"),
    ("slot_boots", "sprites/container/slot/boots"),
    ("slot_shield", "sprites/container/slot/shield"),
    (
        "bundle_bar_border",
        "sprites/container/bundle/bundle_progressbar_border",
    ),
    (
        "bundle_bar_fill",
        "sprites/container/bundle/bundle_progressbar_fill",
    ),
    (
        "bundle_bar_full",
        "sprites/container/bundle/bundle_progressbar_full",
    ),
    (
        "bundle_slot_hl_back",
        "sprites/container/bundle/slot_highlight_back",
    ),
    (
        "bundle_slot_hl_front",
        "sprites/container/bundle/slot_highlight_front",
    ),
    ("bundle_slot_bg", "sprites/container/bundle/slot_background"),
    ("crosshair", "sprites/hud/crosshair"),
    ("atk_full", "sprites/hud/crosshair_attack_indicator_full"),
    (
        "atk_bg",
        "sprites/hud/crosshair_attack_indicator_background",
    ),
    (
        "atk_progress",
        "sprites/hud/crosshair_attack_indicator_progress",
    ),
    (
        "hotbar_atk_bg",
        "sprites/hud/hotbar_attack_indicator_background",
    ),
    (
        "hotbar_atk_progress",
        "sprites/hud/hotbar_attack_indicator_progress",
    ),
    ("hotbar", "sprites/hud/hotbar"),
    ("hotbar_sel", "sprites/hud/hotbar_selection"),
    ("container_inv", "container/inventory"),
    ("container_54", "container/generic_54"),
    ("container_crafting", "container/crafting_table"),
    ("lit_furnace", "sprites/container/furnace/lit_progress"),
    ("arrow_furnace", "sprites/container/furnace/burn_progress"),
    (
        "lit_blast_furnace",
        "sprites/container/blast_furnace/lit_progress",
    ),
    (
        "arrow_blast_furnace",
        "sprites/container/blast_furnace/burn_progress",
    ),
    ("lit_smoker", "sprites/container/smoker/lit_progress"),
    ("arrow_smoker", "sprites/container/smoker/burn_progress"),
    ("container_furnace", "container/furnace"),
    ("container_blast_furnace", "container/blast_furnace"),
    ("container_smoker", "container/smoker"),
    ("container_dispenser", "container/dispenser"),
    ("container_hopper", "container/hopper"),
    ("container_shulker", "container/shulker_box"),
    ("container_anvil", "container/anvil"),
    ("anvil_field", "sprites/container/anvil/text_field"),
    (
        "anvil_field_off",
        "sprites/container/anvil/text_field_disabled",
    ),
    ("anvil_error", "sprites/container/anvil/error"),
    ("container_grindstone", "container/grindstone"),
    ("container_enchanting", "container/enchanting_table"),
    ("container_brewing", "container/brewing_stand"),
    ("container_smithing", "container/smithing"),
    ("container_cartography", "container/cartography_table"),
    ("container_stonecutter", "container/stonecutter"),
    ("container_crafter", "container/crafter"),
    ("container_beacon", "container/beacon"),
    ("container_villager", "container/villager"),
    ("villager_oos", "sprites/container/villager/out_of_stock"),
    (
        "villager_xp_bg",
        "sprites/container/villager/experience_bar_background",
    ),
    (
        "villager_xp_cur",
        "sprites/container/villager/experience_bar_current",
    ),
    (
        "villager_xp_res",
        "sprites/container/villager/experience_bar_result",
    ),
    ("villager_scroller", "sprites/container/villager/scroller"),
    (
        "villager_scroller_off",
        "sprites/container/villager/scroller_disabled",
    ),
    ("villager_arrow", "sprites/container/villager/trade_arrow"),
    (
        "villager_arrow_oos",
        "sprites/container/villager/trade_arrow_out_of_stock",
    ),
    (
        "villager_discount",
        "sprites/container/villager/discount_strikethrough",
    ),
    ("book", "book"),
    ("pg_fwd", "sprites/widget/page_forward"),
    ("pg_fwd_hl", "sprites/widget/page_forward_highlighted"),
    ("pg_back", "sprites/widget/page_backward"),
    ("pg_back_hl", "sprites/widget/page_backward_highlighted"),
    ("container_loom", "container/loom"),
    ("loom_slot_banner", "sprites/container/slot/banner"),
    ("loom_slot_dye", "sprites/container/slot/dye"),
    ("loom_slot_pattern", "sprites/container/slot/banner_pattern"),
    ("loom_scroller", "sprites/container/loom/scroller"),
    (
        "loom_scroller_off",
        "sprites/container/loom/scroller_disabled",
    ),
    ("loom_pat", "sprites/container/loom/pattern"),
    ("loom_pat_sel", "sprites/container/loom/pattern_selected"),
    ("loom_pat_hi", "sprites/container/loom/pattern_highlighted"),
    ("loom_error", "sprites/container/loom/error"),
    ("bcn_btn", "sprites/container/beacon/button"),
    ("bcn_btn_off", "sprites/container/beacon/button_disabled"),
    ("bcn_btn_sel", "sprites/container/beacon/button_selected"),
    ("bcn_btn_hi", "sprites/container/beacon/button_highlighted"),
    ("bcn_confirm", "sprites/container/beacon/confirm"),
    ("bcn_cancel", "sprites/container/beacon/cancel"),
    (
        "ench_slot",
        "sprites/container/enchanting_table/enchantment_slot",
    ),
    (
        "ench_slot_off",
        "sprites/container/enchanting_table/enchantment_slot_disabled",
    ),
    (
        "ench_slot_hi",
        "sprites/container/enchanting_table/enchantment_slot_highlighted",
    ),
    ("ench_lvl_1", "sprites/container/enchanting_table/level_1"),
    ("ench_lvl_2", "sprites/container/enchanting_table/level_2"),
    ("ench_lvl_3", "sprites/container/enchanting_table/level_3"),
    (
        "ench_lvl_1_off",
        "sprites/container/enchanting_table/level_1_disabled",
    ),
    (
        "ench_lvl_2_off",
        "sprites/container/enchanting_table/level_2_disabled",
    ),
    (
        "ench_lvl_3_off",
        "sprites/container/enchanting_table/level_3_disabled",
    ),
    ("brew_fuel", "sprites/container/brewing_stand/fuel_length"),
    (
        "brew_progress",
        "sprites/container/brewing_stand/brew_progress",
    ),
    ("brew_bubbles", "sprites/container/brewing_stand/bubbles"),
    ("ct_items", "container/creative_inventory/tab_items"),
    ("ct_search", "container/creative_inventory/tab_item_search"),
    ("ct_inventory", "container/creative_inventory/tab_inventory"),
    (
        "ct_top_sel_7",
        "sprites/container/creative_inventory/tab_top_selected_7",
    ),
    (
        "ct_top_7",
        "sprites/container/creative_inventory/tab_top_unselected_7",
    ),
    (
        "ct_bot_sel_7",
        "sprites/container/creative_inventory/tab_bottom_selected_7",
    ),
    (
        "ct_bot_7",
        "sprites/container/creative_inventory/tab_bottom_unselected_7",
    ),
    (
        "ct_scroller",
        "sprites/container/creative_inventory/scroller",
    ),
    (
        "ct_scroller_off",
        "sprites/container/creative_inventory/scroller_disabled",
    ),
    (
        "ct_top_sel_1",
        "sprites/container/creative_inventory/tab_top_selected_1",
    ),
    (
        "ct_top_1",
        "sprites/container/creative_inventory/tab_top_unselected_1",
    ),
    (
        "ct_bot_sel_1",
        "sprites/container/creative_inventory/tab_bottom_selected_1",
    ),
    (
        "ct_bot_1",
        "sprites/container/creative_inventory/tab_bottom_unselected_1",
    ),
    (
        "ct_top_sel_2",
        "sprites/container/creative_inventory/tab_top_selected_2",
    ),
    (
        "ct_top_2",
        "sprites/container/creative_inventory/tab_top_unselected_2",
    ),
    (
        "ct_bot_sel_2",
        "sprites/container/creative_inventory/tab_bottom_selected_2",
    ),
    (
        "ct_bot_2",
        "sprites/container/creative_inventory/tab_bottom_unselected_2",
    ),
    (
        "ct_top_sel_3",
        "sprites/container/creative_inventory/tab_top_selected_3",
    ),
    (
        "ct_top_3",
        "sprites/container/creative_inventory/tab_top_unselected_3",
    ),
    (
        "ct_bot_sel_3",
        "sprites/container/creative_inventory/tab_bottom_selected_3",
    ),
    (
        "ct_bot_3",
        "sprites/container/creative_inventory/tab_bottom_unselected_3",
    ),
    (
        "ct_top_sel_4",
        "sprites/container/creative_inventory/tab_top_selected_4",
    ),
    (
        "ct_top_4",
        "sprites/container/creative_inventory/tab_top_unselected_4",
    ),
    (
        "ct_bot_sel_4",
        "sprites/container/creative_inventory/tab_bottom_selected_4",
    ),
    (
        "ct_bot_4",
        "sprites/container/creative_inventory/tab_bottom_unselected_4",
    ),
    (
        "ct_top_sel_5",
        "sprites/container/creative_inventory/tab_top_selected_5",
    ),
    (
        "ct_top_5",
        "sprites/container/creative_inventory/tab_top_unselected_5",
    ),
    (
        "ct_bot_sel_5",
        "sprites/container/creative_inventory/tab_bottom_selected_5",
    ),
    (
        "ct_bot_5",
        "sprites/container/creative_inventory/tab_bottom_unselected_5",
    ),
    (
        "ct_top_sel_6",
        "sprites/container/creative_inventory/tab_top_selected_6",
    ),
    (
        "ct_top_6",
        "sprites/container/creative_inventory/tab_top_unselected_6",
    ),
];

/// Имя текстуры иконки в рисовалке.
fn icon_key(name: &str, cx: &crate::itemicon::Ctx) -> String {
    format!(
        "item:{name}:{:?}:{:?}:{:?}:{:?}:{:?}",
        cx.potion, cx.dye, cx.clock, cx.compass, cx.lode
    )
}

static BAKES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
const BAKES_PER_FRAME: u32 = 6;

/// Начало кадра GUI: сброс счётчика выпечки иконок.
pub fn new_frame() {
    BAKES.store(0, std::sync::atomic::Ordering::Relaxed);
}

/// Загрузить иконку предмета, если ещё нет.
// TODO(234): блоки — 3D-куб из модели (gui light), пока плоская текстура block/<name>.
fn ensure_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    name: &str,
    cx: &crate::itemicon::Ctx,
) -> bool {
    let key = icon_key(name, cx);
    if p.has(&key) {
        return true;
    }
    // Бюджет выпечки на кадр: иначе открытие креатива (~60 иконок × 2 мс) даёт микрофриз.
    if BAKES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= BAKES_PER_FRAME {
        return false;
    }
    if let Some(ic) = crate::itemicon::bake(rm, name, cx) {
        p.add_texture(dev, q, &key, &ic.rgba, ic.w, ic.h, ic.w > 32);
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
            p.add_texture(
                dev,
                q,
                &key,
                &rgba[..(side * h * 4) as usize],
                side,
                h,
                false,
            );
            return true;
        }
    }
    false
}

/// VANILLA: Hud#extractCrosshair — спрайт 15×15 по центру (целочисленное деление), инверсный бленд;
/// индикатор атаки под прицелом. Спектатор: только если цель интерактивна (`has_target`).
/// `strength` — Player#getAttackStrengthScale; `living_target` — цель-сущность LivingEntity; `delay` — тиков на атаку.
#[allow(clippy::too_many_arguments)]
pub fn draw_crosshair(
    p: &mut GuiPainter,
    gw: f32,
    gh: f32,
    game_mode: i8,
    has_target: bool,
    strength: f32,
    living_target: bool,
    delay: f32,
) {
    if game_mode == 3 && !has_target {
        return;
    }
    let (x, y) = (((gw - 15.0) / 2.0).floor(), ((gh - 15.0) / 2.0).floor());
    p.blit_inverted("crosshair", x, y, 15.0, 15.0, [0.0, 0.0, 1.0, 1.0]);
    let (ax, ay) = ((gw / 2.0).floor() - 8.0, (gh / 2.0).floor() - 7.0 + 16.0);
    if living_target && strength >= 1.0 && delay > 5.0 {
        p.blit_inverted("atk_full", ax, ay, 16.0, 16.0, [0.0, 0.0, 1.0, 1.0]);
    } else if strength < 1.0 {
        let progress = (strength * 17.0) as i32;
        p.blit_inverted("atk_bg", ax, ay, 16.0, 4.0, [0.0, 0.0, 1.0, 1.0]);
        if progress > 0 {
            let w = progress.min(16) as f32;
            p.blit_inverted("atk_progress", ax, ay, w, 4.0, [0.0, 0.0, w / 16.0, 1.0]);
        }
    }
}

/// VANILLA: Hud#extractItemHotbar — индикатор атаки рядом с хотбаром (attackIndicator = HOTBAR):
/// справа, при левой основной руке — слева; заполнение снизу вверх, 19 шагов.
pub fn draw_hotbar_attack(p: &mut GuiPainter, gw: f32, gh: f32, strength: f32, left_handed: bool) {
    if strength >= 1.0 {
        return;
    }
    let y = gh - 20.0;
    let cx = (gw / 2.0).floor();
    let x = if left_handed {
        cx - 91.0 - 22.0
    } else {
        cx + 91.0 + 6.0
    };
    let progress = (strength * 19.0) as i32 as f32;
    p.blit(
        "hotbar_atk_bg",
        x,
        y,
        18.0,
        18.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFF_FFFF,
    );
    if progress > 0.0 {
        p.blit(
            "hotbar_atk_progress",
            x,
            y + 18.0 - progress,
            18.0,
            progress,
            [0.0, (18.0 - progress) / 18.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
    }
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
    p.blit(
        "hotbar",
        x0,
        y0,
        182.0,
        22.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFF_FFFF,
    );
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
    if let Some(name) = ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize) {
        patch_icon(p, dev, q, rm, name, &s.patch, x, y);
        // VANILLA: GuiGraphicsExtractor#itemBar — чёрная подложка 13×2 и цветная полоса высотой 1.
        if let Some((w, rgb)) = crate::tooltip::durability_bar(name, &s.patch) {
            p.blit(
                "white",
                x + 2.0,
                y + 13.0,
                13.0,
                2.0,
                [0.0, 0.0, 1.0, 1.0],
                0xFF00_0000,
            );
            p.blit(
                "white",
                x + 2.0,
                y + 13.0,
                w as f32,
                1.0,
                [0.0, 0.0, 1.0, 1.0],
                0xFF00_0000 | rgb,
            );
        }
        // VANILLA: GuiGraphicsExtractor#itemCooldown — белая полупрозрачная плашка (Integer.MAX_VALUE).
        let cd = crate::cooldown::percent(name);
        if cd > 0.0 {
            let top = (16.0 * (1.0 - cd)).floor();
            let h = (16.0 * cd).ceil();
            p.blit(
                "white",
                x,
                y + top,
                16.0,
                h,
                [0.0, 0.0, 1.0, 1.0],
                0x7FFF_FFFF,
            );
        }
    }
}

/// Иконка предмета с патчем компонентов (тинт, зелья, блеск зачарований) в (x, y), 16×16.
/// Общая точка для хотбара/инвентаря (`stack_icon`) и сетки креатива — блеск рисуется всюду одинаково.
#[allow(clippy::too_many_arguments)]
pub fn patch_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    name: &str,
    patch: &ezzmc_protocol::item::ComponentPatch,
    x: f32,
    y: f32,
) {
    let cx = crate::itemicon::dynamic(name, crate::itemicon::ctx_of(patch));
    name_icon_ctx(p, dev, q, rm, name, &cx, x, y);
    if has_foil(name, patch) && p.has(&icon_key(name, &cx)) {
        glint_frame(p, 0.5, 0.75);
        p.glint(&icon_key(name, &cx), x, y, 16.0, 16.0);
    }
}

/// VANILLA: ItemStack#hasFoil — `enchantment_glint_override`, иначе `Item#isFoil` (есть чары;
/// компас — ещё и `lodestone_tracker`). Предметы с override=true по умолчанию — по Items.java 26.3.
pub fn has_foil(name: &str, patch: &ezzmc_protocol::item::ComponentPatch) -> bool {
    use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
    let find = |n: &str| {
        let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == n)? as u32;
        patch.added.iter().find(|(i, _)| *i == id).map(|(_, b)| b)
    };
    let removed = |n: &str| {
        DATA_COMPONENT_TYPE
            .iter()
            .position(|x| *x == n)
            .is_some_and(|id| patch.removed.contains(&(id as u32)))
    };
    if let Some(b) = find("enchantment_glint_override") {
        return b.first() == Some(&1);
    }
    if !removed("enchantment_glint_override")
        && matches!(
            name,
            "enchanted_golden_apple"
                | "experience_bottle"
                | "written_book"
                | "nether_star"
                | "enchanted_book"
                | "end_crystal"
                | "debug_stick"
        )
    {
        return true;
    }
    (name == "compass" && find("lodestone_tracker").is_some())
        || find("enchantments").is_some_and(|b| b.first().is_some_and(|n| *n > 0))
}

/// Параметры блеска кадра. VANILLA: TextureTransform#setupGlintTexturing(8) — translate(-o0,o1)·rotZ(π/18)·scale;
/// glintSpeed 0.5, glintStrength 0.75 (Options). VANILLA-UNSURE: ванильные UV — атлас предметов
/// (спрайт ≈ 1/64 атласа), у нас UV иконки 0..1, поэтому масштаб 8/64.
pub fn glint_frame(p: &mut GuiPainter, speed: f64, strength: f32) {
    let ms = ((std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)) as f64
        * speed
        * 8.0) as u64;
    let o0 = (ms % 110_000) as f32 / 110_000.0;
    let o1 = (ms % 30_000) as f32 / 30_000.0;
    let (sn, cs) = (std::f32::consts::PI / 18.0).sin_cos();
    let k = 8.0 / 64.0;
    p.set_glint_params([k * cs, k * sn, -k * sn, k * cs], [-o0, o1], strength);
}

/// Иконка предмета по имени.
pub fn name_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    name: &str,
    x: f32,
    y: f32,
) {
    name_icon_ctx(p, dev, q, rm, name, &crate::itemicon::Ctx::default(), x, y);
}

#[allow(clippy::too_many_arguments)]
pub fn name_icon_ctx(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    name: &str,
    cx: &crate::itemicon::Ctx,
    x: f32,
    y: f32,
) {
    let cx = &crate::itemicon::dynamic(name, *cx);
    if ensure_icon(p, dev, q, rm, name, cx) {
        p.blit(
            &icon_key(name, cx),
            x,
            y,
            16.0,
            16.0,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
    }
}

/// VANILLA: GuiGraphics#renderItemDecorations — количество справа внизу, с тенью.
pub fn stack_count(t: &mut TextRenderer, s: &ItemStack, x: f32, y: f32) {
    if s.count != 1 {
        let txt = s.count.to_string();
        t.draw(&txt, x + 17.0 - t.text_width(&txt), y + 9.0, 0xFFFFFF, true);
    }
}

/// VANILLA: предмет курсора рисуется поверх всего (иконка и количество выше слотов и подсветки).
#[allow(clippy::too_many_arguments)]
pub fn cursor_item(
    p: &mut GuiPainter,
    t: &mut TextRenderer,
    g: &ezzmc_render::Gpu,
    view: &wgpu::TextureView,
    size: (u32, u32),
    rm: &ResourceManager,
    s: &ItemStack,
    mx: f32,
    my: f32,
) {
    let (w, h) = size;
    t.flush(&g.device, &g.queue, view, w, h);
    t.begin(w, h);
    p.begin(t.scale());
    stack_icon(p, &g.device, &g.queue, rm, s, mx - 8.0, my - 8.0);
    p.flush(&g.device, &g.queue, view, w, h, None);
    stack_count(t, s, mx - 8.0, my - 8.0);
}

/// Значок пустого слота брони/щита InventoryMenu (5–8 броня, 45 вторая рука).
/// VANILLA: InventoryMenu (Slot#getNoItemIcon, ARMOR_SLOT_TEXTURES / EMPTY_SLOT_SHIELD).
pub fn empty_slot_icon(p: &mut GuiPainter, slot: usize, x: f32, y: f32) {
    let key = match slot {
        5 => "slot_helmet",
        6 => "slot_chestplate",
        7 => "slot_leggings",
        8 => "slot_boots",
        45 => "slot_shield",
        _ => return,
    };
    p.blit(key, x, y, 16.0, 16.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
}
