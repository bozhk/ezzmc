#![allow(clippy::approx_constant)] // VANILLA: литералы углов как в исходнике Mojang
//! Модели не-гуманоидных мобов: четвероногие (свинья, корова), крипер, паук, курица.
//! VANILLA: QuadrupedModel, PigModel, CowModel, CreeperModel, SpiderModel, AdultChickenModel (26.3).

use crate::entitymodel::{BoxDef, Model, Pose};
use crate::humanoid::HumanoidParts;
use std::f32::consts::PI;

const HALF_PI: f32 = PI / 2.0;

/// VANILLA: QuadrupedModel#createBodyMesh + createLegs. Голову подменяет вызывающий.
fn quadruped(
    m: &mut Model,
    leg: f32,
    mirror_left: bool,
    mirror_right: bool,
    body_tex: [f32; 2],
    body: [f32; 6],
) {
    let b = BoxDef::new(
        body_tex,
        [body[0], body[1], body[2]],
        [body[3], body[4], body[5]],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 17.0 - leg, 2.0)
        },
        &[b],
    );
    legs(m, leg, mirror_left, mirror_right, [3.0, 7.0, -5.0]);
}

/// Четыре ноги: `x` — смещение от центра, `zh/zf` — задняя/передняя.
fn legs(m: &mut Model, leg: f32, mirror_left: bool, mirror_right: bool, p: [f32; 3]) {
    let [x, zh, zf] = p;
    let mk = |mirror: bool| {
        let b = BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, leg, 4.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    let y = 24.0 - leg;
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-x, y, zh),
        &[mk(mirror_right)],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(x, y, zh),
        &[mk(mirror_left)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-x, y, zf),
        &[mk(mirror_right)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(x, y, zf),
        &[mk(mirror_left)],
    );
}

/// VANILLA: PigModel#createBasePigModel (64×64, без деформации).
pub fn pig_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 12.0, -6.0),
        &[
            BoxDef::new([0.0, 0.0], [-4.0, -4.0, -8.0], [8.0, 8.0, 8.0]),
            BoxDef::new([16.0, 16.0], [-2.0, 0.0, -9.0], [4.0, 3.0, 1.0]),
        ],
    );
    quadruped(
        &mut m,
        6.0,
        true,
        false,
        [28.0, 8.0],
        [-5.0, -10.0, -7.0, 10.0, 16.0, 8.0],
    );
    m
}

/// VANILLA: CowModel#createBaseCowModel (64×64).
/// `variant`: 0 temperate (CowModel), 1 warm (WarmCowModel), 2 cold (ColdCowModel).
pub fn cow_mesh(variant: u8) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head_pose = Pose::offset(0.0, 4.0, -8.0);
    let body_pose = Pose {
        x_rot: HALF_PI,
        ..Pose::offset(0.0, 5.0, 2.0)
    };
    match variant {
        // VANILLA: WarmCowModel#createBodyLayer — длинные горизонтальные рога.
        1 => {
            m.add(
                "head",
                None,
                head_pose,
                &[
                    BoxDef::new([0.0, 0.0], [-4.0, -4.0, -6.0], [8.0, 8.0, 6.0]),
                    BoxDef::new([1.0, 33.0], [-3.0, 1.0, -7.0], [6.0, 3.0, 1.0]),
                    BoxDef::new([27.0, 0.0], [-8.0, -3.0, -5.0], [4.0, 2.0, 2.0]),
                    BoxDef::new([39.0, 0.0], [-8.0, -5.0, -5.0], [2.0, 2.0, 2.0]),
                    BoxDef::new([27.0, 0.0], [4.0, -3.0, -5.0], [4.0, 2.0, 2.0]).mirror(),
                    BoxDef::new([39.0, 0.0], [6.0, -5.0, -5.0], [2.0, 2.0, 2.0]).mirror(),
                ],
            );
            m.add(
                "body",
                None,
                body_pose,
                &[
                    BoxDef::new([18.0, 4.0], [-6.0, -10.0, -7.0], [12.0, 18.0, 10.0]),
                    BoxDef::new([52.0, 0.0], [-2.0, 2.0, -8.0], [4.0, 6.0, 1.0]),
                ],
            );
        }
        // VANILLA: ColdCowModel#createBodyLayer — шерсть (inflate 0.5) и рога-дочерние части головы.
        2 => {
            let head = m.add(
                "head",
                None,
                head_pose,
                &[
                    BoxDef::new([0.0, 0.0], [-4.0, -4.0, -6.0], [8.0, 8.0, 6.0]),
                    BoxDef::new([9.0, 33.0], [-3.0, 1.0, -7.0], [6.0, 3.0, 1.0]),
                ],
            );
            m.add(
                "right_horn",
                Some(head),
                Pose {
                    x_rot: 1.5708,
                    ..Pose::offset(-4.5, -2.5, -3.5)
                },
                &[BoxDef::new(
                    [0.0, 40.0],
                    [-1.5, -4.5, -0.5],
                    [2.0, 6.0, 2.0],
                )],
            );
            m.add(
                "left_horn",
                Some(head),
                Pose {
                    x_rot: 1.5708,
                    ..Pose::offset(5.5, -2.5, -5.0)
                },
                &[BoxDef::new(
                    [0.0, 32.0],
                    [-1.5, -3.0, -0.5],
                    [2.0, 6.0, 2.0],
                )],
            );
            m.add(
                "body",
                None,
                body_pose,
                &[
                    BoxDef::new([20.0, 32.0], [-6.0, -10.0, -7.0], [12.0, 18.0, 10.0]).grow(0.5),
                    BoxDef::new([18.0, 4.0], [-6.0, -10.0, -7.0], [12.0, 18.0, 10.0]),
                    BoxDef::new([52.0, 0.0], [-2.0, 2.0, -8.0], [4.0, 6.0, 1.0]),
                ],
            );
        }
        _ => {
            m.add(
                "head",
                None,
                head_pose,
                &[
                    BoxDef::new([0.0, 0.0], [-4.0, -4.0, -6.0], [8.0, 8.0, 6.0]),
                    BoxDef::new([1.0, 33.0], [-3.0, 1.0, -7.0], [6.0, 3.0, 1.0]),
                    BoxDef::new([22.0, 0.0], [-5.0, -5.0, -5.0], [1.0, 3.0, 1.0]),
                    BoxDef::new([22.0, 0.0], [4.0, -5.0, -5.0], [1.0, 3.0, 1.0]),
                ],
            );
            m.add(
                "body",
                None,
                body_pose,
                &[
                    BoxDef::new([18.0, 4.0], [-6.0, -10.0, -7.0], [12.0, 18.0, 10.0]),
                    BoxDef::new([52.0, 0.0], [-2.0, 2.0, -8.0], [4.0, 6.0, 1.0]),
                ],
            );
        }
    }
    let leg = |mirror: bool| {
        let b = BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-4.0, 12.0, 7.0),
        &[leg(false)],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(4.0, 12.0, 7.0),
        &[leg(true)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-4.0, 12.0, -5.0),
        &[leg(false)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(4.0, 12.0, -5.0),
        &[leg(true)],
    );
    m
}

/// VANILLA: QuadrupedModel#setupAnim (и тот же код у CreeperModel / курицы для ног).
pub fn quadruped_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let a = (walk_pos * 0.6662).cos() * 1.4 * walk_speed;
    let b = (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed;
    set_x(m, "right_hind_leg", a);
    set_x(m, "left_hind_leg", b);
    set_x(m, "right_front_leg", b);
    set_x(m, "left_front_leg", a);
}

fn set_head(m: &mut Model, y_rot: f32, x_rot: f32) {
    if let Some(i) = m.index("head") {
        m.parts[i].pose.y_rot = y_rot.to_radians();
        m.parts[i].pose.x_rot = x_rot.to_radians();
    }
}

fn set_x(m: &mut Model, name: &str, v: f32) {
    if let Some(i) = m.index(name) {
        m.parts[i].pose.x_rot = v;
    }
}

/// VANILLA: CreeperModel#createBodyLayer (64×32).
pub fn creeper_mesh(grow: f32) -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 6.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(grow)],
    );
    m.add(
        "body",
        None,
        Pose::offset(0.0, 6.0, 0.0),
        &[BoxDef::new([16.0, 16.0], [-4.0, 0.0, -2.0], [8.0, 12.0, 4.0]).grow(grow)],
    );
    let leg = BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, 6.0, 4.0]).grow(grow);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-2.0, 18.0, 4.0),
        &[leg],
    );
    m.add("left_hind_leg", None, Pose::offset(2.0, 18.0, 4.0), &[leg]);
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-2.0, 18.0, -4.0),
        &[leg],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(2.0, 18.0, -4.0),
        &[leg],
    );
    m
}

/// VANILLA: CreeperModel#setupAnim. Поля left/right в конструкторе привязаны к противоположным
/// детям, поэтому фазы ног обратны QuadrupedModel: right_hind=b, left_hind=a, right_front=a, left_front=b.
pub fn creeper_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let a = (walk_pos * 0.6662).cos() * 1.4 * walk_speed;
    let b = (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed;
    set_x(m, "right_hind_leg", b);
    set_x(m, "left_hind_leg", a);
    set_x(m, "right_front_leg", a);
    set_x(m, "left_front_leg", b);
}

/// VANILLA: SpiderModel#createSpiderBodyLayer (64×32).
pub fn spider_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 15.0, -3.0),
        &[BoxDef::new(
            [32.0, 4.0],
            [-4.0, -4.0, -8.0],
            [8.0, 8.0, 8.0],
        )],
    );
    m.add(
        "body0",
        None,
        Pose::offset(0.0, 15.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-3.0, -3.0, -3.0], [6.0, 6.0, 6.0])],
    );
    m.add(
        "body1",
        None,
        Pose::offset(0.0, 15.0, 9.0),
        &[BoxDef::new(
            [0.0, 12.0],
            [-5.0, -4.0, -6.0],
            [10.0, 8.0, 12.0],
        )],
    );
    let right = BoxDef::new([18.0, 0.0], [-15.0, -1.0, -1.0], [16.0, 2.0, 2.0]);
    let left = BoxDef::new([18.0, 0.0], [-1.0, -1.0, -1.0], [16.0, 2.0, 2.0]).mirror();
    let p = |x: f32, z: f32, yr: f32, zr: f32| Pose {
        y_rot: yr,
        z_rot: zr,
        ..Pose::offset(x, 15.0, z)
    };
    let q = PI / 4.0;
    let e = PI / 8.0;
    let mid = 0.581_194_64_f32;
    m.add("right_hind_leg", None, p(-4.0, 2.0, q, -q), &[right]);
    m.add("left_hind_leg", None, p(4.0, 2.0, -q, q), &[left]);
    m.add(
        "right_middle_hind_leg",
        None,
        p(-4.0, 1.0, e, -mid),
        &[right],
    );
    m.add("left_middle_hind_leg", None, p(4.0, 1.0, -e, mid), &[left]);
    m.add(
        "right_middle_front_leg",
        None,
        p(-4.0, 0.0, -e, -mid),
        &[right],
    );
    m.add("left_middle_front_leg", None, p(4.0, 0.0, e, mid), &[left]);
    m.add("right_front_leg", None, p(-4.0, -1.0, -q, -q), &[right]);
    m.add("left_front_leg", None, p(4.0, -1.0, q, q), &[left]);
    m
}

/// VANILLA: SpiderModel#setupAnim.
pub fn spider_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let pos = walk_pos * 0.6662;
    let sp = walk_speed;
    // (имя пары, фаза): hind 0, middle_hind π, middle_front π/2, front 3π/2.
    let groups = [
        ("hind", 0.0),
        ("middle_hind", PI),
        ("middle_front", HALF_PI),
        ("front", PI * 1.5),
    ];
    for (g, phase) in groups {
        let swing = -((pos * 2.0 + phase).cos() * 0.4) * sp;
        let step = ((pos + phase).sin() * 0.4).abs() * sp;
        let (rn, ln) = (format!("right_{g}_leg"), format!("left_{g}_leg"));
        if let Some(i) = m.index(&rn) {
            m.parts[i].pose.y_rot += swing;
            m.parts[i].pose.z_rot += step;
        }
        if let Some(i) = m.index(&ln) {
            m.parts[i].pose.y_rot -= swing;
            m.parts[i].pose.z_rot -= step;
        }
    }
}

/// VANILLA: AdultChickenModel#createBodyLayer (64×32).
pub fn chicken_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 15.0, -4.0),
        &[BoxDef::new([0.0, 0.0], [-2.0, -6.0, -2.0], [4.0, 6.0, 3.0])],
    );
    m.add(
        "beak",
        Some(head),
        Pose::default(),
        &[BoxDef::new(
            [14.0, 0.0],
            [-2.0, -4.0, -4.0],
            [4.0, 2.0, 2.0],
        )],
    );
    m.add(
        "red_thing",
        Some(head),
        Pose::default(),
        &[BoxDef::new(
            [14.0, 4.0],
            [-1.0, -2.0, -3.0],
            [2.0, 2.0, 2.0],
        )],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 16.0, 0.0)
        },
        &[BoxDef::new([0.0, 9.0], [-3.0, -4.0, -3.0], [6.0, 8.0, 6.0])],
    );
    let leg = BoxDef::new([26.0, 0.0], [-1.0, 0.0, -3.0], [3.0, 5.0, 3.0]);
    m.add("right_leg", None, Pose::offset(-2.0, 19.0, 1.0), &[leg]);
    m.add("left_leg", None, Pose::offset(1.0, 19.0, 1.0), &[leg]);
    m.add(
        "right_wing",
        None,
        Pose::offset(-4.0, 13.0, 0.0),
        &[BoxDef::new([24.0, 13.0], [0.0, 0.0, -3.0], [1.0, 4.0, 6.0])],
    );
    m.add(
        "left_wing",
        None,
        Pose::offset(4.0, 13.0, 0.0),
        &[BoxDef::new(
            [24.0, 13.0],
            [-1.0, 0.0, -3.0],
            [1.0, 4.0, 6.0],
        )],
    );
    m
}

/// VANILLA: ChickenModel#setupAnim + AdultChickenModel#setupAnim. `flap`/`flap_speed` — состояние Chicken.
pub fn chicken_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    walk_pos: f32,
    walk_speed: f32,
    flap: f32,
    flap_speed: f32,
) {
    set_head(m, y_rot, x_rot);
    let angle = (flap.sin() + 1.0) * flap_speed;
    set_x(m, "right_leg", (walk_pos * 0.6662).cos() * 1.4 * walk_speed);
    set_x(
        m,
        "left_leg",
        (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed,
    );
    if let Some(i) = m.index("right_wing") {
        m.parts[i].pose.z_rot = angle;
    }
    if let Some(i) = m.index("left_wing") {
        m.parts[i].pose.z_rot = -angle;
    }
}

/// VANILLA: SheepModel#createBodyLayer (64×32): createBodyMesh(12, false, true) + своя голова/тело.
pub fn sheep_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 6.0, -8.0),
        &[BoxDef::new([0.0, 0.0], [-3.0, -4.0, -6.0], [6.0, 6.0, 8.0])],
    );
    quadruped(
        &mut m,
        12.0,
        false,
        true,
        [28.0, 8.0],
        [-4.0, -10.0, -7.0, 8.0, 16.0, 6.0],
    );
    // quadruped() ставит тело на y = 17 − leg = 5 — как в ваниле.
    m
}

/// VANILLA: SheepFurModel#createFurLayer (64×32) — шерсть и подшёрсток (деформации 0.6/1.75/0.5).
pub fn sheep_fur_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 6.0, -8.0),
        &[BoxDef::new([0.0, 0.0], [-3.0, -4.0, -4.0], [6.0, 6.0, 6.0]).grow(0.6)],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 5.0, 2.0)
        },
        &[BoxDef::new([28.0, 8.0], [-4.0, -10.0, -7.0], [8.0, 16.0, 6.0]).grow(1.75)],
    );
    let leg = BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, 6.0, 4.0]).grow(0.5);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-3.0, 12.0, 7.0),
        &[leg],
    );
    m.add("left_hind_leg", None, Pose::offset(3.0, 12.0, 7.0), &[leg]);
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-3.0, 12.0, -5.0),
        &[leg],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(3.0, 12.0, -5.0),
        &[leg],
    );
    m
}

/// VANILLA: ColorLerper.Type.SHEEP#getColor — цвет шерсти по DyeColor (textureDiffuseColor × 0.75,
/// белый = 0xE6E6E6).
pub fn sheep_color(dye: u8) -> [u8; 4] {
    const DIFFUSE: [u32; 16] = [
        16383998, 16351261, 13061821, 3847130, 16701501, 8439583, 15961002, 4673362, 10329495,
        1481884, 8991416, 3949738, 8606770, 6192150, 11546150, 1908001,
    ];
    let d = dye as usize & 15;
    if d == 0 {
        return [230, 230, 230, 255];
    }
    let c = DIFFUSE[d];
    let f = |sh: u32| (((c >> sh) & 255) as f32 * 0.75).floor() as u8;
    [f(16), f(8), f(0), 255]
}

/// VANILLA: Mth#triangleWave.
fn triangle_wave(x: f32, period: f32) -> f32 {
    ((x % period - period * 0.5).abs() - period * 0.25) / (period * 0.25)
}

/// VANILLA: EndermanModel#createBodyLayer (64×32; HumanoidModel.createMesh(NONE, −14)).
pub fn enderman_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 32.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, -13.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([0.0, 16.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(-0.5)],
    );
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, -14.0, 0.0),
        &[BoxDef::new(
            [32.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    let arm = |mirror: bool| {
        let b = BoxDef::new([56.0, 0.0], [-1.0, -2.0, -1.0], [2.0, 30.0, 2.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    let leg = |mirror: bool| {
        let b = BoxDef::new([56.0, 0.0], [-1.0, 0.0, -1.0], [2.0, 30.0, 2.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    let right_arm = m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, -12.0, 0.0),
        &[arm(false)],
    );
    let left_arm = m.add(
        "left_arm",
        None,
        Pose::offset(5.0, -12.0, 0.0),
        &[arm(true)],
    );
    let right_leg = m.add(
        "right_leg",
        None,
        Pose::offset(-2.0, -5.0, 0.0),
        &[leg(false)],
    );
    let left_leg = m.add("left_leg", None, Pose::offset(2.0, -5.0, 0.0), &[leg(true)]);
    (
        m,
        HumanoidParts {
            head,
            body,
            right_arm,
            left_arm,
            right_leg,
            left_leg,
        },
    )
}

/// VANILLA: EndermanModel#setupAnim (после HumanoidModel#setupAnim). `creepy` — Enderman#isCreepy.
pub fn enderman_tweak(m: &mut Model, p: &HumanoidParts, creepy: bool) {
    for i in [p.right_arm, p.left_arm, p.right_leg, p.left_leg] {
        let r = &mut m.parts[i].pose.x_rot;
        *r = (*r * 0.5).clamp(-0.4, 0.4);
    }
    if creepy {
        m.parts[p.head].pose.y -= 5.0;
        if let Some(h) = m.index("hat") {
            m.parts[h].pose.y += 5.0;
        }
    }
}

/// VANILLA: IronGolemModel#createBodyLayer (128×128).
pub fn iron_golem_mesh() -> Model {
    let mut m = Model::new([128.0, 128.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, -7.0, -2.0),
        &[
            BoxDef::new([0.0, 0.0], [-4.0, -12.0, -5.5], [8.0, 10.0, 8.0]),
            BoxDef::new([24.0, 0.0], [-1.0, -5.0, -7.5], [2.0, 4.0, 2.0]),
        ],
    );
    m.add(
        "body",
        None,
        Pose::offset(0.0, -7.0, 0.0),
        &[
            BoxDef::new([0.0, 40.0], [-9.0, -2.0, -6.0], [18.0, 12.0, 11.0]),
            BoxDef::new([0.0, 70.0], [-4.5, 10.0, -3.0], [9.0, 5.0, 6.0]).grow(0.5),
        ],
    );
    m.add(
        "right_arm",
        None,
        Pose::offset(0.0, -7.0, 0.0),
        &[BoxDef::new(
            [60.0, 21.0],
            [-13.0, -2.5, -3.0],
            [4.0, 30.0, 6.0],
        )],
    );
    m.add(
        "left_arm",
        None,
        Pose::offset(0.0, -7.0, 0.0),
        &[BoxDef::new(
            [60.0, 58.0],
            [9.0, -2.5, -3.0],
            [4.0, 30.0, 6.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-4.0, 11.0, 0.0),
        &[BoxDef::new(
            [37.0, 0.0],
            [-3.5, -3.0, -3.0],
            [6.0, 16.0, 5.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(5.0, 11.0, 0.0),
        &[BoxDef::new([60.0, 0.0], [-3.5, -3.0, -3.0], [6.0, 16.0, 5.0]).mirror()],
    );
    m
}

/// VANILLA: IronGolemModel#setupAnim (без атаки/цветка — нужны события сущности).
pub fn iron_golem_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let w = triangle_wave(walk_pos, 13.0);
    set_x(m, "right_arm", (-0.2 + 1.5 * w) * walk_speed);
    set_x(m, "left_arm", (-0.2 - 1.5 * w) * walk_speed);
    set_x(m, "right_leg", -1.5 * w * walk_speed);
    set_x(m, "left_leg", 1.5 * w * walk_speed);
}

const BLAZE_PARTS: [&str; 12] = [
    "part0", "part1", "part2", "part3", "part4", "part5", "part6", "part7", "part8", "part9",
    "part10", "part11",
];

/// VANILLA: BlazeModel#createBodyLayer (64×32).
pub fn blaze_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 0.0], [-4.0, -4.0, -4.0], [8.0, 8.0, 8.0])],
    );
    let rod = BoxDef::new([0.0, 16.0], [0.0, 0.0, 0.0], [2.0, 8.0, 2.0]);
    for n in BLAZE_PARTS {
        m.add(n, None, Pose::default(), &[rod]);
    }
    m
}

/// VANILLA: BlazeModel#setupAnim (позиции стержней считаются здесь, исходные не нужны).
pub fn blaze_anim(m: &mut Model, y_rot: f32, x_rot: f32, age: f32) {
    set_head(m, y_rot, x_rot);
    let mut set = |i: usize, x: f32, y: f32, z: f32| {
        if let Some(k) = m.index(BLAZE_PARTS[i]) {
            let p = &mut m.parts[k].pose;
            p.x = x;
            p.y = y;
            p.z = z;
        }
    };
    let mut angle = age * PI * -0.1;
    for i in 0..4 {
        set(
            i,
            angle.cos() * 9.0,
            -2.0 + (((i * 2) as f32 + age) * 0.25).cos(),
            angle.sin() * 9.0,
        );
        angle += HALF_PI;
    }
    angle = PI / 4.0 + age * PI * 0.03;
    for i in 4..8 {
        set(
            i,
            angle.cos() * 7.0,
            2.0 + (((i * 2) as f32 + age) * 0.25).cos(),
            angle.sin() * 7.0,
        );
        angle += HALF_PI;
    }
    angle = 0.471_238_9 + age * PI * -0.05;
    for i in 8..12 {
        set(
            i,
            angle.cos() * 5.0,
            11.0 + ((i as f32 * 1.5 + age) * 0.5).cos(),
            angle.sin() * 5.0,
        );
        angle += HALF_PI;
    }
}

/// VANILLA: SlimeModel#createInnerBodyLayer (64×32) — ядро, глаза, рот.
pub fn slime_inner_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "cube",
        None,
        Pose::default(),
        &[BoxDef::new(
            [0.0, 16.0],
            [-3.0, 17.0, -3.0],
            [6.0, 6.0, 6.0],
        )],
    );
    m.add(
        "right_eye",
        None,
        Pose::default(),
        &[BoxDef::new(
            [32.0, 0.0],
            [-3.25, 18.0, -3.5],
            [2.0, 2.0, 2.0],
        )],
    );
    m.add(
        "left_eye",
        None,
        Pose::default(),
        &[BoxDef::new(
            [32.0, 4.0],
            [1.25, 18.0, -3.5],
            [2.0, 2.0, 2.0],
        )],
    );
    m.add(
        "mouth",
        None,
        Pose::default(),
        &[BoxDef::new([32.0, 8.0], [0.0, 21.0, -3.5], [1.0, 1.0, 1.0])],
    );
    m
}

/// VANILLA: SlimeModel#createOuterBodyLayer (64×32).
pub fn slime_outer_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "cube",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 0.0], [-4.0, 16.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m
}

const MAGMA_PARTS: [&str; 8] = [
    "cube0", "cube1", "cube2", "cube3", "cube4", "cube5", "cube6", "cube7",
];

/// VANILLA: MagmaCubeModel#createBodyLayer (64×64).
pub fn magma_cube_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    for (i, n) in MAGMA_PARTS.iter().enumerate() {
        let (mut u, mut v) = (0.0, 0.0);
        if i > 0 && i < 4 {
            v += 9.0 * i as f32;
        } else if i > 3 {
            u = 32.0;
            v += 9.0 * i as f32 - 36.0;
        }
        m.add(
            n,
            None,
            Pose::default(),
            &[BoxDef::new(
                [u, v],
                [-4.0, 16.0 + i as f32, -4.0],
                [8.0, 1.0, 8.0],
            )],
        );
    }
    m.add(
        "inside_cube",
        None,
        Pose::default(),
        &[BoxDef::new(
            [24.0, 40.0],
            [-2.0, 18.0, -2.0],
            [4.0, 4.0, 4.0],
        )],
    );
    m
}

/// VANILLA: MagmaCubeModel#setupAnim.
pub fn magma_cube_anim(m: &mut Model, squish: f32) {
    let s = squish.max(0.0);
    for (i, n) in MAGMA_PARTS.iter().enumerate() {
        if let Some(k) = m.index(n) {
            m.parts[k].pose.y = -(4.0 - i as f32) * s * 1.7;
        }
    }
}

/// VANILLA: GhastModel#createBodyLayer (64×32, MeshTransformer.scaling(4.5) — масштаб задаёт вызывающий).
/// Длины щупалец — `RandomSource(1660)`: legacy LCG, `nextInt(7) + 8`.
pub fn ghast_mesh() -> Model {
    const NAMES: [&str; 9] = [
        "tentacle0",
        "tentacle1",
        "tentacle2",
        "tentacle3",
        "tentacle4",
        "tentacle5",
        "tentacle6",
        "tentacle7",
        "tentacle8",
    ];
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "body",
        None,
        Pose::offset(0.0, 17.6, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-8.0, -8.0, -8.0],
            [16.0, 16.0, 16.0],
        )],
    );
    const MASK: u64 = (1 << 48) - 1;
    let mut seed: u64 = (1660 ^ 0x5_DEEC_E66D) & MASK;
    let mut next31 = || {
        seed = (seed.wrapping_mul(0x5_DEEC_E66D).wrapping_add(0xB)) & MASK;
        (seed >> 17) as i32
    };
    for (i, n) in NAMES.iter().enumerate() {
        let xo = (((i % 3) as f32 - (i / 3 % 2) as f32 * 0.5 + 0.25) / 2.0 * 2.0 - 1.0) * 5.0;
        let yo = ((i / 3) as f32 / 2.0 * 2.0 - 1.0) * 5.0;
        // Random#nextInt(7): bound не степень двойки.
        let len = loop {
            let bits = next31();
            let val = bits % 7;
            if bits.wrapping_sub(val).wrapping_add(6) >= 0 {
                break val + 8;
            }
        };
        m.add(
            n,
            None,
            Pose::offset(xo, 24.6, yo),
            &[BoxDef::new(
                [0.0, 0.0],
                [-1.0, 0.0, -1.0],
                [2.0, len as f32, 2.0],
            )],
        );
    }
    m
}

/// VANILLA: HappyGhastModel#createBodyLayer (64×64; масштаб ×4 — `mscale` в клиенте, у детёныша ×0.95).
pub fn happy_ghast_mesh(baby: bool) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 16.0, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-8.0, -8.0, -8.0],
            [16.0, 16.0, 16.0],
        )],
    );
    if baby {
        m.add(
            "inner_body",
            Some(body),
            Pose::offset(0.0, 8.0, 0.0),
            &[BoxDef::new([0.0, 32.0], [-8.0, -16.0, -8.0], [16.0, 16.0, 16.0]).grow(-0.5)],
        );
    }
    const T: [(f32, f32, f32); 9] = [
        (-3.75, -5.0, 5.0),
        (1.25, -5.0, 7.0),
        (6.25, -5.0, 4.0),
        (-6.25, 0.0, 5.0),
        (-1.25, 0.0, 5.0),
        (3.75, 0.0, 7.0),
        (-3.75, 5.0, 8.0),
        (1.25, 5.0, 8.0),
        (6.25, 5.0, 5.0),
    ];
    const N: [&str; 9] = [
        "tentacle0",
        "tentacle1",
        "tentacle2",
        "tentacle3",
        "tentacle4",
        "tentacle5",
        "tentacle6",
        "tentacle7",
        "tentacle8",
    ];
    for (i, (x, z, len)) in T.into_iter().enumerate() {
        m.add(
            N[i],
            Some(body),
            Pose::offset(x, 7.0, z),
            &[BoxDef::new([0.0, 0.0], [-1.0, 0.0, -1.0], [2.0, len, 2.0])],
        );
    }
    m
}

