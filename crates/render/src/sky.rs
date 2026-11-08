//! Небо: временная шкала `day` (углы солнца/луны/звёзд, цвет неба, закат, звёзды),
//! погодные модификаторы и геометрия (звёзды, диски, веер заката).
//!
//! VANILLA: Timelines#bootstrap(DAY, MOON), KeyframeTrackSampler, EasingType.CubicBezier,
//! WeatherAttributes, SkyRenderer (1.21.11). Только CPU: GPU-рисование — в клиенте.
//! Поддержан Overworld-скайбокс; End/Nether (skybox END/NONE) — отдельные задачи.

use ezzmc_world::noise::LegacyRandom;
use glam::{Mat3, Vec3};

pub const DAY_PERIOD: i64 = 24000;
pub const MOON_PHASES: i64 = 8;

// ---------- ARGB-утилиты (net.minecraft.util.ARGB) ----------

const fn a(c: u32) -> i32 {
    (c >> 24) as i32
}
const fn r(c: u32) -> i32 {
    ((c >> 16) & 255) as i32
}
const fn g(c: u32) -> i32 {
    ((c >> 8) & 255) as i32
}
const fn b(c: u32) -> i32 {
    (c & 255) as i32
}
fn argb(a_: i32, r_: i32, g_: i32, b_: i32) -> u32 {
    ((a_ as u32 & 255) << 24)
        | ((r_ as u32 & 255) << 16)
        | ((g_ as u32 & 255) << 8)
        | (b_ as u32 & 255)
}

/// Mth.lerpInt: `a + floor(t * (b - a))`.
fn lerp_int(t: f32, x: i32, y: i32) -> i32 {
    x + (t * (y - x) as f32).floor() as i32
}

/// ARGB.srgbLerp
pub fn srgb_lerp(t: f32, c1: u32, c2: u32) -> u32 {
    argb(
        lerp_int(t, a(c1), a(c2)),
        lerp_int(t, r(c1), r(c2)),
        lerp_int(t, g(c1), g(c2)),
        lerp_int(t, b(c1), b(c2)),
    )
}

/// ARGB.multiply (покомпонентно, `-1` — нейтральный).
pub fn multiply(c1: u32, c2: u32) -> u32 {
    if c1 == u32::MAX {
        return c2;
    }
    if c2 == u32::MAX {
        return c1;
    }
    argb(
        a(c1) * a(c2) / 255,
        r(c1) * r(c2) / 255,
        g(c1) * g(c2) / 255,
        b(c1) * b(c2) / 255,
    )
}

/// ARGB.greyscale
fn greyscale(c: u32) -> u32 {
    let v = (r(c) as f32 * 0.3 + g(c) as f32 * 0.59 + b(c) as f32 * 0.11) as i32;
    argb(a(c), v, v, v)
}

/// ARGB.scaleRGB(int, float)
fn scale_rgb(c: u32, k: f32) -> u32 {
    let s = |v: i32| ((v as f32 * k) as i32).clamp(0, 255);
    argb(a(c), s(r(c)), s(g(c)), s(b(c)))
}

/// ColorModifier.BLEND_TO_GRAY
fn blend_to_gray(c: u32, brightness: f32, factor: f32) -> u32 {
    srgb_lerp(factor, c, scale_rgb(greyscale(c), brightness))
}

/// ARGB.colorFromFloat(1, r, g, b)
fn color_f(a_: f32, r_: f32, g_: f32, b_: f32) -> u32 {
    let q = |v: f32| (v * 255.0 + 0.5).floor() as i32;
    argb(q(a_), q(r_), q(g_), q(b_))
}

// ---------- Easing и сэмплер ключевых кадров ----------

/// EasingType.CubicBezier (4 итерации Ньютона–Рафсона).
#[derive(Clone, Copy)]
pub struct CubicBezier {
    xc: [f32; 3],
    yc: [f32; 3],
}

