// VANILLA: core/position_color.vsh+fsh и gui.vsh+fsh (идентичны по поведению).
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: vec4<f32>) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.color = color;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    if (i.color.a == 0.0) {
        discard;
    }
    return i.color * dyn_tf.color_modulator;
}
