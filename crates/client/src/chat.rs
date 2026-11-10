//! Чат: история, перенос строк, рендер HUD/экрана, прокрутка, клики.
//! VANILLA: ChatComponent, ChatScreen, GuiMessage, ComponentRenderUtils#wrapComponents.
use crate::editbox::{EditBox, History};
use crate::ui::lang::Lang;
use ezzmc_protocol::text::{ClickEvent, Component, Content, Style};
use ezzmc_render::text::{Fmt, TextRenderer};
use std::collections::VecDeque;

const COLORS: [u32; 16] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xFFAA00, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF,
];

/// Индикатор слева (GuiMessageTag). VANILLA: система/not_secure — 0xD0D0D0.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tag {
    System,
    NotSecure,
}

impl Tag {
    fn color(self) -> u32 {
        0xD0D0D0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub text: String,
    pub color: u32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub obf: bool,
    pub click: Option<ClickEvent>,
    pub insertion: Option<String>,
}

impl Span {
    fn plain(text: String) -> Self {
        Self {
            text,
            color: 0xFFFFFF,
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            obf: false,
            click: None,
            insertion: None,
        }
    }

    fn fmt(&self, line_start: bool) -> Fmt {
        Fmt {
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            strike: self.strike,
            obfuscated: self.obf,
            line_start,
        }
    }
}

fn merge(p: &Span, s: &Style) -> Span {
    Span {
        text: String::new(),
        color: s.color.unwrap_or(p.color),
        bold: s.bold.unwrap_or(p.bold),
        italic: s.italic.unwrap_or(p.italic),
        underline: s.underlined.unwrap_or(p.underline),
        strike: s.strikethrough.unwrap_or(p.strike),
        obf: s.obfuscated.unwrap_or(p.obf),
        click: s.click.clone().or_else(|| p.click.clone()),
        insertion: s.insertion.clone().or_else(|| p.insertion.clone()),
    }
}

/// VANILLA: StringDecomposer#iterateFormatted — коды `§` внутри текста.
fn push_text(out: &mut Vec<Span>, text: &str, base: &Span) {
    let mut st = base.clone();
    let mut buf = String::new();
    let flush = |buf: &mut String, st: &Span, out: &mut Vec<Span>| {
        if !buf.is_empty() {
            out.push(Span {
                text: std::mem::take(buf),
                ..st.clone()
            });
        }
    };
    let mut it = text.chars();
    while let Some(c) = it.next() {
        if c != '§' {
            buf.push(c);
            continue;
        }
        let Some(code) = it.next() else { break };
        let code = code.to_ascii_lowercase();
        flush(&mut buf, &st, out);
        if let Some(i) = code.to_digit(16) {
            st = Span {
                color: COLORS[i as usize],
                bold: false,
                italic: false,
                underline: false,
                strike: false,
                obf: false,
                ..st
            };
        } else {
            match code {
                'k' => st.obf = true,
                'l' => st.bold = true,
                'm' => st.strike = true,
                'n' => st.underline = true,
                'o' => st.italic = true,
                'r' => st = base.clone(),
                _ => {}
            }
        }
    }
    flush(&mut buf, &st, out);
}

fn walk(c: &Component, lang: &Lang, parent: &Span, out: &mut Vec<Span>) {
    let st = merge(parent, &c.style);
    match &c.content {
        Content::Text(s) => push_text(out, s, &st),
        Content::Translatable {
            key,
            fallback,
            args,
        } => {
            let tpl = lang
                .get(key)
                .map(str::to_owned)
                .or_else(|| fallback.clone())
                .unwrap_or_else(|| key.clone());
            translate(&tpl, args, lang, &st, out);
        }
        Content::Score { name, .. } => push_text(out, name, &st),
        Content::Selector { pattern, .. } => push_text(out, pattern, &st),
        Content::Keybind(k) => push_text(out, k, &st),
        Content::Nbt { path, .. } => push_text(out, path, &st),
    }
    for s in &c.siblings {
        walk(s, lang, &st, out);
    }
}

/// VANILLA: TranslatableContents#decomposeTemplate — `%s`, `%n$s`, `%%`.
fn translate(tpl: &str, args: &[Component], lang: &Lang, st: &Span, out: &mut Vec<Span>) {
    let b: Vec<char> = tpl.chars().collect();
    let (mut i, mut next) = (0, 0usize);
    let mut lit = String::new();
    while i < b.len() {
        if b[i] != '%' {
            lit.push(b[i]);
            i += 1;
            continue;
        }
        if b.get(i + 1) == Some(&'%') {
            lit.push('%');
            i += 2;
            continue;
        }
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
            push_text(out, &std::mem::take(&mut lit), st);
            if let Some(a) = idx.and_then(|n| args.get(n)) {
                walk(a, lang, st, out);
            }
            i = j + 1;
        } else {
            lit.push('%');
            i += 1;
        }
    }
    push_text(out, &lit, st);
}

