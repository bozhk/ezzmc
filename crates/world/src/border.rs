//! Граница мира на клиенте.
//! VANILLA: WorldBorder (Static/MovingBorderExtent), BorderStatus,
//! ClientPacketListener#handleInitializeBorder/handleSetBorder* (26.3).

use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// VANILLA: WorldBorder.MAX_SIZE (5.999997E7F как double).
pub const MAX_SIZE: f64 = 5.999_997e7_f32 as f64;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Extent {
    Static {
        size: f64,
    },
    Moving {
        from: f64,
        to: f64,
        lerp_begin: i64,
        lerp_end: i64,
        duration: f64,
        progress: i64,
        size: f64,
        prev: f64,
    },
}

/// VANILLA: BorderStatus — цвет стены.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Growing,
    Shrinking,
    Stationary,
}

impl Status {
    pub fn color(self) -> u32 {
        match self {
            Status::Growing => 4259712,
            Status::Shrinking => 16724016,
            Status::Stationary => 2138367,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldBorder {
    pub center_x: f64,
    pub center_z: f64,
    pub absolute_max_size: i32,
    pub warning_time: i32,
    pub warning_blocks: i32,
    extent: Extent,
}

impl Default for WorldBorder {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_z: 0.0,
            absolute_max_size: 29_999_984,
            // VANILLA: поля класса — warningTime 15, warningBlocks 5.
            warning_time: 15,
            warning_blocks: 5,
            extent: Extent::Static { size: MAX_SIZE },
        }
    }
}

fn moving_size(from: f64, to: f64, duration: f64, progress: i64) -> f64 {
    let p = (duration - progress as f64) / duration;
    if p < 1.0 {
        from + p * (to - from)
    } else {
        to
    }
}

impl WorldBorder {
    pub fn set_size(&mut self, size: f64) {
        self.extent = Extent::Static { size };
    }

    /// VANILLA: WorldBorder#lerpSizeBetween.
    pub fn lerp_size_between(&mut self, from: f64, to: f64, ticks: i64, game_time: i64) {
        if from == to {
            self.extent = Extent::Static { size: to };
            return;
        }
        let duration = ticks as f64;
        let s = moving_size(from, to, duration, ticks);
        self.extent = Extent::Moving {
            from,
            to,
            lerp_begin: game_time,
            lerp_end: game_time + ticks,
            duration,
            progress: ticks,
            size: s,
            prev: s,
        };
    }

    /// VANILLA: WorldBorder#tick → MovingBorderExtent#update.
    pub fn tick(&mut self) {
        if let Extent::Moving {
            from,
            to,
            duration,
            progress,
            size,
            prev,
            ..
        } = &mut self.extent
        {
            *progress -= 1;
            *prev = *size;
            *size = moving_size(*from, *to, *duration, *progress);
            if *progress <= 0 {
                self.extent = Extent::Static { size: *to };
            }
        }
    }

    pub fn size(&self) -> f64 {
        match self.extent {
            Extent::Static { size } | Extent::Moving { size, .. } => size,
        }
    }

    fn size_at(&self, partial: f32) -> f64 {
        match self.extent {
            Extent::Static { size } => size,
            Extent::Moving { size, prev, .. } => prev + f64::from(partial) * (size - prev),
        }
    }

    fn clamp(&self, v: f64) -> f64 {
        let m = f64::from(self.absolute_max_size);
        v.clamp(-m, m)
    }

    pub fn min_x(&self, partial: f32) -> f64 {
        self.clamp(self.center_x - self.size_at(partial) / 2.0)
    }
    pub fn max_x(&self, partial: f32) -> f64 {
        self.clamp(self.center_x + self.size_at(partial) / 2.0)
    }
    pub fn min_z(&self, partial: f32) -> f64 {
        self.clamp(self.center_z - self.size_at(partial) / 2.0)
    }
    pub fn max_z(&self, partial: f32) -> f64 {
        self.clamp(self.center_z + self.size_at(partial) / 2.0)
    }

    /// VANILLA: WorldBorder#getDistanceToBorder (без partial).
    pub fn distance_to(&self, x: f64, z: f64) -> f64 {
        let n = z - self.min_z(0.0);
        let s = self.max_z(0.0) - z;
        let w = x - self.min_x(0.0);
        let e = self.max_x(0.0) - x;
        w.min(e).min(n).min(s)
    }

    pub fn is_within(&self, x: f64, z: f64, margin: f64) -> bool {
        x >= self.min_x(0.0) - margin
            && x < self.max_x(0.0) + margin
            && z >= self.min_z(0.0) - margin
            && z < self.max_z(0.0) + margin
    }

    pub fn status(&self) -> Status {
        match self.extent {
            Extent::Static { .. } => Status::Stationary,
            Extent::Moving { from, to, .. } if to < from => Status::Shrinking,
            Extent::Moving { .. } => Status::Growing,
        }
    }

