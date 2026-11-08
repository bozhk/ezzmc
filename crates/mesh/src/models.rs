//! Модели блоков для мешера (TODO-137): blockstate → baked quads по всем состояниям, слои, тинты.
//! VANILLA: BlockStateModelSet / BlockModelDefinition#instantiate / BlockColors#createDefault (1.21.11).
use crate::fluid::FluidSprites;
use crate::layers::{layer_of_name, RenderLayer};
use crate::section::{ModelPart, ModelProvider, Neighborhood, SectionPos};
use ezzmc_assets::atlas::Identifier;
use ezzmc_assets::bake::{bake_elements, ModelState, QuadCollection, SpriteGetter, SpriteUv};
use ezzmc_assets::blockstate::{BlockModelDefinition, PropertyDef, StateModel, StateView};
use ezzmc_assets::model::ModelLoader;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_world::biome_color::{
    blend_tint, BiomeColors, ColorMaps, FOLIAGE_BIRCH, FOLIAGE_EVERGREEN,
};
use ezzmc_world::block::{BlockInfo, BlockState};
use ezzmc_world::generated::b26_3::{BLOCKS, STATE_COUNT};
use ezzmc_world::noise::LegacyRandom;
use std::collections::HashMap;
use std::sync::Arc;

/// Запечённая модель: квады + флаг AO модели.
#[derive(Debug, Default)]
pub struct Baked {
    pub quads: QuadCollection,
    pub ao: bool,
    /// Спрайт слота `particle` модели (VANILLA: getParticleMaterial).
    pub particle: Option<SpriteUv>,
}

/// Взвешенный набор вариантов одной «части».
#[derive(Debug)]
struct Weighted {
    entries: Vec<(Arc<Baked>, u32)>,
    total: u32,
}

impl Weighted {
    // VANILLA: WeightedList#getRandomOrThrow — random.nextInt(totalWeight), единственный вариант random не тратит
    fn pick(&self, rnd: &mut LegacyRandom) -> &Arc<Baked> {
        if self.entries.len() == 1 {
            return &self.entries[0].0;
        }
        let mut r = rnd.next_int_bound(self.total.max(1) as i32) as u32;
        for (b, w) in &self.entries {
            if r < *w {
                return b;
            }
            r -= *w;
        }
        &self.entries[0].0
    }
}

/// Источник тинта состояния.
// VANILLA: BlockColors#createDefault
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TintKind {
    None,
    Grass,
    /// `double_block_half=upper` берёт цвет с блока ниже.
    GrassDouble {
        upper: bool,
    },
    /// pink_petals / wildflowers: tint_index 0 — без цвета.
    GrassPetals,
    Foliage,
    DryFoliage,
    Water,
    Fixed(u32),
    Redstone(u8),
    Stem(u8),
    LilyPad,
}

struct StateEntry {
    parts: Vec<Weighted>,
    layer: RenderLayer,
    tint: TintKind,
}

/// Данные биомных цветов (`ColorMaps` из `colormap/*.png` + цвета биомов из реестра).
#[derive(Default, Clone)]
pub struct Tints {
    pub maps: ColorMaps,
    /// По id биома реестра.
    pub biomes: Vec<Option<BiomeColors>>,
    /// «Biome Blend» 0..7 (по умолчанию 2).
    pub blend_radius: i32,
}

impl Tints {
    fn biome_at(
        &self,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
    ) -> BiomeColors {
        let id = hood.biome(base, x, y, z).unwrap_or(0) as usize;
        self.biomes
            .get(id)
            .and_then(Clone::clone)
            .unwrap_or_else(|| BiomeColors {
                temperature: 0.8,
                downfall: 0.4,
                water_color: 4_159_204,
                ..Default::default()
            })
    }

