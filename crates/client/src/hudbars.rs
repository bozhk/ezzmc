//! HUD: сердца, броня, еда, полоса опыта и уровень.
//! VANILLA: Gui#extractPlayerHealth/extractHearts/extractArmor/extractFood, ExperienceBar, ContextualBar#extractExperienceLevel.
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use ezzmc_world::noise::LegacyRandom;

/// Спрайты (имя → путь). Имена сердец: `heart_{hardcore_}{full|half|container}{_blinking}`.
pub const SPRITES: &[(&str, &str)] = &[
    ("heart_container", "sprites/hud/heart/container"),
    (
        "heart_container_blinking",
        "sprites/hud/heart/container_blinking",
    ),
    (
        "heart_hardcore_container",
        "sprites/hud/heart/container_hardcore",
    ),
    (
        "heart_hardcore_container_blinking",
        "sprites/hud/heart/container_hardcore_blinking",
    ),
    ("heart_full", "sprites/hud/heart/full"),
    ("heart_full_blinking", "sprites/hud/heart/full_blinking"),
    ("heart_half", "sprites/hud/heart/half"),
    ("heart_half_blinking", "sprites/hud/heart/half_blinking"),
    ("heart_hardcore_full", "sprites/hud/heart/hardcore_full"),
    (
        "heart_hardcore_full_blinking",
        "sprites/hud/heart/hardcore_full_blinking",
    ),
    ("heart_hardcore_half", "sprites/hud/heart/hardcore_half"),
    (
        "heart_hardcore_half_blinking",
        "sprites/hud/heart/hardcore_half_blinking",
    ),
    ("armor_full", "sprites/hud/armor_full"),
    ("armor_half", "sprites/hud/armor_half"),
    ("armor_empty", "sprites/hud/armor_empty"),
    ("food_full", "sprites/hud/food_full"),
    ("food_half", "sprites/hud/food_half"),
    ("food_empty", "sprites/hud/food_empty"),
    ("xp_bg", "sprites/hud/experience_bar_background"),
    ("xp_progress", "sprites/hud/experience_bar_progress"),
    ("loc_bg", "sprites/hud/locator_bar_background"),
    ("loc_up", "sprites/hud/locator_bar_arrow_up"),
    ("loc_down", "sprites/hud/locator_bar_arrow_down"),
    ("loc_dot0", "sprites/hud/locator_bar_dot/default_0"),
    ("loc_dot1", "sprites/hud/locator_bar_dot/default_1"),
    ("loc_dot2", "sprites/hud/locator_bar_dot/default_2"),
    ("loc_dot3", "sprites/hud/locator_bar_dot/default_3"),
];

/// Снимок состояния игрока для HUD.
pub struct Stats {
    pub health: f32,
    pub max_health: f32,
    pub food: i32,
    pub saturation: f32,
    pub armor: f32,
    pub xp_progress: f32,
    pub xp_level: i32,
    pub hardcore: bool,
    pub tick: u64,
}

/// VANILLA: Gui#lastHealth/displayHealth/healthBlinkTime/lastHealthTime.
#[derive(Default)]
pub struct HudState {
    last_health: i32,
    display_health: i32,
    blink_time: i64,
    last_health_ms: u128,
}

fn blit9(p: &mut GuiPainter, name: &str, x: i32, y: i32) {
    p.blit(
        name,
        x as f32,
        y as f32,
        9.0,
        9.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFF_FFFF,
    );
}

