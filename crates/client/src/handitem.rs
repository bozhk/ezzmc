//! Предмет в руке: `items/<имя>.json` → модель → квады для 1-го лица и F5.
//! `builtin/generated` экструдируется из пикселей спрайта (толщина 1/16), блочные модели — из элементов.
use crate::itemicon::{eval, Ctx};
use ezzmc_assets::atlas::Identifier;
use ezzmc_assets::bake::{bake_elements, ModelState, SpriteGetter, SpriteUv};
use ezzmc_assets::block_atlas::BlockAtlas;
use ezzmc_assets::model::{DisplayContext, ModelLoader};
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::particledraw::push_vertex;
use glam::{EulerRot, Mat4, Quat, Vec3};
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;

struct Quad {
    pos: [[f32; 3]; 4],
    uv: [[f32; 2]; 4],
    color: [u8; 3],
    /// Цвет без освещения и нормаль (модельное пространство) — для мирового света предметов на земле.
    tint: [u8; 3],
    normal: [f32; 3],
    /// Спрайт из атласа предметов (иначе — блоков).
    items: bool,
}

/// Модель предмета в руке.
pub struct HandMesh {
    quads: Vec<Quad>,
    first_person: Mat4,
    third_person: Mat4,
    first_person_left: Mat4,
    third_person_left: Mat4,
    ground: Mat4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    FirstPerson,
    ThirdPerson,
    FirstPersonLeft,
    ThirdPersonLeft,
    /// ItemDisplayContext.GROUND — предмет-сущность.
    Ground,
}

impl HandMesh {
    /// VANILLA: ItemStackRenderState#getModelBoundingBox (контекст GROUND): (minY, размер по Z).
    pub fn ground_bounds(&self) -> (f32, f32) {
        let (mut min_y, mut min_z, mut max_z) = (f32::MAX, f32::MAX, f32::MIN);
        for q in &self.quads {
            for p in q.pos {
                let p = self.ground.transform_point3(Vec3::from(p));
                min_y = min_y.min(p.y);
                min_z = min_z.min(p.z);
                max_z = max_z.max(p.z);
            }
        }
        if self.quads.is_empty() {
            return (0.0, 0.0);
        }
        (min_y, max_z - min_z)
    }

    /// Запечь вершины: `root` — матрица руки/камеры, затем трансформ `display` модели.
    // VANILLA: ItemTransform#apply + ItemStackRenderState#render (translate −0.5).
    pub fn bake(
        &self,
        root: Mat4,
        view: View,
        light: (u8, u8),
        blocks: &mut Vec<u8>,
        items: &mut Vec<u8>,
    ) {
        let m = root
            * match view {
                View::FirstPerson => self.first_person,
                View::ThirdPerson => self.third_person,
                View::FirstPersonLeft => self.first_person_left,
                View::ThirdPersonLeft => self.third_person_left,
                View::Ground => self.ground,
            };
        // VANILLA: Lighting.Entry.LEVEL (core/entity.vsh minecraft_mix_light): max(0, dot) без abs,
        // нормаль — в мировом пространстве (предмет вращается → освещение меняется).
        let l0 = Vec3::new(0.2, 1.0, -0.7).normalize();
        let l1 = Vec3::new(-0.2, 1.0, 0.7).normalize();
        for q in &self.quads {
            let out = if q.items { &mut *items } else { &mut *blocks };
            let c = if view == View::Ground {
                let n = m
                    .transform_vector3(Vec3::from(q.normal))
                    .normalize_or_zero();
                let k = ((n.dot(l0).max(0.0) + n.dot(l1).max(0.0)) * 0.6 + 0.4).min(1.0);
                let f = |v: u8| (f32::from(v) * k) as u8;
                [f(q.tint[0]), f(q.tint[1]), f(q.tint[2]), 255]
            } else {
                [q.color[0], q.color[1], q.color[2], 255]
            };
            for i in 0..4 {
                let p = m.transform_point3(Vec3::from(q.pos[i]));
                push_vertex(out, p.to_array(), q.uv[i], c, light);
            }
        }
    }
}

/// Кэш моделей и атласы спрайтов.
pub struct HandItems {
    blocks: HashMap<Identifier, SpriteUv>,
    items: BlockAtlas,
    cache: HashMap<(String, Ctx), Option<Rc<HandMesh>>>,
}

struct Getter<'a> {
    items: &'a HashMap<Identifier, SpriteUv>,
    blocks: &'a HashMap<Identifier, SpriteUv>,
}
impl SpriteGetter for Getter<'_> {
    fn get(&self, id: &Identifier) -> Option<SpriteUv> {
        self.items.get(id).or_else(|| self.blocks.get(id)).copied()
    }
    fn missing(&self) -> SpriteUv {
        self.blocks
            .get(&Identifier::new("minecraft", "missingno"))
            .copied()
            .unwrap_or(SpriteUv {
                u0: 0.0,
                v0: 0.0,
                u1: 1.0,
                v1: 1.0,
            })
    }
}

