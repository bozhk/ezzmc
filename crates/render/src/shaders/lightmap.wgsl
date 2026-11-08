// VANILLA: core/lightmap.fsh + screenquad.vsh (1.21.11). Рисуется в 16×16 текстуру.
@group(0) @binding(7) var<uniform> info: LightmapInfo;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

// VANILLA: screenquad.vsh
@vertex
fn vs_main(@builtin(vertex_index) id: u32) -> VsOut {
    var o: VsOut;
    let uv = vec2<f32>(f32((id << 1u) & 2u), f32(id & 2u));
    o.clip = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    o.tex_coord = uv;
    return o;
}

fn get_brightness(level: f32) -> f32 {
    return level / (4.0 - 3.0 * level);
}

fn not_gamma(color: vec3<f32>) -> vec3<f32> {
    let max_component = max(max(color.x, color.y), color.z);
    let max_inverted = 1.0 - max_component;
    let max_scaled = 1.0 - max_inverted * max_inverted * max_inverted * max_inverted;
    return color * (max_scaled / max_component);
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let block_brightness = get_brightness(floor(i.tex_coord.x * 16.0) / 15.0) * info.block_factor;
    let sky_brightness = get_brightness(floor(i.tex_coord.y * 16.0) / 15.0) * info.sky_factor;

    var color = vec3<f32>(
        block_brightness,
        block_brightness * ((block_brightness * 0.6 + 0.4) * 0.6 + 0.4),
        block_brightness * (block_brightness * block_brightness * 0.6 + 0.4),
    );
    color = mix(color, info.ambient_color, info.ambient_light_factor);
    color += info.sky_light_color * sky_brightness;
    color = mix(color, vec3<f32>(0.75), 0.04);

    if (info.ambient_light_factor == 0.0) {
        let darkened = color * vec3<f32>(0.7, 0.6, 0.6);
        color = mix(color, darkened, info.darken_world_factor);
    }
    if (info.night_vision_factor > 0.0) {
        let max_component = max(color.r, max(color.g, color.b));
        if (max_component < 1.0) {
            let bright = color / max_component;
            color = mix(color, bright, info.night_vision_factor);
        }
    }
    if (info.ambient_light_factor == 0.0) {
        color = color - vec3<f32>(info.darkness_scale);
    }
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    let ng = not_gamma(color);
    color = mix(color, ng, info.brightness_factor);
    color = mix(color, vec3<f32>(0.75), 0.04);
    return vec4<f32>(color, 1.0);
}
