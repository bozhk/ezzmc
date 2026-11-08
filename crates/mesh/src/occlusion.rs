//! Cave culling секций: обход видимости от камеры по `VisibilitySet` (TODO-143).
//! VANILLA: SectionOcclusionGraph#runUpdates / initializeQueueForFullUpdate (1.21.11).
//! Синхронный порт полного обхода; инкрементальные обновления и асинхронность — забота вызывающей стороны.
use crate::cull::{offset, opposite};
use crate::section::SectionPos;
use crate::vis::VisibilitySet;
use ezzmc_assets::model::Direction;
use glam::DVec3;
use std::collections::{HashMap, VecDeque};

/// Расстояние (блоки), с которого включается «продвинутая» проверка по лучу.
const MIN_ADVANCED_CULLING_DISTANCE: i32 = 60;
// VANILLA: ceil(sqrt(3) * 16)
const CEILED_SECTION_DIAGONAL: f64 = 28.0;

/// Область секций вокруг камеры (`ViewArea`).
pub trait ViewArea {
    /// `viewArea.getRenderSection(p) != null`.
    fn exists(&self, p: SectionPos) -> bool;
    /// `None` — секция ещё не скомпилирована (`UNCOMPILED`: ничего не видно); пустая — `all(true)`.
    fn visibility(&self, p: SectionPos) -> Option<VisibilitySet>;
    /// `RenderSection#hasAllNeighbors` — загружены соседние колонки.
    fn has_all_neighbors(&self, p: SectionPos) -> bool;
    fn min_section_y(&self) -> i32;
    /// Включительно (`getMaxSectionY`).
    fn max_section_y(&self) -> i32;
    /// Дистанция прорисовки в чанках.
    fn view_distance(&self) -> i32;
}

#[derive(Clone, Copy, Debug)]
struct Node {
    source: u8,
    directions: u8,
}

impl Node {
    fn new(dir: Option<Direction>) -> Node {
        Node {
            source: dir.map_or(0, |d| 1 << d.index()),
            directions: 0,
        }
    }
    fn set_directions(&mut self, from: u8, d: Direction) {
        self.directions |= from | 1 << d.index();
    }
    fn has_direction(&self, d: Direction) -> bool {
        self.directions & (1 << d.index()) != 0
    }
}

/// VANILLA: ChunkTrackingView#isInViewDistance — круг в чанках, без поправки.
fn in_view_distance(cam: SectionPos, p: SectionPos, vd: i32) -> bool {
    let dx = i64::from((p.x - cam.x).abs().saturating_sub(1).max(0));
    let dz = i64::from((p.z - cam.z).abs().saturating_sub(1).max(0));
    dx * dx + dz * dz < i64::from(vd) * i64::from(vd)
}

fn relative(
    view: &dyn ViewArea,
    cam: SectionPos,
    p: SectionPos,
    d: Direction,
) -> Option<SectionPos> {
    let o = offset(d);
    let q = SectionPos {
        x: p.x + o[0],
        y: p.y + o[1],
        z: p.z + o[2],
    };
    let vd = view.view_distance();
    if !in_view_distance(cam, q, vd) || (cam.y - q.y).abs() > vd {
        return None;
    }
    view.exists(q).then_some(q)
}

fn center(p: SectionPos) -> DVec3 {
    DVec3::new(
        f64::from(p.x * 16 + 8),
        f64::from(p.y * 16 + 8),
        f64::from(p.z * 16 + 8),
    )
}

