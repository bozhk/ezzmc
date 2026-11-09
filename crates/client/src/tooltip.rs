//! Подсказка предмета (TODO-235): название (custom_name/item_name/lang/зелья), цвет редкости, зелья (эффекты,
//! «When Applied:»), пластинка (описание jukebox_song), пустой бандл (описание), чары, lore, атрибуты («When in Main
//! Hand…»), Unbreakable, спаунер (без имени сущности), F3+H (прочность, id), рамка.
//! Нет: сетка и полоска бандла (визуальный компонент), имя сущности спаунера, категория креатива, tooltip_display,
//! компоненты (F3+H), OverrideText-стили.
//! VANILLA: GuiGraphics#renderTooltip / TooltipRenderUtil (фон 0xF0100010, рамка 0x505000FF→0x5028007F).
use crate::ui::lang::Lang;
use ezzmc_protocol::generated::r26_3::{DATA_COMPONENT_TYPE, ITEM, MOB_EFFECT, POTION};
use ezzmc_protocol::item::{ComponentPatch, ItemStack};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_render::text::TextRenderer;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

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

/// Реестр атрибутов (короткие имена, порядок = holder id) для attribute_modifiers. Заполняется из Configuration.
pub static ATTRIBUTES: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Реестр `minecraft:jukebox_song` (порядок = holder id): компонент `description` каждой пластинки.
pub static JUKEBOX_DESC: Mutex<Vec<Option<Tag>>> = Mutex::new(Vec::new());

/// F3+H: расширенная подсказка (прочность, id предмета). VANILLA: KeyboardHandler — F3+H → advanced tooltips.
pub static ADVANCED: AtomicBool = AtomicBool::new(false);

/// Группы слотов в порядке EquipmentSlotGroup (id 0..10). VANILLA: ItemStack#addAttributeTooltips.
const SLOT_ORDER: [&str; 11] = [
    "any", "mainhand", "offhand", "hand", "feet", "legs", "chest", "head", "armor", "body",
    "saddle",
];
const GRAY: u32 = 0xAAAAAA;
const DARK_GRAY: u32 = 0x555555;
const DARK_GREEN: u32 = 0x00AA00;
const DARK_PURPLE: u32 = 0xAA00AA;
const BLUE: u32 = 0x5555FF;
const RED: u32 = 0xFF5555;

#[derive(Clone)]
enum Disp {
    Default,
    Hidden,
    Text(String),
}

/// Модификатор атрибута: из патча стака, из дефолтов предмета или эффекта (tooltip_data.txt).
#[derive(Clone)]
struct AttrMod {
    attr: String,
    id: String,
    amount: f64,
    /// 0 add_value, 1 add_multiplied_base, 2 add_multiplied_total.
    op: i32,
    slot: String,
    display: Disp,
}

/// Эффект зелья: короткое имя, длительность в тиках (-1 = бесконечно), амплификатор.
#[derive(Clone)]
struct Eff {
    name: String,
    dur: i32,
    amp: i32,
}

/// Дефолты из `tools/gen_tooltip_data.py`: редкость, макс. прочность, модификаторы, сентименты, зелья, эффекты.
#[derive(Default)]
struct Defaults {
    max_damage: HashMap<String, i32>,
    /// Редкость по умолчанию: 0 common, 1 uncommon, 2 rare, 3 epic.
    rarity: HashMap<String, i32>,
    mods: HashMap<String, Vec<AttrMod>>,
    sentiment: HashMap<String, String>,
    /// Зелье (ключ реестра) → (суффикс lang-ключа, эффекты по умолчанию).
    potions: HashMap<String, (String, Vec<Eff>)>,
    /// Эффект → категория (BENEFICIAL/HARMFUL/NEUTRAL).
    effect_cat: HashMap<String, String>,
    /// Эффект → модификаторы атрибутов на единицу (амплификатор+1).
    effect_mods: HashMap<String, Vec<AttrMod>>,
}

static DEFAULTS: OnceLock<Defaults> = OnceLock::new();