impl HandItems {
    pub fn new(rm: &ResourceManager, blocks: &HashMap<Identifier, SpriteUv>) -> Option<Self> {
        let items = BlockAtlas::build_named(rm, "items", 0)
            .map_err(|e| tracing::warn!("атлас предметов: {e}"))
            .ok()?;
        Some(Self {
            blocks: blocks.clone(),
            items,
            cache: HashMap::new(),
        })
    }

    /// RGBA8 атласа предметов (ширина, высота).
    pub fn items_rgba(&self) -> (Vec<u8>, u32, u32) {
        (
            self.items.mips[0].to_rgba8(),
            self.items.width,
            self.items.height,
        )
    }

    pub fn get(&mut self, rm: &ResourceManager, name: &str, cx: &Ctx) -> Option<Rc<HandMesh>> {
        let key = (name.to_owned(), *cx);
        if let Some(m) = self.cache.get(&key) {
            return m.clone();
        }
        let m = self.build(rm, name, cx).map(Rc::new);
        self.cache.insert(key, m.clone());
        m
    }

    fn build(&self, rm: &ResourceManager, name: &str, cx: &Ctx) -> Option<HandMesh> {
        let build_ctx = |dc: DisplayContext| -> Option<(Vec<Quad>, Mat4)> {
            let json: Value =
                serde_json::from_slice(&rm.read(&format!("assets/minecraft/items/{name}.json"))?)
                    .ok()?;
            let mut layers = Vec::new();
            eval(json.get("model")?, cx, dc.serialized_name(), &mut layers);
            let mut quads = Vec::new();
            let mut xf = None;
            for (model, tints) in layers {
                let mut loader = ModelLoader::new(rm);
                let Some((resolved, _)) = loader.resolve(&Identifier::parse(&model)) else {
                    continue;
                };
                let t = resolved.transform(dc);
                xf.get_or_insert_with(|| {
                    // VANILLA: ItemTransform#apply (translation уже ×1/16; левая рука: −x, −rotY, −rotZ).
                    let (tx, ry, rz) = if dc.left_hand() {
                        (-t.translation[0], -t.rotation[1], -t.rotation[2])
                    } else {
                        (t.translation[0], t.rotation[1], t.rotation[2])
                    };
                    let rot = Quat::from_euler(
                        EulerRot::XYZ,
                        t.rotation[0].to_radians(),
                        ry.to_radians(),
                        rz.to_radians(),
                    );
                    Mat4::from_translation(Vec3::new(tx, t.translation[1], t.translation[2]))
                        * Mat4::from_quat(rot)
                        * Mat4::from_scale(Vec3::from(t.scale))
                        * Mat4::from_translation(Vec3::splat(-0.5))
                });
                let slots = resolved.texture_slots();
                let elements = resolved.elements();
                if elements.is_empty() {
                    for i in 0..5 {
                        let Some(id) = slots.get(&format!("layer{i}")) else {
                            break;
                        };
                        let tint = tints.get(i).copied().unwrap_or(0xFF_FFFF);
                        self.extrude(id, tint, i as f32, &mut quads);
                    }
                } else {
                    let g = Getter {
                        items: &self.items.sprites,
                        blocks: &self.blocks,
                    };
                    let baked = bake_elements(elements, &slots, &g, &ModelState::default(), name);
                    for q in baked.unculled.iter().chain(baked.culled.iter().flatten()) {
                        let tint = if q.tint_index >= 0 {
                            tints
                                .get(q.tint_index as usize)
                                .copied()
                                .unwrap_or(0xFF_FFFF)
                        } else {
                            0xFF_FFFF
                        };
                        let a = Vec3::from(q.pos[1]) - Vec3::from(q.pos[0]);
                        let b = Vec3::from(q.pos[2]) - Vec3::from(q.pos[0]);
                        let n = a.cross(b).normalize_or_zero();
                        // VANILLA-UNSURE: освещение 3D-предметов — формулой GUI (Side) по нормали модели.
                        let l0 = Vec3::new(-0.933, 0.263, -0.244);
                        let l1 = Vec3::new(-0.104, 0.977, 0.188);
                        let shade = ((n.dot(l0).abs().max(n.dot(l1).abs())) * 0.6 + 0.4).min(1.0);
                        quads.push(Quad {
                            pos: q.pos,
                            uv: q.uv,
                            color: tint_rgb(tint, shade),
                            tint: tint_rgb(tint, 1.0),
                            normal: n.to_array(),
                            items: self.items.sprites.contains_key(&q.sprite),
                        });
                    }
                }
            }
            Some((quads, xf?))
        };
        let (quads, first_person) = build_ctx(DisplayContext::FirstPersonRightHand)?;
        let third_person =
            build_ctx(DisplayContext::ThirdPersonRightHand).map_or(first_person, |(_, m)| m);
        let pick = |dc| build_ctx(dc).map_or(first_person, |(_, m)| m);
        Some(HandMesh {
            quads,
            first_person,
            third_person,
            first_person_left: pick(DisplayContext::FirstPersonLeftHand),
            third_person_left: pick(DisplayContext::ThirdPersonLeftHand),
            ground: pick(DisplayContext::Ground),
        })
    }

