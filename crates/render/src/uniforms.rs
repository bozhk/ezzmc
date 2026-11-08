//! Std140-раскладка uniform-блоков ванили (`include/*.glsl`) для WGSL-шейдеров.
//! Совпадает с правилами WGSL `uniform` (vec3 выравнивается по 16, размер структуры кратен 16).
use glam::Mat4;

fn f(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn i(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn mat(out: &mut Vec<u8>, m: &Mat4) {
    for v in m.to_cols_array() {
        f(out, v);
    }
}
fn pad_to(out: &mut Vec<u8>, size: usize) {
    out.resize(size, 0);
}

/// `Projection` (binding 0), 64 байта.
#[derive(Debug, Clone, Copy)]
pub struct ProjectionUniform {
    pub proj_mat: Mat4,
}
impl ProjectionUniform {
    pub const SIZE: usize = 64;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        mat(&mut o, &self.proj_mat);
        o
    }
}

/// `Fog` (binding 1), 48 байт.
#[derive(Debug, Clone, Copy)]
pub struct FogUniform {
    pub color: [f32; 4],
    pub environmental_start: f32,
    pub environmental_end: f32,
    pub render_distance_start: f32,
    pub render_distance_end: f32,
    pub sky_end: f32,
    pub clouds_end: f32,
}
impl FogUniform {
    pub const SIZE: usize = 48;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        for v in self.color {
            f(&mut o, v);
        }
        for v in [
            self.environmental_start,
            self.environmental_end,
            self.render_distance_start,
            self.render_distance_end,
            self.sky_end,
            self.clouds_end,
        ] {
            f(&mut o, v);
        }
        pad_to(&mut o, Self::SIZE);
        o
    }
}

/// `Globals` (binding 2), 64 байта.
#[derive(Debug, Clone, Copy)]
pub struct GlobalsUniform {
    pub camera_block_pos: [i32; 3],
    pub camera_offset: [f32; 3],
    pub screen_size: [f32; 2],
    pub glint_alpha: f32,
    pub game_time: f32,
    pub menu_blur_radius: i32,
    pub use_rgss: i32,
}
impl GlobalsUniform {
    pub const SIZE: usize = 64;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        for v in self.camera_block_pos {
            i(&mut o, v);
        }
        pad_to(&mut o, 16);
        for v in self.camera_offset {
            f(&mut o, v);
        }
        pad_to(&mut o, 32);
        f(&mut o, self.screen_size[0]);
        f(&mut o, self.screen_size[1]);
        f(&mut o, self.glint_alpha);
        f(&mut o, self.game_time);
        i(&mut o, self.menu_blur_radius);
        i(&mut o, self.use_rgss);
        pad_to(&mut o, Self::SIZE);
        o
    }
}

/// `DynamicTransforms` (binding 3), 160 байт.
#[derive(Debug, Clone, Copy)]
pub struct DynamicTransformsUniform {
    pub model_view_mat: Mat4,
    pub color_modulator: [f32; 4],
    pub model_offset: [f32; 3],
    pub texture_mat: Mat4,
}
impl DynamicTransformsUniform {
    pub const SIZE: usize = 160;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        mat(&mut o, &self.model_view_mat);
        for v in self.color_modulator {
            f(&mut o, v);
        }
        for v in self.model_offset {
            f(&mut o, v);
        }
        pad_to(&mut o, 96);
        mat(&mut o, &self.texture_mat);
        o
    }
}

/// `ChunkSection` (binding 4), 96 байт.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSectionUniform {
    pub model_view_mat: Mat4,
    pub chunk_visibility: f32,
    pub texture_size: [i32; 2],
    pub chunk_position: [i32; 3],
}
impl ChunkSectionUniform {
    pub const SIZE: usize = 96;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        mat(&mut o, &self.model_view_mat);
        f(&mut o, self.chunk_visibility);
        pad_to(&mut o, 72);
        i(&mut o, self.texture_size[0]);
        i(&mut o, self.texture_size[1]);
        pad_to(&mut o, 80);
        for v in self.chunk_position {
            i(&mut o, v);
        }
        pad_to(&mut o, Self::SIZE);
        o
    }
}

/// `Lighting` (binding 5), 32 байта.
#[derive(Debug, Clone, Copy)]
pub struct LightingUniform {
    pub light0_direction: [f32; 3],
    pub light1_direction: [f32; 3],
}
impl LightingUniform {
    pub const SIZE: usize = 32;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        for v in self.light0_direction {
            f(&mut o, v);
        }
        pad_to(&mut o, 16);
        for v in self.light1_direction {
            f(&mut o, v);
        }
        pad_to(&mut o, Self::SIZE);
        o
    }
}