fn defaults() -> &'static Defaults {
    DEFAULTS.get_or_init(|| {
        let mut d = Defaults::default();
        for l in include_str!("../data/tooltip_data.txt").lines() {
            let f: Vec<&str> = l.split('|').collect();
            match f.as_slice() {
                ["I", name, rar, max, _] => {
                    let r = match *rar {
                        "uncommon" => 1,
                        "rare" => 2,
                        "epic" => 3,
                        _ => 0,
                    };
                    d.rarity.insert(name.to_string(), r);
                    if let Ok(m) = max.parse::<i32>() {
                        d.max_damage.insert(name.to_string(), m);
                    }
                }
                ["M", name, attr, amount, op, slot, id] => {
                    if let Ok(amount) = amount.parse::<f64>() {
                        d.mods.entry(name.to_string()).or_default().push(AttrMod {
                            attr: attr.to_string(),
                            id: id.to_string(),
                            amount,
                            op: op_id(op),
                            slot: slot.to_string(),
                            display: Disp::Default,
                        });
                    }
                }
                ["A", attr, sent] => {
                    d.sentiment.insert(attr.to_string(), sent.to_string());
                }
                ["P", pid, lang, effs] => {
                    let list = effs
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .filter_map(|e| {
                            let mut p = e.split(':');
                            Some(Eff {
                                name: p.next()?.to_string(),
                                dur: p.next()?.parse().ok()?,
                                amp: p.next()?.parse().ok()?,
                            })
                        })
                        .collect();
                    d.potions
                        .insert(pid.to_string(), (lang.to_string(), list));
                }
                ["E", eff, cat] => {
                    d.effect_cat.insert(eff.to_string(), cat.to_string());
                }
                ["T", eff, attr, amount, op, id] => {
                    if let Ok(amount) = amount.parse::<f64>() {
                        d.effect_mods
                            .entry(eff.to_string())
                            .or_default()
                            .push(AttrMod {
                                attr: attr.to_string(),
                                id: id.to_string(),
                                amount,
                                op: op_id(op),
                                slot: "any".into(),
                                display: Disp::Default,
                            });
                    }
                }
                _ => {}
            }
        }
        d
    })
}

/// VANILLA: AttributeModifier.Operation#id — 0 add_value, 1 add_multiplied_base, 2 add_multiplied_total.
fn op_id(op: &str) -> i32 {
    match op {
        "add_value" => 0,
        "add_multiplied_base" => 1,
        _ => 2,
    }
}

/// VANILLA: ItemAttributeModifiers.Entry#STREAM_CODEC — attribute (holder VarInt), modifier (id, double, op VarInt),
/// slot (VarInt), display (VarInt тип; 2 = OverrideText с компонентом).
fn parse_attr_patch(lang: &Lang, b: &[u8], names: &[String]) -> Vec<AttrMod> {
    let mut r = Reader::new(b);
    let n = r.var_int().unwrap_or(0).max(0);
    let mut out = Vec::new();
    for _ in 0..n {
        let (Ok(h), Ok(id), Ok(amount), Ok(op), Ok(slot), Ok(disp)) = (
            r.var_int(),
            r.string(32767),
            r.f64(),
            r.var_int(),
            r.var_int(),
            r.var_int(),
        ) else {
            break;
        };
        let display = match disp {
            1 => Disp::Hidden,
            2 => match nbt::read_network(&mut r)
                .ok()
                .flatten()
                .and_then(|t| Component::from_nbt(&t).ok())
            {
                Some(c) => Disp::Text(c.to_plain(&|k| {
                    let s = lang.tr(k);
                    (s != k).then_some(s)
                })),
                None => break,
            },
            _ => Disp::Default,
        };
        let (Some(attr), Some(slot)) = (names.get(h as usize), SLOT_ORDER.get(slot as usize))
        else {
            continue;
        };
        out.push(AttrMod {
            attr: attr.clone(),
            id,
            amount,
            op,
            slot: slot.to_string(),
            display,
        });
    }
    out
}

