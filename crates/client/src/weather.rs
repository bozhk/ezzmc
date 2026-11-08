//! Сбор колонок дождя/снега. VANILLA: WeatherEffectRenderer#extractRenderState,
//! createRainColumnInstance / createSnowColumnInstance (26.3).
use ezzmc_render::weatherdraw::ColumnInstance;
use ezzmc_world::biome_color::BiomeColors;
use ezzmc_world::chunk_packet::HeightmapType;
use ezzmc_world::level::Level;
use ezzmc_world::noise::LegacyRandom;
use ezzmc_world::precipitation::{precipitation_at, Precipitation};

/// VANILLA: Options#weatherRadius по умолчанию.
pub const WEATHER_RADIUS: i32 = 10;
/// VANILLA-UNSURE: Level#getSeaLevel берётся из измерения; для overworld 63.
const SEA_LEVEL: i32 = 63;

#[derive(Default)]
pub struct Columns {
    pub rain: Vec<ColumnInstance>,
    pub snow: Vec<ColumnInstance>,
}

/// `partial` = 0 (клиентский partial_tick не ведётся — vanilla-diff).
pub fn collect(
    level: &Level,
    biomes: &[Option<BiomeColors>],
    cam: [f64; 3],
    radius: i32,
    partial: f32,
) -> Columns {
    let mut out = Columns::default();
    if level.rain_level <= 0.0 {
        return out;
    }
    let (cx, cy, cz) = (
        cam[0].floor() as i32,
        cam[1].floor() as i32,
        cam[2].floor() as i32,
    );
    let ticks = level.game_time;
    let wrapped = (ticks & 131_071) as i32;
    let mut random = LegacyRandom::new(0);
    for z in cz - radius..=cz + radius {
        for x in cx - radius..=cx + radius {
            let Some(chunk) = level.chunks.get(x >> 4, z >> 4) else {
                continue;
            };
            let (lx, lz) = ((x & 15) as usize, (z & 15) as usize);
            let Some(terrain) = chunk.height(HeightmapType::MotionBlocking, lx, lz) else {
                continue;
            };
            let y0 = (cy - radius).max(terrain);
            let y1 = (cy + radius).max(terrain);
            if y1 - y0 == 0 {
                continue;
            }
            let biome = chunk.chunk.biome(lx, cy, lz) as usize;
            let Some(Some(b)) = biomes.get(biome) else {
                continue;
            };
            let p = precipitation_at(b, x, cy, z, SEA_LEVEL);
            if p == Precipitation::None {
                continue;
            }
            // VANILLA: seed = x*x*3121 + x*45238971 ^ z*z*418711 + z*13761 (int-арифметика).
            let seed = x
                .wrapping_mul(x)
                .wrapping_mul(3121)
                .wrapping_add(x.wrapping_mul(45_238_971))
                ^ z.wrapping_mul(z)
                    .wrapping_mul(418_711)
                    .wrapping_add(z.wrapping_mul(13_761));
            random.set_seed(i64::from(seed));
            let ly = cy.max(terrain);
            let block = chunk.block_light(lx, ly, lz).unwrap_or(0);
            let sky = chunk.sky_light(lx, ly, lz).unwrap_or(15);
            match p {
                Precipitation::Rain => {
                    let tick_offset = (x
                        .wrapping_mul(x)
                        .wrapping_mul(3121)
                        .wrapping_add(x.wrapping_mul(45_238_971))
                        .wrapping_add(z.wrapping_mul(z).wrapping_mul(418_711))
                        .wrapping_add(z.wrapping_mul(13_761)))
                        & 0xFF;
                    let speed = 3.0 + random.next_float();
                    let off = -((wrapped + tick_offset) as f32 + partial) / 32.0 * speed;
                    out.rain.push(ColumnInstance {
                        x,
                        z,
                        bottom_y: y0,
                        top_y: y1,
                        u_offset: 0.0,
                        v_offset: off % 32.0,
                        light: (block, sky),
                    });
                }
                Precipitation::Snow => {
                    let time = wrapped as f32 + partial;
                    let u = (random.next_double()
                        + f64::from(time * 0.01_f32 * random.next_gaussian() as f32))
                        as f32;
                    let v = (random.next_double()
                        + f64::from(time * random.next_gaussian() as f32 * 0.001_f32))
                        as f32;
                    let v_off = -((ticks & 511) as f32 + partial) / 512.0;
                    out.snow.push(ColumnInstance {
                        x,
                        z,
                        bottom_y: y0,
                        top_y: y1,
                        u_offset: u,
                        v_offset: v_off + v,
                        light: ((block * 3 + 15) / 4, (sky * 3 + 15) / 4),
                    });
                }
                Precipitation::None => {}
            }
        }
    }
    out
}
