//! Броня на гуманоиде-игроке (мир и превью в инвентаре).
//! VANILLA: EquipmentLayerRenderer / HumanoidArmorLayer, HumanoidModel#createMesh(deformation 1.0 / 0.5),
//! equipment/<материал>.json (слои humanoid / humanoid_leggings, leather — dyeable + overlay).
//! Трим: `textures/trims/entity/humanoid[_leggings]/<паттерн>.png` + палитра `palettes/trim/<материал>.png`.
//! Не сделано: блеск зачарований, элитры, варианты по компоненту `equippable`.
use ezzmc_assets::pack::ResourceManager;
use ezzmc_assets::texture::Image;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_render::humanoid::{self, HumanoidParts, HumanoidState};
use glam::Mat4;
use std::collections::HashMap;
use std::sync::Mutex;

/// Реестры `minecraft:trim_material` / `minecraft:trim_pattern` (имена в порядке holder id).
pub static TRIM_REGISTRY: Mutex<(Vec<String>, Vec<String>)> = Mutex::new((Vec::new(), Vec::new()));

/// VANILLA: assets/minecraft/textures/trims/entity/humanoid — 18 паттернов.
const PATTERNS: [&str; 18] = [
    "bolt",
    "coast",
    "dune",
    "eye",
    "flow",
    "host",
    "raiser",
    "rib",
    "sentry",
    "shaper",
    "silence",
    "snout",
    "spire",
    "tide",
    "vex",
    "ward",
    "wayfinder",
    "wild",
];
/// VANILLA: assets/minecraft/textures/palettes/trim — 16 палитр.
const PALETTES: [&str; 16] = [
    "amethyst",
    "copper",
    "copper_darker",
    "diamond",
    "diamond_darker",
    "emerald",
    "gold",
    "gold_darker",
    "iron",
    "iron_darker",
    "lapis",
    "netherite",
    "netherite_darker",
    "quartz",
    "redstone",
    "resin",
];
/// Ячеек в строке атласа (каждая 64×32).
const COLS: u32 = 16;

/// Слоты брони в порядке: голова, грудь, ноги, ступни.
pub const HEAD: usize = 0;
pub const CHEST: usize = 1;
pub const LEGS: usize = 2;
const SUFFIX: [&str; 4] = ["_helmet", "_chestplate", "_leggings", "_boots"];
/// VANILLA: LeatherArmor — цвет без красителя (-6265536).
const LEATHER_UNDYED: u32 = 0xA0_6540;
const MATERIALS: [&str; 9] = [
    "chainmail",
    "copper",
    "diamond",
    "gold",
    "iron",
    "leather",
    "leather_overlay",
    "netherite",
    "turtle_scute",
];

/// Что надето в слоте: материал (имя текстуры) и цвет красителя кожаной.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Look {
    pub material: &'static str,
    pub dye: Option<u32>,
    /// (индекс паттерна, индекс палитры) в `PATTERNS`/`PALETTES`.
    pub trim: Option<(u8, u8)>,
    /// ItemStack#hasFoil — блеск зачарований.
    pub foil: bool,
}

/// VANILLA: ArmorTrim.STREAM_CODEC — Holder(материал), Holder(паттерн); inline-записи (0) не поддержаны.
fn trim_of(material: &str, patch: &ezzmc_protocol::item::ComponentPatch) -> Option<(u8, u8)> {
    use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == "trim")? as u32;
    let (_, b) = patch.added.iter().find(|(i, _)| *i == id)?;
    let mut i = 0usize;
    let varint = |i: &mut usize| -> Option<u32> {
        let mut v = 0u32;
        for sh in (0..35).step_by(7) {
            let byte = *b.get(*i)?;
            *i += 1;
            v |= u32::from(byte & 0x7F) << sh;
            if byte & 0x80 == 0 {
                return Some(v);
            }
        }
        None
    };
    let m = varint(&mut i)?.checked_sub(1)? as usize;
    let p = varint(&mut i)?.checked_sub(1)? as usize;
    let reg = TRIM_REGISTRY.lock().ok()?;
    let mname = reg.0.get(m)?.trim_start_matches("minecraft:").to_string();
    let pname = reg.1.get(p)?.trim_start_matches("minecraft:").to_string();
    let pat = PATTERNS.iter().position(|x| *x == pname)?;
    // VANILLA: equipment/<материал>.json trim_overrides — тот же материал → <имя>_darker.
    let pal_name = if mname == material {
        format!("{mname}_darker")
    } else {
        mname
    };
    let pal = PALETTES.iter().position(|x| *x == pal_name)?;
    Some((pat as u8, pal as u8))
}