    /// VANILLA: ItemModelGenerator — две грани z=7.5/8.5 и боковые полоски по краю непрозрачных пикселей.
    fn extrude(&self, id: &Identifier, tint: u32, layer: f32, out: &mut Vec<Quad>) {
        let Some(uv) = self.items.sprites.get(id).copied() else {
            return;
        };
        let aw = self.items.width as f32;
        let ah = self.items.height as f32;
        let (x0, y0) = ((uv.u0 * aw).round() as u32, (uv.v0 * ah).round() as u32);
        let (w, h) = (
            ((uv.u1 - uv.u0) * aw).round() as u32,
            ((uv.v1 - uv.v0) * ah).round() as u32,
        );
        if w == 0 || h == 0 {
            return;
        }
        let img = &self.items.mips[0];
        let opaque = |px: i32, py: i32| {
            px >= 0
                && py >= 0
                && (px as u32) < w
                && (py as u32) < h
                && img.get(x0 + px as u32, y0 + py as u32) >> 24 != 0
        };
        let color = tint_rgb(tint, 1.0);
        let eps = layer * 0.0005;
        let (zb, zf) = (7.5 / 16.0 - eps, 8.5 / 16.0 + eps);
        let items = true;
        // Грани: в координатах модели X вправо, Y вверх.
        let face = |z: f32, flip: bool, nz: f32| {
            let (ua, ub) = if flip { (uv.u1, uv.u0) } else { (uv.u0, uv.u1) };
            Quad {
                pos: [[0.0, 0.0, z], [1.0, 0.0, z], [1.0, 1.0, z], [0.0, 1.0, z]],
                uv: [[ua, uv.v1], [ub, uv.v1], [ub, uv.v0], [ua, uv.v0]],
                color,
                tint: color,
                normal: [0.0, 0.0, nz],
                items,
            }
        };
        out.push(face(zf, false, 1.0));
        out.push(face(zb, false, -1.0));
        let (fx, fy) = (1.0 / w as f32, 1.0 / h as f32);
        let du = (uv.u1 - uv.u0) / w as f32;
        let dv = (uv.v1 - uv.v0) / h as f32;
        for py in 0..h as i32 {
            for px in 0..w as i32 {
                if !opaque(px, py) {
                    continue;
                }
                let u = uv.u0 + (px as f32 + 0.5) * du;
                let v = uv.v0 + (py as f32 + 0.5) * dv;
                let (xl, xr) = (px as f32 * fx, (px + 1) as f32 * fx);
                let (yt, yb) = (1.0 - py as f32 * fy, 1.0 - (py + 1) as f32 * fy);
                let mut side = |a: [f32; 2], b: [f32; 2], normal: [f32; 3]| {
                    out.push(Quad {
                        pos: [
                            [a[0], a[1], zb],
                            [b[0], b[1], zb],
                            [b[0], b[1], zf],
                            [a[0], a[1], zf],
                        ],
                        uv: [[u, v]; 4],
                        color,
                        tint: color,
                        normal,
                        items,
                    });
                };
                if !opaque(px - 1, py) {
                    side([xl, yb], [xl, yt], [-1.0, 0.0, 0.0]);
                }
                if !opaque(px + 1, py) {
                    side([xr, yb], [xr, yt], [1.0, 0.0, 0.0]);
                }
                if !opaque(px, py - 1) {
                    side([xl, yt], [xr, yt], [0.0, 1.0, 0.0]);
                }
                if !opaque(px, py + 1) {
                    side([xl, yb], [xr, yb], [0.0, -1.0, 0.0]);
                }
            }
        }
    }
}

fn tint_rgb(tint: u32, shade: f32) -> [u8; 3] {
    let c = |s: u32| (((tint >> s) & 255) as f32 * shade) as u8;
    [c(16), c(8), c(0)]
}

/// Имя предмета и контекст для стека (часы/компасы динамические).
pub fn stack_ctx(s: &ezzmc_protocol::item::ItemStack) -> Option<(&'static str, Ctx)> {
    let name = ezzmc_protocol::generated::r26_3::ITEM.get(s.item as usize)?;
    Some((
        name,
        crate::itemicon::dynamic(name, crate::itemicon::ctx_of(&s.patch)),
    ))
}
