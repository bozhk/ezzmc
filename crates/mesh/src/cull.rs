//! Отсечение граней: порт `Block#shouldRenderFace` (TODO-136).
//!
//! `getFaceOcclusionShape`: форма окклюзии = контур (`getShape`) для блоков с `canOcclude`, иначе пустая.
//! VANILLA-UNSURE: переопределения `getOcclusionShape` у FenceBlock/FenceGateBlock/Lectern/SculkShrieker
//! не сняты в данных — используется контур; на результат отсечения (не полный куб в обе стороны) не влияет.
use ezzmc_assets::model::Direction;
use ezzmc_world::block::{BlockState, FLAG_CAN_OCCLUDE};
use ezzmc_world::generated::p26_3::SHAPES;
use std::cell::RefCell;
use std::collections::HashMap;

/// Прямоугольник проекции грани: (u0, v0, u1, v1) в долях блока.
type Rect = [f64; 4];

const EPS: f64 = 1e-7;

fn outline(state: BlockState) -> &'static [[f64; 6]] {
    match state.props() {
        Some(p) if p.has(FLAG_CAN_OCCLUDE) => SHAPES
            .get(usize::from(p.outline_shape))
            .copied()
            .unwrap_or(&[]),
        _ => &[],
    }
}

fn is_full_block(boxes: &[[f64; 6]]) -> bool {
    boxes.iter().any(|b| {
        b[0] <= 0.0 && b[1] <= 0.0 && b[2] <= 0.0 && b[3] >= 1.0 && b[4] >= 1.0 && b[5] >= 1.0
    })
}

/// VANILLA: VoxelShape#getFaceShape — боксы, касающиеся плоскости грани, спроецированные в 2D.
fn face_rects(boxes: &[[f64; 6]], dir: Direction) -> Vec<Rect> {
    let mut out = Vec::new();
    for b in boxes {
        let (touch, r) = match dir {
            Direction::Down => (b[1] <= EPS, [b[0], b[2], b[3], b[5]]),
            Direction::Up => (b[4] >= 1.0 - EPS, [b[0], b[2], b[3], b[5]]),
            Direction::North => (b[2] <= EPS, [b[0], b[1], b[3], b[4]]),
            Direction::South => (b[5] >= 1.0 - EPS, [b[0], b[1], b[3], b[4]]),
            Direction::West => (b[0] <= EPS, [b[2], b[1], b[5], b[4]]),
            Direction::East => (b[3] >= 1.0 - EPS, [b[2], b[1], b[5], b[4]]),
        };
        if touch && r[2] > r[0] && r[3] > r[1] {
            out.push(r);
        }
    }
    out
}

fn covers(rects: &[Rect], cx: f64, cy: f64) -> bool {
    rects
        .iter()
        .any(|r| cx > r[0] && cx < r[2] && cy > r[1] && cy < r[3])
}

/// VANILLA: `Shapes.joinIsNotEmpty(a, b, ONLY_FIRST)` для плоских граней: есть ли часть `a`, не закрытая `b`.
fn first_not_covered(a: &[Rect], b: &[Rect]) -> bool {
    let mut xs: Vec<f64> = a.iter().chain(b).flat_map(|r| [r[0], r[2]]).collect();
    let mut ys: Vec<f64> = a.iter().chain(b).flat_map(|r| [r[1], r[3]]).collect();
    xs.sort_by(f64::total_cmp);
    ys.sort_by(f64::total_cmp);
    xs.dedup();
    ys.dedup();
    for x in xs.windows(2) {
        for y in ys.windows(2) {
            let (cx, cy) = ((x[0] + x[1]) * 0.5, (y[0] + y[1]) * 0.5);
            if covers(a, cx, cy) && !covers(b, cx, cy) {
                return true;
            }
        }
    }
    false
}

thread_local! {
    // VANILLA: Block#OCCLUSION_CACHE (ключ — пара форм; здесь — пара состояний+направление)
    static CACHE: RefCell<HashMap<(u16, u16, u8), bool>> = RefCell::new(HashMap::new());
}

