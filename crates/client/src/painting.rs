//! Картины. VANILLA: PaintingRenderer#submit/extractRenderState/renderPainting (26.3),
//! Painting#recreateFromPacket (направление из data), DATA_PAINTING_VARIANT_ID (#9 после HangingEntity#DATA_DIRECTION #8; holder id+1),
//! RenderTypes.entitySolidZOffsetForward, атлас paintings (front = asset_id, back = `back`).

use ezzmc_render::particledraw::push_vertex;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use std::sync::Mutex;

/// Вариант картины: asset (путь без ns), ширина, высота (в блоках).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub asset: String,
    pub width: i32,
    pub height: i32,
}

/// Реестр `minecraft:painting_variant` сервера (порядок = id холдера − 1).
pub static PAINTING_REG: Mutex<Vec<Variant>> = Mutex::new(Vec::new());

/// Ванильные варианты 26.3 (data/minecraft/painting_variant/*.json): имя, asset, w, h.
/// Нужны, когда сервер шлёт записи без данных (known packs).
const VANILLA: &[(&str, &str, i32, i32)] = &[
    ("alban", "alban", 1, 1),
    ("aztec", "aztec", 1, 1),
    ("aztec2", "aztec2", 1, 1),
    ("backyard", "backyard", 3, 4),
    ("baroque", "baroque", 2, 2),
    ("bomb", "bomb", 1, 1),
    ("bouquet", "bouquet", 3, 3),
    ("burning_skull", "burning_skull", 4, 4),
    ("bust", "bust", 2, 2),
    ("cavebird", "cavebird", 3, 3),
    ("changing", "changing", 4, 2),
    ("cotan", "cotan", 3, 3),
    ("courbet", "courbet", 2, 1),
    ("creebet", "creebet", 2, 1),
    ("dennis", "dennis", 3, 3),
    ("donkey_kong", "donkey_kong", 4, 3),
    ("earth", "earth", 2, 2),
    ("endboss", "endboss", 3, 3),
    ("fern", "fern", 3, 3),
    ("fighters", "fighters", 4, 2),
    ("finding", "finding", 4, 2),
    ("fire", "fire", 2, 2),
    ("graham", "graham", 1, 2),
    ("humble", "humble", 2, 2),
    ("kebab", "kebab", 1, 1),
    ("lowmist", "lowmist", 4, 2),
    ("match", "match", 2, 2),
    ("meditative", "meditative", 1, 1),
    ("orb", "orb", 4, 4),
    ("owlemons", "owlemons", 3, 3),
    ("passage", "passage", 4, 2),
    ("pigscene", "pigscene", 4, 4),
    ("plant", "plant", 1, 1),
    ("pointer", "pointer", 4, 4),
    ("pond", "pond", 3, 4),
    ("pool", "pool", 2, 1),
    ("prairie_ride", "prairie_ride", 1, 2),
    ("sea", "sea", 2, 1),
    ("skeleton", "skeleton", 4, 3),
    ("skull_and_roses", "skull_and_roses", 2, 2),
    ("stage", "stage", 2, 2),
    ("sunflowers", "sunflowers", 3, 3),
    ("sunset", "sunset", 2, 1),
    ("tides", "tides", 3, 3),
    ("unpacked", "unpacked", 4, 4),
    ("void", "void", 2, 2),
    ("wanderer", "wanderer", 1, 2),
    ("wasteland", "wasteland", 1, 1),
    ("water", "water", 2, 2),
    ("wind", "wind", 2, 2),
    ("wither", "wither", 2, 2),
];

/// Заполняет реестр из registry_data: NBT записи или ванильная таблица по имени.
pub fn set_registry(entries: &[(String, Option<ezzmc_protocol::nbt::Tag>)]) {
    use ezzmc_protocol::nbt::Tag;
    let v: Vec<Variant> = entries
        .iter()
        .map(|(name, nbt)| {
            let short = name.strip_prefix("minecraft:").unwrap_or(name);
            let from_nbt = nbt.as_ref().and_then(|t| {
                let Tag::Compound(c) = t else { return None };
                let get = |k: &str| c.iter().find(|(n, _)| n == k).map(|(_, v)| v);
                let asset = match get("asset_id")? {
                    Tag::String(s) => s.clone(),
                    _ => return None,
                };
                let int = |k: &str| match get(k) {
                    Some(Tag::Int(i)) => Some(*i),
                    Some(Tag::Byte(i)) => Some(i32::from(*i)),
                    Some(Tag::Short(i)) => Some(i32::from(*i)),
                    _ => None,
                };
                Some(Variant {
                    asset: asset
                        .strip_prefix("minecraft:")
                        .unwrap_or(&asset)
                        .to_owned(),
                    width: int("width")?,
                    height: int("height")?,
                })
            });
            from_nbt.unwrap_or_else(|| {
                let (_, a, w, h) = VANILLA
                    .iter()
                    .find(|(n, ..)| *n == short)
                    .copied()
                    .unwrap_or(("", "kebab", 1, 1));
                Variant {
                    asset: a.to_owned(),
                    width: w,
                    height: h,
                }
            })
        })
        .collect();
    ezzmc_world::entities::set_painting_sizes(v.iter().map(|x| (x.width, x.height)).collect());
    if let Ok(mut g) = PAINTING_REG.lock() {
        *g = v;
    }
}