impl CubicBezier {
    fn curve(p1: f32, p2: f32) -> [f32; 3] {
        [3.0 * p1 - 3.0 * p2 + 1.0, -6.0 * p1 + 3.0 * p2, 3.0 * p1]
    }
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            xc: Self::curve(x1, x2),
            yc: Self::curve(y1, y2),
        }
    }
    /// `EasingType.symmetricCubicBezier`.
    pub fn symmetric(x: f32, y: f32) -> Self {
        Self::new(x, y, 1.0 - x, 1.0 - y)
    }
    fn sample(c: &[f32; 3], t: f32) -> f32 {
        ((c[0] * t + c[1]) * t + c[2]) * t
    }
    fn gradient(c: &[f32; 3], t: f32) -> f32 {
        (3.0 * c[0] * t + 2.0 * c[1]) * t + c[2]
    }
    pub fn apply(&self, x: f32) -> f32 {
        let mut t = x;
        for _ in 0..4 {
            let grad = Self::gradient(&self.xc, t);
            if grad < 1.0e-5 {
                break;
            }
            t -= (Self::sample(&self.xc, t) - x) / grad;
        }
        Self::sample(&self.yc, t)
    }
}

#[derive(Clone, Copy)]
pub enum Easing {
    Constant,
    Linear,
    Bezier(CubicBezier),
}

impl Easing {
    fn apply(self, t: f32) -> f32 {
        match self {
            Easing::Constant => 0.0,
            Easing::Linear => t,
            Easing::Bezier(c) => c.apply(t),
        }
    }
}

/// VANILLA: KeyframeTrackSampler#sample для периодической дорожки.
/// `keys` — кадры в порядке возрастания тиков (равные тики — как в билдере).
pub fn sample_track<T: Copy>(
    keys: &[(i64, T)],
    period: i64,
    easing: Easing,
    lerp: impl Fn(f32, T, T) -> T,
    time: i64,
) -> T {
    if keys.len() == 1 {
        return keys[0].1;
    }
    let first = keys[0];
    let last = keys[keys.len() - 1];
    let t = time.rem_euclid(period);
    // Сегменты: (from_t, from_v, to_t, to_v)
    let seg_ok = |ft: i64, fv: T, tt: i64, tv: T| -> Option<T> {
        if t < tt {
            Some(if t <= ft {
                fv
            } else {
                let f = (t - ft) as f32 / (tt - ft) as f32;
                lerp(easing.apply(f), fv, tv)
            })
        } else {
            None
        }
    };
    if let Some(v) = seg_ok(last.0 - period, last.1, first.0, first.1) {
        return v;
    }
    for w in keys.windows(2) {
        if let Some(v) = seg_ok(w[0].0, w[0].1, w[1].0, w[1].1) {
            return v;
        }
    }
    // Последний сегмент: last → first + period (на нём `t >= to` даёт toValue).
    let (ft, fv, tt, tv) = (last.0, last.1, first.0 + period, first.1);
    if t <= ft {
        fv
    } else if t >= tt {
        tv
    } else {
        lerp(easing.apply((t - ft) as f32 / (tt - ft) as f32), fv, tv)
    }
}

fn lerp_f(t: f32, x: f32, y: f32) -> f32 {
    x + t * (y - x)
}

// ---------- Данные дорожек Timelines.DAY ----------

const NIGHT_SKY: u32 = 0xFF00_0000;

/// SUNRISE_SUNSET_COLOR (тик, ARGB как знаковый int из Timelines → u32).
const SUNRISE: &[(i64, i32)] = &[
    (71, 1609540403),
    (310, 703969843),
    (565, 117167155),
    (730, 16770355),
    (11270, 16770355),
    (11397, 83679283),
    (11522, 268028723),
    (11690, 703969843),
    (11929, 1609540403),
    (12243, -1310226637),
    (12358, -857440717),
    (12512, -371166669),
    (12613, -153261261),
    (12732, -19242189),
    (12841, -19440589),
    (13035, -321760973),
    (13252, -1043577037),
    (13775, 918435635),
    (13888, 532362547),
    (14039, 163001139),
    (14192, 11744051),
    (21807, 11678515),
    (21961, 163001139),
    (22112, 532362547),
    (22225, 918435635),
    (22748, -1043577037),
    (22965, -321760973),
    (23159, -19440589),
    (23272, -19242189),
    (23488, -371166669),
    (23642, -857440717),
    (23757, -1310226637),
];

