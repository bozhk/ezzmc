//! Хранилище чанков клиента (TODO-118/120) и применение пакета чанка.
//! VANILLA: ClientChunkCache (+Storage), LevelChunk (клиентская часть), 1.21.11.
use crate::block::BlockState;
use crate::chunk::{Chunk, ChunkFormat};
use crate::chunk_packet::{BlockEntityInfo, HeightmapType, LevelChunkPacket};
use crate::light::{DataLayer, LightUpdate};
use ezzmc_protocol::types::{BlockPos, ChunkPos};
use std::collections::HashMap;
use std::sync::Arc;

/// Свет колонки: секции света от `min_section - 1` до `max_section + 1` (section_count + 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkLight {
    pub sky: Vec<Option<DataLayer>>,
    pub block: Vec<Option<DataLayer>>,
}

impl ChunkLight {
    pub fn new(section_count: usize) -> Self {
        ChunkLight {
            sky: vec![None; section_count + 2],
            block: vec![None; section_count + 2],
        }
    }

    pub fn apply(&mut self, u: &LightUpdate) {
        fn put(dst: &mut [Option<DataLayer>], upd: &[(usize, DataLayer)], empty: &[usize]) {
            for &i in empty {
                if let Some(s) = dst.get_mut(i) {
                    *s = Some(DataLayer::filled(0));
                }
            }
            for (i, l) in upd {
                if let Some(s) = dst.get_mut(*i) {
                    *s = Some(l.clone());
                }
            }
        }
        put(&mut self.sky, &u.sky, &u.empty_sky);
        put(&mut self.block, &u.block, &u.empty_block);
    }

    fn lookup(
        layers: &[Option<DataLayer>],
        min_section: i32,
        x: usize,
        y: i32,
        z: usize,
    ) -> Option<u8> {
        let i = (y >> 4) - min_section + 1;
        let l = layers.get(usize::try_from(i).ok()?)?.as_ref()?;
        Some(l.get(x & 15, (y & 15) as usize, z & 15))
    }
}

/// Клиентская колонка: блоки + хейтмапы + block entities + свет.
#[derive(Clone, Debug)]
pub struct ClientChunk {
    pub pos: ChunkPos,
    pub chunk: Chunk,
    pub heightmaps: Vec<(HeightmapType, Vec<i64>)>,
    /// Ключ — (x, y, z) внутри чанка по x,z.
    pub block_entities: HashMap<(u8, i32, u8), BlockEntityInfo>,
    pub light: ChunkLight,
}

impl ClientChunk {
    /// VANILLA: LevelReader#getHeight → ChunkAccess#getHeight(type,x,z)+1 = значение карты + min_y.
    /// Карта — SimpleBitStorage (без пересечения long), bits = ceilLog2(height+1).
    pub fn height(&self, ty: HeightmapType, x: usize, z: usize) -> Option<i32> {
        let data = &self.heightmaps.iter().find(|(t, _)| *t == ty)?.1;
        let total = self.chunk.sections().len() as u32 * 16;
        let bits = 32 - total.leading_zeros(); // ceilLog2(total+1) для total, не степени двойки − 1
        let bits = bits.max(1) as usize;
        let per_long = 64 / bits;
        let i = (z & 15) * 16 + (x & 15);
        let word = *data.get(i / per_long)? as u64;
        let v = (word >> ((i % per_long) * bits)) & ((1u64 << bits) - 1);
        Some(v as i32 + self.chunk.min_y())
    }

    pub fn sky_light(&self, x: usize, y: i32, z: usize) -> Option<u8> {
        ChunkLight::lookup(&self.light.sky, self.chunk.min_y() >> 4, x, y, z)
    }
    pub fn block_light(&self, x: usize, y: i32, z: usize) -> Option<u8> {
        ChunkLight::lookup(&self.light.block, self.chunk.min_y() >> 4, x, y, z)
    }
}

/// VANILLA: ClientChunkCache.calculateStorageRange.
fn storage_range(view_radius: i32) -> i32 {
    view_radius.max(2) + 3
}

/// Кольцевая сетка (2r+1)² вокруг центра, индекс — floorMod, как в Storage.
pub struct ChunkCache {
    chunks: Vec<Option<Arc<ClientChunk>>>,
    radius: i32,
    range: i32,
    center: ChunkPos,
    count: usize,
    format: ChunkFormat,
    /// Секции (x, y, z), чей свет изменил клиентский светодвижок (для ремеша).
    pub light_dirty: std::collections::HashSet<(i32, i32, i32)>,
}

impl ChunkCache {
    pub fn new(format: ChunkFormat, view_radius: i32) -> Self {
        let radius = storage_range(view_radius);
        let range = radius * 2 + 1;
        ChunkCache {
            chunks: vec![None; (range * range) as usize],
            radius,
            range,
            center: ChunkPos { x: 0, z: 0 },
            count: 0,
            format,
            light_dirty: Default::default(),
        }
    }