/// `CloudInfo` (binding 6), 48 байт.
#[derive(Debug, Clone, Copy)]
pub struct CloudInfoUniform {
    pub cloud_color: [f32; 4],
    pub cloud_offset: [f32; 3],
    pub cell_size: [f32; 3],
}
impl CloudInfoUniform {
    pub const SIZE: usize = 48;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        for v in self.cloud_color {
            f(&mut o, v);
        }
        for v in self.cloud_offset {
            f(&mut o, v);
        }
        pad_to(&mut o, 32);
        for v in self.cell_size {
            f(&mut o, v);
        }
        pad_to(&mut o, Self::SIZE);
        o
    }
}

/// `LightmapInfo` (binding 7), 64 байта.
#[derive(Debug, Clone, Copy)]
pub struct LightmapInfoUniform {
    pub ambient_light_factor: f32,
    pub sky_factor: f32,
    pub block_factor: f32,
    pub night_vision_factor: f32,
    pub darkness_scale: f32,
    pub darken_world_factor: f32,
    pub brightness_factor: f32,
    pub sky_light_color: [f32; 3],
    pub ambient_color: [f32; 3],
}
impl LightmapInfoUniform {
    pub const SIZE: usize = 64;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(Self::SIZE);
        for v in [
            self.ambient_light_factor,
            self.sky_factor,
            self.block_factor,
            self.night_vision_factor,
            self.darkness_scale,
            self.darken_world_factor,
            self.brightness_factor,
        ] {
            f(&mut o, v);
        }
        pad_to(&mut o, 32);
        for v in self.sky_light_color {
            f(&mut o, v);
        }
        pad_to(&mut o, 48);
        for v in self.ambient_color {
            f(&mut o, v);
        }
        pad_to(&mut o, Self::SIZE);
        o
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Размеры и смещения должны совпадать с тем, что считает naga для WGSL-структур.
    #[test]
    fn sizes_match_wgsl_layout() {
        let src = format!(
            "{}\n@vertex fn vs_main() -> @builtin(position) vec4<f32> {{ return vec4<f32>(0.0); }}",
            include_str!("shaders/common.wgsl")
        );
        let module = naga::front::wgsl::parse_str(&src).expect("parse");
        let mut layouter = naga::proc::Layouter::default();
        layouter.update(module.to_ctx()).expect("layout");
        let size_of = |name: &str| {
            let (h, _) = module
                .types
                .iter()
                .find(|(_, t)| t.name.as_deref() == Some(name))
                .expect(name);
            layouter[h].to_stride() as usize
        };
        assert_eq!(size_of("Projection"), ProjectionUniform::SIZE);
        assert_eq!(size_of("Fog"), FogUniform::SIZE);
        assert_eq!(size_of("Globals"), GlobalsUniform::SIZE);
        assert_eq!(size_of("DynamicTransforms"), DynamicTransformsUniform::SIZE);
        assert_eq!(size_of("ChunkSection"), ChunkSectionUniform::SIZE);
        assert_eq!(size_of("Lighting"), LightingUniform::SIZE);
        assert_eq!(size_of("CloudInfo"), CloudInfoUniform::SIZE);
        assert_eq!(size_of("LightmapInfo"), LightmapInfoUniform::SIZE);
    }

    #[test]
    fn byte_lengths() {
        let m = Mat4::IDENTITY;
        assert_eq!(ProjectionUniform { proj_mat: m }.to_bytes().len(), 64);
        assert_eq!(
            GlobalsUniform {
                camera_block_pos: [1, 2, 3],
                camera_offset: [0.5; 3],
                screen_size: [1.0; 2],
                glint_alpha: 0.0,
                game_time: 0.0,
                menu_blur_radius: 0,
                use_rgss: 0
            }
            .to_bytes()
            .len(),
            64
        );
        let g = GlobalsUniform {
            camera_block_pos: [1, 2, 3],
            camera_offset: [0.5, 0.25, 0.125],
            screen_size: [9.0, 8.0],
            glint_alpha: 0.0,
            game_time: 0.0,
            menu_blur_radius: 7,
            use_rgss: 1,
        }
        .to_bytes();
        assert_eq!(i32::from_le_bytes(g[0..4].try_into().unwrap()), 1);
        assert_eq!(f32::from_le_bytes(g[16..20].try_into().unwrap()), 0.5);
        assert_eq!(f32::from_le_bytes(g[32..36].try_into().unwrap()), 9.0);
        assert_eq!(i32::from_le_bytes(g[48..52].try_into().unwrap()), 7);
        assert_eq!(i32::from_le_bytes(g[52..56].try_into().unwrap()), 1);
        let c = ChunkSectionUniform {
            model_view_mat: m,
            chunk_visibility: 1.0,
            texture_size: [1024, 512],
            chunk_position: [16, 32, 48],
        }
        .to_bytes();
        assert_eq!(i32::from_le_bytes(c[72..76].try_into().unwrap()), 1024);
        assert_eq!(i32::from_le_bytes(c[80..84].try_into().unwrap()), 16);
    }
}
