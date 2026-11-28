//! Состояния блоков, которые ≤1.12.2 не передаются по сети и считаются клиентом по соседям:
//! соединения заборов, панелей, стен, провода и растяжки, форма ступенек, `in_wall` ворот,
//! половины дверей и растений, двойные сундуки, снег на дёрне, огонь, лоза, стебли, хорус.
//! VANILLA: `Block#getActualState` (1.12.2), результат — в свойствах состояний 26.3.
//! VANILLA-UNSURE: `BlockFaceShape.SOLID` и «полный куб» — по форме коллизии 26.3.
use crate::flammable::FLAMMABLE;
use crate::placement::{clockwise, with_stairs_shape};
use crate::support::{sturdy, wire_connects, Get, Support};
use ezzmc_protocol::types::BlockPos;
use ezzmc_world::block::{BlockState, FLAG_CAN_OCCLUDE};
use ezzmc_world::storage::ChunkCache;

/// Последний протокол, где эти состояния выводятся из соседей (1.12.2).
pub const LAST_PROTOCOL: i32 = 340;

/// Горизонтали: имя свойства, индекс направления, смещение по x и z.
const SIDES: [(&str, u8, i32, i32); 4] = [
    ("north", 2, 0, -1),
    ("south", 3, 0, 1),
    ("west", 4, -1, 0),
    ("east", 5, 1, 0),
];
const DOWN: u8 = 0;
const UP: u8 = 1;

fn is_stem(n: &str) -> bool {
    matches!(
        n,
        "melon_stem" | "pumpkin_stem" | "attached_melon_stem" | "attached_pumpkin_stem"
    )
}

fn is_fence(n: &str) -> bool {
    n.ends_with("_fence")
}

fn is_wall(n: &str) -> bool {
    n.ends_with("cobblestone_wall")
}

fn is_pane(n: &str) -> bool {
    n == "iron_bars" || n.ends_with("glass_pane")
}

fn is_double_plant(n: &str) -> bool {
    matches!(
        n,
        "sunflower" | "lilac" | "tall_grass" | "large_fern" | "rose_bush" | "peony"
    )
}

/// Нужен ли блоку пересчёт по соседям.
fn relevant(s: BlockState) -> bool {
    let n = s.name();
    is_fence(n)
        || is_wall(n)
        || is_pane(n)
        || is_stem(n)
        || is_double_plant(n)
        || n.ends_with("_stairs")
        || n.ends_with("_fence_gate")
        || n.ends_with("_door")
        || matches!(
            n,
            "redstone_wire"
                | "tripwire"
                | "chest"
                | "trapped_chest"
                | "grass_block"
                | "podzol"
                | "mycelium"
                | "fire"
                | "vine"
                | "repeater"
                | "chorus_plant"
        )
}

/// VANILLA: Block#isExceptBlockForAttachWithPiston / `method_14325`, `method_14348` (1.12.2) —
/// блоки, к полной грани которых заборы, стены и панели не цепляются.
fn refuses(n: &str, pane: bool) -> bool {
    let common = n.ends_with("shulker_box")
        || n.ends_with("_leaves")
        || matches!(
            n,
            "beacon"
                | "cauldron"
                | "water_cauldron"
                | "glowstone"
                | "ice"
                | "sea_lantern"
                | "piston"
                | "sticky_piston"
                | "piston_head"
                | "barrier"
                | "melon"
                | "carved_pumpkin"
                | "jack_o_lantern"
        );
    // Люки и стекло отвергают только заборы и стены.
    common || (!pane && (n.ends_with("_trapdoor") || n == "glass" || n.ends_with("stained_glass")))
}

/// Ворота подставляют столб в сторону `dir`, если стоят поперёк неё.
fn gate_across(s: BlockState, dir: u8) -> bool {
    s.name().ends_with("_fence_gate")
        && matches!(s.property("facing"), Some("north" | "south")) != (dir < 4)
}

fn offset(p: [i32; 3], d: u8) -> [i32; 3] {
    let mut q = p;
    match d {
        0 => q[1] -= 1,
        1 => q[1] += 1,
        2 => q[2] -= 1,
        3 => q[2] += 1,
        4 => q[0] -= 1,
        _ => q[0] += 1,
    }
    q
}