/// VANILLA: ATTRIBUTE_MODIFIER_FORMAT = DecimalFormat("#.##") — до двух знаков, без хвостовых нулей.
fn fmt_num(v: f64) -> String {
    let s = format!("{v:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() || s == "-0" {
        "0".into()
    } else {
        s.into()
    }
}

/// Подстановка `%s` по порядку и `%%` → `%` (как в lang-шаблонах Mojang).
fn fill(tpl: &str, args: &[&str]) -> String {
    let mut s = tpl.to_owned();
    for a in args {
        s = s.replacen("%s", a, 1);
    }
    s.replace("%%", "%")
}

/// VANILLA: StringUtil#formatTickDuration (tickrate 20): `mm:ss`, при часах `hh:mm:ss`.
fn fmt_ticks(ticks: i32) -> String {
    let s = (ticks as f32 / 20.0).floor() as i64;
    let (m, sec) = (s / 60, s % 60);
    let (h, m) = (m / 60, m % 60);
    if h > 0 {
        format!("{h:02}:{m:02}:{sec:02}")
    } else {
        format!("{m:02}:{sec:02}")
    }
}

/// VANILLA: Attribute#getStyle — POSITIVE: при росте синий, NEGATIVE: красный; NEUTRAL серый.
fn sent_color(sent: &str, increase: bool) -> u32 {
    match (sent, increase) {
        ("POSITIVE", true) | ("NEGATIVE", false) => BLUE,
        ("POSITIVE", false) | ("NEGATIVE", true) => RED,
        _ => GRAY,
    }
}

/// Имя атрибута из lang (`attribute.name.<attr>`), иначе короткое имя.
fn attr_name(lang: &Lang, attr: &str) -> String {
    let key = format!("attribute.name.{attr}");
    match lang.tr(&key) {
        s if s == key => attr.to_owned(),
        s => s,
    }
}

/// VANILLA: ItemAttributeModifiers.Display.Default#apply. Базовый урон/скорость игрока — 1.0 и 4.0
/// (Player#createAttributes), в подсказке показываются суммой с базой цветом DARK_GREEN.
fn attr_line(lang: &Lang, m: &AttrMod) -> Option<(String, u32)> {
    match &m.display {
        Disp::Hidden => return None,
        Disp::Text(t) => return Some((t.clone(), GRAY)),
        Disp::Default => {}
    }
    let name = attr_name(lang, &m.attr);
    let mut amount = m.amount;
    let mut with_base = false;
    if m.id == "base_attack_damage" {
        amount += 1.0;
        with_base = true;
    } else if m.id == "base_attack_speed" {
        amount += 4.0;
        with_base = true;
    }
    let shown = if m.op != 0 {
        amount * 100.0
    } else if m.attr == "knockback_resistance" {
        amount * 10.0
    } else {
        amount
    };
    let sent = defaults()
        .sentiment
        .get(&m.attr)
        .map(String::as_str)
        .unwrap_or("POSITIVE");
    if with_base {
        let tpl = lang.tr(&format!("attribute.modifier.equals.{}", m.op));
        return Some((
            format!(" {}", fill(&tpl, &[&fmt_num(shown), &name])),
            DARK_GREEN,
        ));
    }
    if amount > 0.0 {
        let tpl = lang.tr(&format!("attribute.modifier.plus.{}", m.op));
        Some((
            fill(&tpl, &[&fmt_num(shown), &name]),
            sent_color(sent, true),
        ))
    } else if amount < 0.0 {
        let tpl = lang.tr(&format!("attribute.modifier.take.{}", m.op));
        Some((
            fill(&tpl, &[&fmt_num(-shown), &name]),
            sent_color(sent, false),
        ))
    } else {
        None
    }
}

/// VANILLA: ItemStack#addAttributeTooltips — заголовок группы (пустая строка + «When …:») на первый видимый модификатор.
fn attribute_lines(lang: &Lang, mods: &[AttrMod], out: &mut Vec<(String, u32)>) {
    for slot in SLOT_ORDER {
        let mut first = true;
        for m in mods.iter().filter(|m| m.slot == slot) {
            if matches!(m.display, Disp::Hidden) {
                continue;
            }
            if first {
                out.push((String::new(), GRAY));
                out.push((lang.tr(&format!("item.modifiers.{slot}")), GRAY));
                first = false;
            }
            if let Some(l) = attr_line(lang, m) {
                out.push(l);
            }
        }
    }
}