/// Компонент → плоские спаны (цвет по умолчанию белый).
pub fn flatten(c: &Component, lang: &Lang, colors: bool) -> Vec<Span> {
    let mut out = Vec::new();
    walk(c, lang, &Span::plain(String::new()), &mut out);
    if !colors {
        // VANILLA: chatColors=false — без цвета и форматирования.
        for s in &mut out {
            *s = Span {
                text: std::mem::take(&mut s.text),
                click: s.click.take(),
                insertion: s.insertion.take(),
                ..Span::plain(String::new())
            };
        }
    }
    out
}

struct Msg {
    added: u64,
    spans: Vec<Span>,
    tag: Tag,
    wrapped: bool,
}

struct Seg {
    span: Span,
    x: f32,
}

struct Line {
    added: u64,
    segs: Vec<Seg>,
    tag: Tag,
}

/// Настройки чата (Options).
#[derive(Clone, Copy)]
pub struct Cfg {
    pub opacity: f32,
    pub bg: f32,
    pub scale: f32,
    pub width: f32,
    pub h_focused: f32,
    pub h_unfocused: f32,
    pub spacing: f32,
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            bg: 0.5,
            scale: 1.0,
            width: 1.0,
            h_focused: 1.0,
            h_unfocused: 0.4375,
            spacing: 0.0,
        }
    }
}

impl Cfg {
    fn width_px(&self) -> i32 {
        (self.width * 280.0 + 40.0).floor() as i32
    }

    fn height_px(&self, open: bool) -> i32 {
        let p = if open {
            self.h_focused
        } else {
            self.h_unfocused
        };
        (p * 160.0 + 20.0).floor() as i32
    }

    fn entry_h(&self) -> i32 {
        (9.0 * (self.spacing + 1.0)) as i32
    }

    fn per_page(&self, open: bool) -> usize {
        (self.height_px(open) / self.entry_h().max(1)).max(1) as usize
    }
}

/// VANILLA: CommandSuggestions (+ SuggestionsList) в режиме ChatScreen: подсказки приходят от сервера.
#[derive(Default)]
pub struct Sugg {
    next_id: i32,
    pending: i32,
    /// Поле при последней проверке (смена = onEdit).
    seen: String,
    /// Начальный текст (prefill): подсказки не показываем, пока его не изменили.
    initial: String,
    keep: bool,
    /// VANILLA: Options#autoSuggestions — false: подсказки только по Tab.
    pub auto: bool,
    /// Tab при выключенном auto: следующая проверка поля шлёт запрос.
    pub force: bool,
    pub items: Vec<String>,
    start: usize,
    len: usize,
    pub shown: bool,
    cur: usize,
    offset: usize,
    original: String,
    tab_cycles: bool,
    /// Серая подсказка-хвост после курсора.
    ghost: Option<String>,
    rect: (f32, f32, f32, f32),
}

const SUGG_LIMIT: usize = 10;

fn sub_chars(s: &str, a: usize, b: usize) -> String {
    s.chars().skip(a).take(b.saturating_sub(a)).collect()
}

impl Sugg {
    pub fn reset(&mut self, text: &str) {
        *self = Self {
            next_id: self.next_id,
            seen: text.to_owned(),
            initial: text.to_owned(),
            ..Self::default()
        };
    }

    /// VANILLA: ChatScreen#onEdit → CommandSuggestions#updateCommandInfo. Возвращает запрос (id, текст до курсора).
    pub fn seen_reset(&mut self) {
        self.seen.clear();
    }

