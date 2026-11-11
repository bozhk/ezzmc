// VANILLA: core/terrain.vsh + terrain.fsh (1.21.11). Вершины упакованы (ezzmc_mesh::vertex, 16 байт); origin секции — из storage-буфера по instance_index (TODO-144).
// Конвейеры solid/cutout/translucent различаются только override alpha_cutout (0 = выключено).
override alpha_cutout: f32 = 0.0;

@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(2) var<uniform> globals: Globals;
@group(0) @binding(4) var<uniform> chunk: ChunkSection;
// origin секции в блоках (xyz), индекс = слот секции = first_instance индирект-команды
@group(0) @binding(8) var<storage, read> section_origins: array<vec4<i32>>;
@group(1) @binding(0) var atlas: texture_2d<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;
@group(1) @binding(2) var lightmap: texture_2d<f32>;
@group(1) @binding(3) var lightmap_sampler: sampler;

struct VsIn {
    @builtin(instance_index) slot: u32,
    @location(0) pos: vec4<u32>,     // Uint16x4: (coord+8)*2048 ×3, слово света block | sky<<8
    @location(1) uv0: vec2<u32>,     // Uint16x2, ÷65536
    @location(2) color: vec4<f32>,   // Unorm8x4
}

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) spherical: f32,
    @location(1) cylindrical: f32,
    @location(2) color: vec4<f32>,
    @location(3) uv: vec2<f32>,
}

// VANILLA: terrain.vsh#minecraft_sample_lightmap
fn sample_lightmap(uv: vec2<i32>) -> vec4<f32> {
    let p = clamp(vec2<f32>(uv) / 256.0 + 0.5 / 16.0, vec2<f32>(0.5 / 16.0), vec2<f32>(15.5 / 16.0));
    return textureSampleLevel(lightmap, lightmap_sampler, p, 0.0);
}

@vertex
fn vs_main(v: VsIn) -> VsOut {
    var o: VsOut;
    let local = vec3<f32>(v.pos.xyz) / 2048.0 - 8.0;
    let origin = section_origins[v.slot].xyz;
    let pos = local + vec3<f32>(origin - globals.camera_block_pos) + globals.camera_offset;
    o.clip = projection.proj_mat * chunk.model_view_mat * vec4<f32>(pos, 1.0);
    o.spherical = fog_spherical_distance(pos);
    o.cylindrical = fog_cylindrical_distance(pos);
    o.color = v.color * sample_lightmap(vec2<i32>(i32(v.pos.w & 255u), i32(v.pos.w >> 8u)));
    o.uv = vec2<f32>(v.uv0) / 65536.0;
    return o;
}

// VANILLA: terrain.fsh#sampleNearest
fn sample_nearest_g(uv_in: vec2<f32>, pixel_size: vec2<f32>, du: vec2<f32>, dv: vec2<f32>, texel_screen_size: vec2<f32>) -> vec4<f32> {
    let uv_texel = uv_in / pixel_size;
    let texel_center = round(uv_texel) - 0.5;
    var texel_offset = uv_texel - texel_center;
    texel_offset = (texel_offset - 0.5) * pixel_size / texel_screen_size + 0.5;
    texel_offset = clamp(texel_offset, vec2<f32>(0.0), vec2<f32>(1.0));
    let uv = (texel_center + texel_offset) * pixel_size;
    return textureSampleGrad(atlas, atlas_sampler, uv, du, dv);
}

// VANILLA: terrain.fsh#sampleRGSS (Rotated Grid Super-Sampling)
fn sample_rgss(uv: vec2<f32>, pixel_size: vec2<f32>, du: vec2<f32>, dv: vec2<f32>) -> vec4<f32> {
    let texel_screen_size = sqrt(du * du + dv * dv);
    let max_texel_size = max(texel_screen_size.x, texel_screen_size.y);
    let min_pixel_size = min(pixel_size.x, pixel_size.y);
    let blend_factor = smoothstep(min_pixel_size * 1.0, min_pixel_size * 2.0, max_texel_size);

    let du_len = length(du);
    let dv_len = length(dv);
    let effective = sqrt(min(du_len, dv_len) * max(du_len, dv_len));
    let mip_exact = max(0.0, log2(effective / min_pixel_size));
    let mip_low = floor(mip_exact);
    let mip_high = mip_low + 1.0;
    let mip_blend = fract(mip_exact);

    var offsets = array<vec2<f32>, 4>(
        vec2<f32>(0.125, 0.375),
        vec2<f32>(-0.125, -0.375),
        vec2<f32>(0.375, -0.125),
        vec2<f32>(-0.375, 0.125),
    );
    var low = vec4<f32>(0.0);
    var high = vec4<f32>(0.0);
    for (var i = 0; i < 4; i++) {
        let s = uv + offsets[i] * pixel_size;
        low += textureSampleLevel(atlas, atlas_sampler, s, mip_low);
        high += textureSampleLevel(atlas, atlas_sampler, s, mip_high);
    }
    low *= 0.25;
    high *= 0.25;
    let rgss = mix(low, high, mip_blend);
    let nearest = sample_nearest_g(uv, pixel_size, du, dv, texel_screen_size);
    return mix(nearest, rgss, blend_factor);
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let pixel_size = 1.0 / vec2<f32>(chunk.texture_size);
    let du = dpdx(i.uv);
    let dv = dpdy(i.uv);
    var tex: vec4<f32>;
    if (globals.use_rgss == 1) {
        tex = sample_rgss(i.uv, pixel_size, du, dv);
    } else {
        tex = sample_nearest_g(i.uv, pixel_size, du, dv, sqrt(du * du + dv * dv));
    }
    var color = tex * i.color;
    color = mix(fog.color * vec4<f32>(1.0, 1.0, 1.0, color.a), color, chunk.chunk_visibility);
    if (alpha_cutout > 0.0 && color.a < alpha_cutout) {
        discard;
    }
    return apply_fog(color, i.spherical, i.cylindrical, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end, fog.color);
}