fn comp<'a>(patch: &'a ComponentPatch, name: &str) -> Option<&'a [u8]> {
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == name)? as u32;
    patch
        .added
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, b)| &b[..])
}

/// Компонент-текст в строку с переводом (`Component#to_plain`).
fn plain(lang: &Lang, tag: &Tag) -> Option<String> {
    let c = Component::from_nbt(tag).ok()?;
    Some(c.to_plain(&|k| {
        let s = lang.tr(k);
        (s != k).then_some(s)
    }))
}

fn text_of(lang: &Lang, b: &[u8]) -> Option<String> {
    let tag = nbt::read_network(&mut Reader::new(b)).ok()??;
    plain(lang, &tag)
}

/// Декодер `potion_contents`. VANILLA: PotionContents#STREAM_CODEC — Holder<Potion> (VarInt, без сдвига),
/// опц. цвет (i32), список MobEffectInstance (holder VarInt + Details), опц. custom_name (STRING).
struct PotionData {
    potion: Option<i32>,
    /// (holder эффекта, длительность, амплификатор).
    effects: Vec<(i32, i32, i32)>,
    custom_name: Option<String>,
}

/// VANILLA: MobEffectInstance.Details#STREAM_CODEC — amplifier, duration, ambient, particles, icon, hidden (рекурсивно).
fn details(r: &mut Reader) -> Option<(i32, i32)> {
    let amp = r.var_int().ok()?;
    let dur = r.var_int().ok()?;
    r.bool().ok()?;
    r.bool().ok()?;
    r.bool().ok()?;
    if r.bool().ok()? {
        details(r)?;
    }
    Some((amp, dur))
}

fn decode_potion(b: &[u8]) -> Option<PotionData> {
    let mut r = Reader::new(b);
    let potion = if r.bool().ok()? {
        Some(r.var_int().ok()?)
    } else {
        None
    };
    if r.bool().ok()? {
        r.i32().ok()?;
    }
    let n = r.var_int().ok()?;
    let mut effects = Vec::new();
    for _ in 0..n.max(0) {
        let id = r.var_int().ok()?;
        let (amp, dur) = details(&mut r)?;
        effects.push((id, dur, amp));
    }
    let custom_name = if r.bool().ok()? {
        Some(r.string(32767).ok()?)
    } else {
        None
    };
    Some(PotionData {
        potion,
        effects,
        custom_name,
    })
}

/// Префикс lang-ключа названия зелья по предмету. VANILLA: PotionItem / TippedArrowItem#getName.
fn potion_prefix(name: &str) -> Option<&'static str> {
    match name {
        "potion" => Some("item.minecraft.potion.effect."),
        "splash_potion" => Some("item.minecraft.splash_potion.effect."),
        "lingering_potion" => Some("item.minecraft.lingering_potion.effect."),
        "tipped_arrow" => Some("item.minecraft.tipped_arrow.effect."),
        _ => None,
    }
}

/// VANILLA: PotionContents#getName — custom_name, иначе potion.name, иначе `empty` («Uncraftable Potion»).
fn potion_suffix(pd: &PotionData) -> String {
    if let Some(c) = &pd.custom_name {
        return c.clone();
    }
    match pd.potion.and_then(|id| POTION.get(id as usize)) {
        Some(pid) => defaults()
            .potions
            .get(*pid)
            .map(|p| p.0.clone())
            .unwrap_or_else(|| pid.to_string()),
        None => "empty".into(),
    }
}

