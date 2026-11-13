//! Подсказка предмета (TODO-235): название (custom_name/item_name/lang/зелья), цвет редкости, зелья (эффекты,
//! «When Applied:»), пластинка (описание jukebox_song), пустой бандл (описание), чары, lore, атрибуты («When in Main
//! Hand…»), Unbreakable, спаунер (без имени сущности), F3+H (прочность, id), рамка.
//! Бандл: сетка/полоска (`draw_block`, после тултипов кадра). Нет: выбранный предмет бандла (колесо), имя сущности спаунера, категория креатива, tooltip_display,
//! компоненты (F3+H), OverrideText-стили.
//! VANILLA: GuiGraphics#renderTooltip / TooltipRenderUtil (фон 0xF0100010, рамка 0x505000FF→0x5028007F).
use crate::chat;
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
/// Реестр `minecraft:jukebox_song` (порядок = holder id): (имя без `minecraft:`, компонент `description`).
pub static JUKEBOX_DESC: Mutex<Vec<(String, Option<Tag>)>> = Mutex::new(Vec::new());

/// Трим: описания материалов и узоров (порядок = holder id) из реестров `trim_material` / `trim_pattern`.
#[allow(clippy::type_complexity)]
pub static TRIM_DESC: Mutex<(Vec<Option<Tag>>, Vec<Option<Tag>>)> =
    Mutex::new((Vec::new(), Vec::new()));

/// F3+H: расширенная подсказка (прочность, id предмета). VANILLA: KeyboardHandler — F3+H → advanced tooltips.
pub static ADVANCED: AtomicBool = AtomicBool::new(false);
/// Игрок в креативе (`TooltipFlag#isCreative`): `hide_tooltip` не прячет подсказку.
pub static CREATIVE: AtomicBool = AtomicBool::new(false);

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
    /// Предмет → имена дефолтных компонентов (для F3+H «N components»).
    protos: HashMap<String, Vec<String>>,
    /// Пластинка → ключ песни из дефолтного `jukebox_playable`.
    song: HashMap<String, String>,
}

static DEFAULTS: OnceLock<Defaults> = OnceLock::new();

