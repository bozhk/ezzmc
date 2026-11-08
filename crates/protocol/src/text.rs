//! Text Component: разбор из JSON и NBT, стили, события, плоский текст (TODO-056).
// VANILLA: ComponentSerialization, Style, ClickEvent/HoverEvent (1.21.5+), TranslatableContents
// Отрисовка — в GUI (гл. 11). Здесь только модель и разбор. Legacy-форматы (clickEvent/hoverEvent
// с `value`) принимаются как есть; полный разбор старых версий — гл. 18.
use crate::nbt::Tag;
use crate::{ProtocolError, ProtocolResult};
use serde_json::Value;

const MAX_DEPTH: usize = 64;

/// VANILLA: ChatFormatting цвета.
pub const NAMED_COLORS: [(&str, u32); 16] = [
    ("black", 0x000000),
    ("dark_blue", 0x0000AA),
    ("dark_green", 0x00AA00),
    ("dark_aqua", 0x00AAAA),
    ("dark_red", 0xAA0000),
    ("dark_purple", 0xAA00AA),
    ("gold", 0xFFAA00),
    ("gray", 0xAAAAAA),
    ("dark_gray", 0x555555),
    ("blue", 0x5555FF),
    ("green", 0x55FF55),
    ("aqua", 0x55FFFF),
    ("red", 0xFF5555),
    ("light_purple", 0xFF55FF),
    ("yellow", 0xFFFF55),
    ("white", 0xFFFFFF),
];

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Text(String),
    Translatable {
        key: String,
        fallback: Option<String>,
        args: Vec<Component>,
    },
    Score {
        name: String,
        objective: String,
    },
    Selector {
        pattern: String,
        separator: Option<Box<Component>>,
    },
    Keybind(String),
    Nbt {
        path: String,
        interpret: bool,
        source: NbtSource,
        separator: Option<Box<Component>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum NbtSource {
    Block(String),
    Entity(String),
    Storage(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClickEvent {
    OpenUrl(String),
    OpenFile(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(i32),
    CopyToClipboard(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoverEvent {
    ShowText(Box<Component>),
    /// Предмет/сущность храним как есть (разбор — вместе с Slot, TODO-057).
    ShowItem(Value),
    ShowEntity(Value),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    /// 0xRRGGBB
    pub color: Option<u32>,
    pub shadow_color: Option<i32>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub font: Option<String>,
    pub insertion: Option<String>,
    pub click: Option<ClickEvent>,
    pub hover: Option<HoverEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    pub content: Content,
    pub style: Style,
    pub siblings: Vec<Component>,
}

impl Component {
    pub fn text(s: impl Into<String>) -> Self {
        Self {
            content: Content::Text(s.into()),
            style: Style::default(),
            siblings: Vec::new(),
        }
    }

    pub fn from_json_str(s: &str) -> ProtocolResult<Self> {
        let v: Value =
            serde_json::from_str(s).map_err(|e| ProtocolError::Other(format!("json: {e}")))?;
        Self::from_json(&v)
    }

    pub fn from_json(v: &Value) -> ProtocolResult<Self> {
        parse(v, 0)
    }

    /// Сетевой формат 1.20.3+: компонент как NBT (строка / список / compound).
    pub fn from_nbt(t: &Tag) -> ProtocolResult<Self> {
        parse(&nbt_to_json(t), 0)
    }

    /// Плоский текст без стилей. `lang`: ключ → шаблон (`%s`, `%1$s`). Нет перевода → fallback → ключ.
    pub fn to_plain(&self, lang: &dyn Fn(&str) -> Option<String>) -> String {
        let mut out = String::new();
        self.write_plain(lang, &mut out);
        out
    }

    fn write_plain(&self, lang: &dyn Fn(&str) -> Option<String>, out: &mut String) {
        match &self.content {
            Content::Text(s) => out.push_str(s),
            Content::Translatable {
                key,
                fallback,
                args,
            } => {
                let tpl = lang(key)
                    .or_else(|| fallback.clone())
                    .unwrap_or_else(|| key.clone());
                format_translation(&tpl, args, lang, out);
            }
            Content::Score { name, .. } => out.push_str(name),
            Content::Selector { pattern, .. } => out.push_str(pattern),
            Content::Keybind(k) => out.push_str(k),
            Content::Nbt { path, .. } => out.push_str(path),
        }
        for s in &self.siblings {
            s.write_plain(lang, out);
        }
    }
}

/// VANILLA: TranslatableContents#decomposeTemplate — `%s`, `%n$s`, `%%`; неверный формат → как есть.
fn format_translation(
    tpl: &str,
    args: &[Component],
    lang: &dyn Fn(&str) -> Option<String>,
    out: &mut String,
) {
    let b: Vec<char> = tpl.chars().collect();
    let (mut i, mut next) = (0, 0usize);
    while i < b.len() {
        if b[i] != '%' {
            out.push(b[i]);
            i += 1;
            continue;
        }
        if b.get(i + 1) == Some(&'%') {
            out.push('%');
            i += 2;
            continue;
        }
        // %s | %<digits>$s
        let mut j = i + 1;
        let mut num = String::new();
        while j < b.len() && b[j].is_ascii_digit() {
            num.push(b[j]);
            j += 1;
        }
        let idx = if b.get(j) == Some(&'$') && !num.is_empty() {
            j += 1;
            num.parse::<usize>().ok().map(|n| n.wrapping_sub(1))
        } else if num.is_empty() {
            let n = next;
            next += 1;
            Some(n)
        } else {
            None
        };
        if b.get(j) == Some(&'s') {
            if let Some(a) = idx.and_then(|n| args.get(n)) {
                a.write_plain(lang, out);
            }
            i = j + 1;
        } else {
            out.push('%');
            i += 1;
        }
    }
}

// ---------- разбор ----------

fn bad(what: &'static str) -> ProtocolError {
    ProtocolError::InvalidValue { what, value: 0 }
}

fn nbt_to_json(t: &Tag) -> Value {
    match t {
        Tag::Byte(v) => Value::from(*v),
        Tag::Short(v) => Value::from(*v),
        Tag::Int(v) => Value::from(*v),
        Tag::Long(v) => Value::from(*v),
        Tag::Float(v) => {
            serde_json::Number::from_f64(f64::from(*v)).map_or(Value::Null, Value::Number)
        }
        Tag::Double(v) => serde_json::Number::from_f64(*v).map_or(Value::Null, Value::Number),
        Tag::String(s) => Value::String(s.clone()),
        Tag::List(v) => Value::Array(v.iter().map(nbt_to_json).collect()),
        Tag::Compound(v) => {
            Value::Object(v.iter().map(|(k, t)| (k.clone(), nbt_to_json(t))).collect())
        }
        Tag::ByteArray(v) => Value::Array(v.iter().map(|x| Value::from(*x)).collect()),
        Tag::IntArray(v) => Value::Array(v.iter().map(|x| Value::from(*x)).collect()),
        Tag::LongArray(v) => Value::Array(v.iter().map(|x| Value::from(*x)).collect()),
    }
}

fn as_bool(v: &Value) -> Option<bool> {
    match v {
        Value::Bool(b) => Some(*b),
        Value::Number(n) => n.as_i64().map(|x| x != 0),
        _ => None,
    }
}

fn str_of(v: &Value) -> Option<String> {
    v.as_str().map(str::to_owned)
}

fn parse(v: &Value, depth: usize) -> ProtocolResult<Component> {
    if depth > MAX_DEPTH {
        return Err(ProtocolError::TooLarge {
            what: "text component depth",
            len: depth,
            limit: MAX_DEPTH,
        });
    }
    match v {
        Value::String(s) => Ok(Component::text(s.clone())),
        Value::Bool(b) => Ok(Component::text(b.to_string())),
        Value::Number(n) => Ok(Component::text(n.to_string())),
        Value::Array(a) => {
            // VANILLA: первый элемент — база, остальные — siblings
            let mut it = a.iter();
            let mut first = parse(
                it.next().ok_or_else(|| bad("empty component array"))?,
                depth + 1,
            )?;
            for x in it {
                first.siblings.push(parse(x, depth + 1)?);
            }
            Ok(first)
        }
        Value::Object(o) => {
            let content = parse_content(o, depth)?;
            let style = parse_style(o, depth)?;
            let mut siblings = Vec::new();
            if let Some(Value::Array(a)) = o.get("extra") {
                for x in a {
                    siblings.push(parse(x, depth + 1)?);
                }
            }
            Ok(Component {
                content,
                style,
                siblings,
            })
        }
        Value::Null => Err(bad("null component")),
    }
}

fn parse_content(o: &serde_json::Map<String, Value>, depth: usize) -> ProtocolResult<Content> {
    if let Some(t) = o.get("text") {
        return Ok(Content::Text(match t {
            Value::String(s) => s.clone(),
            other => parse(other, depth + 1)?.to_plain(&|_| None),
        }));
    }
    if let Some(k) = o.get("translate").and_then(str_of) {
        let mut args = Vec::new();
        if let Some(Value::Array(a)) = o.get("with").or_else(|| o.get("args")) {
            for x in a {
                args.push(parse(x, depth + 1)?);
            }
        }
        return Ok(Content::Translatable {
            key: k,
            fallback: o.get("fallback").and_then(str_of),
            args,
        });
    }
    if let Some(Value::Object(s)) = o.get("score") {
        return Ok(Content::Score {
            name: s
                .get("name")
                .and_then(str_of)
                .ok_or_else(|| bad("score.name"))?,
            objective: s
                .get("objective")
                .and_then(str_of)
                .ok_or_else(|| bad("score.objective"))?,
        });
    }
    let sep = |o: &serde_json::Map<String, Value>| -> ProtocolResult<Option<Box<Component>>> {
        o.get("separator")
            .map(|s| parse(s, depth + 1).map(Box::new))
            .transpose()
    };
    if let Some(p) = o.get("selector").and_then(str_of) {
        return Ok(Content::Selector {
            pattern: p,
            separator: sep(o)?,
        });
    }
    if let Some(k) = o.get("keybind").and_then(str_of) {
        return Ok(Content::Keybind(k));
    }
    if let Some(path) = o.get("nbt").and_then(str_of) {
        let source = if let Some(b) = o.get("block").and_then(str_of) {
            NbtSource::Block(b)
        } else if let Some(e) = o.get("entity").and_then(str_of) {
            NbtSource::Entity(e)
        } else if let Some(s) = o.get("storage").and_then(str_of) {
            NbtSource::Storage(s)
        } else {
            return Err(bad("nbt source"));
        };
        return Ok(Content::Nbt {
            path,
            interpret: o.get("interpret").and_then(as_bool).unwrap_or(false),
            source,
            separator: sep(o)?,
        });
    }
    // VANILLA: компонент без содержимого допустим (только стиль/extra) — пустой текст
    Ok(Content::Text(String::new()))
}

fn parse_color(s: &str) -> Option<u32> {
    if let Some(h) = s.strip_prefix('#') {
        return (h.len() == 6)
            .then(|| u32::from_str_radix(h, 16).ok())
            .flatten();
    }
    NAMED_COLORS.iter().find(|(n, _)| *n == s).map(|(_, c)| *c)
}

fn parse_style(o: &serde_json::Map<String, Value>, depth: usize) -> ProtocolResult<Style> {
    let b = |k: &str| o.get(k).and_then(as_bool);
    let click = o
        .get("click_event")
        .or_else(|| o.get("clickEvent"))
        .map(parse_click)
        .transpose()?;
    let hover = o
        .get("hover_event")
        .or_else(|| o.get("hoverEvent"))
        .map(|h| parse_hover(h, depth))
        .transpose()?;
    Ok(Style {
        // неизвестный цвет: ванильный codec даст ошибку; мы мягче — игнорируем
        color: o.get("color").and_then(Value::as_str).and_then(parse_color),
        shadow_color: o.get("shadow_color").and_then(|v| match v {
            Value::Number(n) => n.as_i64().map(|x| x as i32),
            Value::Array(a) if a.len() == 4 => {
                let c: Vec<i64> = a
                    .iter()
                    .filter_map(|x| x.as_f64().map(|f| (f * 255.0).round() as i64))
                    .collect();
                (c.len() == 4).then(|| ((c[3] << 24) | (c[0] << 16) | (c[1] << 8) | c[2]) as i32)
            }
            _ => None,
        }),
        bold: b("bold"),
        italic: b("italic"),
        underlined: b("underlined"),
        strikethrough: b("strikethrough"),
        obfuscated: b("obfuscated"),
        font: o.get("font").and_then(str_of),
        insertion: o.get("insertion").and_then(str_of),
        click,
        hover,
    })
}

fn parse_click(v: &Value) -> ProtocolResult<ClickEvent> {
    let o = v.as_object().ok_or_else(|| bad("click_event"))?;
    let action = o
        .get("action")
        .and_then(Value::as_str)
        .ok_or_else(|| bad("click action"))?;
    // 1.21.5+: поля url/path/command/page/value; legacy: всегда value
    let get = |k: &str| o.get(k).or_else(|| o.get("value")).and_then(str_of);
    let need = |s: Option<String>| s.ok_or_else(|| bad("click value"));
    Ok(match action {
        "open_url" => ClickEvent::OpenUrl(need(get("url"))?),
        "open_file" => ClickEvent::OpenFile(need(get("path"))?),
        "run_command" => ClickEvent::RunCommand(need(get("command"))?),
        "suggest_command" => ClickEvent::SuggestCommand(need(get("command"))?),
        "copy_to_clipboard" => ClickEvent::CopyToClipboard(need(get("value"))?),
        "change_page" => {
            let p = o
                .get("page")
                .or_else(|| o.get("value"))
                .ok_or_else(|| bad("page"))?;
            let n = p
                .as_i64()
                .or_else(|| p.as_str().and_then(|s| s.parse().ok()))
                .ok_or_else(|| bad("page"))?;
            ClickEvent::ChangePage(i32::try_from(n).map_err(|_| bad("page"))?)
        }
        _ => return Err(bad("click action")),
    })
}

fn parse_hover(v: &Value, depth: usize) -> ProtocolResult<HoverEvent> {
    let o = v.as_object().ok_or_else(|| bad("hover_event"))?;
    let action = o
        .get("action")
        .and_then(Value::as_str)
        .ok_or_else(|| bad("hover action"))?;
    match action {
        "show_text" => {
            let c = o
                .get("value")
                .or_else(|| o.get("contents"))
                .ok_or_else(|| bad("hover value"))?;
            Ok(HoverEvent::ShowText(Box::new(parse(c, depth + 1)?)))
        }
        // 1.21.5+: поля лежат прямо в событии; legacy: в `contents`
        "show_item" => Ok(HoverEvent::ShowItem(
            o.get("contents").cloned().unwrap_or_else(|| v.clone()),
        )),
        "show_entity" => Ok(HoverEvent::ShowEntity(
            o.get("contents").cloned().unwrap_or_else(|| v.clone()),
        )),
        _ => Err(bad("hover action")),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn no_lang(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn plain_forms() {
        assert_eq!(
            Component::from_json_str("\"hi\"")
                .unwrap()
                .to_plain(&no_lang),
            "hi"
        );
        assert_eq!(
            Component::from_json_str("42").unwrap().to_plain(&no_lang),
            "42"
        );
        let c = Component::from_json_str(r#"["a",{"text":"b"},"c"]"#).unwrap();
        assert_eq!(c.to_plain(&no_lang), "abc");
        let c = Component::from_json_str(r#"{"text":"x","extra":["y",{"text":"z","bold":true}]}"#)
            .unwrap();
        assert_eq!(c.to_plain(&no_lang), "xyz");
        assert_eq!(c.siblings[1].style.bold, Some(true));
    }

    #[test]
    fn style_and_events() {
        let c = Component::from_json_str(
            r##"{"text":"t","color":"#ff8800","italic":false,"font":"minecraft:alt",
            "click_event":{"action":"open_url","url":"https://x"},
            "hover_event":{"action":"show_text","value":"tip"}}"##,
        )
        .unwrap();
        assert_eq!(c.style.color, Some(0xff8800));
        assert_eq!(c.style.italic, Some(false));
        assert_eq!(c.style.click, Some(ClickEvent::OpenUrl("https://x".into())));
        assert!(matches!(c.style.hover, Some(HoverEvent::ShowText(_))));
        let c = Component::from_json_str(
            r#"{"text":"t","color":"gold","clickEvent":{"action":"run_command","value":"/x"}}"#,
        )
        .unwrap();
        assert_eq!(c.style.color, Some(0xFFAA00));
        assert_eq!(c.style.click, Some(ClickEvent::RunCommand("/x".into())));
    }

    #[test]
    fn translate() {
        let lang = |k: &str| match k {
            "chat.type.text" => Some("<%s> %s".to_owned()),
            "x.y" => Some("%2$s then %1$s, 100%%".to_owned()),
            _ => None,
        };
        let c = Component::from_json_str(
            r#"{"translate":"chat.type.text","with":["Bob",{"text":"yo"}]}"#,
        )
        .unwrap();
        assert_eq!(c.to_plain(&lang), "<Bob> yo");
        let c = Component::from_json_str(r#"{"translate":"x.y","with":["A","B"]}"#).unwrap();
        assert_eq!(c.to_plain(&lang), "B then A, 100%");
        let c =
            Component::from_json_str(r#"{"translate":"nope.key","fallback":"fb %s","with":["Q"]}"#)
                .unwrap();
        assert_eq!(c.to_plain(&lang), "fb Q");
        let c = Component::from_json_str(r#"{"translate":"nope.key"}"#).unwrap();
        assert_eq!(c.to_plain(&lang), "nope.key");
    }

    #[test]
    fn other_contents() {
        assert!(matches!(
            Component::from_json_str(r#"{"score":{"name":"a","objective":"o"}}"#)
                .unwrap()
                .content,
            Content::Score { .. }
        ));
        assert!(matches!(
            Component::from_json_str(r#"{"keybind":"key.jump"}"#)
                .unwrap()
                .content,
            Content::Keybind(_)
        ));
        assert!(matches!(
            Component::from_json_str(r#"{"nbt":"a.b","storage":"ns:s","interpret":true}"#)
                .unwrap()
                .content,
            Content::Nbt {
                interpret: true,
                ..
            }
        ));
        assert!(Component::from_json_str("[]").is_err());
        assert!(Component::from_json_str("null").is_err());
        assert!(Component::from_json_str("{").is_err());
    }

    #[test]
    fn from_nbt_network_form() {
        let t = crate::nbt::parse_snbt("{text:'a',bold:1b,extra:[{text:'b'},{text:'c'}]}").unwrap();
        let c = Component::from_nbt(&t).unwrap();
        assert_eq!(c.to_plain(&no_lang), "abc");
        assert_eq!(c.style.bold, Some(true));
        assert_eq!(
            Component::from_nbt(&Tag::String("s".into()))
                .unwrap()
                .to_plain(&no_lang),
            "s"
        );
    }

    #[test]
    fn depth_limited() {
        let mut s = String::new();
        for _ in 0..200 {
            s.push_str(r#"{"text":"a","extra":["#);
        }
        s.push_str("\"x\"");
        for _ in 0..200 {
            s.push_str("]}");
        }
        assert!(Component::from_json_str(&s).is_err());
    }
}
