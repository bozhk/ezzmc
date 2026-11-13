//! Тесты ComputeIndirect на живом адаптере (без адаптера — пропуск): отрисовка и Hi-Z-culling.
use ezzmc_render::gpu::{BackendChoice, Gpu};
use ezzmc_render::hiz::HiZ;
use ezzmc_render::pipeline_mode::RenderPipelineMode;
use ezzmc_render::renderer::{supported, CullCamera, Renderer};
use ezzmc_render::voxel_vertex::{QuadMeta, VoxelVertex};
use glam::{Mat4, Vec3};

const W: u32 = 64;
const COLOR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

fn quad(x0: u8, x1: u8, y0: u8, y1: u8, z: u8) -> ([VoxelVertex; 4], QuadMeta) {
    // CCW для камеры на -Z, смотрящей в +Z (рендер-пайплайн режет задние грани, cull Back)
    let c = [(x0, y0), (x0, y1), (x1, y1), (x1, y0)];
    (
        std::array::from_fn(|i| VoxelVertex::pack([c[i].0, c[i].1, z], 1, 3, 15, 15, i as u8)),
        QuadMeta {
            texture: 0,
            tint: u32::MAX,
        },
    )
}

fn camera(dir: Vec3) -> Mat4 {
    let proj = Mat4::perspective_rh(90f32.to_radians(), 1.0, 0.1, 1000.0);
    proj * Mat4::look_to_rh(Vec3::ZERO, dir, Vec3::Y)
}

struct Frame {
    color: wgpu::Texture,
    depth: wgpu::Texture,
}

fn frame_targets(gpu: &Gpu) -> Frame {
    let mk = |format, usage| {
        gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: W,
                height: W,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        })
    };
    Frame {
        color: mk(
            COLOR,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        ),
        depth: mk(
            wgpu::TextureFormat::Depth32Float,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        ),
    }
}

/// Один кадр: cull → draw → (опц.) Hi-Z из глубины → чтение пикселей.
fn run(
    gpu: &Gpu,
    r: &Renderer,
    cam: &CullCamera,
    use_hiz: Option<&HiZ>,
    build_hiz: Option<&HiZ>,
) -> (Vec<u8>, u32) {
    let f = frame_targets(gpu);
    let mut enc = gpu.device.create_command_encoder(&Default::default());
    r.encode_cull(gpu, &mut enc, cam, use_hiz);
    let cv = f.color.create_view(&Default::default());
    let dv = f.depth.create_view(&Default::default());
    {
        let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &cv,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &dv,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        r.draw(&mut pass);
    }
    if let Some(h) = build_hiz {
        h.build(&gpu.device, &mut enc, &dv);
    }
    let buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u64::from(W * W * 4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    enc.copy_texture_to_buffer(
        f.color.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(W * 4),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width: W,
            height: W,
            depth_or_array_layers: 1,
        },
    );
    gpu.queue.submit([enc.finish()]);
    let visible = r.read_visible_count(gpu);
    buf.slice(..).map_async(wgpu::MapMode::Read, |_| ());
    let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
    let px = buf.slice(..).get_mapped_range().to_vec();
    (px, visible)
}

/// `compact`: устройство с `MULTI_DRAW_INDIRECT_COUNT` (compact-вывод), иначе — путь «слот→слот».
fn setup(compact: bool) -> Option<(Gpu, Renderer)> {
    let mut gpu = Gpu::headless(BackendChoice::Auto).ok()?;
    {
        let want = if compact {
            wgpu::Features::INDIRECT_FIRST_INSTANCE | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT
        } else {
            wgpu::Features::INDIRECT_FIRST_INSTANCE
        };
        if !gpu.adapter.features().contains(want) {
            eprintln!("нет MULTI_DRAW_INDIRECT_COUNT — пропуск compact");
            return None;
        }
        let (d, q) = pollster::block_on(gpu.adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: want,
            required_limits: gpu.adapter.limits(),
            ..Default::default()
        }))
        .ok()?;
        gpu.device = d;
        gpu.queue = q;
    }
    if !supported(&gpu) {
        eprintln!("GPU не поддерживает ComputeIndirect — пропуск");
        return None;
    }
    let mut r = Renderer::new(&gpu, RenderPipelineMode::ComputeIndirect, COLOR, 4096, 16)?;
    // A: стена z=0 на всю секцию; B: маленький квад за ней (z=16)
    let (v, q) = quad(0, 16, 0, 16, 0);
    assert!(r.upload_section(&gpu.queue, [0, 0, 0], &v, &[q]));
    let (v, q) = quad(4, 12, 4, 12, 0);
    assert!(r.upload_section(&gpu.queue, [0, 0, 1], &v, &[q]));
    assert_eq!(r.is_compact(), compact);
    Some((gpu, r))
}