/// Семейства, у которых соседний блок того же типа скрывает грань
/// (`HalfTransparentBlock`/`PowderSnowBlock`: `$$1.is(this)`).
// VANILLA-UNSURE: список имён собран по иерархии классов (TransparentBlock/HalfTransparentBlock/IceBlock/
// WaterloggedTransparentBlock), в декомпилированном client-источнике нет таблицы Blocks → имён.
fn skips_same_block(name: &str) -> bool {
    matches!(
        name,
        "glass"
            | "tinted_glass"
            | "ice"
            | "frosted_ice"
            | "blue_ice"
            | "honey_block"
            | "slime_block"
            | "powder_snow"
    ) || name.ends_with("_stained_glass")
        || name.ends_with("copper_grate")
}

/// Наследники IronBarsBlock: решётки, панели стекла, медные решётки.
fn is_iron_bars_block(n: &str) -> bool {
    n == "iron_bars" || n == "glass_pane" || n.ends_with("_stained_glass_pane") || is_bars_tag(n)
}

/// Тег `minecraft:bars` (железная и медные решётки).
fn is_bars_tag(n: &str) -> bool {
    n == "iron_bars" || n.ends_with("copper_bars")
}

fn dir_prop(d: Direction) -> Option<&'static str> {
    match d {
        Direction::North => Some("north"),
        Direction::South => Some("south"),
        Direction::West => Some("west"),
        Direction::East => Some("east"),
        Direction::Down | Direction::Up => None,
    }
}

/// VANILLA: IronBarsBlock#skipRendering — соседняя решётка того же блока (или оба из тега BARS):
/// вертикальные грани скрыты, боковые — если обе стороны соединены.
fn iron_bars_skip(state: BlockState, neighbor: BlockState, dir: Direction) -> bool {
    let (n, nn) = (state.name(), neighbor.name());
    let opp_prop = dir_prop(opposite(dir));
    let tag_match = is_bars_tag(nn)
        && is_bars_tag(n)
        && opp_prop.is_some_and(|p| neighbor.property(p).is_some());
    if !(n == nn || tag_match) {
        return false;
    }
    let Some(prop) = dir_prop(dir) else {
        return true;
    };
    state.property(prop) == Some("true")
        && opp_prop.is_some_and(|p| neighbor.property(p) == Some("true"))
}

/// VANILLA: BlockBehaviour#skipRendering (Block, HalfTransparentBlock, PowderSnowBlock, LeavesBlock, MangroveRootsBlock).
pub fn skip_rendering(
    state: BlockState,
    neighbor: BlockState,
    dir: Direction,
    cutout_leaves: bool,
) -> bool {
    let (n, nn) = (state.name(), neighbor.name());
    if skips_same_block(n) && n == nn {
        return true;
    }
    if is_iron_bars_block(n) && iron_bars_skip(state, neighbor, dir) {
        return true;
    }
    if !cutout_leaves && n.ends_with("_leaves") && nn.ends_with("_leaves") {
        return true;
    }
    n == "mangrove_roots"
        && nn == "mangrove_roots"
        && matches!(dir, Direction::Up | Direction::Down)
}

/// VANILLA: Block#shouldRenderFace(state, neighborState, direction)
pub fn should_render_face(
    state: BlockState,
    neighbor: BlockState,
    dir: Direction,
    cutout_leaves: bool,
) -> bool {
    let n_boxes = outline(neighbor);
    let opp = opposite(dir);
    let n_face = face_rects(n_boxes, opp);
    if is_full_block(n_boxes) {
        return false;
    }
    if skip_rendering(state, neighbor, dir, cutout_leaves) {
        return false;
    }
    if n_face.is_empty() {
        return true;
    }
    let s_face = face_rects(outline(state), dir);
    if s_face.is_empty() {
        return true;
    }
    let key = (state.0, neighbor.0, dir.index() as u8);
    if let Some(v) = CACHE.with(|c| c.borrow().get(&key).copied()) {
        return v;
    }
    let v = first_not_covered(&s_face, &n_face);
    CACHE.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= 4096 {
            c.clear();
        }
        c.insert(key, v);
    });
    v
}

/// VANILLA: LiquidBlockRenderer#isFaceOccludedByState(dir, height, state) — грань жидкости высотой `height`
/// (в сторону `dir`) закрыта соседом `state`.
pub fn fluid_face_occluded(dir: Direction, height: f32, state: BlockState) -> bool {
    let boxes = outline(state);
    let n_face = face_rects(boxes, opposite(dir));
    if n_face.is_empty() {
        return false;
    }
    if is_full_block(boxes) {
        return dir != Direction::Up || height == 1.0;
    }
    let h = f64::from(height);
    // VANILLA: Shapes.blockOccludes(box(0,0,0,1,h,1), neighborFace, dir)
    let own: Vec<Rect> = match dir {
        Direction::Down => vec![[0.0, 0.0, 1.0, 1.0]],
        Direction::Up => {
            if h < 1.0 - EPS {
                return false;
            }
            vec![[0.0, 0.0, 1.0, 1.0]]
        }
        _ => vec![[0.0, 0.0, 1.0, h]],
    };
    !first_not_covered(&own, &n_face)
}

