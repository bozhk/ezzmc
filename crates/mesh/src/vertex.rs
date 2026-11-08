//! Упакованная вершина террейна, 16 байт (TODO-135).
//!
//! Раскладка (little-endian), под вершинные форматы wgpu:
//! * `0..8`  — `Uint16x4`: x, y, z позиции в секции и слово света;
//! * `8..12` — `Unorm16x2`: UV в атласе;
//! * `12..16`— `Unorm8x4`: цвет (тинт × затенение/AO) RGBA.
//!
//! Позиция: `u16 = (coord + 8) * 2048`, диапазон секции −8..24 блоков, шаг 1/2048 (точнее 1/16 пикселя модели).
//! Свет: `block_u8 | sky_u8 << 8` — ванильный `uv2` (0..240 = уровень·16, дробный из-за сглаживания),
//! шейдер делит на 256 и сэмплирует lightmap.
use ezzmc_assets::bake::BakedQuad;

pub const POS_BIAS: f32 = 8.0;
pub const POS_SCALE: f32 = 2048.0;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackedVertex {
    pub pos: [u16; 3],
    pub light: u16,
    pub uv: [u16; 2],
    pub color: [u8; 4],
}

impl PackedVertex {
    pub const SIZE: usize = 16;

    pub fn pack(pos: [f32; 3], uv: [f32; 2], color: [u8; 4], block: u8, sky: u8) -> Self {
        let p = |c: f32| ((c + POS_BIAS) * POS_SCALE).round().clamp(0.0, 65535.0) as u16;
        let t = |c: f32| (c.clamp(0.0, 1.0) * 65535.0).round() as u16;
        PackedVertex {
            pos: [p(pos[0]), p(pos[1]), p(pos[2])],
            light: u16::from(block) | (u16::from(sky) << 8),
            uv: [t(uv[0]), t(uv[1])],
            color,
        }
    }

    pub fn position(&self) -> [f32; 3] {
        self.pos.map(|v| f32::from(v) / POS_SCALE - POS_BIAS)
    }

    pub fn uv(&self) -> [f32; 2] {
        self.uv.map(|v| f32::from(v) / 65535.0)
    }

    pub fn block_light(&self) -> u8 {
        (self.light & 255) as u8
    }

    pub fn sky_light(&self) -> u8 {
        (self.light >> 8) as u8
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut o = [0u8; 16];
        for (i, v) in self.pos.iter().enumerate() {
            o[i * 2..i * 2 + 2].copy_from_slice(&v.to_le_bytes());
        }
        o[6..8].copy_from_slice(&self.light.to_le_bytes());
        o[8..10].copy_from_slice(&self.uv[0].to_le_bytes());
        o[10..12].copy_from_slice(&self.uv[1].to_le_bytes());
        o[12..16].copy_from_slice(&self.color);
        o
    }
}

/// 4 вершины квада в локальных координатах секции (`origin` — позиция блока в секции).
/// `color` и `light` — по вершинам (`light` = ванильный packed `block<<4 | sky<<20`).
pub fn pack_quad(
    q: &BakedQuad,
    origin: [f32; 3],
    color: [[u8; 4]; 4],
    light: [u32; 4],
) -> [PackedVertex; 4] {
    std::array::from_fn(|i| {
        let p = q.pos[i];
        PackedVertex::pack(
            [p[0] + origin[0], p[1] + origin[1], p[2] + origin[2]],
            q.uv[i],
            color[i],
            (light[i] & 0xFF) as u8,
            ((light[i] >> 16) & 0xFF) as u8,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_size() {
        assert_eq!(std::mem::size_of::<PackedVertex>(), PackedVertex::SIZE);
        let v = PackedVertex::pack([15.5, -0.0625, 3.25], [0.25, 0.75], [1, 2, 3, 4], 112, 240);
        assert_eq!(v.position(), [15.5, -0.0625, 3.25]);
        assert!((v.uv()[0] - 0.25).abs() < 1e-4 && (v.uv()[1] - 0.75).abs() < 1e-4);
        assert_eq!((v.block_light(), v.sky_light()), (112, 240));
        let b = v.to_bytes();
        assert_eq!(u16::from_le_bytes([b[6], b[7]]), 112 | (240 << 8));
        assert_eq!(&b[12..], &[1, 2, 3, 4]);
        // границы диапазона зажимаются
        assert_eq!(
            PackedVertex::pack([-100.0, 100.0, 0.0], [0.0; 2], [0; 4], 0, 0).pos[..2],
            [0, 65535]
        );
    }
}
