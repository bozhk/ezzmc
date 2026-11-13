//! Текст на табличках (стоячие/настенные/подвесные), обе стороны, светящийся текст и обводка.
//! VANILLA: AbstractSignRenderer#submitSignText/getDarkColor, StandingSignRenderer/HangingSignRenderer
//! (textTransformation, TEXT_OFFSET), SignText (CODEC: messages/color/has_glowing_text), DyeColor#getTextColor.
//! Доска рисуется обычной JSON-моделью блока; здесь только надпись (мировая плоскость, `TextRenderer::set_world`).
//! VANILLA-UNSURE: filtered_messages не используются (фильтрация чата отключена); при выходе строки за
//! максимальную ширину обрезаем хвост, а не переносим (Font#split → первая строка).

use crate::berscan::{block_entity_tag, in_view, Scanner};
use crate::ui::lang::Lang;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::text::Component;
use ezzmc_render::text::TextRenderer;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};

/// VANILLA: DyeColor (имя, textColor).
const TEXT_COLORS: [(&str, u32); 16] = [
    ("white", 16777215),
    ("orange", 16738335),
    ("magenta", 16711935),
    ("light_blue", 10141901),
    ("yellow", 16776960),
    ("lime", 12582656),
    ("pink", 16738740),
    ("gray", 8421504),
    ("light_gray", 13882323),
    ("cyan", 65535),
    ("purple", 10494192),
    ("blue", 255),
    ("brown", 9127187),
    ("green", 65280),
    ("red", 16711680),
    ("black", 0),
];

/// VANILLA: AbstractSignRenderer#BLACK_TEXT_OUTLINE_COLOR (-988212).
const BLACK_OUTLINE: u32 = 0x00F0_EBCC;
/// VANILLA: AbstractSignRenderer#OUTLINE_RENDER_DISTANCE = 16².
const OUTLINE_DIST_SQ: f64 = 256.0;

fn is_sign_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| s.name().ends_with("_sign")))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

/// VANILLA: SignText#getColor + DyeColor#getTextColor.
fn text_color(name: &str) -> u32 {
    TEXT_COLORS
        .iter()
        .find(|(n, _)| *n == name)
        .map_or(0, |&(_, c)| c)
}

/// VANILLA: AbstractSignRenderer#getDarkColor.
fn dark_color(color: u32, glowing: bool) -> u32 {
    if color == 0 && glowing {
        return BLACK_OUTLINE;
    }
    // ARGB.scaleRGB(color, 0.4F)
    let k = |sh: u32| ((((color >> sh) & 255) as f32 * 0.4) as u32) << sh;
    k(16) | k(8) | k(0)
}

/// VANILLA: Direction#toYRot.
fn facing_yrot(f: &str) -> Option<f32> {
    Some(match f {
        "south" => 0.0,
        "west" => 90.0,
        "north" => 180.0,
        "east" => 270.0,
        _ => return None,
    })
}

/// Одна сторона таблички, готовая к рисованию.
struct Face {
    /// Матрица текста без сдвига на позицию блока.
    local: Mat4,
    lines: [String; 4],
    glowing: bool,
    /// Основной цвет текста и цвет обводки/затемнения.
    color: u32,
    dark: u32,
    outline: bool,
    /// Яркость освещения (0..1) для не светящегося текста.
    light: f32,
}

struct Item {
    pos: [f64; 3],
    line_h: f32,
    max_w: f32,
    faces: Vec<Face>,
}

pub struct Signs {
    scan: Scanner,
    lang: Lang,
    items: Vec<Item>,
}

impl Default for Signs {
    fn default() -> Self {
        Signs {
            scan: Scanner::default(),
            lang: Lang::load(b"{}"),
            items: Vec::new(),
        }
    }
}

