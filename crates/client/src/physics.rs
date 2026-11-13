//! Локальный игрок: тик движения, коллизии, прыжок, спринт, подкрадывание, творческий полёт.
//!
//! VANILLA: LocalPlayer#aiStep, Player#aiStep/travel/maybeBackOffFromEdge, LivingEntity#aiStep/travel/
//! travelInAir/jumpFromGround, Entity#move/collide/collideWithShapes/moveRelative (26.3).
//! Не сделано (см. TODO-166/167): вода/лава/плавание, лестницы, паутина/ягоды, ползание, эффекты, падение с уроном.
//! VANILLA-UNSURE: смещение коллизии блоков (OffsetType) не учитывается; `isHorizontalCollisionMinor` = false.
use ezzmc_mesh::blockinfo as bi;
use ezzmc_world::level::Level;

const EPS: f64 = 1.0e-7;
/// VANILLA: Attributes.STEP_HEIGHT игрока.
const STEP_HEIGHT: f32 = 0.6;
/// VANILLA: Attributes.GRAVITY / JUMP_STRENGTH / MOVEMENT_SPEED / SNEAKING_SPEED.
const GRAVITY: f64 = 0.08;
const JUMP_STRENGTH: f32 = 0.42;
const MOVEMENT_SPEED: f32 = 0.1;
const SNEAKING_SPEED: f32 = 0.3;
/// VANILLA: EntityType.PLAYER = EntityDimensions.scalable(0.6F, 1.8F).
/// Размеры хранятся как f32, как в ванили (EntityDimensions: float width/height);
/// в f64 переводим только при расчёте AABB, иначе полуширина расходится с сервером.
const WIDTH: f32 = 0.6;
const HEIGHT_STANDING: f32 = 1.8;
const HEIGHT_CROUCHING: f32 = 1.5;
const EYE_STANDING: f64 = 1.62;
const EYE_CROUCHING: f64 = 1.27;
/// VANILLA: Pose.SWIMMING — хитбокс 0.6×0.6, глаза 0.4.
const HEIGHT_SWIMMING: f32 = 0.6;
const EYE_SWIMMING: f64 = 0.4;

/// Доступ к миру для физики (позволяет тестировать без чанков).
pub trait Collider {
    /// Боксы коллизии блока `[x0,y0,z0,x1,y1,z1]` в координатах блока.
    fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]];
    /// Имя блока без пространства имён (`"air"` для пустоты).
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str;
    /// Сдвиг боксов блока (`getOffset` у бамбука/сталагмита; форма в данных уже содержит смещение (0,0)).
    fn box_offset(&self, _x: i32, _y: i32, _z: i32) -> [f64; 3] {
        [0.0; 3]
    }
    /// Динамические боксы (движущийся поршень) в координатах блока.
    fn extra_boxes(&self, _x: i32, _y: i32, _z: i32) -> Vec<[f64; 6]> {
        Vec::new()
    }
    /// Боксы сущностей, с которыми можно столкнуться (лодки), пересекающие `region` (мировые координаты).
    fn entity_boxes(&self, _region: [f64; 6]) -> Vec<[f64; 6]> {
        Vec::new()
    }
    /// Загружен ли чанк (для `travelInAir`).
    fn has_chunk(&self, x: i32, z: i32) -> bool;
    /// Высота воды в блоке (0 — воды нет; 1 — полный блок или вода сверху).
    fn water_height(&self, _x: i32, _y: i32, _z: i32) -> f32 {
        0.0
    }
    /// Течение воды в блоке (`FlowingFluid#getFlow`, нормализовано).
    fn water_flow(&self, _x: i32, _y: i32, _z: i32) -> [f64; 3] {
        [0.0; 3]
    }
    /// Вода в блоке — источник (`FluidState#isSource`).
    fn water_is_source(&self, _x: i32, _y: i32, _z: i32) -> bool {
        false
    }
    /// Высота лавы в блоке (как `water_height`).
    fn lava_height(&self, _x: i32, _y: i32, _z: i32) -> f32 {
        0.0
    }
    /// Течение лавы в блоке.
    fn lava_flow(&self, _x: i32, _y: i32, _z: i32) -> [f64; 3] {
        [0.0; 3]
    }
}

impl Collider for Level {
    fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
        let s = self.chunks.block(x, y, z);
        s.props()
            .map_or(&[], |p| bi::collision_boxes_of(p.collision_shape))
    }
    fn box_offset(&self, x: i32, y: i32, z: i32) -> [f64; 3] {
        let s = self.chunks.block(x, y, z);
        if s.is_air() {
            return [0.0; 3];
        }
        let o = bi::collision_offset(s, x, z);
        [f64::from(o[0]), f64::from(o[1]), f64::from(o[2])]
    }
    fn extra_boxes(&self, x: i32, y: i32, z: i32) -> Vec<[f64; 6]> {
        match self.pistons.get(&(x, y, z)) {
            Some(e) if self.chunks.block(x, y, z).name() == "moving_piston" => {
                crate::pistonview::collision_boxes(e, None)
            }
            _ => Vec::new(),
        }
    }
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str {
        self.chunks.block(x, y, z).name()
    }
    /// VANILLA: Level#getEntityCollisions — Entity#canCollideWith: лодки (AbstractBoat#canBeCollidedWith),
    /// кроме той, на которой сидит игрок (isPassengerOfSameVehicle).
    fn entity_boxes(&self, region: [f64; 6]) -> Vec<[f64; 6]> {
        let own_vehicle = self.riders.vehicle_of.get(&self.own_id).copied();
        let mut out = Vec::new();
        for e in self.entities.map.values() {
            if Some(e.id) == own_vehicle
                || (e.pos[0] - region[0]).abs() > 64.0
                || (e.pos[2] - region[2]).abs() > 64.0
            {
                continue;
            }
            let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else {
                continue;
            };
            if !(name.ends_with("_boat") || name.ends_with("_raft")) {
                continue;
            }
            let (w, h) = ezzmc_world::entity_size::size(name).unwrap_or((1.375, 0.5625));
            let r = f64::from(w) / 2.0;
            let bb = [
                e.pos[0] - r,
                e.pos[1],
                e.pos[2] - r,
                e.pos[0] + r,
                e.pos[1] + f64::from(h),
                e.pos[2] + r,
            ];
            if overlaps(&region, &bb) {
                out.push(bb);
            }
        }
        out
    }
    fn has_chunk(&self, x: i32, z: i32) -> bool {
        self.chunks.get(x >> 4, z >> 4).is_some()
    }
    /// VANILLA: FluidState#getHeight (FlowingFluid: жидкость сверху → 1).
    fn water_height(&self, x: i32, y: i32, z: i32) -> f32 {
        fluid_height_of(self, x, y, z, false)
    }
    fn water_flow(&self, x: i32, y: i32, z: i32) -> [f64; 3] {
        fluid_flow_of(self, x, y, z, false)
    }
    fn water_is_source(&self, x: i32, y: i32, z: i32) -> bool {
        bi::fluid(self.chunks.block(x, y, z)).is_some_and(|f| !f.lava && f.source)
    }
    fn lava_height(&self, x: i32, y: i32, z: i32) -> f32 {
        fluid_height_of(self, x, y, z, true)
    }
    fn lava_flow(&self, x: i32, y: i32, z: i32) -> [f64; 3] {
        fluid_flow_of(self, x, y, z, true)
    }
}

pub(crate) fn fluid_height_of(level: &Level, x: i32, y: i32, z: i32, lava: bool) -> f32 {
    let Some(fl) = bi::fluid(level.chunks.block(x, y, z)).filter(|f| f.lava == lava) else {
        return 0.0;
    };
    if bi::fluid(level.chunks.block(x, y + 1, z)).is_some_and(|a| a.lava == lava) {
        return 1.0;
    }
    fl.own_height()
}
fn fluid_flow_of(level: &Level, x: i32, y: i32, z: i32, lava: bool) -> [f64; 3] {
    let Some(fl) = bi::fluid(level.chunks.block(x, y, z)).filter(|f| f.lava == lava) else {
        return [0.0; 3];
    };
    let g = ezzmc_mesh::fluid::FnBlocks(|p: [i32; 3]| level.chunks.block(p[0], p[1], p[2]));
    ezzmc_mesh::fluid::flow3(&g, [x, y, z], &fl)
}

/// VANILLA: EntityFluidInteraction#update для произвольного бокса: (высота, поток·0.014/0.0023333
/// усреднённый) отдельно для воды и лавы. Возвращает `[(height, push); 2]` (вода, лава).
pub fn fluid_interaction(level: &Level, b: [f64; 6]) -> [(f64, [f64; 3]); 2] {
    let acc = fluid_accumulate(level, b);
    let push = |a: &FluidAcc, scale: f64| -> [f64; 3] {
        if a.n == 0 || len_sq(a.cur) < f64::from(1.0e-5f32) {
            return [0.0; 3];
        }
        let k = scale / f64::from(a.n);
        a.cur.map(|c| c * k)
    };
    [
        (acc[0].height, push(&acc[0], 0.014)),
        (acc[1].height, push(&acc[1], 0.007 / 3.0)),
    ]
}

/// VANILLA: EntityFluidInteraction#update — высота и накопленное течение (вода, лава) для бокса.
pub(crate) fn fluid_accumulate(level: &Level, b: [f64; 6]) -> [FluidAcc; 2] {
    let d = 0.001;
    let bx = [b[0] + d, b[1] + d, b[2] + d, b[3] - d, b[4] - d, b[5] - d];
    let lo = [
        bx[0].floor() as i32,
        bx[1].floor() as i32,
        bx[2].floor() as i32,
    ];
    let hi = [
        bx[3].ceil() as i32 - 1,
        bx[4].ceil() as i32 - 1,
        bx[5].ceil() as i32 - 1,
    ];
    let mut acc = [FluidAcc::default(), FluidAcc::default()];
    for x in lo[0]..=hi[0] {
        for y in lo[1]..=hi[1] {
            for z in lo[2]..=hi[2] {
                for (i, lava) in [false, true].into_iter().enumerate() {
                    let hh = fluid_height_of(level, x, y, z, lava);
                    if hh <= 0.0 {
                        continue;
                    }
                    let top = f64::from(y) + f64::from(hh);
                    if top < bx[1] {
                        continue;
                    }
                    let a = &mut acc[i];
                    a.height = a.height.max(top - b[1]);
                    let mut fv = fluid_flow_of(level, x, y, z, lava);
                    a.cur_h = a.cur_h.max(a.height);
                    if a.cur_h < 0.4 {
                        fv = fv.map(|c| c * a.cur_h);
                    }
                    for (t, c) in a.cur.iter_mut().zip(fv) {
                        *t += c;
                    }
                    a.n += 1;
                }
            }
        }
    }
    acc
}

/// Состояние клавиш на тик.
#[derive(Clone, Copy, Default, Debug)]
pub struct Input {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sneak: bool,
    pub sprint: bool,
}

