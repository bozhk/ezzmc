//! Вход на сервер 1.20.1 (протокол 763), в том числе Forge (FML3) с подделкой хендшейка.
//! 1.20.1: нет фазы Configuration; реестры приходят внутри Play `login` (NBT-кодек).
//! Формат FML3 — `docs/ref/handoff-2026-11-13-forge-1201.md`.
// VANILLA: ClientHandshakePacketListenerImpl (1.20.1), ClientboundLoginPacket (1.20.1)
use crate::conn::Connection;
use crate::session::{PacketIds, RegistryData, ServerConfig};
use crate::{NetError, NetResult};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::text::Component;
use ezzmc_protocol::types::{offline_uuid, GameProfile};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, ProtocolError, ProtocolResult, State};

const LOGIN_WRAPPER: &str = "fml:loginwrapper";
const HANDSHAKE_CHANNEL: &str = "fml:handshake";

/// Суффикс поля хоста в Handshake для Forge (FML3).
pub const FML3_SUFFIX: &str = "\0FML3\0";

/// Результат входа: профиль, реестры из `login`, сам пакет `login` (возвращается в очередь приёма).
pub struct Joined {
    pub profile: GameProfile,
    pub cfg: ServerConfig,
}

/// Ответ клиента на сообщение `fml:handshake` (внутреннее тело) или `None` — ответа нет.
/// VANILLA-UNSURE: подделка — моды и каналы эхом из ModList, реестры пустые.
fn fml_reply(inner: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(inner);
    let mut w = Writer::new();
    match r.u8()? {
        // S2CModList → C2SModListReply (дискриминатор 2)
        1 => {
            let n = r.length("fml mods", 4096)?;
            let mods: Vec<String> = (0..n).map(|_| r.string(256)).collect::<Result<_, _>>()?;
            let n = r.length("fml channels", 8192)?;
            let chans: Vec<(String, String)> = (0..n)
                .map(|_| Ok((r.string(32767)?, r.string(32767)?)))
                .collect::<ProtocolResult<_>>()?;
            w.u8(2).var_int(mods.len() as i32);
            mods.iter().for_each(|m| {
                w.string(m);
            });
            w.var_int(chans.len() as i32);
            chans.iter().for_each(|(c, v)| {
                w.string(c).string(v);
            });
            w.var_int(0);
        }
        // S2CRegistry: снимок minecraft:block → карта модовых состояний (DEC-018)
        3 => {
            if let Err(e) = read_registry_snapshot(&mut r) {
                tracing::warn!("Forge: снимок реестра не разобран: {e}");
            }
            w.u8(99);
        }
        // S2CConfigData → C2SAcknowledge (99)
        4 => {
            w.u8(99);
        }
        // S2CModData — ответа нет
        5 => return Ok(None),
        6 => {
            return Err(ProtocolError::Other(
                "Forge: ChannelMismatch (сервер отверг список модов)".into(),
            ))
        }
        d => {
            return Err(ProtocolError::Other(format!(
                "Forge: неизвестное сообщение хендшейка {d}"
            )))
        }
    }
    Ok(Some(w.into_vec()))
}

/// `S2CRegistry`: `identifier name, bool has, [snapshot]`; snapshot =
/// `map<identifier, varint> ids, map<identifier, identifier> aliases, map<identifier, string> overrides,
/// list<varint> blocked`. VANILLA (Forge 47.4.28): HandshakeMessages$S2CRegistry#decode,
/// ForgeRegistry$Snapshot#read(FriendlyByteBuf). Нужен только `minecraft:block`.
fn read_registry_snapshot(r: &mut Reader) -> ProtocolResult<()> {
    let name = r.string(32767)?;
    let wanted = matches!(
        name.as_str(),
        "minecraft:block" | "minecraft:menu" | "minecraft:item" | "minecraft:enchantment"
    );
    if !wanted || !r.bool()? {
        return Ok(());
    }
    let n = r.length("fml registry ids", 1 << 20)?;
    let mut ids = Vec::with_capacity(n);
    for _ in 0..n {
        let key = r.string(32767)?;
        let id = r.var_int()?;
        let id = u32::try_from(id)
            .ok()
            .filter(|&i| i < 1 << 20)
            .ok_or_else(|| ProtocolError::Other(format!("fml registry: id {id}")))?;
        ids.push((id, key));
    }
    if name != "minecraft:block" {
        ezzmc_world::modded::set_server_registry(&name, &ids);
        tracing::info!("Forge: реестр {name} сервера: {n} записей");
        return Ok(());
    }
    let (mapped, missing) = ezzmc_world::modded::set_server_blocks(
        &mut ids,
        &ezzmc_world::legacy::vanilla_1_20_1_state_count,
        ezzmc_world::legacy::vanilla_1_20_1_states(),
    );
    tracing::info!(
        "Forge: реестр блоков сервера {n}, модовых состояний сопоставлено {mapped}, блоков без дампа {missing}"
    );
    Ok(())
}

/// Тело ответа `custom_query` (serverbound) на запрос Forge/неизвестного канала.
fn answer_query(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let tx = r.var_int()?;
    let channel = r.string(32767)?;
    let mut w = Writer::new();
    w.var_int(tx);
    if channel != LOGIN_WRAPPER {
        w.bool(false);
        return Ok(w.into_vec());
    }
    let inner_channel = r.string(32767)?;
    let n = r.length("fml wrapper", 1 << 21)?;
    let inner = r.bytes(n)?;
    if inner_channel != HANDSHAKE_CHANNEL {
        w.bool(false);
        return Ok(w.into_vec());
    }
    match fml_reply(inner)? {
        Some(reply) => {
            w.bool(true).string(HANDSHAKE_CHANNEL);
            w.var_int(reply.len() as i32).bytes(&reply);
            Ok(w.into_vec())
        }
        None => Ok(Vec::new()),
    }
}

