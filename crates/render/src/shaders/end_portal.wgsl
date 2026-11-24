// VANILLA: core/rendertype_end_portal.vsh + fsh (26.3), pipelines END_PORTAL (15 слоёв) и END_GATEWAY (16).
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
/// x — GameTime (доля суток, 0..1), остальное — выравнивание.
@group(0) @binding(4) var<uniform> portal_time: vec4<f32>;
@group(1) @binding(0) var sky_tex: texture_2d<f32>;
@group(1) @binding(1) var portal_tex: texture_2d<f32>;
@group(1) @binding(2) var tex_sampler: sampler;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) tex_proj: vec4<f32>,
    @location(1) spherical: f32,
    @location(2) cylindrical: f32,
    @location(3) @interpolate(flat) layers: f32,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) layers: f32) -> VsOut {
    var o: VsOut;
    let p = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.clip = p;
    // projection_from_position
    var t = p * 0.5;
    t = vec4<f32>(t.x + t.w, t.y + t.w, p.z, p.w);
    o.tex_proj = t;
    o.spherical = fog_spherical_distance(position);
    o.cylindrical = fog_cylindrical_distance(position);
    o.layers = layers;
    return o;
}

fn colors(i: u32) -> vec3<f32> {
    var c = array<vec3<f32>, 16>(
        vec3<f32>(0.022087, 0.098399, 0.110818),
        vec3<f32>(0.011892, 0.095924, 0.089485),
        vec3<f32>(0.027636, 0.101689, 0.100326),
        vec3<f32>(0.046564, 0.109883, 0.114838),
        vec3<f32>(0.064901, 0.117696, 0.097189),
        vec3<f32>(0.063761, 0.086895, 0.123646),
        vec3<f32>(0.084817, 0.111994, 0.166380),
        vec3<f32>(0.097489, 0.154120, 0.091064),
        vec3<f32>(0.106152, 0.131144, 0.195191),
        vec3<f32>(0.097721, 0.110188, 0.187229),
        vec3<f32>(0.133516, 0.138278, 0.148582),
        vec3<f32>(0.070006, 0.243332, 0.235792),
        vec3<f32>(0.196766, 0.142899, 0.214696),
        vec3<f32>(0.047281, 0.315338, 0.321970),
        vec3<f32>(0.204675, 0.390010, 0.302066),
        vec3<f32>(0.080955, 0.314821, 0.661491),
    );
    return c[i];
}

fn end_portal_layer(layer: f32) -> mat4x4<f32> {
    let translate = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 17.0 / layer),
        vec4<f32>(0.0, 1.0, 0.0, (2.0 + layer / 1.5) * (portal_time.x * 1.5)),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
    let a = radians((layer * layer * 4321.0 + layer * 9.0) * 2.0);
    let s = (4.5 - layer / 4.0) * 2.0;
    // mat2(cos, -sin, sin, cos) * s, расширено до mat4 (столбцы).
    let sr = mat4x4<f32>(
        vec4<f32>(s * cos(a), -s * sin(a), 0.0, 0.0),
        vec4<f32>(s * sin(a), s * cos(a), 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
    let scale_translate = mat4x4<f32>(
        vec4<f32>(0.5, 0.0, 0.0, 0.25),
        vec4<f32>(0.0, 0.5, 0.0, 0.25),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
    return sr * translate * scale_translate;
}

// textureProj(sampler2D, vec4): координаты xy / w.
fn tex_proj(t: texture_2d<f32>, p: vec4<f32>) -> vec3<f32> {
    return textureSampleLevel(t, tex_sampler, p.xy / p.w, 0.0).rgb;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    var color = tex_proj(sky_tex, i.tex_proj) * colors(0u);
    let n = u32(i.layers + 0.5);
    for (var k = 0u; k < n; k = k + 1u) {
        color = color + tex_proj(portal_tex, i.tex_proj * end_portal_layer(f32(k + 1u))) * colors(k);
    }
    return apply_fog(vec4<f32>(color, 1.0), i.spherical, i.cylindrical, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end, fog.color);
}