    pub fn format(&self) -> &ChunkFormat {
        &self.format
    }

    pub fn loaded_count(&self) -> usize {
        self.count
    }

    pub fn center(&self) -> ChunkPos {
        self.center
    }

    fn index(&self, x: i32, z: i32) -> usize {
        (z.rem_euclid(self.range) * self.range + x.rem_euclid(self.range)) as usize
    }

    /// VANILLA: Storage#inRange.
    fn in_range(&self, x: i32, z: i32) -> bool {
        (x - self.center.x).abs() <= self.radius && (z - self.center.z).abs() <= self.radius
    }

    /// VANILLA: updateViewCenter (ClientboundSetChunkCacheCenterPacket).
    pub fn set_center(&mut self, x: i32, z: i32) {
        self.center = ChunkPos { x, z };
    }

    /// Текущая дальность прорисовки (чанки), как её задал сервер.
    pub fn view_radius(&self) -> i32 {
        self.radius - 3
    }

    /// VANILLA: updateViewRadius (ClientboundSetChunkCacheRadiusPacket) — пересоздаёт сетку.
    pub fn set_view_radius(&mut self, view_radius: i32) {
        let new_radius = storage_range(view_radius);
        if new_radius == self.radius {
            return;
        }
        let old = std::mem::take(&mut self.chunks);
        self.radius = new_radius;
        self.range = new_radius * 2 + 1;
        self.chunks = vec![None; (self.range * self.range) as usize];
        self.count = 0;
        for c in old.into_iter().flatten() {
            if self.in_range(c.pos.x, c.pos.z) {
                let i = self.index(c.pos.x, c.pos.z);
                self.chunks[i] = Some(c);
                self.count += 1;
            }
        }
    }

    pub fn get(&self, x: i32, z: i32) -> Option<&Arc<ClientChunk>> {
        if !self.in_range(x, z) {
            return None;
        }
        self.chunks[self.index(x, z)]
            .as_ref()
            .filter(|c| c.pos.x == x && c.pos.z == z)
    }

    /// VANILLA: drop (ClientboundForgetLevelChunkPacket).
    pub fn drop_chunk(&mut self, x: i32, z: i32) -> bool {
        if !self.in_range(x, z) {
            return false;
        }
        let i = self.index(x, z);
        if self.chunks[i]
            .as_ref()
            .is_some_and(|c| c.pos.x == x && c.pos.z == z)
        {
            self.chunks[i] = None;
            self.count -= 1;
            return true;
        }
        false
    }

    /// VANILLA: replaceWithPacketData + queueLightUpdate. Вне радиуса — игнор (false).
    pub fn apply_packet(&mut self, p: LevelChunkPacket) -> bool {
        if !self.in_range(p.x, p.z) {
            return false;
        }
        let mut light = ChunkLight::new(self.format.section_count());
        light.apply(&p.light);
        let be = p
            .block_entities
            .into_iter()
            .map(|b| ((b.x, b.y, b.z), b))
            .collect();
        let chunk = Arc::new(ClientChunk {
            pos: ChunkPos { x: p.x, z: p.z },
            chunk: p.chunk,
            heightmaps: p.heightmaps,
            block_entities: be,
            light,
        });
        let i = self.index(p.x, p.z);
        if self.chunks[i].is_none() {
            self.count += 1;
        }
        self.chunks[i] = Some(chunk);
        true
    }

    /// Обновление света (ClientboundLightUpdatePacket). false, если чанка нет.
    pub fn apply_light(&mut self, x: i32, z: i32, u: &LightUpdate) -> bool {
        if !self.in_range(x, z) {
            return false;
        }
        let i = self.index(x, z);
        match &mut self.chunks[i] {
            Some(c) if c.pos.x == x && c.pos.z == z => {
                Arc::make_mut(c).light.apply(u);
                true
            }
            _ => false,
        }
    }

    /// Заменить/добавить block entity по мировой позиции. false — чанка нет.
    pub fn set_block_entity(&mut self, pos: BlockPos, info: BlockEntityInfo) -> bool {
        match self.get_mut(pos.x >> 4, pos.z >> 4) {
            Some(c) => {
                c.block_entities
                    .insert(((pos.x & 15) as u8, pos.y, (pos.z & 15) as u8), info);
                true
            }
            None => false,
        }
    }

    pub fn remove_block_entity(&mut self, pos: BlockPos) {
        if let Some(c) = self.get_mut(pos.x >> 4, pos.z >> 4) {
            c.block_entities
                .remove(&((pos.x & 15) as u8, pos.y, (pos.z & 15) as u8));
        }
    }