/// VANILLA: Standing/HangingSignRenderer#textTransformation.
fn transform(hanging: bool, wall: bool, angle: f32, front: bool) -> Mat4 {
    let rot = Mat4::from_rotation_y((-angle).to_radians());
    let flip = Mat4::from_rotation_y(180.0f32.to_radians());
    if hanging {
        let mut m = Mat4::from_translation(Vec3::new(0.5, 0.9375, 0.5))
            * rot
            * Mat4::from_translation(Vec3::new(0.0, -0.3125, 0.0));
        if !front {
            m *= flip;
        }
        let s = 0.014_062_5;
        m * Mat4::from_translation(Vec3::new(0.0, -0.32, 0.073))
            * Mat4::from_scale(Vec3::new(s, -s, s))
    } else {
        let mut m = Mat4::from_translation(Vec3::splat(0.5)) * rot;
        if wall {
            m *= Mat4::from_translation(Vec3::new(0.0, -0.3125, -0.4375));
        }
        if !front {
            m *= flip;
        }
        let s = 0.010_416_667;
        m * Mat4::from_translation(Vec3::new(0.0, 0.333_333_34, 0.046_666_667))
            * Mat4::from_scale(Vec3::new(s, -s, s))
    }
}

/// Блок-сущность → сторона: `front_text`/`back_text`. None — нет текста (все строки пустые).
fn read_side(tag: &Tag, key: &str, lang: &Lang) -> Option<([String; 4], u32, bool)> {
    let side = tag.get(key)?;
    let Some(Tag::List(msgs)) = side.get("messages") else {
        return None;
    };
    let color_name = side.get("color").and_then(Tag::as_str).unwrap_or("black");
    let glowing = side.get("has_glowing_text").and_then(Tag::as_i64) == Some(1);
    let color = text_color(color_name);
    let dark = dark_color(color, glowing);
    let mut lines: [String; 4] = Default::default();
    let mut any = false;
    for (i, m) in msgs.iter().take(4).enumerate() {
        let Ok(c) = Component::from_nbt(m) else {
            continue;
        };
        let base = if glowing { color } else { dark };
        let spans = crate::chat::flatten_styled(&c, lang, base, false);
        let s = crate::chat::spans_to_legacy(&spans, base);
        any |= spans.iter().any(|sp| !sp.text.is_empty());
        lines[i] = s;
    }
    any.then_some((lines, color, glowing))
}

/// Брайтность по уровню света (VANILLA: LightTexture, ambient-кривая v/(4-3v)).
fn brightness(level: i32) -> f32 {
    let v = level.clamp(0, 15) as f32 / 15.0;
    (v / (4.0 - 3.0 * v)).max(0.04)
}

impl Signs {
    pub fn bake(&mut self, level: &Level, cam: [f64; 3], versions: (u64, u64), sky_darken: i32) {
        self.scan.update(level, cam, versions, is_sign_id);
        self.items.clear();
        for &(p, s) in &self.scan.list {
            if !in_view(p, cam) {
                continue;
            }
            let name = s.name();
            let hanging = name.contains("hanging");
            let wall = name.contains("_wall_");
            let angle = if wall {
                match s.property("facing").and_then(facing_yrot) {
                    Some(a) => a,
                    None => continue,
                }
            } else {
                let seg: i32 = s
                    .property("rotation")
                    .and_then(|r| r.parse().ok())
                    .unwrap_or(0);
                (seg & 15) as f32 * 22.5
            };
            let Some(tag) = block_entity_tag(level, p) else {
                continue;
            };
            let (bl, sl) = crate::particles::light_at(
                level,
                f64::from(p[0]),
                f64::from(p[1]),
                f64::from(p[2]),
            );
            let light = brightness(i32::from(bl).max(i32::from(sl) - sky_darken));
            let d = |i: usize| f64::from(p[i]) + 0.5 - cam[i];
            let near = d(0) * d(0) + d(1) * d(1) + d(2) * d(2) < OUTLINE_DIST_SQ;
            let mut faces = Vec::new();
            for (key, front) in [("front_text", true), ("back_text", false)] {
                let Some((lines, color, glowing)) = read_side(tag, key, &self.lang) else {
                    continue;
                };
                faces.push(Face {
                    local: transform(hanging, wall, angle, front),
                    lines,
                    glowing,
                    color,
                    dark: dark_color(color, glowing),
                    outline: glowing && (color == 0 || near),
                    light,
                });
            }
            if faces.is_empty() {
                continue;
            }
            self.items.push(Item {
                pos: [f64::from(p[0]), f64::from(p[1]), f64::from(p[2])],
                line_h: if hanging { 9.0 } else { 10.0 },
                max_w: if hanging { 60.0 } else { 90.0 },
                faces,
            });
        }
    }