fn at(get: Get, p: [i32; 3]) -> BlockState {
    get(p[0], p[1], p[2])
}

/// Полная грань `face` у соседа (форма ступеньки учитывается).
fn solid_face(get: Get, p: [i32; 3], face: u8) -> bool {
    let s = with_stairs_shape(at(get, p), p, get);
    sturdy(s, face, Support::Full)
}

fn normal_cube(s: BlockState) -> bool {
    s.props().is_some_and(|p| p.has(FLAG_CAN_OCCLUDE))
        && s.name() != "redstone_block"
        && (0..6).all(|d| sturdy(s, d, Support::Full))
}

/// Соединяется ли забор/стена/панель с соседом в горизонтальную сторону `dir`.
fn joins(own: &str, get: Get, p: [i32; 3], dir: u8) -> bool {
    let q = offset(p, dir);
    let n = at(get, q);
    let name = n.name();
    let pane = is_pane(own);
    if !refuses(name, pane) && solid_face(get, q, dir ^ 1) {
        return true;
    }
    if pane {
        is_pane(name)
    } else if is_wall(own) {
        is_wall(name) || gate_across(n, dir)
    } else {
        // Деревянные заборы и адский — разные материалы.
        is_fence(name) && (own == "nether_brick_fence") == (name == "nether_brick_fence")
            || gate_across(n, dir)
    }
}

