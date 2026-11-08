//! Swapchain (TODO-033): конфигурация поверхности, resize, режим present, восстановление.
//! VANILLA: options `enableVsync` (true по умолчанию) → Fifo; «макс. FPS = unlimited» без vsync → Immediate/Mailbox.
use crate::Gpu;

/// Режим презентации. `Vsync` — Fifo (есть везде); остальные — если поддерживает поверхность.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentMode {
    Vsync,
    /// Без разрыва и без блокировки, если доступен; иначе Fifo.
    Mailbox,
    /// Без vsync (возможен tearing); иначе Mailbox → Fifo.
    Immediate,
}

impl PresentMode {
    /// Выбор реального режима из списка поддерживаемых (чистая функция — тестируется).
    pub fn resolve(self, supported: &[wgpu::PresentMode]) -> wgpu::PresentMode {
        use wgpu::PresentMode as W;
        let order: &[W] = match self {
            Self::Vsync => &[W::Fifo],
            Self::Mailbox => &[W::Mailbox, W::Fifo],
            Self::Immediate => &[W::Immediate, W::Mailbox, W::Fifo],
        };
        order
            .iter()
            .copied()
            .find(|m| supported.contains(m))
            .unwrap_or(W::Fifo)
    }
}

/// Результат попытки получить кадр.
pub enum Acquire {
    Frame(wgpu::SurfaceTexture),
    /// Кадр пропущен (таймаут, окно свёрнуто, поверхность пересоздана) — повторить на следующем.
    Skip,
    /// Устройство потеряно: нужно пересоздать `Gpu` и `Swapchain`.
    DeviceLost,
}

pub struct Swapchain {
    pub config: wgpu::SurfaceConfiguration,
    pub mode: PresentMode,
}

impl Swapchain {
    /// Настраивает поверхность `gpu.surface` (sRGB-формат, если есть).
    pub fn new(gpu: &Gpu, width: u32, height: u32, mode: PresentMode) -> Option<Self> {
        let surface = gpu.surface.as_ref()?;
        let caps = surface.get_capabilities(&gpu.adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            // VANILLA: GL-кадр без sRGB-кодирования — шейдеры ванили считают в «гамма-пространстве».
            .find(|f| *f == wgpu::TextureFormat::Bgra8Unorm)
            .or_else(|| {
                caps.formats
                    .iter()
                    .copied()
                    .find(|f| *f == wgpu::TextureFormat::Rgba8Unorm)
            })
            .or_else(|| caps.formats.first().copied())?;
        let alpha_mode = caps
            .alpha_modes
            .first()
            .copied()
            .unwrap_or(wgpu::CompositeAlphaMode::Auto);
        let config = wgpu::SurfaceConfiguration {
            usage: if caps.usages.contains(wgpu::TextureUsages::COPY_SRC) {
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC
            } else {
                wgpu::TextureUsages::RENDER_ATTACHMENT
            },
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: mode.resolve(&caps.present_modes),
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&gpu.device, &config);
        tracing::info!(
            "swapchain {}x{} {:?} {:?}",
            config.width,
            config.height,
            config.format,
            config.present_mode
        );
        Some(Self { config, mode })
    }

    pub fn resize(&mut self, gpu: &Gpu, width: u32, height: u32) {
        let (w, h) = (width.max(1), height.max(1));
        if (w, h) == (self.config.width, self.config.height) {
            return;
        }
        self.config.width = w;
        self.config.height = h;
        self.reconfigure(gpu);
    }

    pub fn set_present_mode(&mut self, gpu: &Gpu, mode: PresentMode) {
        let Some(surface) = gpu.surface.as_ref() else {
            return;
        };
        let caps = surface.get_capabilities(&gpu.adapter);
        self.mode = mode;
        self.config.present_mode = mode.resolve(&caps.present_modes);
        self.reconfigure(gpu);
    }

    fn reconfigure(&self, gpu: &Gpu) {
        if let Some(s) = gpu.surface.as_ref() {
            s.configure(&gpu.device, &self.config);
        }
    }

    /// Берёт следующий кадр; Outdated/Lost/Suboptimal → переконфигурация.
    pub fn acquire(&mut self, gpu: &Gpu) -> Acquire {
        if gpu.is_device_lost() {
            return Acquire::DeviceLost;
        }
        let Some(surface) = gpu.surface.as_ref() else {
            return Acquire::Skip;
        };
        match surface.get_current_texture() {
            Ok(f) if f.suboptimal => {
                drop(f);
                self.reconfigure(gpu);
                Acquire::Skip
            }
            Ok(f) => Acquire::Frame(f),
            Err(wgpu::SurfaceError::Timeout) => Acquire::Skip,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                self.reconfigure(gpu);
                Acquire::Skip
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                tracing::error!("поверхность: нехватка памяти");
                Acquire::DeviceLost
            }
            Err(e) => {
                tracing::warn!("поверхность: {e}");
                Acquire::Skip
            }
        }
    }

