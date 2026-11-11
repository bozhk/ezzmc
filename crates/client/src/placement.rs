//! Предсказание состояния блока при постановке (VANILLA: Block#getStateForPlacement, BlockPlaceContext).
//! Не считаем то, что требует соседей/canSurvive: форму ступенек, тип сундука, double-плиту,
//! `open`/`powered` по сигналу, `in_wall` ворот. Сервер поправит это ответом.
use ezzmc_world::block::BlockState;

/// Индексы совпадают с `Direction#3DDataValue`: down, up, north, south, west, east.
const NAMES: [&str; 6] = ["down", "up", "north", "south", "west", "east"];
const DOWN: u8 = 0;
const UP: u8 = 1;

/// Контекст постановки (BlockPlaceContext).
pub struct Ctx {
    /// Клетка постановки (`getClickedPos`).
    pub pos: [i32; 3],
    /// Грань кликнутого блока (`getClickedFace`).
    pub face: u8,
    /// Точка клика в мировых координатах (`getClickLocation`).
    pub click: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    /// `replacingClickedOnBlock`.
    pub replace_clicked: bool,
    /// Текущее состояние в клетке постановки (лоза сливается с уже стоящей).
    pub here: BlockState,
}

fn dir_index(name: &str) -> Option<u8> {
    NAMES.iter().position(|n| *n == name).map(|i| i as u8)
}

fn offset(pos: [i32; 3], d: u8) -> [i32; 3] {
    let (dx, dy, dz) = match d {
        0 => (0, -1, 0),
        1 => (0, 1, 0),
        2 => (0, 0, -1),
        3 => (0, 0, 1),
        4 => (-1, 0, 0),
        _ => (1, 0, 0),
    };
    [pos[0] + dx, pos[1] + dy, pos[2] + dz]
}

/// VANILLA: StairBlock#getStairsShape (+ canTakeShape) — форма ступеньки по соседям (внутренний/
/// внешний угол). Для не-ступенек состояние возвращается как есть.
pub fn with_stairs_shape(
    state: BlockState,
    pos: [i32; 3],
    get: &dyn Fn(i32, i32, i32) -> BlockState,
) -> BlockState {
    if !state.name().ends_with("_stairs") {
        return state;
    }
    let (Some(facing), Some(half)) = (
        state.property("facing").and_then(dir_index),
        state.property("half"),
    ) else {
        return state;
    };
    let nb = |d: u8| {
        let p = offset(pos, d);
        get(p[0], p[1], p[2])
    };
    let stairs = |s: BlockState| s.name().ends_with("_stairs");
    let nfacing = |s: BlockState| s.property("facing").and_then(dir_index);
    let can_take = |d: u8| {
        let n = nb(d);
        !stairs(n) || nfacing(n) != Some(facing) || n.property("half") != Some(half)
    };
    let ccw = opposite(clockwise(facing));
    let shape = (|| {
        let ahead = nb(facing);
        if stairs(ahead) && ahead.property("half") == Some(half) {
            if let Some(af) = nfacing(ahead) {
                if axis(af) != axis(facing) && can_take(opposite(af)) {
                    return if af == ccw {
                        "outer_left"
                    } else {
                        "outer_right"
                    };
                }
            }
        }
        let back = nb(opposite(facing));
        if stairs(back) && back.property("half") == Some(half) {
            if let Some(bf) = nfacing(back) {
                if axis(bf) != axis(facing) && can_take(bf) {
                    return if bf == ccw {
                        "inner_left"
                    } else {
                        "inner_right"
                    };
                }
            }
        }
        "straight"
    })();
    state.with("shape", shape).unwrap_or(state)
}

fn opposite(d: u8) -> u8 {
    d ^ 1
}

fn horizontal(d: u8) -> bool {
    d >= 2
}

/// Direction#getAxis: 0 = X, 1 = Y, 2 = Z.
fn axis(d: u8) -> u8 {
    match d {
        0 | 1 => 1,
        2 | 3 => 2,
        _ => 0,
    }
}

fn clockwise(d: u8) -> u8 {
    match d {
        2 => 5,
        5 => 3,
        3 => 4,
        4 => 2,
        d => d,
    }
}

/// VANILLA: Direction#fromYRot (Entity#getDirection).
fn horizontal_direction(yaw: f32) -> u8 {
    const BY_Q: [u8; 4] = [3, 4, 2, 5]; // юг, запад, север, восток
    BY_Q[((f64::from(yaw) / 90.0 + 0.5).floor() as i32).rem_euclid(4) as usize]
}

