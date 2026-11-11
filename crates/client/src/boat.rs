//! Управляемая лодка (TODO-206): клиентская симуляция, когда игрок — первый пассажир.
//!
//! VANILLA: AbstractBoat#tick/getStatus/checkInWater/isUnderwater/getWaterLevelAbove/getGroundFriction/
//! floatBoat/controlBoat/setInput/positionRider (26.3), Entity#move (без ступенек), Mth#sin/cos (таблица).
//! Не портировано: bubble column, столкновения с сущностями, поводки, паутина/слизь, лава.
use crate::physics::{
    block_friction_of, block_speed_factor, collide_with, expand_towards, gather, overlaps, Collider,
};

const SIN_SCALE: f64 = 10_430.378_350_470_453;

fn sin_table() -> &'static [f32] {
    static T: std::sync::OnceLock<Vec<f32>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        (0..65536)
            .map(|i| (f64::from(i) / SIN_SCALE).sin() as f32)
            .collect()
    })
}

/// VANILLA: Mth#sin — табличный синус (для Grim нужен тот же результат, что у сервера).
pub fn mth_sin(x: f64) -> f32 {
    sin_table()[((x * SIN_SCALE) as i64 & 65535) as usize]
}

/// VANILLA: Mth#cos.
pub fn mth_cos(x: f64) -> f32 {
    sin_table()[((x * SIN_SCALE + 16384.0) as i64 & 65535) as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    InWater,
    UnderWater,
    UnderFlowingWater,
    OnLand,
    InAir,
}

/// VANILLA: AbstractBoat#setInput (left, right, up, down).
#[derive(Clone, Copy, Default, Debug)]
pub struct BoatInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

/// Состояние тела лодки, которым владеет `Entity`.
#[derive(Clone, Copy, Debug)]
pub struct BoatBody {
    pub pos: [f64; 3],
    pub vel: [f64; 3],
    pub yaw: f32,
    pub on_ground: bool,
    /// Entity#isNoGravity.
    pub no_gravity: bool,
    /// EntityDimensions (float).
    pub width: f32,
    pub height: f32,
}

impl BoatBody {
    fn aabb(&self) -> [f64; 6] {
        let half = f64::from(self.width / 2.0);
        let h = f64::from(self.height);
        let p = self.pos;
        [
            p[0] - half,
            p[1],
            p[2] - half,
            p[0] + half,
            p[1] + h,
            p[2] + half,
        ]
    }
}

/// Течение воды в боксе: (высота воды, накопленный поток, число клеток).
/// Данные тика лодки для сети: вёсла, позиция, yaw, on_ground.
pub type BoatOut = ((bool, bool), [f64; 3], f32, bool);

pub type FluidSample = (f64, [f64; 3], u32);

#[derive(Default, Debug)]
pub struct BoatSim {
    pub delta_rotation: f32,
    pub status: Option<Status>,
    old_status: Option<Status>,
    water_level: f64,
    land_friction: f32,
    last_yd: f64,
    /// Ввод, выставленный ПОСЛЕ тика игрока (лодка использует ввод прошлого тика).
    pub input: BoatInput,
}

fn floor(v: f64) -> i32 {
    v.floor() as i32
}
fn ceil(v: f64) -> i32 {
    v.ceil() as i32
}

impl BoatSim {
    /// VANILLA: AbstractBoat#tick (клиентская ветка isLocalInstanceAuthoritative).
    /// Возвращает состояние вёсел (left, right) для `ServerboundPaddleBoatPacket`.
    pub fn tick(
        &mut self,
        b: &mut BoatBody,
        w: &dyn Collider,
        fluid: &dyn Fn([f64; 6]) -> FluidSample,
    ) -> (bool, bool) {
        self.old_status = self.status;
        self.status = Some(self.get_status(b, w));
        // Entity#baseTick: течение воды (лодка не игрок: нормализованный поток).
        let (height, cur, n) = fluid(b.aabb());
        if height > 0.0 && n != 0 {
            apply_current(&mut b.vel, cur, 0.014);
        }
        self.float_boat(b, w);
        let paddle = self.control_boat(b);
        self.do_move(b, w);
        paddle
    }

    fn get_status(&mut self, b: &BoatBody, w: &dyn Collider) -> Status {
        if let Some(s) = self.is_underwater(b, w) {
            self.water_level = b.aabb()[4];
            return s;
        }
        if self.check_in_water(b, w) {
            return Status::InWater;
        }
        let friction = self.ground_friction(b, w);
        if friction > 0.0 {
            self.land_friction = friction;
            Status::OnLand
        } else {
            Status::InAir
        }
    }

    fn is_underwater(&self, b: &BoatBody, w: &dyn Collider) -> Option<Status> {
        let a = b.aabb();
        let max_y = a[4] + 0.001;
        let mut under = false;
        for x in floor(a[0])..ceil(a[3]) {
            for y in floor(a[4])..ceil(max_y) {
                for z in floor(a[2])..ceil(a[5]) {
                    let h = w.water_height(x, y, z);
                    if h > 0.0 && max_y < f64::from(y as f32 + h) {
                        if !w.water_is_source(x, y, z) {
                            return Some(Status::UnderFlowingWater);
                        }
                        under = true;
                    }
                }
            }
        }
        under.then_some(Status::UnderWater)
    }

    fn check_in_water(&mut self, b: &BoatBody, w: &dyn Collider) -> bool {
        let a = b.aabb();
        let mut in_water = false;
        self.water_level = -f64::MAX;
        for x in floor(a[0])..ceil(a[3]) {
            for y in floor(a[1])..ceil(a[1] + 0.001) {
                for z in floor(a[2])..ceil(a[5]) {
                    let h = w.water_height(x, y, z);
                    if h > 0.0 {
                        let height = y as f32 + h;
                        self.water_level = self.water_level.max(f64::from(height));
                        in_water |= a[1] < f64::from(height);
                    }
                }
            }
        }
        in_water
    }

    fn water_level_above(&self, b: &BoatBody, w: &dyn Collider) -> f32 {
        let a = b.aabb();
        let (x0, x1) = (floor(a[0]), ceil(a[3]));
        let (z0, z1) = (floor(a[2]), ceil(a[5]));
        let (y0, y1) = (floor(a[4]), ceil(a[4] - self.last_yd));
        'rows: for y in y0..y1 {
            let mut bh = 0.0f32;
            for x in x0..x1 {
                for z in z0..z1 {
                    let h = w.water_height(x, y, z);
                    if h > 0.0 {
                        bh = bh.max(h);
                    }
                    if bh >= 1.0 {
                        continue 'rows;
                    }
                }
            }
            if bh < 1.0 {
                return y as f32 + bh;
            }
        }
        (y1 + 1) as f32
    }

    fn ground_friction(&self, b: &BoatBody, w: &dyn Collider) -> f32 {
        let a = b.aabb();
        let bx = [a[0], a[1] - 0.001, a[2], a[3], a[1], a[5]];
        let (x0, x1) = (floor(bx[0]) - 1, ceil(bx[3]) + 1);
        let (y0, y1) = (floor(bx[1]) - 1, ceil(bx[4]) + 1);
        let (z0, z1) = (floor(bx[2]) - 1, ceil(bx[5]) + 1);
        let mut friction = 0.0f32;
        let mut count = 0i32;
        for x in x0..x1 {
            for z in z0..z1 {
                let edges = i32::from(x == x0 || x == x1 - 1) + i32::from(z == z0 || z == z1 - 1);
                if edges == 2 {
                    continue;
                }
                for y in y0..y1 {
                    if edges > 0 && y != y0 && y != y1 - 1 {
                        continue;
                    }
                    let name = w.name(x, y, z);
                    if name == "lily_pad" {
                        continue;
                    }
                    let off = w.box_offset(x, y, z);
                    let hit = w.boxes(x, y, z).iter().any(|s| {
                        let wb = [
                            s[0] + f64::from(x) + off[0],
                            s[1] + f64::from(y) + off[1],
                            s[2] + f64::from(z) + off[2],
                            s[3] + f64::from(x) + off[0],
                            s[4] + f64::from(y) + off[1],
                            s[5] + f64::from(z) + off[2],
                        ];
                        overlaps(&wb, &bx)
                    });
                    if hit {
                        friction += block_friction_of(name);
                        count += 1;
                    }
                }
            }
        }
        friction / count as f32
    }

    fn float_boat(&mut self, b: &mut BoatBody, w: &dyn Collider) {
        let status = self.status.unwrap_or(Status::InAir);
        // VANILLA: AbstractBoat#floatBoat — isNoGravity() ? 0 : -0.04.
        let mut vspeed = if b.no_gravity { 0.0 } else { -0.04 };
        let mut buoyancy = 0.0f64;
        let inv_friction: f32;
        if self.old_status == Some(Status::InAir)
            && status != Status::InAir
            && status != Status::OnLand
        {
            self.water_level = b.pos[1];
            let target_y = f64::from(self.water_level_above(b, w)) - f64::from(b.height) + 0.101;
            let mut moved = b.aabb();
            let dy = target_y - b.pos[1];
            moved[1] += dy;
            moved[4] += dy;
            if gather(w, moved).is_empty() {
                b.pos[1] = target_y;
                b.vel[1] = 0.0;
                self.last_yd = 0.0;
            }
            self.status = Some(Status::InWater);
        } else {
            match status {
                Status::InWater => {
                    buoyancy = (self.water_level - b.pos[1]) / f64::from(b.height);
                    inv_friction = 0.9;
                }
                Status::UnderFlowingWater => {
                    vspeed = -7.0e-4;
                    inv_friction = 0.9;
                }
                Status::UnderWater => {
                    buoyancy = f64::from(0.01f32);
                    inv_friction = 0.45;
                }
                Status::InAir => inv_friction = 0.9,
                Status::OnLand => {
                    inv_friction = self.land_friction;
                    // Пассажир — игрок.
                    self.land_friction /= 2.0;
                }
            }
            let f = f64::from(inv_friction);
            b.vel = [b.vel[0] * f, b.vel[1] + vspeed, b.vel[2] * f];
            self.delta_rotation *= inv_friction;
            if buoyancy > 0.0 {
                b.vel[1] = (b.vel[1] + buoyancy * (0.04 / 0.65)) * 0.75;
            }
        }
    }

    fn control_boat(&mut self, b: &mut BoatBody) -> (bool, bool) {
        let i = self.input;
        let mut acceleration = 0.0f32;
        if i.left {
            self.delta_rotation -= 1.0;
        }
        if i.right {
            self.delta_rotation += 1.0;
        }
        if i.right != i.left && !i.up && !i.down {
            acceleration += 0.005;
        }
        b.yaw += self.delta_rotation;
        if i.up {
            acceleration += 0.04;
        }
        if i.down {
            acceleration -= 0.005;
        }
        let rad = f64::from(std::f32::consts::PI / 180.0);
        let s = mth_sin(f64::from(-b.yaw * rad as f32));
        let c = mth_cos(f64::from(b.yaw * rad as f32));
        b.vel[0] += f64::from(s * acceleration);
        b.vel[2] += f64::from(c * acceleration);
        ((i.right && !i.left) || i.up, (i.left && !i.right) || i.up)
    }

    /// VANILLA: Entity#move(SELF) без ступенек и подкрадывания + restituteMovementAfterCollisions.
    fn do_move(&mut self, b: &mut BoatBody, w: &dyn Collider) {
        let delta = b.vel;
        // AbstractBoat#checkFallDamage: lastYd = deltaMovement.y.
        self.last_yd = b.vel[1];
        let bb = b.aabb();
        let shapes = gather(w, expand_towards(bb, delta));
        let m = collide_with(delta, bb, &shapes);
        let len = m[0] * m[0] + m[1] * m[1] + m[2] * m[2];
        let dlen = delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2];
        if len > 1.0e-7 || dlen - len < 1.0e-7 {
            for (p, d) in b.pos.iter_mut().zip(m) {
                *p += d;
            }
        }
        let eq = |a: f64, c: f64| (c - a).abs() < 1.0e-5;
        let x_col = !eq(delta[0], m[0]);
        let z_col = !eq(delta[2], m[2]);
        let v_col = delta[1] != m[1];
        b.on_ground = v_col && delta[1] < 0.0;
        if x_col {
            b.vel[0] = 0.0;
        }
        if z_col {
            b.vel[2] = 0.0;
        }
        if v_col {
            b.vel[1] = 0.0;
        }
        let f = f64::from(block_speed_factor(w, b.pos));
        b.vel[0] *= f;
        b.vel[2] *= f;
    }
}

