//! Луч маяка: секции луча (клиентская часть `BeaconBlockEntity#tick`) и геометрия `BeaconRenderer`.
//! VANILLA: BeaconBlockEntity#tick (сканирование вверх, цвета стекла усредняются), BeaconRenderer#submitBeaconBeam.
//! Отличие: секции считаются целиком при изменении мира, а не по 10 блоков за тик (результат тот же).
//! Луч шлюза: возраст берётся из NBT `Age` + тики с момента наблюдения, кулдаун — из block_event(1).
//! Нет: луч дальше окна `berscan` (≈64 блока;
//! ванильный `getViewDistance` = 256).

use crate::berscan::Scanner;
use ezzmc_render::beacondraw::{BeamBatches, BeamVertex};
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;

const SOLID_RADIUS: f32 = 0.2;
const GLOW_RADIUS: f32 = 0.25;
const BEAM_SCALE_THRESHOLD: f32 = 96.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section {
    /// ARGB (`DyeColor#getTextureDiffuseColor`, альфа 255).
    pub color: u32,
    pub height: i32,
}

/// `DyeColor#getTextureDiffuseColor` без альфы, порядок DyeColor.
const DYES: [(&str, u32); 16] = [
    ("white", 0xF9FFFE),
    ("orange", 0xF9801D),
    ("magenta", 0xC74EBD),
    ("light_blue", 0x3AB3DA),
    ("yellow", 0xFED83D),
    ("lime", 0x80C71F),
    ("pink", 0xF38BAA),
    ("gray", 0x474F52),
    ("light_gray", 0x9D9D97),
    ("cyan", 0x169C9C),
    ("purple", 0x8932B8),
    ("blue", 0x3C44AA),
    ("brown", 0x835432),
    ("green", 0x5E7C16),
    ("red", 0xB02E26),
    ("black", 0x1D1D21),
];

/// Цвет блока-проводника луча (`BeaconBeamBlock`): маяк, цветное стекло и панели. 0 — не проводник.
fn beam_color_by_name(name: &str) -> u32 {
    let color = |c: &str| {
        DYES.iter()
            .find(|(n, _)| *n == c)
            .map_or(0, |(_, rgb)| 0xFF00_0000 | rgb)
    };
    if name == "beacon" {
        return color("white");
    }
    name.strip_suffix("_stained_glass")
        .or_else(|| name.strip_suffix("_stained_glass_pane"))
        .map_or(0, color)
}

fn lut() -> &'static [u32] {
    static LUT: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
    LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| BlockState::from_id(i).map_or(0, |s| beam_color_by_name(s.name())))
            .collect()
    })
}

fn is_beacon(id: u32) -> bool {
    BlockState::from_id(id).is_some_and(|s| s.name() == "beacon")
}

fn avg_argb(a: u32, b: u32) -> u32 {
    let ch = |s: u32| ((a >> s & 0xFF) + (b >> s & 0xFF)) / 2;
    ch(24) << 24 | ch(16) << 16 | ch(8) << 8 | ch(0)
}

/// Секции луча над маяком (`checkingBeamSections` после полного прохода). Пусто — луча нет.
pub fn compute_sections(level: &Level, x: i32, y: i32, z: i32) -> Vec<Section> {
    let Some(ch) = level.chunks.get(x >> 4, z >> 4) else {
        return Vec::new();
    };
    let top = ch.chunk.min_y() + ch.chunk.sections().len() as i32 * 16;
    sections_from(|cy| level.chunks.block(x, cy, z), y, top)
}

fn sections_from(block_at: impl Fn(i32) -> BlockState, y: i32, top: i32) -> Vec<Section> {
    let table = lut();
    let mut secs: Vec<Section> = Vec::new();
    for cy in y..top {
        let st = block_at(cy);
        let color = table.get(st.id() as usize).copied().unwrap_or(0);
        if color != 0 {
            if secs.len() <= 1 {
                secs.push(Section { color, height: 1 });
            } else if let Some(last) = secs.last_mut() {
                if last.color == color {
                    last.height += 1;
                } else {
                    let avg = avg_argb(last.color, color);
                    secs.push(Section {
                        color: avg,
                        height: 1,
                    });
                }
            }
        } else {
            let dampening = st.props().map_or(0, |p| p.light_block);
            match secs.last_mut() {
                None => return Vec::new(),
                Some(_) if dampening >= 15 && st.name() != "bedrock" => return Vec::new(),
                Some(last) => last.height += 1,
            }
        }
    }
    secs
}