    pub fn on_edit(&mut self, edit: &EditBox) -> Option<(i32, String)> {
        if edit.text == self.seen {
            return None;
        }
        self.seen = edit.text.clone();
        if self.keep {
            return None;
        }
        let forced = std::mem::take(&mut self.force);
        self.shown = false;
        self.ghost = None;
        self.pending = 0;
        if !self.auto && !forced {
            return None;
        }
        if edit.text == self.initial || !edit.text.starts_with('/') || edit.cursor < 1 {
            return None;
        }
        self.next_id += 1;
        self.pending = self.next_id;
        Some((self.next_id, sub_chars(&edit.text, 0, edit.cursor)))
    }

    /// Ответ сервера. VANILLA: sortSuggestions (сначала начинающиеся с последнего слова) + showSuggestions.
    pub fn on_reply(&mut self, id: i32, start: i32, len: i32, items: Vec<String>, edit: &EditBox) {
        if id != self.pending || items.is_empty() {
            return;
        }
        let before: String = sub_chars(&edit.text, 0, edit.cursor);
        let word = before
            .rfind(' ')
            .map_or(0, |i| before[..i].chars().count() + 1);
        let last = sub_chars(&before, word, usize::MAX).to_lowercase();
        let (mut a, mut b) = (Vec::new(), Vec::new());
        for it in items {
            if it.starts_with(&last) || it.starts_with(&format!("minecraft:{last}")) {
                a.push(it);
            } else {
                b.push(it);
            }
        }
        a.extend(b);
        self.items = a;
        self.start = start.max(0) as usize;
        self.len = len.max(0) as usize;
        self.original = edit.text.clone();
        self.cur = 0;
        self.offset = 0;
        self.tab_cycles = false;
        self.shown = true;
        self.select(0, edit);
    }

    fn apply(&self, i: usize) -> String {
        let n = self.original.chars().count();
        let end = (self.start + self.len).min(n);
        let tail = sub_chars(&self.original, end, n);
        format!(
            "{}{}{}",
            sub_chars(&self.original, 0, self.start),
            self.items[i],
            tail
        )
    }

    fn select(&mut self, i: isize, edit: &EditBox) {
        let n = self.items.len() as isize;
        self.cur = (i.rem_euclid(n)) as usize;
        let full = self.apply(self.cur);
        self.ghost = full.strip_prefix(&edit.text).map(str::to_owned);
    }

    pub fn cycle(&mut self, d: isize, edit: &EditBox) {
        self.select(self.cur as isize + d, edit);
        let maxoff = self.items.len().saturating_sub(SUGG_LIMIT);
        if self.cur < self.offset {
            self.offset = self.cur.min(maxoff);
        } else if self.cur > self.offset + SUGG_LIMIT - 1 {
            self.offset = (self.cur + 1).saturating_sub(SUGG_LIMIT).min(maxoff);
        }
    }

    /// VANILLA: SuggestionsList#useSuggestion.
    pub fn use_current(&mut self, edit: &mut EditBox) {
        let text = self.apply(self.cur);
        let end = self.start + self.items[self.cur].chars().count();
        self.keep = true;
        edit.set(&text);
        edit.cursor = end.min(edit.len());
        edit.sel = edit.cursor;
        self.seen = edit.text.clone();
        self.select(self.cur as isize, edit);
        self.keep = false;
        self.tab_cycles = true;
    }

    /// VANILLA: SuggestionsList#keyPressed (Up/Down/Tab/Esc). true = клавиша съедена.
    pub fn key(&mut self, k: SuggKey, shift: bool, edit: &mut EditBox) -> bool {
        if !self.shown {
            return false;
        }
        match k {
            SuggKey::Up => {
                self.cycle(-1, edit);
                self.tab_cycles = false;
            }
            SuggKey::Down => {
                self.cycle(1, edit);
                self.tab_cycles = false;
            }
            SuggKey::Tab => {
                if self.tab_cycles {
                    self.cycle(if shift { -1 } else { 1 }, edit);
                }
                self.use_current(edit);
            }
            SuggKey::Esc => {
                self.shown = false;
                self.ghost = None;
            }
        }
        true
    }