/// Ключ текстуры для `Mobs::out` (`@` — путь от textures/, интернируется).
pub fn tex_key(path: String) -> &'static str {
    static KEYS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
    let Ok(mut g) = KEYS.lock() else {
        return "@painting/back";
    };
    if let Some(k) = g.iter().find(|k| **k == path) {
        return k;
    }
    let k: &'static str = Box::leak(path.into_boxed_str());
    g.push(k);
    k
}

/// Direction#get2DDataValue из 3D (2 N, 3 S, 4 W, 5 E): S 0, W 1, N 2, E 3.
fn dir2d(d3: i32) -> i32 {
    match d3 {
        3 => 0,
        4 => 1,
        2 => 2,
        5 => 3,
        _ => 0,
    }
}

/// Блок света сегмента. VANILLA: PaintingRenderer#extractRenderState.
fn segment_pos(pos: [f64; 3], d3: i32, ox: f32, oy: f32) -> [f64; 3] {
    let (mut x, y, mut z) = (
        pos[0].floor(),
        (pos[1] + f64::from(oy)).floor(),
        pos[2].floor(),
    );
    match d3 {
        2 => x = (pos[0] + f64::from(ox)).floor(),
        4 => z = (pos[2] - f64::from(ox)).floor(),
        3 => x = (pos[0] - f64::from(ox)).floor(),
        5 => z = (pos[2] + f64::from(ox)).floor(),
        _ => {}
    }
    [x, y, z]
}

/// Квады одной картины (координаты от центра, до поворота): (лицо?, 4 × (pos, uv, нормаль), сегмент).
#[allow(clippy::type_complexity)]
pub fn quads(w: i32, h: i32) -> Vec<(bool, [([f32; 3], [f32; 2]); 4], [i32; 3], (i32, i32))> {
    let (ox, oy) = (-w as f32 / 2.0, -h as f32 / 2.0);
    let e = 0.031_25f32;
    // back-спрайт: полный, полоски 1/16 для торцов (getV(0.0625)/getU(0.0625)).
    let (bu0, bu1, bv0, bv1) = (0.0, 1.0, 0.0, 1.0);
    let (tv1, lu1) = (0.0625, 0.0625);
    let (du, dv) = (1.0 / f64::from(w), 1.0 / f64::from(h));
    let mut out = Vec::new();
    for sx in 0..w {
        for sy in 0..h {
            let x0 = ox + (sx + 1) as f32;
            let x1 = ox + sx as f32;
            let y0 = oy + (sy + 1) as f32;
            let y1 = oy + sy as f32;
            let fu0 = (du * f64::from(w - sx)) as f32;
            let fu1 = (du * f64::from(w - (sx + 1))) as f32;
            let fv0 = (dv * f64::from(h - sy)) as f32;
            let fv1 = (dv * f64::from(h - (sy + 1))) as f32;
            let seg = (sx, sy);
            out.push((
                true,
                [
                    ([x0, y1, -e], [fu1, fv0]),
                    ([x1, y1, -e], [fu0, fv0]),
                    ([x1, y0, -e], [fu0, fv1]),
                    ([x0, y0, -e], [fu1, fv1]),
                ],
                [0, 0, -1],
                seg,
            ));
            out.push((
                false,
                [
                    ([x0, y0, e], [bu1, bv0]),
                    ([x1, y0, e], [bu0, bv0]),
                    ([x1, y1, e], [bu0, bv1]),
                    ([x0, y1, e], [bu1, bv1]),
                ],
                [0, 0, 1],
                seg,
            ));
            if sy == h - 1 {
                out.push((
                    false,
                    [
                        ([x0, y0, -e], [bu0, bv0]),
                        ([x1, y0, -e], [bu1, bv0]),
                        ([x1, y0, e], [bu1, tv1]),
                        ([x0, y0, e], [bu0, tv1]),
                    ],
                    [0, 1, 0],
                    seg,
                ));
            }
            if sy == 0 {
                out.push((
                    false,
                    [
                        ([x0, y1, e], [bu0, bv0]),
                        ([x1, y1, e], [bu1, bv0]),
                        ([x1, y1, -e], [bu1, tv1]),
                        ([x0, y1, -e], [bu0, tv1]),
                    ],
                    [0, -1, 0],
                    seg,
                ));
            }
            if sx == w - 1 {
                out.push((
                    false,
                    [
                        ([x0, y0, e], [lu1, bv0]),
                        ([x0, y1, e], [lu1, bv1]),
                        ([x0, y1, -e], [bu0, bv1]),
                        ([x0, y0, -e], [bu0, bv0]),
                    ],
                    [-1, 0, 0],
                    seg,
                ));
            }
            if sx == 0 {
                out.push((
                    false,
                    [
                        ([x1, y0, -e], [lu1, bv0]),
                        ([x1, y1, -e], [lu1, bv1]),
                        ([x1, y1, e], [bu0, bv1]),
                        ([x1, y0, e], [bu0, bv0]),
                    ],
                    [1, 0, 0],
                    seg,
                ));
            }
        }
    }
    out
}