    pub fn lerp_speed(&self) -> f64 {
        match self.extent {
            Extent::Static { .. } => 0.0,
            Extent::Moving {
                from,
                to,
                lerp_begin,
                lerp_end,
                ..
            } => (from - to).abs() / (lerp_end - lerp_begin) as f64,
        }
    }

    pub fn lerp_target(&self) -> f64 {
        match self.extent {
            Extent::Static { size } => size,
            Extent::Moving { to, .. } => to,
        }
    }

    /// Сила красной виньетки 0..1. VANILLA: Hud#extractVignette (borderWarningStrength).
    pub fn warning_strength(&self, x: f64, z: f64) -> f32 {
        let dist = self.distance_to(x, z) as f32;
        let moving = (self.lerp_speed() * f64::from(self.warning_time))
            .min((self.lerp_target() - self.size()).abs());
        let warn = f64::from(self.warning_blocks).max(moving);
        if f64::from(dist) < warn {
            (1.0 - (f64::from(dist) / warn) as f32).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Коробка внутри границы для коллизии: (minX, minZ, maxX, maxZ) с floor/ceil.
    /// VANILLA: BorderExtent#getCollisionShape — INFINITY минус box(floor(min), ceil(max)).
    pub fn collision_box(&self) -> [f64; 4] {
        [
            self.min_x(0.0).floor(),
            self.min_z(0.0).floor(),
            self.max_x(0.0).ceil(),
            self.max_z(0.0).ceil(),
        ]
    }

    /// VANILLA: WorldBorder#isInsideCloseToBorder — нужна ли граница в коллизиях.
    pub fn inside_close(&self, x: f64, z: f64, bb_xsize: f64, bb_zsize: f64) -> bool {
        let m = bb_xsize.abs().max(bb_zsize.abs()).max(1.0);
        self.distance_to(x, z) < m * 2.0 && self.is_within(x, z, m)
    }

    /// Пакеты границы; `Ok(false)` — не наш пакет.
    pub fn handle(&mut self, name: &str, body: &[u8], game_time: i64) -> ProtocolResult<bool> {
        let mut r = Reader::new(body);
        match name {
            "initialize_border" => {
                self.center_x = r.f64()?;
                self.center_z = r.f64()?;
                let old = r.f64()?;
                let new = r.f64()?;
                let lerp = r.var_long()?;
                if lerp > 0 {
                    self.lerp_size_between(old, new, lerp, game_time);
                } else {
                    self.set_size(new);
                }
                self.absolute_max_size = r.var_int()?;
                self.warning_blocks = r.var_int()?;
                self.warning_time = r.var_int()?;
            }
            "set_border_center" => {
                self.center_x = r.f64()?;
                self.center_z = r.f64()?;
            }
            "set_border_lerp_size" => {
                let old = r.f64()?;
                let new = r.f64()?;
                let lerp = r.var_long()?;
                self.lerp_size_between(old, new, lerp, game_time);
            }
            "set_border_size" => self.set_size(r.f64()?),
            "set_border_warning_delay" => self.warning_time = r.var_int()?,
            "set_border_warning_distance" => self.warning_blocks = r.var_int()?,
            _ => return Ok(false),
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_shrinks_and_settles() {
        let mut b = WorldBorder::default();
        b.lerp_size_between(100.0, 50.0, 10, 0);
        assert_eq!(b.status(), Status::Shrinking);
        assert_eq!(b.size(), 100.0);
        for _ in 0..5 {
            b.tick();
        }
        assert!((b.size() - 75.0).abs() < 1e-9);
        assert!((b.min_x(0.5) + 38.75).abs() < 1e-9, "partial между 80 и 75");
        for _ in 0..5 {
            b.tick();
        }
        assert_eq!(b.status(), Status::Stationary);
        assert_eq!(b.size(), 50.0);
        assert_eq!(b.max_x(0.0), 25.0);
    }

    #[test]
    fn warning_and_packets() {
        let mut b = WorldBorder::default();
        let mut w = ezzmc_protocol::wire::Writer::new();
        w.f64(10.0).f64(-4.0).f64(20.0).f64(20.0).var_long(0);
        w.var_int(29_999_984).var_int(5).var_int(15);
        assert!(b.handle("initialize_border", &w.into_vec(), 0).unwrap());
        assert_eq!((b.min_x(0.0), b.max_z(0.0)), (0.0, 6.0));
        // 2 блока до восточной стены, предупреждение 5 → 1 − 2/5.
        assert!((b.warning_strength(18.0, -4.0) - 0.6).abs() < 1e-6);
        assert_eq!(b.warning_strength(10.0, -4.0), 0.0);
        assert!(b.inside_close(19.5, -4.0, 0.6, 0.6));
    }
}
