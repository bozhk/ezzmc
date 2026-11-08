//! Слои рендера секции (TODO-141): порт `ItemBlockRenderTypes#getChunkRenderType` + `ChunkSectionLayer` 1.21.11.
//! Порядок отрисовки = порядок вариантов: solid, cutout, translucent, tripwire.
use crate::layers_table::{CUTOUT, CUTOUT_GROUPS, TRANSLUCENT, TRANSLUCENT_GROUPS, TRIPWIRE};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RenderLayer {
    Solid = 0,
    Cutout = 1,
    Translucent = 2,
    Tripwire = 3,
}

impl RenderLayer {
    pub const COUNT: usize = 4;
    pub const ALL: [RenderLayer; 4] = [
        RenderLayer::Solid,
        RenderLayer::Cutout,
        RenderLayer::Translucent,
        RenderLayer::Tripwire,
    ];

    pub fn index(self) -> usize {
        self as usize
    }

    /// Порог альфа-отсечения в шейдере (`alpha_cutout`).
    // VANILLA: RenderPipelines.*_TERRAIN: cutout 0.5, translucent 0.01, tripwire 0.1, solid — нет
    pub fn alpha_cutout(self) -> f32 {
        match self {
            RenderLayer::Solid => 0.0,
            RenderLayer::Translucent => 0.01,
            RenderLayer::Cutout => 0.5,
            RenderLayer::Tripwire => 0.1,
        }
    }

    pub fn translucent(self) -> bool {
        matches!(self, RenderLayer::Translucent | RenderLayer::Tripwire)
    }
}

/// Слой по имени блока (без `minecraft:`).
pub fn layer_of_name(name: &str, cutout_leaves: bool) -> RenderLayer {
    // VANILLA: `instanceof LeavesBlock` → CUTOUT при «красивой» графике, иначе SOLID
    if name.ends_with("_leaves") {
        return if cutout_leaves {
            RenderLayer::Cutout
        } else {
            RenderLayer::Solid
        };
    }
    let sets = LayerSets::get();
    if sets.tripwire.contains(name) {
        RenderLayer::Tripwire
    } else if sets.cutout.contains(name) || CUTOUT_GROUPS.iter().any(|g| name.ends_with(g)) {
        RenderLayer::Cutout
    } else if sets.translucent.contains(name)
        || TRANSLUCENT_GROUPS.iter().any(|g| name.ends_with(g))
    {
        RenderLayer::Translucent
    } else {
        RenderLayer::Solid
    }
}

struct LayerSets {
    cutout: HashSet<&'static str>,
    translucent: HashSet<&'static str>,
    tripwire: HashSet<&'static str>,
}

impl LayerSets {
    fn get() -> &'static LayerSets {
        static S: std::sync::OnceLock<LayerSets> = std::sync::OnceLock::new();
        S.get_or_init(|| LayerSets {
            cutout: CUTOUT.iter().copied().collect(),
            translucent: TRANSLUCENT.iter().copied().collect(),
            tripwire: TRIPWIRE.iter().copied().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_layers() {
        assert_eq!(layer_of_name("stone", true), RenderLayer::Solid);
        assert_eq!(layer_of_name("oak_leaves", true), RenderLayer::Cutout);
        assert_eq!(layer_of_name("oak_leaves", false), RenderLayer::Solid);
        assert_eq!(layer_of_name("glass", true), RenderLayer::Translucent);
        assert_eq!(layer_of_name("grass_block", true), RenderLayer::Cutout);
        assert_eq!(
            layer_of_name("waxed_exposed_copper_bars", true),
            RenderLayer::Cutout
        );
        assert_eq!(layer_of_name("tripwire", true), RenderLayer::Tripwire);
        assert_eq!(
            layer_of_name("blue_stained_glass_pane", true),
            RenderLayer::Translucent
        );
    }
}