/// STAR_BRIGHTNESS (модификатор MAXIMUM над базой 0).
const STARS: &[(i64, f32)] = &[
    (92, 0.037),
    (627, 0.0),
    (11373, 0.0),
    (11732, 0.016),
    (11959, 0.044),
    (12399, 0.143),
    (12729, 0.258),
    (13228, 0.5),
    (22772, 0.5),
    (23032, 0.364),
    (23356, 0.225),
    (23758, 0.101),
];

/// Ванильная биомная база неба Overworld (plains), если биом неизвестен.
pub const DEFAULT_SKY_COLOR: u32 = 0xFF78_A7FF;

/// Результат: то, что `SkyRenderer#extractRenderState` кладёт в SkyRenderState (Overworld).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkyState {
    /// Радианы: Axis.XP вращение солнца/луны/звёзд.
    pub sun_angle: f32,
    pub moon_angle: f32,
    pub star_angle: f32,
    /// `1 - rainLevel`, альфа солнца/луны.
    pub rain_brightness: f32,
    pub star_brightness: f32,
    /// ARGB веера заката (альфа 0 → не рисовать).
    pub sunrise_color: u32,
    /// Индекс фазы луны (MoonPhase.index, 0 = полнолуние).
    pub moon_phase: u8,
    /// Цвет неба ARGB (непрозрачный).
    pub sky_color: u32,
    /// SKY_LIGHT_FACTOR (0.24 ночью .. 1 днём, с погодой) — множитель неба в lightmap.
    pub sky_light_factor: f32,
    /// SKY_LIGHT_COLOR RGB 0..1 (ночью `0.48,0.48,1`).
    pub sky_light_color: [f32; 3],
}

/// VANILLA: attributeProbe.getValue(.., partialTicks) — угол между прошлым и текущим тиком (кратчайшая дуга).
pub fn lerp_angles(prev: &SkyState, cur: &SkyState, partial: f32) -> SkyState {
    let l = |a: f32, b: f32| {
        let tau = std::f32::consts::TAU;
        let d = (b - a + std::f32::consts::PI).rem_euclid(tau) - std::f32::consts::PI;
        a + d * partial
    };
    let mut s = *cur;
    s.sun_angle = l(prev.sun_angle, cur.sun_angle);
    s.moon_angle = l(prev.moon_angle, cur.moon_angle);
    s.star_angle = l(prev.star_angle, cur.star_angle);
    s
}

