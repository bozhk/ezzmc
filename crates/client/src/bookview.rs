//! Экран книги на кафедре: страница, перелистывание, ПКМ по ссылкам.
//! VANILLA: LecternScreen / BookViewScreen (192×192 из book.png 256×256), LecternMenu
//! (слот 0 = книга, data[0] = страница), WrittenBookContent / WritableBookContent.
//! Клиент страницу не меняет: кнопки меню 1 назад, 2 вперёд, 3 взять книгу, 100+N — на страницу N.
//! VANILLA-UNSURE: перенос строк — свой (по пробелу, `\n`, длинное слово рвётся по символу), не StringSplitter.
use crate::chat::{self, Span};
use crate::inventory::OpenMenu;
use crate::netthread::NetCmd;
use crate::ui::lang::Lang;
use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
use ezzmc_protocol::item::ItemStack;
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::{ClickEvent, Component};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::{Fmt, TextRenderer};

const TEXT_W: f32 = 114.0;
const MAX_LINES: usize = 128 / 9;

/// Левый край книги. VANILLA: BookViewScreen#backgroundLeft.
pub fn left(gw: f32) -> f32 {
    ((gw - 192.0) / 2.0).floor()
}

/// Верх книги. VANILLA: BookViewScreen#backgroundTop.
pub const TOP: f32 = 2.0;

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

fn comp<'a>(s: &'a ItemStack, name: &str) -> Option<&'a [u8]> {
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == name)? as u32;
    s.patch
        .added
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, b)| &b[..])
}

/// Страницы книги. VANILLA: BookViewScreen.BookAccess#fromItem (фильтр текста выключен).
pub fn pages(s: &ItemStack) -> Vec<Component> {
    fn written(b: &[u8]) -> ProtocolResult<Vec<Component>> {
        let mut r = Reader::new(b);
        r.string(32)?;
        r.optional(|r| r.string(32))?;
        r.string(32767)?;
        r.var_int()?;
        let n = r.var_int()?.max(0);
        let mut out = Vec::new();
        for _ in 0..n {
            out.push(component(&mut r)?);
            r.optional(component)?;
        }
        Ok(out)
    }
    fn writable(b: &[u8]) -> ProtocolResult<Vec<Component>> {
        let mut r = Reader::new(b);
        let n = r.var_int()?.max(0);
        let mut out = Vec::new();
        for _ in 0..n {
            out.push(Component::text(r.string(1024)?));
            r.optional(|r| r.string(1024))?;
        }
        Ok(out)
    }
    if let Some(b) = comp(s, "written_book_content") {
        return written(b).unwrap_or_default();
    }
    comp(s, "writable_book_content")
        .and_then(|b| writable(b).ok())
        .unwrap_or_default()
}

/// Текущая страница, зажатая в пределы книги.
fn current(o: &OpenMenu, count: usize) -> usize {
    (o.data[0].max(0) as usize).min(count.saturating_sub(1))
}

/// Кусок строки одним стилем: индекс спана, текст, смещение от начала строки.
struct Seg {
    si: usize,
    text: String,
    x: f32,
}

