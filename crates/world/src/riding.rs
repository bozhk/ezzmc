//! Пассажиры и транспорт (TODO-206). VANILLA: ClientPacketListener#handleSetEntityPassengersPacket,
//! Entity#positionRider/getPassengerRidingPosition/getPassengerAttachmentPoint/getVehicleAttachmentPoint,
//! AbstractBoat#getPassengerAttachmentPoint (26.3).
//! VANILLA-UNSURE: переопределения Llama/Camel/Strider/Chicken/Spider/AbstractCubeMob/AbstractHorse
//! (стойка, пониженная точка у жителей) не портированы — берутся точки типа из EntityType.
use crate::level::Level;
use std::collections::HashMap;

/// Пассажиры по id транспорта и обратная связь «пассажир → транспорт».
#[derive(Default, Debug)]
pub struct Riders {
    pub passengers: HashMap<i32, Vec<i32>>,
    pub vehicle_of: HashMap<i32, i32>,
}

impl Riders {
    /// VANILLA: Entity#ejectPassengers + startRiding(force) для каждого из списка пакета.
    pub fn set(&mut self, vehicle: i32, list: Vec<i32>) {
        self.eject(vehicle);
        for &p in &list {
            // Пассажир не может ехать на двух транспортах сразу.
            if let Some(old) = self.vehicle_of.insert(p, vehicle) {
                if let Some(v) = self.passengers.get_mut(&old) {
                    v.retain(|x| *x != p);
                }
            }
        }
        if !list.is_empty() {
            self.passengers.insert(vehicle, list);
        }
    }

    pub fn eject(&mut self, vehicle: i32) {
        if let Some(old) = self.passengers.remove(&vehicle) {
            for p in old {
                self.vehicle_of.remove(&p);
            }
        }
    }

    /// Сущность исчезла: её пассажиры слезают, сама она перестаёт ехать.
    pub fn removed(&mut self, id: i32) {
        self.eject(id);
        if let Some(v) = self.vehicle_of.remove(&id) {
            if let Some(list) = self.passengers.get_mut(&v) {
                list.retain(|x| *x != id);
            }
        }
    }
}

fn type_name(ty: usize) -> &'static str {
    ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .get(ty)
        .copied()
        .unwrap_or("")
}

fn rot_y(p: [f64; 3], yaw: f32) -> [f64; 3] {
    // VANILLA: Vec3#yRot(-yRot * PI/180).
    let a = f64::from(-yaw * (std::f32::consts::PI / 180.0));
    let (s, c) = (a.sin(), a.cos());
    [p[0] * c + p[2] * s, p[1], p[2] * c - p[0] * s]
}

impl Level {
    /// VANILLA: Entity#getPassengerAttachmentPoint (с поворотом по yRot транспорта).
    fn passenger_attachment(&self, vehicle: &crate::entities::Entity, index: usize) -> [f64; 3] {
        let name = type_name(vehicle.ty);
        let count = self.riders.passengers.get(&vehicle.id).map_or(0, Vec::len);
        if name.ends_with("_boat") || name.ends_with("_raft") {
            // VANILLA: AbstractBoat#getPassengerAttachmentPoint; rideHeight: лодка h/3, плот h*0.8888889.
            let h = f64::from(crate::entity_size::size(name).map_or(0.5625, |s| s.1));
            let ride = if name.ends_with("_raft") {
                h * f64::from(0.888_888_9_f32)
            } else {
                h / 3.0
            };
            let single = if name.starts_with("chest_") || name.contains("_chest_") {
                0.15
            } else {
                0.0
            };
            let off = if count > 1 {
                // VANILLA-UNSURE: +0.2 для Animal-пассажира не учтено.
                if index == 0 {
                    0.2
                } else {
                    -0.6
                }
            } else {
                single
            };
            return rot_y([0.0, ride, off], vehicle.yaw);
        }
        let pts = crate::entity_attach::passenger_points(name);
        let p = if pts.is_empty() {
            // Fallback.AT_HEIGHT.
            let h = crate::entity_size::size(name).map_or(0.0, |s| s.1);
            [0.0, h, 0.0]
        } else {
            pts[index.min(pts.len() - 1)]
        };
        rot_y(p.map(f64::from), vehicle.yaw)
    }

    /// VANILLA: Entity#positionRider — позиция пассажира `passenger` (тип `passenger_ty`) на транспорте.
    pub fn ride_position(&self, passenger: i32, passenger_ty: usize) -> Option<[f64; 3]> {
        let vid = *self.riders.vehicle_of.get(&passenger)?;
        let vehicle = self.entities.map.get(&vid)?;
        let index = self
            .riders
            .passengers
            .get(&vid)?
            .iter()
            .position(|x| *x == passenger)?;
        let a = self.passenger_attachment(vehicle, index);
        let v = crate::entity_attach::vehicle_point(type_name(passenger_ty));
        // VEHICLE-точка пассажира тоже поворачивается по его yRot (EntityAttachments#get), но у
        // всех типов с ненулевой точкой x=z=0, поворот не меняет y.
        Some([
            vehicle.pos[0] + a[0] - f64::from(v[0]),
            vehicle.pos[1] + a[1] - f64::from(v[1]),
            vehicle.pos[2] + a[2] - f64::from(v[2]),
        ])
    }

    /// VANILLA: Entity#rideTick чужих пассажиров — позиция следует за транспортом каждый тик.
    pub fn ride_tick(&mut self) {
        let ids: Vec<i32> = self.riders.vehicle_of.keys().copied().collect();
        for id in ids {
            let Some(ty) = self.entities.map.get(&id).map(|e| e.ty) else {
                continue;
            };
            if let Some(p) = self.ride_position(id, ty) {
                if let Some(e) = self.entities.map.get_mut(&id) {
                    e.pos = p;
                    e.snap_target(p);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Entity;

    fn ty(name: &str) -> usize {
        ezzmc_protocol::generated::r26_3::ENTITY_TYPE
            .iter()
            .position(|n| *n == name)
            .unwrap()
    }

    fn level() -> Level {
        Level::new(
            crate::chunk::ChunkFormat {
                min_y: -64,
                height: 384,
                block_global_bits: 15,
                biome_global_bits: 6,
            },
            4,
        )
    }

    #[test]
    fn player_on_cow_and_boat() {
        let mut l = level();
        let player = ty("player");
        l.entities.add(Entity::new(
            10,
            0,
            ty("cow"),
            [1.0, 64.0, 2.0],
            0.0,
            0.0,
            0.0,
        ));
        l.entities.add(Entity::new(
            11,
            0,
            ty("oak_boat"),
            [0.0, 63.0, 0.0],
            90.0,
            0.0,
            0.0,
        ));
        l.riders.set(10, vec![1]);
        // cow: passenger y=1.36875, игрок: vehicle y=0.6.
        let p = l.ride_position(1, player).unwrap();
        assert!((p[1] - (64.0 + 1.36875 - 0.6)).abs() < 1e-6, "{p:?}");
        // Пересадка на лодку: со старого транспорта слезает.
        l.riders.set(11, vec![1]);
        assert!(l.riders.passengers.get(&10).is_none_or(Vec::is_empty));
        let p = l.ride_position(1, player).unwrap();
        assert!((p[1] - (63.0 + 0.5625 / 3.0 - 0.6)).abs() < 1e-5, "{p:?}");
        l.riders.removed(11);
        assert!(l.ride_position(1, player).is_none());
    }
}
