//! Опора блоков при постановке.
//! VANILLA: BlockState#isFaceSturdy + SupportType, Block#canSupportCenter/canSupportRigidBlock,
//! `canSurvive` классов Torch/WallTorch/Lantern/Bell/FaceAttached/Ladder/Sign/Banner/Door/Rail/
//! PressurePlate/Diode/RedStoneWire/Carpet/SnowLayer/Vegetation/SugarCane/Cactus/Vine.
//! VANILLA-UNSURE: форма опоры = форма коллизии (кроме листвы, ворот, грязи/песка душ).
//! Неизвестные классы (грибы, кувшинка, сцена и т. п.) считаем выживающими (как раньше).
use ezzmc_mesh::blockinfo as bi;
use ezzmc_world::block::{in_tag, BlockState, FLAG_SOLID};
use ezzmc_world::generated::t26_3::BLOCK_TAGS;

pub type Get<'a> = &'a dyn Fn(i32, i32, i32) -> BlockState;

/// Индексы `Direction#3DDataValue`: down, up, north, south, west, east.
const DIRS: [[i32; 3]; 6] = [
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
    [-1, 0, 0],
    [1, 0, 0],
];
const NAMES: [&str; 6] = ["down", "up", "north", "south", "west", "east"];
const DOWN: u8 = 0;
const UP: u8 = 1;

static FULL_BOX: [[f64; 6]; 1] = [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Support {
    Full,
    Center,
    Rigid,
}

fn dir_index(name: &str) -> Option<u8> {
    NAMES.iter().position(|n| *n == name).map(|i| i as u8)
}

fn facing(s: BlockState) -> Option<u8> {
    dir_index(s.property("facing")?)
}

fn tag(s: BlockState, t: &str) -> bool {
    in_tag(BLOCK_TAGS, t, s.block_id() as u32)
}

fn is_solid(s: BlockState) -> bool {
    s.props().is_some_and(|p| p.has(FLAG_SOLID))
}

/// VANILLA: BlockBehaviour#getBlockSupportShape.
fn support_boxes(s: BlockState) -> &'static [[f64; 6]] {
    let n = s.name();
    if n.ends_with("_leaves") {
        return &[];
    }
    if matches!(n, "mud" | "soul_sand") {
        return &FULL_BOX;
    }
    if n.ends_with("_fence_gate") && s.property("open") == Some("true") {
        return &[];
    }
    s.props()
        .map_or(&[], |p| bi::collision_boxes_of(p.collision_shape))
}

/// Закрыта ли точка (i, j) сетки 16×16 грани `face` (VoxelShape#getFaceShape).
fn covered(boxes: &[[f64; 6]], face: u8, i: usize, j: usize) -> bool {
    let (u, v) = ((i as f64 + 0.5) / 16.0, (j as f64 + 0.5) / 16.0);
    let (a, ua, va) = match face {
        0 | 1 => (1, 0, 2),
        2 | 3 => (2, 0, 1),
        _ => (0, 1, 2),
    };
    let plane = if face % 2 == 1 { 1.0 - 1e-7 } else { 1e-7 };
    boxes.iter().any(|b| {
        b[a] <= plane
            && b[a + 3] > plane
            && b[ua] <= u
            && u <= b[ua + 3]
            && b[va] <= v
            && v <= b[va + 3]
    })
}

/// VANILLA: BlockState#isFaceSturdy(level, pos, face, supportType).
pub fn sturdy(s: BlockState, face: u8, kind: Support) -> bool {
    let boxes = support_boxes(s);
    if boxes.is_empty() {
        return false;
    }
    (0..16).all(|i| {
        (0..16).all(|j| {
            let need = match kind {
                Support::Full => true,
                Support::Center => (7..9).contains(&i) && (7..9).contains(&j),
                Support::Rigid => !((2..14).contains(&i) && (2..14).contains(&j)),
            };
            !need || covered(boxes, face, i, j)
        })
    })
}

