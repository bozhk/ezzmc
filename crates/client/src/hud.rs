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

/// VANILLA: Hud#extractSleepOverlay — заливка 0x101020, альфа 220·amount.
pub fn draw_sleep_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    gw: f32,
    gh: f32,
    sleep_timer: i32,
) {
    if sleep_timer <= 0 {
        return;
    }
    let t = sleep_timer as f32;
    let mut amount = t / 100.0;
    if amount > 1.0 {
        amount = 1.0 - (t - 100.0) / 10.0;
    }
    let a = (220.0 * amount) as u32;
    const KEY: &str = "white_1x1";
    if !p.has(KEY) {
        p.add_texture(dev, q, KEY, &[255; 4], 1, 1, false);
    }
    p.blit(
        KEY,
        0.0,
        0.0,
        gw,
        gh,
        [0.0, 0.0, 1.0, 1.0],
        (a << 24) | 0x0010_1020,
    );
}

/// VANILLA: Hud#extractPortalOverlay — спрайт nether_portal на весь экран, альфа по интенсивности.
#[allow(clippy::too_many_arguments)]
pub fn draw_portal_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    intensity: f32,
    frame: u32,
) {
    const KEY: &str = "nether_portal_strip";
    if !p.has(KEY) {
        let Some(img) = rm
            .read("assets/minecraft/textures/block/nether_portal.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, KEY, &img.to_rgba8(), img.width, img.height, false);
    }
    let mut a = intensity;
    if a < 1.0 {
        a *= a;
        a *= a;
        a = a * 0.8 + 0.2;
    }
    let argb = (((a.clamp(0.0, 1.0) * 255.0) as u32) << 24) | 0x00FF_FFFF;
    // Полоса 16×(16·N): кадры идут подряд сверху вниз.
    let n = 32u32;
    let f = frame % n;
    let (v0, v1) = (f as f32 / n as f32, (f + 1) as f32 / n as f32);
    p.blit(KEY, 0.0, 0.0, gw, gh, [0.0, v0, 1.0, v1], argb);
}

/// VANILLA: LevelLoadingScreen#extractBackground (NETHER_PORTAL) — `blitSprite(GUI_OPAQUE_TEXTURED_BACKGROUND,
/// nether_portal)` на весь экран. Конвейер без блендинга: альфа текстуры (155..232) не учитывается,
/// поэтому грузится копия с альфой 255. `frame` — кадр анимации (клиентский тик).
#[allow(clippy::too_many_arguments)]
pub fn draw_portal_loading_bg(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    frame: u32,
) {
    const KEY: &str = "nether_portal_opaque";
    if !p.has(KEY) {
        let Some(img) = rm
            .read("assets/minecraft/textures/block/nether_portal.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        let mut px = img.to_rgba8();
        for a in px.iter_mut().skip(3).step_by(4) {
            *a = 255;
        }
        p.add_texture(dev, q, KEY, &px, img.width, img.height, false);
    }
    let n = 32u32;
    let f = frame % n;
    let (v0, v1) = (f as f32 / n as f32, (f + 1) as f32 / n as f32);
    p.blit(KEY, 0.0, 0.0, gw, gh, [0.0, v0, 1.0, v1], 0xFFFF_FFFF);
}

/// VANILLA: ScreenEffectRenderer#submitWater — underwater.png, UV ×4 со сдвигом по взгляду, альфа 0.1·яркость.
/// Квад на весь экран с UV 4 развёрнут тайлами (GUI-сэмплер не повторяет текстуру).
#[allow(clippy::too_many_arguments)]
pub fn draw_water_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    brightness: f32,
    yaw: f32,
    pitch: f32,
) {
    const KEY: &str = "underwater_overlay";
    if !p.has(KEY) {
        let Some(img) = rm
            .read("assets/minecraft/textures/misc/underwater.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, KEY, &img.to_rgba8(), img.width, img.height, false);
    }
    let (uo, vo) = (-yaw / 64.0, pitch / 64.0);
    let b = (brightness.clamp(0.0, 1.0) * 255.0) as u32;
    let argb = (25u32 << 24) | (b << 16) | (b << 8) | b;
    // U(x) = uo + 4·(1 − x/gw), V(y) = vo + 4·y/gh; тайл = целая клетка (k..k+1, m..m+1).
    let (k0, k1) = ((uo - 0.0).floor() as i32, (uo + 4.0).ceil() as i32);
    let (m0, m1) = (vo.floor() as i32, (vo + 4.0).ceil() as i32);
    for k in k0..k1 {
        let x0 = gw * (1.0 - (k as f32 + 1.0 - uo) / 4.0);
        let x1 = gw * (1.0 - (k as f32 - uo) / 4.0);
        for m in m0..m1 {
            let y0 = gh * (m as f32 - vo) / 4.0;
            let y1 = gh * (m as f32 + 1.0 - vo) / 4.0;
            p.blit(KEY, x0, y0, x1 - x0, y1 - y0, [1.0, 0.0, 0.0, 1.0], argb);
        }
    }
}

/// VANILLA: ScreenEffectRenderer#submitFire — два квада fire_1 (0.9 альфа) на z=−0.5, сдвиг (±0.24, −0.3),
/// поворот ±10° вокруг Y. Приближение в 2D: масштаб экрана при fov 70° и z=0.5 (VANILLA-UNSURE: без перспективного перекоса).
#[allow(clippy::too_many_arguments)]
pub fn draw_fire_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    frame: u32,
) {
    const KEY: &str = "fire_overlay_strip";
    if !p.has(KEY) {
        let Some(img) = rm
            .read("assets/minecraft/textures/block/fire_1.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, KEY, &img.to_rgba8(), img.width, img.height, false);
    }
    let n = 32u32;
    let f = frame % n;
    let (v0, v1) = (f as f32 / n as f32, (f + 1) as f32 / n as f32);
    let unit = gh / (2.0 * (35.0f32.to_radians().tan() * 0.5));
    let side = unit * 10.0f32.to_radians().cos();
    for sx in [0.24f32, -0.24] {
        let cx = gw * 0.5 + sx * unit;
        let cy = gh * 0.5 + 0.3 * unit;
        p.blit(
            KEY,
            cx - side * 0.5,
            cy - unit * 0.5,
            side,
            unit,
            [1.0, v0, 0.0, v1],
            0xE5FF_FFFF,
        );
    }
}

/// VANILLA: ScreenEffectRenderer#submitBlockSprite — спрайт частицы блока на весь экран, цвет 0xFF191919.
#[allow(clippy::too_many_arguments)]
pub fn draw_block_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    atlas: &(Vec<u8>, u32, u32),
    gw: f32,
    gh: f32,
    sp: ezzmc_assets::bake::SpriteUv,
) {
    const KEY: &str = "block_atlas_overlay";
    if !p.has(KEY) {
        p.add_texture(dev, q, KEY, &atlas.0, atlas.1, atlas.2, false);
    }
    // buildQuad: левый нижний угол получает (u1, v1), правый верхний (u0, v0) → зеркало по горизонтали.
    p.blit(
        KEY,
        0.0,
        0.0,
        gw,
        gh,
        [sp.u1, sp.v0, sp.u0, sp.v1],
        0xFF19_1919,
    );
}

/// VANILLA: Hud#extractTextureOverlay — текстура misc/<name> на весь экран, белый с альфой.
#[allow(clippy::too_many_arguments)]
pub fn draw_texture_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    name: &str,
    alpha: f32,
    linear: bool,
) {
    let key = format!("overlay_{name}");
    if !p.has(&key) {
        let Some(img) = rm
            .read(&format!("assets/minecraft/textures/misc/{name}.png"))
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, &key, &img.to_rgba8(), img.width, img.height, linear);
    }
    let argb = (((alpha.clamp(0.0, 1.0) * 255.0) as u32) << 24) | 0x00FF_FFFF;
    p.blit(&key, 0.0, 0.0, gw, gh, [0.0, 0.0, 1.0, 1.0], argb);
}