    // VANILLA: ClientLevel#calculateBlockTint
    fn blended(
        &self,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
        f: impl Fn(&BiomeColors, i32, i32) -> u32,
    ) -> u32 {
        blend_tint(x, z, self.blend_radius, |bx, bz| {
            f(&self.biome_at(hood, base, bx, y, bz), bx, bz)
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn color(
        &self,
        kind: TintKind,
        idx: i32,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
    ) -> u32 {
        let grass = |y: i32| {
            self.blended(hood, base, x, y, z, |b, bx, bz| {
                b.grass_color(&self.maps, f64::from(bx), f64::from(bz))
            })
        };
        match kind {
            TintKind::None => 0xFF_FFFF,
            TintKind::Grass => grass(y),
            TintKind::GrassDouble { upper } => grass(if upper { y - 1 } else { y }),
            TintKind::GrassPetals if idx != 0 => grass(y),
            TintKind::GrassPetals => 0xFF_FFFF,
            TintKind::Foliage => {
                self.blended(hood, base, x, y, z, |b, _, _| b.foliage_color(&self.maps))
            }
            TintKind::DryFoliage => self.blended(hood, base, x, y, z, |b, _, _| {
                b.dry_foliage_color(&self.maps)
            }),
            TintKind::Water => self.blended(hood, base, x, y, z, |b, _, _| b.water_color()),
            TintKind::Fixed(c) => c,
            TintKind::Redstone(p) => redstone_color(p),
            TintKind::Stem(age) => {
                let a = u32::from(age);
                ((a * 32) << 16) | ((255 - a * 8) << 8) | (a * 4)
            }
            TintKind::LilyPad => 0x20_8030, // -14647248
        }
    }
}

// VANILLA: RedStoneWireBlock#COLORS
pub fn redstone_color(power: u8) -> u32 {
    let f = f32::from(power.min(15)) / 15.0;
    let r = f * 0.6 + if f > 0.0 { 0.4 } else { 0.3 };
    let g = (f * f * 0.7 - 0.5).clamp(0.0, 1.0);
    let b = (f * f * 0.6 - 0.7).clamp(0.0, 1.0);
    let c = |v: f32| (v * 255.0) as u32;
    (c(r) << 16) | (c(g) << 8) | c(b)
}

/// Все модели блоков 1.21.11.
pub struct BlockModels {
    entries: Vec<Option<StateEntry>>,
    pub tints: Tints,
    cutout_leaves: bool,
    fluid_sprites: Option<FluidSprites>,
}

struct Props(HashMap<String, String>);
impl StateView for Props {
    fn get(&self, prop: &str) -> Option<&str> {
        self.0.get(prop).map(String::as_str)
    }
}

fn tint_kind(name: &str, st: BlockState) -> TintKind {
    let prop = |n: &str| st.property(n);
    match name {
        "tall_grass" | "large_fern" => TintKind::GrassDouble {
            upper: prop("half") == Some("upper"),
        },
        "grass_block" | "fern" | "short_grass" | "potted_fern" | "bush" | "sugar_cane" => {
            TintKind::Grass
        }
        "pink_petals" | "wildflowers" => TintKind::GrassPetals,
        "spruce_leaves" => TintKind::Fixed(FOLIAGE_EVERGREEN & 0xFF_FFFF),
        "birch_leaves" => TintKind::Fixed(FOLIAGE_BIRCH & 0xFF_FFFF),
        "oak_leaves" | "jungle_leaves" | "acacia_leaves" | "dark_oak_leaves" | "vine"
        | "mangrove_leaves" => TintKind::Foliage,
        "leaf_litter" => TintKind::DryFoliage,
        "water" | "bubble_column" | "water_cauldron" => TintKind::Water,
        "redstone_wire" => {
            TintKind::Redstone(prop("power").and_then(|v| v.parse().ok()).unwrap_or(0))
        }
        "attached_melon_stem" | "attached_pumpkin_stem" => TintKind::Fixed(0xE0_C71C), // -2046180
        "melon_stem" | "pumpkin_stem" => {
            TintKind::Stem(prop("age").and_then(|v| v.parse().ok()).unwrap_or(0))
        }
        "lily_pad" => TintKind::LilyPad,
        _ => TintKind::None,
    }
}

impl BlockModels {
    /// Загрузить и запечь все модели. Состояния без blockstate-файла (вода, воздух…) остаются без квадов.
    pub fn build(
        rm: &ResourceManager,
        sprites: &dyn SpriteGetter,
        cutout_leaves: bool,
    ) -> BlockModels {
        let mut loader = ModelLoader::new(rm);
        #[allow(clippy::type_complexity)]
        let mut baked_cache: HashMap<
            (Identifier, usize, usize, usize, bool),
            Option<Arc<Baked>>,
        > = HashMap::new();
        let mut entries: Vec<Option<StateEntry>> = (0..STATE_COUNT).map(|_| None).collect();
        for info in BLOCKS {
            let Some(data) = rm.read(&format!("assets/minecraft/blockstates/{}.json", info.name))
            else {
                continue;
            };
            let def = match BlockModelDefinition::parse(&data) {
                Ok(d) => d,
                Err(e) => {
                    tracing::warn!("blockstate {}: {e}", info.name);
                    continue;
                }
            };
            let defs = prop_defs(info);
            let layer = layer_of_name(info.name, cutout_leaves);
            for id in info.first_state..info.first_state + info.state_count {
                let Some(st) = BlockState::from_id(id) else {
                    continue;
                };
                let view = Props(
                    info.props
                        .iter()
                        .filter_map(|p| {
                            st.property(p.name)
                                .map(|v| (p.name.to_owned(), v.to_owned()))
                        })
                        .collect(),
                );
                let Some(sm) = def.model_for(&defs, &view, info.name) else {
                    continue;
                };
                let sets: Vec<&ezzmc_assets::blockstate::WeightedVariants> = match sm {
                    StateModel::Single(w) => vec![w],
                    StateModel::MultiPart(v) => v,
                };
                let mut parts = Vec::with_capacity(sets.len());
                for w in sets {
                    let mut entries_w = Vec::new();
                    for (variant, weight) in &w.entries {
                        let key = (
                            variant.model.clone(),
                            variant.x.shift(),
                            variant.y.shift(),
                            variant.z.shift(),
                            variant.uv_lock,
                        );
                        let b = baked_cache
                            .entry(key)
                            .or_insert_with(|| {
                                let (resolved, _) = loader.resolve(&variant.model)?;
                                let slots = resolved.texture_slots();
                                let ms = ModelState {
                                    x: variant.x,
                                    y: variant.y,
                                    z: variant.z,
                                    uv_lock: variant.uv_lock,
                                };
                                let particle = slots
                                    .get("particle")
                                    .map(|id| sprites.get(id).unwrap_or_else(|| sprites.missing()));
                                Some(Arc::new(Baked {
                                    particle,
                                    quads: bake_elements(
                                        resolved.elements(),
                                        &slots,
                                        sprites,
                                        &ms,
                                        &variant.model.to_string(),
                                    ),
                                    ao: resolved.ambient_occlusion(),
                                }))
                            })
                            .clone();
                        if let Some(b) = b {
                            entries_w.push((b, *weight));
                        }
                    }
                    if !entries_w.is_empty() {
                        let total = entries_w.iter().map(|e| e.1).sum();
                        parts.push(Weighted {
                            entries: entries_w,
                            total,
                        });
                    }
                }
                // VANILLA 26.x: слой по прозрачности пикселей спрайтов квадов; таблица имён 1.21.11 — запас (max)
                let mut state_layer = layer;
                if state_layer != RenderLayer::Translucent
                    && (cutout_leaves || !info.name.ends_with("_leaves"))
                {
                    'scan: for w in &parts {
                        for (b, _) in &w.entries {
                            let mut all: Vec<&ezzmc_assets::bake::BakedQuad> =
                                b.quads.unculled.iter().collect();
                            for c in &b.quads.culled {
                                all.extend(c.iter());
                            }
                            for q in all {
                                let us = q.uv.map(|p| p[0]);
                                let vs = q.uv.map(|p| p[1]);
                                let r = |a: [f32; 4]| {
                                    [
                                        a.iter().copied().fold(f32::MAX, f32::min),
                                        a.iter().copied().fold(f32::MIN, f32::max),
                                    ]
                                };
                                match sprites.region_transparency(r(us), r(vs)) {
                                    2 => {
                                        state_layer = RenderLayer::Translucent;
                                        break 'scan;
                                    }
                                    1 => state_layer = state_layer.max(RenderLayer::Cutout),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                entries[id as usize] = Some(StateEntry {
                    parts,
                    layer: state_layer,
                    tint: tint_kind(info.name, st),
                });
            }
        }
        BlockModels {
            entries,
            tints: Tints {
                blend_radius: 2,
                ..Default::default()
            },
            cutout_leaves,
            fluid_sprites: load_fluid_sprites(sprites),
        }
    }

    /// VANILLA: BlockStateModelSet#getParticleMaterial — слот `particle` первой части модели.
    pub fn particle_sprite(&self, s: BlockState) -> Option<SpriteUv> {
        let e = self.entry(s)?;
        e.parts.first()?.entries.first()?.0.particle
    }

    /// VANILLA: BlockTintSource#colorAsTerrainParticle (grass_block → белый, остальное как в мире).
    pub fn particle_tint(
        &self,
        s: BlockState,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
    ) -> u32 {
        if s.name() == "grass_block" {
            return 0xFF_FFFF;
        }
        let kind = self.entry(s).map_or(TintKind::None, |e| e.tint);
        self.tints.color(kind, 0, hood, base, x, y, z)
    }

    fn entry(&self, s: BlockState) -> Option<&StateEntry> {
        self.entries.get(s.id() as usize)?.as_ref()
    }
}

// VANILLA: ModelBakery#WATER_STILL / WATER_FLOW / WATER_OVERLAY / LAVA_STILL / LAVA_FLOW
fn load_fluid_sprites(sprites: &dyn SpriteGetter) -> Option<FluidSprites> {
    let g = |n: &str| sprites.get(&Identifier::new("minecraft", &format!("block/{n}")));
    Some(FluidSprites {
        water_still: g("water_still")?,
        water_flow: g("water_flow")?,
        water_overlay: g("water_overlay")?,
        lava_still: g("lava_still")?,
        lava_flow: g("lava_flow")?,
    })
}

fn prop_defs(info: &BlockInfo) -> Vec<PropertyDef> {
    info.props
        .iter()
        .map(|p| PropertyDef {
            name: p.name.to_owned(),
            values: p.values.iter().map(|v| (*v).to_owned()).collect(),
        })
        .collect()
}

// VANILLA: Mth#getSeed(int,int,int)
pub fn pos_seed(x: i32, y: i32, z: i32) -> i64 {
    let mut l = i64::from(x.wrapping_mul(3_129_871))
        ^ i64::from(z).wrapping_mul(116_129_781)
        ^ i64::from(y);
    l = l
        .wrapping_mul(l)
        .wrapping_mul(42_317_861)
        .wrapping_add(l.wrapping_mul(11));
    l >> 16
}

impl ModelProvider for BlockModels {
    fn parts<'a>(
        &'a self,
        state: BlockState,
        x: i32,
        y: i32,
        z: i32,
        out: &mut Vec<ModelPart<'a>>,
    ) {
        let Some(e) = self.entry(state) else { return };
        // VANILLA: random.setSeed(state.getSeed(pos)); model.collectParts(random)
        let mut rnd = LegacyRandom::new(pos_seed(x, y, z));
        for w in &e.parts {
            let b = w.pick(&mut rnd);
            out.push(ModelPart {
                quads: &b.quads,
                ao: b.ao,
            });
        }
    }

    fn layer(&self, state: BlockState) -> RenderLayer {
        self.entry(state).map_or(RenderLayer::Solid, |e| e.layer)
    }

    fn tint(
        &self,
        state: BlockState,
        idx: i32,
        hood: &Neighborhood,
        base: SectionPos,
        x: i32,
        y: i32,
        z: i32,
    ) -> [u8; 3] {
        let kind = self.entry(state).map_or(TintKind::None, |e| e.tint);
        let c = self.tints.color(kind, idx, hood, base, x, y, z);
        [(c >> 16) as u8, (c >> 8) as u8, c as u8]
    }

    fn cutout_leaves(&self) -> bool {
        self.cutout_leaves
    }

    fn fluid_sprites(&self) -> Option<&FluidSprites> {
        self.fluid_sprites.as_ref()
    }

    fn water_tint(&self, hood: &Neighborhood, base: SectionPos, x: i32, y: i32, z: i32) -> [u8; 3] {
        let c = self.tints.color(TintKind::Water, 0, hood, base, x, y, z);
        [(c >> 16) as u8, (c >> 8) as u8, c as u8]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn seed_and_redstone() {
        // Mth.getSeed(0,0,0) = 0
        assert_eq!(pos_seed(0, 0, 0), 0);
        assert_eq!(redstone_color(0), 0x4C_0000);
        // f=1: r=1.0, g=0.7−0.5 (f32 → 0.19999999 → 50), b=0
        assert_eq!(redstone_color(15), 0xFF_3200);
    }

    #[test]
    fn real_models_if_game_present() {
        use ezzmc_assets::block_atlas::BlockAtlas;
        use ezzmc_assets::install::GameInstall;
        use ezzmc_world::generated::b26_3::BLOCKS;
        let Ok(inst) = GameInstall::locate("26.3") else {
            return;
        };
        let rm = inst.resource_manager().unwrap();
        let atlas = BlockAtlas::build(&rm, 4).unwrap();
        let m = BlockModels::build(&rm, &atlas, true);
        let stone = BlockState::from_id(
            BLOCKS
                .iter()
                .find(|b| b.name == "stone")
                .unwrap()
                .default_state,
        )
        .unwrap();
        let mut out = Vec::new();
        m.parts(stone, 0, 0, 0, &mut out);
        assert_eq!(out.len(), 1);
        assert!(out[0].ao);
        for d in ezzmc_assets::model::Direction::ALL {
            assert_eq!(out[0].quads.get_quads(Some(d)).len(), 1);
        }
        let grass = BlockState::from_id(
            BLOCKS
                .iter()
                .find(|b| b.name == "grass_block")
                .unwrap()
                .default_state,
        )
        .unwrap();
        assert_eq!(m.layer(grass), RenderLayer::Cutout);
        if let Some(sp) = ezzmc_world::block::BlockState::from_id(
            BLOCKS
                .iter()
                .find(|b| b.name == "sulfur_spike")
                .unwrap()
                .first_state,
        ) {
            assert_eq!(m.layer(sp), RenderLayer::Cutout);
        }
        out.clear();
        m.parts(grass, 3, 64, 5, &mut out);
        assert!(!out.is_empty());
        let filled = m.entries.iter().flatten().count();
        assert!(filled > 20000);
    }
    /// TODO-146: нестандартные модели на реальных ассетах 1.21.11.
    #[test]
    fn nonstandard_blocks_have_geometry() {
        use ezzmc_assets::block_atlas::BlockAtlas;
        use ezzmc_assets::install::GameInstall;
        use ezzmc_world::generated::b26_3::BLOCKS;
        let Ok(inst) = GameInstall::locate("26.3") else {
            return;
        };
        let rm = inst.resource_manager().unwrap();
        let atlas = BlockAtlas::build(&rm, 4).unwrap();
        let m = BlockModels::build(&rm, &atlas, true);
        let total = |name: &str| -> usize {
            let b = BLOCKS.iter().find(|b| b.name == name).unwrap();
            let st = BlockState::from_id(b.default_state).unwrap();
            let mut out = Vec::new();
            m.parts(st, 1, 70, 2, &mut out);
            out.iter()
                .map(|p| {
                    let mut n = p.quads.get_quads(None).len();
                    for d in ezzmc_assets::model::Direction::ALL {
                        n += p.quads.get_quads(Some(d)).len();
                    }
                    n
                })
                .sum()
        };
        // cross = 2 плоскости × 2 стороны = 4 квада.
        assert_eq!(total("dandelion"), 4);
        assert_eq!(total("short_grass"), 4);
        for name in [
            "oak_stairs",
            "oak_fence",
            "oak_door",
            "glass_pane",
            "oak_slab",
            "fire",
            "vine",
            "white_carpet",
            "snow",
            "lily_pad",
            "oak_trapdoor",
            "ladder",
            "torch",
            "rail",
            "oak_fence_gate",
            "cobblestone_wall",
        ] {
            assert!(total(name) > 0, "{name}: нет геометрии");
        }
        // Плита (bottom) = 6 граней, лестница > 6.
        assert_eq!(total("oak_slab"), 6);
        assert!(total("oak_stairs") > 6);
        assert_eq!(total("white_carpet"), 6);
    }

    /// Отладочный отчёт по всем состояниям: нет квадов / missingno. `cargo test -p ezzmc-mesh audit_all_states -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn audit_all_states() {
        use ezzmc_assets::block_atlas::BlockAtlas;
        use ezzmc_assets::install::GameInstall;
        use ezzmc_world::generated::b26_3::BLOCKS;
        let Ok(inst) = GameInstall::locate("26.3") else {
            return;
        };
        let rm = inst.resource_manager().unwrap();
        let atlas = BlockAtlas::build(&rm, 4).unwrap();
        let m = BlockModels::build(&rm, &atlas, true);
        let mut report = String::new();
        for b in BLOCKS {
            let (mut empty, mut missing, mut ok) = (0, 0, 0);
            let mut ex_empty = None;
            let mut ex_missing = None;
            for id in b.first_state..b.first_state + b.state_count {
                let Some(st) = BlockState::from_id(id) else {
                    continue;
                };
                let mut out = Vec::new();
                m.parts(st, 1, 70, 2, &mut out);
                let mut n = 0;
                let mut miss = false;
                for p in &out {
                    let mut qs: Vec<_> = p.quads.get_quads(None).iter().collect();
                    for d in ezzmc_assets::model::Direction::ALL {
                        qs.extend(p.quads.get_quads(Some(d)).iter());
                    }
                    n += qs.len();
                    miss |= qs.iter().any(|q| q.sprite.path == "missingno");
                }
                if n == 0 {
                    empty += 1;
                    ex_empty.get_or_insert(id);
                } else if miss {
                    missing += 1;
                    ex_missing.get_or_insert(id);
                } else {
                    ok += 1;
                }
            }
            if empty > 0 || missing > 0 {
                report += &format!(
                    "{}\tstates={}\tok={ok}\tempty={empty}\tmissingno={missing}\tex_empty={ex_empty:?}\tex_missing={ex_missing:?}\n",
                    b.name, b.state_count
                );
            }
        }
        std::fs::write(
            std::env::var("AUDIT_OUT").unwrap_or("/tmp/audit.tsv".into()),
            report,
        )
        .unwrap();
    }
}
