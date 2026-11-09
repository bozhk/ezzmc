//! Иконки предметов для GUI: `items/<имя>.json` → модель → плоская текстура (builtin/generated)
//! или программный рендер блочной модели (gui-трансформ, диффузный свет). TODO-234.
use ezzmc_assets::atlas::Identifier;
use ezzmc_assets::bake::{bake_elements, ModelState, SpriteGetter, SpriteUv};
use ezzmc_assets::model::{DisplayContext, GuiLight, ModelLoader};
use ezzmc_assets::pack::ResourceManager;
use ezzmc_assets::texture::Image;
use glam::{EulerRot, Mat3, Vec3};
use serde_json::Value;
use std::collections::HashMap;

/// RGBA8-иконка.
pub struct Icon {
    pub rgba: Vec<u8>,
    pub w: u32,
    pub h: u32,
}

const RENDER: usize = 512;
const OUT: usize = 128;

struct AnySprite;
impl SpriteGetter for AnySprite {
    fn get(&self, _id: &Identifier) -> Option<SpriteUv> {
        Some(SpriteUv {
            u0: 0.0,
            v0: 0.0,
            u1: 1.0,
            v1: 1.0,
        })
    }
    fn missing(&self) -> SpriteUv {
        SpriteUv {
            u0: 0.0,
            v0: 0.0,
            u1: 1.0,
            v1: 1.0,
        }
    }
}

/// Контекст стека для выбора модели и оттенков.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Ctx {
    /// Цвет зелья (`potion_contents`).
    pub potion: Option<u32>,
    /// `dyed_color`.
    pub dye: Option<u32>,
    /// Кадр часов (`minecraft:time`, 0..=64). Заполняется `dynamic`.
    pub clock: Option<u8>,
    /// Кадр компаса (`minecraft:compass`, 0..=32). Заполняется `dynamic`.
    pub compass: Option<u8>,
    /// `lodestone_tracker` есть; внутри — цель (x, z) в текущем измерении.
    pub lode: Option<Option<(i32, i32)>>,
}

/// Дополнить контекст динамическими свойствами предмета по имени (часы/компасы).
// VANILLA: ItemModels `minecraft:time` / `minecraft:compass` (RangeSelectItemModel).
pub fn dynamic(name: &str, mut cx: Ctx) -> Ctx {
    use crate::itemdyn::{compass_entry, Target};
    match name {
        "clock" => cx.clock = Some(crate::itemdyn::clock_entry()),
        "compass" => {
            let t = match cx.lode {
                Some(Some((x, z))) => Target::Lodestone(x, z),
                Some(None) => Target::None,
                None => Target::Spawn,
            };
            cx.compass = Some(compass_entry(t));
        }
        "recovery_compass" => cx.compass = Some(compass_entry(Target::Recovery)),
        _ => {}
    }
    cx
}

/// Цвета зелий 26.3 (`имя rrggbb`), выгрузка `tools/DumpPotionColors.java`.
static POTION_COLORS: &str = include_str!("../data/potion_colors.txt");

fn read_varint(b: &[u8], i: &mut usize) -> Option<u32> {
    let (mut v, mut sh) = (0u32, 0);
    loop {
        let x = *b.get(*i)?;
        *i += 1;
        v |= u32::from(x & 0x7F) << sh;
        if x & 0x80 == 0 {
            return Some(v);
        }
        sh += 7;
        if sh > 28 {
            return None;
        }
    }
}

