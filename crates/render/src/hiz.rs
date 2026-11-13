//! Hi-Z: мип-цепочка глубины прошлого кадра для occlusion-culling (`voxel_cull.wgsl`).
//!
//! Формат `R32Float`, уровень N хранит максимум (самую дальнюю глубину) по блоку 2×2 уровня N−1.
//! Исходная глубина — `Depth32Float`-текстура с `TEXTURE_BINDING`.
pub const HIZ_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R32Float;
const SHADER: &str = include_str!("shaders/hiz_build.wgsl");

pub fn mip_count(w: u32, h: u32) -> u32 {
    32 - w.max(h).max(1).leading_zeros()
}

pub struct HiZ {
    width: u32,
    height: u32,
    mips: u32,
    view: wgpu::TextureView,
    mip_views: Vec<wgpu::TextureView>,
    copy: wgpu::ComputePipeline,
    down: wgpu::ComputePipeline,
    /// bind group'ы downsample (mip m-1 → m) — mip-views неизменны, создаём один раз.
    down_bgs: Vec<wgpu::BindGroup>,
    copy_layout: wgpu::BindGroupLayout,
}

impl HiZ {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let (width, height) = (width.max(1), height.max(1));
        let mips = mip_count(width, height);
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hiz"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: mips,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HIZ_FORMAT,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let mip_views: Vec<wgpu::TextureView> = (0..mips)
            .map(|m| {
                tex.create_view(&wgpu::TextureViewDescriptor {
                    base_mip_level: m,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hiz_build.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let pipe = |entry| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: None,
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let copy = pipe("copy_depth");
        let down = pipe("downsample");
        let down_layout = down.get_bind_group_layout(0);
        let down_bgs = (1..mips as usize)
            .map(|m| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("hiz down"),
                    layout: &down_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&mip_views[m - 1]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(&mip_views[m]),
                        },
                    ],
                })
            })
            .collect();
        Self {
            width,
            height,
            mips,
            view: tex.create_view(&Default::default()),
            mip_views,
            copy_layout: copy.get_bind_group_layout(0),
            copy,
            down,
            down_bgs,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn mips(&self) -> u32 {
        self.mips
    }
    /// Вид на все уровни — для `texture_2d<f32>` в cull-шейдере.
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    /// Строит цепочку из глубины кадра (размер должен совпадать с `size()`).
    pub fn build(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        depth: &wgpu::TextureView,
    ) {
        let groups = |w: u32, h: u32| (w.div_ceil(8), h.div_ceil(8));
        // Один compute-проход: wgpu сам ставит барьеры между dispatch'ами (mip m читает m-1).
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hiz copy"),
            layout: &self.copy_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.mip_views[0]),
                },
            ],
        });
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.copy);
        pass.set_bind_group(0, &bg, &[]);
        let (gx, gy) = groups(self.width, self.height);
        pass.dispatch_workgroups(gx, gy, 1);
        pass.set_pipeline(&self.down);
        for (i, bg) in self.down_bgs.iter().enumerate() {
            let m = i + 1;
            pass.set_bind_group(0, bg, &[]);
            let (gx, gy) = groups((self.width >> m).max(1), (self.height >> m).max(1));
            pass.dispatch_workgroups(gx, gy, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mips() {
        assert_eq!(mip_count(1, 1), 1);
        assert_eq!(mip_count(854, 480), 10);
        assert_eq!(mip_count(1024, 8), 11);
    }

    #[test]
    fn shader_valid() {
        let m = naga::front::wgsl::parse_str(SHADER).expect("wgsl");
        naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::all())
            .validate(&m)
            .expect("valid");
    }
}
