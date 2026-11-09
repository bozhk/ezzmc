//! Динамические свойства предметов: часы (`minecraft:time`) и компасы (`minecraft:compass`).
//! VANILLA: NeedleDirectionHelper (Wobbler), Time, CompassAngleState (26.3).
use std::sync::Mutex;

#[derive(Clone, Copy, Default)]
struct Wobbler {
    rotation: f32,
    delta: f32,
}

impl Wobbler {
    // VANILLA: NeedleDirectionHelper#standardWobbler#update
    fn update(&mut self, target: f32, factor: f32) {
        let t = pmod(target - self.rotation + 0.5, 1.0) - 0.5;
        self.delta += t * 0.1;
        self.delta *= factor;
        self.rotation = pmod(self.rotation + self.delta, 1.0);
    }
}

fn pmod(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

#[derive(Default)]
struct State {
    dimension: String,
    pos: [f64; 3],
    yaw: f32,
    day_time: i64,
    game_time: i64,
    last_tick: Option<i64>,
    spawn: Option<(String, i32, i32)>,
    death: Option<(String, i32, i32)>,
    rng: u64,
    clock_day: Wobbler,
    clock_rand: Wobbler,
    spawn_w: Wobbler,
    lode_w: Wobbler,
    death_w: Wobbler,
    nothing_w: Wobbler,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> Option<R> {
    let mut g = STATE.lock().ok()?;
    Some(f(g.get_or_insert_with(|| State {
        rng: 0x2545_F491_4F6C_DD1D,
        ..State::default()
    })))
}

fn next_f32(s: &mut State) -> f32 {
    s.rng ^= s.rng << 13;
    s.rng ^= s.rng >> 7;
    s.rng ^= s.rng << 17;
    (s.rng >> 40) as f32 / (1u64 << 24) as f32
}

pub fn set_dimension(d: &str) {
    with(|s| s.dimension = d.to_owned());
}
pub fn set_spawn(dim: &str, x: i32, z: i32) {
    with(|s| s.spawn = Some((dim.to_owned(), x, z)));
}
pub fn set_death(d: Option<(String, i32, i32)>) {
    with(|s| s.death = d);
}

/// Раз в кадр: состояние игрока/мира; Wobbler-ы шагают раз в игровой тик.
pub fn update(pos: [f64; 3], yaw: f32, day_time: i64, game_time: i64) {
    with(|s| {
        s.pos = pos;
        s.yaw = yaw;
        s.day_time = day_time;
        s.game_time = game_time;
        if s.last_tick == Some(game_time) {
            return;
        }
        s.last_tick = Some(game_time);
        let overworld = s.dimension == "minecraft:overworld";
        let target = if overworld {
            sun_frac(day_time)
        } else {
            next_f32(s)
        };
        // VANILLA: Time → wobbler(0.9)
        if overworld {
            s.clock_day.update(target, 0.9);
        } else {
            s.clock_rand.update(target, 0.9);
        }
        let r = next_f32(s);
        s.nothing_w.update(r, 0.8);
        let t = 0.5 - (pmod(yaw / 360.0, 1.0) - 0.25);
        s.spawn_w.update(t, 0.8);
        s.lode_w.update(t, 0.8);
        s.death_w.update(t, 0.8);
    });
}

fn sun_frac(day_time: i64) -> f32 {
    // VANILLA: EnvironmentAttributes.SUN_ANGLE / 360
    ezzmc_render::sky::sky_state(day_time, 0.0, 0.0, 0).sun_angle / std::f32::consts::TAU
}

/// Индекс кадра часов 0..=64 (`range_dispatch` scale 64).
pub fn clock_entry() -> u8 {
    with(|s| {
        let v = if s.dimension == "minecraft:overworld" {
            s.clock_day.rotation
        } else {
            s.clock_rand.rotation
        };
        (v * 64.0).round() as u8
    })
    .unwrap_or(0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Spawn,
    Recovery,
    /// Блок лодестоуна (x, z) в текущем измерении.
    Lodestone(i32, i32),
    /// Лодестоун без цели/в другом измерении — вращение.
    None,
}

/// Индекс кадра компаса 0..=32 (`range_dispatch` scale 32); `None` цель → вращение.
// VANILLA: CompassAngleState#calculate
pub fn compass_entry(t: Target) -> u8 {
    with(|s| {
        let (x, z, w) = match t {
            Target::None => return spin(s),
            Target::Spawn => match &s.spawn {
                Some((d, x, z)) if *d == s.dimension => (*x, *z, s.spawn_w),
                _ => return spin(s),
            },
            Target::Recovery => match &s.death {
                Some((d, x, z)) if *d == s.dimension => (*x, *z, s.death_w),
                _ => return spin(s),
            },
            Target::Lodestone(x, z) => (x, z, s.lode_w),
        };
        let (tx, tz) = (f64::from(x) + 0.5, f64::from(z) + 0.5);
        if (tx - s.pos[0]).powi(2) + (tz - s.pos[2]).powi(2) < 1e-5 {
            return spin(s);
        }
        let ang = ((tz - s.pos[2]).atan2(tx - s.pos[0]) / std::f64::consts::TAU) as f32;
        (pmod(ang + w.rotation, 1.0) * 32.0).round() as u8
    })
    .unwrap_or(0)
}

fn spin(s: &State) -> u8 {
    (pmod(s.nothing_w.rotation, 1.0) * 32.0).round() as u8
}

pub fn dimension() -> String {
    with(|s| s.dimension.clone()).unwrap_or_default()
}
