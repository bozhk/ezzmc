//! Очередь перестроения секций (TODO-133).
//! Ближайшие к камере — первыми; секции позади взгляда получают штраф.
use crate::section::SectionPos;
use rustc_hash::FxHashSet as HashSet;

#[derive(Default)]
pub struct RebuildQueue {
    dirty: HashSet<SectionPos>,
    /// Правки блоков игроком/сервером: мешим вне очереди (нужен мгновенный отклик).
    urgent: HashSet<SectionPos>,
    /// Лучшие кандидаты из `dirty`, отсортированы по убыванию приоритета (лучший — в конце).
    batch: Vec<SectionPos>,
    calls: u32,
    /// Растёт при любой пометке секции/блока: дешёвый признак «мир менялся» для потребителей снаружи.
    pub version: u64,
    /// Растёт только при правке блока (не при загрузке чанков): мгновенная реакция блок-сущностей.
    pub block_version: u64,
}

/// Размер пачки кандидатов и период её пересборки (в вызовах `take`): на больших дальностях
/// в `dirty` сотни тысяч секций, полный проход каждый кадр съедал кадр целиком.
const BATCH: usize = 4096;
const REBUILD_EVERY: u32 = 16;

const BEHIND_PENALTY: f32 = 4.0;

impl RebuildQueue {
    pub fn mark(&mut self, pos: SectionPos) {
        self.version += 1;
        self.dirty.insert(pos);
    }

    /// Блок изменился: грязна своя секция и соседние, если блок на границе (меш зависит от соседей).
    pub fn mark_block(&mut self, x: i32, y: i32, z: i32) {
        self.version += 1;
        self.block_version += 1;
        let (sx, sy, sz) = (x >> 4, y >> 4, z >> 4);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let near =
                        |l: i32, d: i32| (d == -1 && l == 0) || (d == 1 && l == 15) || d == 0;
                    if near(x & 15, dx) && near(y & 15, dy) && near(z & 15, dz) {
                        self.urgent.insert(SectionPos {
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
        self.urgent.remove(pos);
    }

    pub fn len(&self) -> usize {
        self.dirty.len() + self.urgent.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty() && self.urgent.is_empty()
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
        let mut out = Vec::new();
        if n == 0 || self.is_empty() {
            return out;
        }
        let mut urgent: Vec<(f32, SectionPos)> = self
            .urgent
            .iter()
            .map(|&p| (Self::priority(p, cam, look), p))
            .collect();
        urgent.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        for (_, p) in urgent.into_iter().take(n) {
            self.urgent.remove(&p);
            self.dirty.remove(&p);
            out.push(p);
        }
        self.calls += 1;
        if out.len() < n && !self.dirty.is_empty() {
            if self.batch.is_empty() || self.calls >= REBUILD_EVERY {
                self.rebuild_batch(cam, look);
            }
            while out.len() < n {
                let Some(p) = self.batch.pop() else { break };
                if self.dirty.remove(&p) {
                    out.push(p);
                }
            }
        }
        out
    }

    fn rebuild_batch(&mut self, cam: [f32; 3], look: [f32; 3]) {
        self.calls = 0;
        let mut all: Vec<(f32, SectionPos)> = self
            .dirty
            .iter()
            .map(|&p| (Self::priority(p, cam, look), p))
            .collect();
        let cmp =
            |a: &(f32, SectionPos), b: &(f32, SectionPos)| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1));
        if BATCH < all.len() {
            all.select_nth_unstable_by(BATCH, cmp);
            all.truncate(BATCH);
        }
        all.sort_unstable_by(|a, b| cmp(b, a));
        self.batch = all.into_iter().map(|(_, p)| p).collect();
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
