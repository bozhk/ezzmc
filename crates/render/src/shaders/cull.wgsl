// PERF (TODO-490): frustum culling секций на GPU. Порт Frustum#isVisible (см. mesh/frustum.rs):
// AABB секции (16³) в координатах относительно камеры против 6 плоскостей, p-вершина по знаку нормали.
struct Params {
    planes: array<vec4<f32>, 6>,
    cam_int: vec4<i32>,
    cam_frac: vec4<f32>,
    slots: u32,
    max_slots: u32,
    layer_mask: u32,
    layers: u32,
}
struct Cmd {
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
}
@group(0) @binding(0) var<uniform> p: Params;
@group(0) @binding(1) var<storage, read> origins: array<vec4<i32>>;
@group(0) @binding(2) var<storage, read> flags: array<u32>;
@group(0) @binding(3) var<storage, read> src: array<Cmd>;
@group(0) @binding(4) var<storage, read_write> dst: array<Cmd>;
@group(0) @binding(5) var<storage, read_write> counts: array<atomic<u32>>;

// true: компактный вывод + счётчик (multi_draw_indirect_count); false: слот→слот, instance_count 0/1.
override COMPACT: bool = false;

fn inside(slot: u32) -> bool {
    // origin − floor(cam) точно в i32, дробная часть камеры вычитается в f32
    let lo = vec3<f32>(origins[slot].xyz - p.cam_int.xyz) - p.cam_frac.xyz;
    let hi = lo + vec3<f32>(16.0);
    for (var i = 0u; i < 6u; i++) {
        let pl = p.planes[i];
        let x = select(hi.x, lo.x, pl.x < 0.0);
        let y = select(hi.y, lo.y, pl.y < 0.0);
        let z = select(hi.z, lo.z, pl.z < 0.0);
        if pl.x * x + pl.y * y + pl.z * z + pl.w < 0.0 {
            return false;
        }
    }
    return true;
}

@compute @workgroup_size(64)
fn cull(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if slot >= p.slots {
        return;
    }
    let vis = flags[slot] != 0u && inside(slot);
    for (var l = 0u; l < p.layers; l++) {
        if (p.layer_mask & (1u << l)) == 0u {
            continue;
        }
        var c = src[l * p.max_slots + slot];
        if COMPACT {
            if vis && c.index_count > 0u {
                let i = atomicAdd(&counts[l], 1u);
                dst[l * p.max_slots + i] = c;
            }
        } else {
            if !vis {
                c.instance_count = 0u;
            }
            dst[l * p.max_slots + slot] = c;
        }
    }
}
