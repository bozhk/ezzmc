//! Диалоги сервера (Dialog API, 1.21.6+): модель, разбор NBT, шаблоны действий.
//! VANILLA: net.minecraft.server.dialog.* (Dialog, CommonDialogData, ActionButton, Input*, DialogBody,
//! action.*), ClientboundShowDialogPacket / ClientboundClearDialogPacket.
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::text::Component;
use std::sync::Mutex;

/// Записи реестра `minecraft:dialog` из Configuration (id, NBT; None = из известного пака).
/// VANILLA: Holder<Dialog> по сетевому id — индекс записи реестра.
pub static DIALOG_REG: Mutex<Vec<(String, Option<Tag>)>> = Mutex::new(Vec::new());

/// VANILLA: DialogAction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum After {
    Close,
    None,
    WaitForResponse,
}

/// VANILLA: ClickEvent (разрешённые с сервера варианты).
#[derive(Clone, Debug)]
pub enum Click {
    RunCommand(String),
    OpenUrl(String),
    SuggestCommand(String),
    CopyToClipboard(String),
    ChangePage,
    ShowDialog(Box<Dialog>),
    Custom(String, Option<Tag>),
}

/// VANILLA: server.dialog.action.Action.
#[derive(Clone, Debug)]
pub enum Act {
    Static(Click),
    /// dynamic/run_command: шаблон с `$(имя)`.
    Template(String),
    /// dynamic/custom: id + additions, значения ввода добавляются в compound.
    Custom(String, Option<Tag>),
}

/// VANILLA: ActionButton + CommonButtonData.
#[derive(Clone, Debug)]
pub struct Btn {
    pub label: Component,
    pub tooltip: Option<Component>,
    pub width: i32,
    pub action: Option<Act>,
}

#[derive(Clone, Debug)]
pub enum Body {
    /// PlainMessage: текст и максимальная ширина.
    Text(Component, i32),
    /// ItemBody — предмет пока не рисуем (описание показываем).
    Item(Option<(Component, i32)>),
}

#[derive(Clone, Debug)]
pub enum Control {
    Text {
        width: i32,
        label: Component,
        label_visible: bool,
        initial: String,
        max_len: usize,
        /// Some((max_lines, height)) — многострочное поле.
        multiline: Option<(Option<i32>, Option<i32>)>,
    },
    Bool {
        label: Component,
        initial: bool,
        on_true: String,
        on_false: String,
    },
    /// (id, display, initial)
    Option {
        width: i32,
        entries: Vec<(String, Option<Component>, bool)>,
        label: Component,
        label_visible: bool,
    },
    Range {
        width: i32,
        label: Component,
        label_format: String,
        start: f32,
        end: f32,
        initial: Option<f32>,
        step: Option<f32>,
    },
}

