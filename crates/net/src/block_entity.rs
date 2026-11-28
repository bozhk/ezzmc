//! Блок-сущности серверов 1.13–1.20.1: id типа и NBT → канон 26.3.
//! До 1.17.1 тип задаётся строкой `id` в NBT, с 1.18 — номером в реестре версии.
use ezzmc_protocol::generated::{
    r1_14_4, r1_15_2, r1_16_5, r1_17_1, r1_18_2, r1_19, r1_19_2, r1_19_3, r1_19_4, r1_20_1, r26_3,
};
use ezzmc_protocol::nbt::Tag;
use serde_json::Value as Json;

/// VANILLA: DyeColor#getId — порядок красителей.
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

/// VANILLA: BannerPattern (≤1.20.4): короткий код → имя узора.
pub const BANNER_PATTERNS: [(&str, &str); 41] = [
    ("b", "base"),
    ("bl", "square_bottom_left"),
    ("br", "square_bottom_right"),
    ("tl", "square_top_left"),
    ("tr", "square_top_right"),
    ("bs", "stripe_bottom"),
    ("ts", "stripe_top"),
    ("ls", "stripe_left"),
    ("rs", "stripe_right"),
    ("cs", "stripe_center"),
    ("ms", "stripe_middle"),
    ("drs", "stripe_downright"),
    ("dls", "stripe_downleft"),
    ("ss", "small_stripes"),
    ("cr", "cross"),
    ("sc", "straight_cross"),
    ("bt", "triangle_bottom"),
    ("tt", "triangle_top"),
    ("bts", "triangles_bottom"),
    ("tts", "triangles_top"),
    ("ld", "diagonal_left"),
    ("rd", "diagonal_up_right"),
    ("lud", "diagonal_up_left"),
    ("rud", "diagonal_right"),
    ("mc", "circle"),
    ("mr", "rhombus"),
    ("vh", "half_vertical"),
    ("hh", "half_horizontal"),
    ("vhr", "half_vertical_right"),
    ("hhb", "half_horizontal_bottom"),
    ("bo", "border"),
    ("cbo", "curly_border"),
    ("cre", "creeper"),
    ("gra", "gradient"),
    ("gru", "gradient_up"),
    ("bri", "bricks"),
    ("sku", "skull"),
    ("flo", "flower"),
    ("moj", "mojang"),
    ("glb", "globe"),
    ("pig", "piglin"),
];

/// Реестр block_entity_type, в номерах которого идут блок-сущности протокола `protocol`.
/// До 1.17.1 номеров на проводе нет — таблица служит промежуточным представлением.
fn types(protocol: i32) -> &'static [&'static str] {
    match protocol {
        ..=498 => r1_14_4::BLOCK_ENTITY_TYPE,
        499..=578 => r1_15_2::BLOCK_ENTITY_TYPE,
        579..=754 => r1_16_5::BLOCK_ENTITY_TYPE,
        755..=756 => r1_17_1::BLOCK_ENTITY_TYPE,
        757..=758 => r1_18_2::BLOCK_ENTITY_TYPE,
        759 => r1_19::BLOCK_ENTITY_TYPE,
        760 => r1_19_2::BLOCK_ENTITY_TYPE,
        761 => r1_19_3::BLOCK_ENTITY_TYPE,
        762 => r1_19_4::BLOCK_ENTITY_TYPE,
        _ => r1_20_1::BLOCK_ENTITY_TYPE,
    }
}

fn bare(name: &str) -> &str {
    name.trim_start_matches("minecraft:")
}

/// Номер типа в таблице версии по строке `id` из NBT.
pub(crate) fn server_id(protocol: i32, tag: &Tag) -> Option<i32> {
    let id = bare(tag.get("id")?.as_str()?);
    let i = types(protocol).iter().position(|n| bare(n) == id)?;
    i32::try_from(i).ok()
}

fn name_of(protocol: i32, id: i32) -> Option<&'static str> {
    let name = types(protocol).get(usize::try_from(id).ok()?)?;
    Some(bare(name))
}

/// Номер типа в каноне; `None` — такой блок-сущности в 26.3 нет (кровать).
fn canon_id(name: &str) -> Option<i32> {
    let i = r26_3::BLOCK_ENTITY_TYPE
        .iter()
        .position(|n| bare(n) == name)?;
    i32::try_from(i).ok()
}

/// Блок-сущность с номером `id` реестра версии → номер и NBT канона.
pub(crate) fn to_canon(protocol: i32, id: i32, tag: Tag) -> Option<(i32, Tag)> {
    let name = name_of(protocol, id)?;
    Some((canon_id(name)?, tag_to_canon(name, tag)))
}

/// Блок-сущность с типом в строке `id` (≤1.17.1) → номер и NBT канона.
pub(crate) fn named_to_canon(tag: Tag) -> Option<(i32, Tag)> {
    let name = bare(tag.get("id")?.as_str()?).to_owned();
    let id = canon_id(&name)?;
    Some((id, tag_to_canon(&name, tag)))
}

fn tag_to_canon(name: &str, tag: Tag) -> Tag {
    match name {
        "sign" | "hanging_sign" => sign(tag),
        "banner" => banner(tag),
        _ => tag,
    }
}

