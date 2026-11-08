//! Осадки в точке. VANILLA: Biome#getPrecipitationAt / getHeightAdjustedTemperature (26.3).
use crate::biome_color::BiomeColors;
use crate::noise::{LegacyRandom, SimplexNoise};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precipitation {
    None,
    Rain,
    Snow,
}

// VANILLA: SimplexNoise(WorldgenRandom(LegacyRandomSource(seed)), discardNoiseOffset=true) — смещения = 0.
fn temperature_noise() -> &'static SimplexNoise {
    static N: OnceLock<SimplexNoise> = OnceLock::new();
    N.get_or_init(|| SimplexNoise::new(&mut LegacyRandom::new(1234)))
}

fn biome_info_noise() -> &'static SimplexNoise {
    static N: OnceLock<SimplexNoise> = OnceLock::new();
    N.get_or_init(|| SimplexNoise::new(&mut LegacyRandom::new(2345)))
}

/// NoiseStack: три слоя (частота, амплитуда) на одном генераторе случайных.
fn frozen_noise() -> &'static [SimplexNoise; 3] {
    static N: OnceLock<[SimplexNoise; 3]> = OnceLock::new();
    N.get_or_init(|| {
        let mut r = LegacyRandom::new(3456);
        [
            SimplexNoise::new(&mut r),
            SimplexNoise::new(&mut r),
            SimplexNoise::new(&mut r),
        ]
    })
}

fn frozen_temperature(x: i32, z: i32) -> f32 {
    const LAYERS: [(f64, f32); 3] = [(1.0, 0.142_857_15), (0.5, 0.285_714_3), (0.25, 0.571_428_6)];
    let (fx, fz) = (f64::from(x) * 0.05, f64::from(z) * 0.05);
    let mut v = 0.0f32;
    for (n, (freq, amp)) in frozen_noise().iter().zip(LAYERS) {
        v += amp * n.get_value_2d(fx * freq, fz * freq) as f32;
    }
    let large = f64::from(v * 7.0);
    let edge = biome_info_noise().get_value_2d(f64::from(x) * 0.2, f64::from(z) * 0.2) as f32;
    if large + f64::from(edge) < 0.3 {
        let small =
            biome_info_noise().get_value_2d(f64::from(x) * 0.09, f64::from(z) * 0.09) as f32;
        if small < 0.8 {
            return 0.2;
        }
    }
    f32::NAN // «без изменений»
}

fn height_adjusted(b: &BiomeColors, x: i32, y: i32, z: i32, sea_level: i32) -> f32 {
    let mut t = b.temperature;
    if b.frozen_modifier {
        let m = frozen_temperature(x, z);
        if !m.is_nan() {
            t = m;
        }
    }
    let snow_level = sea_level + 17;
    if y > snow_level {
        let v = temperature_noise()
            .get_value_2d(f64::from(x as f32 / 8.0), f64::from(z as f32 / 8.0))
            as f32
            * 8.0;
        t - (v + (y - snow_level) as f32) * 0.05 / 40.0
    } else {
        t
    }
}

/// VANILLA: Biome#getPrecipitationAt(pos, seaLevel).
pub fn precipitation_at(b: &BiomeColors, x: i32, y: i32, z: i32, sea_level: i32) -> Precipitation {
    if !b.has_precipitation {
        Precipitation::None
    } else if height_adjusted(b, x, y, z, sea_level) >= 0.15 {
        Precipitation::Rain
    } else {
        Precipitation::Snow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn biome(t: f32, p: bool) -> BiomeColors {
        BiomeColors {
            temperature: t,
            has_precipitation: p,
            ..Default::default()
        }
    }

    #[test]
    fn basic() {
        assert_eq!(
            precipitation_at(&biome(0.8, true), 0, 64, 0, 63),
            Precipitation::Rain
        );
        assert_eq!(
            precipitation_at(&biome(0.0, true), 0, 64, 0, 63),
            Precipitation::Snow
        );
        assert_eq!(
            precipitation_at(&biome(2.0, false), 0, 64, 0, 63),
            Precipitation::None
        );
    }

    #[test]
    fn high_altitude_snows() {
        assert_eq!(
            precipitation_at(&biome(0.3, true), 0, 63 + 17, 0, 63),
            Precipitation::Rain
        );
        assert_eq!(
            precipitation_at(&biome(0.3, true), 0, 63 + 17 + 400, 0, 63),
            Precipitation::Snow
        );
    }
}
