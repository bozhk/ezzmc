//! Инициализация wgpu (TODO-032): instance → adapter → device/queue, выбор бэкенда, fallback.
//! Бэкенд: env `EZZMC_BACKEND` = `vulkan|dx12|metal|gl|auto`; `auto` — Vulkan/DX12/Metal,
//! затем GL, затем программный (fallback) адаптер.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GpuError {
    #[error("не удалось создать поверхность: {0}")]
    Surface(String),
    #[error("не найден подходящий GPU-адаптер (пробовали: {tried})")]
    NoAdapter { tried: String },
    #[error("не удалось создать устройство: {0}")]
    Device(String),
}

/// Выбор бэкенда.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendChoice {
    Auto,
    Vulkan,
    Dx12,
    Metal,
    Gl,
}

impl BackendChoice {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "auto" | "" => Self::Auto,
            "vulkan" | "vk" => Self::Vulkan,
            "dx12" | "d3d12" => Self::Dx12,
            "metal" => Self::Metal,
            "gl" | "gles" | "opengl" => Self::Gl,
            _ => return None,
        })
    }

    pub fn from_env() -> Self {
        std::env::var("EZZMC_BACKEND")
            .ok()
            .and_then(|v| Self::parse(&v))
            .unwrap_or(Self::Auto)
    }

    /// Последовательность попыток (набор бэкендов, программный адаптер?).
    fn attempts(self) -> Vec<(wgpu::Backends, bool)> {
        match self {
            Self::Auto => vec![
                (wgpu::Backends::PRIMARY, false),
                (wgpu::Backends::GL, false),
                (wgpu::Backends::all(), true),
            ],
            Self::Vulkan => vec![(wgpu::Backends::VULKAN, false)],
            Self::Dx12 => vec![(wgpu::Backends::DX12, false)],
            Self::Metal => vec![(wgpu::Backends::METAL, false)],
            Self::Gl => vec![(wgpu::Backends::GL, false)],
        }
    }
}

pub struct Gpu {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// `None` в headless-режиме (тесты).
    pub surface: Option<wgpu::Surface<'static>>,
    /// Выставляется колбэком wgpu при потере устройства (TODO-033).
    pub device_lost: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Gpu {
    /// Создаёт GPU для окна (любой `Into<SurfaceTarget>`, напр. `Arc<winit::window::Window>`).
    pub fn new(
        target: impl Into<wgpu::SurfaceTarget<'static>> + Clone,
        choice: BackendChoice,
    ) -> Result<Self, GpuError> {
        Self::create(Some(&|| target.clone().into()), choice)
    }

    /// Без окна: для тестов и вычислений.
    pub fn headless(choice: BackendChoice) -> Result<Self, GpuError> {
        Self::create(None, choice)
    }

    fn create(
        target: Option<&dyn Fn() -> wgpu::SurfaceTarget<'static>>,
        choice: BackendChoice,
    ) -> Result<Self, GpuError> {
        let mut tried = Vec::new();
        for (backends, force_fallback) in choice.attempts() {
            let label = format!(
                "{backends:?}{}",
                if force_fallback { "+fallback" } else { "" }
            );
            // PERF: валидация indirect-вызовов в wgpu = CPU-проход по каждой команде при `finish` (~17% главного
            // потока при rd=32). Команды террейна формирует сам клиент (в границах арены), ванильный GL их не проверяет.
            // Отладка: WGPU_VALIDATION_INDIRECT_CALL=1.
            let mut flags = wgpu::InstanceFlags::default();
            flags.remove(wgpu::InstanceFlags::VALIDATION_INDIRECT_CALL);
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends,
                flags: flags.with_env(),
                ..Default::default()
            });
            let surface = match target {
                Some(t) => match instance.create_surface(t()) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        tracing::warn!("{label}: поверхность: {e}");
                        tried.push(format!("{label} (surface: {e})"));
                        continue;
                    }
                },
                None => None,
            };
            let adapter =
                pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: force_fallback,
                    compatible_surface: surface.as_ref(),
                }));
            let adapter = match adapter {
                Ok(a) => a,
                Err(e) => {
                    tracing::warn!("{label}: нет адаптера: {e}");
                    tried.push(label);
                    continue;
                }
            };
            let info = adapter.get_info();
            tracing::info!(
                "GPU: {} ({:?}, {:?}), драйвер: {} {}",
                info.name,
                info.backend,
                info.device_type,
                info.driver,
                info.driver_info
            );
            let (device, queue) =
                pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                    label: Some("ezzmc"),
                    // MULTI_DRAW_INDIRECT_COUNT — опционально: compact-вывод culling (gpu_cull, voxel Renderer)
                    required_features: adapter.features()
                        & (crate::terrain::TerrainArena::WANTED_FEATURES
                            | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT),
                    required_limits: adapter.limits(),
                    ..Default::default()
                }))
                .map_err(|e| GpuError::Device(e.to_string()))?;
            device.on_uncaptured_error(std::sync::Arc::new(|e| {
                tracing::error!("wgpu: {e}");
            }));
            let device_lost = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let flag = device_lost.clone();
            device.set_device_lost_callback(move |reason, msg| {
                tracing::error!("устройство потеряно ({reason:?}): {msg}");
                flag.store(true, std::sync::atomic::Ordering::SeqCst);
            });
            return Ok(Self {
                device_lost,
                instance,
                adapter,
                device,
                queue,
                surface,
            });
        }
        Err(GpuError::NoAdapter {
            tried: tried.join(", "),
        })
    }
}

impl Gpu {
    pub fn is_device_lost(&self) -> bool {
        self.device_lost.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backend() {
        assert_eq!(BackendChoice::parse("Vulkan"), Some(BackendChoice::Vulkan));
        assert_eq!(BackendChoice::parse("d3d12"), Some(BackendChoice::Dx12));
        assert_eq!(BackendChoice::parse(""), Some(BackendChoice::Auto));
        assert_eq!(BackendChoice::parse("nope"), None);
    }

    #[test]
    fn auto_has_fallback_chain() {
        let a = BackendChoice::Auto.attempts();
        assert_eq!(a.len(), 3);
        assert!(a.last().is_some_and(|x| x.1));
    }

    /// Нужен GPU/драйвер; без него (CI без адаптера) тест пропускается.
    #[test]
    fn headless_init_if_adapter_present() {
        match Gpu::headless(BackendChoice::Auto) {
            Ok(g) => assert!(g.device.limits().max_texture_dimension_2d >= 2048),
            Err(GpuError::NoAdapter { .. }) => eprintln!("нет адаптера — пропуск"),
            Err(e) => panic!("{e}"),
        }
    }
}