/// Секции, достижимые от камеры, в порядке обхода (ближние первыми).
/// `cave_culling` — опция «Advanced culling» (`$$3`).
// VANILLA: SectionOcclusionGraph#runUpdates
pub fn visible_sections(
    view: &dyn ViewArea,
    camera: [f64; 3],
    cave_culling: bool,
) -> Vec<SectionPos> {
    let cam_pos = DVec3::from(camera);
    let cam = SectionPos {
        x: (camera[0].floor() as i32) >> 4,
        y: (camera[1].floor() as i32) >> 4,
        z: (camera[2].floor() as i32) >> 4,
    };
    let cam_center = (cam.x * 16 + 8, cam.y * 16 + 8, cam.z * 16 + 8);
    let vd = view.view_distance();
    let mut nodes: HashMap<SectionPos, Node> = HashMap::new();
    let mut queue: VecDeque<SectionPos> = VecDeque::new();
    // initializeQueueForFullUpdate
    if view.exists(cam) {
        nodes.insert(cam, Node::new(None));
        queue.push_back(cam);
    } else {
        let below = cam.y < view.min_section_y();
        let y = if below {
            view.min_section_y()
        } else {
            view.max_section_y()
        };
        let dir = if below {
            Direction::Up
        } else {
            Direction::Down
        };
        let mut start = Vec::new();
        for dx in -vd..=vd {
            for dz in -vd..=vd {
                let p = SectionPos {
                    x: cam.x + dx,
                    y,
                    z: cam.z + dz,
                };
                if view.exists(p) && in_view_distance(cam, p, vd) {
                    let mut n = Node::new(Some(dir));
                    n.set_directions(n.directions, dir);
                    if dx > 0 {
                        n.set_directions(n.directions, Direction::East);
                    } else if dx < 0 {
                        n.set_directions(n.directions, Direction::West);
                    }
                    if dz > 0 {
                        n.set_directions(n.directions, Direction::South);
                    } else if dz < 0 {
                        n.set_directions(n.directions, Direction::North);
                    }
                    start.push((p, n));
                }
            }
        }
        let d2 = |p: SectionPos| {
            (center(p)
                - DVec3::new(
                    f64::from(cam_center.0),
                    f64::from(cam_center.1),
                    f64::from(cam_center.2),
                ))
            .length_squared()
        };
        start.sort_by(|a, b| d2(a.0).total_cmp(&d2(b.0)));
        for (p, n) in start {
            nodes.insert(p, n);
            queue.push_back(p);
        }
    }
    let mut order = Vec::new();
    while let Some(p) = queue.pop_front() {
        order.push(p);
        let node = nodes[&p];
        let far = (p.x - cam.x).abs() > MIN_ADVANCED_CULLING_DISTANCE >> 4
            || (p.y - cam.y).abs() > MIN_ADVANCED_CULLING_DISTANCE >> 4
            || (p.z - cam.z).abs() > MIN_ADVANCED_CULLING_DISTANCE >> 4;
        for d in Direction::ALL {
            let Some(q) = relative(view, cam, p, d) else {
                continue;
            };
            if cave_culling && node.has_direction(opposite(d)) {
                continue;
            }
            if cave_culling && node.source != 0 {
                let vis = view.visibility(p);
                let ok = Direction::ALL.iter().any(|&s| {
                    node.source & (1 << s.index()) != 0
                        && vis.is_some_and(|v| v.visible(opposite(s), d))
                });
                if !ok {
                    continue;
                }
            }
            if cave_culling && far && !ray_reaches(view, &nodes, p, d, cam_pos, cam_center) {
                continue;
            }
            if let Some(n) = nodes.get_mut(&q) {
                n.source |= 1 << d.index();
            } else {
                let mut n = Node::new(Some(d));
                n.set_directions(node.directions, d);
                if view.has_all_neighbors(q) {
                    queue.push_back(q);
                    nodes.insert(q, n);
                }
                // иначе секция ждёт соседей (chunksWaitingForNeighbors) — на следующем обновлении
            }
        }
    }
    order
}