fn cam(pos: [f64; 3], dir: Vec3, prev_pos: [f64; 3], prev_dir: Vec3) -> CullCamera {
    CullCamera {
        view_proj: camera(dir),
        pos,
        prev_view_proj: camera(prev_dir),
        prev_pos,
    }
}

fn draws_wall_via_indirect(compact: bool) {
    let Some((gpu, r)) = setup(compact) else {
        return;
    };
    let p = [8.0, 8.0, -10.0];
    let (px, visible) = run(&gpu, &r, &cam(p, Vec3::Z, p, Vec3::Z), None, None);
    assert_eq!(visible, 2, "без Hi-Z обе секции в кадре");
    let at = |x: u32, y: u32| &px[((y * W + x) * 4) as usize..][..4];
    let c = at(W / 2, W / 2);
    assert!(
        c[0] > 200 && c[1] > 200 && c[2] > 200,
        "центр: стена белая, {c:?}"
    );
    assert_eq!(&at(1, 1)[..3], &[0, 0, 0], "угол: фон");
}

fn hiz_culls_hidden_section_only(compact: bool) {
    let Some((gpu, r)) = setup(compact) else {
        return;
    };
    let p = [8.0, 8.0, -10.0];
    let hiz = HiZ::new(&gpu.device, W, W);
    let same = cam(p, Vec3::Z, p, Vec3::Z);
    let (_, v0) = run(&gpu, &r, &same, None, Some(&hiz));
    assert_eq!(v0, 2);
    let (px, v1) = run(&gpu, &r, &same, Some(&hiz), None);
    assert_eq!(v1, 1, "B закрыта стеной A, A видна");
    let c = &px[((W / 2 * W + W / 2) * 4) as usize..][..3];
    assert!(c[0] > 200, "картинка не изменилась");
    // прошлый кадр смотрел в другую сторону: по Hi-Z судить нельзя → видимы обе
    let turned = cam(p, Vec3::Z, p, Vec3::X);
    let (_, v2) = run(&gpu, &r, &turned, Some(&hiz), None);
    assert_eq!(v2, 2, "консервативно при несовпадении прошлого вида");
}

fn frustum_culls_behind_camera(compact: bool) {
    let Some((gpu, r)) = setup(compact) else {
        return;
    };
    let p = [8.0, 8.0, -10.0];
    let (_, v) = run(&gpu, &r, &cam(p, -Vec3::Z, p, -Vec3::Z), None, None);
    assert_eq!(v, 0);
}

fn section_slots_are_reused(compact: bool) {
    let Some((gpu, mut r)) = setup(compact) else {
        return;
    };
    let used = r.used_vertices();
    r.remove_section(&gpu.queue, [0, 0, 1]);
    assert_eq!(r.used_vertices(), used - 4);
    let (v, q) = quad(1, 2, 1, 2, 3);
    assert!(r.upload_section(&gpu.queue, [5, 0, 5], &v, &[q]));
    assert_eq!(r.slot_count(), 2, "слот переиспользован");
    assert_eq!(r.used_vertices(), used);
}

fn assert_send_sync<T: Send + Sync>() {}
#[test]
fn renderer_is_send_sync() {
    assert_send_sync::<Renderer>();
    assert_send_sync::<HiZ>();
}

#[test]
fn draws_wall_via_indirect_plain() {
    draws_wall_via_indirect(false);
}
#[test]
fn draws_wall_via_indirect_compact() {
    draws_wall_via_indirect(true);
}

#[test]
fn hiz_culls_hidden_section_only_plain() {
    hiz_culls_hidden_section_only(false);
}
#[test]
fn hiz_culls_hidden_section_only_compact() {
    hiz_culls_hidden_section_only(true);
}

#[test]
fn frustum_culls_behind_camera_plain() {
    frustum_culls_behind_camera(false);
}
#[test]
fn frustum_culls_behind_camera_compact() {
    frustum_culls_behind_camera(true);
}

#[test]
fn section_slots_are_reused_plain() {
    section_slots_are_reused(false);
}
#[test]
fn section_slots_are_reused_compact() {
    section_slots_are_reused(true);
}
