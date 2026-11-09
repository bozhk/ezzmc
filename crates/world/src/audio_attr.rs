//! Звуковые атрибуты окружения. VANILLA: EnvironmentAttributes.BACKGROUND_MUSIC / AMBIENT_SOUNDS,
//! BackgroundMusic#select, AmbientSounds (26.3). Берутся из `attributes` биома/dimension_type.
use ezzmc_protocol::nbt::Tag;

#[derive(Clone, Debug, PartialEq)]
pub struct MusicDef {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BackgroundMusic {
    pub default: Option<MusicDef>,
    pub creative: Option<MusicDef>,
    pub underwater: Option<MusicDef>,
}

impl BackgroundMusic {
    /// VANILLA: BackgroundMusic#select.
    pub fn select(&self, creative: bool, underwater: bool) -> Option<&MusicDef> {
        if underwater && self.underwater.is_some() {
            self.underwater.as_ref()
        } else if creative && self.creative.is_some() {
            self.creative.as_ref()
        } else {
            self.default.as_ref()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mood {
    pub sound: String,
    pub tick_delay: i32,
    pub search_extent: i32,
    pub offset: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Addition {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AmbientSounds {
    pub loop_sound: Option<String>,
    pub mood: Option<Mood>,
    pub additions: Vec<Addition>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AudioAttrs {
    pub music: Option<BackgroundMusic>,
    pub ambient: Option<AmbientSounds>,
}

fn num(t: Option<&Tag>) -> Option<f64> {
    match t? {
        Tag::Float(f) => Some(f64::from(*f)),
        Tag::Double(d) => Some(*d),
        o => o.as_i64().map(|v| v as f64),
    }
}

fn sound(t: Option<&Tag>) -> Option<String> {
    let s = t?.as_str()?;
    Some(s.strip_prefix("minecraft:").unwrap_or(s).to_owned())
}

fn music_def(t: &Tag) -> Option<MusicDef> {
    Some(MusicDef {
        sound: sound(t.get("sound"))?,
        min_delay: num(t.get("min_delay"))? as i32,
        max_delay: num(t.get("max_delay"))? as i32,
        replace: t
            .get("replace_current_music")
            .and_then(Tag::as_i64)
            .is_some_and(|v| v != 0),
    })
}

fn addition(t: &Tag) -> Option<Addition> {
    Some(Addition {
        sound: sound(t.get("sound"))?,
        tick_chance: num(t.get("tick_chance"))?,
    })
}

impl AudioAttrs {
    /// `attrs` — compound `attributes` записи реестра (биом или dimension_type).
    pub fn from_attributes(attrs: Option<&Tag>) -> AudioAttrs {
        let Some(a) = attrs else {
            return AudioAttrs::default();
        };
        let music = a
            .get("minecraft:audio/background_music")
            .map(|m| BackgroundMusic {
                default: m.get("default").and_then(music_def),
                creative: m.get("creative").and_then(music_def),
                underwater: m.get("underwater").and_then(music_def),
            });
        let ambient = a
            .get("minecraft:audio/ambient_sounds")
            .map(|m| AmbientSounds {
                loop_sound: sound(m.get("loop")),
                mood: m.get("mood").and_then(|t| {
                    Some(Mood {
                        sound: sound(t.get("sound"))?,
                        tick_delay: num(t.get("tick_delay"))? as i32,
                        search_extent: num(t.get("block_search_extent"))? as i32,
                        offset: num(t.get("offset"))?,
                    })
                }),
                additions: match m.get("additions") {
                    Some(Tag::List(l)) => l.iter().filter_map(addition).collect(),
                    Some(t) => addition(t).into_iter().collect(),
                    None => Vec::new(),
                },
            });
        AudioAttrs { music, ambient }
    }
}
