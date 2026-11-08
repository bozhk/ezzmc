//! Видимость между гранями секции для cave-culling (TODO-143).
//! VANILLA: VisGraph + VisibilitySet (1.21.11).
use ezzmc_assets::model::Direction;

/// 6×6 бит: `visible(a, b)` — виден ли выход через грань `b` при входе через грань `a`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VisibilitySet(u64);

impl VisibilitySet {
    /// VANILLA: VisibilitySet#setAll (BitSet.size() = 64 бита).
    pub fn all(v: bool) -> Self {
        VisibilitySet(if v { u64::MAX } else { 0 })
    }

    fn bit(a: Direction, b: Direction) -> u64 {
        1u64 << (a.index() + b.index() * 6)
    }

    pub fn set(&mut self, a: Direction, b: Direction) {
        self.0 |= Self::bit(a, b) | Self::bit(b, a);
    }

    /// VANILLA: VisibilitySet#add(Set<Direction>)
    pub fn add(&mut self, faces: &[Direction]) {
        for &a in faces {
            for &b in faces {
                self.set(a, b);
            }
        }
    }

    pub fn visible(&self, a: Direction, b: Direction) -> bool {
        self.0 & Self::bit(a, b) != 0
    }
}

pub struct VisGraph {
    opaque: Vec<bool>,
    empty: i32,
}

impl Default for VisGraph {
    fn default() -> Self {
        VisGraph {
            opaque: vec![false; 4096],
            empty: 4096,
        }
    }
}

#[inline]
fn index(x: usize, y: usize, z: usize) -> usize {
    x | y << 8 | z << 4
}

impl VisGraph {
    /// Локальные координаты 0..16.
    pub fn set_opaque(&mut self, x: usize, y: usize, z: usize) {
        let i = index(x, y, z);
        // BitSet.set(i,true) + empty-- как в ванили (повторный вызов не нужен: блок посещается один раз)
        if !self.opaque[i] {
            self.opaque[i] = true;
            self.empty -= 1;
        }
    }

    /// VANILLA: VisGraph#resolve
    pub fn resolve(mut self) -> VisibilitySet {
        if 4096 - self.empty < 256 {
            return VisibilitySet::all(true);
        }
        if self.empty == 0 {
            return VisibilitySet::all(false);
        }
        let mut set = VisibilitySet::default();
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let edge = [x, y, z].iter().any(|&c| c == 0 || c == 15);
                    if edge && !self.opaque[index(x, y, z)] {
                        let faces = self.flood_fill(index(x, y, z));
                        set.add(&faces);
                    }
                }
            }
        }
        set
    }

    fn flood_fill(&mut self, start: usize) -> Vec<Direction> {
        let mut faces: Vec<Direction> = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        self.opaque[start] = true;
        while let Some(i) = queue.pop_front() {
            let (x, z, y) = (i & 15, i >> 4 & 15, i >> 8 & 15);
            let mut edge = |c: usize, lo: Direction, hi: Direction| {
                let d = if c == 0 {
                    lo
                } else if c == 15 {
                    hi
                } else {
                    return;
                };
                if !faces.contains(&d) {
                    faces.push(d);
                }
            };
            edge(x, Direction::West, Direction::East);
            edge(y, Direction::Down, Direction::Up);
            edge(z, Direction::North, Direction::South);
            let mut visit = |n: Option<usize>| {
                if let Some(n) = n {
                    if !self.opaque[n] {
                        self.opaque[n] = true;
                        queue.push_back(n);
                    }
                }
            };
            visit((y > 0).then(|| i - 256));
            visit((y < 15).then(|| i + 256));
            visit((z > 0).then(|| i - 16));
            visit((z < 15).then(|| i + 16));
            visit((x > 0).then(|| i - 1));
            visit((x < 15).then(|| i + 1));
        }
        faces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn mostly_empty_is_all_visible() {
        let mut g = VisGraph::default();
        g.set_opaque(3, 3, 3);
        let v = g.resolve();
        assert!(v.visible(West, East) && v.visible(Down, Up));
    }

    #[test]
    fn solid_section_hidden_and_wall_splits() {
        let mut g = VisGraph::default();
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    g.set_opaque(x, y, z);
                }
            }
        }
        let v = g.resolve();
        assert!(!v.visible(West, East));
        // сплошная стена x=8 делит секцию: запад не видит восток, но видит себя/верх
        let mut g = VisGraph::default();
        for y in 0..16 {
            for z in 0..16 {
                g.set_opaque(8, y, z);
            }
        }
        let v = g.resolve();
        assert!(!v.visible(West, East));
        assert!(v.visible(West, Up) && v.visible(East, Down));
    }
}
