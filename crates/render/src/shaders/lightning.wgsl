// VANILLA: core/rendertype_lightning.vsh + fsh (26.3), pipeline LIGHTNING.
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) spherical: f32,
    @location(1) cylindrical: f32,
    @location(2) color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: vec4<f32>) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.spherical = fog_spherical_distance(position);
    o.cylindrical = fog_cylindrical_distance(position);
    o.color = color;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    // Аддитивное смешение: туман гасит весь вклад, а не подмешивает цвет тумана.
    let f = 1.0 - total_fog_value(i.spherical, i.cylindrical, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end);
    return i.color * dyn_tf.color_modulator * f;
}
