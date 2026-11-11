//! Иконки активных эффектов в HUD (TODO-317).
//! VANILLA: Hud#extractEffects, MobEffectInstance#compareTo, MobEffects (категория, цвет).
use crate::effects::Effect;
use std::cmp::Ordering;

/// Фон иконки (путь относительно `textures/gui/`).
pub const SPRITES: &[(&str, &str)] = &[
    ("effect_background", "sprites/hud/effect_background"),
    (
        "effect_background_ambient",
        "sprites/hud/effect_background_ambient",
    ),
];

/// Иконки эффектов: (имя текстуры в painter, путь относительно `assets/minecraft/textures/`).
pub const MOB_EFFECT_SPRITES: &[(&str, &str)] = &[
    ("mob_effect_speed", "mob_effect/speed"),
    ("mob_effect_slowness", "mob_effect/slowness"),
    ("mob_effect_haste", "mob_effect/haste"),
    ("mob_effect_mining_fatigue", "mob_effect/mining_fatigue"),
    ("mob_effect_strength", "mob_effect/strength"),
    ("mob_effect_instant_health", "mob_effect/instant_health"),
    ("mob_effect_instant_damage", "mob_effect/instant_damage"),
    ("mob_effect_jump_boost", "mob_effect/jump_boost"),
    ("mob_effect_nausea", "mob_effect/nausea"),
    ("mob_effect_regeneration", "mob_effect/regeneration"),
    ("mob_effect_resistance", "mob_effect/resistance"),
    ("mob_effect_fire_resistance", "mob_effect/fire_resistance"),
    ("mob_effect_water_breathing", "mob_effect/water_breathing"),
    ("mob_effect_invisibility", "mob_effect/invisibility"),
    ("mob_effect_blindness", "mob_effect/blindness"),
    ("mob_effect_night_vision", "mob_effect/night_vision"),
    ("mob_effect_hunger", "mob_effect/hunger"),
    ("mob_effect_weakness", "mob_effect/weakness"),
    ("mob_effect_poison", "mob_effect/poison"),
    ("mob_effect_wither", "mob_effect/wither"),
    ("mob_effect_health_boost", "mob_effect/health_boost"),
    ("mob_effect_absorption", "mob_effect/absorption"),
    ("mob_effect_saturation", "mob_effect/saturation"),
    ("mob_effect_glowing", "mob_effect/glowing"),
    ("mob_effect_levitation", "mob_effect/levitation"),
    ("mob_effect_luck", "mob_effect/luck"),
    ("mob_effect_unluck", "mob_effect/unluck"),
    ("mob_effect_slow_falling", "mob_effect/slow_falling"),
    ("mob_effect_conduit_power", "mob_effect/conduit_power"),
    ("mob_effect_dolphins_grace", "mob_effect/dolphins_grace"),
    ("mob_effect_bad_omen", "mob_effect/bad_omen"),
    (
        "mob_effect_hero_of_the_village",
        "mob_effect/hero_of_the_village",
    ),
    ("mob_effect_darkness", "mob_effect/darkness"),
    ("mob_effect_trial_omen", "mob_effect/trial_omen"),
    ("mob_effect_raid_omen", "mob_effect/raid_omen"),
    ("mob_effect_wind_charged", "mob_effect/wind_charged"),
    ("mob_effect_weaving", "mob_effect/weaving"),
    ("mob_effect_oozing", "mob_effect/oozing"),
    ("mob_effect_infested", "mob_effect/infested"),
    (
        "mob_effect_breath_of_the_nautilus",
        "mob_effect/breath_of_the_nautilus",
    ),
];

/// VANILLA: MobEffects — (имя, категория B/H/N, цвет).
const INFO: &[(&str, char, i32)] = &[
    ("speed", 'B', 3402751),
    ("slowness", 'H', 9154528),
    ("haste", 'B', 14270531),
    ("mining_fatigue", 'H', 4866583),
    ("strength", 'B', 16762624),
    ("instant_health", 'B', 16262179),
    ("instant_damage", 'H', 11101546),
    ("jump_boost", 'B', 16646020),
    ("nausea", 'H', 5578058),
    ("regeneration", 'B', 13458603),
    ("resistance", 'B', 9520880),
    ("fire_resistance", 'B', 16750848),
    ("water_breathing", 'B', 10017472),
    ("invisibility", 'B', 16185078),
    ("blindness", 'H', 2039587),
    ("night_vision", 'B', 12779366),
    ("hunger", 'H', 5797459),
    ("weakness", 'H', 4738376),
    ("poison", 'H', 8889187),
    ("wither", 'H', 7561558),
    ("health_boost", 'B', 16284963),
    ("absorption", 'B', 2445989),
    ("saturation", 'B', 16262179),
    ("glowing", 'N', 9740385),
    ("levitation", 'H', 13565951),
    ("luck", 'B', 5882118),
    ("unluck", 'H', 12624973),
    ("slow_falling", 'B', 15978425),
    ("conduit_power", 'B', 1950417),
    ("dolphins_grace", 'B', 8954814),
    ("bad_omen", 'N', 745784),
    ("hero_of_the_village", 'B', 4521796),
    ("darkness", 'H', 2696993),
    ("trial_omen", 'N', 1484454),
    ("raid_omen", 'N', 14565464),
    ("wind_charged", 'H', 12438015),
    ("weaving", 'H', 7891290),
    ("oozing", 'H', 10092451),
    ("infested", 'H', 9214860),
    ("breath_of_the_nautilus", 'B', 65518),
];

