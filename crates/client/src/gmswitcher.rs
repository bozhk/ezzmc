//! Переключатель режима игры (F3+F4). VANILLA: GameModeSwitcherScreen (26.3): 4 слота 26×26 с шагом 31,
//! подпись режима над ними, подсказка «F4 — следующий» под ними; F4 при зажатой F3 — следующий режим,
//! отпускание F3 применяет выбор (пакет change_game_mode, если режим сменился и есть права).
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const SPRITES: &[(&str, &str)] = &[
    ("gms_bg", "container/gamemode_switcher"),
    ("gms_slot", "sprites/gamemode_switcher/slot"),
    ("gms_sel", "sprites/gamemode_switcher/selection"),
];

/// Порядок GameModeIcon: creative, survival, adventure, spectator; (id режима, ключ lang, предмет).
const ICONS: [(i8, &str, &str); 4] = [
    (1, "gameMode.creative", "grass_block"),
    (0, "gameMode.survival", "iron_sword"),
    (2, "gameMode.adventure", "buried_treasure_map"),
    (3, "gameMode.spectator", "ender_eye"),
];
const ALL_W: i32 = 4 * 31 - 5;

pub struct GmSwitcher {
    previous: usize,
    hovered: usize,
    first_mouse: Option<(i32, i32)>,
}

pub fn icon_of(mode: i8) -> usize {
    ICONS.iter().position(|i| i.0 == mode).unwrap_or(0)
}

impl GmSwitcher {
    /// VANILLA: getDefaultSelected — прошлый режим, иначе creative↔survival.
    pub fn new(prev_mode: i8, cur_mode: i8) -> Self {
        let m = if prev_mode >= 0 {
            prev_mode
        } else if cur_mode == 1 {
            0
        } else {
            1
        };
        let i = icon_of(m);
        Self {
            previous: i,
            hovered: i,
            first_mouse: None,
        }
    }

    /// F4 при зажатой F3: следующий режим, мышь снова «не двигалась».
    pub fn next(&mut self) {
        self.first_mouse = None;
        self.hovered = (self.hovered + 1) % 4;
    }

    /// Режим, который нужно применить при закрытии.
    pub fn selected_mode(&self) -> i8 {
        ICONS[self.hovered].0
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        lang: &Lang,
        key_name: &str,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        g: &ezzmc_render::Gpu,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        rm: &ResourceManager,
        (mx, my): (f32, f32),
    ) {
        let (dev, q) = (&g.device, &g.queue);
        let _ = self.previous;
        let (gw, gh) = t.gui_size(w, h);
        let (gw, gh) = (gw as i32, gh as i32);
        let (mxi, myi) = (mx.floor() as i32, my.floor() as i32);
        // extractRenderState: первая позиция мыши запоминается; ховер слота меняет выбор только после сдвига.
        let first = *self.first_mouse.get_or_insert((mxi, myi));
        let moved = first != (mxi, myi);
        let x0 = gw / 2 - ALL_W / 2;
        let y0 = gh / 2 - 31;
        if moved {
            for i in 0..4 {
                let x = x0 + i as i32 * 31;
                if mxi >= x && mxi < x + 26 && myi >= y0 && myi < y0 + 26 {
                    self.hovered = i;
                }
            }
        }
        // Фон 125×75 из листа 128×128.
        p.begin(t.scale());
        p.blit(
            "gms_bg",
            (gw / 2 - 62) as f32,
            (gh / 2 - 31 - 27) as f32,
            125.0,
            75.0,
            [0.0, 0.0, 125.0 / 128.0, 75.0 / 128.0],
            0xFFFF_FFFF,
        );
        let full = [0.0, 0.0, 1.0, 1.0];
        for i in 0..4 {
            let x = (x0 + i as i32 * 31) as f32;
            p.blit("gms_slot", x, y0 as f32, 26.0, 26.0, full, 0xFFFF_FFFF);
            if i == self.hovered {
                p.blit("gms_sel", x, y0 as f32, 26.0, 26.0, full, 0xFFFF_FFFF);
            }
        }
        p.flush(dev, q, view, w, h, None);
        p.begin(t.scale());
        for (i, ic) in ICONS.iter().enumerate() {
            let Some(id) = ezzmc_protocol::generated::r26_3::ITEM
                .iter()
                .position(|n| *n == ic.2)
            else {
                continue;
            };
            let s = ItemStack {
                item: id as _,
                count: 1,
                patch: Default::default(),
            };
            crate::hud::stack_icon(
                p,
                dev,
                q,
                rm,
                &s,
                (x0 + i as i32 * 31 + 5) as f32,
                (y0 + 5) as f32,
            );
        }
        p.flush(dev, q, view, w, h, None);
        t.begin(w, h);
        let name = lang.tr(ICONS[self.hovered].1);
        let cx = gw as f32 / 2.0;
        t.draw(
            &name,
            cx - t.text_width(&name) / 2.0,
            (y0 - 20) as f32,
            0xFFFF_FFFF,
            true,
        );
        let hint = lang.fmt("debug.gamemodes.select_next", &[&format!("§b{key_name}§r")]);
        t.draw(
            &hint,
            cx - t.text_width(&hint) / 2.0,
            (gh / 2 + 5) as f32,
            0xFFFF_FFFF,
            true,
        );
        t.flush(dev, q, view, w, h);
        t.begin(w, h);
    }
}
