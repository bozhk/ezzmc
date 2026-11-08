//! Очередь перестроения секций (TODO-133).
//! Ближайшие к камере — первыми; секции позади взгляда получают штраф.
use crate::section::SectionPos;
use std::collections::HashSet;

#[derive(Default)]
pub struct RebuildQueue {
    dirty: HashSet<SectionPos>,
}

const BEHIND_PENALTY: f32 = 4.0;

impl RebuildQueue {
    pub fn mark(&mut self, pos: SectionPos) {
        self.dirty.insert(pos);
    }

    /// Блок изменился: грязна своя секция и соседние, если блок на границе (меш зависит от соседей).
    pub fn mark_block(&mut self, x: i32, y: i32, z: i32) {
        let (sx, sy, sz) = (x >> 4, y >> 4, z >> 4);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let near =
                        |l: i32, d: i32| (d == -1 && l == 0) || (d == 1 && l == 15) || d == 0;
                    if near(x & 15, dx) && near(y & 15, dy) && near(z & 15, dz) {
                        self.dirty.insert(SectionPos {
                            x: sx + dx,
                            y: sy + dy,
                            z: sz + dz,
                        });
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, pos: &SectionPos) {
        self.dirty.remove(pos);
    }

    pub fn len(&self) -> usize {
        self.dirty.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }

    fn priority(p: SectionPos, cam: [f32; 3], look: [f32; 3]) -> f32 {
        let d = [
            p.x as f32 * 16.0 + 8.0 - cam[0],
            p.y as f32 * 16.0 + 8.0 - cam[1],
            p.z as f32 * 16.0 + 8.0 - cam[2],
        ];
        let d2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        if d[0] * look[0] + d[1] * look[1] + d[2] * look[2] < 0.0 {
            d2 * BEHIND_PENALTY
        } else {
            d2
        }
    }

    /// До `n` секций с наименьшим приоритетом; они удаляются из очереди.
    pub fn take(&mut self, n: usize, cam: [f32; 3], look: [f32; 3]) -> Vec<SectionPos> {
        let mut all: Vec<SectionPos> = self.dirty.iter().copied().collect();
        all.sort_by(|a, b| {
            Self::priority(*a, cam, look)
                .total_cmp(&Self::priority(*b, cam, look))
                .then(a.cmp(b))
        });
        all.truncate(n);
        for p in &all {
            self.dirty.remove(p);
        }
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_first_behind_penalised() {
        let mut q = RebuildQueue::default();
        let near_behind = SectionPos { x: 0, y: 0, z: -1 };
        let far_ahead = SectionPos { x: 0, y: 0, z: 2 };
        let near_ahead = SectionPos { x: 0, y: 0, z: 1 };
        for p in [near_behind, far_ahead, near_ahead] {
            q.mark(p);
        }
        let got = q.take(2, [8.0, 8.0, 8.0], [0.0, 0.0, 1.0]);
        assert_eq!(got, vec![near_ahead, near_behind]);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn border_block_dirties_neighbours() {
        let mut q = RebuildQueue::default();
        q.mark_block(5, 5, 5);
        assert_eq!(q.len(), 1);
        q.mark_block(0, 5, 15); // x- и z-граница
        assert_eq!(q.len(), 4); // своя (уже была) + x-1, z+1, угловая
    }
}
