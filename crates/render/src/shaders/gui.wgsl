// GUI-спрайты (пиксели экрана) и куб панорамы (mvp). Одна раскладка группы: uniform, текстура, сэмплер.
struct U { m: mat4x4<f32> }
@group(0) @binding(0) var<uniform> u: U;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_sprite(@location(0) p: vec2<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>) -> VsOut {
    var o: VsOut;
    let size = vec2<f32>(u.m[0][0], u.m[0][1]);
    o.pos = vec4<f32>(p.x / size.x * 2.0 - 1.0, 1.0 - p.y / size.y * 2.0, 0.0, 1.0);
    o.uv = uv;
    o.color = color;
    return o;
}

@vertex
fn vs_pano(@location(0) p: vec2<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>) -> VsOut {
    // p.xy и p.z хранятся как (x, y) + z в color.x — см. gui.rs (куб: pos3 в uv/color).
    var o: VsOut;
    o.pos = u.m * vec4<f32>(p.x, p.y, color.x, 1.0);
    o.uv = uv;
    o.color = vec4<f32>(1.0);
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    return textureSample(tex, samp, i.uv) * i.color;
}
