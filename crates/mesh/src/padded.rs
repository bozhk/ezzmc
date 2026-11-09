//! Копия блоков и света вокруг секции с отступом 2 (мешеру нужны соседи до ±2 по оси грани).
//! VANILLA: SectionCopy / RenderSectionRegion (1.21.11) + LevelRenderer#getLightColor.
use crate::blockinfo::{self, EMISSIVE, SHADE_DARK};
use crate::section::{Neighborhood, SectionPos};
use ezzmc_world::block::BlockState;

pub const PAD: i32 = 2;
pub const N: usize = 16 + 2 * PAD as usize;

/// VANILLA: LightTexture#pack — `block << 4 | sky << 20`.
pub fn pack_light(block: u32, sky: u32) -> u32 {
    block << 4 | sky << 20
}

/// VANILLA: LightTexture#block
pub fn light_block(packed: u32) -> u32 {
    (packed & 0xFFFF) >> 4
}

/// VANILLA: LightTexture#sky
pub fn light_sky(packed: u32) -> u32 {
    ((packed >> 16) & 0xFFFF) >> 4
}

pub struct Padded {
    blocks: Vec<BlockState>,
    /// `block | sky << 4`.
    light: Vec<u8>,
}

impl Padded {
    pub fn fill(hood: &Neighborhood, base: SectionPos) -> Padded {
        let mut blocks = vec![BlockState::AIR; N * N * N];
        let mut light = vec![0u8; N * N * N];
        let (bx, by, bz) = (base.x * 16, base.y * 16, base.z * 16);
        for y in -PAD..16 + PAD {
            for z in -PAD..16 + PAD {
                for x in -PAD..16 + PAD {
                    let i = Self::idx(x, y, z);
                    blocks[i] = hood.block(base, bx + x, by + y, bz + z);
                    if (-1..17).contains(&x) && (-1..17).contains(&y) && (-1..17).contains(&z) {
                        let (b, s) = hood.light(base, bx + x, by + y, bz + z);
                        light[i] = (b & 15) | (s & 15) << 4;
                    }
                }
            }
        }
        Padded { blocks, light }
    }

    /// VANILLA: MovingBlockRenderState#getBlockState — воздух везде, кроме `p`; свет остаётся из мира.
    pub fn isolate(&mut self, p: [i32; 3], state: BlockState) {
        self.blocks.fill(BlockState::AIR);
        if Self::inside(p[0], p[1], p[2]) {
            self.blocks[Self::idx(p[0], p[1], p[2])] = state;
        }
    }

    /// Для тестов: заполнить по локальным координатам.
    pub fn from_fn(f: impl Fn(i32, i32, i32) -> BlockState) -> Padded {
        let mut blocks = vec![BlockState::AIR; N * N * N];
        for y in -PAD..16 + PAD {
            for z in -PAD..16 + PAD {
                for x in -PAD..16 + PAD {
                    blocks[Self::idx(x, y, z)] = f(x, y, z);
                }
            }
        }
        Padded {
            blocks,
            light: vec![0; N * N * N],
        }
    }

    #[inline]
    fn idx(x: i32, y: i32, z: i32) -> usize {
        (((y + PAD) as usize * N) + (z + PAD) as usize) * N + (x + PAD) as usize
    }

    #[inline]
    fn inside(x: i32, y: i32, z: i32) -> bool {
        let r = -PAD..16 + PAD;
        r.contains(&x) && r.contains(&y) && r.contains(&z)
    }

    /// Локальные координаты (−2..18). Вне копии — воздух.
    #[inline]
    pub fn block(&self, p: [i32; 3]) -> BlockState {
        if Self::inside(p[0], p[1], p[2]) {
            self.blocks[Self::idx(p[0], p[1], p[2])]
        } else {
            BlockState::AIR
        }
    }

    /// `BrightnessGetter.DEFAULT`: сырые уровни block/sky в клетке.
    #[inline]
    pub fn raw_light(&self, p: [i32; 3]) -> u32 {
        if Self::inside(p[0], p[1], p[2]) {
            let v = u32::from(self.light[Self::idx(p[0], p[1], p[2])]);
            pack_light(v & 15, v >> 4)
        } else {
            0
        }
    }

    /// VANILLA: LevelRenderer#getLightColor(getter, level, state, pos)
    pub fn light_color(&self, state: BlockState, p: [i32; 3]) -> u32 {
        if blockinfo::has(state, EMISSIVE) {
            return 0xF0_00F0;
        }
        let i = self.raw_light(p);
        let emission = state.props().map_or(0, |s| u32::from(s.light_emission));
        if light_block(i) < emission {
            pack_light(emission, light_sky(i))
        } else {
            i
        }
    }

    /// VANILLA: BlockState#getShadeBrightness
    #[inline]
    pub fn shade_brightness(&self, p: [i32; 3]) -> f32 {
        if blockinfo::has(self.block(p), SHADE_DARK) {
            0.2
        } else {
            1.0
        }
    }
}
