//! `award_stats` версии соединения → id канона. ≤1.12.2 статистика приходит строками
//! (`stat.mineBlock.minecraft.stone`, на 1.7 — с числовым id), 1.13+ — id реестров своей версии.
//! ≤1.6.4 сервер шлёт приращения по числовым id, часть счётчиков ведёт сам клиент ([`Local`]):
//! суммы копятся здесь и лежат в файле `stats/legacy_<ник>.json`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use ezzmc_protocol::generated as gen;
use ezzmc_protocol::version::{self, build_remap, Version};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;
use ezzmc_world::generated::flattening::{ITEMS, NAMES};

// Id типов статистики. VANILLA: Stats (порядок регистрации StatType, 1.13+).
const MINED: i32 = 0;
const CRAFTED: i32 = 1;
const USED: i32 = 2;
const BROKEN: i32 = 3;
const PICKED_UP: i32 = 4;
const DROPPED: i32 = 5;
const KILLED: i32 = 6;
const KILLED_BY: i32 = 7;
const CUSTOM: i32 = 8;

/// VANILLA: Stats (1.13.2) — порядок `Registry.CUSTOM_STAT`; отчёта генератора у 1.13 нет.
#[rustfmt::skip]
const CUSTOM_1_13: &[&str] = &[
    "leave_game",
    "play_one_minute",
    "time_since_death",
    "time_since_rest",
    "sneak_time",
    "walk_one_cm",
    "crouch_one_cm",
    "sprint_one_cm",
    "walk_on_water_one_cm",
    "fall_one_cm",
    "climb_one_cm",
    "fly_one_cm",
    "walk_under_water_one_cm",
    "minecart_one_cm",
    "boat_one_cm",
    "pig_one_cm",
    "horse_one_cm",
    "aviate_one_cm",
    "swim_one_cm",
    "jump",
    "drop",
    "damage_dealt",
    "damage_dealt_absorbed",
    "damage_dealt_resisted",
    "damage_taken",
    "damage_blocked_by_shield",
    "damage_absorbed",
    "damage_resisted",
    "deaths",
    "mob_kills",
    "animals_bred",
    "player_kills",
    "fish_caught",
    "talked_to_villager",
    "traded_with_villager",
    "eat_cake_slice",
    "fill_cauldron",
    "use_cauldron",
    "clean_armor",
    "clean_banner",
    "clean_shulker_box",
    "interact_with_brewingstand",
    "interact_with_beacon",
    "inspect_dropper",
    "inspect_hopper",
    "inspect_dispenser",
    "play_noteblock",
    "tune_noteblock",
    "pot_flower",
    "trigger_trapped_chest",
    "open_enderchest",
    "enchant_item",
    "play_record",
    "interact_with_furnace",
    "interact_with_crafting_table",
    "open_chest",
    "sleep_in_bed",
    "open_shulker_box",
];

/// VANILLA: StatsCounterFix#CUSTOM_MAP, StatsRenameFix (swim/dive → walk_on/under_water).
#[rustfmt::skip]
const CUSTOM_1_12: &[(&str, &str)] = &[
    ("leaveGame", "leave_game"),
    ("playOneMinute", "play_time"),
    ("timeSinceDeath", "time_since_death"),
    ("sneakTime", "sneak_time"),
    ("walkOneCm", "walk_one_cm"),
    ("crouchOneCm", "crouch_one_cm"),
    ("sprintOneCm", "sprint_one_cm"),
    ("swimOneCm", "walk_on_water_one_cm"),
    ("fallOneCm", "fall_one_cm"),
    ("climbOneCm", "climb_one_cm"),
    ("flyOneCm", "fly_one_cm"),
    ("diveOneCm", "walk_under_water_one_cm"),
    ("minecartOneCm", "minecart_one_cm"),
    ("boatOneCm", "boat_one_cm"),
    ("pigOneCm", "pig_one_cm"),
    ("horseOneCm", "horse_one_cm"),
    ("aviateOneCm", "aviate_one_cm"),
    ("jump", "jump"),
    ("drop", "drop"),
    ("damageDealt", "damage_dealt"),
    ("damageTaken", "damage_taken"),
    ("deaths", "deaths"),
    ("mobKills", "mob_kills"),
    ("animalsBred", "animals_bred"),
    ("playerKills", "player_kills"),
    ("fishCaught", "fish_caught"),
    ("talkedToVillager", "talked_to_villager"),
    ("tradedWithVillager", "traded_with_villager"),
    ("cakeSlicesEaten", "eat_cake_slice"),
    ("cauldronFilled", "fill_cauldron"),
    ("cauldronUsed", "use_cauldron"),
    ("armorCleaned", "clean_armor"),
    ("bannerCleaned", "clean_banner"),
    ("brewingstandInteraction", "interact_with_brewingstand"),
    ("beaconInteraction", "interact_with_beacon"),
    ("dropperInspected", "inspect_dropper"),
    ("hopperInspected", "inspect_hopper"),
    ("dispenserInspected", "inspect_dispenser"),
    ("noteblockPlayed", "play_noteblock"),
    ("noteblockTuned", "tune_noteblock"),
    ("flowerPotted", "pot_flower"),
    ("trappedChestTriggered", "trigger_trapped_chest"),
    ("enderchestOpened", "open_enderchest"),
    ("itemEnchanted", "enchant_item"),
    ("recordPlayed", "play_record"),
    ("furnaceInteraction", "interact_with_furnace"),
    ("craftingTableInteraction", "interact_with_crafting_table"),
    ("chestOpened", "open_chest"),
    ("sleepInBed", "sleep_in_bed"),
    ("shulkerBoxOpened", "open_shulker_box"),
];

