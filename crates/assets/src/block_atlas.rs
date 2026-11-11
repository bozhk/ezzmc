//! Атлас блоков: загрузка спрайтов, укладка, мип-цепочка (TODO-137).
//! VANILLA: SpriteLoader#stitch + TextureAtlas (1.21.11), padding = `1 << mip` (анизотропия 1).
use crate::animation::{AnimatedTexture, AnimationMeta};
use crate::atlas::{self, Identifier, SpriteLoad};
use crate::bake::{SpriteGetter, SpriteUv};
use crate::pack::ResourceManager;
use crate::stitcher::{self, Entry};
use crate::texture::{generate_mip_levels, Image, MipmapStrategy};
use std::collections::HashMap;

/// Спрайт-анимация: полоса кадров и положение в атласе (покадровое обновление — на стороне рендера).
#[derive(Debug, Clone)]
pub struct AnimatedSprite {
    pub id: Identifier,
    pub x: u32,
    pub y: u32,
    pub frame_w: u32,
    pub frame_h: u32,
    pub anim: AnimatedTexture,
    pub strip: Image,
}

pub struct BlockAtlas {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    /// `mips[0]` — полный размер; всего `mip_levels + 1` уровней.
    pub mips: Vec<Image>,
    pub sprites: HashMap<Identifier, SpriteUv>,
    pub animated: Vec<AnimatedSprite>,
    missing: SpriteUv,
}

struct Loaded {
    id: Identifier,
    frame: Image,
    strip: Option<(Image, AnimatedTexture)>,
    strategy: MipmapStrategy,
    cutoff_bias: f32,
}

/// VANILLA: TextureMetadataSection (`texture.mipmap_strategy`, `texture.alpha_cutoff_bias`).
fn texture_meta(mcmeta: Option<&[u8]>) -> (MipmapStrategy, f32) {
    let Some(v) = mcmeta.and_then(|d| serde_json::from_slice::<serde_json::Value>(d).ok()) else {
        return (MipmapStrategy::Auto, 0.0);
    };
    let t = &v["texture"];
    let strategy = match t["mipmap_strategy"].as_str() {
        Some("mean") => MipmapStrategy::Mean,
        Some("cutout") => MipmapStrategy::Cutout,
        Some("strict_cutout") => MipmapStrategy::StrictCutout,
        Some("dark_cutout") => MipmapStrategy::DarkCutout,
        _ => MipmapStrategy::Auto,
    };
    (
        strategy,
        t["alpha_cutoff_bias"].as_f64().unwrap_or(0.0) as f32,
    )
}

impl Entry for Loaded {
    fn width(&self) -> u32 {
        self.frame.width
    }
    fn height(&self) -> u32 {
        self.frame.height
    }
    fn name(&self) -> &str {
        &self.id.path
    }
}

impl BlockAtlas {
    /// Собрать атлас `blocks` из ресурсов. `mip_levels` — как опция «Mipmap levels» (по умолчанию 4).
    pub fn build(rm: &ResourceManager, mip_levels: u32) -> Result<BlockAtlas, String> {
        Self::build_named(rm, "blocks", mip_levels)
    }

    /// Атлас по имени (`atlases/<name>.json`): `blocks`, `particles`… VANILLA: SpriteLoader#stitch.
    pub fn build_named(
        rm: &ResourceManager,
        name: &str,
        mip_levels: u32,
    ) -> Result<BlockAtlas, String> {
        let sources = atlas::load_sources(rm, name);
        let mut map = atlas::resolve(rm, &sources);
        let missing_id = Identifier::new("minecraft", "missingno");
        map.entry(missing_id.clone()).or_insert(SpriteLoad::Missing);
        let mut loaded = Vec::with_capacity(map.len());
        for (id, load) in &map {
            let Some(img) = load.load(rm) else { continue };
            let raw_meta = rm.read(&format!(
                "assets/{}/textures/{}.png.mcmeta",
                id.namespace, id.path
            ));
            let (strategy, cutoff_bias) = texture_meta(raw_meta.as_deref());
            let meta = raw_meta.as_deref().and_then(AnimationMeta::parse);
            let (frame, strip) = match meta {
                Some(m) => {
                    let (fw, fh) = m.frame_size(img.width, img.height);
                    match AnimatedTexture::create(&id.path, (fw, fh), img.width, img.height, &m) {
                        Some(a) => {
                            let first = a.frames.first().map_or(0, |f| f.index);
                            let f = img.crop(a.frame_x(first), a.frame_y(first), fw, fh);
                            (f, Some((img, a)))
                        }
                        None if fw > 0 && fh > 0 && fw <= img.width && fh <= img.height => {
                            (img.crop(0, 0, fw, fh), None)
                        }
                        None => (img, None),
                    }
                }
                None => (img, None),
            };
            loaded.push(Loaded {
                id: id.clone(),
                frame,
                strip,
                strategy,
                cutoff_bias,
            });
        }
        let st =
            stitcher::stitch(&loaded, 16384, 16384, mip_levels, 1).map_err(|e| e.to_string())?;
        let (w, h) = (st.width, st.height);
        let mut mips: Vec<Image> = (0..=mip_levels)
            .map(|l| Image::new((w >> l).max(1), (h >> l).max(1)))
            .collect();
        let mut sprites = HashMap::with_capacity(loaded.len());
        let mut animated = Vec::new();
        for &(idx, x, y) in &st.placements {
            let s = &loaded[idx];
            let chain = generate_mip_levels(
                &s.id.path,
                s.frame.clone(),
                mip_levels,
                s.strategy,
                s.cutoff_bias,
            );
            for (l, img) in chain.iter().enumerate() {
                blit_padded(&mut mips[l], img, x >> l, y >> l, st.padding >> l);
            }
            sprites.insert(
                s.id.clone(),
                SpriteUv {
                    u0: x as f32 / w as f32,
                    v0: y as f32 / h as f32,
                    u1: (x + s.frame.width) as f32 / w as f32,
                    v1: (y + s.frame.height) as f32 / h as f32,
                },
            );
            if let Some((strip, anim)) = &s.strip {
                animated.push(AnimatedSprite {
                    id: s.id.clone(),
                    x,
                    y,
                    frame_w: s.frame.width,
                    frame_h: s.frame.height,
                    anim: anim.clone(),
                    strip: strip.clone(),
                });
            }
        }
        let missing = sprites.get(&missing_id).copied().ok_or("нет missingno")?;
        Ok(BlockAtlas {
            width: w,
            height: h,
            mip_levels,
            mips,
            sprites,
            animated,
            missing,
        })
    }
}

