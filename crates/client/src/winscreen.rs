//! Экран концовки (End Poem и титры).
//! VANILLA: WinScreen (26.3): `assets/minecraft/texts/{end.txt,credits.json,postcredits.txt}`,
//! логотип (LogoRenderer) на `height + 50`, текст с `height + 150`, шаг 12 px, ширина 256 px.
//! Фон поэмы — END_PORTAL + `credits_vignette` (ezzmc_render::creditsbg), музыка — Musics.CREDITS (main.rs).

use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::creditsbg::CreditsBackground;
use ezzmc_render::endportaldraw::Rgba;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;
use ezzmc_world::noise::LegacyRandom;

/// VANILLA: WinScreen#SECTION_HEADING (ChatFormatting.WHITE).
const SECTION_HEADING: &str = "§f============";
/// VANILLA: WinScreen#NAME_PREFIX.
const NAME_PREFIX: &str = "           ";
/// VANILLA: WinScreen#OBFUSCATE_TOKEN = WHITE + OBFUSCATED + GREEN + AQUA.
const OBFUSCATE_TOKEN: &str = "§f§k§a§b";
/// VANILLA: Font#split(component, 256) в addPoemLines.
const POEM_WIDTH: f32 = 256.0;

/// Источник строки до разбиения шрифтом (разбиение поэмы требует ширин глифов).
enum Src {
    /// Строка поэмы: делится по 256 px.
    Poem(String),
    /// Готовая строка титров: (текст с §-кодами, по центру).
    Line(String, bool),
    Empty,
}

/// Состояние экрана титров/поэмы Энда.
pub struct WinScreen {
    /// VANILLA: WinScreen#poem — фон END_PORTAL (иначе фон экрана по умолчанию).
    poem: bool,
    bg: Option<CreditsBackground>,
    src: Vec<Src>,
    /// VANILLA: WinScreen#lines + centeredLines (строятся в init, когда есть шрифт).
    lines: Option<Vec<(String, bool)>>,
    scroll: f32,
    unmodified_speed: f32,
    /// VANILLA: speedupActive (пробел).
    pub speedup: bool,
    /// VANILLA: speedupModifiers — левый и правый Ctrl.
    pub ctrl: [bool; 2],
    /// VANILLA: direction = -1, пока зажата стрелка вверх.
    pub reverse: bool,
    last_step: std::time::Instant,
}

impl WinScreen {
    /// VANILLA: WinScreen#init — end.txt (poem), credits.json, postcredits.txt (poem).
    pub fn load(rm: &ResourceManager, player_name: &str, poem: bool) -> Self {
        let mut src = Vec::new();
        let read = |path: &str| -> Option<String> {
            let raw = rm.read(path);
            if raw.is_none() {
                tracing::error!("Couldn't load credits from file {path}");
            }
            raw.and_then(|r| String::from_utf8(r.to_vec()).ok())
        };
        // VANILLA: RandomSource.createThreadLocalInstance(8124371L) один на файл поэмы.
        let add_poem = |src: &mut Vec<Src>, text: &str| {
            let mut rng = LegacyRandom::new(8_124_371);
            for raw in text.lines() {
                let mut line = raw.replace("PLAYERNAME", player_name);
                while let Some(pos) = line.find(OBFUSCATE_TOKEN) {
                    let n = rng.next_int_bound(4) as usize + 3;
                    line = format!(
                        "{}§f§k{}{}",
                        &line[..pos],
                        &"XXXXXXXX"[..n],
                        &line[pos + OBFUSCATE_TOKEN.len()..]
                    );
                }
                src.push(Src::Poem(line));
                src.push(Src::Empty);
            }
            for _ in 0..8 {
                src.push(Src::Empty);
            }
        };
        if poem {
            if let Some(t) = read("assets/minecraft/texts/end.txt") {
                add_poem(&mut src, &t);
            }
        }
        if let Some(t) = read("assets/minecraft/texts/credits.json") {
            match serde_json::from_str::<serde_json::Value>(&t) {
                Ok(v) => add_credits(&mut src, &v),
                Err(e) => tracing::error!("Couldn't load credits from file credits.json: {e}"),
            }
        }
        if poem {
            if let Some(t) = read("assets/minecraft/texts/postcredits.txt") {
                add_poem(&mut src, &t);
            }
        }
        Self {
            poem,
            bg: None,
            src,
            lines: None,
            scroll: 0.0,
            unmodified_speed: if poem { 0.5 } else { 0.75 },
            speedup: false,
            ctrl: [false; 2],
            reverse: false,
            last_step: std::time::Instant::now(),
        }
    }

