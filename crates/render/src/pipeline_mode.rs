//! Выбор пути рендера по GPU (эксперимент, не заменяет `terrain`/`gpu_cull`).
//!
//! * `MeshShaders` — Task+Mesh (аналог Nvidium): culling в task-шейдере, один dispatch на все секции.
//! * `ComputeIndirect` — compute-culling + `multi_draw_indexed_indirect` (аналог Sodium; тот же принцип,
//!   что в `gpu_cull`).
use wgpu::Features;

pub const VENDOR_NVIDIA: u32 = 0x10DE;
pub const VENDOR_AMD: u32 = 0x1002;
pub const VENDOR_INTEL: u32 = 0x8086;
pub const VENDOR_APPLE: u32 = 0x106B;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Other(u32),
}

impl GpuVendor {
    pub fn from_id(id: u32) -> Self {
        match id {
            VENDOR_NVIDIA => Self::Nvidia,
            VENDOR_AMD => Self::Amd,
            VENDOR_INTEL => Self::Intel,
            VENDOR_APPLE => Self::Apple,
            o => Self::Other(o),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderPipelineMode {
    /// Task + Mesh шейдеры. Только NVIDIA и только с `EXPERIMENTAL_MESH_SHADER`.
    MeshShaders,
    /// Compute (frustum / Hi-Z) → indirect-команды → `multi_draw_indexed_indirect`.
    ComputeIndirect,
}

impl RenderPipelineMode {
    /// Чистая функция выбора (тестируется без GPU).
    pub fn select(vendor: GpuVendor, features: Features) -> Self {
        if vendor == GpuVendor::Nvidia && features.contains(Features::EXPERIMENTAL_MESH_SHADER) {
            Self::MeshShaders
        } else {
            Self::ComputeIndirect
        }
    }

    /// `EZZMC_RENDER_MODE=mesh|compute` — принудительно (mesh всё равно требует фичу).
    pub fn detect(adapter: &wgpu::Adapter) -> Self {
        let info = adapter.get_info();
        let vendor = GpuVendor::from_id(info.vendor);
        let features = adapter.features();
        let auto = Self::select(vendor, features);
        let mode = match std::env::var("EZZMC_RENDER_MODE").as_deref() {
            Ok("compute") => Self::ComputeIndirect,
            Ok("mesh") if features.contains(Features::EXPERIMENTAL_MESH_SHADER) => {
                Self::MeshShaders
            }
            _ => auto,
        };
        tracing::info!("render mode: {mode:?} ({vendor:?}, {})", info.name);
        mode
    }

    /// Фичи, которые стоит запросить у устройства для выбранного пути.
    /// NB: для mesh в `DeviceDescriptor` нужен ещё opt-in `experimental_features` (unsafe) — в `Gpu` пока нет.
    pub fn required_features(self, available: Features) -> Features {
        let mut f =
            (Features::INDIRECT_FIRST_INSTANCE | Features::MULTI_DRAW_INDIRECT_COUNT) & available;
        if self == Self::MeshShaders {
            f |= Features::EXPERIMENTAL_MESH_SHADER;
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvidia_with_mesh_picks_mesh() {
        let m = RenderPipelineMode::select(GpuVendor::Nvidia, Features::EXPERIMENTAL_MESH_SHADER);
        assert_eq!(m, RenderPipelineMode::MeshShaders);
    }

    #[test]
    fn others_and_missing_feature_pick_compute() {
        let f = Features::EXPERIMENTAL_MESH_SHADER;
        assert_eq!(
            RenderPipelineMode::select(GpuVendor::Amd, f),
            RenderPipelineMode::ComputeIndirect
        );
        assert_eq!(
            RenderPipelineMode::select(GpuVendor::Nvidia, Features::empty()),
            RenderPipelineMode::ComputeIndirect
        );
    }
}