pub fn opposite(d: Direction) -> Direction {
    match d {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}

/// Смещение соседа по направлению.
pub fn offset(d: Direction) -> [i32; 3] {
    match d {
        Direction::Down => [0, -1, 0],
        Direction::Up => [0, 1, 0],
        Direction::North => [0, 0, -1],
        Direction::South => [0, 0, 1],
        Direction::West => [-1, 0, 0],
        Direction::East => [1, 0, 0],
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn st(name: &str, props: &[(&str, &str)]) -> BlockState {
        let info = ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == name)
            .unwrap();
        let mut b = BlockState::from_id(info.default_state).unwrap();
        for (k, v) in props {
            b = b.with(k, v).unwrap();
        }
        b
    }

    #[test]
    fn stone_next_to_stone_is_culled_air_is_not() {
        let stone = st("stone", &[]);
        for d in Direction::ALL {
            assert!(!should_render_face(stone, stone, d, true));
            assert!(should_render_face(stone, BlockState::AIR, d, true));
        }
    }

    #[test]
    fn glass_same_block_skipped_but_stone_behind_glass_visible() {
        let glass = st("glass", &[]);
        let stone = st("stone", &[]);
        assert!(!should_render_face(glass, glass, Direction::East, true)); // skipRendering
        assert!(!should_render_face(glass, stone, Direction::East, true)); // закрыт полным кубом
        assert!(should_render_face(stone, glass, Direction::East, true)); // glass не окклюдит
    }

    #[test]
    fn slab_partial_occlusion() {
        let stone = st("stone", &[]);
        let bottom = st(
            "stone_slab",
            &[("type", "bottom"), ("waterlogged", "false")],
        );
        let top = st("stone_slab", &[("type", "top"), ("waterlogged", "false")]);
        // бок камня рядом с нижней плитой виден (плита закрывает только низ)
        assert!(should_render_face(stone, bottom, Direction::East, true));
        // нижняя плита над камнем: её низ полный → грань UP камня скрыта; у верхней плиты низ на y=0.5 → виден
        assert!(!should_render_face(stone, bottom, Direction::Up, true));
        assert!(should_render_face(stone, top, Direction::Up, true));
        // грань плиты, прижатая к полному кубу — скрыта; к воздуху — видна
        assert!(!should_render_face(bottom, stone, Direction::East, true));
        assert!(should_render_face(
            bottom,
            BlockState::AIR,
            Direction::Up,
            true
        ));
    }

    #[test]
    fn leaves_fast_graphics_skip() {
        let l = st(
            "oak_leaves",
            &[
                ("distance", "7"),
                ("persistent", "false"),
                ("waterlogged", "false"),
            ],
        );
        assert!(should_render_face(l, l, Direction::East, true));
        assert!(!should_render_face(l, l, Direction::East, false));
    }

    #[test]
    fn iron_bars_skip_connected_sides_only() {
        let bars = |name: &str, props: &[(&str, &str)]| {
            let b = ezzmc_world::generated::b26_3::BLOCKS
                .iter()
                .find(|b| b.name == name)
                .unwrap();
            let mut s = BlockState::from_id(b.default_state).unwrap();
            for (k, v) in props {
                s = s.with(k, v).unwrap();
            }
            s
        };
        let a = bars("copper_bars", &[("east", "true")]);
        let b = bars("copper_bars", &[("west", "true")]);
        assert!(iron_bars_skip(a, b, Direction::East));
        // Соседняя решётка без соединения к нам — грань остаётся.
        let c = bars("copper_bars", &[]);
        assert!(!iron_bars_skip(a, c, Direction::East));
        // Вертикальные грани между решётками скрыты всегда.
        assert!(iron_bars_skip(a, c, Direction::Up));
        // Другой материал (тег BARS) тоже считается.
        let w = bars("waxed_copper_bars", &[("west", "true")]);
        assert!(iron_bars_skip(a, w, Direction::East));
    }
}