/// VANILLA: PotionContents#addPotionTooltip — строка на эффект (уровень, длительность), затем «When Applied:»
/// и модификаторы атрибутов, умноженные на (амплификатор+1). Цвет по категории: HARMFUL красный, остальные синие.
fn potion_lines(lang: &Lang, effects: &[Eff], scale: f32, out: &mut Vec<(String, u32)>) {
    if effects.is_empty() {
        out.push((lang.tr("effect.none"), GRAY));
        return;
    }
    let mut mods: Vec<(String, f64, i32)> = Vec::new();
    for e in effects {
        let mut line = lang.tr(&format!("effect.minecraft.{}", e.name));
        if e.amp > 0 {
            let pot = lang.tr(&format!("potion.potency.{}", e.amp));
            line = fill(&lang.tr("potion.withAmplifier"), &[&line, &pot]);
        }
        // VANILLA: endsWithin(20) — короткие (≤ 1 с) не показывают длительность; бесконечные (-1) показывают.
        if e.dur == -1 || e.dur > 20 {
            let t = if e.dur == -1 {
                lang.tr("effect.duration.infinite")
            } else {
                fmt_ticks((e.dur as f32 * scale).floor() as i32)
            };
            line = fill(&lang.tr("potion.withDuration"), &[&line, &t]);
        }
        let harmful = defaults()
            .effect_cat
            .get(&e.name)
            .is_some_and(|c| c == "HARMFUL");
        out.push((line, if harmful { RED } else { BLUE }));
        for m in defaults().effect_mods.get(&e.name).into_iter().flatten() {
            mods.push((m.attr.clone(), m.amount * f64::from(e.amp + 1), m.op));
        }
    }
    if mods.is_empty() {
        return;
    }
    out.push((String::new(), GRAY));
    out.push((lang.tr("potion.whenDrank"), DARK_PURPLE));
    for (attr, amount, op) in mods {
        let shown = if op != 0 { amount * 100.0 } else { amount };
        let name = attr_name(lang, &attr);
        if amount > 0.0 {
            let tpl = lang.tr(&format!("attribute.modifier.plus.{op}"));
            out.push((fill(&tpl, &[&fmt_num(shown), &name]), BLUE));
        } else if amount < 0.0 {
            let tpl = lang.tr(&format!("attribute.modifier.take.{op}"));
            out.push((fill(&tpl, &[&fmt_num(-shown), &name]), RED));
        }
    }
}

/// Пластинка: описание из `jukebox_song` (`jukebox_playable`: VarInt holder id+1; 0 = прямая запись).
/// VANILLA: JukeboxPlayable#addToTooltip — description с цветом GRAY.
fn jukebox_line(lang: &Lang, b: &[u8]) -> Option<(String, u32)> {
    let mut r = Reader::new(b);
    let v = r.var_int().ok()?;
    let tag = if v > 0 {
        JUKEBOX_DESC
            .lock()
            .ok()?
            .get((v - 1) as usize)?
            .clone()?
    } else {
        nbt::read_network(&mut r).ok()??
    };
    Some((plain(lang, &tag)?, GRAY))
}

/// Пустой бандл: описание серым с переносом по 96 px. VANILLA: ClientBundleTooltip#extractEmptyBundleDescriptionText.
/// Полоска «Empty» и сетка не рисуются (визуальный компонент, см. заголовок модуля).
fn bundle_lines(lang: &Lang, t: &TextRenderer, s: &ItemStack, out: &mut Vec<(String, u32)>) {
    let filled = comp(&s.patch, "bundle_contents")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or(0)
        > 0;
    if filled {
        return;
    }
    let text = lang.tr("item.minecraft.bundle.empty.description");
    let mut cur = String::new();
    for w in text.split(' ') {
        let cand = if cur.is_empty() {
            w.to_owned()
        } else {
            format!("{cur} {w}")
        };
        if !cur.is_empty() && t.text_width(&cand) > 96.0 {
            out.push((std::mem::take(&mut cur), GRAY));
            cur = w.to_owned();
        } else {
            cur = cand;
        }
    }
    if !cur.is_empty() {
        out.push((cur, GRAY));
    }
}

