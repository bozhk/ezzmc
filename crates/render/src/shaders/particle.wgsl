// VANILLA: core/particle.vsh + fsh (1.21.11)
@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;
@group(1) @binding(2) var lightmap: texture_2d<f32>;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) spherical: f32,
    @location(1) cylindrical: f32,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) @interpolate(flat) hurt: f32,
    @location(5) light: vec4<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv2: vec2<i32>,
) -> VsOut {
    var o: VsOut;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(position, 1.0);
    o.spherical = fog_spherical_distance(position);
    o.cylindrical = fog_cylindrical_distance(position);
    o.uv = uv0;
    // texelFetch(Sampler2, UV2 / 16, 0)
    // Флаг красного оверлея урона (OverlayTexture.RED_OVERLAY_RGBA): uv2.x += 4096.
    o.hurt = select(0.0, 1.0, uv2.x >= 4096);
    let l2 = vec2<i32>(uv2.x & 4095, uv2.y);
    o.color = color;
    o.light = textureLoad(lightmap, l2 / 16, 0);
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    // VANILLA: entity.fsh — оверлей урона смешивается ДО умножения на lightMapColor.
    var color = textureSample(tex, tex_sampler, i.uv) * i.color * dyn_tf.color_modulator;
    if (i.hurt > 0.5) {
        // entity.fsh: mix(overlay.rgb, color.rgb, overlay.a), overlay = 0xB2FF0000, a = 0.7 — доля текстуры.
        color = vec4<f32>(mix(vec3<f32>(1.0, 0.0, 0.0), color.rgb, 0.7), color.a);
    }
    color = color * i.light;
    if (color.a < 0.1) {
        discard;
    }
    return apply_fog(color, i.spherical, i.cylindrical, fog.environmental_start, fog.environmental_end, fog.render_distance_start, fog.render_distance_end, fog.color);
}
