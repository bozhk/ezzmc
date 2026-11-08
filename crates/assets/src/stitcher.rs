//! Укладка спрайтов в атлас (TODO-092): точный порт `Stitcher` 1.21.11.

pub trait Entry {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn name(&self) -> &str;
}

struct Holder {
    idx: usize,
    width: u32,
    height: u32,
}

struct Region {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    sub: Option<Vec<Region>>,
    holder: Option<usize>, // индекс в списке holders
}

impl Region {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Region {
            x,
            y,
            width,
            height,
            sub: None,
            holder: None,
        }
    }

    // VANILLA: Stitcher.Region#add (1.21.11)
    fn add(&mut self, hi: usize, w: u32, h: u32) -> bool {
        if self.holder.is_some() {
            return false;
        }
        if w > self.width || h > self.height {
            return false;
        }
        if w == self.width && h == self.height {
            self.holder = Some(hi);
            return true;
        }
        if self.sub.is_none() {
            let mut s = vec![Region::new(self.x, self.y, w, h)];
            let dw = self.width - w;
            let dh = self.height - h;
            if dh > 0 && dw > 0 {
                let a = self.height.max(dw);
                let b = self.width.max(dh);
                if a >= b {
                    s.push(Region::new(self.x, self.y + h, w, dh));
                    s.push(Region::new(self.x + w, self.y, dw, self.height));
                } else {
                    s.push(Region::new(self.x + w, self.y, dw, h));
                    s.push(Region::new(self.x, self.y + h, self.width, dh));
                }
            } else if dw == 0 {
                s.push(Region::new(self.x, self.y + h, w, dh));
            } else if dh == 0 {
                s.push(Region::new(self.x + w, self.y, dw, h));
            }
            self.sub = Some(s);
        }
        if let Some(s) = &mut self.sub {
            for r in s {
                if r.add(hi, w, h) {
                    return true;
                }
            }
        }
        false
    }

    fn walk(&self, out: &mut Vec<(usize, u32, u32)>) {
        if let Some(h) = self.holder {
            out.push((h, self.x, self.y));
        } else if let Some(s) = &self.sub {
            for r in s {
                r.walk(out);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StitchError {
    pub failed: String,
}

impl std::fmt::Display for StitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "не удалось уложить спрайт {} в атлас", self.failed)
    }
}
impl std::error::Error for StitchError {}

/// Результат: размер атласа и положение каждого спрайта (индекс входа, x, y) + padding.
pub struct Stitched {
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    pub placements: Vec<(usize, u32, u32)>,
}

fn smallest_fitting_min_texel(v: u32, mip: u32) -> u32 {
    ((v >> mip) + u32::from(v & ((1 << mip) - 1) != 0)) << mip
}

fn smallest_pow2(v: u32) -> u32 {
    v.max(1).next_power_of_two()
}

// VANILLA: Stitcher#stitch (1.21.11); padding = 1 << mip << clamp(anisotropy-1, 0, 4)
pub fn stitch<T: Entry>(
    entries: &[T],
    max_w: u32,
    max_h: u32,
    mip: u32,
    anisotropy: u32,
) -> Result<Stitched, StitchError> {
    let padding = (1u32 << mip) << anisotropy.saturating_sub(1).min(4);
    let holders: Vec<Holder> = entries
        .iter()
        .enumerate()
        .map(|(idx, e)| Holder {
            idx,
            width: smallest_fitting_min_texel(e.width() + padding * 2, mip),
            height: smallest_fitting_min_texel(e.height() + padding * 2, mip),
        })
        .collect();
    let mut order: Vec<&Holder> = holders.iter().collect();
    // высота ↓, ширина ↓, имя ↑
    order.sort_by(|a, b| {
        b.height
            .cmp(&a.height)
            .then(b.width.cmp(&a.width))
            .then_with(|| entries[a.idx].name().cmp(entries[b.idx].name()))
    });
    let (mut sx, mut sy) = (0u32, 0u32);
    let mut storage: Vec<Region> = Vec::new();
    for h in order {
        let mut done = false;
        for r in &mut storage {
            if r.add(h.idx, h.width, h.height) {
                done = true;
                break;
            }
        }
        if done {
            continue;
        }
        // expand
        let (w1, h1) = (smallest_pow2(sx), smallest_pow2(sy));
        let (w2, h2) = (smallest_pow2(sx + h.width), smallest_pow2(sy + h.height));
        let can_w = w2 <= max_w;
        let can_h = h2 <= max_h;
        if !can_w && !can_h {
            return Err(StitchError {
                failed: entries[h.idx].name().to_owned(),
            });
        }
        let grow_w = can_w && w1 != w2;
        let grow_h = can_h && h1 != h2;
        let right = if grow_w ^ grow_h {
            grow_w
        } else {
            can_w && w1 <= h1
        };
        let mut region;
        if right {
            if sy == 0 {
                sy = h2;
            }
            region = Region::new(sx, 0, w2 - sx, sy);
            sx = w2;
        } else {
            region = Region::new(0, sy, sx, h2 - sy);
            sy = h2;
        }
        region.add(h.idx, h.width, h.height);
        storage.push(region);
    }
    let mut raw = Vec::new();
    for r in &storage {
        r.walk(&mut raw);
    }
    // координаты — левый верх ячейки; спрайт лежит со сдвигом padding (как SpriteLoader).
    let placements = raw
        .into_iter()
        .map(|(i, x, y)| (i, x + padding, y + padding))
        .collect();
    Ok(Stitched {
        width: sx,
        height: sy,
        padding,
        placements,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    struct E(String, u32, u32);
    impl Entry for E {
        fn width(&self) -> u32 {
            self.1
        }
        fn height(&self) -> u32 {
            self.2
        }
        fn name(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn packs_without_overlap_pow2() {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(E(format!("s{i:03}"), 16, 16));
        }
        v.push(E("big".into(), 32, 32));
        v.push(E("tall".into(), 16, 48));
        let r = stitch(&v, 1024, 1024, 0, 1).unwrap();
        assert!(r.width.is_power_of_two() && r.height.is_power_of_two());
        assert_eq!(r.placements.len(), v.len());
        let rects: Vec<(u32, u32, u32, u32)> = r
            .placements
            .iter()
            .map(|&(i, x, y)| {
                (
                    x - r.padding,
                    y - r.padding,
                    v[i].1 + 2 * r.padding,
                    v[i].2 + 2 * r.padding,
                )
            })
            .collect();
        for (i, a) in rects.iter().enumerate() {
            assert!(a.0 + a.2 <= r.width && a.1 + a.3 <= r.height);
            for b in &rects[i + 1..] {
                let sep =
                    a.0 + a.2 <= b.0 || b.0 + b.2 <= a.0 || a.1 + a.3 <= b.1 || b.1 + b.3 <= a.1;
                assert!(sep, "{a:?} пересекается с {b:?}");
            }
        }
    }

    #[test]
    fn too_big_fails_and_mip_padding() {
        let v = vec![E("a".into(), 20, 20)];
        assert!(stitch(&v, 16, 16, 0, 1).is_err());
        // mip=4: padding = 16, ячейка 20+32=52 → 64
        let r = stitch(&v, 1024, 1024, 4, 1).unwrap();
        assert_eq!((r.padding, r.width, r.height), (16, 64, 64));
        assert_eq!(r.placements, vec![(0, 16, 16)]);
        assert_eq!(smallest_fitting_min_texel(52, 4), 64);
    }
}
