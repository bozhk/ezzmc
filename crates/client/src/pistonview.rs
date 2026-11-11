//! Рендер движущихся блоков поршня и коллизия `moving_piston`.
//! VANILLA: PistonHeadRenderer#extractRenderState/submit (+ MovingBlockRenderState),
//! PistonMovingBlockEntity#getCollisionShape.
use ezzmc_mesh::lighting::{ao_light, calculate_shape, flat_light, with_emission, QuadLight};
use ezzmc_mesh::models::BlockModels;
use ezzmc_mesh::padded::Padded;
use ezzmc_mesh::section::{ModelProvider, Neighborhood, SectionPos};
use ezzmc_protocol::types::BlockPos;
use ezzmc_render::particledraw::push_vertex_raw;
use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;
use ezzmc_world::piston::{opposite, step, MovingPiston, DIRS};

fn state_with(name: &str, props: &[(&str, &str)]) -> Option<BlockState> {
    use ezzmc_world::generated::b26_3::BLOCKS;
    let mut s = BlockState::from_id(BLOCKS.iter().find(|b| b.name == name)?.default_state)?;
    for (k, v) in props {
        s = s.with(k, v)?;
    }
    Some(s)
}

fn bool_s(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

/// Блоки к отрисовке: `(состояние, позиция для света/тинта, двигать ли по смещению)`.
fn parts(e: &MovingPiston, pos: BlockPos, p: f32) -> Vec<(BlockState, BlockPos, bool)> {
    let moved = e.moved;
    if moved.is_air() {
        return vec![];
    }
    let md = e.movement_dir();
    let s = step(opposite(md));
    let at = BlockPos {
        x: pos.x + s[0],
        y: pos.y + s[1],
        z: pos.z + s[2],
    };
    if moved.name() == "piston_head" && e.get_progress(p) <= 4.0 {
        let st = moved
            .with("short", bool_s(e.get_progress(p) <= 0.5))
            .unwrap_or(moved);
        vec![(st, at, true)]
    } else if e.source && !e.extending {
        let ty = if moved.name() == "sticky_piston" {
            "sticky"
        } else {
            "normal"
        };
        let facing = moved.property("facing").unwrap_or("north");
        let head = state_with(
            "piston_head",
            &[
                ("type", ty),
                ("facing", facing),
                ("short", bool_s(e.get_progress(p) >= 0.5)),
            ],
        );
        let ms = step(md);
        let base_pos = BlockPos {
            x: at.x + ms[0],
            y: at.y + ms[1],
            z: at.z + ms[2],
        };
        let base = moved.with("extended", "true");
        let mut v = vec![];
        if let Some(h) = head {
            v.push((h, at, true));
        }
        if let Some(b) = base {
            v.push((b, base_pos, false));
        }
        v
    } else {
        vec![(moved, at, true)]
    }
}

fn hood(level: &Level, cx: i32, cz: i32) -> Neighborhood {
    let mut h = Neighborhood::default();
    for dz in -1..=1 {
        for dx in -1..=1 {
            h.chunks[((dz + 1) * 3 + dx + 1) as usize] =
                level.chunks.get(cx + dx, cz + dz).cloned();
        }
    }
    h
}

/// Вершины движущихся блоков (атлас блоков), позиции относительно камеры.
pub fn bake(level: &Level, models: &BlockModels, cam: [f64; 3], partial: f32, out: &mut Vec<u8>) {
    let cardinal = models.cardinal();
    for (&(x, y, z), e) in &level.pistons {
        let pos = BlockPos { x, y, z };
        let off = e.offset(partial);
        for (state, lpos, moves) in parts(e, pos, partial) {
            let o = if moves { off } else { [0.0; 3] };
            let mut ps = Vec::new();
            models.parts(state, lpos.x, lpos.y, lpos.z, &mut ps);
            let (cx, cz) = (lpos.x >> 4, lpos.z >> 4);
            let hd = hood(level, cx, cz);
            let base = SectionPos {
                x: cx,
                y: lpos.y >> 4,
                z: cz,
            };
            let mut pad = Padded::fill(&hd, base);
            let lp = [lpos.x - cx * 16, lpos.y - base.y * 16, lpos.z - cz * 16];
            pad.isolate(lp, state);
            let emission = state.props().map_or(0, |s| s.light_emission);
            let smooth = models.smooth_lighting();
            for part in ps {
                let use_ao = smooth && emission == 0 && part.ao;
                let qc = part.quads;
                let all = qc.unculled.iter().map(|q| (q, None)).chain(
                    qc.culled
                        .iter()
                        .zip(ezzmc_assets::model::Direction::ALL)
                        .flat_map(|(v, d)| v.iter().map(move |q| (q, Some(d)))),
                );
                for (q, cull) in all {
                    let info = calculate_shape(state, q);
                    // VANILLA: ModelBlockRenderer — AO / плоский свет как у террейна.
                    let ql: QuadLight = if use_ao {
                        ao_light(
                            &pad,
                            state,
                            lp,
                            q.direction,
                            q.shade_direction_override,
                            &info,
                            &cardinal,
                        )
                    } else {
                        flat_light(&pad, state, lp, cull, q, &info, &cardinal)
                    };
                    let t = if q.is_tinted() {
                        models.tint(state, q.tint_index, &hd, base, lpos.x, lpos.y, lpos.z)
                    } else {
                        [255; 3]
                    };
                    for i in 0..4 {
                        let b = ql.brightness[i];
                        let c = [
                            (f32::from(t[0]) * b) as u8,
                            (f32::from(t[1]) * b) as u8,
                            (f32::from(t[2]) * b) as u8,
                            255,
                        ];
                        let l = with_emission(ql.lightmap[i], q.light_emission);
                        let light = ((l & 0xFFFF) as u16, ((l >> 16) & 0xFFFF) as u16);
                        let p = [
                            (f64::from(pos.x) + f64::from(o[0]) + f64::from(q.pos[i][0]) - cam[0])
                                as f32,
                            (f64::from(pos.y) + f64::from(o[1]) + f64::from(q.pos[i][1]) - cam[1])
                                as f32,
                            (f64::from(pos.z) + f64::from(o[2]) + f64::from(q.pos[i][2]) - cam[2])
                                as f32,
                        ];
                        push_vertex_raw(out, p, q.uv[i], c, light);
                    }
                }
            }
        }
    }
}

/// VANILLA: PistonMovingBlockEntity#getCollisionShape (боксы в координатах блока).
pub fn collision_boxes(e: &MovingPiston, noclip: Option<u8>) -> Vec<[f64; 6]> {
    use ezzmc_mesh::blockinfo::collision_boxes_of;
    let shape_of = |s: BlockState| -> Vec<[f64; 6]> {
        s.props()
            .map_or(vec![], |p| collision_boxes_of(p.collision_shape).to_vec())
    };
    let mut out = Vec::new();
    // голова поршня для втягивающегося источника
    if !e.extending && e.source && e.moved.name().ends_with("piston") {
        if let Some(b) = e.moved.with("extended", "true") {
            out.extend(shape_of(b));
        }
    }
    // VANILLA: NOCLIP == movement direction && progress < 1 → только голова втягивающегося источника
    if e.progress < 1.0 && noclip == Some(e.movement_dir()) {
        return out;
    }
    let ep = f64::from(e.extended_progress(e.progress));
    let st = if e.source {
        let facing = DIRS[usize::from(e.dir)];
        let short = e.extending != (1.0 - e.progress < 0.25);
        state_with(
            "piston_head",
            &[("facing", facing), ("short", bool_s(short))],
        )
    } else {
        Some(e.moved)
    };
    let s = step(e.dir);
    let d = [
        f64::from(s[0]) * ep,
        f64::from(s[1]) * ep,
        f64::from(s[2]) * ep,
    ];
    if let Some(st) = st {
        for b in shape_of(st) {
            out.push([
                b[0] + d[0],
                b[1] + d[1],
                b[2] + d[2],
                b[3] + d[0],
                b[4] + d[1],
                b[5] + d[2],
            ]);
        }
    }
    out
}

/// Коллайдер с NOCLIP поршня (VANILLA: PistonMovingBlockEntity#NOCLIP).
struct NoClip<'a> {
    level: &'a Level,
    dir: u8,
}