impl Input {
    /// VANILLA: KeyboardInput#calculateImpulse → Vec2(left, forward), нормализованный.
    fn move_vector(self) -> (f32, f32) {
        let f = |p: bool, m: bool| f32::from(u8::from(p)) - f32::from(u8::from(m));
        // VANILLA: KeyboardInput#tick — new Vec2(left, forward).normalized() (dist < 1e-4 → ZERO).
        let (x, y) = (f(self.left, self.right), f(self.forward, self.back));
        let d = (x * x + y * y).sqrt();
        if d < 1.0e-4 {
            (0.0, 0.0)
        } else {
            (x / d, y / d)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: [f64; 3],
    pub prev_pos: [f64; 3],
    pub vel: [f64; 3],
    pub on_ground: bool,
    pub horizontal_collision: bool,
    /// VANILLA: Entity#minorHorizontalCollision (LocalPlayer#isHorizontalCollisionMinor):
    /// упор скользящий, ввод почти совпадает с фактическим движением (< 8°).
    pub minor_horizontal_collision: bool,
    pub crouching: bool,
    /// VANILLA: LivingEntity#isUsingItem (еда/питьё: ввод ×0.2, спринт не стартует).
    pub using_item: bool,
    /// VANILLA: Pose.SWIMMING вне воды — ползание под 1.5-блочным потолком (isVisuallyCrawling).
    pub crawling: bool,
    /// VANILLA: Entity#hasPose(SWIMMING) на конце прошлого тика (Player#updatePlayerPose идёт после aiStep).
    pose_swimming: bool,
    pub sprinting: bool,
    pub flying: bool,
    /// Уровни эффектов (amplifier+1, 0 = нет); выставляются клиентом перед тиком.
    pub fx: EffectLevels,
    /// VANILLA: Entity#isInWater / isUnderWater (глаза в воде) / isSwimming (поза плавания).
    pub in_water: bool,
    pub in_lava: bool,
    pub under_water: bool,
    pub swimming: bool,
    /// Наклон камеры (градусы) — для плавания «по взгляду».
    pub pitch: f32,
    /// VANILLA: LocalPlayer#xBob/yBob (+O) — качание руки от 1-го лица.
    pub x_bob: f32,
    pub y_bob: f32,
    pub x_bob_o: f32,
    pub y_bob_o: f32,
    /// VANILLA: FirstPersonHandsAndItems#mainHandHeight (+old) и Player#itemSwapTicker.
    pub hand_h: f32,
    pub hand_h_o: f32,
    /// VANILLA: Entity#getFluidHeight(WATER) по AABB.
    fluid_h: f64,
    lava_h: f64,
    pub mayfly: bool,
    /// VANILLA: Abilities#flyingSpeed (по умолчанию 0.05).
    pub fly_speed: f32,
    pub fall_distance: f64,
    /// VANILLA: Entity#stuckSpeedMultiplier.
    stuck: [f64; 3],
    piston_deltas: [f64; 3],
    piston_time: i64,
    no_jump_delay: u8,
    jump_trigger_time: u8,
    sprint_trigger_time: u8,
    /// VANILLA: Options#sprintWindow (по умолчанию 7, 0 — двойное нажатие выключено).
    pub sprint_window: u8,
    was_jump: bool,
    /// Приседание (Shift) прошлого тика. VANILLA: LocalPlayer#aiStep читает keyPresses.shift()
    /// до input.tick(), поэтому поза движения и медленный ход отстают на тик от нажатия.
    was_sneak: bool,
    /// VANILLA: Options#autoJump, LocalPlayer#autoJumpTime.
    pub auto_jump: bool,
    auto_jump_time: u8,
    /// Последний ввод движения (left, forward) и yaw — для updateAutoJump.
    last_move: (f32, f32),
    last_yaw: f32,
    /// Модифицированный ввод (xxa, zza) и yaw текущего тика — для minor-проверки упора.
    move_input: (f32, f32, f32),
    had_forward: bool,
    /// VANILLA: ClientAvatarState walkDist/walkDistO/bob/bobO (боббинг камеры).
    walk_dist: f32,
    walk_dist_o: f32,
    bob: f32,
    bob_o: f32,
    /// VANILLA: LivingEntity#hurtTime/hurtDuration, Player#hurtDir, deathTime — тряска камеры (GameRenderer#bobHurt).
    pub hurt_time: i32,
    pub hurt_dir: f32,
    pub death_time: i32,
    pub dead: bool,
    /// VANILLA: Camera#eyeHeight/eyeHeightOld — плавная высота глаз (присед).
    eye_h: f64,
    eye_h_o: f64,
    pub anim: crate::avatar::AvatarAnim,
}

impl Player {
    pub fn new(pos: [f64; 3]) -> Self {
        Self {
            pos,
            prev_pos: pos,
            vel: [0.0; 3],
            on_ground: false,
            horizontal_collision: false,
            minor_horizontal_collision: false,
            crouching: false,
            using_item: false,
            crawling: false,
            pose_swimming: false,
            sprinting: false,
            flying: false,
            fx: EffectLevels::default(),
            in_water: false,
            in_lava: false,
            under_water: false,
            swimming: false,
            pitch: 0.0,
            x_bob: 0.0,
            y_bob: 0.0,
            x_bob_o: 0.0,
            y_bob_o: 0.0,
            hand_h: 0.0,
            hand_h_o: 0.0,
            fluid_h: 0.0,
            lava_h: 0.0,
            mayfly: false,
            fly_speed: 0.05,
            fall_distance: 0.0,
            stuck: [0.0; 3],
            piston_deltas: [0.0; 3],
            piston_time: i64::MIN,
            no_jump_delay: 0,
            jump_trigger_time: 0,
            sprint_trigger_time: 0,
            sprint_window: 7,
            was_jump: false,
            was_sneak: false,
            auto_jump: false,
            auto_jump_time: 0,
            last_move: (0.0, 0.0),
            last_yaw: 0.0,
            move_input: (0.0, 0.0, 0.0),
            had_forward: false,
            walk_dist: 0.0,
            walk_dist_o: 0.0,
            bob: 0.0,
            bob_o: 0.0,
            hurt_time: 0,
            hurt_dir: 0.0,
            death_time: 0,
            dead: false,
            eye_h: EYE_STANDING,
            eye_h_o: EYE_STANDING,
            anim: crate::avatar::AvatarAnim::default(),
        }
    }

    /// Телепорт сервера: позиция, скорость не трогается (VANILLA: handleMovePlayer сохраняет относительное).
    pub fn teleport(&mut self, pos: [f64; 3]) {
        self.pos = pos;
        self.prev_pos = pos;
        self.vel = [0.0; 3];
    }

    pub fn eye_height(&self) -> f64 {
        if self.swimming || self.crawling {
            EYE_SWIMMING
        } else if self.crouching {
            EYE_CROUCHING
        } else {
            EYE_STANDING
        }
    }

    fn height(&self) -> f64 {
        if self.swimming || self.crawling {
            f64::from(HEIGHT_SWIMMING)
        } else if self.crouching {
            f64::from(HEIGHT_CROUCHING)
        } else {
            f64::from(HEIGHT_STANDING)
        }
    }

    pub fn aabb_pub(&self) -> [f64; 6] {
        self.aabb()
    }

    /// VANILLA: Entity#move(MoverType.PISTON) + limitPistonMovement (0.51 за тик по оси).
    pub fn piston_move(&mut self, w: &dyn Collider, mut m: [f64; 3], game_time: i64) {
        if self.piston_time != game_time {
            self.piston_deltas = [0.0; 3];
            self.piston_time = game_time;
        }
        if len_sq(m) > 1.0e-8 {
            for (mi, pd) in m.iter_mut().zip(self.piston_deltas.iter_mut()) {
                if *mi != 0.0 {
                    let c = (*mi + *pd).clamp(-0.51, 0.51);
                    *mi = c - *pd;
                    *pd = c;
                }
            }
        }
        if m == [0.0; 3] {
            return;
        }
        let r = self.collide(w, m);
        self.pos = [self.pos[0] + r[0], self.pos[1] + r[1], self.pos[2] + r[2]];
    }

    pub(crate) fn aabb(&self) -> [f64; 6] {
        aabb_at(self.pos, self.height())
    }

    /// VANILLA: GameRenderer#bobView — матрица в пространстве вида (T·Rz·Rx).
    pub fn bob_matrix(&self, pt: f32) -> glam::Mat4 {
        let wda = self.walk_dist - self.walk_dist_o;
        let d = -(self.walk_dist + wda * pt);
        let bob = self.bob_o + (self.bob - self.bob_o) * pt;
        let pi = std::f32::consts::PI;
        glam::Mat4::from_translation(glam::Vec3::new(
            (d * pi).sin() * bob * 0.5,
            -((d * pi).cos() * bob).abs(),
            0.0,
        )) * glam::Mat4::from_rotation_z(((d * pi).sin() * bob * 3.0).to_radians())
            * glam::Mat4::from_rotation_x((((d * pi - 0.2).cos() * bob).abs() * 5.0).to_radians())
    }

    /// VANILLA: LivingEntity#handleDamageEvent / LocalPlayer#hurtTo → hurtTime = hurtDuration = 10.
    /// `dir` — Player#animateHurt(yaw) (пакет hurt_animation); для остальных источников hurtDir не меняется.
    pub fn hurt(&mut self, dir: Option<f32>) {
        self.hurt_time = 10;
        if let Some(d) = dir {
            self.hurt_dir = d;
        }
    }

    /// VANILLA: GameRenderer#bobHurt — матрица в пространстве вида (Rz(death)·Ry(−dir)·Rz(tilt)·Ry(dir)).
    pub fn hurt_matrix(&self, pt: f32, tilt_strength: f32) -> glam::Mat4 {
        let mut m = glam::Mat4::IDENTITY;
        if self.dead {
            let dur = (self.death_time as f32 + pt).min(20.0);
            m = glam::Mat4::from_rotation_z((40.0 - 8000.0 / (dur + 200.0)).to_radians());
        }
        let hurt = self.hurt_time as f32 - pt;
        if hurt < 0.0 {
            return m;
        }
        let hurt = hurt / 10.0;
        let hurt = (hurt * hurt * hurt * hurt * std::f32::consts::PI).sin();
        let rr = self.hurt_dir.to_radians();
        m * glam::Mat4::from_rotation_y(-rr)
            * glam::Mat4::from_rotation_z((-hurt * 14.0 * tilt_strength).to_radians())
            * glam::Mat4::from_rotation_y(rr)
    }

    /// После `tick`: камера и анимация аватара. VANILLA: Camera#tick (eyeHeight += (target − eyeHeight) · 0.5)
    /// и LivingEntity#tick (тело/голова/ходьба/взмах).
    pub fn post_tick(&mut self, yaw: f32, pitch: f32) {
        self.eye_h_o = self.eye_h;
        self.eye_h += (self.eye_height() - self.eye_h) * 0.5;
        self.anim.swimming = self.swimming || self.crawling; // VANILLA: Entity#isVisuallySwimming (Pose.SWIMMING)
        self.anim.in_water = self.in_water;
        self.anim.tick(self.pos, self.prev_pos, yaw, pitch);
        // VANILLA: LivingEntity#aiStep (hurtTime--) и tickDeath (deathTime++ до 20, пока здоровье ≤ 0).
        if self.hurt_time > 0 {
            self.hurt_time -= 1;
        }
        if self.dead && self.death_time < 20 {
            self.death_time += 1;
        } else if !self.dead {
            self.death_time = 0;
        }
        // LocalPlayer#aiStep: xBob/yBob += (rot − bob)·0.5.
        self.y_bob_o = self.y_bob;
        self.x_bob_o = self.x_bob;
        self.x_bob += (pitch - self.x_bob) * 0.5;
        self.y_bob += (yaw - self.y_bob) * 0.5;
    }

    /// VANILLA: LivingEntity#swing(MAIN_HAND).
    pub fn swing(&mut self) {
        self.anim.swing();
    }

    /// Глаза с интерполяцией между тиками.
    pub fn eye_lerp(&self, pt: f64) -> [f64; 3] {
        let l = |a: f64, b: f64| a + (b - a) * pt;
        [
            l(self.prev_pos[0], self.pos[0]),
            l(self.prev_pos[1], self.pos[1]) + l(self.eye_h_o, self.eye_h),
            l(self.prev_pos[2], self.pos[2]),
        ]
    }

    /// VANILLA: LocalPlayer#aiStep (порядок как в ванили).
    pub fn tick(&mut self, w: &dyn Collider, input: Input, yaw: f32) {
        let mut input = input;
        // VANILLA: LocalPlayer#aiStep — autoJumpTime → input.makeJump().
        let mut was_auto_jump = false;
        if self.auto_jump_time > 0 {
            self.auto_jump_time -= 1;
            was_auto_jump = true;
            input.jump = true;
        }
        self.last_yaw = yaw;
        self.last_move = input.move_vector();
        self.prev_pos = self.pos;
        self.update_fluid_state(w);
        // ClientAvatarState#tick + AbstractClientPlayer#updateBob
        self.walk_dist_o = self.walk_dist;
        let t_bob = if self.on_ground {
            (((self.vel[0] * self.vel[0] + self.vel[2] * self.vel[2]).sqrt()) as f32).min(0.1)
        } else {
            0.0
        };
        self.bob_o = self.bob;
        self.bob += (t_bob - self.bob) * 0.4;
        self.jump_trigger_time = self.jump_trigger_time.saturating_sub(1);
        self.sprint_trigger_time = self.sprint_trigger_time.saturating_sub(1);
        self.no_jump_delay = self.no_jump_delay.saturating_sub(1);

        let was_jump = self.was_jump;
        self.was_jump = input.jump;
        // VANILLA: LocalPlayer#aiStep — crouching и wasShiftKeyDown берутся из keyPresses ДО input.tick(),
        // то есть из Shift прошлого тика. Grim: PlayerBaseTick.isSlowMovement → wasSneaking.
        let was_sneak = self.was_sneak;
        self.was_sneak = input.sneak;
        let had_forward = self.had_forward;
        let (left, fwd) = input.move_vector();
        self.had_forward = fwd > 1.0e-5;

        // crouching: pose
        let want_crouch = !self.flying && !self.swimming && was_sneak;
        let stand_fits = self.fits(w, f64::from(HEIGHT_STANDING));
        // Player#updatePlayerPose: не влезаем в стоя/присед → поза SWIMMING (ползём).
        // VANILLA: Player#updatePlayerPose — поза меняется, только если влезает хотя бы SWIMMING (0.6);
        // внутри блока/двери поза остаётся прежней.
        if self.fits(w, f64::from(HEIGHT_SWIMMING)) {
            let want = !self.flying && !self.swimming && (want_crouch || !stand_fits);
            self.crawling = !self.flying
                && !self.swimming
                && !stand_fits
                && !self.fits(w, f64::from(HEIGHT_CROUCHING));
            self.crouching = want && !self.crawling;
        }

        // спринт
        // VANILLA: LocalPlayer#aiStep — isSlowDueToUsingItem обнуляет sprintTriggerTime.
        if was_sneak || input.back || self.using_item {
            self.sprint_trigger_time = 0;
        }
        let moving_slowly = self.crouching || self.crawling;
        // VANILLA: LocalPlayer#isMovingSlowly = isCrouching || Entity#isVisuallyCrawling (поза SWIMMING вне воды).
        // Поза обновляется в конце тика, поэтому на первом тике после выхода из воды вход ещё замедлен (×0.3).
        let slow_input = moving_slowly || (self.pose_swimming && !self.in_water);
        let shallow = self.in_water && !self.under_water;
        if !self.sprinting
            && fwd > 1.0e-5 // VANILLA: ClientInput#hasForwardImpulse
            && (!shallow || self.flying)
            && (!slow_input || self.under_water)
            && !self.using_item
        // VANILLA: canStartSprinting → !isSlowDueToUsingItem
        {
            if !had_forward {
                if self.sprint_trigger_time > 0 {
                    self.sprinting = true;
                } else {
                    // VANILLA: options.sprintWindow
                    self.sprint_trigger_time = self.sprint_window;
                }
            }
            if input.sprint {
                self.sprinting = true;
            }
        }
        if self.sprinting {
            // LocalPlayer#shouldStopSwimSprinting / shouldStopRunSprinting.
            let stop = if self.swimming {
                !self.in_water || (fwd <= 1.0e-5 && !self.on_ground && !input.sneak)
            } else {
                (shallow && !self.flying)
                    || fwd <= 1.0e-5
                    // VANILLA: LocalPlayer#shouldStopRunSprinting — horizontalCollision && !minorHorizontalCollision
                    || (self.horizontal_collision && !self.minor_horizontal_collision)
            };
            if stop {
                self.sprinting = false;
            }
        }

        // творческий полёт: двойное нажатие прыжка
        if self.mayfly && !was_jump && input.jump && !was_auto_jump {
            if self.jump_trigger_time == 0 {
                self.jump_trigger_time = 7;
            } else if !self.swimming {
                self.flying = !self.flying;
                if self.flying && self.on_ground {
                    self.jump_from_ground(w, input.sprint && self.sprinting, yaw);
                }
                self.jump_trigger_time = 0;
            }
        }

        if self.flying {
            self.fall_distance = 0.0;
            let dy = i32::from(input.jump) - i32::from(input.sneak);
            if dy != 0 {
                self.vel[1] += f64::from(dy as f32 * self.fly_speed * 3.0);
            }
        }

        // LivingEntity#aiStep: гашение малых скоростей
        if self.vel[0] * self.vel[0] + self.vel[2] * self.vel[2] < 9.0e-6 {
            self.vel[0] = 0.0;
            self.vel[2] = 0.0;
        }
        if self.vel[1].abs() < 0.003 {
            self.vel[1] = 0.0;
        }

        // applyInput
        let (xxa, zza) = modify_input((left, fwd), slow_input, self.using_item);

        // LocalPlayer#aiStep: sneak в воде → goDownInWater.
        // VANILLA: Player#isAffectedByFluids = !abilities.flying (в полёте вода не тормозит и не топит).
        if self.in_water && input.sneak && !self.flying {
            self.vel[1] -= f64::from(0.04f32);
        }
        // прыжок. VANILLA: LivingEntity#aiStep (jumpInLiquid / порог getFluidJumpThreshold = 0.4).
        if input.jump && !self.flying {
            let fh = if self.in_lava {
                self.lava_h
            } else {
                self.fluid_h
            };
            let in_water_h = self.in_water && fh > 0.0;
            // Entity#getFluidJumpThreshold
            let thr = if self.eye_height() < 0.4 { 0.0 } else { 0.4 };
            if !in_water_h || (self.on_ground && fh <= thr) {
                if !self.in_lava || (self.on_ground && self.lava_h <= thr) {
                    if (self.on_ground || (in_water_h && fh <= thr)) && self.no_jump_delay == 0 {
                        self.jump_from_ground(w, self.sprinting, yaw);
                        self.no_jump_delay = 10;
                    }
                } else {
                    self.vel[1] += f64::from(0.04f32);
                }
            } else {
                self.vel[1] += f64::from(0.04f32);
            }
        } else {
            self.no_jump_delay = 0;
        }

        // travel. VANILLA: Player#travel (плавание по взгляду) → LivingEntity#travel.
        if self.swimming {
            let look_y = f64::from(-(self.pitch * (std::f32::consts::PI / 180.0)).sin());
            let mult = if look_y < -0.2 { 0.085 } else { 0.06 };
            let above = w.water_height(
                self.pos[0].floor() as i32,
                (self.pos[1] + 0.9).floor() as i32,
                self.pos[2].floor() as i32,
            ) > 0.0;
            if look_y <= 0.0 || input.jump || above {
                self.vel[1] += (look_y - self.vel[1]) * mult;
            }
        }
        let oy = self.vel[1];
        self.move_input = (xxa, zza, yaw);
        // VANILLA: LivingEntity#shouldTravelInFluid = isInLiquid && isAffectedByFluids && !canStandOnFluid.
        if self.in_water && !self.flying {
            self.travel_in_water(w, (xxa, zza), yaw);
        } else if self.in_lava && !self.flying {
            self.travel_in_lava(w, (xxa, zza), yaw);
        } else {
            self.travel(w, (xxa, zza), yaw);
        }
        if self.flying {
            self.vel[1] = oy * 0.6;
        }
        if self.on_ground && self.flying {
            self.flying = false;
        }
        // VANILLA: Player#tick → updatePlayerPose после aiStep: поза SWIMMING, если влезаем хотя бы в неё.
        if self.fits(w, f64::from(HEIGHT_SWIMMING)) {
            self.pose_swimming = self.swimming || self.crawling;
        }
    }

    fn apply_current(&mut self, a: &FluidAcc, scale: f64) {
        if a.n != 0 && len_sq(a.cur) >= f64::from(1.0e-5f32) {
            let k = scale / f64::from(a.n);
            let mut imp = a.cur.map(|c| c * k);
            let l = len_sq(imp).sqrt();
            if self.vel[0].abs() < 0.003 && self.vel[2].abs() < 0.003 && l < 0.0045 && l > 0.0 {
                imp = imp.map(|c| c / l * 0.0045);
            }
            for (v, i) in self.vel.iter_mut().zip(imp) {
                *v += i;
            }
        }
    }

    /// VANILLA: Entity#updateFluidInteraction (вода) + updateSwimming, в начале тика (baseTick).
    fn update_fluid_state(&mut self, w: &dyn Collider) {
        let b = self.aabb();
        let d = 0.001;
        let bx = [b[0] + d, b[1] + d, b[2] + d, b[3] - d, b[4] - d, b[5] - d];
        let (x0, y0, z0) = (
            bx[0].floor() as i32,
            bx[1].floor() as i32,
            bx[2].floor() as i32,
        );
        let (x1, y1, z1) = (
            bx[3].ceil() as i32 - 1,
            bx[4].ceil() as i32 - 1,
            bx[5].ceil() as i32 - 1,
        );
        // VANILLA: EntityFluidInteraction#update (вода и лава, CurrentAccumulator на каждую).
        let mut acc = [FluidAcc::default(), FluidAcc::default()];
        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    for (i, lava) in [false, true].into_iter().enumerate() {
                        let hh = if lava {
                            w.lava_height(x, y, z)
                        } else {
                            w.water_height(x, y, z)
                        };
                        if hh <= 0.0 {
                            continue;
                        }
                        let top = f64::from(y) + f64::from(hh);
                        if top < bx[1] {
                            continue;
                        }
                        let a = &mut acc[i];
                        a.height = a.height.max(top - b[1]);
                        if !self.flying {
                            let mut fv = if lava {
                                w.lava_flow(x, y, z)
                            } else {
                                w.water_flow(x, y, z)
                            };
                            a.cur_h = a.cur_h.max(a.height);
                            if a.cur_h < 0.4 {
                                fv = fv.map(|c| c * a.cur_h);
                            }
                            for (t, c) in a.cur.iter_mut().zip(fv) {
                                *t += c;
                            }
                            a.n += 1;
                        }
                    }
                }
            }
        }
        let found = acc[0].height > 0.0;
        self.in_water = found;
        self.fluid_h = acc[0].height;
        self.in_lava = acc[1].height > 0.0;
        self.lava_h = acc[1].height;
        // CurrentAccumulator#applyTo: вода 0.014, лава 0.0023333 (FAST_LAVA в Незере — 0.007).
        // VANILLA-UNSURE: измерение не учитывается, берём обычную лаву.
        if found {
            self.apply_current(&acc[0], 0.014);
        }
        if self.in_lava {
            self.apply_current(&acc[1], 0.007 / 3.0);
        }
        if found {
            self.fall_distance = 0.0;
        }
        if self.in_lava {
            self.fall_distance *= 0.5;
        }
        let eye_y = self.pos[1] + self.eye_height();
        let (ex, ey, ez) = (
            self.pos[0].floor() as i32,
            eye_y.floor() as i32,
            self.pos[2].floor() as i32,
        );
        let eh = w.water_height(ex, ey, ez);
        self.under_water = eh > 0.0 && eye_y <= f64::from(ey) + f64::from(eh);
        // Entity#updateSwimming.
        let here = block_pos(self.pos);
        // VANILLA: Player#isSwimming = !flying && !spectator && super.isSwimming.
        let was = self.swimming && !self.flying;
        self.swimming = !self.flying
            && if was {
                self.sprinting && self.in_water
            } else {
                self.sprinting
                    && self.under_water
                    && w.water_height(here[0], here[1], here[2]) > 0.0
            };
    }

    /// VANILLA: LivingEntity#travelInWater (без эффектов/зачарований).
    fn travel_in_water(&mut self, w: &dyn Collider, input: (f32, f32), yaw: f32) {
        let is_falling = self.vel[1] <= 0.0;
        let old_y = self.pos[1];
        let slow = if self.sprinting { 0.9f32 } else { 0.8f32 };
        self.move_relative(0.02, input, yaw);
        let d = self.vel;
        self.do_move(w, d);
        let mut m = self.vel;
        if self.horizontal_collision && self.on_climbable(w) {
            m[1] = 0.2;
        }
        let mut v = [
            m[0] * f64::from(slow),
            m[1] * f64::from(0.8f32),
            m[2] * f64::from(slow),
        ];
        if !self.sprinting {
            v[1] = fluid_falling_y(v[1], is_falling, self.effective_gravity());
        }
        self.vel = v;
        self.jump_out_of_fluid(w, old_y);
    }

    /// VANILLA: LivingEntity#travelInLava.
    fn travel_in_lava(&mut self, w: &dyn Collider, input: (f32, f32), yaw: f32) {
        let is_falling = self.vel[1] <= 0.0;
        let old_y = self.pos[1];
        self.move_relative(0.02, input, yaw);
        let d = self.vel;
        self.do_move(w, d);
        let thr = if self.eye_height() < 0.4 { 0.0 } else { 0.4 };
        if self.lava_h <= thr {
            let m = self.vel;
            self.vel = [m[0] * 0.5, m[1] * f64::from(0.8f32), m[2] * 0.5];
            if !self.sprinting {
                self.vel[1] = fluid_falling_y(self.vel[1], is_falling, self.effective_gravity());
            }
        } else {
            self.vel = self.vel.map(|c| c * 0.5);
        }
        self.vel[1] -= GRAVITY / 4.0;
        self.jump_out_of_fluid(w, old_y);
    }

    /// VANILLA: LivingEntity#jumpOutOfFluid.
    fn jump_out_of_fluid(&mut self, w: &dyn Collider, old_y: f64) {
        if self.horizontal_collision {
            let v = self.vel;
            let o = [v[0], v[1] + f64::from(0.6f32) - self.pos[1] + old_y, v[2]];
            let sb = shift(self.aabb(), o[0], o[1], o[2]);
            if gather(w, sb).iter().all(|c| !overlaps(&sb, c)) {
                self.vel[1] = f64::from(0.3f32);
            }
        }
    }

    /// VANILLA: LivingEntity#onClimbable (тег `climbable`; люки-«лестницы» не учтены).
    // VANILLA-UNSURE: TrapDoorBlock#trapdoorUsableAsLadder нужен blockstate (open/facing).
    fn on_climbable(&self, w: &dyn Collider) -> bool {
        let p = block_pos(self.pos);
        !self.flying
            && matches!(
                w.name(p[0], p[1], p[2]),
                "ladder"
                    | "vine"
                    | "scaffolding"
                    | "weeping_vines"
                    | "weeping_vines_plant"
                    | "twisting_vines"
                    | "twisting_vines_plant"
                    | "cave_vines"
                    | "cave_vines_plant"
            )
    }

    fn fits(&self, w: &dyn Collider, height: f64) -> bool {
        // VANILLA: ...makeBoundingBox(position()).deflate(1.0E-7)
        let mut b = aabb_at(self.pos, height);
        for i in 0..3 {
            b[i] += 1.0e-7;
            b[i + 3] -= 1.0e-7;
        }
        // VANILLA: canPlayerFitWithinBlocksAndEntitiesWhen — без пересечения с коллизиями.
        gather(w, b).iter().all(|c| !overlaps(&b, c))
    }

    /// VANILLA: LivingEntity#getJumpPower + jumpFromGround.
    fn jump_from_ground(&mut self, w: &dyn Collider, sprinting: bool, yaw: f32) {
        // VANILLA: LivingEntity#getJumpPower + getJumpBoostPower.
        let boost = if self.fx.jump_boost > 0 {
            0.1f32 * self.fx.jump_boost as f32
        } else {
            0.0
        };
        let power = JUMP_STRENGTH * block_jump_factor(w, self.pos) + boost;
        if power <= 1.0e-5 {
            return;
        }
        self.vel[1] = f64::from(power).max(self.vel[1]);
        if sprinting {
            let a = yaw * (std::f32::consts::PI / 180.0);
            self.vel[0] += f64::from(-a.sin() * 0.2);
            self.vel[2] += f64::from(a.cos() * 0.2);
        }
    }

    /// VANILLA: LivingEntity#travelInAir (+ Player#travel для полёта).
    fn travel(&mut self, w: &dyn Collider, input: (f32, f32), yaw: f32) {
        let below = below_pos(self.pos);
        let block_friction = if self.on_ground {
            block_friction_of(w.name(below[0], below[1], below[2]))
        } else {
            1.0
        };
        let speed = self.speed();
        let influenced = if self.on_ground {
            if block_friction > 0.6 {
                speed * (0.216_000_02 / (block_friction * block_friction * block_friction))
            } else {
                speed
            }
        } else {
            self.flying_speed()
        };
        self.move_relative(influenced, input, yaw);
        // LivingEntity#handleOnClimbable
        let climbing = self.on_climbable(w);
        if climbing {
            self.fall_distance = 0.0;
            let p = block_pos(self.pos);
            let mut yd = self.vel[1].max(f64::from(-0.15f32));
            if yd < 0.0 && self.crouching && w.name(p[0], p[1], p[2]) != "scaffolding" {
                yd = 0.0;
            }
            let c = f64::from(0.15f32);
            self.vel = [self.vel[0].clamp(-c, c), yd, self.vel[2].clamp(-c, c)];
        }
        let d = self.vel;
        self.do_move(w, d);
        let mut my = self.vel[1];
        if (self.horizontal_collision || self.was_jump) && self.on_climbable(w) {
            my = 0.2;
        }
        if self.fx.levitation > 0 {
            // VANILLA: LivingEntity#travelInAir — levitation вместо гравитации.
            my += (0.05 * f64::from(self.fx.levitation) - self.vel[1]) * 0.2;
        } else if w.has_chunk(below[0], below[2]) {
            my -= self.effective_gravity();
        } else if self.pos[1] > -64.0 {
            my = -0.1;
        } else {
            my = 0.0;
        }
        let air_drag = 0.91f32;
        let friction = f64::from(block_friction * air_drag);
        self.vel = [
            self.vel[0] * friction,
            my * f64::from(0.98f32),
            self.vel[2] * friction,
        ];
    }

    /// VANILLA: AbstractClientPlayer#getFieldOfViewModifier (fovEffectScale = 1).
    pub fn fov_target(&self) -> f32 {
        let mut m = 1.0f32;
        if self.flying {
            m *= 1.1;
        }
        m * (self.speed() / MOVEMENT_SPEED + 1.0) / 2.0
    }

    fn speed(&self) -> f32 {
        // Attributes.MOVEMENT_SPEED: каждый ADD_MULTIPLIED_TOTAL множит отдельно
        // (спринт +0.3, Speed +0.2·lvl, Slowness −0.15·lvl).
        let mut v = f64::from(MOVEMENT_SPEED);
        if self.sprinting {
            v *= 1.0 + f64::from(0.3f32);
        }
        if self.fx.speed > 0 {
            v *= 1.0 + f64::from(0.2f32) * f64::from(self.fx.speed);
        }
        if self.fx.slowness > 0 {
            v *= 1.0 + f64::from(-0.15f32) * f64::from(self.fx.slowness);
        }
        v.clamp(0.0, 1024.0) as f32
    }

    /// VANILLA: LivingEntity#getEffectiveGravity (Slow Falling при vel.y <= 0).
    fn effective_gravity(&self) -> f64 {
        if self.vel[1] <= 0.0 && self.fx.slow_falling {
            GRAVITY.min(0.01)
        } else {
            GRAVITY
        }
    }

    fn flying_speed(&self) -> f32 {
        if self.flying {
            if self.sprinting {
                self.fly_speed * 2.0
            } else {
                self.fly_speed
            }
        } else if self.sprinting {
            0.025_999_999
        } else {
            0.02
        }
    }

    /// VANILLA: Entity#moveRelative / getInputVector.
    fn move_relative(&mut self, speed: f32, input: (f32, f32), yaw: f32) {
        let len = input.0 * input.0 + input.1 * input.1;
        if f64::from(len) < 1.0e-7 {
            return;
        }
        let (mut x, mut z) = input;
        if len > 1.0 {
            let l = len.sqrt();
            x /= l;
            z /= l;
        }
        x *= speed;
        z *= speed;
        let (s, c) = (yaw * (std::f32::consts::PI / 180.0)).sin_cos();
        self.vel[0] += f64::from(x * c - z * s);
        self.vel[2] += f64::from(z * c + x * s);
    }

    /// VANILLA: Entity#move (MoverType.SELF).
    fn do_move(&mut self, w: &dyn Collider, delta: [f64; 3]) {
        let before = self.pos;
        let before_xz = (self.pos[0], self.pos[2]);
        // VANILLA: Entity#move — stuckSpeedMultiplier (паутина/ягоды): движение умножается, скорость обнуляется.
        let mut delta = delta;
        if len_sq(self.stuck) > 1.0e-7 {
            for (d, k) in delta.iter_mut().zip(self.stuck) {
                *d *= k;
            }
            self.vel = [0.0; 3];
            self.stuck = [0.0; 3];
        }
        let delta = self.back_off_from_edge(w, delta);
        let m = self.collide(w, delta);
        tracing::debug!(target: "dbg", "do_move req={:.6?} res={:.6?} pos={:.6?} on_ground_before={}", delta, m, self.pos, self.on_ground);
        let ml = len_sq(m);
        if ml > 1.0e-7 || len_sq(delta) - ml < 1.0e-7 {
            for (p, d) in self.pos.iter_mut().zip(m) {
                *p += d;
            }
        }
        // LocalPlayer#move: addWalkedDistance(length(dx, dz) * 0.6)
        let (dx, dz) = (
            (self.pos[0] - before[0]) as f32,
            (self.pos[2] - before[2]) as f32,
        );
        // Отладка (dbg): AABB внутри твёрдого бокса после шага — коллизия такой бокс игнорирует (gap < -EPS).
        if tracing::enabled!(target: "dbg", tracing::Level::DEBUG) {
            let a = self.aabb();
            let region = [
                a[0] - 0.01,
                a[1] - 0.01,
                a[2] - 0.01,
                a[3] + 0.01,
                a[4] + 0.01,
                a[5] + 0.01,
            ];
            for b in gather(w, region) {
                let ov = [
                    a[3].min(b[3]) - a[0].max(b[0]),
                    a[4].min(b[4]) - a[1].max(b[1]),
                    a[5].min(b[5]) - a[2].max(b[2]),
                ];
                if ov.iter().all(|&o| o > 1e-9) {
                    tracing::debug!(
                        target: "dbg",
                        "penetrate pos={:.6?} delta={:.6?} box={:.3?} overlap={:.6?}",
                        self.pos,
                        delta,
                        b,
                        ov
                    );
                }
            }
        }
        self.walk_dist += dx.hypot(dz) * 0.6;
        // LocalPlayer#move → updateAutoJump(deltaX, deltaZ) (после обновления on_ground — см. ниже).
        let auto_delta = (
            (self.pos[0] - before_xz.0) as f32,
            (self.pos[2] - before_xz.1) as f32,
        );
        let x_col = !mth_equal(delta[0], m[0]);
        let z_col = !mth_equal(delta[2], m[2]);
        self.horizontal_collision = x_col || z_col;
        // VANILLA: Entity#move → minorHorizontalCollision = isHorizontalCollisionMinor(movement)
        self.minor_horizontal_collision = self.horizontal_collision
            && is_horizontal_collision_minor(self.move_input, [m[0], m[2]]);
        let vertical = delta[1] != m[1];
        let below = vertical && delta[1] < 0.0;
        self.on_ground = below;
        // checkFallDamage
        if self.on_ground {
            self.fall_distance = 0.0;
        } else if m[1] < 0.0 {
            self.fall_distance -= m[1];
        }
        if vertical {
            // VANILLA: Block#updateEntityMovementAfterFallOn: слизь −y·1.0, кровать −y·0.66 (если не Shift).
            let on = w.name(
                self.pos[0].floor() as i32,
                (self.pos[1] - 0.2).floor() as i32,
                self.pos[2].floor() as i32,
            );
            let k = bounce_factor(on);
            if below && k > 0.0 && !self.crouching && self.vel[1] < 0.0 {
                self.vel[1] = -self.vel[1] * k;
            } else {
                self.vel[1] = 0.0;
            }
        }
        if x_col {
            self.vel[0] = 0.0;
        }
        if z_col {
            self.vel[2] = 0.0;
        }
        let f = f64::from(block_speed_factor(w, self.pos));
        self.vel[0] *= f;
        self.vel[2] *= f;
        // Entity#applyEffectsFromBlocks: stepOn (на земле), затем entityInside.
        if self.on_ground {
            let on = w.name(
                self.pos[0].floor() as i32,
                (self.pos[1] - 0.2).floor() as i32,
                self.pos[2].floor() as i32,
            );
            // VANILLA: SlimeBlock#stepOn
            let ay = self.vel[1].abs();
            if on == "slime_block" && ay < 0.1 && !self.crouching {
                let k = 0.4 + ay * 0.2;
                self.vel[0] *= k;
                self.vel[2] *= k;
            }
        }
        self.honey_slide(w);
        self.stuck = self.inside_stuck(w);
        self.update_auto_jump(w, auto_delta.0, auto_delta.1);
    }

    /// VANILLA: LocalPlayer#updateAutoJump + canAutoJump (26.3). Jump Boost не отслеживается (jumpHeight = 1.2).
    fn update_auto_jump(&mut self, w: &dyn Collider, xa: f32, za: f32) {
        let (mx, mz) = self.last_move;
        // canAutoJump: !isPassenger не нужно (пассажира нет); isStayingOnGroundSurface = приседание.
        if !(self.auto_jump
            && self.auto_jump_time == 0
            && self.on_ground
            && !self.crouching
            && mx * mx + mz * mz > 0.0
            && block_jump_factor(w, self.pos) >= 1.0)
        {
            return;
        }
        let speed = self.speed();
        let (mut dx, mut dz) = (f64::from(xa), f64::from(za));
        let mut dist_sq = xa * xa + za * za;
        if dist_sq <= 0.001 {
            let (ixa, iza) = (speed * mx, speed * mz);
            let rad = self.last_yaw * (std::f32::consts::PI / 180.0);
            let (sin, cos) = (rad.sin(), rad.cos());
            dx = f64::from(ixa * cos - iza * sin);
            dz = f64::from(iza * cos + ixa * sin);
            dist_sq = (dx * dx + dz * dz) as f32;
            if dist_sq <= 0.001 {
                return;
            }
        }
        let inv = 1.0 / dist_sq.sqrt();
        let dir = [dx * f64::from(inv), 0.0, dz * f64::from(inv)];
        // getForward (только x/z).
        let (yr, xr) = (
            self.last_yaw * (std::f32::consts::PI / 180.0),
            self.pitch * (std::f32::consts::PI / 180.0),
        );
        let (fx, fz) = (
            f64::from(-yr.sin() * xr.cos()),
            f64::from(yr.cos() * xr.cos()),
        );
        if ((fx * dir[0] + fz * dir[2]) as f32) < -0.15 {
            return;
        }
        let bb = self.aabb();
        let solid = |x: i32, y: i32, z: i32| !w.boxes(x, y, z).is_empty();
        let (px, pz) = (self.pos[0].floor() as i32, self.pos[2].floor() as i32);
        let mut ceil_y = bb[4].floor() as i32;
        if solid(px, ceil_y, pz) {
            return;
        }
        ceil_y += 1;
        if solid(px, ceil_y, pz) {
            return;
        }
        let jump_height = 1.2f32;
        let look = (speed * 7.0).max(1.0 / inv);
        let begin = self.pos;
        let end = [
            self.pos[0] + f64::from(xa) + dir[0] * f64::from(look),
            self.pos[1],
            self.pos[2] + f64::from(za) + dir[2] * f64::from(look),
        ];
        let (pw, ph) = (f64::from(WIDTH), bb[4] - bb[1]);
        let pw_f = f64::from(pw as f32);
        let test = [
            begin[0].min(end[0]) - pw_f,
            begin[1].min(end[1]),
            begin[2].min(end[2]) - pw_f,
            begin[0].max(end[0]) + pw_f,
            begin[1].max(end[1]) + ph,
            begin[2].max(end[2]) + pw_f,
        ];
        let y51 = f64::from(0.51f32);
        // right = dir × (0,1,0) = (-dir.z, 0, dir.x)
        let off = [-dir[2] * pw_f * 0.5, dir[0] * pw_f * 0.5];
        let seg = |sx: f64, sz: f64| -> [f64; 6] {
            let (ex, ez) = (end[0] + sx, end[2] + sz);
            let (bx, bz) = (begin[0] + sx, begin[2] + sz);
            [
                bx.min(ex),
                begin[1] + y51,
                bz.min(ez),
                bx.max(ex),
                begin[1] + y51,
                bz.max(ez),
            ]
        };
        let left = seg(-off[0], -off[1]);
        let right = seg(off[0], off[1]);
        // AABB#intersects(Vec3, Vec3) — нестрогое по краям не нужно: строгое пересечение.
        let hit = |b: &[f64; 6], s: &[f64; 6]| {
            b[0] < s[3] && b[3] > s[0] && b[1] < s[4] && b[4] > s[1] && b[2] < s[5] && b[5] > s[2]
        };
        let mut obstacle = f32::MIN_POSITIVE;
        let mut ceiling = ceil_y;
        for b in gather(w, test) {
            if !(hit(&b, &left) || hit(&b, &right)) {
                continue;
            }
            obstacle = b[4] as f32;
            let (cx, cy, cz) = (
                ((b[0] + b[3]) * 0.5).floor() as i32,
                ((b[1] + b[4]) * 0.5).floor() as i32,
                ((b[2] + b[5]) * 0.5).floor() as i32,
            );
            let mut steps = 1;
            while (steps as f32) < jump_height {
                let (ay, shape) = (cy + steps, w.boxes(cx, cy + steps, cz));
                if !shape.is_empty() {
                    let max_y = shape.iter().map(|s| s[4]).fold(0.0f64, f64::max);
                    obstacle = (max_y + f64::from(ay)) as f32;
                    if obstacle - self.pos[1] as f32 > jump_height {
                        return;
                    }
                }
                if steps > 1 {
                    ceiling += 1;
                    if solid(px, ceiling, pz) {
                        return;
                    }
                }
                steps += 1;
            }
            break;
        }
        if obstacle != f32::MIN_POSITIVE {
            let ydelta = obstacle - self.pos[1] as f32;
            if ydelta > 0.5 && ydelta <= jump_height {
                self.auto_jump_time = 1;
            }
        }
    }

    /// VANILLA: HoneyBlock#entityInside → isSlidingDown/doSlideMovement.
    fn honey_slide(&mut self, w: &dyn Collider) {
        if self.on_ground {
            return;
        }
        let b = self.aabb();
        let e = 1.0e-5;
        let old_dy = self.vel[1] / f64::from(0.98_f32) + 0.08;
        let new_dy = |d: f64| (d - 0.08) * f64::from(0.98_f32);
        for x in (b[0] + e).floor() as i32..=(b[3] - e).floor() as i32 {
            for y in (b[1] + e).floor() as i32..=(b[4] - e).floor() as i32 {
                for z in (b[2] + e).floor() as i32..=(b[5] - e).floor() as i32 {
                    if w.name(x, y, z) != "honey_block" {
                        continue;
                    }
                    if self.pos[1] > f64::from(y) + 0.9375 - 1.0e-7 || old_dy >= -0.08 {
                        continue;
                    }
                    let ov = 0.4375 + f64::from(WIDTH / 2.0);
                    let dx = (f64::from(x) + 0.5 - self.pos[0]).abs();
                    let dz = (f64::from(z) + 0.5 - self.pos[2]).abs();
                    if dx + 1.0e-7 <= ov && dz + 1.0e-7 <= ov {
                        continue;
                    }
                    if old_dy < -0.13 {
                        let k = -0.05 / old_dy;
                        self.vel[0] *= k;
                        self.vel[2] *= k;
                    }
                    self.vel[1] = new_dy(-0.05);
                    self.fall_distance = 0.0;
                    return;
                }
            }
        }
    }

    /// VANILLA: Block#entityInside → makeStuckInBlock для блоков в AABB (паутина, ягодный куст).
    fn inside_stuck(&self, w: &dyn Collider) -> [f64; 3] {
        let b = self.aabb();
        let e = 1.0e-5;
        let mut out = [0.0; 3];
        for x in (b[0] + e).floor() as i32..=(b[3] - e).floor() as i32 {
            for y in (b[1] + e).floor() as i32..=(b[4] - e).floor() as i32 {
                for z in (b[2] + e).floor() as i32..=(b[5] - e).floor() as i32 {
                    let k = match w.name(x, y, z) {
                        "cobweb" => [0.25, 0.05, 0.25],
                        "sweet_berry_bush" => [0.8, 0.75, 0.8],
                        _ => continue,
                    };
                    // makeStuckInBlock: берётся более сильное замедление по осям
                    if len_sq(out) > 1.0e-7 {
                        for (o, v) in out.iter_mut().zip(k) {
                            *o = f64::min(*o, v);
                        }
                    } else {
                        out = k;
                    }
                }
            }
        }
        out
    }

    /// VANILLA: Player#maybeBackOffFromEdge (подкрадывание у края).
    fn back_off_from_edge(&self, w: &dyn Collider, d: [f64; 3]) -> [f64; 3] {
        let max_down = f64::from(STEP_HEIGHT);
        if self.flying || d[1] > 0.0 || !self.crouching || !self.above_ground(w, max_down) {
            return d;
        }
        let (mut dx, mut dz) = (d[0], d[2]);
        let (sx, sz) = (dx.signum() * 0.05, dz.signum() * 0.05);
        while dx != 0.0 && self.can_fall_at_least(w, dx, 0.0, max_down) {
            if dx.abs() <= 0.05 {
                dx = 0.0;
                break;
            }
            dx -= sx;
        }
        while dz != 0.0 && self.can_fall_at_least(w, 0.0, dz, max_down) {
            if dz.abs() <= 0.05 {
                dz = 0.0;
                break;
            }
            dz -= sz;
        }
        while dx != 0.0 && dz != 0.0 && self.can_fall_at_least(w, dx, dz, max_down) {
            dx = if dx.abs() <= 0.05 { 0.0 } else { dx - sx };
            dz = if dz.abs() <= 0.05 { 0.0 } else { dz - sz };
        }
        [dx, d[1], dz]
    }

    fn above_ground(&self, w: &dyn Collider, max_down: f64) -> bool {
        self.on_ground
            || (self.fall_distance < max_down
                && !self.can_fall_at_least(w, 0.0, 0.0, max_down - self.fall_distance))
    }

    fn can_fall_at_least(&self, w: &dyn Collider, dx: f64, dz: f64, min_h: f64) -> bool {
        let b = self.aabb();
        let q = [
            b[0] + EPS + dx,
            b[1] - min_h - EPS,
            b[2] + EPS + dz,
            b[3] - EPS + dx,
            b[1],
            b[5] - EPS + dz,
        ];
        gather(w, q).iter().all(|c| !overlaps(&q, c))
    }

    /// VANILLA: Entity#collide (со ступеньками).
    fn collide(&self, w: &dyn Collider, m: [f64; 3]) -> [f64; 3] {
        let bb = self.aabb();
        let region = expand_towards(bb, [m[0], m[1] + f64::from(STEP_HEIGHT), m[2]]);
        let region = [
            region[0],
            region[1] - 1.0,
            region[2],
            region[3],
            region[4],
            region[5],
        ];
        let mut colliders = gather(w, region);
        colliders.extend(w.entity_boxes(region));
        tracing::debug!(target: "dbg", "collide bb={:.3?} m={:.6?} boxes={:.3?}", bb, m, colliders);
        let step = if len_sq(m) == 0.0 {
            m
        } else {
            collide_with(m, bb, &colliders)
        };
        let (xc, yc, zc) = (m[0] != step[0], m[1] != step[1], m[2] != step[2]);
        let on_ground_after = yc && m[1] < 0.0;
        if STEP_HEIGHT > 0.0 && (on_ground_after || self.on_ground) && (xc || zc) {
            let grounded = if on_ground_after {
                shift(bb, 0.0, step[1], 0.0)
            } else {
                bb
            };
            let skip = step[1] as f32;
            let mut cands: Vec<f32> = Vec::new();
            for c in &colliders {
                for coord in [c[1], c[4]] {
                    let rel = (coord - grounded[1]) as f32;
                    if rel < 0.0 || rel == skip || rel > STEP_HEIGHT {
                        continue;
                    }
                    if !cands.contains(&rel) {
                        cands.push(rel);
                    }
                }
            }
            cands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            for h in cands {
                let s = collide_with([m[0], f64::from(h), m[2]], grounded, &colliders);
                if s[0] * s[0] + s[2] * s[2] > step[0] * step[0] + step[2] * step[2] {
                    let dist = bb[1] - grounded[1];
                    return [s[0], s[1] - dist, s[2]];
                }
            }
        }
        step
    }
}

