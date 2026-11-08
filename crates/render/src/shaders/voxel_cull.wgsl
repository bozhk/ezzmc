// ComputeIndirect: culling секций, один поток = одна секция. ЗДЕСЬ отсекается геометрия на этом пути:
//   1) frustum — 8 углов AABB в clip-пространстве текущего кадра;
//   2) Hi-Z — те же углы через матрицу ПРОШЛОГО кадра против Hi-Z прошлого кадра.
// Скрытая секция: в compact-режиме команда не пишется вовсе (счётчик не растёт); иначе instance_count = 0.
struct Section {
    origin: vec3<i32>,
    first_vertex: u32,
    quad_count: u32,
    _p0: u32,
    _p1: u32,
    _p2: u32,
}
struct Cmd {
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
}
struct Params {
    view_proj: mat4x4<f32>,
    prev_view_proj: mat4x4<f32>,
    cam_int: vec4<i32>,
    cam_frac: vec4<f32>,
    prev_cam_int: vec4<i32>,
    prev_cam_frac: vec4<f32>,
    section_count: u32,
    use_hiz: u32,
    hiz_mips: u32,
    _pad: u32,
}
@group(0) @binding(0) var<uniform> p: Params;
@group(0) @binding(1) var<storage, read> sections: array<Section>;
@group(0) @binding(2) var<storage, read_write> draws: array<Cmd>;
@group(0) @binding(3) var<storage, read_write> count: atomic<u32>;
@group(0) @binding(4) var hiz: texture_2d<f32>;

override COMPACT: bool = false;

fn corner(lo: vec3<f32>, i: u32) -> vec3<f32> {
    return lo + vec3<f32>(f32(i & 1u), f32((i >> 1u) & 1u), f32((i >> 2u) & 1u)) * 16.0;
}

fn frustum_visible(lo: vec3<f32>) -> bool {
    var all_out = 63u; // бит k выставлен, пока ВСЕ углы снаружи плоскости k
    for (var i = 0u; i < 8u; i++) {
        let c = p.view_proj * vec4<f32>(corner(lo, i), 1.0);
        var m = 0u;
        if c.x < -c.w { m |= 1u; }
        if c.x > c.w { m |= 2u; }
        if c.y < -c.w { m |= 4u; }
        if c.y > c.w { m |= 8u; }
        if c.z < 0.0 { m |= 16u; }
        if c.z > c.w { m |= 32u; }
        all_out &= m;
    }
    return all_out == 0u;
}

// true — секция гарантированно закрыта геометрией прошлого кадра. Любое сомнение → false (видима).
fn hiz_occluded(origin: vec3<i32>) -> bool {
    let lo = vec3<f32>(origin - p.prev_cam_int.xyz) - p.prev_cam_frac.xyz;
    var mn = vec2<f32>(1e30);
    var mx = vec2<f32>(-1e30);
    var zmin = 1e30;
    for (var i = 0u; i < 8u; i++) {
        let c = p.prev_view_proj * vec4<f32>(corner(lo, i), 1.0);
        if c.w <= 1e-4 || c.z < 0.0 { return false; } // пересекает ближнюю плоскость/позади камеры
        let n = c.xyz / c.w;
        mn = min(mn, n.xy);
        mx = max(mx, n.xy);
        zmin = min(zmin, n.z);
    }
    // частично за краем прошлого экрана: по Hi-Z судить нельзя (за краем глубины нет)
    if mn.x < -1.0 || mn.y < -1.0 || mx.x > 1.0 || mx.y > 1.0 { return false; }
    let uv_min = vec2<f32>(mn.x * 0.5 + 0.5, 0.5 - mx.y * 0.5);
    let uv_max = vec2<f32>(mx.x * 0.5 + 0.5, 0.5 - mn.y * 0.5);
    let dims0 = vec2<f32>(textureDimensions(hiz, 0));
    let size = (uv_max - uv_min) * dims0;
    // уровень на один ниже «2×2»: прямоугольник покрывает ≤ 3×3 текселей (9 загрузок), зато Hi-Z
    // точнее — на грубом уровне тексель на границе стены захватывал бы фон и давал far = 1.0
    let lv = u32(clamp(ceil(log2(max(max(size.x, size.y), 1.0))) - 1.0, 0.0, f32(p.hiz_mips - 1u)));
    let ld = vec2<i32>(textureDimensions(hiz, i32(lv)));
    let t0 = clamp(vec2<i32>(floor(uv_min * vec2<f32>(ld))), vec2<i32>(0), ld - vec2<i32>(1));
    let t1 = clamp(vec2<i32>(floor(uv_max * vec2<f32>(ld))), vec2<i32>(0), ld - vec2<i32>(1));
    var far = 0.0;
    for (var y = t0.y; y <= t1.y; y++) {
        for (var x = t0.x; x <= t1.x; x++) {
            far = max(far, textureLoad(hiz, vec2<i32>(x, y), i32(lv)).r);
        }
    }
    // ближайшая точка секции дальше самой дальней глубины в зоне → закрыта (запас на ошибки округления)
    return zmin > far + 1e-5;
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if i >= p.section_count { return; }
    let s = sections[i];
    var visible = s.quad_count > 0u;
    if visible {
        let lo = vec3<f32>(s.origin - p.cam_int.xyz) - p.cam_frac.xyz;
        visible = frustum_visible(lo);
        if visible && p.use_hiz != 0u {
            visible = !hiz_occluded(s.origin);
        }
    }
    let cmd = Cmd(s.quad_count * 6u, 1u, 0u, i32(s.first_vertex), i);
    if COMPACT {
        if visible {
            let k = atomicAdd(&count, 1u);
            draws[k] = cmd;
        }
    } else {
        draws[i] = Cmd(cmd.index_count, select(0u, 1u, visible), 0u, cmd.base_vertex, i);
    }
}