/// Броня по слоту из стека. `slot`: HEAD/CHEST/LEGS/FEET.
pub fn look_of(slot: usize, st: &ItemStack) -> Option<Look> {
    let name = *ezzmc_protocol::generated::r26_3::ITEM.get(st.item as usize)?;
    if slot == CHEST && name == "elytra" {
        return Some(Look {
            material: "elytra",
            dye: None,
            trim: None,
            foil: crate::hud::has_foil(name, &st.patch),
        });
    }
    let base = name.strip_suffix(SUFFIX[slot])?;
    let material = match base {
        "golden" => "gold",
        "turtle" => "turtle_scute",
        "chainmail" | "copper" | "diamond" | "iron" | "leather" | "netherite" => {
            MATERIALS.iter().copied().find(|m| *m == base)?
        }
        _ => return None,
    };
    let dye = (material == "leather")
        .then(|| crate::itemicon::ctx_of(&st.patch).dye)
        .flatten();
    let trim = trim_of(material, &st.patch);
    let foil = crate::hud::has_foil(name, &st.patch);
    Some(Look {
        material,
        dye,
        trim,
        foil,
    })
}

/// Броня из слотов InventoryMenu (5–8: голова→ботинки).
pub fn looks_of_menu(slots: &[Option<ItemStack>]) -> [Option<Look>; 4] {
    let mut out = [None; 4];
    for (i, o) in out.iter_mut().enumerate() {
        *o = slots
            .get(crate::inventory::ARMOR_START + i)
            .and_then(|s| s.as_ref())
            .and_then(|s| look_of(i, s));
    }
    out
}

/// Броня из слотов сущности (голова, грудь, ноги, ступни).
pub fn looks_of_entity(armor: &[Option<ItemStack>; 4]) -> [Option<Look>; 4] {
    let mut out = [None; 4];
    for (i, o) in out.iter_mut().enumerate() {
        *o = armor[i].as_ref().and_then(|s| look_of(i, s));
    }
    out
}

pub struct ArmorKit {
    /// Атлас: сетка ячеек 64×32 (`COLS` в строке), RGBA.
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    cells: HashMap<(bool, &'static str), u32>,
    /// Ячейки трима: (внутренний, паттерн, палитра) → индекс.
    trims: HashMap<(bool, u8, u8), u32>,
    outer: (Model, HumanoidParts),
    inner: (Model, HumanoidParts),
    /// VANILLA: ElytraModel — части left_wing (0), right_wing (1).
    elytra: Model,
    /// Игрок/моб летит на элитрах (общий флаг 7); выставляет вызывающий перед `bake`.
    pub fall_flying: bool,
}

/// VANILLA: ElytraModel#createLayer (64×32).
fn elytra_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    let q = std::f32::consts::PI / 12.0;
    let mut p = Pose::offset(5.0, 0.0, 0.0);
    p.x_rot = q;
    p.z_rot = -q;
    m.add(
        "left_wing",
        None,
        p,
        &[BoxDef::new([22.0, 0.0], [-10.0, 0.0, 0.0], [10.0, 20.0, 2.0]).grow(1.0)],
    );
    let mut p = Pose::offset(-5.0, 0.0, 0.0);
    p.x_rot = q;
    p.z_rot = q;
    m.add(
        "right_wing",
        None,
        p,
        &[BoxDef::new([22.0, 0.0], [0.0, 0.0, 0.0], [10.0, 20.0, 2.0])
            .grow(1.0)
            .mirror()],
    );
    m
}

/// VANILLA: ElytraAnimationState (цель без сглаживания) + ElytraModel#setupAnim.
fn elytra_pose(m: &mut Model, crouching: bool, flying: bool) {
    use std::f32::consts::PI;
    // Полёт — горизонтальный (ratio = 1): lerp(1, π/12, π/9), lerp(1, −π/12, −π/2).
    let (rx, ry, rz) = if flying {
        (PI / 9.0, 0.0, -PI / 2.0)
    } else if crouching {
        (PI * 2.0 / 9.0, 0.087_266_46, -PI / 4.0)
    } else {
        (PI / 12.0, 0.0, -PI / 12.0)
    };
    let y = if crouching { 3.0 } else { 0.0 };
    for (i, sign) in [(0usize, 1.0f32), (1, -1.0)] {
        let part = &mut m.parts[i];
        part.pose = part.initial;
        part.pose.y = y;
        part.pose.x_rot = rx;
        part.pose.y_rot = ry * sign;
        part.pose.z_rot = rz * sign;
        part.visible = true;
    }
}

