//! Ванильные шейдеры 1.21.11 в виде WGSL (TODO-103).
//!
//! Раскладка привязок единая для всех шейдеров:
//! * `group(0)`: 0 Projection, 1 Fog, 2 Globals, 3 DynamicTransforms, 4 ChunkSection,
//!   5 Lighting, 6 CloudInfo, 7 LightmapInfo (uniform-буферы, std140-раскладка из `uniforms`);
//! * `group(1)`: текстуры/сэмплеры/буфер граней облаков (см. заголовок каждого шейдера).
//!
//! Точки входа: `vs_main` / `fs_main`.

const COMMON: &str = include_str!("shaders/common.wgsl");

/// Шейдер ванили, имеющий WGSL-эквивалент.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shader {
    /// `terrain.vsh/fsh` (solid/cutout/translucent — override `alpha_cutout`)
    Terrain,
    /// `sky.vsh/fsh`
    Sky,
    /// `stars.vsh/fsh`
    Stars,
    /// `rendertype_clouds.vsh/fsh`
    Clouds,
    /// `lightmap.fsh` + `screenquad.vsh`
    Lightmap,
    /// `position_color` и `gui`
    PositionColor,
    /// `position_tex_color`
    PositionTexColor,
    /// `position_tex` (CELESTIAL)
    PositionTex,
    /// `particle.vsh/fsh`
    Particle,
    /// `rendertype_lightning.vsh/fsh`
    Lightning,
    /// GPU-скиннинг мобов (entity.vsh + матрицы частей на экземпляр)
    Skinned,
    /// GPU-инстансинг предметов-сущностей (дропы)
    ItemInst,
}

impl Shader {
    pub const ALL: [Shader; 12] = [
        Shader::Terrain,
        Shader::Sky,
        Shader::Stars,
        Shader::Clouds,
        Shader::Lightmap,
        Shader::PositionColor,
        Shader::PositionTexColor,
        Shader::PositionTex,
        Shader::Particle,
        Shader::Lightning,
        Shader::Skinned,
        Shader::ItemInst,
    ];

    fn body(self) -> &'static str {
        match self {
            Shader::Terrain => include_str!("shaders/terrain.wgsl"),
            Shader::Sky => include_str!("shaders/sky.wgsl"),
            Shader::Stars => include_str!("shaders/stars.wgsl"),
            Shader::Clouds => include_str!("shaders/clouds.wgsl"),
            Shader::Lightmap => include_str!("shaders/lightmap.wgsl"),
            Shader::PositionColor => include_str!("shaders/position_color.wgsl"),
            Shader::PositionTexColor => include_str!("shaders/position_tex_color.wgsl"),
            Shader::PositionTex => include_str!("shaders/position_tex.wgsl"),
            Shader::Particle => include_str!("shaders/particle.wgsl"),
            Shader::Lightning => include_str!("shaders/lightning.wgsl"),
            Shader::Skinned => include_str!("shaders/skinned.wgsl"),
            Shader::ItemInst => include_str!("shaders/itemlit.wgsl"),
        }
    }

    /// Полный исходник модуля (общая часть + тело шейдера).
    pub fn source(self) -> String {
        format!("{COMMON}\n{}", self.body())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn all_shaders_parse_and_validate() {
        for s in Shader::ALL {
            let src = s.source();
            let module = naga::front::wgsl::parse_str(&src)
                .unwrap_or_else(|e| panic!("{s:?}: {}", e.emit_to_string(&src)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap_or_else(|e| panic!("{s:?}: {e:?}"));
            let names: Vec<_> = module
                .entry_points
                .iter()
                .map(|e| e.name.as_str())
                .collect();
            assert!(names.contains(&"vs_main"), "{s:?}");
            assert!(names.contains(&"fs_main"), "{s:?}");
        }
    }
}
