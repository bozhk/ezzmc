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
/// VANILLA: Abilities#getFlyingSpeed.
const WIDTH: f64 = 0.6;
const HEIGHT_STANDING: f64 = 1.8;
const HEIGHT_CROUCHING: f64 = 1.5;
const EYE_STANDING: f64 = 1.62;
const EYE_CROUCHING: f64 = 1.27;
/// VANILLA: Pose.SWIMMING — хитбокс 0.6×0.6, глаза 0.4.
const HEIGHT_SWIMMING: f64 = 0.6;
const EYE_SWIMMING: f64 = 0.4;

/// Доступ к миру для физики (позволяет тестировать без чанков).
pub trait Collider {
    /// Боксы коллизии блока `[x0,y0,z0,x1,y1,z1]` в координатах блока.
    fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]];
    /// Имя блока без пространства имён (`"air"` для пустоты).
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str;
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
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str {
        self.chunks.block(x, y, z).name()
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
    fn lava_height(&self, x: i32, y: i32, z: i32) -> f32 {
        fluid_height_of(self, x, y, z, true)
    }
    fn lava_flow(&self, x: i32, y: i32, z: i32) -> [f64; 3] {
        fluid_flow_of(self, x, y, z, true)
    }
}

fn fluid_height_of(level: &Level, x: i32, y: i32, z: i32, lava: bool) -> f32 {
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
    /// VANILLA: KeyboardInput#calculateImpulse → Vec2(left, forward).
    fn move_vector(self) -> (f32, f32) {
        let f = |p: bool, m: bool| f32::from(u8::from(p)) - f32::from(u8::from(m));
        (f(self.left, self.right), f(self.forward, self.back))
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: [f64; 3],
    pub prev_pos: [f64; 3],
    pub vel: [f64; 3],
    pub on_ground: bool,
    pub horizontal_collision: bool,
    pub crouching: bool,
    /// VANILLA: Pose.SWIMMING вне воды — ползание под 1.5-блочным потолком (isVisuallyCrawling).
    pub crawling: bool,
    pub sprinting: bool,
    pub flying: bool,
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
    swap_ticker: u32,
    /// VANILLA: Entity#getFluidHeight(WATER) по AABB.
    fluid_h: f64,
    lava_h: f64,
    pub mayfly: bool,
    /// VANILLA: Abilities#flyingSpeed (по умолчанию 0.05).
    pub fly_speed: f32,
    pub fall_distance: f64,
    /// VANILLA: Entity#stuckSpeedMultiplier.
    stuck: [f64; 3],
    no_jump_delay: u8,
    jump_trigger_time: u8,
    sprint_trigger_time: u8,
    was_jump: bool,
    had_forward: bool,
    /// VANILLA: ClientAvatarState walkDist/walkDistO/bob/bobO (боббинг камеры).
    walk_dist: f32,
    walk_dist_o: f32,
    bob: f32,
    bob_o: f32,
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
            crouching: false,
            crawling: false,
            sprinting: false,
            flying: false,
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
            swap_ticker: 0,
            fluid_h: 0.0,
            lava_h: 0.0,
            mayfly: false,
            fly_speed: 0.05,
            fall_distance: 0.0,
            stuck: [0.0; 3],
            no_jump_delay: 0,
            jump_trigger_time: 0,
            sprint_trigger_time: 0,
            was_jump: false,
            had_forward: false,
            walk_dist: 0.0,
            walk_dist_o: 0.0,
            bob: 0.0,
            bob_o: 0.0,
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
            HEIGHT_SWIMMING
        } else if self.crouching {
            HEIGHT_CROUCHING
        } else {
            HEIGHT_STANDING
        }
    }

    fn aabb(&self) -> [f64; 6] {
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

    /// После `tick`: камера и анимация аватара. VANILLA: Camera#tick (eyeHeight += (target − eyeHeight) · 0.5)
    /// и LivingEntity#tick (тело/голова/ходьба/взмах).
    pub fn post_tick(&mut self, yaw: f32, pitch: f32) {
        self.eye_h_o = self.eye_h;
        self.eye_h += (self.eye_height() - self.eye_h) * 0.5;
        self.anim.swimming = self.swimming;
        self.anim.in_water = self.in_water;
        self.anim.tick(self.pos, self.prev_pos, yaw, pitch);
        // LocalPlayer#aiStep: xBob/yBob += (rot − bob)·0.5.
        self.y_bob_o = self.y_bob;
        self.x_bob_o = self.x_bob;
        self.x_bob += (pitch - self.x_bob) * 0.5;
        self.y_bob += (yaw - self.y_bob) * 0.5;
        // Player#aiStep: itemSwapTicker++; FirstPersonHandsAndItems#tick (пустая рука, delay = 5 тиков).
        self.swap_ticker += 1;
        self.hand_h_o = self.hand_h;
        let scale = ((self.swap_ticker as f32 + 1.0) / 5.0).clamp(0.0, 1.0);
        let target = scale * scale * scale;
        self.hand_h += (target - self.hand_h).clamp(-0.4, 0.4);
    }

    /// VANILLA: LivingEntity#swing(MAIN_HAND).
    pub fn swing(&mut self) {
        // MultiPlayerGameMode/Minecraft#startAttack: resetAttackStrengthTicker (itemSwapTicker = 0).
        self.swap_ticker = 0;
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
        let had_forward = self.had_forward;
        let (left, fwd) = input.move_vector();
        self.had_forward = fwd > 1.0e-5;

        // crouching: pose
        let want_crouch = !self.flying && !self.swimming && input.sneak;
        let stand_fits = self.fits(w, HEIGHT_STANDING);
        // Player#updatePlayerPose: не влезаем в стоя/присед → поза SWIMMING (ползём).
        let want = !self.flying && !self.swimming && (want_crouch || !stand_fits);
        self.crawling =
            !self.flying && !self.swimming && !stand_fits && !self.fits(w, HEIGHT_CROUCHING);
        self.crouching = want && !self.crawling;

        // спринт
        if input.sneak || input.back {
            self.sprint_trigger_time = 0;
        }
        let moving_slowly = self.crouching || self.crawling;
        let shallow = self.in_water && !self.under_water;
        if !self.sprinting
            && fwd >= 0.8
            && (!shallow || self.flying)
            && (!moving_slowly || self.under_water)
        {
            if !had_forward {
                if self.sprint_trigger_time > 0 {
                    self.sprinting = true;
                } else {
                    // VANILLA: options.sprintWindow (по умолчанию 7)
                    self.sprint_trigger_time = 7;
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
                moving_slowly
                    || (shallow && !self.flying)
                    || fwd <= 1.0e-5
                    || self.horizontal_collision
            };
            if stop {
                self.sprinting = false;
            }
        }

        // творческий полёт: двойное нажатие прыжка
        if self.mayfly && !was_jump && input.jump {
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
        let (xxa, zza) = modify_input((left, fwd), moving_slowly);

        // LocalPlayer#aiStep: sneak в воде → goDownInWater.
        if self.in_water && input.sneak {
            self.vel[1] -= f64::from(0.04f32);
        }
        // прыжок. VANILLA: LivingEntity#aiStep (jumpInLiquid / порог getFluidJumpThreshold = 0.4).
        if input.jump {
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
        if self.in_water {
            self.travel_in_water(w, (xxa, zza), yaw);
        } else if self.in_lava {
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
            v[1] = fluid_falling_y(v[1], is_falling);
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
                self.vel[1] = fluid_falling_y(self.vel[1], is_falling);
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
        let b = aabb_at(self.pos, height);
        // VANILLA: canPlayerFitWithinBlocksAndEntitiesWhen — без пересечения с коллизиями.
        gather(w, b).iter().all(|c| !overlaps(&b, c))
    }

    /// VANILLA: LivingEntity#getJumpPower + jumpFromGround.
    fn jump_from_ground(&mut self, w: &dyn Collider, sprinting: bool, yaw: f32) {
        let power = JUMP_STRENGTH * block_jump_factor(w, self.pos);
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
        if w.has_chunk(below[0], below[2]) {
            my -= GRAVITY;
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
        // Attributes.MOVEMENT_SPEED + модификатор спринта ADD_MULTIPLIED_TOTAL 0.3
        if self.sprinting {
            (f64::from(MOVEMENT_SPEED) * 1.3) as f32
        } else {
            MOVEMENT_SPEED
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
        self.walk_dist += dx.hypot(dz) * 0.6;
        let x_col = !mth_equal(delta[0], m[0]);
        let z_col = !mth_equal(delta[2], m[2]);
        self.horizontal_collision = x_col || z_col;
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
                    let ov = 0.4375 + WIDTH / 2.0;
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
        let colliders = gather(w, region);
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
fn modify_input(i: (f32, f32), slow: bool) -> (f32, f32) {
    if i.0 * i.0 + i.1 * i.1 == 0.0 {
        return i;
    }
    let mut v = (i.0 * 0.98, i.1 * 0.98);
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
fn fluid_falling_y(y: f64, is_falling: bool) -> f64 {
    if is_falling && (y - 0.005).abs() >= 0.003 && (y - GRAVITY / 16.0).abs() < 0.003 {
        -0.003
    } else {
        y - GRAVITY / 16.0
    }
}

#[derive(Default)]
struct FluidAcc {
    height: f64,
    cur_h: f64,
    cur: [f64; 3],
    n: u32,
}

fn mth_equal(a: f64, b: f64) -> bool {
    (b - a).abs() < 1.0e-5
}

fn len_sq(v: [f64; 3]) -> f64 {
    v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
}

fn aabb_at(p: [f64; 3], h: f64) -> [f64; 6] {
    [
        p[0] - WIDTH / 2.0,
        p[1],
        p[2] - WIDTH / 2.0,
        p[0] + WIDTH / 2.0,
        p[1] + h,
        p[2] + WIDTH / 2.0,
    ]
}

fn shift(b: [f64; 6], x: f64, y: f64, z: f64) -> [f64; 6] {
    [b[0] + x, b[1] + y, b[2] + z, b[3] + x, b[4] + y, b[5] + z]
}

/// VANILLA: AABB#expandTowards.
fn expand_towards(b: [f64; 6], d: [f64; 3]) -> [f64; 6] {
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

fn overlaps(a: &[f64; 6], b: &[f64; 6]) -> bool {
    a[0] < b[3] && a[3] > b[0] && a[1] < b[4] && a[4] > b[1] && a[2] < b[5] && a[5] > b[2]
}

/// Боксы коллизии всех блоков, пересекающих `region` (мировые координаты).
fn gather(w: &dyn Collider, region: [f64; 6]) -> Vec<[f64; 6]> {
    let lo = |v: f64| (v - EPS).floor() as i32 - 1;
    let hi = |v: f64| (v + EPS).floor() as i32 + 1;
    let mut out = Vec::new();
    for x in lo(region[0])..=hi(region[3]) {
        for y in lo(region[1])..=hi(region[4]) {
            for z in lo(region[2])..=hi(region[5]) {
                for b in w.boxes(x, y, z) {
                    let wb = [
                        b[0] + f64::from(x),
                        b[1] + f64::from(y),
                        b[2] + f64::from(z),
                        b[3] + f64::from(x),
                        b[4] + f64::from(y),
                        b[5] + f64::from(z),
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
fn collide_with(m: [f64; 3], bb: [f64; 6], shapes: &[[f64; 6]]) -> [f64; 3] {
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
fn block_friction_of(name: &str) -> f32 {
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
fn block_speed_factor(w: &dyn Collider, p: [f64; 3]) -> f32 {
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