/// Перенос страницы по ширине 114. VANILLA: Font#split(text, 114).
fn wrap(t: &TextRenderer, spans: &[Span]) -> Vec<Vec<Seg>> {
    // Плоский список символов со стилем.
    let mut ch: Vec<(usize, char, f32)> = Vec::new();
    for (si, sp) in spans.iter().enumerate() {
        for c in sp.text.chars() {
            let mut b = [0u8; 4];
            ch.push((si, c, t.width_fmt(c.encode_utf8(&mut b), sp.bold)));
        }
    }
    let mut lines: Vec<Vec<(usize, char, f32)>> = Vec::new();
    let mut cur: Vec<(usize, char, f32)> = Vec::new();
    let mut w = 0.0;
    for item in ch {
        if item.1 == '\n' {
            lines.push(std::mem::take(&mut cur));
            w = 0.0;
            continue;
        }
        if w + item.2 > TEXT_W && !cur.is_empty() {
            // Разрыв по последнему пробелу (он съедается), иначе посреди слова.
            if let Some(sp) = cur.iter().rposition(|c| c.1 == ' ') {
                let rest: Vec<_> = cur.split_off(sp + 1);
                cur.pop();
                lines.push(std::mem::replace(&mut cur, rest));
            } else {
                lines.push(std::mem::take(&mut cur));
            }
            w = cur.iter().map(|c| c.2).sum();
        }
        w += item.2;
        cur.push(item);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
        .into_iter()
        .map(|l| {
            let mut segs: Vec<Seg> = Vec::new();
            let mut x = 0.0;
            for (si, c, cw) in l {
                match segs.last_mut() {
                    Some(s) if s.si == si => s.text.push(c),
                    _ => segs.push(Seg {
                        si,
                        text: c.to_string(),
                        x,
                    }),
                }
                x += cw;
            }
            segs
        })
        .collect()
}

fn page_lines(
    t: &TextRenderer,
    lang: &Lang,
    o: &OpenMenu,
) -> (Vec<Span>, Vec<Vec<Seg>>, usize, usize) {
    let Some(book) = o.slots.first().and_then(Option::as_ref) else {
        return (Vec::new(), Vec::new(), 0, 0);
    };
    let pg = pages(book);
    let cur = current(o, pg.len());
    // VANILLA: ComponentUtils.mergeStyles(page, PAGE_TEXT_STYLE) — чёрный по умолчанию, без тени.
    let spans = pg
        .get(cur)
        .map(|c| chat::flatten_styled(c, lang, 0x000000, false))
        .unwrap_or_default();
    let lines = wrap(t, &spans);
    (spans, lines, cur, pg.len())
}

#[derive(Clone, Copy, PartialEq)]
enum Btn {
    Back,
    Forward,
    Done,
    Take,
}

/// Прямоугольники кнопок (x, y, w, h). VANILLA: createPageControlButtons / LecternScreen#createMenuControls.
fn buttons(gw: f32, cur: usize, count: usize, may_build: bool) -> Vec<(Btn, f32, f32, f32, f32)> {
    let (l, top) = (left(gw), TOP);
    let mut v = Vec::new();
    if cur + 1 < count {
        v.push((Btn::Forward, l + 116.0, top + 157.0, 23.0, 13.0));
    }
    if cur > 0 {
        v.push((Btn::Back, l + 43.0, top + 157.0, 23.0, 13.0));
    }
    let y = top + 192.0 + 2.0;
    let mid = (gw as i32 / 2) as f32;
    if may_build {
        v.push((Btn::Done, mid - 100.0, y, 98.0, 20.0));
        v.push((Btn::Take, mid + 2.0, y, 98.0, 20.0));
    } else {
        v.push((Btn::Done, ((gw as i32 - 200) / 2) as f32, y, 200.0, 20.0));
    }
    v
}

fn hit(b: &(Btn, f32, f32, f32, f32), mx: f32, my: f32) -> bool {
    mx >= b.1 && my >= b.2 && mx < b.1 + b.3 && my < b.2 + b.4
}

/// Фон книги и кнопки (спрайты). Вызывать до `draw_text`.
#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(
    p: &mut GuiPainter,
    t: &TextRenderer,
    lang: &Lang,
    o: &OpenMenu,
    gw: f32,
    mx: f32,
    my: f32,
    may_build: bool,
) {
    let (_, _, cur, count) = page_lines(t, lang, o);
    let k = 192.0 / 256.0;
    p.blit(
        "book",
        left(gw),
        TOP,
        192.0,
        192.0,
        [0.0, 0.0, k, k],
        0xFFFF_FFFF,
    );
    for b in buttons(gw, cur, count, may_build) {
        let hov = hit(&b, mx, my);
        match b.0 {
            Btn::Back | Btn::Forward => {
                let name = match (b.0, hov) {
                    (Btn::Forward, false) => "pg_fwd",
                    (Btn::Forward, true) => "pg_fwd_hl",
                    (_, false) => "pg_back",
                    (_, true) => "pg_back_hl",
                };
                p.blit(
                    name,
                    b.1,
                    b.2,
                    23.0,
                    13.0,
                    [0.0, 0.0, 1.0, 1.0],
                    0xFFFF_FFFF,
                );
            }
            _ => p.nine_slice(
                if hov { "button_hl" } else { "button" },
                b.1,
                b.2,
                b.3,
                b.4,
                [200.0, 20.0],
                [3.0; 4],
                0xFFFF_FFFF,
            ),
        }
    }
}