/// VANILLA: Block#canSupportCenter.
fn support_center(s: BlockState, face: u8) -> bool {
    if face == DOWN && tag(s, "unstable_bottom_center") {
        return false;
    }
    sturdy(s, face, Support::Center)
}

/// VANILLA: BlockState#canSurvive для состояния `s` в клетке `p`.
pub fn can_survive(s: BlockState, get: Get, p: [i32; 3]) -> bool {
    let n = s.name();
    let nb = |d: u8| {
        let v = DIRS[d as usize];
        get(p[0] + v[0], p[1] + v[1], p[2] + v[2])
    };
    let below = nb(DOWN);
    let above = nb(UP);

    // FaceAttachedHorizontalDirectionalBlock: рычаг, кнопки, точило.
    if n != "bell" {
        if let (Some(face), Some(f)) = (s.property("face"), facing(s)) {
            let connected = match face {
                "ceiling" => DOWN,
                "floor" => UP,
                _ => f,
            };
            return sturdy(nb(connected ^ 1), connected, Support::Full);
        }
    }
    if n == "bell" {
        return match s.property("attachment") {
            Some("ceiling") => support_center(above, DOWN),
            Some("floor") => sturdy(below, UP, Support::Full),
            _ => true,
        };
    }
    if n.ends_with("wall_torch") {
        let Some(f) = facing(s) else { return true };
        return sturdy(nb(f ^ 1), f, Support::Full);
    }
    if n.ends_with("torch") {
        return support_center(below, UP);
    }
    if n.ends_with("lantern") && s.property("hanging").is_some() {
        return if s.property("hanging") == Some("true") {
            support_center(above, DOWN)
        } else {
            support_center(below, UP)
        };
    }
    if n == "ladder" || n == "tripwire_hook" {
        let Some(f) = facing(s) else { return true };
        return sturdy(nb(f ^ 1), f, Support::Full);
    }
    if n.ends_with("_wall_sign") || n.ends_with("_wall_banner") {
        let Some(f) = facing(s) else { return true };
        return is_solid(nb(f ^ 1));
    }
    if (n.ends_with("_sign") && !n.contains("hanging")) || n.ends_with("_banner") {
        return is_solid(below);
    }
    if n.ends_with("_door") {
        return if s.property("half") == Some("upper") {
            below.name() == n
        } else {
            sturdy(below, UP, Support::Full)
        };
    }
    if matches!(
        n,
        "rail" | "powered_rail" | "detector_rail" | "activator_rail"
    ) || matches!(n, "repeater" | "comparator")
    {
        return sturdy(below, UP, Support::Rigid);
    }
    if n.ends_with("_pressure_plate") {
        return sturdy(below, UP, Support::Rigid) || support_center(below, UP);
    }
    if n == "redstone_wire" {
        return sturdy(below, UP, Support::Full) || below.name() == "hopper";
    }
    if n.ends_with("_carpet") {
        return !below.is_air();
    }
    if n == "snow" {
        return if tag(below, "cannot_support_snow_layer") {
            false
        } else if tag(below, "support_override_snow_layer") {
            true
        } else {
            sturdy(below, UP, Support::Full)
                || (below.name() == "snow" && below.property("layers") == Some("8"))
        };
    }
    if n == "vine" {
        return vine_survives(s, above, &nb);
    }
    if n == "cocoa" {
        let Some(f) = facing(s) else { return true };
        return tag(nb(f), "supports_cocoa");
    }
    if n == "sugar_cane" {
        return sugar_cane_survives(below, get, p);
    }
    if n == "cactus" {
        for d in 2..6u8 {
            let x = nb(d);
            if is_solid(x) || bi::fluid(x).is_some_and(|f| f.lava) {
                return false;
            }
        }
        let liquid = matches!(above.name(), "water" | "lava");
        return (below.name() == "cactus" || tag(below, "supports_cactus")) && !liquid;
    }
    // VANILLA: BambooStalkBlock/BambooSaplingBlock#canSurvive — под ними блок из #supports_bamboo.
    if matches!(n, "bamboo" | "bamboo_sapling") {
        return tag(below, "supports_bamboo");
    }
    // VANILLA: SeagrassBlock#mayPlaceOn (isFaceSturdy(UP) && !#cannot_support_seagrass);
    // TallSeagrass (DoublePlantBlock): верх живёт только над нижней половиной.
    if matches!(n, "seagrass" | "tall_seagrass") {
        return if s.property("half") == Some("upper") {
            below.name() == n && below.property("half") == Some("lower")
        } else {
            sturdy(below, UP, Support::Full) && !tag(below, "cannot_support_seagrass")
        };
    }
    // VANILLA: GrowingPlantBlock#canSurvive (kelp/kelp_plant растут вверх).
    if matches!(n, "kelp" | "kelp_plant") {
        return !tag(below, "cannot_support_kelp")
            && (matches!(below.name(), "kelp" | "kelp_plant") || sturdy(below, UP, Support::Full));
    }
    // VegetationBlock / BushBlock / CropBlock и др.
    let on = |t: &str| tag(below, t);
    let double_upper = s.property("half") == Some("upper");
    if tag(s, "small_flowers")
        || tag(s, "tall_flowers")
        || (tag(s, "saplings") && !n.contains("propagule") && !n.contains("azalea"))
        || matches!(
            n,
            "short_grass" | "fern" | "tall_grass" | "large_fern" | "sweet_berry_bush"
        )
    {
        return if double_upper {
            below.name() == n && below.property("half") == Some("lower")
        } else {
            on("supports_vegetation")
        };
    }
    if matches!(n, "dead_bush" | "short_dry_grass" | "tall_dry_grass") {
        return on("supports_dry_vegetation");
    }
    if matches!(
        n,
        "wheat" | "carrots" | "potatoes" | "beetroots" | "torchflower_crop"
    ) {
        return on("supports_crops");
    }
    true
}

