//! Разбор чат-пакетов 26.3 (system_chat / player_chat / disguised_chat) и оформление по ChatType.
//! VANILLA: ClientboundPlayerChatPacket, ClientboundDisguisedChatPacket, ClientboundSystemChatPacket,
//! ChatType.Bound#decorate, ChatTypeDecoration#decorate.
use ezzmc_net::session::RegistryData;
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::text::{Component, Content, Style};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// Параметры ChatTypeDecoration.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Param {
    Sender,
    Target,
    Content,
}

#[derive(Clone, Debug, Default)]
pub struct Decoration {
    pub key: String,
    pub params: Vec<Param>,
    pub style: Style,
}

fn style_from_nbt(t: &Tag) -> Style {
    // Style — compound без текста: добавляем пустой текст и берём стиль разобранного компонента.
    let Tag::Compound(v) = t else {
        return Style::default();
    };
    let mut v = v.clone();
    v.push(("text".into(), Tag::String(String::new())));
    Component::from_nbt(&Tag::Compound(v)).map_or_else(|_| Style::default(), |c| c.style)
}

fn decoration_from_nbt(t: &Tag) -> Option<Decoration> {
    let key = t.get("translation_key")?.as_str()?.to_owned();
    let params = match t.get("parameters") {
        Some(Tag::List(l)) => l
            .iter()
            .filter_map(|p| match p.as_str()? {
                "sender" => Some(Param::Sender),
                "target" => Some(Param::Target),
                "content" => Some(Param::Content),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };
    let style = t.get("style").map(style_from_nbt).unwrap_or_default();
    Some(Decoration { key, params, style })
}

fn gray_italic() -> Style {
    Style {
        color: Some(0xAAAAAA),
        italic: Some(true),
        ..Style::default()
    }
}

/// VANILLA: data/minecraft/chat_type/*.json — запасные значения, если сервер не прислал NBT (known pack).
fn default_decoration(name: &str) -> Decoration {
    use Param::{Content as C, Sender as S, Target as T};
    let n = name.strip_prefix("minecraft:").unwrap_or(name);
    let (key, params, style) = match n {
        "emote_command" => ("chat.type.emote", vec![S, C], Style::default()),
        "say_command" => ("chat.type.announcement", vec![S, C], Style::default()),
        "msg_command_incoming" => (
            "commands.message.display.incoming",
            vec![S, C],
            gray_italic(),
        ),
        "msg_command_outgoing" => (
            "commands.message.display.outgoing",
            vec![T, C],
            gray_italic(),
        ),
        "team_msg_command_incoming" => ("chat.type.team.text", vec![T, S, C], Style::default()),
        "team_msg_command_outgoing" => ("chat.type.team.sent", vec![T, S, C], Style::default()),
        _ => ("chat.type.text", vec![S, C], Style::default()),
    };
    Decoration {
        key: key.into(),
        params,
        style,
    }
}

/// Таблица `minecraft:chat_type` (индекс = holder id).
pub fn chat_types(reg: Option<&RegistryData>) -> Vec<Decoration> {
    let Some(r) = reg else { return Vec::new() };
    r.entries
        .iter()
        .map(|(name, data)| {
            data.as_ref()
                .and_then(|t| t.get("chat"))
                .and_then(decoration_from_nbt)
                .unwrap_or_else(|| default_decoration(name))
        })
        .collect()
}

pub struct Bound {
    pub deco: Decoration,
    pub name: Component,
    pub target: Option<Component>,
}

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

fn read_decoration(r: &mut Reader) -> ProtocolResult<Decoration> {
    let key = r.string(32767)?;
    let n = r.var_int()?.clamp(0, 16);
    let mut params = Vec::new();
    for _ in 0..n {
        match r.var_int()? {
            0 => params.push(Param::Sender),
            1 => params.push(Param::Target),
            _ => params.push(Param::Content),
        }
    }
    let style = nbt::read_network(r)?
        .map(|t| style_from_nbt(&t))
        .unwrap_or_default();
    Ok(Decoration { key, params, style })
}

/// ChatType.Bound: Holder<ChatType> (id+1, 0 — inline), name, optional targetName.
fn read_bound(r: &mut Reader, types: &[Decoration]) -> ProtocolResult<Bound> {
    let id = r.var_int()?;
    let deco = if id > 0 {
        types.get(id as usize - 1).cloned().unwrap_or_default()
    } else {
        let d = read_decoration(r)?;
        let _narration = read_decoration(r)?;
        d
    };
    let name = component(r)?;
    let target = if r.bool()? { Some(component(r)?) } else { None };
    Ok(Bound { deco, name, target })
}

/// VANILLA: ChatType.Bound#decorate → Component.translatable(key, params).withStyle(style).
pub fn decorate(b: &Bound, content: Component) -> Component {
    let args = b
        .deco
        .params
        .iter()
        .map(|p| match p {
            Param::Sender => b.name.clone(),
            Param::Target => b.target.clone().unwrap_or_else(|| Component::text("")),
            Param::Content => content.clone(),
        })
        .collect();
    Component {
        content: Content::Translatable {
            key: b.deco.key.clone(),
            fallback: None,
            args,
        },
        style: b.deco.style.clone(),
        siblings: Vec::new(),
    }
}

/// `system_chat`: (текст, overlay).
pub fn parse_system(body: &[u8]) -> ProtocolResult<(Component, bool)> {
    let mut r = Reader::new(body);
    let c = component(&mut r)?;
    Ok((c, r.bool()?))
}

/// `disguised_chat`: оформленный текст.
pub fn parse_disguised(body: &[u8], types: &[Decoration]) -> ProtocolResult<Component> {
    let mut r = Reader::new(body);
    let msg = component(&mut r)?;
    let b = read_bound(&mut r, types)?;
    Ok(decorate(&b, msg))
}

/// `player_chat`: оформленный текст (подписи и фильтры пропускаются — клиент без ключей).
pub fn parse_player(body: &[u8], types: &[Decoration]) -> ProtocolResult<Component> {
    let mut r = Reader::new(body);
    r.var_int()?; // globalIndex
    r.uuid()?; // sender
    r.var_int()?; // index
    if r.bool()? {
        r.bytes(256)?; // signature
    }
    let content = r.string(256)?;
    r.i64()?; // timestamp
    r.i64()?; // salt
    let n = r.var_int()?.clamp(0, 20);
    for _ in 0..n {
        if r.var_int()? == 0 {
            r.bytes(256)?;
        }
    }
    let unsigned = if r.bool()? {
        Some(component(&mut r)?)
    } else {
        None
    };
    // FilterMask: 0 пропуск, 1 всё скрыто, 2 частично + bitset.
    let filter = r.var_int()?;
    if filter == 2 {
        let n = r.var_int()?.clamp(0, 4096);
        for _ in 0..n {
            r.i64()?;
        }
    }
    let b = read_bound(&mut r, types)?;
    let msg = unsigned.unwrap_or_else(|| Component::text(content));
    Ok(decorate(&b, msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    fn nbt_str(w: &mut Writer, s: &str) {
        nbt::write_network(&mut w.buf, Some(&Tag::String(s.into()))).unwrap();
    }

    #[test]
    fn player_chat_text() {
        let types = vec![default_decoration("minecraft:chat")];
        let mut w = Writer::new();
        w.var_int(0).uuid(1).var_int(0).bool(false);
        w.string("hello").i64(0).i64(0).var_int(0);
        w.bool(false).var_int(0);
        w.var_int(1); // chat_type id 0 + 1
        nbt_str(&mut w, "Bob");
        w.bool(false);
        let c = parse_player(&w.buf, &types).unwrap();
        let s = c.to_plain(&|k| (k == "chat.type.text").then(|| "<%s> %s".to_owned()));
        assert_eq!(s, "<Bob> hello");
    }

    #[test]
    fn system() {
        let mut w = Writer::new();
        nbt_str(&mut w, "hi");
        w.bool(true);
        let (c, o) = parse_system(&w.buf).unwrap();
        assert!(o);
        assert_eq!(c.to_plain(&|_| None), "hi");
    }
}

/// VANILLA: ClientboundCommandSuggestionsPacket — varint id, start, length, список (строка + необязательный тултип).
pub fn parse_suggestions(body: &[u8]) -> ProtocolResult<(i32, i32, i32, Vec<String>)> {
    let mut r = Reader::new(body);
    let (id, start, len) = (r.var_int()?, r.var_int()?, r.var_int()?);
    let n = r.var_int()?.clamp(0, 10_000);
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(r.string(32767)?);
        if r.bool()? {
            component(&mut r)?;
        }
    }
    Ok((id, start, len, v))
}