fn flag(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

fn set(s: BlockState, prop: &str, v: &str) -> BlockState {
    s.with(prop, v).unwrap_or(s)
}

/// VANILLA: RedStoneWireBlock#getWireType (1.12.2); крест и линию из одного соединения
/// дорисовывает модель — в состоянии 26.3 это свойства противоположных сторон.
fn wire(mut s: BlockState, p: [i32; 3], get: Get) -> BlockState {
    let open = !normal_cube(at(get, offset(p, UP)));
    let mut m = [0u8; 4];
    for (i, &(_, d, _, _)) in SIDES.iter().enumerate() {
        let q = offset(p, d);
        let n = at(get, q);
        m[i] = if wire_connects(n, Some(d))
            || (!normal_cube(n) && wire_connects(at(get, offset(q, DOWN)), None))
        {
            1
        } else if open
            && (sturdy(n, UP, Support::Full) || n.name() == "glowstone")
            && wire_connects(at(get, offset(q, UP)), None)
        {
            if sturdy(n, d ^ 1, Support::Full) {
                2
            } else {
                1
            }
        } else {
            0
        };
    }
    // SIDES: север, юг, запад, восток.
    let (ns, ew) = (m[0] == 0 && m[1] == 0, m[2] == 0 && m[3] == 0);
    if !(ns && ew) {
        if ns {
            m[2] = m[2].max(1);
            m[3] = m[3].max(1);
        }
        if ew {
            m[0] = m[0].max(1);
            m[1] = m[1].max(1);
        }
    }
    for (i, &(name, ..)) in SIDES.iter().enumerate() {
        s = set(s, name, ["none", "side", "up"][usize::from(m[i])]);
    }
    s
}

fn dir_of(s: BlockState) -> Option<u8> {
    let f = s.property("facing")?;
    SIDES.iter().find(|e| e.0 == f).map(|e| e.1)
}

/// Состояние блока с учётом соседей; прочие блоки возвращаются как есть.
pub fn actual_state(s: BlockState, p: [i32; 3], get: Get) -> BlockState {
    if !relevant(s) {
        return s;
    }
    let n = s.name();
    let mut out = s;
    if n.ends_with("_stairs") {
        return with_stairs_shape(s, p, get);
    }
    if is_fence(n) || is_pane(n) {
        for (side, d, ..) in SIDES {
            out = set(out, side, flag(joins(n, get, p, d)));
        }
    } else if is_wall(n) {
        let c = SIDES.map(|(_, d, ..)| joins(n, get, p, d));
        for (i, (side, ..)) in SIDES.iter().enumerate() {
            out = set(out, side, if c[i] { "low" } else { "none" });
        }
        let straight = matches!(c, [true, true, false, false] | [false, false, true, true]);
        let up = !straight || !at(get, offset(p, UP)).is_air();
        out = set(out, "up", flag(up));
    } else if n.ends_with("_fence_gate") {
        let sides = match dir_of(s) {
            Some(2 | 3) => [4, 5],
            _ => [2, 3],
        };
        let in_wall = sides.iter().any(|&d| is_wall(at(get, offset(p, d)).name()));
        out = set(out, "in_wall", flag(in_wall));
    } else if n.ends_with("_door") {
        let lower = s.property("half") == Some("lower");
        let other = at(get, offset(p, if lower { UP } else { DOWN }));
        if other.name() == n {
            let copied: &[&str] = if lower {
                &["hinge", "powered"]
            } else {
                &["facing", "open"]
            };
            for &prop in copied {
                if let Some(v) = other.property(prop) {
                    out = set(out, prop, v);
                }
            }
        }
    } else if is_double_plant(n) {
        if s.property("half") == Some("upper") {
            let below = at(get, offset(p, DOWN));
            if is_double_plant(below.name()) && below.property("half") == Some("lower") {
                out = set(below, "half", "upper");
            }
        }
    } else if is_stem(n) {
        out = stem(s, p, get);
    } else {
        match n {
            "redstone_wire" => out = wire(s, p, get),
            "tripwire" => {
                for (side, d, ..) in SIDES {
                    let nb = at(get, offset(p, d));
                    let joined = nb.name() == "tripwire"
                        || nb.name() == "tripwire_hook" && dir_of(nb) == Some(d ^ 1);
                    out = set(out, side, flag(joined));
                }
            }
            "chest" | "trapped_chest" => {
                if let Some(f) = dir_of(s) {
                    let right = clockwise(f);
                    // VANILLA: ChestType#getConnectedDirection (26.3): left — по часовой.
                    let kind = if at(get, offset(p, right)).name() == n {
                        "left"
                    } else if at(get, offset(p, right ^ 1)).name() == n {
                        "right"
                    } else {
                        "single"
                    };
                    out = set(out, "type", kind);
                }
            }
            "grass_block" | "podzol" | "mycelium" => {
                let above = at(get, offset(p, UP)).name();
                out = set(out, "snowy", flag(matches!(above, "snow" | "snow_block")));
            }
            "fire" => {
                let below = at(get, offset(p, DOWN));
                let grounded =
                    sturdy(below, UP, Support::Full) || FLAMMABLE.contains(&below.name());
                for (side, d) in SIDES.map(|e| (e.0, e.1)).into_iter().chain([("up", UP)]) {
                    let burns = FLAMMABLE.contains(&at(get, offset(p, d)).name());
                    out = set(out, side, flag(!grounded && burns));
                }
            }
            "vine" => {
                let up = solid_face(get, offset(p, UP), DOWN);
                out = set(out, "up", flag(up));
            }
            "repeater" => {
                if let Some(f) = dir_of(s) {
                    let locked = [clockwise(f), clockwise(f) ^ 1].iter().any(|&d| {
                        let nb = at(get, offset(p, d));
                        matches!(nb.name(), "repeater" | "comparator")
                            && nb.property("powered") == Some("true")
                            && dir_of(nb) == Some(d)
                    });
                    out = set(out, "locked", flag(locked));
                }
            }
            "chorus_plant" => {
                for (side, d) in SIDES
                    .map(|e| (e.0, e.1))
                    .into_iter()
                    .chain([("up", UP), ("down", DOWN)])
                {
                    let nb = at(get, offset(p, d)).name();
                    let joined = matches!(nb, "chorus_plant" | "chorus_flower")
                        || d == DOWN && nb == "end_stone";
                    out = set(out, side, flag(joined));
                }
            }
            _ => {}
        }
    }
    out
}

/// VANILLA: StemBlock#getActualState (1.12.2): спелый стебель тянется к своему плоду
/// (в 26.3 это отдельный блок `attached_*_stem`).
fn stem(s: BlockState, p: [i32; 3], get: Get) -> BlockState {
    let melon = s.name().ends_with("melon_stem");
    let (plain, attached, fruit) = if melon {
        ("melon_stem", "attached_melon_stem", "melon")
    } else {
        ("pumpkin_stem", "attached_pumpkin_stem", "carved_pumpkin")
    };
    let was_attached = s.name() == attached;
    if !was_attached && s.property("age") != Some("7") {
        return s;
    }
    let by_name = |name: &str| {
        ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == name)
            .map(|b| BlockState(b.default_state))
    };
    let side = SIDES
        .iter()
        .find(|&&(_, d, ..)| at(get, offset(p, d)).name() == fruit);
    match side {
        Some(&(side, ..)) => by_name(attached).map_or(s, |a| set(a, "facing", side)),
        None if was_attached => by_name(plain).map_or(s, |a| set(a, "age", "7")),
        None => s,
    }
}