    /// Рисует надписи в слой с depth-тестом. `vp` — проекция·вид относительно `cam`, `(w, h)` — окно в px.
    pub fn draw(&self, t: &mut TextRenderer, vp: Mat4, cam: [f64; 3], (w, h): (f32, f32)) {
        for it in &self.items {
            let origin = Mat4::from_translation(Vec3::new(
                (it.pos[0] - cam[0]) as f32,
                (it.pos[1] - cam[1]) as f32,
                (it.pos[2] - cam[2]) as f32,
            ));
            for f in &it.faces {
                let mid = 4.0 * it.line_h / 2.0;
                let m = vp * origin * f.local;
                // Яркость: светящийся текст — полная (lightVal 15728880), иначе по уровню света.
                let lit = |c: u32| {
                    if f.glowing {
                        c
                    } else {
                        let k = |sh: u32| ((((c >> sh) & 255) as f32 * f.light) as u32) << sh;
                        k(16) | k(8) | k(0)
                    }
                };
                for (i, line) in f.lines.iter().enumerate() {
                    if line.is_empty() {
                        continue;
                    }
                    let line = fit(t, line, it.max_w);
                    let x = (-t.text_width(&line) / 2.0).floor();
                    let y = i as f32 * it.line_h - mid;
                    if f.outline {
                        // VANILLA: Font.DisplayMode.POLYGON_OFFSET + outline — 8 копий цветом обводки.
                        let plain = strip_codes(&line);
                        for (dx, dy) in [
                            (-1.0, -1.0),
                            (0.0, -1.0),
                            (1.0, -1.0),
                            (-1.0, 0.0),
                            (1.0, 0.0),
                            (-1.0, 1.0),
                            (0.0, 1.0),
                            (1.0, 1.0),
                        ] {
                            t.set_world(Some((m, w, h, 1.0e-5)));
                            t.draw(&plain, x + dx, y + dy, f.dark, false);
                        }
                    }
                    t.set_world(Some((m, w, h, 3.0e-5)));
                    t.draw(
                        &line,
                        x,
                        y,
                        lit(if f.glowing { f.color } else { f.dark }),
                        false,
                    );
                }
            }
        }
        t.set_world(None);
    }
}

/// Обрезает хвост, пока строка шире `max`.
fn fit(t: &TextRenderer, s: &str, max: f32) -> String {
    let mut s = s.to_owned();
    while t.text_width(&s) > max {
        s.pop();
        if s.ends_with('§') {
            s.pop();
        }
        if s.is_empty() {
            break;
        }
    }
    s
}

fn strip_codes(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '§' {
            it.next();
        } else {
            out.push(c);
        }
    }
    out
}

/// Данные для редактора таблички: (имя блока, строки, цвет текста).
/// VANILLA: AbstractSignEditScreen (sign.getText(front).getMessage(i).getString(), getColor().getTextColor()).
pub fn edit_info(level: &Level, p: [i32; 3], front: bool) -> Option<(String, [String; 4], u32)> {
    let st = level.chunks.block(p[0], p[1], p[2]);
    if !is_sign_id(st.id()) {
        return None;
    }
    let mut lines: [String; 4] = Default::default();
    let mut color = 0;
    if let Some(side) = block_entity_tag(level, p)
        .and_then(|t| t.get(if front { "front_text" } else { "back_text" }))
    {
        color = text_color(side.get("color").and_then(Tag::as_str).unwrap_or("black"));
        if let Some(Tag::List(msgs)) = side.get("messages") {
            for (i, m) in msgs.iter().take(4).enumerate() {
                if let Ok(c) = Component::from_nbt(m) {
                    lines[i] = c.to_plain(&|_| None);
                }
            }
        }
    }
    Some((st.name().to_owned(), lines, color))
}