/// VANILLA: LocalPlayer#modifyInput (0.98, подкрадывание, «квадратное» движение).
fn modify_input(i: (f32, f32), slow: bool, using: bool) -> (f32, f32) {
    if i.0 * i.0 + i.1 * i.1 == 0.0 {
        return i;
    }
    let mut v = (i.0 * 0.98, i.1 * 0.98);
    // VANILLA: LocalPlayer#modifyInput — itemUseSpeedMultiplier (UseEffects.DEFAULT 0.2).
    if using {
        v = (v.0 * 0.2, v.1 * 0.2);
    }
    if slow {
        v = (v.0 * SNEAKING_SPEED, v.1 * SNEAKING_SPEED);
    }
    let len = (v.0 * v.0 + v.1 * v.1).sqrt();
    if len <= 0.0 {
        return v;
    }
    let d = (v.0 / len, v.1 / len);
    let (ax, ay) = (d.0.abs(), d.1.abs());
    let tan = if ay > ax { ax / ay } else { ay / ax };
    let to_square = (1.0 + tan * tan).sqrt();
    let l = (len * to_square).min(1.0);
    (d.0 * l, d.1 * l)
}

/// VANILLA: LivingEntity#getFluidFallingAdjustedMovement (baseGravity = 0.08), только Y.
fn fluid_falling_y(y: f64, is_falling: bool, gravity: f64) -> f64 {
    if is_falling && (y - 0.005).abs() >= 0.003 && (y - gravity / 16.0).abs() < 0.003 {
        -0.003
    } else {
        y - gravity / 16.0
    }
}