/// Offline-вход 1.20.1 (+ FML3-хендшейк, если сервер его запросит) до первого Play-пакета `login`.
/// Handshake (с суффиксом `FML3`) отправляет вызывающий (`FML3_SUFFIX`).
pub fn login(conn: &mut Connection, ids: &PacketIds, name: &str) -> NetResult<Joined> {
    ezzmc_world::modded::clear_server();
    use Direction::{Clientbound as C, Serverbound as S};
    let mut w = Writer::new();
    // VANILLA: ServerboundHelloPacket (1.20.1): name, Optional<UUID>
    w.string(name).bool(true).uuid(offline_uuid(name));
    conn.send(ids.get(State::Login, S, "hello")?, &w.buf)?;
    let profile = loop {
        let p = conn.recv()?;
        match ids.name_of(State::Login, C, p.id) {
            Some("login_disconnect") => {
                let s = Reader::new(&p.body).string(262144)?;
                let text = Component::from_json_str(&s)
                    .map(|c| c.to_plain(&|_| None))
                    .unwrap_or(s);
                return Err(NetError::Disconnected(text));
            }
            Some("hello") => {
                return Err(NetError::Disconnected(
                    "сервер в online-mode: нужен вход через лаунчер (Microsoft)".into(),
                ))
            }
            Some("login_compression") => conn.set_compression(Reader::new(&p.body).var_int()?),
            Some("custom_query") => {
                let reply = answer_query(&p.body)?;
                if !reply.is_empty() {
                    conn.send(ids.get(State::Login, S, "custom_query")?, &reply)?;
                }
            }
            Some("login_finished") => {
                // 1.20.1: uuid, имя (без свойств)
                let mut r = Reader::new(&p.body);
                let id = r.uuid()?;
                let name = r.string(16)?;
                break GameProfile {
                    id,
                    name,
                    properties: Vec::new(),
                };
            }
            _ => {}
        }
    };
    // Login → Play напрямую (нет Configuration).
    conn.set_state_raw(State::Play);
    // Ждём `login`; всё, что пришло раньше (Forge custom_payload), — в очередь приёма.
    let mut early = Vec::new();
    let login_pkt = loop {
        let p = conn.recv()?;
        if ids.name_of(State::Play, C, p.id) == Some("login") {
            break p;
        }
        early.push(p);
    };
    let cfg = ServerConfig {
        registries: parse_login_registries(&login_pkt.body)?,
        ..ServerConfig::default()
    };
    conn.push_front(login_pkt);
    for p in early.into_iter().rev() {
        conn.push_front(p);
    }
    Ok(Joined { profile, cfg })
}

/// Тело `login` 1.20.1 → (реестры, остальное). Формат:
/// int id, bool hardcore, i8 gameType, i8 prevGameType, var n + string levels, NBT registries,
/// string dimensionType, string dimension, long seed, var maxPlayers, var chunkRadius, var simDist,
/// bool reducedDebug, bool showDeathScreen, bool isDebug, bool isFlat, opt<GlobalPos> lastDeath, var portalCooldown.
struct LoginBody {
    player_id: i32,
    hardcore: bool,
    game_type: i8,
    prev_game_type: i8,
    levels: Vec<String>,
    registries: Tag,
    dimension_type: String,
    dimension: String,
    seed: i64,
    max_players: i32,
    chunk_radius: i32,
    sim_distance: i32,
    reduced_debug: bool,
    show_death: bool,
    is_debug: bool,
    is_flat: bool,
    last_death: Option<(String, (i32, i32, i32))>,
    portal_cooldown: i32,
}

fn read_login(body: &[u8]) -> ProtocolResult<LoginBody> {
    let mut r = Reader::new(body);
    let player_id = r.i32()?;
    let hardcore = r.bool()?;
    let game_type = r.i8()?;
    let prev_game_type = r.i8()?;
    let n = r.length("levels", 1024)?;
    let levels = (0..n).map(|_| r.string(32767)).collect::<Result<_, _>>()?;
    let registries = nbt::read_named(&mut r)?
        .map(|(_, t)| t)
        .ok_or_else(|| ProtocolError::Other("login: пустой NBT реестров".into()))?;
    Ok(LoginBody {
        player_id,
        hardcore,
        game_type,
        prev_game_type,
        levels,
        registries,
        dimension_type: r.string(32767)?,
        dimension: r.string(32767)?,
        seed: r.i64()?,
        max_players: r.var_int()?,
        chunk_radius: r.var_int()?,
        sim_distance: r.var_int()?,
        reduced_debug: r.bool()?,
        show_death: r.bool()?,
        is_debug: r.bool()?,
        is_flat: r.bool()?,
        last_death: r.optional(|r| Ok((r.string(32767)?, r.block_pos()?)))?,
        portal_cooldown: r.var_int()?,
    })
}

/// Реестры из NBT-кодека 1.20.1: `{ "minecraft:x": { type, value: [ {name, id, element} ] } }`,
/// записи по возрастанию `id`.
fn parse_login_registries(body: &[u8]) -> ProtocolResult<Vec<RegistryData>> {
    let l = read_login(body)?;
    let Tag::Compound(regs) = &l.registries else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for (name, reg) in regs {
        let Some(Tag::List(values)) = reg.get("value") else {
            continue;
        };
        let mut entries: Vec<(i64, String, Option<Tag>)> = values
            .iter()
            .filter_map(|v| {
                Some((
                    v.get("id")?.as_i64()?,
                    v.get("name")?.as_str()?.to_owned(),
                    v.get("element").cloned(),
                ))
            })
            .collect();
        entries.sort_by_key(|e| e.0);
        let norm = |t: Option<Tag>| match name.as_str() {
            "minecraft:dimension_type" => t.map(dimension_to_canon),
            "minecraft:worldgen/biome" => t.map(biome_to_canon),
            _ => t,
        };
        out.push(RegistryData {
            registry: name.clone(),
            entries: entries.into_iter().map(|(_, n, t)| (n, norm(t))).collect(),
        });
    }
    // У 1.20.1 зачарований в кодеке нет — подсказкам нужен реестр (TODO-508c).
    out.push(enchant_registry());
    Ok(out)
}

fn put(t: &mut Vec<(String, Tag)>, k: &str, v: Tag) {
    t.retain(|(n, _)| n != k);
    t.push((k.to_owned(), v));
}

/// dimension_type 1.20.1/1.21.10 → поля канона (`has_fixed_time`, `skybox`, `cardinal_light`).
pub(crate) fn dimension_to_canon(t: Tag) -> Tag {
    let Tag::Compound(mut v) = t else { return t };
    let fixed = v.iter().any(|(k, _)| k == "fixed_time");
    let fx = v
        .iter()
        .find(|(k, _)| k == "effects")
        .and_then(|(_, t)| t.as_str().map(str::to_owned))
        .unwrap_or_default();
    put(&mut v, "has_fixed_time", Tag::Byte(i8::from(fixed)));
    let (sky, card) = match fx.trim_start_matches("minecraft:") {
        "the_end" => ("end", "default"),
        "the_nether" => ("none", "nether"),
        _ => ("overworld", "default"),
    };
    put(&mut v, "skybox", Tag::String(sky.into()));
    put(&mut v, "cardinal_light", Tag::String(card.into()));
    Tag::Compound(v)
}

/// biome 1.20.1/1.21.10 → канон: цвета неба/тумана из `effects` в `attributes`.
pub(crate) fn biome_to_canon(t: Tag) -> Tag {
    let Tag::Compound(mut v) = t else { return t };
    let mut attrs = Vec::new();
    if let Some(fx) = v
        .iter()
        .find(|(k, _)| k == "effects")
        .map(|(_, t)| t.clone())
    {
        for k in ["sky_color", "fog_color", "water_fog_color"] {
            if let Some(c) = fx.get(k) {
                attrs.push((format!("minecraft:visual/{k}"), c.clone()));
            }
        }
    }
    put(&mut v, "attributes", Tag::Compound(attrs));
    Tag::Compound(v)
}

static DIM_TYPES: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn dim_index(name: &str) -> i32 {
    DIM_TYPES
        .lock()
        .ok()
        .and_then(|g| g.iter().position(|n| n == name))
        .unwrap_or(0) as i32
}

