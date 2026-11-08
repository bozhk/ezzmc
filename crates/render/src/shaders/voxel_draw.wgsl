// Отрисовка секций: вершины тянутся из storage (vertex pulling), индексы — общий шаблон квадов.
// Раскладка VoxelVertex — см. voxel_vertex.rs. Origin секции — по instance_index (= слот).
struct Section {
    origin: vec3<i32>,
    first_vertex: u32,
    quad_count: u32,
    _p0: u32,
    _p1: u32,
    _p2: u32,
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
struct QuadMeta { texture: u32, tint: u32 }

@group(0) @binding(0) var<uniform> p: Params;
@group(0) @binding(1) var<storage, read> sections: array<Section>;
@group(0) @binding(5) var<storage, read> vertices: array<u32>;
@group(0) @binding(6) var<storage, read> quads: array<QuadMeta>;
@group(0) @binding(7) var<storage, read> rects: array<vec4<f32>>; // u0, v0, u1, v1 по texture id
@group(0) @binding(8) var atlas: texture_2d<f32>;
@group(0) @binding(9) var atlas_samp: sampler;

struct Fog {
    color: vec4<f32>,
    env_start: f32,
    env_end: f32,
    rd_start: f32,
    rd_end: f32,
    sky_end: f32,
    clouds_end: f32,
}
@group(0) @binding(10) var<uniform> fog: Fog;
@group(0) @binding(11) var lightmap: texture_2d<f32>;
@group(0) @binding(12) var lightmap_samp: sampler;

// VANILLA: fog.glsl#linear_fog_value / apply_fog (как в terrain.wgsl)
fn linear_fog_value(d: f32, start: f32, end: f32) -> f32 {
    if (d <= start) { return 0.0; }
    if (d >= end) { return 1.0; }
    return (d - start) / (end - start);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) rel: vec3<f32>,
}

// Ванильные направленные тени граней: down, up, north, south, west, east.
const FACE_SHADE = array<f32, 6>(0.5, 1.0, 0.8, 0.8, 0.6, 0.6);
// VANILLA-UNSURE: AO в 2 битах — ступени 1.0..0.4, ваниль усредняет 4 соседа дробно (0.2..1.0).
const AO_SHADE = array<f32, 4>(0.4, 0.6, 0.8, 1.0);
const CORNER_UV = array<vec2<f32>, 4>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 1.0));

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, @builtin(instance_index) slot: u32) -> VsOut {
    let v = vertices[vi];
    let local = vec3<f32>(f32(v & 31u), f32((v >> 5u) & 31u), f32((v >> 10u) & 31u));
    let face = (v >> 15u) & 7u;
    let ao = (v >> 18u) & 3u;
    let corner = (v >> 28u) & 3u;
    let rel = vec3<f32>(sections[slot].origin - p.cam_int.xyz) - p.cam_frac.xyz + local;
    let q = quads[vi / 4u];
    let r = rects[q.texture];
    let t = CORNER_UV[corner];
    let tint = unpack4x8unorm(q.tint);
    // VANILLA: terrain.vsh#minecraft_sample_lightmap — уровень L (0..15) → (L+0.5)/16 (= l*16/256 + 0.5/16).
    let lm_uv = vec2<f32>(f32((v >> 20u) & 15u) + 0.5, f32((v >> 24u) & 15u) + 0.5) / 16.0;
    let light = textureSampleLevel(lightmap, lightmap_samp, lm_uv, 0.0).rgb;
    let shade = FACE_SHADE[min(face, 5u)] * AO_SHADE[ao];
    var o: VsOut;
    o.pos = p.view_proj * vec4<f32>(rel, 1.0);
    o.rel = rel;
    o.uv = mix(r.xy, r.zw, t);
    o.color = vec4<f32>(tint.rgb * shade * light, tint.a);
    return o;
}

// VANILLA: terrain.fsh#sampleNearest (как в terrain.wgsl)
fn sample_nearest_g(uv_in: vec2<f32>, pixel_size: vec2<f32>, texel_screen_size: vec2<f32>, du: vec2<f32>, dv: vec2<f32>) -> vec4<f32> {
    let uv_texel = uv_in / pixel_size;
    let texel_center = round(uv_texel) - 0.5;
    var texel_offset = uv_texel - texel_center;
    texel_offset = (texel_offset - 0.5) * pixel_size / texel_screen_size + 0.5;
    texel_offset = clamp(texel_offset, vec2<f32>(0.0), vec2<f32>(1.0));
    let uv = (texel_center + texel_offset) * pixel_size;
    return textureSampleGrad(atlas, atlas_samp, uv, du, dv);
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let pixel_size = 1.0 / vec2<f32>(textureDimensions(atlas, 0));
    let du = dpdx(i.uv);
    let dv = dpdy(i.uv);
    let c = sample_nearest_g(i.uv, pixel_size, sqrt(du * du + dv * dv), du, dv);
    if c.a < 0.5 { discard; } // VANILLA: cutout-порог 0.5 (солид-текстуры непрозрачны)
    let lit = c.rgb * i.color.rgb;
    let fv = max(
        linear_fog_value(length(i.rel), fog.env_start, fog.env_end),
        linear_fog_value(max(length(i.rel.xz), abs(i.rel.y)), fog.rd_start, fog.rd_end),
    );
    return vec4<f32>(mix(lit, fog.color.rgb, fv * fog.color.a), 1.0);
}
