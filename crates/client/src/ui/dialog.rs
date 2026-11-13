//! Экран диалога сервера (Dialog API).
//! VANILLA: DialogScreen / SimpleDialogScreen / ButtonListDialogScreen / DialogControlSet /
//! WaitingForResponseScreen / DialogScreen.WarningScreen (26.3).
use super::{edit_apply, wrap_n, Action, Frame, State, UiKey, WHITE};
use crate::chat;
use crate::dialog::{self, Act, After, Body, Btn, Click, Control, Dialog, Kind};
use crate::editbox::EditBox;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::text::Component;

/// Значение элемента ввода.
pub enum Val {
    Text(EditBox),
    Bool(bool),
    Opt(usize),
    Range(f32),
}

pub struct Ds {
    dlg: Dialog,
    vals: Vec<Val>,
    scroll: f32,
    focus: usize,
    /// Нажатие кнопки по индексу (MCP): сначала сетка, потом подвал.
    press: Option<usize>,
    prev: Option<Box<Scr>>,
}

/// Экран диалога: сам диалог, «ждём ответ сервера» или предупреждение о кастомных экранах.
pub enum Scr {
    Dialog(Box<Ds>),
    Wait(Box<Ds>, u64),
    Warn(Box<Scr>),
}

/// Исходящее действие (забирает main).
pub enum Out {
    Command(String),
    Custom(String, Option<Tag>),
}

impl Ds {
    fn new(dlg: Dialog, prev: Option<Box<Scr>>) -> Self {
        let vals = dlg
            .inputs
            .iter()
            .map(|i| match &i.control {
                Control::Text {
                    initial, max_len, ..
                } => {
                    let mut e = EditBox::new(*max_len);
                    e.set(initial);
                    Val::Text(e)
                }
                Control::Bool { initial, .. } => Val::Bool(*initial),
                Control::Option { entries, .. } => {
                    Val::Opt(entries.iter().position(|e| e.2).unwrap_or(0))
                }
                Control::Range {
                    start,
                    end,
                    initial,
                    ..
                } => Val::Range(dialog::range_initial_slider(*start, *end, *initial)),
            })
            .collect();
        Self {
            dlg,
            vals,
            scroll: 0.0,
            focus: 0,
            press: None,
            prev,
        }
    }

    /// Значение ввода как подстановка `$(key)`.
    /// VANILLA: Input#asTemplateSubstitution (text — escapeWithoutQuotes).
    fn arg(&self, key: &str) -> String {
        for (i, v) in self.dlg.inputs.iter().zip(&self.vals) {
            if i.key != key {
                continue;
            }
            return match (&i.control, v) {
                (_, Val::Text(e)) => dialog::escape_without_quotes(&e.text),
                (
                    Control::Bool {
                        on_true, on_false, ..
                    },
                    Val::Bool(b),
                ) => {
                    if *b {
                        on_true.clone()
                    } else {
                        on_false.clone()
                    }
                }
                (Control::Option { entries, .. }, Val::Opt(n)) => {
                    entries.get(*n).map(|e| e.0.clone()).unwrap_or_default()
                }
                (
                    Control::Range {
                        start,
                        end,
                        initial,
                        step,
                        ..
                    },
                    Val::Range(s),
                ) => dialog::range_str(dialog::range_value(*start, *end, *initial, *step, *s)),
                _ => String::new(),
            };
        }
        String::new()
    }

    /// Значение ввода как NBT для `dynamic/custom` (String / Byte / String / Float).
    fn tag(&self, n: usize) -> Tag {
        let i = &self.dlg.inputs[n];
        match (&i.control, &self.vals[n]) {
            (_, Val::Text(e)) => Tag::String(e.text.clone()),
            (Control::Bool { .. }, Val::Bool(b)) => Tag::Byte(i8::from(*b)),
            (Control::Option { entries, .. }, Val::Opt(k)) => {
                Tag::String(entries.get(*k).map(|e| e.0.clone()).unwrap_or_default())
            }
            (
                Control::Range {
                    start,
                    end,
                    initial,
                    step,
                    ..
                },
                Val::Range(s),
            ) => Tag::Float(dialog::range_value(*start, *end, *initial, *step, *s)),
            _ => Tag::Byte(0),
        }
    }
}

