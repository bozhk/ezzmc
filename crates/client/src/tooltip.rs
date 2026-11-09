//! Подсказка предмета (TODO-235, часть): название (custom_name/item_name/lang), цвет редкости, чары, lore,
//! Unbreakable, рамка. Нет: атрибуты, прочность (только F3+H), эффекты зелий, категория креатива, tooltip_display.
//! VANILLA: GuiGraphics#renderTooltip / TooltipRenderUtil (фон 0xF0100010, рамка 0x505000FF→0x5028007F).
use crate::ui::lang::Lang;
use ezzmc_protocol::generated::r26_3::{DATA_COMPONENT_TYPE, ITEM};
use ezzmc_protocol::item::{ComponentPatch, ItemStack};
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_render::text::TextRenderer;
use std::sync::Mutex;

/// Отображаемое имя предмета по id реестра. VANILLA: Item#getDescriptionId (item./block.minecraft.*).
pub fn item_title(lang: &Lang, name: &str) -> String {
    for p in ["item", "block"] {
        let key = format!("{p}.minecraft.{name}");
        let s = lang.tr(&key);
        if s != key {
            return s;
        }
    }
    name.to_owned()
}

/// Зачарования сервера: (имя, макс. уровень) в порядке registry_data (holder id = индекс).
pub static ENCHANTS: Mutex<Vec<(String, i32)>> = Mutex::new(Vec::new());

fn comp<'a>(patch: &'a ComponentPatch, name: &str) -> Option<&'a [u8]> {
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == name)? as u32;
    patch
        .added
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, b)| &b[..])
}

fn text_of(lang: &Lang, b: &[u8]) -> Option<String> {
    let tag = nbt::read_network(&mut Reader::new(b)).ok()??;
    let c = Component::from_nbt(&tag).ok()?;
    Some(c.to_plain(&|k| {
        let s = lang.tr(k);
        (s != k).then_some(s)
    }))
}

/// VANILLA: ItemStack#getTooltipLines (часть) + getStyledHoverName/Rarity.
/// Редкость: компонент `rarity`, иначе по предмету; чары поднимают common/uncommon → rare, rare → epic.
pub fn stack_lines(lang: &Lang, s: &ItemStack) -> Vec<(String, u32)> {
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    let mut rarity = comp(&s.patch, "rarity")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or(match name {
            "golden_apple"
            | "enchanted_golden_apple"
            | "elytra"
            | "nether_star"
            | "totem_of_undying" => {
                if name == "enchanted_golden_apple" {
                    3
                } else {
                    1
                }
            }
            "experience_bottle" | "enchanted_book" | "heart_of_the_sea" | "end_crystal"
            | "dragon_head" => 1,
            "beacon" | "conduit" | "dragon_egg" | "trident" => 2,
            _ => 0,
        });
    let ench: Vec<(i32, i32)> = comp(&s.patch, "enchantments")
        .map(|b| {
            let mut r = Reader::new(b);
            let n = r.var_int().unwrap_or(0);
            (0..n)
                .map_while(|_| Some((r.var_int().ok()?, r.var_int().ok()?)))
                .collect()
        })
        .unwrap_or_default();
    if !ench.is_empty() && name != "enchanted_book" {
        rarity = match rarity {
            0 | 1 => 2,
            _ => 3,
        };
    }
    let rc = [0xFFFFFF, 0xFFFF55, 0x55FFFF, 0xFF55FF][rarity.clamp(0, 3) as usize];
    let title = comp(&s.patch, "custom_name")
        .and_then(|b| text_of(lang, b))
        .or_else(|| comp(&s.patch, "item_name").and_then(|b| text_of(lang, b)))
        .unwrap_or_else(|| item_title(lang, name));
    let mut out = vec![(title, rc)];
    let reg = ENCHANTS.lock().map(|g| g.clone()).unwrap_or_default();
    for (h, lvl) in ench {
        let Some((n, max)) = reg.get(h as usize) else {
            continue;
        };
        let short = n.strip_prefix("minecraft:").unwrap_or(n);
        let key = format!("enchantment.minecraft.{short}");
        let mut t = lang.tr(&key);
        if !(*max == 1 && lvl == 1) {
            let lk = format!("enchantment.level.{lvl}");
            let l = lang.tr(&lk);
            t = format!("{t} {}", if l == lk { lvl.to_string() } else { l });
        }
        let curse = matches!(short, "binding_curse" | "vanishing_curse");
        out.push((t, if curse { 0xFF5555 } else { 0xAAAAAA }));
    }
    if let Some(b) = comp(&s.patch, "lore") {
        let mut r = Reader::new(b);
        let n = r.var_int().unwrap_or(0);
        for _ in 0..n {
            let Ok(Some(tag)) = nbt::read_network(&mut r) else {
                break;
            };
            if let Ok(c) = Component::from_nbt(&tag) {
                out.push((
                    c.to_plain(&|k| {
                        let s = lang.tr(k);
                        (s != k).then_some(s)
                    }),
                    0xAA00AA,
                ));
            }
        }
    }
    if comp(&s.patch, "unbreakable").is_some() {
        out.push((lang.tr("item.unbreakable"), 0x5555FF));
    }
    out
}

pub fn draw_colored(
    t: &mut TextRenderer,
    lines: &[(String, u32)],
    mx: f32,
    my: f32,
    gw: f32,
    gh: f32,
) {
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|l| t.text_width(&l.0)).fold(0.0, f32::max);
    let h = if lines.len() > 1 {
        10.0 * lines.len() as f32 + 2.0
    } else {
        8.0
    };
    let mut x = mx + 12.0;
    let mut y = my - 12.0;
    if x + w + 4.0 > gw {
        x = (mx - 12.0 - w).max(4.0);
    }
    y = y.clamp(4.0, (gh - h - 4.0).max(4.0));
    t.fill(x - 3.0, y - 4.0, x + w + 3.0, y + h + 4.0, 0xF010_0010);
    t.fill(x - 3.0, y - 3.0, x - 2.0, y + h + 3.0, 0x5050_00FF);
    t.fill(x + w + 2.0, y - 3.0, x + w + 3.0, y + h + 3.0, 0x5028_007F);
    t.fill(x - 3.0, y - 3.0, x + w + 3.0, y - 2.0, 0x5050_00FF);
    t.fill(x - 3.0, y + h + 2.0, x + w + 3.0, y + h + 3.0, 0x5028_007F);
    for (i, l) in lines.iter().enumerate() {
        t.draw(
            &l.0,
            x,
            y + i as f32 * 10.0 + if i > 0 { 2.0 } else { 0.0 },
            l.1,
            true,
        );
    }
}
