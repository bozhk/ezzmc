//! Компиляция моделей → baked quads (TODO-098): порт `FaceBakery`, `SimpleUnbakedGeometry#bake`,
//! `BlockModelRotation`, `BlockMath#getFaceTransformation`, `FaceInfo` 1.21.11.
use crate::atlas::Identifier;
use crate::model::{
    Axis, Direction, Element, ElementFace, ElementRotation, Quadrant, RotationValue, TextureSlots,
    Uvs,
};
use glam::{Mat4, Vec3};

/// UV-прямоугольник спрайта в атласе (0..1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpriteUv {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

impl SpriteUv {
    // VANILLA: TextureAtlasSprite#getU/getV
    pub fn get_u(&self, u: f32) -> f32 {
        self.u0 + (self.u1 - self.u0) * u
    }

    pub fn get_v(&self, v: f32) -> f32 {
        self.v0 + (self.v1 - self.v0) * v
    }
}

/// Источник спрайтов атласа блоков.
pub trait SpriteGetter {
    fn get(&self, id: &Identifier) -> Option<SpriteUv>;
    /// `missingno`
    fn missing(&self) -> SpriteUv;
    /// Прозрачность области атласа (UV 0..1): 0 — непрозрачна, 1 — есть альфа=0 (cutout), 2 — есть 0<альфа<255 (translucent).
    // VANILLA: SpriteContents#computeTransparency → ChunkSectionLayer#byTransparency (26.x: слой считается по пикселям спрайта)
    fn region_transparency(&self, _u: [f32; 2], _v: [f32; 2]) -> u8 {
        0
    }
}

/// Один baked-квад: 4 вершины в блочных координатах 0..1, UV уже в координатах атласа.
#[derive(Debug, Clone, PartialEq)]
pub struct BakedQuad {
    pub pos: [[f32; 3]; 4],
    pub uv: [[f32; 2]; 4],
    pub tint_index: i32,
    pub direction: Direction,
    pub sprite: Identifier,
    pub shade_direction_override: Option<Direction>,
    pub light_emission: u8,
}

impl BakedQuad {
    pub fn is_tinted(&self) -> bool {
        self.tint_index != -1
    }
}

/// Квады модели: без cull-грани и по шести cull-граням (индекс `Direction::index`).
// VANILLA: QuadCollection
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuadCollection {
    pub unculled: Vec<BakedQuad>,
    pub culled: [Vec<BakedQuad>; 6],
}

impl QuadCollection {
    pub fn get_quads(&self, side: Option<Direction>) -> &[BakedQuad] {
        match side {
            None => &self.unculled,
            Some(d) => &self.culled[d.index()],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.unculled.is_empty() && self.culled.iter().all(Vec::is_empty)
    }
}

/// Поворот варианта (`x`,`y`,`z`) и uvlock.
// VANILLA: Variant.SimpleModelState#asModelState → BlockModelRotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModelState {
    pub x: Quadrant,
    pub y: Quadrant,
    pub z: Quadrant,
    pub uv_lock: bool,
}

/// Точный поворот на `quarter` четвертей оборота вокруг оси (без ошибок sin/cos).
fn rot_exact(axis: Axis, quarter: i32) -> Mat4 {
    let q = quarter.rem_euclid(4) as usize;
    let c = [1.0, 0.0, -1.0, 0.0][q];
    let s = [0.0, 1.0, 0.0, -1.0][q];
    // glam: столбцы
    match axis {
        Axis::X => Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]),
        Axis::Y => Mat4::from_cols_array(&[
            c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]),
        Axis::Z => Mat4::from_cols_array(&[
            c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]),
    }
}

fn unit(d: Direction) -> Vec3 {
    match d {
        Direction::Down => Vec3::new(0.0, -1.0, 0.0),
        Direction::Up => Vec3::new(0.0, 1.0, 0.0),
        Direction::North => Vec3::new(0.0, 0.0, -1.0),
        Direction::South => Vec3::new(0.0, 0.0, 1.0),
        Direction::West => Vec3::new(-1.0, 0.0, 0.0),
        Direction::East => Vec3::new(1.0, 0.0, 0.0),
    }
}

// VANILLA: Direction#getApproximateNearest(float,float,float)
fn approximate_nearest(v: Vec3) -> Direction {
    let mut best = Direction::North;
    let mut best_dot = f32::MIN_POSITIVE;
    for d in Direction::ALL {
        let dot = v.dot(unit(d));
        if dot > best_dot {
            best_dot = dot;
            best = d;
        }
    }
    best
}

fn is_identity(m: &Mat4) -> bool {
    *m == Mat4::IDENTITY
}