fn info(name: &str) -> (char, i32) {
    INFO.iter()
        .find(|e| e.0 == name)
        .map_or(('H', 0), |e| (e.1, e.2))
}

/// VANILLA: MobEffect#isBeneficial — только BENEFICIAL.
fn beneficial(name: &str) -> bool {
    info(name).0 == 'B'
}

pub fn sprite_name(effect: &str) -> String {
    format!("mob_effect_{effect}")
}

/// VANILLA: MobEffectInstance#compareTo.
fn compare(a: &Effect, b: &Effect) -> Ordering {
    let (ca, cb) = (info(a.name()).1, info(b.name()).1);
    if (a.duration <= 32147 || b.duration <= 32147) && !(a.ambient && b.ambient) {
        a.ambient
            .cmp(&b.ambient)
            .then((a.duration == -1).cmp(&(b.duration == -1)))
            .then(a.duration.cmp(&b.duration))
            .then(ca.cmp(&cb))
    } else {
        a.ambient.cmp(&b.ambient).then(ca.cmp(&cb))
    }
}

pub struct Icon {
    pub x: f32,
    pub y: f32,
    pub ambient: bool,
    pub sprite: String,
    pub alpha: f32,
}

/// VANILLA: Hud#extractEffects (мигание при endsWithin(200)).
fn blink_alpha(duration: i32) -> f32 {
    let used = (10 - duration / 20) as f32;
    let a = (duration as f32 / 10.0 / 5.0 * 0.5).clamp(0.0, 0.5)
        + (duration as f32 * std::f32::consts::PI / 5.0).cos()
            * (used / 10.0 * 0.25).clamp(0.0, 0.25);
    a.clamp(0.0, 1.0)
}

/// Раскладка иконок: сортировка по убыванию compareTo, полезные в верхнем ряду.
pub fn layout<'a>(effects: impl Iterator<Item = &'a Effect>, gui_w: f32) -> Vec<Icon> {
    let mut list: Vec<&Effect> = effects.collect();
    list.sort_by(|a, b| compare(b, a));
    let (mut good, mut bad) = (0, 0);
    let mut out = Vec::new();
    for e in list {
        if !e.show_icon {
            continue;
        }
        let (x, y);
        if beneficial(e.name()) {
            good += 1;
            x = gui_w - 25.0 * good as f32;
            y = 1.0;
        } else {
            bad += 1;
            x = gui_w - 25.0 * bad as f32;
            y = 27.0;
        }
        let alpha = if !e.ambient && e.ends_within(200) {
            blink_alpha(e.duration)
        } else {
            1.0
        };
        out.push(Icon {
            x,
            y,
            ambient: e.ambient,
            sprite: sprite_name(e.name()),
            alpha,
        });
    }
    out
}

pub fn draw(p: &mut ezzmc_render::gui::GuiPainter, icons: &[Icon]) {
    for i in icons {
        let bg = if i.ambient {
            "effect_background_ambient"
        } else {
            "effect_background"
        };
        p.blit(bg, i.x, i.y, 24.0, 24.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
        let a = (i.alpha * 255.0) as u32;
        p.blit(
            &i.sprite,
            i.x + 3.0,
            i.y + 3.0,
            18.0,
            18.0,
            [0.0, 0.0, 1.0, 1.0],
            (a << 24) | 0x00FF_FFFF,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eff(name: &str, dur: i32, ambient: bool) -> Effect {
        let id = ezzmc_protocol::generated::r26_3::MOB_EFFECT
            .iter()
            .position(|n| *n == name)
            .unwrap() as i32;
        let mut ef = crate::effects::Effects::default();
        ef.update(id, 0, dur, if ambient { 1 | 4 } else { 4 });
        let e = ef.iter().next().unwrap().clone();
        e
    }

    #[test]
    fn rows_and_order() {
        let list = [
            eff("speed", 600, false),
            eff("poison", 100, false),
            eff("haste", 300, false),
        ];
        let l = layout(list.iter(), 400.0);
        // по убыванию длительности: speed (600), haste (300) в верхнем ряду; poison — во втором
        assert_eq!(l[0].sprite, "mob_effect_speed");
        assert_eq!((l[0].x, l[0].y), (375.0, 1.0));
        assert_eq!((l[1].x, l[1].y), (350.0, 1.0));
        assert_eq!((l[2].x, l[2].y), (375.0, 27.0));
    }

    #[test]
    fn ambient_first_and_blink() {
        let list = [eff("speed", 100, true), eff("haste", 600, false)];
        let l = layout(list.iter(), 100.0);
        // reverse(compareTo): ambient «больше», поэтому идёт первым (ближе к краю экрана)
        assert_eq!(l[0].sprite, "mob_effect_speed");
        assert_eq!(l[1].alpha, 1.0);
        let b = layout([eff("haste", 100, false)].iter(), 100.0);
        assert!((0.0..=1.0).contains(&b[0].alpha) && b[0].alpha < 1.0);
    }
}
