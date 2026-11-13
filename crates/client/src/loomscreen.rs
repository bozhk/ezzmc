//! Экран ткацкого станка: список узоров, скроллбар, превью результата.
//! VANILLA: LoomScreen, LoomMenu (data[0] = selectedBannerPatternIndex, slotsChanged → selectablePatterns),
//! теги banner_pattern/no_item_required и ItemTags#LOOM_PATTERNS (provides_banner_patterns).
//! VANILLA-UNSURE: превью результата рисуется плоским фронтом флага (UV 1..21 × 1..41 текстуры 64×64) без 3D-модели/освещения.
use crate::inventory::{Inventory, OpenMenu};
use crate::netthread::NetCmd;
use ezzmc_protocol::generated::r26_3::{DATA_COMPONENT_TYPE, ITEM};
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use std::sync::Mutex;

/// Реестр `minecraft:banner_pattern` сервера (порядок = holder id). Заполняется из Configuration.
pub static REG: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Слоты меню: banner, dye, pattern, result.
const BANNER: usize = 0;
const DYE: usize = 1;
const PATTERN: usize = 2;
const RESULT: usize = 3;

/// VANILLA: data/minecraft/tags/banner_pattern/no_item_required.json (порядок важен — индекс уходит серверу).
const NO_ITEM: [&str; 32] = [
    "square_bottom_left",
    "square_bottom_right",
    "square_top_left",
    "square_top_right",
    "stripe_bottom",
    "stripe_top",
    "stripe_left",
    "stripe_right",
    "stripe_center",
    "stripe_middle",
    "stripe_downright",
    "stripe_downleft",
    "small_stripes",
    "cross",
    "straight_cross",
    "triangle_bottom",
    "triangle_top",
    "triangles_bottom",
    "triangles_top",
    "diagonal_left",
    "diagonal_up_right",
    "diagonal_up_left",
    "diagonal_right",
    "circle",
    "rhombus",
    "half_vertical",
    "half_horizontal",
    "half_vertical_right",
    "half_horizontal_bottom",
    "border",
    "gradient",
    "gradient_up",
];

/// VANILLA: provides_banner_patterns предметов-узоров.
fn item_pattern(item: &str) -> Option<&'static str> {
    Some(match item {
        "bordure_indented_banner_pattern" => "curly_border",
        "creeper_banner_pattern" => "creeper",
        "field_masoned_banner_pattern" => "bricks",
        "flower_banner_pattern" => "flower",
        "flow_banner_pattern" => "flow",
        "globe_banner_pattern" => "globe",
        "guster_banner_pattern" => "guster",
        "mojang_banner_pattern" => "mojang",
        "piglin_banner_pattern" => "piglin",
        "skull_banner_pattern" => "skull",
        _ => return None,
    })
}

fn name_of(st: &ItemStack) -> &'static str {
    ITEM.get(st.item as usize).copied().unwrap_or("")
}

fn comp<'a>(st: &'a ItemStack, name: &str) -> Option<&'a [u8]> {
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == name)? as u32;
    st.patch
        .added
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, b)| &b[..])
}

fn slot(o: &OpenMenu, i: usize) -> Option<&ItemStack> {
    o.slots.get(i)?.as_ref()
}

/// Слои компонента banner_patterns: (имя узора, индекс DyeColor). VANILLA: BannerPatternLayers#STREAM_CODEC.
pub fn layers(st: &ItemStack) -> Vec<(String, usize)> {
    let Some(b) = comp(st, "banner_patterns") else {
        return Vec::new();
    };
    let reg = REG.lock().map(|g| g.clone()).unwrap_or_default();
    let mut r = ezzmc_protocol::wire::Reader::new(b);
    let mut out = Vec::new();
    let Ok(n) = r.var_int() else { return out };
    for _ in 0..n.max(0) {
        let Ok(h) = r.var_int() else { break };
        let name = if h == 0 {
            // inline BannerPattern: assetId + translationKey
            let (Ok(id), Ok(_)) = (r.string(256), r.string(256)) else {
                break;
            };
            id.strip_prefix("minecraft:").unwrap_or(&id).to_owned()
        } else {
            reg.get((h - 1) as usize)
                .map(|s| s.strip_prefix("minecraft:").unwrap_or(s).to_owned())
                .unwrap_or_default()
        };
        let Ok(c) = r.var_int() else { break };
        out.push((name, c.max(0) as usize));
    }
    out
}

/// Число слоёв без разбора реестра (достаточно первого VarInt).
fn layer_count(st: &ItemStack) -> usize {
    comp(st, "banner_patterns")
        .and_then(|b| ezzmc_protocol::wire::Reader::new(b).var_int().ok())
        .map_or(0, |n| n.max(0) as usize)
}