fn defaults() -> &'static Defaults {
    DEFAULTS.get_or_init(|| {
        let mut d = Defaults::default();
        for l in include_str!("../data/tooltip_data.txt").lines() {
            let f: Vec<&str> = l.split('|').collect();
            match f.as_slice() {
                ["I", name, rar, max, names] => {
                    d.protos.insert(
                        name.to_string(),
                        names
                            .split(',')
                            .filter(|n| !n.is_empty())
                            .map(str::to_owned)
                            .collect(),
                    );
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
                    d.potions.insert(pid.to_string(), (lang.to_string(), list));
                }
                ["J", name, song] => {
                    d.song.insert(name.to_string(), song.to_string());
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

/// VANILLA: Item#isBarVisible/getBarWidth/getBarColor — полоска прочности: (ширина 0..=13, цвет RGB).
/// Видна только у повреждённых предметов (damage > 0 и max_damage > 0).
pub fn durability_bar(name: &str, patch: &ComponentPatch) -> Option<(u32, u32)> {
    let damage = comp(patch, "damage")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or(0);
    let max = comp(patch, "max_damage")
        .and_then(|b| Reader::new(b).var_int().ok())
        .or_else(|| defaults().max_damage.get(name).copied())
        .unwrap_or(0);
    if damage <= 0 || max <= 0 {
        return None;
    }
    let (d, m) = (damage as f32, max as f32);
    let width = (13.0 - d * 13.0 / m).round().clamp(0.0, 13.0) as u32;
    // Mth#hsvToRgb(max(0, (max - damage) / max) / 3, 1, 1)
    let h = ((m - d) / m).max(0.0) / 3.0 * 6.0;
    let f = h - h.floor();
    let (r, g, b) = match h.floor() as i32 % 6 {
        0 => (1.0, f, 0.0),
        1 => (1.0 - f, 1.0, 0.0),
        _ => (0.0, 1.0, f),
    };
    let c = |v: f32| (v * 255.0 + 0.5) as u32;
    Some((width, (c(r) << 16) | (c(g) << 8) | c(b)))
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

/// Строка без кодов форматирования `§` (для MCP/тестов).
pub fn strip_codes(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '§' {
            it.next();
        } else {
            out.push(c);
        }
    }
    out
}

/// Компонент-текст со стилем родителя (цвет, курсив) в строку с кодами `§`.
fn styled_text(lang: &Lang, b: &[u8], color: u32, italic: bool) -> Option<String> {
    let tag = nbt::read_network(&mut Reader::new(b)).ok()??;
    let c = Component::from_nbt(&tag).ok()?;
    let sp = chat::flatten_styled(&c, lang, color, italic);
    Some(chat::spans_to_legacy(&sp, color))
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

/// Пластинка из патча: `jukebox_playable` = VarInt holder id+1 (0 = прямая запись).
/// VANILLA: JukeboxPlayable#addToTooltip — description с цветом GRAY.
fn jukebox_line(lang: &Lang, b: &[u8]) -> Option<(String, u32)> {
    let mut r = Reader::new(b);
    let v = r.var_int().ok()?;
    let tag = if v > 0 {
        JUKEBOX_DESC.lock().ok()?.get((v - 1) as usize)?.1.clone()?
    } else {
        nbt::read_network(&mut r).ok()??
    };
    Some((plain(lang, &tag)?, GRAY))
}

/// Описание песни по id реестра jukebox_song (level_event 1010). VANILLA: JukeboxSong#description.
pub fn song_description(lang: &Lang, id: i32) -> Option<String> {
    let tag = JUKEBOX_DESC
        .lock()
        .ok()?
        .get(usize::try_from(id).ok()?)?
        .1
        .clone()?;
    plain(lang, &tag)
}

/// Описание песни по ключу реестра (дефолт пластинки из данных). VANILLA: jukebox_song.description.
fn song_tag(key: &str) -> Option<Tag> {
    JUKEBOX_DESC
        .lock()
        .ok()?
        .iter()
        .find(|(n, _)| n == key)
        .and_then(|(_, t)| t.clone())
}

/// Визуальный блок тултипа бандла (ClientBundleTooltip), рисуется после текста: `draw_block`.
struct Block {
    /// Левый верхний угол блока и ширина тултипа (контент 96 по центру).
    x: f32,
    y: f32,
    w: f32,
    items: Vec<ItemStack>,
    /// Вес содержимого: (числитель, знаменатель).
    weight: (i128, i128),
    desc: Vec<String>,
    /// Индекс выбранного предмета (колесо) или -1.
    sel: i32,
}

/// Маркер строки: место под визуальный блок (по индексу 1, после имени).
const BLOCK_MARK: &str = "\u{1}block";
static PENDING: Mutex<Option<Block>> = Mutex::new(None);
static BUNDLE_SRC: Mutex<Option<(Vec<ItemStack>, Vec<String>)>> = Mutex::new(None);
/// Выбор предмета бандла под курсором: (слот меню, индекс или -1) и накопленное колесо.
/// VANILLA: BundleMouseActions + BundleItem#getSelectedItemIndex (на сервер не уходит в компоненте, только пакетом).
static SEL: Mutex<(i32, i32, f32)> = Mutex::new((-1, -1, 0.0));

fn selected() -> i32 {
    SEL.lock().map_or(-1, |g| g.1)
}

/// VANILLA: BundleItem#getNumberOfItemsToShow.
fn items_to_show(n: usize) -> usize {
    let avail = if n > 12 { 11 } else { 12 };
    let on_row = n % 4;
    let empty = if on_row == 0 { 0 } else { 4 - on_row };
    n.min(avail - empty)
}

/// Колесо над бандлом в слоте `slot`. Возвращает пакет (слот, индекс).
/// VANILLA: BundleMouseActions#onMouseScrolled + ScrollWheelHandler.
pub fn bundle_wheel(slot: usize, s: &ItemStack, delta: f32) -> Option<(i32, i32)> {
    if !ITEM
        .get(s.item as usize)
        .is_some_and(|n| n.ends_with("bundle"))
    {
        return None;
    }
    let n = comp(&s.patch, "bundle_contents")
        .map(read_contents)
        .map_or(0, |v| items_to_show(v.len()));
    if n == 0 {
        return None;
    }
    let mut g = SEL.lock().ok()?;
    let slot = slot as i32;
    if g.0 != slot {
        *g = (slot, -1, 0.0);
    }
    if g.2 != 0.0 && g.2.signum() != delta.signum() {
        g.2 = 0.0;
    }
    g.2 += delta;
    let wheel = g.2 as i32;
    if wheel == 0 {
        return None;
    }
    g.2 -= wheel as f32;
    let lim = n as i32;
    let mut sel = (g.1 - wheel.signum()).max(-1);
    while sel < 0 {
        sel += lim;
    }
    while sel >= lim {
        sel -= lim;
    }
    if sel == g.1 {
        return None;
    }
    g.1 = sel;
    Some((slot, sel))
}

/// Курсор ушёл с бандла (или сменился слот): сбросить выбор. VANILLA: BundleMouseActions#onStopHovering.
pub fn bundle_unhover(hovered: Option<usize>) -> Option<(i32, i32)> {
    let mut g = SEL.lock().ok()?;
    if g.1 >= 0 && hovered.map(|h| h as i32) != Some(g.0) {
        let old = g.0;
        *g = (-1, -1, 0.0);
        return Some((old, -1));
    }
    None
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs().max(1)
    } else {
        gcd(b, a % b)
    }
}

/// Вес стека. VANILLA: BundleContents#getWeight (бандл в бандле = 1/16 + вес содержимого; пчёлы = 1; иначе 1/maxStack).
fn stack_weight(s: &ItemStack) -> (i128, i128) {
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    let (n, d) = if let Some(b) = comp(&s.patch, "bundle_contents") {
        let (n, d) = contents_weight(&read_contents(b));
        (n * 16 + d, d * 16)
    } else if comp(&s.patch, "bees")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or(0)
        > 0
    {
        (1, 1)
    } else {
        let max = comp(&s.patch, "max_stack_size")
            .and_then(|b| Reader::new(b).var_int().ok())
            .unwrap_or_else(|| crate::itemdata::max_stack(name))
            .max(1);
        (1, i128::from(max))
    };
    (n * i128::from(s.count), d)
}

fn contents_weight(items: &[ItemStack]) -> (i128, i128) {
    let (mut n, mut d) = (0i128, 1i128);
    for s in items {
        let (a, b) = stack_weight(s);
        n = n * b + a * d;
        d *= b;
        let g = gcd(n, d);
        n /= g;
        d /= g;
    }
    (n, d)
}

fn read_contents(b: &[u8]) -> Vec<ItemStack> {
    let mut r = Reader::new(b);
    let n = r.var_int().unwrap_or(0).clamp(0, 4096);
    let mut v = Vec::new();
    for _ in 0..n {
        match ezzmc_protocol::item::read_template(&mut r) {
            Ok(s) => v.push(s),
            Err(_) => break,
        }
    }
    v
}

/// Бандл: маркер блока и данные (описание пустого — серым с переносом по 96 px).
/// VANILLA: ClientBundleTooltip (сетка 4×≤3 по 24 px, полоска 96×13), extractEmptyBundleDescriptionText.
fn bundle_lines(lang: &Lang, t: &TextRenderer, s: &ItemStack, out: &mut Vec<(String, u32)>) {
    let items = comp(&s.patch, "bundle_contents")
        .map(read_contents)
        .unwrap_or_default();
    let mut desc = Vec::new();
    if items.is_empty() {
        let text = lang.tr("item.minecraft.bundle.empty.description");
        let mut cur = String::new();
        for w in text.split(' ') {
            let cand = if cur.is_empty() {
                w.to_owned()
            } else {
                format!("{cur} {w}")
            };
            if !cur.is_empty() && t.text_width(&cand) > 96.0 {
                desc.push(std::mem::take(&mut cur));
                cur = w.to_owned();
            } else {
                cur = cand;
            }
        }
        if !cur.is_empty() {
            desc.push(cur);
        }
    }
    if let Ok(mut g) = BUNDLE_SRC.lock() {
        *g = Some((items, desc));
    }
    out.push((BLOCK_MARK.to_owned(), 0));
}

/// Высота блока. VANILLA: ClientBundleTooltip#getHeight.
fn block_height(items: usize, desc: usize) -> f32 {
    if items == 0 {
        (desc * 9 + 13 + 8) as f32
    } else {
        (items.min(12).div_ceil(4) * 24 + 13 + 8) as f32
    }
}

/// Рисует блок бандла поверх уже выведенного тултипа (спрайты → иконки → текст). Вызывать после тултипов кадра.
/// VANILLA: ClientBundleTooltip#extractImage.
#[allow(clippy::too_many_arguments)]
pub fn draw_block(
    p: &mut ezzmc_render::gui::GuiPainter,
    t: &mut TextRenderer,
    lang: &Lang,
    g: &ezzmc_render::Gpu,
    view: &wgpu::TextureView,
    size: (u32, u32),
    rm: &ezzmc_assets::pack::ResourceManager,
) {
    let Some(b) = PENDING.lock().ok().and_then(|mut g| g.take()) else {
        return;
    };
    let (w, h) = size;
    t.flush(&g.device, &g.queue, view, w, h);
    t.begin(w, h);
    p.begin(t.scale());
    let left = b.x + ((b.w - 96.0) / 2.0).floor();
    let n = b.items.len();
    let mut texts = Vec::<(String, f32, f32, bool)>::new();
    let bar_y = if n == 0 {
        for (i, l) in b.desc.iter().enumerate() {
            texts.push((l.clone(), left, b.y + i as f32 * 9.0, false));
        }
        b.y + b.desc.len() as f32 * 9.0 + 4.0
    } else {
        let gy = n.min(12).div_ceil(4);
        let over = n > 12;
        let rows = (if over { 11 } else { 12 }) - (if n % 4 == 0 { 0 } else { 4 - n % 4 });
        let shown = n.min(rows);
        let (xs, ys) = (left + 96.0, b.y + gy as f32 * 24.0);
        let mut slot = 1usize;
        for row in 1..=gy {
            for col in 1..=4usize {
                let (dx, dy) = (xs - col as f32 * 24.0, ys - row as f32 * 24.0);
                if over && col * row == 1 {
                    let hidden: i32 = b.items.iter().skip(shown).map(|s| s.count).sum();
                    let txt = format!("+{hidden}");
                    let tw = t.text_width(&txt);
                    texts.push((txt, dx + 12.0 - (tw / 2.0).floor(), dy + 10.0, false));
                } else if shown >= slot {
                    let idx = shown - slot;
                    let hl = b.sel == idx as i32;
                    let it = &b.items[idx];
                    p.nine_slice(
                        if hl {
                            "bundle_slot_hl_back"
                        } else {
                            "bundle_slot_bg"
                        },
                        dx,
                        dy,
                        24.0,
                        24.0,
                        [24.0, 24.0],
                        [4.0; 4],
                        0xFFFF_FFFF,
                    );
                    crate::hud::stack_icon(p, &g.device, &g.queue, rm, it, dx + 4.0, dy + 4.0);
                    if hl {
                        p.nine_slice(
                            "bundle_slot_hl_front",
                            dx,
                            dy,
                            24.0,
                            24.0,
                            [24.0, 24.0],
                            [4.0; 4],
                            0xFFFF_FFFF,
                        );
                    }
                    if it.count != 1 {
                        let c = it.count.to_string();
                        texts.push((
                            c.clone(),
                            dx + 4.0 + 17.0 - t.text_width(&c),
                            dy + 13.0,
                            true,
                        ));
                    }
                    slot += 1;
                }
            }
        }
        b.y + gy as f32 * 24.0 + 4.0
    };
    // Полоска. VANILLA: extractProgressbar (fill = trunc(weight·94), ≥1 → full).
    let (wn, wd) = b.weight;
    let fill = (wn * 94 / wd.max(1)).clamp(0, 94) as f32;
    let full = wn >= wd;
    if fill > 0.0 {
        p.nine_slice(
            if full {
                "bundle_bar_full"
            } else {
                "bundle_bar_fill"
            },
            left + 1.0,
            bar_y,
            fill,
            13.0,
            [6.0, 6.0],
            [2.0; 4],
            0xFFFF_FFFF,
        );
    }
    p.nine_slice(
        "bundle_bar_border",
        left,
        bar_y,
        96.0,
        13.0,
        [12.0, 12.0],
        [2.0; 4],
        0xFFFF_FFFF,
    );
    let label = if wn == 0 {
        Some(lang.tr("item.minecraft.bundle.empty"))
    } else if full {
        Some(lang.tr("item.minecraft.bundle.full"))
    } else {
        None
    };
    p.flush(&g.device, &g.queue, view, w, h, None);
    for (s, x, y, shadow) in texts {
        t.draw(
            &s,
            x,
            y,
            if shadow || s.starts_with('+') {
                0xFFFFFF
            } else {
                0xAAAAAA
            },
            true,
        );
    }
    if let Some(l) = label {
        let tw = t.text_width(&l);
        t.draw(
            &l,
            left + 48.0 - (tw / 2.0).floor(),
            bar_y + 3.0,
            0xFFFFFF,
            true,
        );
    }
    // Имя выбранного предмета над тултипом. VANILLA: extractSelectedItemTooltip.
    if let Some(sel) = b.items.get(usize::try_from(b.sel).unwrap_or(usize::MAX)) {
        let mut lines = stack_lines(lang, t, sel);
        if let Ok(mut g) = BUNDLE_SRC.lock() {
            *g = None;
        }
        lines.truncate(1);
        if let Some((name, col)) = lines.first() {
            let tw = t.text_width(name);
            let x = (b.x + b.w / 2.0 - 12.0 - tw / 2.0).floor();
            let y = b.y - 15.0;
            frame(t, x, y, tw, 8.0);
            t.draw(name, x, y, *col, true);
        }
    }
}

/// VANILLA: ItemStack#getTooltipLines (часть) + getStyledHoverName/Rarity.
/// Редкость: компонент `rarity`, иначе по предмету из данных; чары поднимают common/uncommon → rare, rare → epic.
/// Скрытые компоненты предмета (`tooltip_display`). VANILLA: TooltipDisplay#shows (hiddenComponents).
/// Флаг `hide_tooltip` из `tooltip_display`. VANILLA: ItemStack#getTooltipLines.
fn hide_tooltip(patch: &ComponentPatch) -> bool {
    comp(patch, "tooltip_display")
        .and_then(|b| Reader::new(b).bool().ok())
        .unwrap_or(false)
}

fn hidden_components(patch: &ComponentPatch) -> Vec<&'static str> {
    let Some(b) = comp(patch, "tooltip_display") else {
        return Vec::new();
    };
    let mut r = Reader::new(b);
    let _hide_tooltip = r.bool().unwrap_or(false);
    let n = r.var_int().unwrap_or(0);
    (0..n)
        .filter_map(|_| r.var_int().ok())
        .filter_map(|id| DATA_COMPONENT_TYPE.get(id as usize).copied())
        .collect()
}

/// Список (holder зачарования, уровень) из компонента `enchantments` / `stored_enchantments`.
fn ench_list(patch: &ComponentPatch, key: &str) -> Vec<(i32, i32)> {
    comp(patch, key)
        .map(|b| {
            let mut r = Reader::new(b);
            let n = r.var_int().unwrap_or(0);
            (0..n)
                .map_while(|_| Some((r.var_int().ok()?, r.var_int().ok()?)))
                .collect()
        })
        .unwrap_or_default()
}

/// Уровень зачарования по короткому имени (`efficiency`) в `enchantments` стека; 0 — нет.
pub fn enchant_level(patch: &ComponentPatch, name: &str) -> i32 {
    let reg = ENCHANTS.lock().map(|g| g.clone()).unwrap_or_default();
    ench_list(patch, "enchantments")
        .into_iter()
        .find(|&(h, _)| {
            reg.get(h as usize)
                .is_some_and(|(n, _)| n.strip_prefix("minecraft:").unwrap_or(n) == name)
        })
        .map_or(0, |(_, l)| l)
}

/// Строки зачарований. VANILLA: ItemEnchantments#addToTooltip — проклятия красным, остальное серым.
fn enchant_lines(lang: &Lang, ench: &[(i32, i32)], out: &mut Vec<(String, u32)>) {
    let reg = ENCHANTS.lock().map(|g| g.clone()).unwrap_or_default();
    for &(h, lvl) in ench {
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
}

/// Имя сущности спаунера из `block_entity_data` (`SpawnData.entity.id`). VANILLA: Spawner#getSpawnEntityDisplayName.
fn spawner_entity(patch: &ComponentPatch) -> Option<String> {
    let b = comp(patch, "block_entity_data")?;
    let mut r = Reader::new(b);
    // Тип block entity (holder VarInt), затем сам compound.
    r.var_int().ok()?;
    let tag = nbt::read_network(&mut r).ok()??;
    let id = tag.get("SpawnData")?.get("entity")?.get("id")?.as_str()?;
    Some(id.to_owned())
}

/// Трим брони: «Upgrade:», узор и материал цветом материала. VANILLA: ArmorTrim#addToTooltip.
fn trim_lines(lang: &Lang, b: &[u8], out: &mut Vec<(String, u32)>) -> Option<()> {
    let mut r = Reader::new(b);
    // Holder на проводе: id+1 (0 = прямая запись, здесь не поддержана).
    let m = r.var_int().ok()?.checked_sub(1)? as usize;
    let p = r.var_int().ok()?.checked_sub(1)? as usize;
    let (mat, pat) = {
        let g = TRIM_DESC.lock().ok()?;
        (g.0.get(m)?.clone()?, g.1.get(p)?.clone()?)
    };
    let color = Component::from_nbt(&mat)
        .ok()?
        .style
        .color
        .unwrap_or(0xFFFFFF);
    out.push((lang.tr("item.minecraft.smithing_template.upgrade"), GRAY));
    out.push((format!(" {}", plain(lang, &pat)?), color));
    out.push((format!(" {}", plain(lang, &mat)?), color));
    Some(())
}

/// Цвет окраски. VANILLA: DyedItemColor#addToTooltip — при F3+H hex, иначе «Dyed».
fn dyed_lines(lang: &Lang, b: &[u8], out: &mut Vec<(String, u32)>) {
    if ADVANCED.load(Ordering::Relaxed) {
        let rgb = Reader::new(b).i32().unwrap_or(0) as u32;
        out.push((
            fill(&lang.tr("item.color"), &[&format!("#{rgb:06X}")]),
            GRAY,
        ));
    } else {
        out.push((lang.tr("item.dyed"), GRAY));
    }
}

/// Число компонентов стака (F3+H). VANILLA: PatchedDataComponentMap#size — дефолты предмета ± патч.
fn component_count(name: &str, patch: &ComponentPatch) -> usize {
    let mut set: Vec<&str> = defaults()
        .protos
        .get(name)
        .map(|v| v.iter().map(String::as_str).collect())
        .unwrap_or_default();
    for (id, _) in &patch.added {
        if let Some(n) = DATA_COMPONENT_TYPE.get(*id as usize) {
            if !set.contains(n) {
                set.push(n);
            }
        }
    }
    for id in &patch.removed {
        if let Some(n) = DATA_COMPONENT_TYPE.get(*id as usize) {
            set.retain(|x| x != n);
        }
    }
    set.len()
}

/// Имя предмета без стиля/цвета. VANILLA: ItemStack#getHoverName (custom_name → item_name → зелье → lang).
pub fn hover_name(lang: &Lang, s: &ItemStack) -> String {
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    comp(&s.patch, "custom_name")
        .and_then(|b| text_of(lang, b))
        .or_else(|| comp(&s.patch, "item_name").and_then(|b| text_of(lang, b)))
        .or_else(|| {
            potion_prefix(name).map(|p| {
                let pd = comp(&s.patch, "potion_contents").and_then(decode_potion);
                let suf = pd
                    .as_ref()
                    .map(potion_suffix)
                    .unwrap_or_else(|| "empty".into());
                lang.tr(&format!("{p}{suf}"))
            })
        })
        .unwrap_or_else(|| item_title(lang, name))
}

/// Предмет несёт свой `custom_name` (патч). VANILLA: DataComponents#CUSTOM_NAME.
pub fn has_custom_name(s: &ItemStack) -> bool {
    comp(&s.patch, "custom_name").is_some()
}

/// Строки тултипа в порядке `ItemStack#addDetailsToTooltip`.
pub fn stack_lines(lang: &Lang, t: &TextRenderer, s: &ItemStack) -> Vec<(String, u32)> {
    let name = ITEM.get(s.item as usize).copied().unwrap_or("air");
    // VANILLA: ItemStack#getTooltipLines — вне креатива hide_tooltip даёт пустую подсказку
    // (ветка OP_NBT_WARNING не портирована: нужен shouldPrintOpWarning).
    if hide_tooltip(&s.patch) && !CREATIVE.load(Ordering::Relaxed) {
        return Vec::new();
    }
    let hidden = hidden_components(&s.patch);
    let shows = |c: &str| !hidden.contains(&c);
    let ench = ench_list(&s.patch, "enchantments");
    let stored = ench_list(&s.patch, "stored_enchantments");
    let mut rarity = comp(&s.patch, "rarity")
        .and_then(|b| Reader::new(b).var_int().ok())
        .unwrap_or_else(|| defaults().rarity.get(name).copied().unwrap_or(0));
    if !ench.is_empty() && name != "enchanted_book" {
        rarity = match rarity {
            0 | 1 => 2,
            _ => 3,
        };
    }
    let rc = [0xFFFFFF, 0xFFFF55, 0x55FFFF, 0xFF55FF][rarity.clamp(0, 3) as usize];
    let potion = comp(&s.patch, "potion_contents").and_then(decode_potion);
    // VANILLA: ItemStack#getStyledHoverName — custom_name курсивом (цвет редкости — родитель).
    let title = comp(&s.patch, "custom_name")
        .and_then(|b| styled_text(lang, b, rc, true))
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
    if let (Some(pd), true) = (&potion, shows("potion_contents")) {
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
    if shows("jukebox_playable") {
        // Патч: holder id+1 или прямая запись. Нет патча — дефолт предмета (ключ песни из данных).
        let jb = match comp(&s.patch, "jukebox_playable") {
            Some(b) => jukebox_line(lang, b),
            None => defaults()
                .song
                .get(name)
                .and_then(|k| song_tag(k))
                .and_then(|t| plain(lang, &t))
                .map(|t| (t, GRAY)),
        };
        out.extend(jb);
    }
    if shows("trim") {
        if let Some(b) = comp(&s.patch, "trim") {
            trim_lines(lang, b, &mut out);
        }
    }
    if shows("stored_enchantments") {
        enchant_lines(lang, &stored, &mut out);
    }
    if shows("enchantments") {
        enchant_lines(lang, &ench, &mut out);
    }
    if shows("dyed_color") {
        if let Some(b) = comp(&s.patch, "dyed_color") {
            dyed_lines(lang, b, &mut out);
        }
    }
    if let (Some(b), true) = (comp(&s.patch, "lore"), shows("lore")) {
        let mut r = Reader::new(b);
        let n = r.var_int().unwrap_or(0);
        for _ in 0..n {
            let Ok(Some(tag)) = nbt::read_network(&mut r) else {
                break;
            };
            if let Ok(c) = Component::from_nbt(&tag) {
                // VANILLA: ItemStack#LORE_STYLE — тёмно-фиолетовый курсив.
                let sp = chat::flatten_styled(&c, lang, 0xAA00AA, true);
                out.push((chat::spans_to_legacy(&sp, 0xAA00AA), 0xAA00AA));
            }
        }
    }
    if shows("attribute_modifiers") {
        let names = ATTRIBUTES.lock().map(|g| g.clone()).unwrap_or_default();
        let mods = match comp(&s.patch, "attribute_modifiers") {
            Some(b) => parse_attr_patch(lang, b, &names),
            None => defaults().mods.get(name).cloned().unwrap_or_default(),
        };
        attribute_lines(lang, &mods, &mut out);
    }
    if comp(&s.patch, "unbreakable").is_some() && shows("unbreakable") {
        out.push((lang.tr("item.unbreakable"), 0x5555FF));
    }
    // VANILLA: ItemStack#addDetailsToTooltip — OMINOUS_BOTTLE_AMPLIFIER, затем SUSPICIOUS_STEW_EFFECTS.
    // VANILLA: OminousBottleAmplifier#addToTooltip — Bad Omen на 120000 тиков, уровень = значение.
    if shows("ominous_bottle_amplifier") {
        if let Some(b) = comp(&s.patch, "ominous_bottle_amplifier") {
            let amp = Reader::new(b).var_int().unwrap_or(0);
            let effs = [Eff {
                name: "bad_omen".into(),
                dur: 120_000,
                amp,
            }];
            potion_lines(lang, &effs, 1.0, &mut out);
        }
    }
    // VANILLA: SuspiciousStewEffects#addToTooltip — только при флаге креатива.
    // Запись: holderRegistry (сырой id, без +1), затем VarInt длительности.
    if shows("suspicious_stew_effects") && CREATIVE.load(Ordering::Relaxed) {
        if let Some(b) = comp(&s.patch, "suspicious_stew_effects") {
            let mut r = Reader::new(b);
            let n = r.var_int().unwrap_or(0);
            let mut effs = Vec::new();
            for _ in 0..n {
                let (Ok(id), Ok(dur)) = (r.var_int(), r.var_int()) else {
                    break;
                };
                if let Some(nm) = MOB_EFFECT.get(id as usize) {
                    effs.push(Eff {
                        name: nm.strip_prefix("minecraft:").unwrap_or(nm).to_owned(),
                        dur,
                        amp: 0,
                    });
                }
            }
            potion_lines(lang, &effs, 1.0, &mut out);
        }
    }
    // VANILLA: ItemStack#addDetailsToTooltip — спаунер: имя сущности, иначе «Interact with Spawn Egg:».
    // VANILLA: Spawner#appendHoverText.
    if matches!(name, "monster_spawner" | "trial_spawner") && shows("block_entity_data") {
        match spawner_entity(&s.patch) {
            Some(id) => {
                let (ns, p) = id.split_once(':').unwrap_or(("minecraft", &id));
                out.push((lang.tr(&format!("entity.{ns}.{p}")), GRAY));
            }
            None => {
                out.push((String::new(), GRAY));
                out.push((lang.tr("block.minecraft.spawner.desc1"), GRAY));
                out.push((
                    format!(" {}", lang.tr("block.minecraft.spawner.desc2")),
                    BLUE,
                ));
            }
        }
    }
    // VANILLA: ItemStack#addDetailsToTooltip — при F3+H: прочность, id предмета и число компонентов.
    if ADVANCED.load(Ordering::Relaxed) {
        let damage = comp(&s.patch, "damage")
            .and_then(|b| Reader::new(b).var_int().ok())
            .unwrap_or(0);
        let max = comp(&s.patch, "max_damage")
            .and_then(|b| Reader::new(b).var_int().ok())
            .or_else(|| defaults().max_damage.get(name).copied())
            .unwrap_or(0);
        if damage > 0 && max > 0 && shows("damage") {
            let tpl = lang.tr("item.durability");
            out.push((
                fill(&tpl, &[&(max - damage).to_string(), &max.to_string()]),
                GRAY,
            ));
        }
        out.push((format!("minecraft:{name}"), DARK_GRAY));
        let count = component_count(name, &s.patch);
        if count > 0 {
            let tpl = lang.tr("item.components");
            out.push((fill(&tpl, &[&count.to_string()]), DARK_GRAY));
        }
    }
    out
}

/// Фон и рамка тултипа вокруг текста (x, y, ширина, высота). VANILLA: TooltipRenderUtil.
fn frame(t: &mut TextRenderer, x: f32, y: f32, w: f32, h: f32) {
    t.fill(x - 3.0, y - 4.0, x + w + 3.0, y + h + 4.0, 0xF010_0010);
    t.fill(x - 3.0, y - 3.0, x - 2.0, y + h + 3.0, 0x5050_00FF);
    t.fill(x + w + 2.0, y - 3.0, x + w + 3.0, y + h + 3.0, 0x5028_007F);
    t.fill(x - 3.0, y - 3.0, x + w + 3.0, y - 2.0, 0x5050_00FF);
    t.fill(x - 3.0, y + h + 2.0, x + w + 3.0, y + h + 3.0, 0x5028_007F);
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
    // Маркер визуального блока (бандл): место высотой bh после имени, ширина не меньше 96.
    let mark = lines.iter().position(|l| l.0 == BLOCK_MARK);
    let src = mark.and_then(|_| BUNDLE_SRC.lock().ok().and_then(|mut g| g.take()));
    let bh = src
        .as_ref()
        .map_or(0.0, |(i, d)| block_height(i.len(), d.len()));
    let text: Vec<&(String, u32)> = lines.iter().filter(|l| l.0 != BLOCK_MARK).collect();
    let mut w = text.iter().map(|l| t.text_width(&l.0)).fold(0.0, f32::max);
    if src.is_some() {
        w = w.max(96.0);
    }
    let h = (if text.len() > 1 || src.is_some() {
        10.0 * text.len() as f32 + 2.0
    } else {
        8.0
    }) + bh;
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
    let mark_at = mark.unwrap_or(usize::MAX);
    // Блок идёт сразу после предыдущих строк (имя: 10 + зазор 2), остальные строки сдвигаются на bh.
    if let Some((items, desc)) = &src {
        let at = mark_at.min(text.len());
        if let Ok(mut g) = PENDING.lock() {
            *g = Some(Block {
                x,
                y: y + at as f32 * 10.0 + if at > 0 { 2.0 } else { 0.0 },
                w,
                items: items.clone(),
                weight: contents_weight(items),
                desc: desc.clone(),
                sel: selected(),
            });
        }
    }
    for (i, l) in text.iter().enumerate() {
        let dy = if i >= mark_at { bh } else { 0.0 };
        t.draw(
            &l.0,
            x,
            y + i as f32 * 10.0 + if i > 0 { 2.0 } else { 0.0 } + dy,
            l.1,
            true,
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod durability_tests {
    use super::*;

    fn patch(damage: u8, max: u8) -> ComponentPatch {
        use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
        let id = |n: &str| DATA_COMPONENT_TYPE.iter().position(|x| *x == n).unwrap() as u32;
        ComponentPatch {
            added: vec![
                (id("damage"), vec![damage].into()),
                (id("max_damage"), vec![max].into()),
            ],
            removed: vec![],
        }
    }

    #[test]
    fn bar_hidden_when_undamaged() {
        assert!(durability_bar("iron_pickaxe", &patch(0, 100)).is_none());
    }

    #[test]
    fn bar_width_and_color() {
        // 50/100: ширина round(13 − 6.5) = 7 (round half up в Java → 7), цвет — жёлтый (h = 1/6).
        let (w, rgb) = durability_bar("iron_pickaxe", &patch(50, 100)).unwrap();
        assert_eq!(w, 7);
        assert_eq!(rgb, 0xFFFF00);
        // почти сломан: красный
        let (w, rgb) = durability_bar("iron_pickaxe", &patch(100, 100)).unwrap();
        assert_eq!((w, rgb), (0, 0xFF0000));
    }
}