/// VANILLA: BlockBehaviour#isSignalSource по имени (RedStoneWireBlock#shouldConnectTo).
/// VANILLA-UNSURE: список имён, а не `isSignalSource()` классов.
fn is_signal_source(s: BlockState) -> bool {
    let n = s.name();
    matches!(
        n,
        "lever"
            | "redstone_torch"
            | "redstone_wall_torch"
            | "redstone_block"
            | "comparator"
            | "daylight_detector"
            | "target"
            | "tripwire_hook"
            | "lectern"
            | "sculk_sensor"
            | "calibrated_sculk_sensor"
            | "lightning_rod"
            | "trapped_chest"
            | "detector_rail"
            | "observer"
            | "repeater"
            | "redstone_wire"
    ) || n.ends_with("_button")
        || n.ends_with("_pressure_plate")
}

/// VANILLA: RedStoneWireBlock#shouldConnectTo(state, direction); `dir` — индекс 2..5 или None.
fn wire_connects(s: BlockState, dir: Option<u8>) -> bool {
    match s.name() {
        "redstone_wire" => true,
        "repeater" => {
            let f = facing(s);
            dir.is_some_and(|d| f == Some(d) || f == Some(d ^ 1))
        }
        "observer" => dir.is_some() && facing(s) == dir,
        _ => is_signal_source(s) && dir.is_some(),
    }
}