/// VANILLA: AbstractBedBlock#getStateForPlacement / BedBlock#setPlacedBy — клетка головы кровати
/// (`pos.relative(facing)`, facing — горизонтальное направление игрока).
pub fn bed_head_pos(pos: [i32; 3], yaw: f32) -> [i32; 3] {
    let d = match horizontal_direction(yaw) {
        2 => [0, 0, -1],
        3 => [0, 0, 1],
        4 => [-1, 0, 0],
        _ => [1, 0, 0],
    };
    [pos[0] + d[0], pos[1], pos[2] + d[2]]
}

/// VANILLA: Direction#orderedByNearest.
fn ordered_by_nearest(yaw: f32, pitch: f32) -> [u8; 6] {
    let p = pitch * (std::f32::consts::PI / 180.0);
    let y = -yaw * (std::f32::consts::PI / 180.0);
    let (ps, pc, ys, yc) = (p.sin(), p.cos(), y.sin(), y.cos());
    let (x_pos, y_pos, z_pos) = (ys > 0.0, ps < 0.0, yc > 0.0);
    let (x_yaw, y_mag, z_yaw) = (ys.abs(), ps.abs(), yc.abs());
    let (x_mag, z_mag) = (x_yaw * pc, z_yaw * pc);
    let ax = if x_pos { 5 } else { 4 };
    let ay = if y_pos { UP } else { DOWN };
    let az = if z_pos { 3 } else { 2 };
    let (a, b, c) = if x_yaw > z_yaw {
        if y_mag > x_mag {
            (ay, ax, az)
        } else if z_mag > y_mag {
            (ax, az, ay)
        } else {
            (ax, ay, az)
        }
    } else if y_mag > z_mag {
        (ay, az, ax)
    } else if x_mag > y_mag {
        (az, ax, ay)
    } else {
        (az, ay, ax)
    };
    [a, b, c, opposite(c), opposite(b), opposite(a)]
}

/// VANILLA: BlockPlaceContext#getNearestLookingDirections.
fn nearest_directions(c: &Ctx) -> [u8; 6] {
    let mut d = ordered_by_nearest(c.yaw, c.pitch);
    if !c.replace_clicked {
        let want = opposite(c.face);
        if let Some(i) = d.iter().position(|&x| x == want) {
            d.copy_within(0..i, 1);
            d[0] = want;
        }
    }
    d
}

/// VANILLA: RotationSegment#convertToSegment (SegmentedAnglePrecision(4)).
fn segment(degrees: f32) -> i32 {
    ((degrees * 16.0 / 360.0 + 0.5).floor() as i32) & 15
}

fn block_default(name: &str) -> Option<BlockState> {
    let info = ezzmc_world::generated::b26_3::BLOCKS
        .iter()
        .find(|b| b.name == name)?;
    BlockState::from_id(info.default_state)
}

fn upper_half(c: &Ctx) -> bool {
    // clickedFace != DOWN && (clickedFace == UP || !(click.y - pos.y > 0.5)) → нижняя
    !(c.face != DOWN && (c.face == UP || c.click[1] - f64::from(c.pos[1]) <= 0.5))
}

fn set(st: BlockState, prop: &str, v: &str) -> Option<BlockState> {
    st.with(prop, v)
}

/// Результат расчёта постановки.
pub enum Placed {
    /// Состояние посчитано.
    State(BlockState),
    /// Ваниль вернула бы `null` / блок не выживет: постановки нет.
    Never,
}

