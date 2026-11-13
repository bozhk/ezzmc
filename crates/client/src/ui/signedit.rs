//! Экран правки таблички. VANILLA: AbstractSignEditScreen / SignEditScreen / HangingSignEditScreen
//! (4 строки, ↑/↓/Enter — смена строки, Esc/Done — закрыть и отправить ServerboundSignUpdatePacket).
//! VANILLA-UNSURE: закрытие при уходе игрока/удалении таблички (isValid) не делаем.

use super::{edit_apply, Action, Frame, State, UiKey, WHITE};
use crate::editbox::{EditBox, Move};

/// Деревья, для которых есть GUI-текстуры `signs/` и `hanging_signs/`.
pub const WOODS: [&str; 13] = [
    "acacia", "bamboo", "birch", "cherry", "crimson", "dark_oak", "jungle", "mangrove", "oak",
    "pale_oak", "poplar", "spruce", "warped",
];

pub struct SignEdit {
    pub pos: [i32; 3],
    pub front: bool,
    line: usize,
    boxes: [EditBox; 4],
    wood: String,
    hanging: bool,
    wall: bool,
    /// Цвет текста в редакторе (VANILLA: textColor), без альфы.
    color: u32,
    opened_ms: Option<u64>,
}

/// `name` — имя блока (`oak_wall_sign`, `bamboo_hanging_sign`), `lines` — текущий текст, `color` — цвет.
pub fn open(pos: [i32; 3], front: bool, name: &str, lines: &[String; 4], color: u32) -> SignEdit {
    let hanging = name.contains("hanging");
    let wall = name.contains("_wall_");
    let wood = name
        .trim_end_matches("_sign")
        .trim_end_matches("_hanging")
        .trim_end_matches("_wall")
        .to_owned();
    let wood = if WOODS.contains(&wood.as_str()) {
        wood
    } else {
        "oak".to_owned()
    };
    let mk = |s: &String| {
        let mut e = EditBox::new(384);
        e.set(s);
        e
    };
    SignEdit {
        pos,
        front,
        line: 0,
        boxes: [mk(&lines[0]), mk(&lines[1]), mk(&lines[2]), mk(&lines[3])],
        wood,
        hanging,
        wall,
        color,
        opened_ms: None,
    }
}

impl SignEdit {
    pub fn lines(&self) -> [String; 4] {
        std::array::from_fn(|i| self.boxes[i].text.clone())
    }
}

pub(super) fn screen(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    now_ms: u64,
    action: &mut Action,
) {
    let Some(mut s) = st.sign.take() else {
        *action = Action::CloseSign;
        return;
    };
    let start = *s.opened_ms.get_or_insert(now_ms);
    let (line_h, max_w) = if s.hanging { (9.0, 60.0) } else { (10.0, 90.0) };
    let mut done = false;
    for k in keys {
        match k {
            // VANILLA: AbstractSignEditScreen#keyPressed
            UiKey::Escape => done = true,
            UiKey::Up => {
                s.line = (s.line + 3) & 3;
                let n = s.boxes[s.line].len();
                s.boxes[s.line].cursor = n;
                s.boxes[s.line].sel = n;
            }
            UiKey::Down | UiKey::Enter => {
                s.line = (s.line + 1) & 3;
                let n = s.boxes[s.line].len();
                s.boxes[s.line].cursor = n;
                s.boxes[s.line].sel = n;
            }
            _ => {}
        }
    }
    let rest: Vec<UiKey> = keys
        .iter()
        .filter(|k| !matches!(k, UiKey::Up | UiKey::Down | UiKey::Enter | UiKey::Escape))
        .copied()
        .collect();
    {
        let e = &mut s.boxes[s.line];
        let saved = (e.text.clone(), e.cursor, e.sel);
        edit_apply(e, &rest, typed, &mut st.clip_out);
        // VANILLA: validator `font.width(s) <= sign.getMaxTextLineWidth()`.
        if f.t.text_width(&e.text) > max_w {
            (e.text, e.cursor, e.sel) = saved;
        }
    }
    let (w, h) = (f.w, f.h);
    f.center("sign.edit", 40.0, WHITE);
    if s.hanging {
        f.center("hanging_sign.edit", 40.0, WHITE);
    }
    // Фон-спрайт и положение текста.
    let (cx, cy, sc) = if s.hanging {
        let k = 4.5;
        f.p.blit(
            &format!("hsign_{}", s.wood),
            w / 2.0 - 8.0 * k,
            112.0 - 8.0 * k,
            16.0 * k,
            16.0 * k,
            [0.0, 0.0, 1.0, 1.0],
            0xFFFF_FFFF,
        );
        (w / 2.0, 125.0, 1.0)
    } else {
        let k = 3.9;
        let dh = if s.wall { 12.0 } else { 26.0 };
        f.p.blit(
            &format!("sign_{}", s.wood),
            w / 2.0 - 12.0 * k,
            117.0 - 13.0 * k,
            24.0 * k,
            dh * k,
            [0.0, 0.0, 1.0, dh / 26.0],
            0xFFFF_FFFF,
        );
        (w / 2.0, 90.0, 0.976_562_8)
    };
    f.t.set_transform(Some((cx, cy, 0.0, sc)));
    let mid = 4.0 * line_h / 2.0;
    let show_cursor = ((now_ms - start) / 300) % 2 == 0;
    for (i, b) in s.boxes.iter().enumerate() {
        let tw = f.t.text_width(&b.text);
        let x = (-tw / 2.0).floor();
        let y = i as f32 * line_h - mid;
        f.t.draw(&b.text, x, y, s.color, false);
        if i != s.line {
            continue;
        }
        let cs: Vec<char> = b.text.chars().collect();
        let width = |n: usize| {
            f.t.text_width(&cs[..n.min(cs.len())].iter().collect::<String>())
        };
        let cx_ = x + width(b.cursor);
        let (a, e) = b.selection();
        if a < e {
            f.t.fill(x + width(a), y, x + width(e), y + line_h, 0xFF30_60E0);
        }
        if show_cursor {
            if b.cursor >= cs.len() {
                f.t.draw("_", cx_, y, s.color, false);
            } else {
                f.t.fill(cx_, y - 1.0, cx_ + 1.0, y + line_h, 0xFF00_0000 | s.color);
            }
        }
    }
    f.t.set_transform(None);
    if f.button(
        w / 2.0 - 100.0,
        (h / 4.0).floor() + 144.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        done = true;
    }
    let _ = Move::End;
    if done {
        st.sign_out = Some((s.pos, s.front, s.lines()));
        *action = Action::CloseSign;
    } else {
        st.sign = Some(s);
    }
}