/// `login` 1.20.1 → канон (`CommonPlayerSpawnInfo` внутри; реестры в канон не пишутся —
/// клиент берёт их из `ServerConfig`).
pub fn login_to_canon(body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let l = read_login(body)?;
    // Индекс типа измерения = позиция в реестре dimension_type (по id).
    if let Some(r) = parse_login_registries(body)?
        .iter()
        .find(|r| r.registry == "minecraft:dimension_type")
    {
        if let Ok(mut g) = DIM_TYPES.lock() {
            *g = r.entries.iter().map(|(n, _)| n.clone()).collect();
        }
    }
    PLAYER_ID.store(l.player_id, std::sync::atomic::Ordering::Relaxed);
    let dim_idx = dim_index(&l.dimension_type);
    let mut w = Writer::new();
    w.i32(l.player_id).bool(l.hardcore);
    w.var_int(l.levels.len() as i32);
    l.levels.iter().for_each(|s| {
        w.string(s);
    });
    w.var_int(l.max_players)
        .var_int(l.chunk_radius)
        .var_int(l.sim_distance)
        .bool(l.reduced_debug)
        .bool(l.show_death)
        .bool(false); // do_limited_crafting
                      // CommonPlayerSpawnInfo (26.3)
    w.var_int(dim_idx as i32).string(&l.dimension).i64(l.seed);
    w.u8(l.game_type as u8).u8(l.prev_game_type as u8);
    w.bool(l.is_debug).bool(l.is_flat);
    match &l.last_death {
        Some((d, (x, y, z))) => {
            w.bool(true).string(d).block_pos(*x, *y, *z);
        }
        None => {
            w.bool(false);
        }
    }
    w.var_int(l.portal_cooldown).var_int(63); // sea_level (VANILLA: по умолчанию 63)
    w.bool(false); // enforces_secure_chat
    Ok(w.into_vec())
}

/// 1.20.1 ItemStack (bool, id, byte count, NBT) → канон (count, id, патч из NBT, TODO-508c).
fn item_to_canon(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.var_int(0);
        return Ok(());
    }
    let id = r.var_int()?;
    let count = r.i8()?;
    let tag = nbt::read_named(r)?.map(|(_, t)| t);
    // Модовый предмет Forge (снимок FML minecraft:item) → MOD_ITEM_BASE + id сервера (TODO-508).
    let canon = u32::try_from(id)
        .ok()
        .and_then(ezzmc_world::modded::modded_item)
        .or_else(|| {
            item_map()
                .get(usize::try_from(id).unwrap_or(usize::MAX))
                .copied()
                .flatten()
        });
    match canon {
        Some(c) if count > 0 => {
            let book =
                ezzmc_protocol::generated::r26_3::ITEM.get(c as usize) == Some(&"enchanted_book");
            let added = tag
                .as_ref()
                .map(|t| nbt_to_components(t, book))
                .unwrap_or_default();
            w.var_int(i32::from(count))
                .var_int(c as i32)
                .var_int(added.len() as i32)
                .var_int(0);
            for (id, v) in added {
                w.var_int(id as i32).bytes(&v);
            }
        }
        _ => {
            w.var_int(0);
        }
    }
    Ok(())
}

/// Номер типа компонента 26.3 по имени.
fn component_id(name: &str) -> Option<u32> {
    ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE
        .iter()
        .position(|n| *n == name)
        .map(|i| i as u32)
}

/// VANILLA: max_level ванильных зачарований (data/minecraft/enchantment/*.json, 26.3; у
/// 1.20.1 те же значения — VANILLA-UNSURE, исходника 1.20.1 нет). Для подсказки уровня.
const ENCHANT_MAX: &[(&str, i32)] = &[
    ("aqua_affinity", 1),
    ("bane_of_arthropods", 5),
    ("binding_curse", 1),
    ("blast_protection", 4),
    ("channeling", 1),
    ("depth_strider", 3),
    ("efficiency", 5),
    ("feather_falling", 4),
    ("fire_aspect", 2),
    ("fire_protection", 4),
    ("flame", 1),
    ("fortune", 3),
    ("frost_walker", 2),
    ("impaling", 5),
    ("infinity", 1),
    ("knockback", 2),
    ("looting", 3),
    ("loyalty", 3),
    ("luck_of_the_sea", 3),
    ("lure", 3),
    ("mending", 1),
    ("multishot", 1),
    ("piercing", 4),
    ("power", 5),
    ("projectile_protection", 4),
    ("protection", 4),
    ("punch", 2),
    ("quick_charge", 3),
    ("respiration", 3),
    ("riptide", 3),
    ("sharpness", 5),
    ("silk_touch", 1),
    ("smite", 5),
    ("soul_speed", 3),
    ("sweeping_edge", 3),
    ("swift_sneak", 3),
    ("thorns", 3),
    ("unbreaking", 3),
    ("vanishing_curse", 1),
];

/// Имя зачарования 1.20.1 → 26.3 (VANILLA: datafix переименование `sweeping` → `sweeping_edge`, 1.20.5).
fn enchant_canon_name(n: &str) -> String {
    let full = if n.contains(':') {
        n.to_owned()
    } else {
        format!("minecraft:{n}")
    };
    if full == "minecraft:sweeping" {
        "minecraft:sweeping_edge".to_owned()
    } else {
        full
    }
}

/// Реестр зачарований сервера 1.20.1 (у него нет registry_data): снимок FML, иначе ванильный
/// список 1.20.1. Holder id канона = индекс в этом списке.
fn enchant_names() -> Vec<String> {
    if let Some(fml) = ezzmc_world::modded::server_names("minecraft:enchantment") {
        return fml.iter().map(|n| enchant_canon_name(n)).collect();
    }
    ezzmc_protocol::generated::r1_20_1::ENCHANTMENT
        .iter()
        .map(|n| enchant_canon_name(n))
        .collect()
}

fn enchant_holder(name: &str) -> Option<u32> {
    let want = enchant_canon_name(name);
    enchant_names()
        .iter()
        .position(|n| *n == want)
        .map(|i| i as u32)
}

/// Синтетический `registry_data minecraft:enchantment` для подсказок (имена + max_level).
fn enchant_registry() -> RegistryData {
    RegistryData {
        registry: "minecraft:enchantment".into(),
        entries: enchant_names()
            .into_iter()
            .map(|n| {
                let max = ENCHANT_MAX
                    .iter()
                    .find(|(k, _)| n.strip_prefix("minecraft:") == Some(*k))
                    .map_or(1, |(_, m)| *m);
                (
                    n,
                    Some(Tag::Compound(vec![("max_level".into(), Tag::Int(max))])),
                )
            })
            .collect(),
    }
}