/// VANILLA: GhastModel#animateTentacles.
pub fn ghast_anim(m: &mut Model, age: f32) {
    for i in 0..9 {
        let name = format!("tentacle{i}");
        if let Some(k) = m.index(&name) {
            m.parts[k].pose.x_rot = 0.2 * (age * 0.3 + i as f32).sin() + 0.4;
        }
    }
}

const SF_SIZES: [[f32; 3]; 7] = [
    [3.0, 2.0, 2.0],
    [4.0, 3.0, 2.0],
    [6.0, 4.0, 3.0],
    [3.0, 3.0, 3.0],
    [2.0, 2.0, 3.0],
    [2.0, 1.0, 2.0],
    [1.0, 1.0, 2.0],
];
const SF_TEX: [[f32; 2]; 7] = [
    [0.0, 0.0],
    [0.0, 4.0],
    [0.0, 9.0],
    [0.0, 16.0],
    [0.0, 22.0],
    [11.0, 0.0],
    [13.0, 4.0],
];
const SEG: [&str; 7] = [
    "segment0", "segment1", "segment2", "segment3", "segment4", "segment5", "segment6",
];
const LAYER: [&str; 3] = ["layer0", "layer1", "layer2"];

/// VANILLA: SilverfishModel#createBodyLayer (64×32).
pub fn silverfish_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    let mut z = [0.0f32; 7];
    let mut placement = -3.5;
    for i in 0..7 {
        let s = SF_SIZES[i];
        m.add(
            SEG[i],
            None,
            Pose::offset(0.0, 24.0 - s[1], placement),
            &[BoxDef::new(SF_TEX[i], [s[0] * -0.5, 0.0, s[2] * -0.5], s)],
        );
        z[i] = placement;
        if i < 6 {
            placement += (s[2] + SF_SIZES[i + 1][2]) * 0.5;
        }
    }
    m.add(
        LAYER[0],
        None,
        Pose::offset(0.0, 16.0, z[2]),
        &[BoxDef::new(
            [20.0, 0.0],
            [-5.0, 0.0, SF_SIZES[2][2] * -0.5],
            [10.0, 8.0, SF_SIZES[2][2]],
        )],
    );
    m.add(
        LAYER[1],
        None,
        Pose::offset(0.0, 20.0, z[4]),
        &[BoxDef::new(
            [20.0, 11.0],
            [-3.0, 0.0, SF_SIZES[4][2] * -0.5],
            [6.0, 4.0, SF_SIZES[4][2]],
        )],
    );
    m.add(
        LAYER[2],
        None,
        Pose::offset(0.0, 19.0, z[1]),
        &[BoxDef::new(
            [20.0, 18.0],
            [-3.0, 0.0, SF_SIZES[4][2] * -0.5],
            [6.0, 5.0, SF_SIZES[1][2]],
        )],
    );
    m
}

/// VANILLA: SilverfishModel#setupAnim.
pub fn silverfish_anim(m: &mut Model, age: f32) {
    let mut ry = [0.0f32; 7];
    let mut px = [0.0f32; 7];
    for i in 0..7 {
        let a = age * 0.9 + i as f32 * 0.15 * PI;
        let d = (i as f32 - 2.0).abs();
        ry[i] = a.cos() * PI * 0.05 * (1.0 + d);
        px[i] = a.sin() * PI * 0.2 * d;
        if let Some(k) = m.index(SEG[i]) {
            m.parts[k].pose.y_rot = ry[i];
            m.parts[k].pose.x = px[i];
        }
    }
    for (l, src) in [(0, 2), (1, 4), (2, 1)] {
        if let Some(k) = m.index(LAYER[l]) {
            m.parts[k].pose.y_rot = ry[src];
            if l > 0 {
                m.parts[k].pose.x = px[src];
            }
        }
    }
}

/// VANILLA: EndermiteModel#createBodyLayer (64×32).
pub fn endermite_mesh() -> Model {
    const SIZES: [[f32; 3]; 4] = [
        [4.0, 3.0, 2.0],
        [6.0, 4.0, 5.0],
        [3.0, 3.0, 1.0],
        [1.0, 2.0, 1.0],
    ];
    const TEX: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 5.0], [0.0, 14.0], [0.0, 18.0]];
    let mut m = Model::new([64.0, 32.0]);
    let mut placement = -3.5;
    for i in 0..4 {
        let s = SIZES[i];
        m.add(
            SEG[i],
            None,
            Pose::offset(0.0, 24.0 - s[1], placement),
            &[BoxDef::new(TEX[i], [s[0] * -0.5, 0.0, s[2] * -0.5], s)],
        );
        if i < 3 {
            placement += (s[2] + SIZES[i + 1][2]) * 0.5;
        }
    }
    m
}

/// VANILLA: EndermiteModel#setupAnim.
#[allow(clippy::needless_range_loop)]
pub fn endermite_anim(m: &mut Model, age: f32) {
    for i in 0..4 {
        let a = age * 0.9 + i as f32 * 0.15 * PI;
        let d = (i as f32 - 2.0).abs();
        if let Some(k) = m.index(SEG[i]) {
            m.parts[k].pose.y_rot = a.cos() * PI * 0.01 * (1.0 + d);
            m.parts[k].pose.x = a.sin() * PI * 0.1 * d;
        }
    }
}

/// VANILLA: VillagerModel#createBodyModel (64×64).
pub fn villager_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new(
            [0.0, 0.0],
            [-4.0, -10.0, -4.0],
            [8.0, 10.0, 8.0],
        )],
    );
    // VANILLA: VillagerModel#createBodyModel — hat (деформация 0.51) и hat_rim (поворот −π/2 по X).
    let hat = m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -10.0, -4.0], [8.0, 10.0, 8.0]).grow(0.51)],
    );
    m.add(
        "hat_rim",
        Some(hat),
        Pose {
            x_rot: -HALF_PI,
            ..Pose::default()
        },
        &[BoxDef::new(
            [30.0, 47.0],
            [-8.0, -8.0, -6.0],
            [16.0, 16.0, 1.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -2.0, 0.0),
        &[BoxDef::new(
            [24.0, 0.0],
            [-1.0, -1.0, -6.0],
            [2.0, 4.0, 2.0],
        )],
    );
    let body = m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new(
            [16.0, 20.0],
            [-4.0, 0.0, -3.0],
            [8.0, 12.0, 6.0],
        )],
    );
    m.add(
        "jacket",
        Some(body),
        Pose::default(),
        &[BoxDef::new([0.0, 38.0], [-4.0, 0.0, -3.0], [8.0, 20.0, 6.0]).grow(0.5)],
    );
    m.add(
        "arms",
        None,
        Pose {
            x_rot: -0.75,
            ..Pose::offset(0.0, 3.0, -1.0)
        },
        &[
            BoxDef::new([44.0, 22.0], [-8.0, -2.0, -2.0], [4.0, 8.0, 4.0]),
            BoxDef::new([44.0, 22.0], [4.0, -2.0, -2.0], [4.0, 8.0, 4.0]).mirror(),
            BoxDef::new([40.0, 38.0], [-4.0, 2.0, -2.0], [8.0, 4.0, 4.0]),
        ],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-2.0, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 22.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(2.0, 12.0, 0.0),
        &[BoxDef::new([0.0, 22.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    m
}

/// VANILLA: BabyVillagerModel#createBodyModel (64×64).
pub fn villager_baby_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let arms = m.add("arms", None, Pose::offset(0.0, 17.5, 0.0), &[]);
    m.add(
        "right_hand",
        Some(arms),
        Pose {
            x_rot: -1.0472,
            ..Pose::offset(-3.0, 1.4025, -0.9599)
        },
        &[
            BoxDef::new([36.0, 15.0], [-1.0, -2.4925, -1.8401], [2.0, 4.0, 2.0]),
            BoxDef::new([16.0, 15.0], [5.0, -2.4925, -1.8401], [2.0, 4.0, 2.0]),
        ],
    );
    m.add(
        "middlearm_r1",
        Some(arms),
        Pose {
            x_rot: -1.0472,
            ..Pose::offset(0.0, 0.9024, -1.8175)
        },
        &[BoxDef::new(
            [24.0, 17.0],
            [-2.0, -0.9924, -0.9825],
            [4.0, 2.0, 2.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-1.0, 21.5, 0.0),
        &[BoxDef::new(
            [8.0, 23.0],
            [-1.0, -0.5, -1.0],
            [2.0, 3.0, 2.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(1.0, 21.5, 0.0),
        &[BoxDef::new(
            [0.0, 23.0],
            [-1.0, -0.5, -1.0],
            [2.0, 3.0, 2.0],
        )],
    );
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 16.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -3.5], [8.0, 8.0, 7.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::offset(0.0, -4.0, 0.0),
        &[BoxDef::new([0.0, 30.0], [-4.0, -4.0, -3.5], [8.0, 8.0, 7.0]).grow(0.3)],
    );
    m.add(
        "hat_rim",
        Some(head),
        Pose::offset(0.0, -4.5, 0.0),
        &[BoxDef::new(
            [0.0, 45.0],
            [-7.0, -0.5, -6.0],
            [14.0, 1.0, 12.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -2.0, -4.0),
        &[BoxDef::new([23.0, 0.0], [-1.0, 0.0, -0.5], [2.0, 2.0, 1.0])],
    );
    m.add(
        "body",
        None,
        Pose::offset(0.0, 18.75, 0.0),
        &[BoxDef::new(
            [0.0, 15.0],
            [-2.0, -2.75, -1.5],
            [4.0, 5.0, 3.0],
        )],
    );
    m.add(
        "bb_main",
        None,
        Pose::offset(0.5, 24.0, 0.0),
        &[BoxDef::new([16.0, 21.0], [-2.5, -8.0, -1.5], [4.0, 6.0, 3.0]).grow(0.2)],
    );
    m
}

/// VANILLA: VillagerModel#setupAnim.
pub fn villager_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    walk_pos: f32,
    walk_speed: f32,
    unhappy: bool,
    age: f32,
) {
    set_head(m, y_rot, x_rot);
    // VANILLA: VillagerModel#setupAnim — isUnhappy: головой «нет».
    if let Some(i) = m.index("head") {
        if unhappy {
            m.parts[i].pose.z_rot = 0.3 * (0.45 * age).sin();
            m.parts[i].pose.x_rot = 0.4;
        } else {
            m.parts[i].pose.z_rot = 0.0;
        }
    }
    set_x(
        m,
        "right_leg",
        (walk_pos * 0.6662).cos() * 1.4 * walk_speed * 0.5,
    );
    set_x(
        m,
        "left_leg",
        (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed * 0.5,
    );
}

/// VANILLA: AdultWolfModel#createBodyLayer (64×32).
pub fn wolf_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    let head = m.add("head", None, Pose::offset(-1.0, 13.5, -7.0), &[]);
    m.add(
        "real_head",
        Some(head),
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-2.0, -3.0, -2.0], [6.0, 6.0, 4.0]),
            BoxDef::new([16.0, 14.0], [-2.0, -5.0, 0.0], [2.0, 2.0, 1.0]),
            BoxDef::new([16.0, 14.0], [2.0, -5.0, 0.0], [2.0, 2.0, 1.0]),
            BoxDef::new([0.0, 10.0], [-0.5, -0.001, -5.0], [3.0, 3.0, 4.0]),
        ],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 14.0, 2.0)
        },
        &[BoxDef::new(
            [18.0, 14.0],
            [-3.0, -2.0, -3.0],
            [6.0, 9.0, 6.0],
        )],
    );
    m.add(
        "upper_body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(-1.0, 14.0, -3.0)
        },
        &[BoxDef::new(
            [21.0, 0.0],
            [-3.0, -3.0, -3.0],
            [8.0, 6.0, 7.0],
        )],
    );
    let leg = |mirror: bool| {
        let b = BoxDef::new([0.0, 18.0], [0.0, 0.0, -1.0], [2.0, 8.0, 2.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-2.5, 16.0, 7.0),
        &[leg(true)],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(0.5, 16.0, 7.0),
        &[leg(false)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-2.5, 16.0, -4.0),
        &[leg(true)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(0.5, 16.0, -4.0),
        &[leg(false)],
    );
    let tail = m.add(
        "tail",
        None,
        Pose {
            x_rot: PI / 5.0,
            ..Pose::offset(-1.0, 12.0, 8.0)
        },
        &[],
    );
    m.add(
        "real_tail",
        Some(tail),
        Pose::default(),
        &[BoxDef::new([9.0, 18.0], [0.0, 0.0, -1.0], [2.0, 8.0, 2.0])],
    );
    m
}

/// VANILLA: WolfModel#setupAnim (спокойный волк: не сидит, не стряхивает воду; tailAngle = π/5).
pub fn wolf_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let a = (walk_pos * 0.6662).cos() * 1.4 * walk_speed;
    let b = (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed;
    set_x(m, "right_hind_leg", a);
    set_x(m, "left_hind_leg", b);
    set_x(m, "right_front_leg", b);
    set_x(m, "left_front_leg", a);
    if let Some(i) = m.index("tail") {
        m.parts[i].pose.y_rot = a;
    }
}

/// VANILLA: AdultFelineModel#createBodyMesh (64×32; ocelot; кот — тот же меш × 0.8, задаёт вызывающий).
pub fn feline_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 15.0, -9.0),
        &[
            BoxDef::new([0.0, 0.0], [-2.5, -2.0, -3.0], [5.0, 4.0, 5.0]),
            BoxDef::new([0.0, 24.0], [-1.5, -0.001, -4.0], [3.0, 2.0, 2.0]),
            BoxDef::new([0.0, 10.0], [-2.0, -3.0, 0.0], [1.0, 1.0, 2.0]),
            BoxDef::new([6.0, 10.0], [1.0, -3.0, 0.0], [1.0, 1.0, 2.0]),
        ],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 12.0, -10.0)
        },
        &[BoxDef::new(
            [20.0, 0.0],
            [-2.0, 3.0, -8.0],
            [4.0, 16.0, 6.0],
        )],
    );
    m.add(
        "tail1",
        None,
        Pose {
            x_rot: 0.9,
            ..Pose::offset(0.0, 15.0, 8.0)
        },
        &[BoxDef::new([0.0, 15.0], [-0.5, 0.0, 0.0], [1.0, 8.0, 1.0])],
    );
    m.add(
        "tail2",
        None,
        Pose::offset(0.0, 20.0, 14.0),
        &[BoxDef::new([4.0, 15.0], [-0.5, 0.0, 0.0], [1.0, 8.0, 1.0]).grow(-0.02)],
    );
    let hind = BoxDef::new([8.0, 13.0], [-1.0, 0.0, 1.0], [2.0, 6.0, 2.0]);
    let front = BoxDef::new([40.0, 0.0], [-1.0, 0.0, 0.0], [2.0, 10.0, 2.0]);
    m.add("left_hind_leg", None, Pose::offset(1.1, 18.0, 5.0), &[hind]);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-1.1, 18.0, 5.0),
        &[hind],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(1.2, 14.1, -5.0),
        &[front],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-1.2, 14.1, -5.0),
        &[front],
    );
    m
}

/// VANILLA: AdultFelineModel#setupAnim (без сидения/лежания/присядки; бег — флаг сущности).
pub fn feline_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    walk_pos: f32,
    walk_speed: f32,
    sprint: bool,
) {
    set_head(m, y_rot, x_rot);
    let (p, s) = (walk_pos, walk_speed);
    let c = |a: f32| (p * 0.6662 + a).cos() * s;
    if sprint {
        set_x(m, "tail1", HALF_PI);
        set_x(m, "left_hind_leg", c(0.0));
        set_x(m, "right_hind_leg", c(0.3));
        set_x(m, "left_front_leg", c(PI + 0.3));
        set_x(m, "right_front_leg", c(PI));
        set_x(m, "tail2", 1.727_876_1 + PI / 10.0 * p.cos() * s);
        if let (Some(t1), Some(t2)) = (m.index("tail1"), m.index("tail2")) {
            m.parts[t2].pose.y = m.parts[t1].pose.y;
            m.parts[t2].pose.z += 2.0;
        }
    } else {
        set_x(m, "left_hind_leg", c(0.0));
        set_x(m, "right_hind_leg", c(PI));
        set_x(m, "left_front_leg", c(PI));
        set_x(m, "right_front_leg", c(0.0));
        set_x(m, "tail2", 1.727_876_1 + PI / 4.0 * p.cos() * s);
    }
}

