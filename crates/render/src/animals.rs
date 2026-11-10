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
pub fn creeper_mesh() -> Model {
    let mut m = Model::new([64.0, 32.0]);
    m.add(
        "head",
        None,
        Pose::offset(0.0, 6.0, 0.0),
        &[BoxDef::new([0.0, 0.0], [-4.0, -8.0, -4.0], [8.0, 8.0, 8.0])],
    );
    m.add(
        "body",
        None,
        Pose::offset(0.0, 6.0, 0.0),
        &[BoxDef::new(
            [16.0, 16.0],
            [-4.0, 0.0, -2.0],
            [8.0, 12.0, 4.0],
        )],
    );
    let leg = BoxDef::new([0.0, 16.0], [-2.0, 0.0, -2.0], [4.0, 6.0, 4.0]);
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

/// VANILLA: VillagerModel#createBodyModel (64×64) без шляпы (hat/hat_rim — слой профессии).
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

/// VANILLA: VillagerModel#setupAnim (без isUnhappy).
pub fn villager_anim(m: &mut Model, y_rot: f32, x_rot: f32, walk_pos: f32, walk_speed: f32) {
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
    m.add(
        "head",
        None,
        Pose::default(),
        &[
            BoxDef::new([0.0, 0.0], [-4.0, -10.0, -4.0], [8.0, 10.0, 8.0]),
            BoxDef::new([24.0, 0.0], [-1.0, -3.0, -6.0], [2.0, 4.0, 2.0]),
        ],
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

/// Сдвиг модели кальмара по Y: SquidRenderer#setupRotations (translate 0.5 до и −1.2 после поворота) = 0.7 блока вниз.
const SQUID_DY: f32 = 11.2;

/// VANILLA: SquidModel#createBodyLayer (64×32), с учётом сдвига setupRotations (см. SQUID_DY).
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
        Pose::offset(0.0, 8.0 + SQUID_DY, 0.0),
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
                ..Pose::offset(a.cos() * 5.0, 15.0 + SQUID_DY, a.sin() * 5.0)
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

/// VANILLA: SquidModel#setupAnim (tentacle.xRot = tentacleAngle).
// VANILLA-UNSURE: tentacleAngle берётся упрощённо по возрасту (Squid#aiStep не портирован), без наклона по движению.
pub fn squid_anim(m: &mut Model, age: f32) {
    let t = age * 0.15;
    let ang = if t.rem_euclid(PI * 2.0) < PI {
        let f = t.rem_euclid(PI * 2.0) / PI;
        (f * f).sin() * PI * 0.25
    } else {
        PI * 0.25
    };
    for n in SQUID_TENTACLES {
        set_x(m, n, ang);
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

/// VANILLA: GuardianModel#setupAnim (взгляд глаза не портирован).
/// `tail` = tailAnimation, `spikes` = spikesAnimation (Guardian#aiStep).
pub fn guardian_anim(m: &mut Model, y_rot: f32, x_rot: f32, age: f32, tail: f32, spikes: f32) {
    set_head(m, y_rot, x_rot);
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
// VANILLA-UNSURE: flapAngle в полёте — синус по возрасту (Parrot#aiStep flap не портирован); сидение/вечеринка/плечо не различаются.
pub fn parrot_anim(
    m: &mut Model,
    y_rot: f32,
    x_rot: f32,
    pos: f32,
    speed: f32,
    age: f32,
    flying: bool,
) {
    set_head(m, y_rot, x_rot);
    let flap = if flying {
        (age * 0.9).sin() * 0.5 + 0.5
    } else {
        0.0
    };
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
