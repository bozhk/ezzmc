//! Экран отладки F3.
//!
//! VANILLA: DebugScreenOverlay#extractRenderState + DebugScreenEntries (профиль DEFAULT, 26.3):
//! DebugEntryVersion/Fps/Position/SectionPosition/Light/LookingAt/ChunkRenderStats/Memory/SystemSpecs.
//! Не портировано (нет данных клиента): TPS/сервер-бренд, сущности/частицы, биом, хайтмапы,
//! локальная сложность, звук, измерение (FC), графики.
use crate::worldview::WorldView;
use ezzmc_mesh::blockinfo as bi;
use ezzmc_render::text::TextRenderer;
use ezzmc_world::block::BlockState;

/// Всё, что F3 берёт не из мира.
pub struct Env<'a> {
    pub fps: u32,
    pub fps_limit: u32,
    pub present: &'a str,
    pub yaw: f32,
    pub pitch: f32,
    pub window: (u32, u32),
    pub gpu: &'a wgpu::AdapterInfo,
}

const PLAYER_EYE: f64 = 1.62;

/// VANILLA: Mth#wrapDegrees.
fn wrap_degrees(d: f32) -> f32 {
    let mut v = d % 360.0;
    if v >= 180.0 {
        v -= 360.0;
    }
    if v < -180.0 {
        v += 360.0;
    }
    v
}

/// VANILLA: Direction#fromYRot — 0 = SOUTH, 1 = WEST, 2 = NORTH, 3 = EAST.
fn facing(yaw: f32) -> (&'static str, &'static str) {
    match ((f64::from(yaw) / 90.0 + 0.5).floor() as i64).rem_euclid(4) {
        0 => ("south", "Towards positive Z"),
        1 => ("west", "Towards negative X"),
        2 => ("north", "Towards negative Z"),
        _ => ("east", "Towards positive X"),
    }
}

fn state_lines(
    title: &str,
    pos: [i32; 3],
    name: &str,
    props: Vec<(String, String)>,
) -> Vec<String> {
    let mut v = vec![
        format!("§n{title}: {}, {}, {}", pos[0], pos[1], pos[2]),
        format!("minecraft:{name}"),
    ];
    for (k, val) in props {
        let c = match val.as_str() {
            "true" => "§a",
            "false" => "§c",
            _ => "",
        };
        v.push(format!("{k}: {c}{val}"));
    }
    v
}

fn block_props(s: BlockState) -> Vec<(String, String)> {
    s.block()
        .props
        .iter()
        .filter_map(|p| {
            s.property(p.name)
                .map(|v| (p.name.to_string(), v.to_string()))
        })
        .collect()
}

fn fluid_props(f: bi::Fluid) -> (&'static str, Vec<(String, String)>) {
    let n = match (f.lava, f.source) {
        (false, true) => "water",
        (false, false) => "flowing_water",
        (true, true) => "lava",
        (true, false) => "flowing_lava",
    };
    let mut p = vec![("falling".to_string(), f.falling.to_string())];
    if !f.source {
        p.push(("level".to_string(), f.amount.to_string()));
    }
    (n, p)
}

/// RSS читается из /proc, поэтому значение кешируется на секунду (экран F3 перерисовывается каждый кадр).
fn rss_mib() -> u64 {
    use std::sync::Mutex;
    use std::time::{Duration, Instant};
    static CACHE: Mutex<Option<(Instant, u64)>> = Mutex::new(None);
    let mut c = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, v)) = *c {
        if at.elapsed() < Duration::from_secs(1) {
            return v;
        }
    }
    let v = std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find_map(|l| l.strip_prefix("VmRSS:"))
                .and_then(|l| l.split_whitespace().next()?.parse::<u64>().ok())
        })
        .map_or(0, |kb| kb / 1024);
    *c = Some((Instant::now(), v));
    v
}