/// UV внутри ячейки → UV атласа в вершинах формата `ParticleDraw` (u,v на байтах 12..20).
fn remap_uv(verts: &mut [u8], cell: u32, height: u32) {
    let (cx, cy) = ((cell % COLS) as f32, (cell / COLS) as f32);
    let rows = (height / 32) as f32;
    for vert in verts.chunks_exact_mut(32) {
        let u = f32::from_le_bytes([vert[12], vert[13], vert[14], vert[15]]);
        let v = f32::from_le_bytes([vert[16], vert[17], vert[18], vert[19]]);
        vert[12..16].copy_from_slice(&((cx + u) / COLS as f32).to_le_bytes());
        vert[16..20].copy_from_slice(&((cy + v) / rows).to_le_bytes());
    }
}

/// VANILLA: HumanoidModel#createMesh(deformation) (64×32, без hat).
fn armor_mesh(grow: f32) -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 32.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(grow)],
    );
    let body = m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new([16.0, 16.0], [-4.0, 0.0, -2.0], [8.0, 12.0, 4.0]).grow(grow)],
    );
    let limb = |tex: [f32; 2], ox: f32, oy: f32, mirror: bool| {
        let b = BoxDef::new(tex, [ox, oy, -2.0], [4.0, 12.0, 4.0]).grow(grow);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    let right_arm = m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[limb([40.0, 16.0], -3.0, -2.0, false)],
    );
    let left_arm = m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[limb([40.0, 16.0], -1.0, -2.0, true)],
    );
    let right_leg = m.add(
        "right_leg",
        None,
        Pose::offset(-1.9, 12.0, 0.0),
        &[limb([0.0, 16.0], -2.0, 0.0, false)],
    );
    let left_leg = m.add(
        "left_leg",
        None,
        Pose::offset(1.9, 12.0, 0.0),
        &[limb([0.0, 16.0], -2.0, 0.0, true)],
    );
    (
        m,
        HumanoidParts {
            head,
            body,
            right_arm,
            left_arm,
            right_leg,
            left_leg,
        },
    )
}

impl ArmorKit {
    pub fn new(rm: &ResourceManager) -> Self {
        let mut cells = HashMap::new();
        let mut trims = HashMap::new();
        let mut imgs: Vec<Vec<u8>> = Vec::new();
        let load = |path: &str| {
            rm.read(path)
                .and_then(|d| Image::decode_png(&d).ok())
                .filter(|i| i.width == 64 && i.height == 32)
        };
        for (inner, dir) in [(false, "humanoid"), (true, "humanoid_leggings")] {
            for &m in &MATERIALS {
                let path = format!("assets/minecraft/textures/entity/equipment/{dir}/{m}.png");
                let Some(img) = load(&path) else { continue };
                cells.insert((inner, m), imgs.len() as u32);
                imgs.push(img.to_rgba8());
            }
        }
        if let Some(img) = load("assets/minecraft/textures/entity/equipment/wings/elytra.png") {
            cells.insert((false, "elytra"), imgs.len() as u32);
            imgs.push(img.to_rgba8());
        }
        // VANILLA: PaletteMapping — цвет базовой палитры i → цвет целевой палитры i.
        let pal = |path: &str| -> Option<Vec<[u8; 4]>> {
            let img = rm.read(path).and_then(|d| Image::decode_png(&d).ok())?;
            Some(
                img.to_rgba8()
                    .chunks_exact(4)
                    .take(img.width as usize)
                    .map(|c| [c[0], c[1], c[2], c[3]])
                    .collect(),
            )
        };
        let base = pal("assets/minecraft/textures/palettes/trim_base.png");
        if let Some(base) = base {
            let targets: Vec<Option<Vec<[u8; 4]>>> = PALETTES
                .iter()
                .map(|p| pal(&format!("assets/minecraft/textures/palettes/trim/{p}.png")))
                .collect();
            for (inner, dir) in [(false, "humanoid"), (true, "humanoid_leggings")] {
                for (pi, pat) in PATTERNS.iter().enumerate() {
                    let path = format!("assets/minecraft/textures/trims/entity/{dir}/{pat}.png");
                    let Some(img) = load(&path) else { continue };
                    let src = img.to_rgba8();
                    for (ti, t) in targets.iter().enumerate() {
                        let Some(t) = t else { continue };
                        let mut out = src.clone();
                        for px in out.chunks_exact_mut(4) {
                            if px[3] == 0 {
                                continue;
                            }
                            if let Some(k) = base.iter().position(|b| b[..3] == px[..3]) {
                                if let Some(c) = t.get(k) {
                                    px[..3].copy_from_slice(&c[..3]);
                                }
                            }
                        }
                        trims.insert((inner, pi as u8, ti as u8), imgs.len() as u32);
                        imgs.push(out);
                    }
                }
            }
        }
        let n = imgs.len().max(1) as u32;
        let rows = n.div_ceil(COLS);
        let width = 64 * COLS;
        let height = 32 * rows;
        let mut rgba = vec![0u8; (width * height * 4) as usize];
        for (i, img) in imgs.iter().enumerate() {
            let (cx, cy) = (i as u32 % COLS, i as u32 / COLS);
            for y in 0..32u32 {
                let dst = (((cy * 32 + y) * width + cx * 64) * 4) as usize;
                let src = (y * 64 * 4) as usize;
                rgba[dst..dst + 256].copy_from_slice(&img[src..src + 256]);
            }
        }
        ArmorKit {
            rgba,
            width,
            height,
            cells,
            trims,
            // VANILLA: LayerDefinitions — OUTER_ARMOR_DEFORMATION 1.0, INNER_ARMOR_DEFORMATION 0.5.
            outer: armor_mesh(1.0),
            inner: armor_mesh(0.5),
            elytra: elytra_mesh(),
            fall_flying: false,
        }
    }