/// VANILLA: AdultPiglinModel#createBodyLayer (64×64): PlayerModel (без оверлеев) + голова с пятачком и ушами.
pub fn piglin_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-5.0, -8.0, -4.0], [10.0, 8.0, 8.0]),
            BoxDef::new([31.0, 1.0], [-2.0, -4.0, -5.0], [4.0, 4.0, 1.0]),
            BoxDef::new([2.0, 4.0], [2.0, -2.0, -5.0], [1.0, 2.0, 1.0]),
            BoxDef::new([2.0, 0.0], [-3.0, -2.0, -5.0], [1.0, 2.0, 1.0]),
        ],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose {
            z_rot: -PI / 6.0,
            ..Pose::offset(4.5, -6.0, 0.0)
        },
        &[BoxDef::new([51.0, 6.0], [0.0, 0.0, -2.0], [1.0, 5.0, 4.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose {
            z_rot: PI / 6.0,
            ..Pose::offset(-4.5, -6.0, 0.0)
        },
        &[BoxDef::new([39.0, 6.0], [-1.0, 0.0, -2.0], [1.0, 5.0, 4.0])],
    );
    let body = m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new(
            [16.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    let right_arm = m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [40.0, 16.0],
            [-3.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let left_arm = m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new(
            [32.0, 48.0],
            [-1.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let right_leg = m.add(
        "right_leg",
        None,
        Pose::offset(-1.9, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 16.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let left_leg = m.add(
        "left_leg",
        None,
        Pose::offset(1.9, 12.0, 0.0),
        &[BoxDef::new(
            [16.0, 48.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    (
        m,
        HumanoidParts {
            head,
            body,
            right_arm,
            left_arm,
            right_leg,
            left_leg,
        },
    )
}

/// VANILLA: AbstractPiglinModel#setupAnim — покачивание ушей (взрослый: 30°).
pub fn piglin_ears(m: &mut Model, age: f32, walk_pos: f32, walk_speed: f32) {
    let default = 30.0_f32.to_radians();
    let f = age * 0.1 + walk_pos * 0.5;
    let amp = 0.08 + walk_speed * 0.4;
    if let Some(i) = m.index("left_ear") {
        m.parts[i].pose.z_rot = -default - (f * 1.2).cos() * amp;
    }
    if let Some(i) = m.index("right_ear") {
        m.parts[i].pose.z_rot = default + f.cos() * amp;
    }
}

fn humanoid_parts(m: &Model) -> HumanoidParts {
    let i = |n: &str| m.index(n).unwrap_or(0);
    HumanoidParts {
        head: i("head"),
        body: i("body"),
        right_arm: i("right_arm"),
        left_arm: i("left_arm"),
        right_leg: i("right_leg"),
        left_leg: i("left_leg"),
    }
}

/// VANILLA: DrownedModel#createBodyLayer (64×64): зомби, но левая рука/нога с собственной раскладкой UV.
// VANILLA-UNSURE: внешний слой drowned_outer_layer (grow 0.25) и анимация плавания не портированы.
pub fn drowned_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(0.5)],
    );
    m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new(
            [16.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [40.0, 16.0],
            [-3.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new(
            [32.0, 48.0],
            [-1.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-1.9, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 16.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(1.9, 12.0, 0.0),
        &[BoxDef::new(
            [16.0, 48.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    let p = humanoid_parts(&m);
    (m, p)
}

/// VANILLA: ZombieVillagerModel#createBodyLayer (64×64), без шляпы/полей (hat_rim скрыт без профессии).
// VANILLA-UNSURE: слой профессии/биома (VillagerProfessionLayer) не портирован.
pub fn zombie_villager_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-4.0, -10.0, -4.0], [8.0, 10.0, 8.0]),
            BoxDef::new([24.0, 0.0], [-1.0, -3.0, -6.0], [2.0, 4.0, 2.0]),
        ],
    );
    // VANILLA: ZombieVillagerModel#createBodyLayer — hat (0.5) и hat_rim (поворот −π/2 по X).
    let hat = m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -10.0, -4.0], [8.0, 10.0, 8.0]).grow(0.5)],
    );
    m.add(
        "hat_rim",
        Some(hat),
        Pose {
            x_rot: -HALF_PI,
            ..Pose::default()
        },
        &[BoxDef::new(
            [30.0, 47.0],
            [-8.0, -8.0, -6.0],
            [16.0, 16.0, 1.0],
        )],
    );
    m.add(
        "body",
        None,
        Pose::default(),
        &[
            BoxDef::new([16.0, 20.0], [-4.0, 0.0, -3.0], [8.0, 12.0, 6.0]),
            BoxDef::new([0.0, 38.0], [-4.0, 0.0, -3.0], [8.0, 20.0, 6.0]).grow(0.05),
        ],
    );
    m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [44.0, 22.0],
            [-3.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new([44.0, 22.0], [-1.0, -2.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-2.0, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 22.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(2.0, 12.0, 0.0),
        &[BoxDef::new([0.0, 22.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    let p = humanoid_parts(&m);
    (m, p)
}

/// VANILLA: BabyZombieVillagerModel#createBodyLayer (64×64).
pub fn zombie_villager_baby_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "body",
        None,
        Pose::offset(0.0, 18.75, 0.0),
        &[
            BoxDef::new([0.0, 15.0], [-2.0, -2.75, -1.5], [4.0, 5.0, 3.0]),
            BoxDef::new([16.0, 22.0], [-2.0, -2.75, -1.5], [4.0, 6.0, 3.0]).grow(0.1),
        ],
    );
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 16.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -3.5], [8.0, 8.0, 7.0])],
    );
    m.add(
        "hat",
        Some(head),
        Pose::offset(0.0, -4.0, 0.0),
        &[BoxDef::new([0.0, 31.0], [-4.0, -4.0, -3.5], [8.0, 8.0, 7.0]).grow(0.3)],
    );
    m.add(
        "hat_rim",
        Some(head),
        Pose::offset(0.0, -4.5, 0.0),
        &[BoxDef::new(
            [0.0, 46.0],
            [-7.0, -0.5, -6.0],
            [14.0, 1.0, 12.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -1.0, -4.0),
        &[BoxDef::new(
            [23.0, 0.0],
            [-1.0, -1.0, -0.5],
            [2.0, 2.0, 1.0],
        )],
    );
    m.add(
        "right_arm",
        None,
        Pose::offset(-3.0, 15.5, 0.0),
        &[BoxDef::new(
            [24.0, 15.0],
            [-1.0, -0.5, -1.0],
            [2.0, 5.0, 2.0],
        )],
    );
    m.add(
        "left_arm",
        None,
        Pose::offset(3.0, 15.5, 0.0),
        &[BoxDef::new(
            [16.0, 15.0],
            [-1.0, -0.5, -1.0],
            [2.0, 5.0, 2.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-1.0, 21.5, 0.0),
        &[BoxDef::new(
            [8.0, 23.0],
            [-1.0, -0.5, -1.0],
            [2.0, 3.0, 2.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(1.0, 21.5, 0.0),
        &[BoxDef::new(
            [0.0, 23.0],
            [-1.0, -0.5, -1.0],
            [2.0, 3.0, 2.0],
        )],
    );
    let p = humanoid_parts(&m);
    (m, p)
}

/// VANILLA: IllagerModel#createBodyLayer (64×64). Шляпа скрыта, `arms` (скрещённые) и left/right_arm переключаются.
pub fn illager_mesh() -> (Model, HumanoidParts) {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new(
            [0.0, 0.0],
            [-4.0, -10.0, -4.0],
            [8.0, 10.0, 8.0],
        )],
    );
    let hat = m.add(
        "hat",
        Some(head),
        Pose::default(),
        &[BoxDef::new([32.0, 0.0], [-4.0, -10.0, -4.0], [8.0, 12.0, 8.0]).grow(0.45)],
    );
    m.parts[hat].visible = false;
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -2.0, 0.0),
        &[BoxDef::new(
            [24.0, 0.0],
            [-1.0, -1.0, -6.0],
            [2.0, 4.0, 2.0],
        )],
    );
    m.add(
        "body",
        None,
        Pose::default(),
        &[
            BoxDef::new([16.0, 20.0], [-4.0, 0.0, -3.0], [8.0, 12.0, 6.0]),
            BoxDef::new([0.0, 38.0], [-4.0, 0.0, -3.0], [8.0, 20.0, 6.0]).grow(0.5),
        ],
    );
    let arms = m.add(
        "arms",
        None,
        Pose {
            x_rot: -0.75,
            ..Pose::offset(0.0, 3.0, -1.0)
        },
        &[
            BoxDef::new([44.0, 22.0], [-8.0, -2.0, -2.0], [4.0, 8.0, 4.0]),
            BoxDef::new([40.0, 38.0], [-4.0, 2.0, -2.0], [8.0, 4.0, 4.0]),
            BoxDef::new([44.0, 22.0], [4.0, -2.0, -2.0], [4.0, 8.0, 4.0]).mirror(),
        ],
    );
    let _ = arms;
    m.add(
        "right_leg",
        None,
        Pose::offset(-2.0, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 22.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(2.0, 12.0, 0.0),
        &[BoxDef::new([0.0, 22.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    m.add(
        "right_arm",
        None,
        Pose::offset(-5.0, 2.0, 0.0),
        &[BoxDef::new(
            [40.0, 46.0],
            [-3.0, -2.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_arm",
        None,
        Pose::offset(5.0, 2.0, 0.0),
        &[BoxDef::new([40.0, 46.0], [-1.0, -2.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    let p = humanoid_parts(&m);
    (m, p)
}

/// VANILLA: IllagerModel#setupAnim — поза рук: скрещённые (CROSSED) либо атака (ATTACKING без предмета).
// VANILLA-UNSURE: арбалет/заклинание/лук/праздник не портированы; агрессивные руки = зомби-руки + bobArms.
// VANILLA: Pillager#getArmPose — без арбалета: ATTACKING или NEUTRAL (руки висят, не CROSSED).
pub fn illager_pose(
    m: &mut Model,
    p: &HumanoidParts,
    aggressive: bool,
    attack: f32,
    age: f32,
    neutral: bool,
) {
    if aggressive {
        crate::humanoid::zombie_arms(m, p, true, attack);
        // VANILLA: AnimationUtils#bobArms
        let a = age;
        let (r, l) = (p.right_arm, p.left_arm);
        m.parts[r].pose.z_rot += 1.0 * ((a * 0.09).cos() * 0.05 + 0.05);
        m.parts[l].pose.z_rot -= 1.0 * ((a * 0.09).cos() * 0.05 + 0.05);
        m.parts[r].pose.x_rot += (a * 0.067).sin() * 0.05;
        m.parts[l].pose.x_rot -= (a * 0.067).sin() * 0.05;
    }
    if let Some(i) = m.index("arms") {
        m.parts[i].visible = !aggressive && !neutral;
    }
    m.parts[p.right_arm].visible = aggressive || neutral;
    m.parts[p.left_arm].visible = aggressive || neutral;
}

/// VANILLA: WitchModel#createBodyLayer (64×128): деревенская модель + шляпа-конус, родинка.
pub fn witch_mesh() -> Model {
    let mut m = Model::new([64.0, 128.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[BoxDef::new(
            [0.0, 0.0],
            [-4.0, -10.0, -4.0],
            [8.0, 10.0, 8.0],
        )],
    );
    let hat = m.add(
        "hat",
        Some(head),
        Pose::offset(-5.0, -10.03125, -5.0),
        &[BoxDef::new([0.0, 64.0], [0.0, 0.0, 0.0], [10.0, 2.0, 10.0])],
    );
    let hat2 = m.add(
        "hat2",
        Some(hat),
        Pose {
            x_rot: -0.05235988,
            z_rot: 0.02617994,
            ..Pose::offset(1.75, -4.0, 2.0)
        },
        &[BoxDef::new([0.0, 76.0], [0.0, 0.0, 0.0], [7.0, 4.0, 7.0])],
    );
    let hat3 = m.add(
        "hat3",
        Some(hat2),
        Pose {
            x_rot: -0.10471976,
            z_rot: 0.05235988,
            ..Pose::offset(1.75, -4.0, 2.0)
        },
        &[BoxDef::new([0.0, 87.0], [0.0, 0.0, 0.0], [4.0, 4.0, 4.0])],
    );
    m.add(
        "hat4",
        Some(hat3),
        Pose {
            x_rot: -PI / 15.0,
            z_rot: 0.10471976,
            ..Pose::offset(1.75, -2.0, 2.0)
        },
        &[BoxDef::new([0.0, 95.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]).grow(0.25)],
    );
    let nose = m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -2.0, 0.0),
        &[BoxDef::new(
            [24.0, 0.0],
            [-1.0, -1.0, -6.0],
            [2.0, 4.0, 2.0],
        )],
    );
    m.add(
        "mole",
        Some(nose),
        Pose::offset(0.0, -2.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [0.0, 3.0, -6.75], [1.0, 1.0, 1.0]).grow(-0.25)],
    );
    let body = m.add(
        "body",
        None,
        Pose::default(),
        &[BoxDef::new(
            [16.0, 20.0],
            [-4.0, 0.0, -3.0],
            [8.0, 12.0, 6.0],
        )],
    );
    m.add(
        "jacket",
        Some(body),
        Pose::default(),
        &[BoxDef::new([0.0, 38.0], [-4.0, 0.0, -3.0], [8.0, 18.0, 6.0]).grow(0.5)],
    );
    m.add(
        "arms",
        None,
        Pose {
            x_rot: -0.75,
            ..Pose::offset(0.0, 3.0, -1.0)
        },
        &[
            BoxDef::new([44.0, 22.0], [-8.0, -2.0, -2.0], [4.0, 8.0, 4.0]),
            BoxDef::new([44.0, 22.0], [4.0, -2.0, -2.0], [4.0, 8.0, 4.0]).mirror(),
            BoxDef::new([40.0, 38.0], [-4.0, 2.0, -2.0], [8.0, 4.0, 4.0]),
        ],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(-2.0, 12.0, 0.0),
        &[BoxDef::new(
            [0.0, 22.0],
            [-2.0, 0.0, -2.0],
            [4.0, 12.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(2.0, 12.0, 0.0),
        &[BoxDef::new([0.0, 22.0], [-2.0, 0.0, -2.0], [4.0, 12.0, 4.0]).mirror()],
    );
    m
}

/// VANILLA: WitchModel#setupAnim (нос качается по entityId%10; держание предмета не портировано).
pub fn witch_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    walk_pos: f32,
    walk_speed: f32,
    age: f32,
    id: i32,
) {
    set_head(m, y_rot, x_rot);
    set_x(
        m,
        "right_leg",
        (walk_pos * 0.6662).cos() * 1.4 * walk_speed * 0.5,
    );
    set_x(
        m,
        "left_leg",
        (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed * 0.5,
    );
    let speed = 0.01 * (id.rem_euclid(10)) as f32;
    if let Some(i) = m.index("nose") {
        m.parts[i].pose.x_rot = (age * speed).sin() * 4.5 * PI / 180.0;
        m.parts[i].pose.z_rot = (age * speed).cos() * 2.5 * PI / 180.0;
    }
}

/// VANILLA: PolarBearModel#createBodyLayer (128×64); масштаб модели 1.2 задаётся в `mscale`.
// VANILLA-UNSURE: стойка на задних лапах (standScale) не портирована.
pub fn polar_bear_mesh() -> Model {
    let mut m = Model::new([128.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 10.0, -16.0),
        &[
            BoxDef::new([0.0, 0.0], [-3.5, -3.0, -3.0], [7.0, 7.0, 7.0]),
            BoxDef::new([0.0, 44.0], [-2.5, 1.0, -6.0], [5.0, 3.0, 3.0]),
            BoxDef::new([26.0, 0.0], [-4.5, -4.0, -1.0], [2.0, 2.0, 1.0]),
            BoxDef::new([26.0, 0.0], [2.5, -4.0, -1.0], [2.0, 2.0, 1.0]).mirror(),
        ],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(-2.0, 9.0, 12.0)
        },
        &[
            BoxDef::new([0.0, 19.0], [-5.0, -13.0, -7.0], [14.0, 14.0, 11.0]),
            BoxDef::new([39.0, 0.0], [-4.0, -25.0, -7.0], [12.0, 12.0, 10.0]),
        ],
    );
    let hind = [BoxDef::new(
        [50.0, 22.0],
        [-2.0, 0.0, -2.0],
        [4.0, 10.0, 8.0],
    )];
    m.add("right_hind_leg", None, Pose::offset(-4.5, 14.0, 6.0), &hind);
    m.add("left_hind_leg", None, Pose::offset(4.5, 14.0, 6.0), &hind);
    let front = [BoxDef::new(
        [50.0, 40.0],
        [-2.0, 0.0, -2.0],
        [4.0, 10.0, 6.0],
    )];
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-3.5, 14.0, -8.0),
        &front,
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(3.5, 14.0, -8.0),
        &front,
    );
    m
}

/// VANILLA: AdultRabbitModel#createBodyLayer (64×64). Без keyframe-анимации прыжка (RabbitAnimation.HOP).
pub fn rabbit_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose {
            x_rot: -0.3927,
            ..Pose::offset(0.0, 23.0, 4.0)
        },
        &[BoxDef::new(
            [0.0, 0.0],
            [-4.0, -6.0, -9.0],
            [8.0, 6.0, 10.0],
        )],
    );
    m.add(
        "tail",
        Some(body),
        Pose::offset(0.0, -4.9916, 0.0125),
        &[BoxDef::new(
            [20.0, 16.0],
            [-2.0, -3.0084, -1.0125],
            [4.0, 4.0, 4.0],
        )],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose {
            x_rot: 0.3927,
            ..Pose::offset(0.0, -5.2929, -8.1213)
        },
        &[BoxDef::new(
            [0.0, 16.0],
            [-2.5, -3.0, -4.0],
            [5.0, 5.0, 5.0],
        )],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(1.5, -3.7071, -0.8787),
        &[BoxDef::new(
            [32.0, 0.0],
            [-1.0, -4.2929, -0.1213],
            [2.0, 5.0, 1.0],
        )],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-1.5, -3.7071, -0.8787),
        &[BoxDef::new(
            [26.0, 0.0],
            [-1.0, -4.2929, -0.1213],
            [2.0, 5.0, 1.0],
        )],
    );
    let fl = m.add(
        "frontlegs",
        Some(body),
        Pose::offset(0.0, -1.5349, -6.3108),
        &[],
    );
    m.add(
        "right_front_leg",
        Some(fl),
        Pose {
            x_rot: 0.3927,
            ..Pose::offset(-2.0, 1.9239, 0.3827)
        },
        &[BoxDef::new(
            [36.0, 18.0],
            [-0.9, -1.0, -0.9],
            [2.0, 4.0, 2.0],
        )],
    );
    m.add(
        "left_front_leg",
        Some(fl),
        Pose {
            x_rot: 0.3927,
            ..Pose::offset(2.0, 1.9239, 0.4827)
        },
        &[BoxDef::new(
            [44.0, 18.0],
            [-1.0, -1.0, -1.0],
            [2.0, 4.0, 2.0],
        )],
    );
    let bl = m.add("backlegs", None, Pose::offset(0.0, 23.0, 4.0), &[]);
    let rh = m.add(
        "right_hind_leg",
        Some(bl),
        Pose::offset(-3.0, 0.5, 0.0),
        &[],
    );
    m.add(
        "right_haunch",
        Some(rh),
        Pose {
            y_rot: 0.3927,
            ..Pose::offset(0.0, -0.5, 0.0)
        },
        &[BoxDef::new(
            [20.0, 24.0],
            [-1.0, 0.0, -5.0],
            [2.0, 1.0, 6.0],
        )],
    );
    let lh = m.add("left_hind_leg", Some(bl), Pose::offset(3.0, 0.5, 0.0), &[]);
    m.add(
        "left_haunch",
        Some(lh),
        Pose {
            y_rot: -0.3927,
            ..Pose::offset(0.0, -0.5, 0.0)
        },
        &[BoxDef::new(
            [36.0, 24.0],
            [-1.0, 0.0, -5.0],
            [2.0, 1.0, 6.0],
        )],
    );
    m
}

pub fn rabbit_anim(m: &mut Model, y_rot: f32, x_rot: f32) {
    set_head(m, y_rot, x_rot);
}

/// VANILLA: SquidModel#createBodyLayer (64×32). Сдвиг/наклоны — SquidRenderer#setupRotations (в клиенте, mobs.rs).
pub const SQUID_TENTACLES: [&str; 8] = [
    "tentacle0",
    "tentacle1",
    "tentacle2",
    "tentacle3",
    "tentacle4",
    "tentacle5",
    "tentacle6",
    "tentacle7",
];

pub fn squid_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "body",
        None,
        Pose::offset(0.0, 8.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-6.0, -8.0, -6.0], [12.0, 16.0, 12.0]).grow(0.02)],
    );
    for (i, name) in SQUID_TENTACLES.iter().enumerate() {
        let a = i as f32 * PI * 2.0 / 8.0;
        let yr = i as f32 * PI * -2.0 / 8.0 + HALF_PI;
        m.add(
            name,
            None,
            Pose {
                y_rot: yr,
                ..Pose::offset(a.cos() * 5.0, 15.0, a.sin() * 5.0)
            },
            &[BoxDef::new(
                [48.0, 0.0],
                [-1.0, 0.0, -1.0],
                [2.0, 18.0, 2.0],
            )],
        );
    }
    m
}

/// VANILLA: SquidModel#setupAnim (tentacle.xRot = state.tentacleAngle, угол считает Squid#aiStep).
pub fn squid_anim(m: &mut Model, tentacle_angle: f32) {
    for n in SQUID_TENTACLES {
        set_x(m, n, tentacle_angle);
    }
}

/// VANILLA: AdultFoxModel#createBodyLayer (48×32).
pub fn fox_mesh() -> Model {
    let mut m = Model::new([48.0, 32.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(-1.0, 16.5, -3.0),
        &[BoxDef::new([1.0, 5.0], [-3.0, -2.0, -5.0], [8.0, 6.0, 6.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::default(),
        &[BoxDef::new([8.0, 1.0], [-3.0, -4.0, -4.0], [2.0, 2.0, 1.0])],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::default(),
        &[BoxDef::new([15.0, 1.0], [3.0, -4.0, -4.0], [2.0, 2.0, 1.0])],
    );
    m.add(
        "nose",
        Some(head),
        Pose::default(),
        &[BoxDef::new(
            [6.0, 18.0],
            [-1.0, 2.01, -8.0],
            [4.0, 2.0, 3.0],
        )],
    );
    let body = m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 16.0, -6.0)
        },
        &[BoxDef::new(
            [24.0, 15.0],
            [-3.0, 3.999, -3.5],
            [6.0, 11.0, 6.0],
        )],
    );
    let left = BoxDef::new([4.0, 24.0], [2.0, 0.5, -1.0], [2.0, 6.0, 2.0]).grow(0.001);
    let right = BoxDef::new([13.0, 24.0], [2.0, 0.5, -1.0], [2.0, 6.0, 2.0]).grow(0.001);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-5.0, 17.5, 7.0),
        &[right],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(-1.0, 17.5, 7.0),
        &[left],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-5.0, 17.5, 0.0),
        &[right],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(-1.0, 17.5, 0.0),
        &[left],
    );
    m.add(
        "tail",
        Some(body),
        Pose {
            x_rot: -0.05235988,
            ..Pose::offset(-4.0, 15.0, -1.0)
        },
        &[BoxDef::new([30.0, 0.0], [2.0, 0.0, -1.0], [4.0, 9.0, 5.0])],
    );
    m
}

/// VANILLA: FoxModel#setupAnim + AdultFoxModel#setWalkingPose (ходьба и взгляд).
// VANILLA-UNSURE: позы сидения/сна/подкрадывания/прыжка не портированы (нужны флаги Fox#17).
pub fn fox_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_head(m, y_rot, x_rot);
    let a = (walk_pos * 0.6662).cos() * 1.4 * walk_speed;
    let b = (walk_pos * 0.6662 + PI).cos() * 1.4 * walk_speed;
    set_x(m, "right_hind_leg", a);
    set_x(m, "left_hind_leg", b);
    set_x(m, "right_front_leg", b);
    set_x(m, "left_front_leg", a);
}

/// VANILLA: AdultTurtleModel#createBodyLayer (128×64).
pub fn turtle_mesh() -> Model {
    let mut m = Model::new([128.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 19.0, -10.0),
        &[BoxDef::new([3.0, 0.0], [-3.0, -1.0, -3.0], [6.0, 5.0, 6.0])],
    );
    let belly_pose = Pose {
        x_rot: HALF_PI,
        ..Pose::offset(0.0, 11.0, -10.0)
    };
    m.add(
        "body",
        None,
        belly_pose,
        &[
            BoxDef::new([7.0, 37.0], [-9.5, 3.0, -10.0], [19.0, 20.0, 6.0]),
            BoxDef::new([31.0, 1.0], [-5.5, 3.0, -13.0], [11.0, 18.0, 3.0]),
        ],
    );
    let egg = m.add(
        "egg_belly",
        None,
        belly_pose,
        &[BoxDef::new(
            [70.0, 33.0],
            [-4.5, 3.0, -14.0],
            [9.0, 18.0, 1.0],
        )],
    );
    m.parts[egg].visible = false;
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-3.5, 22.0, 11.0),
        &[BoxDef::new([1.0, 23.0], [-2.0, 0.0, 0.0], [4.0, 1.0, 10.0])],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(3.5, 22.0, 11.0),
        &[BoxDef::new([1.0, 12.0], [-2.0, 0.0, 0.0], [4.0, 1.0, 10.0])],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-5.0, 21.0, -4.0),
        &[BoxDef::new(
            [27.0, 30.0],
            [-13.0, 0.0, -2.0],
            [13.0, 1.0, 5.0],
        )],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(5.0, 21.0, -4.0),
        &[BoxDef::new(
            [27.0, 24.0],
            [0.0, 0.0, -2.0],
            [13.0, 1.0, 5.0],
        )],
    );
    m
}

/// VANILLA: TurtleModel#setupAnim. `on_land` ≈ onGround (VANILLA-UNSURE: без проверки isInWater).
pub fn turtle_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    walk_pos: f32,
    walk_speed: f32,
    on_land: bool,
) {
    quadruped_anim(m, y_rot, x_rot, walk_pos, walk_speed);
    let set = |m: &mut Model, n: &str, f: &dyn Fn(&mut Pose)| {
        if let Some(i) = m.index(n) {
            f(&mut m.parts[i].pose);
        }
    };
    if on_land {
        let swing_pos = walk_pos * 5.0;
        let front = swing_pos.cos();
        let hind = swing_pos.cos();
        set(m, "right_front_leg", &|p| {
            p.y_rot = -front * 8.0 * walk_speed
        });
        set(m, "left_front_leg", &|p| p.y_rot = front * 8.0 * walk_speed);
        set(m, "right_hind_leg", &|p| p.y_rot = -hind * 3.0 * walk_speed);
        set(m, "left_hind_leg", &|p| p.y_rot = hind * 3.0 * walk_speed);
    } else {
        let swing = (walk_pos * 0.6662 * 0.6).cos() * 0.5 * walk_speed;
        set(m, "right_hind_leg", &|p| p.x_rot = swing);
        set(m, "left_hind_leg", &|p| p.x_rot = -swing);
        set(m, "right_front_leg", &|p| p.z_rot = -swing);
        set(m, "left_front_leg", &|p| p.z_rot = swing);
    }
}

/// VANILLA: PandaModel#createBodyLayer (64×64).
// VANILLA-UNSURE: позы сидения/лёжа/кувырка/чихания и варианты (gene) не портированы.
pub fn panda_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 11.5, -17.0),
        &[
            BoxDef::new([0.0, 6.0], [-6.5, -5.0, -4.0], [13.0, 10.0, 9.0]),
            BoxDef::new([45.0, 16.0], [-3.5, 0.0, -6.0], [7.0, 5.0, 2.0]),
            BoxDef::new([52.0, 25.0], [3.5, -8.0, -1.0], [5.0, 4.0, 1.0]),
            BoxDef::new([52.0, 25.0], [-8.5, -8.0, -1.0], [5.0, 4.0, 1.0]),
        ],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 10.0, 0.0)
        },
        &[BoxDef::new(
            [0.0, 25.0],
            [-9.5, -13.0, -6.5],
            [19.0, 26.0, 13.0],
        )],
    );
    let leg = [BoxDef::new([40.0, 0.0], [-3.0, 0.0, -3.0], [6.0, 9.0, 6.0])];
    m.add("right_hind_leg", None, Pose::offset(-5.5, 15.0, 9.0), &leg);
    m.add("left_hind_leg", None, Pose::offset(5.5, 15.0, 9.0), &leg);
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-5.5, 15.0, -9.0),
        &leg,
    );
    m.add("left_front_leg", None, Pose::offset(5.5, 15.0, -9.0), &leg);
    m
}

/// VANILLA: AbstractEquineModel#createBodyMesh (64×64) + DonkeyModel#modifyMesh (длинные уши) при `donkey`.
// VANILLA-UNSURE: сундуки осла (hasChest) скрыты; сёдла/броня/маркировки не портированы.
pub fn equine_mesh(donkey: bool) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 11.0, 5.0),
        &[BoxDef::new([0.0, 32.0], [-5.0, -8.0, -17.0], [10.0, 10.0, 22.0]).grow(0.05)],
    );
    let hp = m.add(
        "head_parts",
        None,
        Pose {
            x_rot: PI / 6.0,
            ..Pose::offset(0.0, 4.0, -12.0)
        },
        &[BoxDef::new(
            [0.0, 35.0],
            [-2.05, -6.0, -2.0],
            [4.0, 12.0, 7.0],
        )],
    );
    let head = m.add(
        "head",
        Some(hp),
        Pose::default(),
        &[BoxDef::new(
            [0.0, 13.0],
            [-3.0, -11.0, -2.0],
            [6.0, 5.0, 7.0],
        )],
    );
    m.add(
        "mane",
        Some(hp),
        Pose::default(),
        &[BoxDef::new(
            [56.0, 36.0],
            [-1.0, -11.0, 5.01],
            [2.0, 16.0, 2.0],
        )],
    );
    m.add(
        "upper_mouth",
        Some(hp),
        Pose::default(),
        &[BoxDef::new(
            [0.0, 25.0],
            [-2.0, -11.0, -7.0],
            [4.0, 5.0, 5.0],
        )],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(4.0, 14.0, 7.0),
        &[BoxDef::new([48.0, 21.0], [-3.0, -1.01, -1.0], [4.0, 11.0, 4.0]).mirror()],
    );
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-4.0, 14.0, 7.0),
        &[BoxDef::new(
            [48.0, 21.0],
            [-1.0, -1.01, -1.0],
            [4.0, 11.0, 4.0],
        )],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(4.0, 14.0, -10.0),
        &[BoxDef::new([48.0, 21.0], [-3.0, -1.01, -1.9], [4.0, 11.0, 4.0]).mirror()],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-4.0, 14.0, -10.0),
        &[BoxDef::new(
            [48.0, 21.0],
            [-1.0, -1.01, -1.9],
            [4.0, 11.0, 4.0],
        )],
    );
    m.add(
        "tail",
        Some(body),
        Pose {
            x_rot: PI / 6.0,
            ..Pose::offset(0.0, -5.0, 2.0)
        },
        &[BoxDef::new(
            [42.0, 36.0],
            [-1.5, 0.0, 0.0],
            [3.0, 14.0, 4.0],
        )],
    );
    if donkey {
        let a = PI / 12.0;
        m.add(
            "left_ear",
            Some(head),
            Pose {
                x_rot: a,
                z_rot: a,
                ..Pose::offset(1.25, -10.0, 4.0)
            },
            &[BoxDef::new([0.0, 12.0], [-1.0, -7.0, 0.0], [2.0, 7.0, 1.0])],
        );
        m.add(
            "right_ear",
            Some(head),
            Pose {
                x_rot: a,
                z_rot: -a,
                ..Pose::offset(-1.25, -10.0, 4.0)
            },
            &[BoxDef::new([0.0, 12.0], [-1.0, -7.0, 0.0], [2.0, 7.0, 1.0])],
        );
    } else {
        m.add(
            "left_ear",
            Some(head),
            Pose::default(),
            &[BoxDef::new([19.0, 16.0], [0.55, -13.0, 4.0], [2.0, 3.0, 1.0]).grow(-0.001)],
        );
        m.add(
            "right_ear",
            Some(head),
            Pose::default(),
            &[BoxDef::new([19.0, 16.0], [-2.55, -13.0, 4.0], [2.0, 3.0, 1.0]).grow(-0.001)],
        );
    }
    m
}

/// VANILLA: AbstractEquineModel#setupAnim при standAnimation = eatAnimation = 0 (без вставания/еды/воды).
pub fn equine_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    let clamped = y_rot.clamp(-20.0, 20.0);
    let mut head_x = x_rot.to_radians();
    if walk_speed > 0.2 {
        head_x += (walk_pos * 0.8).cos() * 0.15 * walk_speed;
    }
    if let Some(i) = m.index("head_parts") {
        m.parts[i].pose.x_rot = PI / 6.0 + head_x;
        m.parts[i].pose.y_rot = clamped.to_radians();
    }
    let leg1 = (walk_pos * 0.6662 + PI).cos();
    set_x(m, "left_hind_leg", -leg1 * 0.5 * walk_speed);
    set_x(m, "right_hind_leg", leg1 * 0.5 * walk_speed);
    set_x(m, "left_front_leg", leg1 * 0.8 * walk_speed);
    set_x(m, "right_front_leg", -leg1 * 0.8 * walk_speed);
    if let Some(i) = m.index("tail") {
        let p = &mut m.parts[i].pose;
        p.x_rot = PI / 6.0 + walk_speed * 0.75;
        p.y += walk_speed;
        p.z += walk_speed * 2.0;
    }
}

/// VANILLA: GoatModel#createBodyLayer (64×64). Оба рога показаны.
// VANILLA-UNSURE: hasLeftHorn/hasRightHorn и таран (rammingXHeadRot) не читаются.
pub fn goat_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::offset(1.0, 14.0, 0.0),
        &[
            BoxDef::new([2.0, 61.0], [-6.0, -11.0, -10.0], [3.0, 2.0, 1.0]),
            BoxDef::new([2.0, 61.0], [2.0, -11.0, -10.0], [3.0, 2.0, 1.0]).mirror(),
            BoxDef::new([23.0, 52.0], [-0.5, -3.0, -14.0], [0.0, 7.0, 5.0]),
        ],
    );
    m.add(
        "left_horn",
        Some(head),
        Pose::default(),
        &[BoxDef::new(
            [12.0, 55.0],
            [-0.01, -16.0, -10.0],
            [2.0, 7.0, 2.0],
        )],
    );
    m.add(
        "right_horn",
        Some(head),
        Pose::default(),
        &[BoxDef::new(
            [12.0, 55.0],
            [-2.99, -16.0, -10.0],
            [2.0, 7.0, 2.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose {
            x_rot: 0.9599,
            ..Pose::offset(0.0, -8.0, -8.0)
        },
        &[BoxDef::new(
            [34.0, 46.0],
            [-3.0, -4.0, -8.0],
            [5.0, 7.0, 10.0],
        )],
    );
    m.add(
        "body",
        None,
        Pose::offset(0.0, 24.0, 0.0),
        &[
            BoxDef::new([1.0, 1.0], [-4.0, -17.0, -7.0], [9.0, 11.0, 16.0]),
            BoxDef::new([0.0, 28.0], [-5.0, -18.0, -8.0], [11.0, 14.0, 11.0]),
        ],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(1.0, 14.0, 4.0),
        &[BoxDef::new([36.0, 29.0], [0.0, 4.0, 0.0], [3.0, 6.0, 3.0])],
    );
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-3.0, 14.0, 4.0),
        &[BoxDef::new([49.0, 29.0], [0.0, 4.0, 0.0], [3.0, 6.0, 3.0])],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(1.0, 14.0, -6.0),
        &[BoxDef::new([49.0, 2.0], [0.0, 0.0, 0.0], [3.0, 10.0, 3.0])],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-3.0, 14.0, -6.0),
        &[BoxDef::new([35.0, 2.0], [0.0, 0.0, 0.0], [3.0, 10.0, 3.0])],
    );
    m
}

/// VANILLA: LlamaModel#createBodyLayer (128×64), сундуки скрыты (hasChest не читается).
pub fn llama_mesh() -> Model {
    let mut m = Model::new([128.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 7.0, -6.0),
        &[
            BoxDef::new([0.0, 0.0], [-2.0, -14.0, -10.0], [4.0, 4.0, 9.0]),
            BoxDef::new([0.0, 14.0], [-4.0, -16.0, -6.0], [8.0, 18.0, 6.0]),
            BoxDef::new([17.0, 0.0], [-4.0, -19.0, -4.0], [3.0, 3.0, 2.0]),
            BoxDef::new([17.0, 0.0], [1.0, -19.0, -4.0], [3.0, 3.0, 2.0]),
        ],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 5.0, 2.0)
        },
        &[BoxDef::new(
            [29.0, 0.0],
            [-6.0, -10.0, -7.0],
            [12.0, 18.0, 10.0],
        )],
    );
    let leg = [BoxDef::new(
        [29.0, 29.0],
        [-2.0, 0.0, -2.0],
        [4.0, 14.0, 4.0],
    )];
    m.add("right_hind_leg", None, Pose::offset(-3.5, 10.0, 6.0), &leg);
    m.add("left_hind_leg", None, Pose::offset(3.5, 10.0, 6.0), &leg);
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-3.5, 10.0, -5.0),
        &leg,
    );
    m.add("left_front_leg", None, Pose::offset(3.5, 10.0, -5.0), &leg);
    m
}

/// VANILLA: SnowGolemModel#createBodyLayer (64×64), все боксы с deformation −0.5.
// VANILLA-UNSURE: тыква на голове (SnowGolemHeadLayer, блок) не рисуется.
pub fn snow_golem_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 4.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]).grow(-0.5)],
    );
    let arm = [BoxDef::new([32.0, 0.0], [-1.0, 0.0, -1.0], [12.0, 2.0, 2.0]).grow(-0.5)];
    m.add(
        "left_arm",
        None,
        Pose {
            z_rot: 1.0,
            ..Pose::offset(5.0, 6.0, 1.0)
        },
        &arm,
    );
    m.add(
        "right_arm",
        None,
        Pose {
            y_rot: PI,
            z_rot: -1.0,
            ..Pose::offset(-5.0, 6.0, -1.0)
        },
        &arm,
    );
    m.add(
        "upper_body",
        None,
        Pose::offset(0.0, 13.0, 0.0),
        &[BoxDef::new([0.0, 16.0], [-5.0, -10.0, -5.0], [10.0, 10.0, 10.0]).grow(-0.5)],
    );
    m.add(
        "lower_body",
        None,
        Pose::offset(0.0, 24.0, 0.0),
        &[BoxDef::new([0.0, 36.0], [-6.0, -12.0, -6.0], [12.0, 12.0, 12.0]).grow(-0.5)],
    );
    m
}

/// VANILLA: SnowGolemModel#setupAnim.
pub fn snow_golem_anim(m: &mut Model, y_rot: f32, x_rot: f32) {
    set_head(m, y_rot, x_rot);
    let yr = y_rot.to_radians() * 0.25;
    let (sin, cos) = (yr.sin(), yr.cos());
    if let Some(i) = m.index("upper_body") {
        m.parts[i].pose.y_rot = yr;
    }
    if let Some(i) = m.index("left_arm") {
        let p = &mut m.parts[i].pose;
        p.y_rot = yr;
        p.x = cos * 5.0;
        p.z = -sin * 5.0;
    }
    if let Some(i) = m.index("right_arm") {
        let p = &mut m.parts[i].pose;
        p.y_rot = yr + PI;
        p.x = -cos * 5.0;
        p.z = sin * 5.0;
    }
}

/// VANILLA: AdultCamelModel#createBodyMesh (128×128).
// VANILLA-UNSURE: keyframe-анимации (CamelAnimation) не портированы — ходьба приближена качанием ног.
pub fn camel_mesh() -> Model {
    let mut m = Model::new([128.0, 128.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 4.0, 9.5),
        &[BoxDef::new(
            [0.0, 25.0],
            [-7.5, -12.0, -23.5],
            [15.0, 12.0, 27.0],
        )],
    );
    m.add(
        "hump",
        Some(body),
        Pose::offset(0.0, -12.0, -10.0),
        &[BoxDef::new(
            [74.0, 0.0],
            [-4.5, -5.0, -5.5],
            [9.0, 5.0, 11.0],
        )],
    );
    m.add(
        "tail",
        Some(body),
        Pose::offset(0.0, -9.0, 3.5),
        &[BoxDef::new(
            [122.0, 0.0],
            [-1.5, 0.0, 0.0],
            [3.0, 14.0, 0.0],
        )],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, -3.0, -19.5),
        &[
            BoxDef::new([60.0, 24.0], [-3.5, -7.0, -15.0], [7.0, 8.0, 19.0]),
            BoxDef::new([21.0, 0.0], [-3.5, -21.0, -15.0], [7.0, 14.0, 7.0]),
            BoxDef::new([50.0, 0.0], [-2.5, -21.0, -21.0], [5.0, 5.0, 6.0]),
        ],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(2.5, -21.0, -9.5),
        &[BoxDef::new([45.0, 0.0], [-0.5, 0.5, -1.0], [3.0, 1.0, 2.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-2.5, -21.0, -9.5),
        &[BoxDef::new([67.0, 0.0], [-2.5, 0.5, -1.0], [3.0, 1.0, 2.0])],
    );
    let leg = |u: f32, v: f32| [BoxDef::new([u, v], [-2.5, 2.0, -2.5], [5.0, 21.0, 5.0])];
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(4.9, 1.0, 9.5),
        &leg(58.0, 16.0),
    );
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-4.9, 1.0, 9.5),
        &leg(94.0, 16.0),
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(4.9, 1.0, -10.5),
        &leg(0.0, 0.0),
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-4.9, 1.0, -10.5),
        &leg(0.0, 26.0),
    );
    m
}

