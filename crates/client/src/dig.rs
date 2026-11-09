//! Ломание блоков: порт MultiPlayerGameMode#startDestroyBlock/continueDestroyBlock (26.3).
//! Только рука (инвентаря ещё нет): скорость 1.0, правильный инструмент = блок не требует инструмента.
use ezzmc_world::block::{BlockState, FLAG_REQUIRES_CORRECT_TOOL};

/// ServerboundPlayerActionPacket.Action.
pub const START_DESTROY_BLOCK: i32 = 0;
pub const ABORT_DESTROY_BLOCK: i32 = 1;
pub const STOP_DESTROY_BLOCK: i32 = 2;

/// Режимы игры (GameType).
pub const SURVIVAL: i8 = 0;
pub const CREATIVE: i8 = 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DigCmd {
    pub action: i32,
    pub pos: [i32; 3],
    pub face: u8,
    pub seq: i32,
}

/// Цель взгляда на текущий тик.
#[derive(Clone, Copy, Debug)]
pub struct Aim {
    pub pos: [i32; 3],
    pub face: u8,
    pub state: BlockState,
}

/// Состояние игрока, влияющее на скорость (Player#getDestroySpeed).
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub on_ground: bool,
    pub under_water: bool,
}

#[derive(Debug, Default)]
pub struct Digger {
    destroying: bool,
    pos: [i32; 3],
    progress: f32,
    delay: u8,
    seq: i32,
    /// VANILLA: MultiPlayerGameMode#destroyTicks.
    ticks: u32,
    /// Тик копания, на котором играет hit-звук (destroyTicks % 4 == 0).
    pub hit_sound: bool,
}

/// VANILLA: BlockStateBase#getDestroyProgress (рука, без эффектов).
pub fn destroy_progress(state: BlockState, body: Body) -> f32 {
    let Some(p) = state.props() else { return 0.0 };
    if p.destroy_time < 0.0 {
        return 0.0;
    }
    let mut speed = 1.0f32;
    if body.under_water {
        speed /= 5.0; // нет Aqua Affinity
    }
    if !body.on_ground {
        speed /= 5.0;
    }
    let divisor = if p.has(FLAG_REQUIRES_CORRECT_TOOL) {
        100.0
    } else {
        30.0
    };
    speed / p.destroy_time / divisor
}

impl Digger {
    fn cmd(&mut self, action: i32, pos: [i32; 3], face: u8) -> DigCmd {
        self.seq += 1;
        DigCmd {
            action,
            pos,
            face,
            seq: self.seq,
        }
    }

    /// Клавиша отпущена или цели нет: `stopDestroyBlock`.
    fn stop(&mut self, out: &mut Vec<DigCmd>, face: u8) {
        if self.destroying {
            let p = self.pos;
            out.push(self.cmd(ABORT_DESTROY_BLOCK, p, face));
            self.destroying = false;
            self.progress = 0.0;
        }
    }

    /// Один клиентский тик. Возвращает команды серверу и блоки, ломаемые локально (предсказание).
    pub fn tick(
        &mut self,
        held: bool,
        aim: Option<Aim>,
        mode: i8,
        body: Body,
    ) -> (Vec<DigCmd>, Vec<[i32; 3]>) {
        let (mut out, mut broken) = (Vec::new(), Vec::new());
        // Minecraft#continueAttack: без удержания / без блока — abort; режимы 2,3 не ломают.
        let aim =
            aim.filter(|a| held && !a.state.is_air() && (mode == SURVIVAL || mode == CREATIVE));
        let Some(a) = aim else {
            self.stop(&mut out, 0);
            self.delay = self.delay.saturating_sub(1);
            return (out, broken);
        };
        if self.delay > 0 {
            self.delay -= 1;
            return (out, broken);
        }
        if mode == CREATIVE {
            self.delay = 5;
            out.push(self.cmd(START_DESTROY_BLOCK, a.pos, a.face));
            broken.push(a.pos);
            return (out, broken);
        }
        if self.destroying && self.pos == a.pos {
            // VANILLA: continueDestroyBlock — hitSound каждые 4 тика, затем destroyTicks++.
            self.hit_sound = self.ticks % 4 == 0;
            self.ticks += 1;
            self.progress += destroy_progress(a.state, body);
            if self.progress >= 1.0 {
                self.destroying = false;
                out.push(self.cmd(STOP_DESTROY_BLOCK, a.pos, a.face));
                broken.push(a.pos);
                self.progress = 0.0;
                self.delay = 5;
            }
            return (out, broken);
        }
        // startDestroyBlock: новая цель
        self.begin(&mut out, &mut broken, a, body);
        (out, broken)
    }