    /// Заливает кадр цветом (проверка цепочки до появления настоящего рендера).
    pub fn clear_and_present(
        &self,
        gpu: &Gpu,
        frame: wgpu::SurfaceTexture,
        rgb: [f64; 3],
        capture: bool,
    ) -> Option<Screenshot> {
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut enc = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        drop(enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: rgb[0],
                        g: rgb[1],
                        b: rgb[2],
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        }));
        gpu.queue.submit([enc.finish()]);
        let shot = if capture {
            self.read_frame(gpu, &frame.texture)
        } else {
            None
        };
        frame.present();
        shot
    }

    /// Кадр рисует вызывающий: `draw(view, width, height)`; затем скриншот (если нужен) и present.
    pub fn draw_and_present(
        &self,
        gpu: &Gpu,
        frame: wgpu::SurfaceTexture,
        capture: bool,
        draw: impl FnOnce(&wgpu::TextureView, u32, u32),
    ) -> Option<Screenshot> {
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        draw(&view, self.config.width, self.config.height);
        let shot = if capture {
            self.read_frame(gpu, &frame.texture)
        } else {
            None
        };
        frame.present();
        shot
    }

    /// Копирует текстуру кадра в RGBA8 (sRGB-байты как на экране). До `present`.
    /// VANILLA: Screenshot#takeScreenshot — сохраняется содержимое основного буфера кадра.
    pub fn read_frame(&self, gpu: &Gpu, tex: &wgpu::Texture) -> Option<Screenshot> {
        if !self.config.usage.contains(wgpu::TextureUsages::COPY_SRC) {
            tracing::warn!("скриншот: поверхность не поддерживает COPY_SRC");
            return None;
        }
        let (w, h) = (self.config.width, self.config.height);
        let bgra = matches!(
            self.config.format,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
        );
        if !bgra
            && !matches!(
                self.config.format,
                wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb
            )
        {
            tracing::warn!("скриншот: формат {:?} не поддержан", self.config.format);
            return None;
        }
        let padded = (w * 4).next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("screenshot"),
            size: u64::from(padded) * u64::from(h),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut enc = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        enc.copy_texture_to_buffer(
            tex.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue.submit([enc.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        buf.slice(..).map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
        rx.recv().ok()?.ok()?;
        let data = buf.slice(..).get_mapped_range();
        let pixels = unpack_rows(&data, w, h, padded, bgra);
        drop(data);
        buf.unmap();
        Some(Screenshot {
            width: w,
            height: h,
            rgba: pixels,
        })
    }
}

/// Готовый снимок кадра: RGBA8, строки сверху вниз, альфа = 255.
pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Убирает выравнивание строк, при необходимости BGRA→RGBA, альфа=255 (ваниль пишет без альфы).
fn unpack_rows(data: &[u8], w: u32, h: u32, padded: u32, bgra: bool) -> Vec<u8> {
    let row = (w * 4) as usize;
    let mut out = Vec::with_capacity(row * h as usize);
    for y in 0..h as usize {
        let src = &data[y * padded as usize..y * padded as usize + row];
        for px in src.chunks_exact(4) {
            let (r, b) = if bgra { (px[2], px[0]) } else { (px[0], px[2]) };
            out.extend_from_slice(&[r, px[1], b, 255]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use wgpu::PresentMode as W;

    #[test]
    fn unpack_strips_padding_and_swizzles() {
        // 2x2, padded=12 (строка 8 + 4 мусора)
        let data = [
            1, 2, 3, 9, 4, 5, 6, 9, 0xEE, 0xEE, 0xEE, 0xEE, //
            7, 8, 9, 9, 10, 11, 12, 9, 0xEE, 0xEE, 0xEE, 0xEE,
        ];
        let o = unpack_rows(&data, 2, 2, 12, true);
        assert_eq!(
            o,
            [3, 2, 1, 255, 6, 5, 4, 255, 9, 8, 7, 255, 12, 11, 10, 255]
        );
        let o = unpack_rows(&data, 2, 2, 12, false);
        assert_eq!(&o[..4], &[1, 2, 3, 255]);
    }

    #[test]
    fn present_mode_resolution() {
        let all = [W::Fifo, W::Mailbox, W::Immediate];
        assert_eq!(PresentMode::Vsync.resolve(&all), W::Fifo);
        assert_eq!(PresentMode::Mailbox.resolve(&all), W::Mailbox);
        assert_eq!(PresentMode::Immediate.resolve(&all), W::Immediate);
        assert_eq!(
            PresentMode::Immediate.resolve(&[W::Fifo, W::Mailbox]),
            W::Mailbox
        );
        assert_eq!(PresentMode::Mailbox.resolve(&[W::Fifo]), W::Fifo);
        assert_eq!(PresentMode::Immediate.resolve(&[]), W::Fifo);
    }
}
