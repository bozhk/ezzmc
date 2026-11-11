//! Песочница: безоконный стенд физики/мира без Minecraft, сервера и GPU.
//!
//! Запуск (мгновенно, без `--connect`):
//!   `ezzmc-client --sandbox script.sb`  · `ezzmc-client --sandbox -e "fill 0 64 3 0 66 3 stone; tick 30 w"` ·
//!   `echo "…" | ezzmc-client --sandbox`. Справка: команда `help`. Код выхода 1, если был `expect`-провал/ошибка.
//!
//! Мир — настоящий `Level` (ваниль-коллизии, жидкости, формы блоков), игрок — настоящий `physics::Player`.
//! Скрипт: команды по строкам или через `;`, `#` — комментарий.
use crate::physics::{Collider, Input, Player};
use ezzmc_world::block::BlockState;
use ezzmc_world::chunk::{Chunk, ChunkFormat};
use ezzmc_world::chunk_packet::LevelChunkPacket;
use ezzmc_world::generated::b26_3::BLOCKS;
use ezzmc_world::level::Level;
use ezzmc_world::light::LightUpdate;

const FORMAT: ChunkFormat = ChunkFormat {
    min_y: -64,
    height: 384,
    block_global_bits: 15,
    biome_global_bits: 6,
};
const MAX_FILL: i64 = 4_000_000;
const DEFAULT_CHUNKS: i32 = 3;
/// Высота верхней грани пола по умолчанию (игрок стоит на y=64).
const DEFAULT_FLOOR_Y: i32 = 63;

