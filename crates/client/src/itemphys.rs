//! Клиентская физика предметов-сущностей между пакетами сервера.
//! VANILLA: ItemEntity#tick (клиентская ветка) + Entity#move (MoverType.SELF).
//! Вода/лава: Entity#updateFluidHeightAndDoFluidPushing + ItemEntity#setFluidMovement.
use crate::physics::{block_friction_of, expand_towards, fluid_interaction, gather, Collider};
use ezzmc_world::level::Level;

/// VANILLA: EntityType.ITEM — 0.25×0.25.
const SIZE: f64 = 0.25;
const GRAVITY: f64 = 0.04;

pub fn tick(level: &mut Level) {
    let Some(item_ty) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "item")
    else {
        return;
    };
    let ids: Vec<i32> = level
        .entities
        .iter()
        .filter(|e| e.ty == item_ty)
        .map(|e| e.id)
        .collect();
    for id in ids {
        let Some(e) = level.entities.map.get(&id) else {
            continue;
        };
        let (mut pos, mut vel, mut ground, age) = (e.pos, e.vel, e.on_ground, e.age);
        // Чанк не загружен — не падаем сквозь мир.
        if !level.has_chunk((pos[0].floor() as i32) >> 4, (pos[2].floor() as i32) >> 4) {
            continue;
        }
        // baseTick: течение жидкости толкает предмет.
        let fb = [
            pos[0] - SIZE / 2.0,
            pos[1],
            pos[2] - SIZE / 2.0,
            pos[0] + SIZE / 2.0,
            pos[1] + SIZE,
            pos[2] + SIZE / 2.0,
        ];
        let [(wh, wpush), (lh, lpush)] = fluid_interaction(level, fb);
        for i in 0..3 {
            vel[i] += wpush[i] + lpush[i];
        }
        // ItemEntity#tick: вода → 0.99, лава → 0.95 (высота > 0.1F), иначе гравитация.
        let fluid_k = if wh > f64::from(0.1f32) {
            Some(0.99)
        } else if lh > f64::from(0.1f32) {
            Some(0.95)
        } else {
            None
        };
        if let Some(k) = fluid_k {
            let up = if vel[1] < f64::from(0.06f32) {
                f64::from(5.0e-4f32)
            } else {
                0.0
            };
            vel = [vel[0] * k, vel[1] + up, vel[2] * k];
        } else {
            vel[1] -= GRAVITY;
        }
        let horiz = vel[0] * vel[0] + vel[2] * vel[2];
        if !ground || horiz > 1.0e-5 || (i64::from(age) + i64::from(id)) % 4 == 0 {
            // Entity#move
            let bb = [
                pos[0] - SIZE / 2.0,
                pos[1],
                pos[2] - SIZE / 2.0,
                pos[0] + SIZE / 2.0,
                pos[1] + SIZE,
                pos[2] + SIZE / 2.0,
            ];
            let colliders = gather(level, expand_towards(bb, vel));
            let m = collide(vel, bb, &colliders);
            for i in 0..3 {
                pos[i] += m[i];
            }
            if m[0] != vel[0] {
                vel[0] = 0.0;
            }
            if m[2] != vel[2] {
                vel[2] = 0.0;
            }
            ground = m[1] != vel[1] && vel[1] < 0.0;
            if m[1] != vel[1] {
                vel[1] = 0.0; // Block#updateEntityMovementAfterFallOn
            }
            let mut friction = 0.98;
            if ground {
                let (bx, bz) = (pos[0].floor() as i32, pos[2].floor() as i32);
                let by = (pos[1] - 0.999_999).floor() as i32;
                friction = f64::from(block_friction_of(level.name(bx, by, bz))) * 0.98;
            }
            vel = [vel[0] * friction, vel[1] * 0.98, vel[2] * friction];
            if ground && vel[1] < 0.0 {
                vel[1] *= -0.5;
            }
        }
        if let Some(e) = level.entities.map.get_mut(&id) {
            e.pos = pos;
            e.vel = vel;
            e.on_ground = ground;
        }
    }
}

fn collide(m: [f64; 3], bb: [f64; 6], shapes: &[[f64; 6]]) -> [f64; 3] {
    // Тот же порядок осей, что у Shapes#collide (Y, затем бо́льшая из X/Z).
    crate::physics::collide_with(m, bb, shapes)
}
