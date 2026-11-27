//! Отличия протокола 1.12.2 (340) от 1.13.2: чанк и предметы приводятся к формату 1.13.2, дальше идут
//! путём `v1132`. Id состояний блоков остаются `id<<4 | meta` (переводит `world::legacy`).

use ezzmc_protocol::generated::r1_13_2::ITEM;
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;
use ezzmc_world::generated::flattening::ITEMS;

const SECTION_BLOCKS: usize = 4096;
const LIGHT_BYTES: usize = 2048;
const BIOMES: usize = 256;
const FILLED_MAP: u16 = 358;
const SPAWN_EGG: u16 = 383;

/// Числовые id зачарований 1.12.2. VANILLA: Enchantment#register (1.12.2).
const ENCHANTMENTS: &[(i16, &str)] = &[
    (0, "protection"),
    (1, "fire_protection"),
    (2, "feather_falling"),
    (3, "blast_protection"),
    (4, "projectile_protection"),
    (5, "respiration"),
    (6, "aqua_affinity"),
    (7, "thorns"),
    (8, "depth_strider"),
    (9, "frost_walker"),
    (10, "binding_curse"),
    (16, "sharpness"),
    (17, "smite"),
    (18, "bane_of_arthropods"),
    (19, "knockback"),
    (20, "fire_aspect"),
    (21, "looting"),
    (22, "sweeping"),
    (32, "efficiency"),
    (33, "silk_touch"),
    (34, "unbreaking"),
    (35, "fortune"),
    (48, "power"),
    (49, "punch"),
    (50, "flame"),
    (51, "infinity"),
    (61, "luck_of_the_sea"),
    (62, "lure"),
    (70, "mending"),
    (71, "vanishing_curse"),
];

/// Пакеты, формат которых отличается и пока не переводится (сущности, plugin-каналы).
pub(crate) fn is_untranslated(name: &str) -> bool {
    matches!(
        name,
        "add_entity"
            | "add_mob"
            | "add_player"
            | "add_painting"
            | "add_experience_orb"
            | "set_entity_data"
            | "custom_payload"
    )
}

/// `level_chunk` 1.12.2 → 1.13.2: у прямой палитры убирается пустой счётчик, биомы byte → int.
/// VANILLA: ChunkDataS2CPacket#writeData, PaletteContainer#write, RegistryPalette#write (1.12.2).
pub(crate) fn chunk_to_1_13(body: &[u8], sky: bool) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let (x, z) = (r.i32()?, r.i32()?);
    let full = r.bool()?;
    let mask = r.var_int()?;
    let len = r.length("chunk data", 1 << 21)?;
    let mut data = Reader::new(r.bytes(len)?);
    let mut out = Writer::with_capacity(len + BIOMES * 4);
    for _ in 0..(mask & 0xFFFF).count_ones() {
        let bits = data.u8()?;
        out.u8(bits);
        let palette = data.length("chunk palette", 256)?;
        // VANILLA: PaletteContainer#setPaletteSize (1.12.2): палитра при bits <= 8.
        if bits <= 8 {
            out.var_int(palette as i32);
            for _ in 0..palette {
                out.var_int(data.var_int()?);
            }
        }
        let n = data.length("section longs", SECTION_BLOCKS)?;
        out.var_int(n as i32).bytes(data.bytes(n * 8)?);
        let light = if sky { 2 } else { 1 } * LIGHT_BYTES;
        out.bytes(data.bytes(light)?);
    }
    if full {
        for biome in data.bytes(BIOMES)? {
            out.i32(i32::from(*biome));
        }
    }
    let out = out.into_vec();
    let mut w = Writer::with_capacity(out.len() + r.remaining() + 16);
    w.i32(x).i32(z).bool(full).var_int(mask);
    w.var_int(out.len() as i32).bytes(&out).bytes(r.rest());
    Ok(w.into_vec())
}

fn item_name(id: i32) -> Option<&'static str> {
    let name = ITEM.get(usize::try_from(id).ok()?)?;
    Some(name.trim_start_matches("minecraft:"))
}

fn item_by_name(name: &str) -> Option<i32> {
    ITEM.iter()
        .position(|n| n.trim_start_matches("minecraft:") == name)
        .map(|i| i as i32)
}

fn take(entries: &mut Vec<(String, Tag)>, key: &str) -> Option<Tag> {
    let i = entries.iter().position(|(k, _)| k == key)?;
    Some(entries.remove(i).1)
}