impl ModelState {
    /// `Quadrant.fromXYZAngles`: порядок X → Y → Z, каждый поворот — на −n·90° (BLOCK_ROT_*).
    // VANILLA: Quadrant#fromXYZAngles, OctahedralGroup.BLOCK_ROT_*
    pub fn transformation(&self) -> Mat4 {
        let rx = rot_exact(Axis::X, -(self.x.shift() as i32));
        let ry = rot_exact(Axis::Y, -(self.y.shift() as i32));
        let rz = rot_exact(Axis::Z, -(self.z.shift() as i32));
        rz * (ry * rx)
    }

    // VANILLA: BlockMath#getFaceTransformation
    fn face_transformation(t: &Mat4, face: Direction) -> Mat4 {
        if is_identity(t) {
            return *t;
        }
        let l2g = |d: Direction| match d {
            Direction::South => Mat4::IDENTITY,
            Direction::East => rot_exact(Axis::Y, 1),
            Direction::West => rot_exact(Axis::Y, -1),
            Direction::North => rot_exact(Axis::Y, 2),
            Direction::Up => rot_exact(Axis::X, -1),
            Direction::Down => rot_exact(Axis::X, 1),
        };
        let x = *t * l2g(face);
        let n = x.transform_vector3(Vec3::Z);
        let d = approximate_nearest(n);
        l2g(d).inverse() * x
    }

    /// Обратная матрица грани для uvlock (иначе — единичная).
    // VANILLA: BlockModelRotation.WithUvLock#inverseFaceTransformation
    pub fn inverse_face_transformation(&self, face: Direction) -> Mat4 {
        if !self.uv_lock {
            return Mat4::IDENTITY;
        }
        Self::face_transformation(&self.transformation(), face).inverse()
    }

    /// Поворот cull-грани вместе с блоком.
    // VANILLA: Direction#rotate(Matrix4fc, Direction)
    pub fn rotate_direction(&self, d: Direction) -> Direction {
        let t = self.transformation();
        if is_identity(&t) {
            return d;
        }
        approximate_nearest(t.transform_vector3(unit(d)))
    }
}

#[derive(Clone, Copy)]
enum Extent {
    MinX,
    MinY,
    MinZ,
    MaxX,
    MaxY,
    MaxZ,
}

impl Extent {
    fn select(self, from: Vec3, to: Vec3) -> f32 {
        match self {
            Extent::MinX => from.x,
            Extent::MinY => from.y,
            Extent::MinZ => from.z,
            Extent::MaxX => to.x,
            Extent::MaxY => to.y,
            Extent::MaxZ => to.z,
        }
    }

    fn select6(self, b: [f32; 6]) -> f32 {
        b[self as usize]
    }
}

use Extent::{MaxX as XM, MaxY as YM, MaxZ as ZM, MinX as Xm, MinY as Ym, MinZ as Zm};

/// Вершины граней: `[face][vertex] = (x, y, z)` экстентов.
// VANILLA: FaceInfo
const FACE_INFO: [[(Extent, Extent, Extent); 4]; 6] = [
    // DOWN
    [(Xm, Ym, ZM), (Xm, Ym, Zm), (XM, Ym, Zm), (XM, Ym, ZM)],
    // UP
    [(Xm, YM, Zm), (Xm, YM, ZM), (XM, YM, ZM), (XM, YM, Zm)],
    // NORTH
    [(XM, YM, Zm), (XM, Ym, Zm), (Xm, Ym, Zm), (Xm, YM, Zm)],
    // SOUTH
    [(Xm, YM, ZM), (Xm, Ym, ZM), (XM, Ym, ZM), (XM, YM, ZM)],
    // WEST
    [(Xm, YM, Zm), (Xm, Ym, Zm), (Xm, Ym, ZM), (Xm, YM, ZM)],
    // EAST
    [(XM, YM, ZM), (XM, Ym, ZM), (XM, Ym, Zm), (XM, YM, Zm)],
];

// VANILLA: FaceBakery#defaultFaceUV
fn default_face_uv(from: Vec3, to: Vec3, d: Direction) -> Uvs {
    let (f, t) = (from, to);
    let (a, b, c, e) = match d {
        Direction::Down => (f.x, 16.0 - t.z, t.x, 16.0 - f.z),
        Direction::Up => (f.x, f.z, t.x, t.z),
        Direction::North => (16.0 - t.x, 16.0 - t.y, 16.0 - f.x, 16.0 - f.y),
        Direction::South => (f.x, 16.0 - t.y, t.x, 16.0 - f.y),
        Direction::West => (f.z, 16.0 - t.y, t.z, 16.0 - f.y),
        Direction::East => (16.0 - t.z, 16.0 - t.y, 16.0 - f.z, 16.0 - f.y),
    };
    Uvs {
        min_u: a,
        min_v: b,
        max_u: c,
        max_v: e,
    }
}

