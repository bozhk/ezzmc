// VANILLA: core/rendertype_beacon_beam.vsh + fsh (26.3): tex * vertexColor * ColorModulator, туман по 1/gl_FragCoord.w.
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
@group(1) @binding(0) var beam_tex: texture_2d<f32>;
@group(1) @binding(1) var beam_sampler: sampler;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.color = color;
    o.uv = uv;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let c = textureSample(beam_tex, beam_sampler, i.uv) * i.color * dyn_tf.color_modulator;
    let d = 1.0 / i.clip.w;
    return apply_fog(c, d, d, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end, fog.color);
}
