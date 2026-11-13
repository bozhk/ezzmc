//! Блоки, чанки, палитры, свет, сущности.
pub mod audio_attr;
pub mod biome_color;
pub mod block;
pub mod chunk;
pub mod chunk_packet;
pub mod dimension;
pub mod entities;
pub mod entity_attach;
pub mod entity_size;
pub mod generated;
pub mod legacy;
pub mod level;
pub mod light;
pub mod lightengine;
pub mod lightning;
pub mod noise;
pub mod palette;
pub mod piston;
pub mod precipitation;
pub mod registry;
pub mod riding;
pub mod storage;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use generated::b26_3::{BLOCKS, STATE_COUNT};

    #[test]
    fn decode_oak_stairs_first_state() {
        // reports/blocks.json 26.3: oak_stairs id 5463 = north/top/straight/waterlogged=true
        let (b, idx) = block::decode_state(BLOCKS, 5463).unwrap();
        assert_eq!(BLOCKS[b].name, "oak_stairs");
        assert_eq!(idx, [0, 0, 0, 0]);
        let (b, idx) = block::decode_state(BLOCKS, 5464).unwrap();
        assert_eq!((BLOCKS[b].name, idx[3]), ("oak_stairs", 1));
        assert_eq!(BLOCKS[1].name, "stone");
        assert!(block::decode_state(BLOCKS, STATE_COUNT).is_none());
        assert_eq!(STATE_COUNT, 35723);
    }
}