/// VANILLA: Hud#extractSpyglassOverlay — квадрат scope по центру (min(w,h)·scale), остальное чёрное.
pub fn draw_spyglass_overlay(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    gw: f32,
    gh: f32,
    scale: f32,
) {
    let key = "overlay_spyglass_scope";
    if !p.has(key) {
        let Some(img) = rm
            .read("assets/minecraft/textures/misc/spyglass_scope.png")
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, key, &img.to_rgba8(), img.width, img.height, false);
    }
    let side = gw.min(gh);
    let size = (side * scale).floor();
    let left = ((gw - size) / 2.0).floor();
    let top = ((gh - size) / 2.0).floor();
    let (right, bottom) = (left + size, top + size);
    let uv = [0.0, 0.0, 1.0, 1.0];
    p.blit(key, left, top, size, size, uv, 0xFFFF_FFFF);
    let black = 0xFF00_0000;
    p.blit("white", 0.0, bottom, gw, gh - bottom, uv, black);
    p.blit("white", 0.0, 0.0, gw, top, uv, black);
    p.blit("white", 0.0, top, left, bottom - top, uv, black);
    p.blit("white", right, top, gw - right, bottom - top, uv, black);
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

/// Иконка стека уменьшенного размера `size` без декораций (оверлей вариантов книги рецептов).
#[allow(clippy::too_many_arguments)]
pub fn stack_icon_sized(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    s: &ItemStack,
    x: f32,
    y: f32,
    size: f32,
) {
    let Some(name) = ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize) else {
        return;
    };
    let cx = crate::itemicon::dynamic(name, crate::itemicon::ctx_of(&s.patch));
    if ensure_icon(p, dev, q, rm, name, &cx) {
        p.blit(
            &icon_key(name, &cx),
            x,
            y,
            size,
            size,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
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
    if s.item >= ezzmc_world::modded::MOD_ITEM_BASE {
        mod_item_icon(p, dev, q, s.item, x, y);
        return;
    }
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

/// Иконка модового предмета Forge (TODO-508): снимок `GuiGraphics#renderItem` из дампа `.ezbake` v2
/// (страница атласа иконок грузится в рисовалку при первом обращении). Нет дампа/иконки — пусто.
pub fn mod_item_icon(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    item: u32,
    x: f32,
    y: f32,
) {
    let Some(bake) = ezzmc_assets::modbake::global() else {
        return;
    };
    let Some(name) = ezzmc_world::modded::server_name(
        "minecraft:item",
        item - ezzmc_world::modded::MOD_ITEM_BASE,
    ) else {
        return;
    };
    let Some((page, ix, iy, cell)) = bake.item_by_name(&name).and_then(|i| bake.item_icon(i))
    else {
        return;
    };
    let key = format!("modicons:{page}");
    if !p.has(&key) {
        let Some(img) = bake
            .icon_page_png(page)
            .and_then(|d| ezzmc_assets::texture::Image::decode_png(d).ok())
        else {
            return;
        };
        p.add_texture(dev, q, &key, &img.to_rgba8(), img.width, img.height, false);
    }
    let k = 1.0 / bake.icon_page as f32;
    let (u, v, c) = (ix as f32 * k, iy as f32 * k, cell as f32 * k);
    p.blit(&key, x, y, 16.0, 16.0, [u, v, u + c, v + c], 0xFFFF_FFFF);
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
        glint_frame(
            p,
            f64::from(crate::armor::glint_speed()),
            ezzmc_render::particledraw::glint_strength(),
        );
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
