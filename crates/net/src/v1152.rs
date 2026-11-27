//! Отличия протоколов 1.14–1.15.2 (477–578): 1.14.x → 1.15.2, 1.15.x → 1.16.1: тела приводятся к формату 1.16.1, дальше идут общим
//! путём `v1165`/`v1201`.

use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;

/// Имя пакета 1.15.2 → имя того же пакета в следующих версиях.
pub fn alias(name: &'static str) -> &'static str {
    match name {
        "set_equipped_item" => "set_equipment",
        "set_spawn_position" => "set_default_spawn_position",
        _ => name,
    }
}

/// UUID из строки с дефисами (мусор в строке даёт 0).
pub(crate) fn parse_uuid(s: &str) -> u128 {
    let hex: String = s.chars().filter(|c| *c != '-').collect();
    u128::from_str_radix(&hex, 16).unwrap_or(0)
}

/// Предыдущий режим игры в 1.15.2 не передаётся.
const NO_PREVIOUS_MODE: u8 = 255;

/// VANILLA: DimensionType#getById (1.15.2): -1, 0, 1.
fn dimension_name(id: i32) -> &'static str {
    match id {
        -1 => "minecraft:the_nether",
        1 => "minecraft:the_end",
        _ => "minecraft:overworld",
    }
}

/// VANILLA: DimensionType#DEFAULT_* (1.16.1) — свойства встроенных измерений.
fn dimension(name: &str) -> Tag {
    let nether = name.ends_with("the_nether");
    let end = name.ends_with("the_end");
    let over = !nether && !end;
    let flag = |b: bool| Tag::Byte(i8::from(b));
    let mut f = vec![
        ("name".into(), Tag::String(name.to_owned())),
        ("has_skylight".into(), flag(over)),
        ("has_ceiling".into(), flag(nether)),
        ("ultrawarm".into(), flag(nether)),
        ("natural".into(), flag(over)),
        ("shrunk".into(), flag(nether)),
        ("piglin_safe".into(), flag(nether)),
        ("bed_works".into(), flag(over)),
        ("respawn_anchor_works".into(), flag(nether)),
        ("has_raids".into(), flag(!nether)),
        (
            "logical_height".into(),
            Tag::Int(if nether { 128 } else { 256 }),
        ),
        (
            "infiniburn".into(),
            Tag::String(format!(
                "minecraft:infiniburn_{}",
                name.trim_start_matches("minecraft:")
                    .trim_start_matches("the_")
            )),
        ),
        (
            "ambient_light".into(),
            Tag::Float(if nether { 0.1 } else { 0.0 }),
        ),
    ];
    if nether {
        f.push(("fixed_time".into(), Tag::Long(18000)));
    } else if end {
        f.push(("fixed_time".into(), Tag::Long(6000)));
    }
    Tag::Compound(f)
}

/// `generic.maxHealth` → `minecraft:generic.max_health`.
fn attribute_name(old: &str) -> String {
    let mut out = String::from("minecraft:");
    for c in old.chars() {
        if c.is_ascii_uppercase() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// long[] с переносом через границу long → без переноса.
fn repack(longs: &[u64], bits: u32, n: usize) -> Vec<u64> {
    let per_long = (64 / bits) as usize;
    let mut out = vec![0u64; n.div_ceil(per_long)];
    for (i, v) in crate::v1165::unpack(longs, bits, n, true)
        .into_iter()
        .enumerate()
    {
        out[i / per_long] |= v << ((i % per_long) as u32 * bits);
    }
    out
}

fn read_longs(r: &mut Reader, max: usize) -> ProtocolResult<Vec<u64>> {
    let n = r.length("longs", max)?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(r.i64()? as u64);
    }
    Ok(out)
}

/// Секции чанка 1.15.2 → 1.16.
/// VANILLA: LevelChunkSection#read (1.15.2): short, byte bits, палитра при bits < 9, long[].
fn sections(data: &[u8], mask: i32) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(data);
    let mut w = Writer::with_capacity(data.len() + 512);
    for _ in 0..(mask & 0xFFFF).count_ones() {
        w.i16(r.i16()?);
        let raw = r.u8()?;
        let bits = u32::from(raw).clamp(4, 31);
        w.u8(raw);
        if bits < 9 {
            let n = r.length("chunk palette", 256)?;
            w.var_int(n as i32);
            for _ in 0..n {
                w.var_int(r.var_int()?);
            }
        }
        let longs = repack(&read_longs(&mut r, 4096)?, bits, 4096);
        w.var_int(longs.len() as i32);
        for l in longs {
            w.i64(l as i64);
        }
    }
    Ok(w.into_vec())
}