    /// ЛКМ по списку. VANILLA: SuggestionsList#mouseClicked.
    pub fn click(&mut self, x: f32, y: f32, edit: &mut EditBox) -> bool {
        let (rx, ry, rw, rh) = self.rect;
        if !self.shown || x < rx || x >= rx + rw || y < ry || y >= ry + rh {
            return false;
        }
        let line = ((y - ry) / 12.0) as usize + self.offset;
        if line < self.items.len() {
            self.cur = line;
            self.use_current(edit);
        }
        true
    }

    pub fn draw(&mut self, t: &mut TextRenderer, gw: f32, gh: f32, edit: &EditBox) {
        if let (Some(g), true) = (&self.ghost, edit.cursor == edit.len()) {
            let x = 4.0 + t.text_width(&edit.text);
            t.draw(g, x, gh - 12.0, 0x808080, true);
        }
        if !self.shown {
            return;
        }
        let limit = self.items.len().min(SUGG_LIMIT);
        let maxw = self
            .items
            .iter()
            .map(|s| t.text_width(s))
            .fold(0.0f32, f32::max)
            .floor();
        let sx = 4.0 + t.text_width(&sub_chars(&edit.text, 0, self.start)).floor();
        let x = sx.clamp(0.0, (4.0 + (gw - 4.0) - maxw).max(0.0));
        let (rx, ry, rw) = (x - 1.0, gh - 12.0 - 3.0 - limit as f32 * 12.0, maxw + 1.0);
        self.rect = (rx, ry, rw, limit as f32 * 12.0);
        let fill = 0xCF00_0000;
        let (prev, next) = (self.offset > 0, self.items.len() > self.offset + limit);
        if prev || next {
            t.fill(rx, ry - 1.0, rx + rw, ry, fill);
            t.fill(
                rx,
                ry + limit as f32 * 12.0,
                rx + rw,
                ry + limit as f32 * 12.0 + 1.0,
                fill,
            );
            for i in (0..rw as i32).step_by(2) {
                if prev {
                    t.fill(
                        rx + i as f32,
                        ry - 1.0,
                        rx + i as f32 + 1.0,
                        ry,
                        0xFFFF_FFFF,
                    );
                }
                if next {
                    let yb = ry + limit as f32 * 12.0;
                    t.fill(
                        rx + i as f32,
                        yb,
                        rx + i as f32 + 1.0,
                        yb + 1.0,
                        0xFFFF_FFFF,
                    );
                }
            }
        }
        for i in 0..limit {
            let yy = ry + 12.0 * i as f32;
            t.fill(rx, yy, rx + rw, yy + 12.0, fill);
            let col = if i + self.offset == self.cur {
                0xFFFF00
            } else {
                0xAAAAAA
            };
            t.draw(&self.items[i + self.offset], rx + 1.0, yy + 2.0, col, true);
        }
    }
}

#[derive(Clone, Copy)]
pub enum SuggKey {
    Up,
    Down,
    Tab,
    Esc,
}

pub struct Chat {
    pub sg: Sugg,
    msgs: VecDeque<Msg>,
    /// Индекс 0 — самая новая строка (VANILLA: trimmedMessages.addFirst).
    lines: Vec<Line>,
    wrap_w: i32,
    scroll: usize,
    new_since_scroll: bool,
    pub open: bool,
    pub edit: EditBox,
    pub hist: History,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            sg: Sugg::default(),
            msgs: VecDeque::new(),
            lines: Vec::new(),
            wrap_w: -1,
            scroll: 0,
            new_since_scroll: false,
            open: false,
            edit: EditBox::new(256),
            hist: History::default(),
        }
    }
}

impl Chat {
    pub fn add(&mut self, c: &Component, tick: u64, tag: Tag, lang: &Lang, colors: bool) {
        let spans = flatten(c, lang, colors);
        tracing::info!(
            "[CHAT] {}",
            spans.iter().map(|s| s.text.as_str()).collect::<String>()
        );
        self.msgs.push_back(Msg {
            added: tick,
            spans,
            tag,
            wrapped: false,
        });
        while self.msgs.len() > 100 {
            self.msgs.pop_front();
        }
    }

