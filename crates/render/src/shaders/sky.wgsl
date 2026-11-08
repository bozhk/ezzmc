// VANILLA: core/sky.vsh + sky.fsh (1.21.11)
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) spherical: f32,
    @location(1) cylindrical: f32,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.spherical = fog_spherical_distance(position);
    o.cylindrical = fog_cylindrical_distance(position);
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    return apply_fog(dyn_tf.color_modulator, i.spherical, i.cylindrical, 0.0, fog.sky_end, fog.sky_end, fog.sky_end, fog.color);
}
