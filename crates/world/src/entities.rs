//! Клиентское хранилище сущностей (TODO-203/207). VANILLA: ClientLevel#entityStorage,
//! ClientPacketListener#handleAddEntity/handleMoveEntity/handleTeleportEntity/
//! handleEntityPositionSync/handleRotateEntity/handleRemoveEntities (26.3),
//! Entity#lerpPositionAndRotationStep (InterpolationHandler, 3 шага).
use std::collections::HashMap;

/// VANILLA: Mth#unpackDegrees.
pub fn unpack_degrees(b: i8) -> f32 {
    f32::from(b) * 360.0 / 256.0
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: i32,
    pub uuid: u128,
    /// Индекс в реестре `entity_type`.
    pub ty: usize,
    pub pos: [f64; 3],
    pub prev_pos: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub prev_yaw: f32,
    pub prev_pitch: f32,
    pub head_yaw: f32,
    pub prev_head_yaw: f32,
    pub on_ground: bool,
    /// Общие флаги (метаданные #0): 1 = присед, 8 = спринт, 16 = плавание.
    pub flags: u8,
    /// Метаданные мобов (индекс по ванили): Mob#15 флаги, AgeableMob#16 детёныш, Creeper#16 swellDir,
    /// #17 powered, Sheep#18 цвет|стрижка. Какой смысл — решает тип сущности.
    pub mob_flags: u8,
    pub baby: bool,
    pub swell_dir: i32,
    pub bool17: bool,
    pub byte18: u8,
    /// VarInt-метаданные мобов (индекс ≥16): INT и реестровые холдеры (вариант кота/лягушки…).
    pub ints: Vec<(u8, i32)>,
    /// Метаданные #16 типа BYTE (Bat#DATA_ID_FLAGS: бит 1 = висит).
    pub byte16: u8,
    /// Стак предмета (метаданные #8 у `item`).
    pub item: Option<ezzmc_protocol::item::ItemStack>,
    /// Entity#tickCount.
    pub age: u32,
    /// Entity#deltaMovement (физику ведёт клиент: предметы).
    pub vel: [f64; 3],
    /// Нет интерполяции: пакет позиции ставит позицию сразу (ItemEntity: getInterpolation()==null).
    pub snap: bool,
    /// PrimedTnt#DATA_FUSE_ID (метаданные #8, INT); -1 — не взведённый TNT.
    pub fuse: i32,
    /// Снаряд, который клиент ведёт сам: 0 нет, 1 стрела, 2 бросаемый предмет.
    pub proj: u8,
    /// Гравитация снаряда за тик.
    pub gravity: f64,
    /// Снаряд упёрся в блок (стоит до пакета движения).
    pub stuck: bool,
    /// Стрела: id владельца (поле `data` пакета add_entity), 0 — нет.
    pub owner: i32,
    /// Поле `data` пакета add_entity (падающий блок: id состояния блока).
    pub data: i32,
    /// AbstractArrow#ID_FLAGS (1 — крит, 2 — noPhysics).
    pub arrow_flags: u8,
    /// AbstractArrow#shakeTime (7 после попадания в блок).
    pub shake: i32,
    /// Projectile#leftOwner.
    pub left_owner: bool,
    /// Клиент сам убрал снаряд (попал в другого игрока).
    pub dead: bool,
    /// Предмет в основной/второй руке (set_equipment).
    pub held: Option<ezzmc_protocol::item::ItemStack>,
    pub held_off: Option<ezzmc_protocol::item::ItemStack>,
    /// Броня (set_equipment): голова, грудь, ноги, ступни.
    pub armor: [Option<ezzmc_protocol::item::ItemStack>; 4],
    /// Счётчик взмахов (animate 0/3); рендер сравнивает с виденным.
    pub swing_seq: u32,
    /// Момент последнего урона (entity_event 2/33/36/37/44/57 → hurtTime=10; 3 — смерть).
    pub hurt_at: Option<std::time::Instant>,
    /// Момент смерти (entity_event 3): deathTime растёт 0→20 тиков, потом сервер убирает сущность.
    pub death_at: Option<std::time::Instant>,
    /// Длительность красного оверлея, сек (10 тиков при уроне, 20 при смерти).
    pub hurt_dur: f32,
    /// Squid#handleEntityEvent(19): счётчик сбросов tentacleMovement.
    pub squid_reset: u32,
    /// Счётчик crit-анимаций (animate 4/5): 1 — обычный, 2 — магический.
    pub crit_seq: (u32, u8),
    steps: u32,
    target: [f64; 3],
    target_rot: [f32; 3],
}

/// Число шагов интерполяции по умолчанию (VANILLA: InterpolationHandler(entity, 3)).
const LERP_STEPS: u32 = 3;