/// VANILLA: CamelModel#applyHeadRotation (+ приближённая ходьба).
pub fn camel_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    if let Some(i) = m.index("head") {
        m.parts[i].pose.y_rot = y_rot.clamp(-30.0, 30.0).to_radians();
        m.parts[i].pose.x_rot = x_rot.clamp(-25.0, 45.0).to_radians();
    }
    let a = (walk_pos * 0.6662).cos() * 0.8 * walk_speed;
    set_x(m, "left_hind_leg", a);
    set_x(m, "right_hind_leg", -a);
    set_x(m, "left_front_leg", -a);
    set_x(m, "right_front_leg", a);
}

fn set_y(m: &mut Model, name: &str, v: f32) {
    if let Some(i) = m.index(name) {
        m.parts[i].pose.y_rot = v;
    }
}

/// VANILLA: CodModel#createBodyLayer (32×32).
pub fn cod_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    m.add(
        "body",
        None,
        Pose::offset(0.0, 22.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-1.0, -2.0, 0.0], [2.0, 4.0, 7.0])],
    );
    m.add(
        "head",
        None,
        Pose::offset(0.0, 22.0, 0.0),
        &[BoxDef::new(
            [11.0, 0.0],
            [-1.0, -2.0, -3.0],
            [2.0, 4.0, 3.0],
        )],
    );
    m.add(
        "nose",
        None,
        Pose::offset(0.0, 22.0, -3.0),
        &[BoxDef::new([0.0, 0.0], [-1.0, -2.0, -1.0], [2.0, 3.0, 1.0])],
    );
    m.add(
        "right_fin",
        None,
        Pose {
            z_rot: -PI / 4.0,
            ..Pose::offset(-1.0, 23.0, 0.0)
        },
        &[BoxDef::new([22.0, 1.0], [-2.0, 0.0, -1.0], [2.0, 0.0, 2.0])],
    );
    m.add(
        "left_fin",
        None,
        Pose {
            z_rot: PI / 4.0,
            ..Pose::offset(1.0, 23.0, 0.0)
        },
        &[BoxDef::new([22.0, 4.0], [0.0, 0.0, -1.0], [2.0, 0.0, 2.0])],
    );
    m.add(
        "tail_fin",
        None,
        Pose::offset(0.0, 22.0, 7.0),
        &[BoxDef::new([22.0, 3.0], [0.0, -2.0, 0.0], [0.0, 4.0, 4.0])],
    );
    m.add(
        "top_fin",
        None,
        Pose::offset(0.0, 20.0, 0.0),
        &[BoxDef::new(
            [20.0, -6.0],
            [0.0, -1.0, -1.0],
            [0.0, 1.0, 6.0],
        )],
    );
    m
}

/// VANILLA: CodModel#setupAnim (в воде; isInWater не отслеживается — см. VANILLA-UNSURE в mobs.rs).
pub fn cod_anim(m: &mut Model, age: f32) {
    set_y(m, "tail_fin", -0.45 * (0.6 * age).sin());
}

/// VANILLA: SalmonModel#createBodyLayer (32×32).
pub fn salmon_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let front = m.add(
        "body_front",
        None,
        Pose::offset(0.0, 20.0, -7.2),
        &[BoxDef::new([0.0, 0.0], [-1.5, -2.5, 0.0], [3.0, 5.0, 8.0])],
    );
    let back = m.add(
        "body_back",
        None,
        Pose::offset(0.0, 20.0, 0.8000002),
        &[BoxDef::new([0.0, 13.0], [-1.5, -2.5, 0.0], [3.0, 5.0, 8.0])],
    );
    m.add(
        "head",
        None,
        Pose::offset(0.0, 20.0, -7.2),
        &[BoxDef::new(
            [22.0, 0.0],
            [-1.0, -2.0, -3.0],
            [2.0, 4.0, 3.0],
        )],
    );
    m.add(
        "back_fin",
        Some(back),
        Pose::offset(0.0, 0.0, 8.0),
        &[BoxDef::new([20.0, 10.0], [0.0, -2.5, 0.0], [0.0, 5.0, 6.0])],
    );
    m.add(
        "top_front_fin",
        Some(front),
        Pose::offset(0.0, -4.5, 5.0),
        &[BoxDef::new([2.0, 1.0], [0.0, 0.0, 0.0], [0.0, 2.0, 3.0])],
    );
    m.add(
        "top_back_fin",
        Some(back),
        Pose::offset(0.0, -4.5, -1.0),
        &[BoxDef::new([0.0, 2.0], [0.0, 0.0, 0.0], [0.0, 2.0, 4.0])],
    );
    m.add(
        "right_fin",
        None,
        Pose {
            z_rot: -PI / 4.0,
            ..Pose::offset(-1.5, 21.5, -7.2)
        },
        &[BoxDef::new([-4.0, 0.0], [-2.0, 0.0, 0.0], [2.0, 0.0, 2.0])],
    );
    m.add(
        "left_fin",
        None,
        Pose {
            z_rot: PI / 4.0,
            ..Pose::offset(1.5, 21.5, -7.2)
        },
        &[BoxDef::new([0.0, 0.0], [0.0, 0.0, 0.0], [2.0, 0.0, 2.0])],
    );
    m
}

/// VANILLA: SalmonModel#setupAnim (в воде).
pub fn salmon_anim(m: &mut Model, age: f32) {
    set_y(m, "body_back", -0.25 * (0.6 * age).sin());
}

/// VANILLA: DolphinModel#createBodyLayer (64×64).
pub fn dolphin_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 22.0, -5.0),
        &[BoxDef::new(
            [22.0, 0.0],
            [-4.0, -7.0, 0.0],
            [8.0, 7.0, 13.0],
        )],
    );
    m.add(
        "back_fin",
        Some(body),
        Pose {
            x_rot: PI / 3.0,
            ..Pose::offset(0.0, 0.0, 0.0)
        },
        &[BoxDef::new([51.0, 0.0], [-0.5, 0.0, 8.0], [1.0, 4.0, 5.0])],
    );
    m.add(
        "left_fin",
        Some(body),
        Pose {
            x_rot: PI / 3.0,
            z_rot: PI * 2.0 / 3.0,
            ..Pose::offset(2.0, -2.0, 4.0)
        },
        &[BoxDef::new([48.0, 20.0], [-0.5, -4.0, 0.0], [1.0, 4.0, 7.0]).mirror()],
    );
    m.add(
        "right_fin",
        Some(body),
        Pose {
            x_rot: PI / 3.0,
            z_rot: -PI * 2.0 / 3.0,
            ..Pose::offset(-2.0, -2.0, 4.0)
        },
        &[BoxDef::new(
            [48.0, 20.0],
            [-0.5, -4.0, 0.0],
            [1.0, 4.0, 7.0],
        )],
    );
    let tail = m.add(
        "tail",
        Some(body),
        Pose {
            x_rot: -0.10471976,
            ..Pose::offset(0.0, -2.5, 11.0)
        },
        &[BoxDef::new(
            [0.0, 19.0],
            [-2.0, -2.5, 0.0],
            [4.0, 5.0, 11.0],
        )],
    );
    m.add(
        "tail_fin",
        Some(tail),
        Pose::offset(0.0, 0.0, 9.0),
        &[BoxDef::new(
            [19.0, 20.0],
            [-5.0, -0.5, 0.0],
            [10.0, 1.0, 6.0],
        )],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, -4.0, -3.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -3.0, -3.0], [8.0, 7.0, 6.0])],
    );
    m.add(
        "nose",
        Some(head),
        Pose::default(),
        &[BoxDef::new([0.0, 13.0], [-1.0, 2.0, -7.0], [2.0, 2.0, 4.0])],
    );
    m
}

/// VANILLA: DolphinModel#setupAnim (state.yRot/xRot — поворот головы относительно тела).
pub fn dolphin_anim(m: &mut Model, y_rot: f32, x_rot: f32, age: f32, moving: bool) {
    if let Some(i) = m.index("body") {
        m.parts[i].pose.x_rot = x_rot.to_radians();
        m.parts[i].pose.y_rot = y_rot.to_radians();
        if moving {
            m.parts[i].pose.x_rot += -0.05 - 0.05 * (age * 0.3).cos();
        }
    }
    if moving {
        set_x(m, "tail", -0.1 * (age * 0.3).cos());
        set_x(m, "tail_fin", -0.2 * (age * 0.3).cos());
    }
}

/// VANILLA: HoglinModel#createBodyLayer (128×64).
pub fn hoglin_mesh() -> Model {
    let mut m = Model::new([128.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 7.0, 0.0),
        &[BoxDef::new(
            [1.0, 1.0],
            [-8.0, -7.0, -13.0],
            [16.0, 14.0, 26.0],
        )],
    );
    m.add(
        "mane",
        Some(body),
        Pose::offset(0.0, -14.0, -7.0),
        &[BoxDef::new([90.0, 33.0], [0.0, 0.0, -9.0], [0.0, 10.0, 19.0]).grow(0.001)],
    );
    let head = m.add(
        "head",
        None,
        Pose {
            x_rot: 0.87266463,
            ..Pose::offset(0.0, 2.0, -12.0)
        },
        &[BoxDef::new(
            [61.0, 1.0],
            [-7.0, -3.0, -19.0],
            [14.0, 6.0, 19.0],
        )],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose {
            z_rot: -PI * 2.0 / 9.0,
            ..Pose::offset(-6.0, -2.0, -3.0)
        },
        &[BoxDef::new([1.0, 1.0], [-6.0, -1.0, -2.0], [6.0, 1.0, 4.0])],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose {
            z_rot: PI * 2.0 / 9.0,
            ..Pose::offset(6.0, -2.0, -3.0)
        },
        &[BoxDef::new([1.0, 6.0], [0.0, -1.0, -2.0], [6.0, 1.0, 4.0])],
    );
    m.add(
        "right_horn",
        Some(head),
        Pose::offset(-7.0, 2.0, -12.0),
        &[BoxDef::new(
            [10.0, 13.0],
            [-1.0, -11.0, -1.0],
            [2.0, 11.0, 2.0],
        )],
    );
    m.add(
        "left_horn",
        Some(head),
        Pose::offset(7.0, 2.0, -12.0),
        &[BoxDef::new(
            [1.0, 13.0],
            [-1.0, -11.0, -1.0],
            [2.0, 11.0, 2.0],
        )],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-4.0, 10.0, -8.5),
        &[BoxDef::new(
            [66.0, 42.0],
            [-3.0, 0.0, -3.0],
            [6.0, 14.0, 6.0],
        )],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(4.0, 10.0, -8.5),
        &[BoxDef::new(
            [41.0, 42.0],
            [-3.0, 0.0, -3.0],
            [6.0, 14.0, 6.0],
        )],
    );
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-5.0, 13.0, 10.0),
        &[BoxDef::new(
            [21.0, 45.0],
            [-2.5, 0.0, -2.5],
            [5.0, 11.0, 5.0],
        )],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(5.0, 13.0, 10.0),
        &[BoxDef::new(
            [0.0, 45.0],
            [-2.5, 0.0, -2.5],
            [5.0, 11.0, 5.0],
        )],
    );
    m
}

/// VANILLA: HoglinModel#setupAnim (без удара головой — attackAnimationRemainingTicks не хранится).
pub fn hoglin_anim(m: &mut Model, y_rot: f32, walk_pos: f32, walk_speed: f32) {
    if let Some(i) = m.index("right_ear") {
        m.parts[i].pose.z_rot = -PI * 2.0 / 9.0 - walk_speed * walk_pos.sin();
    }
    if let Some(i) = m.index("left_ear") {
        m.parts[i].pose.z_rot = PI * 2.0 / 9.0 + walk_speed * walk_pos.sin();
    }
    set_y(m, "head", y_rot.to_radians());
    let a = walk_pos.cos() * 1.2 * walk_speed;
    let b = (walk_pos + PI).cos() * 1.2 * walk_speed;
    set_x(m, "right_front_leg", a);
    set_x(m, "left_front_leg", b);
    set_x(m, "right_hind_leg", b);
    set_x(m, "left_hind_leg", a);
}

const GSP_X_ROT: [f32; 12] = [
    1.75, 0.25, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 1.25, 0.75, 0.0, 0.0,
];
const GSP_Y_ROT: [f32; 12] = [
    0.0, 0.0, 0.0, 0.0, 0.25, 1.75, 1.25, 0.75, 0.0, 0.0, 0.0, 0.0,
];
const GSP_Z_ROT: [f32; 12] = [
    0.0, 0.0, 0.25, 1.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.75, 1.25,
];
const GSP_X: [f32; 12] = [
    0.0, 0.0, 8.0, -8.0, -8.0, 8.0, 8.0, -8.0, 0.0, 0.0, 8.0, -8.0,
];
const GSP_Y: [f32; 12] = [
    -8.0, -8.0, -8.0, -8.0, 0.0, 0.0, 0.0, 0.0, 8.0, 8.0, 8.0, 8.0,
];
const GSP_Z: [f32; 12] = [
    8.0, -8.0, 0.0, 0.0, -8.0, -8.0, 8.0, 8.0, 8.0, -8.0, 0.0, 0.0,
];
const GSP_NAMES: [&str; 12] = [
    "spike0", "spike1", "spike2", "spike3", "spike4", "spike5", "spike6", "spike7", "spike8",
    "spike9", "spike10", "spike11",
];

/// VANILLA: GuardianModel#createBodyLayer (64×64).
pub fn guardian_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let head = m.add(
        "head",
        None,
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-6.0, 10.0, -8.0], [12.0, 12.0, 16.0]),
            BoxDef::new([0.0, 28.0], [-8.0, 10.0, -6.0], [2.0, 12.0, 12.0]),
            BoxDef::new([0.0, 28.0], [6.0, 10.0, -6.0], [2.0, 12.0, 12.0]).mirror(),
            BoxDef::new([16.0, 40.0], [-6.0, 8.0, -6.0], [12.0, 2.0, 12.0]),
            BoxDef::new([16.0, 40.0], [-6.0, 22.0, -6.0], [12.0, 2.0, 12.0]),
        ],
    );
    for i in 0..12 {
        m.add(
            GSP_NAMES[i],
            Some(head),
            Pose {
                x_rot: PI * GSP_X_ROT[i],
                y_rot: PI * GSP_Y_ROT[i],
                z_rot: PI * GSP_Z_ROT[i],
                ..Pose::offset(GSP_X[i], 16.0 + GSP_Y[i], GSP_Z[i])
            },
            &[BoxDef::new([0.0, 0.0], [-1.0, -4.5, -1.0], [2.0, 9.0, 2.0])],
        );
    }
    m.add(
        "eye",
        Some(head),
        Pose::offset(0.0, 0.0, -8.25),
        &[BoxDef::new([8.0, 0.0], [-1.0, 15.0, 0.0], [2.0, 2.0, 1.0])],
    );
    let t0 = m.add(
        "tail0",
        Some(head),
        Pose::default(),
        &[BoxDef::new([40.0, 0.0], [-2.0, 14.0, 7.0], [4.0, 4.0, 8.0])],
    );
    let t1 = m.add(
        "tail1",
        Some(t0),
        Pose::offset(-1.5, 0.5, 14.0),
        &[BoxDef::new([0.0, 54.0], [0.0, 14.0, 0.0], [3.0, 3.0, 7.0])],
    );
    m.add(
        "tail2",
        Some(t1),
        Pose::offset(0.5, 0.5, 6.0),
        &[
            BoxDef::new([41.0, 32.0], [0.0, 14.0, 0.0], [2.0, 2.0, 6.0]),
            BoxDef::new([25.0, 19.0], [1.0, 10.5, 3.0], [1.0, 9.0, 9.0]),
        ],
    );
    m
}

/// VANILLA: GuardianModel#setupAnim; `eye` = (eye.x, eye.y) — взгляд на камеру.
/// `tail` = tailAnimation, `spikes` = spikesAnimation (Guardian#aiStep).
pub fn guardian_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    age: f32,
    tail: f32,
    spikes: f32,
    eye: (f32, f32),
) {
    set_head(m, y_rot, x_rot);
    if let Some(j) = m.index("eye") {
        m.parts[j].pose.x = eye.0;
        m.parts[j].pose.y = eye.1;
    }
    let withdrawal = (1.0 - spikes) * 0.55;
    for i in 0..12 {
        let o = 1.0 + (age * 1.5 + i as f32).cos() * 0.01 - withdrawal;
        if let Some(j) = m.index(GSP_NAMES[i]) {
            m.parts[j].pose.x = GSP_X[i] * o;
            m.parts[j].pose.y = 16.0 + GSP_Y[i] * o;
            m.parts[j].pose.z = GSP_Z[i] * o;
        }
    }
    set_y(m, "tail0", tail.sin() * PI * 0.05);
    set_y(m, "tail1", tail.sin() * PI * 0.1);
    set_y(m, "tail2", tail.sin() * PI * 0.15);
}

fn set_z(m: &mut Model, name: &str, v: f32) {
    if let Some(i) = m.index(name) {
        m.parts[i].pose.z_rot = v;
    }
}

fn add_pos(m: &mut Model, name: &str, dx: f32, dy: f32) {
    if let Some(i) = m.index(name) {
        m.parts[i].pose.x += dx;
        m.parts[i].pose.y += dy;
    }
}

fn set_pos_y(m: &mut Model, name: &str, v: f32) {
    if let Some(i) = m.index(name) {
        m.parts[i].pose.y = v;
    }
}

/// VANILLA: PhantomModel#createBodyLayer (64×64).
pub fn phantom_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose {
            x_rot: -0.1,
            ..Pose::default()
        },
        &[BoxDef::new([0.0, 8.0], [-3.0, -2.0, -8.0], [5.0, 3.0, 9.0])],
    );
    let tb = m.add(
        "tail_base",
        Some(body),
        Pose::offset(0.0, -2.0, 1.0),
        &[BoxDef::new([3.0, 20.0], [-2.0, 0.0, 0.0], [3.0, 2.0, 6.0])],
    );
    m.add(
        "tail_tip",
        Some(tb),
        Pose::offset(0.0, 0.5, 6.0),
        &[BoxDef::new([4.0, 29.0], [-1.0, 0.0, 0.0], [1.0, 1.0, 6.0])],
    );
    let lw = m.add(
        "left_wing_base",
        Some(body),
        Pose {
            z_rot: 0.1,
            ..Pose::offset(2.0, -2.0, -8.0)
        },
        &[BoxDef::new([23.0, 12.0], [0.0, 0.0, 0.0], [6.0, 2.0, 9.0])],
    );
    m.add(
        "left_wing_tip",
        Some(lw),
        Pose {
            z_rot: 0.1,
            ..Pose::offset(6.0, 0.0, 0.0)
        },
        &[BoxDef::new([16.0, 24.0], [0.0, 0.0, 0.0], [13.0, 1.0, 9.0])],
    );
    let rw = m.add(
        "right_wing_base",
        Some(body),
        Pose {
            z_rot: -0.1,
            ..Pose::offset(-3.0, -2.0, -8.0)
        },
        &[BoxDef::new([23.0, 12.0], [-6.0, 0.0, 0.0], [6.0, 2.0, 9.0]).mirror()],
    );
    m.add(
        "right_wing_tip",
        Some(rw),
        Pose {
            z_rot: -0.1,
            ..Pose::offset(-6.0, 0.0, 0.0)
        },
        &[BoxDef::new([16.0, 24.0], [-13.0, 0.0, 0.0], [13.0, 1.0, 9.0]).mirror()],
    );
    m.add(
        "head",
        Some(body),
        Pose {
            x_rot: 0.2,
            ..Pose::offset(0.0, 1.0, -7.0)
        },
        &[BoxDef::new([0.0, 0.0], [-4.0, -2.0, -5.0], [7.0, 3.0, 5.0])],
    );
    m
}

/// VANILLA: PhantomModel#setupAnim; flap_time = id*3 + ageInTicks.
pub fn phantom_anim(m: &mut Model, flap_time: f32) {
    let anim = flap_time * 7.448451 * PI / 180.0;
    let w = anim.cos() * 16.0 * PI / 180.0;
    set_z(m, "left_wing_base", w);
    set_z(m, "left_wing_tip", w);
    set_z(m, "right_wing_base", -w);
    set_z(m, "right_wing_tip", -w);
    let t = -(5.0 + (anim * 2.0).cos() * 5.0) * PI / 180.0;
    set_x(m, "tail_base", t);
    set_x(m, "tail_tip", t);
}

/// VANILLA: RavagerModel#createBodyLayer (128×128).
pub fn ravager_mesh() -> Model {
    let mut m = Model::new([128.0, 128.0]);
    let neck = m.add(
        "neck",
        None,
        Pose::offset(0.0, -7.0, 5.5),
        &[BoxDef::new(
            [68.0, 73.0],
            [-5.0, -1.0, -18.0],
            [10.0, 10.0, 18.0],
        )],
    );
    let head = m.add(
        "head",
        Some(neck),
        Pose::offset(0.0, 16.0, -17.0),
        &[
            BoxDef::new([0.0, 0.0], [-8.0, -20.0, -14.0], [16.0, 20.0, 16.0]),
            BoxDef::new([0.0, 0.0], [-2.0, -6.0, -18.0], [4.0, 8.0, 4.0]),
        ],
    );
    m.add(
        "right_horn",
        Some(head),
        Pose {
            x_rot: 1.0995574,
            ..Pose::offset(-10.0, -14.0, -8.0)
        },
        &[BoxDef::new(
            [74.0, 55.0],
            [0.0, -14.0, -2.0],
            [2.0, 14.0, 4.0],
        )],
    );
    m.add(
        "left_horn",
        Some(head),
        Pose {
            x_rot: 1.0995574,
            ..Pose::offset(8.0, -14.0, -8.0)
        },
        &[BoxDef::new([74.0, 55.0], [0.0, -14.0, -2.0], [2.0, 14.0, 4.0]).mirror()],
    );
    m.add(
        "mouth",
        Some(head),
        Pose::offset(0.0, -2.0, 2.0),
        &[BoxDef::new(
            [0.0, 36.0],
            [-8.0, 0.0, -16.0],
            [16.0, 3.0, 16.0],
        )],
    );
    m.add(
        "body",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, 1.0, 2.0)
        },
        &[
            BoxDef::new([0.0, 55.0], [-7.0, -10.0, -7.0], [14.0, 16.0, 20.0]),
            BoxDef::new([0.0, 91.0], [-6.0, 6.0, -7.0], [12.0, 13.0, 18.0]),
        ],
    );
    let leg = |u: f32, mirror: bool| {
        let b = BoxDef::new([u, 0.0], [-4.0, 0.0, -4.0], [8.0, 37.0, 8.0]);
        if mirror {
            b.mirror()
        } else {
            b
        }
    };
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-8.0, -13.0, 18.0),
        &[leg(96.0, false)],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(8.0, -13.0, 18.0),
        &[leg(96.0, true)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-8.0, -13.0, -5.0),
        &[leg(64.0, false)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(8.0, -13.0, -5.0),
        &[leg(64.0, true)],
    );
    m
}

/// VANILLA: RavagerModel#setupAnim (без атаки/оглушения/рёва — состояния не хранятся).
pub fn ravager_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
    set_x(m, "mouth", PI * 0.01);
    set_head(m, y_rot, x_rot);
    let leg = 0.4 * walk_speed;
    set_x(m, "right_hind_leg", (walk_pos * 0.6662).cos() * leg);
    set_x(m, "left_hind_leg", (walk_pos * 0.6662 + PI).cos() * leg);
    set_x(m, "right_front_leg", (walk_pos * 0.6662 + PI).cos() * leg);
    set_x(m, "left_front_leg", (walk_pos * 0.6662).cos() * leg);
}

/// VANILLA: AdultStriderModel#createBodyLayer (64×128).
pub fn strider_mesh() -> Model {
    let mut m = Model::new([64.0, 128.0]);
    m.add(
        "right_leg",
        None,
        Pose::offset(-4.0, 8.0, 0.0),
        &[BoxDef::new(
            [0.0, 32.0],
            [-2.0, 0.0, -2.0],
            [4.0, 16.0, 4.0],
        )],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(4.0, 8.0, 0.0),
        &[BoxDef::new(
            [0.0, 55.0],
            [-2.0, 0.0, -2.0],
            [4.0, 16.0, 4.0],
        )],
    );
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 1.0, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-8.0, -6.0, -8.0],
            [16.0, 14.0, 16.0],
        )],
    );
    let bristle = |m: &mut Model,
                   name: &'static str,
                   x: f32,
                   y: f32,
                   z_rot: f32,
                   u: f32,
                   v: f32,
                   right: bool| {
        let b = if right {
            BoxDef::new([u, v], [-12.0, 0.0, 0.0], [12.0, 0.0, 16.0]).mirror()
        } else {
            BoxDef::new([u, v], [0.0, 0.0, 0.0], [12.0, 0.0, 16.0])
        };
        m.add(
            name,
            Some(body),
            Pose {
                z_rot,
                ..Pose::offset(x, y, -8.0)
            },
            &[b],
        );
    };
    bristle(
        &mut m,
        "right_bottom_bristle",
        -8.0,
        4.0,
        -1.2217305,
        16.0,
        65.0,
        true,
    );
    bristle(
        &mut m,
        "right_middle_bristle",
        -8.0,
        -1.0,
        -1.134464,
        16.0,
        49.0,
        true,
    );
    bristle(
        &mut m,
        "right_top_bristle",
        -8.0,
        -5.0,
        -0.87266463,
        16.0,
        33.0,
        true,
    );
    bristle(
        &mut m,
        "left_top_bristle",
        8.0,
        -6.0,
        0.87266463,
        16.0,
        33.0,
        false,
    );
    bristle(
        &mut m,
        "left_middle_bristle",
        8.0,
        -2.0,
        1.134464,
        16.0,
        49.0,
        false,
    );
    bristle(
        &mut m,
        "left_bottom_bristle",
        8.0,
        3.0,
        1.2217305,
        16.0,
        65.0,
        false,
    );
    m
}

/// VANILLA: StriderModel#setupAnim + AdultStriderModel#customAnimations.
pub fn strider_anim(m: &mut Model, y_rot: f32, x_rot: f32, pos: f32, speed: f32, age: f32) {
    let speed = speed.min(0.25);
    if let Some(i) = m.index("body") {
        let p = &mut m.parts[i].pose;
        p.x_rot = x_rot.to_radians();
        p.y_rot = y_rot.to_radians();
        p.z_rot = 0.1 * (pos * 1.5).sin() * 4.0 * speed;
        p.y = 2.0 - 2.0 * (pos * 1.5).cos() * 2.0 * speed;
    }
    for (name, off, sgn) in [("left_leg", 0.0, 1.0f32), ("right_leg", PI, 1.0)] {
        if let Some(i) = m.index(name) {
            let p = &mut m.parts[i].pose;
            p.x_rot = (pos * 0.75 + off).sin() * 2.0 * speed;
            p.z_rot = (PI / 18.0) * (pos * 0.75 + off).cos() * speed * sgn;
        }
    }
    set_pos_y(
        m,
        "left_leg",
        8.0 + 2.0 * (pos * 0.75 + PI).sin() * 2.0 * speed,
    );
    set_pos_y(m, "right_leg", 8.0 + 2.0 * (pos * 0.75).sin() * 2.0 * speed);
    let flow = (pos * 1.5 + PI).cos() * speed;
    let base = [
        (
            "right_bottom_bristle",
            -1.2217305,
            flow * 1.3,
            0.05 * (age * -0.4).sin(),
        ),
        (
            "right_middle_bristle",
            -1.134464,
            flow * 1.2,
            0.1 * (age * 0.2).sin(),
        ),
        (
            "right_top_bristle",
            -0.87266463,
            flow * 0.6,
            0.1 * (age * 0.4).sin(),
        ),
        (
            "left_top_bristle",
            0.87266463,
            flow * 0.6,
            0.1 * (age * 0.4).sin(),
        ),
        (
            "left_middle_bristle",
            1.134464,
            flow * 1.2,
            0.1 * (age * 0.2).sin(),
        ),
        (
            "left_bottom_bristle",
            1.2217305,
            flow * 1.3,
            0.05 * (age * -0.4).sin(),
        ),
    ];
    for (n, b, a, c) in base {
        set_z(m, n, b + a + c);
    }
}

