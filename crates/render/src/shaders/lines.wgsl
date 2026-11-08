// Линии толщиной в пиксели: каждый отрезок — квад, растянутый в экранном пространстве.
// VANILLA: RenderPipelines.LINES (контур блока): цвет с альфой, глубина с небольшим сдвигом к камере.
struct U { view_proj: mat4x4<f32>, color: vec4<f32>, screen: vec2<f32>, width: f32, bias: f32 }
@group(0) @binding(0) var<uniform> u: U;

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, @location(0) a: vec3<f32>, @location(1) b: vec3<f32>) -> @builtin(position) vec4<f32> {
    var pa = u.view_proj * vec4<f32>(a, 1.0);
    var pb = u.view_proj * vec4<f32>(b, 1.0);
    let e = 0.001;
    if (pa.w < e && pb.w < e) { return vec4<f32>(2.0, 2.0, 2.0, 1.0); }
    if (pa.w < e) { pa = mix(pa, pb, (e - pa.w) / (pb.w - pa.w)); }
    if (pb.w < e) { pb = mix(pb, pa, (e - pb.w) / (pa.w - pb.w)); }
    let sa = pa.xy / pa.w;
    let sb = pb.xy / pb.w;
    var d = (sb - sa) * u.screen;
    let len = length(d);
    if (len > 0.0001) { d = d / len; } else { d = vec2<f32>(1.0, 0.0); }
    let n = vec2<f32>(-d.y, d.x) * u.width / u.screen;
    var corner = array<vec3<f32>, 6>(
        vec3<f32>(0.0, 0.0, -1.0), vec3<f32>(1.0, 0.0, -1.0), vec3<f32>(1.0, 0.0, 1.0),
        vec3<f32>(0.0, 0.0, -1.0), vec3<f32>(1.0, 0.0, 1.0), vec3<f32>(0.0, 0.0, 1.0));
    let c = corner[vi];
    let s = select(sa, sb, c.x > 0.5);
    let z = select(pa.z / pa.w, pb.z / pb.w, c.x > 0.5) - u.bias;
    // Продолжение на полширины вдоль линии, чтобы углы контура смыкались.
    let ext = d / u.screen * u.width * select(-1.0, 1.0, c.x > 0.5);
    return vec4<f32>(s + n * c.z + ext, z, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return u.color;
}