/// Цвет красителя в слоте. VANILLA: DataComponents.DYE (только `<цвет>_dye`), по умолчанию white.
fn dye_name(st: Option<&ItemStack>) -> &'static str {
    let n = st.map_or("", name_of);
    crate::banner::DYES
        .iter()
        .map(|d| d.0)
        .find(|c| n.strip_suffix("_dye") == Some(*c))
        .unwrap_or("white")
}

fn max_patterns(o: &OpenMenu) -> bool {
    slot(o, BANNER).is_some_and(|b| layer_count(b) >= 6)
}

/// VANILLA: LoomMenu#slotsChanged — список доступных узоров.
fn selectable(o: &OpenMenu) -> Vec<&'static str> {
    let (Some(_), Some(_)) = (slot(o, BANNER), slot(o, DYE)) else {
        return Vec::new();
    };
    match slot(o, PATTERN) {
        None => NO_ITEM.to_vec(),
        Some(p) => item_pattern(name_of(p)).into_iter().collect(),
    }
}

fn display_patterns(o: &OpenMenu) -> bool {
    slot(o, BANNER).is_some()
        && slot(o, DYE).is_some()
        && !max_patterns(o)
        && !selectable(o).is_empty()
}

fn rows(n: usize) -> usize {
    n.div_ceil(4)
}

fn start_row(o: &OpenMenu, n: usize) -> usize {
    let off = rows(n).saturating_sub(4);
    ((o.scroll * off as f32 + 0.5) as usize).min(off)
}

fn scroll_active(o: &OpenMenu) -> bool {
    display_patterns(o) && selectable(o).len() > 16
}

/// Ключи текстур флага: «lmb_<имя>» из `textures/entity/banner/<имя>.png`.
pub fn texture_names() -> impl Iterator<Item = &'static str> {
    crate::banner::PATTERNS
        .iter()
        .map(|p| p.0)
        .chain(std::iter::once("banner_base"))
}

/// Передний фасад флага в текстуре 64×64. VANILLA: BannerFlagModel (куб 20×40×1 в (1,1)).
const FRONT: [f32; 4] = [1.0 / 64.0, 1.0 / 64.0, 21.0 / 64.0, 41.0 / 64.0];

fn argb(c: u32) -> u32 {
    0xFF00_0000 | c
}

#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(p: &mut GuiPainter, o: &OpenMenu, l: f32, t: f32, mx: f32, my: f32) {
    let full = [0.0, 0.0, 1.0, 1.0];
    for (i, key, x, y) in [
        (BANNER, "loom_slot_banner", 13.0, 26.0),
        (DYE, "loom_slot_dye", 33.0, 26.0),
        (PATTERN, "loom_slot_pattern", 23.0, 45.0),
    ] {
        if slot(o, i).is_none() {
            p.blit(key, l + x, t + y, 16.0, 16.0, full, 0xFFFF_FFFF);
        }
    }
    let list = selectable(o);
    let active = scroll_active(o);
    let sy = (41.0 * o.scroll).floor();
    p.blit(
        if active {
            "loom_scroller"
        } else {
            "loom_scroller_off"
        },
        l + 119.0,
        t + 13.0 + sy,
        12.0,
        15.0,
        full,
        0xFFFF_FFFF,
    );
    let has_max = max_patterns(o);
    if has_max {
        p.blit(
            "loom_error",
            l + 138.0,
            t + 52.0,
            26.0,
            26.0,
            full,
            0xFFFF_FFFF,
        );
    } else if let Some(res) = slot(o, RESULT) {
        // Превью: флаг 20×40 в (141,8). База + слои узора.
        let base = name_of(res)
            .strip_suffix("_banner")
            .and_then(|c| crate::banner::DYES.iter().find(|d| d.0 == c))
            .map_or(0xFFFFFF, |d| d.1);
        p.blit(
            "lmb_banner_base",
            l + 141.0,
            t + 8.0,
            20.0,
            40.0,
            FRONT,
            argb(base),
        );
        for (name, c) in layers(res) {
            let col = crate::banner::DYES.get(c).map_or(0xFFFFFF, |d| d.1);
            p.blit(
                &format!("lmb_{name}"),
                l + 141.0,
                t + 8.0,
                20.0,
                40.0,
                FRONT,
                argb(col),
            );
        }
    }
    if !display_patterns(o) {
        return;
    }
    let sr = start_row(o, list.len());
    let sel = i32::from(o.data[0]);
    for row in 0..4 {
        for col in 0..4 {
            let index = (row + sr) * 4 + col;
            let Some(name) = list.get(index) else { return };
            let (x, y) = (l + 60.0 + col as f32 * 14.0, t + 13.0 + row as f32 * 14.0);
            let hi = mx >= x && my >= y && mx < x + 14.0 && my < y + 14.0;
            let bg = if index as i32 == sel {
                "loom_pat_sel"
            } else if hi {
                "loom_pat_hi"
            } else {
                "loom_pat"
            };
            p.blit(bg, x, y, 14.0, 14.0, full, 0xFFFF_FFFF);
            // VANILLA: extractBannerOnButton — серая заливка 5×10 и кусок текстуры узора (21×40 из 64×64).
            p.blit("white", x + 4.0, y + 2.0, 5.0, 10.0, full, argb(4673362));
            p.blit(
                &format!("lmb_{name}"),
                x + 4.0,
                y + 2.0,
                5.0,
                10.0,
                [0.0, 1.0 / 64.0, 21.0 / 64.0, 41.0 / 64.0],
                0xFFFF_FFFF,
            );
        }
    }
}

