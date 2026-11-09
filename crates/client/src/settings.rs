//! Применение options.txt к игре: снимок `ViewCfg` для рендера и `ClientInformation` для сервера.
//!
//! VANILLA: Options (fov, gamma, bobView, entityShadows, particles, graphicsMode, mainHand,
//! hideLightningFlashes, renderDistance, modelPart_*, chatColors).
use crate::ui::options::{Def, Options, ACCESSIBILITY, CHAT, SKIN, VIDEO};
use ezzmc_net::session::ClientInformation;

/// Снимок настроек, влияющих на кадр мира. Дёшево копируется, обновляется каждый кадр.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewCfg {
    pub fov: f32,
    /// renderDistance из настроек (или EZZMC_RENDER_DISTANCE): ограничивает радиус тумана и дальней плоскости.
    pub render_distance: i32,
    pub gamma: f32,
    pub bob_view: bool,
    pub entity_shadows: bool,
    /// 0 = all, 1 = decreased, 2 = minimal.
    pub particles: u8,
    /// graphicsMode: 0 = fast, 1 = fancy, 2 = fabulous.
    pub graphics: u8,
    pub left_handed: bool,
    pub hide_lightning_flashes: bool,
    /// Маска PlayerModelPart (бит 0 cape … бит 6 hat).
    pub parts: u8,
    pub sprint_window: u8,
    pub auto_jump: bool,
}

impl Default for ViewCfg {
    fn default() -> Self {
        Self {
            fov: 70.0,
            render_distance: 12,
            gamma: 0.5,
            bob_view: true,
            entity_shadows: true,
            particles: 0,
            graphics: 1,
            left_handed: false,
            hide_lightning_flashes: false,
            parts: 0x7F,
            sprint_window: 7,
            auto_jump: false,
        }
    }
}

fn find(list: &'static [Def], key: &str) -> &'static Def {
    list.iter().find(|d| d.key == key).expect("ключ настройки")
}

impl ViewCfg {
    pub fn from_options(o: &Options) -> Self {
        let b = |l: &'static [Def], k: &str| o.get(find(l, k)) == "true";
        Self {
            fov: o.f32(&crate::ui::options::FOV),
            render_distance: render_distance(o),
            gamma: o.f32(find(VIDEO, "gamma")),
            bob_view: b(VIDEO, "bobView"),
            entity_shadows: b(VIDEO, "entityShadows"),
            particles: o.get(find(VIDEO, "particles")).parse().unwrap_or(0),
            graphics: o.get(find(VIDEO, "graphicsMode")).parse().unwrap_or(1),
            left_handed: o.get(find(SKIN, "mainHand")) == "left",
            hide_lightning_flashes: b(ACCESSIBILITY, "hideLightningFlashes"),
            parts: model_mask(o),
            sprint_window: o
                .f32(find(crate::ui::options::CONTROLS, "sprintWindow"))
                .round()
                .clamp(0.0, 10.0) as u8,
            auto_jump: b(crate::ui::options::CONTROLS, "autoJump"),
        }
    }
}

/// VANILLA: PlayerModelPart — cape 0, jacket 1, left_sleeve 2, right_sleeve 3, left_pants 4,
/// right_pants 5, hat 6 (порядок SKIN совпадает).
fn model_mask(o: &Options) -> u8 {
    let mut mask = 0u8;
    for (i, d) in SKIN.iter().take(7).enumerate() {
        if o.get(d) == "true" {
            mask |= 1 << i;
        }
    }
    mask
}

/// Видимость внешних слоёв модели игрока по маске.
pub fn apply_parts(m: &mut ezzmc_render::entitymodel::Model, mask: u8) {
    for p in &mut m.parts {
        let bit = match p.name {
            "jacket" => 1,
            "left_sleeve" => 2,
            "right_sleeve" => 3,
            "left_pants" => 4,
            "right_pants" => 5,
            "hat" => 6,
            _ => continue,
        };
        p.visible = mask & (1 << bit) != 0;
    }
}

/// VANILLA: Options#buildPlayerInformation.
/// renderDistance клиента (чанки): переменная EZZMC_RENDER_DISTANCE перекрывает настройку.
pub fn render_distance(o: &Options) -> i32 {
    std::env::var("EZZMC_RENDER_DISTANCE")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or_else(|| o.get(&VIDEO[0]).parse::<i32>().unwrap_or(12))
        .clamp(2, 127)
}

pub fn client_info(o: &Options) -> ClientInformation {
    let mask = model_mask(o);
    ClientInformation {
        view_distance: render_distance(o) as i8,
        chat_colors: o.get(&CHAT[0]) == "true",
        model_customisation: mask,
        main_hand: i32::from(o.get(find(SKIN, "mainHand")) != "left"),
        particle_status: o.get(find(VIDEO, "particles")).parse().unwrap_or(0),
        ..ClientInformation::default()
    }
}