/// Матрица поворота элемента (с `rescale`).
// VANILLA: BlockElementRotation#computeTransform
fn rotation_transform(r: &ElementRotation) -> Mat4 {
    let mut m = match r.value {
        // Matrix4f#rotationZYX(z, y, x)
        RotationValue::Euler { x, y, z } => {
            let d = std::f32::consts::PI / 180.0;
            Mat4::from_rotation_z(z * d)
                * Mat4::from_rotation_y(y * d)
                * Mat4::from_rotation_x(x * d)
        }
        RotationValue::Single { axis, angle } => {
            if angle == 0.0 {
                Mat4::IDENTITY
            } else {
                let a = angle * (std::f32::consts::PI / 180.0);
                match axis {
                    Axis::X => Mat4::from_rotation_x(a),
                    Axis::Y => Mat4::from_rotation_y(a),
                    Axis::Z => Mat4::from_rotation_z(a),
                }
            }
        }
    };
    if r.rescale && !is_identity(&m) {
        let f = |v: Vec3| {
            let t = m.transform_vector3(v);
            1.0 / t.x.abs().max(t.y.abs()).max(t.z.abs())
        };
        let s = Vec3::new(f(Vec3::X), f(Vec3::Y), f(Vec3::Z));
        m *= Mat4::from_scale(s);
    }
    m
}

fn rotate_vertex_by(v: Vec3, origin: Vec3, m: &Mat4) -> Vec3 {
    m.transform_point3(v - origin) + origin
}

// VANILLA: org.joml.GeometryUtils#normal (+ FaceBakery#findClosestDirection)
fn calculate_facing(p: &[Vec3; 4]) -> Option<Direction> {
    let n = (p[1] - p[0]).cross(p[2] - p[0]).normalize();
    if !n.is_finite() {
        return None;
    }
    let mut best = None;
    let mut best_dot = 0.0f32;
    for d in Direction::ALL {
        let dot = n.dot(unit(d));
        if dot >= 0.0 && dot > best_dot {
            best_dot = dot;
            best = Some(d);
        }
    }
    best
}

// VANILLA: FaceBakery#recalculateWinding
fn recalculate_winding(p: &mut [Vec3; 4], uv: &mut [[f32; 2]; 4], dir: Direction) {
    let mut b = [999.0f32, 999.0, 999.0, -999.0, -999.0, -999.0];
    for v in p.iter() {
        b[0] = b[0].min(v.x);
        b[1] = b[1].min(v.y);
        b[2] = b[2].min(v.z);
        b[3] = b[3].max(v.x);
        b[4] = b[4].max(v.y);
        b[5] = b[5].max(v.z);
    }
    let info = &FACE_INFO[dir.index()];
    for (i, &(ex, ey, ez)) in info.iter().enumerate() {
        let (x, y, z) = (ex.select6(b), ey.select6(b), ez.select6(b));
        let found = (i..4).find(|&j| p[j].x == x && p[j].y == y && p[j].z == z);
        match found {
            Some(j) if j != i => {
                p.swap(j, i);
                uv.swap(j, i);
            }
            Some(_) => {}
            None => tracing::error!("Can't find vertex to swap"),
        }
    }
}

