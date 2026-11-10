//! Туман. VANILLA: FogRenderer#setupFog/computeFogColor + fog/environment/* (26.3).
//! Свободная камера без сущности: эффекты Blindness/Darkness/NightVision и water vision не
//! применяются (VANILLA-UNSURE: подключить, когда появится игрок).

use crate::sky::atmospheric_fog_color;
use crate::uniforms::FogUniform;

/// VANILLA: FogType (без NONE — он сводится к Atmospheric).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FogType {
    Atmospheric,
    Water,
    Lava,
    PowderSnow,
}

/// Атрибуты среды (дефолты EnvironmentAttributes; Overworld не задаёт свои start/end).
#[derive(Clone, Copy, Debug)]
pub struct FogAttrs {
    pub fog_start: f32,
    pub fog_end: f32,
    pub sky_fog_end: f32,
    pub cloud_fog_end: f32,
    pub water_fog_start: f32,
    pub water_fog_end: f32,
    /// Базовый цвет тумана биома (0xRRGGBB) и воды.
    pub base_fog: u32,
    pub base_sky: u32,
    pub water_fog_color: u32,
}

impl Default for FogAttrs {
    fn default() -> Self {
        FogAttrs {
            fog_start: 0.0,
            fog_end: 1024.0,
            sky_fog_end: 512.0,
            cloud_fog_end: 2048.0,
            water_fog_start: -8.0,
            water_fog_end: 96.0,
            base_fog: 0xC0D8FF,
            base_sky: crate::sky::DEFAULT_SKY_COLOR,
            water_fog_color: 0x05_05_33,
        }
    }
}

pub struct FogInput {
    pub fog_type: FogType,
    pub attrs: FogAttrs,
    pub day_time: i64,
    pub rain: f32,
    pub thunder: f32,
    pub view_chunks: i32,
    /// Camera#forwardVector.x
    pub forward_x: f32,
    pub spectator: bool,
    /// Y камеры и minY мира (затемнение у дна, `voidDarknessOnsetRange` по умолчанию 32).
    pub cam_y: f64,
    pub min_y: i32,
    pub void_darkness_onset: f32,
    /// LocalPlayer#getWaterVision (0 вне воды).
    pub water_vision: f32,
    /// Options#cloudRange (чанки).
    pub cloud_range: i32,
    /// Есть ли у измерения дневной цикл (таймлайн `day`, skybox overworld). Нет — Nether/End:
    /// дневная дорожка и закатный туман не применяются, цвет = чистый FOG_COLOR биома.
    pub day_cycle: bool,
}

/// Плавный множитель дождевого тумана (AtmosphericFogEnvironment#rainFogMultiplier).
#[derive(Default, Debug)]
pub struct FogState {
    pub rain_fog_multiplier: f32,
    /// LocalPlayer#waterVisionTime (0..600 тиков).
    pub water_vision_time: f32,
}

impl FogState {
    /// VANILLA: LocalPlayer#aiStep — рост (+1, у наблюдателя +10) под водой, спад −10 вне воды.
    pub fn tick_water_vision(&mut self, eye_in_water: bool, delta_ticks: f32, spectator: bool) {
        let d = if eye_in_water {
            delta_ticks * if spectator { 10.0 } else { 1.0 }
        } else {
            -10.0 * delta_ticks
        };
        self.water_vision_time = (self.water_vision_time + d).clamp(0.0, 600.0);
    }

    /// VANILLA: LocalPlayer#getWaterVision
    pub fn water_vision(&self, eye_in_water: bool) -> f32 {
        if !eye_in_water {
            return 0.0;
        }
        let t = self.water_vision_time;
        if t >= 600.0 {
            return 1.0;
        }
        let a = (t / 100.0).clamp(0.0, 1.0);
        let b = if t < 100.0 {
            0.0
        } else {
            ((t - 100.0) / 500.0).clamp(0.0, 1.0)
        };
        a * 0.6 + b * 0.399_999_98
    }

    /// `sky_light` — уровень небесного света в блоке камеры, `rains_in_biome` — hasPrecipitation.
    /// `delta_ticks` — тиков с прошлого кадра (DeltaTracker#getGameTimeDeltaTicks).
    pub fn update_rain(
        &mut self,
        rain: f32,
        sky_light: u8,
        rains_in_biome: bool,
        delta_ticks: f32,
    ) {
        let sky = ((f32::from(sky_light) - 8.0) / 7.0).clamp(0.0, 1.0);
        let target = rain * sky * if rains_in_biome { 1.0 } else { 0.5 };
        self.rain_fog_multiplier += (target - self.rain_fog_multiplier) * delta_ticks * 0.2;
    }
}

fn rgb(c: u32) -> [f32; 3] {
    [
        ((c >> 16) & 255) as f32 / 255.0,
        ((c >> 8) & 255) as f32 / 255.0,
        (c & 255) as f32 / 255.0,
    ]
}

