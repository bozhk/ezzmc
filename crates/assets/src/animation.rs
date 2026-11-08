//! Анимированные текстуры `.mcmeta` (TODO-094): порт `AnimationMetadataSection` и
//! `SpriteContents.AnimatedTexture/AnimationState` 1.21.11.
use serde_json::Value;

/// Кадр анимации: индекс в ленте и длительность в тиках.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameInfo {
    pub index: u32,
    pub time: u32,
}

/// Секция `animation` из `<tex>.png.mcmeta`.
// VANILLA: AnimationMetadataSection (CODEC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationMeta {
    /// `index`, `time` (None → `frametime`)
    pub frames: Option<Vec<(u32, Option<u32>)>>,
    pub frame_width: Option<u32>,
    pub frame_height: Option<u32>,
    pub default_frame_time: u32,
    pub interpolate: bool,
}

fn positive(v: &Value) -> Option<u32> {
    v.as_u64()
        .filter(|&n| n > 0 && n <= u64::from(i32::MAX as u32))
        .map(|n| n as u32)
}

impl AnimationMeta {
    /// Разобрать содержимое `.mcmeta`; `None` — нет секции `animation` или она битая.
    pub fn parse(mcmeta: &[u8]) -> Option<AnimationMeta> {
        let root: Value = serde_json::from_slice(mcmeta).ok()?;
        let a = root.get("animation")?.as_object()?;
        let frames = match a.get("frames") {
            None => None,
            Some(list) => {
                let mut out = Vec::new();
                for f in list.as_array()? {
                    out.push(match f {
                        Value::Number(n) => (u32::try_from(n.as_u64()?).ok()?, None),
                        Value::Object(o) => (
                            u32::try_from(o.get("index")?.as_u64()?).ok()?,
                            match o.get("time") {
                                None => None,
                                Some(t) => Some(positive(t)?),
                            },
                        ),
                        _ => return None,
                    });
                }
                Some(out)
            }
        };
        let opt_pos = |k: &str| -> Option<Option<u32>> {
            match a.get(k) {
                None => Some(None),
                Some(v) => positive(v).map(Some),
            }
        };
        Some(AnimationMeta {
            frames,
            frame_width: opt_pos("width")?,
            frame_height: opt_pos("height")?,
            default_frame_time: match a.get("frametime") {
                None => 1,
                Some(v) => positive(v)?,
            },
            interpolate: match a.get("interpolate") {
                None => false,
                Some(v) => v.as_bool()?,
            },
        })
    }

    /// Размер кадра по размеру PNG.
    // VANILLA: AnimationMetadataSection#calculateFrameSize
    pub fn frame_size(&self, w: u32, h: u32) -> (u32, u32) {
        match (self.frame_width, self.frame_height) {
            (Some(fw), Some(fh)) => (fw, fh),
            (Some(fw), None) => (fw, h),
            (None, Some(fh)) => (w, fh),
            (None, None) => {
                let m = w.min(h);
                (m, m)
            }
        }
    }
}

/// Раскладка анимации спрайта: кадры + размер ряда в кадрах.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimatedTexture {
    pub frames: Vec<FrameInfo>,
    pub frame_row_size: u32,
    pub interpolate: bool,
}

impl AnimatedTexture {
    /// `None` — не анимирован (≤ 1 кадра).
    // VANILLA: SpriteContents#createAnimatedTexture
    pub fn create(
        name: &str,
        frame: (u32, u32),
        img_w: u32,
        img_h: u32,
        meta: &AnimationMeta,
    ) -> Option<AnimatedTexture> {
        let (fw, fh) = frame;
        if fw == 0 || fh == 0 {
            return None;
        }
        let row = img_w / fw;
        let total = row * (img_h / fh);
        let dflt = meta.default_frame_time;
        let frames: Vec<FrameInfo> = match &meta.frames {
            None => (0..total)
                .map(|index| FrameInfo { index, time: dflt })
                .collect(),
            Some(list) => {
                let mut out = Vec::new();
                let mut used = vec![false; total as usize];
                for (n, &(index, time)) in list.iter().enumerate() {
                    let time = time.unwrap_or(dflt);
                    if index >= total {
                        tracing::warn!("Invalid frame index on sprite {name} frame {n}: {index}");
                        continue;
                    }
                    used[index as usize] = true;
                    out.push(FrameInfo { index, time });
                }
                let unused: Vec<usize> = (0..total as usize).filter(|&i| !used[i]).collect();
                if !unused.is_empty() {
                    tracing::warn!("Unused frames in sprite {name}: {unused:?}");
                }
                out
            }
        };
        (frames.len() > 1).then_some(AnimatedTexture {
            frames,
            frame_row_size: row.max(1),
            interpolate: meta.interpolate,
        })
    }