/// Уровни клиентских эффектов, влияющих на физику (amplifier+1, 0 = нет).
#[derive(Clone, Copy, Debug, Default)]
pub struct EffectLevels {
    pub speed: i32,
    pub slowness: i32,
    pub jump_boost: i32,
    pub levitation: i32,
    pub slow_falling: bool,
}

#[derive(Default)]
pub(crate) struct FluidAcc {
    pub(crate) height: f64,
    cur_h: f64,
    pub(crate) cur: [f64; 3],
    pub(crate) n: u32,
}

fn mth_equal(a: f64, b: f64) -> bool {
    (b - a).abs() < 1.0e-5
}

/// VANILLA: LocalPlayer#isHorizontalCollisionMinor — угол между вводом (в мировых осях по yaw)
/// и фактическим горизонтальным движением меньше 8° (0.13962634 рад).
fn is_horizontal_collision_minor(input: (f32, f32, f32), movement: [f64; 2]) -> bool {
    let (xxa, zza, yaw) = input;
    let rad = yaw * (std::f32::consts::PI / 180.0);
    let (sin, cos) = (f64::from(rad.sin()), f64::from(rad.cos()));
    let (xa, za) = (f64::from(xxa), f64::from(zza));
    let gx = xa * cos - za * sin;
    let gz = za * cos + xa * sin;
    let a2 = gx * gx + gz * gz;
    let m2 = movement[0] * movement[0] + movement[1] * movement[1];
    if a2 < 1.0e-5 || m2 < 1.0e-5 {
        return false;
    }
    let dot = gx * movement[0] + gz * movement[1];
    (dot / (a2 * m2).sqrt()).acos() < 0.139_626_34
}