/// VANILLA: EntityFluidInteraction.CurrentAccumulator#applyTo для не-игрока.
fn apply_current(vel: &mut [f64; 3], cur: [f64; 3], scale: f64) {
    let l2 = cur[0] * cur[0] + cur[1] * cur[1] + cur[2] * cur[2];
    if l2 < f64::from(1.0e-5f32) {
        return;
    }
    let l = l2.sqrt();
    let mut imp = cur.map(|c| c / l * scale);
    let il = (imp[0] * imp[0] + imp[1] * imp[1] + imp[2] * imp[2]).sqrt();
    if vel[0].abs() < 0.003 && vel[2].abs() < 0.003 && il < 0.0045 {
        imp = imp.map(|c| c / il * 0.0045);
    }
    for k in 0..3 {
        vel[k] += imp[k];
    }
}

/// VANILLA: AbstractBoat#clampRotation — пассажир ±105° от носа; возвращает новый yaw игрока.
pub fn clamp_passenger_yaw(passenger_yaw: f32, boat_yaw: f32) -> f32 {
    let wrap = |d: f32| {
        let mut d = d % 360.0;
        if d >= 180.0 {
            d -= 360.0;
        }
        if d < -180.0 {
            d += 360.0;
        }
        d
    };
    let d = wrap(passenger_yaw - boat_yaw);
    passenger_yaw + (d.clamp(-105.0, 105.0) - d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_passenger_yaw_limits_105() {
        assert_eq!(clamp_passenger_yaw(150.0, 0.0), 105.0);
        assert_eq!(clamp_passenger_yaw(-150.0, 0.0), -105.0);
        assert_eq!(clamp_passenger_yaw(60.0, 0.0), 60.0);
        // через ±180: нос 170°, взгляд -170° = +20° от носа → без изменений
        assert!((clamp_passenger_yaw(-170.0, 170.0) - -170.0).abs() < 1e-4);
        // нос 0, взгляд 380° (=20°) не трогаем, 480° (=120°) → 465°
        assert!((clamp_passenger_yaw(480.0, 0.0) - 465.0).abs() < 1e-3);
    }

    const FULL: [[f64; 6]; 1] = [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];
    /// Дно из камня при y < `floor`; вода — слои `floor..water_top` (источники).
    struct World {
        floor: i32,
        water_top: i32,
        ice: bool,
    }
    impl Collider for World {
        fn boxes(&self, _x: i32, y: i32, _z: i32) -> &'static [[f64; 6]] {
            if y < self.floor {
                &FULL
            } else {
                &[]
            }
        }
        fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
            if y < self.floor {
                if self.ice {
                    "ice"
                } else {
                    "stone"
                }
            } else if y < self.water_top {
                "water"
            } else {
                "air"
            }
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
        fn water_height(&self, _: i32, y: i32, _: i32) -> f32 {
            if y >= self.floor && y < self.water_top {
                if y + 1 < self.water_top {
                    1.0
                } else {
                    8.0 / 9.0
                }
            } else {
                0.0
            }
        }
        fn water_is_source(&self, _: i32, y: i32, _: i32) -> bool {
            y >= self.floor && y < self.water_top
        }
    }

    fn body(y: f64) -> BoatBody {
        BoatBody {
            pos: [0.5, y, 0.5],
            vel: [0.0; 3],
            yaw: 0.0,
            on_ground: false,
            no_gravity: false,
            width: 1.375,
            height: 0.5625,
        }
    }
    fn no_fluid() -> impl Fn([f64; 6]) -> FluidSample {
        |_| (0.0, [0.0; 3], 0)
    }

    #[test]
    fn table_sin_matches_vanilla_table() {
        assert_eq!(mth_sin(0.0), 0.0);
        assert!((mth_cos(0.0) - 1.0).abs() < 1.0e-6);
        assert!((mth_sin(std::f64::consts::FRAC_PI_2) - 1.0).abs() < 1.0e-4);
    }

    #[test]
    fn falls_into_water_and_floats() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let mut b = body(3.0);
        let mut s = BoatSim::default();
        for _ in 0..200 {
            s.tick(&mut b, &w, &no_fluid());
        }
        // Плавает у поверхности воды (верх воды 0 - 1/9).
        assert!(b.pos[1] > -0.6 && b.pos[1] < 0.1, "y={}", b.pos[1]);
        assert_eq!(s.status, Some(Status::InWater));
    }

    #[test]
    fn no_gravity_boat_hangs_in_air() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let mut b = body(3.0);
        b.no_gravity = true;
        let mut s = BoatSim::default();
        for _ in 0..40 {
            s.tick(&mut b, &w, &no_fluid());
        }
        assert!((b.pos[1] - 3.0).abs() < 1.0e-9, "y={}", b.pos[1]);
    }

    #[test]
    fn forward_accelerates_along_yaw() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let mut b = body(-0.4);
        let mut s = BoatSim::default();
        s.input.up = true;
        for _ in 0..40 {
            s.tick(&mut b, &w, &no_fluid());
        }
        // yaw 0 → +Z.
        assert!(b.pos[2] > 1.0 && b.vel[2] > 0.05, "{:?}", b.pos);
        assert!((b.pos[0] - 0.5).abs() < 1.0e-6);
    }

    #[test]
    fn land_ice_is_slippery_stone_is_not() {
        let run = |ice: bool| {
            let w = World {
                floor: 0,
                water_top: 0,
                ice,
            };
            let mut b = body(0.0);
            let mut s = BoatSim::default();
            s.tick(&mut b, &w, &no_fluid());
            assert_eq!(s.status, Some(Status::OnLand));
            b.vel = [0.0, 0.0, 0.5];
            for _ in 0..10 {
                s.tick(&mut b, &w, &no_fluid());
            }
            b.pos[2]
        };
        assert!(run(true) > run(false));
    }

    #[test]
    fn hitting_wall_zeroes_velocity() {
        struct Wall;
        impl Collider for Wall {
            fn boxes(&self, x: i32, y: i32, _z: i32) -> &'static [[f64; 6]] {
                if y < 0 || x >= 3 {
                    &FULL
                } else {
                    &[]
                }
            }
            fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
                if y < 0 {
                    "stone"
                } else {
                    "air"
                }
            }
            fn has_chunk(&self, _: i32, _: i32) -> bool {
                true
            }
        }
        let mut b = body(0.0);
        b.pos[0] = 2.0;
        b.vel = [1.0, 0.0, 0.0];
        let mut s = BoatSim::default();
        s.tick(&mut b, &Wall, &no_fluid());
        assert_eq!(b.vel[0], 0.0);
        assert!(
            (b.pos[0] + 1.375 / 2.0 - 3.0).abs() < 1.0e-3,
            "{}",
            b.pos[0]
        );
    }

    #[test]
    fn turning_changes_yaw_and_decays() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let mut b = body(-0.4);
        let mut s = BoatSim::default();
        s.input.right = true;
        for _ in 0..5 {
            s.tick(&mut b, &w, &no_fluid());
        }
        assert!(b.yaw > 0.0, "yaw={}", b.yaw);
        s.input = BoatInput::default();
        let (y0, d0) = (b.yaw, s.delta_rotation);
        s.tick(&mut b, &w, &no_fluid());
        // Без ввода deltaRotation гасится трением, yaw не стоит на месте.
        assert!(s.delta_rotation.abs() < d0.abs());
        assert!(b.yaw != y0 || d0 == 0.0);
    }

    #[test]
    fn reverse_is_slower_than_forward() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let run = |fwd: bool| {
            let mut b = body(-0.4);
            let mut s = BoatSim::default();
            s.input.up = fwd;
            s.input.down = !fwd;
            for _ in 0..30 {
                s.tick(&mut b, &w, &no_fluid());
            }
            b.pos[2] - 0.5
        };
        let (f, r) = (run(true), run(false));
        assert!(f > 0.0 && r < 0.0 && f.abs() > r.abs() * 3.0, "{f} {r}");
    }

    #[test]
    fn paddle_state_follows_input() {
        let w = World {
            floor: -3,
            water_top: 0,
            ice: false,
        };
        let mut b = body(-0.4);
        let mut s = BoatSim::default();
        s.input.right = true;
        assert_eq!(s.tick(&mut b, &w, &no_fluid()), (true, false));
        s.input = BoatInput {
            up: true,
            ..Default::default()
        };
        assert_eq!(s.tick(&mut b, &w, &no_fluid()), (true, true));
        s.input = BoatInput {
            left: true,
            right: true,
            ..Default::default()
        };
        assert_eq!(s.tick(&mut b, &w, &no_fluid()), (false, false));
    }

    #[test]
    fn submerged_boat_sinks_to_floor() {
        // VANILLA: UNDER_WATER — гравитация сильнее плавучести (сервер через 60 тиков высадит пассажира).
        let w = World {
            floor: -3,
            water_top: 3,
            ice: false,
        };
        let mut b = body(-1.0);
        let mut s = BoatSim::default();
        for _ in 0..300 {
            s.tick(&mut b, &w, &no_fluid());
        }
        assert!((b.pos[1] + 3.0).abs() < 1.0e-3, "y={}", b.pos[1]);
        assert_eq!(s.status, Some(Status::UnderWater));
    }

    #[test]
    fn clamp_limits_to_105() {
        assert!((clamp_passenger_yaw(170.0, 0.0) - 105.0).abs() < 1.0e-4);
        assert!((clamp_passenger_yaw(-170.0, 0.0) + 105.0).abs() < 1.0e-4);
        assert_eq!(clamp_passenger_yaw(30.0, 0.0), 30.0);
    }
}
