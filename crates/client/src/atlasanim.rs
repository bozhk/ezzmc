//! Анимация спрайтов атласа блоков (вода, лава, огонь, портал, лесные листья и т. д.).
//! VANILLA: SpriteContents.Ticker#tickAndUpload / AnimationState (1.21.11, 20 тиков/с);
//! интерполяция кадров — `animate_sprite_interpolate` (здесь на CPU по каждому mip-уровню).
use ezzmc_assets::animation::{interpolate_pixel, AnimationState};
use ezzmc_assets::block_atlas::BlockAtlas;
use ezzmc_assets::texture::{generate_mip_levels, Image, MipmapStrategy};
use std::collections::HashMap;
use std::time::Instant;

struct Sprite {
    x: u32,
    y: u32,
    anim: ezzmc_assets::animation::AnimatedTexture,
    state: AnimationState,
    /// Мип-цепочки уникальных кадров: индекс кадра → уровни (0 — полный размер).
    frames: HashMap<u32, Vec<Image>>,
}

pub struct AtlasAnim {
    sprites: Vec<Sprite>,
    last: Instant,
    /// Остаток времени (в тиках), не дошедший до целого тика.
    acc: f64,
}

impl AtlasAnim {
    pub fn new(atlas: &BlockAtlas) -> AtlasAnim {
        let mip = atlas.mip_levels;
        let sprites = atlas
            .animated
            .iter()
            .map(|s| {
                let mut frames = HashMap::new();
                for idx in s.anim.unique_frames() {
                    let f = s.strip.crop(
                        s.anim.frame_x(idx) * s.frame_w,
                        s.anim.frame_y(idx) * s.frame_h,
                        s.frame_w,
                        s.frame_h,
                    );
                    frames.insert(
                        idx,
                        generate_mip_levels(&s.id.path, f, mip, MipmapStrategy::Auto, 0.0),
                    );
                }
                Sprite {
                    x: s.x,
                    y: s.y,
                    state: s.anim.create_state(),
                    anim: s.anim.clone(),
                    frames,
                }
            })
            .collect();
        AtlasAnim {
            sprites,
            last: Instant::now(),
            acc: 0.0,
        }
    }

    /// Прокрутить анимации по реальному времени и залить изменившиеся кадры в `tex`.
    pub fn update(&mut self, queue: &wgpu::Queue, tex: &wgpu::Texture) {
        let now = Instant::now();
        self.acc += now.duration_since(self.last).as_secs_f64() * 20.0;
        self.last = now;
        let ticks = (self.acc as u32).min(10);
        self.acc -= f64::from(self.acc as u32);
        for _ in 0..ticks {
            for s in &mut self.sprites {
                s.state.tick(&s.anim);
            }
            for s in &self.sprites {
                if s.state.needs_to_draw(&s.anim) {
                    upload(queue, tex, s);
                }
            }
        }
    }
}

fn upload(queue: &wgpu::Queue, tex: &wgpu::Texture, s: &Sprite) {
    let cur = s.state.current_index(&s.anim);
    let Some(a) = s.frames.get(&cur) else { return };
    let interp = s.anim.interpolate.then(|| {
        let next = s.state.next_index(&s.anim);
        (s.frames.get(&next), s.state.progress_milli(&s.anim))
    });
    for (l, img) in a.iter().enumerate() {
        let data = match &interp {
            Some((Some(b), milli)) if cur != s.state.next_index(&s.anim) && b.len() > l => {
                let n = &b[l];
                let mut out = Vec::with_capacity(img.px.len() * 4);
                for (&p, &q) in img.px.iter().zip(&n.px) {
                    let v = interpolate_pixel(p, q, *milli);
                    out.extend_from_slice(&[
                        (v >> 16) as u8,
                        (v >> 8) as u8,
                        v as u8,
                        (v >> 24) as u8,
                    ]);
                }
                out
            }
            _ => img.to_rgba8(),
        };
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: tex,
                mip_level: l as u32,
                origin: wgpu::Origin3d {
                    x: s.x >> l,
                    y: s.y >> l,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(img.width * 4),
                rows_per_image: Some(img.height),
            },
            wgpu::Extent3d {
                width: img.width,
                height: img.height,
                depth_or_array_layers: 1,
            },
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn animated_sprites_present_if_game_present() {
        use ezzmc_assets::install::GameInstall;
        let Ok(inst) = GameInstall::locate("26.3") else {
            return;
        };
        let rm = inst.resource_manager().unwrap();
        let atlas = BlockAtlas::build(&rm, 4).unwrap();
        let names: Vec<_> = atlas.animated.iter().map(|s| s.id.path.as_str()).collect();
        eprintln!(
            "animated={} lava={}",
            names.len(),
            names.contains(&"block/lava_still")
        );
        assert!(names.contains(&"block/lava_still"));
        assert!(names.contains(&"block/water_still"));
        assert!(names.contains(&"block/fire_0"));
    }
}
