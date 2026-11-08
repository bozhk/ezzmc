// VANILLA: core/stars.vsh + stars.fsh (1.21.11)
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return dyn_tf.color_modulator;
}