pub(crate) fn len_sq(v: [f64; 3]) -> f64 {
    v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
}

fn aabb_at(p: [f64; 3], h: f64) -> [f64; 6] {
    // VANILLA: EntityDimensions#makeBoundingBox — w = width / 2.0F (float), затем x ± w.
    let half = f64::from(WIDTH / 2.0);
    [
        p[0] - half,
        p[1],
        p[2] - half,
        p[0] + half,
        p[1] + h,
        p[2] + half,
    ]
}

fn shift(b: [f64; 6], x: f64, y: f64, z: f64) -> [f64; 6] {
    [b[0] + x, b[1] + y, b[2] + z, b[3] + x, b[4] + y, b[5] + z]
}

/// VANILLA: AABB#expandTowards.
pub(crate) fn expand_towards(b: [f64; 6], d: [f64; 3]) -> [f64; 6] {
    let mut r = b;
    for i in 0..3 {
        if d[i] < 0.0 {
            r[i] += d[i];
        } else {
            r[i + 3] += d[i];
        }
    }
    r
}

pub(crate) fn overlaps(a: &[f64; 6], b: &[f64; 6]) -> bool {
    a[0] < b[3] && a[3] > b[0] && a[1] < b[4] && a[4] > b[1] && a[2] < b[5] && a[5] > b[2]
}