/// NBT предмета 1.20.1 → компоненты 26.3 (id, точные байты значения).
/// VANILLA: ItemStackComponentizationFix#fixItemStack/fixDisplay/fixEnchantments (26.3):
/// Damage, RepairCost, Unbreakable, Enchantments, StoredEnchantments (книга), display.Name/Lore/color.
/// HideFlags (show_in_tooltip → tooltip_display) не переносится.
fn nbt_to_components(t: &Tag, book: bool) -> Vec<(u32, Vec<u8>)> {
    let mut out: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut put = |name: &str, v: Vec<u8>| {
        if let Some(id) = component_id(name) {
            out.push((id, v));
        }
    };
    let int = |k: &str| t.get(k).and_then(Tag::as_i64);
    for (k, c) in [("Damage", "damage"), ("RepairCost", "repair_cost")] {
        if let Some(v) = int(k).and_then(|v| i32::try_from(v).ok()) {
            // VANILLA: moveTagToComponent с умолчанием 0 — нулевое значение не переносится.
            if v != 0 {
                let mut w = Writer::new();
                w.var_int(v);
                put(c, w.into_vec());
            }
        }
    }
    if int("Unbreakable").is_some_and(|v| v != 0) {
        put("unbreakable", Vec::new());
    }
    let mut ench = |key: &str, comp: &str| {
        let Some(Tag::List(list)) = t.get(key) else {
            return;
        };
        let levels: Vec<(u32, i32)> = list
            .iter()
            .filter_map(|e| {
                let id = e.get("id")?.as_str()?;
                let lvl = e.get("lvl")?.as_i64()?.clamp(0, 255) as i32;
                (lvl > 0).then_some((enchant_holder(id)?, lvl))
            })
            .collect();
        if levels.is_empty() {
            let mut w = Writer::new();
            w.bool(true);
            put("enchantment_glint_override", w.into_vec());
            return;
        }
        let mut w = Writer::new();
        w.var_int(levels.len() as i32);
        for (h, l) in levels {
            w.var_int(h as i32).var_int(l);
        }
        put(comp, w.into_vec());
    };
    ench("Enchantments", "enchantments");
    if book {
        ench("StoredEnchantments", "stored_enchantments");
    }
    if let Some(d) = t.get("display") {
        let valid = |s: &str| serde_json::from_str::<serde_json::Value>(s).is_ok();
        if let Some(n) = d.get("Name").and_then(Tag::as_str).filter(|s| valid(s)) {
            put("custom_name", json_component(n));
        }
        if let Some(Tag::List(lore)) = d.get("Lore") {
            let lines: Vec<&str> = lore
                .iter()
                .filter_map(Tag::as_str)
                .filter(|s| valid(s))
                .take(256)
                .collect();
            let mut w = Writer::new();
            w.var_int(lines.len() as i32);
            for l in lines {
                w.bytes(&json_component(l));
            }
            put("lore", w.into_vec());
        }
        if let Some(c) = d.get("color").and_then(Tag::as_i64) {
            let mut w = Writer::new();
            w.i32(c as i32);
            put("dyed_color", w.into_vec());
        }
    }
    out
}

fn item_map() -> &'static [Option<u32>] {
    static T: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::version::V1_20_1.item,
            ezzmc_protocol::version::CANON.item,
        )
    })
}

fn menu_map() -> &'static [Option<u32>] {
    static T: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::generated::r1_20_1::MENU,
            ezzmc_protocol::generated::r26_3::MENU,
        )
    })
}

/// Тип меню сервера 1.20.1 → канон: ванильное 26.3 по имени, модовое — `MOD_MENU_BASE + id`.
/// Без снимка FML (чистый 1.20.1) — по ванильной таблице 1.20.1.
fn menu_to_canon(id: i32) -> Option<i32> {
    use ezzmc_world::modded::{has_server_registry, server_name, MOD_MENU_BASE};
    let uid = u32::try_from(id).ok()?;
    if !has_server_registry("minecraft:menu") {
        return menu_map()
            .get(uid as usize)
            .copied()
            .flatten()
            .map(|c| c as i32);
    }
    let name = server_name("minecraft:menu", uid)?;
    match name.strip_prefix("minecraft:") {
        Some(v) => ezzmc_protocol::generated::r26_3::MENU
            .iter()
            .position(|m| *m == v)
            .map(|p| p as i32),
        None => Some(MOD_MENU_BASE + id),
    }
}

/// Канал Forge `fml:play` (SimpleChannel): дискриминатор 1 — OpenContainer.
const FML_PLAY: &str = "fml:play";

/// `custom_payload` — это `fml:play` OpenContainer.
fn is_fml_open_container(body: &[u8]) -> bool {
    let mut r = Reader::new(body);
    r.string(32767).is_ok_and(|c| c == FML_PLAY) && r.u8().is_ok_and(|d| d == 1)
}

/// Имя пакета в каноне после перевода тела (клиент разбирает по имени).
pub fn canon_name<'a>(name: &'a str, body: &[u8]) -> &'a str {
    match name {
        // Forge NetworkHooks.openScreen → ванильный open_screen (см. play_to_canon).
        "custom_payload" if is_fml_open_container(body) => "open_screen",
        "player_chat" | "disguised_chat" => "system_chat",
        "set_carried_item" => "set_held_slot",
        // окно -1 в container_set_slot → курсор (тело уже без окна: см. play_to_canon)
        "container_set_slot" if body.first().is_some_and(|b| *b == 0xFF) => "set_cursor_item",
        n => n,
    }
}

/// Канонический HashedStack (present, item, count, 0, 0) → ItemStack 1.20.1 без NBT.
fn hashed_to_1201(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.bool(false);
        return Ok(());
    }
    let (item, count) = (r.var_int()?, r.var_int()?);
    for _ in 0..r.length("hashed added", 256)? {
        r.var_int()?;
        r.i32()?;
    }
    for _ in 0..r.length("hashed removed", 256)? {
        r.var_int()?;
    }
    static BACK: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    let back = BACK.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::version::CANON.item,
            ezzmc_protocol::version::V1_20_1.item,
        )
    });
    let modded = u32::try_from(item)
        .ok()
        .and_then(|i| i.checked_sub(ezzmc_world::modded::MOD_ITEM_BASE));
    match modded.or_else(|| {
        back.get(usize::try_from(item).unwrap_or(usize::MAX))
            .copied()
            .flatten()
    }) {
        Some(id) if count > 0 => {
            w.bool(true)
                .var_int(id as i32)
                .u8(count.min(127) as u8)
                .u8(0);
        }
        _ => {
            w.bool(false);
        }
    }
    Ok(())
}

static PLAYER_TYPE: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "player")
});

fn entity_type_map() -> &'static [Option<u32>] {
    static T: std::sync::OnceLock<Vec<Option<u32>>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        ezzmc_protocol::version::build_remap(
            ezzmc_protocol::version::V1_20_1.entity_type,
            ezzmc_protocol::version::CANON.entity_type,
        )
    })
}

/// VANILLA: LpVec3#write (канон 26.3), масштаб ≤ 3 без продолжения.
fn lp_vec3(w: &mut Writer, v: [f64; 3]) {
    let max = v.iter().fold(0.0f64, |m, c| m.max(c.abs()));
    if max < 1.0 / 32766.0 {
        w.u8(0);
        return;
    }
    let scale = max.ceil().clamp(1.0, 3.0);
    let q = |c: f64| {
        ((c / scale * 0.5 + 0.5) * 32766.0)
            .round()
            .clamp(0.0, 32766.0) as u64
    };
    let buf = scale as u64 | q(v[0]) << 3 | q(v[1]) << 18 | q(v[2]) << 33;
    w.u8(buf as u8)
        .u8((buf >> 8) as u8)
        .i32((buf >> 16) as u32 as i32);
}