/// Пересчитать клетки; изменённые записываются в хранилище и возвращаются.
fn apply(chunks: &mut ChunkCache, cells: impl Iterator<Item = [i32; 3]>) -> Vec<BlockPos> {
    let get = |x: i32, y: i32, z: i32| chunks.block(x, y, z);
    // Сначала всё читается, потом пишется: расчёт опирается только на пришедшие свойства.
    let changes: Vec<([i32; 3], BlockState)> = cells
        .filter_map(|p| {
            let old = at(&get, p);
            let new = actual_state(old, p, &get);
            (new != old).then_some((p, new))
        })
        .collect();
    changes
        .into_iter()
        .filter(|&([x, y, z], s)| chunks.set_shape(x, y, z, s))
        .map(|([x, y, z], _)| BlockPos::new(x, y, z))
        .collect()
}

/// Пришла колонка (x, z): пересчитать её и прилегающие ряды четырёх соседних колонок.
pub fn fix_chunk(chunks: &mut ChunkCache, x: i32, z: i32) {
    let f = *chunks.format();
    let mut cells = Vec::new();
    // Колонка и диапазон локальных x/z в ней.
    let areas = [
        (x, z, 0..16, 0..16),
        (x - 1, z, 15..16, 0..16),
        (x + 1, z, 0..1, 0..16),
        (x, z - 1, 0..16, 15..16),
        (x, z + 1, 0..16, 0..1),
    ];
    for (cx, cz, xs, zs) in areas {
        let Some(c) = chunks.get(cx, cz) else {
            continue;
        };
        for (i, section) in c.chunk.sections().iter().enumerate() {
            if section.has_only_air()
                || !section
                    .states()
                    .maybe_has(|id| BlockState::from_id(id).is_some_and(relevant))
            {
                continue;
            }
            let base = f.min_y + i as i32 * 16;
            for y in 0..16 {
                for lz in zs.clone() {
                    for lx in xs.clone() {
                        let s = section.block(lx as usize, y as usize, lz as usize);
                        if relevant(s) {
                            cells.push([cx * 16 + lx, base + y, cz * 16 + lz]);
                        }
                    }
                }
            }
        }
    }
    apply(chunks, cells.into_iter());
}