/// VANILLA: Gui#extractPlayerHealth (без поглощения, эффектов и воздуха).
pub fn draw_health(p: &mut GuiPainter, st: &mut HudState, s: &Stats, gw: f32, gh: f32) {
    let cur = s.health.ceil() as i32;
    let tick = s.tick as i64;
    let blink = st.blink_time > tick && ((st.blink_time - tick) / 3) % 2 == 1;
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis());
    // VANILLA-UNSURE: damageCooldownTime > 0 считаем истинным при падении здоровья.
    if cur < st.last_health {
        st.last_health_ms = ms;
        st.blink_time = tick + 20;
    } else if cur > st.last_health && st.last_health != 0 && st.blink_time > tick {
        st.last_health_ms = ms;
        st.blink_time = tick + 10;
    }
    if ms.saturating_sub(st.last_health_ms) > 1000 {
        st.display_health = cur;
        st.last_health_ms = ms;
    }
    st.last_health = cur;
    let old = st.display_health;
    let mut rng = LegacyRandom::new(s.tick.wrapping_mul(312_871) as i64);
    let x_left = (gw / 2.0) as i32 - 91;
    let x_right = (gw / 2.0) as i32 + 91;
    let y_base = gh as i32 - 39;
    let max_health = s.max_health.max(old as f32).max(cur as f32);
    let rows = (max_health / 2.0 / 10.0).ceil() as i32;
    let row_h = (10 - (rows - 2)).max(3);

    // Броня.
    let armor = s.armor as i32;
    if armor > 0 {
        let y = y_base - (rows - 1) * row_h - 10;
        for i in 0..10 {
            let x = x_left + i * 8;
            let n = if i * 2 + 1 < armor {
                "armor_full"
            } else if i * 2 + 1 == armor {
                "armor_half"
            } else {
                "armor_empty"
            };
            blit9(p, n, x, y);
        }
    }

    // Сердца.
    let hc = if s.hardcore { "hardcore_" } else { "" };
    let containers = (max_health / 2.0).ceil() as i32;
    for idx in (0..containers).rev() {
        let (row, col) = (idx / 10, idx % 10);
        let x = x_left + col * 8;
        let mut y = y_base - row * row_h;
        if cur <= 4 {
            y += rng.next_int_bound(2);
        }
        let bl = |b: bool| if b { "_blinking" } else { "" };
        blit9(p, &format!("heart_{hc}container{}", bl(blink)), x, y);
        let halves = idx * 2;
        if blink && halves < old {
            let half = halves + 1 == old;
            blit9(
                p,
                &format!("heart_{hc}{}_blinking", if half { "half" } else { "full" }),
                x,
                y,
            );
        }
        if halves < cur {
            let half = halves + 1 == cur;
            blit9(
                p,
                &format!("heart_{hc}{}", if half { "half" } else { "full" }),
                x,
                y,
            );
        }
    }

    // Еда. VANILLA: Gui#extractFood.
    let food = s.food;
    for i in 0..10 {
        let mut y = y_base;
        if s.saturation <= 0.0 && s.tick % (food as u64 * 3 + 1) == 0 {
            y += rng.next_int_bound(3) - 1;
        }
        let x = x_right - i * 8 - 9;
        blit9(p, "food_empty", x, y);
        if i * 2 + 1 < food {
            blit9(p, "food_full", x, y);
        } else if i * 2 + 1 == food {
            blit9(p, "food_half", x, y);
        }
    }
}

/// VANILLA: Player#getXpNeededForNextLevel.
fn xp_needed(level: i32) -> i32 {
    if level >= 30 {
        112 + (level - 30) * 9
    } else if level >= 15 {
        37 + (level - 15) * 5
    } else {
        7 + level * 2
    }
}

/// VANILLA: ExperienceBar#extractBackground.
pub fn draw_xp_bar(p: &mut GuiPainter, s: &Stats, gw: f32, gh: f32) {
    if xp_needed(s.xp_level) <= 0 {
        return;
    }
    let (x, y) = (((gw as i32) - 182) / 2, gh as i32 - 24 - 5);
    let progress = (s.xp_progress * 183.0) as i32;
    p.blit(
        "xp_bg",
        x as f32,
        y as f32,
        182.0,
        5.0,
        [0.0, 0.0, 1.0, 1.0],
        0xFFFF_FFFF,
    );
    if progress > 0 {
        let w = progress.min(182) as f32;
        p.blit(
            "xp_progress",
            x as f32,
            y as f32,
            w,
            5.0,
            [0.0, 0.0, w / 182.0, 1.0],
            0xFFFF_FFFF,
        );
    }
}