/// VANILLA: StatsCounterFix#ENTITIES — имена, не выводимые из PascalCase; плюс переименования
/// 1.13/1.14.
const ENTITIES_1_12: &[(&str, &str)] = &[
    ("EntityHorse", "horse"),
    ("EvocationIllager", "evoker"),
    ("evocation_illager", "evoker"),
    ("LavaSlime", "magma_cube"),
    ("MushroomCow", "mooshroom"),
    ("Ozelot", "ocelot"),
    ("PigZombie", "zombified_piglin"),
    ("zombie_pigman", "zombified_piglin"),
    ("VindicationIllager", "vindicator"),
    ("vindication_illager", "vindicator"),
];

fn canon_index(list: &[&str], name: &str) -> Option<i32> {
    let at = list.iter().position(|n| *n == name)?;
    i32::try_from(at).ok()
}

/// Имя сущности из ключа статистики ≤1.12.2 → канон.
fn entity_name(old: &str) -> String {
    let old = old.strip_prefix("minecraft.").unwrap_or(old);
    if let Some((_, new)) = ENTITIES_1_12.iter().find(|(o, _)| *o == old) {
        return (*new).to_owned();
    }
    let mut out = String::with_capacity(old.len() + 4);
    for (i, c) in old.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// Числовой id ≤1.12.2 из хвоста ключа: число (1.7) или имя реестра `minecraft.x` (1.8+).
/// Имя может быть и у блока, и у предмета с разными id: блок — меньший, предмет — больший.
fn legacy_id(tail: &str, block: bool) -> Option<u16> {
    if let Ok(id) = tail.parse() {
        return Some(id);
    }
    let name = tail.strip_prefix("minecraft.")?;
    let mut ids = NAMES.iter().filter(|(n, _)| *n == name).map(|&(_, i)| i);
    if block {
        ids.next()
    } else {
        ids.next_back()
    }
}

fn block_1_12(tail: &str) -> Option<i32> {
    let id = legacy_id(tail, true)?;
    let state = ezzmc_world::legacy::state_to_canon(u32::from(id) << 4);
    let block = ezzmc_world::block::BlockState::from_id(state)?;
    if block.is_air() {
        return None;
    }
    i32::try_from(block.block_id()).ok()
}

fn item_1_12(tail: &str) -> Option<i32> {
    let id = legacy_id(tail, false)?;
    let at = ITEMS
        .binary_search_by_key(&(id, 0), |&(i, d, _)| (i, d))
        .ok()?;
    let name = gen::r1_13_2::ITEM.get(usize::from(ITEMS.get(at)?.2))?;
    let map = build_remap(std::slice::from_ref(name), gen::r26_3::ITEM);
    i32::try_from(map.first().copied().flatten()?).ok()
}

/// Ключ статистики ≤1.12.2 → (тип, id значения) канона. `None` — в каноне такой нет.
// VANILLA: StatsCounterFix#unpackLegacyKey.
fn legacy_key(key: &str) -> Option<(i32, i32)> {
    let rest = key.strip_prefix("stat.")?;
    if let Some((_, new)) = CUSTOM_1_12.iter().find(|(o, _)| *o == rest) {
        return Some((CUSTOM, canon_index(gen::r26_3::CUSTOM_STAT, new)?));
    }
    let (kind, tail) = rest.split_once('.')?;
    let entity = || canon_index(gen::r26_3::ENTITY_TYPE, &entity_name(tail));
    Some(match kind {
        "mineBlock" => (MINED, block_1_12(tail)?),
        "craftItem" => (CRAFTED, item_1_12(tail)?),
        "useItem" => (USED, item_1_12(tail)?),
        "breakItem" => (BROKEN, item_1_12(tail)?),
        "pickup" => (PICKED_UP, item_1_12(tail)?),
        "drop" => (DROPPED, item_1_12(tail)?),
        "killEntity" => (KILLED, entity()?),
        "entityKilledBy" => (KILLED_BY, entity()?),
        _ => return None,
    })
}

fn custom_stats(protocol: i32) -> &'static [&'static str] {
    match protocol {
        ..=404 => CUSTOM_1_13,
        405..=498 => gen::r1_14_4::CUSTOM_STAT,
        499..=578 => gen::r1_15_2::CUSTOM_STAT,
        579..=754 => gen::r1_16_5::CUSTOM_STAT,
        755..=763 => gen::r1_20_1::CUSTOM_STAT,
        764 => gen::r1_20_2::CUSTOM_STAT,
        765 => gen::r1_20_4::CUSTOM_STAT,
        766 => gen::r1_20_6::CUSTOM_STAT,
        767 => gen::r1_21_1::CUSTOM_STAT,
        768 => gen::r1_21_3::CUSTOM_STAT,
        769 => gen::r1_21_4::CUSTOM_STAT,
        770 => gen::r1_21_5::CUSTOM_STAT,
        771 => gen::r1_21_6::CUSTOM_STAT,
        772 => gen::r1_21_8::CUSTOM_STAT,
        773 => gen::r1_21_10::CUSTOM_STAT,
        774 => gen::r1_21_11::CUSTOM_STAT,
        775 => gen::r26_1::CUSTOM_STAT,
        776 => gen::r26_2::CUSTOM_STAT,
        _ => gen::r26_3::CUSTOM_STAT,
    }
}