/// Состояние неба Overworld.
/// `day_time` — время суток из Update Time, `rain`/`thunder` — уровни 0..1 (Level#getRainLevel),
/// `base_sky_color` — RGB биомного неба (с биом-смешиванием), `ARGB`.
pub fn sky_state(day_time: i64, rain: f32, thunder: f32, base_sky_color: u32) -> SkyState {
    let ease = Easing::Bezier(CubicBezier::symmetric(0.362, 0.241));
    let angle = |keys: &[(i64, f32)]| {
        sample_track(keys, DAY_PERIOD, ease, lerp_f, day_time) * (std::f32::consts::PI / 180.0)
    };
    let sun_angle = angle(&[(6000, 360.0), (6000, 0.0)]);
    let moon_angle = angle(&[(6000, 540.0), (6000, 180.0)]);
    let star_angle = angle(&[(6000, 360.0), (6000, 0.0)]);

    // SKY_COLOR: база × дорожка (MULTIPLY_RGB), затем погода (BLEND_TO_GRAY).
    let m = sample_track(
        &[
            (133, u32::MAX),
            (11867, u32::MAX),
            (13670, NIGHT_SKY),
            (22330, NIGHT_SKY),
        ],
        DAY_PERIOD,
        Easing::Linear,
        srgb_lerp,
        day_time,
    );
    let mut sky = multiply(base_sky_color | 0xFF00_0000, m);
    let rain_only = rain - thunder;
    if rain_only > 0.0 {
        let gray = blend_to_gray(sky, 0.6, 0.75);
        sky = srgb_lerp(rain_only, sky, gray);
    }
    if thunder > 0.0 {
        let gray = blend_to_gray(sky, 0.24, 0.94);
        sky = srgb_lerp(thunder, sky, gray);
    }

    // SUNRISE_SUNSET_COLOR: дорожка, затем MULTIPLY_ARGB погоды.
    let keys: Vec<(i64, u32)> = SUNRISE.iter().map(|&(t, c)| (t, c as u32)).collect();
    let mut sunrise = sample_track(&keys, DAY_PERIOD, Easing::Linear, srgb_lerp, day_time);
    if rain_only > 0.0 {
        let m = multiply(sunrise, color_f(1.0, 0.5, 0.5, 0.6));
        sunrise = srgb_lerp(rain_only, sunrise, m);
    }
    if thunder > 0.0 {
        let m = multiply(sunrise, color_f(1.0, 0.25, 0.25, 0.3));
        sunrise = srgb_lerp(thunder, sunrise, m);
    }

    // STAR_BRIGHTNESS: MAXIMUM(0, дорожка); дождь → set 0 (stateChangeLerp по уровню).
    let mut stars = sample_track(STARS, DAY_PERIOD, Easing::Linear, lerp_f, day_time).max(0.0);
    if rain_only > 0.0 {
        stars = lerp_f(rain_only, stars, 0.0);
    }
    if thunder > 0.0 {
        stars = lerp_f(thunder, stars, 0.0);
    }

    // MOON: период 8 суток, фаза = ключ каждые 24000 (интерполяция дискретная).
    let moon_phase = (day_time.rem_euclid(DAY_PERIOD * MOON_PHASES) / DAY_PERIOD) as u8;

    // VANILLA: Timelines.DAY — SKY_LIGHT_FACTOR (MULTIPLY) / SKY_LIGHT_COLOR (MULTIPLY_RGB), затем погода.
    const NIGHT_LIGHT_COLOR: u32 = 0xFF7A_7AFF; // colorFromFloat(1, .48, .48, 1)
    let mut lf = sample_track(
        &[(730, 1.0f32), (11270, 1.0), (13140, 0.24), (22860, 0.24)],
        DAY_PERIOD,
        Easing::Linear,
        lerp_f,
        day_time,
    );
    let mut lc = sample_track(
        &[
            (730, u32::MAX),
            (11270, u32::MAX),
            (13140, NIGHT_LIGHT_COLOR),
            (22860, NIGHT_LIGHT_COLOR),
        ],
        DAY_PERIOD,
        Easing::Linear,
        srgb_lerp,
        day_time,
    );
    // WeatherAttributes.RAIN/THUNDER: ALPHA_BLEND к (0.24, α) / к ночному цвету с α.
    for (level, alpha) in [(rain_only, 0.3125f32), (thunder, 0.527_343_75)] {
        if level > 0.0 {
            let f2 = lerp_f(alpha, lf, 0.24);
            lf = lerp_f(level, lf, f2);
            let c2 = srgb_lerp(alpha, lc, NIGHT_LIGHT_COLOR);
            lc = srgb_lerp(level, lc, c2);
        }
    }
    let sky_light_color = [
        ((lc >> 16) & 255) as f32 / 255.0,
        ((lc >> 8) & 255) as f32 / 255.0,
        (lc & 255) as f32 / 255.0,
    ];

    SkyState {
        sky_light_factor: lf,
        sky_light_color,
        sun_angle,
        moon_angle,
        star_angle,
        rain_brightness: 1.0 - rain,
        star_brightness: stars,
        sunrise_color: sunrise,
        moon_phase,
        sky_color: sky,
    }
}