/// VANILLA: RedStoneWireBlock#useWithoutItem — клик по точке/кресту меняет блок только если
/// `getConnectionState` даёт другое состояние (иначе PASS, например у настоящего креста).
pub fn wire_click_changes(s: BlockState, get: Get, p: [i32; 3]) -> bool {
    let sides = ["north", "south", "east", "west"].map(|n| match s.property(n) {
        Some("side") => 1u8,
        Some("up") => 2,
        _ => 0,
    });
    let cross = sides.iter().all(|&v| v != 0);
    let dot = sides.iter().all(|&v| v == 0);
    if !cross && !dot {
        return false;
    }
    let at = |d: usize, dy: i32| {
        let o = DIRS[d];
        get(p[0] + o[0], p[1] + dy, p[2] + o[2])
    };
    let above = get(p[0], p[1] + 1, p[2]);
    let can_climb = !is_solid(above);
    let on = |x: BlockState| sturdy(x, UP, Support::Full) || x.name() == "hopper";
    // getMissingConnections: стороны north, south, east, west = направления 2, 3, 5, 4.
    let mut m = [0u8; 4];
    for (i, d) in [2usize, 3, 5, 4].into_iter().enumerate() {
        let n = at(d, 0);
        let nd = Some(d as u8);
        let opp = (d as u8) ^ 1;
        m[i] = 'side: {
            if can_climb
                && (n.name().ends_with("_trapdoor") || on(n))
                && wire_connects(at(d, 1), None)
            {
                break 'side if sturdy(n, opp, Support::Full) { 2 } else { 1 };
            }
            if wire_connects(n, nd) || (!is_solid(n) && wire_connects(at(d, -1), None)) {
                1
            } else {
                0
            }
        };
    }
    // getConnectionState: у креста вход — «точка» (defaultBlockState), у точки — крест.
    if cross && m.iter().all(|&v| v == 0) {
        return sides != m;
    }
    let (n, so, e, w) = (m[0] != 0, m[1] != 0, m[2] != 0, m[3] != 0);
    let (ns, ew) = (!n && !so, !e && !w);
    if !w && ns {
        m[3] = 1;
    }
    if !e && ns {
        m[2] = 1;
    }
    if !n && ew {
        m[0] = 1;
    }
    if !so && ew {
        m[1] = 1;
    }
    sides != m
}

/// VANILLA: MultifaceBlock#canAttachTo — грань полная по форме опоры ИЛИ по форме коллизии
/// (листва: опора пуста, коллизия — куб, поэтому лоза на листву ставится).
fn can_attach_to(s: BlockState, face: u8) -> bool {
    if sturdy(s, face, Support::Full) {
        return true;
    }
    let boxes = s
        .props()
        .map_or(&[][..], |p| bi::collision_boxes_of(p.collision_shape));
    !boxes.is_empty() && (0..16).all(|i| (0..16).all(|j| covered(boxes, face, i, j)))
}

fn vine_survives(s: BlockState, above: BlockState, nb: &dyn Fn(u8) -> BlockState) -> bool {
    let mut any = false;
    // VANILLA: VineBlock#getUpdatedState — UP проверяется через isAcceptableNeighbour(…, DOWN).
    if s.property("up") == Some("true") && can_attach_to(above, DOWN) {
        any = true;
    }
    for (d, prop) in [(2u8, "north"), (3, "south"), (4, "west"), (5, "east")] {
        if s.property(prop) == Some("true")
            && (can_attach_to(nb(d), d ^ 1)
                || (above.name() == "vine" && above.property(prop) == Some("true")))
        {
            any = true;
        }
    }
    any
}

