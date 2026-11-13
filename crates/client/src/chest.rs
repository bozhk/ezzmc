//! Сундуки: блок-сущность-рендер (одиночный, двойной, эндер, ловушка, медные).
//! VANILLA: ChestRenderer#submit/extractRenderState, ChestModel (26.3), ChestLidController,
//! ChestBlockEntity#triggerEvent (событие 1: число зрителей), DoubleBlockCombiner/BrightnessCombiner.
//! Блок сундука в мешере без граней (модель `block/chest` только с частицей), всё рисуем здесь.

use crate::mobs::SkinKey;
use ezzmc_render::entitymodel::{BoxDef, Model, Pose};
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use glam::{Mat4, Vec3};
use rayon::prelude::*;
use std::collections::HashMap;

// Маски граней (порядок Cube: DOWN, UP, WEST, NORTH, EAST, SOUTH).
const ALL_BUT_EAST: u8 = 0x3f & !(1 << 4);
const ALL_BUT_WEST: u8 = 0x3f & !(1 << 2);

/// VANILLA: ChestType.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ty {
    Single,
    Left,
    Right,
}

/// Материал сундука (ChestRenderState.ChestMaterialType) → префикс текстуры.
fn material(name: &str, xmas: bool) -> &'static str {
    let n = name.strip_prefix("waxed_").unwrap_or(name);
    match n {
        "copper_chest" => "copper",
        "exposed_copper_chest" => "copper_exposed",
        "weathered_copper_chest" => "copper_weathered",
        "oxidized_copper_chest" => "copper_oxidized",
        "ender_chest" => "ender",
        _ if xmas => "christmas",
        "trapped_chest" => "trapped",
        _ => "normal",
    }
}

/// VANILLA: Sheets#chooseSprite — путь текстуры под `textures/entity/chest/`.
fn texture(mat: &'static str, ty: Ty) -> &'static str {
    macro_rules! t {
        ($($m:literal),*) => {
            match (mat, ty) {
                $(
                    ($m, Ty::Single) => concat!("chest/", $m),
                    ($m, Ty::Left) => concat!("chest/", $m, "_left"),
                    ($m, Ty::Right) => concat!("chest/", $m, "_right"),
                )*
                _ => "chest/normal",
            }
        };
    }
    if mat == "ender" {
        return "chest/ender";
    }
    t!(
        "normal",
        "trapped",
        "christmas",
        "copper",
        "copper_exposed",
        "copper_weathered",
        "copper_oxidized"
    )
}

fn is_chest(name: &str) -> bool {
    // chest, trapped_chest, ender_chest, [waxed_][exposed_|weathered_|oxidized_]copper_chest
    name.ends_with("chest")
}

/// Радиус окна скана сундуков в колонках (64 блока / 16 + запас на вторую половину двойного сундука).
const SCAN_RADIUS: i32 = 5;

/// Таблица «id состояния → сундук» (строится один раз): скан колонок без сравнения строк.
fn is_chest_id(id: u32) -> bool {
    static LUT: std::sync::OnceLock<Vec<bool>> = std::sync::OnceLock::new();
    let lut = LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).is_some_and(|s| is_chest(s.name())))
            .collect()
    });
    lut.get(id as usize).copied().unwrap_or(false)
}

/// Одиночная модель (для иконок GUI).
pub fn model_single() -> Model {
    model(Ty::Single)
}

/// VANILLA: ChestModel.createSingleBodyLayer / createDoubleBodyRightLayer / createDoubleBodyLeftLayer.
fn model(ty: Ty) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let (bx, bw, lx, lw, kx, kw, faces) = match ty {
        Ty::Single => (1.0, 14.0, 1.0, 14.0, 7.0, 2.0, 0x3f),
        Ty::Right => (1.0, 15.0, 1.0, 15.0, 15.0, 1.0, ALL_BUT_EAST),
        Ty::Left => (0.0, 15.0, 0.0, 15.0, 0.0, 1.0, ALL_BUT_WEST),
    };
    m.add(
        "bottom",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 19.0], [bx, 0.0, 1.0], [bw, 10.0, 14.0]).faces(faces)],
    );
    let hinge = Pose::offset(0.0, 9.0, 1.0);
    m.add(
        "lid",
        None,
        hinge,
        &[BoxDef::new([0.0, 0.0], [lx, 0.0, 0.0], [lw, 5.0, 14.0]).faces(faces)],
    );
    m.add(
        "lock",
        None,
        hinge,
        &[BoxDef::new([0.0, 0.0], [kx, -2.0, 14.0], [kw, 4.0, 1.0]).faces(faces)],
    );
    m
}

