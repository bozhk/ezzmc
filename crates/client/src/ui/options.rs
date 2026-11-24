//! Опции (`options.txt`, строки `ключ:значение`) и описания экранов настроек.
//!
//! VANILLA: Options / OptionInstance (значения по умолчанию ванильные). Чужие ключи файла сохраняются.
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub enum Kind {
    Toggle,
    /// min, max, шаг (0 — непрерывный).
    Slider(f32, f32, f32),
    /// (значение в файле, ключ перевода).
    Cycle(&'static [(&'static str, &'static str)]),
    /// Логарифмический слайдер: целый шаг min..max, значение = 10^(шаг/100) (Options#logMouse).
    Log(i32, i32),
}

pub struct Def {
    pub key: &'static str,
    pub lang: &'static str,
    pub kind: Kind,
    pub default: &'static str,
}

const fn d(key: &'static str, lang: &'static str, kind: Kind, default: &'static str) -> Def {
    Def {
        key,
        lang,
        kind,
        default,
    }
}

use Kind::{Cycle, Log, Slider, Toggle};

const CHAT_VIS: &[(&str, &str)] = &[
    ("0", "options.chat.visibility.full"),
    ("1", "options.chat.visibility.system"),
    ("2", "options.chat.visibility.hidden"),
];

const HOLD_TOGGLE: &[(&str, &str)] = &[
    ("false", "options.key.hold"),
    ("true", "options.key.toggle"),
];

pub const FOV: Def = d("fov", "options.fov", Slider(30.0, 110.0, 1.0), "70.0");

pub const VIDEO: &[Def] = &[
    // VANILLA: VideoSettingsScreen — порядок «Quality & Performance» → «Display» → «Preferences».
    d(
        "biomeBlendRadius",
        "options.biomeBlendRadius",
        Slider(0.0, 7.0, 1.0),
        "2",
    ),
    d(
        "renderDistance",
        "options.renderDistance",
        Slider(2.0, 127.0, 1.0),
        "12",
    ),
    d(
        "prioritizeChunkUpdates",
        "options.prioritizeChunkUpdates",
        Cycle(&[
            ("0", "options.prioritizeChunkUpdates.none"),
            ("1", "options.prioritizeChunkUpdates.byPlayer"),
            ("2", "options.prioritizeChunkUpdates.nearby"),
        ]),
        "0",
    ),
    d(
        "simulationDistance",
        "options.simulationDistance",
        Slider(5.0, 32.0, 1.0),
        "12",
    ),
    d("ao", "options.ao", Toggle, "true"),
    d(
        "renderClouds",
        "options.renderClouds",
        Cycle(&[
            ("false", "options.off"),
            ("fast", "options.clouds.fast"),
            ("true", "options.clouds.fancy"),
        ]),
        "true",
    ),
    d(
        "particles",
        "options.particles",
        Cycle(&[
            ("0", "options.particles.all"),
            ("1", "options.particles.decreased"),
            ("2", "options.particles.minimal"),
        ]),
        "0",
    ),
    d(
        "mipmapLevels",
        "options.mipmapLevels",
        Slider(0.0, 4.0, 1.0),
        "4",
    ),
    d("entityShadows", "options.entityShadows", Toggle, "true"),
    d(
        "entityDistanceScaling",
        "options.entityDistanceScaling",
        Slider(0.5, 5.0, 0.25),
        "1.0",
    ),
    d(
        "menuBackgroundBlurriness",
        "options.accessibility.menu_background_blurriness",
        Slider(0.0, 10.0, 1.0),
        "5",
    ),
    d(
        "cloudRange",
        "options.renderCloudsDistance",
        Slider(2.0, 128.0, 1.0),
        "128",
    ),
    // VANILLA: Options#cutoutLeaves («See-Through Leaves»).
    d("cutoutLeaves", "options.cutoutLeaves", Toggle, "true"),
    d(
        "improvedTransparency",
        "options.improvedTransparency",
        Toggle,
        "false",
    ),
    d(
        "textureFiltering",
        "options.textureFiltering",
        Cycle(&[
            ("0", "options.textureFiltering.none"),
            ("1", "options.textureFiltering.rgss"),
            ("2", "options.textureFiltering.anisotropic"),
        ]),
        "0",
    ),
    d(
        "maxAnisotropyBit",
        "options.maxAnisotropy",
        Slider(1.0, 3.0, 1.0),
        "2",
    ),
    d(
        "weatherRadius",
        "options.weatherRadius",
        Slider(3.0, 10.0, 1.0),
        "10",
    ),
    d(
        "maxFps",
        "options.framerateLimit",
        Slider(10.0, 260.0, 10.0),
        "120",
    ),
    d("enableVsync", "options.vsync", Toggle, "true"),
    d(
        "inactivityFpsLimit",
        "options.inactivityFpsLimit",
        Cycle(&[
            ("minimized", "options.inactivityFpsLimit.minimized"),
            ("afk", "options.inactivityFpsLimit.afk"),
        ]),
        "afk",
    ),
    d("guiScale", "options.guiScale", Slider(0.0, 8.0, 1.0), "0"),
    d("fullscreen", "options.fullscreen", Toggle, "false"),
    d(
        "exclusiveFullscreen",
        "options.exclusiveFullscreen",
        Toggle,
        "false",
    ),
    d("gamma", "options.gamma", Slider(0.0, 1.0, 0.0), "0.5"),
    d(
        "showAutosaveIndicator",
        "options.autosaveIndicator",
        Toggle,
        "true",
    ),
    d("vignette", "options.vignette", Toggle, "true"),
    d(
        "attackIndicator",
        "options.attackIndicator",
        Cycle(&[
            ("0", "options.attack.off"),
            ("1", "options.attack.crosshair"),
            ("2", "options.attack.hotbar"),
        ]),
        "1",
    ),
    d(
        "chunkSectionFadeInTime",
        "options.chunkFade",
        Slider(0.0, 2.0, 0.05),
        "0.75",
    ),
    d("bobView", "options.viewBobbing", Toggle, "true"),
    d(
        "graphicsMode",
        "options.graphics",
        Cycle(&[
            ("0", "options.graphics.fast"),
            ("1", "options.graphics.fancy"),
            ("2", "options.graphics.fabulous"),
        ]),
        "1",
    ),
    // Не ваниль: путь отрисовки кубов (compute: куллинг/Hi-Z на GPU + MDI; выкл — ванильный Packed).
    d(
        "computeRenderer",
        "options.ezzmc.computeRenderer",
        Toggle,
        "true",
    ),
];

/// Индексы в [`VIDEO`] (порядок ванильный, см. тест `video_indices`).
pub const RENDER_DISTANCE: usize = 1;
pub const MAX_FPS: usize = 17;
pub const VSYNC: usize = 18;
pub const GUI_SCALE: usize = 20;
pub const FULLSCREEN: usize = 21;

/// Индекс «Compute Renderer» в [`VIDEO`].
pub const COMPUTE_RENDERER: usize = 30;

/// Индекс «See-Through Leaves» в [`VIDEO`].
pub const CUTOUT_LEAVES: usize = 12;

pub const SOUND: &[Def] = &[
    d(
        "soundCategory_master",
        "soundCategory.master",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_music",
        "soundCategory.music",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_record",
        "soundCategory.record",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_weather",
        "soundCategory.weather",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_block",
        "soundCategory.block",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_hostile",
        "soundCategory.hostile",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_neutral",
        "soundCategory.neutral",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_player",
        "soundCategory.player",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_ambient",
        "soundCategory.ambient",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "soundCategory_voice",
        "soundCategory.voice",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    // VANILLA: SoundOptionsScreen#addOptions — устройство, субтитры+направленный звук, частота музыки+toast.
    // Выбор устройства не реализован (cpal берёт системное): единственное значение «по умолчанию».
    d(
        "soundDevice",
        "options.audioDevice",
        Cycle(&[("", "options.audioDevice.default")]),
        "",
    ),
    d("showSubtitles", "options.showSubtitles", Toggle, "false"),
    d(
        "directionalAudio",
        "options.directionalAudio",
        Toggle,
        "false",
    ),
    d(
        "musicFrequency",
        "options.music_frequency",
        Cycle(&[
            ("DEFAULT", "options.music_frequency.default"),
            ("FREQUENT", "options.music_frequency.frequent"),
            ("CONSTANT", "options.music_frequency.constant"),
        ]),
        "DEFAULT",
    ),
    d(
        "musicToast",
        "options.musicToast",
        Cycle(&[
            ("never", "options.musicToast.never"),
            ("pause", "options.musicToast.pauseMenu"),
            ("pause_and_toast", "options.musicToast.pauseMenuAndToast"),
        ]),
        "never",
    ),
];

pub const SKIN: &[Def] = &[
    d("modelPart_cape", "options.modelPart.cape", Toggle, "true"),
    d(
        "modelPart_jacket",
        "options.modelPart.jacket",
        Toggle,
        "true",
    ),
    d(
        "modelPart_left_sleeve",
        "options.modelPart.left_sleeve",
        Toggle,
        "true",
    ),
    d(
        "modelPart_right_sleeve",
        "options.modelPart.right_sleeve",
        Toggle,
        "true",
    ),
    d(
        "modelPart_left_pants_leg",
        "options.modelPart.left_pants_leg",
        Toggle,
        "true",
    ),
    d(
        "modelPart_right_pants_leg",
        "options.modelPart.right_pants_leg",
        Toggle,
        "true",
    ),
    d("modelPart_hat", "options.modelPart.hat", Toggle, "true"),
    d(
        "mainHand",
        "options.mainHand",
        Cycle(&[
            ("left", "options.mainHand.left"),
            ("right", "options.mainHand.right"),
        ]),
        "right",
    ),
];

pub const CHAT: &[Def] = &[
    // VANILLA: Options#chatVisibility (ChatVisiblity: FULL/SYSTEM/HIDDEN, в файле — id).
    d(
        "chatVisibility",
        "options.chat.visibility",
        Cycle(CHAT_VIS),
        "0",
    ),
    d("chatColors", "options.chat.color", Toggle, "true"),
    d("chatLinks", "options.chat.links", Toggle, "true"),
    d(
        "chatLinksPrompt",
        "options.chat.links.prompt",
        Toggle,
        "true",
    ),
    d(
        "autoSuggestions",
        "options.autoSuggestCommands",
        Toggle,
        "true",
    ),
    d(
        "chatOpacity",
        "options.chat.opacity",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "textBackgroundOpacity",
        "options.accessibility.text_background_opacity",
        Slider(0.0, 1.0, 0.0),
        "0.5",
    ),
    d(
        "chatScale",
        "options.chat.scale",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "chatWidth",
        "options.chat.width",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "chatHeightFocused",
        "options.chat.height.focused",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "chatHeightUnfocused",
        "options.chat.height.unfocused",
        Slider(0.0, 1.0, 0.0),
        "0.4375",
    ),
    d(
        "chatLineSpacing",
        "options.chat.line_spacing",
        Slider(0.0, 1.0, 0.0),
        "0.0",
    ),
];

/// VANILLA: MouseSettingsScreen#options.
pub const MOUSE: &[Def] = &[
    d(
        "mouseSensitivity",
        "options.sensitivity",
        Slider(0.0, 1.0, 0.0),
        "0.5",
    ),
    d(
        "mouseWheelSensitivity",
        "options.mouseWheelSensitivity",
        Log(-200, 100),
        "1.0",
    ),
    d(
        "discrete_mouse_scroll",
        "options.discrete_mouse_scroll",
        Toggle,
        "false",
    ),
    d("invertXMouse", "options.invertMouseX", Toggle, "false"),
    d("invertYMouse", "options.invertMouseY", Toggle, "false"),
    d(
        "allowCursorChanges",
        "options.allowCursorChanges",
        Toggle,
        "true",
    ),
];

/// VANILLA: ControlsScreen#options (после кнопок Mouse Settings / Key Binds).
pub const CONTROLS: &[Def] = &[
    d("toggleCrouch", "key.sneak", Cycle(HOLD_TOGGLE), "false"),
    d("toggleSprint", "key.sprint", Cycle(HOLD_TOGGLE), "false"),
    d("toggleAttack", "key.attack", Cycle(HOLD_TOGGLE), "false"),
    d("toggleUse", "key.use", Cycle(HOLD_TOGGLE), "false"),
    d("autoJump", "options.autoJump", Toggle, "false"),
    d(
        "sprintWindow",
        "options.sprintWindow",
        Slider(0.0, 10.0, 1.0),
        "7",
    ),
    d("quitShortcuts", "options.quitShortcuts", Toggle, "true"),
    d(
        "operatorItemsTab",
        "options.operatorItemsTab",
        Toggle,
        "false",
    ),
];

pub const ACCESSIBILITY: &[Def] = &[
    // VANILLA: AccessibilityOptionsScreen#options (без narrator/highContrast/narratorHotkey — нет озвучки и пака).
    d("showSubtitles", "options.showSubtitles", Toggle, "false"),
    d(
        "menuBackgroundBlurriness",
        "options.accessibility.menu_background_blurriness",
        Slider(0.0, 10.0, 1.0),
        "5",
    ),
    d(
        "textBackgroundOpacity",
        "options.accessibility.text_background_opacity",
        Slider(0.0, 1.0, 0.0),
        "0.5",
    ),
    d(
        "backgroundForChatOnly",
        "options.accessibility.text_background",
        Cycle(&[
            ("true", "options.accessibility.text_background.chat"),
            ("false", "options.accessibility.text_background.everywhere"),
        ]),
        "true",
    ),
    d(
        "chatOpacity",
        "options.chat.opacity",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "chatLineSpacing",
        "options.chat.line_spacing",
        Slider(0.0, 1.0, 0.0),
        "0.0",
    ),
    d(
        "chatDelay",
        "options.chat.delay_instant",
        Slider(0.0, 6.0, 0.1),
        "0.0",
    ),
    d(
        "notificationDisplayTime",
        "options.notifications.display_time",
        Slider(0.5, 10.0, 0.1),
        "1.0",
    ),
    d("bobView", "options.viewBobbing", Toggle, "true"),
    d(
        "screenEffectScale",
        "options.screenEffectScale",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "fovEffectScale",
        "options.fovEffectScale",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "darknessEffectScale",
        "options.darknessEffectScale",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "damageTiltStrength",
        "options.damageTiltStrength",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "glintSpeed",
        "options.glintSpeed",
        Slider(0.0, 1.0, 0.0),
        "0.5",
    ),
    d(
        "glintStrength",
        "options.glintStrength",
        Slider(0.0, 1.0, 0.0),
        "0.75",
    ),
    d(
        "hideLightningFlashes",
        "options.hideLightningFlashes",
        Toggle,
        "false",
    ),
    d(
        "darkMojangStudiosBackground",
        "options.darkMojangStudiosBackgroundColor",
        Toggle,
        "false",
    ),
    d(
        "panoramaScrollSpeed",
        "options.accessibility.panorama_speed",
        Slider(0.0, 1.0, 0.0),
        "1.0",
    ),
    d(
        "hideSplashTexts",
        "options.hideSplashTexts",
        Toggle,
        "false",
    ),
    d(
        "rotateWithMinecart",
        "options.rotateWithMinecart",
        Toggle,
        "false",
    ),
    d(
        "highContrastBlockOutline",
        "options.accessibility.high_contrast_block_outline",
        Toggle,
        "false",
    ),
];

pub struct Options {
    path: PathBuf,
    kv: Vec<(String, String)>,
}

impl Options {
    pub fn load(path: &Path) -> Self {
        let kv = std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.split_once(':'))
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        let o = Self {
            path: path.to_owned(),
            kv,
        };
        o.apply_audio();
        o
    }

    /// VANILLA: SoundManager#updateSourceVolume — громкость категорий из options.txt.
    pub fn apply_audio(&self) {
        use crate::audio::Cat::*;
        let cats = [
            Master, Music, Records, Weather, Blocks, Hostile, Neutral, Players, Ambient, Voice,
        ];
        for (def, cat) in SOUND.iter().zip(cats) {
            crate::audio::set_volume(cat, self.f32(def));
        }
    }

    /// Все пары `ключ:значение` (для ключей клавиш `key_*`).
    pub fn pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.kv.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn get(&self, def: &Def) -> &str {
        self.kv
            .iter()
            .find(|(k, _)| k == def.key)
            .map_or(def.default, |(_, v)| v)
    }

    /// Значение произвольного ключа (вне таблиц `Def`).
    pub fn raw(&self, key: &str) -> Option<&str> {
        self.kv
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    pub fn set(&mut self, key: &str, value: String) {
        match self.kv.iter_mut().find(|(k, _)| k == key) {
            Some(e) => e.1 = value,
            None => self.kv.push((key.to_owned(), value)),
        }
        if key.starts_with("soundCategory") {
            self.apply_audio();
        }
    }

    /// Значение опции из ACCESSIBILITY по ключу.
    pub fn acc(&self, key: &str) -> f32 {
        ACCESSIBILITY
            .iter()
            .find(|d| d.key == key)
            .map_or(0.0, |d| self.f32(d))
    }

    pub fn acc_bool(&self, key: &str) -> bool {
        ACCESSIBILITY
            .iter()
            .find(|d| d.key == key)
            .is_some_and(|d| self.get(d) == "true")
    }

    pub fn f32(&self, def: &Def) -> f32 {
        self.get(def)
            .parse()
            .unwrap_or_else(|_| def.default.parse().unwrap_or(0.0))
    }

    pub fn save(&self) {
        let s: String = self.kv.iter().map(|(k, v)| format!("{k}:{v}\n")).collect();
        if let Err(e) = std::fs::write(&self.path, s) {
            tracing::error!("options.txt: {e}");
        }
    }
}

pub fn format_num(v: f32, step: f32) -> String {
    if step >= 1.0 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:?}")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_roundtrip() {
        let p = std::env::temp_dir().join("ezzmc_opts_test.txt");
        std::fs::write(&p, "foo:bar\nmaxFps:60\n").unwrap();
        let mut o = Options::load(&p);
        assert_eq!(o.f32(&VIDEO[MAX_FPS]), 60.0);
        assert_eq!(o.get(&VIDEO[RENDER_DISTANCE]), "12");
        o.set("renderDistance", "8".into());
        o.save();
        let o2 = Options::load(&p);
        assert_eq!(o2.get(&VIDEO[RENDER_DISTANCE]), "8");
        assert_eq!(o2.kv[0].0, "foo");
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn video_indices() {
        for (i, k) in [
            (RENDER_DISTANCE, "renderDistance"),
            (MAX_FPS, "maxFps"),
            (VSYNC, "enableVsync"),
            (GUI_SCALE, "guiScale"),
            (FULLSCREEN, "fullscreen"),
        ] {
            assert_eq!(VIDEO[i].key, k);
        }
    }

    #[test]
    fn compute_renderer_index() {
        assert_eq!(VIDEO[COMPUTE_RENDERER].key, "computeRenderer");
    }

    #[test]
    fn cutout_leaves_index() {
        assert_eq!(VIDEO[CUTOUT_LEAVES].key, "cutoutLeaves");
    }
}