fn rename_display(entries: &mut [(String, Tag)], f: impl Fn(&str) -> String) {
    let name = entries
        .iter_mut()
        .find(|(k, _)| k == "display")
        .and_then(|(_, t)| match t {
            Tag::Compound(d) => d.iter_mut().find(|(k, _)| k == "Name"),
            _ => None,
        });
    if let Some((_, Tag::String(s))) = name {
        *s = f(s);
    }
}

fn map_enchantments(list: &mut Tag, f: impl Fn(&Tag) -> Option<Tag>) {
    if let Tag::List(items) = list {
        for item in items {
            if let Tag::Compound(fields) = item {
                for (k, v) in fields {
                    if k == "id" {
                        if let Some(new) = f(v) {
                            *v = new;
                        }
                    }
                }
            }
        }
    }
}

/// VANILLA: ItemInstanceTheFlatteningFix, ItemStackEnchantmentFix, ItemCustomNameToJson (1.13.2).
fn tag_to_1_13(entries: &mut [(String, Tag)]) {
    rename_display(entries, |s| serde_json::json!({ "text": s }).to_string());
    for (k, v) in entries.iter_mut() {
        if k == "ench" || k == "StoredEnchantments" {
            map_enchantments(v, |id| {
                let id = id.as_i64()?;
                let (_, name) = ENCHANTMENTS.iter().find(|(n, _)| i64::from(*n) == id)?;
                Some(Tag::String(format!("minecraft:{name}")))
            });
        }
        if k == "ench" {
            *k = "Enchantments".into();
        }
    }
}

fn tag_to_1_12(entries: &mut [(String, Tag)]) {
    rename_display(entries, |s| {
        serde_json::from_str::<serde_json::Value>(s)
            .ok()
            .and_then(|v| v.get("text")?.as_str().map(str::to_owned))
            .unwrap_or_else(|| s.to_owned())
    });
    for (k, v) in entries.iter_mut() {
        if k == "Enchantments" || k == "StoredEnchantments" {
            map_enchantments(v, |id| {
                let name = id.as_str()?.trim_start_matches("minecraft:");
                let (n, _) = ENCHANTMENTS.iter().find(|(_, e)| *e == name)?;
                Some(Tag::Short(*n))
            });
        }
        if k == "Enchantments" {
            *k = "ench".into();
        }
    }
}

fn write_tag(w: &mut Writer, entries: Vec<(String, Tag)>) -> ProtocolResult<()> {
    if entries.is_empty() {
        w.u8(0);
        return Ok(());
    }
    let mut out = Vec::new();
    nbt::write_named(&mut out, "", &Tag::Compound(entries))?;
    w.bytes(&out);
    Ok(())
}

fn read_tag(r: &mut Reader) -> ProtocolResult<Vec<(String, Tag)>> {
    Ok(match nbt::read_named(r)? {
        Some((_, Tag::Compound(entries))) => entries,
        _ => Vec::new(),
    })
}

/// Предмет 1.12.2 (short id, byte count, short damage, NBT) → 1.13.2 (bool, varint id, byte, NBT).
/// VANILLA: PacketByteBuf#readItemStack (1.12.2).
pub(crate) fn item_to_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    let id = r.i16()?;
    let Ok(id) = u16::try_from(id) else {
        w.bool(false);
        return Ok(());
    };
    let count = r.u8()?;
    let damage = r.i16()?;
    let mut tag = read_tag(r)?;
    let meta = u16::try_from(damage).unwrap_or(0);
    let exact = |d: u16| {
        ITEMS
            .binary_search_by_key(&(id, d), |e| (e.0, e.1))
            .ok()
            .map(|i| i32::from(ITEMS[i].2))
    };
    let new = if id == SPAWN_EGG {
        // Вид яйца — в NBT `EntityTag.id`.
        tag.iter()
            .find(|(k, _)| k == "EntityTag")
            .and_then(|(_, t)| t.get("id")?.as_str())
            .and_then(|e| {
                let e = e.trim_start_matches("minecraft:");
                item_by_name(&format!("{e}_spawn_egg"))
            })
    } else if id == FILLED_MAP {
        tag.push(("map".into(), Tag::Int(i32::from(damage))));
        exact(0)
    } else {
        exact(meta).or_else(|| {
            // Нет варианта с таким damage — это прочность: она переезжает в NBT.
            let base = exact(0)?;
            tag.push(("Damage".into(), Tag::Int(i32::from(damage))));
            Some(base)
        })
    };
    let Some(new) = new else {
        w.bool(false);
        return Ok(());
    };
    tag_to_1_13(&mut tag);
    w.bool(true).var_int(new).u8(count);
    write_tag(w, tag)
}