/// Куда перейти после кадра.
enum Go {
    Stay,
    /// Закрыть текущий (вернуться на предыдущий экран или в игру).
    Back,
    /// Показать диалог из действия.
    Show(Box<Dialog>),
    Wait,
    Warn,
    Disconnect,
}

pub fn open(d: Dialog) -> Scr {
    Scr::Dialog(Box::new(Ds::new(d, None)))
}

/// Действие кнопки. Возвращает диалог, который надо показать (ShowDialog).
fn run(ds: &Ds, act: &Act, out: &mut Vec<Out>, clip: &mut Option<String>) -> Option<Box<Dialog>> {
    let cmd = |s: &str| Out::Command(s.strip_prefix('/').unwrap_or(s).to_owned());
    match act {
        Act::Static(c) => match c {
            Click::RunCommand(s) => out.push(cmd(s)),
            Click::Custom(id, p) => out.push(Out::Custom(id.clone(), p.clone())),
            Click::CopyToClipboard(s) => *clip = Some(s.clone()),
            Click::ShowDialog(d) => return Some(d.clone()),
            Click::OpenUrl(u) => tracing::info!("диалог: open_url {u} (не поддержано)"),
            Click::SuggestCommand(s) => {
                tracing::info!("диалог: suggest_command {s} (не поддержано)")
            }
            Click::ChangePage => {}
        },
        Act::Template(t) => out.push(cmd(&dialog::instantiate(t, &|k| ds.arg(k)))),
        Act::Custom(id, add) => {
            let mut fields = match add {
                Some(Tag::Compound(c)) => c.clone(),
                _ => Vec::new(),
            };
            for (n, i) in ds.dlg.inputs.iter().enumerate() {
                fields.retain(|(k, _)| *k != i.key);
                fields.push((i.key.clone(), ds.tag(n)));
            }
            out.push(Out::Custom(id.clone(), Some(Tag::Compound(fields))));
        }
    }
    None
}

fn text(f: &Frame, c: &Component) -> String {
    chat::spans_to_legacy(&chat::flatten(c, f.lang, true), WHITE)
}

pub(super) fn screen(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    wheel: f32,
    now_ms: u64,
    action: &mut Action,
) {
    let Some(mut scr) = st.dlg.take() else {
        *action = Action::CloseDialog;
        return;
    };
    let esc = keys.iter().any(|k| matches!(k, UiKey::Escape));
    let go = match &mut scr {
        Scr::Dialog(ds) => view(
            f,
            ds,
            keys,
            typed,
            wheel,
            now_ms,
            esc,
            &mut st.clip_out,
            &mut st.dlg_out,
        ),
        Scr::Wait(_, t0) => wait_view(f, *t0, now_ms, esc),
        Scr::Warn(_) => warn_view(f, esc),
    };
    st.dlg = match go {
        Go::Stay => Some(scr),
        Go::Back => match scr {
            Scr::Dialog(ds) | Scr::Wait(ds, _) => ds.prev.map(|b| *b),
            Scr::Warn(inner) => Some(*inner),
        },
        Go::Show(d) => {
            let (prev, after) = match &mut scr {
                Scr::Dialog(ds) => (ds.prev.take(), ds.dlg.after),
                _ => (None, After::Close),
            };
            let prev = if after == After::Close {
                prev
            } else {
                // Диалог остаётся под новым (и вернётся при закрытии).
                if let Scr::Dialog(ds) = &mut scr {
                    ds.prev = prev;
                }
                Some(Box::new(scr))
            };
            Some(Scr::Dialog(Box::new(Ds::new(*d, prev))))
        }
        Go::Wait => match scr {
            Scr::Dialog(ds) => Some(Scr::Wait(ds, now_ms)),
            o => Some(o),
        },
        Go::Warn => Some(Scr::Warn(Box::new(scr))),
        Go::Disconnect => {
            *action = Action::Disconnect;
            None
        }
    };
    if st.dlg.is_none() && !matches!(action, Action::Disconnect) {
        *action = Action::CloseDialog;
    }
}

