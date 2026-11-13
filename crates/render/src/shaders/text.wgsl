// Текст: квады в пикселях экрана, шрифтовой атлас ванильного ascii.png, альфа-смешивание.
struct Screen { size: vec2<f32>, _p: vec2<f32> }
@group(0) @binding(0) var<uniform> screen: Screen;
@group(0) @binding(1) var font: texture_2d<f32>;
@group(0) @binding(2) var font_samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) p: vec4<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>) -> VsOut {
    var o: VsOut;
    let ndc = vec3<f32>(p.x / screen.size.x * 2.0 - 1.0, 1.0 - p.y / screen.size.y * 2.0, p.z);
    o.pos = vec4<f32>(ndc * p.w, p.w);
    o.uv = uv;
    o.color = color;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let t = textureSample(font, font_samp, max(i.uv, vec2<f32>(0.0)));
    if (i.uv.x < 0.0) { return i.color; }
    if (t.a < 0.1) { discard; }
    return t * i.color;
}
