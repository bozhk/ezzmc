//! Расширенное управление агентом (не ваниль, игру не меняет): «глаза» (блок под прицелом, блок по
//! координатам, поиск и сводка блоков вокруг, ASCII-срезы и вид сверху, список сущностей с углами
//! наведения) и «руки» (задачи: идти по A*-пути, копать, использовать блок, бить сущность).
//! Команды разбираются в `control.rs::control_command`, задачи тикаются в `control_tick`.
use crate::{
    control::{err, ok},
    keymap::Input,
    App,
};
use ezzmc_mesh::blockinfo as bi;
use ezzmc_world::{block::BlockState, level::Level};
use serde_json::{json, Value};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::mpsc::Sender,
    time::{Duration, Instant},
};

type P = [i32; 3];

const FACES: [&str; 6] = ["down", "up", "north", "south", "west", "east"];
/// Единичные нормали граней в порядке Direction.
const NORMALS: [[i32; 3]; 6] = [
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
    [-1, 0, 0],
    [1, 0, 0],
];

/// Текущая задача агента (одна за раз): отвечает на запрос, когда завершится.
pub struct Task {
    pub kind: Kind,
    pub reply: Sender<Value>,
    pub started: Instant,
    pub until: Instant,
    /// Нажатые задачей ввод-кнопки (чтобы отпустить в конце и не дёргать каждый кадр).
    held: [bool; 4],
}

pub enum Kind {
    Goto {
        path: Vec<P>,
        idx: usize,
        goal: P,
        range: f64,
        sprint: bool,
        replans: u8,
        last_move: (Instant, [f64; 3]),
    },
    Mine {
        pos: P,
        was: String,
    },
    Use {
        pos: P,
        face: Option<u8>,
        phase: u8,
        at: Instant,
        ms: u64,
        placed_at: Option<P>,
    },
    Attack {
        id: i32,
        left: u32,
        done: u32,
        pressed: bool,
    },
}

const KEYS: [&str; 4] = ["key.forward", "key.jump", "key.sprint", "key.attack"];
const FWD: usize = 0;
const JUMP: usize = 1;
const SPRINT: usize = 2;
const ATTACK: usize = 3;

fn name_of(s: BlockState) -> &'static str {
    s.name()
}

/// Высота верхней грани коллизии блока (0 — проходим).
fn top(s: BlockState) -> f64 {
    s.props()
        .map_or(&[][..], |p| bi::collision_boxes_of(p.collision_shape))
        .iter()
        .map(|b| b[4])
        .fold(0.0, f64::max)
}

fn is_water(s: BlockState) -> bool {
    matches!(s.name(), "water" | "bubble_column") || s.property("waterlogged") == Some("true")
}

fn hazard(s: BlockState) -> bool {
    matches!(
        s.name(),
        "lava"
            | "fire"
            | "soul_fire"
            | "magma_block"
            | "cactus"
            | "sweet_berry_bush"
            | "cobweb"
            | "wither_rose"
            | "campfire"
            | "soul_campfire"
            | "powder_snow"
            | "pointed_dripstone"
            | "nether_portal"
            | "end_portal"
    )
}

