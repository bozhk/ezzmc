// GPU-скиннинг мобов: статическая сетка + матрицы частей на экземпляр (storage).
// Результат совпадает с запеканием на CPU (Model::bake): VANILLA entity.vsh/fsh + Lighting (Entry.LEVEL).
// Экземпляр = `stride` vec4: parts*4 (колонки матриц частей относительно камеры) + свет (block, sky) 0..15.
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;
@group(1) @binding(2) var lightmap: texture_2d<f32>;
@group(1) @binding(3) var glint_tex: texture_2d<f32>;
@group(1) @binding(4) var glint_sampler: sampler;

struct SkinCfg {
    stride: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}
@group(2) @binding(0) var<storage, read> inst: array<vec4<f32>>;
@group(2) @binding(1) var<uniform> cfg: SkinCfg;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) spherical: f32,
    @location(1) cylindrical: f32,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) light: vec4<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) part: u32,
    @builtin(instance_index) ii: u32,
) -> VsOut {
    let first = ii * cfg.stride;
    let b = first + part * 4u;
    let m = mat4x4<f32>(inst[b], inst[b + 1u], inst[b + 2u], inst[b + 3u]);
    let world = (m * vec4<f32>(position, 1.0)).xyz;
    // Lighting.Entry.LEVEL: minecraft_mix_light, затем (255*shade) as u8 / 255.
    let n = normalize((m * vec4<f32>(normal, 0.0)).xyz);
    let l0 = normalize(vec3<f32>(0.2, 1.0, -0.7));
    let l1 = normalize(vec3<f32>(-0.2, 1.0, 0.7));
    let shade = min(1.0, (max(dot(l0, n), 0.0) + max(dot(l1, n), 0.0)) * 0.6 + 0.4);
    let c = floor(255.0 * shade) / 255.0;
    let lv = inst[first + cfg.stride - 1u];
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(world, 1.0);
    o.spherical = fog_spherical_distance(world);
    o.cylindrical = fog_cylindrical_distance(world);
    o.uv = uv0;
    o.color = vec4<f32>(c, c, c, 1.0);
    o.light = textureLoad(lightmap, vec2<i32>(i32(lv.x), i32(lv.y)), 0);
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    var color = textureSample(tex, tex_sampler, i.uv) * i.color * dyn_tf.color_modulator;
    color = color * i.light;
    if (color.a < 0.1) {
        discard;
    }
    return apply_fog(color, i.spherical, i.cylindrical, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end, fog.color);
}