/// VANILLA: VexModel#createBodyLayer (32×32); part "root" (0,−2.5,0) свёрнут в смещения детей.
pub fn vex_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 17.5, 0.0),
        &[BoxDef::new([0.0, 0.0], [-2.5, -5.0, -2.5], [5.0, 5.0, 5.0])],
    );
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 17.5, 0.0),
        &[
            BoxDef::new([0.0, 10.0], [-1.5, 0.0, -1.0], [3.0, 4.0, 2.0]),
            BoxDef::new([0.0, 16.0], [-1.5, 1.0, -1.0], [3.0, 5.0, 2.0]).grow(-0.2),
        ],
    );
    m.add(
        "right_arm",
        Some(body),
        Pose::offset(-1.75, 0.25, 0.0),
        &[BoxDef::new([23.0, 0.0], [-1.25, -0.5, -1.0], [2.0, 4.0, 2.0]).grow(-0.1)],
    );
    m.add(
        "left_arm",
        Some(body),
        Pose::offset(1.75, 0.25, 0.0),
        &[BoxDef::new([23.0, 6.0], [-0.75, -0.5, -1.0], [2.0, 4.0, 2.0]).grow(-0.1)],
    );
    m.add(
        "left_wing",
        Some(body),
        Pose::offset(0.5, 1.0, 1.0),
        &[BoxDef::new([16.0, 14.0], [0.0, 0.0, 0.0], [0.0, 5.0, 8.0]).mirror()],
    );
    m.add(
        "right_wing",
        Some(body),
        Pose::offset(-0.5, 1.0, 1.0),
        &[BoxDef::new([16.0, 14.0], [0.0, 0.0, 0.0], [0.0, 5.0, 8.0])],
    );
    m
}

/// VANILLA: VexModel#setupAnim (без заряда/предметов).
pub fn vex_anim(m: &mut Model, y_rot: f32, x_rot: f32, age: f32) {
    set_head(m, y_rot, x_rot);
    let bob = (age * 5.5 * PI / 180.0).cos() * 0.1;
    set_z(m, "right_arm", PI / 5.0 + bob);
    set_z(m, "left_arm", -(PI / 5.0 + bob));
    set_x(m, "body", PI / 20.0);
    let lw = 1.0995574 + (age * 45.836624 * PI / 180.0).cos() * (PI / 180.0) * 16.2;
    set_y(m, "left_wing", lw);
    set_y(m, "right_wing", -lw);
    set_x(m, "left_wing", 0.47123888);
    set_z(m, "left_wing", -0.47123888);
    set_x(m, "right_wing", 0.47123888);
    set_z(m, "right_wing", 0.47123888);
}

/// VANILLA: AdultBeeModel#createBodyLayer (64×64); "bone" (0,19,0) — родитель.
pub fn bee_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let bone = m.add("bone", None, Pose::offset(0.0, 19.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(bone),
        Pose::default(),
        &[BoxDef::new(
            [0.0, 0.0],
            [-3.5, -4.0, -5.0],
            [7.0, 7.0, 10.0],
        )],
    );
    m.add(
        "stinger",
        Some(body),
        Pose::default(),
        &[BoxDef::new([26.0, 7.0], [0.0, -1.0, 5.0], [0.0, 1.0, 2.0])],
    );
    m.add(
        "left_antenna",
        Some(body),
        Pose::offset(0.0, -2.0, -5.0),
        &[BoxDef::new([2.0, 0.0], [1.5, -2.0, -3.0], [1.0, 2.0, 3.0])],
    );
    m.add(
        "right_antenna",
        Some(body),
        Pose::offset(0.0, -2.0, -5.0),
        &[BoxDef::new([2.0, 3.0], [-2.5, -2.0, -3.0], [1.0, 2.0, 3.0])],
    );
    m.add(
        "right_wing",
        Some(bone),
        Pose {
            y_rot: -0.2618,
            ..Pose::offset(-1.5, -4.0, -3.0)
        },
        &[BoxDef::new([0.0, 18.0], [-9.0, 0.0, 0.0], [9.0, 0.0, 6.0]).grow(0.001)],
    );
    m.add(
        "left_wing",
        Some(bone),
        Pose {
            y_rot: 0.2618,
            ..Pose::offset(1.5, -4.0, -3.0)
        },
        &[BoxDef::new([0.0, 18.0], [0.0, 0.0, 0.0], [9.0, 0.0, 6.0])
            .grow(0.001)
            .mirror()],
    );
    m.add(
        "front_legs",
        Some(bone),
        Pose::offset(1.5, 3.0, -2.0),
        &[BoxDef::new([26.0, 1.0], [-5.0, 0.0, 0.0], [7.0, 2.0, 0.0])],
    );
    m.add(
        "middle_legs",
        Some(bone),
        Pose::offset(1.5, 3.0, 0.0),
        &[BoxDef::new([26.0, 3.0], [-5.0, 0.0, 0.0], [7.0, 2.0, 0.0])],
    );
    m.add(
        "back_legs",
        Some(bone),
        Pose::offset(1.5, 3.0, 2.0),
        &[BoxDef::new([26.0, 5.0], [-5.0, 0.0, 0.0], [7.0, 2.0, 0.0])],
    );
    m
}

/// VANILLA: BeeModel#setupAnim + AdultBeeModel#bobUpAndDown (без злости/переворота; жало видно).
pub fn bee_anim(m: &mut Model, age: f32, on_ground: bool) {
    if on_ground {
        return;
    }
    let sp = age * 120.32113 * PI / 180.0;
    set_y(m, "right_wing", 0.0);
    set_z(m, "right_wing", sp.cos() * PI * 0.15);
    set_y(m, "left_wing", 0.0);
    set_z(m, "left_wing", -(sp.cos() * PI * 0.15));
    set_x(m, "front_legs", PI / 4.0);
    set_x(m, "middle_legs", PI / 4.0);
    set_x(m, "back_legs", PI / 4.0);
    let s = (age * 0.18).cos();
    set_x(m, "bone", 0.1 + s * PI * 0.025);
    if let Some(i) = m.index("bone") {
        m.parts[i].pose.y = 19.0 - (age * 0.18).cos() * 0.9;
    }
    set_x(m, "front_legs", -s * PI * 0.1 + PI / 8.0);
    set_x(m, "back_legs", -s * PI * 0.05 + PI / 4.0);
    set_x(m, "left_antenna", s * PI * 0.03);
    set_x(m, "right_antenna", s * PI * 0.03);
}

/// VANILLA: ParrotModel#createBodyLayer (32×32).
pub fn parrot_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    m.add(
        "body",
        None,
        Pose {
            x_rot: 0.4937,
            ..Pose::offset(0.0, 16.5, -3.0)
        },
        &[BoxDef::new([2.0, 8.0], [-1.5, 0.0, -1.5], [3.0, 6.0, 3.0])],
    );
    m.add(
        "tail",
        None,
        Pose {
            x_rot: 1.015,
            ..Pose::offset(0.0, 21.07, 1.16)
        },
        &[BoxDef::new(
            [22.0, 1.0],
            [-1.5, -1.0, -1.0],
            [3.0, 4.0, 1.0],
        )],
    );
    m.add(
        "left_wing",
        None,
        Pose {
            x_rot: -0.6981,
            y_rot: -PI,
            ..Pose::offset(1.5, 16.94, -2.76)
        },
        &[BoxDef::new([19.0, 8.0], [-0.5, 0.0, -1.5], [1.0, 5.0, 3.0])],
    );
    m.add(
        "right_wing",
        None,
        Pose {
            x_rot: -0.6981,
            y_rot: -PI,
            ..Pose::offset(-1.5, 16.94, -2.76)
        },
        &[BoxDef::new([19.0, 8.0], [-0.5, 0.0, -1.5], [1.0, 5.0, 3.0])],
    );
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 15.69, -2.76),
        &[BoxDef::new([2.0, 2.0], [-1.0, -1.5, -1.0], [2.0, 3.0, 2.0])],
    );
    m.add(
        "head2",
        Some(head),
        Pose::offset(0.0, -2.0, -1.0),
        &[BoxDef::new(
            [10.0, 0.0],
            [-1.0, -0.5, -2.0],
            [2.0, 1.0, 4.0],
        )],
    );
    m.add(
        "beak1",
        Some(head),
        Pose::offset(0.0, -0.5, -1.5),
        &[BoxDef::new(
            [11.0, 7.0],
            [-0.5, -1.0, -0.5],
            [1.0, 2.0, 1.0],
        )],
    );
    m.add(
        "beak2",
        Some(head),
        Pose::offset(0.0, -1.75, -2.45),
        &[BoxDef::new([16.0, 7.0], [-0.5, 0.0, -0.5], [1.0, 2.0, 1.0])],
    );
    m.add(
        "feather",
        Some(head),
        Pose {
            x_rot: -0.2214,
            ..Pose::offset(0.0, -2.15, 0.15)
        },
        &[BoxDef::new([2.0, 18.0], [0.0, -4.0, -2.0], [0.0, 5.0, 4.0])],
    );
    m.add(
        "left_leg",
        None,
        Pose {
            x_rot: -0.0299,
            ..Pose::offset(1.0, 22.0, -1.05)
        },
        &[BoxDef::new(
            [14.0, 18.0],
            [-0.5, 0.0, -0.5],
            [1.0, 2.0, 1.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose {
            x_rot: -0.0299,
            ..Pose::offset(-1.0, 22.0, -1.05)
        },
        &[BoxDef::new(
            [14.0, 18.0],
            [-0.5, 0.0, -0.5],
            [1.0, 2.0, 1.0],
        )],
    );
    m
}

/// VANILLA: ParrotModel#setupAnim (STANDING на земле, FLYING в воздухе).
// VANILLA-UNSURE: flapAngle в полёте — синус по возрасту (Parrot#aiStep flap не портирован); вечеринка/плечо не различаются (SITTING есть).
#[allow(clippy::too_many_arguments)]
pub fn parrot_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    pos: f32,
    speed: f32,
    flap_phase: f32,
    flap_speed: f32,
    flying: bool,
    sitting: bool,
    party: bool,
    age: f32,
) {
    set_head(m, y_rot, x_rot);
    if party {
        // VANILLA: ParrotModel#setupAnim, Pose.PARTY (приоритет выше SITTING в ParrotRenderer).
        let flap = (flap_phase.sin() + 1.0) * flap_speed;
        let (xp, yp) = (age.cos(), age.sin());
        add_pos(m, "head", xp, yp);
        set_x(m, "head", 0.0);
        set_y(m, "head", 0.0);
        set_z(m, "head", age.sin() * 0.4);
        add_pos(m, "body", xp, yp);
        set_z(m, "left_wing", -0.0873 - flap);
        add_pos(m, "left_wing", xp, yp);
        set_z(m, "right_wing", 0.0873 + flap);
        add_pos(m, "right_wing", xp, yp);
        add_pos(m, "tail", xp, yp);
        return;
    }
    if sitting {
        // VANILLA: ParrotModel#prepare/setupAnim, Pose.SITTING (ветка без взмахов).
        let o = 1.9;
        set_x(m, "tail", 1.015 + PI / 6.0);
        set_pos_y(m, "head", 15.69 + o);
        set_pos_y(m, "tail", 21.07 + o);
        set_pos_y(m, "body", 16.5 + o);
        set_pos_y(m, "left_wing", 16.94 + o);
        set_pos_y(m, "right_wing", 16.94 + o);
        set_pos_y(m, "left_leg", 22.0 + o);
        set_pos_y(m, "right_leg", 22.0 + o);
        set_x(m, "left_leg", -0.0299 + PI / 2.0);
        set_x(m, "right_leg", -0.0299 + PI / 2.0);
        set_z(m, "left_wing", -0.0873);
        set_z(m, "right_wing", 0.0873);
        return;
    }
    // VANILLA: ParrotRenderer — flapAngle = (sin(flap) + 1) · flapSpeed (Parrot#calculateFlapping).
    let flap = (flap_phase.sin() + 1.0) * flap_speed;
    if flying {
        set_x(m, "left_leg", -0.0299 + PI * 2.0 / 9.0);
        set_x(m, "right_leg", -0.0299 + PI * 2.0 / 9.0);
    } else {
        set_x(m, "left_leg", -0.0299 + (pos * 0.6662).cos() * 1.4 * speed);
        set_x(
            m,
            "right_leg",
            -0.0299 + (pos * 0.6662 + PI).cos() * 1.4 * speed,
        );
    }
    let bob = flap * 0.3;
    set_x(m, "tail", 1.015 + (pos * 0.6662).cos() * 0.3 * speed);
    set_pos_y(m, "head", 15.69 + bob);
    set_pos_y(m, "tail", 21.07 + bob);
    set_pos_y(m, "body", 16.5 + bob);
    set_pos_y(m, "left_wing", 16.94 + bob);
    set_pos_y(m, "right_wing", 16.94 + bob);
    set_pos_y(m, "left_leg", 22.0 + bob);
    set_pos_y(m, "right_leg", 22.0 + bob);
    set_z(m, "left_wing", -0.0873 - flap);
    set_z(m, "right_wing", 0.0873 + flap);
}

/// VANILLA: BatModel#createBodyLayer (32×32).
pub fn bat_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 17.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-1.5, 0.0, -1.0], [3.0, 5.0, 2.0])],
    );
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 17.0, 0.0),
        &[BoxDef::new([0.0, 7.0], [-2.0, -3.0, -1.0], [4.0, 3.0, 2.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-1.5, -2.0, 0.0),
        &[BoxDef::new([1.0, 15.0], [-2.5, -4.0, 0.0], [3.0, 5.0, 0.0])],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(1.1, -3.0, 0.0),
        &[BoxDef::new([8.0, 15.0], [-0.1, -3.0, 0.0], [3.0, 5.0, 0.0])],
    );
    let rw = m.add(
        "right_wing",
        Some(body),
        Pose::offset(-1.5, 0.0, 0.0),
        &[BoxDef::new([12.0, 0.0], [-2.0, -2.0, 0.0], [2.0, 7.0, 0.0])],
    );
    m.add(
        "right_wing_tip",
        Some(rw),
        Pose::offset(-2.0, 0.0, 0.0),
        &[BoxDef::new([16.0, 0.0], [-6.0, -2.0, 0.0], [6.0, 8.0, 0.0])],
    );
    let lw = m.add(
        "left_wing",
        Some(body),
        Pose::offset(1.5, 0.0, 0.0),
        &[BoxDef::new([12.0, 7.0], [0.0, -2.0, 0.0], [2.0, 7.0, 0.0])],
    );
    m.add(
        "left_wing_tip",
        Some(lw),
        Pose::offset(2.0, 0.0, 0.0),
        &[BoxDef::new([16.0, 8.0], [0.0, -2.0, 0.0], [6.0, 8.0, 0.0])],
    );
    m.add(
        "feet",
        Some(body),
        Pose::offset(0.0, 5.0, 0.0),
        &[BoxDef::new([16.0, 16.0], [-1.5, 0.0, 0.0], [3.0, 2.0, 0.0])],
    );
    m
}

/// VANILLA: BatModel#setupAnim — висит: голова по yRot + BAT_RESTING, иначе BAT_FLYING.
pub fn bat_anim(m: &mut Model, y_rot: f32, age: f32, resting: bool) {
    let t = (age * 50.0) as i64;
    if resting {
        if let Some(i) = m.index("head") {
            m.parts[i].pose.y_rot = y_rot.to_radians();
        }
        crate::keyframes_data::BAT_RESTING.apply(m, t, 1.0);
    } else {
        crate::keyframes_data::BAT_FLYING.apply(m, t, 1.0);
    }
}

/// VANILLA: FrogModel#createBodyLayer (48×48).
pub fn frog_mesh() -> Model {
    let mut m = Model::new([48.0, 48.0]);
    let root = m.add("root", None, Pose::offset(0.0, 24.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(root),
        Pose::offset(0.0, -2.0, 4.0),
        &[
            BoxDef::new([3.0, 1.0], [-3.5, -2.0, -8.0], [7.0, 3.0, 9.0]),
            BoxDef::new([23.0, 22.0], [-3.5, -1.0, -8.0], [7.0, 0.0, 9.0]),
        ],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, -2.0, -1.0),
        &[
            BoxDef::new([23.0, 13.0], [-3.5, -1.0, -7.0], [7.0, 0.0, 9.0]),
            BoxDef::new([0.0, 13.0], [-3.5, -2.0, -7.0], [7.0, 3.0, 9.0]),
        ],
    );
    let eyes = m.add("eyes", Some(head), Pose::offset(-0.5, 0.0, 2.0), &[]);
    m.add(
        "right_eye",
        Some(eyes),
        Pose::offset(-1.5, -3.0, -6.5),
        &[BoxDef::new([0.0, 0.0], [-1.5, -1.0, -1.5], [3.0, 2.0, 3.0])],
    );
    m.add(
        "left_eye",
        Some(eyes),
        Pose::offset(2.5, -3.0, -6.5),
        &[BoxDef::new([0.0, 5.0], [-1.5, -1.0, -1.5], [3.0, 2.0, 3.0])],
    );
    let cb = m.add(
        "croaking_body",
        Some(body),
        Pose::offset(0.0, -1.0, -5.0),
        &[BoxDef::new([26.0, 5.0], [-3.5, -0.1, -2.9], [7.0, 2.0, 3.0]).grow(-0.1)],
    );
    m.parts[cb].visible = false;
    m.add(
        "tongue",
        Some(body),
        Pose::offset(0.0, -1.01, 1.0),
        &[BoxDef::new(
            [17.0, 13.0],
            [-2.0, 0.0, -7.1],
            [4.0, 0.0, 7.0],
        )],
    );
    let la = m.add(
        "left_arm",
        Some(body),
        Pose::offset(4.0, -1.0, -6.5),
        &[BoxDef::new([0.0, 32.0], [-1.0, 0.0, -1.0], [2.0, 3.0, 3.0])],
    );
    m.add(
        "left_hand",
        Some(la),
        Pose::offset(0.0, 3.0, -1.0),
        &[BoxDef::new(
            [18.0, 40.0],
            [-4.0, 0.01, -4.0],
            [8.0, 0.0, 8.0],
        )],
    );
    let ra = m.add(
        "right_arm",
        Some(body),
        Pose::offset(-4.0, -1.0, -6.5),
        &[BoxDef::new([0.0, 38.0], [-1.0, 0.0, -1.0], [2.0, 3.0, 3.0])],
    );
    m.add(
        "right_hand",
        Some(ra),
        Pose::offset(0.0, 3.0, 0.0),
        &[BoxDef::new(
            [2.0, 40.0],
            [-4.0, 0.01, -5.0],
            [8.0, 0.0, 8.0],
        )],
    );
    let ll = m.add(
        "left_leg",
        Some(root),
        Pose::offset(3.5, -3.0, 4.0),
        &[BoxDef::new(
            [14.0, 25.0],
            [-1.0, 0.0, -2.0],
            [3.0, 3.0, 4.0],
        )],
    );
    m.add(
        "left_foot",
        Some(ll),
        Pose::offset(2.0, 3.0, 0.0),
        &[BoxDef::new(
            [2.0, 32.0],
            [-4.0, 0.01, -4.0],
            [8.0, 0.0, 8.0],
        )],
    );
    let rl = m.add(
        "right_leg",
        Some(root),
        Pose::offset(-3.5, -3.0, 4.0),
        &[BoxDef::new([0.0, 25.0], [-2.0, 0.0, -2.0], [3.0, 3.0, 4.0])],
    );
    m.add(
        "right_foot",
        Some(rl),
        Pose::offset(-2.0, 3.0, 0.0),
        &[BoxDef::new(
            [18.0, 32.0],
            [-4.0, 0.01, -4.0],
            [8.0, 0.0, 8.0],
        )],
    );
    m
}

/// VANILLA: FrogModel#setupAnim (walk / swim; прыжок, кваканье, язык — по метаданным, пока нет).
pub fn frog_anim(m: &mut Model, pos: f32, speed: f32, swimming: bool) {
    use crate::keyframes_data::{FROG_SWIM, FROG_WALK};
    if swimming {
        FROG_SWIM.apply_walk(m, pos, speed, 1.0, 2.5);
    } else {
        FROG_WALK.apply_walk(m, pos, speed, 1.5, 2.5);
    }
}

/// VANILLA: NautilusModel#createBodyLayer (128×128) / #createBabyBodyLayer (64×64).
pub fn nautilus_mesh(baby: bool) -> Model {
    let mut m = Model::new(if baby { [64.0, 64.0] } else { [128.0, 128.0] });
    if baby {
        let root = m.add("root", None, Pose::offset(-0.5, 28.0, -0.5), &[]);
        m.add(
            "shell",
            Some(root),
            Pose::offset(3.0, -8.0, -2.0),
            &[
                BoxDef::new([0.0, 0.0], [-6.0, -4.0, -1.0], [7.0, 4.0, 7.0]),
                BoxDef::new([0.0, 11.0], [-6.0, 0.0, -1.0], [7.0, 4.0, 9.0]),
                BoxDef::new([23.0, 11.0], [-6.0, 0.0, 5.0], [7.0, 4.0, 0.0]),
            ],
        );
        let body = m.add(
            "body",
            Some(root),
            Pose::offset(0.5, -5.0, 3.0),
            &[
                BoxDef::new([0.0, 24.0], [-2.5, -3.01, -1.0], [5.0, 4.0, 7.0]),
                BoxDef::new([0.0, 35.0], [-2.5, -3.01, 4.1], [5.0, 4.0, 0.0]),
            ],
        );
        m.add(
            "upper_mouth",
            Some(body),
            Pose::offset(0.0, -2.01, 3.9),
            &[BoxDef::new([24.0, 24.0], [-2.5, -1.0, 0.0], [5.0, 2.0, 2.0]).grow(-0.001)],
        );
        m.add(
            "inner_mouth",
            Some(body),
            Pose::offset(0.0, -1.01, 4.9),
            &[BoxDef::new(
                [24.0, 32.0],
                [-1.5, -1.0, -1.0],
                [3.0, 2.0, 2.0],
            )],
        );
        m.add(
            "lower_mouth",
            Some(body),
            Pose::offset(0.0, -0.01, 3.9),
            &[BoxDef::new([24.0, 28.0], [-2.5, -1.0, 0.0], [5.0, 2.0, 2.0]).grow(-0.001)],
        );
        return m;
    }
    let root = m.add("root", None, Pose::offset(0.0, 29.0, -6.0), &[]);
    m.add(
        "shell",
        Some(root),
        Pose::offset(0.0, -13.0, 5.0),
        &[
            BoxDef::new([0.0, 0.0], [-7.0, -10.0, -7.0], [14.0, 10.0, 16.0]),
            BoxDef::new([0.0, 26.0], [-7.0, 0.0, -7.0], [14.0, 8.0, 20.0]),
            BoxDef::new([48.0, 26.0], [-7.0, 0.0, 6.0], [14.0, 8.0, 0.0]),
        ],
    );
    let body = m.add(
        "body",
        Some(root),
        Pose::offset(0.0, -8.5, 12.3),
        &[
            BoxDef::new([0.0, 54.0], [-5.0, -4.51, -3.0], [10.0, 8.0, 14.0]),
            BoxDef::new([0.0, 76.0], [-5.0, -4.51, 7.0], [10.0, 8.0, 0.0]),
        ],
    );
    m.add(
        "upper_mouth",
        Some(body),
        Pose::offset(0.0, -2.51, 7.0),
        &[BoxDef::new([54.0, 54.0], [-5.0, -2.0, 0.0], [10.0, 4.0, 4.0]).grow(-0.001)],
    );
    m.add(
        "inner_mouth",
        Some(body),
        Pose::offset(0.0, -0.51, 7.5),
        &[BoxDef::new(
            [54.0, 70.0],
            [-3.0, -2.0, -0.5],
            [6.0, 4.0, 4.0],
        )],
    );
    m.add(
        "lower_mouth",
        Some(body),
        Pose::offset(0.0, 1.49, 7.0),
        &[BoxDef::new([54.0, 62.0], [-5.0, -1.98, 0.0], [10.0, 4.0, 4.0]).grow(-0.001)],
    );
    m
}

/// VANILLA: NautilusModel#setupAnim — поворот тела (±10°) + SWIMMING.applyWalk(pos + age/5, speed + 0.2, 2, 3).
pub fn nautilus_anim(m: &mut Model, yr: f32, xr: f32, wp: f32, ws: f32, age: f32) {
    if let Some(i) = m.index("body") {
        m.parts[i].pose.y_rot = yr.clamp(-10.0, 10.0).to_radians();
        m.parts[i].pose.x_rot = xr.clamp(-10.0, 10.0).to_radians();
    }
    crate::keyframes_data::SWIMMING.apply_walk(m, wp + age / 5.0, ws + 0.2, 2.0, 3.0);
}

/// VANILLA: TadpoleModel#createBodyLayer (16×16).
pub fn tadpole_mesh() -> Model {
    let mut m = Model::new([16.0, 16.0]);
    m.add(
        "body",
        None,
        Pose::offset(0.0, 22.0, -3.0),
        &[BoxDef::new([0.0, 0.0], [-1.5, -1.0, 0.0], [3.0, 2.0, 3.0])],
    );
    m.add(
        "tail",
        None,
        Pose::offset(0.0, 22.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 2.0, 7.0])],
    );
    m
}

/// VANILLA: TadpoleModel#setupAnim (isInWater не отслеживается — считаем в воде).
pub fn tadpole_anim(m: &mut Model, age: f32) {
    set_y(m, "tail", -0.25 * (0.3 * age).sin());
}

/// VANILLA: BoatModel#createBoatModel / createChestBoatModel, RaftModel#createRaftModel /
/// createChestRaftModel (128×64, с сундуком 128×128). Модель в пространстве рендерера лодки.
pub fn boat_mesh(raft: bool, chest: bool) -> Model {
    let mut m = Model::new([128.0, if chest { 128.0 } else { 64.0 }]);
    let paddle = |tex: [f32; 2], blade_x: f32| {
        [
            BoxDef::new(tex, [-1.0, 0.0, -5.0], [2.0, 2.0, 18.0]),
            BoxDef::new(tex, [blade_x, -3.0, 8.0], [1.0, 6.0, 7.0]),
        ]
    };
    let paddle_y = if raft { -4.0 } else { -5.0 };
    if raft {
        m.add(
            "bottom",
            None,
            Pose {
                x_rot: 1.5708,
                ..Pose::offset(0.0, -2.1, 1.0)
            },
            &[
                BoxDef::new([0.0, 0.0], [-14.0, -11.0, -4.0], [28.0, 20.0, 4.0]),
                BoxDef::new([0.0, 0.0], [-14.0, -9.0, -8.0], [28.0, 16.0, 4.0]),
            ],
        );
    } else {
        let sides = |name, tex, org: [f32; 3], dim: [f32; 3], pose: Pose, m: &mut Model| {
            m.add(name, None, pose, &[BoxDef::new(tex, org, dim)]);
        };
        sides(
            "bottom",
            [0.0, 0.0],
            [-14.0, -9.0, -3.0],
            [28.0, 16.0, 3.0],
            Pose {
                x_rot: HALF_PI,
                ..Pose::offset(0.0, 3.0, 1.0)
            },
            &mut m,
        );
        sides(
            "back",
            [0.0, 19.0],
            [-13.0, -7.0, -1.0],
            [18.0, 6.0, 2.0],
            Pose {
                y_rot: PI * 3.0 / 2.0,
                ..Pose::offset(-15.0, 4.0, 4.0)
            },
            &mut m,
        );
        sides(
            "front",
            [0.0, 27.0],
            [-8.0, -7.0, -1.0],
            [16.0, 6.0, 2.0],
            Pose {
                y_rot: HALF_PI,
                ..Pose::offset(15.0, 4.0, 0.0)
            },
            &mut m,
        );
        sides(
            "right",
            [0.0, 35.0],
            [-14.0, -7.0, -1.0],
            [28.0, 6.0, 2.0],
            Pose {
                y_rot: PI,
                ..Pose::offset(0.0, 4.0, -9.0)
            },
            &mut m,
        );
        sides(
            "left",
            [0.0, 43.0],
            [-14.0, -7.0, -1.0],
            [28.0, 6.0, 2.0],
            Pose::offset(0.0, 4.0, 9.0),
            &mut m,
        );
    }
    let (lt, rt) = if raft {
        ([0.0, 24.0], [40.0, 24.0])
    } else {
        ([62.0, 0.0], [62.0, 20.0])
    };
    m.add(
        "left_paddle",
        None,
        Pose {
            z_rot: PI / 16.0,
            ..Pose::offset(3.0, paddle_y, 9.0)
        },
        &paddle(lt, -1.001),
    );
    m.add(
        "right_paddle",
        None,
        Pose {
            y_rot: PI,
            z_rot: PI / 16.0,
            ..Pose::offset(3.0, paddle_y, -9.0)
        },
        &paddle(rt, 0.001),
    );
    if chest {
        let dy = if raft { -5.1 } else { 0.0 };
        let c = |name, tex: [f32; 2], dim: [f32; 3], org: [f32; 3], m: &mut Model| {
            m.add(
                name,
                None,
                Pose {
                    y_rot: -HALF_PI,
                    ..Pose::offset(org[0], org[1] + dy, org[2])
                },
                &[BoxDef::new(tex, [0.0; 3], dim)],
            );
        };
        c(
            "chest_bottom",
            [0.0, 76.0],
            [12.0, 8.0, 12.0],
            [-2.0, -5.0, -6.0],
            &mut m,
        );
        c(
            "chest_lid",
            [0.0, 59.0],
            [12.0, 4.0, 12.0],
            [-2.0, -9.0, -6.0],
            &mut m,
        );
        c(
            "chest_lock",
            [0.0, 59.0],
            [2.0, 4.0, 1.0],
            [-1.0, -6.0, -1.0],
            &mut m,
        );
    }
    m
}