// VANILLA: FaceBakery#bakeQuad
#[allow(clippy::too_many_arguments)]
fn bake_quad(
    from: Vec3,
    to: Vec3,
    face: &ElementFace,
    sprite_id: &Identifier,
    sprite: SpriteUv,
    dir: Direction,
    state: &ModelState,
    rotation: Option<&ElementRotation>,
    shade_direction_override: Option<Direction>,
    light_emission: u8,
) -> BakedQuad {
    let uvs = face.uvs.unwrap_or_else(|| default_face_uv(from, to, dir));
    let inv = state.inverse_face_transformation(dir);
    let model_t = state.transformation();
    let elem_t = rotation.map(|r| (Vec3::from(r.origin), rotation_transform(r)));
    let mut pos = [Vec3::ZERO; 4];
    let mut uv = [[0.0f32; 2]; 4];
    for i in 0..4 {
        let (ex, ey, ez) = FACE_INFO[dir.index()][i];
        let mut v = Vec3::new(
            ex.select(from, to),
            ey.select(from, to),
            ez.select(from, to),
        ) / 16.0;
        if let Some((o, m)) = &elem_t {
            v = rotate_vertex_by(v, *o, m);
        }
        if !is_identity(&model_t) {
            v = rotate_vertex_by(v, Vec3::splat(0.5), &model_t);
        }
        // BlockElementFace#getU/getV
        let ri = face.rotation.rotate_vertex_index(i);
        let (mut u, mut w) = (uvs.vertex_u(ri) / 16.0, uvs.vertex_v(ri) / 16.0);
        if !is_identity(&inv) {
            let t = inv.transform_point3(Vec3::new(u - 0.5, w - 0.5, 0.0));
            u = t.x + 0.5;
            w = t.y + 0.5;
        }
        pos[i] = v;
        uv[i] = [sprite.get_u(u), sprite.get_v(w)];
    }
    let facing = calculate_facing(&pos);
    if rotation.is_none() {
        if let Some(f) = facing {
            recalculate_winding(&mut pos, &mut uv, f);
        }
    }
    BakedQuad {
        pos: pos.map(|v| v.to_array()),
        uv,
        tint_index: face.tint_index,
        direction: facing.unwrap_or(Direction::Up),
        sprite: sprite_id.clone(),
        shade_direction_override,
        light_emission,
    }
}

// VANILLA: SpriteGetter#resolveSlot
fn resolve_slot(
    slots: &TextureSlots,
    name: &str,
    sprites: &dyn SpriteGetter,
    model_name: &str,
) -> (Identifier, SpriteUv) {
    match slots.get(name) {
        Some(id) => match sprites.get(id) {
            Some(s) => (id.clone(), s),
            None => {
                tracing::warn!("Unable to find sprite '{id}' referenced by model '{model_name}'");
                (Identifier::new("minecraft", "missingno"), sprites.missing())
            }
        },
        None => {
            tracing::warn!("Unable to resolve texture reference: {name} in {model_name}");
            (Identifier::new("minecraft", "missingno"), sprites.missing())
        }
    }
}

