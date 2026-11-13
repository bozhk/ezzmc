//! Секция и колонка чанка (TODO-114/115/116).
//! VANILLA: LevelChunkSection, LevelChunk#replaceWithPacketData (1.21.11).
use crate::block::BlockState;
use crate::palette::{Kind, PalettedContainer};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// Параметры измерения, нужные для разбора колонки.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkFormat {
    /// dimension_type.min_y (кратно 16).
    pub min_y: i32,
    /// dimension_type.height (кратно 16).
    pub height: i32,
    /// ceillog2(число состояний блоков).
    pub block_global_bits: u32,
    /// ceillog2(размер реестра биомов) — динамический, из registry_data.
    pub biome_global_bits: u32,
}

impl ChunkFormat {
    pub fn section_count(&self) -> usize {
        (self.height >> 4).max(0) as usize
    }
}

/// VANILLA: LevelChunkSection — 16³ блоков + 4³ биомов.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    non_empty_block_count: i16,
    /// VANILLA (26.3): LevelChunkSection#fluidCount — приходит в пакете; при set_block не ведётся (TODO).
    fluid_count: i16,
    states: PalettedContainer,
    biomes: PalettedContainer,
}

impl Section {
    pub fn empty(f: &ChunkFormat) -> Self {
        Section {
            non_empty_block_count: 0,
            fluid_count: 0,
            states: PalettedContainer::new(Kind::BlockStates, f.block_global_bits, 0),
            biomes: PalettedContainer::new(Kind::Biomes, f.biome_global_bits, 0),
        }
    }

    /// VANILLA: LevelChunkSection#read (26.3) — short blockCount, short fluidCount, states, biomes.
    pub fn read(f: &ChunkFormat, r: &mut Reader<'_>) -> ProtocolResult<Self> {
        let non_empty_block_count = r.i16()?;
        // VANILLA (1.21.11): LevelChunkSection#write без fluidCount; id состояний → канон.
        let legacy = crate::legacy::is_legacy();
        let fluid_count = if legacy { 1 } else { r.i16()? };
        let mut states = PalettedContainer::read(Kind::BlockStates, f.block_global_bits, r)?;
        if legacy {
            states.map_values(crate::legacy::state_to_canon);
        }
        let biomes = PalettedContainer::read(Kind::Biomes, f.biome_global_bits, r)?;
        Ok(Section {
            non_empty_block_count,
            fluid_count,
            states,
            biomes,
        })
    }

    /// VANILLA: hasFluid (26.3).
    pub fn has_fluid(&self) -> bool {
        self.fluid_count > 0
    }

    /// VANILLA: hasOnlyAir.
    pub fn has_only_air(&self) -> bool {
        self.non_empty_block_count == 0
    }

    pub fn states(&self) -> &PalettedContainer {
        &self.states
    }

    pub fn biomes(&self) -> &PalettedContainer {
        &self.biomes
    }

    /// Блок в секции (x,y,z — 0..16).
    pub fn block(&self, x: usize, y: usize, z: usize) -> BlockState {
        BlockState::from_id(self.states.get(x, y, z)).unwrap_or(BlockState::AIR)
    }

    /// Биом по ячейке 4×4×4 (x,y,z — 0..4).
    pub fn biome(&self, x: usize, y: usize, z: usize) -> u32 {
        self.biomes.get(x, y, z)
    }

    /// VANILLA: LevelChunkSection#setBlockState — обновляет счётчик непустых блоков.
    /// (tickingBlockCount/FluidCount не ведутся — нужны только случайным тикам, гл. 8.)
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, state: BlockState) -> BlockState {
        let old =
            BlockState::from_id(self.states.set(x, y, z, state.id())).unwrap_or(BlockState::AIR);
        if !old.is_air() {
            self.non_empty_block_count -= 1;
        }
        if !state.is_air() {
            self.non_empty_block_count += 1;
        }
        old
    }
}

/// Колонка секций по высоте измерения (VANILLA: LevelChunk.sections).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    min_section: i32,
    sections: Vec<Section>,
}

impl Chunk {
    pub fn empty(f: &ChunkFormat) -> Self {
        Chunk {
            min_section: f.min_y >> 4,
            sections: (0..f.section_count()).map(|_| Section::empty(f)).collect(),
        }
    }