impl Entity {
    /// VarInt-метаданная по индексу (вариант/окрас), если приходила.
    pub fn int_meta(&self, idx: u8) -> Option<i32> {
        self.ints.iter().find(|(i, _)| *i == idx).map(|(_, v)| *v)
    }

    pub fn new(
        id: i32,
        uuid: u128,
        ty: usize,
        pos: [f64; 3],
        yaw: f32,
        pitch: f32,
        head: f32,
    ) -> Self {
        let (proj, gravity) = projectile_kind(ty);
        Entity {
            id,
            uuid,
            ty,
            pos,
            prev_pos: pos,
            yaw,
            pitch,
            prev_yaw: yaw,
            prev_pitch: pitch,
            head_yaw: head,
            prev_head_yaw: head,
            on_ground: false,
            flags: 0,
            mob_flags: 0,
            baby: false,
            swell_dir: 0,
            bool17: false,
            byte18: 0,
            ints: Vec::new(),
            byte16: 0,
            item: None,
            age: 0,
            vel: [0.0; 3],
            snap: false,
            fuse: -1,
            proj,
            gravity,
            stuck: false,
            owner: 0,
            data: 0,
            arrow_flags: 0,
            shake: 0,
            left_owner: false,
            dead: false,
            held: None,
            held_off: None,
            armor: [None, None, None, None],
            swing_seq: 0,
            hurt_at: None,
            hurt_dur: 0.0,
            squid_reset: 0,
            death_at: None,
            crit_seq: (0, 0),
            steps: 0,
            target: pos,
            target_rot: [yaw, pitch, head],
        }
    }

    /// Идёт ли интерполяция к позиции из пакета.
    pub fn interpolating(&self) -> bool {
        self.steps > 0
    }

    /// VANILLA: AbstractArrow#tick / ThrowableProjectile#tick (клиентская часть): полёт по скорости,
    /// сопротивление 0.99, гравитация; останов в блоке (`solid` — точка внутри коллизии блока).
    /// Стрела, попавшая в другого игрока (`others`: id, хитбокс), убирается: RemotePlayer#hurtClient
    /// возвращает true → AbstractArrow#onHitEntity → discard.
    // VANILLA-UNSURE: вода (сопротивление 0.6/0.8), попадания в мобы (клиентский deflect) и в
    // локального игрока не учитываются; leftOwner упрощён (владелец вне хитбокса стрелы).
    #[allow(clippy::type_complexity)]
    pub fn tick_projectile(
        &mut self,
        solid: &dyn Fn(f64, f64, f64) -> bool,
        others: &[(i32, ([f64; 3], [f64; 3]))],
    ) {
        if self.proj == 0 {
            return;
        }
        if self.shake > 0 {
            self.shake -= 1;
        }
        if self.stuck || self.vel == [0.0; 3] {
            return;
        }
        // VANILLA: ThrowableProjectile#tick — сначала applyGravity + applyInertia (0.99), потом
        // движение (у стрелы наоборот: движение, затем drag и гравитация).
        if self.proj == 2 {
            self.vel = [
                self.vel[0] * 0.99,
                (self.vel[1] - self.gravity) * 0.99,
                self.vel[2] * 0.99,
            ];
        }
        let v = self.vel;
        let mut last = self.pos;
        let mut hit = false;
        for i in 1..=8 {
            let k = f64::from(i) / 8.0;
            let p = [
                self.pos[0] + v[0] * k,
                self.pos[1] + v[1] * k,
                self.pos[2] + v[2] * k,
            ];
            if solid(p[0], p[1], p[2]) {
                hit = true;
                break;
            }
            last = p;
        }
        if self.proj == 1 {
            // VANILLA: ProjectileUtil#computeMargin = clamp((tickCount − 2)/20, 0, 0.3).
            let margin = ((f64::from(self.age) - 2.0) / 20.0).clamp(0.0, 0.3);
            for (id, (mn, mx)) in others {
                let inflated = |m: f64| {
                    (
                        [mn[0] - m, mn[1] - m, mn[2] - m],
                        [mx[0] + m, mx[1] + m, mx[2] + m],
                    )
                };
                if *id == self.owner && !self.left_owner {
                    // Владелец ещё не покинут (хитбокс стрелы 0.5).
                    let (a, b) = inflated(0.25);
                    let inside = (0..3).all(|k| self.pos[k] >= a[k] && self.pos[k] <= b[k]);
                    if inside {
                        continue;
                    }
                    self.left_owner = true;
                }
                let (a, b) = inflated(margin);
                if segment_hits_box(self.pos, last, a, b) {
                    self.dead = true;
                    return;
                }
            }
        }
        self.pos = last;
        if hit {
            // VANILLA: AbstractArrow#onHitBlock — отступ 0.05 против хода, shakeTime = 7.
            for (p, vk) in self.pos.iter_mut().zip(v.iter()) {
                *p -= vk.signum() * 0.05 * f64::from(u8::from(*vk != 0.0));
            }
            self.stuck = true;
            self.vel = [0.0; 3];
            if self.proj == 1 {
                self.shake = 7;
                self.arrow_flags &= !1;
            }
            return;
        }
        if self.proj == 1 {
            let h = v[0].hypot(v[2]);
            let yaw = v[0].atan2(v[2]).to_degrees() as f32;
            let pitch = v[1].atan2(h).to_degrees() as f32;
            self.yaw += wrap(yaw - self.yaw) * 0.2;
            self.pitch += wrap(pitch - self.pitch) * 0.2;
        }
        if self.proj == 1 {
            self.vel = [v[0] * 0.99, v[1] * 0.99 - self.gravity, v[2] * 0.99];
        }
    }

