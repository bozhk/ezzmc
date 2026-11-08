// VANILLA: core/position_tex.vsh + fsh (1.21.11) — солнце/луна (CELESTIAL).
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) uv0: vec2<f32>) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.uv = uv0;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(tex, tex_sampler, i.uv) * dyn_tf.color_modulator;
    if (color.a == 0.0) {
        discard;
    }
    return color;
}