fn blit(dst: &mut Image, src: &Image, x0: u32, y0: u32) {
    for y in 0..src.height {
        for x in 0..src.width {
            if x0 + x < dst.width && y0 + y < dst.height {
                dst.set(x0 + x, y0 + y, src.get(x, y));
            }
        }
    }
}

/// Копирует спрайт и растягивает его край на `pad` пикселей (clamp-to-edge), чтобы
/// билинейная/mip-фильтрация у границы не цепляла соседний спрайт или пустой фон атласа.
fn blit_padded(dst: &mut Image, src: &Image, x0: u32, y0: u32, pad: u32) {
    blit(dst, src, x0, y0);
    if src.width == 0 || src.height == 0 {
        return;
    }
    let (w, h) = (i64::from(src.width), i64::from(src.height));
    let p = i64::from(pad);
    for dy in -p..h + p {
        for dx in -p..w + p {
            if (0..w).contains(&dx) && (0..h).contains(&dy) {
                continue;
            }
            let (tx, ty) = (i64::from(x0) + dx, i64::from(y0) + dy);
            if tx < 0 || ty < 0 || tx >= i64::from(dst.width) || ty >= i64::from(dst.height) {
                continue;
            }
            let v = src.get(dx.clamp(0, w - 1) as u32, dy.clamp(0, h - 1) as u32);
            dst.set(tx as u32, ty as u32, v);
        }
    }
}

impl SpriteGetter for BlockAtlas {
    fn get(&self, id: &Identifier) -> Option<SpriteUv> {
        self.sprites.get(id).copied()
    }
    fn missing(&self) -> SpriteUv {
        self.missing
    }
    fn region_transparency(&self, u: [f32; 2], v: [f32; 2]) -> u8 {
        let img = &self.mips[0];
        let (w, h) = (img.width as f32, img.height as f32);
        let x0 = (u[0] * w).floor().max(0.0) as u32;
        let x1 = ((u[1] * w).ceil() as u32).min(img.width);
        let y0 = (v[0] * h).floor().max(0.0) as u32;
        let y1 = ((v[1] * h).ceil() as u32).min(img.height);
        let mut t = 0u8;
        for y in y0..y1 {
            for x in x0..x1 {
                let a = img.px[(y * img.width + x) as usize] >> 24;
                if a == 0 {
                    t = t.max(1);
                } else if a < 255 {
                    return 2;
                }
            }
        }
        t
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::install::GameInstall;

    #[test]
    fn real_atlas_if_game_present() {
        let Ok(inst) = GameInstall::locate("26.3") else {
            return;
        };
        let Ok(rm) = inst.resource_manager() else {
            return;
        };
        let a = BlockAtlas::build(&rm, 4).unwrap();
        assert!(a.width.is_power_of_two() && a.height.is_power_of_two());
        let stone = a.get(&Identifier::parse("block/stone")).unwrap();
        assert!(stone.u1 > stone.u0 && stone.v1 <= 1.0);
        assert!(a.animated.iter().any(|s| s.id.path == "block/water_still"));
        assert_eq!(a.mips.len(), 5);
        eprintln!(
            "atlas {}x{} sprites={} anim={}",
            a.width,
            a.height,
            a.sprites.len(),
            a.animated.len()
        );
    }
}
