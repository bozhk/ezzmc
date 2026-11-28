//! Счётчики статистики, которые на 1.5.2–1.6.4 ведёт только клиент: время игры, прыжки,
//! пройденные расстояния. Суммы и файл — `ezzmc_net::stats`.
//!
//! VANILLA: PlayerEntity#tick, #jump, #method_3209 (пешком), #method_3212 (на транспорте),
//! #method_2490 (падение) — 1.6.4.

use ezzmc_net::stats::{add_local, Local};

/// Тик игрока пешком после физики.
pub struct Move {
    pub delta: [f64; 3],
    pub under_water: bool,
    pub in_water: bool,
    pub climbing: bool,
    pub on_ground: bool,
    pub was_ground: bool,
    pub jumped: bool,
    /// Высота падения до этого тика.
    pub fall_before: f64,
    /// Разрешён полёт: падение не считается.
    pub may_fly: bool,
}

/// Расстояние в сантиметрах (VANILLA: Math.round(sqrt(…) * 100)).
fn cm(parts: &[f64]) -> i32 {
    let length = parts.iter().map(|d| d * d).sum::<f64>().sqrt() as f32;
    (length * 100.0).round() as i32
}

fn walked(m: &Move) -> Vec<(Local, i32)> {
    let [x, y, z] = m.delta;
    let mut out = vec![(Local::PlayTime, 1)];
    if m.jumped {
        out.push((Local::Jump, 1));
    }
    let (stat, amount, min) = if m.under_water {
        (Local::Dive, cm(&[x, y, z]), 0)
    } else if m.in_water {
        (Local::Swim, cm(&[x, z]), 0)
    } else if m.climbing {
        (Local::Climb, (y * 100.0).round() as i32, 0)
    } else if m.on_ground {
        (Local::Walk, cm(&[x, z]), 0)
    } else {
        (Local::Fly, cm(&[x, z]), 25)
    };
    if amount > min {
        out.push((stat, amount));
    }
    if m.on_ground && !m.was_ground && !m.may_fly {
        let height = m.fall_before + (-y).max(0.0);
        if height >= 2.0 {
            out.push((Local::Fall, (height * 100.0).round() as i32));
        }
    }
    out
}

/// Тик пешком.
pub fn walk_tick(m: &Move) {
    for (stat, amount) in walked(m) {
        add_local(stat, amount);
    }
}

/// Тик на транспорте: `vehicle` — имя типа сущности, `delta` — смещение игрока.
pub fn ride_tick(vehicle: &str, delta: [f64; 3]) {
    add_local(Local::PlayTime, 1);
    let stat = if vehicle.ends_with("minecart") {
        Local::Minecart
    } else if vehicle.ends_with("boat") {
        Local::Boat
    } else if vehicle == "pig" {
        Local::Pig
    } else {
        return;
    };
    let amount = cm(&delta);
    if amount > 0 {
        add_local(stat, amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn still() -> Move {
        Move {
            delta: [0.0; 3],
            under_water: false,
            in_water: false,
            climbing: false,
            on_ground: true,
            was_ground: true,
            jumped: false,
            fall_before: 0.0,
            may_fly: false,
        }
    }

    #[test]
    fn walking_and_jumping() {
        assert_eq!(walked(&still()), vec![(Local::PlayTime, 1)]);
        let m = Move {
            delta: [0.3, 0.0, 0.4],
            jumped: true,
            ..still()
        };
        assert_eq!(
            walked(&m),
            vec![(Local::PlayTime, 1), (Local::Jump, 1), (Local::Walk, 50)]
        );
    }

    #[test]
    fn landing_counts_the_fall() {
        let m = Move {
            delta: [0.0, -0.5, 0.0],
            was_ground: false,
            fall_before: 3.0,
            ..still()
        };
        assert_eq!(walked(&m), vec![(Local::PlayTime, 1), (Local::Fall, 350)]);
    }
}