// ---------- serverbound: канон → 1.20.1 ----------

static PLAYER_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static SHIFT_DOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn id_of(name: &str) -> Option<i32> {
    ezzmc_protocol::generated::v1_20_1::PACKETS
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.name == name)
        .map(|p| p.id as i32)
}

fn name_of_id(id: i32) -> Option<&'static str> {
    ezzmc_protocol::generated::v1_20_1::PACKETS
        .iter()
        .find(|p| p.state == State::Play && p.dir == Direction::Serverbound && p.id as i32 == id)
        .map(|p| p.name)
}

/// Пакет Play, собранный клиентом в формате канона (id уже из таблицы 1.20.1) → список
/// пакетов 1.20.1 `(id, тело)`. Пустой список — пакет отбрасывается.
pub fn play_from_canon(id: i32, body: &[u8]) -> ProtocolResult<Vec<(i32, Vec<u8>)>> {
    use std::sync::atomic::Ordering::Relaxed;
    let Some(name) = name_of_id(id) else {
        return Ok(Vec::new());
    };
    let mut r = Reader::new(body);
    let mut w = Writer::new();
    match name {
        // flags (бит0 onGround, бит1 horizontalCollision) → bool onGround
        "move_player_pos"
        | "move_player_pos_rot"
        | "move_player_rot"
        | "move_player_status_only" => {
            let n = match name {
                "move_player_pos" => 24,
                "move_player_pos_rot" => 32,
                "move_player_rot" => 8,
                _ => 0,
            };
            w.bytes(r.bytes(n)?).bool(r.u8()? & 1 != 0);
        }
        "accept_teleportation"
        | "keep_alive"
        | "pong"
        | "custom_payload"
        | "swing"
        | "player_action"
        | "set_carried_item"
        | "container_close"
        | "teleport_to_entity"
        // тело одинаково в 1.20.1 и каноне: возрождение (perform_respawn), удар/взаимодействие
        // с сущностью (клиент уже пишет формат ServerboundInteractPacket старых версий), полёт,
        // наковальня, торговля, табличка, подсказки команд, вёсла лодки
        | "client_command"
        | "interact"
        | "player_abilities"
        | "rename_item"
        | "select_trade"
        | "sign_update"
        | "command_suggestion"
        | "paddle_boat" => {
            w.bytes(body);
        }
        // канон + bool onGround в конце; 1.20.1 — без него
        "move_vehicle" => {
            w.bytes(&body[..body.len().saturating_sub(1)]);
        }
        // тело канона = тело 1.20.1 + хвостовой байт checksum
        "chat" | "chat_command" => {
            w.bytes(&body[..body.len().saturating_sub(1)]);
        }
        "client_information" => {
            // без particle_status в конце
            w.string(&r.string(16)?)
                .u8(r.u8()?)
                .var_int(r.var_int()?)
                .bool(r.bool()?)
                .u8(r.u8()?)
                .var_int(r.var_int()?)
                .bool(r.bool()?)
                .bool(r.bool()?);
        }
        "player_command" => {
            let (eid, action, data) = (r.var_int()?, r.var_int()?, r.var_int()?);
            // 1.20.1: перед STOP_SLEEPING ещё два действия шифта
            w.var_int(eid).var_int(action + 2).var_int(data);
        }
        // каноническое Input.shift → press/release shift key (player_command 0/1)
        "player_input" => {
            let shift = r.u8()? & 32 != 0;
            if SHIFT_DOWN.swap(shift, Relaxed) == shift {
                return Ok(Vec::new());
            }
            w.var_int(PLAYER_ID.load(Relaxed))
                .var_int(i32::from(!shift))
                .var_int(0);
            return Ok(id_of("player_command")
                .map(|i| (i, w.into_vec()))
                .into_iter()
                .collect());
        }
        "container_click" => {
            let (win, state) = (r.var_int()?, r.var_int()?);
            w.u8(win as u8)
                .var_int(state)
                .i16(r.i16()?)
                .u8(r.u8()?)
                .var_int(r.var_int()?);
            let n = r.length("changed slots", 128)?;
            w.var_int(n as i32);
            for _ in 0..n {
                w.i16(r.i16()?);
                hashed_to_1201(&mut r, &mut w)?;
            }
            hashed_to_1201(&mut r, &mut w)?;
        }
        "container_button_click" => {
            w.u8(r.var_int()? as u8).u8(r.var_int()? as u8);
        }
        "use_item" => {
            // hand, sequence, yRot, xRot → hand, sequence
            w.var_int(r.var_int()?).var_int(r.var_int()?);
        }
        "use_item_on" => {
            w.var_int(r.var_int()?);
            w.bytes(r.bytes(8)?).var_int(r.var_int()?);
            w.bytes(r.bytes(12)?).bool(r.bool()?);
            r.bool()?; // worldBorderHit — в 1.20.1 нет
            w.var_int(r.var_int()?);
        }
        _ => return Ok(Vec::new()),
    }
    Ok(vec![(id, w.into_vec())])
}

// ---------- прочие пакеты clientbound ----------

fn nbt_bytes(t: &Tag) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = nbt::write_network(&mut out, Some(t));
    out
}

/// JSON-компонент → NBT-тег канона (смешанные списки оборачиваются в `{text:..}`).
fn json_to_tag(v: &serde_json::Value) -> Tag {
    use serde_json::Value as J;
    match v {
        J::String(s) => Tag::String(s.clone()),
        J::Bool(b) => Tag::Byte(i8::from(*b)),
        J::Number(n) => n
            .as_i64()
            .and_then(|i| i32::try_from(i).ok())
            .map_or_else(|| Tag::Double(n.as_f64().unwrap_or(0.0)), Tag::Int),
        J::Object(m) => Tag::Compound(m.iter().map(|(k, v)| (k.clone(), json_to_tag(v))).collect()),
        J::Array(a) => {
            let items: Vec<Tag> = a.iter().map(json_to_tag).collect();
            let same = items.windows(2).all(|w| w[0].id() == w[1].id());
            if same {
                Tag::List(items)
            } else {
                Tag::List(
                    items
                        .into_iter()
                        .map(|t| match t {
                            Tag::Compound(_) => t,
                            Tag::String(s) => Tag::Compound(vec![("text".into(), Tag::String(s))]),
                            o => {
                                Tag::Compound(vec![("text".into(), Tag::String(format!("{o:?}")))])
                            }
                        })
                        .collect(),
                )
            }
        }
        J::Null => Tag::String(String::new()),
    }
}

/// JSON-строка компонента (1.20.1) → NBT-байты канона.
fn json_component(s: &str) -> Vec<u8> {
    let v = serde_json::from_str::<serde_json::Value>(s)
        .unwrap_or_else(|_| serde_json::Value::String(s.to_owned()));
    nbt_bytes(&json_to_tag(&v))
}

/// BitSet (varint + long[]) → bitset_bytes канона.
fn bitset_to_canon(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let longs = r.bitset()?;
    w.bitset_bytes(&longs);
    Ok(())
}

