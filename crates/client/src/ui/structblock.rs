//! Экран структурного блока. VANILLA: StructureBlockEditScreen (режимы SAVE/LOAD/CORNER/DATA,
//! поля имени/позиции/размера/целостности/сида/данных, кнопки поворота и зеркала, SAVE/LOAD/DETECT).
//! VANILLA-UNSURE: Alt+клик по кнопке режима (альтернативный список с DATA) не делаем.

use super::{edit_apply, Action, Frame, State, UiKey, GRAY, WHITE};
use crate::editbox::EditBox;
use ezzmc_protocol::nbt::Tag;

/// Индексы полей: имя, позиция XYZ, размер XYZ, целостность, сид, данные.
const NAME: usize = 0;
const POS: usize = 1;
const SIZE: usize = 4;
const INTEGRITY: usize = 7;
const SEED: usize = 8;
const DATA: usize = 9;

pub struct StructBlock {
    pub pos: [i32; 3],
    /// 0 SAVE, 1 LOAD, 2 CORNER, 3 DATA (StructureMode#ordinal).
    mode: u8,
    /// 0 NONE, 1 LEFT_RIGHT, 2 FRONT_BACK.
    mirror: u8,
    /// 0 NONE, 1 CLOCKWISE_90, 2 CLOCKWISE_180, 3 COUNTERCLOCKWISE_90.
    rot: u8,
    include_entities: bool,
    strict: bool,
    show_air: bool,
    show_bb: bool,
    fields: [EditBox; 10],
    focus: usize,
    opened_ms: Option<u64>,
}

/// Результат для main: ServerboundSetStructureBlockPacket.
pub struct StructOut {
    pub pos: [i32; 3],
    /// UpdateType: 0 UPDATE_DATA, 1 SAVE_AREA, 2 LOAD_AREA, 3 SCAN_AREA.
    pub update: u8,
    pub mode: u8,
    pub name: String,
    pub offset: [i32; 3],
    pub size: [i32; 3],
    pub mirror: u8,
    pub rot: u8,
    pub data: String,
    pub integrity: f32,
    pub seed: i64,
    pub flags: u8,
}

pub fn open(pos: [i32; 3]) -> StructBlock {
    let mk = |n| EditBox::new(n);
    let mut s = StructBlock {
        pos,
        mode: 3,
        mirror: 0,
        rot: 0,
        include_entities: true,
        strict: false,
        show_air: false,
        show_bb: true,
        fields: [
            mk(128),
            mk(15),
            mk(15),
            mk(15),
            mk(15),
            mk(15),
            mk(15),
            mk(15),
            mk(31),
            mk(128),
        ],
        focus: NAME,
        opened_ms: None,
    };
    s.update(None);
    s
}

/// VANILLA: «0.0###» (DecimalFormat, Locale.ROOT).
fn fmt_integrity(v: f32) -> String {
    let s = format!("{v:.4}");
    let t = s.trim_end_matches('0');
    if t.ends_with('.') {
        format!("{t}0")
    } else {
        t.to_owned()
    }
}

/// VANILLA: Screen#isValidCharacterForName.
fn valid_name_char(cur: &str, c: char, cursor: usize) -> bool {
    let colon = cur.chars().position(|x| x == ':').map_or(-1, |i| i as i64);
    let slash = cur.chars().position(|x| x == '/').map_or(-1, |i| i as i64);
    let cursor = cursor as i64;
    match c {
        ':' => (slash == -1 || cursor <= slash) && colon == -1,
        '/' => cursor > colon,
        '_' | '-' | '.' | 'a'..='z' | '0'..='9' => true,
        _ => false,
    }
}

impl StructBlock {
    /// VANILLA: StructureBlockEntity#loadAdditional (значения из block_entity_data; по умолчанию как у сущности).
    pub fn update(&mut self, tag: Option<&Tag>) {
        let get = |k: &str| tag.and_then(|t| t.get(k));
        let int = |k: &str, d: i64, lo: i64, hi: i64| {
            get(k).and_then(Tag::as_i64).unwrap_or(d).clamp(lo, hi)
        };
        let flag = |k: &str, d: bool| get(k).and_then(Tag::as_i64).map_or(d, |v| v != 0);
        let text = |k: &str| get(k).and_then(Tag::as_str).unwrap_or("").to_owned();
        self.fields[NAME].set(&text("name"));
        self.fields[DATA].set(&text("metadata"));
        for (i, k) in ["posX", "posY", "posZ"].iter().enumerate() {
            let d = if i == 1 { 1 } else { 0 };
            self.fields[POS + i].set(&int(k, d, -48, 48).to_string());
        }
        for (i, k) in ["sizeX", "sizeY", "sizeZ"].iter().enumerate() {
            self.fields[SIZE + i].set(&int(k, 0, 0, 48).to_string());
        }
        self.rot = match get("rotation").and_then(Tag::as_str) {
            Some("CLOCKWISE_90") => 1,
            Some("CLOCKWISE_180") => 2,
            Some("COUNTERCLOCKWISE_90") => 3,
            _ => 0,
        };
        self.mirror = match get("mirror").and_then(Tag::as_str) {
            Some("LEFT_RIGHT") => 1,
            Some("FRONT_BACK") => 2,
            _ => 0,
        };
        self.mode = match get("mode").and_then(Tag::as_str) {
            Some("SAVE") => 0,
            Some("LOAD") => 1,
            Some("CORNER") => 2,
            _ => 3,
        };
        self.include_entities = !flag("ignoreEntities", true);
        self.strict = flag("strict", false);
        self.show_air = flag("showair", false);
        self.show_bb = flag("showboundingbox", true);
        let integ = match get("integrity") {
            Some(Tag::Float(v)) => *v,
            _ => 1.0,
        };
        self.fields[INTEGRITY].set(&fmt_integrity(integ));
        self.fields[SEED].set(&int("seed", 0, i64::MIN, i64::MAX).to_string());
        self.fix_focus();
    }