/// Обратный перевод предмета для serverbound.
pub(crate) fn item_from_1_13_2(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    if !r.bool()? {
        w.i16(-1);
        return Ok(());
    }
    let id = r.var_int()?;
    let count = r.u8()?;
    let mut tag = read_tag(r)?;
    let found = ITEMS
        .iter()
        .find(|e| i32::from(e.2) == id)
        .map(|e| (e.0, e.1))
        .or_else(|| {
            item_name(id)?
                .ends_with("_spawn_egg")
                .then_some((SPAWN_EGG, 0))
        });
    let Some((old, mut damage)) = found else {
        w.i16(-1);
        return Ok(());
    };
    let moved = if old == FILLED_MAP {
        take(&mut tag, "map")
    } else if damage == 0 {
        take(&mut tag, "Damage")
    } else {
        None
    };
    if let Some(v) = moved.and_then(|t| t.as_i64()) {
        damage = u16::try_from(v).unwrap_or(0);
    }
    tag_to_1_12(&mut tag);
    w.i16(old as i16).u8(count).i16(damage as i16);
    write_tag(w, tag)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn roundtrip(id: i16, damage: i16, tag: Vec<(String, Tag)>) -> (String, Vec<(String, Tag)>) {
        let mut old = Writer::new();
        old.i16(id).u8(3).i16(damage);
        write_tag(&mut old, tag).unwrap();
        let old = old.into_vec();
        let mut new = Writer::new();
        item_to_1_13_2(&mut Reader::new(&old), &mut new).unwrap();
        let new = new.into_vec();
        let mut back = Writer::new();
        item_from_1_13_2(&mut Reader::new(&new), &mut back).unwrap();
        assert_eq!(back.into_vec(), old);
        let mut r = Reader::new(&new);
        assert!(r.bool().unwrap());
        let name = item_name(r.var_int().unwrap()).unwrap().to_owned();
        r.u8().unwrap();
        (name, read_tag(&mut r).unwrap())
    }

    #[test]
    fn items() {
        assert_eq!(roundtrip(1, 1, Vec::new()).0, "granite");
        assert_eq!(roundtrip(35, 14, Vec::new()).0, "red_wool");
        let (name, tag) = roundtrip(276, 7, Vec::new());
        assert_eq!(name, "diamond_sword");
        assert_eq!(tag, [("Damage".to_owned(), Tag::Int(7))]);
        assert_eq!(roundtrip(358, 12, Vec::new()).0, "filled_map");
        let egg = vec![(
            "EntityTag".to_owned(),
            Tag::Compound(vec![("id".into(), Tag::String("minecraft:zombie".into()))]),
        )];
        assert_eq!(roundtrip(383, 0, egg).0, "zombie_spawn_egg");
        let ench = vec![(
            "ench".to_owned(),
            Tag::List(vec![Tag::Compound(vec![
                ("id".into(), Tag::Short(16)),
                ("lvl".into(), Tag::Short(5)),
            ])]),
        )];
        assert_eq!(roundtrip(276, 0, ench).1[0].0, "Enchantments");
    }

    #[test]
    fn chunk_direct_palette_and_biomes() {
        let mut data = Writer::new();
        data.u8(13).var_int(0).var_int(832);
        data.bytes(&[0; 832 * 8]);
        data.bytes(&[0; LIGHT_BYTES * 2]).bytes(&[7; BIOMES]);
        let mut body = Writer::new();
        body.i32(1).i32(2).bool(true).var_int(1);
        body.var_int(data.buf.len() as i32)
            .bytes(&data.buf)
            .var_int(0);
        let out = chunk_to_1_13(&body.buf, true).unwrap();
        let mut r = Reader::new(&out);
        r.bytes(4 + 4 + 1 + 1).unwrap();
        let len = r.var_int().unwrap() as usize;
        assert_eq!(len, 1 + 2 + 832 * 8 + LIGHT_BYTES * 2 + BIOMES * 4);
        assert!(chunk_to_1_13(&body.buf[..40], true).is_err());
    }
}