    /// Текст последних строк (для управления агентом): от старых к новым.
    pub fn plain_lines(&self, n: usize) -> Vec<String> {
        let k = self.msgs.len().saturating_sub(n);
        self.msgs
            .iter()
            .skip(k)
            .map(|m| m.spans.iter().map(|s| s.text.as_str()).collect())
            .collect()
    }

    fn char_w(t: &TextRenderer, c: char, bold: bool) -> f32 {
        let mut b = [0u8; 4];
        t.width_fmt(c.encode_utf8(&mut b), bold)
    }

    /// VANILLA: StringSplitter#splitLines + INDENT у продолжений.
    fn wrap(t: &TextRenderer, spans: &[Span], max_w: f32) -> Vec<Vec<Seg>> {
        let chars: Vec<(char, usize)> = spans
            .iter()
            .enumerate()
            .flat_map(|(i, s)| s.text.chars().map(move |c| (c, i)))
            .collect();
        let indent = Self::char_w(t, ' ', false);
        let mut lines: Vec<Vec<(char, usize)>> = Vec::new();
        let (mut start, mut first) = (0usize, true);
        let n = chars.len();
        loop {
            let avail = if first { max_w } else { max_w - indent };
            let (mut x, mut last_sp, mut i) = (0.0f32, None, start);
            let mut brk = None;
            while i < n {
                let (c, si) = chars[i];
                if c == '\n' {
                    brk = Some((i, i + 1));
                    break;
                }
                let cw = Self::char_w(t, c, spans[si].bold);
                if x + cw > avail && i > start {
                    brk = Some(if c == ' ' {
                        (i, i + 1)
                    } else if let Some(sp) = last_sp {
                        (sp, sp + 1)
                    } else {
                        (i, i)
                    });
                    break;
                }
                if c == ' ' {
                    last_sp = Some(i);
                }
                x += cw;
                i += 1;
            }
            match brk {
                Some((e, ns)) => {
                    lines.push(chars[start..e].to_vec());
                    start = ns;
                    first = false;
                }
                None => {
                    lines.push(chars[start..n].to_vec());
                    break;
                }
            }
        }
        lines
            .into_iter()
            .enumerate()
            .map(|(li, l)| {
                let mut segs: Vec<Seg> = Vec::new();
                let mut x = 0.0;
                if li > 0 {
                    segs.push(Seg {
                        span: Span::plain(" ".into()),
                        x: 0.0,
                    });
                    x = indent;
                }
                let mut cur: Option<(usize, String, f32)> = None;
                for (c, si) in l {
                    match &mut cur {
                        Some((s, text, _)) if *s == si => text.push(c),
                        _ => {
                            if let Some((s, text, sx)) = cur.take() {
                                segs.push(Seg {
                                    span: Span {
                                        text,
                                        ..spans[s].clone()
                                    },
                                    x: sx,
                                });
                            }
                            cur = Some((si, c.to_string(), x));
                        }
                    }
                    x += Self::char_w(t, c, spans[si].bold);
                }
                if let Some((s, text, sx)) = cur {
                    segs.push(Seg {
                        span: Span {
                            text,
                            ..spans[s].clone()
                        },
                        x: sx,
                    });
                }
                segs
            })
            .collect()
    }

    /// Перенос новых сообщений (и всех — при смене ширины).
    fn layout(&mut self, t: &TextRenderer, cfg: &Cfg) {
        let w = (cfg.width_px() as f32 / cfg.scale).floor() as i32;
        if w != self.wrap_w {
            self.wrap_w = w;
            self.lines.clear();
            for m in &mut self.msgs {
                m.wrapped = false;
            }
        }
        let open = self.open;
        for m in &mut self.msgs {
            if m.wrapped {
                continue;
            }
            m.wrapped = true;
            for segs in Self::wrap(t, &m.spans, w as f32) {
                if open && self.scroll > 0 {
                    self.new_since_scroll = true;
                    self.scroll += 1;
                }
                self.lines.insert(
                    0,
                    Line {
                        added: m.added,
                        segs,
                        tag: m.tag,
                    },
                );
            }
        }
        self.lines.truncate(100);
        let max = self.lines.len().saturating_sub(cfg.per_page(open));
        self.scroll = self.scroll.min(max);
    }