impl crate::physics::Collider for NoClip<'_> {
    fn boxes(&self, x: i32, y: i32, z: i32) -> &'static [[f64; 6]] {
        crate::physics::Collider::boxes(self.level, x, y, z)
    }
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str {
        crate::physics::Collider::name(self.level, x, y, z)
    }
    fn extra_boxes(&self, x: i32, y: i32, z: i32) -> Vec<[f64; 6]> {
        match self.level.pistons.get(&(x, y, z)) {
            Some(e) if self.level.chunks.block(x, y, z).name() == "moving_piston" => {
                collision_boxes(e, Some(self.dir))
            }
            _ => Vec::new(),
        }
    }
    fn has_chunk(&self, x: i32, z: i32) -> bool {
        crate::physics::Collider::has_chunk(self.level, x, z)
    }
}

fn ov(a: &[f64; 6], b: &[f64; 6]) -> bool {
    a[0] < b[3] && a[3] > b[0] && a[1] < b[4] && a[4] > b[1] && a[2] < b[5] && a[5] > b[2]
}

fn axis_sign(d: u8) -> (usize, f64) {
    let s = step(d);
    let i = (0..3).find(|&i| s[i] != 0).unwrap_or(1);
    (i, f64::from(s[i]))
}

/// VANILLA: PistonMath#getMovementArea.
fn movement_area(b: [f64; 6], d: u8, amount: f64) -> [f64; 6] {
    let (i, sg) = axis_sign(d);
    let delta = amount * sg;
    let (mn, mx) = (delta.min(0.0), delta.max(0.0));
    let mut r = b;
    let base = if sg > 0.0 { b[i + 3] } else { b[i] };
    r[i] = base + mn;
    r[i + 3] = base + mx;
    r
}