/// Изменились блоки: пересчитать их и всё, что от них зависит. Возвращает затронутые клетки.
pub fn fix_around(chunks: &mut ChunkCache, changed: &[BlockPos]) -> Vec<BlockPos> {
    let mut cells = std::collections::BTreeSet::new();
    for p in changed {
        for dy in -1..=1 {
            cells.insert([p.x, p.y + dy, p.z]);
            for (.., dx, dz) in SIDES {
                cells.insert([p.x + dx, p.y + dy, p.z + dz]);
            }
        }
    }
    let mut out = apply(chunks, cells.into_iter());
    // Новая форма ступеньки меняет соединения соседних заборов.
    let around: Vec<[i32; 3]> = out
        .iter()
        .filter(|p| chunks.block(p.x, p.y, p.z).name().ends_with("_stairs"))
        .flat_map(|p| SIDES.map(|(.., dx, dz)| [p.x + dx, p.y, p.z + dz]))
        .collect();
    out.extend(apply(chunks, around.into_iter()));
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn state(name: &str, props: &[(&str, &str)]) -> BlockState {
        let b = ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == name)
            .unwrap();
        props.iter().fold(BlockState(b.default_state), |s, (k, v)| {
            s.with(k, v).unwrap()
        })
    }

    fn run(world: &[([i32; 3], BlockState)], p: [i32; 3]) -> BlockState {
        let m: HashMap<[i32; 3], BlockState> = world.iter().copied().collect();
        let get = |x, y, z| m.get(&[x, y, z]).copied().unwrap_or(BlockState::AIR);
        actual_state(at(&get, p), p, &get)
    }

    const O: [i32; 3] = [0, 0, 0];
    const N: [i32; 3] = [0, 0, -1];
    const S: [i32; 3] = [0, 0, 1];
    const W: [i32; 3] = [-1, 0, 0];
    const E: [i32; 3] = [1, 0, 0];

    #[test]
    fn fence_joins_stone_fence_and_gate_but_not_glass_or_nether() {
        let w = [
            (O, state("oak_fence", &[])),
            (N, state("stone", &[])),
            (S, state("glass", &[])),
            (W, state("nether_brick_fence", &[])),
            (E, state("oak_fence_gate", &[("facing", "north")])),
        ];
        let s = run(&w, O);
        assert_eq!(s.property("north"), Some("true"));
        assert_eq!(s.property("south"), Some("false"));
        assert_eq!(s.property("west"), Some("false"));
        assert_eq!(s.property("east"), Some("true"));
    }

    #[test]
    fn pane_joins_glass_and_bars() {
        let w = [
            (O, state("glass_pane", &[])),
            (N, state("glass", &[])),
            (S, state("iron_bars", &[])),
            (W, state("oak_fence", &[])),
        ];
        let s = run(&w, O);
        assert_eq!(s.property("north"), Some("true"));
        assert_eq!(s.property("south"), Some("true"));
        assert_eq!(s.property("west"), Some("false"));
    }

    #[test]
    fn straight_wall_has_no_post() {
        let w = [
            (O, state("cobblestone_wall", &[])),
            (N, state("cobblestone_wall", &[])),
            (S, state("stone", &[])),
        ];
        let s = run(&w, O);
        assert_eq!(s.property("north"), Some("low"));
        assert_eq!(s.property("up"), Some("false"));
    }

    #[test]
    fn door_halves_share_properties() {
        let w = [
            (
                O,
                state("oak_door", &[("facing", "east"), ("open", "true")]),
            ),
            (
                [0, 1, 0],
                state("oak_door", &[("half", "upper"), ("hinge", "right")]),
            ),
        ];
        assert_eq!(run(&w, O).property("hinge"), Some("right"));
        let upper = run(&w, [0, 1, 0]);
        assert_eq!(upper.property("facing"), Some("east"));
        assert_eq!(upper.property("open"), Some("true"));
    }

    #[test]
    fn wire_line_step_and_dot() {
        let wire = state("redstone_wire", &[]);
        let dot = run(&[(O, wire)], O);
        assert_eq!(dot.property("north"), Some("none"));
        let line = run(&[(O, wire), (N, wire)], O);
        assert_eq!(line.property("south"), Some("side"));
        assert_eq!(line.property("east"), Some("none"));
        let up = run(&[(O, wire), (E, state("stone", &[])), ([1, 1, 0], wire)], O);
        assert_eq!(up.property("east"), Some("up"));
        assert_eq!(up.property("west"), Some("side"));
    }

    #[test]
    fn double_chest_and_stem() {
        let chest = state("chest", &[("facing", "north")]);
        let w = [(O, chest), (E, chest)];
        assert_eq!(run(&w, O).property("type"), Some("left"));
        assert_eq!(run(&w, E).property("type"), Some("right"));
        let ripe = state("melon_stem", &[("age", "7")]);
        let s = run(&[(O, ripe), (W, state("melon", &[]))], O);
        assert_eq!(s.name(), "attached_melon_stem");
        assert_eq!(s.property("facing"), Some("west"));
        assert_eq!(run(&[(O, s)], O), ripe);
    }

    #[test]
    fn snow_and_tall_plant() {
        let w = [
            (O, state("grass_block", &[])),
            ([0, 1, 0], state("snow", &[])),
        ];
        assert_eq!(run(&w, O).property("snowy"), Some("true"));
        let w = [
            (O, state("rose_bush", &[("half", "lower")])),
            ([0, 1, 0], state("sunflower", &[("half", "upper")])),
        ];
        assert_eq!(run(&w, [0, 1, 0]).name(), "rose_bush");
    }
}