/// VANILLA: ChestLidController.
#[derive(Default, Clone, Copy)]
struct Lid {
    should_open: bool,
    open: f32,
    o_open: f32,
}

impl Lid {
    fn tick(&mut self) {
        self.o_open = self.open;
        if !self.should_open && self.open > 0.0 {
            self.open = (self.open - 0.1).max(0.0);
        } else if self.should_open && self.open < 1.0 {
            self.open = (self.open + 0.1).min(1.0);
        }
    }
    fn get(&self, a: f32) -> f32 {
        self.o_open + (self.open - self.o_open) * a
    }
}

/// Найденный в мире сундук.
#[derive(Clone, Copy)]
struct Inst {
    pos: [i32; 3],
    /// Направление «вперёд» (VANILLA: Direction#toYRot): юг 0, запад 90, север 180, восток 270.
    yrot: f32,
    ty: Ty,
    tex: &'static str,
    /// Позиция второй половины двойного сундука.
    other: Option<[i32; 3]>,
}

pub struct Chests {
    models: [Model; 3],
    list: Vec<Inst>,
    /// Кэш сундуков по колонкам (x, z) -> (поколение скана, сундуки): новые чанки сканируются по одному.
    cols: HashMap<(i32, i32), (u32, Vec<Inst>)>,
    scan_gen: u32,
    scan_cam_chunk: Option<(i32, i32)>,
    lids: HashMap<[i32; 3], Lid>,
    last_tick: Option<i64>,
    /// Версия мира (`RebuildQueue::version`) и тик последнего скана: скан только при изменениях.
    scan_version: Option<u64>,
    scan_block_version: Option<u64>,
    scan_tick: i64,
    xmas: bool,
}

impl Default for Chests {
    fn default() -> Self {
        use chrono::Datelike;
        let now = chrono::Local::now();
        Chests {
            models: [model(Ty::Single), model(Ty::Left), model(Ty::Right)],
            list: Vec::new(),
            cols: HashMap::new(),
            scan_gen: 0,
            scan_cam_chunk: None,
            lids: HashMap::new(),
            last_tick: None,
            scan_version: None,
            scan_block_version: None,
            scan_tick: 0,
            // VANILLA: SpecialDates#isExtendedChristmas — 24…26 декабря.
            xmas: now.month() == 12 && (24..=26).contains(&now.day()),
        }
    }
}

/// Направление по `facing`: (dx, dz, yrot).
fn dir(facing: &str) -> Option<(i32, i32, f32)> {
    Some(match facing {
        "south" => (0, 1, 0.0),
        "west" => (-1, 0, 90.0),
        "north" => (0, -1, 180.0),
        "east" => (1, 0, 270.0),
        _ => return None,
    })
}

/// VANILLA: Direction#getClockWise.
fn clockwise(f: &str) -> &'static str {
    match f {
        "north" => "east",
        "east" => "south",
        "south" => "west",
        _ => "north",
    }
}

fn counter_clockwise(f: &str) -> &'static str {
    match f {
        "north" => "west",
        "west" => "south",
        "south" => "east",
        _ => "north",
    }
}

impl Chests {
    /// VANILLA: ChestBlockEntity#triggerEvent (b0 == 1 → открыт, если зрителей > 0).
    pub fn block_event(&mut self, pos: [i32; 3], b0: i32, b1: i32) {
        if b0 == 1 {
            let lid = self.lids.entry(pos).or_default();
            lid.should_open = b1 > 0;
        }
    }

