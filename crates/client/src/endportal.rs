//! Портал Энда и шлюз Энда (BER): геометрия кубов для `EndPortalDraw`.
//! VANILLA: TheEndPortalRenderer (куб y∈[0.375,0.75], грани по оси Y), TheEndGatewayRenderer (единичный куб,
//! `Block#shouldRenderFace`). Луч шлюза (beacon beam при спавне/кулдауне) — отдельная задача.

use crate::berscan::{in_view, Scanner};
use ezzmc_render::endportaldraw::PortalVertex;
use ezzmc_world::block::{BlockState, FLAG_CAN_OCCLUDE};
use ezzmc_world::level::Level;

const PORTAL_LAYERS: f32 = 15.0;
const GATEWAY_LAYERS: f32 = 16.0;

fn lut() -> &'static [u8] {
    static LUT: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    LUT.get_or_init(|| {
        (0..ezzmc_world::generated::b26_3::STATE_COUNT)
            .map(|i| match BlockState::from_id(i).map(|s| s.name()) {
                Some("end_portal") => 1,
                Some("end_gateway") => 2,
                _ => 0,
            })
            .collect()
    })
}

fn is_id(id: u32) -> bool {
    lut().get(id as usize).copied().unwrap_or(0) != 0
}

#[derive(Default)]
pub struct EndPortals {
    scan: Scanner,
}

/// Грань куба [x0,y0,z0]–[x1,y1,z1]: 4 угла.
fn face(min: [f32; 3], max: [f32; 3], dir: usize) -> [[f32; 3]; 4] {
    let [x0, y0, z0] = min;
    let [x1, y1, z1] = max;
    match dir {
        0 => [[x0, y0, z1], [x0, y0, z0], [x1, y0, z0], [x1, y0, z1]], // down
        1 => [[x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0]], // up
        2 => [[x1, y1, z0], [x1, y0, z0], [x0, y0, z0], [x0, y1, z0]], // north
        3 => [[x0, y1, z1], [x0, y0, z1], [x1, y0, z1], [x1, y1, z1]], // south
        4 => [[x0, y1, z0], [x0, y0, z0], [x0, y0, z1], [x0, y1, z1]], // west
        _ => [[x1, y1, z1], [x1, y0, z1], [x1, y0, z0], [x1, y1, z0]], // east
    }
}

const OFFSETS: [[i32; 3]; 6] = [
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
    [-1, 0, 0],
    [1, 0, 0],
];

impl EndPortals {
    /// Вершины (по 4 на квад) относительно камеры.
    pub fn build(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        versions: (u64, u64),
        out: &mut Vec<PortalVertex>,
    ) {
        out.clear();
        self.scan.update(level, cam, versions, is_id);
        let table = lut();
        for (p, s) in &self.scan.list {
            if !in_view(*p, cam) {
                continue;
            }
            let kind = table.get(s.id() as usize).copied().unwrap_or(0);
            let o = [
                (f64::from(p[0]) - cam[0]) as f32,
                (f64::from(p[1]) - cam[1]) as f32,
                (f64::from(p[2]) - cam[2]) as f32,
            ];
            let (lo, hi, layers) = if kind == 1 {
                // Transformation: translate (0,0.375,0), scale (1,0.375,1).
                ([0.0, 0.375, 0.0], [1.0, 0.75, 1.0], PORTAL_LAYERS)
            } else {
                ([0.0; 3], [1.0; 3], GATEWAY_LAYERS)
            };
            for (dir, d) in OFFSETS.iter().enumerate() {
                let show = if kind == 1 {
                    dir < 2 // shouldRenderFace: ось Y
                } else {
                    // VANILLA-UNSURE: Block#shouldRenderFace ≈ «сосед не тот же блок и не полный непрозрачный».
                    let n = level.chunks.block(p[0] + d[0], p[1] + d[1], p[2] + d[2]);
                    n.name() != "end_gateway"
                        && !n.props().is_some_and(|pr| pr.has(FLAG_CAN_OCCLUDE))
                };
                if !show {
                    continue;
                }
                for c in face(lo, hi, dir) {
                    out.push([o[0] + c[0], o[1] + c[1], o[2] + c[2], layers]);
                }
            }
        }
    }
}