    /// VANILLA: replaceWithPacketData — секции подряд снизу вверх.
    pub fn read(f: &ChunkFormat, r: &mut Reader<'_>) -> ProtocolResult<Self> {
        let mut sections = Vec::with_capacity(f.section_count());
        for _ in 0..f.section_count() {
            sections.push(Section::read(f, r)?);
        }
        Ok(Chunk {
            min_section: f.min_y >> 4,
            sections,
        })
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn min_y(&self) -> i32 {
        self.min_section << 4
    }

    /// Максимальный Y (включительно).
    pub fn max_y(&self) -> i32 {
        self.min_y() + (self.sections.len() as i32) * 16 - 1
    }

    fn section_index(&self, y: i32) -> Option<usize> {
        let i = (y >> 4) - self.min_section;
        (i >= 0 && (i as usize) < self.sections.len()).then_some(i as usize)
    }

    pub fn section_at(&self, y: i32) -> Option<&Section> {
        self.section_index(y).map(|i| &self.sections[i])
    }

    /// x,z — внутри чанка (0..16), y — абсолютный. Вне высоты — воздух (как EmptyLevelChunk/ванильный getBlockState).
    pub fn block(&self, x: usize, y: i32, z: usize) -> BlockState {
        match self.section_at(y) {
            Some(s) => s.block(x & 15, (y & 15) as usize, z & 15),
            None => BlockState::AIR,
        }
    }

    /// VANILLA: LevelChunk#setBlockState (только данные). Вне высоты → None.
    pub fn set_block(
        &mut self,
        x: usize,
        y: i32,
        z: usize,
        state: BlockState,
    ) -> Option<BlockState> {
        let i = self.section_index(y)?;
        Some(self.sections[i].set_block(x & 15, (y & 15) as usize, z & 15, state))
    }

    /// VANILLA: getNoiseBiome — квантование по 4 блока (x,z внутри чанка, y абсолютный).
    pub fn biome(&self, x: usize, y: i32, z: usize) -> u32 {
        // BiomeManager#getNoiseBiome(x>>2,y>>2,z>>2); вне высоты зажимаем (как LevelChunk#getNoiseBiome).
        let y = y.clamp(self.min_y(), self.max_y());
        match self.section_at(y) {
            Some(s) => s.biome((x & 15) >> 2, ((y & 15) >> 2) as usize, (z & 15) >> 2),
            None => 0,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    const F: ChunkFormat = ChunkFormat {
        min_y: -64,
        height: 384,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    #[test]
    fn empty_chunk_dimensions() {
        let c = Chunk::empty(&F);
        assert_eq!(c.sections().len(), 24);
        assert_eq!((c.min_y(), c.max_y()), (-64, 319));
        assert_eq!(c.block(0, 100, 0), BlockState::AIR);
        assert_eq!(c.block(0, 400, 0), BlockState::AIR);
        assert_eq!(c.block(0, -65, 0), BlockState::AIR);
    }

    #[test]
    fn set_get_negative_y_and_counts() {
        let mut c = Chunk::empty(&F);
        let stone = BlockState::from_id(1).unwrap();
        assert!(c.section_at(-64).unwrap().has_only_air());
        assert_eq!(c.set_block(3, -64, 5, stone), Some(BlockState::AIR));
        assert_eq!(c.block(3, -64, 5), stone);
        assert!(!c.section_at(-50).unwrap().has_only_air());
        c.set_block(3, -64, 5, BlockState::AIR);
        assert!(c.section_at(-64).unwrap().has_only_air());
        assert_eq!(c.set_block(0, 320, 0, stone), None);
        c.set_block(15, 319, 15, stone);
        assert_eq!(c.block(15, 319, 15), stone);
    }

    #[test]
    fn read_columns_from_wire() {
        // 24 секций: нижняя — камень сплошной (single), остальные воздух.
        let mut w = Writer::new();
        for i in 0..24 {
            w.i16(if i == 0 { 4096 } else { 0 });
            w.i16(0); // fluidCount (26.3)
            w.u8(0).var_int(if i == 0 { 1 } else { 0 }); // states single
            w.u8(0).var_int(3); // biomes single = 3
        }
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let c = Chunk::read(&F, &mut r).unwrap();
        assert!(r.is_empty());
        assert_eq!(c.block(7, -64, 7).id(), 1);
        assert_eq!(c.block(7, -48, 7), BlockState::AIR);
        assert_eq!(c.biome(7, 200, 7), 3);
        assert_eq!(c.biome(7, 9999, 7), 3); // зажим по высоте
                                            // обрезанные данные → ошибка, не паника
        let mut r = Reader::new(&buf[..buf.len() - 1]);
        assert!(Chunk::read(&F, &mut r).is_err());
    }
}