/// Таблицы «id версии → id канона» по типу статистики.
struct Remap {
    block: Vec<Option<u32>>,
    item: Vec<Option<u32>>,
    entity: Vec<Option<u32>>,
    custom: Vec<Option<u32>>,
}

impl Remap {
    fn new(v: Version) -> Self {
        // VANILLA: StatsRenameFix — `play_one_minute` → `play_time` (1.17).
        let custom: Vec<&str> = custom_stats(v.protocol)
            .iter()
            .map(|&n| match n {
                "play_one_minute" => "play_time",
                n => n,
            })
            .collect();
        Remap {
            block: build_remap(v.block, gen::r26_3::BLOCK),
            item: build_remap(v.item, gen::r26_3::ITEM),
            entity: build_remap(v.entity_type, gen::r26_3::ENTITY_TYPE),
            custom: build_remap(&custom, gen::r26_3::CUSTOM_STAT),
        }
    }

    fn get(&self, ty: i32, value: i32) -> Option<i32> {
        let table = match ty {
            MINED => &self.block,
            CRAFTED..=DROPPED => &self.item,
            KILLED | KILLED_BY => &self.entity,
            CUSTOM => &self.custom,
            _ => return None,
        };
        let id = table.get(usize::try_from(value).ok()?).copied().flatten()?;
        i32::try_from(id).ok()
    }
}

/// Счётчики ≤1.6.4, которые ведёт только клиент; значение — числовой id.
/// VANILLA: Stats (1.6.4), `localOnly`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Local {
    LeaveGame = 1004,
    PlayTime = 1100,
    Walk = 2000,
    Swim = 2001,
    Fall = 2002,
    Climb = 2003,
    Fly = 2004,
    Dive = 2005,
    Minecart = 2006,
    Boat = 2007,
    Pig = 2008,
    Jump = 2010,
}

/// Как часто изменившиеся суммы пишутся в файл.
const SAVE_EVERY: Duration = Duration::from_secs(10);

struct Totals {
    /// id статистики ≤1.6.4 → сумма.
    map: BTreeMap<i32, i32>,
    path: Option<PathBuf>,
    dirty: bool,
    saved: Option<Instant>,
}

static TOTALS: Mutex<Totals> = Mutex::new(Totals {
    map: BTreeMap::new(),
    path: None,
    dirty: false,
    saved: None,
});

