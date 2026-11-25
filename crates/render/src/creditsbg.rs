//! Фон экрана титров/поэмы Энда.
//! VANILLA: WinScreen#extractBackground (poem) — `fill(RenderPipelines.END_PORTAL, end_sky + end_portal,
//! 0, 0, width, height)`, затем WinScreen#extractVignette — `blit(VIGNETTE, credits_vignette.png)`
//! на весь экран, цвет -1 (белый). Отдельный проход без глубины, очистка чёрным.
use crate::endportaldraw::{EndPortalDraw, PortalVertex, Rgba};
use crate::vignette::VignetteDraw;
use glam::Mat4;
use wgpu::util::DeviceExt;

/// VANILLA: PORTAL_LAYERS у END_PORTAL.
const LAYERS: f32 = 15.0;

pub struct CreditsBackground {
    portal: EndPortalDraw,
    vignette: VignetteDraw,
    // Буферы держит bind group портала; поля — чтобы владение было явным.
    _projection: wgpu::Buffer,
    _fog: wgpu::Buffer,
}

impl CreditsBackground {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: wgpu::TextureFormat,
        sky: &Rgba<'_>,
        portal: &Rgba<'_>,
        vignette: &Rgba<'_>,
    ) -> Self {
        // Вершины задаются сразу в NDC: проекция и model-view единичные. Для текстурных
        // координат шейдера (projection_from_position) это то же, что GUI-орто на весь экран.
        let projection = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("credits.projection"),
            contents: bytemuck_mat(&Mat4::IDENTITY),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        // Fog.color.a = 0 → apply_fog не меняет цвет (в GUI тумана нет).
        let fog = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("credits.fog"),
            contents: &[0u8; 48],
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let portal = EndPortalDraw::new(device, queue, color, None, &projection, &fog, sky, portal);
        let vignette = VignetteDraw::new(
            device,
            queue,
            color,
            None,
            vignette.data,
            vignette.width,
            vignette.height,
        );
        vignette.prepare(queue, 1.0, 0.0);
        CreditsBackground {
            portal,
            vignette,
            _projection: projection,
            _fog: fog,
        }
    }

    /// Очищает `target` и рисует портал + виньетку. `game_time`/`partial` — время мира
    /// (VANILLA: GlobalSettingsUniform GameTime, как у портала в мире).
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        game_time: i64,
        partial: f32,
    ) {
        let verts: [PortalVertex; 4] = [
            [-1.0, -1.0, 0.0, LAYERS],
            [1.0, -1.0, 0.0, LAYERS],
            [1.0, 1.0, 0.0, LAYERS],
            [-1.0, 1.0, 0.0, LAYERS],
        ];
        self.portal
            .prepare(device, queue, Mat4::IDENTITY, &verts, game_time, partial);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("credits.bg"),
        });
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("credits.bg"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            self.portal.draw(&mut pass);
            self.vignette.draw(&mut pass);
        }
        queue.submit([enc.finish()]);
    }
}

fn bytemuck_mat(m: &Mat4) -> &[u8] {
    let a: &[f32; 16] = m.as_ref();
    // SAFETY: [f32; 16] — POD, выравнивание u8 = 1.
    unsafe { std::slice::from_raw_parts(a.as_ptr().cast::<u8>(), 64) }
}