/// VRAM (занято/всего, МиБ) и мощность (Вт) по `nvidia-smi`; опрос раз в секунду в фоне.
/// `None`, если утилиты нет (не NVIDIA) — тогда строка не выводится.
fn gpu_stats() -> Option<(u32, u32, f32)> {
    use std::sync::{Mutex, Once};
    static STATS: Mutex<Option<(u32, u32, f32)>> = Mutex::new(None);
    static START: Once = Once::new();
    START.call_once(|| {
        let _ = std::thread::Builder::new()
            .name("gpu-stats".into())
            .spawn(|| loop {
                let Ok(out) = std::process::Command::new("nvidia-smi")
                    .args([
                        "--query-gpu=memory.used,memory.total,power.draw",
                        "--format=csv,noheader,nounits",
                    ])
                    .output()
                else {
                    return;
                };
                if !out.status.success() {
                    return;
                }
                let t = String::from_utf8_lossy(&out.stdout);
                let mut it = t.lines().next().unwrap_or("").split(',').map(str::trim);
                let used = it.next().and_then(|x| x.parse().ok());
                let total = it.next().and_then(|x| x.parse().ok());
                let watts = it.next().and_then(|x| x.parse().ok()).unwrap_or(0.0);
                if let (Some(u), Some(m), Ok(mut g)) = (used, total, STATS.lock()) {
                    *g = Some((u, m, watts));
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            });
    });
    STATS.lock().ok().and_then(|g| *g)
}

fn cpu_info() -> &'static str {
    static C: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    C.get_or_init(|| {
        let t = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let n = t.lines().filter(|l| l.starts_with("processor")).count();
        let name = t
            .lines()
            .find_map(|l| l.strip_prefix("model name"))
            .and_then(|l| l.split(':').nth(1))
            .map_or("<unknown>".to_string(), |s| {
                s.split_whitespace().collect::<Vec<_>>().join(" ")
            });
        format!("{n}x {name}")
    })
}

fn vendor_name(id: u32) -> String {
    match id {
        0x10DE => "NVIDIA Corporation".into(),
        0x1002 | 0x1022 => "ATI Technologies Inc.".into(),
        0x8086 => "Intel".into(),
        0x1414 => "Microsoft".into(),
        _ => format!("0x{id:04X}"),
    }
}