/// VANILLA: ItemStack#getTooltipLines (часть) + getStyledHoverName/Rarity.
/// Редкость: компонент `rarity`, иначе по предмету из данных; чары поднимают common/uncommon → rare, rare → epic.
pub fn stack_lines(lang: &Lang, t: &TextRenderer, s: &ItemStack) -> Vec<(String, u32)> {
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    let mut rarity = comp(&s.patch, "rarity")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or_else(|| defaults().rarity.get(name).copied().unwrap_or(0));
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
    let potion = comp(&s.patch, "potion_contents").and_then(decode_potion);
    let title = comp(&s.patch, "custom_name")
        .and_then(|b| text_of(lang, b))
        .or_else(|| comp(&s.patch, "item_name").and_then(|b| text_of(lang, b)))
        .or_else(|| {
            potion_prefix(name).map(|p| {
                let suf = potion
                    .as_ref()
                    .map(potion_suffix)
                    .unwrap_or_else(|| "empty".into());
                lang.tr(&format!("{p}{suf}"))
            })
        })
        .unwrap_or_else(|| item_title(lang, name));
    let mut out = vec![(title, rc)];
    if name.ends_with("bundle") {
        bundle_lines(lang, t, s, &mut out);
    }
    // VANILLA: ItemStack#addDetailsToTooltip — порядок: POTION_CONTENTS → JUKEBOX_PLAYABLE → ENCHANTMENTS → …
    if let Some(pd) = &potion {
        let mut effs: Vec<Eff> = pd
            .potion
            .and_then(|id| POTION.get(id as usize))
            .and_then(|pid| defaults().potions.get(*pid))
            .map(|p| p.1.clone())
            .unwrap_or_default();
        for (id, dur, amp) in &pd.effects {
            if let Some(n) = MOB_EFFECT.get(*id as usize) {
                effs.push(Eff {
                    name: n.strip_prefix("minecraft:").unwrap_or(n).to_owned(),
                    dur: *dur,
                    amp: *amp,
                });
            }
        }
        let scale = comp(&s.patch, "potion_duration_scale")
            .and_then(|b| Reader::new(b).f32().ok())
            .unwrap_or(1.0);
        potion_lines(lang, &effs, scale, &mut out);
    }
    if let Some(l) = comp(&s.patch, "jukebox_playable").and_then(|b| jukebox_line(lang, b)) {
        out.push(l);
    }
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
    let names = ATTRIBUTES.lock().map(|g| g.clone()).unwrap_or_default();
    let mods = match comp(&s.patch, "attribute_modifiers") {
        Some(b) => parse_attr_patch(lang, b, &names),
        None => defaults().mods.get(name).cloned().unwrap_or_default(),
    };
    attribute_lines(lang, &mods, &mut out);
    if comp(&s.patch, "unbreakable").is_some() {
        out.push((lang.tr("item.unbreakable"), 0x5555FF));
    }
    // VANILLA: ItemStack#addDetailsToTooltip — спаунер без BLOCK_ENTITY_DATA: «Interact with Spawn Egg:».
    // VANILLA: Spawner#appendHoverText. Имя сущности (при наличии data) пока не выводится.
    if matches!(name, "monster_spawner" | "trial_spawner")
        && comp(&s.patch, "block_entity_data").is_none()
    {
        out.push((String::new(), GRAY));
        out.push((lang.tr("block.minecraft.spawner.desc1"), GRAY));
        out.push((format!(" {}", lang.tr("block.minecraft.spawner.desc2")), BLUE));
    }
    // VANILLA: ItemStack#addDetailsToTooltip — при F3+H: прочность и id предмета в конце.
    if ADVANCED.load(Ordering::Relaxed) {
        let damage = comp(&s.patch, "damage")
            .and_then(|b| Reader::new(b).var_int().ok())
            .unwrap_or(0);
        let max = comp(&s.patch, "max_damage")
            .and_then(|b| Reader::new(b).var_int().ok())
            .or_else(|| defaults().max_damage.get(name).copied())
            .unwrap_or(0);
        if damage > 0 && max > 0 {
            let tpl = lang.tr("item.durability");
            out.push((
                fill(&tpl, &[&(max - damage).to_string(), &max.to_string()]),
                GRAY,
            ));
        }
        out.push((format!("minecraft:{name}"), DARK_GRAY));
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
