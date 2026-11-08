// VANILLA: core/rendertype_clouds.vsh + .fsh (1.21.11). `CloudFaces` (isamplerBuffer) → storage-буфер i32.
const FLAG_MASK_DIR: i32 = 7;
const FLAG_INSIDE_FACE: i32 = 16;
const FLAG_USE_TOP_COLOR: i32 = 32;
const FLAG_EXTRA_Z: i32 = 64;
const FLAG_EXTRA_X: i32 = 128;

@group(0) @binding(0) var<uniform> projection: Projection;
@group(0) @binding(1) var<uniform> fog: Fog;
@group(0) @binding(3) var<uniform> dyn_tf: DynamicTransforms;
@group(0) @binding(6) var<uniform> cloud: CloudInfo;
@group(1) @binding(0) var<storage, read> cloud_faces: array<i32>;

var<private> VERTICES: array<vec3<f32>, 24> = array<vec3<f32>, 24>(
    // bottom
    vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(1.0, 0.0, 1.0), vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 0.0, 0.0),
    // top
    vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 1.0, 1.0), vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(1.0, 1.0, 0.0),
    // north
    vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0),
    // south
    vec3<f32>(1.0, 0.0, 1.0), vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(0.0, 1.0, 1.0), vec3<f32>(0.0, 0.0, 1.0),
    // west
    vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 1.0, 1.0), vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, 0.0),
    // east
    vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(1.0, 0.0, 1.0),
);

var<private> FACE_COLORS: array<f32, 6> = array<f32, 6>(0.7, 1.0, 0.8, 0.8, 0.9, 0.9);

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) distance: f32,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_id: u32) -> VsOut {
    var o: VsOut;
    let quad_vertex = i32(vertex_id % 4u);
    let index = i32(vertex_id / 4u) * 3;
    var cell_x = cloud_faces[index];
    var cell_z = cloud_faces[index + 1];
    let dir_and_flags = cloud_faces[index + 2];
    let direction = dir_and_flags & FLAG_MASK_DIR;
    let is_inside = (dir_and_flags & FLAG_INSIDE_FACE) == FLAG_INSIDE_FACE;
    let use_top = (dir_and_flags & FLAG_USE_TOP_COLOR) == FLAG_USE_TOP_COLOR;
    cell_x = (cell_x << 1u) | ((dir_and_flags & FLAG_EXTRA_X) >> 7u);
    cell_z = (cell_z << 1u) | ((dir_and_flags & FLAG_EXTRA_Z) >> 6u);
    var qv = quad_vertex;
    if (is_inside) {
        qv = 3 - quad_vertex;
    }
    let face_vertex = VERTICES[direction * 4 + qv];
    let pos = face_vertex * cloud.cell_size + vec3<f32>(f32(cell_x), 0.0, f32(cell_z)) * cloud.cell_size + cloud.cloud_offset;
    o.clip = projection.proj_mat * dyn_tf.model_view_mat * vec4<f32>(pos, 1.0);
    o.distance = fog_spherical_distance(pos);
    var shade = FACE_COLORS[direction];
    if (use_top) {
        shade = 1.0;
    }
    o.color = vec4<f32>(shade, shade, shade, 1.0) * cloud.cloud_color;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    var color = i.color;
    color.a *= 1.0 - linear_fog_value(i.distance, 0.0, fog.clouds_end);
    return color;
}