/// VANILLA: BambooStalkBlock/SeagrassBlock/KelpBlock#getStateForPlacement — растения воды и бамбук.
/// Бамбук не ставится в клетку с любой жидкостью; ламинария и морская трава — только в полную воду
/// (`fluid.is(WATER) && isFull`). `None` — предмет не из этой группы.
pub fn fluid_plant(
    name: &str,
    c: &Ctx,
    get: &dyn Fn(i32, i32, i32) -> BlockState,
    ok: &dyn Fn(BlockState) -> bool,
) -> Option<Placed> {
    let fluid = ezzmc_mesh::blockinfo::fluid(c.here);
    let full_water = fluid.is_some_and(|f| !f.lava && f.amount == 8);
    let st = match name {
        "bamboo" => {
            if fluid.is_some() {
                return Some(Placed::Never);
            }
            let below = get(c.pos[0], c.pos[1] - 1, c.pos[2]);
            let above = get(c.pos[0], c.pos[1] + 1, c.pos[2]);
            if !ezzmc_world::block::in_tag(
                ezzmc_world::generated::t26_3::BLOCK_TAGS,
                "supports_bamboo",
                below.block_id() as u32,
            ) {
                return Some(Placed::Never);
            }
            let stalk = block_default("bamboo")?;
            if below.name() == "bamboo_sapling" {
                stalk.with("age", "0")?
            } else if below.name() == "bamboo" {
                stalk.with(
                    "age",
                    if below.property("age") == Some("0") {
                        "0"
                    } else {
                        "1"
                    },
                )?
            } else if above.name() == "bamboo" {
                stalk.with("age", above.property("age")?)?
            } else {
                block_default("bamboo_sapling")?
            }
        }
        "seagrass" | "kelp" => {
            if !full_water {
                return Some(Placed::Never);
            }
            block_default(name)?
        }
        _ => return None,
    };
    Some(if ok(st) {
        Placed::State(st)
    } else {
        Placed::Never
    })
}

/// VANILLA: BlockItem#getPlacementState + canPlace (mustSurvive). `ok` — `canSurvive` состояния в
/// клетке постановки. Блок без особой логики ставится как `default_state`.
pub fn place_state(name: &str, c: &Ctx, ok: &dyn Fn(BlockState) -> bool) -> Placed {
    let Some(base) = block_default(name) else {
        return Placed::Never;
    };
    match state_for_placement(name, c, ok) {
        Ok(Some(s)) if ok(s) => Placed::State(s),
        Ok(Some(_)) | Err(()) => Placed::Never,
        Ok(None) if ok(base) => Placed::State(base),
        Ok(None) => Placed::Never,
    }
}

/// `Ok(None)` — не считаем (берётся `default_state`), `Err` — ваниль вернула `null`.
fn state_for_placement(
    name: &str,
    c: &Ctx,
    ok: &dyn Fn(BlockState) -> bool,
) -> Result<Option<BlockState>, ()> {
    let Some(base) = block_default(name) else {
        return Ok(None);
    };
    let r = state_inner(name, c, ok, base);
    match r {
        Some(s) => Ok(Some(s)),
        None if is_looping(name) => Err(()),
        None => Ok(None),
    }
}

/// Блоки, у которых `None` из расчёта = ваниль не нашла подходящего направления.
fn is_looping(name: &str) -> bool {
    name == "lever"
        || name == "grindstone"
        || name == "ladder"
        || name == "vine"
        || name == "bell"
        || name.ends_with("_button")
        || name.ends_with("lantern")
        || name.ends_with("torch")
        || name.ends_with("_sign")
        || name.ends_with("_banner")
        || name.ends_with("_skull")
        || name.ends_with("_head")
}