/// VANILLA: PistonMovingBlockEntity#getMovement.
fn get_movement(outside: &[f64; 6], d: u8, bb: &[f64; 6]) -> f64 {
    let (i, sg) = axis_sign(d);
    if sg > 0.0 {
        outside[i + 3] - bb[i]
    } else {
        bb[i + 3] - outside[i]
    }
}

fn mv(b: [f64; 6], o: [f64; 3]) -> [f64; 6] {
    [
        b[0] + o[0],
        b[1] + o[1],
        b[2] + o[2],
        b[3] + o[0],
        b[4] + o[1],
        b[5] + o[2],
    ]
}

fn move_by_piston(level: &Level, pl: &mut crate::physics::Player, noclip: u8, delta: f64, dir: u8) {
    let s = step(dir);
    let m = [
        delta * f64::from(s[0]),
        delta * f64::from(s[1]),
        delta * f64::from(s[2]),
    ];
    pl.piston_move(&NoClip { level, dir: noclip }, m, level.game_time);
}

/// VANILLA: PistonMovingBlockEntity#moveCollidedEntities + moveStuckEntities для локального игрока
/// (вызывать перед `Level::tick_pistons`, для каждого клиентского тика).
pub fn push_player(level: &Level, pl: &mut crate::physics::Player) {
    use ezzmc_mesh::blockinfo::collision_boxes_of;
    let shape_of = |s: BlockState| -> Vec<[f64; 6]> {
        s.props()
            .map_or(vec![], |p| collision_boxes_of(p.collision_shape).to_vec())
    };
    let keys: Vec<_> = level.pistons.keys().copied().collect();
    for k in keys {
        let Some(e) = level.pistons.get(&k) else {
            continue;
        };
        if e.progress >= 1.0 || level.chunks.block(k.0, k.1, k.2).name() != "moving_piston" {
            continue;
        }
        let movement = e.movement_dir();
        let dp = f64::from((e.progress + 0.5) - e.progress);
        let ep = f64::from(e.extended_progress(e.progress));
        let ds = step(e.dir);
        let off = [
            f64::from(k.0) + ep * f64::from(ds[0]),
            f64::from(k.1) + ep * f64::from(ds[1]),
            f64::from(k.2) + ep * f64::from(ds[2]),
        ];
        // getCollisionRelatedBlockState
        let rel = if !e.extending && e.source && e.moved.name().ends_with("piston") {
            state_with(
                "piston_head",
                &[
                    ("short", bool_s(e.progress > 0.25)),
                    ("facing", DIRS[usize::from(e.dir)]),
                ],
            )
        } else {
            Some(e.moved)
        };
        let shapes = rel.map(shape_of).unwrap_or_default();
        if !shapes.is_empty() {
            let mut bounds = shapes[0];
            for b in &shapes {
                for i in 0..3 {
                    bounds[i] = bounds[i].min(b[i]);
                    bounds[i + 3] = bounds[i + 3].max(b[i + 3]);
                }
            }
            let aabb = mv(bounds, off);
            let area = movement_area(aabb, movement, dp);
            let player = pl.aabb_pub();
            if ov(&player, &area) || ov(&player, &aabb) {
                if e.moved.name() == "slime_block" {
                    let (i, sg) = axis_sign(movement);
                    pl.vel[i] = sg;
                }
                let mut delta = 0.0f64;
                for sb in &shapes {
                    let moving = movement_area(mv(*sb, off), movement, dp);
                    let pb = pl.aabb_pub();
                    if ov(&moving, &pb) {
                        delta = delta.max(get_movement(&moving, movement, &pb));
                        if delta >= dp {
                            break;
                        }
                    }
                }
                if delta > 0.0 {
                    delta = delta.min(dp) + 0.01;
                    move_by_piston(level, pl, movement, delta, movement);
                    if !e.extending && e.source {
                        // fixEntityWithinPistonBase
                        let eb = pl.aabb_pub();
                        let bx = [
                            f64::from(k.0),
                            f64::from(k.1),
                            f64::from(k.2),
                            f64::from(k.0) + 1.0,
                            f64::from(k.1) + 1.0,
                            f64::from(k.2) + 1.0,
                        ];
                        if ov(&eb, &bx) {
                            let opp = opposite(movement);
                            let d1 = get_movement(&bx, opp, &eb) + 0.01;
                            let mut inter = eb;
                            for i in 0..3 {
                                inter[i] = eb[i].max(bx[i]);
                                inter[i + 3] = eb[i + 3].min(bx[i + 3]);
                            }
                            let d2 = get_movement(&bx, opp, &inter) + 0.01;
                            if (d1 - d2).abs() < 0.01 {
                                let d = d1.min(dp) + 0.01;
                                move_by_piston(level, pl, movement, d, opp);
                            }
                        }
                    }
                }
            }
        }
        // moveStuckEntities (мёд)
        if e.moved.name() == "honey_block" && axis_sign(movement).0 != 1 && pl.on_ground {
            let top = shape_of(e.moved).iter().fold(0.0f64, |m, b| m.max(b[4]));
            let aabb = mv([0.0, top, 0.0, 1.0, 1.500_001, 1.0], off);
            let pb = pl.aabb_pub();
            let (cx, cz) = (pl.pos[0], pl.pos[2]);
            if ov(&pb, &aabb) && cx >= aabb[0] && cx <= aabb[3] && cz >= aabb[2] && cz <= aabb[5] {
                move_by_piston(level, pl, movement, dp, movement);
            }
        }
    }
}

