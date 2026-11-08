//! Модели блоков/предметов JSON (TODO-095): порт `BlockModel`, `BlockElement(Face|Rotation)`,
//! `ItemTransform(s)`, `TextureSlots`, `ModelDiscovery` 1.21.11.
use crate::atlas::Identifier;
use crate::pack::ResourceManager;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Направление; порядок как `Direction` в ванили (DOWN, UP, NORTH, SOUTH, WEST, EAST).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    pub fn by_name(s: &str) -> Option<Direction> {
        Some(match s {
            "down" => Direction::Down,
            "up" => Direction::Up,
            "north" => Direction::North,
            "south" => Direction::South,
            "west" => Direction::West,
            "east" => Direction::East,
            _ => return None,
        })
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Поворот грани текстуры/блока на 0/90/180/270.
// VANILLA: com.mojang.math.Quadrant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Quadrant {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl Quadrant {
    // VANILLA: Quadrant#parseJson
    pub fn parse(deg: i64) -> Result<Quadrant, String> {
        match deg.rem_euclid(360) {
            0 => Ok(Quadrant::R0),
            90 => Ok(Quadrant::R90),
            180 => Ok(Quadrant::R180),
            270 => Ok(Quadrant::R270),
            _ => Err(format!(
                "Invalid rotation {deg} found, only 0/90/180/270 allowed"
            )),
        }
    }

    pub fn shift(self) -> usize {
        self as usize
    }

    // VANILLA: Quadrant#rotateVertexIndex
    pub fn rotate_vertex_index(self, i: usize) -> usize {
        (i + self.shift()) % 4
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uvs {
    pub min_u: f32,
    pub min_v: f32,
    pub max_u: f32,
    pub max_v: f32,
}

impl Uvs {
    // VANILLA: BlockElementFace.UVs#getVertexU/V
    pub fn vertex_u(&self, i: usize) -> f32 {
        if i == 0 || i == 1 {
            self.min_u
        } else {
            self.max_u
        }
    }

    pub fn vertex_v(&self, i: usize) -> f32 {
        if i == 0 || i == 3 {
            self.min_v
        } else {
            self.max_v
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementFace {
    pub cull: Option<Direction>,
    pub tint_index: i32,
    /// `#слот` или идентификатор текстуры
    pub texture: String,
    pub uvs: Option<Uvs>,
    pub rotation: Quadrant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotationValue {
    Euler { x: f32, y: f32, z: f32 },
    Single { axis: Axis, angle: f32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementRotation {
    /// уже умножено на 1/16
    pub origin: [f32; 3],
    pub value: RotationValue,
    pub rescale: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub faces: BTreeMap<Direction, ElementFace>,
    pub rotation: Option<ElementRotation>,
    /// VANILLA: CuboidModelElement#shadeDirectionOverride (26.3; ключа `shade` больше нет)
    pub shade_direction_override: Option<Direction>,
    pub light_emission: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemTransform {
    pub rotation: [f32; 3],
    pub translation: [f32; 3],
    pub scale: [f32; 3],
    /// `NO_TRANSFORM` — единственный экземпляр без записи в JSON
    pub is_none: bool,
}

impl ItemTransform {
    pub const NONE: ItemTransform = ItemTransform {
        rotation: [0.0; 3],
        translation: [0.0; 3],
        scale: [1.0; 3],
        is_none: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayContext {
    ThirdPersonLeftHand,
    ThirdPersonRightHand,
    FirstPersonLeftHand,
    FirstPersonRightHand,
    Head,
    Gui,
    Ground,
    Fixed,
    OnShelf,
}

impl DisplayContext {
    pub const ALL: [DisplayContext; 9] = [
        DisplayContext::ThirdPersonLeftHand,
        DisplayContext::ThirdPersonRightHand,
        DisplayContext::FirstPersonLeftHand,
        DisplayContext::FirstPersonRightHand,
        DisplayContext::Head,
        DisplayContext::Gui,
        DisplayContext::Ground,
        DisplayContext::Fixed,
        DisplayContext::OnShelf,
    ];

    pub fn serialized_name(self) -> &'static str {
        match self {
            DisplayContext::ThirdPersonLeftHand => "thirdperson_lefthand",
            DisplayContext::ThirdPersonRightHand => "thirdperson_righthand",
            DisplayContext::FirstPersonLeftHand => "firstperson_lefthand",
            DisplayContext::FirstPersonRightHand => "firstperson_righthand",
            DisplayContext::Head => "head",
            DisplayContext::Gui => "gui",
            DisplayContext::Ground => "ground",
            DisplayContext::Fixed => "fixed",
            DisplayContext::OnShelf => "on_shelf",
        }
    }
}

/// Девять трансформов отображения.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemTransforms {
    pub t: [ItemTransform; 9],
}

impl ItemTransforms {
    pub const NONE: ItemTransforms = ItemTransforms {
        t: [ItemTransform::NONE; 9],
    };

    pub fn get(&self, c: DisplayContext) -> ItemTransform {
        self.t[c as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiLight {
    Front,
    Side,
}

impl GuiLight {
    pub fn light_like_block(self) -> bool {
        self == GuiLight::Side
    }
}

/// Содержимое слота `textures`: ссылка на другой слот или текстура.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotContents {
    Reference(String),
    Texture(Identifier),
}

/// Одна модель из JSON, без разрешения parent.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockModel {
    pub elements: Option<Vec<Element>>,
    pub gui_light: Option<GuiLight>,
    pub ambient_occlusion: Option<bool>,
    pub transforms: Option<ItemTransforms>,
    pub textures: BTreeMap<String, SlotContents>,
    pub parent: Option<Identifier>,
}

type R<T> = Result<T, String>;

fn vec3(o: &Map<String, Value>, key: &str) -> R<[f32; 3]> {
    let a = o
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("Missing {key}, expected to find a JsonArray"))?;
    if a.len() != 3 {
        return Err(format!("Expected 3 {key} values, found: {}", a.len()));
    }
    let mut out = [0.0; 3];
    for (i, v) in a.iter().enumerate() {
        out[i] = v
            .as_f64()
            .ok_or_else(|| format!("Expected {key}[{i}] to be a Float"))? as f32;
    }
    Ok(out)
}

fn vec3_or(o: &Map<String, Value>, key: &str, d: [f32; 3]) -> R<[f32; 3]> {
    if o.contains_key(key) {
        vec3(o, key)
    } else {
        Ok(d)
    }
}

fn position(o: &Map<String, Value>, key: &str) -> R<[f32; 3]> {
    let v = vec3(o, key)?;
    if v.iter().any(|&c| !(-16.0..=32.0).contains(&c)) {
        return Err(format!(
            "'{key}' specifier exceeds the allowed boundaries: {v:?}"
        ));
    }
    Ok(v)
}

fn parse_face(v: &Value) -> R<ElementFace> {
    let o = v.as_object().ok_or("face: expected object")?;
    let texture = o
        .get("texture")
        .and_then(Value::as_str)
        .ok_or("Missing texture, expected to find a String")?
        .to_owned();
    let cull = o
        .get("cullface")
        .and_then(Value::as_str)
        .and_then(Direction::by_name);
    let tint_index = o.get("tintindex").and_then(Value::as_i64).unwrap_or(-1) as i32;
    let uvs = match o.get("uv") {
        None => None,
        Some(u) => {
            let a = u.as_array().ok_or("uv: expected array")?;
            if a.len() != 4 {
                return Err(format!("Expected 4 uv values, found: {}", a.len()));
            }
            let f = |i: usize| a[i].as_f64().map(|x| x as f32).ok_or("uv: expected Float");
            Some(Uvs {
                min_u: f(0)?,
                min_v: f(1)?,
                max_u: f(2)?,
                max_v: f(3)?,
            })
        }
    };
    let rotation = Quadrant::parse(o.get("rotation").and_then(Value::as_i64).unwrap_or(0))?;
    Ok(ElementFace {
        cull,
        tint_index,
        texture,
        uvs,
        rotation,
    })
}

fn parse_rotation(o: &Map<String, Value>) -> R<Option<ElementRotation>> {
    let Some(r) = o.get("rotation") else {
        return Ok(None);
    };
    let r = r.as_object().ok_or("rotation: expected object")?;
    let mut origin = vec3(r, "origin")?;
    for c in &mut origin {
        *c *= 0.0625;
    }
    let f = |k: &str, d: f32| r.get(k).and_then(Value::as_f64).map_or(d, |x| x as f32);
    let value = if !r.contains_key("axis") && !r.contains_key("angle") {
        if !r.contains_key("x") && !r.contains_key("y") && !r.contains_key("z") {
            return Err(
                "Missing rotation value, expected either 'axis' and 'angle' or 'x', 'y' and 'z'"
                    .into(),
            );
        }
        RotationValue::Euler {
            x: f("x", 0.0),
            y: f("y", 0.0),
            z: f("z", 0.0),
        }
    } else {
        let name = r
            .get("axis")
            .and_then(Value::as_str)
            .ok_or("Missing axis")?
            .to_lowercase();
        let axis = match name.as_str() {
            "x" => Axis::X,
            "y" => Axis::Y,
            "z" => Axis::Z,
            _ => return Err(format!("Invalid rotation axis: {name}")),
        };
        let angle = r
            .get("angle")
            .and_then(Value::as_f64)
            .ok_or("Missing angle")? as f32;
        RotationValue::Single { axis, angle }
    };
    let rescale = r.get("rescale").and_then(Value::as_bool).unwrap_or(false);
    Ok(Some(ElementRotation {
        origin,
        value,
        rescale,
    }))
}

// VANILLA: BlockElement.Deserializer#deserialize
fn parse_element(v: &Value) -> R<Element> {
    let o = v.as_object().ok_or("element: expected object")?;
    let from = position(o, "from")?;
    let to = position(o, "to")?;
    let rotation = parse_rotation(o)?;
    let faces_obj = o
        .get("faces")
        .and_then(Value::as_object)
        .ok_or("Missing faces, expected to find a JsonObject")?;
    let mut faces = BTreeMap::new();
    for (k, fv) in faces_obj {
        let d = Direction::by_name(k).ok_or_else(|| format!("Unknown facing: {k}"))?;
        faces.insert(d, parse_face(fv)?);
    }
    if faces.is_empty() {
        return Err("Expected between 1 and 6 unique faces, got 0".into());
    }
    // VANILLA: CuboidModelElement.Deserializer#getShadeDirectionOverride
    let shade_direction_override = match o.get("shade_direction_override") {
        None => None,
        Some(v) => {
            let name = v
                .as_str()
                .ok_or("Expected shade_direction_override to be a string")?;
            Some(
                Direction::by_name(name)
                    .ok_or_else(|| format!("Unknown shade direction override: {name}"))?,
            )
        }
    };
    let light_emission = match o.get("light_emission") {
        None => 0,
        Some(Value::Number(n)) => match n.as_i64() {
            Some(i) if (0..=15).contains(&i) => i as u8,
            _ => {
                return Err(
                    "Expected 'light_emission' to be an Integer between (inclusive) 0 and 15"
                        .into(),
                )
            }
        },
        Some(_) => {
            return Err(
                "Expected 'light_emission' to be an Integer between (inclusive) 0 and 15".into(),
            )
        }
    };
    Ok(Element {
        from,
        to,
        faces,
        rotation,
        shade_direction_override,
        light_emission,
    })
}

// VANILLA: ItemTransform.Deserializer#deserialize
fn parse_transform(v: &Value) -> R<ItemTransform> {
    let o = v.as_object().ok_or("transform: expected object")?;
    let rotation = vec3_or(o, "rotation", [0.0; 3])?;
    let mut translation = vec3_or(o, "translation", [0.0; 3])?;
    for c in &mut translation {
        *c = (*c * 0.0625).clamp(-5.0, 5.0);
    }
    let mut scale = vec3_or(o, "scale", [1.0; 3])?;
    for c in &mut scale {
        *c = c.clamp(-4.0, 4.0);
    }
    Ok(ItemTransform {
        rotation,
        translation,
        scale,
        is_none: false,
    })
}

// VANILLA: ItemTransforms.Deserializer#deserialize
fn parse_transforms(v: &Value) -> R<ItemTransforms> {
    let o = v.as_object().ok_or("display: expected object")?;
    let get = |c: DisplayContext| -> R<ItemTransform> {
        match o.get(c.serialized_name()) {
            Some(t) => parse_transform(t),
            None => Ok(ItemTransform::NONE),
        }
    };
    use DisplayContext::*;
    let tp_r = get(ThirdPersonRightHand)?;
    let mut tp_l = get(ThirdPersonLeftHand)?;
    if tp_l.is_none {
        tp_l = tp_r;
    }
    let fp_r = get(FirstPersonRightHand)?;
    let mut fp_l = get(FirstPersonLeftHand)?;
    if fp_l.is_none {
        fp_l = fp_r;
    }
    // порядок DisplayContext::ALL
    Ok(ItemTransforms {
        t: [
            tp_l,
            tp_r,
            fp_l,
            fp_r,
            get(Head)?,
            get(Gui)?,
            get(Ground)?,
            get(Fixed)?,
            get(OnShelf)?,
        ],
    })
}

impl BlockModel {
    // VANILLA: BlockModel.Deserializer#deserialize
    pub fn parse(data: &[u8]) -> Result<BlockModel, String> {
        let v: Value = serde_json::from_slice(data).map_err(|e| e.to_string())?;
        let o = v.as_object().ok_or("model: expected object")?;
        let elements = match o.get("elements") {
            None => None,
            Some(e) => Some(
                e.as_array()
                    .ok_or("elements: expected array")?
                    .iter()
                    .map(parse_element)
                    .collect::<R<Vec<_>>>()?,
            ),
        };
        let parent = match o.get("parent").and_then(Value::as_str) {
            None | Some("") => None,
            Some(p) => Some(Identifier::parse(p)),
        };
        let mut textures = BTreeMap::new();
        if let Some(t) = o.get("textures") {
            for (k, val) in t.as_object().ok_or("textures: expected object")? {
                // VANILLA: Material#CODEC (26.3) — строка или {sprite, force_translucent}.
                // VANILLA-UNSURE: force_translucent пока не учитывается (слой берётся из ChunkSectionLayer-таблицы).
                let s = match val {
                    Value::String(s) => s.as_str(),
                    Value::Object(m) => m
                        .get("sprite")
                        .and_then(Value::as_str)
                        .ok_or("textures: expected sprite string")?,
                    _ => return Err("textures: expected string".into()),
                };
                textures.insert(
                    k.clone(),
                    match s.strip_prefix('#') {
                        Some(r) => SlotContents::Reference(r.to_owned()),
                        None => SlotContents::Texture(Identifier::parse(s)),
                    },
                );
            }
        }
        let ambient_occlusion = match o.get("ambientocclusion") {
            None => None,
            Some(b) => Some(b.as_bool().ok_or("ambientocclusion: expected bool")?),
        };
        let transforms = match o.get("display") {
            None => None,
            Some(d) => Some(parse_transforms(d)?),
        };
        let gui_light = match o.get("gui_light").and_then(Value::as_str) {
            None => None,
            Some("front") => Some(GuiLight::Front),
            Some("side") => Some(GuiLight::Side),
            Some(x) => return Err(format!("Invalid gui light: {x}")),
        };
        Ok(BlockModel {
            elements,
            gui_light,
            ambient_occlusion,
            transforms,
            textures,
            parent,
        })
    }
}

/// Разрешённые слоты текстур: `слот → текстура`.
// VANILLA: TextureSlots (Resolver#resolve)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextureSlots {
    pub values: HashMap<String, Identifier>,
}

impl TextureSlots {
    /// `#name` или `name`.
    pub fn get(&self, name: &str) -> Option<&Identifier> {
        self.values.get(name.strip_prefix('#').unwrap_or(name))
    }
}

/// Модель с цепочкой предков (`chain[0]` — сама модель, далее parent...).
#[derive(Debug, Clone)]
pub struct ResolvedModel {
    pub id: Identifier,
    pub chain: Vec<Arc<BlockModel>>,
}

impl ResolvedModel {
    // VANILLA: ResolvedModel#findTopTextureSlots
    pub fn texture_slots(&self) -> TextureSlots {
        let mut tex: HashMap<String, Identifier> = HashMap::new();
        let mut refs: HashMap<String, String> = HashMap::new();
        // от самого дальнего предка к потомку: потомок перекрывает
        for m in self.chain.iter().rev() {
            for (k, c) in &m.textures {
                match c {
                    SlotContents::Texture(t) => {
                        refs.remove(k);
                        tex.insert(k.clone(), t.clone());
                    }
                    SlotContents::Reference(r) => {
                        tex.remove(k);
                        refs.insert(k.clone(), r.clone());
                    }
                }
            }
        }
        let mut progress = true;
        while progress && !refs.is_empty() {
            progress = false;
            let keys: Vec<String> = refs.keys().cloned().collect();
            for k in keys {
                if let Some(t) = tex.get(&refs[&k]).cloned() {
                    tex.insert(k.clone(), t);
                    refs.remove(&k);
                    progress = true;
                }
            }
        }
        for (k, r) in &refs {
            tracing::warn!("Unresolved texture references in {}: #{k}-> #{r}", self.id);
        }
        TextureSlots { values: tex }
    }

    pub fn ambient_occlusion(&self) -> bool {
        self.chain
            .iter()
            .find_map(|m| m.ambient_occlusion)
            .unwrap_or(true)
    }

    pub fn gui_light(&self) -> GuiLight {
        self.chain
            .iter()
            .find_map(|m| m.gui_light)
            .unwrap_or(GuiLight::Side)
    }

    /// Элементы ближайшей модели цепочки, где они заданы.
    pub fn elements(&self) -> &[Element] {
        self.chain
            .iter()
            .find_map(|m| m.elements.as_deref())
            .unwrap_or(&[])
    }

    // VANILLA: ResolvedModel#findTopTransform
    pub fn transform(&self, c: DisplayContext) -> ItemTransform {
        for m in &self.chain {
            if let Some(t) = &m.transforms {
                let x = t.get(c);
                if !x.is_none {
                    return x;
                }
            }
        }
        ItemTransform::NONE
    }

    pub fn transforms(&self) -> ItemTransforms {
        let mut t = [ItemTransform::NONE; 9];
        for c in DisplayContext::ALL {
            t[c as usize] = self.transform(c);
        }
        ItemTransforms { t }
    }
}

/// Загрузчик моделей `assets/<ns>/models/<path>.json` с кэшем.
pub struct ModelLoader<'a> {
    rm: &'a ResourceManager,
    cache: HashMap<Identifier, Option<Arc<BlockModel>>>,
}

/// Встроенные родители без файла (`builtin/generated` и т.п.).
pub fn is_builtin(id: &Identifier) -> bool {
    id.namespace == "minecraft" && id.path.starts_with("builtin/")
}

impl<'a> ModelLoader<'a> {
    pub fn new(rm: &'a ResourceManager) -> Self {
        ModelLoader {
            rm,
            cache: HashMap::new(),
        }
    }

    pub fn file(id: &Identifier) -> String {
        format!("assets/{}/models/{}.json", id.namespace, id.path)
    }

    /// Загрузить одну модель (без предков); `None` — нет файла или битый JSON.
    pub fn load(&mut self, id: &Identifier) -> Option<Arc<BlockModel>> {
        if let Some(c) = self.cache.get(id) {
            return c.clone();
        }
        let path = Self::file(id);
        let m = match self.rm.read(&path) {
            None => None,
            Some(d) => match BlockModel::parse(&d) {
                Ok(m) => Some(Arc::new(m)),
                Err(e) => {
                    tracing::warn!("Couldn't parse model {id}: {e}");
                    None
                }
            },
        };
        self.cache.insert(id.clone(), m.clone());
        m
    }

    /// Разрешить цепочку parent. Цикл → `None` (как «Model ignored due to cyclic dependency»);
    /// отсутствующий родитель → `None` (ваниль подставляет missing-модель — решает вызывающий).
    /// `builtin/*` в цепочку не входит, но имя последнего builtin возвращается во втором поле.
    // VANILLA: ModelDiscovery#resolve
    pub fn resolve(&mut self, id: &Identifier) -> Option<(ResolvedModel, Option<Identifier>)> {
        let mut chain = Vec::new();
        let mut seen: Vec<Identifier> = Vec::new();
        let mut cur = id.clone();
        let mut builtin = None;
        loop {
            if is_builtin(&cur) {
                builtin = Some(cur);
                break;
            }
            if seen.contains(&cur) {
                tracing::warn!("Model {id} ignored due to cyclic dependency");
                return None;
            }
            seen.push(cur.clone());
            let m = self.load(&cur).or_else(|| {
                tracing::warn!("Missing block model: {cur}");
                None
            })?;
            let next = m.parent.clone();
            chain.push(m);
            match next {
                Some(p) => cur = p,
                None => break,
            }
        }
        Some((
            ResolvedModel {
                id: id.clone(),
                chain,
            },
            builtin,
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::pack::Pack;
    use crate::source::DirSource;

    #[test]
    fn parse_cube() {
        let m = BlockModel::parse(
            br##"{"parent":"block/block","ambientocclusion":false,"textures":{"all":"block/stone","p":"#all"},
            "elements":[{"from":[0,0,0],"to":[16,16,16],"rotation":{"origin":[8,8,8],"axis":"Y","angle":45,"rescale":true},
            "shade_direction_override":"up","faces":{"down":{"texture":"#all","cullface":"down","uv":[0,0,16,16],"rotation":90,"tintindex":0}}}],
            "display":{"gui":{"rotation":[30,225,0],"translation":[0,0,20],"scale":[0.625,0.625,0.625]}}}"##,
        )
        .unwrap();
        assert_eq!(m.parent, Some(Identifier::parse("minecraft:block/block")));
        let e = &m.elements.as_ref().unwrap()[0];
        assert_eq!(e.shade_direction_override, Some(Direction::Up));
        assert_eq!(e.rotation.as_ref().unwrap().origin, [0.5; 3]);
        let f = &e.faces[&Direction::Down];
        assert_eq!(
            (f.tint_index, f.rotation, f.cull),
            (0, Quadrant::R90, Some(Direction::Down))
        );
        let t = m.transforms.unwrap();
        // translation 20/16 клампится до 5 → 1.25 остаётся (20*0.0625)
        assert!((t.get(DisplayContext::Gui).translation[2] - 1.25).abs() < 1e-6);
        // left hand копирует right: оба не заданы → NONE
        assert!(t.get(DisplayContext::ThirdPersonLeftHand).is_none);
    }

    #[test]
    fn errors() {
        assert!(BlockModel::parse(
            br#"{"elements":[{"from":[0,0,0],"to":[40,1,1],"faces":{"up":{"texture":"a"}}}]}"#
        )
        .is_err());
        assert!(
            BlockModel::parse(br#"{"elements":[{"from":[0,0,0],"to":[1,1,1],"faces":{}}]}"#)
                .is_err()
        );
        assert!(BlockModel::parse(br#"{"elements":[{"from":[0,0,0],"to":[1,1,1],"faces":{"up":{"texture":"a","rotation":45}}}]}"#).is_err());
        assert!(BlockModel::parse(br#"{"gui_light":"x"}"#).is_err());
    }

    #[test]
    fn chain_and_slots() {
        let dir = tempfile::tempdir().unwrap();
        let w = |p: &str, s: &str| {
            let f = dir.path().join(p);
            std::fs::create_dir_all(f.parent().unwrap()).unwrap();
            std::fs::write(f, s).unwrap();
        };
        w(
            "assets/minecraft/models/block/cube.json",
            r##"{"elements":[{"from":[0,0,0],"to":[16,16,16],"faces":{"up":{"texture":"#top"}}}],"textures":{"particle":"#top"},"ambientocclusion":false}"##,
        );
        w(
            "assets/minecraft/models/block/cube_all.json",
            r##"{"parent":"block/cube","textures":{"top":"#all"}}"##,
        );
        w(
            "assets/minecraft/models/block/stone.json",
            r##"{"parent":"minecraft:block/cube_all","textures":{"all":"block/stone"}}"##,
        );
        w(
            "assets/minecraft/models/block/a.json",
            r##"{"parent":"block/b"}"##,
        );
        w(
            "assets/minecraft/models/block/b.json",
            r##"{"parent":"block/a"}"##,
        );
        w(
            "assets/minecraft/models/item/x.json",
            r##"{"parent":"builtin/generated"}"##,
        );
        let rm = ResourceManager::new(vec![Pack::new("t", Box::new(DirSource::new(dir.path())))]);
        let mut l = ModelLoader::new(&rm);
        let (m, b) = l.resolve(&Identifier::parse("block/stone")).unwrap();
        assert!(b.is_none());
        assert_eq!(m.chain.len(), 3);
        let s = m.texture_slots();
        assert_eq!(s.get("#particle").unwrap().path, "block/stone");
        assert_eq!(s.get("top").unwrap().path, "block/stone");
        assert!(!m.ambient_occlusion());
        assert_eq!(m.elements().len(), 1);
        assert!(l.resolve(&Identifier::parse("block/a")).is_none());
        let (_, b) = l.resolve(&Identifier::parse("item/x")).unwrap();
        assert_eq!(b.unwrap().path, "builtin/generated");
        assert!(l.resolve(&Identifier::parse("block/none")).is_none());
    }
}
