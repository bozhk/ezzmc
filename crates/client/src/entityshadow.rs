//! Тени сущностей. VANILLA: EntityRenderer#extractShadow/#extractShadowPiece +
//! ShadowFeatureRenderer#prepare (26.3). Тень — квады `shadow.png` на верхних гранях блоков под ногами.
use ezzmc_mesh::blockinfo as bi;
use ezzmc_render::particledraw::push_vertex;
use ezzmc_world::level::Level;

/// VANILLA: EntityRenderer.MAX_SHADOW_RADIUS.
const MAX_RADIUS: f32 = 32.0;

/// Вклад в тень одной сущности. `pos` — позиция ног (интерполированная), `cam` — камера,
/// `sky_darken` — Level#getSkyDarken. Квады пишутся в формате `push_vertex` относительно камеры.
pub fn bake(
    level: &Level,
    pos: [f64; 3],
    cam: [f64; 3],
    radius: f32,
    strength: f32,
    sky_darken: i32,
    out: &mut Vec<u8>,
) {
    let radius = radius.min(MAX_RADIUS);
    if radius <= 0.0 {
        return;
    }
    let dist_sq = (pos[0] - cam[0]).powi(2) + (pos[1] - cam[1]).powi(2) + (pos[2] - cam[2]).powi(2);
    let pow = ((1.0 - dist_sq / 256.0) * f64::from(strength)) as f32;
    if pow <= 0.0 {
        return;
    }
    let r = f64::from(radius);
    let (x0, x1) = ((pos[0] - r).floor() as i32, (pos[0] + r).floor() as i32);
    let (z0, z1) = ((pos[2] - r).floor() as i32, (pos[2] + r).floor() as i32);
    let depth = (pow / 0.5 - 1.0).min(radius);
    let y0 = (pos[1] - f64::from(depth)).floor() as i32;
    let y1 = pos[1].floor() as i32;
    for z in z0..=z1 {
        for x in x0..=x1 {
            for y in y0..=y1 {
                piece(level, pos, cam, radius, pow, sky_darken, [x, y, z], out);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn piece(
    level: &Level,
    pos: [f64; 3],
    cam: [f64; 3],
    radius: f32,
    pow: f32,
    sky_darken: i32,
    [x, y, z]: [i32; 3],
    out: &mut Vec<u8>,
) {
    let power_at_depth = pow - (pos[1] - f64::from(y)) as f32 * 0.5;
    let below = level.chunks.block(x, y - 1, z);
    // VANILLA: RenderShape.INVISIBLE (воздух, барьер, свет, structure_void, moving_piston).
    if matches!(
        below.name(),
        "air" | "cave_air" | "void_air" | "barrier" | "light" | "structure_void" | "moving_piston"
    ) {
        return;
    }
    let Some(chunk) = level.chunks.get(x >> 4, z >> 4) else {
        return;
    };
    // VANILLA: Level#getMaxLocalRawBrightness(pos) = max(blockLight, skyLight − skyDarken).
    let (lx, lz) = ((x & 15) as usize, (z & 15) as usize);
    let block = i32::from(chunk.block_light(lx, y, lz).unwrap_or(0));
    let sky = i32::from(chunk.sky_light(lx, y, lz).unwrap_or(15));
    let brightness = block.max((sky - sky_darken).max(0));
    if brightness <= 3 {
        return;
    }
    // VANILLA: belowState.isCollisionShapeFullBlock — кровать, плита и т. п. тени не получают.
    if !bi::has(below, bi::COLLISION_FULL) {
        return;
    }
    let shape = bi::outline_boxes(below);
    if shape.is_empty() {
        return;
    }
    // VoxelShape#bounds
    let mut b = [f64::MAX, f64::MAX, f64::MAX, f64::MIN, f64::MIN, f64::MIN];
    for s in shape {
        for i in 0..3 {
            b[i] = b[i].min(s[i]);
            b[i + 3] = b[i + 3].max(s[i + 3]);
        }
    }
    // Lightmap.getBrightness (ambientLight = 0, как в Overworld).
    let v = brightness as f32 / 15.0;
    let lm = v / (4.0 - 3.0 * v);
    let alpha = (power_at_depth * 0.5 * lm).clamp(0.0, 1.0);
    let (rx, ry, rz) = (
        (f64::from(x) - pos[0]) as f32,
        (f64::from(y) - pos[1]) as f32,
        (f64::from(z) - pos[2]) as f32,
    );
    let x01 = rx + b[0] as f32;
    let x11 = rx + b[3] as f32;
    let y01 = ry + b[1] as f32;
    let z01 = rz + b[2] as f32;
    let z11 = rz + b[5] as f32;
    let u0 = -x01 / 2.0 / radius + 0.5;
    let u1 = -x11 / 2.0 / radius + 0.5;
    let v0 = -z01 / 2.0 / radius + 0.5;
    let v1 = -z11 / 2.0 / radius + 0.5;
    // Позиции: относительно сущности → относительно камеры.
    let off = [
        (pos[0] - cam[0]) as f32,
        (pos[1] - cam[1]) as f32,
        (pos[2] - cam[2]) as f32,
    ];
    let c = [255, 255, 255, (alpha * 255.0).round() as u8];
    for (px, pz, u, v) in [
        (x01, z01, u0, v0),
        (x01, z11, u0, v1),
        (x11, z11, u1, v1),
        (x11, z01, u1, v0),
    ] {
        push_vertex(
            out,
            [px + off[0], y01 + off[1], pz + off[2]],
            [u, v],
            c,
            (15, 15),
        );
    }
}
