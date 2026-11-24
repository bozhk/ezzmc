//! Экран правки книги с пером и подписи книги.
//! VANILLA: BookEditScreen / BookSignScreen / MultilineTextField (114×126 текста, 1024 символа на страницу,
//! 14 строк, ≤100 страниц; «Готово» сохраняет и шлёт ServerboundEditBookPacket, Esc — закрывает без сохранения).
//! VANILLA-UNSURE: PageUp/PageDown (листание клавишами) не делаем; перенос строк — по пробелу, как в bookview.

use super::{edit_apply, Action, Frame, State, UiKey};
use crate::editbox::{EditBox, Move};

const TEXT_W: f32 = 114.0;
const LINE_LIMIT: usize = 126 / 9;
const MAX_PAGES: usize = 100;
const TOP: f32 = 2.0;

pub struct BookEdit {
    /// Слот инвентаря (хотбар 0..8 или 40 для второй руки) — уходит в пакет.
    pub slot: i32,
    pages: Vec<String>,
    cur: usize,
    edit: EditBox,
    /// Экран подписи (BookSignScreen).
    signing: bool,
    title: EditBox,
    author: String,
    opened_ms: Option<u64>,
}

/// Результат для main: слот, страницы, заголовок (Some — книга подписывается).
pub struct BookOut {
    pub slot: i32,
    pub pages: Vec<String>,
    pub title: Option<String>,
}

pub fn open(slot: i32, pages: Vec<String>, author: &str) -> BookEdit {
    let mut pages = pages;
    if pages.is_empty() {
        pages.push(String::new());
    }
    let mut edit = EditBox::new(1024);
    edit.set(&pages[0]);
    BookEdit {
        slot,
        pages,
        cur: 0,
        edit,
        signing: false,
        title: EditBox::new(15),
        author: author.to_owned(),
        opened_ms: None,
    }
}

/// Строки страницы как диапазоны символов. VANILLA: MultilineTextField#reflowDisplayLines (StringSplitter).
fn wrap(f: &Frame, text: &str) -> Vec<(usize, usize)> {
    let cs: Vec<char> = text.chars().collect();
    let mut lines = Vec::new();
    let (mut start, mut w) = (0usize, 0.0f32);
    let mut last_space: Option<usize> = None;
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == '\n' {
            lines.push((start, i + 1));
            start = i + 1;
            w = 0.0;
            last_space = None;
            i += 1;
            continue;
        }
        let cw = f.t.text_width(&c.to_string());
        if w + cw > TEXT_W && i > start {
            let end = last_space.map_or(i, |s| s + 1);
            lines.push((start, end));
            start = end;
            w = f.t.text_width(&cs[start..i].iter().collect::<String>());
            last_space = cs[start..i]
                .iter()
                .rposition(|c| *c == ' ')
                .map(|p| p + start);
        }
        if c == ' ' {
            last_space = Some(i);
        }
        w += cw;
        i += 1;
    }
    lines.push((start, cs.len()));
    lines
}

fn width(f: &Frame, cs: &[char], a: usize, b: usize) -> f32 {
    let s: String = cs[a..b].iter().filter(|c| **c != '\n').collect();
    f.t.text_width(&s)
}

fn line_of(lines: &[(usize, usize)], cursor: usize) -> usize {
    lines.iter().rposition(|(s, _)| *s <= cursor).unwrap_or(0)
}

/// Индекс символа в строке по x (ближайшая граница). VANILLA: MultilineTextField#seekCursorToPoint.
fn col_at(f: &Frame, cs: &[char], line: (usize, usize), x: f32) -> usize {
    let (s, e) = line;
    let e = if e > s && cs[e - 1] == '\n' { e - 1 } else { e };
    let mut best = s;
    for i in s..=e {
        if width(f, cs, s, i) <= x + 0.001 {
            best = i;
        } else {
            let prev = width(f, cs, s, best);
            if width(f, cs, s, i) - x < x - prev {
                best = i;
            }
            break;
        }
    }
    best
}

fn page_label(f: &Frame, b: &BookEdit) -> String {
    f.lang.fmt(
        "book.pageIndicator",
        &[&(b.cur + 1).to_string(), &b.pages.len().to_string()],
    )
}

fn set_page(b: &mut BookEdit, n: usize) {
    b.pages[b.cur] = b.edit.text.clone();
    b.cur = n;
    let t = b.pages[n].clone();
    b.edit.set(&t);
}