    /// VANILLA: ChatComponent#scrollChat.
    pub fn scroll_by(&mut self, d: i32, cfg: &Cfg) {
        let max = self.lines.len().saturating_sub(cfg.per_page(true)) as i32;
        self.scroll = (self.scroll as i32 + d).clamp(0, max.max(0)) as usize;
        if self.scroll == 0 {
            self.new_since_scroll = false;
        }
    }

    pub fn page(&mut self, up: bool, cfg: &Cfg) {
        let p = cfg.per_page(true) as i32 - 1;
        self.scroll_by(if up { p } else { -p }, cfg);
    }

    pub fn show(&mut self, text: &str) {
        self.open = true;
        self.edit.set(text);
        self.sg.reset(&self.edit.text);
        self.hist.reset();
        self.scroll = 0;
        self.new_since_scroll = false;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.scroll = 0;
        self.new_since_scroll = false;
    }

    /// Видимые строки: (индекс снизу, строка, alpha).
    fn visible(&self, cfg: &Cfg, tick: u64) -> Vec<(usize, &Line, f32)> {
        let per = cfg.per_page(self.open);
        let n = (self.lines.len().saturating_sub(self.scroll)).min(per);
        (0..n)
            .rev()
            .filter_map(|i| {
                let l = &self.lines[i + self.scroll];
                let a = if self.open {
                    1.0
                } else {
                    // VANILLA: AlphaCalculator.timeBased.
                    let t = 1.0 - (tick.saturating_sub(l.added)) as f32 / 200.0;
                    let t = (t * 10.0).clamp(0.0, 1.0);
                    t * t
                };
                (a > 1e-5).then_some((i, l, a))
            })
            .collect()
    }

    /// Рисует чат (HUD или за экраном ввода). `gw/gh` — размер в GUI-пикселях.
    pub fn draw(&mut self, t: &mut TextRenderer, gh: f32, tick: u64, cfg: &Cfg) {
        self.layout(t, cfg);
        let total = self.lines.len();
        if total == 0 {
            return;
        }
        let scale = cfg.scale.max(0.01);
        let max_w = (cfg.width_px() as f32 / scale).ceil();
        let bottom = ((gh - 40.0) / scale).floor();
        let text_op = cfg.opacity * 0.9 + 0.1;
        let eh = cfg.entry_h() as f32;
        let dy = (8.0 * (cfg.spacing as f64 + 1.0) - 4.0 * cfg.spacing as f64).round() as f32;
        let vis = self.visible(cfg, tick);
        t.set_transform(Some((0.0, 0.0, 0.0, scale)));
        let black = |a: f32| (((a.clamp(0.0, 1.0)) * 255.0) as u32) << 24;
        for (i, _, a) in &vis {
            let eb = bottom - *i as f32 * eh;
            t.fill(0.0, eb - eh, max_w + 8.0, eb, black(a * cfg.bg));
        }
        for (i, l, a) in &vis {
            let eb = bottom - *i as f32 * eh;
            let ty = eb - dy;
            for (k, s) in l.segs.iter().enumerate() {
                t.draw_fmt(
                    &s.span.text,
                    4.0 + s.x,
                    ty,
                    s.span.color,
                    a * text_op,
                    s.span.fmt(k == 0),
                    true,
                );
            }
            // Индикатор слева (handleTag).
            let c = l.tag.color();
            let al = (((a * text_op).clamp(0.0, 1.0)) * 255.0) as u32;
            t.fill(0.0, eb - eh, 2.0, eb, (al << 24) | c);
        }
        if self.open {
            let ch = vis.len() as f32 * eh;
            let virt = total as f32 * eh;
            if virt != ch {
                let y = (self.scroll as f32 * ch / total as f32 - bottom).floor();
                let h = (ch * ch / virt).floor();
                let alpha = if y > 0.0 { 170u32 } else { 96 };
                let col = if self.new_since_scroll {
                    0xCC3333
                } else {
                    0x33333A
                };
                let sx = max_w + 4.0 + 4.0;
                t.fill(sx, -y - h, sx + 2.0, -y, (alpha << 24) | col);
            }
        }
        t.set_transform(None);
    }

