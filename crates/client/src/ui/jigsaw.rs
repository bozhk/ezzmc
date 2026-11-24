//! Экран блока-головоломки. VANILLA: JigsawBlockEditScreen (поля pool/name/target/final_state,
//! приоритеты, соединение (только вертикальная ориентация), слайдер уровней, keep_jigsaws,
//! Generate/Done/Cancel; Enter — Готово при валидных идентификаторах).
//! VANILLA-UNSURE: тултипы приоритетов (EditBox#setTooltip) не рисуем.

use super::{edit_apply, Action, Frame, State, UiKey, GRAY};
use crate::editbox::EditBox;
use ezzmc_protocol::nbt::Tag;

const POOL: usize = 0;
const NAME: usize = 1;
const TARGET: usize = 2;
const FINAL: usize = 3;
const SEL: usize = 4;
const PLACE: usize = 5;

pub struct Jigsaw {
    pub pos: [i32; 3],
    /// Передняя грань вертикальна (up/down) → доступно соединение.
    vertical: bool,
    /// 0 ROLLABLE, 1 ALIGNED.
    joint: u8,
    levels: i32,
    slider: f32,
    keep: bool,
    fields: [EditBox; 6],
    focus: usize,
    opened_ms: Option<u64>,
}

/// Результат для main.
pub struct JigsawOut {
    pub pos: [i32; 3],
    pub name: String,
    pub target: String,
    pub pool: String,
    pub final_state: String,
    pub joint: u8,
    pub selection: i32,
    pub placement: i32,
    /// Some((levels, keep)) — после отправки данных ещё и ServerboundJigsawGeneratePacket.
    pub generate: Option<(i32, bool)>,
}

/// VANILLA: Identifier#tryParse — `ns:path`, ns [a-z0-9_.-], path [a-z0-9_./-].
fn valid_id(s: &str) -> bool {
    let (ns, path) = s.split_once(':').unwrap_or(("minecraft", s));
    let ns_ok = ns
        .chars()
        .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '-' | '.'));
    let path_ok = path
        .chars()
        .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '-' | '.' | '/'));
    ns_ok && path_ok
}

/// VANILLA: Identifier.parse — без `:` пространство `minecraft`.
fn full_id(s: &str) -> String {
    if s.contains(':') {
        s.to_owned()
    } else {
        format!("minecraft:{s}")
    }
}

/// `orientation` — значение свойства блока (`up_north`, `east_up`, …): первая часть — front.
pub fn open(pos: [i32; 3], orientation: &str, tag: Option<&Tag>) -> Jigsaw {
    let mk = EditBox::new;
    let mut s = Jigsaw {
        pos,
        vertical: orientation.starts_with("up_") || orientation.starts_with("down_"),
        joint: 0,
        levels: 0,
        slider: 0.0,
        keep: true,
        fields: [mk(128), mk(128), mk(128), mk(256), mk(3), mk(3)],
        focus: POOL,
        opened_ms: None,
    };
    let get = |k: &str| tag.and_then(|t| t.get(k));
    let text = |k: &str, d: &str| get(k).and_then(Tag::as_str).unwrap_or(d).to_owned();
    // VANILLA: JigsawBlockEntity#loadAdditional (пустой Identifier — minecraft:empty).
    s.fields[POOL].set(&text("pool", "minecraft:empty"));
    s.fields[NAME].set(&text("name", "minecraft:empty"));
    s.fields[TARGET].set(&text("target", "minecraft:empty"));
    s.fields[FINAL].set(&text("final_state", "minecraft:air"));
    let int = |k: &str| get(k).and_then(Tag::as_i64).unwrap_or(0);
    s.fields[SEL].set(&int("selection_priority").to_string());
    s.fields[PLACE].set(&int("placement_priority").to_string());
    s.joint = match get("joint").and_then(Tag::as_str) {
        Some("aligned") => 1,
        Some(_) => 0,
        // StructureTemplate#getDefaultJointType: вертикальная ориентация → ROLLABLE, иначе ALIGNED.
        None => u8::from(!s.vertical),
    };
    s
}

impl Jigsaw {
    fn valid(&self) -> bool {
        [POOL, NAME, TARGET]
            .iter()
            .all(|&i| valid_id(&self.fields[i].text))
    }