/// Клиентское состояние `TheEndGatewayBlockEntity` (age, teleportCooldown).
struct Gateway {
    /// Возраст в тик `base_time`.
    age: i64,
    base_time: i64,
    /// Значение `Age` из последнего увиденного тега (сброс базы при смене).
    tag_age: Option<i64>,
    /// Тик получения block_event(1) (кулдаун 40).
    cooldown_at: Option<i64>,
}

fn is_gateway(id: u32) -> bool {
    BlockState::from_id(id).is_some_and(|s| s.name() == "end_gateway")
}

#[derive(Default)]
pub struct Beacons {
    scan: Scanner,
    beams: Vec<([i32; 3], Vec<Section>)>,
    gw_scan: Scanner,
    gateways: std::collections::HashMap<[i32; 3], Gateway>,
    now: i64,
}

impl Beacons {
    /// Заполняет `out[0]` (текстура маяка) квадами луча относительно камеры.
    pub fn build(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        out: &mut BeamBatches,
    ) {
        if self.scan.update(level, cam, versions, is_beacon) {
            self.beams = self
                .scan
                .list
                .iter()
                .map(|(p, _)| (*p, compute_sections(level, p[0], p[1], p[2])))
                .collect();
        }
        self.now = level.game_time;
        self.build_gateways(level, cam, versions, out);
        // VANILLA: BeaconRenderer#extract — animationTime = floorMod(gameTime, 40) + partialTicks.
        let anim = level.game_time.rem_euclid(40) as f32 + level.partial_tick();
        for (p, secs) in &self.beams {
            if secs.is_empty() || !crate::berscan::in_view(*p, cam) {
                continue;
            }
            let dx = f64::from(p[0]) + 0.5 - cam[0];
            let dz = f64::from(p[2]) + 0.5 - cam[2];
            let scale = ((dx * dx + dz * dz).sqrt() as f32 / BEAM_SCALE_THRESHOLD).max(1.0);
            let base = [dx as f32, (f64::from(p[1]) - cam[1]) as f32, dz as f32];
            let mut start = 0;
            for (i, s) in secs.iter().enumerate() {
                let height = if i == secs.len() - 1 { 2048 } else { s.height };
                let [solid, glow] = &mut out[0];
                beam_quads(
                    solid,
                    glow,
                    &Beam {
                        base,
                        scale: 1.0,
                        anim,
                        start,
                        height,
                        color: s.color,
                        solid_radius: SOLID_RADIUS * scale,
                        glow_radius: GLOW_RADIUS * scale,
                    },
                );
                start += s.height;
            }
        }
    }
}

impl Beacons {
    /// VANILLA: TheEndGatewayBlockEntity#triggerEvent — b0 == 1 → teleportCooldown = 40.
    pub fn block_event(&mut self, pos: [i32; 3], b0: i32) {
        if b0 == 1 {
            if let Some(g) = self.gateways.get_mut(&pos) {
                g.cooldown_at = Some(self.now);
            }
        }
    }

    /// VANILLA: TheEndGatewayRenderer#extractRenderState + submit (луч при спавне или кулдауне).
    fn build_gateways(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        out: &mut BeamBatches,
    ) {
        self.gw_scan.update(level, cam, versions, is_gateway);
        let now = level.game_time;
        let partial = level.partial_tick();
        let list: Vec<[i32; 3]> = self.gw_scan.list.iter().map(|(p, _)| *p).collect();
        self.gateways.retain(|p, _| list.contains(p));
        let anim = now.rem_euclid(40) as f32 + partial;
        for p in list {
            let tag_age = crate::berscan::block_entity_tag(level, p)
                .and_then(|t| t.get("Age"))
                .and_then(ezzmc_protocol::nbt::Tag::as_i64);
            let g = self.gateways.entry(p).or_insert(Gateway {
                age: tag_age.unwrap_or(200),
                base_time: now,
                tag_age,
                cooldown_at: None,
            });
            if g.tag_age != tag_age {
                g.tag_age = tag_age;
                g.age = tag_age.unwrap_or(g.age + (now - g.base_time));
                g.base_time = now;
            }
            if !crate::berscan::in_view(p, cam) {
                continue;
            }
            let age = g.age + (now - g.base_time);
            let spawning = age < 200;
            let cooldown_left = g.cooldown_at.map_or(0, |t| 40 - (now - t));
            if !spawning && cooldown_left <= 0 {
                continue;
            }
            let pct = if spawning {
                ((age as f32 + partial) / 200.0).clamp(0.0, 1.0)
            } else {
                1.0 - ((cooldown_left as f32 - partial) / 40.0).clamp(0.0, 1.0)
            };
            let top = level.chunks.get(p[0] >> 4, p[2] >> 4).map_or(320, |c| {
                c.chunk.min_y() + c.chunk.sections().len() as i32 * 16
            });
            let distance = if spawning { (top - 1) as f32 } else { 50.0 };
            let scale = (pct * std::f32::consts::PI).sin();
            let height = (scale * distance).floor() as i32;
            if height <= 0 {
                continue;
            }
            let color = if spawning { 0xFFC7_4EBD } else { 0xFF89_32B8 };
            let [solid, glow] = &mut out[1];
            beam_quads(
                solid,
                glow,
                &Beam {
                    base: [
                        (f64::from(p[0]) + 0.5 - cam[0]) as f32,
                        (f64::from(p[1]) - cam[1]) as f32,
                        (f64::from(p[2]) + 0.5 - cam[2]) as f32,
                    ],
                    scale,
                    anim,
                    start: -height,
                    height: height * 2,
                    color,
                    solid_radius: 0.15,
                    glow_radius: 0.175,
                },
            );
        }
    }
}

