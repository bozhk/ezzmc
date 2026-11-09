//! Поршни на клиенте: PistonBaseBlock#triggerEvent/moveBlocks, PistonStructureResolver,
//! PistonMovingBlockEntity (тик, finalTick). Ядро без соседних обновлений форм/сигналов:
//! их рассылает сервер блок-апдейтами (VANILLA-UNSURE: updateNeighbourShapes на клиенте не делаем).
//! Данные `data/piston_data.txt` — `tools/gen_piston_data.py` (pushReaction, BlockEntity).
use crate::block::BlockState;
use crate::level::Level;
use ezzmc_protocol::types::BlockPos;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

/// VANILLA: Direction#get3DDataValue — down, up, north, south, west, east.
pub const DIRS: [&str; 6] = ["down", "up", "north", "south", "west", "east"];
const STEP: [[i32; 3]; 6] = [
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
    [-1, 0, 0],
    [1, 0, 0],
];

pub fn opposite(d: u8) -> u8 {
    d ^ 1
}

pub fn step(d: u8) -> [i32; 3] {
    STEP[usize::from(d % 6)]
}

pub fn dir_of(name: &str) -> Option<u8> {
    DIRS.iter().position(|d| *d == name).map(|i| i as u8)
}

fn rel(p: BlockPos, d: u8, n: i32) -> BlockPos {
    let s = step(d);
    BlockPos {
        x: p.x + s[0] * n,
        y: p.y + s[1] * n,
        z: p.z + s[2] * n,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Push {
    PushPull,
    Push,
    Popped,
    Immoveable,
}

struct Tables {
    push: HashMap<&'static str, Push>,
    entity: HashSet<&'static str>,
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mut t = Tables {
            push: HashMap::new(),
            entity: HashSet::new(),
        };
        for l in include_str!("../data/piston_data.txt").lines() {
            let mut it = l.split(' ');
            match (it.next(), it.next(), it.next()) {
                (Some("P"), Some(n), Some(r)) => {
                    let r = match r {
                        "PUSH" => Push::Push,
                        "POPPED" => Push::Popped,
                        "IMMOVEABLE" => Push::Immoveable,
                        _ => Push::PushPull,
                    };
                    t.push.insert(n, r);
                }
                (Some("E"), Some(n), _) => {
                    t.entity.insert(n);
                }
                _ => {}
            }
        }
        t
    })
}

/// VANILLA: BlockStateBase#getPistonPushReaction.
pub fn push_reaction(s: BlockState) -> Push {
    tables()
        .push
        .get(s.name())
        .copied()
        .unwrap_or(Push::PushPull)
}

/// VANILLA: BlockStateBase#hasBlockEntity.
pub fn has_block_entity(s: BlockState) -> bool {
    tables().entity.contains(s.name())
}

fn default_of(name: &str) -> Option<BlockState> {
    use crate::generated::b26_3::BLOCKS;
    BLOCKS
        .iter()
        .find(|b| b.name == name)
        .and_then(|b| BlockState::from_id(b.default_state))
}

fn state_with(name: &str, props: &[(&str, &str)]) -> Option<BlockState> {
    let mut s = default_of(name)?;
    for (k, v) in props {
        s = s.with(k, v)?;
    }
    Some(s)
}

fn is(s: BlockState, n: &str) -> bool {
    s.name() == n
}

/// VANILLA: PistonMovingBlockEntity.
#[derive(Clone, Debug)]
pub struct MovingPiston {
    pub moved: BlockState,
    pub dir: u8,
    pub extending: bool,
    pub source: bool,
    pub progress: f32,
    pub progress_o: f32,
    pub last_ticked: i64,
    pub death_ticks: u8,
}

impl MovingPiston {
    fn new(moved: BlockState, dir: u8, extending: bool, source: bool) -> Self {
        MovingPiston {
            moved,
            dir,
            extending,
            source,
            progress: 0.0,
            progress_o: 0.0,
            last_ticked: 0,
            death_ticks: 0,
        }
    }

    /// VANILLA: #getProgress(a).
    pub fn get_progress(&self, a: f32) -> f32 {
        let a = a.min(1.0);
        self.progress_o + (self.progress - self.progress_o) * a
    }

    /// VANILLA: #getExtendedProgress.
    pub fn extended_progress(&self, p: f32) -> f32 {
        if self.extending {
            p - 1.0
        } else {
            1.0 - p
        }
    }

