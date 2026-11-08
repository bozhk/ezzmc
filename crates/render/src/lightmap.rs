//! Текстура освещения 16×16 (TODO-140): порт `LightTexture#updateLightTexture` + `core/lightmap.fsh` (1.21.11).
//! Считается на CPU (256 текселей) — результат совпадает с шейдером `lightmap.wgsl` и не требует render-pass.
//! Раскладка: столбец x = уровень блочного света, строка y = уровень неба (как сэмплирует `terrain.wgsl`).
use crate::uniforms::LightmapInfoUniform;

/// Вход `LightTexture#updateLightTexture`.
#[derive(Clone, Copy, Debug)]
pub struct LightmapParams {
    /// `dimensionType().ambientLight()`.
    pub ambient_light: f32,
    /// `SKY_LIGHT_FACTOR` (+ вспышка Энда).
    pub sky_factor: f32,
    /// `blockLightFlicker + 1.4`.
    pub block_factor: f32,
    /// Ночное зрение / сила кондуита.
    pub night_vision: f32,
    pub darkness_scale: f32,
    pub darken_world: f32,
    /// Опция «Brightness» 0..1.
    pub gamma: f32,
    /// `getEffectBlendFactor(DARKNESS) * darknessEffectScale`.
    pub darkness_effect: f32,
    /// `SKY_LIGHT_COLOR` (0..1).
    pub sky_light_color: [f32; 3],
    /// `(1,1,1)`, в Энде с вспышкой `(0.99, 1.12, 1.0)`.
    pub ambient_color: [f32; 3],
}

impl Default for LightmapParams {
    fn default() -> Self {
        LightmapParams {
            ambient_light: 0.0,
            sky_factor: 1.0,
            block_factor: 1.4,
            night_vision: 0.0,
            darkness_scale: 0.0,
            darken_world: 0.0,
            gamma: 0.5,
            darkness_effect: 0.0,
            sky_light_color: [1.0; 3],
            ambient_color: [10.0 / 255.0; 3],
        }
    }
}

impl LightmapParams {
    pub fn to_uniform(&self) -> LightmapInfoUniform {
        LightmapInfoUniform {
            ambient_light_factor: self.ambient_light,
            sky_factor: self.sky_factor,
            block_factor: self.block_factor,
            night_vision_factor: self.night_vision,
            darkness_scale: self.darkness_scale,
            darken_world_factor: self.darken_world,
            // VANILLA: Math.max(0.0F, gamma - darknessEffect)
            brightness_factor: (self.gamma - self.darkness_effect).max(0.0),
            sky_light_color: self.sky_light_color,
            ambient_color: self.ambient_color,
        }
    }
}

/// VANILLA: LightTexture#tick — мерцание красной компоненты блочного света.
#[derive(Clone, Debug)]
pub struct Flicker {
    value: f32,
    rng: u64,
}

impl Default for Flicker {
    fn default() -> Self {
        Flicker {
            value: 0.0,
            rng: 0x2545_F491_4F6C_DD1D,
        }
    }
}

impl Flicker {
    fn next_f32(&mut self) -> f32 {
        // xorshift64*: источник не обязан совпадать с javaRandom (RandomSource.create() не сидирован)
        self.rng ^= self.rng >> 12;
        self.rng ^= self.rng << 25;
        self.rng ^= self.rng >> 27;
        ((self.rng.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 40) as f32) / (1u64 << 24) as f32
    }

    pub fn tick(&mut self) {
        let a = self.next_f32() - self.next_f32();
        self.value += a * self.next_f32() * self.next_f32() * 0.1;
        self.value *= 0.9;
    }

    /// `blockLightFlicker + 1.4`
    pub fn block_factor(&self) -> f32 {
        self.value + 1.4
    }
}

/// VANILLA: LightTexture#calculateDarknessScale
pub fn darkness_scale(tick_count: f32, partial: f32, blend: f32) -> f32 {
    let k = 0.45 * blend;
    (((tick_count - partial) * std::f32::consts::PI * 0.025).cos() * k).max(0.0)
}

fn brightness(level: f32) -> f32 {
    level / (4.0 - 3.0 * level)
}

fn not_gamma(c: [f32; 3]) -> [f32; 3] {
    let m = c[0].max(c[1]).max(c[2]);
    let inv = 1.0 - m;
    let scaled = 1.0 - inv * inv * inv * inv;
    c.map(|v| v * (scaled / m))
}

fn mix3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t)
}