/// VANILLA: ContextualBar#extractExperienceLevel (контур 4 сторон + зелёный текст).
pub fn draw_xp_level(t: &mut TextRenderer, level: i32, gw: f32, gh: f32) {
    if level <= 0 {
        return;
    }
    let s = level.to_string();
    let x = (gw - t.text_width(&s)) / 2.0;
    let y = gh - 24.0 - 9.0 - 2.0;
    for (dx, dy) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
        t.draw(&s, x + dx, y + dy, 0x000000, false);
    }
    t.draw(&s, x, y, 0x80FF20, false);
}

/// Данные точки (VANILLA: TrackedWaypoint).
#[derive(Clone, Debug)]
pub struct Wp {
    pub key: String,
    pub hash: i32,
    pub color: Option<u32>,
    pub kind: WpKind,
}

#[derive(Clone, Debug)]
pub enum WpKind {
    Empty,
    Pos(i32, i32, i32),
    Chunk(i32, i32),
    Azimuth(f32),
}

/// VANILLA: ARGB.setBrightness(ARGB.opaque(hash), 0.9) — HSB, B заменяется.
fn set_brightness(rgb: u32, b: f32) -> u32 {
    let (r, g, bl) = (
        ((rgb >> 16) & 255) as f32 / 255.0,
        ((rgb >> 8) & 255) as f32 / 255.0,
        (rgb & 255) as f32 / 255.0,
    );
    let mx = r.max(g).max(bl);
    let mn = r.min(g).min(bl);
    let d = mx - mn;
    let sat = if mx == 0.0 { 0.0 } else { d / mx };
    let hue = if sat == 0.0 {
        0.0
    } else if mx == r {
        ((g - bl) / d).rem_euclid(6.0) / 6.0
    } else if mx == g {
        ((bl - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    let h6 = hue * 6.0;
    let f = h6 - h6.floor();
    let (p, q, t) = (
        b * (1.0 - sat),
        b * (1.0 - sat * f),
        b * (1.0 - sat * (1.0 - f)),
    );
    let (r, g, bl) = match h6.floor() as i32 % 6 {
        0 => (b, t, p),
        1 => (q, b, p),
        2 => (p, b, t),
        3 => (p, q, b),
        4 => (t, p, b),
        _ => (b, p, q),
    };
    let c = |v: f32| (v * 255.0 + 0.5) as u32;
    (c(r) << 16) | (c(g) << 8) | c(bl)
}

/// Есть ли что показывать в локатор-баре (VANILLA: hasWaypoints).
/// VANILLA: Hud#nextContextualInfoState — в выживании XP недавно менялся → полоса опыта.
pub fn show_locator(has_wps: bool, survival: bool, xp_recent: bool) -> bool {
    has_wps && !(survival && xp_recent)
}

/// VANILLA: LocatorBar#extractRenderState. `cam` — глаза камеры, `yaw`/`pitch` в градусах, `fov` — вертикальный.
#[allow(clippy::too_many_arguments)]
pub fn draw_locator(
    p: &mut GuiPainter,
    wps: &std::collections::HashMap<String, Wp>,
    cam: [f64; 3],
    yaw: f32,
    pitch: f32,
    fov: f32,
    gw: f32,
    gh: f32,
) {
    let (x, top) = ((gw as i32 - 182) / 2, gh as i32 - 24 - 5);
    // VANILLA: locator_bar_background.png.mcmeta — nine_slice 12×5, border l5 r5 t1 b1.
    let (xf, tf) = (x as f32, top as f32);
    let white = 0xFFFF_FFFF;
    p.blit(
        "loc_bg",
        xf,
        tf,
        5.0,
        5.0,
        [0.0, 0.0, 5.0 / 12.0, 1.0],
        white,
    );
    p.blit(
        "loc_bg",
        xf + 5.0,
        tf,
        172.0,
        5.0,
        [5.0 / 12.0, 0.0, 7.0 / 12.0, 1.0],
        white,
    );
    p.blit(
        "loc_bg",
        xf + 177.0,
        tf,
        5.0,
        5.0,
        [7.0 / 12.0, 0.0, 1.0, 1.0],
        white,
    );
    // VANILLA: стрелки — анимация: кадр 0 (10 тиков), кадр 1 (4 тика), высота кадра 5.
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis());
    let frame = if (ms / 50) % 14 < 10 { 0.0 } else { 0.5 };
    let mid = ((gw as i32 - 9) as f32 / 2.0).ceil() as i32;
    let tan_half = (f64::from(fov).to_radians() / 2.0).tan();
    for w in wps.values() {
        let (angle, dist2, ndc_y) = match w.kind {
            WpKind::Empty => continue,
            WpKind::Azimuth(a) => {
                let ang = wrap_deg(a.to_degrees() - yaw);
                let ndc = (f64::from(pitch).to_radians().tan() / tan_half) as f32;
                (ang, f64::INFINITY, ndc)
            }
            WpKind::Pos(..) | WpKind::Chunk(..) => {
                let (tx, ty, tz) = match w.kind {
                    WpKind::Pos(a, b, c) => {
                        (f64::from(a) + 0.5, f64::from(b) + 0.5, f64::from(c) + 0.5)
                    }
                    WpKind::Chunk(a, b) => {
                        (f64::from(a) * 16.0 + 8.0, cam[1], f64::from(b) * 16.0 + 8.0)
                    }
                    _ => continue,
                };
                let (dx, dy, dz) = (cam[0] - tx, cam[1] - ty, cam[2] - tz);
                // rotateClockwise90: (−d.z, d.x).
                let wa = (dx).atan2(-dz).to_degrees();
                let ang = wrap_deg(wa as f32 - yaw);
                let horiz = (dx * dx + dz * dz).sqrt();
                let ndc = if matches!(w.kind, WpKind::Pos(..)) {
                    let elev = (-dy).atan2(horiz);
                    ((elev + f64::from(pitch).to_radians()).tan() / tan_half) as f32
                } else {
                    (f64::from(pitch).to_radians().tan() / tan_half) as f32
                };
                (ang, dx * dx + dy * dy + dz * dz, ndc)
            }
        };
        if !(angle > -60.0 && angle <= 60.0) {
            continue;
        }
        let dot = (angle * 173.0 / 2.0 / 60.0).floor() as i32;
        let col = w
            .color
            .unwrap_or_else(|| set_brightness((w.hash as u32) & 0xFF_FFFF, 0.9));
        let col = 0xFF00_0000 | col;
        // WaypointStyle default: near 128, far 332, 4 спрайта.
        let (near2, far2) = (128.0f64 * 128.0, 332.0f64 * 332.0);
        // VANILLA: WaypointStyle#sprite — lerpInt(t, 1, n−1) при n=4 даёт 1 между near и far.
        let idx = if dist2 < near2 {
            0
        } else if dist2 >= far2 {
            3
        } else {
            1
        };
        p.blit(
            &format!("loc_dot{idx}"),
            (mid + dot) as f32,
            (top - 2) as f32,
            9.0,
            9.0,
            [0.0, 0.0, 1.0, 1.0],
            col,
        );
        let arrow = if ndc_y > 1.0 {
            Some(("loc_up", top - 6))
        } else if ndc_y < -1.0 {
            Some(("loc_down", top + 6))
        } else {
            None
        };
        if let Some((n, ay)) = arrow {
            p.blit(
                n,
                (mid + dot + 1) as f32,
                ay as f32,
                7.0,
                5.0,
                [0.0, frame, 1.0, frame + 0.5],
                0xFFFF_FFFF,
            );
        }
    }
}

fn wrap_deg(a: f32) -> f32 {
    let mut d = a % 360.0;
    if d >= 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0;
    }
    d
}
