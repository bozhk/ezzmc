//! Рендер-свойства состояний для мешера: таблица флагов по id состояния (строится один раз).
//! VANILLA: BlockBehaviour.BlockStateBase#isCollisionShapeFullBlock / isSolidRender / isViewBlocking /
//! emissiveRendering / getShadeBrightness (1.21.11).
use ezzmc_world::block::{BlockState, FLAG_BLOCKS_MOTION, FLAG_CAN_OCCLUDE};
use ezzmc_world::generated::b26_3::{BLOCKS, STATE_COUNT};
use ezzmc_world::generated::p26_3::SHAPES;
use std::sync::OnceLock;

/// `Block.isShapeFullBlock(collisionShape)`.
pub const COLLISION_FULL: u8 = 1;
/// `isViewBlocking` (по умолчанию = `isSuffocating` = `blocksMotion && collisionFull`).
pub const VIEW_BLOCKING: u8 = 2;
/// `emissiveRendering` (магма, активный sculk sensor) — свет 0xF000F0.
pub const EMISSIVE: u8 = 4;
/// `isSolidRender`: `canOcclude` и полный куб формы окклюзии.
pub const SOLID_RENDER: u8 = 8;
/// `getShadeBrightness == 0.2` (иначе 1.0).
pub const SHADE_DARK: u8 = 16;
/// `OffsetType.XZ` (цветы, высокая трава, бамбук…).
pub const OFFSET_XZ: u8 = 32;
/// `OffsetType.XYZ` (трава, папоротник, малый капельник…).
pub const OFFSET_XYZ: u8 = 64;

fn is_full(boxes: &[[f64; 6]]) -> bool {
    boxes.iter().any(|b| {
        b[0] <= 0.0 && b[1] <= 0.0 && b[2] <= 0.0 && b[3] >= 1.0 && b[4] >= 1.0 && b[5] >= 1.0
    })
}

fn shape(i: u16) -> &'static [[f64; 6]] {
    SHAPES.get(usize::from(i)).copied().unwrap_or(&[])
}

/// VANILLA: BlockState#getCollisionShape по индексу формы.
pub fn collision_boxes_of(i: u16) -> &'static [[f64; 6]] {
    shape(i)
}

/// VANILLA: BlockState#getShape (контур выбора), боксы `[x0,y0,z0,x1,y1,z1]` в координатах блока.
pub fn outline_boxes(state: BlockState) -> &'static [[f64; 6]] {
    state.props().map_or(&[], |p| shape(p.outline_shape))
}

fn compute(state: BlockState) -> u8 {
    let Some(p) = state.props() else { return 0 };
    let name = state.name();
    let mut f = 0u8;
    let full = is_full(shape(p.collision_shape));
    if full {
        f |= COLLISION_FULL;
    }
    if p.has(FLAG_CAN_OCCLUDE) && is_full(shape(p.outline_shape)) {
        f |= SOLID_RENDER;
    }
    // VANILLA: Blocks.java — isViewBlocking(never/always/…) поверх умолчания
    let view = match name {
        "mangrove_roots"
        | "glass"
        | "tinted_glass"
        | "moving_piston"
        | "trial_spawner"
        | "vault"
        | "test_instance_block" => false,
        "farmland" | "soul_sand" | "dirt_path" | "mud" => true,
        "snow" => state.property("layers").and_then(|v| v.parse::<u8>().ok()) >= Some(8),
        "piston" | "sticky_piston" => state.property("extended") != Some("true"),
        n if n.ends_with("_leaves")
            || n.ends_with("_stained_glass")
            || n.ends_with("copper_grate") =>
        {
            false
        }
        // закрытая шалкер-коробка (block entity нет в мешере — считаем закрытой)
        n if n.ends_with("shulker_box") => true,
        _ => p.has(FLAG_BLOCKS_MOTION) && full,
    };
    if view {
        f |= VIEW_BLOCKING;
    }
    if name == "magma_block"
        || (name.ends_with("sculk_sensor")
            && state.property("sculk_sensor_phase") == Some("active"))
    {
        f |= EMISSIVE;
    }
    // VANILLA: getShadeBrightness: по умолчанию full ? 0.2 : 1.0 + переопределения блоков
    let dark = match name {
        "soul_sand" | "mud" => true,
        "structure_void" | "barrier" | "light" | "glass" | "tinted_glass" => false,
        "snow" => state.property("layers") == Some("8"),
        n if n.ends_with("_stained_glass") || n.ends_with("copper_grate") => false,
        _ => full,
    };
    if dark {
        f |= SHADE_DARK;
    }
    f | offset_flag(name)
}