/// Подсказка кнопки узора. VANILLA: LoomScreen#extractBackground (setTooltipForNextFrame).
pub fn hover(
    lang: &crate::ui::lang::Lang,
    o: &OpenMenu,
    l: f32,
    t: f32,
    mx: f32,
    my: f32,
) -> Option<Vec<(String, u32)>> {
    if !display_patterns(o) {
        return None;
    }
    let list = selectable(o);
    let sr = start_row(o, list.len());
    for row in 0..4 {
        for col in 0..4 {
            let index = (row + sr) * 4 + col;
            let name = list.get(index)?;
            let (x, y) = (l + 60.0 + col as f32 * 14.0, t + 13.0 + row as f32 * 14.0);
            if mx >= x
                && my >= y
                && mx < x + 14.0
                && my < y + 14.0
                && index as i32 != i32::from(o.data[0])
            {
                let key = format!("block.minecraft.banner.{name}.{}", dye_name(slot(o, DYE)));
                return Some(vec![(lang.tr(&key), 0xFFFF_FFFF)]);
            }
        }
    }
    None
}

pub enum Act {
    Pass,
    Handled,
}

/// VANILLA: LoomScreen#mouseClicked — выбор узора (ContainerButton) и захват скроллбара.
pub fn click(inv: &mut Inventory, send: &mut Vec<NetCmd>, l: f32, t: f32, mx: f32, my: f32) -> Act {
    let Some(o) = inv.open.as_mut() else {
        return Act::Pass;
    };
    if !display_patterns(o) {
        return Act::Pass;
    }
    let list = selectable(o);
    let sr = start_row(o, list.len());
    for row in 0..4 {
        for col in 0..4 {
            let index = (row + sr) * 4 + col;
            let (x, y) = (l + 60.0 + col as f32 * 14.0, t + 13.0 + row as f32 * 14.0);
            if index < list.len() && mx >= x && my >= y && mx < x + 14.0 && my < y + 14.0 {
                // LoomMenu#clickMenuButton: индекс в пределах списка, локально ставим выбор.
                o.data[0] = index as i16;
                send.push(NetCmd::ContainerButton(o.id, index as i32));
                return Act::Handled;
            }
        }
    }
    let (sx, sy) = (l + 119.0, t + 9.0);
    if mx >= sx && mx < sx + 12.0 && my >= sy && my < sy + 56.0 {
        o.scrolling = true;
    }
    Act::Pass
}

/// VANILLA: LoomScreen#mouseDragged. Возвращает true, если перетаскивание скроллбара активно.
pub fn drag(inv: &mut Inventory, t: f32, my: f32) -> bool {
    let Some(o) = inv.open.as_mut() else {
        return false;
    };
    let off = rows(selectable(o).len()) as i32 - 4;
    if o.scrolling && display_patterns(o) && off > 0 {
        let y0 = t + 13.0;
        o.scroll = ((my - y0 - 7.5) / (56.0 - 15.0)).clamp(0.0, 1.0);
        return true;
    }
    false
}

pub fn release(inv: &mut Inventory) {
    if let Some(o) = inv.open.as_mut() {
        o.scrolling = false;
    }
}

/// VANILLA: LoomScreen#mouseScrolled.
pub fn wheel(inv: &mut Inventory, dy: f32) {
    let Some(o) = inv.open.as_mut() else { return };
    let off = rows(selectable(o).len()) as i32 - 4;
    if display_patterns(o) && off > 0 {
        o.scroll = (o.scroll - dy / off as f32).clamp(0.0, 1.0);
    }
}