/// VANILLA: core/lightmap.fsh#main (26.3) для одного текселя (`block`, `sky` = 0..15).
/// `BlockLightTint` = атрибут по умолчанию (#FFD88C), `NightVisionColor` = #999999.
pub fn texel(info: &LightmapInfoUniform, block: u32, sky: u32) -> [f32; 3] {
    const BLOCK_TINT: [f32; 3] = [1.0, 216.0 / 255.0, 140.0 / 255.0];
    const NIGHT_VISION: [f32; 3] = [0.6; 3];
    let block_level = block as f32 / 15.0;
    let bb = brightness(block_level) * info.block_factor;
    let sb = brightness(sky as f32 / 15.0) * info.sky_factor;
    let nv = NIGHT_VISION.map(|v| v * info.night_vision_factor);
    let mut c = [
        info.ambient_color[0].max(nv[0]),
        info.ambient_color[1].max(nv[1]),
        info.ambient_color[2].max(nv[2]),
    ];
    for (v, s) in c.iter_mut().zip(info.sky_light_color) {
        *v += s * sb;
    }
    let parabolic = (2.0 * block_level - 1.0) * (2.0 * block_level - 1.0);
    let tint = mix3(BLOCK_TINT, [1.0; 3], 0.9 * parabolic);
    for (v, t) in c.iter_mut().zip(tint) {
        *v += t * bb;
    }
    c = mix3(
        c,
        [c[0] * 0.7, c[1] * 0.6, c[2] * 0.6],
        info.darken_world_factor,
    );
    c = c.map(|v| (v - info.darkness_scale).clamp(0.0, 1.0));
    mix3(c, not_gamma(c), info.brightness_factor)
}

/// 16×16 RGBA8 (строка = небо, столбец = блок).
pub fn compute(info: &LightmapInfoUniform) -> Vec<u8> {
    let mut out = Vec::with_capacity(16 * 16 * 4);
    for sky in 0..16 {
        for block in 0..16 {
            let c = texel(info, block, sky);
            // RGBA8Unorm: round(x·255), как запись в 8-битную цель
            out.extend(c.map(|v| (v * 255.0).round() as u8));
            out.push(255);
        }
    }
    out
}

/// GPU-текстура освещения.
pub struct LightmapTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl LightmapTexture {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("lightmap"),
            size: wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        LightmapTexture { texture, view }
    }

    pub fn update(&self, queue: &wgpu::Queue, params: &LightmapParams) {
        let data = compute(&params.to_uniform());
        queue.write_texture(
            self.texture.as_image_copy(),
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(64),
                rows_per_image: Some(16),
            },
            wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn px(d: &[u8], block: usize, sky: usize) -> [u8; 3] {
        let i = (sky * 16 + block) * 4;
        [d[i], d[i + 1], d[i + 2]]
    }

    #[test]
    fn day_overworld_corners() {
        // день: sky_factor 1, белый свет неба, ambient 0, gamma 0.5
        let d = compute(&LightmapParams::default().to_uniform());
        // sky 15: белый (26.3: без финального mix к 0.75)
        let w = px(&d, 0, 15);
        assert!(w[0] == w[1] && w[1] == w[2] && w[0] == 255, "{w:?}");
        // пещера без света: блок 0, небо 0 → ambient #0a0a0a + not_gamma, почти чёрный
        let dark = px(&d, 0, 0);
        assert!(dark[0] > 0 && dark[0] < 40, "{dark:?}");
        // монотонность по небу
        let mut prev = 0;
        for s in 0..16 {
            let v = px(&d, 0, s)[1];
            assert!(v >= prev);
            prev = v;
        }
        // слабый блочный свет (block 8): тёплый оттенок R > G > B (при 14 всё насыщается до белого)
        let t = px(&d, 8, 0);
        assert!(t[0] > t[1] && t[1] > t[2], "{t:?}");
    }

    #[test]
    fn night_vision_brightens_and_nether_ambient() {
        let night = LightmapParams {
            sky_factor: 0.0,
            ..Default::default()
        };
        let dark = px(&compute(&night.to_uniform()), 0, 0);
        let nv = LightmapParams {
            night_vision: 1.0,
            ..night
        };
        let lit = px(&compute(&nv.to_uniform()), 0, 0);
        assert!(lit.iter().max().copied() >= Some(150));
        assert!(lit[0] > dark[0]);
        // darkness_effect уменьшает gamma, но не ниже 0
        let u = LightmapParams {
            gamma: 0.2,
            darkness_effect: 0.5,
            ..Default::default()
        }
        .to_uniform();
        assert_eq!(u.brightness_factor, 0.0);
    }

    #[test]
    fn flicker_and_darkness_scale() {
        let mut f = Flicker::default();
        for _ in 0..200 {
            f.tick();
            assert!((f.block_factor() - 1.4).abs() < 0.5);
        }
        assert_eq!(darkness_scale(0.0, 0.0, 1.0), 0.45);
        assert_eq!(darkness_scale(20.0, 0.0, 1.0), 0.0); // cos(π/2·…) < 0 → 0
    }
}