/// VANILLA: TntRenderer#submit — блок tnt вокруг центра (0,0.5,0), при fuse<10 раздувается
/// на getSwellAmount; при isLit (fuse/5 чётное) — белый сплошной (OverlayTexture белый).
/// VANILLA-UNSURE: белый оверлей (alpha≈0.7) заменён сплошной белой текстурой.
pub fn bake_tnt(
    level: &Level,
    models: &BlockModels,
    cam: [f64; 3],
    partial: f32,
    out: &mut Vec<u8>,
    sky_darken: i32,
    shadow: &mut Vec<u8>,
) {
    let Some(tnt_ty) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "tnt")
    else {
        return;
    };
    let Some(tnt) = state_with("tnt", &[]) else {
        return;
    };
    let white_uv = state_with("white_concrete", &[]).and_then(|w| {
        let mut ps = Vec::new();
        models.parts(w, 0, 0, 0, &mut ps);
        ps.iter().find_map(|p| {
            p.quads
                .unculled
                .iter()
                .chain(p.quads.culled.iter().flatten())
                .next()
                .map(|q| {
                    let c = |i: usize| q.uv.iter().map(|u| u[i]).sum::<f32>() / 4.0;
                    [c(0), c(1)]
                })
        })
    });
    let cardinal = models.cardinal();
    let mut ps = Vec::new();
    models.parts(tnt, 0, 0, 0, &mut ps);
    for e in level.entities.iter().filter(|e| e.ty == tnt_ty) {
        let lerp = |a: f64, b: f64| a + (b - a) * f64::from(partial);
        let pos = [
            lerp(e.prev_pos[0], e.pos[0]),
            lerp(e.prev_pos[1], e.pos[1]),
            lerp(e.prev_pos[2], e.pos[2]),
        ];
        let fuse = e.fuse as f32 - partial + 1.0;
        let scale = if fuse < 10.0 {
            let g = (1.0 - fuse / 10.0).clamp(0.0, 1.0);
            1.0 + g * g * g * g * 0.3
        } else {
            1.0
        };
        let lit = fuse >= 0.0 && ((fuse / 5.0) as i32) % 2 == 0;
        // VANILLA: TntRenderer — shadowRadius 0.5.
        crate::entityshadow::bake(level, pos, cam, 0.5, 1.0, sky_darken, shadow);
        let (bl, sl) = crate::particles::light_at(level, pos[0], pos[1] + 0.5, pos[2]);
        // VANILLA: белый оверлей (u=15 → альфа 0.25): итог = 0.75 белого + 0.25 освещённой текстуры.
        let light = if lit {
            (15 * 16, 15 * 16)
        } else {
            (u16::from(bl) * 16, u16::from(sl) * 16)
        };
        for part in &ps {
            let qc = &part.quads;
            let all = qc.unculled.iter().chain(qc.culled.iter().flatten());
            for q in all {
                let shade = ezzmc_mesh::lighting::get_shade(
                    &cardinal,
                    q.direction,
                    q.shade_direction_override,
                );
                let v = (255.0 * if lit { 0.75 + 0.25 * shade } else { shade }) as u8;
                let c = [v, v, v, 255];
                for i in 0..4 {
                    let uv = match (lit, white_uv) {
                        (true, Some(w)) => w,
                        _ => q.uv[i],
                    };
                    let p = [
                        (pos[0] + f64::from(scale) * (f64::from(q.pos[i][0]) - 0.5) - cam[0])
                            as f32,
                        (pos[1] + 0.5 + f64::from(scale) * (f64::from(q.pos[i][1]) - 0.5) - cam[1])
                            as f32,
                        (pos[2] + f64::from(scale) * (f64::from(q.pos[i][2]) - 0.5) - cam[2])
                            as f32,
                    ];
                    push_vertex_raw(out, p, uv, c, light);
                }
            }
        }
    }
}

