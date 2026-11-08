// Общие структуры и функции. Порт include/{fog,globals,chunksection,dynamictransforms,projection,light}.glsl (1.21.11).
// Раскладка uniform-блоков совпадает с std140 ванили (см. uniforms.rs).
// group(0): uniform-буферы; group(1): текстуры/сэмплеры/буферы данных.

struct Projection {
    proj_mat: mat4x4<f32>,
}

struct Fog {
    color: vec4<f32>,
    environmental_start: f32,
    environmental_end: f32,
    render_distance_start: f32,
    render_distance_end: f32,
    sky_end: f32,
    clouds_end: f32,
}

struct Globals {
    camera_block_pos: vec3<i32>,
    camera_offset: vec3<f32>,
    screen_size: vec2<f32>,
    glint_alpha: f32,
    game_time: f32,
    menu_blur_radius: i32,
    use_rgss: i32,
}

struct DynamicTransforms {
    model_view_mat: mat4x4<f32>,
    color_modulator: vec4<f32>,
    model_offset: vec3<f32>,
    texture_mat: mat4x4<f32>,
}

struct ChunkSection {
    model_view_mat: mat4x4<f32>,
    chunk_visibility: f32,
    texture_size: vec2<i32>,
    chunk_position: vec3<i32>,
}

struct Lighting {
    light0_direction: vec3<f32>,
    light1_direction: vec3<f32>,
}

struct CloudInfo {
    cloud_color: vec4<f32>,
    cloud_offset: vec3<f32>,
    cell_size: vec3<f32>,
}

struct LightmapInfo {
    ambient_light_factor: f32,
    sky_factor: f32,
    block_factor: f32,
    night_vision_factor: f32,
    darkness_scale: f32,
    darken_world_factor: f32,
    brightness_factor: f32,
    sky_light_color: vec3<f32>,
    ambient_color: vec3<f32>,
}

// VANILLA: fog.glsl#linear_fog_value
fn linear_fog_value(vertex_distance: f32, fog_start: f32, fog_end: f32) -> f32 {
    if (vertex_distance <= fog_start) {
        return 0.0;
    } else if (vertex_distance >= fog_end) {
        return 1.0;
    }
    return (vertex_distance - fog_start) / (fog_end - fog_start);
}

// VANILLA: fog.glsl#total_fog_value
fn total_fog_value(spherical: f32, cylindrical: f32, env_start: f32, env_end: f32, rd_start: f32, rd_end: f32) -> f32 {
    return max(linear_fog_value(spherical, env_start, env_end), linear_fog_value(cylindrical, rd_start, rd_end));
}

// VANILLA: fog.glsl#apply_fog
fn apply_fog(in_color: vec4<f32>, spherical: f32, cylindrical: f32, env_start: f32, env_end: f32, rd_start: f32, rd_end: f32, fog_color: vec4<f32>) -> vec4<f32> {
    let fog_value = total_fog_value(spherical, cylindrical, env_start, env_end, rd_start, rd_end);
    return vec4<f32>(mix(in_color.rgb, fog_color.rgb, fog_value * fog_color.a), in_color.a);
}

// VANILLA: fog.glsl#fog_spherical_distance
fn fog_spherical_distance(pos: vec3<f32>) -> f32 {
    return length(pos);
}

// VANILLA: fog.glsl#fog_cylindrical_distance
fn fog_cylindrical_distance(pos: vec3<f32>) -> f32 {
    return max(length(pos.xz), abs(pos.y));
}

// VANILLA: light.glsl#minecraft_mix_light
fn minecraft_mix_light(light_dir0: vec3<f32>, light_dir1: vec3<f32>, normal: vec3<f32>, color: vec4<f32>) -> vec4<f32> {
    let light = vec2<f32>(dot(light_dir0, normal), dot(light_dir1, normal));
    let light_value = max(vec2<f32>(0.0), light);
    let light_accum = min(1.0, (light_value.x + light_value.y) * 0.6 + 0.4);
    return vec4<f32>(color.rgb * light_accum, color.a);
}