/// Параметры `BeaconRenderer#submitBeaconBeam`.
pub struct Beam {
    /// Низ столба относительно камеры: x, z — центр блока, y — нижняя грань.
    pub base: [f32; 3],
    pub scale: f32,
    pub anim: f32,
    pub start: i32,
    pub height: i32,
    pub color: u32,
    pub solid_radius: f32,
    pub glow_radius: f32,
}

fn frac(v: f32) -> f32 {
    v - v.floor()
}

pub fn beam_quads(solid: &mut Vec<BeamVertex>, glow: &mut Vec<BeamVertex>, b: &Beam) {
    let end = b.start + b.height;
    let scroll = if b.height < 0 { b.anim } else { -b.anim };
    let v_off = frac(scroll * 0.2 - (scroll * 0.1).floor());
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, (c >> 24) as u8];
    let [bx, by, bz] = b.base;
    // Сплошной слой: поворот вокруг Y на anim·2.25 − 45°.
    let r = b.solid_radius;
    let (sin, cos) = (b.anim * 2.25 - 45.0).to_radians().sin_cos();
    let rot = |x: f32, z: f32| [bx + x * cos + z * sin, bz - x * sin + z * cos];
    let vv2 = -1.0 + v_off;
    let vv1 = b.height as f32 * b.scale * (0.5 / r) + vv2;
    let (wn, en, ws, es) = (rot(0.0, r), rot(r, 0.0), rot(-r, 0.0), rot(0.0, -r));
    let c = rgba(b.color);
    for (a, d) in [(wn, en), (es, ws), (en, es), (ws, wn)] {
        quad(solid, by, b.start, end, a, d, vv1, vv2, c);
    }
    // Свечение: без поворота, альфа 32.
    let g = b.glow_radius;
    let at = |x: f32, z: f32| [bx + x, bz + z];
    let (wn, en, ws, es) = (at(-g, -g), at(g, -g), at(-g, g), at(g, g));
    let vv1 = b.height as f32 * b.scale + vv2;
    let c = rgba((32 << 24) | (b.color & 0x00FF_FFFF));
    for (a, d) in [(wn, en), (es, ws), (en, es), (ws, wn)] {
        quad(glow, by, b.start, end, a, d, vv1, vv2, c);
    }
}

#[allow(clippy::too_many_arguments)]
fn quad(
    out: &mut Vec<BeamVertex>,
    by: f32,
    start: i32,
    end: i32,
    w: [f32; 2],
    e: [f32; 2],
    vv1: f32,
    vv2: f32,
    c: [u8; 4],
) {
    let y = |v: i32| by + v as f32;
    out.push(([w[0], y(end), w[1]], [1.0, vv1], c));
    out.push(([w[0], y(start), w[1]], [1.0, vv2], c));
    out.push(([e[0], y(start), e[1]], [0.0, vv2], c));
    out.push(([e[0], y(end), e[1]], [0.0, vv1], c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_and_average() {
        assert_eq!(beam_color_by_name("beacon"), 0xFFF9FFFE);
        assert_eq!(beam_color_by_name("red_stained_glass_pane"), 0xFFB02E26);
        assert_eq!(beam_color_by_name("glass"), 0);
        assert_eq!(avg_argb(0xFF000000, 0xFF0000FE), 0xFF00007F);
    }
}