enum El {
    Msg(Vec<String>),
    Input(usize),
    Grid,
}

#[allow(clippy::too_many_arguments)]
fn view(
    f: &mut Frame,
    ds: &mut Ds,
    keys: &[UiKey],
    typed: &str,
    wheel: f32,
    now_ms: u64,
    esc: bool,
    clip: &mut Option<String>,
    out: &mut Vec<Out>,
) -> Go {
    let (w, h) = (f.w, f.h);
    let cx = w / 2.0;
    let mut go = Go::Stay;
    let mut tip: Option<String> = None;
    let mut clicked: Option<Option<Act>> = None;

    // Esc: onCancel + закрыть.
    if esc && ds.dlg.can_close_with_escape {
        if let Some(a) = ds.dlg.on_cancel.clone() {
            run(ds, &a, out, clip);
        }
        return Go::Back;
    }

    // Фокус и клавиши для текстовых полей.
    let texts: Vec<usize> = ds
        .vals
        .iter()
        .enumerate()
        .filter(|(_, v)| matches!(v, Val::Text(_)))
        .map(|(i, _)| i)
        .collect();
    if !texts.is_empty() {
        let pos = texts.iter().position(|&i| i == ds.focus).unwrap_or(0);
        for k in keys {
            match k {
                UiKey::Tab => ds.focus = texts[(pos + 1) % texts.len()],
                UiKey::BackTab => ds.focus = texts[(pos + texts.len() - 1) % texts.len()],
                _ => {}
            }
        }
        if !texts.contains(&ds.focus) {
            ds.focus = texts[0];
        }
        let fi = ds.focus;
        if let Val::Text(e) = &mut ds.vals[fi] {
            edit_apply(e, keys, typed, clip);
        }
    }
    let caret = (now_ms / 500) % 2 == 0;

    // Шапка: заголовок + кнопка-предупреждение.
    let title = text(f, &ds.dlg.title);
    let tw = f.t.text_width(&title);
    let x0 = ((w - (tw + 30.0)) / 2.0).floor();
    f.t.draw(&title, x0, 11.0, WHITE, true);
    let (wx, wy) = (x0 + tw + 10.0, 6.0);
    let hov = f.hover(wx, wy, 20.0, 20.0);
    let sprite = if hov { "dlg_warn_hl" } else { "dlg_warn" };
    f.p.blit(sprite, wx, wy, 20.0, 20.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
    if hov {
        crate::cursor::request(crate::cursor::Kind::PointingHand);
        tip = Some(f.lang.tr("menu.custom_screen_info.tooltip"));
        if f.click {
            f.click = false;
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
            go = Go::Warn;
        }
    }

    // Подвал: кнопки основного ряда (Simple) или exit (List).
    let footer_btns: Vec<Btn> = match &ds.dlg.kind {
        Kind::Simple(b) => b.clone(),
        Kind::List { exit, .. } => exit.iter().cloned().collect(),
    };
    let footer = if footer_btns.is_empty() { 5.0 } else { 33.0 };
    let (top, bottom) = (33.0, h - footer);
    if !footer_btns.is_empty() {
        let total: f32 = footer_btns.iter().map(|b| b.width as f32).sum::<f32>()
            + 8.0 * (footer_btns.len() as f32 - 1.0);
        let mut x = cx - total / 2.0;
        for b in &footer_btns {
            let bw = b.width as f32;
            let lab = text(f, &b.label);
            if f.hover(x, h - 27.0, bw, 20.0) {
                if let Some(t) = &b.tooltip {
                    tip = Some(text(f, t));
                }
            }
            if f.button(x, h - 27.0, bw, 20.0, &lab, true) {
                clicked = Some(b.action.clone());
            }
            x += bw + 8.0;
        }
    }

    // Колонка тела.
    let mut els: Vec<(El, f32)> = Vec::new();
    for b in &ds.dlg.body {
        let (c, mw) = match b {
            Body::Text(c, mw) => (c, *mw),
            Body::Item(Some((c, mw))) => (c, *mw),
            Body::Item(None) => continue,
        };
        let lines = wrap_n(f.t, &text(f, c), mw as f32, usize::MAX);
        let hh = 9.0 * lines.len() as f32;
        els.push((El::Msg(lines), hh));
    }
    for (n, i) in ds.dlg.inputs.iter().enumerate() {
        let hh = match &i.control {
            Control::Text {
                label_visible,
                multiline,
                ..
            } => {
                let lab = if *label_visible { 13.0 } else { 0.0 };
                let field = multiline.map_or(20.0, |(lines, height)| {
                    height.map_or_else(
                        || lines.map_or(40.0, |l| l as f32 * 9.0 + 8.0),
                        |v| v as f32,
                    )
                });
                lab + field
            }
            Control::Bool { .. } => 17.0,
            _ => 20.0,
        };
        els.push((El::Input(n), hh));
    }
    let (grid, cols) = match &ds.dlg.kind {
        Kind::List {
            buttons, columns, ..
        } if !buttons.is_empty() => (buttons.len(), *columns),
        _ => (0, 1),
    };
    if grid > 0 {
        let rows = grid.div_ceil(cols) as f32;
        els.push((El::Grid, rows * 20.0 + (rows - 1.0) * 2.0));
    }
    let total = els.iter().map(|e| e.1).sum::<f32>() + 10.0 * (els.len() as f32 - 1.0).max(0.0);
    let avail = bottom - top;
    let mut y = if total <= avail {
        ds.scroll = 0.0;
        // VANILLA: HeaderAndFooterLayout — contentY = min(header + 30, h - footer - contentH).
        (top + 30.0).min(bottom - total)
    } else {
        ds.scroll = (ds.scroll - wheel * 12.0).clamp(0.0, total - avail);
        top - ds.scroll
    };
    let mut grid_click: Option<usize> = None;
    for (el, hh) in &els {
        let visible = y >= top && y + hh <= bottom;
        if visible {
            match el {
                El::Msg(lines) => {
                    for (i, l) in lines.iter().enumerate() {
                        let lw = f.t.text_width(l);
                        f.t.draw(l, cx - lw / 2.0, y + 9.0 * i as f32, WHITE, true);
                    }
                }
                El::Input(n) => input(f, ds, *n, cx, y, caret),
                El::Grid => {
                    if let Kind::List { buttons, .. } = &ds.dlg.kind {
                        grid_click = grid_buttons(f, buttons, cols, cx, y, &mut tip);
                    }
                }
            }
        }
        y += hh + 10.0;
    }
    if let (Some(n), Kind::List { buttons, .. }) = (grid_click, &ds.dlg.kind) {
        clicked = Some(buttons[n].action.clone());
    }
    if let Some(n) = ds.press.take() {
        let g = match &ds.dlg.kind {
            Kind::List { buttons, .. } => buttons.as_slice(),
            Kind::Simple(_) => &[],
        };
        clicked = if n < g.len() {
            Some(g[n].action.clone())
        } else {
            footer_btns.get(n - g.len()).map(|b| b.action.clone())
        };
    }

    if let Some(lines) = tip {
        let ls: Vec<(String, u32)> = lines.split('\n').map(|l| (l.to_owned(), WHITE)).collect();
        crate::tooltip::draw_colored(f.t, &ls, f.mx, f.my, w, h);
    }

    if let Some(act) = clicked {
        let shown = act.as_ref().and_then(|a| run(ds, a, out, clip));
        return match shown {
            Some(d) => Go::Show(d),
            None => match ds.dlg.after {
                After::Close => Go::Back,
                After::None => go,
                After::WaitForResponse => Go::Wait,
            },
        };
    }
    go
}

/// Один элемент ввода. VANILLA: InputControlHandlers.
fn input(f: &mut Frame, ds: &mut Ds, n: usize, cx: f32, y: f32, caret: bool) {
    let focus = ds.focus;
    match (&ds.dlg.inputs[n].control, &mut ds.vals[n]) {
        (
            Control::Text {
                width,
                label,
                label_visible,
                multiline,
                ..
            },
            Val::Text(e),
        ) => {
            let wd = *width as f32;
            let x = cx - wd / 2.0;
            let mut fy = y;
            if *label_visible {
                let l = chat::spans_to_legacy(&chat::flatten(label, f.lang, true), WHITE);
                f.t.draw(&l, x, y, WHITE, true);
                fy += 13.0;
            }
            let hh = multiline.map_or(20.0, |(lines, height)| {
                height.map_or_else(
                    || lines.map_or(40.0, |l| l as f32 * 9.0 + 8.0),
                    |v| v as f32,
                )
            });
            if f.edit_box(x, fy, wd, hh, e, focus == n, caret) {
                ds.focus = n;
            }
        }
        (Control::Bool { label, .. }, Val::Bool(b)) => {
            let l = chat::spans_to_legacy(&chat::flatten(label, f.lang, true), WHITE);
            let lw = f.t.text_width(&l);
            let x = (cx - (17.0 + 4.0 + lw) / 2.0).floor();
            let hov = f.hover(x, y, 17.0 + 4.0 + lw, 17.0);
            let sprite = match (*b, hov) {
                (false, false) => "dlg_check",
                (false, true) => "dlg_check_hl",
                (true, false) => "dlg_check_on",
                (true, true) => "dlg_check_on_hl",
            };
            f.p.blit(sprite, x, y, 17.0, 17.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
            f.t.draw(&l, x + 21.0, y + 4.5, WHITE, true);
            if hov {
                crate::cursor::request(crate::cursor::Kind::PointingHand);
                if f.click {
                    f.click = false;
                    crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
                    *b = !*b;
                }
            }
        }
        (
            Control::Option {
                width,
                entries,
                label,
                label_visible,
            },
            Val::Opt(k),
        ) => {
            let wd = *width as f32;
            let Some(e) = entries.get(*k) else { return };
            let value = e.1.as_ref().map_or_else(
                || e.0.clone(),
                |c| chat::spans_to_legacy(&chat::flatten(c, f.lang, true), WHITE),
            );
            let lab = if *label_visible {
                let l = chat::spans_to_legacy(&chat::flatten(label, f.lang, true), WHITE);
                f.lang.fmt("options.generic_value", &[&l, &value])
            } else {
                value
            };
            if f.button(cx - wd / 2.0, y, wd, 20.0, &lab, true) {
                *k = (*k + 1) % entries.len();
            }
        }
        (
            Control::Range {
                width,
                label,
                label_format,
                start,
                end,
                initial,
                step,
            },
            Val::Range(s),
        ) => {
            let wd = *width as f32;
            let l = chat::spans_to_legacy(&chat::flatten(label, f.lang, true), WHITE);
            let v = dialog::range_str(dialog::range_value(*start, *end, *initial, *step, *s));
            let lab = f.lang.fmt(label_format, &[&l, &v]);
            if f.slider(cx - wd / 2.0, y, wd, 20.0, &lab, s) {
                // Привязка к шагу: показываем значение, а ползунок держим на нём.
                let val = dialog::range_value(*start, *end, *initial, *step, *s);
                if step.is_some() && end != start {
                    *s = ((val - *start) / (*end - *start)).clamp(0.0, 1.0);
                }
            }
        }
        _ => {}
    }
}

/// Сетка кнопок: колонки с зазором 2, неполная последняя строка центрируется.
/// Возвращает индекс нажатой кнопки.
fn grid_buttons(
    f: &mut Frame,
    buttons: &[Btn],
    cols: usize,
    cx: f32,
    y: f32,
    tip: &mut Option<String>,
) -> Option<usize> {
    let rows = buttons.len().div_ceil(cols);
    let mut colw = vec![0.0_f32; cols];
    for (i, b) in buttons.iter().enumerate() {
        let c = i % cols;
        colw[c] = colw[c].max(b.width as f32);
    }
    let gw: f32 = colw.iter().sum::<f32>() + 2.0 * (cols as f32 - 1.0);
    let mut hit = None;
    for r in 0..rows {
        let row: Vec<usize> = (r * cols..buttons.len().min((r + 1) * cols)).collect();
        let ry = y + 22.0 * r as f32;
        let full = row.len() == cols;
        let rw: f32 = if full {
            gw
        } else {
            row.iter().map(|&i| buttons[i].width as f32).sum::<f32>()
                + 2.0 * (row.len() as f32 - 1.0)
        };
        let mut x = cx - rw / 2.0;
        for (c, &i) in row.iter().enumerate() {
            let b = &buttons[i];
            let cell = if full { colw[c] } else { b.width as f32 };
            let bw = b.width as f32;
            let bx = x + (cell - bw) / 2.0;
            let lab = chat::spans_to_legacy(&chat::flatten(&b.label, f.lang, true), WHITE);
            if f.hover(bx, ry, bw, 20.0) {
                if let Some(t) = &b.tooltip {
                    *tip = Some(chat::spans_to_legacy(
                        &chat::flatten(t, f.lang, true),
                        WHITE,
                    ));
                }
            }
            if f.button(bx, ry, bw, 20.0, &lab, true) {
                hit = Some(i);
            }
            x += cell + 2.0;
        }
    }
    hit
}

/// VANILLA: WaitingForResponseScreen — кнопка Back видна с 1 с, активна с 5 с.
fn wait_view(f: &mut Frame, t0: u64, now_ms: u64, esc: bool) -> Go {
    let secs = now_ms.saturating_sub(t0) / 1000;
    let top = ((f.h - 53.0) / 2.0).floor();
    f.center("gui.waitingForResponse.title", top + 12.0, WHITE);
    let active = secs >= 5;
    if secs >= 1 {
        let lab = if active {
            f.lang.tr("gui.back")
        } else {
            f.lang.fmt(
                "gui.waitingForResponse.button.inactive",
                &[&(5 - secs).to_string()],
            )
        };
        if f.button(f.w / 2.0 - 100.0, top + 33.0, 200.0, 20.0, &lab, active) {
            return Go::Back;
        }
    }
    if esc && active {
        return Go::Back;
    }
    Go::Stay
}

/// VANILLA: DialogScreen.WarningScreen (ConfirmScreen: Disconnect / Back).
fn warn_view(f: &mut Frame, esc: bool) -> Go {
    f.center("menu.custom_screen_info.title", 70.0, WHITE);
    let msg = f.lang.tr("menu.custom_screen_info.contents");
    let lines = wrap_n(f.t, &msg, f.w - 50.0, usize::MAX);
    for (i, l) in lines.iter().enumerate() {
        f.center_raw(l, 90.0 + 9.0 * i as f32, WHITE);
    }
    let by = (90.0 + 9.0 * lines.len() as f32 + 12.0).min(f.h - 27.0);
    let x = f.w / 2.0 - 155.0;
    if f.button(x, by, 150.0, 20.0, &f.lang.tr("menu.disconnect"), true) {
        return Go::Disconnect;
    }
    if f.button(x + 160.0, by, 150.0, 20.0, &f.lang.tr("gui.back"), true) || esc {
        return Go::Back;
    }
    Go::Stay
}

fn plain(c: &Component, lang: &super::Lang) -> String {
    chat::flatten(c, lang, false)
        .iter()
        .map(|s| s.text.as_str())
        .collect()
}

/// Для MCP: заголовок, вид экрана и подписи кнопок (сначала сетка, потом подвал).
pub fn info(scr: &Scr, lang: &super::Lang) -> (String, &'static str, Vec<String>) {
    match scr {
        Scr::Dialog(ds) => {
            let mut l = Vec::new();
            match &ds.dlg.kind {
                Kind::Simple(b) => l.extend(b.iter().map(|b| plain(&b.label, lang))),
                Kind::List { buttons, exit, .. } => {
                    l.extend(buttons.iter().map(|b| plain(&b.label, lang)));
                    l.extend(exit.iter().map(|b| plain(&b.label, lang)));
                }
            }
            (plain(&ds.dlg.title, lang), "dialog", l)
        }
        Scr::Wait(ds, _) => (plain(&ds.dlg.title, lang), "wait", Vec::new()),
        Scr::Warn(_) => (lang.tr("menu.custom_screen_info.title"), "warn", Vec::new()),
    }
}

/// Для MCP: нажать кнопку по индексу в следующем кадре.
pub fn press(scr: &mut Scr, n: usize) {
    if let Scr::Dialog(ds) = scr {
        ds.press = Some(n);
    }
}
