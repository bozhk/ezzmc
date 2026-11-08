//! Цвета биомов: colormap `grass/foliage/dry_foliage`, вода, небо/туман (порт 1.21.11).
use crate::noise::{LegacyRandom, PerlinSimplexNoise};
use ezzmc_protocol::nbt::Tag;
use std::sync::OnceLock;

// VANILLA: GrassColor / FoliageColor / DryFoliageColor
const GRASS_DEFAULT: u32 = (-65281i32) as u32;
const FOLIAGE_DEFAULT: u32 = (-12_012_264i32) as u32;
const DRY_FOLIAGE_DEFAULT: u32 = (-10_732_494i32) as u32;

/// Три colormap-текстуры 256×256 (ARGB). Пустой массив → цвет по умолчанию.
#[derive(Debug, Clone, Default)]
pub struct ColorMaps {
    pub grass: Vec<u32>,
    pub foliage: Vec<u32>,
    pub dry_foliage: Vec<u32>,
}

// VANILLA: ColorMapColorUtil#get
fn colormap_get(temp: f64, down: f64, pixels: &[u32], default: u32) -> u32 {
    let down = down * temp;
    let x = ((1.0 - temp) * 255.0) as i32;
    let y = ((1.0 - down) * 255.0) as i32;
    let idx = ((y << 8) | x) as usize;
    pixels.get(idx).copied().unwrap_or(default)
}

impl ColorMaps {
    pub fn grass(&self, temp: f64, down: f64) -> u32 {
        colormap_get(temp, down, &self.grass, GRASS_DEFAULT)
    }
    pub fn foliage(&self, temp: f64, down: f64) -> u32 {
        colormap_get(temp, down, &self.foliage, FOLIAGE_DEFAULT)
    }
    pub fn dry_foliage(&self, temp: f64, down: f64) -> u32 {
        colormap_get(temp, down, &self.dry_foliage, DRY_FOLIAGE_DEFAULT)
    }
}

// VANILLA: Biome#BIOME_INFO_NOISE
fn biome_info_noise() -> &'static PerlinSimplexNoise {
    static N: OnceLock<PerlinSimplexNoise> = OnceLock::new();
    N.get_or_init(|| PerlinSimplexNoise::new(&mut LegacyRandom::new(2345), &[0]))
}

/// VANILLA: BiomeSpecialEffects.GrassColorModifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GrassColorModifier {
    #[default]
    None,
    DarkForest,
    Swamp,
}

impl GrassColorModifier {
    pub fn by_name(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "dark_forest" => Some(Self::DarkForest),
            "swamp" => Some(Self::Swamp),
            _ => None,
        }
    }

    pub fn modify(self, x: f64, z: f64, color: u32) -> u32 {
        match self {
            Self::None => color,
            Self::DarkForest => ((color & 0x00FE_FEFE) + 2_634_762) >> 1,
            Self::Swamp => {
                let v = biome_info_noise().get_value(x * 0.0225, z * 0.0225, false);
                if v < -0.1 {
                    5_011_004
                } else {
                    6_975_545
                }
            }
        }
    }
}

/// Всё, что клиенту нужно от биома для цвета (из сетевого реестра `worldgen/biome`).
// VANILLA: Biome + BiomeSpecialEffects + visual/* attributes
#[derive(Debug, Clone, Default)]
pub struct BiomeColors {
    pub temperature: f32,
    pub downfall: f32,
    /// VANILLA: Biome.ClimateSettings#hasPrecipitation.
    pub has_precipitation: bool,
    /// VANILLA: TemperatureModifier.FROZEN.
    pub frozen_modifier: bool,
    pub water_color: u32,
    pub foliage_override: Option<u32>,
    pub dry_foliage_override: Option<u32>,
    pub grass_override: Option<u32>,
    pub grass_modifier: GrassColorModifier,
    pub sky_color: Option<u32>,
    pub fog_color: Option<u32>,
    pub water_fog_color: Option<u32>,
}

/// `#rrggbb` или Int (как `ExtraCodecs.STRING_RGB_COLOR`).
fn color_of(t: &Tag) -> Option<u32> {
    match t {
        Tag::Int(i) => Some(*i as u32),
        Tag::String(s) => u32::from_str_radix(s.strip_prefix('#')?, 16).ok(),
        _ => None,
    }
}

impl BiomeColors {
    /// Разбор NBT записи реестра биомов; `None`, если нет обязательных полей.
    pub fn from_nbt(t: &Tag) -> Option<BiomeColors> {
        let fx = t.get("effects")?;
        let attrs = t.get("attributes");
        let attr = |name: &str| attrs.and_then(|a| a.get(name)).and_then(color_of);
        let num = |k: &str| match t.get(k)? {
            Tag::Float(f) => Some(*f),
            Tag::Double(d) => Some(*d as f32),
            _ => None,
        };
        // VANILLA: Biome.ClimateSettings.CODEC (temperature, downfall обязательны)
        Some(BiomeColors {
            temperature: num("temperature")?,
            downfall: num("downfall")?,
            has_precipitation: t
                .get("has_precipitation")
                .and_then(Tag::as_i64)
                .is_some_and(|v| v != 0),
            frozen_modifier: matches!(
                t.get("temperature_modifier").and_then(Tag::as_str),
                Some("frozen" | "minecraft:frozen")
            ),
            water_color: color_of(fx.get("water_color")?)?,
            foliage_override: fx.get("foliage_color").and_then(color_of),
            dry_foliage_override: fx.get("dry_foliage_color").and_then(color_of),
            grass_override: fx.get("grass_color").and_then(color_of),
            grass_modifier: fx
                .get("grass_color_modifier")
                .and_then(Tag::as_str)
                .and_then(|s| GrassColorModifier::by_name(s.trim_start_matches("minecraft:")))
                .unwrap_or_default(),
            sky_color: attr("minecraft:visual/sky_color"),
            fog_color: attr("minecraft:visual/fog_color"),
            water_fog_color: attr("minecraft:visual/water_fog_color"),
        })
    }