    /// Спан под курсором (координаты GUI).
    pub fn span_at(
        &mut self,
        t: &TextRenderer,
        gh: f32,
        mx: f32,
        my: f32,
        cfg: &Cfg,
    ) -> Option<Span> {
        self.layout(t, cfg);
        let scale = cfg.scale.max(0.01);
        let (x, y) = (mx / scale - 4.0, my / scale);
        let bottom = ((gh - 40.0) / scale).floor();
        let eh = cfg.entry_h() as f32;
        let vis = self.visible(cfg, u64::MAX);
        for (i, l, _) in vis {
            let eb = bottom - i as f32 * eh;
            if y >= eb - eh && y < eb {
                for s in &l.segs {
                    let w = t.width_fmt(&s.span.text, s.span.bold);
                    if x >= s.x && x < s.x + w {
                        return Some(s.span.clone());
                    }
                }
                return None;
            }
        }
        None
    }

    /// Поле ввода ChatScreen (над ним уже нарисован чат).
    pub fn draw_input(&mut self, t: &mut TextRenderer, gw: f32, gh: f32, bg: f32) {
        t.fill(
            2.0,
            gh - 14.0,
            gw - 2.0,
            gh - 2.0,
            ((bg.clamp(0.0, 1.0) * 255.0) as u32) << 24,
        );
        let chars: Vec<char> = self.edit.text.chars().collect();
        let width = gw - 4.0 - 4.0;
        let mut pre = vec![0.0f32];
        for c in &chars {
            let mut b = [0u8; 4];
            let w = t.text_width(c.encode_utf8(&mut b));
            pre.push(pre.last().copied().unwrap_or(0.0) + w);
        }
        let wd = |a: usize, b: usize| pre[b] - pre[a.min(b)];
        let cur = self.edit.cursor;
        let mut first = 0;
        while first < cur && wd(first, cur) > width - 8.0 {
            first += 1;
        }
        let mut last = chars.len();
        while last > first && wd(first, last) > width {
            last -= 1;
        }
        // VANILLA: EditBox#renderWidget — без рамки textY = getY() = gh-12
        let (x0, ty) = (4.0, gh - 12.0);
        let (sa, sb) = self.edit.selection();
        if sa != sb {
            let (a, b) = (sa.clamp(first, last), sb.clamp(first, last));
            t.fill(
                x0 + wd(first, a),
                ty - 1.0,
                x0 + wd(first, b),
                ty + 10.0,
                0xFF3A6FD8,
            );
        }
        let txt: String = chars[first..last].iter().collect();
        t.draw(&txt, x0, ty, 0xE0E0E0, true);
        let blink = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() / 300);
        if blink % 2 == 0 && cur >= first && cur <= last {
            let cx = x0 + wd(first, cur);
            if cur == chars.len() {
                t.draw("_", cx, ty, 0xE0E0E0, true);
            } else {
                t.fill(cx, ty - 1.0, cx + 1.0, ty + 10.0, 0xFFD0D0D0);
            }
        }
        self.sg.draw(t, gw, gh, &self.edit);
    }
}

/// VANILLA: StringUtils.normalizeSpace(trim) — серии пробелов/табов → один пробел.
pub fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_space() {
        assert_eq!(normalize("  a \t  b  "), "a b");
        assert_eq!(normalize("   "), "");
    }

    #[test]
    fn section_codes() {
        let mut out = Vec::new();
        push_text(&mut out, "a§cb§lc§rd", &Span::plain(String::new()));
        assert_eq!(out.len(), 4);
        assert_eq!(out[1].color, 0xFF5555);
        assert!(out[2].bold && out[2].color == 0xFF5555);
        assert!(!out[3].bold && out[3].color == 0xFFFFFF);
    }

    #[test]
    fn translate_args() {
        let lang = Lang::load(br#"{"chat.type.text":"<%s> %s"}"#);
        let c = Component {
            content: Content::Translatable {
                key: "chat.type.text".into(),
                fallback: None,
                args: vec![Component::text("Bob"), Component::text("hi")],
            },
            style: Style::default(),
            siblings: vec![],
        };
        let s: String = flatten(&c, &lang, true)
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert_eq!(s, "<Bob> hi");
    }
}