    /// Крит-стрела в полёте (частицы CRIT каждый тик).
    pub fn crit_arrow(&self) -> bool {
        self.proj == 1 && self.arrow_flags & 1 != 0 && !self.stuck
    }

    /// VANILLA: Entity#isShiftKeyDown (общий флаг 1 → бит 2).
    pub fn crouching(&self) -> bool {
        self.flags & 2 != 0
    }

    /// VANILLA: Entity#isFallFlying (общий флаг 7 → бит 128).
    pub fn fall_flying(&self) -> bool {
        self.flags & 128 != 0
    }

    /// VANILLA: Entity#isInvisible (общий флаг 5 → бит 32).
    pub fn invisible(&self) -> bool {
        self.flags & 32 != 0
    }

    /// VANILLA: Entity#isSwimming (общий флаг 4 → бит 16).
    pub fn swimming(&self) -> bool {
        self.flags & 16 != 0
    }

    /// VANILLA: InterpolationHandler#interpolateTo.
    pub fn set_target(&mut self, pos: Option<[f64; 3]>, rot: Option<(f32, f32)>) {
        // VANILLA: Entity#moveOrInterpolateTo — у снарядов (стрела, жемчуг, снежок…) обработчик
        // интерполяции NO_OP: пакет сервера просто ставит позицию/поворот (setPos/setRot, без сброса
        // xOld), дальше клиент сам считает полёт. Лерп к цели поверх своей физики давал рывки.
        let direct = self.snap || self.proj != 0;
        if let Some(p) = pos {
            self.target = p;
            if direct {
                self.pos = p;
            }
        }
        if let Some((y, x)) = rot {
            self.target_rot[0] = y;
            self.target_rot[1] = x;
            if self.proj != 0 {
                self.yaw = y;
                self.pitch = x;
            }
        }
        self.steps = if direct { 0 } else { LERP_STEPS };
    }

    pub fn target_pos(&self) -> [f64; 3] {
        self.target
    }

    pub fn target_yaw_pitch(&self) -> (f32, f32) {
        (self.target_rot[0], self.target_rot[1])
    }

    pub fn set_head_target(&mut self, head: f32) {
        self.target_rot[2] = head;
        self.steps = self.steps.max(LERP_STEPS);
    }

    /// Один клиентский тик (xOld = x; затем шаг интерполяции).
    pub fn tick(&mut self) {
        self.age = self.age.wrapping_add(1);
        // VANILLA: PrimedTnt#tick — запал убывает каждый тик (клиент тоже).
        if self.fuse >= 0 {
            self.fuse -= 1;
        }
        self.prev_pos = self.pos;
        self.prev_yaw = self.yaw;
        self.prev_pitch = self.pitch;
        self.prev_head_yaw = self.head_yaw;
        if self.steps > 0 {
            let k = 1.0 / f64::from(self.steps);
            for i in 0..3 {
                self.pos[i] += (self.target[i] - self.pos[i]) * k;
            }
            let kf = k as f32;
            self.yaw += wrap(self.target_rot[0] - self.yaw) * kf;
            self.pitch += (self.target_rot[1] - self.pitch) * kf;
            self.head_yaw += wrap(self.target_rot[2] - self.head_yaw) * kf;
            self.steps -= 1;
        }
    }
}

/// Пересечение отрезка [a, b] с AABB (slab-метод).
fn segment_hits_box(a: [f64; 3], b: [f64; 3], mn: [f64; 3], mx: [f64; 3]) -> bool {
    let (mut t0, mut t1) = (0.0_f64, 1.0_f64);
    for k in 0..3 {
        let d = b[k] - a[k];
        if d.abs() < 1e-12 {
            if a[k] < mn[k] || a[k] > mx[k] {
                return false;
            }
        } else {
            let (mut ta, mut tb) = ((mn[k] - a[k]) / d, (mx[k] - a[k]) / d);
            if ta > tb {
                std::mem::swap(&mut ta, &mut tb);
            }
            t0 = t0.max(ta);
            t1 = t1.min(tb);
            if t0 > t1 {
                return false;
            }
        }
    }
    true
}