/// Карты высот: 256 значений по 9 бит.
fn heightmaps(tag: Tag) -> Tag {
    let Tag::Compound(fields) = tag else {
        return tag;
    };
    Tag::Compound(
        fields
            .into_iter()
            .map(|(k, v)| match v {
                Tag::LongArray(l) => {
                    let l: Vec<u64> = l.into_iter().map(|x| x as u64).collect();
                    let l = repack(&l, 9, 256).into_iter().map(|x| x as i64);
                    (k, Tag::LongArray(l.collect()))
                }
                v => (k, v),
            })
            .collect(),
    )
}

/// Тело клиентского пакета 1.15.2 → (имя, тело) 1.16.1; `None` — формат не менялся.
pub fn from_1_15_2(name: &str, body: &[u8]) -> ProtocolResult<Option<(&'static str, Vec<u8>)>> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 64);
    let out_name = match name {
        // VANILLA: ClientboundLoginPacket (1.15.2): int id, byte режим, int измерение, long seed,
        // byte maxPlayers, строка типа уровня, varint дальность, 2 bool.
        "login" => {
            w.i32(r.i32()?).u8(r.u8()?).u8(NO_PREVIOUS_MODE);
            let dim = dimension_name(r.i32()?);
            let seed = r.bytes(8)?;
            let max_players = r.u8()?;
            let level_type = r.string(16)?;
            let names = ["overworld", "the_nether", "the_end"].map(|n| format!("minecraft:{n}"));
            w.var_int(names.len() as i32);
            for n in &names {
                w.string(n);
            }
            let codec = Tag::Compound(vec![(
                "dimension".into(),
                Tag::List(names.iter().map(|n| dimension(n)).collect()),
            )]);
            let mut nbt_out = Vec::new();
            nbt::write_named(&mut nbt_out, "", &codec)?;
            w.bytes(&nbt_out).string(dim).string(dim).bytes(seed);
            w.u8(max_players).var_int(r.var_int()?);
            w.bool(r.bool()?).bool(r.bool()?);
            w.bool(level_type == "debug_all_block_states")
                .bool(level_type == "flat");
            "login"
        }
        // VANILLA: ClientboundRespawnPacket (1.15.2): int измерение, long seed, byte режим,
        // строка типа уровня.
        "respawn" => {
            let dim = dimension_name(r.i32()?);
            w.string(dim).string(dim).bytes(r.bytes(8)?);
            w.u8(r.u8()?).u8(NO_PREVIOUS_MODE);
            let level_type = r.string(16)?;
            w.bool(level_type == "debug_all_block_states")
                .bool(level_type == "flat")
                .bool(false);
            "respawn"
        }
        // VANILLA: ClientboundLevelChunkPacket (1.15.2): без forgetOldData, long[] с переносом.
        "level_chunk" => {
            w.i32(r.i32()?).i32(r.i32()?);
            let full = r.bool()?;
            let mask = r.var_int()?;
            w.bool(full).bool(false).var_int(mask);
            let maps = nbt::read_named(&mut r)?.map_or_else(|| Tag::Compound(Vec::new()), |t| t.1);
            let mut nbt_out = Vec::new();
            nbt::write_named(&mut nbt_out, "", &heightmaps(maps))?;
            w.bytes(&nbt_out);
            if full {
                w.bytes(r.bytes(4096)?);
            }
            let len = r.length("chunk data", 1 << 21)?;
            let data = sections(r.bytes(len)?, mask)?;
            w.var_int(data.len() as i32).bytes(&data).bytes(r.rest());
            "level_chunk"
        }
        // VANILLA: ClientboundLightUpdatePacket (1.15.2): без trustEdges.
        "light_update" => {
            w.var_int(r.var_int()?).var_int(r.var_int()?);
            w.bool(true).bytes(r.rest());
            "light_update"
        }
        // VANILLA: ClientboundChatPacket (1.15.2): без UUID отправителя.
        "chat" => {
            w.bytes(body).uuid(0);
            "chat"
        }
        // VANILLA: ClientboundSetEquippedItemPacket (1.15.2): varint сущность, varint слот, предмет.
        "set_equipment" => {
            w.var_int(r.var_int()?);
            w.u8(r.var_int()? as u8).bytes(r.rest());
            "set_equipment"
        }
        // VANILLA: ClientboundAddGlobalEntityPacket (1.15.2) — молния; в 1.16 это add_entity.
        "add_global_entity" => {
            let id = r.var_int()?;
            r.u8()?;
            let kind = ezzmc_protocol::version::V1_15_2
                .entity_type
                .iter()
                .position(|n| n.ends_with("lightning_bolt"));
            let Some(kind) = kind else {
                return Ok(None);
            };
            w.var_int(id).uuid(id as u128).var_int(kind as i32);
            w.bytes(r.bytes(24)?).u8(0).u8(0).i32(0);
            w.i16(0).i16(0).i16(0);
            "add_entity"
        }
        // VANILLA: ClientboundUpdateAttributesPacket (1.15.2): имена атрибутов в camelCase.
        "update_attributes" => {
            w.var_int(r.var_int()?);
            let n = r.i32()?;
            w.i32(n);
            for _ in 0..n {
                w.string(&attribute_name(&r.string(64)?));
                w.bytes(r.bytes(8)?);
                let m = r.length("modifiers", 1024)?;
                w.var_int(m as i32);
                // UUID, double, byte операция
                w.bytes(r.bytes(m.saturating_mul(25))?);
            }
            "update_attributes"
        }
        _ => return Ok(None),
    };
    Ok(Some((out_name, w.into_vec())))
}