fn sugar_cane_survives(below: BlockState, get: Get, p: [i32; 3]) -> bool {
    if below.name() == "sugar_cane" {
        return true;
    }
    if !tag(below, "supports_sugar_cane") {
        return false;
    }
    [[-1, 0], [1, 0], [0, -1], [0, 1]].iter().any(|o| {
        let x = get(p[0] + o[0], p[1] - 1, p[2] + o[1]);
        bi::fluid(x).is_some_and(|f| !f.lava) || tag(x, "supports_sugar_cane_adjacently")
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn st(name: &str) -> BlockState {
        let info = ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == name)
            .unwrap();
        BlockState::from_id(info.default_state).unwrap()
    }

    /// Мир: клетки из карты, остальное — воздух.
    fn survives(s: BlockState, cells: &[([i32; 3], BlockState)]) -> bool {
        let m: HashMap<[i32; 3], BlockState> = cells.iter().copied().collect();
        let get = |x: i32, y: i32, z: i32| m.get(&[x, y, z]).copied().unwrap_or(BlockState::AIR);
        can_survive(s, &get, [0, 64, 0])
    }

    #[test]
    fn vine_attaches_to_leaves() {
        let v = st("vine").with("north", "true").unwrap();
        let leaves = st("oak_leaves");
        let get = |_x: i32, _y: i32, z: i32| if z == -1 { leaves } else { BlockState::AIR };
        assert!(can_survive(v, &get, [0, 64, 0]));
    }

    #[test]
    fn wire_click_only_when_state_changes() {
        let all = |v: &str| {
            let mut w = st("redstone_wire");
            for d in ["north", "south", "east", "west"] {
                w = w.with(d, v).unwrap();
            }
            w
        };
        let floor = ([0, 63, 0], st("stone"));
        let run = |s: BlockState, extra: &[([i32; 3], BlockState)]| {
            let mut c = vec![floor];
            c.extend_from_slice(extra);
            let m: HashMap<[i32; 3], BlockState> = c.into_iter().collect();
            let get =
                |x: i32, y: i32, z: i32| m.get(&[x, y, z]).copied().unwrap_or(BlockState::AIR);
            wire_click_changes(s, &get, [0, 64, 0])
        };
        // Одинокая точка → крест: меняется.
        assert!(run(all("none"), &[]));
        // Крест при пустом окружении → точка: меняется.
        assert!(run(all("side"), &[]));
        // Настоящий крест (провода со всех сторон): PASS.
        let n: Vec<_> = [[0, 64, -1], [0, 64, 1], [1, 64, 0], [-1, 64, 0]]
            .into_iter()
            .map(|p| (p, st("redstone_wire")))
            .chain([[0, 63, -1], [0, 63, 1], [1, 63, 0], [-1, 63, 0]].map(|p| (p, st("stone"))))
            .collect();
        assert!(!run(all("side"), &n));
        // Линия (не точка и не крест): PASS.
        assert!(!run(all("none").with("north", "side").unwrap(), &[]));
    }

    #[test]
    fn button_needs_full_face() {
        let btn = st("stone_button").with("face", "floor").unwrap();
        assert!(survives(btn, &[([0, 63, 0], st("stone"))]));
        assert!(!survives(btn, &[([0, 63, 0], st("oak_trapdoor"))]));
        let slab = st("stone_slab").with("type", "bottom").unwrap();
        assert!(!survives(btn, &[([0, 63, 0], slab)]));
        let top = st("stone_slab").with("type", "top").unwrap();
        assert!(survives(btn, &[([0, 63, 0], top)]));
        assert!(!survives(btn, &[]));
    }

    #[test]
    fn torch_needs_center() {
        let t = st("torch");
        assert!(survives(t, &[([0, 63, 0], st("stone"))]));
        assert!(!survives(t, &[]));
        let slab = st("stone_slab").with("type", "bottom").unwrap();
        assert!(!survives(t, &[([0, 63, 0], slab)]));
        assert!(survives(t, &[([0, 63, 0], st("oak_fence"))]));
        assert!(!survives(t, &[([0, 63, 0], st("oak_leaves"))]));
    }

    #[test]
    fn vine_needs_support() {
        let v = st("vine").with("north", "true").unwrap();
        assert!(!survives(v, &[]));
        assert!(survives(v, &[([0, 64, -1], st("stone"))]));
        let up = st("vine").with("up", "true").unwrap();
        assert!(survives(up, &[([0, 65, 0], st("stone"))]));
        assert!(!survives(up, &[]));
        // Лоза под лозой с той же гранью держится.
        let hung = [([0, 65, 0], v)];
        assert!(survives(v, &hung));
    }

    #[test]
    fn plants_and_rails() {
        assert!(survives(
            st("dandelion"),
            &[([0, 63, 0], st("grass_block"))]
        ));
        assert!(!survives(st("dandelion"), &[([0, 63, 0], st("stone"))]));
        assert!(!survives(st("rail"), &[]));
        assert!(survives(st("rail"), &[([0, 63, 0], st("stone"))]));
        assert!(!survives(st("oak_door"), &[]));
        assert!(survives(st("oak_door"), &[([0, 63, 0], st("stone"))]));
    }
}