    fn out(&self, generate: bool) -> JigsawOut {
        let num = |i: usize| self.fields[i].text.trim().parse::<i32>().unwrap_or(0);
        JigsawOut {
            pos: self.pos,
            name: full_id(&self.fields[NAME].text),
            target: full_id(&self.fields[TARGET].text),
            pool: full_id(&self.fields[POOL].text),
            final_state: self.fields[FINAL].text.clone(),
            joint: self.joint,
            selection: num(SEL),
            placement: num(PLACE),
            generate: generate.then_some((self.levels, self.keep)),
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
    let Some(mut s) = st.jigsaw.take() else {
        *action = Action::CloseJigsaw;
        return;
    };
    let start = *s.opened_ms.get_or_insert(now_ms);
    let cx = (f.w / 2.0).floor();
    let mut out: Option<JigsawOut> = None;
    let mut cancel = false;
    let mut rest: Vec<UiKey> = Vec::new();
    for k in keys {
        match k {
            UiKey::Escape => cancel = true,
            UiKey::Enter if s.valid() => out = Some(s.out(false)),
            UiKey::Enter => {}
            UiKey::Tab | UiKey::BackTab => {
                let n = s.fields.len();
                s.focus = if matches!(k, UiKey::Tab) {
                    (s.focus + 1) % n
                } else {
                    (s.focus + n - 1) % n
                };
            }
            _ => rest.push(*k),
        }
    }
    let fo = s.focus;
    edit_apply(&mut s.fields[fo], &rest, typed, &mut st.clip_out);
    let caret = ((now_ms - start) / 300) % 2 == 0;
    // VANILLA: подписи цветом -6250336 (0xFFA0A0A0).
    let lbl =
        |f: &mut Frame, key: &str, x: f32, y: f32| f.t.draw(&f.lang.tr(key), x, y, GRAY, true);
    let edit = |f: &mut Frame, s: &mut Jigsaw, i: usize, x: f32, y: f32, wd: f32| {
        if f.edit_box(x, y, wd, 20.0, &mut s.fields[i], s.focus == i, caret) {
            s.focus = i;
        }
    };
    lbl(f, "jigsaw_block.pool", cx - 153.0, 10.0);
    edit(f, &mut s, POOL, cx - 153.0, 20.0, 300.0);
    lbl(f, "jigsaw_block.name", cx - 153.0, 45.0);
    edit(f, &mut s, NAME, cx - 153.0, 55.0, 300.0);
    lbl(f, "jigsaw_block.target", cx - 153.0, 80.0);
    edit(f, &mut s, TARGET, cx - 153.0, 90.0, 300.0);
    lbl(f, "jigsaw_block.final_state", cx - 153.0, 115.0);
    edit(f, &mut s, FINAL, cx - 153.0, 125.0, 300.0);
    lbl(f, "jigsaw_block.selection_priority", cx - 153.0, 150.0);
    edit(f, &mut s, SEL, cx - 153.0, 160.0, 98.0);
    lbl(f, "jigsaw_block.placement_priority", cx - 50.0, 150.0);
    edit(f, &mut s, PLACE, cx - 50.0, 160.0, 98.0);
    if s.vertical {
        lbl(f, "jigsaw_block.joint_label", cx + 53.0, 150.0);
        let key =
            ["jigsaw_block.joint.rollable", "jigsaw_block.joint.aligned"][usize::from(s.joint)];
        if f.button(cx + 54.0, 160.0, 100.0, 20.0, &f.lang.tr(key), true) {
            s.joint = (s.joint + 1) % 2;
        }
    }
    // VANILLA: слайдер 0..20, Mth.floor(clampedLerp).
    let mut v = s.slider;
    let label = f.lang.fmt("jigsaw_block.levels", &[&s.levels.to_string()]);
    if f.slider(cx - 154.0, 185.0, 100.0, 20.0, &label, &mut v) {
        s.slider = v;
        s.levels = (v * 20.0).floor() as i32;
    }
    let keep_label = format!(
        "{}: {}",
        f.lang.tr("jigsaw_block.keep_jigsaws"),
        f.lang.tr(if s.keep { "options.on" } else { "options.off" })
    );
    if f.button(cx - 50.0, 185.0, 100.0, 20.0, &keep_label, true) {
        s.keep = !s.keep;
    }
    let valid = s.valid();
    if f.button(
        cx + 54.0,
        185.0,
        100.0,
        20.0,
        &f.lang.tr("jigsaw_block.generate"),
        valid,
    ) {
        out = Some(s.out(true));
    }
    if f.button(
        cx - 154.0,
        210.0,
        150.0,
        20.0,
        &f.lang.tr("gui.done"),
        valid,
    ) {
        out = Some(s.out(false));
    }
    if f.button(cx + 4.0, 210.0, 150.0, 20.0, &f.lang.tr("gui.cancel"), true) {
        cancel = true;
    }
    if out.is_some() {
        st.jigsaw_out = out;
        *action = Action::CloseJigsaw;
    } else if cancel {
        *action = Action::CloseJigsaw;
    } else {
        st.jigsaw = Some(s);
    }
}