/// Строка с JSON-компонентом → значение; не JSON — простой текст.
fn component(raw: &str) -> Json {
    serde_json::from_str(raw).unwrap_or_else(|_| Json::String(raw.to_owned()))
}

/// Строки таблички → список NBT. Список NBT однороден, поэтому смесь компонентов и простых строк
/// выравнивает `json_to_tag`.
fn lines(raw: impl Iterator<Item = Json>) -> Tag {
    crate::v1201::json_to_tag(&Json::Array(raw.collect()))
}

/// Сторона таблички: строки `messages` (JSON до 1.21.5) → компоненты.
fn side_from_json(side: &mut Tag) {
    let Tag::Compound(fields) = side else { return };
    for (key, value) in fields {
        if let (true, Tag::List(list)) = (key == "messages" || key == "filtered_messages", &*value)
        {
            if list.iter().all(|l| matches!(l, Tag::String(_))) {
                *value = lines(list.iter().filter_map(Tag::as_str).map(component));
            }
        }
    }
}

/// VANILLA: SignBlockEntity#load — до 1.20 `Text1`..`Text4`, `Color`, `GlowingText`; с 1.20 до
/// 1.21.5 `front_text`/`back_text` со строками JSON.
pub(crate) fn sign(mut tag: Tag) -> Tag {
    if tag.get("front_text").is_some() || tag.get("back_text").is_some() {
        if let Tag::Compound(fields) = &mut tag {
            for (key, value) in fields {
                if key == "front_text" || key == "back_text" {
                    side_from_json(value);
                }
            }
        }
        return tag;
    }
    let messages = lines((1..=4).map(|i| {
        tag.get(&format!("Text{i}"))
            .and_then(Tag::as_str)
            .map_or_else(|| Json::String(String::new()), component)
    }));
    let color = tag.get("Color").and_then(Tag::as_str).unwrap_or("black");
    let glowing = tag.get("GlowingText").and_then(Tag::as_i64) == Some(1);
    let front = Tag::Compound(vec![
        ("messages".into(), messages),
        ("color".into(), Tag::String(color.to_owned())),
        ("has_glowing_text".into(), Tag::Byte(i8::from(glowing))),
    ]);
    Tag::Compound(vec![("front_text".into(), front)])
}

/// VANILLA: BannerBlockEntity (1.13–1.20.4): `Patterns` [{Pattern, Color}] → `patterns`.
fn banner(tag: Tag) -> Tag {
    let Some(Tag::List(layers)) = tag.get("Patterns") else {
        return tag;
    };
    let layers = layers
        .iter()
        .filter_map(|l| {
            let code = l.get("Pattern")?.as_str()?;
            let name = BANNER_PATTERNS.iter().find(|(c, _)| *c == code)?.1;
            let color = usize::try_from(l.get("Color")?.as_i64()?).ok()?;
            Some(Tag::Compound(vec![
                ("pattern".into(), Tag::String(format!("minecraft:{name}"))),
                (
                    "color".into(),
                    Tag::String((*COLORS.get(color)?).to_owned()),
                ),
            ]))
        })
        .collect();
    Tag::Compound(vec![("patterns".into(), Tag::List(layers))])
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::{nbt, wire::Reader};

    #[test]
    fn old_sign_becomes_front_text() {
        let tag = Tag::Compound(vec![
            ("id".into(), Tag::String("minecraft:sign".into())),
            ("Text1".into(), Tag::String(r#"{"text":"hi"}"#.into())),
            ("Color".into(), Tag::String("red".into())),
        ]);
        let (id, out) = named_to_canon(tag).unwrap();
        assert_eq!(bare(r26_3::BLOCK_ENTITY_TYPE[id as usize]), "sign");
        let front = out.get("front_text").unwrap();
        assert_eq!(front.get("color").and_then(Tag::as_str), Some("red"));
        let Some(Tag::List(lines)) = front.get("messages") else {
            panic!("no messages");
        };
        assert_eq!(lines[0].get("text").and_then(Tag::as_str), Some("hi"));
    }

    #[test]
    fn sign_with_empty_lines_writes_a_uniform_list() {
        let tag = Tag::Compound(vec![(
            "Text1".into(),
            Tag::String(r#"{"text":"hi"}"#.into()),
        )]);
        let mut out = Vec::new();
        nbt::write_network(&mut out, Some(&sign(tag))).unwrap();
        let back = nbt::read_network(&mut Reader::new(&out)).unwrap().unwrap();
        let Some(Tag::List(lines)) = back.get("front_text").and_then(|f| f.get("messages")) else {
            panic!("no messages");
        };
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0].get("text").and_then(Tag::as_str), Some("hi"));
    }

    #[test]
    fn numbered_sign_of_1_20_1_decodes_json_lines() {
        let front = Tag::Compound(vec![(
            "messages".into(),
            Tag::List(vec![Tag::String(r#"{"text":"a"}"#.into())]),
        )]);
        let tag = Tag::Compound(vec![("front_text".into(), front)]);
        let id = types(763).iter().position(|n| bare(n) == "sign").unwrap();
        let (_, out) = to_canon(763, id as i32, tag).unwrap();
        let Some(Tag::List(lines)) = out.get("front_text").and_then(|f| f.get("messages")) else {
            panic!("no messages");
        };
        assert_eq!(lines[0].get("text").and_then(Tag::as_str), Some("a"));
    }
}