/// Все картины в радиусе 64 → геометрия в `mobs` (лицо — своя текстура, остальное — `back`).
pub fn bake(level: &Level, cam: [f64; 3], mobs: &mut crate::mobs::Mobs) {
    let Some(ty) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "painting")
    else {
        return;
    };
    let reg = match PAINTING_REG.lock() {
        Ok(g) => g.clone(),
        Err(_) => return,
    };
    let back = tex_key("@painting/back".to_owned());
    for e in level.entities.map.values() {
        if e.ty != ty {
            continue;
        }
        let Some((p, ..)) = e.painting_shape() else {
            continue;
        };
        let d = [p[0] - cam[0], p[1] - cam[1], p[2] - cam[2]];
        if d[0] * d[0] + d[1] * d[1] + d[2] * d[2] > 64.0 * 64.0 {
            continue;
        }
        let Some(v) = e
            .int_meta(9)
            .and_then(|id| usize::try_from(id - 1).ok())
            .and_then(|i| reg.get(i))
        else {
            continue;
        };
        let front = tex_key(format!("@painting/{}", v.asset));
        let rot = (180 - dir2d(e.data) * 90) as f32;
        let m = Mat4::from_translation(Vec3::new(d[0] as f32, d[1] as f32, d[2] as f32))
            * Mat4::from_rotation_y(rot.to_radians());
        let (ox, oy) = (-v.width as f32 / 2.0, -v.height as f32 / 2.0);
        for (is_front, verts, n, (sx, sy)) in quads(v.width, v.height) {
            let sp = segment_pos(p, e.data, sx as f32 + ox + 0.5, sy as f32 + oy + 0.5);
            let light = crate::particles::light_at(level, sp[0] + 0.5, sp[1] + 0.5, sp[2] + 0.5);
            let nn = m.transform_vector3(Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32));
            let s = (ezzmc_render::entitymodel::mix_light(nn) * 255.0) as u8;
            let out = mobs.out(if is_front { front } else { back });
            for (pos, uv) in verts {
                let q = m.transform_point3(Vec3::from(pos));
                push_vertex(out, q.to_array(), uv, [s, s, s, 255], light);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_counts() {
        // 1×1: лицо, тыл, верх, низ, два бока.
        assert_eq!(quads(1, 1).len(), 6);
        // 4×3: 12 лиц + 12 тылов + 4 верха + 4 низа + 3 + 3 бока.
        assert_eq!(quads(4, 3).len(), 38);
        assert_eq!(VANILLA.len(), 51);
        assert_eq!(dir2d(5), 3);
    }

    #[test]
    fn registry_fallback() {
        set_registry(&[
            ("minecraft:kebab".into(), None),
            ("minecraft:pointer".into(), None),
        ]);
        let g = PAINTING_REG.lock().unwrap();
        assert_eq!(
            g[1],
            Variant {
                asset: "pointer".into(),
                width: 4,
                height: 4
            }
        );
    }
}