/// Текст страницы, индикатор и подписи кнопок.
pub fn draw_text(t: &mut TextRenderer, lang: &Lang, o: &OpenMenu, gw: f32, may_build: bool) {
    let (spans, lines, cur, count) = page_lines(t, lang, o);
    let l = left(gw);
    // VANILLA: «book.pageIndicator» выровнен вправо в (left+148, top+16), чёрный, без тени.
    let ind = lang.fmt(
        "book.pageIndicator",
        &[&(cur + 1).to_string(), &count.max(1).to_string()],
    );
    let w = t.width_fmt(&ind, false);
    t.draw(&ind, l + 148.0 - w, TOP + 16.0, 0x000000, false);
    for (i, line) in lines.iter().take(MAX_LINES).enumerate() {
        for s in line {
            let sp = &spans[s.si];
            let f = Fmt {
                bold: sp.bold,
                italic: sp.italic,
                underline: sp.underline,
                strike: sp.strike,
                obfuscated: sp.obf,
                line_start: false,
            };
            t.draw_fmt(
                &s.text,
                l + 36.0 + s.x,
                TOP + 30.0 + i as f32 * 9.0,
                sp.color,
                1.0,
                f,
                false,
            );
        }
    }
    for b in buttons(gw, cur, count, may_build) {
        let key = match b.0 {
            Btn::Done => "gui.done",
            Btn::Take => "lectern.take_book",
            _ => continue,
        };
        let s = lang.tr(key);
        let tw = t.width_fmt(&s, false);
        t.draw(
            &s,
            b.1 + (b.3 - tw) / 2.0,
            b.2 + (b.4 - 8.0) / 2.0,
            0xFFFFFF,
            true,
        );
    }
}

pub enum Act {
    /// Клик мимо виджетов книги.
    Pass,
    /// Отправить пакет кнопки меню.
    Send(NetCmd),
    /// Закрыть экран (отправив закрытие), затем, возможно, выполнить команду.
    Close(Option<String>),
}

/// Клик. VANILLA: BookViewScreen#mouseClicked (ПКМ по тексту) + кнопки страниц/меню.
#[allow(clippy::too_many_arguments)]
pub fn click(
    t: &TextRenderer,
    lang: &Lang,
    o: &OpenMenu,
    gw: f32,
    mx: f32,
    my: f32,
    right: bool,
    may_build: bool,
) -> Act {
    let (spans, lines, cur, count) = page_lines(t, lang, o);
    if right {
        let l = left(gw);
        let row = ((my - (TOP + 30.0)) / 9.0).floor();
        if row >= 0.0 && (row as usize) < lines.len().min(MAX_LINES) && my >= TOP + 30.0 {
            let rx = mx - (l + 36.0);
            for s in &lines[row as usize] {
                let sp = &spans[s.si];
                let w = t.width_fmt(&s.text, sp.bold);
                if rx >= s.x && rx < s.x + w {
                    return match &sp.click {
                        // VANILLA: ClickEvent.ChangePage → forcePage(page − 1) → кнопка 100 + (page − 1).
                        Some(ClickEvent::ChangePage(pg)) => {
                            let target = pg - 1;
                            if target != o.data[0] as i32 {
                                Act::Send(NetCmd::ContainerButton(o.id, 100 + target))
                            } else {
                                Act::Pass
                            }
                        }
                        Some(ClickEvent::RunCommand(c)) => Act::Close(Some(c.clone())),
                        _ => Act::Pass,
                    };
                }
            }
        }
        return Act::Pass;
    }
    for b in buttons(gw, cur, count, may_build) {
        if hit(&b, mx, my) {
            return match b.0 {
                Btn::Back => Act::Send(NetCmd::ContainerButton(o.id, 1)),
                Btn::Forward => Act::Send(NetCmd::ContainerButton(o.id, 2)),
                Btn::Take => Act::Send(NetCmd::ContainerButton(o.id, 3)),
                Btn::Done => Act::Close(None),
            };
        }
    }
    Act::Pass
}

/// Стрелки ←/→. VANILLA: BookViewScreen#keyPressed (кнопка нажимается даже скрытой — решает сервер).
pub fn arrow(o: &OpenMenu, forward: bool) -> NetCmd {
    NetCmd::ContainerButton(o.id, if forward { 2 } else { 1 })
}