/// VANILLA: AbstractBoatModel#setupAnim/animatePaddle (время гребли левым/правым веслом).
pub fn boat_anim(m: &mut Model, left: f32, right: f32) {
    let clamped_lerp = |t: f32, a: f32, b: f32| a + (b - a) * t.clamp(0.0, 1.0);
    for (side, time) in [(0usize, left), (1, right)] {
        let name = if side == 0 {
            "left_paddle"
        } else {
            "right_paddle"
        };
        let Some(i) = m.index(name) else { continue };
        let x = clamped_lerp((-time).sin() * 0.5 + 0.5, -PI / 3.0, -PI / 12.0);
        let mut y = clamped_lerp((-time + 1.0).sin() * 0.5 + 0.5, -PI / 4.0, PI / 4.0);
        if side == 1 {
            y = PI - y;
        }
        m.parts[i].pose.x_rot = x;
        m.parts[i].pose.y_rot = y;
    }
}

/// VANILLA: BoatModel#createWaterPatch — только глубина (RenderTypes#waterMask), текстура не нужна.
pub fn boat_water_patch() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "water_patch",
        None,
        Pose {
            x_rot: HALF_PI,
            ..Pose::offset(0.0, -3.0, 1.0)
        },
        &[BoxDef::new(
            [0.0, 0.0],
            [-14.0, -9.0, -3.0],
            [28.0, 16.0, 3.0],
        )],
    );
    m
}

#[cfg(test)]
mod boat_tests {
    use super::*;

    #[test]
    fn boat_parts_and_paddle_anim() {
        assert_eq!(boat_mesh(false, false).parts.len(), 7);
        assert_eq!(boat_mesh(true, false).parts.len(), 3);
        assert_eq!(boat_mesh(false, true).parts.len(), 10);
        let mut m = boat_mesh(false, false);
        boat_anim(&mut m, 0.0, 0.0);
        let (l, r) = (
            m.index("left_paddle").unwrap_or(0),
            m.index("right_paddle").unwrap_or(0),
        );
        // time=0: x = lerp(0.5, −π/3, −π/12), y_left = lerp(sin(1)/2+0.5, −π/4, π/4).
        let x = -PI / 3.0 + (PI / 3.0 - PI / 12.0) * 0.5;
        assert!((m.parts[l].pose.x_rot - x).abs() < 1e-5);
        assert!((m.parts[l].pose.y_rot + m.parts[r].pose.y_rot - PI).abs() < 1e-5);
    }
}

fn puffer_part(
    m: &mut Model,
    name: &'static str,
    uv: [f32; 2],
    o: [f32; 3],
    s: [f32; 3],
    p: [f32; 3],
    rot: [f32; 3],
) {
    m.add(
        name,
        None,
        Pose {
            x_rot: rot[0],
            y_rot: rot[1],
            z_rot: rot[2],
            ..Pose::offset(p[0], p[1], p[2])
        },
        &[BoxDef::new(uv, o, s)],
    );
}

/// VANILLA: PufferfishBigModel#createBodyLayer (32×32). `state` 2.
/// VANILLA: PufferfishMidModel#createBodyLayer — `state` 1; PufferfishSmallModel — `state` 0.
pub fn pufferfish_mesh(state: u8) -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let q = PI / 4.0;
    let z = [0.0; 3];
    let rx = |a: f32| [a, 0.0, 0.0];
    let ry = |a: f32| [0.0, a, 0.0];
    match state {
        0 => {
            puffer_part(
                &mut m,
                "body",
                [0.0, 27.0],
                [-1.5, -2.0, -1.5],
                [3.0, 2.0, 3.0],
                [0.0, 23.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "right_eye",
                [24.0, 6.0],
                [-1.5, 0.0, -1.5],
                [1.0, 1.0, 1.0],
                [0.0, 20.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "left_eye",
                [28.0, 6.0],
                [0.5, 0.0, -1.5],
                [1.0, 1.0, 1.0],
                [0.0, 20.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "back_fin",
                [-3.0, 0.0],
                [-1.5, 0.0, 0.0],
                [3.0, 0.0, 3.0],
                [0.0, 22.0, 1.5],
                z,
            );
            puffer_part(
                &mut m,
                "right_fin",
                [25.0, 0.0],
                [-1.0, 0.0, 0.0],
                [1.0, 0.0, 2.0],
                [-1.5, 22.0, -1.5],
                z,
            );
            puffer_part(
                &mut m,
                "left_fin",
                [25.0, 0.0],
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 2.0],
                [1.5, 22.0, -1.5],
                z,
            );
        }
        1 => {
            puffer_part(
                &mut m,
                "body",
                [12.0, 22.0],
                [-2.5, -5.0, -2.5],
                [5.0, 5.0, 5.0],
                [0.0, 22.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "right_blue_fin",
                [24.0, 0.0],
                [-2.0, 0.0, 0.0],
                [2.0, 0.0, 2.0],
                [-2.5, 18.0, -1.5],
                z,
            );
            puffer_part(
                &mut m,
                "left_blue_fin",
                [24.0, 3.0],
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 2.0],
                [2.5, 18.0, -1.5],
                z,
            );
            puffer_part(
                &mut m,
                "top_front_fin",
                [19.0, 17.0],
                [-2.5, -1.0, 0.0],
                [5.0, 1.0, 0.0],
                [0.0, 17.0, -2.5],
                rx(q),
            );
            puffer_part(
                &mut m,
                "top_back_fin",
                [11.0, 17.0],
                [-2.5, -1.0, 0.0],
                [5.0, 1.0, 0.0],
                [0.0, 17.0, 2.5],
                rx(-q),
            );
            puffer_part(
                &mut m,
                "right_front_fin",
                [5.0, 17.0],
                [-1.0, -5.0, 0.0],
                [1.0, 5.0, 0.0],
                [-2.5, 22.0, -2.5],
                ry(-q),
            );
            puffer_part(
                &mut m,
                "right_back_fin",
                [9.0, 17.0],
                [-1.0, -5.0, 0.0],
                [1.0, 5.0, 0.0],
                [-2.5, 22.0, 2.5],
                ry(q),
            );
            puffer_part(
                &mut m,
                "left_back_fin",
                [1.0, 17.0],
                [0.0, -5.0, 0.0],
                [1.0, 5.0, 0.0],
                [2.5, 22.0, 2.5],
                ry(-q),
            );
            puffer_part(
                &mut m,
                "left_front_fin",
                [1.0, 17.0],
                [0.0, -5.0, 0.0],
                [1.0, 5.0, 0.0],
                [2.5, 22.0, -2.5],
                ry(q),
            );
            puffer_part(
                &mut m,
                "bottom_back_fin",
                [18.0, 20.0],
                [0.0, 0.0, 0.0],
                [5.0, 1.0, 0.0],
                [-2.5, 22.0, 2.5],
                rx(q),
            );
            puffer_part(
                &mut m,
                "bottom_front_fin",
                [17.0, 19.0],
                [-2.5, 0.0, 0.0],
                [5.0, 1.0, 1.0],
                [0.0, 22.0, -2.5],
                rx(-q),
            );
        }
        _ => {
            puffer_part(
                &mut m,
                "body",
                [0.0, 0.0],
                [-4.0, -8.0, -4.0],
                [8.0, 8.0, 8.0],
                [0.0, 22.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "right_blue_fin",
                [24.0, 0.0],
                [-2.0, 0.0, -1.0],
                [2.0, 1.0, 2.0],
                [-4.0, 15.0, -2.0],
                z,
            );
            puffer_part(
                &mut m,
                "left_blue_fin",
                [24.0, 3.0],
                [0.0, 0.0, -1.0],
                [2.0, 1.0, 2.0],
                [4.0, 15.0, -2.0],
                z,
            );
            puffer_part(
                &mut m,
                "top_front_fin",
                [15.0, 17.0],
                [-4.0, -1.0, 0.0],
                [8.0, 1.0, 0.0],
                [0.0, 14.0, -4.0],
                rx(q),
            );
            puffer_part(
                &mut m,
                "top_middle_fin",
                [14.0, 16.0],
                [-4.0, -1.0, 0.0],
                [8.0, 1.0, 1.0],
                [0.0, 14.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "top_back_fin",
                [23.0, 18.0],
                [-4.0, -1.0, 0.0],
                [8.0, 1.0, 0.0],
                [0.0, 14.0, 4.0],
                rx(-q),
            );
            puffer_part(
                &mut m,
                "right_front_fin",
                [5.0, 17.0],
                [-1.0, -8.0, 0.0],
                [1.0, 8.0, 0.0],
                [-4.0, 22.0, -4.0],
                ry(-q),
            );
            puffer_part(
                &mut m,
                "left_front_fin",
                [1.0, 17.0],
                [0.0, -8.0, 0.0],
                [1.0, 8.0, 0.0],
                [4.0, 22.0, -4.0],
                ry(q),
            );
            puffer_part(
                &mut m,
                "bottom_front_fin",
                [15.0, 20.0],
                [-4.0, 0.0, 0.0],
                [8.0, 1.0, 0.0],
                [0.0, 22.0, -4.0],
                rx(-q),
            );
            puffer_part(
                &mut m,
                "bottom_middle_fin",
                [15.0, 20.0],
                [-4.0, 0.0, 0.0],
                [8.0, 1.0, 0.0],
                [0.0, 22.0, 0.0],
                z,
            );
            puffer_part(
                &mut m,
                "bottom_back_fin",
                [15.0, 20.0],
                [-4.0, 0.0, 0.0],
                [8.0, 1.0, 0.0],
                [0.0, 22.0, 4.0],
                rx(q),
            );
            puffer_part(
                &mut m,
                "right_back_fin",
                [9.0, 17.0],
                [-1.0, -8.0, 0.0],
                [1.0, 8.0, 0.0],
                [-4.0, 22.0, 4.0],
                ry(q),
            );
            puffer_part(
                &mut m,
                "left_back_fin",
                [9.0, 17.0],
                [0.0, -8.0, 0.0],
                [1.0, 8.0, 0.0],
                [4.0, 22.0, 4.0],
                ry(-q),
            );
        }
    }
    m
}

/// VANILLA: Pufferfish*Model#setupAnim — колебание боковых плавников.
pub fn pufferfish_anim(m: &mut Model, state: u8, age: f32) {
    let (r, l) = if state == 0 {
        ("right_fin", "left_fin")
    } else {
        ("right_blue_fin", "left_blue_fin")
    };
    let s = 0.4 * (age * 0.2).sin();
    set_z(m, r, -0.2 + s);
    set_z(m, l, 0.2 - s);
}

/// VANILLA: DyeColor#getTextureDiffuseColor (цвет без множителя овцы).
pub fn dye_diffuse(dye: u8) -> [u8; 4] {
    const DIFFUSE: [u32; 16] = [
        16383998, 16351261, 13061821, 3847130, 16701501, 8439583, 15961002, 4673362, 10329495,
        1481884, 8991416, 3949738, 8606770, 6192150, 11546150, 1908001,
    ];
    let c = DIFFUSE[dye as usize & 15];
    [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255]
}

/// VANILLA: TropicalFishSmallModel/LargeModel#createBodyLayer (32×32); `g` — CubeDeformation
/// (0 для тела, 0.01 для слоя узора).
pub fn tropical_fish_mesh(large: bool, g: f32) -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let q = PI / 4.0;
    let mut add =
        |name: &'static str, uv: [f32; 2], o: [f32; 3], s: [f32; 3], p: [f32; 3], y: f32| {
            m.add(
                name,
                None,
                Pose {
                    y_rot: y,
                    ..Pose::offset(p[0], p[1], p[2])
                },
                &[BoxDef::new(uv, o, s).grow(g)],
            );
        };
    if large {
        add(
            "body",
            [0.0, 20.0],
            [-1.0, -3.0, -3.0],
            [2.0, 6.0, 6.0],
            [0.0, 19.0, 0.0],
            0.0,
        );
        add(
            "tail",
            [21.0, 16.0],
            [0.0, -3.0, 0.0],
            [0.0, 6.0, 5.0],
            [0.0, 19.0, 3.0],
            0.0,
        );
        add(
            "right_fin",
            [2.0, 16.0],
            [-2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [-1.0, 20.0, 0.0],
            q,
        );
        add(
            "left_fin",
            [2.0, 12.0],
            [0.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [1.0, 20.0, 0.0],
            -q,
        );
        add(
            "top_fin",
            [20.0, 11.0],
            [0.0, -4.0, 0.0],
            [0.0, 4.0, 6.0],
            [0.0, 16.0, -3.0],
            0.0,
        );
        add(
            "bottom_fin",
            [20.0, 21.0],
            [0.0, 0.0, 0.0],
            [0.0, 4.0, 6.0],
            [0.0, 22.0, -3.0],
            0.0,
        );
    } else {
        add(
            "body",
            [0.0, 0.0],
            [-1.0, -1.5, -3.0],
            [2.0, 3.0, 6.0],
            [0.0, 22.0, 0.0],
            0.0,
        );
        add(
            "tail",
            [22.0, -6.0],
            [0.0, -1.5, 0.0],
            [0.0, 3.0, 6.0],
            [0.0, 22.0, 3.0],
            0.0,
        );
        add(
            "right_fin",
            [2.0, 16.0],
            [-2.0, -1.0, 0.0],
            [2.0, 2.0, 0.0],
            [-1.0, 22.5, 0.0],
            q,
        );
        add(
            "left_fin",
            [2.0, 12.0],
            [0.0, -1.0, 0.0],
            [2.0, 2.0, 0.0],
            [1.0, 22.5, 0.0],
            -q,
        );
        add(
            "top_fin",
            [10.0, -5.0],
            [0.0, -3.0, 0.0],
            [0.0, 3.0, 6.0],
            [0.0, 20.5, -3.0],
            0.0,
        );
    }
    m
}

/// VANILLA: TropicalFish*Model#setupAnim.
pub fn tropical_fish_anim(m: &mut Model, age: f32, in_water: bool) {
    let a = if in_water { 1.0 } else { 1.5 };
    set_y(m, "tail", -a * 0.45 * (0.6 * age).sin());
}

/// VANILLA: AdultAxolotlModel#createBodyLayer (64×64).
pub fn axolotl_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let f = 0.001;
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 19.5, 5.0),
        &[
            BoxDef::new([0.0, 11.0], [-4.0, -2.0, -9.0], [8.0, 4.0, 10.0]),
            BoxDef::new([2.0, 17.0], [0.0, -3.0, -8.0], [0.0, 5.0, 9.0]),
        ],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, 0.0, -9.0),
        &[BoxDef::new([0.0, 1.0], [-4.0, -3.0, -5.0], [8.0, 5.0, 5.0]).grow(f)],
    );
    m.add(
        "top_gills",
        Some(head),
        Pose::offset(0.0, -3.0, -1.0),
        &[BoxDef::new([3.0, 37.0], [-4.0, -3.0, 0.0], [8.0, 3.0, 0.0]).grow(f)],
    );
    m.add(
        "left_gills",
        Some(head),
        Pose::offset(-4.0, 0.0, -1.0),
        &[BoxDef::new([0.0, 40.0], [-3.0, -5.0, 0.0], [3.0, 7.0, 0.0]).grow(f)],
    );
    m.add(
        "right_gills",
        Some(head),
        Pose::offset(4.0, 0.0, -1.0),
        &[BoxDef::new([11.0, 40.0], [0.0, -5.0, 0.0], [3.0, 7.0, 0.0]).grow(f)],
    );
    let left = BoxDef::new([2.0, 13.0], [-1.0, 0.0, 0.0], [3.0, 5.0, 0.0]).grow(f);
    let right = BoxDef::new([2.0, 13.0], [-2.0, 0.0, 0.0], [3.0, 5.0, 0.0]).grow(f);
    m.add(
        "right_hind_leg",
        Some(body),
        Pose::offset(-3.5, 1.0, -1.0),
        &[right],
    );
    m.add(
        "left_hind_leg",
        Some(body),
        Pose::offset(3.5, 1.0, -1.0),
        &[left],
    );
    m.add(
        "right_front_leg",
        Some(body),
        Pose::offset(-3.5, 1.0, -8.0),
        &[right],
    );
    m.add(
        "left_front_leg",
        Some(body),
        Pose::offset(3.5, 1.0, -8.0),
        &[left],
    );
    m.add(
        "tail",
        Some(body),
        Pose::offset(0.0, 0.0, 1.0),
        &[BoxDef::new([2.0, 19.0], [0.0, -3.0, 0.0], [0.0, 5.0, 12.0])],
    );
    m
}

/// VANILLA: BabyAxolotlModel#createBodyLayer (32×32).
pub fn axolotl_baby_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let root = m.add("root", None, Pose::offset(0.0, 24.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(root),
        Pose::offset(0.0, -1.25, 1.75),
        &[
            BoxDef::new([0.0, 0.0], [-2.0, -0.75, -2.75], [4.0, 2.0, 6.0]),
            BoxDef::new([0.0, 12.0], [0.0, -1.75, -2.75], [0.0, 3.0, 5.0]),
        ],
    );
    let flat = |u: f32, v: f32, x: f32| BoxDef::new([u, v], [x, 0.0, -0.5], [3.0, 0.0, 1.0]);
    m.add(
        "right_front_leg",
        Some(body),
        Pose::offset(-2.0, 0.25, -1.25),
        &[flat(20.0, 16.0, -3.0)],
    );
    let rh = m.add(
        "right_hind_leg",
        Some(body),
        Pose {
            x_rot: 0.0,
            y_rot: 1.5708,
            z_rot: 1.5708,
            ..Pose::offset(-2.0, 0.25, 1.75)
        },
        &[],
    );
    m.add(
        "right_leg_r1",
        Some(rh),
        Pose {
            x_rot: -1.5708,
            y_rot: 0.0,
            z_rot: 1.5708,
            ..Pose::offset(0.0, 0.0, 0.0)
        },
        &[flat(20.0, 14.0, 0.0)],
    );
    m.add(
        "left_front_leg",
        Some(body),
        Pose::offset(2.0, 0.25, -1.25),
        &[flat(20.0, 13.0, 0.0)],
    );
    m.add(
        "left_hind_leg",
        Some(body),
        Pose::offset(2.0, 0.25, 1.75),
        &[flat(20.0, 14.0, 0.0)],
    );
    m.add(
        "tail",
        Some(body),
        Pose::offset(0.0, -0.25, 3.25),
        &[BoxDef::new([10.0, 9.0], [0.0, -1.5, -1.0], [0.0, 3.0, 8.0])],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, 0.25, -2.75),
        &[BoxDef::new([0.0, 8.0], [-3.0, -2.0, -4.0], [6.0, 3.0, 4.0])],
    );
    m.add(
        "left_gills",
        Some(head),
        Pose::offset(3.0, -0.5, -2.0),
        &[BoxDef::new([20.0, 8.0], [0.0, -3.5, 0.0], [3.0, 5.0, 0.0])],
    );
    m.add(
        "right_gills",
        Some(head),
        Pose::offset(-3.0, -0.5, -2.0),
        &[BoxDef::new([20.0, 3.0], [-3.0, -3.5, 0.0], [3.0, 5.0, 0.0])],
    );
    m.add(
        "top_gills",
        Some(head),
        Pose::offset(0.0, -2.0, -2.0),
        &[BoxDef::new([20.0, 0.0], [-3.0, -3.0, 0.0], [6.0, 3.0, 0.0])],
    );
    m
}

/// VANILLA: BabyAxolotlModel#setupAnim. `state` — какое из семи AnimationState запущено
/// (0 swim, 1 walk, 2 walkUnderwater, 3 idleUnderwater, 4 idleUnderwaterOnGround, 5 idleOnGround,
/// 6 playDead), `ms` — миллисекунды с его старта.
pub fn axolotl_baby_anim(m: &mut Model, state: u8, ms: i64, wp: f32, ws: f32) {
    use crate::keyframes_data::{
        AXOLOTL_WALK_FLOOR, BABY_AXOLOTL_IDLE_FLOOR, BABY_AXOLOTL_PLAY_DEAD, BABY_AXOLOTL_SWIM,
        IDLE_FLOOR_UNDERWATER, IDLE_UNDERWATER, WALK_FLOOR_UNDERWATER,
    };
    if state == 1 {
        AXOLOTL_WALK_FLOOR.apply_walk(m, wp, ws, 15.0, 30.0);
    }
    let def = match state {
        0 => &*BABY_AXOLOTL_SWIM,
        // VANILLA: walkUnderwaterAnimation.apply(walkAnimationState, …) — тот же стейт, что и у ходьбы.
        1 | 2 => &*WALK_FLOOR_UNDERWATER,
        3 => &*IDLE_UNDERWATER,
        4 => &*IDLE_FLOOR_UNDERWATER,
        5 => &*BABY_AXOLOTL_IDLE_FLOOR,
        _ => &*BABY_AXOLOTL_PLAY_DEAD,
    };
    def.apply(m, ms, 1.0);
}

/// VANILLA: AdultAxolotlModel#setupAnim. `f` = [playingDead, inWater, onGround, moving]
/// (BinaryAnimator-факторы, уже с easing); `yr`/`xr` — градусы.
pub fn axolotl_anim(m: &mut Model, yr: f32, xr: f32, age: f32, f: [f32; 4]) {
    let [dead, water, ground, moving] = f;
    let not_moving = 1.0 - moving;
    let mirrored = 1.0 - ground.min(moving);
    let ix = |n: &str| m.index(n).expect("axolotl part");
    let (body, head, top, lg, rg) = (
        ix("body"),
        ix("head"),
        ix("top_gills"),
        ix("left_gills"),
        ix("right_gills"),
    );
    let (lh, rh, lf, rf, tail) = (
        ix("left_hind_leg"),
        ix("right_hind_leg"),
        ix("left_front_leg"),
        ix("right_front_leg"),
        ix("tail"),
    );
    let p = &mut m.parts;
    p[body].pose.y_rot += yr.to_radians();
    // setupSwimmingAnimation
    let fa = moving.min(water);
    if fa > 1.0e-5 {
        let a = age * 0.33;
        let (s, c) = (a.sin(), a.cos());
        let sway = 0.13 * s;
        p[body].pose.x_rot += (xr.to_radians() + sway) * fa;
        p[head].pose.x_rot -= sway * 1.8 * fa;
        p[body].pose.y -= 0.45 * c * fa;
        p[top].pose.x_rot += (-0.5 * s - 0.8) * fa;
        let g = (0.3 * s + 0.9) * fa;
        p[lg].pose.y_rot += g;
        p[rg].pose.y_rot -= g;
        p[tail].pose.y_rot += 0.3 * (a * 0.9).cos() * fa;
        p[lh].pose.x_rot += 1.884_955_6 * fa;
        p[lh].pose.y_rot += -0.4 * s * fa;
        p[lh].pose.z_rot += PI / 2.0 * fa;
        p[lf].pose.x_rot += 1.884_955_6 * fa;
        p[lf].pose.y_rot += (-0.2 * c - 0.1) * fa;
        p[lf].pose.z_rot += PI / 2.0 * fa;
    }
    // setupWaterHoveringAnimation
    let fa = not_moving.min(water);
    if fa > 1.0e-5 {
        let a = age * 0.075;
        let c = a.cos();
        let s = a.sin() * 0.15;
        let bx = (-0.15 + 0.075 * c) * fa;
        p[body].pose.x_rot += bx;
        p[body].pose.y -= s * fa;
        p[head].pose.x_rot -= bx;
        p[top].pose.x_rot += 0.2 * c * fa;
        let g = (-0.3 * c - 0.19) * fa;
        p[lg].pose.y_rot += g;
        p[rg].pose.y_rot -= g;
        p[lh].pose.x_rot += (PI * 3.0 / 4.0 - c * 0.11) * fa;
        p[lh].pose.y_rot += 0.471_238_94 * fa;
        p[lh].pose.z_rot += 1.727_876_1 * fa;
        p[lf].pose.x_rot += (PI / 4.0 - c * 0.2) * fa;
        p[lf].pose.y_rot += 2.042_035 * fa;
        p[tail].pose.y_rot += 0.5 * c * fa;
    }
    // setupGroundCrawlingAnimation
    let fa = moving.min(ground);
    if fa > 1.0e-5 {
        let a = age * 0.11;
        let c = a.cos();
        let hind = (c * c - 2.0 * c) / 5.0;
        let front = 0.7 * c;
        let ht = 0.09 * c * fa;
        p[head].pose.y_rot += ht;
        p[tail].pose.y_rot += ht;
        let g = (0.6 - 0.08 * (c * c + 2.0 * a.sin())) * fa;
        p[top].pose.x_rot += g;
        p[lg].pose.y_rot -= g;
        p[rg].pose.y_rot += g;
        let (hx, fx) = (0.942_477_9 * fa, 1.099_557_4 * fa);
        p[lh].pose.x_rot += hx;
        p[lh].pose.y_rot += (1.5 - hind) * fa;
        p[lh].pose.z_rot += -0.1 * fa;
        p[lf].pose.x_rot += fx;
        p[lf].pose.y_rot += (PI / 2.0 - front) * fa;
        p[rh].pose.x_rot += hx;
        p[rh].pose.y_rot += (-1.0 - hind) * fa;
        p[rf].pose.x_rot += fx;
        p[rf].pose.y_rot += (-PI / 2.0 - front) * fa;
    }
    // setupLayStillOnGroundAnimation
    let fa = not_moving.min(ground);
    if fa > 1.0e-5 {
        let a = age * 0.09;
        let (s, c) = (a.sin(), a.cos());
        let mv = s * s - 2.0 * s;
        let mv2 = c * c - 3.0 * s;
        p[head].pose.x_rot += -0.09 * mv * fa;
        p[head].pose.z_rot += -0.2 * fa;
        p[tail].pose.y_rot += (-0.1 + 0.1 * mv) * fa;
        let g = (0.6 + 0.05 * mv2) * fa;
        p[top].pose.x_rot += g;
        p[lg].pose.y_rot -= g;
        p[rg].pose.y_rot += g;
        p[lh].pose.x_rot += 1.1 * fa;
        p[lh].pose.y_rot += 1.0 * fa;
        p[lf].pose.x_rot += 0.8 * fa;
        p[lf].pose.y_rot += 2.3 * fa;
        p[lf].pose.z_rot -= 0.5 * fa;
    }
    // setupPlayDeadAnimation
    if dead > 1.0e-5 {
        p[lh].pose.x_rot += 1.413_716_7 * dead;
        p[lh].pose.y_rot += 1.099_557_4 * dead;
        p[lh].pose.z_rot += PI / 4.0 * dead;
        p[lf].pose.x_rot += PI / 4.0 * dead;
        p[lf].pose.y_rot += 2.042_035 * dead;
        p[body].pose.x_rot += -0.15 * dead;
        p[body].pose.z_rot += 0.35 * dead;
    }
    // applyMirrorLegRotations
    if mirrored > 1.0e-5 {
        let (a, b) = (p[lh].pose, p[lf].pose);
        p[rh].pose.x_rot += a.x_rot * mirrored;
        p[rh].pose.y_rot += -a.y_rot * mirrored;
        p[rh].pose.z_rot += -a.z_rot * mirrored;
        p[rf].pose.x_rot += b.x_rot * mirrored;
        p[rf].pose.y_rot += -b.y_rot * mirrored;
        p[rf].pose.z_rot += -b.z_rot * mirrored;
    }
}

/// VANILLA: AllayModel#createBodyLayer (32×32). Корень "root" смещён на (0, 23.5, 0).
pub fn allay_mesh() -> Model {
    let mut m = Model::new([32.0, 32.0]);
    let root = m.add("root", None, Pose::offset(0.0, 23.5, 0.0), &[]);
    m.add(
        "head",
        Some(root),
        Pose::offset(0.0, -3.99, 0.0),
        &[BoxDef::new([0.0, 0.0], [-2.5, -5.0, -2.5], [5.0, 5.0, 5.0])],
    );
    let body = m.add(
        "body",
        Some(root),
        Pose::offset(0.0, -4.0, 0.0),
        &[
            BoxDef::new([0.0, 10.0], [-1.5, 0.0, -1.0], [3.0, 4.0, 2.0]),
            BoxDef::new([0.0, 16.0], [-1.5, 0.0, -1.0], [3.0, 5.0, 2.0]).grow(-0.2),
        ],
    );
    m.add(
        "right_arm",
        Some(body),
        Pose::offset(-1.75, 0.5, 0.0),
        &[BoxDef::new([23.0, 0.0], [-0.75, -0.5, -1.0], [1.0, 4.0, 2.0]).grow(-0.01)],
    );
    m.add(
        "left_arm",
        Some(body),
        Pose::offset(1.75, 0.5, 0.0),
        &[BoxDef::new([23.0, 6.0], [-0.25, -0.5, -1.0], [1.0, 4.0, 2.0]).grow(-0.01)],
    );
    let wing = BoxDef::new([16.0, 14.0], [0.0, 1.0, 0.0], [0.0, 5.0, 8.0]);
    m.add(
        "right_wing",
        Some(body),
        Pose::offset(-0.5, 0.0, 0.6),
        &[wing],
    );
    m.add(
        "left_wing",
        Some(body),
        Pose::offset(0.5, 0.0, 0.6),
        &[wing],
    );
    m
}