    /// Смещение блока при рендере (getXOff/getYOff/getZOff).
    pub fn offset(&self, a: f32) -> [f32; 3] {
        let e = self.extended_progress(self.get_progress(a));
        let s = step(self.dir);
        [s[0] as f32 * e, s[1] as f32 * e, s[2] as f32 * e]
    }

    /// VANILLA: #getMovementDirection.
    pub fn movement_dir(&self) -> u8 {
        if self.extending {
            self.dir
        } else {
            opposite(self.dir)
        }
    }
}

/// Результат события поршня: что перерисовать и какие блоки «сломались» (частицы 2001).
#[derive(Default)]
pub struct PistonChanges {
    pub changed: Vec<BlockPos>,
    pub destroyed: Vec<(BlockPos, BlockState)>,
}

type Key = (i32, i32, i32);
fn key(p: BlockPos) -> Key {
    (p.x, p.y, p.z)
}

impl Level {
    fn bs(&self, p: BlockPos) -> BlockState {
        self.chunks.block(p.x, p.y, p.z)
    }

    fn put(&mut self, p: BlockPos, s: BlockState, ch: &mut PistonChanges) {
        if self.chunks.set_block(p.x, p.y, p.z, s) {
            ch.changed.push(p);
        }
    }

    /// VANILLA: PistonBaseBlock.isPushable.
    fn piston_pushable(
        &self,
        s: BlockState,
        p: BlockPos,
        dir: u8,
        allow_destroyable: bool,
        connection: u8,
    ) -> bool {
        let f = self.chunks.format();
        let (min_y, max_y) = (f.min_y, f.min_y + f.height - 1);
        if p.y < min_y || p.y > max_y {
            return false;
        }
        if s.is_air() {
            return true;
        }
        if dir == 0 && p.y == min_y {
            return false;
        }
        if dir == 1 && p.y == max_y {
            return false;
        }
        if !is(s, "piston") && !is(s, "sticky_piston") {
            if s.props().is_some_and(|q| q.destroy_time < 0.0) {
                return false;
            }
            match push_reaction(s) {
                Push::Immoveable => return false,
                Push::Popped => return allow_destroyable,
                Push::Push => return dir == connection,
                Push::PushPull => {}
            }
        } else if s.property("extended") == Some("true") {
            return false;
        }
        !has_block_entity(s)
    }

    /// VANILLA: PistonBaseBlock#triggerEvent (клиентская ветка). `state` — блок поршня в `pos`.
    pub fn piston_event(&mut self, pos: BlockPos, b0: u8, b1: u8) -> PistonChanges {
        let mut ch = PistonChanges::default();
        let state = self.bs(pos);
        let sticky = is(state, "sticky_piston");
        if !sticky && !is(state, "piston") {
            return ch;
        }
        let Some(dir) = state.property("facing").and_then(dir_of) else {
            return ch;
        };
        let ty = if sticky { "sticky" } else { "normal" };
        if b0 == 0 {
            if !self.piston_move_blocks(pos, dir, true, sticky, &mut ch) {
                return ch;
            }
            if let Some(e) = state.with("extended", "true") {
                self.put(pos, e, &mut ch);
            }
        } else if b0 == 1 || b0 == 2 {
            let arm = rel(pos, dir, 1);
            self.piston_final_tick(arm, &mut ch);
            let Some(moving) = state_with(
                "moving_piston",
                &[("facing", DIRS[usize::from(dir)]), ("type", ty)],
            ) else {
                return ch;
            };
            self.put(pos, moving, &mut ch);
            let base = state
                .with("extended", "false")
                .and_then(|s| s.with("facing", DIRS[usize::from(b1 & 7) % 6]));
            if let Some(base) = base {
                self.pistons
                    .insert(key(pos), MovingPiston::new(base, dir, false, true));
            }
            if sticky {
                let two = rel(pos, dir, 2);
                let ms = self.bs(two);
                let mut piece = false;
                if is(ms, "moving_piston") {
                    if let Some(e) = self.pistons.get(&key(two)) {
                        if e.dir == dir && e.extending {
                            self.piston_final_tick(two, &mut ch);
                            piece = true;
                        }
                    }
                }
                if !piece {
                    if b0 != 1
                        || ms.is_air()
                        || !self.piston_pushable(ms, two, opposite(dir), false, dir)
                        || (push_reaction(ms) != Push::PushPull
                            && !is(ms, "piston")
                            && !is(ms, "sticky_piston"))
                    {
                        self.put(arm, BlockState::AIR, &mut ch);
                    } else {
                        self.piston_move_blocks(pos, dir, false, sticky, &mut ch);
                    }
                }
            } else {
                self.put(arm, BlockState::AIR, &mut ch);
            }
        }
        ch
    }