// VANILLA: Blocks.java `.offsetType(...)` (1.21.11)
fn offset_flag(name: &str) -> u8 {
    match name {
        "short_grass" | "fern" | "short_dry_grass" | "tall_dry_grass" | "small_dripleaf" => {
            OFFSET_XYZ
        }
        "mangrove_propagule" | "tall_seagrass" | "dandelion" | "torchflower" | "poppy"
        | "blue_orchid" | "allium" | "azure_bluet" | "red_tulip" | "orange_tulip"
        | "white_tulip" | "pink_tulip" | "oxeye_daisy" | "cornflower" | "wither_rose"
        | "lily_of_the_valley" | "sunflower" | "lilac" | "rose_bush" | "peony" | "tall_grass"
        | "large_fern" | "pitcher_plant" | "bamboo_sapling" | "bamboo" | "warped_roots"
        | "nether_sprouts" | "crimson_roots" | "pointed_dripstone" | "hanging_roots"
        | "open_eyeblossom" | "closed_eyeblossom" => OFFSET_XZ,
        _ => 0,
    }
}

/// VANILLA: Mth#getSeed(x, 0, z) (int-умножение переполняется как в Java).
fn mth_seed(x: i32, y: i32, z: i32) -> i64 {
    let a = i64::from(x.wrapping_mul(3_129_871))
        ^ i64::from(z).wrapping_mul(116_129_781)
        ^ i64::from(y);
    a.wrapping_mul(a)
        .wrapping_mul(42_317_861)
        .wrapping_add(a.wrapping_mul(11))
        >> 16
}

/// VANILLA: BlockStateBase#getOffset(pos) — смещение модели для блоков с offsetType (мировые x, z).
pub fn block_offset(state: BlockState, x: i32, z: i32) -> [f32; 3] {
    let fl = flags(state);
    if fl & (OFFSET_XZ | OFFSET_XYZ) == 0 {
        return [0.0; 3];
    }
    let seed = mth_seed(x, 0, z);
    let name = state.name();
    // getMaxHorizontalOffset: 0.25 (pointed_dripstone 0.125); getMaxVerticalOffset: 0.2 (small_dripleaf 0.1)
    let max_h: f32 = if name == "pointed_dripstone" {
        0.125
    } else {
        0.25
    };
    let max_v: f32 = if name == "small_dripleaf" { 0.1 } else { 0.2 };
    let h = |s: i64| {
        let v = (f64::from((s & 15) as f32 / 15.0_f32) - 0.5) * 0.5;
        v.clamp(-f64::from(max_h), f64::from(max_h))
    };
    let dx = h(seed);
    let dz = h(seed >> 8);
    let dy = if fl & OFFSET_XYZ != 0 {
        (f64::from(((seed >> 4) & 15) as f32 / 15.0_f32) - 1.0) * f64::from(max_v)
    } else {
        0.0
    };
    [dx as f32, dy as f32, dz as f32]
}

/// Жидкость в клетке (`BlockState#getFluidState`).
// VANILLA: LiquidBlock#getFluidState (level 0 = источник, 1..7 = течение amount 8−level, ≥8 = падающая),
// BlockStateProperties.WATERLOGGED → источник воды, ламинарные «водные» блоки (келп, морская трава, пузыри).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fluid {
    pub lava: bool,
    /// 1..=8
    pub amount: u8,
    pub falling: bool,
    pub source: bool,
}

impl Fluid {
    /// VANILLA: FlowingFluid#getOwnHeight = amount / 9
    pub fn own_height(&self) -> f32 {
        f32::from(self.amount) / 9.0
    }

    pub fn same(&self, other: Option<Fluid>) -> bool {
        other.is_some_and(|o| o.lava == self.lava)
    }
}

fn compute_fluid(state: BlockState) -> Option<Fluid> {
    let water_source = Fluid {
        lava: false,
        amount: 8,
        falling: false,
        source: true,
    };
    match state.name() {
        "water" | "lava" => {
            let level: u8 = state.property("level")?.parse().ok()?;
            let lava = state.name() == "lava";
            Some(match level {
                0 => Fluid {
                    lava,
                    ..water_source
                },
                1..=7 => Fluid {
                    lava,
                    amount: 8 - level,
                    falling: false,
                    source: false,
                },
                _ => Fluid {
                    lava,
                    amount: 8,
                    falling: true,
                    source: false,
                },
            })
        }
        "kelp" | "kelp_plant" | "seagrass" | "tall_seagrass" | "bubble_column" => {
            Some(water_source)
        }
        _ if state.property("waterlogged") == Some("true") => Some(water_source),
        _ => None,
    }
}