/// VANILLA: AllayModel#setupAnim (без танца и предмета в руках). `yr`/`xr` — градусы.
pub fn allay_anim(m: &mut Model, yr: f32, xr: f32, walk_pos: f32, walk_speed: f32, age: f32) {
    let flap = age * 20.0_f32.to_radians() + walk_pos;
    let flap_amount = flap.cos() * PI * 0.15 + walk_speed;
    let bob = age * 9.0_f32.to_radians();
    let fly = (walk_speed / 0.3).min(1.0);
    let idle = 1.0 - fly;
    let ix = |n: &str| m.index(n).expect("allay part");
    let (root, head, body) = (ix("root"), ix("head"), ix("body"));
    let (ra, la, rw, lw) = (
        ix("right_arm"),
        ix("left_arm"),
        ix("right_wing"),
        ix("left_wing"),
    );
    let p = &mut m.parts;
    p[head].pose.x_rot = xr.to_radians();
    p[head].pose.y_rot = yr.to_radians();
    p[rw].pose.x_rot = 0.436_332_32 * idle;
    p[rw].pose.y_rot = -PI / 4.0 + flap_amount;
    p[lw].pose.x_rot = 0.436_332_32 * idle;
    p[lw].pose.y_rot = PI / 4.0 - flap_amount;
    p[body].pose.x_rot = fly * PI / 4.0;
    p[root].pose.y += bob.cos() * 0.25 * idle;
    let arm = 0.436_332_32 - (bob + PI * 1.5).cos() * PI * 0.075 * idle;
    p[ra].pose.x_rot = 0.0;
    p[la].pose.x_rot = 0.0;
    p[la].pose.z_rot = -arm;
    p[ra].pose.z_rot = arm;
}

/// VANILLA: AdultArmadilloModel#createBodyLayer (64×64). Кубы тела вынесены в дочернюю часть
/// `body_skin`, чтобы скрывать их (ModelPart#skipDraw), не пряча голову.
pub fn armadillo_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add("body", None, Pose::offset(0.0, 21.0, 4.0), &[]);
    m.add(
        "body_skin",
        Some(body),
        Pose::offset(0.0, 0.0, 0.0),
        &[
            BoxDef::new([0.0, 20.0], [-4.0, -7.0, -10.0], [8.0, 8.0, 12.0]).grow(0.3),
            BoxDef::new([0.0, 40.0], [-4.0, -7.0, -10.0], [8.0, 8.0, 12.0]),
        ],
    );
    m.add(
        "tail",
        Some(body),
        Pose {
            x_rot: 0.5061,
            ..Pose::offset(0.0, -3.0, 1.0)
        },
        &[BoxDef::new(
            [44.0, 53.0],
            [-0.5, -0.0865, 0.0933],
            [1.0, 6.0, 1.0],
        )],
    );
    let head = m.add("head", Some(body), Pose::offset(0.0, -2.0, -11.0), &[]);
    m.add(
        "head_cube",
        Some(head),
        Pose {
            x_rot: -0.3927,
            ..Pose::offset(0.0, 0.0, 0.0)
        },
        &[BoxDef::new(
            [43.0, 15.0],
            [-1.5, -1.0, -1.0],
            [3.0, 5.0, 2.0],
        )],
    );
    let re = m.add("right_ear", Some(head), Pose::offset(-1.0, -1.0, 0.0), &[]);
    m.add(
        "right_ear_cube",
        Some(re),
        Pose {
            x_rot: 0.1886,
            y_rot: -0.3864,
            z_rot: -0.0718,
            ..Pose::offset(-0.5, 0.0, -0.6)
        },
        &[BoxDef::new(
            [43.0, 10.0],
            [-2.0, -3.0, 0.0],
            [2.0, 5.0, 0.0],
        )],
    );
    let le = m.add("left_ear", Some(head), Pose::offset(1.0, -2.0, 0.0), &[]);
    m.add(
        "left_ear_cube",
        Some(le),
        Pose {
            x_rot: 0.1886,
            y_rot: 0.3864,
            z_rot: 0.0718,
            ..Pose::offset(0.5, 1.0, -0.6)
        },
        &[BoxDef::new([47.0, 10.0], [0.0, -3.0, 0.0], [2.0, 5.0, 0.0])],
    );
    let leg = |u: f32, v: f32| BoxDef::new([u, v], [-1.0, 0.0, -1.0], [2.0, 3.0, 2.0]);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-2.0, 21.0, 4.0),
        &[leg(51.0, 31.0)],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(2.0, 21.0, 4.0),
        &[leg(42.0, 31.0)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(-2.0, 21.0, -4.0),
        &[leg(51.0, 43.0)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(2.0, 21.0, -4.0),
        &[leg(42.0, 43.0)],
    );
    m.add(
        "cube",
        None,
        Pose::offset(0.0, 24.0, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-5.0, -10.0, -6.0],
            [10.0, 10.0, 10.0],
        )],
    );
    m
}

/// VANILLA: BabyArmadilloModel#createBodyLayer (64×64). Кубы тела вынесены в `body_skin` (вместо skipDraw).
pub fn armadillo_baby_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add("body", None, Pose::offset(0.0, 20.0, 0.5), &[]);
    m.add(
        "body_skin",
        Some(body),
        Pose::offset(0.0, 0.0, 0.0),
        &[
            BoxDef::new([0.0, 0.0], [-2.5, -2.0, -3.5], [5.0, 4.0, 7.0]).grow(0.3),
            BoxDef::new([0.0, 11.0], [-2.5, -2.0, -3.0], [5.0, 4.0, 6.0]),
        ],
    );
    let tail = m.add("tail", Some(body), Pose::offset(0.0, 0.0, 3.4), &[]);
    m.add(
        "right_ear_cube",
        Some(tail),
        Pose {
            x_rot: -1.0472,
            ..Pose::offset(0.0, 1.5, 1.0)
        },
        &[BoxDef::new(
            [22.0, 11.0],
            [-0.5, -0.5, -2.0],
            [1.0, 1.0, 4.0],
        )],
    );
    let head = m.add("head", Some(body), Pose::offset(0.0, 0.0, -3.2), &[]);
    let hc = m.add(
        "head_cube",
        Some(head),
        Pose {
            x_rot: 0.7417649,
            ..Pose::offset(0.0, 0.0, 0.0)
        },
        &[BoxDef::new(
            [20.0, 17.0],
            [-1.0, -2.0, -4.0],
            [2.0, 2.0, 4.0],
        )],
    );
    m.add(
        "right_ear",
        Some(hc),
        Pose {
            x_rot: -0.4363,
            y_rot: -0.1134,
            z_rot: 0.0524,
            ..Pose::offset(-1.0, -2.0, -0.3)
        },
        &[BoxDef::new([28.0, 8.0], [-1.8, -2.0, 0.0], [2.0, 3.0, 0.0]).mirror()],
    );
    m.add(
        "left_ear",
        Some(hc),
        Pose {
            x_rot: -0.4363,
            y_rot: 0.1134,
            z_rot: -0.0524,
            ..Pose::offset(1.0, -2.0, -0.3)
        },
        &[BoxDef::new([28.0, 8.0], [-0.2, -2.0, 0.0], [2.0, 3.0, 0.0])],
    );
    let leg = |u: f32, v: f32| BoxDef::new([u, v], [-1.0, 0.0, -1.0], [2.0, 2.0, 2.0]);
    m.add(
        "right_hind_leg",
        None,
        Pose::offset(-1.5, 22.0, 2.5),
        &[leg(20.0, 27.0).mirror()],
    );
    m.add(
        "left_hind_leg",
        None,
        Pose::offset(1.5, 22.0, 2.5),
        &[leg(20.0, 27.0)],
    );
    m.add(
        "right_front_leg",
        None,
        Pose::offset(1.5, 22.0, -1.5),
        &[leg(20.0, 23.0)],
    );
    m.add(
        "left_front_leg",
        None,
        Pose::offset(-1.5, 22.0, -1.5),
        &[leg(24.0, 0.0).mirror()],
    );
    m.add(
        "cube",
        None,
        Pose::offset(0.0, 20.7, 0.5),
        &[BoxDef::new([0.0, 25.0], [-3.0, -3.0, -3.0], [6.0, 6.0, 6.0]).grow(0.3)],
    );
    m
}

/// VANILLA: ArmadilloModel#setupAnim. `roll_up`/`roll_out` — миллисекунды запущенных AnimationState.
#[allow(clippy::too_many_arguments)]
pub fn armadillo_anim(
    m: &mut Model,
    yr: f32,
    xr: f32,
    wp: f32,
    ws: f32,
    hiding: bool,
    roll_up: Option<i64>,
    roll_out: Option<i64>,
    baby: bool,
) {
    use crate::keyframes_data::{
        ARMADILLO_BABY_ROLL_OUT, ARMADILLO_BABY_ROLL_UP, ARMADILLO_BABY_WALK, ARMADILLO_ROLL_OUT,
        ARMADILLO_ROLL_UP, ARMADILLO_WALK,
    };
    let (walk, up, out) = if baby {
        (
            &*ARMADILLO_BABY_WALK,
            &*ARMADILLO_BABY_ROLL_UP,
            &*ARMADILLO_BABY_ROLL_OUT,
        )
    } else {
        (&*ARMADILLO_WALK, &*ARMADILLO_ROLL_UP, &*ARMADILLO_ROLL_OUT)
    };
    let vis = |m: &mut Model, n: &str, v: bool| {
        if let Some(i) = m.index(n) {
            m.parts[i].visible = v;
        }
    };
    vis(m, "body_skin", !hiding);
    vis(m, "left_hind_leg", !hiding);
    vis(m, "right_hind_leg", !hiding);
    vis(m, "tail", !hiding);
    vis(m, "cube", hiding);
    if !hiding {
        set_head(m, yr.clamp(-32.5, 32.5), xr.clamp(-22.5, 25.0));
        walk.apply_walk(m, wp, ws, 16.5, 2.5);
    }
    if let Some(t) = roll_out {
        out.apply(m, t, 1.0);
    }
    if let Some(t) = roll_up {
        up.apply(m, t, 1.0);
    }
}

/// VANILLA: SnifferModel#createBodyLayer (192×192).
pub fn sniffer_mesh() -> Model {
    let mut m = Model::new([192.0, 192.0]);
    let bone = m.add("bone", None, Pose::offset(0.0, 5.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(bone),
        Pose::offset(0.0, 0.0, 0.0),
        &[
            BoxDef::new([62.0, 68.0], [-12.5, -14.0, -20.0], [25.0, 29.0, 40.0]),
            BoxDef::new([62.0, 0.0], [-12.5, -14.0, -20.0], [25.0, 24.0, 40.0]).grow(0.5),
            BoxDef::new([87.0, 68.0], [-12.5, 12.0, -20.0], [25.0, 0.0, 40.0]),
        ],
    );
    let leg = |u: f32, v: f32| BoxDef::new([u, v], [-3.5, -1.0, -4.0], [7.0, 10.0, 8.0]);
    m.add(
        "right_front_leg",
        Some(bone),
        Pose::offset(-7.5, 10.0, -15.0),
        &[leg(32.0, 87.0)],
    );
    m.add(
        "right_mid_leg",
        Some(bone),
        Pose::offset(-7.5, 10.0, 0.0),
        &[leg(32.0, 105.0)],
    );
    m.add(
        "right_hind_leg",
        Some(bone),
        Pose::offset(-7.5, 10.0, 15.0),
        &[leg(32.0, 123.0)],
    );
    m.add(
        "left_front_leg",
        Some(bone),
        Pose::offset(7.5, 10.0, -15.0),
        &[leg(0.0, 87.0)],
    );
    m.add(
        "left_mid_leg",
        Some(bone),
        Pose::offset(7.5, 10.0, 0.0),
        &[leg(0.0, 105.0)],
    );
    m.add(
        "left_hind_leg",
        Some(bone),
        Pose::offset(7.5, 10.0, 15.0),
        &[leg(0.0, 123.0)],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, 6.5, -19.48),
        &[
            BoxDef::new([8.0, 15.0], [-6.5, -7.5, -11.5], [13.0, 18.0, 11.0]),
            BoxDef::new([8.0, 4.0], [-6.5, 7.5, -11.5], [13.0, 0.0, 11.0]),
        ],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(6.51, -7.5, -4.51),
        &[BoxDef::new([2.0, 0.0], [0.0, 0.0, -3.0], [1.0, 19.0, 7.0])],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-6.51, -7.5, -4.51),
        &[BoxDef::new(
            [48.0, 0.0],
            [-1.0, 0.0, -3.0],
            [1.0, 19.0, 7.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -4.5, -11.5),
        &[BoxDef::new(
            [10.0, 45.0],
            [-6.5, -2.0, -9.0],
            [13.0, 2.0, 9.0],
        )],
    );
    m.add(
        "lower_beak",
        Some(head),
        Pose::offset(0.0, 2.5, -12.5),
        &[BoxDef::new(
            [10.0, 57.0],
            [-6.5, -7.0, -8.0],
            [13.0, 12.0, 9.0],
        )],
    );
    m
}

/// VANILLA: SniffletModel#createBodyLayer (128×128).
pub fn sniffer_baby_mesh() -> Model {
    let mut m = Model::new([128.0, 128.0]);
    let bone = m.add("bone", None, Pose::offset(0.0, 24.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(bone),
        Pose::offset(6.0, -3.0, -9.5),
        &[
            BoxDef::new([0.0, 35.0], [-13.0, -14.0, -0.5], [14.0, 14.0, 20.0]).grow(0.25),
            BoxDef::new([0.0, 0.0], [-13.0, -14.0, -0.5], [14.0, 15.0, 20.0]),
            BoxDef::new([68.0, 0.0], [-13.0, 0.0, -0.5], [14.0, 0.0, 20.0]),
        ],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(-6.0, -4.75, 0.0),
        &[
            BoxDef::new([68.0, 20.0], [-5.0, -4.25, -7.5], [10.0, 9.0, 9.0]),
            BoxDef::new([88.0, 20.0], [-5.0, 3.75, -7.5], [10.0, 0.0, 9.0]),
        ],
    );
    m.add(
        "left_ear",
        Some(head),
        Pose::offset(5.0, -4.25, -1.5),
        &[BoxDef::new(
            [104.0, 38.0],
            [0.0, 0.0, -2.0],
            [1.0, 11.0, 3.0],
        )],
    );
    m.add(
        "right_ear",
        Some(head),
        Pose::offset(-5.0, -4.25, -1.5),
        &[BoxDef::new(
            [96.0, 38.0],
            [-1.0, 0.0, -2.0],
            [1.0, 11.0, 3.0],
        )],
    );
    m.add(
        "nose",
        Some(head),
        Pose::offset(0.0, -1.25, -9.5),
        &[BoxDef::new(
            [68.0, 47.0],
            [-5.0, -3.0, -2.0],
            [10.0, 3.0, 4.0],
        )],
    );
    m.add(
        "lower_beak",
        Some(head),
        Pose::offset(0.0, 1.25, -9.5),
        &[BoxDef::new(
            [68.0, 38.0],
            [-5.0, -2.5, -2.0],
            [10.0, 5.0, 4.0],
        )],
    );
    let leg = |u: f32, v: f32| BoxDef::new([u, v], [-2.0, -1.0, -2.0], [4.0, 5.0, 4.0]);
    for (n, x, z, u, v) in [
        ("right_front_leg", -4.0, -7.0, 0.0, 69.0),
        ("right_mid_leg", -4.0, 0.0, 0.0, 78.0),
        ("right_hind_leg", -4.0, 7.0, 0.0, 87.0),
        ("left_front_leg", 4.0, -7.0, 16.0, 69.0),
        ("left_mid_leg", 4.0, 0.0, 16.0, 78.0),
        ("left_hind_leg", 4.0, 7.0, 16.0, 87.0),
    ] {
        m.add(n, Some(bone), Pose::offset(x, -4.0, z), &[leg(u, v)]);
    }
    m
}

/// VANILLA: SnifferModel#setupAnim. `state` — Sniffer.State (#18); `ms` — время анимации
/// текущего состояния (AnimationState стартует при смене состояния).
pub fn sniffer_anim(
    m: &mut Model,
    (yr, xr): (f32, f32),
    wp: f32,
    ws: f32,
    state: i32,
    ms: i64,
    baby: bool,
) {
    use crate::keyframes_data::{
        SNIFFER_DIG, SNIFFER_HAPPY, SNIFFER_LONGSNIFF, SNIFFER_SNIFFSNIFF, SNIFFER_SNIFF_SEARCH,
        SNIFFER_STAND_UP, SNIFFER_WALK,
    };
    set_head(m, yr, xr);
    if state == 4 {
        SNIFFER_SNIFF_SEARCH.apply_walk(m, wp, ws, 9.0, 100.0);
    } else {
        SNIFFER_WALK.apply_walk(m, wp, ws, 9.0, 100.0);
    }
    match state {
        1 => SNIFFER_HAPPY.apply(m, ms, 1.0),
        2 => SNIFFER_SNIFFSNIFF.apply(m, ms, 1.0),
        3 => SNIFFER_LONGSNIFF.apply(m, ms, 1.0),
        5 => SNIFFER_DIG.apply(m, ms, 1.0),
        6 => SNIFFER_STAND_UP.apply(m, ms, 1.0),
        _ => {}
    }
    if baby {
        // VANILLA: SniffletModel#setupAnim — babyTransform.applyStatic().
        crate::keyframes_data::BABY_TRANSFORM.apply(m, 0, 1.0);
    }
}

/// VANILLA: BreezeModel#createBaseMesh + retainPartsAndChildren. `layer`: 0 тело (head+rods, 32×32),
/// 1 ветер (wind_body, 128×128), 2 глаза (eyes, 32×32). Части вне выбранного слоя остаются без кубов.
pub fn breeze_mesh(layer: u8) -> Model {
    let mut m = Model::new(if layer == 1 {
        [128.0, 128.0]
    } else {
        [32.0, 32.0]
    });
    let none: [BoxDef; 0] = [];
    let body = m.add("body", None, Pose::offset(0.0, 0.0, 0.0), &none);
    let rods = m.add("rods", Some(body), Pose::offset(0.0, 8.0, 0.0), &none);
    let rod = |u: f32, v: f32| [BoxDef::new([u, v], [-1.0, 0.0, -3.0], [2.0, 8.0, 2.0])];
    let rpose = |x: f32, z: f32, xr: f32, yr: f32, zr: f32| Pose {
        x,
        y: -3.0,
        z,
        x_rot: xr,
        y_rot: yr,
        z_rot: zr,
    };
    let r = |on: bool| {
        if on {
            rod(0.0, 17.0).to_vec()
        } else {
            Vec::new()
        }
    };
    m.add(
        "rod_1",
        Some(rods),
        rpose(2.5981, 1.5, -2.7489, -1.0472, 3.1416),
        &r(layer == 0),
    );
    m.add(
        "rod_2",
        Some(rods),
        rpose(-2.5981, 1.5, -2.7489, 1.0472, 3.1416),
        &r(layer == 0),
    );
    m.add(
        "rod_3",
        Some(rods),
        rpose(0.0, -3.0, 0.3927, 0.0, 0.0),
        &r(layer == 0),
    );
    let head_boxes = || {
        vec![
            BoxDef::new([4.0, 24.0], [-5.0, -5.0, -4.2], [10.0, 3.0, 4.0]),
            BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]),
        ]
    };
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, 4.0, 0.0),
        &if layer == 0 { head_boxes() } else { Vec::new() },
    );
    m.add(
        "eyes",
        Some(head),
        Pose::offset(0.0, 0.0, 0.0),
        &if layer == 2 { head_boxes() } else { Vec::new() },
    );
    let wb = m.add("wind_body", None, Pose::offset(0.0, 0.0, 0.0), &none);
    let w = layer == 1;
    let wbot = m.add(
        "wind_bottom",
        Some(wb),
        Pose::offset(0.0, 24.0, 0.0),
        &if w {
            vec![BoxDef::new(
                [1.0, 83.0],
                [-2.5, -7.0, -2.5],
                [5.0, 7.0, 5.0],
            )]
        } else {
            Vec::new()
        },
    );
    let wmid = m.add(
        "wind_mid",
        Some(wbot),
        Pose::offset(0.0, -7.0, 0.0),
        &if w {
            vec![
                BoxDef::new([74.0, 28.0], [-6.0, -6.0, -6.0], [12.0, 6.0, 12.0]),
                BoxDef::new([78.0, 32.0], [-4.0, -6.0, -4.0], [8.0, 6.0, 8.0]),
                BoxDef::new([49.0, 71.0], [-2.5, -6.0, -2.5], [5.0, 6.0, 5.0]),
            ]
        } else {
            Vec::new()
        },
    );
    m.add(
        "wind_top",
        Some(wmid),
        Pose::offset(0.0, -6.0, 0.0),
        &if w {
            vec![
                BoxDef::new([0.0, 0.0], [-9.0, -8.0, -9.0], [18.0, 8.0, 18.0]),
                BoxDef::new([6.0, 6.0], [-6.0, -8.0, -6.0], [12.0, 8.0, 12.0]),
                BoxDef::new([105.0, 57.0], [-2.5, -8.0, -2.5], [5.0, 8.0, 5.0]),
            ]
        } else {
            Vec::new()
        },
    );
    m
}

/// VANILLA: BreezeModel#setupAnim. Времена (мс) — AnimationState (None = остановлено).
pub fn breeze_anim(
    m: &mut Model,
    idle: Option<i64>,
    shoot: Option<i64>,
    slide: Option<i64>,
    slide_back: Option<i64>,
    inhale: Option<i64>,
    long_jump: Option<i64>,
) {
    use crate::keyframes_data::{IDLE, INHALE, JUMP, SHOOT, SLIDE, SLIDE_BACK};
    for (a, t) in [
        (&*IDLE, idle),
        (&*SHOOT, shoot),
        (&*SLIDE, slide),
        (&*SLIDE_BACK, slide_back),
        (&*INHALE, inhale),
        (&*JUMP, long_jump),
    ] {
        if let Some(t) = t {
            a.apply(m, t, 1.0);
        }
    }
}

/// VANILLA: WardenModel#createBodyLayer (128×128).
pub fn warden_mesh() -> Model {
    let mut m = Model::new([128.0, 128.0]);
    let bone = m.add("bone", None, Pose::offset(0.0, 24.0, 0.0), &[]);
    let body = m.add(
        "body",
        Some(bone),
        Pose::offset(0.0, -21.0, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-9.0, -13.0, -4.0],
            [18.0, 21.0, 11.0],
        )],
    );
    m.add(
        "right_ribcage",
        Some(body),
        Pose::offset(-7.0, -2.0, -4.0),
        &[BoxDef::new(
            [90.0, 11.0],
            [-2.0, -11.0, -0.1],
            [9.0, 21.0, 0.0],
        )],
    );
    m.add(
        "left_ribcage",
        Some(body),
        Pose::offset(7.0, -2.0, -4.0),
        &[BoxDef::new([90.0, 11.0], [-7.0, -11.0, -0.1], [9.0, 21.0, 0.0]).mirror()],
    );
    let head = m.add(
        "head",
        Some(body),
        Pose::offset(0.0, -13.0, 0.0),
        &[BoxDef::new(
            [0.0, 32.0],
            [-8.0, -16.0, -5.0],
            [16.0, 16.0, 10.0],
        )],
    );
    m.add(
        "right_tendril",
        Some(head),
        Pose::offset(-8.0, -12.0, 0.0),
        &[BoxDef::new(
            [52.0, 32.0],
            [-16.0, -13.0, 0.0],
            [16.0, 16.0, 0.0],
        )],
    );
    m.add(
        "left_tendril",
        Some(head),
        Pose::offset(8.0, -12.0, 0.0),
        &[BoxDef::new(
            [58.0, 0.0],
            [0.0, -13.0, 0.0],
            [16.0, 16.0, 0.0],
        )],
    );
    m.add(
        "right_arm",
        Some(body),
        Pose::offset(-13.0, -13.0, 1.0),
        &[BoxDef::new(
            [44.0, 50.0],
            [-4.0, 0.0, -4.0],
            [8.0, 28.0, 8.0],
        )],
    );
    m.add(
        "left_arm",
        Some(body),
        Pose::offset(13.0, -13.0, 1.0),
        &[BoxDef::new(
            [0.0, 58.0],
            [-4.0, 0.0, -4.0],
            [8.0, 28.0, 8.0],
        )],
    );
    m.add(
        "right_leg",
        Some(bone),
        Pose::offset(-5.9, -13.0, 0.0),
        &[BoxDef::new(
            [76.0, 48.0],
            [-3.1, 0.0, -3.0],
            [6.0, 13.0, 6.0],
        )],
    );
    m.add(
        "left_leg",
        Some(bone),
        Pose::offset(5.9, -13.0, 0.0),
        &[BoxDef::new(
            [76.0, 76.0],
            [-2.9, 0.0, -3.0],
            [6.0, 13.0, 6.0],
        )],
    );
    m
}

/// Времена (мс) AnimationState варденa (None = остановлено).
#[derive(Default, Clone, Copy)]
pub struct WardenTimes {
    pub attack: Option<i64>,
    pub sonic_boom: Option<i64>,
    pub dig: Option<i64>,
    pub emerge: Option<i64>,
    pub roar: Option<i64>,
    pub sniff: Option<i64>,
}

/// VANILLA: WardenModel#setupAnim. `age` — ageInTicks, `tendril` — Warden#getTendrilAnimation.
pub fn warden_anim(
    m: &mut Model,
    (yr, xr): (f32, f32),
    wp: f32,
    ws: f32,
    age: f32,
    tendril: f32,
    t: &WardenTimes,
) {
    use crate::keyframes_data::{
        WARDEN_ATTACK, WARDEN_DIG, WARDEN_EMERGE, WARDEN_ROAR, WARDEN_SNIFF, WARDEN_SONIC_BOOM,
    };
    let p = |m: &mut Model, n: &str| m.index(n).unwrap_or(0);
    let (head, body, ll, rl, la, ra, lt, rt) = (
        p(m, "head"),
        p(m, "body"),
        p(m, "left_leg"),
        p(m, "right_leg"),
        p(m, "left_arm"),
        p(m, "right_arm"),
        p(m, "left_tendril"),
        p(m, "right_tendril"),
    );
    // animateHeadLookTarget
    m.parts[head].pose.x_rot = xr.to_radians();
    m.parts[head].pose.y_rot = yr.to_radians();
    // animateWalk
    let speed = (3.0 * ws).min(0.5);
    let pos = wp * 0.8662;
    let (c, s) = (pos.cos(), pos.sin());
    let speed_min = speed.min(0.35);
    m.parts[head].pose.z_rot += 0.3 * s * speed;
    m.parts[head].pose.x_rot += 1.2 * (pos + std::f32::consts::FRAC_PI_2).cos() * speed_min;
    m.parts[body].pose.z_rot = 0.1 * s * speed;
    m.parts[body].pose.x_rot = c * speed_min;
    m.parts[ll].pose.x_rot = c * speed;
    m.parts[rl].pose.x_rot = (pos + std::f32::consts::PI).cos() * speed;
    m.parts[la].pose.x_rot = -(0.8 * c * speed);
    m.parts[la].pose.z_rot = 0.0;
    m.parts[ra].pose.x_rot = -(0.8 * s * speed);
    m.parts[ra].pose.z_rot = 0.0;
    // resetArmPoses
    m.parts[la].pose.y_rot = 0.0;
    m.parts[la].pose.x = 13.0;
    m.parts[la].pose.y = -13.0;
    m.parts[la].pose.z = 1.0;
    m.parts[ra].pose.y_rot = 0.0;
    m.parts[ra].pose.x = -13.0;
    m.parts[ra].pose.y = -13.0;
    m.parts[ra].pose.z = 1.0;
    // animateIdlePose
    let a = age * 0.1;
    let (wc, wsn) = (a.cos(), a.sin());
    m.parts[head].pose.z_rot += 0.06 * wc;
    m.parts[head].pose.x_rot += 0.06 * wsn;
    m.parts[body].pose.z_rot += 0.025 * wsn;
    m.parts[body].pose.x_rot += 0.025 * wc;
    // animateTendrils
    let tx =
        tendril * ((f64::from(age) * 2.25).cos() * std::f64::consts::PI * f64::from(0.1f32)) as f32;
    m.parts[lt].pose.x_rot = tx;
    m.parts[rt].pose.x_rot = -tx;
    for (a, t) in [
        (&*WARDEN_ATTACK, t.attack),
        (&*WARDEN_SONIC_BOOM, t.sonic_boom),
        (&*WARDEN_DIG, t.dig),
        (&*WARDEN_EMERGE, t.emerge),
        (&*WARDEN_ROAR, t.roar),
        (&*WARDEN_SNIFF, t.sniff),
    ] {
        if let Some(t) = t {
            a.apply(m, t, 1.0);
        }
    }
}