fn state_inner(
    name: &str,
    c: &Ctx,
    ok: &dyn Fn(BlockState) -> bool,
    base: BlockState,
) -> Option<BlockState> {
    let hdir = horizontal_direction(c.yaw);
    let nearest = ordered_by_nearest(c.yaw, c.pitch)[0];

    // Facing = противоположно взгляду (поршни, раздатчики, бочка, командные блоки).
    if matches!(
        name,
        "piston"
            | "sticky_piston"
            | "dispenser"
            | "dropper"
            | "barrel"
            | "command_block"
            | "chain_command_block"
            | "repeating_command_block"
    ) {
        return set(base, "facing", NAMES[opposite(nearest) as usize]);
    }
    if name == "observer" {
        return set(base, "facing", NAMES[nearest as usize]);
    }
    // Facing = противоположно горизонтальному направлению игрока.
    if matches!(
        name,
        "furnace"
            | "blast_furnace"
            | "smoker"
            | "repeater"
            | "comparator"
            | "carved_pumpkin"
            | "jack_o_lantern"
            | "loom"
            | "stonecutter"
            | "beehive"
            | "bee_nest"
            | "chest"
            | "trapped_chest"
            | "ender_chest"
            | "lectern"
            | "end_portal_frame"
            | "chiseled_bookshelf"
            | "vault"
            | "dried_ghast"
            | "big_dripleaf"
            | "small_dripleaf"
    ) || name.ends_with("_glazed_terracotta")
        || name.ends_with("_shelf")
        || name.ends_with("copper_chest")
        || name.ends_with("copper_golem_statue")
    {
        return set(base, "facing", NAMES[opposite(hdir) as usize]);
    }
    // Facing = горизонтальное направление игрока.
    if matches!(
        name,
        "campfire" | "soul_campfire" | "decorated_pot" | "calibrated_sculk_sensor"
    ) {
        return set(base, "facing", NAMES[hdir as usize]);
    }
    // VANILLA: AbstractBedBlock#getStateForPlacement (нога; голову ставит setPlacedBy).
    if name.ends_with("_bed") {
        return set(base, "facing", NAMES[hdir as usize]);
    }
    // CocoaBlock / TripWireHookBlock: первое горизонтальное из nearest (без canSurvive).
    if name == "cocoa" || name == "tripwire_hook" {
        let d = nearest_directions(c).into_iter().find(|&d| horizontal(d))?;
        let f = if name == "cocoa" { d } else { opposite(d) };
        return set(base, "facing", NAMES[f as usize]);
    }
    // CrafterBlock: FrontAndTop из nearest looking.
    if name == "crafter" {
        let front = opposite(nearest);
        let top = match front {
            DOWN => opposite(hdir),
            UP => hdir,
            _ => UP,
        };
        let o = format!("{}_{}", NAMES[front as usize], NAMES[top as usize]);
        return set(base, "orientation", &o);
    }
    // JigsawBlock.
    if name == "jigsaw" {
        let top = if axis(c.face) == 1 {
            opposite(hdir)
        } else {
            UP
        };
        let o = format!("{}_{}", NAMES[c.face as usize], NAMES[top as usize]);
        return set(base, "orientation", &o);
    }
    if name.ends_with("anvil") {
        return set(base, "facing", NAMES[clockwise(hdir) as usize]);
    }
    if name == "hopper" {
        let d = opposite(c.face);
        return set(
            base,
            "facing",
            NAMES[if axis(d) == 1 { DOWN } else { d } as usize],
        );
    }
    if name.ends_with("shulker_box")
        || name.contains("amethyst_bud")
        || name == "amethyst_cluster"
        || name == "end_rod"
        || name == "lightning_rod"
    {
        return set(base, "facing", NAMES[c.face as usize]);
    }
    if base.property("axis").is_some() && matches!(base.property("axis"), Some("x" | "y" | "z")) {
        return set(base, "axis", ["x", "y", "z"][axis(c.face) as usize]);
    }
    if name.ends_with("_stairs") {
        let half = if upper_half(c) { "top" } else { "bottom" };
        return set(base, "facing", NAMES[hdir as usize]).and_then(|s| set(s, "half", half));
    }
    if name.ends_with("_slab") {
        let ty = if upper_half(c) { "top" } else { "bottom" };
        return set(base, "type", ty);
    }
    if name.ends_with("_fence_gate") {
        return set(base, "facing", NAMES[hdir as usize]);
    }
    if name.ends_with("_trapdoor") {
        let (facing, half) = if !c.replace_clicked && horizontal(c.face) {
            let top = c.click[1] - f64::from(c.pos[1]) > 0.5;
            (c.face, top)
        } else {
            (opposite(hdir), c.face != UP)
        };
        return set(base, "facing", NAMES[facing as usize])
            .and_then(|s| set(s, "half", if half { "top" } else { "bottom" }));
    }
    // FaceAttachedHorizontalDirectionalBlock: рычаг, кнопки, точило.
    if name == "lever" || name == "grindstone" || name.ends_with("_button") {
        return nearest_directions(c).into_iter().find_map(|d| {
            let s = if axis(d) == 1 {
                set(base, "face", if d == UP { "ceiling" } else { "floor" })
                    .and_then(|s| set(s, "facing", NAMES[hdir as usize]))
            } else {
                set(base, "face", "wall")
                    .and_then(|s| set(s, "facing", NAMES[opposite(d) as usize]))
            };
            s.filter(|&s| ok(s))
        });
    }
    if name == "ladder" {
        return nearest_directions(c)
            .into_iter()
            .filter(|&d| horizontal(d))
            .find_map(|d| set(base, "facing", NAMES[opposite(d) as usize]).filter(|&s| ok(s)));
    }
    if name.ends_with("lantern") && base.property("hanging").is_some() {
        return nearest_directions(c)
            .into_iter()
            .filter(|&d| axis(d) == 1)
            .find_map(|d| {
                set(base, "hanging", if d == UP { "true" } else { "false" }).filter(|&s| ok(s))
            });
    }
    if name == "bell" {
        // VANILLA-UNSURE: двойная настенная подвеска (double_wall) не считается.
        let s = if axis(c.face) == 1 {
            set(
                base,
                "attachment",
                if c.face == DOWN { "ceiling" } else { "floor" },
            )
            .and_then(|s| set(s, "facing", NAMES[hdir as usize]))
        } else {
            set(base, "attachment", "single_wall")
                .and_then(|s| set(s, "facing", NAMES[opposite(c.face) as usize]))
        };
        return s.filter(|&s| ok(s));
    }
    if name == "vine" {
        // VANILLA: VineBlock#getStateForPlacement — в клетке с лозой новая грань добавляется к её
        // состоянию (опора проверяется для одной грани), а не заменяет его.
        let is_vine = c.here.name() == "vine";
        let cur = if is_vine { c.here } else { base };
        let merged = nearest_directions(c)
            .into_iter()
            .filter(|&d| d != DOWN)
            .filter(|&d| !(is_vine && cur.property(NAMES[d as usize]) == Some("true")))
            .find_map(|d| {
                set(base, NAMES[d as usize], "true")
                    .filter(|&s| ok(s))
                    .and_then(|_| set(cur, NAMES[d as usize], "true"))
            });
        return merged.or(is_vine.then_some(cur));
    }
    // StandingAndWallBlockItem: стоячий вариант (поворот/опора) или настенный.
    let standing_and_wall = |wall: String, rotation: f32| -> Option<BlockState> {
        let wall_base = block_default(&wall)?;
        nearest_directions(c)
            .into_iter()
            .filter(|&d| d != UP)
            .find_map(|d| {
                let s = if d == DOWN {
                    let rot = segment(rotation).to_string();
                    if base.property("rotation").is_some() {
                        set(base, "rotation", &rot)
                    } else {
                        Some(base)
                    }
                } else {
                    set(wall_base, "facing", NAMES[opposite(d) as usize])
                };
                s.filter(|&s| ok(s))
            })
    };
    if name.ends_with("torch") && !name.contains("wall") {
        return standing_and_wall(name.replace("torch", "wall_torch"), 0.0);
    }
    if name.ends_with("_sign") && !name.ends_with("_wall_sign") {
        return standing_and_wall(name.replace("_sign", "_wall_sign"), c.yaw + 180.0);
    }
    if name.ends_with("_banner") && !name.ends_with("_wall_banner") {
        return standing_and_wall(name.replace("_banner", "_wall_banner"), c.yaw + 180.0);
    }
    if let Some(p) = name.strip_suffix("_skull") {
        return standing_and_wall(format!("{p}_wall_skull"), c.yaw);
    }
    if let Some(p) = name.strip_suffix("_head") {
        return standing_and_wall(format!("{p}_wall_head"), c.yaw);
    }
    None
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn ctx(face: u8, yaw: f32, pitch: f32) -> Ctx {
        Ctx {
            pos: [0, 64, 0],
            face,
            click: [0.5, 64.5, 0.5],
            yaw,
            pitch,
            replace_clicked: false,
            here: BlockState::AIR,
        }
    }

    fn sfp(name: &str, c: &Ctx) -> Option<BlockState> {
        match place_state(name, c, &|_| true) {
            Placed::State(s) => Some(s),
            Placed::Never => None,
        }
    }

    fn prop(name: &str, c: &Ctx, p: &str) -> Option<&'static str> {
        sfp(name, c).and_then(|s| s.property(p))
    }

    #[test]
    fn piston_faces_player() {
        // Игрок смотрит на юг (yaw 0): поршень «смотрит» на игрока, то есть на север.
        assert_eq!(prop("piston", &ctx(1, 0.0, 0.0), "facing"), Some("north"));
        // Смотрит вниз: поршень смотрит вверх.
        assert_eq!(prop("piston", &ctx(1, 0.0, 80.0), "facing"), Some("up"));
        assert_eq!(prop("observer", &ctx(1, 0.0, 0.0), "facing"), Some("south"));
    }

    #[test]
    fn furnace_and_stairs() {
        assert_eq!(prop("furnace", &ctx(1, 0.0, 0.0), "facing"), Some("north"));
        assert_eq!(prop("furnace", &ctx(1, 90.0, 0.0), "facing"), Some("east"));
        assert_eq!(
            prop("oak_stairs", &ctx(1, 90.0, 0.0), "facing"),
            Some("west")
        );
        assert_eq!(prop("oak_stairs", &ctx(0, 0.0, 0.0), "half"), Some("top"));
    }

    #[test]
    fn log_axis_and_button() {
        assert_eq!(prop("oak_log", &ctx(4, 0.0, 0.0), "axis"), Some("x"));
        assert_eq!(
            prop("stone_button", &ctx(1, 0.0, 0.0), "face"),
            Some("floor")
        );
        assert_eq!(
            prop("stone_button", &ctx(3, 0.0, 0.0), "face"),
            Some("wall")
        );
        assert_eq!(
            prop("stone_button", &ctx(3, 0.0, 0.0), "facing"),
            Some("south")
        );
    }

    #[test]
    fn bed_head_follows_facing() {
        // yaw 0 — юг (+z), 90 — запад (−x), 180 — север, 270 — восток.
        assert_eq!(bed_head_pos([5, 64, 5], 0.0), [5, 64, 6]);
        assert_eq!(bed_head_pos([5, 64, 5], 90.0), [4, 64, 5]);
        assert_eq!(bed_head_pos([5, 64, 5], 180.0), [5, 64, 4]);
        assert_eq!(bed_head_pos([5, 64, 5], 270.0), [6, 64, 5]);
    }

    #[test]
    fn more_oriented_blocks() {
        // Игрок смотрит на юг (yaw 0), горизонтальное направление south.
        assert_eq!(prop("campfire", &ctx(1, 0.0, 0.0), "facing"), Some("south"));
        assert_eq!(
            prop("decorated_pot", &ctx(1, 0.0, 0.0), "facing"),
            Some("south")
        );
        assert_eq!(
            prop("oak_shelf", &ctx(1, 0.0, 0.0), "facing"),
            Some("north")
        );
        assert_eq!(prop("red_bed", &ctx(1, 90.0, 0.0), "facing"), Some("west"));
        assert_eq!(prop("cocoa", &ctx(1, 0.0, 0.0), "facing"), Some("south"));
        assert_eq!(
            prop("crafter", &ctx(1, 0.0, 0.0), "orientation"),
            Some("north_up")
        );
        assert_eq!(
            prop("jigsaw", &ctx(1, 0.0, 0.0), "orientation"),
            Some("up_north")
        );
    }

    #[test]
    fn stairs_corner_shape() {
        let st = |f: &str| {
            block_default("oak_stairs")
                .unwrap()
                .with("facing", f)
                .unwrap()
        };
        // Ступенька смотрит на север, впереди (z-1) ступенька на запад → внешний левый угол.
        let ahead = st("west");
        let me = st("north");
        let get = |x: i32, _y: i32, z: i32| {
            if (x, z) == (0, -1) {
                ahead
            } else {
                BlockState::AIR
            }
        };
        let r = with_stairs_shape(me, [0, 64, 0], &get);
        assert_eq!(r.property("shape"), Some("outer_left"));
        // Позади (z+1) ступенька на восток → внутренний правый угол.
        let back = st("east");
        let get = |x: i32, _y: i32, z: i32| {
            if (x, z) == (0, 1) {
                back
            } else {
                BlockState::AIR
            }
        };
        let r = with_stairs_shape(me, [0, 64, 0], &get);
        assert_eq!(r.property("shape"), Some("inner_right"));
    }

    #[test]
    fn torch_and_vine() {
        let s = sfp("torch", &ctx(1, 0.0, 0.0)).unwrap();
        assert_eq!(s.name(), "torch");
        let s = sfp("torch", &ctx(5, 0.0, 0.0)).unwrap();
        assert_eq!(
            (s.name(), s.property("facing")),
            ("wall_torch", Some("east"))
        );
        // Лоза на западной стенке блока: клик по грани west → прикрепляется к east.
        assert_eq!(prop("vine", &ctx(4, 0.0, 0.0), "east"), Some("true"));
        // Клетка уже с лозой на севере: новая грань добавляется, север остаётся.
        let mut c = ctx(4, 0.0, 0.0);
        c.here = block_default("vine")
            .unwrap()
            .with("north", "true")
            .unwrap();
        let s = sfp("vine", &c).unwrap();
        assert_eq!(
            (s.property("east"), s.property("north")),
            (Some("true"), Some("true"))
        );
    }
}
