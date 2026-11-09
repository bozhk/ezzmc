//! Клиентский светодвижок: пересчёт блочного и небесного света при смене блока.
//! VANILLA: LevelLightEngine#checkBlock → BlockLightEngine/SkyLightEngine (propagateIncrease/Decrease), 26.3.
//! VANILLA-UNSURE: формы окклюзии граней (лестницы/плиты) не учитываются — только `light_block`.
use crate::block::BlockState;
use crate::storage::ChunkCache;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Sky,
    Block,
}

const DIRS: [(i32, i32, i32); 6] = [
    (0, -1, 0),
    (0, 1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn opacity(s: BlockState) -> u8 {
    s.props().map_or(0, |p| p.light_block)
}

impl ChunkCache {
    fn world_y(&self) -> (i32, i32) {
        let f = self.format();
        (f.min_y, f.min_y + f.height)
    }

    fn light_get(&self, kind: Kind, x: i32, y: i32, z: i32) -> Option<u8> {
        let c = self.get(x >> 4, z >> 4)?;
        let (lx, lz) = ((x & 15) as usize, (z & 15) as usize);
        Some(
            match kind {
                Kind::Sky => c.sky_light(lx, y, lz),
                Kind::Block => c.block_light(lx, y, lz),
            }
            .unwrap_or(0),
        )
    }

    fn light_put(&mut self, kind: Kind, x: i32, y: i32, z: i32, v: u8) {
        let min_section = self.world_y().0 >> 4;
        let Some(c) = self.get_mut(x >> 4, z >> 4) else {
            return;
        };
        let i = ((y >> 4) - min_section + 1) as usize;
        let layers = match kind {
            Kind::Sky => &mut c.light.sky,
            Kind::Block => &mut c.light.block,
        };
        if let Some(slot) = layers.get_mut(i) {
            let l = slot.get_or_insert_with(|| crate::light::DataLayer::filled(0));
            let (lx, ly, lz) = ((x & 15) as usize, (y & 15) as usize, (z & 15) as usize);
            if l.get(lx, ly, lz) != v {
                l.set(lx, ly, lz, v);
                self.light_dirty.insert((x >> 4, y >> 4, z >> 4));
            }
        }
    }

    /// Y блока-«потолка» колонки: ниже него небесные источники кончаются (min_y-1, если колонка открыта).
    fn sky_floor(&self, x: i32, z: i32, over: Option<(i32, BlockState)>) -> i32 {
        let (min_y, max_y) = self.world_y();
        for y in (min_y..max_y).rev() {
            let s = match over {
                Some((oy, st)) if oy == y => st,
                _ => self.block(x, y, z),
            };
            if opacity(s) != 0 {
                return y;
            }
        }
        min_y - 1
    }

    /// VANILLA: LevelLightEngine#checkBlock (с немедленным runLightUpdates).
    pub(crate) fn light_check_block(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        old: BlockState,
        new: BlockState,
    ) {
        let (po, pn) = (old.props(), new.props());
        let (eo, en) = (
            po.map_or(0, |p| p.light_emission),
            pn.map_or(0, |p| p.light_emission),
        );
        if eo == en && opacity(old) == opacity(new) {
            return;
        }
        self.relight_block(x, y, z, en);
        self.relight_sky(x, y, z, old);
    }

    fn relight_block(&mut self, x: i32, y: i32, z: i32, emission: u8) {
        let cur = self.light_get(Kind::Block, x, y, z).unwrap_or(0);
        let mut dec = VecDeque::new();
        let mut inc = VecDeque::new();
        if cur > 0 {
            self.light_put(Kind::Block, x, y, z, 0);
            dec.push_back((x, y, z, cur));
        }
        self.propagate(Kind::Block, dec, &mut inc, None);
        if emission > 0 {
            self.light_put(Kind::Block, x, y, z, emission);
            inc.push_back((x, y, z, emission));
        }
        // Соседи могли начать пропускать свет в клетку (убрали стену).
        for (dx, dy, dz) in DIRS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if let Some(l) = self.light_get(Kind::Block, nx, ny, nz) {
                if l > 1 {
                    inc.push_back((nx, ny, nz, l));
                }
            }
        }
        self.increase(Kind::Block, inc);
    }

    fn relight_sky(&mut self, x: i32, y: i32, z: i32, old: BlockState) {
        let (min_y, max_y) = self.world_y();
        let floor_new = self.sky_floor(x, z, None);
        let floor_old = self.sky_floor(x, z, Some((y, old)));
        let mut dec = VecDeque::new();
        let mut inc = VecDeque::new();
        if floor_new > floor_old {
            // Крыша закрылась: бывшие источники (floor_old, floor_new] гаснут.
            for yy in (floor_old + 1).max(min_y)..=floor_new.min(max_y - 1) {
                if let Some(l) = self.light_get(Kind::Sky, x, yy, z) {
                    if l > 0 {
                        self.light_put(Kind::Sky, x, yy, z, 0);
                        dec.push_back((x, yy, z, l));
                    }
                }
            }
        } else if floor_new < floor_old {
            // Крышу сняли: (floor_new, floor_old] становятся источниками.
            for yy in (floor_new + 1).max(min_y)..=floor_old.min(max_y - 1) {
                self.light_put(Kind::Sky, x, yy, z, 15);
                inc.push_back((x, yy, z, 15));
            }
        } else if y > floor_new {
            // Клетка над потолком — источник, меняется только если сам блок непрозрачен… (floor учтён)
        } else if let Some(l) = self.light_get(Kind::Sky, x, y, z) {
            if l > 0 {
                self.light_put(Kind::Sky, x, y, z, 0);
                dec.push_back((x, y, z, l));
            }
        }
        self.propagate(Kind::Sky, dec, &mut inc, Some((x, z, floor_new)));
        for (dx, dy, dz) in DIRS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if ny < min_y || ny >= max_y {
                continue;
            }
            if let Some(l) = self.light_get(Kind::Sky, nx, ny, nz) {
                if l > 1 {
                    inc.push_back((nx, ny, nz, l));
                }
            }
        }
        self.increase(Kind::Sky, inc);
    }

    /// VANILLA: propagateDecrease. Зависимые клетки гасятся, независимые идут в `inc` (перераспространение).
    fn propagate(
        &mut self,
        kind: Kind,
        mut dec: VecDeque<(i32, i32, i32, u8)>,
        inc: &mut VecDeque<(i32, i32, i32, u8)>,
        col: Option<(i32, i32, i32)>,
    ) {
        let (min_y, max_y) = self.world_y();
        while let Some((x, y, z, l)) = dec.pop_front() {
            for (dx, dy, dz) in DIRS {
                let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                if ny < min_y || ny >= max_y {
                    continue;
                }
                let Some(ln) = self.light_get(kind, nx, ny, nz) else {
                    continue;
                };
                if ln == 0 {
                    continue;
                }
                let sky_down = kind == Kind::Sky
                    && dy == -1
                    && l == 15
                    && ln == 15
                    && col.is_some_and(|(cx, cz, floor)| nx == cx && nz == cz && ny <= floor);
                if ln < l || sky_down {
                    self.light_put(kind, nx, ny, nz, 0);
                    dec.push_back((nx, ny, nz, ln));
                } else {
                    inc.push_back((nx, ny, nz, ln));
                }
            }
        }
    }

    /// VANILLA: propagateIncrease.
    fn increase(&mut self, kind: Kind, mut inc: VecDeque<(i32, i32, i32, u8)>) {
        let (min_y, max_y) = self.world_y();
        while let Some((x, y, z, l)) = inc.pop_front() {
            if self.light_get(kind, x, y, z) != Some(l) {
                continue;
            }
            for (dx, dy, dz) in DIRS {
                let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                if ny < min_y || ny >= max_y {
                    continue;
                }
                let Some(ln) = self.light_get(kind, nx, ny, nz) else {
                    continue;
                };
                let op = opacity(self.block(nx, ny, nz));
                let nl = if kind == Kind::Sky && dy == -1 && l == 15 && op == 0 {
                    15
                } else {
                    l.saturating_sub(op.max(1))
                };
                if nl > ln {
                    self.light_put(kind, nx, ny, nz, nl);
                    if nl > 1 {
                        inc.push_back((nx, ny, nz, nl));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::chunk::{Chunk, ChunkFormat};
    use crate::chunk_packet::LevelChunkPacket;
    use crate::light::{DataLayer, LightUpdate};

    const F: ChunkFormat = ChunkFormat {
        min_y: -64,
        height: 384,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    pub(crate) fn world() -> ChunkCache {
        let mut c = ChunkCache::new(F, 4);
        c.set_center(0, 0);
        for cx in -1..=1 {
            for cz in -1..=1 {
                let mut light = LightUpdate::default();
                for i in 0..F.section_count() + 2 {
                    light.sky.push((i, DataLayer::filled(15)));
                }
                c.apply_packet(LevelChunkPacket {
                    x: cx,
                    z: cz,
                    chunk: Chunk::empty(&F),
                    heightmaps: vec![],
                    block_entities: vec![],
                    light,
                });
            }
        }
        c
    }

    fn state(name: &str) -> BlockState {
        (0..crate::generated::b26_3::STATE_COUNT)
            .filter_map(BlockState::from_id)
            .find(|s| s.name() == name)
            .unwrap()
    }

    #[test]
    fn torch_light_appears_and_disappears() {
        let mut c = world();
        let g = state("glowstone");
        c.set_block(0, 10, 0, g);
        assert_eq!(c.light_get(Kind::Block, 0, 10, 0), Some(15));
        assert_eq!(c.light_get(Kind::Block, 3, 10, 0), Some(12));
        assert_eq!(c.light_get(Kind::Block, 0, 12, 2), Some(11));
        c.set_block(0, 10, 0, BlockState::AIR);
        assert_eq!(c.light_get(Kind::Block, 0, 10, 0), Some(0));
        assert_eq!(c.light_get(Kind::Block, 3, 10, 0), Some(0));
    }

    #[test]
    fn wall_blocks_block_light() {
        let mut c = world();
        c.set_block(0, 10, 0, state("glowstone"));
        c.set_block(1, 10, 0, state("stone"));
        // за стеной свет идёт в обход (над ней)
        assert_eq!(c.light_get(Kind::Block, 2, 10, 0), Some(11));
        c.set_block(1, 10, 0, BlockState::AIR);
        assert_eq!(c.light_get(Kind::Block, 2, 10, 0), Some(13));
    }

    #[test]
    fn sky_roof_close_and_open() {
        let mut c = world();
        c.set_block(0, 100, 0, state("stone"));
        assert_eq!(c.light_get(Kind::Sky, 0, 99, 0), Some(14)); // из соседней колонки
        c.set_block(0, 100, 0, BlockState::AIR);
        assert_eq!(c.light_get(Kind::Sky, 0, 99, 0), Some(15));
        assert_eq!(c.light_get(Kind::Sky, 0, 50, 0), Some(15));
    }
}

#[cfg(test)]
mod track_light {
    use super::*;
    use crate::block::BlockState;

    #[test]
    fn grass_plant_on_dirt_keeps_sky_light() {
        let mut c = super::tests::world();
        let st = |n: &str| {
            (0..crate::generated::b26_3::STATE_COUNT)
                .filter_map(BlockState::from_id)
                .find(|s| s.name() == n)
                .expect("блок есть в реестре 26.3")
        };
        c.set_block(0, 99, 0, st("grass_block"));
        c.set_block(0, 100, 0, st("short_grass"));
        let above = c.light_get(Kind::Sky, 0, 100, 0);
        let above2 = c.light_get(Kind::Sky, 0, 101, 0);
        assert_eq!(above, Some(15));
        assert_eq!(above2, Some(15));
    }
}