    fn clamped(&self) -> (f64, f64) {
        (
            f64::from(self.temperature).clamp(0.0, 1.0),
            f64::from(self.downfall).clamp(0.0, 1.0),
        )
    }

    // VANILLA: Biome#getGrassColor
    pub fn grass_color(&self, maps: &ColorMaps, x: f64, z: f64) -> u32 {
        let base = self.grass_override.unwrap_or_else(|| {
            let (t, d) = self.clamped();
            maps.grass(t, d)
        });
        self.grass_modifier.modify(x, z, base)
    }

    // VANILLA: Biome#getFoliageColor
    pub fn foliage_color(&self, maps: &ColorMaps) -> u32 {
        self.foliage_override.unwrap_or_else(|| {
            let (t, d) = self.clamped();
            maps.foliage(t, d)
        })
    }

    // VANILLA: Biome#getDryFoliageColor
    pub fn dry_foliage_color(&self, maps: &ColorMaps) -> u32 {
        self.dry_foliage_override.unwrap_or_else(|| {
            let (t, d) = self.clamped();
            maps.dry_foliage(t, d)
        })
    }

    // VANILLA: Biome#getWaterColor
    pub fn water_color(&self) -> u32 {
        self.water_color
    }
}

/// Цвета хвойных/берёзы/мангров, фиксированные в ванили.
// VANILLA: FoliageColor#FOLIAGE_*
pub const FOLIAGE_EVERGREEN: u32 = (-10_380_959i32) as u32;
pub const FOLIAGE_BIRCH: u32 = (-8_345_771i32) as u32;
pub const FOLIAGE_MANGROVE: u32 = (-7_158_200i32) as u32;

/// Усреднение цвета в кубе `(2r+1)²` по XZ (`biomeBlendRadius`).
// VANILLA: ClientLevel#calculateBlockTint
pub fn blend_tint<F: FnMut(i32, i32) -> u32>(x: i32, z: i32, radius: i32, mut color_at: F) -> u32 {
    if radius == 0 {
        return color_at(x, z) & 0x00FF_FFFF;
    }
    let n = (radius * 2 + 1) * (radius * 2 + 1);
    let (mut r, mut g, mut b) = (0i32, 0i32, 0i32);
    for dx in -radius..=radius {
        for dz in -radius..=radius {
            let c = color_at(x + dx, z + dz);
            r += ((c >> 16) & 0xFF) as i32;
            g += ((c >> 8) & 0xFF) as i32;
            b += (c & 0xFF) as i32;
        }
    }
    (((r / n) as u32) << 16) | (((g / n) as u32) << 8) | (b / n) as u32
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn colormap_lookup() {
        let mut px = vec![0u32; 65536];
        px[0] = 0xFF11_2233; // temp=1, down=1 → x=0,y=0
        px[255] = 0xFF44_5566; // temp=0 → x=255, y=255*... down*temp=0 → y=255 → idx 65280+255
        px[65280 + 255] = 0xFF77_8899;
        let m = ColorMaps {
            grass: px,
            ..Default::default()
        };
        assert_eq!(m.grass(1.0, 1.0), 0xFF11_2233);
        assert_eq!(m.grass(0.0, 0.5), 0xFF77_8899);
        assert_eq!(m.foliage(0.5, 0.5), FOLIAGE_DEFAULT); // пустая карта
        assert_eq!(FOLIAGE_DEFAULT, 0xFF48_B518);
    }

    #[test]
    fn modifiers_and_nbt() {
        assert_eq!(
            GrassColorModifier::DarkForest.modify(0.0, 0.0, 0xFF00_FF00),
            (0x00_FE00 + 2_634_762) >> 1
        );
        let s = GrassColorModifier::Swamp.modify(10.0, 20.0, 0);
        assert!(s == 5_011_004 || s == 6_975_545);
        let t = Tag::Compound(vec![
            ("temperature".into(), Tag::Float(0.8)),
            ("downfall".into(), Tag::Float(0.4)),
            (
                "effects".into(),
                Tag::Compound(vec![
                    ("water_color".into(), Tag::String("#617b64".into())),
                    ("grass_color_modifier".into(), Tag::String("swamp".into())),
                ]),
            ),
            (
                "attributes".into(),
                Tag::Compound(vec![(
                    "minecraft:visual/sky_color".into(),
                    Tag::String("#78a7ff".into()),
                )]),
            ),
        ]);
        let b = BiomeColors::from_nbt(&t).unwrap();
        assert_eq!(b.water_color(), 0x61_7B64);
        assert_eq!(b.grass_modifier, GrassColorModifier::Swamp);
        assert_eq!(b.sky_color, Some(0x78_A7FF));
        assert_eq!(blend_tint(0, 0, 1, |_, _| 0xFF80_4020), 0x80_4020);
    }
}