/// VANILLA: CLOUD_COLOR (DimensionTypes overworld: white(0.8)) × ночная дорожка (MULTIPLY_ARGB,
/// Timelines.DAY), затем погода (BLEND_TO_GRAY_ARGB). Результат ARGB.
pub fn cloud_color(day_time: i64, rain: f32, thunder: f32) -> u32 {
    let night = color_f(1.0, 0.1, 0.1, 0.15);
    let m = sample_track(
        &[
            (133, u32::MAX),
            (11867, u32::MAX),
            (13670, night),
            (22330, night),
        ],
        DAY_PERIOD,
        Easing::Linear,
        srgb_lerp,
        day_time,
    );
    let mut c = multiply(color_f(0.8, 1.0, 1.0, 1.0), m);
    let rain_only = rain - thunder;
    if rain_only > 0.0 {
        c = srgb_lerp(rain_only, c, blend_to_gray(c, 0.24, 0.5));
    }
    if thunder > 0.0 {
        c = srgb_lerp(thunder, c, blend_to_gray(c, 0.095, 0.94));
    }
    c
}

/// Биомная база тумана Overworld (`dimension_type/overworld.json` → `#c0d8ff`).
pub const DEFAULT_FOG_COLOR: u32 = 0xFFC0_D8FF;

/// Mth.clampedLerp(t, a, b)
fn clamped_lerp(t: f32, x: f32, y: f32) -> f32 {
    if t < 0.0 {
        x
    } else if t > 1.0 {
        y
    } else {
        lerp_f(t, x, y)
    }
}

/// VANILLA: AtmosphericFogEnvironment#getBaseColor (+ FogRenderer#computeFogColor без
/// воды/лавы/слепоты). `forward_x` — компонента взгляда по X (Camera#forwardVector.x),
/// `view_chunks` — дальность прорисовки в чанках, `sky_fog_end` — SKY_FOG_END_DISTANCE (512).
#[allow(clippy::too_many_arguments)]
pub fn atmospheric_fog_color(
    day_time: i64,
    rain: f32,
    thunder: f32,
    base_fog: u32,
    base_sky: u32,
    view_chunks: i32,
    forward_x: f32,
    sky_fog_end: f32,
) -> [f32; 3] {
    let st = sky_state(day_time, rain, thunder, base_sky);
    // FOG_COLOR: база × ночная дорожка, затем погода (MULTIPLY_RGB).
    let m = sample_track(
        &[
            (133, u32::MAX),
            (11867, u32::MAX),
            (13670, color_f(1.0, 0.06, 0.06, 0.09)),
            (22330, color_f(1.0, 0.06, 0.06, 0.09)),
        ],
        DAY_PERIOD,
        Easing::Linear,
        srgb_lerp,
        day_time,
    );
    let mut fog = multiply(base_fog | 0xFF00_0000, m);
    let rain_only = rain - thunder;
    if rain_only > 0.0 {
        fog = srgb_lerp(rain_only, fog, multiply(fog, color_f(1.0, 0.5, 0.5, 0.6)));
    }
    if thunder > 0.0 {
        fog = srgb_lerp(thunder, fog, multiply(fog, color_f(1.0, 0.25, 0.25, 0.3)));
    }
    if view_chunks >= 4 {
        // Закат/восход подмешивается к туману со стороны солнца.
        let side = if st.sun_angle.sin() > 0.0 { -1.0 } else { 1.0 };
        let d = forward_x * side;
        if d > 0.0 {
            let alpha = a(st.sunrise_color) as f32 / 255.0;
            if alpha > 0.0 {
                fog = srgb_lerp(d * alpha, fog, st.sunrise_color | 0xFF00_0000);
            }
        }
    }
    // Небо с погодным затемнением.
    let mut sky = st.sky_color;
    if rain > 0.0 {
        let k1 = 1.0 - rain * 0.5;
        let k2 = 1.0 - rain * 0.4;
        let sc = |v: i32, k: f32| ((v as f32 * k) as i32).clamp(0, 255);
        sky = argb(a(sky), sc(r(sky), k1), sc(g(sky), k1), sc(b(sky), k2));
    }
    if thunder > 0.0 {
        sky = scale_rgb(sky, 1.0 - thunder * 0.5);
    }
    let t = (sky_fog_end / 16.0).min(view_chunks as f32);
    let t = clamped_lerp(t / 32.0, 0.25, 1.0);
    let t = 1.0 - t.powf(0.25);
    let c = srgb_lerp(t, fog, sky);
    [
        r(c) as f32 / 255.0,
        g(c) as f32 / 255.0,
        b(c) as f32 / 255.0,
    ]
}