/// Продвинутый тест: луч от угла секции к камере должен проходить только по уже достигнутым секциям.
fn ray_reaches(
    view: &dyn ViewArea,
    nodes: &HashMap<SectionPos, Node>,
    p: SectionPos,
    d: Direction,
    cam: DVec3,
    cam_center: (i32, i32, i32),
) -> bool {
    let (bx, by, bz) = (p.x * 16, p.y * 16, p.z * 16);
    let axis = d.index() / 2; // 0 = Y, 1 = Z, 2 = X (Down/Up, North/South, West/East)
    let xa = axis == 2;
    let ya = axis == 0;
    let za = axis == 1;
    let sx = if xa {
        cam_center.0 > bx
    } else {
        cam_center.0 < bx
    };
    let sy = if ya {
        cam_center.1 > by
    } else {
        cam_center.1 < by
    };
    let sz = if za {
        cam_center.2 > bz
    } else {
        cam_center.2 < bz
    };
    let mut pos = DVec3::new(
        f64::from(bx + if sx { 16 } else { 0 }),
        f64::from(by + if sy { 16 } else { 0 }),
        f64::from(bz + if sz { 16 } else { 0 }),
    );
    let step = (cam - pos).normalize_or_zero() * CEILED_SECTION_DIAGONAL;
    let min_y = f64::from(view.min_section_y() * 16);
    let max_y = f64::from((view.max_section_y() + 1) * 16);
    while pos.distance_squared(cam) > 3600.0 {
        pos += step;
        if pos.y > max_y || pos.y < min_y {
            break;
        }
        let q = SectionPos {
            x: (pos.x.floor() as i32) >> 4,
            y: (pos.y.floor() as i32) >> 4,
            z: (pos.z.floor() as i32) >> 4,
        };
        if !view.exists(q) || !nodes.contains_key(&q) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    struct Grid {
        walls: HashSet<SectionPos>,
        vd: i32,
    }

    impl ViewArea for Grid {
        fn exists(&self, p: SectionPos) -> bool {
            (0..8).contains(&p.y) && p.x.abs() <= self.vd + 1 && p.z.abs() <= self.vd + 1
        }
        fn visibility(&self, p: SectionPos) -> Option<VisibilitySet> {
            // сплошная секция: из неё нигде не видно
            Some(VisibilitySet::all(!self.walls.contains(&p)))
        }
        fn has_all_neighbors(&self, _: SectionPos) -> bool {
            true
        }
        fn min_section_y(&self) -> i32 {
            0
        }
        fn max_section_y(&self) -> i32 {
            7
        }
        fn view_distance(&self) -> i32 {
            self.vd
        }
    }

    fn at(x: i32, y: i32, z: i32) -> SectionPos {
        SectionPos { x, y, z }
    }

    #[test]
    fn open_world_reaches_everything_in_range() {
        let g = Grid {
            walls: HashSet::new(),
            vd: 3,
        };
        let v = visible_sections(&g, [8.0, 72.0, 8.0], true);
        assert!(v.contains(&at(0, 4, 0)) && v.contains(&at(2, 4, 0)) && v.contains(&at(0, 7, 0)));
        assert_eq!(v[0], at(0, 4, 0));
        // вне радиуса (круг по чанкам) — нет
        assert!(!v.contains(&at(5, 4, 0)));
        let uniq: HashSet<_> = v.iter().collect();
        assert_eq!(uniq.len(), v.len());
    }

    #[test]
    fn solid_wall_blocks_sections_behind_it() {
        // сплошная стена из секций на x=1 (все y и z)
        let mut walls = HashSet::new();
        for y in 0..8 {
            for z in -4..=4 {
                walls.insert(at(1, y, z));
            }
        }
        let g = Grid { walls, vd: 4 };
        let v = visible_sections(&g, [8.0, 72.0, 8.0], true);
        assert!(v.contains(&at(1, 4, 0)), "сама стена достижима");
        assert!(!v.contains(&at(2, 4, 0)), "за стеной скрыто");
        // без cave culling видно всё
        let all = visible_sections(&g, [8.0, 72.0, 8.0], false);
        assert!(all.contains(&at(2, 4, 0)));
    }

    #[test]
    fn camera_above_world_starts_from_top_layer() {
        let g = Grid {
            walls: HashSet::new(),
            vd: 2,
        };
        let v = visible_sections(&g, [8.0, 400.0, 8.0], true);
        assert!(!v.is_empty());
        // вертикальный охват ограничен дистанцией прорисовки (getRelativeFrom): только верхний слой
        assert!(v.contains(&at(0, 7, 0)) && !v.contains(&at(0, 0, 0)));
    }
}