fn fluid_table() -> &'static [Option<Fluid>] {
    static T: OnceLock<Vec<Option<Fluid>>> = OnceLock::new();
    T.get_or_init(|| {
        let mut v = vec![None; STATE_COUNT as usize];
        for b in BLOCKS {
            let has_fluid = b
                .props
                .iter()
                .any(|p| p.name == "waterlogged" || p.name == "level")
                || matches!(
                    b.name,
                    "kelp" | "kelp_plant" | "seagrass" | "tall_seagrass" | "bubble_column"
                );
            if !has_fluid {
                continue;
            }
            for s in b.first_state..b.first_state + b.state_count {
                if let Some(st) = BlockState::from_id(s) {
                    v[s as usize] = compute_fluid(st);
                }
            }
        }
        v
    })
}

pub fn fluid(state: BlockState) -> Option<Fluid> {
    fluid_table().get(usize::from(state.0)).copied().flatten()
}

fn table() -> &'static [u8] {
    static T: OnceLock<Vec<u8>> = OnceLock::new();
    T.get_or_init(|| {
        let mut v = vec![0u8; STATE_COUNT as usize];
        for b in BLOCKS {
            for s in b.first_state..b.first_state + b.state_count {
                if let Some(st) = BlockState::from_id(s) {
                    v[s as usize] = compute(st);
                }
            }
        }
        v
    })
}

pub fn flags(state: BlockState) -> u8 {
    table().get(usize::from(state.0)).copied().unwrap_or(0)
}

pub fn has(state: BlockState, flag: u8) -> bool {
    flags(state) & flag != 0
}

/// `getShadeBrightness`.
pub fn shade_brightness(state: BlockState) -> f32 {
    if has(state, SHADE_DARK) {
        0.2
    } else {
        1.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn st(name: &str) -> BlockState {
        let b = BLOCKS.iter().find(|b| b.name == name).unwrap();
        BlockState::from_id(b.default_state).unwrap()
    }

    #[test]
    fn flags_of_common_blocks() {
        let stone = st("stone");
        assert!(has(
            stone,
            COLLISION_FULL | VIEW_BLOCKING | SOLID_RENDER | SHADE_DARK
        ));
        assert_eq!(shade_brightness(stone), 0.2);
        let glass = st("glass");
        assert!(has(glass, COLLISION_FULL));
        assert!(!has(glass, VIEW_BLOCKING | SOLID_RENDER | SHADE_DARK));
        assert_eq!(shade_brightness(glass), 1.0);
        assert!(has(st("magma_block"), EMISSIVE));
        assert!(!has(BlockState::AIR, COLLISION_FULL | VIEW_BLOCKING));
        assert_eq!(flags(st("oak_leaves")) & VIEW_BLOCKING, 0);
        assert!(has(st("soul_sand"), VIEW_BLOCKING | SHADE_DARK));
        let slab = st("stone_slab");
        assert!(!has(slab, COLLISION_FULL));
    }

    #[test]
    fn offsets_follow_vanilla_ranges() {
        assert_eq!(block_offset(st("stone"), 5, 7), [0.0; 3]);
        let poppy = st("poppy");
        let mut seen = std::collections::HashSet::new();
        for x in -20..20 {
            for z in -20..20 {
                let o = block_offset(poppy, x, z);
                assert!(o[0].abs() <= 0.25 && o[2].abs() <= 0.25 && o[1] == 0.0);
                seen.insert((o[0].to_bits(), o[2].to_bits()));
            }
        }
        assert!(seen.len() > 10);
        let grass = st("short_grass");
        let ys: Vec<f32> = (0..30).map(|x| block_offset(grass, x, x * 3)[1]).collect();
        assert!(ys.iter().all(|y| (-0.2..=0.0).contains(y)) && ys.iter().any(|y| *y < -0.01));
        let drip = st("pointed_dripstone");
        assert!((0..50).all(|x| block_offset(drip, x, 2 * x)[0].abs() <= 0.125));
        // детерминизм и значение Mth.getSeed(0,0,0) = 0 → (0/15-0.5)*0.5 = -0.25
        assert_eq!(block_offset(poppy, 0, 0), [-0.25, 0.0, -0.25]);
    }

    #[test]
    fn fluids() {
        let w = fluid(st("water")).unwrap();
        assert!(w.source && !w.lava && (w.own_height() - 0.888_888_9).abs() < 1e-6);
        let flowing = st("water").with("level", "3").unwrap();
        assert_eq!(fluid(flowing).unwrap().amount, 5);
        assert!(
            fluid(st("water").with("level", "9").unwrap())
                .unwrap()
                .falling
        );
        assert!(fluid(st("lava")).unwrap().lava);
        assert!(fluid(st("stone")).is_none());
        assert!(fluid(st("seagrass")).is_some());
        assert!(fluid(st("oak_stairs").with("waterlogged", "true").unwrap()).is_some());
        assert!(fluid(st("oak_stairs")).is_none());
        assert!(fluid(BlockState::AIR).is_none());
    }
}
