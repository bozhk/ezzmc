//! Экран командного блока. VANILLA: CommandBlockEditScreen / AbstractCommandBlockEditScreen
//! (поле команды 300×20 на y=50, «Прошлый вывод», кнопки режима/условности/триггера, Готово/Отмена;
//! Enter — Готово; данные приходят пакетом block_entity_data → `updateGui`, до этого кнопки неактивны).
//! VANILLA-UNSURE: подсказки команд (CommandSuggestions) не делаем; закрытие при исчезновении блока
//! (isValid) не делаем.

use super::{edit_apply, Action, Frame, State, UiKey, GRAY, WHITE};
use crate::editbox::EditBox;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::text::Component;

pub struct CmdBlock {
    pub pos: [i32; 3],
    /// Mode блока по типу: 0 — SEQUENCE (chain), 1 — AUTO (repeating), 2 — REDSTONE.
    mode: u8,
    conditional: bool,
    auto: bool,
    track: bool,
    /// Пришёл block_entity_data (VANILLA: enableControls(true)).
    loaded: bool,
    edit: EditBox,
    prev: String,
    opened_ms: Option<u64>,
}

/// Результат для main: отправить ServerboundSetCommandBlockPacket.
pub struct CmdOut {
    pub pos: [i32; 3],
    pub command: String,
    pub mode: u8,
    pub track: bool,
    pub conditional: bool,
    pub auto: bool,
}

/// `name` — имя блока (`command_block` / `chain_command_block` / `repeating_command_block`).
pub fn open(pos: [i32; 3], name: &str) -> CmdBlock {
    // VANILLA: CommandBlockEntity#getMode по типу блока.
    let mode = match name {
        "chain_command_block" => 0,
        "repeating_command_block" => 1,
        _ => 2,
    };
    CmdBlock {
        pos,
        mode,
        conditional: false,
        auto: false,
        track: true,
        loaded: false,
        edit: EditBox::new(32500),
        prev: "-".into(),
        opened_ms: None,
    }
}

impl CmdBlock {
    /// VANILLA: CommandBlockEditScreen#updateGui — после block_entity_data.
    pub fn update(&mut self, conditional: bool, tag: Option<&Tag>) {
        let get = |k: &str| tag.and_then(|t| t.get(k));
        self.edit
            .set(get("Command").and_then(Tag::as_str).unwrap_or(""));
        self.track = get("TrackOutput")
            .and_then(Tag::as_i64)
            .is_none_or(|v| v != 0);
        self.auto = get("auto").and_then(Tag::as_i64).is_some_and(|v| v != 0);
        self.conditional = conditional;
        self.prev = if self.track {
            get("LastOutput")
                .map(|t| match Component::from_nbt(t) {
                    Ok(c) => c.to_plain(&|_| None),
                    Err(_) => t.as_str().unwrap_or("").to_owned(),
                })
                .unwrap_or_default()
        } else {
            "-".into()
        };
        self.loaded = true;
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
    let Some(mut s) = st.cmd.take() else {
        *action = Action::CloseCmdBlock;
        return;
    };
    let start = *s.opened_ms.get_or_insert(now_ms);
    let (w, h) = (f.w, f.h);
    let mut done = false;
    let mut cancel = false;
    for k in keys {
        match k {
            UiKey::Escape => cancel = true,
            UiKey::Enter if s.loaded => done = true,
            _ => {}
        }
    }
    let rest: Vec<UiKey> = keys
        .iter()
        .filter(|k| !matches!(k, UiKey::Escape | UiKey::Enter))
        .copied()
        .collect();
    if s.loaded {
        edit_apply(&mut s.edit, &rest, typed, &mut st.clip_out);
    }
    let caret = ((now_ms - start) / 300) % 2 == 0;
    let cx = w / 2.0;
    f.center("advMode.setCommand", 20.0, WHITE);
    f.t.draw(&f.lang.tr("advMode.command"), cx - 149.0, 40.0, GRAY, true);
    f.edit_box(cx - 150.0, 50.0, 300.0, 20.0, &mut s.edit, true, caret);
    let py = 135.0;
    if !s.prev.is_empty() {
        // VANILLA: y = 75 + 5·9 + 1 + previousY − 135 → подпись на y+4.
        let y = 75.0 + 45.0 + 1.0 + py - 135.0;
        f.t.draw(
            &f.lang.tr("advMode.previousOutput"),
            cx - 149.0,
            y + 4.0,
            GRAY,
            true,
        );
        let mut prev = EditBox::new(32500);
        prev.set(&s.prev);
        f.edit_box(cx - 150.0, py, 276.0, 20.0, &mut prev, false, false);
    }
    let on = s.loaded;
    // O / X — trackOutput.
    if f.button(
        cx + 130.0,
        py,
        20.0,
        20.0,
        if s.track { "O" } else { "X" },
        on,
    ) {
        s.track = !s.track;
        s.prev = if s.track { String::new() } else { "-".into() };
    }
    let mode_key = match s.mode {
        0 => "advMode.mode.sequence",
        1 => "advMode.mode.auto",
        _ => "advMode.mode.redstone",
    };
    if f.button(cx - 154.0, 165.0, 100.0, 20.0, &f.lang.tr(mode_key), on) {
        s.mode = (s.mode + 1) % 3;
    }
    let cond_key = if s.conditional {
        "advMode.mode.conditional"
    } else {
        "advMode.mode.unconditional"
    };
    if f.button(cx - 50.0, 165.0, 100.0, 20.0, &f.lang.tr(cond_key), on) {
        s.conditional = !s.conditional;
    }
    let auto_key = if s.auto {
        "advMode.mode.autoexec.bat"
    } else {
        "advMode.mode.redstoneTriggered"
    };
    if f.button(cx + 54.0, 165.0, 100.0, 20.0, &f.lang.tr(auto_key), on) {
        s.auto = !s.auto;
    }
    let by = (h / 4.0).floor() + 132.0;
    if f.button(cx - 154.0, by, 150.0, 20.0, &f.lang.tr("gui.done"), on) {
        done = true;
    }
    if f.button(cx + 4.0, by, 150.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        cancel = true;
    }
    if done {
        st.cmd_out = Some(CmdOut {
            pos: s.pos,
            command: s.edit.text.clone(),
            mode: s.mode,
            track: s.track,
            conditional: s.conditional,
            auto: s.auto,
        });
        *action = Action::CloseCmdBlock;
    } else if cancel {
        *action = Action::CloseCmdBlock;
    } else {
        st.cmd = Some(s);
    }
}