pub(super) fn screen(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    now_ms: u64,
    action: &mut Action,
) {
    let Some(mut b) = st.book.take() else {
        *action = Action::CloseBook;
        return;
    };
    let start = *b.opened_ms.get_or_insert(now_ms);
    let show_cursor = ((now_ms - start) / 300) % 2 == 0;
    let w = f.w;
    let left = ((w - 192.0) / 2.0).floor();
    f.p.blit(
        "book",
        left,
        TOP,
        192.0,
        192.0,
        [0.0, 0.0, 192.0 / 256.0, 192.0 / 256.0],
        0xFFFF_FFFF,
    );
    let mid = (w as i32 / 2) as f32;
    let by = TOP + 192.0 + 2.0;
    let mut done = false;
    let mut esc = false;
    if b.signing {
        // BookSignScreen
        for k in keys {
            if matches!(k, UiKey::Escape) {
                esc = true;
            }
        }
        let rest: Vec<UiKey> = keys
            .iter()
            .filter(|k| !matches!(k, UiKey::Escape | UiKey::Enter))
            .copied()
            .collect();
        edit_apply(&mut b.title, &rest, typed, &mut st.clip_out);
        let ok = !b.title.text.trim().is_empty();
        let confirm = keys.iter().any(|k| matches!(k, UiKey::Enter)) && ok;
        let label = f.lang.tr("book.editTitle");
        let tw = f.t.text_width(&label);
        f.t.draw(
            &label,
            left + 36.0 + (TEXT_W - tw) / 2.0,
            34.0,
            0x00_0000,
            false,
        );
        let t = b.title.text.clone();
        let tx = ((w as i32 - 114) / 2 - 3) as f32 + (114.0 - f.t.text_width(&t)) / 2.0;
        // VANILLA: EditBox без рамки рисует текст на y = getY() (50), не по центру высоты.
        f.t.draw(&t, tx, 50.0, 0x00_0000, false);
        if show_cursor {
            f.t.draw("_", tx + f.t.text_width(&t), 50.0, 0x00_0000, false);
        }
        let by_author = f.lang.fmt("book.byAuthor", &[&b.author]);
        let aw = f.t.text_width(&by_author);
        f.t.draw(
            &by_author,
            left + 36.0 + (TEXT_W - aw) / 2.0,
            60.0,
            0x55_5555,
            false,
        );
        let warn = f.lang.tr("book.finalizeWarning");
        let mut y = 82.0;
        for l in wrap_plain(f, &warn) {
            f.t.draw(&l, left + 36.0, y, 0x00_0000, false);
            y += 9.0;
        }
        if f.button(
            mid - 100.0,
            196.0,
            98.0,
            20.0,
            &f.lang.tr("book.finalizeButton"),
            ok,
        ) || confirm
        {
            done = true;
        }
        if f.button(mid + 2.0, 196.0, 98.0, 20.0, &f.lang.tr("gui.cancel"), true) {
            b.signing = false;
        }
        if esc {
            *action = Action::CloseBook;
            return;
        }
        if done {
            st.book_out = Some(BookOut {
                slot: b.slot,
                pages: finish(&mut b),
                title: Some(b.title.text.trim().to_owned()),
            });
            *action = Action::CloseBook;
        } else {
            st.book = Some(b);
        }
        return;
    }

    // BookEditScreen
    let x0 = ((w as i32 - 114) / 2 - 4) as f32;
    let y0 = 32.0;
    let cs: Vec<char> = b.edit.text.chars().collect();
    let lines = wrap(f, &b.edit.text);
    let mut arrows = Vec::new();
    for k in keys {
        match k {
            UiKey::Escape => esc = true,
            UiKey::Up | UiKey::Down => arrows.push(*k),
            _ => {}
        }
    }
    for k in &arrows {
        let ln = line_of(&lines, b.edit.cursor);
        let x = width(f, &cs, lines[ln].0, b.edit.cursor);
        let to = if matches!(k, UiKey::Up) {
            ln.checked_sub(1)
        } else {
            (ln + 1 < lines.len()).then_some(ln + 1)
        };
        if let Some(t) = to {
            b.edit.cursor = col_at(f, &cs, lines[t], x);
            b.edit.sel = b.edit.cursor;
        } else if matches!(k, UiKey::Up) {
            b.edit.cursor = 0;
            b.edit.sel = 0;
        } else {
            b.edit.cursor = cs.len();
            b.edit.sel = cs.len();
        }
    }
    let enter = keys.iter().filter(|k| matches!(k, UiKey::Enter)).count();
    let rest: Vec<UiKey> = keys
        .iter()
        .filter(|k| !matches!(k, UiKey::Up | UiKey::Down | UiKey::Enter | UiKey::Escape))
        .copied()
        .collect();
    {
        let saved = (b.edit.text.clone(), b.edit.cursor, b.edit.sel);
        edit_apply(&mut b.edit, &rest, typed, &mut st.clip_out);
        for _ in 0..enter {
            // VANILLA: BookEditScreen → MultiLineEditBox#keyPressed: ENTER → insertText("\n").
            let t = std::mem::take(&mut b.edit.text);
            b.edit.text = t;
            let (a, e) = b.edit.selection();
            let cs2: Vec<char> = b.edit.text.chars().collect();
            let mut s: String = cs2[..a].iter().collect();
            s.push('\n');
            s.extend(cs2[e..].iter());
            if s.chars().count() <= b.edit.max {
                b.edit.text = s;
                b.edit.cursor = a + 1;
                b.edit.sel = a + 1;
            }
        }
        // VANILLA: MultilineTextField#overflowsLineLimit.
        if wrap(f, &b.edit.text).len() > LINE_LIMIT {
            (b.edit.text, b.edit.cursor, b.edit.sel) = saved;
        }
    }
    let cs: Vec<char> = b.edit.text.chars().collect();
    let lines = wrap(f, &b.edit.text);
    let _ = Move::End;
    // Клик по тексту.
    let (mx, my) = (f.mx, f.my);
    if f.click && mx >= x0 && mx < x0 + TEXT_W && my >= y0 && my < y0 + LINE_LIMIT as f32 * 9.0 {
        let row = (((my - y0) / 9.0).floor() as usize).min(lines.len() - 1);
        b.edit.cursor = col_at(f, &cs, lines[row], mx - x0);
        b.edit.sel = b.edit.cursor;
    }
    // Текст, выделение и курсор. VANILLA: MultiLineEditBox#extractContents (чёрный, без тени).
    let ln_c = line_of(&lines, b.edit.cursor);
    let (sa, se) = b.edit.selection();
    for (i, (s, e)) in lines.iter().enumerate() {
        let y = y0 + i as f32 * 9.0;
        let txt: String = cs[*s..*e].iter().filter(|c| **c != '\n').collect();
        if sa < se && sa < *e && se > *s {
            let (a, z) = (sa.max(*s), se.min(*e));
            f.t.fill(
                x0 + width(f, &cs, *s, a),
                y,
                x0 + width(f, &cs, *s, z).max(width(f, &cs, *s, a) + 1.0),
                y + 9.0,
                0xFF30_60E0,
            );
        }
        f.t.draw(&txt, x0, y, 0x00_0000, false);
        if i == ln_c && show_cursor && sa == se {
            let cx = x0 + width(f, &cs, *s, b.edit.cursor);
            if b.edit.cursor >= cs.len() && !(cs.last() == Some(&'\n')) {
                f.t.draw("_", cx, y, 0x00_0000, false);
            } else {
                f.t.fill(cx, y - 1.0, cx + 1.0, y + 9.0, 0xFF00_0000);
            }
        }
    }
    // Номер страницы справа.
    let label = page_label(f, &b);
    let lw = f.t.text_width(&label);
    f.t.draw(&label, left + 148.0 - lw, TOP + 16.0, 0x00_0000, false);
    // Кнопки перелистывания: «назад» только не на первой, «вперёд» — всегда (добавляет страницу).
    let (bw, bh) = (23.0, 13.0);
    let back = (left + 43.0, TOP + 157.0);
    let fwd = (left + 116.0, TOP + 157.0);
    let over = |p: (f32, f32)| mx >= p.0 && mx < p.0 + bw && my >= p.1 && my < p.1 + bh;
    let mut go: i32 = 0;
    if b.cur > 0 {
        let hl = over(back);
        f.p.blit(
            if hl { "pg_back_hl" } else { "pg_back" },
            back.0,
            back.1,
            bw,
            bh,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
        if hl && f.click {
            f.click = false;
            go = -1;
        }
    }
    {
        let hl = over(fwd);
        f.p.blit(
            if hl { "pg_fwd_hl" } else { "pg_fwd" },
            fwd.0,
            fwd.1,
            bw,
            bh,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
        if hl && f.click {
            f.click = false;
            go = 1;
        }
    }
    if f.button(
        mid - 100.0,
        by,
        98.0,
        20.0,
        &f.lang.tr("book.signButton"),
        true,
    ) {
        b.pages[b.cur] = b.edit.text.clone();
        b.signing = true;
    }
    if f.button(mid + 2.0, by, 98.0, 20.0, &f.lang.tr("gui.done"), true) {
        done = true;
    }
    if go < 0 && b.cur > 0 {
        let n = b.cur - 1;
        set_page(&mut b, n);
    } else if go > 0 {
        if b.cur + 1 < b.pages.len() {
            let n = b.cur + 1;
            set_page(&mut b, n);
        } else if b.pages.len() < MAX_PAGES {
            // VANILLA: BookEditScreen#appendPageToBook.
            b.pages.push(String::new());
            let n = b.cur + 1;
            set_page(&mut b, n);
        }
    }
    if esc {
        *action = Action::CloseBook;
        return;
    }
    if done {
        st.book_out = Some(BookOut {
            slot: b.slot,
            pages: finish(&mut b),
            title: None,
        });
        *action = Action::CloseBook;
    } else {
        st.book = Some(b);
    }
}

/// VANILLA: BookEditScreen#eraseEmptyTrailingPages.
fn finish(b: &mut BookEdit) -> Vec<String> {
    b.pages[b.cur] = b.edit.text.clone();
    let mut p = b.pages.clone();
    while p.last().is_some_and(String::is_empty) {
        p.pop();
    }
    p
}

/// Перенос простого текста по ширине 114 (подпись «нельзя будет изменить»).
fn wrap_plain(f: &Frame, s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for word in s.split(' ') {
        let cand = if cur.is_empty() {
            word.to_owned()
        } else {
            format!("{cur} {word}")
        };
        if f.t.text_width(&cand) > TEXT_W && !cur.is_empty() {
            out.push(std::mem::replace(&mut cur, word.to_owned()));
        } else {
            cur = cand;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}
