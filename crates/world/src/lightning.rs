//! Клиентская молния. VANILLA: LightningBolt#tick (26.3), ClientLevel#skyFlashTime.
//!
//! Это единственный вид сущностей, который пока заводит клиент (глава 10 не начата):
//! `add_entity` с типом `lightning_bolt` → [`Bolt`], `remove_entities` убирает по id.
use crate::noise::LegacyRandom;

/// VANILLA: LightningBolt.START_LIFE.
const START_LIFE: i32 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Bolt {
    pub id: i32,
    pub pos: [f64; 3],
    life: i32,
    /// VANILLA: LightningBolt#seed — форма разряда (LightningBoltRenderer).
    pub seed: i64,
    flashes: i32,
}

/// Что сделал тик молнии на клиенте.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoltTick {
    /// `life >= 0` → `Level#setSkyFlashTime(2)`.
    pub flash: bool,
    /// Первый тик (`life == 2`): здесь ваниль играет гром и удар (звук не портирован).
    pub struck: bool,
}

pub struct Bolts {
    pub list: Vec<Bolt>,
    /// VANILLA: Entity#random (RandomSource.create() — случайное зерно).
    rng: LegacyRandom,
}

impl Default for Bolts {
    fn default() -> Self {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as i64);
        Bolts {
            list: Vec::new(),
            rng: LegacyRandom::new(t),
        }
    }
}

impl Bolts {
    /// VANILLA: LightningBolt#<init>: seed = nextLong, flashes = nextInt(3) + 1.
    pub fn spawn(&mut self, id: i32, pos: [f64; 3]) {
        // Повторный add_entity с тем же id заменяет сущность (ClientLevel#addEntity).
        self.list.retain(|b| b.id != id);
        let seed = self.rng.next_long();
        let flashes = self.rng.next_int_bound(3) + 1;
        self.list.push(Bolt {
            id,
            pos,
            life: START_LIFE,
            seed,
            flashes,
        });
    }

    pub fn remove(&mut self, id: i32) {
        self.list.retain(|b| b.id != id);
    }

    /// VANILLA: LightningBolt#tick (клиентская ветка). Возвращает: был ли тик со вспышкой.
    pub fn tick(&mut self) -> BoltTick {
        let mut out = BoltTick::default();
        let rng = &mut self.rng;
        self.list.retain_mut(|b| {
            if b.life == START_LIFE {
                out.struck = true;
            }
            b.life -= 1;
            if b.life < 0 {
                if b.flashes == 0 {
                    return false; // discard()
                } else if b.life < -rng.next_int_bound(10) {
                    b.flashes -= 1;
                    b.life = 1;
                    b.seed = rng.next_long();
                }
            }
            if b.life >= 0 {
                out.flash = true;
            }
            true
        });
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bolt_flashes_then_disappears() {
        let mut b = Bolts::default();
        b.spawn(7, [1.0, 64.0, 2.0]);
        let first = b.tick();
        assert!(first.struck && first.flash);
        let mut ticks = 1;
        while !b.list.is_empty() {
            b.tick();
            ticks += 1;
            assert!(ticks < 200, "молния не исчезла");
        }
        // 1..3 вспышки: минимум START_LIFE + 1 тик.
        assert!(ticks >= 3);
    }

    #[test]
    fn respawn_replaces_and_remove_works() {
        let mut b = Bolts::default();
        b.spawn(1, [0.0; 3]);
        b.spawn(1, [5.0, 0.0, 0.0]);
        assert_eq!(b.list.len(), 1);
        assert_eq!(b.list[0].pos[0], 5.0);
        b.remove(1);
        assert!(b.list.is_empty());
    }
}
