//! Блок-энтити ≤1.12.2: то, что в 26.3 — разные блоки (цвет кровати и флага, тип головы,
//! растение в горшке), и данные, нужные рендеру (узоры флага, текст таблички).
//! Сервер присылает их отдельно от состояний, а `block_update` возвращает блок к варианту
//! по умолчанию, поэтому исходный NBT хранится здесь и накладывается повторно.
use ezzmc_net::block_entity::BANNER_PATTERNS;
use ezzmc_net::v1122::LegacyBlockEntities;
use ezzmc_protocol::generated::r26_3::BLOCK_ENTITY_TYPE;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::types::BlockPos;
use ezzmc_world::block::BlockState;
use ezzmc_world::chunk_packet::BlockEntityInfo;
use ezzmc_world::generated::b26_3::BLOCKS;
use ezzmc_world::storage::ChunkCache;
use std::collections::HashMap;

/// VANILLA: DyeColor#getId (1.12.2) — порядок id; у флагов (id красителя) он обратный.
const COLORS: [&str; 16] = [
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

/// VANILLA: SkullBlockEntity#skullType (1.12.2): стоячий и настенный блок.
const SKULLS: [(&str, &str); 6] = [
    ("skeleton_skull", "skeleton_wall_skull"),
    ("wither_skeleton_skull", "wither_skeleton_wall_skull"),
    ("zombie_head", "zombie_wall_head"),
    ("player_head", "player_wall_head"),
    ("creeper_head", "creeper_wall_head"),
    ("dragon_head", "dragon_wall_head"),
];

fn int(tag: &Tag, key: &str) -> Option<usize> {
    usize::try_from(tag.get(key)?.as_i64()?).ok()
}

/// Тот же блок под другим именем: общие свойства сохраняются.
fn retype(s: BlockState, name: &str) -> Option<BlockState> {
    let b = BLOCKS.iter().find(|b| b.name == name)?;
    let mut out = BlockState(b.default_state);
    for prop in BLOCKS.iter().find(|b| b.name == s.name())?.props {
        if let Some(next) = s.property(prop.name).and_then(|v| out.with(prop.name, v)) {
            out = next;
        }
    }
    Some(out)
}

/// VANILLA: FlowerPotBlock#getBlockState (1.12.2): предмет и его data → растение.
fn potted(tag: &Tag) -> Option<&'static str> {
    let data = int(tag, "Data").unwrap_or(0);
    let item = tag.get("Item")?;
    let name = match item.as_str() {
        Some(s) => s.trim_start_matches("minecraft:"),
        // Старая запись числовым id (Block#register, 1.12.2).
        None => match item.as_i64()? {
            6 => "sapling",
            31 => "tallgrass",
            32 => "deadbush",
            37 => "yellow_flower",
            38 => "red_flower",
            39 => "brown_mushroom",
            40 => "red_mushroom",
            81 => "cactus",
            _ => return None,
        },
    };
    Some(match (name, data) {
        ("sapling", 0) => "potted_oak_sapling",
        ("sapling", 1) => "potted_spruce_sapling",
        ("sapling", 2) => "potted_birch_sapling",
        ("sapling", 3) => "potted_jungle_sapling",
        ("sapling", 4) => "potted_acacia_sapling",
        ("sapling", 5) => "potted_dark_oak_sapling",
        ("tallgrass", 0) | ("deadbush", _) => "potted_dead_bush",
        ("tallgrass", 2) => "potted_fern",
        ("yellow_flower", _) => "potted_dandelion",
        ("red_flower", 0) => "potted_poppy",
        ("red_flower", 1) => "potted_blue_orchid",
        ("red_flower", 2) => "potted_allium",
        ("red_flower", 3) => "potted_azure_bluet",
        ("red_flower", 4) => "potted_red_tulip",
        ("red_flower", 5) => "potted_orange_tulip",
        ("red_flower", 6) => "potted_white_tulip",
        ("red_flower", 7) => "potted_pink_tulip",
        ("red_flower", 8) => "potted_oxeye_daisy",
        ("red_mushroom", _) => "potted_red_mushroom",
        ("brown_mushroom", _) => "potted_brown_mushroom",
        ("cactus", _) => "potted_cactus",
        _ => return None,
    })
}

fn string(s: impl Into<String>) -> Tag {
    Tag::String(s.into())
}

/// VANILLA: SignBlockEntity#fromNbt (1.12.2): `Text1`..`Text4` — JSON компонентов.
fn sign_text(tag: &Tag) -> Tag {
    let messages = (1..=4)
        .map(|i| {
            let raw = tag
                .get(&format!("Text{i}"))
                .and_then(Tag::as_str)
                .unwrap_or("");
            match serde_json::from_str::<serde_json::Value>(raw) {
                Ok(v) => ezzmc_net::v1201::json_to_tag(&v),
                Err(_) => string(raw),
            }
        })
        .collect();
    let front = Tag::Compound(vec![
        ("messages".into(), Tag::List(messages)),
        ("color".into(), string("black")),
        ("has_glowing_text".into(), Tag::Byte(0)),
    ]);
    Tag::Compound(vec![("front_text".into(), front)])
}