/// Расстояние от глаз до ближайшей точки AABB сущности (как ванильный reach, а не до центра).
fn aabb_dist(eye: [f64; 3], pos: [f64; 3], w: f64, h: f64) -> f64 {
    let lo = [pos[0] - w / 2.0, pos[1], pos[2] - w / 2.0];
    let hi = [pos[0] + w / 2.0, pos[1] + h, pos[2] + w / 2.0];
    (0..3)
        .map(|i| (eye[i].clamp(lo[i], hi[i]) - eye[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Точка прицела в клетке `bp`: ближайшая к центру из сетки 3×3×3, луч в которую первым попадает именно в `bp`
/// (центр тонкого блока вроде лепестка часто упирается в соседний). Нет такой — центр.
fn best_aim(l: &Level, eye: [f64; 3], bp: P) -> [f64; 3] {
    let c = [
        f64::from(bp[0]) + 0.5,
        f64::from(bp[1]) + 0.5,
        f64::from(bp[2]) + 0.5,
    ];
    let mut cands: Vec<[f64; 3]> = Vec::with_capacity(32);
    // Сначала центры реальных боксов контура (лепесток — четверть клетки, свечи, плиты и т. п.).
    let st = l.chunks.block(bp[0], bp[1], bp[2]);
    let off = ezzmc_mesh::blockinfo::outline_offset(st, bp[0], bp[2]);
    for b in ezzmc_mesh::blockinfo::outline_boxes(st) {
        cands.push([
            f64::from(bp[0]) + f64::from(off[0]) + (b[0] + b[3]) / 2.0,
            f64::from(bp[1]) + f64::from(off[1]) + (b[1] + b[4]) / 2.0,
            f64::from(bp[2]) + f64::from(off[2]) + (b[2] + b[5]) / 2.0,
        ]);
    }
    for dx in [0.0, -0.35, 0.35] {
        for dy in [0.0, -0.35, 0.35] {
            for dz in [0.0, -0.35, 0.35] {
                cands.push([c[0] + dx, c[1] + dy, c[2] + dz]);
            }
        }
    }
    for t in cands {
        let d =
            ((t[0] - eye[0]).powi(2) + (t[1] - eye[1]).powi(2) + (t[2] - eye[2]).powi(2)).sqrt();
        if d < 1e-6 {
            continue;
        }
        let dir = [
            ((t[0] - eye[0]) / d) as f32,
            ((t[1] - eye[1]) / d) as f32,
            ((t[2] - eye[2]) / d) as f32,
        ];
        if crate::pick::raycast_t(l, eye, dir, false, d + 1.0).is_some_and(|(h, _)| h.pos == bp) {
            return t;
        }
    }
    c
}

fn r1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn r2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn props_json(s: BlockState) -> Value {
    let info = s.block();
    let mut m = serde_json::Map::new();
    if let Some((_, idx)) =
        ezzmc_world::block::decode_state(ezzmc_world::generated::b26_3::BLOCKS, s.id())
    {
        for (p, i) in info.props.iter().zip(idx) {
            if let Some(v) = p.values.get(i as usize) {
                m.insert(p.name.to_string(), json!(v));
            }
        }
    }
    Value::Object(m)
}

fn block_json(l: &Level, p: P) -> Value {
    let s = l.chunks.block(p[0], p[1], p[2]);
    let loaded = l.chunks.get(p[0] >> 4, p[2] >> 4).is_some();
    json!({
        "pos": p, "block": name_of(s), "props": props_json(s), "loaded": loaded,
        "air": s.is_air(), "solid": top(s) > 0.0, "water": is_water(s), "hazard": hazard(s),
    })
}

/// Углы (yaw, pitch) от точки `from` на точку `to` (ваниль: yaw 0 = юг +Z, 90 = запад −X).
fn angles(from: [f64; 3], to: [f64; 3]) -> (f32, f32) {
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let yaw = (-dx).atan2(dz).to_degrees();
    let pitch = -dy.atan2((dx * dx + dz * dz).sqrt()).to_degrees();
    (yaw as f32, pitch as f32)
}

fn cardinal(yaw: f32) -> &'static str {
    match (yaw.rem_euclid(360.0) + 45.0) as i32 / 90 % 4 {
        0 => "south(+z)",
        1 => "west(-x)",
        2 => "north(-z)",
        _ => "east(+x)",
    }
}

fn wrap180(a: f32) -> f32 {
    (a + 180.0).rem_euclid(360.0) - 180.0
}

fn item_name(item: u32) -> &'static str {
    ezzmc_protocol::generated::r26_3::ITEM
        .get(item as usize)
        .copied()
        .unwrap_or("?")
}

/// Символы ASCII-карты: частые блоки — фиксированные, остальным выдаём свободные буквы.
struct Legend {
    map: HashMap<&'static str, char>,
    next: Vec<char>,
}

impl Legend {
    fn new() -> Self {
        let mut next: Vec<char> = "ABCDEFGHIJKMNOPQRSUVWXYZabdefhijkmnopqruvwxyz0123456789"
            .chars()
            .collect();
        next.reverse();
        Legend {
            map: HashMap::new(),
            next,
        }
    }

    fn sym(&mut self, s: BlockState) -> char {
        if s.is_air() {
            return '.';
        }
        let n = s.name();
        let fixed = match n {
            "stone" | "deepslate" | "andesite" | "diorite" | "granite" | "tuff" => '#',
            "dirt" | "coarse_dirt" | "rooted_dirt" | "podzol" | "mud" => 'd',
            "grass_block" | "mycelium" => 'g',
            "short_grass" | "tall_grass" | "fern" => '"',
            "water" | "bubble_column" => '~',
            "lava" => '%',
            "sand" | "red_sand" | "sandstone" => 's',
            "gravel" => 'v',
            "cobblestone" => 'c',
            "snow" | "snow_block" | "powder_snow" => '*',
            "ice" | "packed_ice" | "blue_ice" => 'i',
            "bedrock" => 'X',
            "netherrack" => 'n',
            "torch" | "wall_torch" => 't',
            "chest" | "trapped_chest" | "barrel" => 'H',
            "ladder" => 'h',
            "crafting_table" => 'T',
            "furnace" => 'F',
            "glass" | "glass_pane" => 'G',
            _ if n.ends_with("_log") || n.ends_with("_wood") || n.ends_with("_stem") => 'L',
            _ if n.ends_with("_leaves") => 'l',
            _ if n.ends_with("_planks") => 'p',
            _ if n.ends_with("_ore") => 'o',
            _ if n.ends_with("_door") => 'D',
            _ if n.ends_with("_slab") => '_',
            _ if n.ends_with("_stairs") => '/',
            _ if n.ends_with("_fence") || n.ends_with("_fence_gate") || n.ends_with("_wall") => '=',
            _ if n.ends_with("_bed") => 'b',
            _ if n.ends_with("_sign") => 'S',
            _ => '\0',
        };
        if fixed != '\0' {
            // Несколько блоков делят один символ: легенда хранит один пример.
            self.map.entry(n).or_insert(fixed);
            return fixed;
        }
        if let Some(c) = self.map.get(n) {
            return *c;
        }
        let c = self.next.pop().unwrap_or('?');
        self.map.insert(n, c);
        c
    }

    fn json(&self) -> Value {
        let mut m: std::collections::BTreeMap<String, Vec<&str>> = Default::default();
        for (n, c) in &self.map {
            m.entry(c.to_string()).or_default().push(n);
        }
        let mut out = serde_json::Map::new();
        for (c, mut names) in m {
            names.sort_unstable();
            out.insert(c, json!(names.join(", ")));
        }
        out.insert(".".into(), json!("air"));
        out.insert("?".into(), json!("chunk not loaded"));
        out.insert("@".into(), json!("you"));
        Value::Object(out)
    }
}

impl App {
    /// Сжатая картинка мира для `state`: время, погода, цель под прицелом, здоровье, предметы в руках.
    pub(crate) fn ctl_state_extra(&self, s: &mut Value) {
        let Some(v) = &self.view else { return };
        s["health"] = json!(v.health);
        s["hurt_time"] = json!(v.player.hurt_time);
        s["saturation"] = json!(v.saturation);
        s["xp_level"] = json!(v.xp_level);
        s["eye"] = json!(v.cam);
        s["facing"] = json!(cardinal(self.yaw));
        s["block_pos"] = json!([
            v.player.pos[0].floor() as i32,
            v.player.pos[1].floor() as i32,
            v.player.pos[2].floor() as i32
        ]);
        s["in_water"] = json!(v.player.in_water);
        s["in_lava"] = json!(v.player.in_lava);
        s["collided_horizontally"] = json!(v.player.horizontal_collision);
        let held = |i: usize| {
            v.inv
                .slots
                .get(i)
                .and_then(|s| s.as_ref())
                .map(|s| json!({"item": item_name(s.item), "count": s.count}))
        };
        s["held"] = json!(held(36 + usize::from(v.inv.selected)));
        s["offhand"] = json!(held(45));
        s["armor"] = json!([held(5), held(6), held(7), held(8)]);
        s["task"] = json!(self.control_task.as_ref().map(|t| match &t.kind {
            Kind::Goto {
                goal, idx, path, ..
            } => json!({"kind": "goto", "goal": goal, "step": idx, "of": path.len()}),
            Kind::Mine { pos, .. } => json!({"kind": "mine", "pos": pos}),
            Kind::Use { pos, .. } => json!({"kind": "use", "pos": pos}),
            Kind::Attack { id, .. } => json!({"kind": "attack", "id": id}),
        }));
        if let Ok(g) = v.net.level.lock() {
            if let Some(l) = g.as_ref() {
                s["world"] = json!({
                    "dimension": crate::itemdyn::dimension(),
                    "day_time": l.day_time.rem_euclid(24000),
                    "game_time": l.game_time,
                    "raining": l.raining,
                    "thunder": l.thunder_level > 0.5,
                    "loaded_chunks": l.chunks.loaded_count(),
                    "own_id": l.own_id,
                });
            }
        }
        s["target"] = self.ctl_target_json();
    }

    /// Что под прицелом: блок (с гранью и точкой), жидкость, сущность.
    pub(crate) fn ctl_target_json(&self) -> Value {
        let Some(v) = &self.view else {
            return Value::Null;
        };
        let g = v.net.level.lock().ok();
        let l = g.as_ref().and_then(|g| g.as_ref());
        let eye = v.cam;
        let dist = |h: [f64; 3]| {
            r2(
                ((h[0] - eye[0]).powi(2) + (h[1] - eye[1]).powi(2) + (h[2] - eye[2]).powi(2))
                    .sqrt(),
            )
        };
        let blk = |t: &crate::pick::Target| {
            json!({
                "pos": t.pos, "block": name_of(t.state), "props": props_json(t.state),
                "face": FACES[usize::from(t.face).min(5)], "hit": t.hit, "dist": dist(t.hit),
                "in_reach": dist(t.hit) <= self.ctl_reach(),
            })
        };
        let ent = v.entity_target.and_then(|id| {
            let l = l?;
            let e = l.entities.map.get(&id)?;
            Some(json!({"id": id, "type": ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty), "pos": e.pos, "dist": r2(v.entity_dist)}))
        });
        json!({
            "block": v.target.as_ref().map(blk),
            "far_block": v.look_target.as_ref().map(blk),
            "fluid": v.fluid_target.as_ref().map(blk),
            "entity": ent,
            "reach": self.ctl_reach(),
        })
    }

    /// Что в кадре и нет ли проблем с наведением (для скриншота и сверки с эталоном).
    pub(crate) fn ctl_view_report(&self) -> Value {
        let Some(v) = &self.view else {
            return Value::Null;
        };
        let mut warn: Vec<String> = Vec::new();
        let mut ents = Vec::new();
        let mut inside = false;
        if let Ok(g) = v.net.level.lock() {
            if let Some(l) = g.as_ref() {
                inside = top(l.chunks.block(
                    v.cam[0].floor() as i32,
                    v.cam[1].floor() as i32,
                    v.cam[2].floor() as i32,
                )) > 0.9;
                for e in v.visible_entities(l) {
                    if e["in_view"] == true
                        && e["type"] != "item"
                        && e["dist"].as_f64().is_some_and(|d| d <= 32.0)
                    {
                        ents.push(json!({"id": e["id"], "type": e["type"], "dist": e["dist"], "reach_dist": e["reach_dist"], "in_reach": e["in_reach"], "x": e["x"], "y": e["y"]}));
                    }
                }
            }
        }
        ents.truncate(10);
        let wall = v.look_target.as_ref().map(|t| {
            let h = t.hit;
            let d =
                ((h[0] - v.cam[0]).powi(2) + (h[1] - v.cam[1]).powi(2) + (h[2] - v.cam[2]).powi(2))
                    .sqrt();
            (name_of(t.state), r1(d))
        });
        if inside {
            warn.push("camera is inside a solid block: the frame is just that block".into());
        }
        if let Some((n, d)) = wall {
            if d < 2.0 && ents.is_empty() {
                warn.push(format!("staring at {n} from {d} blocks: the frame is mostly a wall; turn away (game_look_at / game_scan) or step back"));
            }
        }
        if self.hud_hidden {
            warn.push("HUD hidden (F1)".into());
        }
        json!({
            "yaw": self.yaw, "pitch": self.pitch, "pos": v.player.pos, "perspective": v.perspective,
            "looking_at": wall.map(|(n, d)| json!({"block": n, "dist": d})),
            "entities_in_view": ents, "camera_inside_block": inside, "warnings": warn,
        })
    }

    /// Точка съёмки: клетка на кольце вокруг объекта (dist ± 1), где можно стоять и откуда объект виден
    /// без блоков на луче. `from_yaw` — предпочтительная сторона (куда смотрит камера от объекта), иначе ближайшая к игроку.
    fn ctl_frame_plan(&self, c: &Value) -> Value {
        let dist = c["dist"].as_f64().unwrap_or(4.0).clamp(1.5, 24.0);
        let from_yaw = c["from_yaw"].as_f64();
        let Some(v) = &self.view else {
            return err("not in game");
        };
        let me = v.player.pos;
        let subj = if let Some(id) = c["entity"].as_i64() {
            match self.with_level(|l, _| {
                let e = l.entities.map.get(&(id as i32))?;
                let name = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty)?;
                let h = ezzmc_world::entity_size::size(name).map_or(0.5, |s| f64::from(s.1));
                Some([e.pos[0], e.pos[1] + h * 0.6, e.pos[2]])
            }) {
                Ok(Some(p)) => p,
                Ok(None) => return err("no such entity"),
                Err(e) => return e,
            }
        } else if let Some(p) = self.ctl_pos(c) {
            [
                f64::from(p[0]) + 0.5,
                f64::from(p[1]) + 0.5,
                f64::from(p[2]) + 0.5,
            ]
        } else {
            return err("x,y,z or entity required");
        };
        let r = self.with_level(|l, _| {
            let mut best: Option<(f64, P, f64)> = None;
            for ring in [dist, dist - 1.0, dist + 1.0, dist - 2.0, dist + 2.0] {
                if ring < 1.0 {
                    continue;
                }
                for k in 0..24 {
                    let a = f64::from(k) * 15.0_f64.to_radians();
                    let (x, z) = (subj[0] + a.sin() * ring, subj[2] - a.cos() * ring);
                    let (bx, bz) = (x.floor() as i32, z.floor() as i32);
                    if l.chunks.get(bx >> 4, bz >> 4).is_none() {
                        continue;
                    }
                    let Some(by) = (-3..=3)
                        .rev()
                        .map(|d| subj[1].floor() as i32 + d)
                        .find(|&y| standable(l, [bx, y, bz]))
                    else {
                        continue;
                    };
                    let eye = [
                        f64::from(bx) + 0.5,
                        f64::from(by) + 1.62,
                        f64::from(bz) + 0.5,
                    ];
                    let d = ((subj[0] - eye[0]).powi(2)
                        + (subj[1] - eye[1]).powi(2)
                        + (subj[2] - eye[2]).powi(2))
                    .sqrt();
                    let dir = [
                        ((subj[0] - eye[0]) / d) as f32,
                        ((subj[1] - eye[1]) / d) as f32,
                        ((subj[2] - eye[2]) / d) as f32,
                    ];
                    let sb = [
                        subj[0].floor() as i32,
                        subj[1].floor() as i32,
                        subj[2].floor() as i32,
                    ];
                    if crate::pick::raycast_t(l, eye, dir, false, d + 0.5)
                        .is_some_and(|(t, hd)| hd < d - 0.9 && t.pos != sb)
                    {
                        continue;
                    }
                    let walk = ((f64::from(bx) + 0.5 - me[0]).powi(2)
                        + (f64::from(bz) + 0.5 - me[2]).powi(2))
                    .sqrt();
                    let side = from_yaw.map_or(0.0, |fy| {
                        // Сторона съёмки: азимут точки от объекта (0 = севернее, 90 = западнее).
                        let ay = (-(a.sin())).atan2(a.cos()).to_degrees();
                        let dd = (ay - fy + 540.0).rem_euclid(360.0) - 180.0;
                        dd.abs() * 0.2
                    });
                    let score = (ring - dist).abs() * 3.0 + walk * 0.1 + side;
                    if best.is_none_or(|b| score < b.0) {
                        best = Some((score, [bx, by, bz], d));
                    }
                }
            }
            best
        });
        match r {
            Ok(Some((_, p, d))) => ok(json!({"stand": p, "subject": subj, "dist": r1(d)})),
            Ok(None) => err("no standable cell with a clear view of the subject at that distance: try a different dist, or the subject is enclosed"),
            Err(e) => e,
        }
    }

    fn ctl_reach(&self) -> f64 {
        match self.view.as_ref().map(|v| v.game_mode) {
            Some(1) => crate::pick::CREATIVE_BLOCK_REACH,
            _ => crate::pick::BLOCK_REACH,
        }
    }

    /// Команды наблюдения и задач; `None` — команда не из этого модуля.
    pub(crate) fn ctl_command(
        &mut self,
        c: &Value,
        reply: &Sender<Value>,
    ) -> Option<Option<Value>> {
        let cmd = c["cmd"].as_str().unwrap_or("");
        Some(Some(match cmd {
            "target" => ok(self.ctl_target_json()),
            "block_at" | "blocks_at" => self.ctl_block_at(c),
            "scan" => self.ctl_scan(c),
            "slice" => self.ctl_slice(c),
            "topdown" => self.ctl_topdown(c),
            "entities" => self.ctl_entities(c),
            "look_at" => self.ctl_look_at(c),
            "frame_plan" => self.ctl_frame_plan(c),
            "cancel" => {
                let had = self.control_task.is_some();
                self.ctl_finish(err("cancelled"));
                ok(json!({"cancelled": had}))
            }
            "goto" | "mine" | "use_block" | "attack" => return Some(self.ctl_start(cmd, c, reply)),
            _ => return None,
        }))
    }

    fn with_level<R>(
        &self,
        f: impl FnOnce(&Level, &crate::worldview::WorldView) -> R,
    ) -> Result<R, Value> {
        let v = self.view.as_ref().ok_or_else(|| err("not in game"))?;
        let g = v.net.level.lock().map_err(|_| err("level lock"))?;
        let l = g.as_ref().ok_or_else(|| err("нет уровня"))?;
        Ok(f(l, v))
    }

    fn ctl_pos(&self, c: &Value) -> Option<P> {
        let a = c["pos"].as_array();
        let get = |k: &str, i: usize| {
            c[k].as_f64()
                .or_else(|| a.and_then(|a| a.get(i)).and_then(Value::as_f64))
                .map(|x| x.floor() as i32)
        };
        Some([get("x", 0)?, get("y", 1)?, get("z", 2)?])
    }

    fn ctl_block_at(&self, c: &Value) -> Value {
        // Один блок {x,y,z} или список {list:[[x,y,z],…]}.
        let r = self.with_level(|l, _| {
            if let Some(list) = c["list"].as_array() {
                let out: Vec<Value> = list
                    .iter()
                    .filter_map(|p| {
                        let a = p.as_array()?;
                        let g = |i: usize| a.get(i)?.as_f64().map(|x| x.floor() as i32);
                        Some(block_json(l, [g(0)?, g(1)?, g(2)?]))
                    })
                    .collect();
                return Some(json!({"blocks": out}));
            }
            let p = self.ctl_pos(c)?;
            Some(block_json(l, p))
        });
        match r {
            Ok(Some(v)) => ok(v),
            Ok(None) => err("x, y, z required"),
            Err(e) => e,
        }
    }

    /// `find` — подстроки имён через запятую; без find — сводка по блокам вокруг.
    fn ctl_scan(&self, c: &Value) -> Value {
        let radius = c["radius"].as_i64().unwrap_or(16).clamp(1, 48) as i32;
        let limit = c["limit"].as_u64().unwrap_or(30) as usize;
        let finds: Vec<String> = c["find"]
            .as_str()
            .map(|s| {
                s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        let r = self.with_level(|l, v| {
            let me = v.player.pos;
            let c0 = [me[0].floor() as i32, me[1].floor() as i32, me[2].floor() as i32];
            let (min_y, max_y) = (l.chunks.format().min_y, l.chunks.format().min_y + l.chunks.format().height);
            let mut counts: HashMap<&'static str, u32> = HashMap::new();
            let mut hits: Vec<(f64, P, &'static str)> = Vec::new();
            for y in (c0[1] - radius).max(min_y)..=(c0[1] + radius).min(max_y - 1) {
                for z in c0[2] - radius..=c0[2] + radius {
                    for x in c0[0] - radius..=c0[0] + radius {
                        let s = l.chunks.block(x, y, z);
                        if s.is_air() {
                            continue;
                        }
                        let n = s.name();
                        if finds.is_empty() {
                            *counts.entry(n).or_default() += 1;
                        } else if finds.iter().any(|f| n.contains(f.as_str())) {
                            let d = ((f64::from(x) + 0.5 - me[0]).powi(2) + (f64::from(y) + 0.5 - me[1]).powi(2) + (f64::from(z) + 0.5 - me[2]).powi(2)).sqrt();
                            hits.push((d, [x, y, z], n));
                        }
                    }
                }
            }
            if finds.is_empty() {
                let mut v: Vec<_> = counts.into_iter().collect();
                v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
                json!({"radius": radius, "counts": v.into_iter().take(limit.max(30)).map(|(n, k)| json!({"block": n, "count": k})).collect::<Vec<_>>()})
            } else {
                hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
                let total = hits.len();
                let found: Vec<Value> = hits
                    .into_iter()
                    .take(limit)
                    .map(|(d, p, n)| {
                        let (yaw, pitch) = angles(v.cam, [f64::from(p[0]) + 0.5, f64::from(p[1]) + 0.5, f64::from(p[2]) + 0.5]);
                        json!({"block": n, "pos": p, "dist": r1(d), "yaw_to": r1(f64::from(yaw)), "pitch_to": r1(f64::from(pitch))})
                    })
                    .collect();
                json!({"radius": radius, "total": total, "found": found})
            }
        });
        r.map_or_else(|e| e, ok)
    }

    /// Горизонтальные срезы: строки с севера (−z) на юг (+z), колонки с запада (−x) на восток (+x).
    fn ctl_slice(&self, c: &Value) -> Value {
        let radius = c["radius"].as_i64().unwrap_or(8).clamp(1, 24) as i32;
        let r = self.with_level(|l, v| {
            let me = v.player.pos;
            let c0 = [me[0].floor() as i32, me[1].floor() as i32, me[2].floor() as i32];
            let ys: Vec<i32> = match c["ys"].as_array() {
                Some(a) => a.iter().filter_map(Value::as_i64).map(|y| y as i32).collect(),
                // по умолчанию: пол под ногами, ноги, голова
                None => {
                    let d = c["dy"].as_array().map_or(vec![-1, 0, 1], |a| a.iter().filter_map(Value::as_i64).map(|x| x as i32).collect());
                    d.into_iter().map(|d| c0[1] + d).collect()
                }
            };
            let mut lg = Legend::new();
            let layers: Vec<Value> = ys
                .iter()
                .map(|&y| {
                    let rows: Vec<String> = (c0[2] - radius..=c0[2] + radius)
                        .map(|z| {
                            (c0[0] - radius..=c0[0] + radius)
                                .map(|x| {
                                    if x == c0[0] && z == c0[2] && y >= c0[1] && y <= c0[1] + 1 {
                                        '@'
                                    } else if l.chunks.get(x >> 4, z >> 4).is_none() {
                                        '?'
                                    } else {
                                        lg.sym(l.chunks.block(x, y, z))
                                    }
                                })
                                .collect()
                        })
                        .collect();
                    json!({"y": y, "rel": y - c0[1], "rows": rows})
                })
                .collect();
            json!({
                "center": c0, "radius": radius, "layers": layers, "legend": lg.json(),
                "axes": "rows: north(-z) top -> south(+z) bottom; cols: west(-x) left -> east(+x) right; @ = you; yaw 0 faces south, 90 west, 180 north, 270 east",
            })
        });
        r.map_or_else(|e| e, ok)
    }

    /// Вид сверху: верхний твёрдый блок колонки + рельеф относительно стоп игрока.
    fn ctl_topdown(&self, c: &Value) -> Value {
        let radius = c["radius"].as_i64().unwrap_or(12).clamp(1, 40) as i32;
        let (up, down) = (
            c["up"].as_i64().unwrap_or(4) as i32,
            c["down"].as_i64().unwrap_or(14) as i32,
        );
        let r = self.with_level(|l, v| {
            let me = v.player.pos;
            let c0 = [me[0].floor() as i32, me[1].floor() as i32, me[2].floor() as i32];
            let mut lg = Legend::new();
            let (mut blocks, mut relief) = (Vec::new(), Vec::new());
            for z in c0[2] - radius..=c0[2] + radius {
                let (mut b, mut h) = (String::new(), String::new());
                for x in c0[0] - radius..=c0[0] + radius {
                    if x == c0[0] && z == c0[2] {
                        b.push('@');
                        h.push('@');
                        continue;
                    }
                    if l.chunks.get(x >> 4, z >> 4).is_none() {
                        b.push('?');
                        h.push('?');
                        continue;
                    }
                    let found = (c0[1] - down..=c0[1] + up).rev().find(|&y| !l.chunks.block(x, y, z).is_air());
                    match found {
                        Some(y) => {
                            b.push(lg.sym(l.chunks.block(x, y, z)));
                            // высота пола: dy = (y+1) - ноги; '.' = вровень
                            let dy = y + 1 - c0[1];
                            h.push(match dy {
                                0 => '.',
                                1..=9 => char::from(b'0' + dy as u8),
                                10.. => 'U',
                                -9..=-1 => char::from(b'a' + (-dy - 1) as u8),
                                _ => 'D',
                            });
                        }
                        None => {
                            b.push(' ');
                            h.push('D');
                        }
                    }
                }
                blocks.push(b);
                relief.push(h);
            }
            json!({
                "center": c0, "radius": radius, "blocks": blocks, "relief": relief, "legend": lg.json(),
                "relief_legend": ". same level as your feet floor; 1-9 floor is N blocks higher (U >9); a-i floor is 1-9 lower (a=1 lower, D deeper/empty); ? unloaded",
                "axes": "rows north(-z) top -> south(+z); cols west(-x) -> east(+x)",
            })
        });
        r.map_or_else(|e| e, ok)
    }

    /// Все сущности вокруг (не только в кадре): координаты, расстояние, углы для `look`, поле зрения.
    fn ctl_entities(&self, c: &Value) -> Value {
        let max_dist = c["max_dist"].as_f64().unwrap_or(32.0);
        let ty = c["type"].as_str();
        let items = c["items"].as_bool().unwrap_or(true);
        let limit = c["limit"].as_u64().unwrap_or(40) as usize;
        let r = self.with_level(|l, v| {
            let eye = v.cam;
            let mut out: Vec<(f64, Value)> = Vec::new();
            for e in l.entities.iter() {
                if e.id == l.own_id {
                    continue;
                }
                let Some(name) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty) else { continue };
                if !items && *name == "item" {
                    continue;
                }
                if ty.is_some_and(|t| !name.contains(t)) {
                    continue;
                }
                let h = ezzmc_world::entity_size::size(name).map_or(0.5, |s| f64::from(s.1));
                let mid = [e.pos[0], e.pos[1] + h * 0.5, e.pos[2]];
                let d = ((mid[0] - eye[0]).powi(2) + (mid[1] - eye[1]).powi(2) + (mid[2] - eye[2]).powi(2)).sqrt();
                if d > max_dist {
                    continue;
                }
                let (yaw, pitch) = angles(eye, mid);
                let w = ezzmc_world::entity_size::size(name).map_or(0.5, |s| f64::from(s.0));
                let mut j = json!({
                    "id": e.id, "type": name, "pos": e.pos, "dist": r1(d), "reach_dist": r1(aabb_dist(eye, e.pos, w, h)), "in_reach": aabb_dist(eye, e.pos, w, h) <= 3.0,
                    "yaw": r1(f64::from(e.yaw)), "head_yaw": r1(f64::from(e.head_yaw)),
                    "dx": r1(e.pos[0] - v.player.pos[0]), "dy": r1(e.pos[1] - v.player.pos[1]), "dz": r1(e.pos[2] - v.player.pos[2]),
                    "yaw_to": r1(f64::from(yaw)), "pitch_to": r1(f64::from(pitch)),
                    "turn": r1(f64::from(wrap180(yaw - self.yaw))),
                    "in_fov": wrap180(yaw - self.yaw).abs() < 55.0,
                });
                if let Some(s) = &e.item {
                    j["item"] = json!({"item": item_name(s.item), "count": s.count});
                }
                if *name == "player" {
                    j["name"] = json!(v.tab.profiles.get(&e.uuid).or_else(|| v.tab.entries.get(&e.uuid).map(|x| &x.name)));
                }
                if e.baby {
                    j["baby"] = json!(true);
                }
                if e.flags & 1 != 0 {
                    j["sneaking"] = json!(true);
                }
                if *name == "ender_dragon" {
                    j["vel"] = json!(e.vel);
                    j["phase"] = json!(e.int_meta(16));
                    j["mob_flags"] = json!(e.mob_flags);
                }
                out.push((d, j));
            }
            out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
            let total = out.len();
            json!({"entities": out.into_iter().take(limit).map(|x| x.1).collect::<Vec<_>>(), "total": total,
                "note": "dist = eye to box centre; reach_dist = eye to nearest point of the hitbox (compare with reach 3.0); turn = degrees to add to yaw (game_turn dyaw) to face it; yaw_to/pitch_to = absolute angles for game_look"})
        });
        r.map_or_else(|e| e, ok)
    }

    /// Развернуть взгляд на точку {x,y,z} (центр блока при целых координатах, face — на центр грани), сущность {entity}.
    fn ctl_look_at(&mut self, c: &Value) -> Value {
        let Some(v) = &self.view else {
            return err("not in game");
        };
        let eye = v.cam;
        let to = if let Some(id) = c["entity"].as_i64() {
            let r = self.with_level(|l, _| {
                let e = l.entities.map.get(&(id as i32))?;
                let name = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty)?;
                let h = ezzmc_world::entity_size::size(name).map_or(0.5, |s| f64::from(s.1));
                Some([e.pos[0], e.pos[1] + h * 0.8, e.pos[2]])
            });
            match r {
                Ok(Some(p)) => p,
                Ok(None) => return err("no such entity"),
                Err(e) => return e,
            }
        } else if let Some(p) = self.ctl_pos(c) {
            let face = c["face"]
                .as_str()
                .and_then(|f| FACES.iter().position(|x| *x == f));
            let mut t = [
                f64::from(p[0]) + 0.5,
                f64::from(p[1]) + 0.5,
                f64::from(p[2]) + 0.5,
            ];
            if let Some(f) = face {
                for i in 0..3 {
                    t[i] += f64::from(NORMALS[f][i]) * 0.5;
                }
            }
            if face.is_none()
                && ["x", "y", "z"]
                    .iter()
                    .all(|k| c[*k].as_f64().is_none_or(|x| x.fract() == 0.0))
            {
                if let Ok(a) = self.with_level(|l, _| best_aim(l, eye, p)) {
                    t = a;
                }
            }
            // дробные координаты — точная точка
            for (i, k) in ["x", "y", "z"].iter().enumerate() {
                if let Some(x) = c[*k].as_f64() {
                    if x.fract() != 0.0 && face.is_none() {
                        t[i] = x;
                    }
                }
            }
            t
        } else {
            return err("x,y,z or entity required");
        };
        let (yaw, pitch) = angles(eye, to);
        self.yaw = self.yaw + wrap180(yaw - self.yaw); // кратчайший поворот: без скачка ≈360° (Grim AimModulo360)
        self.pitch = pitch.clamp(-90.0, 90.0);
        // Прямая видимость: что ближайшее на луче к точке.
        let los = self.with_level(|l, _| {
            let d =
                ((to[0] - eye[0]).powi(2) + (to[1] - eye[1]).powi(2) + (to[2] - eye[2]).powi(2))
                    .sqrt();
            let dir = [
                ((to[0] - eye[0]) / d) as f32,
                ((to[1] - eye[1]) / d) as f32,
                ((to[2] - eye[2]) / d) as f32,
            ];
            let hit = crate::pick::raycast_t(l, eye, dir, false, d + 1.0);
            let blocked = hit.filter(|(t, hd)| {
                *hd < d - 0.9
                    && t.pos
                        != [
                            to[0].floor() as i32,
                            to[1].floor() as i32,
                            to[2].floor() as i32,
                        ]
            });
            (
                r1(d),
                blocked
                    .map(|(t, hd)| json!({"block": name_of(t.state), "pos": t.pos, "at": r1(hd)})),
            )
        });
        let (dist, blocker) = los.unwrap_or((0.0, None));
        ok(
            json!({"yaw": yaw, "pitch": self.pitch, "dist": dist, "clear": blocker.is_none(), "blocked_by": blocker}),
        )
    }

    // ---------------------------------------------------------------- задачи

    fn ctl_key(&mut self, i: usize, down: bool) {
        let Some(t) = self.control_task.as_mut() else {
            return;
        };
        if t.held[i] == down {
            return;
        }
        t.held[i] = down;
        if let Some(b) = self.keys.get(KEYS[i]) {
            let input: Input = b.key;
            self.control_press(input, down);
        }
    }

    fn ctl_release_all(&mut self) {
        for i in 0..4 {
            self.ctl_key(i, false);
        }
    }

    /// Завершить задачу: отпустить ввод и отправить ответ.
    pub(crate) fn ctl_finish(&mut self, answer: Value) {
        self.ctl_release_all();
        if let Some(t) = self.control_task.take() {
            let _ = t.reply.send(answer);
        }
    }

    fn ctl_start(&mut self, cmd: &str, c: &Value, reply: &Sender<Value>) -> Option<Value> {
        if self.view.is_none() || self.menu_active || self.inv_open || self.chat.open {
            return Some(err("not in play screen (close inventory/chat/menu first)"));
        }
        if self.control_task.is_some() {
            return Some(err("another task is running: call cancel or wait"));
        }
        let now = Instant::now();
        let timeout = c["timeout_ms"]
            .as_u64()
            .map(|t| t.min(110_000))
            .unwrap_or(match cmd {
                "goto" => 60_000,
                "mine" => 20_000,
                _ => 8_000,
            });
        let kind = match cmd {
            "goto" => {
                let Some(goal) = self.ctl_pos(c) else {
                    return Some(err("x, y, z required"));
                };
                let mut range = c["range"].as_f64().unwrap_or(0.0);
                // Цель не годится для стояния (лист, вода сверху, стена) — подходим на 1.5 блока.
                if range <= 0.0 && matches!(self.with_level(|l, _| standable(l, goal)), Ok(false)) {
                    range = 1.5;
                }
                let from = self.view.as_ref().map(|v| v.player.pos).unwrap_or([0.0; 3]);
                let path = match self.with_level(|l, _| find_path(l, from, goal, range)) {
                    Ok(Ok(p)) => p,
                    Ok(Err(e)) => return Some(err(e)),
                    Err(e) => return Some(e),
                };
                if path.len() <= 1 {
                    return Some(ok(json!({"arrived": true, "note": "already there"})));
                }
                Kind::Goto {
                    path,
                    idx: 1,
                    goal,
                    range,
                    sprint: c["sprint"].as_bool().unwrap_or(true),
                    replans: 0,
                    last_move: (now, from),
                }
            }
            "mine" => {
                let Some(pos) = self.ctl_pos(c) else {
                    return Some(err("x, y, z required"));
                };
                let was = match self.with_level(|l, _| l.chunks.block(pos[0], pos[1], pos[2])) {
                    Ok(s) if s.is_air() => return Some(err("block is already air")),
                    Ok(s) => s.name().to_string(),
                    Err(e) => return Some(e),
                };
                Kind::Mine { pos, was }
            }
            "use_block" => {
                let Some(pos) = self.ctl_pos(c) else {
                    return Some(err("x, y, z required"));
                };
                let face = c["face"]
                    .as_str()
                    .and_then(|f| FACES.iter().position(|x| *x == f))
                    .map(|f| f as u8);
                Kind::Use {
                    pos,
                    face,
                    phase: 0,
                    at: now,
                    ms: c["ms"].as_u64().unwrap_or(100),
                    placed_at: None,
                }
            }
            _ => {
                let Some(id) = c["id"].as_i64() else {
                    return Some(err("id required (game_entities)"));
                };
                Kind::Attack {
                    id: id as i32,
                    left: c["hits"].as_u64().unwrap_or(1) as u32,
                    done: 0,
                    pressed: false,
                }
            }
        };
        self.control_task = Some(Task {
            kind,
            reply: reply.clone(),
            started: now,
            until: now + Duration::from_millis(timeout),
            held: [false; 4],
        });
        None
    }

    /// Каждый кадр: продвинуть текущую задачу.
    pub(crate) fn ctl_task_tick(&mut self) {
        let Some(task) = self.control_task.as_ref() else {
            return;
        };
        if self.view.is_none() || self.menu_active || self.inv_open || self.chat.open {
            self.ctl_finish(err(
                "screen changed (menu/inventory/chat opened), task aborted",
            ));
            return;
        }
        let (now, expired) = (Instant::now(), Instant::now() >= task.until);
        let dead = self.view.as_ref().is_some_and(|v| v.health <= 0.0);
        if dead {
            self.ctl_finish(err("player is dead"));
            return;
        }
        let Some(mut kind) = self.control_task.as_mut().map(|t| {
            std::mem::replace(
                &mut t.kind,
                Kind::Attack {
                    id: 0,
                    left: 0,
                    done: 0,
                    pressed: false,
                },
            )
        }) else {
            return;
        };
        let outcome = self.ctl_step(&mut kind, now, expired);
        if let Some(t) = self.control_task.as_mut() {
            t.kind = kind;
        }
        if let Some(a) = outcome {
            self.ctl_finish(a);
        }
    }

    /// Один шаг задачи; `Some(ответ)` — задача завершена.
    fn ctl_step(&mut self, kind: &mut Kind, now: Instant, expired: bool) -> Option<Value> {
        let pos = self.view.as_ref()?.player.pos;
        let eye = self.view.as_ref()?.cam;
        match kind {
            Kind::Goto {
                path,
                idx,
                goal,
                range,
                sprint,
                replans,
                last_move,
            } => {
                let g = [
                    f64::from(goal[0]) + 0.5,
                    f64::from(goal[1]),
                    f64::from(goal[2]) + 0.5,
                ];
                let dist_goal = ((pos[0] - g[0]).powi(2) + (pos[2] - g[2]).powi(2)).sqrt();
                let dy_goal = (pos[1] - g[1]).abs();
                let arrived = *idx >= path.len()
                    || (*range > 0.0 && dist_goal <= *range && dy_goal <= (*range).max(1.5));
                if arrived {
                    return Some(ok(
                        json!({"arrived": true, "pos": pos, "ms": self.control_task.as_ref().map_or(0, |t| t.started.elapsed().as_millis() as u64)}),
                    ));
                }
                if expired {
                    return Some(err(format!(
                        "timeout, still {dist_goal:.1} blocks from the goal at ({:.1},{:.1},{:.1})",
                        pos[0], pos[1], pos[2]
                    )));
                }
                // Не двигаемся — перепланировать маршрут (дверь, упавший блок, застряли).
                let moved = ((pos[0] - last_move.1[0]).powi(2)
                    + (pos[1] - last_move.1[1]).powi(2)
                    + (pos[2] - last_move.1[2]).powi(2))
                .sqrt();
                if moved > 0.25 {
                    *last_move = (now, pos);
                } else if now.duration_since(last_move.0) > Duration::from_millis(1800) {
                    if *replans >= 3 {
                        return Some(err(format!(
                            "stuck at ({:.1},{:.1},{:.1}) heading to waypoint {:?}",
                            pos[0],
                            pos[1],
                            pos[2],
                            path.get(*idx)
                        )));
                    }
                    *replans += 1;
                    let (goal, range) = (*goal, *range);
                    match self.with_level(|l, _| find_path(l, pos, goal, range)) {
                        Ok(Ok(p)) => {
                            *path = p;
                            *idx = 1;
                        }
                        Ok(Err(e)) => return Some(err(format!("stuck, replan failed: {e}"))),
                        Err(e) => return Some(e),
                    }
                    *last_move = (now, pos);
                    return None;
                }
                // Пропускаем достигнутые путевые точки.
                while *idx < path.len() {
                    let w = path[*idx];
                    let (dx, dz) = (
                        f64::from(w[0]) + 0.5 - pos[0],
                        f64::from(w[2]) + 0.5 - pos[2],
                    );
                    let near =
                        dx * dx + dz * dz < 0.3 * 0.3 && (pos[1] - f64::from(w[1])).abs() < 1.1;
                    if near
                        || (*idx + 1 < path.len()
                            && dx * dx + dz * dz < 0.05
                            && pos[1] >= f64::from(w[1]) - 0.1)
                    {
                        *idx += 1;
                    } else {
                        break;
                    }
                }
                if *idx >= path.len() {
                    return None; // следующий кадр сообщит о прибытии
                }
                let w = path[*idx];
                let (wx, wz) = (f64::from(w[0]) + 0.5, f64::from(w[2]) + 0.5);
                let (yaw, _) = angles([pos[0], 0.0, pos[2]], [wx, 0.0, wz]);
                self.yaw = self.yaw + wrap180(yaw - self.yaw); // кратчайший поворот: без скачка ≈360° (Grim AimModulo360)
                self.pitch = 0.0;
                let rest = path.len() - *idx;
                let (on_ground, in_water, col) = self.view.as_ref().map(|v| {
                    (
                        v.player.on_ground,
                        v.player.in_water,
                        v.player.horizontal_collision,
                    )
                })?;
                let want_up = f64::from(w[1]) > pos[1] + 0.05;
                let (sprint_on, jump) = (
                    *sprint && rest > 2,
                    (want_up && on_ground) || (in_water && want_up) || (col && on_ground),
                );
                self.ctl_key(FWD, true);
                self.ctl_key(SPRINT, sprint_on);
                self.ctl_key(JUMP, jump || (in_water && f64::from(w[1]) >= pos[1] - 0.1));
                None
            }
            Kind::Mine { pos: bp, was } => {
                let (bp, was) = (*bp, was.clone());
                let cur = self
                    .with_level(|l, _| l.chunks.block(bp[0], bp[1], bp[2]))
                    .ok()?;
                if cur.is_air() || cur.name() != was {
                    return Some(ok(
                        json!({"broken": true, "block": was, "ms": self.control_task.as_ref().map_or(0, |t| t.started.elapsed().as_millis() as u64)}),
                    ));
                }
                let c = [
                    f64::from(bp[0]) + 0.5,
                    f64::from(bp[1]) + 0.5,
                    f64::from(bp[2]) + 0.5,
                ];
                let d =
                    ((c[0] - eye[0]).powi(2) + (c[1] - eye[1]).powi(2) + (c[2] - eye[2]).powi(2))
                        .sqrt();
                if d > self.ctl_reach() + 0.9 {
                    return Some(err(format!(
                        "too far: {d:.1} blocks (reach {}); use game_goto first",
                        self.ctl_reach()
                    )));
                }
                if expired {
                    return Some(err(
                        "timeout: block not broken (wrong tool? try another hotbar slot)",
                    ));
                }
                let c = self.with_level(|l, _| best_aim(l, eye, bp)).unwrap_or(c);
                let (yaw, pitch) = angles(eye, c);
                self.yaw = self.yaw + wrap180(yaw - self.yaw); // кратчайший поворот: без скачка ≈360° (Grim AimModulo360)
                self.pitch = pitch;
                // Если на линии взгляда другой блок — копаем его вместо стены (сообщаем в ответе при ошибке).
                let tgt = self.view.as_ref().and_then(|v| v.target.map(|t| t.pos));
                if let Some(t) = tgt {
                    if t != bp
                        && now.duration_since(self.control_task.as_ref()?.started)
                            > Duration::from_millis(400)
                    {
                        let n = self
                            .with_level(|l, _| l.chunks.block(t[0], t[1], t[2]).name())
                            .ok()?;
                        return Some(err(format!(
                            "line of sight blocked by {n} at {t:?}: mine it first"
                        )));
                    }
                }
                self.ctl_key(ATTACK, true);
                None
            }
            Kind::Use {
                pos: bp,
                face,
                phase,
                at,
                ms,
                placed_at,
            } => {
                let mut c = [
                    f64::from(bp[0]) + 0.5,
                    f64::from(bp[1]) + 0.5,
                    f64::from(bp[2]) + 0.5,
                ];
                if let Some(f) = *face {
                    for i in 0..3 {
                        c[i] += f64::from(NORMALS[usize::from(f)][i]) * 0.45;
                    }
                }
                let (yaw, pitch) = angles(eye, c);
                self.yaw = self.yaw + wrap180(yaw - self.yaw); // кратчайший поворот: без скачка ≈360° (Grim AimModulo360)
                self.pitch = pitch;
                match *phase {
                    0 => {
                        // даём тику физики выбрать place_target по новому взгляду
                        if now.duration_since(*at) > Duration::from_millis(160) {
                            let hit = self.view.as_ref().and_then(|v| v.place_target);
                            *placed_at = hit.map(|t| {
                                [
                                    t.pos[0] + NORMALS[usize::from(t.face).min(5)][0],
                                    t.pos[1] + NORMALS[usize::from(t.face).min(5)][1],
                                    t.pos[2] + NORMALS[usize::from(t.face).min(5)][2],
                                ]
                            });
                            if hit.is_none() {
                                return Some(err("no block under the crosshair for that point (too far / hidden behind another block)"));
                            }
                            let right = Input::Mouse(winit::event::MouseButton::Right);
                            self.control_press(right, true);
                            *phase = 1;
                            *at = now;
                        }
                        None
                    }
                    _ => {
                        if now.duration_since(*at) > Duration::from_millis(*ms) {
                            self.control_press(
                                Input::Mouse(winit::event::MouseButton::Right),
                                false,
                            );
                            let p = *placed_at;
                            let after =
                                p.and_then(|p| self.with_level(|l, _| block_json(l, p)).ok());
                            // ответ чуть позже, чтобы сервер успел подтвердить блок
                            if now.duration_since(*at) < Duration::from_millis(*ms + 250) {
                                return None;
                            }
                            return Some(ok(
                                json!({"used": true, "target_block": bp, "adjacent_block": after}),
                            ));
                        }
                        None
                    }
                }
            }
            Kind::Attack {
                id,
                left,
                done,
                pressed,
            } => {
                let id = *id;
                let ent = self
                    .with_level(|l, _| {
                        let e = l.entities.map.get(&id)?;
                        let name = ezzmc_protocol::generated::r26_3::ENTITY_TYPE.get(e.ty)?;
                        let sz = ezzmc_world::entity_size::size(name).unwrap_or((0.5, 0.5));
                        let (w, h) = (f64::from(sz.0), f64::from(sz.1));
                        Some(([e.pos[0], e.pos[1] + h * 0.7, e.pos[2]], e.pos, w, h))
                    })
                    .ok()?;
                let Some((t, tpos, tw, th)) = ent else {
                    return Some(ok(
                        json!({"hits": *done, "target_gone": true, "note": "entity no longer exists (dead or out of range)"}),
                    ));
                };
                let d =
                    ((t[0] - eye[0]).powi(2) + (t[1] - eye[1]).powi(2) + (t[2] - eye[2]).powi(2))
                        .sqrt();
                let reach_d = aabb_dist(eye, tpos, tw, th);
                if *left == 0 {
                    return Some(ok(json!({"hits": *done, "target_gone": false})));
                }
                if expired {
                    return Some(err(format!(
                        "timeout after {done} hits, target at {d:.1} blocks (nearest hitbox point {reach_d:.1})"
                    )));
                }
                let (yaw, pitch) = angles(eye, t);
                self.yaw = self.yaw + wrap180(yaw - self.yaw); // кратчайший поворот: без скачка ≈360° (Grim AimModulo360)
                self.pitch = pitch;
                if *pressed {
                    *pressed = false;
                    self.ctl_key(ATTACK, false);
                    *left -= 1;
                    *done += 1;
                    return None;
                }
                let v = self.view.as_ref()?;
                if v.entity_target == Some(id) && v.attack_strength(0.0) >= 0.9 {
                    *pressed = true;
                    self.ctl_key(ATTACK, true);
                } else if reach_d > 2.6 {
                    // подойти ближе, пока цель дальше досягаемости
                    self.pitch = 0.0;
                    self.ctl_key(FWD, true);
                } else {
                    self.ctl_key(FWD, false);
                }
                None
            }
        }
    }
}

// -------------------------------------------------------------------- поиск пути (A*)

/// Проходимая клетка для тела (ноги/голова): без коллизии и без вреда.
fn free(l: &Level, p: P) -> bool {
    let s = l.chunks.block(p[0], p[1], p[2]);
    top(s) <= 0.0 && !hazard(s)
}

/// Можно стоять в `p`: ноги свободны (или ступенька ≤0.6), голова свободна, под ногами опора/вода.
fn standable(l: &Level, p: P) -> bool {
    let feet = l.chunks.block(p[0], p[1], p[2]);
    let head = l.chunks.block(p[0], p[1] + 1, p[2]);
    let below = l.chunks.block(p[0], p[1] - 1, p[2]);
    if hazard(feet)
        || hazard(head)
        || top(head) > 0.0
        || top(feet) > 0.6
        || hazard(below) && !is_water(feet)
    {
        return false;
    }
    top(below) > 0.0 || top(feet) > 0.0 || is_water(feet) || is_water(below)
}

fn find_path(l: &Level, from: [f64; 3], goal: P, range: f64) -> Result<Vec<P>, String> {
    let start = [
        from[0].floor() as i32,
        (from[1] + 0.01).floor() as i32,
        from[2].floor() as i32,
    ];
    if l.chunks.get(goal[0] >> 4, goal[2] >> 4).is_none() {
        return Err("goal chunk is not loaded".into());
    }
    let h = |p: P| {
        let (dx, dy, dz) = (
            f64::from(p[0] - goal[0]),
            f64::from(p[1] - goal[1]),
            f64::from(p[2] - goal[2]),
        );
        (dx * dx + dz * dz).sqrt() + dy.abs() * 0.5
    };
    let reached = |p: P| {
        if range > 0.0 {
            let (dx, dz) = (f64::from(p[0] - goal[0]), f64::from(p[2] - goal[2]));
            (dx * dx + dz * dz).sqrt() <= range && (p[1] - goal[1]).abs() <= range.max(1.5) as i32
        } else {
            p == goal
        }
    };
    #[derive(PartialEq)]
    struct N(f64, P);
    impl Eq for N {}
    impl PartialOrd for N {
        fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
            Some(self.cmp(o))
        }
    }
    impl Ord for N {
        fn cmp(&self, o: &Self) -> Ordering {
            o.0.partial_cmp(&self.0).unwrap_or(Ordering::Equal)
        }
    }
    let mut open = BinaryHeap::new();
    let mut g: HashMap<P, f64> = HashMap::new();
    let mut from_: HashMap<P, P> = HashMap::new();
    g.insert(start, 0.0);
    open.push(N(h(start), start));
    let (mut expanded, mut best) = (0, (h(start), start));
    while let Some(N(_, cur)) = open.pop() {
        if reached(cur) {
            let mut path = vec![cur];
            let mut c = cur;
            while let Some(&p) = from_.get(&c) {
                path.push(p);
                c = p;
            }
            path.reverse();
            return Ok(path);
        }
        expanded += 1;
        if expanded > 40_000 {
            break;
        }
        let gc = g[&cur];
        let mut try_move = |n: P, cost: f64, open: &mut BinaryHeap<N>| {
            let ng = gc + cost;
            if g.get(&n).is_none_or(|&o| ng < o) {
                g.insert(n, ng);
                from_.insert(n, cur);
                open.push(N(ng + h(n), n));
                if h(n) < best.0 {
                    best = (h(n), n);
                }
            }
        };
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let t = [cur[0] + dx, cur[1], cur[2] + dz];
            // шаг на ту же высоту
            if standable(l, t) {
                let w = if is_water(l.chunks.block(t[0], t[1], t[2])) {
                    2.5
                } else {
                    1.0
                };
                try_move(t, w, &mut open);
            } else if free(l, t) && free(l, [t[0], t[1] + 1, t[2]]) {
                // обрыв: падаем до 3 блоков
                for k in 1..=3 {
                    let n = [t[0], t[1] - k, t[2]];
                    if !free(l, n) && !is_water(l.chunks.block(n[0], n[1], n[2])) {
                        break;
                    }
                    if standable(l, n) {
                        try_move(n, 1.2 + f64::from(k) * 0.3, &mut open);
                        break;
                    }
                }
            }
            // подъём на 1: нужен потолок над текущей клеткой и свободная колонка цели
            let up = [t[0], t[1] + 1, t[2]];
            if free(l, [cur[0], cur[1] + 2, cur[2]])
                && standable(l, up)
                && top(l.chunks.block(t[0], t[1], t[2])) > 0.6
            {
                try_move(up, 1.6, &mut open);
            }
        }
        // диагонали по ровному
        for (dx, dz) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let t = [cur[0] + dx, cur[1], cur[2] + dz];
            let a = [cur[0] + dx, cur[1], cur[2]];
            let b = [cur[0], cur[1], cur[2] + dz];
            let clear = |p: P| free(l, p) && free(l, [p[0], p[1] + 1, p[2]]);
            if standable(l, t) && clear(a) && clear(b) {
                try_move(t, 1.45, &mut open);
            }
        }
    }
    let (bh, bp) = best;
    Err(format!(
        "no walkable path (explored {expanded} cells); closest reachable cell {bp:?} is {bh:.1} blocks from the goal — it may be enclosed, over water/lava, or need digging"
    ))
}