fn totals() -> MutexGuard<'static, Totals> {
    TOTALS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Ключ 1.7 для id ≤1.6.4. `None` — достижение или статистика без пары в каноне.
fn numeric_key(id: i32) -> Option<String> {
    let name = match id {
        1004 => "leaveGame",
        1100 => "playOneMinute",
        2000 => "walkOneCm",
        2001 => "swimOneCm",
        2002 => "fallOneCm",
        2003 => "climbOneCm",
        2004 => "flyOneCm",
        2005 => "diveOneCm",
        2006 => "minecartOneCm",
        2007 => "boatOneCm",
        2008 => "pigOneCm",
        2010 => "jump",
        2020 => "damageDealt",
        2021 => "damageTaken",
        2022 => "deaths",
        2023 => "mobKills",
        2024 => "playerKills",
        2025 => "fishCaught",
        _ => {
            let kind = match id >> 16 {
                0x100 => "mineBlock",
                0x101 => "craftItem",
                0x102 => "useItem",
                0x103 => "breakItem",
                _ => return None,
            };
            return Some(format!("stat.{kind}.{}", id & 0xFFFF));
        }
    };
    Some(format!("stat.{name}"))
}

/// Приращения сервера для клиентских счётчиков ваниль отбрасывает.
/// VANILLA: ControllablePlayerEntity#method_1261 (1.6.4): `!localOnly`.
fn is_local(id: i32) -> bool {
    (1000..2020).contains(&id)
}

impl Totals {
    fn add(&mut self, id: i32, amount: i32) -> i32 {
        let total = self.map.entry(id).or_insert(0);
        *total = total.saturating_add(amount);
        self.dirty = true;
        *total
    }

    fn save(&mut self) {
        let Some(path) = self.path.as_deref().filter(|_| self.dirty) else {
            return;
        };
        if let Err(e) = write(path, &self.map) {
            tracing::warn!("статистика: не записан {}: {e}", path.display());
        }
        self.dirty = false;
        self.saved = Some(Instant::now());
    }
}

fn write(path: &Path, map: &BTreeMap<i32, i32>) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, serde_json::to_vec(map)?)?;
    std::fs::rename(&tmp, path)
}

/// Имя файла сумм игрока: в нике остаются только буквы, цифры и `_`.
fn file_for(name: &str) -> PathBuf {
    let safe: String = name
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    PathBuf::from("stats").join(format!("legacy_{safe}.json"))
}

/// Новое соединение ≤1.6.4: суммы игрока читаются из файла (нет файла — с нуля).
pub(crate) fn open_totals(name: &str) {
    let mut t = totals();
    t.save();
    let path = file_for(name);
    t.map = match std::fs::read(&path) {
        Ok(data) => serde_json::from_slice(&data).unwrap_or_else(|e| {
            tracing::warn!("статистика: не разобран {}: {e}", path.display());
            BTreeMap::new()
        }),
        Err(_) => BTreeMap::new(),
    };
    t.path = Some(path);
    t.dirty = false;
}

/// Приращение сервера ≤1.6.4 → ключ 1.7 и новая сумма.
pub(crate) fn add_increment(id: i32, amount: i32) -> Option<(String, i32)> {
    if is_local(id) {
        return None;
    }
    let key = numeric_key(id)?;
    let total = totals().add(id, amount);
    Some((key, total))
}

/// Приращение клиентского счётчика; на версиях, где статистику считает сервер, ничего не делает.
pub fn add_local(stat: Local, amount: i32) {
    if version::current().protocol < 0 {
        totals().add(stat as i32, amount);
    }
}

/// Пишет изменившиеся суммы не чаще раза в [`SAVE_EVERY`]; `now` — сразу.
pub fn save_totals(now: bool) {
    let mut t = totals();
    if now || t.saved.is_none_or(|at| at.elapsed() >= SAVE_EVERY) {
        t.save();
    }
}

/// ≤1.6.4: запроса статистики в протоколе нет — тело `award_stats` канона со всеми суммами.
/// `None` — версия присылает статистику сама.
pub fn local_snapshot() -> ProtocolResult<Option<Vec<u8>>> {
    if version::current().protocol >= 0 {
        return Ok(None);
    }
    let entries: Vec<(String, i32)> = totals()
        .map
        .iter()
        .filter_map(|(&id, &total)| Some((numeric_key(id)?, total)))
        .collect();
    let mut w = Writer::new();
    w.var_int(i32::try_from(entries.len()).unwrap_or(i32::MAX));
    for (key, total) in entries {
        w.string(&key).var_int(total);
    }
    to_canon(&w.into_vec()).map(Some)
}