#[derive(Clone, Debug)]
pub struct Input {
    pub key: String,
    pub control: Control,
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Kind {
    /// Notice / Confirmation: основные кнопки в футере.
    Simple(Vec<Btn>),
    /// MultiAction / ServerLinks / DialogList: сетка кнопок + exit.
    List {
        buttons: Vec<Btn>,
        exit: Option<Btn>,
        columns: usize,
    },
}

#[derive(Clone, Debug)]
pub struct Dialog {
    pub title: Component,
    pub can_close_with_escape: bool,
    pub after: After,
    pub body: Vec<Body>,
    pub inputs: Vec<Input>,
    /// onCancel: действие при закрытии (Esc / нет кнопок).
    pub on_cancel: Option<Act>,
    pub kind: Kind,
}

fn comp(t: &Tag) -> Option<Component> {
    Component::from_nbt(t).ok()
}

fn boolean(t: Option<&Tag>, def: bool) -> bool {
    t.and_then(Tag::as_i64).map_or(def, |v| v != 0)
}

fn num(t: Option<&Tag>) -> Option<f64> {
    match t? {
        Tag::Float(v) => Some(f64::from(*v)),
        Tag::Double(v) => Some(*v),
        o => o.as_i64().map(|v| v as f64),
    }
}

fn int(t: Option<&Tag>, def: i32) -> i32 {
    t.and_then(Tag::as_i64).map_or(def, |v| v as i32)
}

fn strng(t: Option<&Tag>, def: &str) -> String {
    t.and_then(Tag::as_str).unwrap_or(def).to_owned()
}

fn plain_message(t: &Tag) -> Option<(Component, i32)> {
    // PlainMessage.CODEC: compound с `contents` либо просто компонент.
    if let Some(c) = t.get("contents") {
        Some((comp(c)?, int(t.get("width"), 200)))
    } else {
        Some((comp(t)?, 200))
    }
}

fn parse_click(t: &Tag) -> Option<Click> {
    let ty = t.get("type")?.as_str()?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    let s = |k: &str| t.get(k).and_then(Tag::as_str).map(str::to_owned);
    Some(match ty {
        "run_command" => Click::RunCommand(s("command")?),
        "open_url" => Click::OpenUrl(s("url")?),
        "suggest_command" => Click::SuggestCommand(s("command")?),
        "copy_to_clipboard" => Click::CopyToClipboard(s("value")?),
        "change_page" => Click::ChangePage,
        "show_dialog" => Click::ShowDialog(Box::new(holder(t.get("dialog")?)?)),
        "custom" => Click::Custom(s("id")?, t.get("payload").cloned()),
        _ => return None,
    })
}

fn parse_action(t: &Tag) -> Option<Act> {
    let ty = t.get("type")?.as_str()?;
    match ty.strip_prefix("minecraft:").unwrap_or(ty) {
        "dynamic/run_command" => Some(Act::Template(t.get("template")?.as_str()?.to_owned())),
        "dynamic/custom" => Some(Act::Custom(
            t.get("id")?.as_str()?.to_owned(),
            t.get("additions").cloned(),
        )),
        _ => parse_click(t).map(Act::Static),
    }
}

fn parse_btn(t: &Tag) -> Option<Btn> {
    Some(Btn {
        label: comp(t.get("label")?)?,
        tooltip: t.get("tooltip").and_then(comp),
        width: int(t.get("width"), 150),
        action: t.get("action").and_then(parse_action),
    })
}

fn parse_control(t: &Tag) -> Option<Control> {
    let ty = t.get("type")?.as_str()?;
    let label = || comp(t.get("label")?);
    Some(match ty.strip_prefix("minecraft:").unwrap_or(ty) {
        "text" => Control::Text {
            width: int(t.get("width"), 200),
            label: label()?,
            label_visible: boolean(t.get("label_visible"), true),
            initial: strng(t.get("initial"), ""),
            max_len: int(t.get("max_length"), 32).max(1) as usize,
            multiline: t.get("multiline").map(|m| {
                (
                    m.get("max_lines").and_then(Tag::as_i64).map(|v| v as i32),
                    m.get("height").and_then(Tag::as_i64).map(|v| v as i32),
                )
            }),
        },
        "boolean" => Control::Bool {
            label: label()?,
            initial: boolean(t.get("initial"), false),
            on_true: strng(t.get("on_true"), "true"),
            on_false: strng(t.get("on_false"), "false"),
        },
        "single_option" => {
            let Some(Tag::List(l)) = t.get("options") else {
                return None;
            };
            let entries: Vec<_> = l
                .iter()
                .filter_map(|e| match e {
                    Tag::String(id) => Some((id.clone(), None, false)),
                    _ => Some((
                        e.get("id")?.as_str()?.to_owned(),
                        e.get("display").and_then(comp),
                        boolean(e.get("initial"), false),
                    )),
                })
                .collect();
            if entries.is_empty() {
                return None;
            }
            Control::Option {
                width: int(t.get("width"), 200),
                entries,
                label: label()?,
                label_visible: boolean(t.get("label_visible"), true),
            }
        }
        "number_range" => Control::Range {
            width: int(t.get("width"), 200),
            label: label()?,
            label_format: strng(t.get("label_format"), "options.generic_value"),
            start: num(t.get("start"))? as f32,
            end: num(t.get("end"))? as f32,
            initial: num(t.get("initial")).map(|v| v as f32),
            step: num(t.get("step")).map(|v| v as f32),
        },
        _ => return None,
    })
}

fn parse_body(t: &Tag) -> Option<Body> {
    let ty = t.get("type")?.as_str()?;
    match ty.strip_prefix("minecraft:").unwrap_or(ty) {
        "plain_message" => plain_message(t).map(|(c, w)| Body::Text(c, w)),
        "item" => Some(Body::Item(t.get("description").and_then(plain_message))),
        _ => None,
    }
}

/// Holder<Dialog> из NBT: строка — id записи реестра, compound — встроенный диалог.
fn holder(t: &Tag) -> Option<Dialog> {
    match t {
        Tag::String(id) => {
            let id = id.strip_prefix("minecraft:").unwrap_or(id).to_owned();
            let reg = DIALOG_REG.lock().ok()?;
            let data = reg
                .iter()
                .find(|(n, _)| n.strip_prefix("minecraft:").unwrap_or(n) == id)
                .and_then(|(_, d)| d.clone())?;
            drop(reg);
            parse(&data)
        }
        _ => parse(t),
    }
}

/// VANILLA: Dialog.DIRECT_CODEC.
pub fn parse(t: &Tag) -> Option<Dialog> {
    let ty = t.get("type")?.as_str()?;
    let ty = ty.strip_prefix("minecraft:").unwrap_or(ty);
    let title = comp(t.get("title")?)?;
    let body = match t.get("body") {
        None => Vec::new(),
        Some(Tag::List(l)) => l.iter().filter_map(parse_body).collect(),
        Some(one) => parse_body(one).into_iter().collect(),
    };
    let inputs = match t.get("inputs") {
        Some(Tag::List(l)) => l
            .iter()
            .filter_map(|i| {
                Some(Input {
                    key: i.get("key")?.as_str()?.to_owned(),
                    control: parse_control(i)?,
                })
            })
            .collect(),
        _ => Vec::new(),
    };
    let after = match t.get("after_action").and_then(Tag::as_str) {
        Some("none") => After::None,
        Some("wait_for_response") => After::WaitForResponse,
        _ => After::Close,
    };
    let btn = |k: &str| t.get(k).and_then(parse_btn);
    let columns = || int(t.get("columns"), 2).max(1) as usize;
    let width = || int(t.get("button_width"), 150);
    let (kind, on_cancel) = match ty {
        "notice" => {
            let b = btn("action").unwrap_or(Btn {
                label: Component::from_json_str(r#"{"translate":"gui.ok"}"#)
                    .unwrap_or_else(|_| Component::text("OK")),
                tooltip: None,
                width: 150,
                action: None,
            });
            let oc = b.action.clone();
            (Kind::Simple(vec![b]), oc)
        }
        "confirmation" => {
            let (y, n) = (btn("yes")?, btn("no")?);
            let oc = n.action.clone();
            (Kind::Simple(vec![y, n]), oc)
        }
        "multi_action" => {
            let Some(Tag::List(l)) = t.get("actions") else {
                return None;
            };
            let buttons: Vec<_> = l.iter().filter_map(parse_btn).collect();
            let exit = btn("exit_action");
            let oc = exit.as_ref().and_then(|e| e.action.clone());
            (
                Kind::List {
                    buttons,
                    exit,
                    columns: columns(),
                },
                oc,
            )
        }
        "server_links" => {
            // Ссылки сервера клиент не хранит (пакет server_links) — список пуст, остаётся exit.
            let exit = btn("exit_action");
            let oc = exit.as_ref().and_then(|e| e.action.clone());
            (
                Kind::List {
                    buttons: Vec::new(),
                    exit,
                    columns: columns(),
                },
                oc,
            )
        }
        "dialog_list" => {
            let w = width();
            let list: Vec<Tag> = match t.get("dialogs") {
                Some(Tag::List(l)) => l.clone(),
                Some(one) => vec![one.clone()],
                None => Vec::new(),
            };
            let buttons: Vec<_> = list
                .iter()
                .filter_map(holder)
                .map(|d| Btn {
                    // VANILLA: DialogListDialogScreen — кнопка с external_title, действие show_dialog.
                    label: d.title.clone(),
                    tooltip: None,
                    width: w,
                    action: Some(Act::Static(Click::ShowDialog(Box::new(d)))),
                })
                .collect();
            let exit = btn("exit_action");
            let oc = exit.as_ref().and_then(|e| e.action.clone());
            (
                Kind::List {
                    buttons,
                    exit,
                    columns: columns(),
                },
                oc,
            )
        }
        _ => return None,
    };
    Some(Dialog {
        title,
        can_close_with_escape: boolean(t.get("can_close_with_escape"), true),
        after,
        body,
        inputs,
        on_cancel,
        kind,
    })
}

/// Сетевой Holder<Dialog>: varint 0 + NBT (встроенный) или id реестра + 1.
pub fn parse_packet(body: &[u8]) -> Option<Dialog> {
    use ezzmc_protocol::nbt;
    use ezzmc_protocol::wire::Reader;
    let mut r = Reader::new(body);
    let id = r.var_int().ok()?;
    if id == 0 {
        let t = nbt::read_network(&mut r).ok()??;
        parse(&t)
    } else {
        let reg = DIALOG_REG.lock().ok()?;
        let data = reg.get((id - 1) as usize)?.1.clone()?;
        drop(reg);
        parse(&data)
    }
}

/// VANILLA: StringTag#escapeWithoutQuotes.
pub fn escape_without_quotes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' | '\'' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

/// VANILLA: ParsedTemplate#instantiate (`$(имя)`, неизвестное → пусто).
pub fn instantiate(raw: &str, args: &dyn Fn(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = raw;
    while let Some(i) = rest.find("$(") {
        let Some(end) = rest[i..].find(')') else {
            break;
        };
        out.push_str(&rest[..i]);
        out.push_str(&args(&rest[i + 2..i + end]));
        rest = &rest[i + end + 1..];
    }
    out.push_str(rest);
    out
}

/// VANILLA: NumberRangeInput.RangeInfo#computeScaledValue.
pub fn range_value(
    start: f32,
    end: f32,
    initial: Option<f32>,
    step: Option<f32>,
    slider: f32,
) -> f32 {
    let value = start + (end - start) * slider;
    let Some(step) = step else { return value };
    let init = initial.unwrap_or((start + end) / 2.0);
    let n = ((value - init) / step).round();
    let out_of_range = |v: f32| {
        let p = if start == end {
            0.5
        } else {
            (v - start) / (end - start)
        };
        !(0.0..=1.0).contains(&p)
    };
    let r = init + n * step;
    if !out_of_range(r) {
        return r;
    }
    let sign = if n > 0.0 {
        1.0
    } else if n < 0.0 {
        -1.0
    } else {
        0.0
    };
    init + (n - sign) * step
}

/// VANILLA: NumberRangeInput.RangeInfo#initialSliderValue.
pub fn range_initial_slider(start: f32, end: f32, initial: Option<f32>) -> f32 {
    let v = initial.unwrap_or((start + end) / 2.0);
    if start == end {
        0.5
    } else {
        (v - start) / (end - start)
    }
}

/// VANILLA: SliderImpl#valueToString (целое без «.0»).
pub fn range_str(v: f32) -> String {
    if (v as i32) as f32 == v {
        (v as i32).to_string()
    } else {
        v.to_string()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn s(k: &str, v: &str) -> (String, Tag) {
        (k.into(), Tag::String(v.into()))
    }

    #[test]
    fn parse_multi_action() {
        let btn = |l: &str| {
            Tag::Compound(vec![
                s("label", l),
                ("width".into(), Tag::Int(120)),
                (
                    "action".into(),
                    Tag::Compound(vec![s("type", "minecraft:run_command"), s("command", "/x")]),
                ),
            ])
        };
        let t = Tag::Compound(vec![
            s("type", "minecraft:multi_action"),
            s("title", "Hi"),
            (
                "body".into(),
                Tag::Compound(vec![
                    s("type", "minecraft:plain_message"),
                    s("contents", "msg"),
                ]),
            ),
            ("actions".into(), Tag::List(vec![btn("a"), btn("b")])),
            ("columns".into(), Tag::Int(3)),
            ("exit_action".into(), btn("exit")),
        ]);
        let d = parse(&t).unwrap();
        assert_eq!(d.body.len(), 1);
        assert!(d.can_close_with_escape);
        let Kind::List {
            buttons,
            exit,
            columns,
        } = &d.kind
        else {
            panic!()
        };
        assert_eq!((buttons.len(), *columns), (2, 3));
        assert!(exit.is_some() && d.on_cancel.is_some());
        assert_eq!(buttons[0].width, 120);
    }

    #[test]
    fn template_and_range() {
        let r = instantiate("say $(a) and $(b)!", &|k| {
            if k == "a" {
                "1".into()
            } else {
                String::new()
            }
        });
        assert_eq!(r, "say 1 and !");
        assert_eq!(escape_without_quotes("a\"b\\\n"), "a\\\"b\\\\\\n");
        assert_eq!(range_value(0.0, 10.0, None, Some(2.0), 0.33), 3.0);
        assert_eq!(range_str(4.0), "4");
        assert_eq!(range_str(2.5), "2.5");
    }
}