/// Биомы чанка 1.14 (256 int по столбцам) → 1024 int 1.15 (ячейки 4×4×4).
/// VANILLA-UNSURE: ваниль 1.14 красит по столбцам блоков; берётся центр ячейки 4×4.
fn biomes_3d(columns: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4096);
    for _ in 0..64 {
        for z in 0..4 {
            for x in 0..4 {
                let at = ((z * 4 + 2) * 16 + x * 4 + 2) * 4;
                out.extend_from_slice(columns.get(at..at + 4).unwrap_or(&[0; 4]));
            }
        }
    }
    out
}

/// Тело клиентского пакета 1.14.x → формат 1.15.2 (`None` — тело не меняется).
pub fn from_1_14_4(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 4096);
    match name {
        // VANILLA: ClientboundLoginPacket (1.14.4): без seed и showDeathScreen. seed 0 — клиент
        // старой версии его не знает.
        "login" => {
            w.bytes(r.bytes(9)?).i64(0).bytes(r.rest()).bool(true);
        }
        // VANILLA: ClientboundRespawnPacket (1.14.4): без seed.
        "respawn" => {
            w.bytes(r.bytes(4)?).i64(0).bytes(r.rest());
        }
        // VANILLA: ClientboundLevelChunkPacket (1.14.4): биомы — 256 int в конце данных секций.
        "level_chunk" => {
            w.bytes(r.bytes(8)?);
            let full = r.bool()?;
            w.bool(full).var_int(r.var_int()?);
            let at = r.pos();
            nbt::read_named(&mut r)?;
            w.bytes(r.since(at));
            let len = r.length("chunk data", 1 << 21)?;
            let mut data = r.bytes(len)?;
            if full {
                let split = data.len().saturating_sub(1024);
                w.bytes(&biomes_3d(&data[split..]));
                data = &data[..split];
            }
            w.var_int(data.len() as i32).bytes(data).bytes(r.rest());
        }
        // VANILLA: ClientboundLevelParticlesPacket (1.14.4): позиция — float.
        "level_particles" => {
            w.bytes(r.bytes(5)?);
            for _ in 0..3 {
                w.f64(f64::from(r.f32()?));
            }
            w.bytes(r.rest());
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

/// Внутренние id пакетов, на которые разворачиваются `add_mob`/`add_player` 1.14.
pub const ADD_MOB_ID: i32 = 0x1001;
pub const ADD_PLAYER_ID: i32 = 0x1002;
pub const ENTITY_DATA_ID: i32 = 0x1003;

/// VANILLA: ClientboundAddMobPacket / AddPlayerPacket (1.14.4) несут метаданные в хвосте
/// (в 1.15 — отдельный `set_entity_data`) → пара (id, тело). `None` — хвоста нет.
pub fn expand_1_14(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<crate::legacy::Packet>>> {
    let (add, fixed) = match name {
        // uuid, varint тип, xyz, 3 угла, скорость
        "add_mob" => (ADD_MOB_ID, None),
        // uuid, xyz, 2 угла
        "add_player" => (ADD_PLAYER_ID, Some(16 + 24 + 2)),
        _ => return Ok(None),
    };
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    match fixed {
        Some(n) => {
            r.bytes(n)?;
        }
        None => {
            r.bytes(16)?;
            r.var_int()?;
            r.bytes(24 + 3 + 6)?;
        }
    }
    let head = r.since(0);
    let tail = r.rest();
    if tail.is_empty() {
        return Ok(None);
    }
    let mut data = Writer::new();
    data.var_int(id).bytes(tail);
    Ok(Some(vec![
        (add, head.to_vec()),
        (ENTITY_DATA_ID, data.into_vec()),
    ]))
}

/// Тело serverbound-пакета формата 1.16 → 1.15.2.
pub(crate) fn to_1_15_2(name: &str, body: &mut Vec<u8>) {
    match name {
        // VANILLA: ServerboundInteractPacket (1.15.2): без флага приседания в конце.
        "interact" => {
            body.pop();
        }
        // VANILLA: ServerboundPlayerAbilitiesPacket (1.15.2): флаги + две скорости.
        "player_abilities" => {
            body.extend_from_slice(&0.05f32.to_be_bytes());
            body.extend_from_slice(&0.1f32.to_be_bytes());
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repack_keeps_values() {
        // 5 бит: значение 12 лежит на границе первого и второго long
        let mut old = vec![0u64; 320];
        old[0] = 0b1_0110 << 60;
        old[1] = 0b1;
        let new = repack(&old, 5, 4096);
        assert_eq!(new.len(), 342);
        assert_eq!(new[1] & 31, 0b1_0110);
    }

    #[test]
    fn chunk_biomes() {
        let columns: Vec<u8> = (0..256i32).flat_map(i32::to_be_bytes).collect();
        let out = biomes_3d(&columns);
        assert_eq!(out.len(), 4096);
        // ячейка x=1 z=2 → столбец x=6 z=10
        assert_eq!(out[(2 * 4 + 1) * 4 + 3], 10 * 16 + 6);
    }

    #[test]
    fn add_player_metadata_splits_once() {
        let mut b = Writer::new();
        b.var_int(5).bytes(&[0; 42]).u8(0xFF);
        let parts = expand_1_14("add_player", &b.buf).unwrap().unwrap();
        assert_eq!(parts[0].1.len(), 43);
        assert_eq!(parts[1].1, [5, 0xFF]);
        assert!(expand_1_14("add_player", &parts[0].1).unwrap().is_none());
    }

    #[test]
    fn names() {
        assert_eq!(
            attribute_name("zombie.spawnReinforcements"),
            "minecraft:zombie.spawn_reinforcements"
        );
        assert_eq!(parse_uuid("00000000-0000-0000-0000-0000000000ff"), 255);
    }
}