fn banner_patterns(tag: &Tag) -> Tag {
    let layers = match tag.get("Patterns") {
        Some(Tag::List(list)) => list.as_slice(),
        _ => &[],
    };
    let layers = layers
        .iter()
        .filter_map(|l| {
            let code = l.get("Pattern")?.as_str()?;
            let name = BANNER_PATTERNS.iter().find(|(c, _)| *c == code)?.1;
            let color = COLORS.get(15usize.checked_sub(int(l, "Color")?)?)?;
            Some(Tag::Compound(vec![
                ("pattern".into(), string(format!("minecraft:{name}"))),
                ("color".into(), string(*color)),
            ]))
        })
        .collect();
    Tag::Compound(vec![("patterns".into(), Tag::List(layers))])
}

/// VANILLA: BlockEntity — до 1.11 имена `Sign`/`Skull`/`Banner`/`FlowerPot`, с 1.11
/// `minecraft:sign` и т. д. (`BlockEntitySchema`).
fn flat_id(id: &str) -> &str {
    match id {
        "Sign" => "sign",
        "Skull" => "skull",
        "Banner" => "banner",
        "FlowerPot" => "flower_pot",
        _ => id.trim_start_matches("minecraft:"),
    }
}

/// Состояние и блок-энтити 26.3 для блока `s` с NBT 1.12.2; `None` — NBT к блоку не относится.
fn convert(s: BlockState, tag: &Tag) -> Option<(BlockState, Option<(&'static str, Tag)>)> {
    let id = flat_id(tag.get("id")?.as_str()?);
    let name = s.name();
    match id {
        "bed" if name.ends_with("_bed") => {
            let color = COLORS.get(int(tag, "color")?)?;
            Some((retype(s, &format!("{color}_bed"))?, None))
        }
        "skull" if name.ends_with("_skull") || name.ends_with("_head") => {
            let (floor, wall) = *SKULLS.get(int(tag, "SkullType")?)?;
            let on_wall = name.contains("_wall_");
            let mut out = retype(s, if on_wall { wall } else { floor })?;
            if !on_wall {
                let rot = (int(tag, "Rot").unwrap_or(0) & 15).to_string();
                out = out.with("rotation", &rot)?;
            }
            Some((out, Some(("skull", Tag::Compound(Vec::new())))))
        }
        "banner" if name.ends_with("_banner") => {
            let color = COLORS.get(15usize.checked_sub(int(tag, "Base")?)?)?;
            let kind = if name.ends_with("_wall_banner") {
                "wall_banner"
            } else {
                "banner"
            };
            let out = retype(s, &format!("{color}_{kind}"))?;
            Some((out, Some(("banner", banner_patterns(tag)))))
        }
        "flower_pot" if name == "flower_pot" || name.starts_with("potted_") => {
            Some((retype(s, potted(tag).unwrap_or("flower_pot"))?, None))
        }
        "sign" if name.ends_with("_sign") => Some((s, Some(("sign", sign_text(tag))))),
        _ => None,
    }
}

/// NBT блок-энтити 1.12.2 по колонкам.
#[derive(Default)]
pub struct Store {
    columns: HashMap<(i32, i32), HashMap<[i32; 3], Tag>>,
}

impl Store {
    pub fn clear(&mut self) {
        self.columns.clear();
    }

    /// Запомнить присланное; возвращает позиции новых записей.
    pub fn feed(&mut self, data: LegacyBlockEntities) -> Vec<[i32; 3]> {
        if let Some(column) = data.full_column {
            self.columns.remove(&column);
        }
        data.entries
            .into_iter()
            .map(|(p, tag)| {
                self.columns
                    .entry((p[0] >> 4, p[2] >> 4))
                    .or_default()
                    .insert(p, tag);
                p
            })
            .collect()
    }

    /// Наложить запись позиции `p`. true — блок или его блок-энтити изменены.
    fn apply_at(&self, chunks: &mut ChunkCache, p: [i32; 3]) -> bool {
        let Some(tag) = self
            .columns
            .get(&(p[0] >> 4, p[2] >> 4))
            .and_then(|c| c.get(&p))
        else {
            return false;
        };
        let Some((state, entity)) = convert(chunks.block(p[0], p[1], p[2]), tag) else {
            return false;
        };
        if !chunks.set_shape(p[0], p[1], p[2], state) {
            return false;
        }
        let pos = BlockPos::new(p[0], p[1], p[2]);
        chunks.remove_block_entity(pos);
        if let Some((kind, tag)) = entity {
            let id = BLOCK_ENTITY_TYPE
                .iter()
                .position(|n| n.trim_start_matches("minecraft:") == kind);
            if let Some(type_id) = id.and_then(|i| u32::try_from(i).ok()) {
                chunks.set_block_entity(
                    pos,
                    BlockEntityInfo {
                        x: (p[0] & 15) as u8,
                        z: (p[2] & 15) as u8,
                        y: p[1],
                        type_id,
                        tag: Some(tag),
                    },
                );
            }
        }
        true
    }

    /// Наложить записи указанных позиций; возвращает изменённые.
    pub fn apply(
        &self,
        chunks: &mut ChunkCache,
        positions: impl IntoIterator<Item = [i32; 3]>,
    ) -> Vec<BlockPos> {
        positions
            .into_iter()
            .filter(|&p| self.apply_at(chunks, p))
            .map(|[x, y, z]| BlockPos::new(x, y, z))
            .collect()
    }

    /// Пришла колонка: наложить все её записи.
    pub fn apply_column(&self, chunks: &mut ChunkCache, x: i32, z: i32) {
        if let Some(c) = self.columns.get(&(x, z)) {
            self.apply(chunks, c.keys().copied());
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn block(name: &str) -> BlockState {
        BlockState(
            BLOCKS
                .iter()
                .find(|b| b.name == name)
                .unwrap()
                .default_state,
        )
    }

    fn nbt(id: &str, fields: &[(&str, Tag)]) -> Tag {
        let mut v = vec![("id".to_string(), string(format!("minecraft:{id}")))];
        v.extend(fields.iter().map(|(k, t)| (k.to_string(), t.clone())));
        Tag::Compound(v)
    }

    #[test]
    fn pre_1_11_ids_are_accepted() {
        let tag = Tag::Compound(vec![
            ("id".into(), string("Sign")),
            ("Text1".into(), string(r#"{"text":"hi"}"#)),
        ]);
        let (_, entity) = convert(block("oak_sign"), &tag).unwrap();
        assert_eq!(entity.map(|e| e.0), Some("sign"));
    }

    #[test]
    fn every_target_block_exists() {
        for c in COLORS {
            for kind in ["bed", "banner", "wall_banner"] {
                block(&format!("{c}_{kind}"));
            }
        }
        for (a, b) in SKULLS {
            block(a);
            block(b);
        }
    }

    #[test]
    fn bed_keeps_part_and_facing() {
        let s = block("red_bed")
            .with("part", "head")
            .and_then(|s| s.with("facing", "east"))
            .unwrap();
        let (out, entity) = convert(s, &nbt("bed", &[("color", Tag::Int(11))])).unwrap();
        assert_eq!(out.name(), "blue_bed");
        assert_eq!(out.property("part"), Some("head"));
        assert_eq!(out.property("facing"), Some("east"));
        assert!(entity.is_none());
    }

    #[test]
    fn skull_type_and_rotation() {
        let tag = nbt(
            "skull",
            &[("SkullType", Tag::Byte(4)), ("Rot", Tag::Byte(6))],
        );
        let (out, _) = convert(block("skeleton_skull"), &tag).unwrap();
        assert_eq!(out.name(), "creeper_head");
        assert_eq!(out.property("rotation"), Some("6"));
        assert!(convert(block("stone"), &tag).is_none());
    }

    #[test]
    fn banner_colors_are_dye_ids() {
        let layer = Tag::Compound(vec![
            ("Pattern".into(), string("bs")),
            ("Color".into(), Tag::Int(1)),
        ]);
        let tag = nbt(
            "banner",
            &[("Base", Tag::Int(4)), ("Patterns", Tag::List(vec![layer]))],
        );
        let (out, entity) = convert(block("white_banner"), &tag).unwrap();
        assert_eq!(out.name(), "blue_banner");
        let (kind, data) = entity.unwrap();
        assert_eq!(kind, "banner");
        let Some(Tag::List(l)) = data.get("patterns") else {
            panic!("no patterns");
        };
        assert_eq!(
            l[0].get("pattern").and_then(Tag::as_str),
            Some("minecraft:stripe_bottom")
        );
        assert_eq!(l[0].get("color").and_then(Tag::as_str), Some("red"));
    }

    #[test]
    fn flower_pot_by_item_and_data() {
        let tag = nbt(
            "flower_pot",
            &[
                ("Item", string("minecraft:red_flower")),
                ("Data", Tag::Int(3)),
            ],
        );
        let (out, _) = convert(block("flower_pot"), &tag).unwrap();
        assert_eq!(out.name(), "potted_azure_bluet");
    }
}