pub const HELP: &str = "\
МИР
  chunks R                 пересоздать мир: чанки [-R,R]² (по умолч. 3 → x,z ∈ [-48,63]); пол stone на y=63
  empty                    пустой мир без пола (после chunks/в начале)
  set X Y Z блок[св=знач]  поставить блок: set 0 64 3 oak_stairs[facing=east,half=top]
  fill X1 Y1 Z1 X2 Y2 Z2 блок   залить параллелепипед (air — стереть)
  floor Y блок             плоскость по всему загруженному миру на высоте Y
  get X Y Z                блок, свойства, коллизия, вода/лава
  slice Y [X1 Z1 X2 Z2]    ASCII-карта слоя Y (# твёрдое, ~ вода, ^ лава, . воздух, @ игрок); по умолч. ±8 вокруг игрока
ИГРОК
  player X Y Z [yaw [pitch]]  новый игрок (yaw 0 → +Z, 90 → −X, 180 → −Z, 270 → +X)
  teleport X Y Z           переставить, сохранив состояние
  vel VX VY VZ             задать скорость
  yaw V · pitch V          повернуть
  flag имя 0|1|число       flying · mayfly · auto_jump · sprint_window · sprinting · crouching · using_item · fly_speed
  effect имя уровень       speed · slowness · jump_boost · levitation · slow_falling (0 — снять)
ВВОД И ВРЕМЯ
  keys w a s d space shift sprint   зажать клавиши (держатся между тиками; `keys` без аргументов — отпустить)
  tick N [клавиши…]        N тиков (клавиши — добавка только на этот вызов); печатает итоговое состояние
  until поле оп знач [max N]  тикать, пока условие не выполнится (по умолч. max 200); оп: == != < <= > >=
  trace on|off             печать состояния после каждого тика
ПРОВЕРКИ
  state                    состояние игрока
  expect поле [оп] знач [eps]  утверждение (eps по умолч. 0.001); провал → код выхода 1
  expect_block X Y Z имя   утверждение о блоке
  print текст…             эхо (метки в скрипте)
ПОЛЯ: x y z vx vy vz ground hcol minor_hcol sprinting crouching flying in_water in_lava under_water
      swimming crawling fall eye t (номер тика)  · булевы = 0/1
";

pub struct Sandbox {
    level: Level,
    chunks: i32,
    player: Player,
    yaw: f32,
    keys: Input,
    trace: bool,
    tick: u64,
    failed: u32,
    /// Печатать в stdout (в тестах выключено — результат в `log`).
    echo: bool,
    pub log: Vec<String>,
}

impl Sandbox {
    pub fn new(echo: bool) -> Self {
        let mut s = Self {
            level: Level::new(FORMAT, DEFAULT_CHUNKS),
            chunks: DEFAULT_CHUNKS,
            player: Player::new([0.5, 64.0, 0.5]),
            yaw: 0.0,
            keys: Input::default(),
            trace: false,
            tick: 0,
            failed: 0,
            echo,
            log: Vec::new(),
        };
        s.rebuild(DEFAULT_CHUNKS, true);
        s
    }

    pub fn failed(&self) -> u32 {
        self.failed
    }

    fn out(&mut self, line: String) {
        if self.echo {
            println!("{line}");
        }
        self.log.push(line);
    }

    /// Пересоздать мир: чанки `[-r, r]²` (пустые), опционально пол.
    fn rebuild(&mut self, r: i32, floor: bool) {
        self.chunks = r;
        self.level = Level::new(FORMAT, r);
        for cz in -r..=r {
            for cx in -r..=r {
                self.level.chunks.apply_packet(LevelChunkPacket {
                    x: cx,
                    z: cz,
                    chunk: Chunk::empty(&FORMAT),
                    heightmaps: Vec::new(),
                    block_entities: Vec::new(),
                    light: LightUpdate::default(),
                });
            }
        }
        if floor {
            if let Some(stone) = parse_state("stone") {
                self.fill_all_y(DEFAULT_FLOOR_Y, stone);
            }
        }
    }

    fn bounds(&self) -> (i32, i32) {
        (-16 * self.chunks, 16 * self.chunks + 15)
    }

    fn fill_all_y(&mut self, y: i32, st: BlockState) {
        let (lo, hi) = self.bounds();
        for z in lo..=hi {
            for x in lo..=hi {
                self.level.chunks.set_block(x, y, z, st);
            }
        }
    }

    pub fn run(&mut self, script: &str) {
        for (n, raw) in script.lines().enumerate() {
            for cmd in raw.split(';') {
                let cmd = cmd.split('#').next().unwrap_or("").trim();
                if cmd.is_empty() {
                    continue;
                }
                if let Err(e) = self.exec(cmd) {
                    self.failed += 1;
                    self.out(format!("ERR строка {}: `{cmd}` — {e}", n + 1));
                }
            }
        }
    }

    fn exec(&mut self, cmd: &str) -> Result<(), String> {
        let t: Vec<&str> = cmd.split_whitespace().collect();
        let a = &t[1..];
        match t[0] {
            "help" => self.out(HELP.to_string()),
            "chunks" => self.rebuild(int(a, 0)?.clamp(0, 16), true),
            "empty" => self.rebuild(self.chunks, false),
            "set" => {
                need(a, 4)?;
                let st = parse_state(a[3]).ok_or_else(|| format!("неизвестный блок `{}`", a[3]))?;
                self.put(int(a, 0)?, int(a, 1)?, int(a, 2)?, st)?;
            }
            "fill" => {
                need(a, 7)?;
                let st = parse_state(a[6]).ok_or_else(|| format!("неизвестный блок `{}`", a[6]))?;
                let (x1, y1, z1, x2, y2, z2) = (
                    int(a, 0)?,
                    int(a, 1)?,
                    int(a, 2)?,
                    int(a, 3)?,
                    int(a, 4)?,
                    int(a, 5)?,
                );
                let vol = i64::from((x2 - x1).abs() + 1)
                    * i64::from((y2 - y1).abs() + 1)
                    * i64::from((z2 - z1).abs() + 1);
                if vol > MAX_FILL {
                    return Err(format!("слишком большой объём {vol}"));
                }
                for y in y1.min(y2)..=y1.max(y2) {
                    for z in z1.min(z2)..=z1.max(z2) {
                        for x in x1.min(x2)..=x1.max(x2) {
                            self.put(x, y, z, st)?;
                        }
                    }
                }
            }
            "floor" => {
                need(a, 2)?;
                let st = parse_state(a[1]).ok_or_else(|| format!("неизвестный блок `{}`", a[1]))?;
                self.fill_all_y(int(a, 0)?, st);
            }
            "get" => self.get(int(a, 0)?, int(a, 1)?, int(a, 2)?)?,
            "slice" => self.slice(a)?,
            "player" => {
                let mut p = Player::new([num(a, 0)?, num(a, 1)?, num(a, 2)?]);
                if a.len() > 3 {
                    self.yaw = num(a, 3)? as f32;
                }
                if a.len() > 4 {
                    p.pitch = num(a, 4)? as f32;
                }
                self.player = p;
                self.tick = 0;
            }
            "teleport" => self.player.teleport([num(a, 0)?, num(a, 1)?, num(a, 2)?]),
            "vel" => self.player.vel = [num(a, 0)?, num(a, 1)?, num(a, 2)?],
            "yaw" => self.yaw = num(a, 0)? as f32,
            "pitch" => self.player.pitch = num(a, 0)? as f32,
            "flag" => {
                need(a, 2)?;
                self.flag(a[0], num(a, 1)?)?;
            }
            "effect" => {
                need(a, 2)?;
                let v = int(a, 1)?;
                let fx = &mut self.player.fx;
                match a[0] {
                    "speed" => fx.speed = v,
                    "slowness" => fx.slowness = v,
                    "jump_boost" => fx.jump_boost = v,
                    "levitation" => fx.levitation = v,
                    "slow_falling" => fx.slow_falling = v != 0,
                    o => return Err(format!("неизвестный эффект `{o}`")),
                }
            }
            "keys" => {
                self.keys = Input::default();
                for k in a {
                    set_key(&mut self.keys, k)?;
                }
            }
            "tick" => {
                let n = int(a, 0)?;
                let mut inp = self.keys;
                for k in &a[1..] {
                    set_key(&mut inp, k)?;
                }
                for _ in 0..n.max(0) {
                    self.step(inp);
                }
                if !self.trace {
                    let s = self.state_line();
                    self.out(s);
                }
            }
            "until" => self.until(a)?,
            "trace" => self.trace = a.first().is_some_and(|v| matches!(*v, "on" | "1" | "true")),
            "state" => {
                let s = self.state_line();
                self.out(s);
            }
            "expect" => self.expect(a)?,
            "expect_block" => {
                need(a, 4)?;
                let got = self
                    .level
                    .chunks
                    .block(int(a, 0)?, int(a, 1)?, int(a, 2)?)
                    .name();
                let want = a[3].trim_start_matches("minecraft:");
                if got != want {
                    self.failed += 1;
                    self.out(format!("FAIL expect_block: ожидался {want}, найден {got}"));
                } else {
                    self.out(format!("ok   block {got}"));
                }
            }
            "print" => self.out(a.join(" ")),
            o => return Err(format!("неизвестная команда `{o}` (help)")),
        }
        Ok(())
    }

    fn put(&mut self, x: i32, y: i32, z: i32, st: BlockState) -> Result<(), String> {
        if !self.level.chunks.set_block(x, y, z, st) {
            let (lo, hi) = self.bounds();
            return Err(format!(
                "({x},{y},{z}) вне мира: x,z ∈ [{lo},{hi}], y ∈ [-64,319] (см. chunks R)"
            ));
        }
        Ok(())
    }

    fn step(&mut self, inp: Input) {
        self.player.tick(&self.level, inp, self.yaw);
        self.player.post_tick(self.yaw, self.player.pitch);
        self.level.game_time += 1;
        self.tick += 1;
        if self.trace {
            let s = self.state_line();
            self.out(s);
        }
    }

    fn field(&self, name: &str) -> Result<f64, String> {
        let p = &self.player;
        let b = |v: bool| f64::from(u8::from(v));
        Ok(match name {
            "x" => p.pos[0],
            "y" => p.pos[1],
            "z" => p.pos[2],
            "vx" => p.vel[0],
            "vy" => p.vel[1],
            "vz" => p.vel[2],
            "ground" => b(p.on_ground),
            "hcol" => b(p.horizontal_collision),
            "minor_hcol" => b(p.minor_horizontal_collision),
            "sprinting" => b(p.sprinting),
            "crouching" => b(p.crouching),
            "flying" => b(p.flying),
            "in_water" => b(p.in_water),
            "in_lava" => b(p.in_lava),
            "under_water" => b(p.under_water),
            "swimming" => b(p.swimming),
            "crawling" => b(p.crawling),
            "fall" => p.fall_distance,
            "eye" => p.eye_height(),
            "t" => self.tick as f64,
            o => return Err(format!("неизвестное поле `{o}`")),
        })
    }

    fn state_line(&self) -> String {
        let p = &self.player;
        let mut flags = String::new();
        for (n, on) in [
            ("ground", p.on_ground),
            ("hcol", p.horizontal_collision),
            ("sprint", p.sprinting),
            ("sneak", p.crouching),
            ("fly", p.flying),
            ("water", p.in_water),
            ("lava", p.in_lava),
            ("swim", p.swimming),
            ("crawl", p.crawling),
        ] {
            if on {
                flags.push(' ');
                flags.push_str(n);
            }
        }
        format!(
            "t={} pos=({:.4},{:.4},{:.4}) vel=({:.4},{:.4},{:.4}) fall={:.2}{flags}",
            self.tick, p.pos[0], p.pos[1], p.pos[2], p.vel[0], p.vel[1], p.vel[2], p.fall_distance
        )
    }

    fn flag(&mut self, name: &str, v: f64) -> Result<(), String> {
        let on = v != 0.0;
        let p = &mut self.player;
        match name {
            "flying" => p.flying = on,
            "mayfly" => p.mayfly = on,
            "auto_jump" => p.auto_jump = on,
            "sprinting" => p.sprinting = on,
            "crouching" => p.crouching = on,
            "using_item" => p.using_item = on,
            "sprint_window" => p.sprint_window = v as u8,
            "fly_speed" => p.fly_speed = v as f32,
            o => return Err(format!("неизвестный флаг `{o}`")),
        }
        Ok(())
    }

    fn until(&mut self, a: &[&str]) -> Result<(), String> {
        need(a, 3)?;
        let (f, op, v) = (a[0], a[1], num(a, 2)?);
        let max = if a.len() >= 5 && a[3] == "max" {
            int(a, 4)?
        } else {
            200
        };
        for _ in 0..max {
            if cmp(self.field(f)?, op, v)? {
                let s = self.state_line();
                self.out(format!("until ok · {s}"));
                return Ok(());
            }
            let inp = self.keys;
            self.step(inp);
        }
        if cmp(self.field(f)?, op, v)? {
            let s = self.state_line();
            self.out(format!("until ok · {s}"));
            return Ok(());
        }
        self.failed += 1;
        let s = self.state_line();
        self.out(format!(
            "FAIL until {f} {op} {v}: за {max} тиков не наступило · {s}"
        ));
        Ok(())
    }

    fn expect(&mut self, a: &[&str]) -> Result<(), String> {
        need(a, 2)?;
        let ops = ["==", "!=", "<=", ">=", "<", ">"];
        let (op, rest) = if ops.contains(&a[1]) {
            (a[1], &a[2..])
        } else {
            ("~", &a[1..])
        };
        let want: f64 = rest
            .first()
            .ok_or("нет значения")?
            .parse()
            .map_err(|_| "не число")?;
        let eps: f64 = rest
            .get(1)
            .map_or(Ok(1.0e-3), |e| e.parse())
            .map_err(|_| "eps не число")?;
        let got = self.field(a[0])?;
        let ok = if op == "~" {
            (got - want).abs() <= eps
        } else {
            cmp(got, op, want)?
        };
        if ok {
            self.out(format!("ok   {} = {got:.5}", a[0]));
        } else {
            self.failed += 1;
            self.out(format!(
                "FAIL expect {} {op} {want} (eps {eps}): получено {got:.6}",
                a[0]
            ));
        }
        Ok(())
    }

    fn get(&mut self, x: i32, y: i32, z: i32) -> Result<(), String> {
        let st = self.level.chunks.block(x, y, z);
        let mut props = String::new();
        for p in st.block().props {
            if let Some(v) = st.property(p.name) {
                props.push_str(&format!(
                    "{}{}={v}",
                    if props.is_empty() { "" } else { "," },
                    p.name
                ));
            }
        }
        let boxes = self.level.boxes(x, y, z);
        let l = &self.level;
        let line = format!(
            "({x},{y},{z}) {}[{props}] id={} коллизия={boxes:?} вода={} лава={}",
            st.name(),
            st.id(),
            l.water_height(x, y, z),
            l.lava_height(x, y, z)
        );
        self.out(line);
        Ok(())
    }

    fn slice(&mut self, a: &[&str]) -> Result<(), String> {
        let y = int(a, 0)?;
        let (px, pz) = (
            self.player.pos[0].floor() as i32,
            self.player.pos[2].floor() as i32,
        );
        let (x1, z1, x2, z2) = if a.len() >= 5 {
            (int(a, 1)?, int(a, 2)?, int(a, 3)?, int(a, 4)?)
        } else {
            (px - 8, pz - 8, px + 8, pz + 8)
        };
        let mut s = format!("слой y={y}, x {x1}..{x2} →, z {z1}..{z2} ↓\n");
        for z in z1..=z2 {
            for x in x1..=x2 {
                let c = if x == px && z == pz {
                    '@'
                } else if !self.level.boxes(x, y, z).is_empty() {
                    '#'
                } else if self.level.water_height(x, y, z) > 0.0 {
                    '~'
                } else if self.level.lava_height(x, y, z) > 0.0 {
                    '^'
                } else {
                    '.'
                };
                s.push(c);
            }
            s.push('\n');
        }
        self.out(s.trim_end().to_string());
        Ok(())
    }
}

fn need(a: &[&str], n: usize) -> Result<(), String> {
    if a.len() < n {
        Err(format!("нужно аргументов: {n}, дано {}", a.len()))
    } else {
        Ok(())
    }
}

fn int(a: &[&str], i: usize) -> Result<i32, String> {
    a.get(i)
        .ok_or_else(|| format!("нет аргумента #{}", i + 1))?
        .parse()
        .map_err(|_| format!("`{}` — не целое число", a[i]))
}

fn num(a: &[&str], i: usize) -> Result<f64, String> {
    a.get(i)
        .ok_or_else(|| format!("нет аргумента #{}", i + 1))?
        .parse()
        .map_err(|_| format!("`{}` — не число", a[i]))
}

fn cmp(a: f64, op: &str, b: f64) -> Result<bool, String> {
    Ok(match op {
        "==" => (a - b).abs() < 1.0e-9,
        "!=" => (a - b).abs() >= 1.0e-9,
        "<" => a < b,
        "<=" => a <= b,
        ">" => a > b,
        ">=" => a >= b,
        o => return Err(format!("неизвестный оператор `{o}`")),
    })
}

fn set_key(i: &mut Input, k: &str) -> Result<(), String> {
    match k {
        "w" | "forward" => i.forward = true,
        "s" | "back" => i.back = true,
        "a" | "left" => i.left = true,
        "d" | "right" => i.right = true,
        "space" | "jump" => i.jump = true,
        "shift" | "sneak" => i.sneak = true,
        "sprint" | "ctrl" => i.sprint = true,
        o => return Err(format!("неизвестная клавиша `{o}`")),
    }
    Ok(())
}

/// `oak_stairs[facing=east,half=top]` / `minecraft:stone` → состояние (свойства не заданы → по умолчанию).
fn parse_state(spec: &str) -> Option<BlockState> {
    let spec = spec.trim_start_matches("minecraft:");
    let (name, props) = match spec.split_once('[') {
        Some((n, p)) => (n, p.trim_end_matches(']')),
        None => (spec, ""),
    };
    let info = BLOCKS.iter().find(|b| b.name == name)?;
    let mut st = BlockState::from_id(info.default_state)?;
    for kv in props.split(',').filter(|s| !s.is_empty()) {
        let (k, v) = kv.split_once('=')?;
        st = st.with(k.trim(), v.trim())?;
    }
    Some(st)
}

/// Точка входа `--sandbox`: скрипт из `-e "…"`, файла или stdin. Возвращает код выхода.
pub fn main(args: &[String]) -> i32 {
    let script = match args.first().map(String::as_str) {
        Some("-e") => args.get(1..).unwrap_or(&[]).join("\n"),
        Some(p) if !p.starts_with("--") => match std::fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("sandbox: не прочитать {p}: {e}");
                return 2;
            }
        },
        _ => {
            let mut s = String::new();
            if std::io::Read::read_to_string(&mut std::io::stdin(), &mut s).is_err() {
                return 2;
            }
            if s.trim().is_empty() {
                println!("{HELP}");
                return 0;
            }
            s
        }
    };
    let mut sb = Sandbox::new(true);
    sb.run(&script);
    i32::from(sb.failed() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Sandbox {
        let mut sb = Sandbox::new(false);
        sb.run(s);
        sb
    }

    #[test]
    fn walks_and_stops_at_wall() {
        // yaw 0 → +Z; стена на z=4 → упор на z = 4 - 0.3.
        let sb =
            run("fill -2 64 4 2 66 4 stone; keys w; tick 40; expect z 3.7 0.01; expect hcol 1");
        assert_eq!(sb.failed(), 0, "{:?}", sb.log);
    }

    #[test]
    fn jump_lands_on_floor() {
        let sb = run("tick 2; tick 1 space; until ground == 1; expect y 64 0.001");
        assert_eq!(sb.failed(), 0, "{:?}", sb.log);
    }

    #[test]
    fn bad_commands_fail() {
        assert_eq!(run("set 0 64 0 nonexistent").failed(), 1);
        assert_eq!(run("set 9999 64 0 stone").failed(), 1);
        assert_eq!(run("expect y 100").failed(), 1);
    }
}
