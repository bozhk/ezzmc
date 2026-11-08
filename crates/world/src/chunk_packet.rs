//! Разбор пакета Chunk Data and Update Light (TODO-117).
//! VANILLA: ClientboundLevelChunkWithLightPacket / ClientboundLevelChunkPacketData (1.21.11).
use crate::chunk::{Chunk, ChunkFormat};
use crate::light::LightUpdate;
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::{ProtocolError, ProtocolResult};

const MAX_CHUNK_BYTES: usize = 2 * 1024 * 1024;

/// VANILLA: Heightmap.Types (id как в STREAM_CODEC).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeightmapType {
    WorldSurfaceWg = 0,
    WorldSurface = 1,
    OceanFloorWg = 2,
    OceanFloor = 3,
    MotionBlocking = 4,
    MotionBlockingNoLeaves = 5,
}

impl HeightmapType {
    /// ByIdMap.continuous(..., OutOfBoundsStrategy.ZERO).
    pub fn from_id(id: i32) -> HeightmapType {
        match id {
            1 => Self::WorldSurface,
            2 => Self::OceanFloorWg,
            3 => Self::OceanFloor,
            4 => Self::MotionBlocking,
            5 => Self::MotionBlockingNoLeaves,
            _ => Self::WorldSurfaceWg,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockEntityInfo {
    /// Позиция внутри чанка.
    pub x: u8,
    pub z: u8,
    pub y: i32,
    /// id в реестре block_entity_type.
    pub type_id: u32,
    pub tag: Option<Tag>,
}

#[derive(Clone, Debug)]
pub struct LevelChunkPacket {
    pub x: i32,
    pub z: i32,
    pub chunk: Chunk,
    /// Сырые битово-упакованные long[] (формат SimpleBitStorage, 9 бит) по типам.
    pub heightmaps: Vec<(HeightmapType, Vec<i64>)>,
    pub block_entities: Vec<BlockEntityInfo>,
    pub light: LightUpdate,
}

impl LevelChunkPacket {
    pub fn read(f: &ChunkFormat, r: &mut Reader<'_>) -> ProtocolResult<Self> {
        let x = r.i32()?;
        let z = r.i32()?;
        // heightmaps: map<enum, long[]>
        let n = r.length("heightmaps", 16)?;
        let mut heightmaps = Vec::with_capacity(n);
        for _ in 0..n {
            let ty = HeightmapType::from_id(r.var_int()?);
            let len = r.length("heightmap longs", 1 << 12)?;
            let mut v = Vec::with_capacity(len.min(r.remaining() / 8));
            for _ in 0..len {
                v.push(r.i64()?);
            }
            heightmaps.push((ty, v));
        }
        let size = r.length("chunk buffer", MAX_CHUNK_BYTES)?;
        let buf = r.bytes(size)?;
        let mut br = Reader::new(buf);
        let chunk = Chunk::read(f, &mut br)?;
        // Остаток буфера после секций ванилью не проверяется.
        let ne = r.length("block entities", 1 << 20)?;
        let mut block_entities = Vec::with_capacity(ne.min(r.remaining() / 4));
        for _ in 0..ne {
            let packed = r.u8()?;
            let y = i32::from(r.i16()?);
            let type_id = u32::try_from(r.var_int()?).map_err(|_| ProtocolError::InvalidValue {
                what: "block entity type id",
                value: -1,
            })?;
            let tag = nbt::read_network(r)?;
            block_entities.push(BlockEntityInfo {
                x: packed >> 4 & 15,
                z: packed & 15,
                y,
                type_id,
                tag,
            });
        }
        let light = LightUpdate::read(r)?;
        Ok(LevelChunkPacket {
            x,
            z,
            chunk,
            heightmaps,
            block_entities,
            light,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    const F: ChunkFormat = ChunkFormat {
        min_y: 0,
        height: 32,
        block_global_bits: 15,
        biome_global_bits: 6,
    };

    #[test]
    fn parse_full_packet() {
        let mut sec = Writer::new();
        for _ in 0..2 {
            sec.i16(0).i16(0).u8(0).var_int(0).u8(0).var_int(1);
        }
        let sec = sec.into_vec();
        let mut w = Writer::new();
        w.i32(-3).i32(7);
        w.var_int(1).var_int(4).var_int(1).i64(0x1234);
        w.var_int(sec.len() as i32).bytes(&sec);
        w.var_int(1);
        w.u8(0x35).i16(70).var_int(7).u8(0x0a).u8(0); // пустой compound
        for _ in 0..4 {
            w.bitset_bytes(&[]);
        }
        w.var_int(0).var_int(0);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let p = LevelChunkPacket::read(&F, &mut r).unwrap();
        assert!(r.is_empty());
        assert_eq!((p.x, p.z), (-3, 7));
        assert_eq!(
            p.heightmaps,
            vec![(HeightmapType::MotionBlocking, vec![0x1234])]
        );
        assert_eq!(p.chunk.sections().len(), 2);
        assert_eq!(p.chunk.biome(0, 0, 0), 1);
        let be = &p.block_entities[0];
        assert_eq!((be.x, be.z, be.y, be.type_id), (3, 5, 70, 7));
        assert!(be.tag.is_some());
    }

    #[test]
    fn truncated_and_oversized_do_not_panic() {
        let mut w = Writer::new();
        w.i32(0).i32(0).var_int(0).var_int(3_000_000);
        let buf = w.into_vec();
        assert!(LevelChunkPacket::read(&F, &mut Reader::new(&buf)).is_err());
        for cut in 0..buf.len() {
            let _ = LevelChunkPacket::read(&F, &mut Reader::new(&buf[..cut]));
        }
    }
}