/// VANILLA: CreakingModel#createBodyLayer (64×64).
pub fn creaking_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let root = m.add("root", None, Pose::offset(0.0, 24.0, 0.0), &[]);
    let ub = m.add(
        "upper_body",
        Some(root),
        Pose::offset(-1.0, -19.0, 0.0),
        &[],
    );
    m.add(
        "head",
        Some(ub),
        Pose::offset(-3.0, -11.0, 0.0),
        &[
            BoxDef::new([0.0, 0.0], [-3.0, -10.0, -3.0], [6.0, 10.0, 6.0]),
            BoxDef::new([28.0, 31.0], [-3.0, -13.0, -3.0], [6.0, 3.0, 6.0]),
            BoxDef::new([12.0, 40.0], [3.0, -13.0, 0.0], [9.0, 14.0, 0.0]),
            BoxDef::new([34.0, 12.0], [-12.0, -14.0, 0.0], [9.0, 14.0, 0.0]),
        ],
    );
    m.add(
        "body",
        Some(ub),
        Pose::offset(0.0, -7.0, 1.0),
        &[
            BoxDef::new([0.0, 16.0], [0.0, -3.0, -3.0], [6.0, 13.0, 5.0]),
            BoxDef::new([24.0, 0.0], [-6.0, -4.0, -3.0], [6.0, 7.0, 5.0]),
        ],
    );
    m.add(
        "right_arm",
        Some(ub),
        Pose::offset(-7.0, -9.5, 1.5),
        &[
            BoxDef::new([22.0, 13.0], [-2.0, -1.5, -1.5], [3.0, 21.0, 3.0]),
            BoxDef::new([46.0, 0.0], [-2.0, 19.5, -1.5], [3.0, 4.0, 3.0]),
        ],
    );
    m.add(
        "left_arm",
        Some(ub),
        Pose::offset(6.0, -9.0, 0.5),
        &[
            BoxDef::new([30.0, 40.0], [0.0, -1.0, -1.5], [3.0, 16.0, 3.0]),
            BoxDef::new([52.0, 12.0], [0.0, -5.0, -1.5], [3.0, 4.0, 3.0]),
            BoxDef::new([52.0, 19.0], [0.0, 15.0, -1.5], [3.0, 4.0, 3.0]),
        ],
    );
    m.add(
        "left_leg",
        Some(root),
        Pose::offset(1.5, -16.0, 0.5),
        &[
            BoxDef::new([42.0, 40.0], [-1.5, 0.0, -1.5], [3.0, 16.0, 3.0]),
            BoxDef::new([45.0, 55.0], [-1.5, 15.7, -4.5], [5.0, 0.0, 9.0]),
        ],
    );
    m.add(
        "right_leg",
        Some(root),
        Pose::offset(-1.0, -17.5, 0.5),
        &[
            BoxDef::new([0.0, 34.0], [-3.0, -1.5, -1.5], [3.0, 19.0, 3.0]),
            BoxDef::new([45.0, 46.0], [-5.0, 17.2, -4.5], [5.0, 0.0, 9.0]),
            BoxDef::new([12.0, 34.0], [-3.0, -4.5, -1.5], [3.0, 3.0, 3.0]),
        ],
    );
    m
}

/// VANILLA: CreakingModel#setupAnim. `attack/invuln/death` — мс от старта AnimationState.
#[allow(clippy::too_many_arguments)]
pub fn creaking_anim(
    m: &mut Model,
    (yr, xr): (f32, f32),
    (wp, ws): (f32, f32),
    can_move: bool,
    attack: Option<i64>,
    invuln: Option<i64>,
    death: Option<i64>,
) {
    use crate::keyframes_data::{
        CREAKING_ATTACK, CREAKING_DEATH, CREAKING_INVULNERABLE, CREAKING_WALK,
    };
    if let Some(i) = m.index("head") {
        m.parts[i].pose.x_rot = xr.to_radians();
        m.parts[i].pose.y_rot = yr.to_radians();
    }
    if can_move {
        CREAKING_WALK.apply_walk(m, wp, ws, 1.0, 1.0);
    }
    for (a, t) in [
        (&*CREAKING_ATTACK, attack),
        (&*CREAKING_INVULNERABLE, invuln),
        (&*CREAKING_DEATH, death),
    ] {
        if let Some(t) = t {
            a.apply(m, t, 1.0);
        }
    }
}

/// VANILLA: ShulkerModel#createBodyLayer (64×64).
pub fn shulker_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "lid",
        None,
        Pose::offset(0.0, 24.0, 0.0),
        &[BoxDef::new(
            [0.0, 0.0],
            [-8.0, -16.0, -8.0],
            [16.0, 12.0, 16.0],
        )],
    );
    m.add(
        "base",
        None,
        Pose::offset(0.0, 24.0, 0.0),
        &[BoxDef::new(
            [0.0, 28.0],
            [-8.0, -8.0, -8.0],
            [16.0, 8.0, 16.0],
        )],
    );
    m.add(
        "head",
        None,
        Pose::offset(0.0, 12.0, 0.0),
        &[BoxDef::new([0.0, 52.0], [-3.0, 0.0, -3.0], [6.0, 6.0, 6.0])],
    );
    m
}

/// VANILLA: ShulkerModel#setupAnim. `head_yaw` = yHeadRot − 180 − yBodyRot (градусы).
pub fn shulker_anim(m: &mut Model, xr: f32, head_yaw: f32, peek: f32, age: f32) {
    let bs = (0.5 + peek) * std::f32::consts::PI;
    let q = -1.0 + bs.sin();
    let extra = if bs > std::f32::consts::PI {
        (age * 0.1).sin() * 0.7
    } else {
        0.0
    };
    if let Some(i) = m.index("lid") {
        m.parts[i].pose.y = 16.0 + bs.sin() * 8.0 + extra;
        m.parts[i].pose.y_rot = if peek > 0.3 {
            q * q * q * q * std::f32::consts::PI * 0.125
        } else {
            0.0
        };
    }
    if let Some(i) = m.index("head") {
        m.parts[i].pose.x_rot = xr.to_radians();
        m.parts[i].pose.y_rot = head_yaw.to_radians();
    }
}

/// VANILLA: WitherBossModel#createBodyLayer(g) (64×64). `grow` = 0 (тело) или 0.5 (WITHER_ARMOR).
pub fn wither_mesh(grow: f32) -> Model {
    let mut m = Model::new([64.0, 64.0]);
    m.add(
        "shoulders",
        None,
        Pose::offset(0.0, 0.0, 0.0),
        &[BoxDef::new([0.0, 16.0], [-10.0, 3.9, -0.5], [20.0, 3.0, 3.0]).grow(grow)],
    );
    let rib_x = 0.204_203_52_f32;
    let rib = BoxDef::new([24.0, 22.0], [-4.0, 1.5, 0.5], [11.0, 2.0, 2.0]).grow(grow);
    m.add(
        "ribcage",
        None,
        Pose {
            x: -2.0,
            y: 6.9,
            z: -0.5,
            x_rot: rib_x,
            y_rot: 0.0,
            z_rot: 0.0,
        },
        &[
            BoxDef::new([0.0, 22.0], [0.0, 0.0, 0.0], [3.0, 10.0, 3.0]).grow(grow),
            rib,
            BoxDef::new([24.0, 22.0], [-4.0, 4.0, 0.5], [11.0, 2.0, 2.0]).grow(grow),
            BoxDef::new([24.0, 22.0], [-4.0, 6.5, 0.5], [11.0, 2.0, 2.0]).grow(grow),
        ],
    );
    m.add(
        "tail",
        None,
        Pose {
            x: -2.0,
            y: 6.9 + rib_x.cos() * 10.0,
            z: -0.5 + rib_x.sin() * 10.0,
            x_rot: 0.832_522_03,
            y_rot: 0.0,
            z_rot: 0.0,
        },
        &[BoxDef::new([12.0, 22.0], [0.0, 0.0, 0.0], [3.0, 6.0, 3.0]).grow(grow)],
    );
    m.add(
        "center_head",
        None,
        Pose::offset(0.0, 0.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -4.0, -4.0], [8.0, 8.0, 8.0]).grow(grow)],
    );
    let side = BoxDef::new([32.0, 0.0], [-4.0, -4.0, -4.0], [6.0, 6.0, 6.0]).grow(grow);
    m.add("right_head", None, Pose::offset(-8.0, 4.0, 0.0), &[side]);
    m.add("left_head", None, Pose::offset(10.0, 4.0, 0.0), &[side]);
    m
}

/// VANILLA: WitherBossModel#setupAnim. `heads` = (yHeadRots, xHeadRots) в градусах, `body_rot` — градусы.
pub fn wither_anim(
    m: &mut Model,
    age: f32,
    (y_rot, x_rot): (f32, f32),
    body_rot: f32,
    y_heads: [f32; 2],
    x_heads: [f32; 2],
) {
    m.reset_pose();
    let d2r = PI / 180.0;
    for (i, name) in ["right_head", "left_head"].into_iter().enumerate() {
        if let Some(h) = m.index(name) {
            m.parts[h].pose.y_rot = (y_heads[i] - body_rot) * d2r;
            m.parts[h].pose.x_rot = x_heads[i] * d2r;
        }
    }
    let anim = (age * 0.1).cos();
    let rib_x = (0.065 + 0.05 * anim) * PI;
    if let Some(r) = m.index("ribcage") {
        m.parts[r].pose.x_rot = rib_x;
    }
    if let Some(t) = m.index("tail") {
        let p = &mut m.parts[t].pose;
        p.x = -2.0;
        p.y = 6.9 + rib_x.cos() * 10.0;
        p.z = -0.5 + rib_x.sin() * 10.0;
        p.x_rot = (0.265 + 0.1 * anim) * PI;
    }
    if let Some(c) = m.index("center_head") {
        m.parts[c].pose.y_rot = y_rot * d2r;
        m.parts[c].pose.x_rot = x_rot * d2r;
    }
}

const DRAGON_NECK: [&str; 5] = ["neck0", "neck1", "neck2", "neck3", "neck4"];
const DRAGON_TAIL: [&str; 12] = [
    "tail0", "tail1", "tail2", "tail3", "tail4", "tail5", "tail6", "tail7", "tail8", "tail9",
    "tail10", "tail11",
];

/// VANILLA: EnderDragonModel#createBodyLayer (256×256). Зеркало `.mirror()` в эталоне не сбрасывается
/// (оба «scale»/«nostril» головы зеркальны), как в исходнике.
pub fn ender_dragon_mesh() -> Model {
    let mut m = Model::new([256.0, 256.0]);
    let b = BoxDef::new;
    let head = m.add(
        "head",
        None,
        Pose::offset(0.0, 20.0, -62.0),
        &[
            b([176.0, 44.0], [-6.0, -1.0, -24.0], [12.0, 5.0, 16.0]),
            b([112.0, 30.0], [-8.0, -8.0, -10.0], [16.0, 16.0, 16.0]),
            b([0.0, 0.0], [-5.0, -12.0, -4.0], [2.0, 4.0, 6.0]).mirror(),
            b([112.0, 0.0], [-5.0, -3.0, -22.0], [2.0, 2.0, 4.0]).mirror(),
            b([0.0, 0.0], [3.0, -12.0, -4.0], [2.0, 4.0, 6.0]).mirror(),
            b([112.0, 0.0], [3.0, -3.0, -22.0], [2.0, 2.0, 4.0]).mirror(),
        ],
    );
    m.add(
        "jaw",
        Some(head),
        Pose::offset(0.0, 4.0, -8.0),
        &[b([176.0, 65.0], [-6.0, 0.0, -16.0], [12.0, 4.0, 16.0])],
    );
    let spine = [
        b([192.0, 104.0], [-5.0, -5.0, -5.0], [10.0, 10.0, 10.0]),
        b([48.0, 0.0], [-1.0, -9.0, -3.0], [2.0, 4.0, 6.0]),
    ];
    for (i, n) in DRAGON_NECK.into_iter().enumerate() {
        m.add(
            n,
            None,
            Pose::offset(0.0, 20.0, -12.0 - i as f32 * 10.0),
            &spine,
        );
    }
    for (i, n) in DRAGON_TAIL.into_iter().enumerate() {
        m.add(
            n,
            None,
            Pose::offset(0.0, 10.0, 60.0 + i as f32 * 10.0),
            &spine,
        );
    }
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 20.0, 8.0),
        &[
            b([0.0, 0.0], [-12.0, -16.0, -16.0], [24.0, 24.0, 64.0]),
            b([220.0, 53.0], [-1.0, -22.0, -10.0], [2.0, 6.0, 12.0]),
            b([220.0, 53.0], [-1.0, -22.0, 10.0], [2.0, 6.0, 12.0]),
            b([220.0, 53.0], [-1.0, -22.0, 30.0], [2.0, 6.0, 12.0]),
        ],
    );
    let rot = |x: f32, y: f32, z: f32, xr: f32| Pose {
        x,
        y,
        z,
        x_rot: xr,
        y_rot: 0.0,
        z_rot: 0.0,
    };
    // Крылья: левое зеркальное, правое нет.
    let lw = m.add(
        "left_wing",
        Some(body),
        Pose::offset(12.0, -15.0, -6.0),
        &[
            b([112.0, 88.0], [0.0, -4.0, -4.0], [56.0, 8.0, 8.0]).mirror(),
            b([-56.0, 88.0], [0.0, 0.0, 2.0], [56.0, 0.0, 56.0]).mirror(),
        ],
    );
    m.add(
        "left_wing_tip",
        Some(lw),
        Pose::offset(56.0, 0.0, 0.0),
        &[
            b([112.0, 136.0], [0.0, -2.0, -2.0], [56.0, 4.0, 4.0]).mirror(),
            b([-56.0, 144.0], [0.0, 0.0, 2.0], [56.0, 0.0, 56.0]).mirror(),
        ],
    );
    let front = [b([112.0, 104.0], [-4.0, -4.0, -4.0], [8.0, 24.0, 8.0])];
    let front_tip = [b([226.0, 138.0], [-3.0, -1.0, -3.0], [6.0, 24.0, 6.0])];
    let front_foot = [b([144.0, 104.0], [-4.0, 0.0, -12.0], [8.0, 4.0, 16.0])];
    let hind = [b([0.0, 0.0], [-8.0, -4.0, -8.0], [16.0, 32.0, 16.0])];
    let hind_tip = [b([196.0, 0.0], [-6.0, -2.0, 0.0], [12.0, 32.0, 12.0])];
    let hind_foot = [b([112.0, 0.0], [-9.0, 0.0, -20.0], [18.0, 6.0, 24.0])];
    let l = m.add(
        "left_front_leg",
        Some(body),
        rot(12.0, 0.0, -6.0, 1.3),
        &front,
    );
    let l = m.add(
        "left_front_leg_tip",
        Some(l),
        rot(0.0, 20.0, -1.0, -0.5),
        &front_tip,
    );
    m.add(
        "left_front_foot",
        Some(l),
        rot(0.0, 23.0, 0.0, 0.75),
        &front_foot,
    );
    let l = m.add(
        "left_hind_leg",
        Some(body),
        rot(16.0, -4.0, 34.0, 1.0),
        &hind,
    );
    let l = m.add(
        "left_hind_leg_tip",
        Some(l),
        rot(0.0, 32.0, -4.0, 0.5),
        &hind_tip,
    );
    m.add(
        "left_hind_foot",
        Some(l),
        rot(0.0, 31.0, 4.0, 0.75),
        &hind_foot,
    );
    let rw = m.add(
        "right_wing",
        Some(body),
        Pose::offset(-12.0, -15.0, -6.0),
        &[
            b([112.0, 88.0], [-56.0, -4.0, -4.0], [56.0, 8.0, 8.0]),
            b([-56.0, 88.0], [-56.0, 0.0, 2.0], [56.0, 0.0, 56.0]),
        ],
    );
    m.add(
        "right_wing_tip",
        Some(rw),
        Pose::offset(-56.0, 0.0, 0.0),
        &[
            b([112.0, 136.0], [-56.0, -2.0, -2.0], [56.0, 4.0, 4.0]),
            b([-56.0, 144.0], [-56.0, 0.0, 2.0], [56.0, 0.0, 56.0]),
        ],
    );
    let r = m.add(
        "right_front_leg",
        Some(body),
        rot(-12.0, 0.0, -6.0, 1.3),
        &front,
    );
    let r = m.add(
        "right_front_leg_tip",
        Some(r),
        rot(0.0, 20.0, -1.0, -0.5),
        &front_tip,
    );
    m.add(
        "right_front_foot",
        Some(r),
        rot(0.0, 23.0, 0.0, 0.75),
        &front_foot,
    );
    let r = m.add(
        "right_hind_leg",
        Some(body),
        rot(-16.0, -4.0, 34.0, 1.0),
        &hind,
    );
    let r = m.add(
        "right_hind_leg_tip",
        Some(r),
        rot(0.0, 32.0, -4.0, 0.5),
        &hind_tip,
    );
    m.add(
        "right_hind_foot",
        Some(r),
        rot(0.0, 31.0, 4.0, 0.75),
        &hind_foot,
    );
    m
}

/// Фаза дракона для `ender_dragon_anim` (EnderDragonRenderState).
#[derive(Clone, Copy, Default)]
pub struct DragonPhase {
    pub landing_or_takeoff: bool,
    pub sitting: bool,
    /// Квадрат расстояния до центра яйца (distToCenterSqr).
    pub distance_to_egg: f64,
}

/// VANILLA: Mth#sin/cos — табличные (long-каст аргумента насыщается: у гигантского flapTime поза
/// замирает, а не мерцает, как у `f32::sin`).
trait MthTrig {
    fn msin(self) -> f32;
    fn mcos(self) -> f32;
}
impl MthTrig for f32 {
    fn msin(self) -> f32 {
        mth_table((f64::from(self) * MTH_SIN_SCALE) as i64)
    }
    fn mcos(self) -> f32 {
        mth_table((f64::from(self) * MTH_SIN_SCALE + 16384.0) as i64)
    }
}
const MTH_SIN_SCALE: f64 = 10_430.378_350_470_453;
fn mth_table(i: i64) -> f32 {
    static T: std::sync::OnceLock<Vec<f32>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        (0..65536)
            .map(|i| (f64::from(i) / MTH_SIN_SCALE).sin() as f32)
            .collect()
    })[(i & 65535) as usize]
}

/// VANILLA: EnderDragonModel#setupAnim. `hist(delay)` = DragonFlightHistory#get(delay, partialTicks)
/// → (y, yRot°). Возвращает позу корня модели (root.y/z/xRot) как матрицу для умножения справа на root.
pub fn ender_dragon_anim(
    m: &mut Model,
    flap_time: f32,
    hist: &dyn Fn(i32) -> (f64, f32),
    ph: DragonPhase,
) -> glam::Mat4 {
    m.reset_pose();
    let d2r = PI / 180.0;
    let wrap = |a: f32| {
        let mut d = a % 360.0;
        if d >= 180.0 {
            d -= 360.0;
        }
        if d < -180.0 {
            d += 360.0;
        }
        d
    };
    let flap = flap_time * (PI * 2.0);
    let jaw = (flap.msin() + 1.0) * 0.2;
    let mut bounce = (flap - 1.0).msin() + 1.0;
    bounce = (bounce * bounce + bounce * 2.0) * 0.05;
    // VANILLA: EnderDragonRenderState#getHeadPartYOffset.
    let head_off = |part: i32, body_y: f64, part_y: f64| -> f32 {
        (if ph.landing_or_takeoff {
            f64::from(part) / (ph.distance_to_egg / 4.0).max(1.0)
        } else if ph.sitting {
            f64::from(part)
        } else if part == 6 {
            0.0
        } else {
            part_y - body_y
        }) as f32
    };
    let (start_y, start_rot) = hist(6);
    let rot2 = wrap(hist(5).1 - hist(10).1);
    let rot = wrap(hist(5).1 + rot2 / 2.0);
    let pose_of = |m: &mut Model, name: &str| m.index(name);
    if let Some(j) = pose_of(m, "jaw") {
        m.parts[j].pose.x_rot = jaw;
    }
    let n0 = m.index("neck0").map_or([0.0, 20.0, -12.0], |i| {
        let p = m.parts[i].pose;
        [p.x, p.y, p.z]
    });
    let (mut xx, mut yy, mut zz) = (n0[0], n0[1], n0[2]);
    for (i, name) in DRAGON_NECK.into_iter().enumerate() {
        let (py, pr) = hist(5 - i as i32);
        let neck_x = (i as f32 * 0.45 + flap).mcos() * 0.15;
        if let Some(n) = m.index(name) {
            let p = &mut m.parts[n].pose;
            p.y_rot = wrap(pr - start_rot) * d2r * 1.5;
            p.x_rot = neck_x + head_off(i as i32, start_y, py) * d2r * 1.5 * 5.0;
            p.z_rot = -wrap(pr - rot) * d2r * 1.5;
            p.y = yy;
            p.z = zz;
            p.x = xx;
            xx -= p.y_rot.msin() * p.x_rot.mcos() * 10.0;
            yy += p.x_rot.msin() * 10.0;
            zz -= p.y_rot.mcos() * p.x_rot.mcos() * 10.0;
        }
    }
    let (cur_y, cur_rot) = hist(0);
    if let Some(h) = m.index("head") {
        let p = &mut m.parts[h].pose;
        p.y = yy;
        p.z = zz;
        p.x = xx;
        p.y_rot = wrap(cur_rot - start_rot) * d2r;
        p.x_rot = wrap(head_off(6, start_y, cur_y)) * d2r * 1.5 * 5.0;
        p.z_rot = -wrap(cur_rot - rot) * d2r;
    }
    let set = |m: &mut Model, name: &str, f: &dyn Fn(&mut Pose)| {
        if let Some(i) = m.index(name) {
            f(&mut m.parts[i].pose);
        }
    };
    set(m, "body", &|p| p.z_rot = -rot2 * 1.5 * d2r);
    let (lx, ly, lz) = (
        0.125 - flap.mcos() * 0.2,
        -0.25,
        -(flap.msin() + 0.125) * 0.8,
    );
    let tip = ((flap + 2.0).msin() + 0.5) * 0.75;
    set(m, "left_wing", &|p| {
        p.x_rot = lx;
        p.y_rot = ly;
        p.z_rot = lz;
    });
    set(m, "left_wing_tip", &|p| p.z_rot = tip);
    set(m, "right_wing", &|p| {
        p.x_rot = lx;
        p.y_rot = -ly;
        p.z_rot = -lz;
    });
    set(m, "right_wing_tip", &|p| p.z_rot = -tip);
    for side in ["left", "right"] {
        for (leg, base) in [
            ("front_leg", 1.3),
            ("front_foot", 0.75),
            ("hind_leg", 1.0),
            ("hind_leg_tip", 0.5),
            ("hind_foot", 0.75),
        ] {
            let name = format!("{side}_{leg}");
            if let Some(i) = m.index(&name) {
                m.parts[i].pose.x_rot = base + bounce * 0.1;
            }
        }
        let name = format!("{side}_front_leg_tip");
        if let Some(i) = m.index(&name) {
            m.parts[i].pose.x_rot = -0.5 - bounce * 0.1;
        }
    }
    let mut tail_x = 0.0_f32;
    let t0 = m.index("tail0").map_or([0.0, 10.0, 60.0], |i| {
        let p = m.parts[i].pose;
        [p.x, p.y, p.z]
    });
    let (mut xx, mut yy, mut zz) = (t0[0], t0[1], t0[2]);
    let (y11, rot11) = hist(11);
    for (i, name) in DRAGON_TAIL.into_iter().enumerate() {
        let (py, pr) = hist(12 + i as i32);
        tail_x += (i as f32 * 0.45 + flap).msin() * 0.05;
        if let Some(t) = m.index(name) {
            let p = &mut m.parts[t].pose;
            p.y_rot = (wrap(pr - rot11) * 1.5 + 180.0) * d2r;
            p.x_rot = tail_x + (py - y11) as f32 * d2r * 1.5 * 5.0;
            p.z_rot = wrap(pr - rot) * d2r * 1.5;
            p.y = yy;
            p.z = zz;
            p.x = xx;
            yy += p.x_rot.msin() * 10.0;
            zz -= p.y_rot.mcos() * p.x_rot.mcos() * 10.0;
            xx -= p.y_rot.msin() * p.x_rot.mcos() * 10.0;
        }
    }
    Pose {
        x: 0.0,
        y: (bounce - 2.0) * 16.0,
        z: -48.0,
        x_rot: bounce * 2.0 * d2r,
        y_rot: 0.0,
        z_rot: 0.0,
    }
    .matrix()
}

/// VANILLA: CopperGolemModel#createBodyLayer (64×64; MeshDefinition сдвинут на y+24).
pub fn copper_golem_mesh() -> Model {
    let mut m = Model::new([64.0, 64.0]);
    let body = m.add(
        "body",
        None,
        Pose::offset(0.0, 19.0, 0.0),
        &[BoxDef::new(
            [0.0, 15.0],
            [-4.0, -6.0, -3.0],
            [8.0, 6.0, 6.0],
        )],
    );
    m.add(
        "head",
        Some(body),
        Pose::offset(0.0, -6.0, 0.0),
        &[
            BoxDef::new([0.0, 0.0], [-4.0, -5.0, -5.0], [8.0, 5.0, 10.0]).grow(0.015),
            BoxDef::new([56.0, 0.0], [-1.0, -2.0, -6.0], [2.0, 3.0, 2.0]),
            BoxDef::new([37.0, 8.0], [-1.0, -9.0, -1.0], [2.0, 4.0, 2.0]).grow(-0.015),
            BoxDef::new([37.0, 0.0], [-2.0, -13.0, -2.0], [4.0, 4.0, 4.0]).grow(-0.015),
        ],
    );
    m.add(
        "right_arm",
        Some(body),
        Pose::offset(-4.0, -6.0, 0.0),
        &[BoxDef::new(
            [36.0, 16.0],
            [-3.0, -1.0, -2.0],
            [3.0, 10.0, 4.0],
        )],
    );
    m.add(
        "left_arm",
        Some(body),
        Pose::offset(4.0, -6.0, 0.0),
        &[BoxDef::new(
            [50.0, 16.0],
            [0.0, -1.0, -2.0],
            [3.0, 10.0, 4.0],
        )],
    );
    m.add(
        "right_leg",
        None,
        Pose::offset(0.0, 19.0, 0.0),
        &[BoxDef::new([0.0, 27.0], [-4.0, 0.0, -2.0], [4.0, 5.0, 4.0])],
    );
    m.add(
        "left_leg",
        None,
        Pose::offset(0.0, 19.0, 0.0),
        &[BoxDef::new([16.0, 27.0], [0.0, 0.0, -2.0], [4.0, 5.0, 4.0])],
    );
    m
}

/// VANILLA: CopperGolemModel#setupAnim — голова по взгляду, ходьба (с предметом в руках — своя анимация,
/// руки не выше −0.8727 по X и т. д.) + idle и interaction-анимации.
#[allow(clippy::too_many_arguments)]
pub fn copper_golem_anim(
    m: &mut Model,
    yr: f32,
    xr: f32,
    wp: f32,
    ws: f32,
    holding: bool,
    idle: Option<i64>,
    inter: Option<(i32, i64)>,
) {
    use crate::keyframes_data::{
        COPPER_GOLEM_CHEST_INTERACTION_ITEM_DROP as DROP,
        COPPER_GOLEM_CHEST_INTERACTION_ITEM_NODROP as NODROP,
        COPPER_GOLEM_CHEST_INTERACTION_NOITEM_GET as GET,
        COPPER_GOLEM_CHEST_INTERACTION_NOITEM_NOGET as NOGET, COPPER_GOLEM_IDLE, COPPER_GOLEM_WALK,
        COPPER_GOLEM_WALK_ITEM,
    };
    set_head(m, yr, xr);
    if holding {
        COPPER_GOLEM_WALK_ITEM.apply_walk(m, wp, ws, 2.0, 2.5);
        for (n, side) in [("right_arm", -1.0f32), ("left_arm", 1.0)] {
            if let Some(i) = m.index(n) {
                let p = &mut m.parts[i].pose;
                p.x_rot = p.x_rot.min(-0.872_664_63);
                if side < 0.0 {
                    p.y_rot = p.y_rot.min(-0.113_446_4);
                    p.z_rot = p.z_rot.min(-0.064_577_185);
                } else {
                    p.y_rot = p.y_rot.max(0.113_446_4);
                    p.z_rot = p.z_rot.max(0.064_577_185);
                }
            }
        }
    } else {
        COPPER_GOLEM_WALK.apply_walk(m, wp, ws, 2.0, 2.5);
    }
    if let Some(t) = idle {
        COPPER_GOLEM_IDLE.apply(m, t, 1.0);
    }
    // VANILLA: CopperGolemState 1 getting_item → NOITEM_GET, 2 → NOITEM_NOGET, 3 dropping_item → ITEM_DROP, 4 → ITEM_NODROP.
    if let Some((st, t)) = inter {
        match st {
            1 => GET.apply(m, t, 1.0),
            2 => NOGET.apply(m, t, 1.0),
            3 => DROP.apply(m, t, 1.0),
            4 => NODROP.apply(m, t, 1.0),
            _ => {}
        }
    }
}