    /// Поля, видимые в текущем режиме (VANILLA: updateMode).
    fn visible(&self) -> Vec<usize> {
        match self.mode {
            0 => vec![NAME, POS, POS + 1, POS + 2, SIZE, SIZE + 1, SIZE + 2],
            1 => vec![NAME, POS, POS + 1, POS + 2, INTEGRITY, SEED],
            2 => vec![NAME],
            _ => vec![DATA],
        }
    }

    fn fix_focus(&mut self) {
        let v = self.visible();
        if !v.contains(&self.focus) {
            self.focus = v[0];
        }
    }

    fn out(&self, update: u8) -> StructOut {
        let num = |i: usize| self.fields[i].text.trim().parse::<i32>().unwrap_or(0);
        StructOut {
            pos: self.pos,
            update,
            mode: self.mode,
            name: self.fields[NAME].text.clone(),
            offset: [num(POS), num(POS + 1), num(POS + 2)],
            size: [num(SIZE), num(SIZE + 1), num(SIZE + 2)],
            mirror: self.mirror,
            rot: self.rot,
            data: self.fields[DATA].text.clone(),
            integrity: self.fields[INTEGRITY].text.trim().parse().unwrap_or(1.0),
            seed: self.fields[SEED].text.trim().parse().unwrap_or(0),
            flags: u8::from(!self.include_entities)
                | u8::from(self.show_air) << 1
                | u8::from(self.show_bb) << 2
                | u8::from(self.strict) << 3,
        }
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
    let Some(mut s) = st.structure.take() else {
        *action = Action::CloseStructure;
        return;
    };
    let start = *s.opened_ms.get_or_insert(now_ms);
    let (w, _h) = (f.w, f.h);
    let cx = (w / 2.0).floor();
    let mut out: Option<StructOut> = None;
    let mut cancel = false;
    let mut rest: Vec<UiKey> = Vec::new();
    for k in keys {
        match k {
            UiKey::Escape => cancel = true,
            UiKey::Enter => out = Some(s.out(0)),
            UiKey::Tab | UiKey::BackTab => {
                let v = s.visible();
                let i = v.iter().position(|&x| x == s.focus).unwrap_or(0);
                let n = v.len();
                let j = if matches!(k, UiKey::Tab) {
                    (i + 1) % n
                } else {
                    (i + n - 1) % n
                };
                s.focus = v[j];
            }
            _ => rest.push(*k),
        }
    }
    let fo = s.focus;
    if fo == NAME {
        // VANILLA: имя фильтруется посимвольно (isValidCharacterForName).
        edit_apply(&mut s.fields[fo], &rest, "", &mut st.clip_out);
        for c in typed.chars() {
            let cur = s.fields[fo].cursor;
            if valid_name_char(&s.fields[fo].text, c, cur) {
                s.fields[fo].type_text(&c.to_string());
            }
        }
    } else {
        edit_apply(&mut s.fields[fo], &rest, typed, &mut st.clip_out);
    }
    let caret = ((now_ms - start) / 300) % 2 == 0;
    let mode = s.mode;
    f.center("block.minecraft.structure_block", 10.0, WHITE);
    let lbl =
        |f: &mut Frame, key: &str, x: f32, y: f32| f.t.draw(&f.lang.tr(key), x, y, GRAY, true);
    let right = |f: &mut Frame, key: &str, y: f32| {
        let t = f.lang.tr(key);
        let x = cx + 154.0 - f.t.text_width(&t);
        f.t.draw(&t, x, y, GRAY, true);
    };
    let edit = |f: &mut Frame, s: &mut StructBlock, i: usize, x: f32, y: f32, wd: f32| {
        if f.edit_box(x, y, wd, 20.0, &mut s.fields[i], s.focus == i, caret) {
            s.focus = i;
        }
    };
    if mode != 3 {
        lbl(f, "structure_block.structure_name", cx - 153.0, 30.0);
        edit(f, &mut s, NAME, cx - 152.0, 40.0, 300.0);
    }
    if mode == 0 || mode == 1 {
        lbl(f, "structure_block.position", cx - 153.0, 70.0);
        edit(f, &mut s, POS, cx - 152.0, 80.0, 80.0);
        edit(f, &mut s, POS + 1, cx - 72.0, 80.0, 80.0);
        edit(f, &mut s, POS + 2, cx + 8.0, 80.0, 80.0);
        right(f, "structure_block.include_entities", 150.0);
    }
    let onoff = |f: &Frame, v: bool| f.lang.tr(if v { "options.on" } else { "options.off" });
    let mut update = None;
    if mode == 0 || mode == 1 {
        let label = onoff(f, s.include_entities);
        if f.button(cx + 104.0, 160.0, 50.0, 20.0, &label, true) {
            s.include_entities = !s.include_entities;
        }
    }
    if mode == 0 {
        lbl(f, "structure_block.size", cx - 153.0, 110.0);
        edit(f, &mut s, SIZE, cx - 152.0, 120.0, 80.0);
        edit(f, &mut s, SIZE + 1, cx - 72.0, 120.0, 80.0);
        edit(f, &mut s, SIZE + 2, cx + 8.0, 120.0, 80.0);
        right(f, "structure_block.detect_size", 110.0);
        right(f, "structure_block.show_air", 70.0);
        if f.button(
            cx + 104.0,
            120.0,
            50.0,
            20.0,
            &f.lang.tr("structure_block.button.detect_size"),
            true,
        ) {
            update = Some(3);
        }
        let label = onoff(f, s.show_air);
        if f.button(cx + 104.0, 80.0, 50.0, 20.0, &label, true) {
            s.show_air = !s.show_air;
        }
        if f.button(
            cx + 104.0,
            185.0,
            50.0,
            20.0,
            &f.lang.tr("structure_block.button.save"),
            true,
        ) {
            update = Some(1);
        }
    }
    if mode == 1 {
        lbl(f, "structure_block.integrity", cx - 153.0, 110.0);
        edit(f, &mut s, INTEGRITY, cx - 152.0, 120.0, 80.0);
        edit(f, &mut s, SEED, cx - 72.0, 120.0, 80.0);
        right(f, "structure_block.strict", 110.0);
        right(f, "structure_block.show_boundingbox", 70.0);
        let label = onoff(f, s.strict);
        if f.button(cx + 104.0, 120.0, 50.0, 20.0, &label, true) {
            s.strict = !s.strict;
        }
        let label = onoff(f, s.show_bb);
        if f.button(cx + 104.0, 80.0, 50.0, 20.0, &label, true) {
            s.show_bb = !s.show_bb;
        }
        let key = ["none", "left_right", "front_back"][usize::from(s.mirror)];
        let label = f.lang.tr(&format!("mirror.{key}"));
        if f.button(cx - 20.0, 185.0, 40.0, 20.0, &label, true) {
            s.mirror = (s.mirror + 1) % 3;
        }
        let xs = [cx - 102.0, cx - 61.0, cx + 21.0, cx + 62.0];
        for (i, (x, name)) in xs.iter().zip(["0", "90", "180", "270"]).enumerate() {
            if f.button(*x, 185.0, 40.0, 20.0, name, s.rot != i as u8) {
                s.rot = i as u8;
            }
        }
        if f.button(
            cx + 104.0,
            185.0,
            50.0,
            20.0,
            &f.lang.tr("structure_block.button.load"),
            true,
        ) {
            update = Some(2);
        }
    }
    if mode == 3 {
        lbl(f, "structure_block.custom_data", cx - 153.0, 110.0);
        edit(f, &mut s, DATA, cx - 152.0, 120.0, 240.0);
    }
    // Кнопка режима (DATA → SAVE: индекс в списке без DATA не найден).
    let mkey = ["save", "load", "corner", "data"][usize::from(mode)];
    if f.button(
        cx - 154.0,
        185.0,
        50.0,
        20.0,
        &f.lang.tr(&format!("structure_block.mode.{mkey}")),
        true,
    ) {
        // VANILLA: DEFAULT_MODES = SAVE, LOAD, CORNER; из CORNER и DATA — SAVE.
        s.mode = if mode >= 2 { 0 } else { mode + 1 };
        s.fix_focus();
    }
    let info = f.lang.tr(&format!("structure_block.mode_info.{mkey}"));
    f.t.draw(&info, cx - 153.0, 174.0, GRAY, true);
    if f.button(cx - 154.0, 210.0, 150.0, 20.0, &f.lang.tr("gui.done"), true) {
        out = Some(s.out(0));
    }
    if f.button(cx + 4.0, 210.0, 150.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        cancel = true;
    }
    if let Some(u) = update {
        out = Some(s.out(u));
    }
    if out.is_some() {
        st.structure_out = out;
        *action = Action::CloseStructure;
    } else if cancel {
        *action = Action::CloseStructure;
    } else {
        st.structure = Some(s);
    }
}
