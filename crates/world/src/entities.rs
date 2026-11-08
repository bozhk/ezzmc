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
    steps: u32,
    target: [f64; 3],
    target_rot: [f32; 3],
}

/// Число шагов интерполяции по умолчанию (VANILLA: InterpolationHandler(entity, 3)).
const LERP_STEPS: u32 = 3;

impl Entity {
    pub fn new(
        id: i32,
        uuid: u128,
        ty: usize,
        pos: [f64; 3],
        yaw: f32,
        pitch: f32,
        head: f32,
    ) -> Self {
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
            steps: 0,
            target: pos,
            target_rot: [yaw, pitch, head],
        }
    }

    /// VANILLA: Entity#isShiftKeyDown (общий флаг 1 → бит 2).
    pub fn crouching(&self) -> bool {
        self.flags & 2 != 0
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
        if let Some(p) = pos {
            self.target = p;
        }
        if let Some((y, x)) = rot {
            self.target_rot[0] = y;
            self.target_rot[1] = x;
        }
        self.steps = LERP_STEPS;
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
    }
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.map.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