/// Тело `award_stats` версии соединения → тело канона; несопоставимые записи выпадают.
pub fn to_canon(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let v = version::current();
    if v.is_canon() {
        return Ok(body.to_vec());
    }
    let remap = (v.protocol > 340).then(|| Remap::new(v));
    let mut r = Reader::new(body);
    let n = r.var_int()?.max(0);
    let mut out = Vec::new();
    for _ in 0..n {
        let entry = match &remap {
            None => legacy_key(&r.string(32767)?),
            Some(m) => {
                let (ty, value) = (r.var_int()?, r.var_int()?);
                m.get(ty, value).map(|value| (ty, value))
            }
        };
        let amount = r.var_int()?;
        if let Some((ty, value)) = entry {
            out.push((ty, value, amount));
        }
    }
    let mut w = Writer::new();
    w.var_int(i32::try_from(out.len()).unwrap_or(i32::MAX));
    for (ty, value, amount) in out {
        w.var_int(ty).var_int(value).var_int(amount);
    }
    Ok(w.into_vec())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn id(list: &[&str], name: &str) -> i32 {
        canon_index(list, name).unwrap()
    }

    #[test]
    fn custom_1_13_names_resolve() {
        let m = Remap::new(version::V1_13_2);
        assert!(m.custom.iter().all(Option::is_some), "{:?}", m.custom);
        for (_, new) in CUSTOM_1_12 {
            id(gen::r26_3::CUSTOM_STAT, new);
        }
    }

    #[test]
    fn increments_accumulate() {
        let stone = 0x0100_0000 + 1;
        // id, которых не трогают другие тесты: суммы общие на процесс
        assert_eq!(
            add_increment(stone, 2),
            Some(("stat.mineBlock.1".to_owned(), 2))
        );
        assert_eq!(
            add_increment(stone, 3),
            Some(("stat.mineBlock.1".to_owned(), 5))
        );
        // прыжки считает только клиент; 0x50_0000 — достижение
        assert_eq!(add_increment(Local::Jump as i32, 1), None);
        assert_eq!(add_increment(0x50_0000, 1), None);
    }

    #[test]
    fn local_stats_have_canon_pairs() {
        for stat in [
            Local::LeaveGame,
            Local::PlayTime,
            Local::Walk,
            Local::Swim,
            Local::Fall,
            Local::Climb,
            Local::Fly,
            Local::Dive,
            Local::Minecart,
            Local::Boat,
            Local::Pig,
            Local::Jump,
        ] {
            let key = numeric_key(stat as i32).unwrap();
            assert!(legacy_key(&key).is_some(), "{key}");
        }
    }

    #[test]
    fn file_name_is_safe() {
        assert_eq!(
            file_for("../Ezz Bot"),
            PathBuf::from("stats").join("legacy_ezzbot.json")
        );
    }

    #[test]
    fn legacy_entities() {
        let entity = |n| id(gen::r26_3::ENTITY_TYPE, n);
        assert_eq!(
            legacy_key("stat.killEntity.Zombie"),
            Some((KILLED, entity("zombie")))
        );
        assert_eq!(
            legacy_key("stat.entityKilledBy.CaveSpider"),
            Some((KILLED_BY, entity("cave_spider")))
        );
        assert_eq!(
            legacy_key("stat.killEntity.PigZombie"),
            Some((KILLED, entity("zombified_piglin")))
        );
    }

    #[test]
    fn legacy_custom_and_items() {
        let custom = |n| id(gen::r26_3::CUSTOM_STAT, n);
        let item = |n| id(gen::r26_3::ITEM, n);
        assert_eq!(legacy_key("stat.drop"), Some((CUSTOM, custom("drop"))));
        assert_eq!(
            legacy_key("stat.playOneMinute"),
            Some((CUSTOM, custom("play_time")))
        );
        assert_eq!(legacy_key("stat.junkFished"), None);
        assert_eq!(
            legacy_key("stat.useItem.minecraft.stone"),
            Some((USED, item("stone")))
        );
        // 1.7: числовой id; имя `bed` — предмет 355, а не блок 26.
        assert_eq!(
            legacy_key("stat.pickup.1"),
            Some((PICKED_UP, item("stone")))
        );
        assert_eq!(item_1_12("minecraft.bed"), item_1_12("355"));
    }
}