    pub fn frame_x(&self, index: u32) -> u32 {
        index % self.frame_row_size
    }

    pub fn frame_y(&self, index: u32) -> u32 {
        index / self.frame_row_size
    }

    /// Уникальные индексы кадров в порядке первого появления.
    pub fn unique_frames(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for f in &self.frames {
            if !v.contains(&f.index) {
                v.push(f.index);
            }
        }
        v
    }

    pub fn create_state(&self) -> AnimationState {
        AnimationState {
            frame: 0,
            sub_frame: 0,
            dirty: true,
        }
    }
}

/// Текущее состояние анимации (по одному на спрайт).
// VANILLA: SpriteContents.AnimationState
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationState {
    pub frame: usize,
    pub sub_frame: u32,
    dirty: bool,
}

impl AnimationState {
    /// Один тик клиента.
    pub fn tick(&mut self, anim: &AnimatedTexture) {
        self.sub_frame += 1;
        self.dirty = false;
        let cur = anim.frames[self.frame];
        if self.sub_frame >= cur.time {
            self.frame = (self.frame + 1) % anim.frames.len();
            self.sub_frame = 0;
            if cur.index != anim.frames[self.frame].index {
                self.dirty = true;
            }
        }
    }

    /// Нужно ли перерисовать спрайт в атлас.
    pub fn needs_to_draw(&self, anim: &AnimatedTexture) -> bool {
        anim.interpolate || self.dirty
    }

    /// Индекс текущего кадра.
    pub fn current_index(&self, anim: &AnimatedTexture) -> u32 {
        anim.frames[self.frame].index
    }

    /// Индекс следующего кадра (для интерполяции).
    pub fn next_index(&self, anim: &AnimatedTexture) -> u32 {
        anim.frames[(self.frame + 1) % anim.frames.len()].index
    }

    /// Прогресс интерполяции в тысячных, как `(int)(progress * 1000.0F)`.
    pub fn progress_milli(&self, anim: &AnimatedTexture) -> u32 {
        let p = self.sub_frame as f32 / anim.frames[self.frame].time as f32;
        (p * 1000.0) as u32
    }
}

/// CPU-эквивалент `animate_sprite_interpolate.fsh`: `mix(cur, next, milli/1000)` по каналам ARGB.
pub fn interpolate_pixel(cur: u32, next: u32, milli: u32) -> u32 {
    let t = milli as f32 / 1000.0;
    let ch = |s: u32| -> u32 {
        let a = ((cur >> s) & 0xFF) as f32;
        let b = ((next >> s) & 0xFF) as f32;
        (a + (b - a) * t).round().clamp(0.0, 255.0) as u32
    };
    ch(24) << 24 | ch(16) << 16 | ch(8) << 8 | ch(0)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_frame_size() {
        let m = AnimationMeta::parse(
            br#"{"animation":{"frametime":2,"interpolate":true,"frames":[0,{"index":1,"time":5}]}}"#,
        )
        .unwrap();
        assert_eq!(m.frames, Some(vec![(0, None), (1, Some(5))]));
        assert!(m.interpolate);
        assert_eq!(m.frame_size(16, 64), (16, 16));
        assert!(AnimationMeta::parse(br#"{"animation":{"frametime":0}}"#).is_none());
        assert!(AnimationMeta::parse(br#"{}"#).is_none());
    }

    #[test]
    fn default_frames_and_tick() {
        let m = AnimationMeta::parse(br#"{"animation":{"frametime":2}}"#).unwrap();
        let a = AnimatedTexture::create("t", (16, 16), 16, 48, &m).unwrap();
        assert_eq!(a.frames.len(), 3);
        let mut s = a.create_state();
        s.tick(&a);
        assert_eq!(s.current_index(&a), 0);
        s.tick(&a);
        assert_eq!((s.current_index(&a), s.needs_to_draw(&a)), (1, true));
        s.tick(&a);
        assert!(!s.needs_to_draw(&a));
        assert_eq!(s.progress_milli(&a), 500);
    }

    #[test]
    fn invalid_frames_dropped_and_static() {
        let m = AnimationMeta::parse(br#"{"animation":{"frames":[0,7]}}"#).unwrap();
        // кадр 7 вне диапазона → остаётся один кадр → не анимировано
        assert!(AnimatedTexture::create("t", (16, 16), 16, 32, &m).is_none());
        let m = AnimationMeta::parse(br#"{"animation":{"width":8,"height":8}}"#).unwrap();
        let a = AnimatedTexture::create("t", (8, 8), 16, 16, &m).unwrap();
        assert_eq!((a.frame_x(3), a.frame_y(3)), (1, 1));
    }

    #[test]
    fn mix() {
        assert_eq!(
            interpolate_pixel(0xFF00_0000, 0xFFFF_FFFF, 500),
            0xFF80_8080
        );
    }
}