fn wrap(d: f32) -> f32 {
    let mut d = d % 360.0;
    if d >= 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0;
    }
    d
}

/// Тип снаряда по индексу `entity_type`: (вид, гравитация).
fn projectile_kind(ty: usize) -> (u8, f64) {
    match ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .get(ty)
        .copied()
    {
        Some("arrow" | "spectral_arrow") => (1, 0.05),
        Some("ender_pearl" | "snowball" | "egg" | "blue_egg" | "brown_egg") => (2, 0.03),
        Some("splash_potion" | "lingering_potion") => (2, 0.05),
        Some("experience_bottle") => (2, 0.07),
        _ => (0, 0.0),
    }
}

/// Индекс `player` в реестре `entity_type` (26.3).
pub fn player_type() -> Option<usize> {
    static T: std::sync::LazyLock<Option<usize>> = std::sync::LazyLock::new(|| {
        ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "player")
    });
    *T
}

#[derive(Default)]
pub struct Entities {
    pub map: HashMap<i32, Entity>,
}

impl Entities {
    pub fn add(&mut self, e: Entity) {
        self.map.insert(e.id, e);
    }
    pub fn remove(&mut self, id: i32) {
        self.map.remove(&id);
    }
    pub fn tick(&mut self) {
        for e in self.map.values_mut() {
            e.tick();
        }
        // VANILLA: PrimedTnt#tick — fuse<=0 → discard (клиент тоже убирает сущность сам).
        self.map.retain(|_, e| e.fuse != 0);
    }
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.map.values()
    }
}

/// Типы без выбираемого хитбокса (Entity#isPickable = false / не цель луча).
const NOT_PICKABLE: &[&str] = &[
    "item",
    "experience_orb",
    "lightning_bolt",
    "area_effect_cloud",
    "marker",
    "interaction",
    "text_display",
    "block_display",
    "item_display",
    "fishing_bobber",
    "eye_of_ender",
];

impl Entity {
    /// Хитбокс (мин, макс) по размерам типа; игрок: присед 1.5, плавание 0.6.
    /// VANILLA: Entity#getBoundingBox, Player#getDefaultDimensions (по флагам метаданных).
    pub fn aabb(&self) -> Option<([f64; 3], [f64; 3])> {
        let name = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(self.ty)?;
        if NOT_PICKABLE.contains(name) {
            return None;
        }
        let (w, mut h) = crate::entity_size::size(name)?;
        if *name == "player" {
            if self.flags & 16 != 0 {
                h = 0.6;
            } else if self.flags & 1 != 0 {
                h = 1.5;
            }
        }
        let r = f64::from(w) / 2.0;
        let p = self.pos;
        Some((
            [p[0] - r, p[1], p[2] - r],
            [p[0] + r, p[1] + f64::from(h), p[2] + r],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_snaps_to_server_position() {
        // VANILLA: у снарядов NO_OP-интерполяция — позиция из пакета ставится сразу.
        let arrow = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == "arrow")
            .expect("arrow есть в реестре 26.3");
        let mut e = Entity::new(1, 0, arrow, [0.0; 3], 0.0, 0.0, 0.0);
        e.set_target(Some([1.0, 2.0, 3.0]), Some((90.0, 10.0)));
        assert_eq!(e.pos, [1.0, 2.0, 3.0]);
        assert!(!e.interpolating());
        assert_eq!((e.yaw, e.pitch), (90.0, 10.0));
    }

    #[test]
    fn lerps_to_target_in_three_ticks() {
        let mut e = Entity::new(1, 0, 0, [0.0; 3], 0.0, 0.0, 0.0);
        e.set_target(Some([3.0, 0.0, 0.0]), Some((90.0, 0.0)));
        e.tick();
        assert!((e.pos[0] - 1.0).abs() < 1e-9);
        e.tick();
        e.tick();
        assert!((e.pos[0] - 3.0).abs() < 1e-9);
        assert!((e.yaw - 90.0).abs() < 1e-4);
        assert!((e.prev_pos[0] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn yaw_takes_short_way() {
        let mut e = Entity::new(1, 0, 0, [0.0; 3], 170.0, 0.0, 170.0);
        e.set_target(None, Some((-170.0, 0.0)));
        for _ in 0..3 {
            e.tick();
        }
        assert!((wrap(e.yaw - 190.0)).abs() < 1e-3);
    }

    #[test]
    fn unpack() {
        assert_eq!(unpack_degrees(64), 90.0);
        assert_eq!(unpack_degrees(-128), -180.0);
    }
}
