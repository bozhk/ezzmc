//! Предмет в руке: `items/<имя>.json` → модель → квады для 1-го лица и F5.
//! `builtin/generated` экструдируется из пикселей спрайта (толщина 1/16), блочные модели — из элементов.
use crate::itemicon::{eval, Ctx, CHEST_MARK, SHULKER_MARK, SKULL_MARK};
use ezzmc_assets::atlas::Identifier;
use ezzmc_assets::bake::{bake_elements, ModelState, SpriteGetter, SpriteUv};
use ezzmc_assets::block_atlas::BlockAtlas;
use ezzmc_assets::model::{DisplayContext, ModelLoader};
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::particledraw::push_vertex_raw;
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

/// Квад предмета в руке для превью (см. `HandMesh::raster`).
pub struct RasterItemQuad {
    pub pos: [Vec3; 4],
    pub uv: [[f32; 2]; 4],
    pub color: [u8; 3],
    pub items: bool,
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

    /// Матрица контекста GROUND (для экземпляра GPU-инстансинга: root * ground).
    pub fn ground_matrix(&self) -> Mat4 {
        self.ground
    }

    /// Статическая сетка для GPU-инстансинга: вершины (36 Б: pos, uv, нормаль, tint u8x4) квадов
    /// атласа блоков и атласа предметов раздельно; индексы 0,1,2,0,2,3 на квад считает рисовальщик.
    pub fn static_mesh(&self) -> (Vec<u8>, Vec<u8>) {
        let (mut blocks, mut items) = (Vec::new(), Vec::new());
        for q in &self.quads {
            let out = if q.items { &mut items } else { &mut blocks };
            for i in 0..4 {
                for v in q.pos[i].iter().chain(q.uv[i].iter()).chain(q.normal.iter()) {
                    out.extend_from_slice(&v.to_le_bytes());
                }
                out.extend_from_slice(&[q.tint[0], q.tint[1], q.tint[2], 255]);
            }
        }
        (blocks, items)
    }

    pub fn quad_count(&self) -> usize {
        self.quads.len()
    }

    /// Квады для программной растеризации превью: (вершины, UV, цвет, из атласа предметов).
    pub fn raster(&self, root: Mat4, view: View) -> Vec<RasterItemQuad> {
        let m = root
            * match view {
                View::ThirdPersonLeft => self.third_person_left,
                _ => self.third_person,
            };
        self.quads
            .iter()
            .map(|q| RasterItemQuad {
                pos: q.pos.map(|p| m.transform_point3(Vec3::from(p))),
                uv: q.uv,
                color: q.color,
                items: q.items,
            })
            .collect()
    }

    /// Запечь вершины: `root` — матрица руки/камеры, затем трансформ `display` модели.
    // VANILLA: ItemTransform#apply + ItemStackRenderState#render (translate −0.5).
    pub fn bake(
        &self,
        root: Mat4,
        view: View,
        light: (u8, u8),
        foil: bool,
        blocks: &mut Vec<u8>,
        items: &mut Vec<u8>,
    ) {
        let lt = (
            u16::from(light.0) * 16
                + if foil {
                    ezzmc_render::particledraw::FOIL_FLAG as u16
                } else {
                    0
                },
            u16::from(light.1) * 16,
        );
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
        // Нормаль — через обратную транспонированную (как normal matrix в entity.vsh), иначе неравномерный масштаб искажает свет.
        let normal_m = m.inverse().transpose();
        for q in &self.quads {
            let out = if q.items { &mut *items } else { &mut *blocks };
            // VANILLA: все предметы в руке и на земле идут через LEVEL-освещение (minecraft_mix_light по нормали
            // в пространстве вида). Плоские экструдированные грани без затенения давали яркий «дефолт» в руке.
            let n = normal_m
                .transform_vector3(Vec3::from(q.normal))
                .normalize_or_zero();
            let k = ((n.dot(l0).max(0.0) + n.dot(l1).max(0.0)) * 0.6 + 0.4).min(1.0);
            let f = |v: u8| (f32::from(v) * k) as u8;
            let c = [f(q.tint[0]), f(q.tint[1]), f(q.tint[2]), 255];
            for i in 0..4 {
                let p = m.transform_point3(Vec3::from(q.pos[i]));
                push_vertex_raw(out, p.to_array(), q.uv[i], c, lt);
            }
        }
    }
}