/// Левая и правая колонки (пустая строка = разделитель, как в ванили).
pub fn build(v: &WorldView, e: &Env) -> (Vec<String>, Vec<String>) {
    let feet = [v.cam[0], v.cam[1] - PLAYER_EYE, v.cam[2]];
    let b = [
        feet[0].floor() as i32,
        feet[1].floor() as i32,
        feet[2].floor() as i32,
    ];
    let (cx, cy, cz) = (b[0] >> 4, b[1] >> 4, b[2] >> 4);
    let (dir, towards) = facing(e.yaw);

    // addPriorityLine: версия → слева, fps → справа.
    let mut left = vec!["Minecraft 26.3 (26.3/vanilla)".to_string()];
    let limit = if e.fps_limit >= crate::display::UNLIMITED_FPS {
        "inf".to_string()
    } else {
        e.fps_limit.to_string()
    };
    let mut right = vec![format!("{} fps T: {limit}{}", e.fps, e.present)];
    left.push(String::new());
    right.push(String::new());

    // addLine: простые строки делятся пополам.
    let regular = [
        "fancy-clouds B: 2".to_string(),
        "Terrain Rendering: naive".to_string(),
        format!(
            "C: {}/{} (s) D: {}",
            v.visible_sections,
            v.resident_sections(),
            v.view_radius()
        ),
    ];
    let mid = regular.len().div_ceil(2);
    left.extend(regular[..mid].iter().cloned());
    left.push(String::new());
    right.extend(regular[mid..].iter().cloned());
    right.push(String::new());

    // Группы: позиция, свет — слева; цель, жидкость, память, система — справа.
    let mut pos = vec![
        format!("XYZ: {:.3} / {:.5} / {:.3}", feet[0], feet[1], feet[2]),
        format!("Block: {} {} {}", b[0], b[1], b[2]),
        format!(
            "Chunk: {cx} {cy} {cz} [{} {} in r.{}.{}.mca]",
            cx & 31,
            cz & 31,
            cx >> 5,
            cz >> 5
        ),
        format!(
            "Facing: {dir} ({towards}) ({:.1} / {:.1})",
            wrap_degrees(e.yaw),
            wrap_degrees(e.pitch)
        ),
    ];
    // Скорость за последний тик (блоков/с): горизонтальная и общая.
    {
        let (p, q) = (v.player.pos, v.player.prev_pos);
        let (dx, dy, dz) = (p[0] - q[0], p[1] - q[1], p[2] - q[2]);
        pos.push(format!(
            "Speed: {:.3} b/s (XZ {:.3})",
            (dx * dx + dy * dy + dz * dz).sqrt() * 20.0,
            (dx * dx + dz * dz).sqrt() * 20.0
        ));
    }
    pos.push(format!(
        "Section-relative: {:02} {:02} {:02}",
        b[0] & 15,
        b[1] & 15,
        b[2] & 15
    ));
    let light = v
        .with_level(|l| {
            let c = l.chunks.get(cx, cz)?;
            let (x, z) = ((b[0] & 15) as usize, (b[2] & 15) as usize);
            let sky = c.sky_light(x, b[1], z).unwrap_or(0);
            let blk = c.block_light(x, b[1], z).unwrap_or(0);
            Some((sky.max(blk), sky, blk))
        })
        .flatten()
        .map_or(
            "Client Light: 0 (0 sky, 0 block)".to_string(),
            |(r, s, k)| format!("Client Light: {r} ({s} sky, {k} block)"),
        );
    left.extend(pos);
    left.push(String::new());
    left.push(light);
    left.push(String::new());

    if let Some(t) = v.look_target {
        right.extend(state_lines(
            "Targeted Block",
            t.pos,
            t.state.name(),
            block_props(t.state),
        ));
        right.push(String::new());
    }
    if let Some((t, f)) = v
        .look_fluid
        .and_then(|t| bi::fluid(t.state).map(|f| (t, f)))
    {
        let (n, p) = fluid_props(f);
        right.extend(state_lines("Targeted Fluid", t.pos, n, p));
        right.push(String::new());
    }
    right.push(v.render_label().to_owned());
    right.push(format!("Mem: {} MiB (RSS)", rss_mib()));
    if let Some((used, total, watts)) = gpu_stats() {
        right.push(format!(
            "VRAM: {used}/{total} MiB ({}%)",
            used * 100 / total.max(1)
        ));
        if watts > 0.0 {
            right.push(format!("GPU Power: {watts:.0} W"));
        }
    }
    right.push(String::new());
    let g = e.gpu;
    right.push(format!("CPU: {}", cpu_info()));
    right.push(format!(
        "Display: {}x{} ({})",
        e.window.0,
        e.window.1,
        vendor_name(g.vendor)
    ));
    let ty = match g.device_type {
        wgpu::DeviceType::IntegratedGpu => " (iGPU)",
        wgpu::DeviceType::DiscreteGpu => " (dGPU)",
        wgpu::DeviceType::VirtualGpu => " (vGPU)",
        wgpu::DeviceType::Cpu => " (software)",
        wgpu::DeviceType::Other => "",
    };
    right.push(format!("{}{ty}", g.name));
    right.push(format!(
        "{:?} {}",
        g.backend,
        g.driver_info.lines().next().unwrap_or("")
    ));
    (left, right)
}

/// VANILLA: DebugScreenOverlay#extractLines — подложка ARGB −1873784752, текст −2039584 без тени.
pub fn draw(t: &mut TextRenderer, left: &[String], right: &[String], gui_w: f32) {
    for (lines, align_left) in [(left, true), (right, false)] {
        for (i, l) in lines.iter().enumerate() {
            if l.is_empty() {
                continue;
            }
            let w = t.text_width(l);
            let x = if align_left { 2.0 } else { gui_w - 2.0 - w };
            let y = 2.0 + 9.0 * i as f32;
            t.fill(x - 1.0, y - 1.0, x + w + 1.0, y + 8.0, 0x9050_5050);
        }
        for (i, l) in lines.iter().enumerate() {
            if l.is_empty() {
                continue;
            }
            let w = t.text_width(l);
            let x = if align_left { 2.0 } else { gui_w - 2.0 - w };
            t.draw(l, x, 2.0 + 9.0 * i as f32, 0xE0E0E0, false);
        }
    }
}