    pub(crate) fn get_mut(&mut self, x: i32, z: i32) -> Option<&mut ClientChunk> {
        if !self.in_range(x, z) {
            return None;
        }
        let i = self.index(x, z);
        match &mut self.chunks[i] {
            Some(c) if c.pos.x == x && c.pos.z == z => Some(Arc::make_mut(c)),
            _ => None,
        }
    }

    /// Блок по мировым координатам; нет чанка/вне высоты → воздух.
    pub fn block(&self, x: i32, y: i32, z: i32) -> BlockState {
        match self.get(x >> 4, z >> 4) {
            Some(c) => c.chunk.block((x & 15) as usize, y, (z & 15) as usize),
            None => BlockState::AIR,
        }
    }

    /// VANILLA: ClientLevel#setServerVerifiedBlockState (только данные). false — чанка нет.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, state: BlockState) -> bool {
        let (cx, cz) = (x >> 4, z >> 4);
        if !self.in_range(cx, cz) {
            return false;
        }
        let i = self.index(cx, cz);
        match &mut self.chunks[i] {
            Some(c) if c.pos.x == cx && c.pos.z == cz => {
                let c = Arc::make_mut(c);
                let old = c.chunk.block((x & 15) as usize, y, (z & 15) as usize);
                let ok = c
                    .chunk
                    .set_block((x & 15) as usize, y, (z & 15) as usize, state)
                    .is_some();
                if ok && old != state {
                    // VANILLA: ClientLevel#sendBlockUpdated → LevelLightEngine#checkBlock.
                    self.light_check_block(x, y, z, old, state);
                }
                ok
            }
            _ => false,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const F: ChunkFormat = ChunkFormat {
        min_y: -64,
        height: 384,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    fn packet(x: i32, z: i32) -> LevelChunkPacket {
        LevelChunkPacket {
            x,
            z,
            chunk: Chunk::empty(&F),
            heightmaps: vec![],
            block_entities: vec![],
            light: LightUpdate::default(),
        }
    }

    #[test]
    fn range_and_ring_index() {
        let mut c = ChunkCache::new(F, 8); // radius 11
        c.set_center(100, -100);
        assert!(c.apply_packet(packet(100, -100)));
        assert!(c.apply_packet(packet(111, -89)));
        assert!(!c.apply_packet(packet(112, -100))); // вне диапазона
        assert_eq!(c.loaded_count(), 2);
        assert!(c.get(100, -100).is_some());
        assert!(c.get(100, -99).is_none());
        // чанк с тем же индексом кольца, но другими координатами — не совпадёт
        assert!(c.get(100 - 23, -100).is_none());
        assert!(c.drop_chunk(100, -100));
        assert!(!c.drop_chunk(100, -100));
        assert_eq!(c.loaded_count(), 1);
    }

    #[test]
    fn replace_keeps_count_and_radius_change_drops_far() {
        let mut c = ChunkCache::new(F, 2); // radius 5
        c.apply_packet(packet(0, 0));
        c.apply_packet(packet(0, 0));
        assert_eq!(c.loaded_count(), 1);
        c.apply_packet(packet(5, 5));
        assert_eq!(c.loaded_count(), 2);
        c.set_view_radius(0); // max(2,0)+3 = 5 → то же
        assert_eq!(c.loaded_count(), 2);
        c.set_center(20, 20);
        c.set_view_radius(3); // radius 6 → старые вне диапазона
        assert_eq!(c.loaded_count(), 0);
    }

    #[test]
    fn world_block_access_and_cow() {
        let mut c = ChunkCache::new(F, 4);
        c.apply_packet(packet(-1, 0));
        let stone = BlockState::from_id(1).unwrap();
        let before = c.get(-1, 0).unwrap().clone();
        assert!(c.set_block(-1, 70, 3, stone));
        assert_eq!(c.block(-1, 70, 3), stone);
        assert_eq!(before.chunk.block(15, 70, 3), BlockState::AIR); // старый Arc не тронут
        assert!(!c.set_block(40, 70, 3, stone));
        assert_eq!(c.block(40, 70, 3), BlockState::AIR);
    }

    #[test]
    fn light_apply() {
        let mut c = ChunkCache::new(F, 4);
        let mut p = packet(0, 0);
        p.light.sky.push((1, DataLayer::filled(15)));
        p.light.empty_block.push(2);
        c.apply_packet(p);
        let ch = c.get(0, 0).unwrap();
        // индекс света 1 = секция min_section (y -64..-49)
        assert_eq!(ch.sky_light(0, -64, 0), Some(15));
        assert_eq!(ch.sky_light(0, -48, 0), None);
        assert_eq!(ch.block_light(0, -48, 0), Some(0));
        let mut u = LightUpdate::default();
        u.block.push((2, DataLayer::filled(7)));
        assert!(c.apply_light(0, 0, &u));
        assert_eq!(c.get(0, 0).unwrap().block_light(1, -48, 1), Some(7));
        assert!(!c.apply_light(9, 9, &u));
    }
}