/// VANILLA: FogRenderer#setupFog + computeFogColor.
pub fn setup_fog(i: &FogInput, st: &FogState) -> FogUniform {
    let rd = i.view_chunks as f32 * 16.0;
    let a = &i.attrs;
    let (base, start, end, sky_end, cloud_end) = match i.fog_type {
        FogType::Lava => {
            // Режим наблюдателя: −8..rd/2; иначе (без Fire Resistance) 0.25..1.
            let (s, e) = if i.spectator {
                (-8.0, rd * 0.5)
            } else {
                (0.25, 1.0)
            };
            (rgb(0x99_19_00), s, e, e, e)
        }
        FogType::PowderSnow => {
            let (s, e) = if i.spectator {
                (-8.0, rd * 0.5)
            } else {
                (0.0, 2.0)
            };
            (rgb(0x9F_BB_CC), s, e, e, e)
        }
        FogType::Water => {
            // VANILLA: WaterFogEnvironment — конец × max(0.25, getWaterVision).
            let e = a.water_fog_end * i.water_vision.max(0.25);
            (rgb(a.water_fog_color), a.water_fog_start, e, e, e)
        }
        FogType::Atmospheric => {
            // VANILLA: без дневного таймлайна (in_nether) SKY_FOG_END=512 даёт фактор 0 → чистый FOG_COLOR.
            let c = if i.day_cycle {
                atmospheric_fog_color(
                    i.day_time,
                    i.rain,
                    i.thunder,
                    a.base_fog,
                    a.base_sky,
                    i.view_chunks,
                    i.forward_x,
                    a.sky_fog_end,
                )
            } else {
                rgb(a.base_fog)
            };
            let m = st.rain_fog_multiplier;
            let s = a.fog_start - 160.0 * m;
            let e = (a.fog_end - 256.0 * m).max(96.0_f32.min(a.fog_end));
            let sky_end = rd.min(a.sky_fog_end);
            // VANILLA: cloudEnd = min(cloudRange*16, CLOUD_FOG_END_DISTANCE)
            (
                c,
                s,
                e,
                sky_end,
                (i.cloud_range as f32 * 16.0).min(a.cloud_fog_end),
            )
        }
    };
    // Затемнение у дна мира (не в лаве/снегу).
    let mut col = base;
    let onset = i.void_darkness_onset;
    let darkness = ((onset + i.min_y as f32 - i.cam_y as f32) / onset).clamp(0.0, 1.0);
    if darkness > 0.0 && !matches!(i.fog_type, FogType::Lava | FogType::PowderSnow) {
        let k = (1.0 - darkness) * (1.0 - darkness);
        col = [col[0] * k, col[1] * k, col[2] * k];
    }
    // VANILLA: FogRenderer#computeFogColor — в воде яркость растёт с getWaterVision (нормировка по max).
    if matches!(i.fog_type, FogType::Water) && col[0] != 0.0 && col[1] != 0.0 && col[2] != 0.0 {
        let k = 1.0 / col[0].max(col[1]).max(col[2]);
        let t = i.water_vision;
        col = col.map(|v| v + (v * k - v) * t);
    }
    let span = (rd / 10.0).clamp(4.0, 64.0);
    FogUniform {
        color: [col[0], col[1], col[2], 1.0],
        environmental_start: start,
        environmental_end: end,
        render_distance_start: rd - span,
        render_distance_end: rd,
        sky_end,
        clouds_end: cloud_end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(t: FogType) -> FogInput {
        FogInput {
            fog_type: t,
            attrs: FogAttrs::default(),
            day_time: 6000,
            rain: 0.0,
            thunder: 0.0,
            view_chunks: 12,
            forward_x: 0.0,
            spectator: false,
            cam_y: 70.0,
            min_y: -64,
            void_darkness_onset: 32.0,
            water_vision: 1.0,
            cloud_range: 64,
            day_cycle: true,
        }
    }

    #[test]
    fn atmospheric_distances() {
        let f = setup_fog(&input(FogType::Atmospheric), &FogState::default());
        assert_eq!(f.environmental_start, 0.0);
        assert_eq!(f.environmental_end, 1024.0);
        assert_eq!(f.render_distance_end, 192.0);
        assert_eq!(f.render_distance_start, 192.0 - 19.2);
        assert_eq!(f.sky_end, 192.0);
    }

    #[test]
    fn rain_pulls_fog_in() {
        let st = FogState {
            rain_fog_multiplier: 1.0,
            ..FogState::default()
        };
        let f = setup_fog(&input(FogType::Atmospheric), &st);
        assert_eq!(f.environmental_start, -160.0);
        assert_eq!(f.environmental_end, 768.0);
    }

    #[test]
    fn lava_and_water() {
        let l = setup_fog(&input(FogType::Lava), &FogState::default());
        assert_eq!((l.environmental_start, l.environmental_end), (0.25, 1.0));
        assert!((l.color[0] - 0x99 as f32 / 255.0).abs() < 1e-6);
        let w = setup_fog(&input(FogType::Water), &FogState::default());
        assert_eq!((w.environmental_start, w.environmental_end), (-8.0, 96.0));
    }

    #[test]
    fn void_darkening() {
        let mut i = input(FogType::Atmospheric);
        i.cam_y = f64::from(i.min_y) - 10.0;
        let dark = setup_fog(&i, &FogState::default());
        let day = setup_fog(&input(FogType::Atmospheric), &FogState::default());
        assert!(dark.color[2] < day.color[2] * 0.01);
    }

    #[test]
    fn nether_fog_is_biome_color_without_day_cycle() {
        let mut i = input(FogType::Atmospheric);
        i.day_cycle = false;
        i.day_time = 18000;
        i.attrs.fog_start = 10.0;
        i.attrs.fog_end = 96.0;
        i.attrs.base_fog = 0x33_08_08;
        let f = setup_fog(&i, &FogState::default());
        assert_eq!(f.environmental_start, 10.0);
        assert_eq!(f.environmental_end, 96.0);
        let c = [
            0x33 as f32 / 255.0,
            0x08 as f32 / 255.0,
            0x08 as f32 / 255.0,
        ];
        for (got, want) in f.color[..3].iter().zip(c) {
            assert!((got - want).abs() < 1e-5, "{got} != {want}");
        }
    }
}