/// Боксы коллизии всех блоков, пересекающих `region` (мировые координаты).
pub(crate) fn gather(w: &dyn Collider, region: [f64; 6]) -> Vec<[f64; 6]> {
    let lo = |v: f64| (v - EPS).floor() as i32 - 1;
    let hi = |v: f64| (v + EPS).floor() as i32 + 1;
    let mut out = Vec::new();
    for x in lo(region[0])..=hi(region[3]) {
        for y in lo(region[1])..=hi(region[4]) {
            for z in lo(region[2])..=hi(region[5]) {
                let extra = w.extra_boxes(x, y, z);
                let off = w.box_offset(x, y, z);
                for b in w.boxes(x, y, z).iter().chain(extra.iter()) {
                    let wb = [
                        b[0] + f64::from(x) + off[0],
                        b[1] + f64::from(y) + off[1],
                        b[2] + f64::from(z) + off[2],
                        b[3] + f64::from(x) + off[0],
                        b[4] + f64::from(y) + off[1],
                        b[5] + f64::from(z) + off[2],
                    ];
                    if overlaps(&region, &wb) {
                        out.push(wb);
                    }
                }
            }
        }
    }
    out
}

/// VANILLA: Shapes#collide по одной оси.
fn collide_axis(ax: usize, bb: [f64; 6], shapes: &[[f64; 6]], mut d: f64) -> f64 {
    if d.abs() < EPS {
        return 0.0;
    }
    let (a, b) = ((ax + 1) % 3, (ax + 2) % 3);
    for s in shapes {
        if s[a] >= bb[a + 3] - EPS || s[a + 3] <= bb[a] + EPS {
            continue;
        }
        if s[b] >= bb[b + 3] - EPS || s[b + 3] <= bb[b] + EPS {
            continue;
        }
        if d > 0.0 {
            let gap = s[ax] - bb[ax + 3];
            if gap >= -EPS {
                d = d.min(gap);
            }
        } else {
            let gap = s[ax + 3] - bb[ax];
            if gap <= EPS {
                d = d.max(gap);
            }
        }
    }
    d
}

/// VANILLA: Entity#collideWithShapes — порядок осей Direction#axisStepOrder (Y, затем большая из X/Z).
pub(crate) fn collide_with(m: [f64; 3], bb: [f64; 6], shapes: &[[f64; 6]]) -> [f64; 3] {
    if shapes.is_empty() {
        return m;
    }
    let order = if m[0].abs() < m[2].abs() {
        [1, 2, 0]
    } else {
        [1, 0, 2]
    };
    let mut r = [0.0; 3];
    for ax in order {
        if m[ax] != 0.0 {
            r[ax] = collide_axis(ax, shift(bb, r[0], r[1], r[2]), shapes, m[ax]);
        }
    }
    r
}

/// VANILLA: Entity#getOnPos(0.500001) → блок под ногами.
fn below_pos(p: [f64; 3]) -> [i32; 3] {
    [
        p[0].floor() as i32,
        (p[1] - 0.500_001).floor() as i32,
        p[2].floor() as i32,
    ]
}

fn block_pos(p: [f64; 3]) -> [i32; 3] {
    [
        p[0].floor() as i32,
        p[1].floor() as i32,
        p[2].floor() as i32,
    ]
}

/// VANILLA: Block#getFriction (Blocks.java).
pub(crate) fn block_friction_of(name: &str) -> f32 {
    match name {
        "slime_block" => 0.8,
        "ice" | "packed_ice" | "frosted_ice" => 0.98,
        "blue_ice" => 0.989,
        _ => 0.6,
    }
}