    /// VANILLA: PistonBaseBlock#moveBlocks.
    fn piston_move_blocks(
        &mut self,
        piston: BlockPos,
        dir: u8,
        extending: bool,
        sticky: bool,
        ch: &mut PistonChanges,
    ) -> bool {
        let arm = rel(piston, dir, 1);
        if !extending && is(self.bs(arm), "piston_head") {
            self.put(arm, BlockState::AIR, ch);
        }
        let Some((to_push, to_destroy)) = self.resolve_piston(piston, dir, extending) else {
            return false;
        };
        let push_dir = if extending { dir } else { opposite(dir) };
        let mut delete_after: HashMap<Key, BlockState> = HashMap::new();
        let shapes: Vec<BlockState> = to_push.iter().map(|p| self.bs(*p)).collect();
        for (p, s) in to_push.iter().zip(&shapes) {
            delete_after.insert(key(*p), *s);
        }
        for p in to_destroy.iter().rev() {
            let s = self.bs(*p);
            if !s.name().contains("fire") {
                ch.destroyed.push((*p, s));
            }
            self.put(*p, BlockState::AIR, ch);
        }
        let facing = DIRS[usize::from(dir)];
        for (i, p) in to_push.iter().enumerate().rev() {
            let np = rel(*p, push_dir, 1);
            delete_after.remove(&key(np));
            if let Some(m) = state_with("moving_piston", &[("facing", facing)]) {
                self.put(np, m, ch);
                self.pistons
                    .insert(key(np), MovingPiston::new(shapes[i], dir, extending, false));
            }
        }
        if extending {
            let ty = if sticky { "sticky" } else { "normal" };
            let head = state_with("piston_head", &[("facing", facing), ("type", ty)]);
            let mov = state_with("moving_piston", &[("facing", facing), ("type", ty)]);
            delete_after.remove(&key(arm));
            if let (Some(h), Some(m)) = (head, mov) {
                self.put(arm, m, ch);
                self.pistons
                    .insert(key(arm), MovingPiston::new(h, dir, true, true));
            }
        }
        for k in delete_after.keys() {
            let p = BlockPos {
                x: k.0,
                y: k.1,
                z: k.2,
            };
            self.put(p, BlockState::AIR, ch);
        }
        true
    }

    /// VANILLA: PistonStructureResolver#resolve → (toPush, toDestroy).
    fn resolve_piston(
        &self,
        piston: BlockPos,
        dir: u8,
        extending: bool,
    ) -> Option<(Vec<BlockPos>, Vec<BlockPos>)> {
        let mut r = Resolver {
            level: self,
            piston,
            extending,
            start: if extending {
                rel(piston, dir, 1)
            } else {
                rel(piston, dir, 2)
            },
            push_dir: if extending { dir } else { opposite(dir) },
            piston_dir: dir,
            to_push: vec![],
            to_destroy: vec![],
        };
        r.resolve().then_some((r.to_push, r.to_destroy))
    }

    /// VANILLA: PistonMovingBlockEntity#finalTick.
    fn piston_final_tick(&mut self, p: BlockPos, ch: &mut PistonChanges) {
        let Some(e) = self.pistons.remove(&key(p)) else {
            return;
        };
        if is(self.bs(p), "moving_piston") {
            let ns = if e.source { BlockState::AIR } else { e.moved };
            self.put(p, ns, ch);
        }
    }

    /// VANILLA: PistonMovingBlockEntity#tick для всех движущихся блоков (клиентский тик).
    pub fn tick_pistons(&mut self) -> PistonChanges {
        let mut ch = PistonChanges::default();
        let keys: Vec<Key> = self.pistons.keys().copied().collect();
        let now = self.game_time;
        for k in keys {
            let p = BlockPos {
                x: k.0,
                y: k.1,
                z: k.2,
            };
            // блок-сущность без блока moving_piston (сервер заменил) — убираем
            if !is(self.bs(p), "moving_piston") && self.chunks.get(p.x >> 4, p.z >> 4).is_some() {
                self.pistons.remove(&k);
                continue;
            }
            let Some(e) = self.pistons.get_mut(&k) else {
                continue;
            };
            e.last_ticked = now;
            e.progress_o = e.progress;
            if e.progress_o >= 1.0 {
                if e.death_ticks < 5 {
                    e.death_ticks += 1;
                } else {
                    let moved = e.moved;
                    self.pistons.remove(&k);
                    if is(self.bs(p), "moving_piston") {
                        let ns = moved.with("waterlogged", "false").unwrap_or(moved);
                        self.put(p, ns, &mut ch);
                    }
                }
            } else {
                e.progress = (e.progress + 0.5).min(1.0);
            }
        }
        ch
    }
}