/// VANILLA: FallingBlockRenderer#submit — блок целиком (без отсечения граней), сдвиг (-0.5,0,-0.5)
/// от низа сущности; не рисуется, пока блок в её клетке уже совпадает с состоянием (приземлился).
pub fn bake_falling(
    level: &Level,
    models: &BlockModels,
    cam: [f64; 3],
    partial: f32,
    out: &mut Vec<u8>,
    sky_darken: i32,
    shadow: &mut Vec<u8>,
) {
    let Some(ty) = ezzmc_protocol::generated::r26_3::ENTITY_TYPE
        .iter()
        .position(|n| *n == "falling_block")
    else {
        return;
    };
    let cardinal = models.cardinal();
    for e in level.entities.iter().filter(|e| e.ty == ty) {
        let Some(state) = u32::try_from(e.data).ok().and_then(BlockState::from_id) else {
            continue;
        };
        if state == BlockState::AIR {
            continue;
        }
        let lerp = |a: f64, b: f64| a + (b - a) * f64::from(partial);
        let pos = [
            lerp(e.prev_pos[0], e.pos[0]),
            lerp(e.prev_pos[1], e.pos[1]),
            lerp(e.prev_pos[2], e.pos[2]),
        ];
        // shouldRender: getBlockState() != level.getBlockState(blockPosition()).
        if level.chunks.block(
            e.pos[0].floor() as i32,
            e.pos[1].floor() as i32,
            e.pos[2].floor() as i32,
        ) == state
        {
            continue;
        }
        // MovingBlockRenderState.blockPos = (x, boundingBox.maxY, z): по ней берётся свет/AO/тинт.
        let lpos = BlockPos {
            x: e.pos[0].floor() as i32,
            y: (e.pos[1] + 0.98).floor() as i32,
            z: e.pos[2].floor() as i32,
        };
        let mut ps = Vec::new();
        models.parts(state, lpos.x, lpos.y, lpos.z, &mut ps);
        // VANILLA: FallingBlockRenderer — shadowRadius 0.5.
        crate::entityshadow::bake(level, pos, cam, 0.5, 1.0, sky_darken, shadow);
        let (cx, cz) = (lpos.x >> 4, lpos.z >> 4);
        let hd = hood(level, cx, cz);
        let base = SectionPos {
            x: cx,
            y: lpos.y >> 4,
            z: cz,
        };
        let mut pad = Padded::fill(&hd, base);
        let lp = [lpos.x - cx * 16, lpos.y - base.y * 16, lpos.z - cz * 16];
        pad.isolate(lp, state);
        let emission = state.props().map_or(0, |s| s.light_emission);
        let smooth = models.smooth_lighting();
        for part in ps {
            let use_ao = smooth && emission == 0 && part.ao;
            let qc = part.quads;
            let all = qc.unculled.iter().map(|q| (q, None)).chain(
                qc.culled
                    .iter()
                    .zip(ezzmc_assets::model::Direction::ALL)
                    .flat_map(|(v, d)| v.iter().map(move |q| (q, Some(d)))),
            );
            for (q, cull) in all {
                let info = calculate_shape(state, q);
                // VANILLA: ModelBlockRenderer — AO / плоский свет как у террейна.
                let ql: QuadLight = if use_ao {
                    ao_light(
                        &pad,
                        state,
                        lp,
                        q.direction,
                        q.shade_direction_override,
                        &info,
                        &cardinal,
                    )
                } else {
                    flat_light(&pad, state, lp, cull, q, &info, &cardinal)
                };
                let t = if q.is_tinted() {
                    models.tint(state, q.tint_index, &hd, base, lpos.x, lpos.y, lpos.z)
                } else {
                    [255; 3]
                };
                for i in 0..4 {
                    let b = ql.brightness[i];
                    let c = [
                        (f32::from(t[0]) * b) as u8,
                        (f32::from(t[1]) * b) as u8,
                        (f32::from(t[2]) * b) as u8,
                        255,
                    ];
                    let l = with_emission(ql.lightmap[i], q.light_emission);
                    let light = ((l & 0xFFFF) as u16, ((l >> 16) & 0xFFFF) as u16);
                    // model.parts дал координаты блока 0..1 (без мировой позиции): сдвиг (-0.5, 0, -0.5).
                    let p = [
                        (pos[0] + f64::from(q.pos[i][0]) - 0.5 - cam[0]) as f32,
                        (pos[1] + f64::from(q.pos[i][1]) - cam[1]) as f32,
                        (pos[2] + f64::from(q.pos[i][2]) - 0.5 - cam[2]) as f32,
                    ];
                    push_vertex_raw(out, p, q.uv[i], c, light);
                }
            }
        }
    }
}