/// Дописать `extra` под `base` (ширина — максимум), пересчитав UV обоих атласов.
/// Дописывает картинки стопкой под атлас; UV старых спрайтов пересчитываются.
fn append_images(base: &mut BlockAtlas, imgs: &[(Identifier, ezzmc_assets::texture::Image)]) {
    if imgs.is_empty() {
        return;
    }
    let w = imgs.iter().map(|(_, i)| i.width).fold(base.width, u32::max);
    let h = base.height + imgs.iter().map(|(_, i)| i.height).sum::<u32>();
    let mut img = ezzmc_assets::texture::Image::new(w, h);
    for y in 0..base.height {
        for x in 0..base.width {
            img.set(x, y, base.mips[0].get(x, y));
        }
    }
    let (wf, hf) = (w as f32, h as f32);
    let (bw, bh) = (base.width as f32, base.height as f32);
    for uv in base.sprites.values_mut() {
        uv.u0 = uv.u0 * bw / wf;
        uv.u1 = uv.u1 * bw / wf;
        uv.v0 = uv.v0 * bh / hf;
        uv.v1 = uv.v1 * bh / hf;
    }
    let mut y0 = base.height;
    for (id, im) in imgs {
        for y in 0..im.height {
            for x in 0..im.width {
                img.set(x, y0 + y, im.get(x, y));
            }
        }
        base.sprites.insert(
            id.clone(),
            SpriteUv {
                u0: 0.0,
                u1: im.width as f32 / wf,
                v0: y0 as f32 / hf,
                v1: (y0 + im.height) as f32 / hf,
            },
        );
        y0 += im.height;
    }
    base.width = w;
    base.height = h;
    base.mips = vec![img];
}