struct Resolver<'a> {
    level: &'a Level,
    piston: BlockPos,
    extending: bool,
    start: BlockPos,
    push_dir: u8,
    piston_dir: u8,
    to_push: Vec<BlockPos>,
    to_destroy: Vec<BlockPos>,
}

fn sticky_block(s: BlockState) -> bool {
    is(s, "slime_block") || is(s, "honey_block")
}

fn can_stick(a: BlockState, b: BlockState) -> bool {
    if is(a, "honey_block") && is(b, "slime_block") {
        return false;
    }
    if is(a, "slime_block") && is(b, "honey_block") {
        return false;
    }
    sticky_block(a) || sticky_block(b)
}

impl Resolver<'_> {
    fn resolve(&mut self) -> bool {
        self.to_push.clear();
        self.to_destroy.clear();
        let next = self.level.bs(self.start);
        if !self
            .level
            .piston_pushable(next, self.start, self.push_dir, false, self.piston_dir)
        {
            if self.extending && push_reaction(next) == Push::Popped {
                self.to_destroy.push(self.start);
                return true;
            }
            return false;
        }
        if !self.add_block_line(self.start, self.push_dir) {
            return false;
        }
        let mut i = 0;
        while i < self.to_push.len() {
            let p = self.to_push[i];
            if sticky_block(self.level.bs(p)) && !self.add_branching(p) {
                return false;
            }
            i += 1;
        }
        true
    }

    fn add_block_line(&mut self, start: BlockPos, direction: u8) -> bool {
        let lv = self.level;
        let mut next = lv.bs(start);
        if next.is_air() {
            return true;
        }
        if !lv.piston_pushable(next, start, self.push_dir, false, direction) {
            return true;
        }
        if start == self.piston || self.to_push.contains(&start) {
            return true;
        }
        let mut count = 1i32;
        if count as usize + self.to_push.len() > 12 {
            return false;
        }
        while sticky_block(next) {
            let pos = rel(start, opposite(self.push_dir), count);
            let prev = next;
            next = lv.bs(pos);
            if next.is_air()
                || !can_stick(prev, next)
                || !lv.piston_pushable(next, pos, self.push_dir, false, opposite(self.push_dir))
                || pos == self.piston
            {
                break;
            }
            count += 1;
            if count as usize + self.to_push.len() > 12 {
                return false;
            }
        }
        let mut added = 0usize;
        for i in (0..count).rev() {
            self.to_push.push(rel(start, opposite(self.push_dir), i));
            added += 1;
        }
        let mut i = 1;
        loop {
            let pos = rel(start, self.push_dir, i);
            if let Some(coll) = self.to_push.iter().position(|p| *p == pos) {
                self.reorder(added, coll);
                for j in 0..=(coll + added) {
                    let bp = self.to_push[j];
                    if sticky_block(lv.bs(bp)) && !self.add_branching(bp) {
                        return false;
                    }
                }
                return true;
            }
            next = lv.bs(pos);
            if next.is_air() {
                return true;
            }
            if !lv.piston_pushable(next, pos, self.push_dir, true, self.push_dir)
                || pos == self.piston
            {
                return false;
            }
            if push_reaction(next) == Push::Popped {
                self.to_destroy.push(pos);
                return true;
            }
            if self.to_push.len() >= 12 {
                return false;
            }
            self.to_push.push(pos);
            added += 1;
            i += 1;
        }
    }

    fn reorder(&mut self, added: usize, coll: usize) {
        let n = self.to_push.len();
        let head = self.to_push[..coll].to_vec();
        let last = self.to_push[n - added..].to_vec();
        let mid = self.to_push[coll..n - added].to_vec();
        self.to_push = [head, last, mid].concat();
    }

    fn add_branching(&mut self, from: BlockPos) -> bool {
        let fs = self.level.bs(from);
        for d in 0..6u8 {
            if d / 2 != self.push_dir / 2 {
                let np = rel(from, d, 1);
                let ns = self.level.bs(np);
                if can_stick(ns, fs) && !self.add_block_line(np, d) {
                    return false;
                }
            }
        }
        true
    }
}