/// Цвет тумана-множителя ночью (используется в M1.5/150).
pub fn night_fog_multiplier() -> u32 {
    color_f(1.0, 0.06, 0.06, 0.09)
}

// ---------- Геометрия ----------

/// Звёзды: `SkyRenderer#buildStars` (RandomSource.create(10842)), 4 вершины на звезду.
pub fn build_stars() -> Vec<[f32; 3]> {
    let mut rnd = LegacyRandom::new(10842);
    let next_float = |rnd: &mut LegacyRandom| rnd.next(24) as f32 * 5.960_464_5e-8;
    let mut out = Vec::with_capacity(1500 * 4);
    for _ in 0..1500 {
        let x = next_float(&mut rnd) * 2.0 - 1.0;
        let y = next_float(&mut rnd) * 2.0 - 1.0;
        let z = next_float(&mut rnd) * 2.0 - 1.0;
        let size = 0.15 + next_float(&mut rnd) * 0.1;
        let len_sq = x * x + y * y + z * z;
        if len_sq <= 0.010_000_001 || len_sq >= 1.0 {
            continue;
        }
        let pos = Vec3::new(x, y, z).normalize() * 100.0;
        let rot = (rnd.next_double() as f32) * std::f32::consts::PI * 2.0;
        // JOML rotateTowards(-pos, up): столбцы [left, up', dir].
        let dir = (-pos).normalize();
        let left = Vec3::Y.cross(dir).normalize();
        let up = dir.cross(left);
        let m = Mat3::from_cols(left, up, dir) * Mat3::from_rotation_z(-rot);
        for (vx, vy) in [(size, -size), (size, size), (-size, size), (-size, -size)] {
            let p = m * Vec3::new(vx, vy, 0.0) + pos;
            out.push(p.to_array());
        }
    }
    out
}

/// Диск неба (TRIANGLE_FAN, 10 вершин): центр `(0, y, 0)` + 9 точек по кругу R=512.
pub fn build_sky_disc(y: f32) -> Vec<[f32; 3]> {
    let radius = y.signum() * 512.0;
    let mut v = vec![[0.0, y, 0.0]];
    let mut deg = -180;
    while deg <= 180 {
        let rad = (deg as f32) * (std::f32::consts::PI / 180.0);
        v.push([radius * rad.cos(), y, 512.0 * rad.sin()]);
        deg += 45;
    }
    v
}