    /// UV внутри ячейки (64×32) → UV всего атласа.
    pub fn atlas_uv(&self, cell: u32, [u, v]: [f32; 2]) -> [f32; 2] {
        let (cx, cy) = ((cell % COLS) as f32, (cell / COLS) as f32);
        let rows = (self.height / 32) as f32;
        [(cx + u) / COLS as f32, (cy + v) / rows]
    }

    /// Слои слота: (ячейка, цвет). Броня (кожаная — краситель + overlay), затем трим.
    fn layers(&self, look: &Look, inner: bool) -> Vec<(u32, [u8; 4])> {
        let mut layers: Vec<(&'static str, [u8; 4])> = Vec::new();
        if look.material == "leather" {
            let c = look.dye.unwrap_or(LEATHER_UNDYED);
            layers.push(("leather", [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255]));
            layers.push(("leather_overlay", [255; 4]));
        } else {
            layers.push((look.material, [255; 4]));
        }
        let mut out: Vec<(u32, [u8; 4])> = layers
            .into_iter()
            .filter_map(|(t, c)| Some((*self.cells.get(&(inner, t))?, c)))
            .collect();
        if let Some((p, t)) = look.trim {
            if let Some(&cell) = self.trims.get(&(inner, p, t)) {
                out.push((cell, [255; 4]));
            }
        }
        out
    }

    /// Запечь броню (формат `ParticleDraw`, UV под атлас). `state` — поза как у тела игрока.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn bake(
        &mut self,
        looks: &[Option<Look>; 4],
        state: &HumanoidState,
        root: Mat4,
        color: [u8; 4],
        light: (u8, u8),
        out: &mut Vec<u8>,
        glint: &mut Vec<u8>,
    ) {
        for (slot, look) in looks.iter().enumerate() {
            let Some(look) = look else { continue };
            // VANILLA: ноги — внутренний слой (leggings), остальное — внешний.
            let inner = slot == LEGS;
            let layers = self.layers(look, inner);
            if look.material == "elytra" {
                // VANILLA: WingsLayer — translate(0, 0, 0.125), крылья от корня модели.
                elytra_pose(&mut self.elytra, state.crouching, self.fall_flying);
                let r = root * Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 0.125));
                for (cell, tint) in layers {
                    let start = out.len();
                    self.elytra.bake(r, tint, light, out);
                    remap_uv(&mut out[start..], cell, self.height);
                }
                if look.foil {
                    self.elytra.bake(r, [255; 4], light, glint);
                }
                continue;
            }
            let (model, parts) = if inner {
                &mut self.inner
            } else {
                &mut self.outer
            };
            let p = *parts;
            set_visible(model, slot);
            humanoid::setup_anim(model, &p, state);
            for (cell, tint) in layers {
                let tinted = [
                    (u32::from(tint[0]) * u32::from(color[0]) / 255) as u8,
                    (u32::from(tint[1]) * u32::from(color[1]) / 255) as u8,
                    (u32::from(tint[2]) * u32::from(color[2]) / 255) as u8,
                    color[3],
                ];
                let start = out.len();
                model.bake(root, tinted, light, out);
                // Вершина 32 Б: pos 12, uv 8, цвет 4, свет 8 → u,v на байтах 12..20.
                for vert in out[start..].chunks_exact_mut(32) {
                    let u = f32::from_le_bytes([vert[12], vert[13], vert[14], vert[15]]);
                    let v = f32::from_le_bytes([vert[16], vert[17], vert[18], vert[19]]);
                    let [nu, nv] = {
                        let (cx, cy) = ((cell % COLS) as f32, (cell / COLS) as f32);
                        let rows = (self.height / 32) as f32;
                        [(cx + u) / COLS as f32, (cy + v) / rows]
                    };
                    vert[12..16].copy_from_slice(&nu.to_le_bytes());
                    vert[16..20].copy_from_slice(&nv.to_le_bytes());
                }
            }
            // VANILLA: EquipmentLayerRenderer — блеск поверх всех слоёв, UV модели 0..1 (текстура блеска
            // с матрицей `glint_matrix`).
            if look.foil {
                model.bake(root, [255; 4], light, glint);
            }
        }
    }

    /// Квады для программной растеризации превью: (квад, row, tint) на каждый слой.
    #[allow(clippy::type_complexity)]
    pub fn raster(
        &mut self,
        looks: &[Option<Look>; 4],
        state: &HumanoidState,
        root: Mat4,
    ) -> (
        Vec<(ezzmc_render::entitymodel::RasterQuad, u32, [u8; 3])>,
        Vec<ezzmc_render::entitymodel::RasterQuad>,
    ) {
        let mut out = Vec::new();
        let mut foil = Vec::new();
        for (slot, look) in looks.iter().enumerate() {
            let Some(look) = look else { continue };
            let inner = slot == LEGS;
            let layers = self.layers(look, inner);
            if look.material == "elytra" {
                elytra_pose(&mut self.elytra, state.crouching, false);
                let r = root * Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 0.125));
                let quads = self.elytra.quads(r);
                for (cell, tint) in layers {
                    out.extend(
                        quads
                            .iter()
                            .map(|q| (*q, cell, [tint[0], tint[1], tint[2]])),
                    );
                }
                if look.foil {
                    foil.extend(quads.iter().cloned());
                }
                continue;
            }
            let (model, parts) = if inner {
                &mut self.inner
            } else {
                &mut self.outer
            };
            let p = *parts;
            set_visible(model, slot);
            humanoid::setup_anim(model, &p, state);
            let quads = model.quads(root);
            for (cell, tint) in layers {
                out.extend(
                    quads
                        .iter()
                        .map(|q| (*q, cell, [tint[0], tint[1], tint[2]])),
                );
            }
            if look.foil {
                foil.extend(quads.iter().cloned());
            }
        }
        (out, foil)
    }
}

/// VANILLA: TextureTransform#setupGlintTexturing(scale) — translate(−o0, o1)·rotZ(π/18)·scale;
/// `Util.getMillis() * glintSpeed(0.5) * 8`. Для брони scale 0.16, для сущностей 0.5.
pub fn glint_matrix(scale: f32) -> Mat4 {
    let ms = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64) as f64
        * 0.5
        * 8.0) as u64;
    let o0 = (ms % 110_000) as f32 / 110_000.0;
    let o1 = (ms % 30_000) as f32 / 30_000.0;
    Mat4::from_translation(glam::Vec3::new(-o0, o1, 0.0))
        * Mat4::from_rotation_z(std::f32::consts::PI / 18.0)
        * Mat4::from_scale(glam::Vec3::new(scale, scale, 1.0))
}

/// VANILLA: ArmorModelSet — видимы только части слота.
fn set_visible(model: &mut Model, slot: usize) {
    for part in &mut model.parts {
        part.visible = match slot {
            HEAD => part.name == "head",
            CHEST => matches!(part.name, "body" | "right_arm" | "left_arm"),
            LEGS => matches!(part.name, "body" | "right_leg" | "left_leg"),
            _ => matches!(part.name, "right_leg" | "left_leg"),
        };
    }
}