    /// Создаёт GPU-фон поэмы (end_sky, end_portal, credits_vignette). Без текстур — чёрный фон.
    pub fn init_background(
        &mut self,
        rm: &ResourceManager,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) {
        if !self.poem {
            return;
        }
        let img = |p: &str| {
            let i = rm
                .read(p)
                .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok());
            if i.is_none() {
                tracing::error!("WinScreen: нет текстуры {p}");
            }
            i.map(|i| (i.to_rgba8(), i.width, i.height))
        };
        let (Some(sky), Some(portal), Some(vig)) = (
            img("assets/minecraft/textures/environment/end_sky.png"),
            img("assets/minecraft/textures/entity/end_portal/end_portal.png"),
            img("assets/minecraft/textures/misc/credits_vignette.png"),
        ) else {
            return;
        };
        fn rgba((d, w, h): &(Vec<u8>, u32, u32)) -> Rgba<'_> {
            Rgba {
                data: d.as_slice(),
                width: *w,
                height: *h,
            }
        }
        self.bg = Some(CreditsBackground::new(
            device,
            queue,
            format,
            &rgba(&sky),
            &rgba(&portal),
            &rgba(&vig),
        ));
    }

    /// VANILLA: WinScreen#calculateScrollSpeed — 5 + 15 за каждый зажатый Ctrl.
    fn scroll_speed(&self) -> f32 {
        let dir = if self.reverse { -1.0 } else { 1.0 };
        if self.speedup {
            let mods = self.ctrl.iter().filter(|c| **c).count() as f32;
            self.unmodified_speed * (5.0 + mods * 15.0) * dir
        } else {
            self.unmodified_speed * dir
        }
    }

    /// VANILLA: extractRenderState (scroll += a·speed, a — тики с прошлого кадра) + tick
    /// (конец при scroll > totalScrollLength + 2·height + 24). `true` — пора respawn.
    pub fn update(&mut self, gh: i32) -> bool {
        let dt = self.last_step.elapsed().as_secs_f32().min(0.1);
        self.last_step = std::time::Instant::now();
        self.scroll = (self.scroll + self.scroll_speed() * dt * 20.0).max(0.0);
        let Some(lines) = &self.lines else {
            return false;
        };
        let total = (lines.len() * 12) as i32;
        self.scroll > (total + gh + gh + 24) as f32
    }

    /// VANILLA: WinScreen#extractRenderState. Оставляет текст строк в `t` (сброс — общий).
    pub fn draw(
        &mut self,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        g: &ezzmc_render::Gpu,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        (game_time, partial): (i64, f32),
    ) {
        let (dev, q) = (&g.device, &g.queue);
        if self.lines.is_none() {
            self.lines = Some(build_lines(t, &self.src));
        }
        let (gw, gh) = t.gui_size(w, h);
        let (gw, gh) = (gw as i32, gh as i32);
        // Накопленный под экраном текст — до фона.
        t.flush(dev, q, view, w, h);
        t.begin(w, h);
        // VANILLA: extractBackground (END_PORTAL на весь экран) + extractVignette.
        if let Some(bg) = self.bg.as_mut() {
            bg.draw(dev, q, view, game_time, partial);
        } else {
            t.fill(0.0, 0.0, gw as f32, gh as f32, 0xFF00_0000);
            t.flush(dev, q, view, w, h);
        }
        let logo_x = gw / 2 - 128;
        let logo_y = gh + 50;
        let y_offs = -self.scroll;
        // VANILLA: LogoRenderer#extractRenderState(graphics, width, 1.0, logoY), сдвиг pose на yOffs.
        p.begin(t.scale());
        p.blit(
            "logo",
            logo_x as f32,
            logo_y as f32 + y_offs,
            256.0,
            44.0,
            [0.0, 0.0, 1.0, 44.0 / 64.0],
            0xFFFF_FFFF,
        );
        p.blit(
            "edition",
            (gw / 2 - 64) as f32,
            (logo_y + 44 - 7) as f32 + y_offs,
            128.0,
            14.0,
            [0.0, 0.0, 1.0, 14.0 / 16.0],
            0xFFFF_FFFF,
        );
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        let Some(lines) = &self.lines else {
            return;
        };
        let mut extra = 0.0f32;
        let mut y_pos = logo_y + 100;
        for (i, (line, centered)) in lines.iter().enumerate() {
            // VANILLA: последняя строка останавливается по центру экрана.
            if i == lines.len() - 1 {
                let diff = y_pos as f32 + y_offs - (gh / 2 - 6) as f32;
                if diff < 0.0 {
                    extra = -diff;
                }
            }
            let y = y_pos as f32 + y_offs;
            if y + 12.0 + 8.0 > 0.0 && y < gh as f32 && !line.is_empty() {
                let x = if *centered {
                    // VANILLA: GuiGraphicsExtractor#centeredText — x - width/2 (целочисленно).
                    (logo_x + 128 - t.text_width(line) as i32 / 2) as f32
                } else {
                    logo_x as f32
                };
                t.draw(line, x, y + extra, 0xFF_FFFF, true);
            }
            y_pos += 12;
        }
    }
}