/// Контекст иконки из патча компонентов стека.
// VANILLA: PotionContents#getColor — custom_color, иначе цвет зелья, иначе вода (-13083194).
pub fn ctx_of(patch: &ezzmc_protocol::item::ComponentPatch) -> Ctx {
    use ezzmc_protocol::generated::r26_3::{DATA_COMPONENT_TYPE, POTION};
    let find = |n: &str| {
        let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == n)? as u32;
        patch.added.iter().find(|(i, _)| *i == id).map(|(_, b)| b)
    };
    let mut cx = Ctx::default();
    if let Some(b) = find("dyed_color") {
        if b.len() >= 4 {
            cx.dye = Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]) & 0xFF_FFFF);
        }
    }
    if let Some(b) = find("potion_contents") {
        let mut i = 1;
        let mut col = None;
        if b.first() == Some(&1) {
            let id = read_varint(b, &mut i);
            col = id.and_then(|id| POTION.get(id as usize)).and_then(|n| {
                POTION_COLORS.lines().find_map(|l| {
                    let (k, v) = l.split_once(' ')?;
                    (k == *n).then(|| u32::from_str_radix(v.trim(), 16).ok())?
                })
            });
        }
        if b.get(i) == Some(&1) && b.len() >= i + 5 {
            let c = &b[i + 1..i + 5];
            col = Some(u32::from_be_bytes([c[0], c[1], c[2], c[3]]) & 0xFF_FFFF);
        }
        cx.potion = col;
    }
    if let Some(b) = find("lodestone_tracker") {
        // Optional<GlobalPos>: bool, dimension (string), BlockPos (packed long); затем tracked.
        let mut target = None;
        if b.first() == Some(&1) {
            let mut i = 1;
            if let Some(n) = read_varint(b, &mut i) {
                let n = n as usize;
                if let (Some(d), Some(l)) = (b.get(i..i + n), b.get(i + n..i + n + 8)) {
                    let v = i64::from_be_bytes([l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7]]);
                    if d == crate::itemdyn::dimension().as_bytes() {
                        target = Some(((v >> 38) as i32, ((v << 26) >> 38) as i32));
                    }
                }
            }
        }
        cx.lode = Some(target);
    }
    cx
}

pub(crate) type Layer = (String, Vec<u32>);

/// VANILLA: ItemModel-дерево (net.minecraft.client.renderer.item.*) для display_context=gui.
/// Свойства, которых нет у иконки (рука, ручка, использование), считаются «ложными»/нулевыми.
pub(crate) fn eval(v: &Value, cx: &Ctx, dc: &str, out: &mut Vec<Layer>) {
    let Some(obj) = v.as_object() else { return };
    let ty = obj.get("type").and_then(Value::as_str).unwrap_or("");
    let ty = ty.trim_start_matches("minecraft:");
    match ty {
        "empty" => {}
        "composite" => {
            for m in obj
                .get("models")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                eval(m, cx, dc, out);
            }
        }
        "select" => {
            let prop = obj.get("property").and_then(Value::as_str).unwrap_or("");
            let hit = obj.get("cases").and_then(Value::as_array).and_then(|cs| {
                cs.iter().find(|c| {
                    c.get("when").is_some_and(|w| {
                        let one = |x: &Value| {
                            x.as_str()
                                .is_some_and(|s| prop.ends_with("display_context") && s == dc)
                        };
                        w.as_array().map_or_else(|| one(w), |a| a.iter().any(one))
                    })
                })
            });
            if let Some(c) = hit.and_then(|c| c.get("model")) {
                eval(c, cx, dc, out);
            } else if let Some(f) = obj.get("fallback") {
                eval(f, cx, dc, out);
            }
        }
        "condition" => {
            let prop = obj.get("property").and_then(Value::as_str).unwrap_or("");
            let on = prop.ends_with("has_component")
                && obj
                    .get("component")
                    .and_then(Value::as_str)
                    .is_some_and(|c| c.ends_with("lodestone_tracker") && cx.lode.is_some());
            if let Some(f) = obj.get(if on { "on_true" } else { "on_false" }) {
                eval(f, cx, dc, out);
            }
        }
        "range_dispatch" => {
            let prop = obj.get("property").and_then(Value::as_str).unwrap_or("");
            // Значение — индекс кадра: threshold k-0.5 ≤ k < (k+1)-0.5.
            let val = match prop.trim_start_matches("minecraft:") {
                "time" => cx.clock.map_or(0.0, f64::from),
                "compass" => cx.compass.map_or(0.0, f64::from),
                _ => 0.0,
            };
            let hit = obj.get("entries").and_then(Value::as_array).and_then(|es| {
                es.iter()
                    .rev()
                    .find(|e| e.get("threshold").and_then(Value::as_f64).unwrap_or(0.0) <= val)
            });
            if let Some(m) = hit.and_then(|e| e.get("model")) {
                eval(m, cx, dc, out);
            } else if let Some(f) = obj.get("fallback") {
                eval(f, cx, dc, out);
            }
        }
        _ => {
            if let Some(m) = obj
                .get("model")
                .and_then(Value::as_str)
                .or_else(|| obj.get("base").and_then(Value::as_str))
            {
                let tints = obj
                    .get("tints")
                    .and_then(Value::as_array)
                    .map(|a| a.iter().map(|t| tint_color(t, cx)).collect())
                    .unwrap_or_default();
                out.push((m.to_owned(), tints));
            }
        }
    }
}

