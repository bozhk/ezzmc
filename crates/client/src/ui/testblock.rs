//! Экран тестового блока. VANILLA: TestBlockEditScreen (режим START/LOG/FAIL/ACCEPT, поле сообщения
//! 240×20 на y=80 — скрыто при START, подсказка режима на y=174, Готово/Отмена).

use super::{edit_apply, Action, Frame, State, UiKey, GRAY, WHITE};
use crate::editbox::EditBox;
use ezzmc_protocol::nbt::Tag;

const MODES: [&str; 4] = ["start", "log", "fail", "accept"];

pub struct TestBlock {
    pub pos: [i32; 3],
    mode: u8,
    edit: EditBox,
    opened_ms: Option<u64>,
}

/// Результат для main: ServerboundSetTestBlockPacket.
pub struct TestOut {
    pub pos: [i32; 3],
    pub mode: u8,
    pub message: String,
}

pub fn open(pos: [i32; 3], tag: Option<&Tag>) -> TestBlock {
    // VANILLA: TestBlockEntity#loadAdditional (режим по умолчанию FAIL).
    let get = |k: &str| tag.and_then(|t| t.get(k)).and_then(Tag::as_str);
    let mode = MODES
        .iter()
        .position(|m| Some(*m) == get("mode"))
        .unwrap_or(2) as u8;
    let mut edit = EditBox::new(128);
    edit.set(get("message").unwrap_or(""));
    TestBlock {
        pos,
        mode,
        edit,
        opened_ms: None,
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
    let Some(mut s) = st.testblock.take() else {
        *action = Action::CloseTestBlock;
        return;
    };
    let start = *s.opened_ms.get_or_insert(now_ms);
    let cx = (f.w / 2.0).floor();
    let mut done = false;
    let mut cancel = false;
    let mut rest: Vec<UiKey> = Vec::new();
    for k in keys {
        match k {
            UiKey::Escape => cancel = true,
            UiKey::Enter => done = true,
            _ => rest.push(*k),
        }
    }
    if s.mode != 0 {
        edit_apply(&mut s.edit, &rest, typed, &mut st.clip_out);
    }
    let caret = ((now_ms - start) / 300) % 2 == 0;
    f.center("block.minecraft.test_block", 10.0, WHITE);
    if s.mode != 0 {
        f.t.draw(
            &f.lang.tr("test_block.message"),
            cx - 153.0,
            70.0,
            GRAY,
            true,
        );
        f.edit_box(cx - 152.0, 80.0, 240.0, 20.0, &mut s.edit, true, caret);
    }
    let m = MODES[usize::from(s.mode)];
    f.t.draw(
        &f.lang.tr(&format!("test_block.mode_info.{m}")),
        cx - 153.0,
        174.0,
        GRAY,
        true,
    );
    if f.button(
        cx - 154.0,
        185.0,
        50.0,
        20.0,
        &f.lang.tr(&format!("test_block.mode.{m}")),
        true,
    ) {
        s.mode = (s.mode + 1) % 4;
    }
    if f.button(cx - 154.0, 210.0, 150.0, 20.0, &f.lang.tr("gui.done"), true) {
        done = true;
    }
    if f.button(cx + 4.0, 210.0, 150.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        cancel = true;
    }
    if done {
        st.testblock_out = Some(TestOut {
            pos: s.pos,
            mode: s.mode,
            message: s.edit.text.clone(),
        });
        *action = Action::CloseTestBlock;
    } else if cancel {
        *action = Action::CloseTestBlock;
    } else {
        st.testblock = Some(s);
    }
}