/// VANILLA: WinScreen#addCreditsFile.
fn add_credits(out: &mut Vec<Src>, root: &serde_json::Value) {
    let s = |v: &serde_json::Value, k: &str| {
        v.get(k)
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string()
    };
    let arr = |v: &serde_json::Value, k: &str| {
        v.get(k)
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default()
    };
    for sec in root.as_array().into_iter().flatten() {
        out.push(Src::Line(SECTION_HEADING.into(), true));
        out.push(Src::Line(format!("§e{}", s(sec, "section")), true));
        out.push(Src::Line(SECTION_HEADING.into(), true));
        out.push(Src::Empty);
        out.push(Src::Empty);
        for disc in arr(sec, "disciplines") {
            let name = s(&disc, "discipline");
            if !name.is_empty() {
                out.push(Src::Line(format!("§e{name}"), true));
                out.push(Src::Empty);
                out.push(Src::Empty);
            }
            for title in arr(&disc, "titles") {
                out.push(Src::Line(format!("§7{}", s(&title, "title")), false));
                for n in arr(&title, "names") {
                    let n = n.as_str().unwrap_or_default();
                    out.push(Src::Line(format!("§f{NAME_PREFIX}{n}"), false));
                }
                out.push(Src::Empty);
                out.push(Src::Empty);
            }
        }
    }
}

fn build_lines(t: &TextRenderer, src: &[Src]) -> Vec<(String, bool)> {
    let mut out = Vec::with_capacity(src.len());
    for s in src {
        match s {
            Src::Empty => out.push((String::new(), false)),
            Src::Line(l, c) => out.push((l.clone(), *c)),
            Src::Poem(l) => {
                for part in split_styled(t, l, POEM_WIDTH) {
                    out.push((part, false));
                }
            }
        }
    }
    out
}

/// VANILLA: StringSplitter#splitLines (LineBreakFinder): перенос по последнему пробелу
/// (пробел съедается), без пробела — перед символом, вылезшим за `max`; стиль переносится.
/// Пустая строка не даёт строк (parts пуст → getRemainder null); строка из одних §-кодов — одну.
fn split_styled(t: &TextRenderer, s: &str, max: f32) -> Vec<String> {
    // Символы с активным префиксом стиля (§-коды) на момент символа.
    let mut chars: Vec<(char, String)> = Vec::new();
    let mut style = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '§' {
            let Some(code) = it.next() else { break };
            let lc = code.to_ascii_lowercase();
            // VANILLA: цвет/§r сбрасывают стиль, k..o добавляются.
            if lc.is_ascii_hexdigit() || lc == 'r' {
                style.clear();
            }
            style.push('§');
            style.push(lc);
            continue;
        }
        chars.push((c, style.clone()));
    }
    let width = |c: char, st: &str| {
        let mut buf = String::with_capacity(st.len() + 4);
        buf.push_str(st);
        buf.push(c);
        t.text_width(&buf)
    };
    let mut out = Vec::new();
    if s.is_empty() {
        return out;
    }
    let mut start = 0usize;
    while start < chars.len() || out.is_empty() {
        let mut w = 0.0;
        let mut last_space: Option<usize> = None;
        let mut end = chars.len();
        let mut next = chars.len();
        for (i, (c, st)) in chars.iter().enumerate().skip(start) {
            if *c == ' ' {
                last_space = Some(i);
            }
            w += width(*c, st);
            if w > max {
                if let Some(sp) = last_space.filter(|sp| *sp > start) {
                    end = sp;
                    next = sp + 1;
                } else {
                    end = i.max(start + 1);
                    next = end;
                }
                break;
            }
        }
        let mut line = String::new();
        let mut cur: Option<&str> = None;
        for (c, st) in &chars[start..end] {
            if cur != Some(st.as_str()) {
                if !st.is_empty() || cur.is_some_and(|p| !p.is_empty()) {
                    line.push_str(if st.is_empty() { "§r" } else { st });
                }
                cur = Some(st);
            }
            line.push(*c);
        }
        out.push(line);
        start = next;
    }
    out
}