fn light_to_canon(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    for _ in 0..4 {
        bitset_to_canon(r, w)?;
    }
    w.bytes(r.rest()); // списки byte[2048] совпадают
    Ok(())
}

pub(crate) fn heightmap_id(name: &str) -> Option<i32> {
    Some(match name {
        "WORLD_SURFACE_WG" => 0,
        "WORLD_SURFACE" => 1,
        "OCEAN_FLOOR_WG" => 2,
        "OCEAN_FLOOR" => 3,
        "MOTION_BLOCKING" => 4,
        "MOTION_BLOCKING_NO_LEAVES" => 5,
        _ => return None,
    })
}

/// Белый список clientbound-пакетов Play 1.20.1 → тело канона (`None` — отбросить).
pub fn play_to_canon(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::new();
    match name {
        "login" => return Ok(Some(login_to_canon(body)?)),
        "keep_alive" => w.bytes(body),
        // VANILLA: ClientboundLevelChunkWithLightPacket (1.20.1): int x, int z, NBT heightmaps,
        // byte[] секции, список BE, свет. Секции разбираются в Section::read (legacy-ветка).
        "level_chunk_with_light" => {
            w.i32(r.i32()?).i32(r.i32()?);
            let hm = nbt::read_named(&mut r)?.map(|(_, t)| t);
            let mut maps = Vec::new();
            if let Some(Tag::Compound(items)) = hm {
                for (k, v) in items {
                    if let (Some(id), Tag::LongArray(l)) = (heightmap_id(&k), v) {
                        maps.push((id, l));
                    }
                }
            }
            w.var_int(maps.len() as i32);
            for (id, l) in maps {
                w.var_int(id).var_int(l.len() as i32);
                l.iter().for_each(|v| {
                    w.i64(*v);
                });
            }
            let n = r.length("chunk buffer", 2 * 1024 * 1024)?;
            w.var_int(n as i32).bytes(r.bytes(n)?);
            // VANILLA-UNSURE: BE-типы 1.20.1 ≠ канон (реестр block_entity_type) — пока без них.
            let be = r.length("block entities", 65536)?;
            for _ in 0..be {
                r.bytes(3)?; // packedXZ, y
                r.var_int()?;
                nbt::read_named(&mut r)?;
            }
            w.var_int(0);
            light_to_canon(&mut r, &mut w)?;
            return Ok(Some(w.into_vec()));
        }
        "light_update" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            light_to_canon(&mut r, &mut w)?;
            return Ok(Some(w.into_vec()));
        }
        // int x, int z → ChunkPos long
        "forget_level_chunk" => {
            let (x, z) = (r.i32()?, r.i32()?);
            w.i64(ezzmc_protocol::types::ChunkPos { x, z }.as_long())
        }
        "set_chunk_cache_center" | "set_chunk_cache_radius" => w.bytes(body),
        "block_update" => w
            .bytes(r.bytes(8)?)
            .var_int(ezzmc_world::legacy::state_to_canon(r.var_int()? as u32) as i32),
        "section_blocks_update" => {
            w.bytes(r.bytes(8)?);
            let n = r.length("section updates", 4096)?;
            w.var_int(n as i32);
            for _ in 0..n {
                let e = r.var_long()?;
                let st = ezzmc_world::legacy::state_to_canon((e >> 12) as u32);
                w.var_long((i64::from(st) << 12) | (e & 4095));
            }
            return Ok(Some(w.into_vec()));
        }
        // 1.20.1: double x,y,z, float yRot, xRot, byte flags, varint id
        // → канон: id, pos, delta(0), yRot, xRot, int relatives
        "player_position" => {
            let pos = [r.f64()?, r.f64()?, r.f64()?];
            let (yr, xr, fl) = (r.f32()?, r.f32()?, r.u8()?);
            let id = r.var_int()?;
            w.var_int(id);
            pos.iter().chain([0.0, 0.0, 0.0].iter()).for_each(|v| {
                w.f64(*v);
            });
            w.f32(yr).f32(xr).i32(i32::from(fl))
        }
        "player_abilities" | "game_event" | "set_health" => w.bytes(body),
        // long gameTime, long dayTime → gameTime + одни часы (id 0): total, partial, rate
        "set_time" => {
            let (game, day) = (r.i64()?, r.i64()?);
            // отрицательное время — суточный цикл остановлен (doDaylightCycle=false)
            w.i64(game)
                .var_int(1)
                .var_int(0)
                .var_long(day.abs())
                .f32(0.0);
            w.f32(if day < 0 { 0.0 } else { 1.0 })
        }
        // JSON-компонент + bool overlay → NBT
        // ---- сущности ----
        // VANILLA: ClientboundAddEntityPacket (1.20.1): id, uuid, type, xyz, yRot-байты, data, short×3 скорость.
        "add_entity" => {
            let (id, uuid, ty) = (r.var_int()?, r.bytes(16)?, r.var_int()?);
            let Some(canon) = entity_type_map()
                .get(usize::try_from(ty).unwrap_or(usize::MAX))
                .copied()
                .flatten()
            else {
                return Ok(None);
            };
            let pos = [r.f64()?, r.f64()?, r.f64()?];
            let (x_rot, y_rot, head, data) = (r.i8()?, r.i8()?, r.i8()?, r.var_int()?);
            let vel = [r.i16()?, r.i16()?, r.i16()?].map(|v| f64::from(v) / 8000.0);
            w.var_int(id).bytes(uuid).var_int(canon as i32);
            pos.iter().for_each(|v| {
                w.f64(*v);
            });
            lp_vec3(&mut w, vel);
            w.u8(x_rot as u8)
                .u8(y_rot as u8)
                .u8(head as u8)
                .var_int(data)
        }
        "add_player" => {
            let (id, uuid) = (r.var_int()?, r.bytes(16)?);
            let Some(ty) = *PLAYER_TYPE else {
                return Ok(None);
            };
            let pos = [r.f64()?, r.f64()?, r.f64()?];
            let (y_rot, x_rot) = (r.i8()?, r.i8()?);
            w.var_int(id).bytes(uuid).var_int(ty as i32);
            pos.iter().for_each(|v| {
                w.f64(*v);
            });
            lp_vec3(&mut w, [0.0; 3]);
            w.u8(x_rot as u8).u8(y_rot as u8).u8(y_rot as u8).var_int(0)
        }
        "remove_entities" | "rotate_head" | "entity_event" | "animate" => w.bytes(body),
        // 1.20.1 совпадает с 1.21.11
        "move_entity_pos" | "move_entity_pos_rot" | "move_entity_rot" => {
            return crate::legacy::clientbound_1_21_11(name, body);
        }
        "teleport_entity" => {
            let id = r.var_int()?;
            let pos = [r.f64()?, r.f64()?, r.f64()?];
            let (y, x, g) = (r.i8()?, r.i8()?, r.bool()?);
            w.var_int(id);
            pos.iter().for_each(|v| {
                w.f64(*v);
            });
            (0..3).for_each(|_| {
                w.f64(0.0);
            });
            w.f32(f32::from(y) * 360.0 / 256.0)
                .f32(f32::from(x) * 360.0 / 256.0)
                .i32(0)
                .bool(g)
        }
        "set_entity_motion" => {
            w.var_int(r.var_int()?);
            lp_vec3(
                &mut w,
                [r.i16()?, r.i16()?, r.i16()?].map(|v| f64::from(v) / 8000.0),
            );
            w.bytes(&[])
        }
        // действия 1.20.1 = канон; только имя в JSON → NBT
        "player_info_update" => {
            let actions = r.u8()?;
            w.u8(actions);
            let n = r.length("player_info entries", 1024)?;
            w.var_int(n as i32);
            for _ in 0..n {
                w.bytes(r.bytes(16)?);
                for bit in 0..6 {
                    if actions & (1 << bit) == 0 {
                        continue;
                    }
                    match bit {
                        0 => {
                            w.string(&r.string(16)?);
                            let props = r.length("profile properties", 16)?;
                            w.var_int(props as i32);
                            for _ in 0..props {
                                w.string(&r.string(64)?).string(&r.string(32767)?);
                                match r.optional(|r| r.string(1024))? {
                                    Some(sig) => w.bool(true).string(&sig),
                                    None => w.bool(false),
                                };
                            }
                        }
                        1 => {
                            let present = r.bool()?;
                            w.bool(present);
                            if present {
                                w.bytes(r.bytes(24)?);
                                let k = r.length("chat key", 512)?;
                                w.var_int(k as i32).bytes(r.bytes(k)?);
                                let sg = r.length("chat key signature", 4096)?;
                                w.var_int(sg as i32).bytes(r.bytes(sg)?);
                            }
                        }
                        2 | 4 => {
                            w.var_int(r.var_int()?);
                        }
                        3 => {
                            w.bool(r.bool()?);
                        }
                        _ => {
                            match r.optional(|r| r.string(262144))? {
                                Some(c) => w.bool(true).bytes(&json_component(&c)),
                                None => w.bool(false),
                            };
                        }
                    }
                }
            }
            w.bytes(&[])
        }
        "player_info_remove" => w.bytes(body),
        // VANILLA: ClientboundPlayerChatPacket (1.20.1) → системное сообщение в формате `chat.type.text`
        // (подписи и типы чата не нужны: клиент работает без проверки подписей).
        "player_chat" => {
            r.bytes(16)?;
            r.var_int()?;
            if r.bool()? {
                r.bytes(256)?;
            }
            let content = r.string(256)?;
            r.bytes(16)?;
            for _ in 0..r.length("last seen", 20)? {
                if r.var_int()? == 0 {
                    r.bytes(256)?;
                }
            }
            let unsigned = r.optional(|r| r.string(262144))?;
            if r.var_int()? == 2 {
                r.bitset()?;
            }
            r.var_int()?;
            let name = r.string(262144)?;
            let name = serde_json::from_str(&name).unwrap_or(serde_json::Value::String(name));
            let text = match unsigned {
                Some(u) => serde_json::from_str(&u).unwrap_or(serde_json::Value::String(u)),
                None => serde_json::json!({ "text": content }),
            };
            let msg = serde_json::json!({ "translate": "chat.type.text", "with": [name, text] });
            w.bytes(&json_component(&msg.to_string())).bool(false)
        }
        "disguised_chat" => {
            let msg = r.string(262144)?;
            w.bytes(&json_component(&msg)).bool(false)
        }
        // ---- инвентарь ----
        "container_set_content" => {
            w.var_int(i32::from(r.u8()?)).var_int(r.var_int()?);
            let n = r.length("container items", 256)?;
            w.var_int(n as i32);
            for _ in 0..=n {
                item_to_canon(&mut r, &mut w)?;
            }
            w.bytes(&[])
        }
        "container_set_slot" => {
            let win = r.u8()?;
            let state = r.var_int()?;
            let slot = r.i16()?;
            // окно -1 — курсор (в каноне отдельный пакет set_cursor_item); -2 — не переводим
            match win {
                0xFF => {
                    item_to_canon(&mut r, &mut w)?;
                }
                0xFE => return Ok(None),
                _ => {
                    w.var_int(i32::from(win)).var_int(state).i16(slot);
                    item_to_canon(&mut r, &mut w)?;
                }
            }
            w.bytes(&[])
        }
        "container_set_data" => w.var_int(i32::from(r.u8()?)).i16(r.i16()?).i16(r.i16()?),
        "container_close" => w.var_int(i32::from(r.u8()?)),
        "open_screen" => {
            let win = r.var_int()?;
            let id = r.var_int()?;
            let Some(menu) = menu_to_canon(id) else {
                tracing::warn!("1.20.1 open_screen: неизвестный тип меню {id}");
                return Ok(None);
            };
            let title = r.string(262144)?;
            w.var_int(win).var_int(menu).bytes(&json_component(&title))
        }
        // VANILLA (Forge 47.4.28): PlayMessages$OpenContainer#decode — varint тип меню (реестр
        // сервера), varint окно, Component JSON, byte[] extra (данные мода; пока не нужны).
        "custom_payload" if is_fml_open_container(body) => {
            r.string(32767)?;
            r.u8()?;
            let id = r.var_int()?;
            let win = r.var_int()?;
            let title = r.string(262144)?;
            let Some(menu) = menu_to_canon(id) else {
                tracing::warn!("fml:play OpenContainer: неизвестный тип меню {id}");
                return Ok(None);
            };
            tracing::debug!(
                "fml:play OpenContainer: окно {win}, меню {:?}",
                ezzmc_world::modded::server_name("minecraft:menu", id as u32)
            );
            w.var_int(win).var_int(menu).bytes(&json_component(&title))
        }
        "set_carried_item" => w.var_int(i32::from(r.u8()?)),
        "set_experience" => w.bytes(body),
        "set_equipment" => {
            w.var_int(r.var_int()?);
            loop {
                let b = r.u8()?;
                w.u8(b);
                item_to_canon(&mut r, &mut w)?;
                if b & 0x80 == 0 {
                    break;
                }
            }
            w.bytes(&[])
        }
        "system_chat" => {
            let s = r.string(262144)?;
            w.bytes(&json_component(&s)).bool(r.bool()?)
        }
        "disconnect" => w.bytes(&json_component(&r.string(262144)?)),
        "respawn" => {
            let (dt, dim, seed) = (r.string(32767)?, r.string(32767)?, r.i64()?);
            let (gt, pg, dbg, flat, keep) = (r.u8()?, r.u8()?, r.bool()?, r.bool()?, r.u8()?);
            let last = r.optional(|r| Ok((r.string(32767)?, r.block_pos()?)))?;
            let portal = r.var_int()?;
            w.var_int(dim_index(&dt))
                .string(&dim)
                .i64(seed)
                .u8(gt)
                .u8(pg);
            w.bool(dbg).bool(flat);
            match last {
                Some((d, (x, y, z))) => w.bool(true).string(&d).block_pos(x, y, z),
                None => w.bool(false),
            };
            w.var_int(portal).var_int(63).u8(keep)
        }
        _ => return Ok(None),
    };
    Ok(Some(w.into_vec()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn modlist_is_echoed() {
        let mut inner = Writer::new();
        inner.u8(1).var_int(2).string("forge").string("modx");
        inner.var_int(1).string("modx:main").string("1");
        inner.var_int(0).var_int(0);
        let mut w = Writer::new();
        w.var_int(7).string(LOGIN_WRAPPER).string(HANDSHAKE_CHANNEL);
        w.var_int(inner.buf.len() as i32).bytes(&inner.buf);
        let out = answer_query(&w.into_vec()).unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 7);
        assert!(r.bool().unwrap());
        assert_eq!(r.string(99).unwrap(), HANDSHAKE_CHANNEL);
        let n = r.var_int().unwrap() as usize;
        let body = r.bytes(n).unwrap();
        let mut b = Reader::new(body);
        assert_eq!(b.u8().unwrap(), 2);
        assert_eq!(b.var_int().unwrap(), 2);
        assert_eq!(b.string(99).unwrap(), "forge");
        assert_eq!(b.string(99).unwrap(), "modx");
        assert_eq!(b.var_int().unwrap(), 1);
        assert_eq!(b.string(99).unwrap(), "modx:main");
        assert_eq!(b.string(99).unwrap(), "1");
        assert_eq!(b.var_int().unwrap(), 0);
        assert!(b.is_empty());
    }

    #[test]
    fn unknown_channel_answers_null() {
        let mut w = Writer::new();
        w.var_int(3).string("foo:bar").bytes(&[1, 2]);
        let out = answer_query(&w.into_vec()).unwrap();
        assert_eq!(out, vec![3, 0]);
    }

    #[test]
    fn respawn_and_interact_pass_through() {
        for name in ["client_command", "interact"] {
            let id = id_of(name).unwrap();
            let out = play_from_canon(id, &[0, 1, 0]).unwrap();
            assert_eq!(out, vec![(id, vec![0, 1, 0])], "{name}");
        }
    }

    #[test]
    fn modded_item_roundtrip() {
        use ezzmc_world::modded::{set_server_registry, MOD_ITEM_BASE};
        set_server_registry(
            "minecraft:item",
            &[
                (1, "minecraft:stone".into()),
                (1500, "mekanism:steel_ingot".into()),
            ],
        );
        // 1.20.1 ItemStack: present, id, count, NBT (TAG_End)
        let mut w = Writer::new();
        w.bool(true).var_int(1500).u8(5).u8(0);
        let body = w.into_vec();
        let mut out = Writer::new();
        item_to_canon(&mut Reader::new(&body), &mut out).unwrap();
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 5);
        assert_eq!(r.var_int().unwrap() as u32, MOD_ITEM_BASE + 1500);
        // обратно (container_click): HashedStack → 1.20.1 id сервера
        let mut h = Writer::new();
        h.bool(true)
            .var_int((MOD_ITEM_BASE + 1500) as i32)
            .var_int(5)
            .var_int(0)
            .var_int(0);
        let h = h.into_vec();
        let mut back = Writer::new();
        hashed_to_1201(&mut Reader::new(&h), &mut back).unwrap();
        let back = back.into_vec();
        let mut r = Reader::new(&back);
        assert!(r.bool().unwrap());
        assert_eq!(r.var_int().unwrap(), 1500);
        assert_eq!(r.u8().unwrap(), 5);
    }

    #[test]
    fn item_nbt_to_components() {
        // TODO-508c: алмазный меч 1.20.1 с Damage, именем, описанием и sharpness III.
        let id = ezzmc_protocol::generated::r1_20_1::ITEM
            .iter()
            .position(|n| *n == "diamond_sword")
            .unwrap() as i32;
        let ench = |n: &str, l: i16| {
            Tag::Compound(vec![
                ("id".into(), Tag::String(n.into())),
                ("lvl".into(), Tag::Short(l)),
            ])
        };
        let tag = Tag::Compound(vec![
            ("Damage".into(), Tag::Int(12)),
            (
                "Enchantments".into(),
                Tag::List(vec![ench("minecraft:sharpness", 3)]),
            ),
            (
                "display".into(),
                Tag::Compound(vec![
                    ("Name".into(), Tag::String(r#"{"text":"Меч"}"#.into())),
                    (
                        "Lore".into(),
                        Tag::List(vec![Tag::String(r#""строка""#.into())]),
                    ),
                ]),
            ),
        ]);
        let mut w = Writer::new();
        w.bool(true).var_int(id).u8(1);
        let mut nb = Vec::new();
        nbt::write_named(&mut nb, "", &tag).unwrap();
        w.bytes(&nb);
        let body = w.into_vec();
        let mut out = Writer::new();
        item_to_canon(&mut Reader::new(&body), &mut out).unwrap();
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        let st = ezzmc_protocol::item::read_stack(&mut r).unwrap().unwrap();
        assert_eq!(r.remaining(), 0);
        let get = |n: &str| {
            st.patch
                .added
                .iter()
                .find(|(i, _)| Some(*i) == component_id(n))
                .map(|(_, v)| v.to_vec())
        };
        assert_eq!(get("damage"), Some(vec![12]));
        let sharp = enchant_holder("minecraft:sharpness").unwrap() as u8;
        assert_eq!(get("enchantments"), Some(vec![1, sharp, 3]));
        assert!(get("custom_name").is_some() && get("lore").is_some());
        let reg = enchant_registry();
        let e = &reg.entries[sharp as usize];
        assert_eq!(e.0, "minecraft:sharpness");
        assert_eq!(
            e.1.as_ref()
                .and_then(|t| t.get("max_level"))
                .and_then(Tag::as_i64),
            Some(5)
        );
        assert!(reg
            .entries
            .iter()
            .any(|(n, _)| n == "minecraft:sweeping_edge"));
    }

    #[test]
    fn fml_open_container_becomes_open_screen() {
        use ezzmc_world::modded::{set_server_registry, MOD_MENU_BASE};
        // Снимок FML minecraft:menu: 0 — ванильный generic_9x1, 7 — модовая печь.
        set_server_registry(
            "minecraft:menu",
            &[
                (0, "minecraft:generic_9x1".into()),
                (7, "ironfurnaces:iron_furnace".into()),
            ],
        );
        let mut w = Writer::new();
        w.string(FML_PLAY).u8(1).var_int(7).var_int(3);
        w.string(r#"{"text":"Iron Furnace"}"#);
        w.var_int(8).i64(0);
        let body = w.into_vec();
        assert_eq!(canon_name("custom_payload", &body), "open_screen");
        let out = play_to_canon("custom_payload", &body).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 3);
        assert_eq!(r.var_int().unwrap(), MOD_MENU_BASE + 7);
        // Ванильное меню из снимка → id 26.3.
        let mut w = Writer::new();
        w.var_int(2).var_int(0).string(r#"{"text":"x"}"#);
        let out = play_to_canon("open_screen", &w.into_vec())
            .unwrap()
            .unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), 2);
        assert_eq!(r.var_int().unwrap(), 0);
        // Чужой канал не трогаем.
        let mut w = Writer::new();
        w.string("minecraft:brand").string("forge");
        assert_eq!(
            canon_name("custom_payload", &w.into_vec()),
            "custom_payload"
        );
    }
}