    /// VANILLA: MultiPlayerGameMode#startDestroyBlock (новая цель или предмгновенный блок).
    fn begin(&mut self, out: &mut Vec<DigCmd>, broken: &mut Vec<[i32; 3]>, a: Aim, body: Body) {
        if self.destroying {
            let p = self.pos;
            out.push(self.cmd(ABORT_DESTROY_BLOCK, p, a.face));
        }
        let first = destroy_progress(a.state, body);
        out.push(self.cmd(START_DESTROY_BLOCK, a.pos, a.face));
        if first >= 1.0 {
            self.destroying = false;
            broken.push(a.pos);
        } else {
            self.destroying = true;
            self.pos = a.pos;
            self.progress = 0.0;
            self.ticks = 0;
        }
    }

    /// VANILLA: Minecraft#startAttack по блоку — клик вызывает startDestroyBlock напрямую,
    /// destroyDelay его не блокирует (он действует только на continueDestroyBlock при удержании).
    pub fn click(
        &mut self,
        aim: Option<Aim>,
        mode: i8,
        body: Body,
    ) -> (Vec<DigCmd>, Vec<[i32; 3]>) {
        let (mut out, mut broken) = (Vec::new(), Vec::new());
        let Some(a) = aim.filter(|a| !a.state.is_air() && (mode == SURVIVAL || mode == CREATIVE))
        else {
            return (out, broken);
        };
        if mode == CREATIVE {
            self.delay = 5;
            out.push(self.cmd(START_DESTROY_BLOCK, a.pos, a.face));
            broken.push(a.pos);
        } else if !(self.destroying && self.pos == a.pos) {
            self.begin(&mut out, &mut broken, a, body);
        }
        (out, broken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(name: &str) -> BlockState {
        (0..30000u32)
            .filter_map(BlockState::from_id)
            .find(|s| s.name() == name)
            .expect("блок есть")
    }

    const GROUND: Body = Body {
        on_ground: true,
        under_water: false,
    };

    #[test]
    fn dirt_by_hand_takes_vanilla_ticks() {
        // земля: hardness 0.5, рука → 1/0.5/30 = 1/15 за тик → 15 тиков (+1 тик старта)
        let st = state("dirt");
        let aim = Some(Aim {
            pos: [0, 0, 0],
            face: 1,
            state: st,
        });
        let mut d = Digger::default();
        let mut ticks = 0;
        loop {
            ticks += 1;
            let (cmds, broken) = d.tick(true, aim, SURVIVAL, GROUND);
            if !broken.is_empty() {
                assert_eq!(cmds.last().map(|c| c.action), Some(STOP_DESTROY_BLOCK));
                break;
            }
            assert!(ticks < 100);
        }
        assert!((15..=17).contains(&ticks), "ticks={ticks}");
    }

    #[test]
    fn creative_breaks_instantly_then_delays() {
        let aim = Some(Aim {
            pos: [1, 2, 3],
            face: 0,
            state: state("stone"),
        });
        let mut d = Digger::default();
        let (c, b) = d.tick(true, aim, CREATIVE, GROUND);
        assert_eq!((c.len(), b.len()), (1, 1));
        for _ in 0..5 {
            assert!(d.tick(true, aim, CREATIVE, GROUND).1.is_empty());
        }
        assert_eq!(d.tick(true, aim, CREATIVE, GROUND).1.len(), 1);
    }

    #[test]
    fn release_aborts() {
        let aim = Some(Aim {
            pos: [0, 0, 0],
            face: 1,
            state: state("stone"),
        });
        let mut d = Digger::default();
        d.tick(true, aim, SURVIVAL, GROUND);
        let (c, _) = d.tick(false, aim, SURVIVAL, GROUND);
        assert_eq!(c.first().map(|c| c.action), Some(ABORT_DESTROY_BLOCK));
    }
}