fn append_atlas(base: &mut BlockAtlas, extra: &BlockAtlas) {
    let (w, h) = (base.width.max(extra.width), base.height + extra.height);
    let mut img = ezzmc_assets::texture::Image::new(w, h);
    for y in 0..base.height {
        for x in 0..base.width {
            img.set(x, y, base.mips[0].get(x, y));
        }
    }
    for y in 0..extra.height {
        for x in 0..extra.width {
            img.set(x, base.height + y, extra.mips[0].get(x, y));
        }
    }
    let (wf, hf) = (w as f32, h as f32);
    let (bw, bh) = (base.width as f32, base.height as f32);
    let (ew, eh) = (extra.width as f32, extra.height as f32);
    for uv in base.sprites.values_mut() {
        uv.u0 = uv.u0 * bw / wf;
        uv.u1 = uv.u1 * bw / wf;
        uv.v0 = uv.v0 * bh / hf;
        uv.v1 = uv.v1 * bh / hf;
    }
    for (id, uv) in &extra.sprites {
        base.sprites.insert(
            id.clone(),
            SpriteUv {
                u0: uv.u0 * ew / wf,
                u1: uv.u1 * ew / wf,
                v0: (bh + uv.v0 * eh) / hf,
                v1: (bh + uv.v1 * eh) / hf,
            },
        );
    }
    base.width = w;
    base.height = h;
    base.mips = vec![img];
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
        let mut items = BlockAtlas::build_named(rm, "items", 0)
            .map_err(|e| tracing::warn!("атлас предметов: {e}"))
            .ok()?;
        // Спрайты сундуков (`atlases/chests.json`, ChestModel) — в один атлас с предметами,
        // чтобы рисовать их теми же буферами руки/земли.
        if let Ok(chests) = BlockAtlas::build_named(rm, "chests", 0) {
            append_atlas(&mut items, &chests);
        }
        // Текстуры голов (SkullModel) — тоже в атлас предметов.
        let mut skins = Vec::new();
        for t in crate::skull::item_textures().chain(crate::shulkerbox::item_textures()) {
            if let Some(img) = rm
                .read(&format!("assets/minecraft/textures/entity/{t}.png"))
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
            {
                skins.push((Identifier::new("minecraft", &format!("entity/{t}")), img));
            }
        }
        append_images(&mut items, &skins);
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
            for (model, tints, off) in layers {
                let chest = model.strip_prefix(CHEST_MARK).map(str::to_owned);
                let skull = model.strip_prefix(SKULL_MARK).map(str::to_owned);
                let shulker = model.strip_prefix(SHULKER_MARK).map(str::to_owned);
                let mut loader = ModelLoader::new(rm);
                // Сундук: display-трансформы берутся из `item/chest` (template_chest);
                // голова — из базовой модели special-записи.
                let mid = if chest.is_some() {
                    "minecraft:item/chest"
                } else if let Some(b) = skull.as_ref().or(shulker.as_ref()) {
                    b
                } else {
                    &model
                };
                let Some((resolved, _)) = loader.resolve(&Identifier::parse(mid)) else {
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
                if let Some(tex) = chest {
                    self.chest_quads(&tex, &mut quads);
                    continue;
                }
                if skull.is_some() {
                    self.skull_quads(name, &mut quads);
                    continue;
                }
                if shulker.is_some() {
                    self.shulker_quads(name, &mut quads);
                    continue;
                }
                let slots = resolved.texture_slots();
                let elements = resolved.elements();
                if elements.is_empty() {
                    for i in 0..5 {
                        let Some(id) = slots.get(&format!("layer{i}")) else {
                            break;
                        };
                        let tint = tints.get(i).copied().unwrap_or(0xFF_FFFF);
                        self.extrude(rm, id, tint, i as f32, &mut quads);
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
                            pos: q.pos.map(|p| [p[0] + off[0], p[1] + off[1], p[2] + off[2]]),
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

    /// VANILLA: SkullSpecialRenderer — модель головы предмета.
    fn skull_quads(&self, item: &str, out: &mut Vec<Quad>) {
        let Some((model, tex)) = crate::skull::item_model(item) else {
            return;
        };
        self.model_quads(&model, tex, crate::skull::item_root(), out);
    }

    /// VANILLA: ShulkerBoxSpecialRenderer — модель шалкер-коробки предмета (крышка закрыта).
    fn shulker_quads(&self, item: &str, out: &mut Vec<Quad>) {
        let Some((model, tex)) = crate::shulkerbox::item_model(item) else {
            return;
        };
        self.model_quads(&model, tex, crate::shulkerbox::item_root(), out);
    }

    fn model_quads(
        &self,
        model: &ezzmc_render::entitymodel::Model,
        tex: &str,
        root: Mat4,
        out: &mut Vec<Quad>,
    ) {
        let id = Identifier::new("minecraft", &format!("entity/{tex}"));
        let Some(uv) = self.items.sprites.get(&id).copied() else {
            return;
        };
        for q in model.quads(root) {
            let n = q.normal;
            let l0 = Vec3::new(-0.933, 0.263, -0.244);
            let l1 = Vec3::new(-0.104, 0.977, 0.188);
            let shade = ((n.dot(l0).abs().max(n.dot(l1).abs())) * 0.6 + 0.4).min(1.0);
            out.push(Quad {
                pos: q.pos.map(|p| p.to_array()),
                uv: q
                    .uv
                    .map(|[u, v]| [uv.u0 + u * (uv.u1 - uv.u0), uv.v0 + v * (uv.v1 - uv.v0)]),
                color: tint_rgb(0xFF_FFFF, shade),
                tint: tint_rgb(0xFF_FFFF, 1.0),
                normal: n.to_array(),
                items: true,
            });
        }
    }

    /// VANILLA: ChestSpecialRenderer — модель одиночного сундука (openness 0) из `ChestModel`.
    fn chest_quads(&self, tex: &str, out: &mut Vec<Quad>) {
        let id = Identifier::new("minecraft", &format!("entity/chest/{tex}"));
        let Some(uv) = self.items.sprites.get(&id).copied() else {
            return;
        };
        for q in crate::chest::model_single().quads(Mat4::IDENTITY) {
            let n = q.normal;
            // VANILLA-UNSURE: освещение — как у блочных моделей в руке (формула GUI по нормали).
            let l0 = Vec3::new(-0.933, 0.263, -0.244);
            let l1 = Vec3::new(-0.104, 0.977, 0.188);
            let shade = ((n.dot(l0).abs().max(n.dot(l1).abs())) * 0.6 + 0.4).min(1.0);
            out.push(Quad {
                pos: q.pos.map(|p| p.to_array()),
                uv: q
                    .uv
                    .map(|[u, v]| [uv.u0 + u * (uv.u1 - uv.u0), uv.v0 + v * (uv.v1 - uv.v0)]),
                color: tint_rgb(0xFF_FFFF, shade),
                tint: tint_rgb(0xFF_FFFF, 1.0),
                normal: n.to_array(),
                items: true,
            });
        }
    }

    /// VANILLA: ItemModelGenerator — две грани z=7.5/8.5 и боковые полоски по краю непрозрачных пикселей.
    fn extrude(
        &self,
        rm: &ResourceManager,
        id: &Identifier,
        tint: u32,
        layer: f32,
        out: &mut Vec<Quad>,
    ) {
        // Текстура блока в плоском предмете (item/vine → block/vine): UV из атласа блоков,
        // непрозрачность — из самого PNG.
        let block_img;
        let (uv, items, img): (SpriteUv, bool, &ezzmc_assets::texture::Image) =
            if let Some(uv) = self.items.sprites.get(id).copied() {
                (uv, true, &self.items.mips[0])
            } else if let Some(uv) = self.blocks.get(id).copied() {
                let path = format!("assets/{}/textures/{}.png", id.namespace, id.path);
                let Some(img) = rm
                    .read(&path)
                    .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
                else {
                    return;
                };
                block_img = img;
                (uv, false, &block_img)
            } else {
                return;
            };
        let (aw, ah, x0, y0, w, h);
        if items {
            aw = self.items.width as f32;
            ah = self.items.height as f32;
            x0 = (uv.u0 * aw).round() as u32;
            y0 = (uv.v0 * ah).round() as u32;
            w = ((uv.u1 - uv.u0) * aw).round() as u32;
            h = ((uv.v1 - uv.v0) * ah).round() as u32;
        } else {
            (x0, y0, w, h) = (0, 0, img.width, img.width.min(img.height));
        }
        if w == 0 || h == 0 {
            return;
        }
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