// VANILLA-UNSURE: биомные tints (трава/листва) — цвета равнин по умолчанию.
pub(crate) fn tint_color(t: &Value, cx: &Ctx) -> u32 {
    let ty = t.get("type").and_then(Value::as_str).unwrap_or("");
    match ty {
        "minecraft:constant" => t
            .get("value")
            .and_then(Value::as_i64)
            .map_or(0xFFFFFF, |c| c as u32 & 0xFFFFFF),
        "minecraft:grass" => 0x91BD59,
        "minecraft:foliage" => 0x77AB2F,
        "minecraft:dry_foliage" => 0x8A6C43,
        "minecraft:potion" if cx.potion.is_some() => cx.potion.unwrap_or(0),
        "minecraft:dye" if cx.dye.is_some() => cx.dye.unwrap_or(0),
        "minecraft:firework"
        | "minecraft:potion"
        | "minecraft:map_color"
        | "minecraft:custom_model_data" => t
            .get("default")
            .and_then(Value::as_i64)
            .map_or(0xFFFFFF, |c| c as u32 & 0xFFFFFF),
        _ => t
            .get("default")
            .and_then(Value::as_i64)
            .map_or(0xFFFFFF, |c| c as u32 & 0xFFFFFF),
    }
}

pub(crate) fn load_tex(
    rm: &ResourceManager,
    id: &Identifier,
    cache: &mut HashMap<Identifier, Option<Image>>,
) {
    cache.entry(id.clone()).or_insert_with(|| {
        let d = rm.read(&format!("assets/{}/textures/{}.png", id.namespace, id.path))?;
        let img = Image::decode_png(&d).ok()?;
        // анимированные полосы: первый кадр
        let side = img.width;
        Some(if img.height > side {
            img.crop(0, 0, side, side)
        } else {
            img
        })
    });
}

fn tint_px(px: u32, tint: u32) -> u32 {
    let ch = |p: u32, t: u32, s: u32| (((p >> s) & 255) * ((t >> s) & 255) / 255) << s;
    (px & 0xFF00_0000) | ch(px, tint, 16) | ch(px, tint, 8) | ch(px, tint, 0)
}

/// Собрать иконку предмета; `None` — нет json/модели/текстур.
pub fn bake(rm: &ResourceManager, name: &str, cx: &Ctx) -> Option<Icon> {
    let json: Value =
        serde_json::from_slice(&rm.read(&format!("assets/minecraft/items/{name}.json"))?).ok()?;
    let mut layers = Vec::new();
    eval(json.get("model")?, cx, "gui", &mut layers);
    let mut out: Option<Icon> = None;
    for (model, tints) in layers {
        let Some(ic) = bake_one(rm, name, &model, &tints) else {
            continue;
        };
        out = Some(match out {
            None => ic,
            Some(base) => overlay(base, ic),
        });
    }
    out
}