/// VANILLA: SlimeBlock/BedBlock#updateEntityMovementAfterFallOn (LivingEntity → множитель 1.0).
fn bounce_factor(name: &str) -> f64 {
    match name {
        "slime_block" => 1.0,
        n if n.ends_with("_bed") => 0.66,
        _ => 0.0,
    }
}

fn speed_factor_of(name: &str) -> f32 {
    match name {
        "soul_sand" | "honey_block" => 0.4,
        _ => 1.0,
    }
}

fn jump_factor_of(name: &str) -> f32 {
    if name == "honey_block" {
        0.5
    } else {
        1.0
    }
}

/// VANILLA: Entity#getBlockSpeedFactor.
pub(crate) fn block_speed_factor(w: &dyn Collider, p: [f64; 3]) -> f32 {
    let h = block_pos(p);
    let n = w.name(h[0], h[1], h[2]);
    let here = speed_factor_of(n);
    if n != "water" && n != "bubble_column" && here == 1.0 {
        let b = below_pos(p);
        speed_factor_of(w.name(b[0], b[1], b[2]))
    } else {
        here
    }
}

/// VANILLA: Entity#getBlockJumpFactor.
fn block_jump_factor(w: &dyn Collider, p: [f64; 3]) -> f32 {
    let h = block_pos(p);
    let here = jump_factor_of(w.name(h[0], h[1], h[2]));
    if here == 1.0 {
        let b = below_pos(p);
        jump_factor_of(w.name(b[0], b[1], b[2]))
    } else {
        here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Плоский мир: сплошной камень при y < 0 (верх на y=0), плюс дополнительные блоки.
    struct Flat {
        extra: Vec<[i32; 3]>,
    }
    const FULL: [[f64; 6]; 1] = [[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];
    impl Collider for Flat {
        fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
            if y < 0 || self.extra.contains(&[x, y, z]) {
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

    fn fwd() -> Input {
        Input {
            forward: true,
            ..Input::default()
        }
    }

    /// Упор лбом в стену (стенд владельца): стена на x∈[-12,-11], игрок на x=-10.698, идёт на -x.
    #[test]
    fn wall_hug_does_not_jitter() {
        struct Wall;
        impl Collider for Wall {
            fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
                if y < 0 || (x == -12 && (0..=2).contains(&y) && (-3..=3).contains(&z)) {
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
        let w = Wall;
        let mut p = Player::new([-10.698, 0.0, 0.0]);
        for _ in 0..5 {
            p.tick(&w, Input::default(), 90.0);
        }
        let mut xs = Vec::new();
        for _ in 0..40 {
            p.tick(&w, fwd(), 90.0);
            xs.push(p.pos[0]);
        }
        eprintln!("xs={xs:?}");
        // После упора позиция должна стоять, а не дёргаться.
        let last = &xs[20..];
        let spread = last.iter().cloned().fold(f64::MIN, f64::max)
            - last.iter().cloned().fold(f64::MAX, f64::min);
        assert!(spread < 1e-9, "дрожание: spread={spread} xs={last:?}");
    }

    struct Special(&'static str);
    impl Collider for Special {
        fn boxes(&self, _: i32, y: i32, _: i32) -> &'static [[f64; 6]] {
            if y < 0 {
                &FULL
            } else {
                &[]
            }
        }
        fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
            if y < 0 {
                self.0
            } else {
                "air"
            }
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
    }

    #[test]
    fn auto_jump_climbs_one_block_step() {
        let w = Flat {
            extra: (3..30)
                .flat_map(|z| (-1..=1).map(move |x| [x, 0, z]))
                .collect(),
        };
        let run = |auto: bool| {
            let mut p = Player::new([0.5, 0.0, 0.5]);
            p.auto_jump = auto;
            for _ in 0..40 {
                p.tick(&w, fwd(), 0.0);
            }
            p.pos
        };
        assert!(run(true)[1] >= 1.0 && run(true)[2] > 3.0);
        assert!(run(false)[1] < 0.1);
    }

    #[test]
    fn slime_bounces_and_sneak_suppresses() {
        let w = Special("slime_block");
        let mut p = Player::new([0.5, 3.0, 0.5]);
        let mut bounced = false;
        for _ in 0..40 {
            p.tick(&w, Input::default(), 0.0);
            bounced |= p.vel[1] > 0.1;
        }
        assert!(bounced);
        let mut q = Player::new([0.5, 3.0, 0.5]);
        let sneak = Input {
            sneak: true,
            ..Input::default()
        };
        for _ in 0..40 {
            q.tick(&w, sneak, 0.0);
            assert!(q.vel[1] <= 0.0);
        }
    }

    #[test]
    fn crawls_under_one_block_gap() {
        // потолок на y=1 (зазор 1.0 < 1.5): ползём
        let low = Flat {
            extra: vec![[0, 1, 0]],
        };
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.tick(&low, Input::default(), 0.0);
        assert!(p.crawling && !p.crouching);
    }

    #[test]
    fn visually_crawling_slows_input_on_first_tick_after_water() {
        // VANILLA: LocalPlayer#isMovingSlowly → Entity#isVisuallyCrawling (поза SWIMMING вне воды).
        let w = Flat { extra: vec![] };
        let fwd = Input {
            forward: true,
            sprint: true,
            ..Input::default()
        };
        let air = |pose_swimming: bool| {
            let mut p = Player::new([0.5, 10.0, 0.5]);
            p.sprinting = true;
            p.pose_swimming = pose_swimming;
            p.tick(&w, fwd, 0.0);
            p.vel[2]
        };
        let normal = air(false);
        let slowed = air(true);
        assert!(
            slowed > 0.0 && slowed < normal * 0.5,
            "{slowed} vs {normal}"
        );
    }

    #[test]
    fn falls_and_lands() {
        let w = Flat { extra: vec![] };
        let mut p = Player::new([0.5, 5.0, 0.5]);
        for _ in 0..60 {
            p.tick(&w, Input::default(), 0.0);
        }
        assert!(p.on_ground);
        assert!((p.pos[1] - 0.0).abs() < 1e-9, "y={}", p.pos[1]);
    }

    #[test]
    fn jump_peak_like_vanilla() {
        let w = Flat { extra: vec![] };
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.tick(&w, Input::default(), 0.0);
        let mut peak = 0.0f64;
        for _ in 0..30 {
            p.tick(
                &w,
                Input {
                    jump: true,
                    ..Input::default()
                },
                0.0,
            );
            peak = peak.max(p.pos[1]);
        }
        // ваниль: пик прыжка ≈ 1.2522
        assert!((peak - 1.2522).abs() < 0.01, "peak={peak}");
    }

    #[test]
    fn effects_change_speed_jump_and_gravity() {
        let w = Flat { extra: vec![] };
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.fx.speed = 2;
        p.fx.slowness = 1;
        // 0.1 · (1+0.4) · (1−0.15)
        assert!((p.speed() - 0.119).abs() < 1e-6, "{}", p.speed());
        // Jump Boost II: +0.2 к силе прыжка
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.tick(&w, Input::default(), 0.0);
        p.fx.jump_boost = 2;
        let mut vy = 0.0;
        for _ in 0..4 {
            p.tick(
                &w,
                Input {
                    jump: true,
                    ..Input::default()
                },
                0.0,
            );
            vy = p.vel[1];
            if vy > 0.0 {
                break;
            }
        }
        assert!((vy - (0.62 - 0.08) * 0.98).abs() < 1e-5, "{vy}");
        // Slow Falling: падение с гравитацией 0.01
        let mut p = Player::new([0.5, 10.0, 0.5]);
        p.fx.slow_falling = true;
        p.tick(&w, Input::default(), 0.0);
        assert!((p.vel[1] - (-0.01 * 0.98)).abs() < 1e-6, "{}", p.vel[1]);
        // Levitation I: (0.05 − 0)·0.2 = 0.01 вверх за тик
        let mut p = Player::new([0.5, 10.0, 0.5]);
        p.fx.levitation = 1;
        p.tick(&w, Input::default(), 0.0);
        assert!((p.vel[1] - 0.01 * 0.98).abs() < 1e-6, "{}", p.vel[1]);
    }

    #[test]
    fn walking_speed_and_stop_at_wall() {
        let w = Flat {
            extra: vec![[0, 0, 8], [0, 1, 8]],
        };
        let mut p = Player::new([0.5, 0.0, 0.5]);
        for _ in 0..5 {
            p.tick(&w, Input::default(), 0.0);
        }
        // 20 тиков ходьбы: ≈ 4.3 бл/с (до установившейся скорости)
        let z0 = p.pos[2];
        for _ in 0..20 {
            p.tick(&w, fwd(), 0.0);
        }
        let d = p.pos[2] - z0;
        assert!(d > 3.5 && d < 4.4, "d={d}");
        for _ in 0..40 {
            p.tick(&w, fwd(), 0.0);
        }
        // стена на z=3, ширина 0.6 → упор в z=7.7
        assert!((p.pos[2] - 7.7).abs() < 1e-6, "z={}", p.pos[2]);
        assert!(p.horizontal_collision);
    }

    #[test]
    fn steps_up_one_slab_height_but_not_full_block() {
        // ступенька высотой 0.5 (нижняя плита)
        struct Slab;
        const SLAB: [[f64; 6]; 1] = [[0.0, 0.0, 0.0, 1.0, 0.5, 1.0]];
        impl Collider for Slab {
            fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
                if y < 0 {
                    &FULL
                } else if y == 0 && z >= 2 && x == 0 {
                    &SLAB
                } else {
                    &[]
                }
            }
            fn name(&self, _: i32, _: i32, _: i32) -> &'static str {
                "stone"
            }
            fn has_chunk(&self, _: i32, _: i32) -> bool {
                true
            }
        }
        let w = Slab;
        let mut p = Player::new([0.5, 0.0, 0.5]);
        for _ in 0..60 {
            p.tick(&w, fwd(), 0.0);
        }
        assert!(p.pos[2] > 3.0, "не взошёл: z={}", p.pos[2]);
        assert!((p.pos[1] - 0.5).abs() < 1e-6, "y={}", p.pos[1]);
    }

    /// VANILLA: Shift меняет медленный ход только со следующего тика (LocalPlayer#aiStep читает
    /// keyPresses прошлого тика). Нажатие и отпускание не должны замедлять/ускорять текущий тик.
    #[test]
    fn sneak_takes_effect_one_tick_later() {
        struct Floor;
        impl Collider for Floor {
            fn boxes(&self, _: i32, y: i32, _: i32) -> &'static [[f64; 6]] {
                if y == -1 {
                    &FULL
                } else {
                    &[]
                }
            }
            fn name(&self, _: i32, _: i32, _: i32) -> &'static str {
                "stone"
            }
            fn has_chunk(&self, _: i32, _: i32) -> bool {
                true
            }
        }
        let w = Floor;
        let walk = Input {
            forward: true,
            ..Input::default()
        };
        let sneak = Input {
            sneak: true,
            ..walk
        };
        let mut a = Player::new([0.5, 0.0, 0.5]);
        let mut b = Player::new([0.5, 0.0, 0.5]);
        a.on_ground = true;
        b.on_ground = true;
        // разгон без Shift
        for _ in 0..20 {
            a.tick(&w, walk, 0.0);
            b.tick(&w, walk, 0.0);
        }
        let step = |p: &mut Player, inp: Input| {
            let z0 = p.pos[2];
            p.tick(&w, inp, 0.0);
            p.pos[2] - z0
        };
        // нажатие Shift: тик нажатия идёт на полной скорости, следующий — медленно
        let base = step(&mut b, walk);
        let press = step(&mut a, sneak);
        assert!(
            (press - base).abs() < 1e-9,
            "нажатие замедлило тот же тик: {press} vs {base}"
        );
        let press_next = step(&mut a, sneak);
        let base_next = step(&mut b, walk);
        assert!(
            press_next < base_next * 0.8,
            "со следующего тика должно быть подкрадывание: {press_next} vs {base_next}"
        );
        // отпускание: тик отпускания ещё медленный, со следующего — полная скорость
        for _ in 0..10 {
            a.tick(&w, sneak, 0.0);
            b.tick(&w, sneak, 0.0);
        }
        // копия в том же состоянии: её тик с Shift даёт базу для тика отпускания
        let mut c = a.clone();
        let release = step(&mut a, walk);
        let base_rel = step(&mut c, sneak);
        assert!(
            (release - base_rel).abs() < 1e-9,
            "отпускание ускорило тот же тик: {release} vs {base_rel}"
        );
        let release_next = step(&mut a, walk);
        let base_rel_next = step(&mut c, walk);
        assert!(
            release_next > base_rel_next * 1.2,
            "после отпускания должна быть полная скорость: {release_next} vs {base_rel_next}"
        );
    }

    #[test]
    fn sneak_does_not_fall_off_edge() {
        // платформа: x∈[0,1), z∈[0,1), остальное — пустота
        struct Plat;
        impl Collider for Plat {
            fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
                if y == -1 && x == 0 && z == 0 {
                    &FULL
                } else {
                    &[]
                }
            }
            fn name(&self, _: i32, _: i32, _: i32) -> &'static str {
                "stone"
            }
            fn has_chunk(&self, _: i32, _: i32) -> bool {
                true
            }
        }
        let w = Plat;
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.on_ground = true;
        let inp = Input {
            forward: true,
            sneak: true,
            ..Input::default()
        };
        for _ in 0..80 {
            p.tick(&w, inp, 0.0);
        }
        assert!(p.on_ground && p.pos[1] == 0.0, "упал: y={}", p.pos[1]);
        assert!(p.pos[2] < 1.4 && p.pos[2] > 0.9, "z={}", p.pos[2]);
    }

    /// Вода глубиной 5 блоков (y ∈ [-5, 0)) над дном; сверху воздух.
    struct Pool;
    impl Collider for Pool {
        fn boxes(&self, _: i32, y: i32, _: i32) -> &'static [[f64; 6]] {
            if y < -5 {
                &FULL
            } else {
                &[]
            }
        }
        fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
            if (-5..0).contains(&y) {
                "water"
            } else {
                "air"
            }
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
        fn water_height(&self, _: i32, y: i32, _: i32) -> f32 {
            if (-5..0).contains(&y) {
                1.0
            } else {
                0.0
            }
        }
    }

    #[test]
    fn sprint_in_deep_water_starts_swimming_and_follows_pitch() {
        let mut p = Player::new([0.5, -4.0, 0.5]);
        p.pitch = 45.0; // взгляд вниз
        let inp = Input {
            forward: true,
            sprint: true,
            ..Input::default()
        };
        for _ in 0..3 {
            p.tick(&Pool, inp, 0.0);
        }
        assert!(p.in_water && p.under_water);
        assert!(p.swimming, "должен плыть");
        assert_eq!(p.eye_height(), EYE_SWIMMING);
        let y0 = p.pos[1];
        for _ in 0..10 {
            p.tick(&Pool, inp, 0.0);
        }
        assert!(p.pos[1] < y0, "взгляд вниз тянет вниз");
        // без спринта — обычная вода: не плывём
        let mut q = Player::new([0.5, -4.0, 0.5]);
        for _ in 0..3 {
            q.tick(&Pool, fwd(), 0.0);
        }
        assert!(q.in_water && !q.swimming);
    }

    /// Вода с течением на +X, дна нет.
    struct River;
    impl Collider for River {
        fn boxes(&self, _: i32, _: i32, _: i32) -> &'static [[f64; 6]] {
            &[]
        }
        fn name(&self, _: i32, _: i32, _: i32) -> &'static str {
            "water"
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
        fn water_height(&self, _: i32, _: i32, _: i32) -> f32 {
            1.0
        }
        fn water_flow(&self, _: i32, _: i32, _: i32) -> [f64; 3] {
            [1.0, 0.0, 0.0]
        }
    }

    #[test]
    fn water_current_pushes_player() {
        let mut p = Player::new([0.5, 0.0, 0.5]);
        for _ in 0..20 {
            p.tick(&River, Input::default(), 0.0);
        }
        assert!(p.pos[0] > 0.5 && p.vel[0] > 0.0, "течение сносит по +X");
        let mut f = Player::new([0.5, 0.0, 0.5]);
        f.flying = true;
        for _ in 0..20 {
            f.tick(&River, Input::default(), 0.0);
        }
        assert!(f.pos[0] < p.pos[0], "в полёте течение слабее/нет");
    }

    /// Лавовый бассейн (y -5..0) с дном.
    struct LavaPool;
    impl Collider for LavaPool {
        fn boxes(&self, _: i32, y: i32, _: i32) -> &'static [[f64; 6]] {
            if y < -5 {
                &FULL
            } else {
                &[]
            }
        }
        fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
            if (-5..0).contains(&y) {
                "lava"
            } else {
                "air"
            }
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
        fn lava_height(&self, _: i32, y: i32, _: i32) -> f32 {
            if (-5..0).contains(&y) {
                1.0
            } else {
                0.0
            }
        }
    }

    #[test]
    fn lava_slows_and_jump_floats_up() {
        let mut p = Player::new([0.5, -3.0, 0.5]);
        p.tick(&LavaPool, Input::default(), 0.0);
        assert!(p.in_lava && !p.in_water);
        // падение в лаве медленное (×0.5 и gravity/4), не как в воздухе
        for _ in 0..10 {
            p.tick(&LavaPool, Input::default(), 0.0);
        }
        assert!(p.vel[1] > -0.1, "лава тормозит: {}", p.vel[1]);
        let y0 = p.pos[1];
        let j = Input {
            jump: true,
            ..Input::default()
        };
        for _ in 0..20 {
            p.tick(&LavaPool, j, 0.0);
        }
        assert!(p.pos[1] > y0, "прыжок всплывает в лаве");
    }

    /// Лестница в x=0 (воздух вокруг), под y=-1 пол.
    struct Ladder;
    impl Collider for Ladder {
        fn boxes(&self, _: i32, y: i32, _: i32) -> &'static [[f64; 6]] {
            if y < 0 {
                &FULL
            } else {
                &[]
            }
        }
        fn name(&self, _: i32, y: i32, _: i32) -> &'static str {
            if (0..10).contains(&y) {
                "ladder"
            } else {
                "air"
            }
        }
        fn has_chunk(&self, _: i32, _: i32) -> bool {
            true
        }
    }

    #[test]
    fn ladder_limits_fall_and_jump_climbs() {
        let mut p = Player::new([0.5, 5.0, 0.5]);
        let mut worst = 0.0f64;
        for _ in 0..30 {
            let y = p.pos[1];
            p.tick(&Ladder, Input::default(), 0.0);
            worst = worst.min(p.pos[1] - y);
        }
        assert!(worst >= -0.15 - 1e-6, "скольжение ≤0.15/тик: {worst}");
        let y0 = p.pos[1];
        let j = Input {
            jump: true,
            ..Input::default()
        };
        for _ in 0..10 {
            p.tick(&Ladder, j, 0.0);
        }
        assert!(p.pos[1] > y0, "прыжок лезет вверх");
    }

    #[test]
    fn double_jump_toggles_flight() {
        let w = Flat { extra: vec![] };
        let mut p = Player::new([0.5, 0.0, 0.5]);
        p.mayfly = true;
        let j = Input {
            jump: true,
            ..Input::default()
        };
        p.tick(&w, Input::default(), 0.0);
        p.tick(&w, j, 0.0);
        p.tick(&w, Input::default(), 0.0);
        p.tick(&w, j, 0.0);
        assert!(p.flying);
    }
}

