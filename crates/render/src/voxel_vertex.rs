//! Вершина воксельной секции в одном `u32` (эксперимент для полных кубов).
//!
//! ```text
//! bits  0..5   x      (0..31; секция 0..16, запас под границы/AO-соседей)
//!       5..10  y
//!       10..15 z
//!       15..18 face   (Direction: 0 down,1 up,2 north,3 south,4 west,5 east)
//!       18..20 ao     (0..3)
//!       20..24 block light (0..15)
//!       24..28 sky light   (0..15)
//!       28..30 corner (0..3 — угол квада → UV)
//!       30..32 резерв
//! ```
//! ID текстуры (>= 10 бит) в 32 бита не влезает, поэтому он лежит в `QuadMeta` (по одному на 4 вершины,
//! индекс = `vertex_index / 4`). Тинт биома тоже в `QuadMeta`.
//!
//! Это НЕ замена `ezzmc_mesh::vertex::PackedVertex` (16 Б): ванильные модели (кресты, лестницы,
//! дробные UV, сглаженный свет) в 5 бит на координату не выражаются.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VoxelVertex(pub u32);

impl VoxelVertex {
    pub const SIZE: usize = 4;

    pub fn pack(pos: [u8; 3], face: u8, ao: u8, block: u8, sky: u8, corner: u8) -> Self {
        debug_assert!(pos.iter().all(|&c| c < 32) && face < 6 && ao < 4);
        debug_assert!(block < 16 && sky < 16 && corner < 4);
        let v = u32::from(pos[0] & 31)
            | u32::from(pos[1] & 31) << 5
            | u32::from(pos[2] & 31) << 10
            | u32::from(face & 7) << 15
            | u32::from(ao & 3) << 18
            | u32::from(block & 15) << 20
            | u32::from(sky & 15) << 24
            | u32::from(corner & 3) << 28;
        Self(v)
    }

    pub fn pos(self) -> [u8; 3] {
        [
            (self.0 & 31) as u8,
            (self.0 >> 5 & 31) as u8,
            (self.0 >> 10 & 31) as u8,
        ]
    }
    pub fn face(self) -> u8 {
        (self.0 >> 15 & 7) as u8
    }
    pub fn ao(self) -> u8 {
        (self.0 >> 18 & 3) as u8
    }
    pub fn block_light(self) -> u8 {
        (self.0 >> 20 & 15) as u8
    }
    pub fn sky_light(self) -> u8 {
        (self.0 >> 24 & 15) as u8
    }
    pub fn corner(self) -> u8 {
        (self.0 >> 28 & 3) as u8
    }
}

/// Данные на квад (4 вершины): слой атласа и тинт RGBA8.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct QuadMeta {
    pub texture: u32,
    pub tint: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_extremes() {
        let v = VoxelVertex::pack([31, 0, 17], 5, 3, 15, 9, 2);
        assert_eq!(v.pos(), [31, 0, 17]);
        assert_eq!(
            (v.face(), v.ao(), v.block_light(), v.sky_light(), v.corner()),
            (5, 3, 15, 9, 2)
        );
        assert_eq!(v.0 >> 30, 0);
    }
}