/// Веер заката (TRIANGLE_FAN, 18 вершин): `(x, y, z, ARGB)`.
pub fn build_sunrise_fan() -> Vec<([f32; 3], u32)> {
    let mut v = vec![([0.0, 100.0, 0.0], 0xFFFF_FFFF)];
    for i in 0..=16 {
        let ang = i as f32 * (std::f32::consts::PI * 2.0) / 16.0;
        let (s, c) = (ang.sin(), ang.cos());
        v.push(([s * 120.0, c * 120.0, -c * 40.0], 0x00FF_FFFF));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deg(rad: f32) -> f32 {
        rad.to_degrees()
    }

    #[test]
    fn sun_angle_noon_and_midnight() {
        let noon = sky_state(6000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert!(deg(noon.sun_angle).abs() < 1e-3);
        let mid = sky_state(18000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert!((deg(mid.sun_angle) - 180.0).abs() < 0.1);
        // Луна напротив солнца.
        assert!((deg(mid.moon_angle) - 360.0).abs() < 0.1 || deg(mid.moon_angle).abs() < 0.1);
        // t=0 (рассвет): доля 0.75 по bezier-easing; кривая центрально-симметрична:
        // ease(0.25) + ease(0.75) = 1 → углы в сумме 360°.
        let dawn = sky_state(0, 0.0, 0.0, DEFAULT_SKY_COLOR);
        let dusk = sky_state(12000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert!((deg(dawn.sun_angle) + deg(dusk.sun_angle) - 360.0).abs() < 0.1);
        assert!(deg(dawn.sun_angle) > 270.0 && deg(dawn.sun_angle) < 300.0);
    }

    #[test]
    fn sky_dark_at_night_bright_at_day() {
        let day = sky_state(6000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert_eq!(day.sky_color, DEFAULT_SKY_COLOR);
        let night = sky_state(18000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert_eq!(night.sky_color & 0xFF_FFFF, 0);
        assert!((night.star_brightness - 0.5).abs() < 1e-6);
        assert_eq!(day.star_brightness, 0.0);
        // Дождь убирает звёзды и серит небо.
        let rainy = sky_state(18000, 1.0, 0.0, DEFAULT_SKY_COLOR);
        assert_eq!(rainy.star_brightness, 0.0);
        assert_eq!(rainy.rain_brightness, 0.0);
    }

    #[test]
    fn rain_grays_day_sky() {
        let day = sky_state(6000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        let rain = sky_state(6000, 1.0, 0.0, DEFAULT_SKY_COLOR);
        assert_ne!(day.sky_color, rain.sky_color);
        // Серый 0.6 яркости темнее каждого канала (120,167,255 → ~97): все каналы падают,
        // синий сильнее всего.
        assert!(r(rain.sky_color) < r(day.sky_color));
        assert!(g(rain.sky_color) < g(day.sky_color));
        assert!(b(rain.sky_color) < b(day.sky_color));
        assert!(b(day.sky_color) - b(rain.sky_color) > r(day.sky_color) - r(rain.sky_color));
    }

    #[test]
    fn sunrise_only_at_dusk_dawn() {
        let noon = sky_state(6000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert_eq!(a(noon.sunrise_color), 0);
        let dusk = sky_state(12841, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert!(a(dusk.sunrise_color) > 200, "{:x}", dusk.sunrise_color);
    }

    #[test]
    fn moon_phase_cycle() {
        assert_eq!(sky_state(0, 0.0, 0.0, DEFAULT_SKY_COLOR).moon_phase, 0);
        assert_eq!(
            sky_state(24000 * 3 + 5, 0.0, 0.0, DEFAULT_SKY_COLOR).moon_phase,
            3
        );
        assert_eq!(
            sky_state(24000 * 8, 0.0, 0.0, DEFAULT_SKY_COLOR).moon_phase,
            0
        );
    }

    #[test]
    fn geometry_counts() {
        let stars = build_stars();
        assert!(stars.len() % 4 == 0 && stars.len() > 4 * 300);
        for q in stars.chunks(4) {
            let c = (Vec3::from(q[0]) + Vec3::from(q[2])) * 0.5;
            assert!((c.length() - 100.0).abs() < 0.5);
        }
        assert_eq!(build_sky_disc(16.0).len(), 10);
        assert_eq!(build_sunrise_fan().len(), 18);
    }

    #[test]
    fn sky_light_factor_day_night() {
        let d = sky_state(6000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert_eq!(d.sky_light_factor, 1.0);
        assert_eq!(d.sky_light_color, [1.0; 3]);
        let n = sky_state(18000, 0.0, 0.0, DEFAULT_SKY_COLOR);
        assert!((n.sky_light_factor - 0.24).abs() < 1e-6);
        assert!(n.sky_light_color[0] < 0.5);
        let r = sky_state(6000, 1.0, 0.0, DEFAULT_SKY_COLOR);
        assert!((r.sky_light_factor - (1.0 + 0.3125 * (0.24 - 1.0))).abs() < 1e-5);
    }
}

#[cfg(test)]
mod cloud_tests {
    use super::*;
    #[test]
    fn cloud_color_day_night() {
        let d = cloud_color(6000, 0.0, 0.0);
        assert_eq!(d >> 24, 204);
        assert_eq!(d & 0xFFFFFF, 0xFFFFFF);
        let n = cloud_color(18000, 0.0, 0.0);
        assert!(((n >> 16) & 255) < 40);
    }
}
