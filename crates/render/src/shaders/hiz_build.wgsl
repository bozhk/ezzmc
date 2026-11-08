// Hi-Z: мип-цепочка R32Float «самая дальняя глубина» (стандартная глубина wgpu: 0 близко, 1 далеко).
@group(0) @binding(0) var src_depth: texture_depth_2d;
@group(0) @binding(1) var dst0: texture_storage_2d<r32float, write>;

@compute @workgroup_size(8, 8)
fn copy_depth(@builtin(global_invocation_id) id: vec3<u32>) {
    let d = textureDimensions(dst0);
    if id.x >= d.x || id.y >= d.y { return; }
    let z = textureLoad(src_depth, vec2<i32>(id.xy), 0);
    textureStore(dst0, vec2<i32>(id.xy), vec4<f32>(z, 0.0, 0.0, 0.0));
}

@group(0) @binding(2) var src_mip: texture_2d<f32>;
@group(0) @binding(3) var dst_mip: texture_storage_2d<r32float, write>;

// Каждый тексель уровня N = max по блоку 2×2 уровня N−1; при нечётном размере крайний тексель
// захватывает ещё одну строку/столбец — иначе в Hi-Z потерялась бы дальняя глубина (отсечение стало бы небезопасным).
@compute @workgroup_size(8, 8)
fn downsample(@builtin(global_invocation_id) id: vec3<u32>) {
    let d = textureDimensions(dst_mip);
    if id.x >= d.x || id.y >= d.y { return; }
    let s = vec2<i32>(textureDimensions(src_mip));
    let b = vec2<i32>(id.xy) * 2;
    var hi = b + vec2<i32>(1);
    if id.x + 1u == d.x { hi.x = s.x - 1; }
    if id.y + 1u == d.y { hi.y = s.y - 1; }
    var z = 0.0;
    for (var y = b.y; y <= hi.y; y++) {
        for (var x = b.x; x <= hi.x; x++) {
            z = max(z, textureLoad(src_mip, vec2<i32>(x, y), 0).r);
        }
    }
    textureStore(dst_mip, vec2<i32>(id.xy), vec4<f32>(z, 0.0, 0.0, 0.0));
}