/// Запечь элементы модели.
// VANILLA: SimpleUnbakedGeometry#bake
pub fn bake_elements(
    elements: &[Element],
    slots: &TextureSlots,
    sprites: &dyn SpriteGetter,
    state: &ModelState,
    debug: &str,
) -> QuadCollection {
    let mut out = QuadCollection::default();
    for e in elements {
        let (f, t) = (Vec3::from(e.from), Vec3::from(e.to));
        let (mut ax, mut ay, mut az) = (true, true, true);
        if f.x == t.x {
            ay = false;
            az = false;
        }
        if f.y == t.y {
            ax = false;
            az = false;
        }
        if f.z == t.z {
            ax = false;
            ay = false;
        }
        if !(ax || ay || az) {
            continue;
        }
        for (dir, face) in &e.faces {
            let ok = match dir {
                Direction::West | Direction::East => ax,
                Direction::Down | Direction::Up => ay,
                Direction::North | Direction::South => az,
            };
            if !ok {
                continue;
            }
            let (sid, sprite) = resolve_slot(slots, &face.texture, sprites, debug);
            let q = bake_quad(
                f,
                t,
                face,
                &sid,
                sprite,
                *dir,
                state,
                e.rotation.as_ref(),
                e.shade_direction_override,
                e.light_emission,
            );
            match face.cull {
                None => out.unculled.push(q),
                Some(c) => out.culled[state.rotate_direction(c).index()].push(q),
            }
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::model::BlockModel;

    struct Sprites;
    impl SpriteGetter for Sprites {
        fn get(&self, _: &Identifier) -> Option<SpriteUv> {
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
                u1: 0.0,
                v1: 0.0,
            }
        }
    }

    fn slots() -> TextureSlots {
        let mut s = TextureSlots::default();
        s.values
            .insert("a".into(), Identifier::parse("block/stone"));
        s
    }

    fn cube(extra: &str) -> Vec<Element> {
        let faces: Vec<String> = ["down", "up", "north", "south", "west", "east"]
            .iter()
            .map(|d| format!(r##""{d}":{{"texture":"#a","cullface":"{d}"{extra}}}"##))
            .collect();
        let j = format!(
            r#"{{"elements":[{{"from":[0,0,0],"to":[16,16,16],"faces":{{{}}}}}]}}"#,
            faces.join(",")
        );
        BlockModel::parse(j.as_bytes()).unwrap().elements.unwrap()
    }

    #[test]
    fn full_cube_faces_and_directions() {
        let q = bake_elements(&cube(""), &slots(), &Sprites, &ModelState::default(), "t");
        assert!(q.unculled.is_empty());
        for d in Direction::ALL {
            let v = &q.culled[d.index()];
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].direction, d);
        }
        // UP: вершины (0,1,0),(0,1,1),(1,1,1),(1,1,0); UV по умолчанию 0,0..1,1
        let up = &q.culled[Direction::Up.index()][0];
        assert_eq!(up.pos[0], [0.0, 1.0, 0.0]);
        assert_eq!(up.pos[2], [1.0, 1.0, 1.0]);
        assert_eq!(up.uv[0], [0.0, 0.0]);
        assert_eq!(up.uv[2], [1.0, 1.0]);
        // NORTH: UV от 16-to.x..., первая вершина (1,1,0) → u=min? (maxX → vertex_u(0)=minU)
        let n = &q.culled[Direction::North.index()][0];
        assert_eq!(n.pos[0], [1.0, 1.0, 0.0]);
        assert_eq!(n.uv[0], [0.0, 0.0]);
    }

    #[test]
    fn y_rotation_moves_faces_and_cull() {
        let st = ModelState {
            y: Quadrant::R90,
            ..Default::default()
        };
        let q = bake_elements(&cube(""), &slots(), &Sprites, &st, "t");
        // y=90: север → восток (поворот по часовой при взгляде сверху)
        let east = &q.culled[Direction::East.index()];
        assert_eq!(east.len(), 1);
        assert_eq!(east[0].direction, Direction::East);
        assert_eq!(st.rotate_direction(Direction::North), Direction::East);
        assert_eq!(st.rotate_direction(Direction::East), Direction::South);
        assert_eq!(st.rotate_direction(Direction::Up), Direction::Up);
        // x=90: север → низ, верх → север (как dispenser facing=down)
        let sx = ModelState {
            x: Quadrant::R90,
            ..Default::default()
        };
        assert_eq!(sx.rotate_direction(Direction::North), Direction::Down);
        assert_eq!(sx.rotate_direction(Direction::Up), Direction::North);
    }

    #[test]
    fn element_rotation_and_flat_element() {
        let m = BlockModel::parse(
            br##"{"elements":[{"from":[0,8,8],"to":[16,8,8],"faces":{"up":{"texture":"#a"}}},
            {"from":[0,0,8],"to":[16,16,8],"rotation":{"origin":[8,8,8],"axis":"y","angle":45,"rescale":true},
            "faces":{"north":{"texture":"#a"},"south":{"texture":"#a"}}}]}"##,
        )
        .unwrap();
        let q = bake_elements(
            &m.elements.unwrap(),
            &slots(),
            &Sprites,
            &ModelState::default(),
            "t",
        );
        // первый элемент вырожден (from.y==to.y==... все три оси) → пропущен
        assert_eq!(q.unculled.len(), 2);
        let p = &q.unculled[0].pos;
        // поворот 45° вокруг Y с rescale: x ∈ [0,1] сохраняется после масштаба 1/cos45
        let minx = p.iter().map(|v| v[0]).fold(f32::MAX, f32::min);
        let maxx = p.iter().map(|v| v[0]).fold(f32::MIN, f32::max);
        assert!(
            (minx - 0.0).abs() < 1e-5 && (maxx - 1.0).abs() < 1e-5,
            "{minx} {maxx}"
        );
    }

    #[test]
    fn uvlock_keeps_texture_orientation() {
        // верхняя грань при y=90 с uvlock: UV вращаются обратно, текстура остаётся «по миру»
        let lock = ModelState {
            y: Quadrant::R90,
            uv_lock: true,
            ..Default::default()
        };
        let free = ModelState {
            y: Quadrant::R90,
            ..Default::default()
        };
        let a = bake_elements(&cube(r#","uv":[0,0,8,16]"#), &slots(), &Sprites, &lock, "t");
        let b = bake_elements(&cube(r#","uv":[0,0,8,16]"#), &slots(), &Sprites, &free, "t");
        let ua = &a.culled[Direction::Up.index()][0].uv;
        let ub = &b.culled[Direction::Up.index()][0].uv;
        assert_ne!(ua, ub);
        // без uvlock набор UV прежний (порядок вершин меняет recalculateWinding)
        let mut sorted: Vec<[f32; 2]> = ub.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(sorted, vec![[0.0, 0.0], [0.0, 1.0], [0.5, 0.0], [0.5, 1.0]]);
    }

    #[test]
    fn missing_texture_uses_missingno() {
        let q = bake_elements(
            &cube(""),
            &TextureSlots::default(),
            &Sprites,
            &ModelState::default(),
            "t",
        );
        assert_eq!(q.culled[0][0].sprite.path, "missingno");
    }
}