/// Наложить `top` на `base` (меньшую сторону растягиваем до большей, nearest).
fn overlay(base: Icon, top: Icon) -> Icon {
    let (w, h) = (base.w.max(top.w), base.h.max(top.h));
    let fit = |i: &Icon| -> Vec<u8> {
        if i.w == w && i.h == h {
            return i.rgba.clone();
        }
        let mut v = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let (sx, sy) = (x * i.w / w, y * i.h / h);
                let (o, so) = (((y * w + x) * 4) as usize, ((sy * i.w + sx) * 4) as usize);
                v[o..o + 4].copy_from_slice(&i.rgba[so..so + 4]);
            }
        }
        v
    };
    let (mut b, t) = (fit(&base), fit(&top));
    for (d, s) in b.chunks_mut(4).zip(t.chunks(4)) {
        let a = u32::from(s[3]);
        if a == 0 {
            continue;
        }
        let da = u32::from(d[3]);
        let oa = a + da * (255 - a) / 255;
        for c in 0..3 {
            d[c] =
                ((u32::from(s[c]) * a + u32::from(d[c]) * da * (255 - a) / 255) / oa.max(1)) as u8;
        }
        d[3] = oa as u8;
    }
    Icon { rgba: b, w, h }
}

fn bake_one(rm: &ResourceManager, name: &str, model: &str, tints: &[u32]) -> Option<Icon> {
    let mid = Identifier::parse(model);
    let mut loader = ModelLoader::new(rm);
    let (resolved, builtin) = loader.resolve(&mid)?;
    let slots = resolved.texture_slots();
    let mut tex = HashMap::new();
    let elements = resolved.elements();
    if elements.is_empty() {
        // builtin/generated: слои layer0..N
        let _ = builtin;
        let mut out: Option<Icon> = None;
        for i in 0..5 {
            let Some(id) = slots.get(&format!("layer{i}")) else {
                break;
            };
            load_tex(rm, id, &mut tex);
            let Some(Some(img)) = tex.get(id) else {
                continue;
            };
            let tint = tints.get(i).copied().unwrap_or(0xFFFFFF);
            let rgba = img.to_rgba8();
            let o = out.get_or_insert_with(|| Icon {
                rgba: vec![0; rgba.len()],
                w: img.width,
                h: img.height,
            });
            if rgba.len() != o.rgba.len() {
                continue;
            }
            for (d, s) in o.rgba.chunks_mut(4).zip(rgba.chunks(4)) {
                if s[3] == 0 {
                    continue;
                }
                let t = [(tint >> 16) as u8, (tint >> 8) as u8, tint as u8];
                for c in 0..3 {
                    d[c] = (u32::from(s[c]) * u32::from(t[c]) / 255) as u8;
                }
                d[3] = s[3];
            }
        }
        return out;
    }
    let quads = bake_elements(elements, &slots, &AnySprite, &ModelState::default(), name);
    let t = resolved.transform(DisplayContext::Gui);
    let rot = Mat3::from_euler(
        EulerRot::XYZ,
        t.rotation[0].to_radians(),
        t.rotation[1].to_radians(),
        t.rotation[2].to_radians(),
    );
    let (sc, tr) = (Vec3::from(t.scale), Vec3::from(t.translation) / 16.0);
    let xf = |v: Vec3| rot * ((v - Vec3::splat(0.5)) * sc) + tr;
    let lit = resolved.gui_light() == GuiLight::Side;
    let s = RENDER as f32;
    let mut color = vec![0u32; RENDER * RENDER];
    let mut depth = vec![f32::MIN; RENDER * RENDER];
    let all = quads.unculled.iter().chain(quads.culled.iter().flatten());
    for q in all {
        let p: Vec<Vec3> = q.pos.iter().map(|&a| xf(Vec3::from(a))).collect();
        let n = (p[1] - p[0]).cross(p[2] - p[0]).normalize_or_zero();
        // нормаль в экранных (y вверх) координатах: порядок вершин FaceInfo против часовой
        if n.z <= 0.0 {
            continue;
        }
        load_tex(rm, &q.sprite, &mut tex);
        let Some(Some(img)) = tex.get(&q.sprite) else {
            continue;
        };
        let shade = if lit {
            // VANILLA: Lighting.Entry.ITEMS_3D — свет заранее повёрнут позой предмета
            // (scaling(1,-1,1)·rotateYXZ·rotateYXZ), нормали в шейдере с перевёрнутым Y;
            // здесь Y света уже перевёрнут под наши нормали (y вверх).
            let l0 = Vec3::new(-0.933, 0.263, -0.244);
            let l1 = Vec3::new(-0.104, 0.977, 0.188);
            (n.dot(l0).max(0.0) + n.dot(l1).max(0.0))
                .mul_add(0.6, 0.4)
                .min(1.0)
        } else {
            1.0
        };
        let tint = if q.tint_index >= 0 {
            tints
                .get(q.tint_index as usize)
                .copied()
                .unwrap_or(0xFFFFFF)
        } else {
            0xFFFFFF
        };
        let scr: Vec<(f32, f32, f32)> = p
            .iter()
            .map(|v| ((v.x + 0.5) * s, (0.5 - v.y) * s, v.z))
            .collect();
        for tri in [[0, 1, 2], [0, 2, 3]] {
            let (a, b, c) = (scr[tri[0]], scr[tri[1]], scr[tri[2]]);
            let area = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
            if area.abs() < 1e-6 {
                continue;
            }
            let minx = a.0.min(b.0).min(c.0).floor().max(0.0) as usize;
            let maxx = (a.0.max(b.0).max(c.0).ceil() as usize).min(RENDER);
            let miny = a.1.min(b.1).min(c.1).floor().max(0.0) as usize;
            let maxy = (a.1.max(b.1).max(c.1).ceil() as usize).min(RENDER);
            for y in miny..maxy {
                for x in minx..maxx {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    let w0 = ((b.0 - px) * (c.1 - py) - (b.1 - py) * (c.0 - px)) / area;
                    let w1 = ((c.0 - px) * (a.1 - py) - (c.1 - py) * (a.0 - px)) / area;
                    let w2 = 1.0 - w0 - w1;
                    if w0 < -1e-4 || w1 < -1e-4 || w2 < -1e-4 {
                        continue;
                    }
                    let z = w0 * a.2 + w1 * b.2 + w2 * c.2;
                    if z < depth[y * RENDER + x] {
                        continue;
                    }
                    let u = w0 * q.uv[tri[0]][0] + w1 * q.uv[tri[1]][0] + w2 * q.uv[tri[2]][0];
                    let v = w0 * q.uv[tri[0]][1] + w1 * q.uv[tri[1]][1] + w2 * q.uv[tri[2]][1];
                    let tx =
                        ((u * img.width as f32) as i64).clamp(0, i64::from(img.width) - 1) as u32;
                    let ty =
                        ((v * img.height as f32) as i64).clamp(0, i64::from(img.height) - 1) as u32;
                    let mut px32 = img.get(tx, ty);
                    if px32 >> 24 < 128 {
                        continue;
                    }
                    px32 = tint_px(px32, tint) | 0xFF00_0000;
                    let sh = |c: u32| ((c as f32 * shade) as u32).min(255);
                    depth[y * RENDER + x] = z;
                    color[y * RENDER + x] = 0xFF00_0000
                        | (sh((px32 >> 16) & 255) << 16)
                        | (sh((px32 >> 8) & 255) << 8)
                        | sh(px32 & 255);
                }
            }
        }
    }
    // 2×2 box-downsample до OUT
    let f = RENDER / OUT;
    let mut rgba = vec![0u8; OUT * OUT * 4];
    for y in 0..OUT {
        for x in 0..OUT {
            let (mut r, mut g, mut b, mut n) = (0u32, 0u32, 0u32, 0u32);
            for dy in 0..f {
                for dx in 0..f {
                    let c = color[(y * f + dy) * RENDER + x * f + dx];
                    if c >> 24 != 0 {
                        r += (c >> 16) & 255;
                        g += (c >> 8) & 255;
                        b += c & 255;
                        n += 1;
                    }
                }
            }
            if let (Some(rr), Some(gg), Some(bb)) =
                (r.checked_div(n), g.checked_div(n), b.checked_div(n))
            {
                let o = (y * OUT + x) * 4;
                rgba[o] = rr as u8;
                rgba[o + 1] = gg as u8;
                rgba[o + 2] = bb as u8;
                rgba[o + 3] = (n * 255 / (f * f) as u32) as u8;
            }
        }
    }
    Some(Icon {
        rgba,
        w: OUT as u32,
        h: OUT as u32,
    })
}