#[cfg(test)]
mod minor_collision_tests {
    use super::*;

    /// Ввод вперёд (yaw 0 → движение по +Z). Скольжение почти параллельно стене — minor.
    #[test]
    fn slide_within_8_degrees_is_minor() {
        let input = (0.0, 1.0, 0.0);
        assert!(is_horizontal_collision_minor(input, [0.02, 0.5]));
        assert!(!is_horizontal_collision_minor(input, [0.1, 0.5]));
    }

    /// Полный упор (нулевое движение) — не minor: обычный горизонтальный упор.
    #[test]
    fn full_stop_is_not_minor() {
        assert!(!is_horizontal_collision_minor((0.0, 1.0, 0.0), [0.0, 0.0]));
    }

    /// Игрок падает на бокс сущности (лодка) и стоит на нём без проваливания (сетбэк-баг).
    #[test]
    fn stands_on_entity_box() {
        struct BoatWorld;
        impl Collider for BoatWorld {
            fn boxes(&self, _: i32, _: i32, _: i32) -> &'static [[f64; 6]] {
                &[]
            }
            fn name(&self, _: i32, _: i32, _: i32) -> &'static str {
                "air"
            }
            fn has_chunk(&self, _: i32, _: i32) -> bool {
                true
            }
            fn entity_boxes(&self, region: [f64; 6]) -> Vec<[f64; 6]> {
                let bb = [-0.6875, 0.0, -0.6875, 0.6875, 0.5625, 0.6875];
                if overlaps(&region, &bb) {
                    vec![bb]
                } else {
                    Vec::new()
                }
            }
        }
        let w = BoatWorld;
        let mut p = Player::new([0.0, 2.0, 0.0]);
        for _ in 0..60 {
            p.tick(&w, Input::default(), 0.0);
        }
        assert!(p.on_ground, "не стоит: y={}", p.pos[1]);
        assert!((p.pos[1] - 0.5625).abs() < 1e-6, "y={}", p.pos[1]);
    }

    /// `Level::entity_boxes`: чужая лодка даёт бокс, своя (под игроком) и не-лодки — нет.
    #[test]
    fn level_entity_boxes_boats_only() {
        let mut l = Level::new(
            ezzmc_world::chunk::ChunkFormat {
                min_y: 0,
                height: 256,
                block_global_bits: 15,
                biome_global_bits: 6,
            },
            2,
        );
        let boat = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "oak_boat")
            .unwrap_or(0);
        let cow = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "cow")
            .unwrap_or(0);
        let mk = |id, ty, pos| {
            ezzmc_world::entities::Entity::new(id, id as u128, ty, pos, 0.0, 0.0, 0.0)
        };
        l.entities.add(mk(10, boat, [5.0, 64.0, 5.0]));
        l.entities.add(mk(11, boat, [8.0, 64.0, 5.0]));
        l.entities.add(mk(12, cow, [5.0, 64.0, 5.0]));
        let region = [3.0, 63.0, 3.0, 7.0, 66.0, 7.0];
        let b = Collider::entity_boxes(&l, region);
        assert_eq!(b.len(), 1, "{b:?}");
        assert!((b[0][3] - b[0][0] - 1.375).abs() < 1e-9 && (b[0][4] - 64.0 - 0.5625).abs() < 1e-6);
        // своя лодка исключается
        l.own_id = 1;
        l.riders.vehicle_of.insert(1, 10);
        assert!(Collider::entity_boxes(&l, region).is_empty());
    }
}