    /// Пересканировать загруженные колонки; тики крышек; запечь геометрию в `out(tex)`.
    pub fn bake(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        (world_version, block_version): (u64, u64),
        mobs: &mut crate::mobs::Mobs,
        frustum: Option<&ezzmc_mesh::frustum::Frustum>,
    ) {
        let now = level.game_time;
        let ticks = match self.last_tick.replace(now) {
            Some(prev) if now >= prev => (now - prev).min(5),
            Some(_) => 0,
            None => 1,
        };
        // Полный обход колонок дорог (десятки тысяч секций при rd=32): только при изменении мира,
        // плюс страховочный пересчёт раз в 100 тиков.
        // Правка блока — сразу; загрузка чанков при беге меняет версию почти каждый тик, поэтому
        // для неё пересчёт не чаще раза в 10 тиков (VANILLA-UNSURE: сундук в новом чанке до 0.5 с позже).
        let cam_chunk = (
            (cam[0] / 16.0).floor() as i32,
            (cam[2] / 16.0).floor() as i32,
        );
        let moved = self.scan_cam_chunk != Some(cam_chunk);
        let block_changed = self.scan_block_version != Some(block_version);
        let world_changed = self.scan_version != Some(world_version) && now - self.scan_tick >= 10;
        let world_changed = world_changed || moved;
        let safety = now - self.scan_tick >= 100 || now < self.scan_tick;
        if block_changed || world_changed || safety {
            self.scan_version = Some(world_version);
            self.scan_block_version = Some(block_version);
            self.scan_tick = now;
            self.scan_cam_chunk = Some(cam_chunk);
            // Правка блока и страховка — полный пересчёт; загрузка чанков — только новые колонки.
            self.rescan(level, cam, block_changed || safety);
        }
        for _ in 0..ticks {
            for l in self.lids.values_mut() {
                l.tick();
            }
        }
        // Закрытые и неактуальные крышки не храним.
        self.lids.retain(|_, l| l.should_open || l.open > 0.0);
        let pt = level.partial_tick();
        // VANILLA: BlockEntityRenderDispatcher — getRenderBoundingBox против Frustum; getViewDistance = 64.
        let vis: Vec<&Inst> = self
            .list
            .iter()
            .filter(|c| {
                let d = |i: usize| f64::from(c.pos[i]) + 0.5 - cam[i];
                if d(0) * d(0) + d(1) * d(1) + d(2) * d(2) > 64.0 * 64.0 {
                    return false;
                }
                frustum.is_none_or(|fr| {
                    let lo = [
                        f64::from(c.pos[0]),
                        f64::from(c.pos[1]),
                        f64::from(c.pos[2]),
                    ];
                    fr.is_visible(lo, [lo[0] + 1.0, lo[1] + 1.0, lo[2] + 1.0])
                })
            })
            .collect();
        let lids = &self.lids;
        let models = &self.models;
        // GPU-скиннинг: группы (тип модели, текстура); экземпляр = матрицы частей + свет.
        let mut groups: HashMap<(u8, &'static str), Vec<&Inst>> = HashMap::new();
        for c in vis {
            let idx = match c.ty {
                Ty::Single => 0,
                Ty::Left => 1,
                Ty::Right => 2,
            };
            groups.entry((idx, c.tex)).or_default().push(c);
        }
        const CHUNK: usize = 256;
        for ((idx, tex), list) in groups {
            let base = &models[usize::from(idx)];
            let (stride, out) = mobs.skin_out(SkinKey::Chest(idx), tex, || base.skin_mesh());
            let start = out.len();
            out.resize(start + list.len() * stride, 0);
            out[start..]
                .par_chunks_mut(CHUNK * stride)
                .zip(list.par_chunks(CHUNK))
                .for_each_init(
                    || (base.clone(), Vec::<Mat4>::with_capacity(base.parts.len())),
                    |(m, mats), (dst, cs)| {
                        for (slot, c) in dst.chunks_exact_mut(stride).zip(cs) {
                            let open_at = |p: [i32; 3]| lids.get(&p).map_or(0.0, |l| l.get(pt));
                            let mut open = open_at(c.pos);
                            let mut light = block_light(level, c.pos);
                            if let Some(o) = c.other {
                                // VANILLA: opennessCombiner = max; BrightnessCombiner = max по компонентам.
                                open = open.max(open_at(o));
                                let l2 = block_light(level, o);
                                light = (light.0.max(l2.0), light.1.max(l2.1));
                            }
                            // VANILLA: ChestRenderer#submit — open = 1 − (1 − open)³ … 1 − open·open·open.
                            let t = 1.0 - open;
                            let open = 1.0 - t * t * t;
                            m.reset_pose();
                            // VANILLA: ChestModel#setupAnim — lid.xRot = −open·π/2; замок вместе с крышкой.
                            let rot = -(open * std::f32::consts::FRAC_PI_2);
                            for name in ["lid", "lock"] {
                                if let Some(i) = m.index(name) {
                                    m.parts[i].pose.x_rot = rot;
                                }
                            }
                            // VANILLA: ChestRenderer#createModelTransformation — вращение вокруг (0.5, 0, 0.5).
                            let root = Mat4::from_translation(Vec3::new(
                                (f64::from(c.pos[0]) - cam[0]) as f32,
                                (f64::from(c.pos[1]) - cam[1]) as f32,
                                (f64::from(c.pos[2]) - cam[2]) as f32,
                            )) * Mat4::from_translation(Vec3::new(0.5, 0.0, 0.5))
                                * Mat4::from_rotation_y((-c.yrot).to_radians())
                                * Mat4::from_translation(Vec3::new(-0.5, 0.0, -0.5));
                            m.part_matrices_into(root, mats);
                            let mut o = 0;
                            for x in mats.iter() {
                                for f in x.to_cols_array() {
                                    slot[o..o + 4].copy_from_slice(&f.to_le_bytes());
                                    o += 4;
                                }
                            }
                            slot[o..o + 4].copy_from_slice(&f32::from(light.0).to_le_bytes());
                            slot[o + 4..o + 8].copy_from_slice(&f32::from(light.1).to_le_bytes());
                        }
                    },
                );
        }
    }

    /// VANILLA: ClientLevel — сундуки в клиентских блок-сущностях; здесь находим их по состояниям
    /// блоков в загруженных колонках (секции без сундуков отсекаются по палитре).
    fn rescan(&mut self, level: &Level, cam: [f64; 3], full: bool) {
        if full {
            self.cols.clear();
        }
        self.scan_gen = self.scan_gen.wrapping_add(1);
        let gen = self.scan_gen;
        // VANILLA: BlockEntityRenderer#getViewDistance = 64 → сундуки рисуются не дальше 4 колонок;
        // сканируем только окно вокруг камеры (а не весь радиус загрузки: при rd=32 это ~4500 колонок).
        let (cx, cz) = (
            (cam[0] / 16.0).floor() as i32,
            (cam[2] / 16.0).floor() as i32,
        );
        let r = SCAN_RADIUS;
        for (x, z) in (-r..=r).flat_map(|dx| (-r..=r).map(move |dz| (dx, dz))) {
            let Some(ch) = level.chunks.get(cx + x, cz + z) else {
                continue;
            };
            let key = (ch.pos.x, ch.pos.z);
            if let Some(e) = self.cols.get_mut(&key) {
                e.0 = gen;
                continue;
            }
            let found = self.scan_chunk(ch);
            self.cols.insert(key, (gen, found));
        }
        // Выгруженные колонки (и вышедшие из радиуса) забываем: при повторной загрузке данные могут отличаться.
        self.cols.retain(|_, e| e.0 == gen);
        self.list.clear();
        for e in self.cols.values() {
            self.list.extend_from_slice(&e.1);
        }
    }

    fn scan_chunk(&self, ch: &ezzmc_world::storage::ClientChunk) -> Vec<Inst> {
        let mut out = Vec::new();
        let chunk = &ch.chunk;
        for (si, sec) in chunk.sections().iter().enumerate() {
            if sec.has_only_air() || !sec.states().maybe_has(is_chest_id) {
                continue;
            }
            let y0 = chunk.min_y() + si as i32 * 16;
            for y in 0..16usize {
                for z in 0..16usize {
                    for x in 0..16usize {
                        let s = sec.block(x, y, z);
                        if !is_chest_id(s.id()) {
                            continue;
                        }
                        let pos = [
                            ch.pos.x * 16 + x as i32,
                            y0 + y as i32,
                            ch.pos.z * 16 + z as i32,
                        ];
                        if let Some(i) = self.inst(s, pos) {
                            out.push(i);
                        }
                    }
                }
            }
        }
        out
    }

    fn inst(&self, s: BlockState, pos: [i32; 3]) -> Option<Inst> {
        let facing = s.property("facing")?;
        let (_, _, yrot) = dir(facing)?;
        let ty = match s.property("type") {
            Some("left") => Ty::Left,
            Some("right") => Ty::Right,
            _ => Ty::Single,
        };
        // VANILLA: ChestBlock#getConnectedDirection — LEFT: по часовой, RIGHT: против.
        let other = match ty {
            Ty::Single => None,
            Ty::Left => dir(clockwise(facing)),
            Ty::Right => dir(counter_clockwise(facing)),
        }
        .map(|(dx, dz, _)| [pos[0] + dx, pos[1], pos[2] + dz]);
        let mat = material(s.name(), self.xmas);
        Some(Inst {
            pos,
            yrot,
            ty,
            tex: texture(mat, ty),
            other,
        })
    }
}

fn block_light(level: &Level, p: [i32; 3]) -> (u8, u8) {
    crate::particles::light_at(level, f64::from(p[0]), f64::from(p[1]), f64::from(p[2]))
}
