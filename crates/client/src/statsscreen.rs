//! Экран статистики. VANILLA: StatsScreen (+ GeneralStatisticsList, ItemStatisticsList,
//! MobsStatisticsList, MenuTabBar, HeaderAndFooterLayout, AbstractSelectionList) из 26.3.
//! Данные — `stats::Stats` (пакет award_stats).
use crate::stats::{self, Formatter, Stats};
use crate::ui::lang::Lang;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::generated::r26_3 as reg;
use ezzmc_protocol::item::ItemStack;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const SPRITES: &[(&str, &str)] = &[
    ("st_tab", "sprites/widget/tab"),
    ("st_tab_sel", "sprites/widget/tab_selected"),
    ("st_tab_hl", "sprites/widget/tab_highlighted"),
    ("st_tab_sel_hl", "sprites/widget/tab_selected_highlighted"),
    ("st_tab_hdr", "tab_header_background"),
    ("st_slot", "sprites/container/slot"),
    ("st_header", "sprites/statistics/header"),
    ("st_sort_up", "sprites/statistics/sort_up"),
    ("st_sort_down", "sprites/statistics/sort_down"),
    ("st_block_mined", "sprites/statistics/block_mined"),
    ("st_item_broken", "sprites/statistics/item_broken"),
    ("st_item_crafted", "sprites/statistics/item_crafted"),
    ("st_item_used", "sprites/statistics/item_used"),
    ("st_item_picked_up", "sprites/statistics/item_picked_up"),
    ("st_item_dropped", "sprites/statistics/item_dropped"),
    ("st_scroller", "sprites/widget/scroller"),
    ("st_scroller_bg", "sprites/widget/scroller_background"),
];

/// VANILLA: HeaderAndFooterLayout — шапка = высота панели вкладок (24), подвал 33.
const HEADER: i32 = 24;
const FOOTER: i32 = 33;
const ROW_W: i32 = 280;
/// VANILLA: ItemStatisticsList — колонки: [block mined] + [broken, crafted, used, picked_up, dropped].
const COLUMNS: [u32; 6] = [
    stats::MINED,
    stats::BROKEN,
    stats::CRAFTED,
    stats::USED,
    stats::PICKED_UP,
    stats::DROPPED,
];
const COLUMN_SPRITES: [&str; 6] = [
    "st_block_mined",
    "st_item_broken",
    "st_item_crafted",
    "st_item_used",
    "st_item_picked_up",
    "st_item_dropped",
];
const COLUMN_NAMES: [&str; 6] = ["mined", "broken", "crafted", "used", "picked_up", "dropped"];
const ROW_H: [i32; 3] = [14, 22, 36];

/// Результат клика.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Click {
    None,
    /// «Done» — вернуться на предыдущий экран.
    Close,
}

struct General {
    /// id в реестре custom_stat.
    id: u32,
    name: String,
    fmt: Formatter,
}

#[derive(Default)]
pub struct StatsScreen {
    pub tab: usize,
    scroll: [f32; 3],
    /// VANILLA: ItemStatisticsList#sortColumn/sortOrder (индекс колонки, -1 вниз / 1 вверх).
    pub sort_col: Option<usize>,
    pub sort_order: i32,
    built: bool,
    general: Vec<General>,
    /// id предметов в текущем порядке.
    items: Vec<u32>,
    /// id типов существ.
    mobs: Vec<u32>,
    /// Перетаскивание ползунка: последняя y курсора.
    drag: Option<f32>,
}

fn round_toward(v: i32, f: i32) -> i32 {
    (v + f - 1).div_euclid(f) * f
}

fn px(r: (i32, i32, i32, i32), k: f32) -> [u32; 4] {
    let f = |v: i32| (v.max(0) as f32 * k) as u32;
    [f(r.0), f(r.1), f(r.2 - r.0), f(r.3 - r.1)]
}

/// VANILLA: MenuTabBar#arrangeElements — (x, ширина) каждой из 3 вкладок.
fn tab_rects(w: i32) -> [(i32, i32); 3] {
    let tabs_w = w.min(400) - 28;
    let tab_w = round_toward(tabs_w / 3, 2);
    let x0 = round_toward((w - tabs_w) / 2, 2);
    [(x0, tab_w), (x0 + tab_w, tab_w), (x0 + 2 * tab_w, tab_w)]
}

impl StatsScreen {
    /// VANILLA: StatsScreen#onStatsUpdated — списки строятся один раз, когда пришёл ответ.
    fn build(&mut self, st: &Stats, lang: &Lang) {
        self.built = true;
        // GeneralStatisticsList: все пользовательские статистики, по названию.
        self.general = reg::CUSTOM_STAT
            .iter()
            .enumerate()
            .map(|(i, n)| General {
                id: i as u32,
                name: lang.tr(&format!("stat.minecraft.{n}")),
                fmt: stats::custom_formatter(n),
            })
            .collect();
        self.general.sort_by(|a, b| a.name.cmp(&b.name));
        // ItemStatisticsList: предметы с ненулевой статистикой + предметы добытых блоков.
        let mut items = Vec::new();
        for id in 0..reg::ITEM.len() as u32 {
            if reg::ITEM[id as usize] == "air" {
                continue;
            }
            let mut add = COLUMNS[1..].iter().any(|&t| st.get(t, id) > 0);
            if !add {
                add = stats::item_block(id).is_some_and(|b| st.get(stats::MINED, b) > 0);
            }
            if add {
                items.push(id);
            }
        }
        self.items = items;
        // MobsStatisticsList.
        self.mobs = (0..reg::ENTITY_TYPE.len() as u32)
            .filter(|&e| st.get(stats::KILLED, e) > 0 || st.get(stats::KILLED_BY, e) > 0)
            .collect();
    }

    fn row_count(&self, tab: usize) -> usize {
        match tab {
            0 => self.general.len(),
            // + HeaderEntry
            1 if !self.items.is_empty() => self.items.len() + 1,
            1 => 0,
            _ => self.mobs.len(),
        }
    }

    /// VANILLA: AbstractScrollArea#maxScrollAmount.
    fn max_scroll(&self, tab: usize, list_h: i32) -> f32 {
        let content = self.row_count(tab) as i32 * ROW_H[tab] + 4;
        (content - list_h).max(0) as f32
    }

    fn tab_active(&self, tab: usize, st: &Stats) -> bool {
        if !st.received {
            return tab == 0;
        }
        match tab {
            0 => true,
            1 => !self.items.is_empty(),
            _ => !self.mobs.is_empty(),
        }
    }

    /// VANILLA: ItemStatisticsList#sortByColumn — вниз → вверх → сброс.
    fn sort_by_column(&mut self, col: usize, st: &Stats) {
        if self.sort_col != Some(col) {
            self.sort_col = Some(col);
            self.sort_order = -1;
        } else if self.sort_order == -1 {
            self.sort_order = 1;
        } else {
            self.sort_col = None;
            self.sort_order = 0;
        }
        self.sort_items(st);
    }

    /// VANILLA: ItemRowComparator.
    fn sort_items(&mut self, st: &Stats) {
        let (col, order) = (self.sort_col, self.sort_order);
        self.items.sort_by(|&a, &b| {
            let key = |item: u32| -> i32 {
                match col {
                    None => 0,
                    Some(0) => stats::item_block(item).map_or(-1, |bl| st.get(stats::MINED, bl)),
                    Some(c) => st.get(COLUMNS[c], item),
                }
            };
            let (k1, k2) = (key(a), key(b));
            let o = if k1 == k2 { a.cmp(&b) } else { k1.cmp(&k2) };
            if order < 0 {
                o.reverse()
            } else if order > 0 {
                o
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    fn list_h(h: i32) -> i32 {
        h - HEADER - FOOTER
    }

    /// Клик левой кнопкой (GUI-пиксели). VANILLA: вкладки, «Done», кнопки сортировки, ползунок.
    pub fn click(&mut self, st: &Stats, (gw, gh): (f32, f32), (mx, my): (f32, f32)) -> Click {
        let (w, h) = (gw as i32, gh as i32);
        let (mxi, myi) = (mx.floor() as i32, my.floor() as i32);
        // «Done»: 200×20 по центру подвала.
        if mxi >= w / 2 - 100 && mxi < w / 2 + 100 && myi >= h - 27 && myi < h - 7 {
            crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
            return Click::Close;
        }
        for (i, (x, tw)) in tab_rects(w).iter().enumerate() {
            if mxi >= *x && mxi < x + tw && (0..HEADER).contains(&myi) && self.tab_active(i, st) {
                if self.tab != i {
                    self.tab = i;
                    crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
                }
                return Click::None;
            }
        }
        if !st.received || !self.built {
            return Click::None;
        }
        let list_h = Self::list_h(h);
        let max = self.max_scroll(self.tab, list_h);
        // Ползунок (AbstractSelectionList#scrollBarX = rowRight + 6 + 2).
        let sbx = w / 2 + ROW_W / 2 + 8;
        if max > 0.0 && mxi >= sbx && mxi <= sbx + 6 && myi >= HEADER && myi < HEADER + list_h {
            self.drag = Some(my);
            return Click::None;
        }
        // Кнопки сортировки в заголовке списка предметов.
        if self.tab == 1 && !self.items.is_empty() {
            let (cx, cy) = (
                w / 2 - ROW_W / 2 + 2,
                HEADER + 2 - self.scroll[1] as i32 + 2,
            );
            if myi >= HEADER && myi < HEADER + list_h {
                for col in 0..6 {
                    let bx = cx + 75 + 40 * col as i32 - 18;
                    if mxi >= bx && mxi < bx + 18 && myi > cy && myi < cy + 19 {
                        crate::audio::play_ui(
                            "ui.button.click",
                            crate::audio::Cat::Master,
                            0.25,
                            1.0,
                        );
                        self.sort_by_column(col, st);
                        break;
                    }
                }
            }
        }
        Click::None
    }

    /// Колесо: VANILLA AbstractScrollArea#mouseScrolled — `scroll −= dy · rate` (rate = высота строки / 2).
    pub fn wheel(&mut self, h: f32, dy: f32) {
        let list_h = Self::list_h(h as i32);
        let t = self.tab;
        let max = self.max_scroll(t, list_h);
        let rate = (ROW_H[t] / 2) as f32;
        self.scroll[t] = (self.scroll[t] - dy * rate).clamp(0.0, max);
    }

    /// Состояние для агента: (вкладка, строк, прокрутка, сортировка).
    pub fn snapshot(&self, st: &Stats) -> (usize, usize, f32, Option<usize>, i32, bool) {
        (
            self.tab,
            self.row_count(self.tab),
            self.scroll[self.tab],
            self.sort_col,
            self.sort_order,
            st.received,
        )
    }

    /// Выбор вкладки и сортировки из агента (то же, что клик).
    pub fn select_tab(&mut self, st: &Stats, tab: usize) -> bool {
        if tab < 3 && self.tab_active(tab, st) {
            self.tab = tab;
            true
        } else {
            false
        }
    }

    pub fn sort(&mut self, st: &Stats, col: usize) {
        if col < 6 && self.built {
            self.sort_by_column(col, st);
        }
    }

    /// VANILLA: StatsScreen#extractRenderState.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        st: &Stats,
        lang: &Lang,
        t: &mut TextRenderer,
        p: &mut GuiPainter,
        g: &ezzmc_render::Gpu,
        view: &wgpu::TextureView,
        (w, h): (u32, u32),
        rm: &ResourceManager,
        (mx, my): (f32, f32),
        down: bool,
        now_ms: u64,
    ) {
        let (dev, q) = (&g.device, &g.queue);
        if st.received && !self.built {
            self.build(st, lang);
        }
        let k = t.scale() as f32;
        let (gw, gh) = t.gui_size(w, h);
        let (gwi, ghi) = (gw as i32, gh as i32);
        let (mxi, myi) = (mx.floor() as i32, my.floor() as i32);
        let list_h = Self::list_h(ghi);
        let list_rect = (0, HEADER, gwi, HEADER + list_h);
        let tab = self.tab;
        let max = self.max_scroll(tab, list_h);
        // Перетаскивание ползунка (AbstractScrollArea#mouseDragged).
        if let Some(last) = self.drag {
            if down && max > 0.0 {
                let sh = scroller_h(list_h, self.row_count(tab) as i32 * ROW_H[tab] + 4);
                let denom = (list_h - sh).max(1) as f32;
                self.scroll[tab] = (self.scroll[tab] + (my - last) * max / denom).clamp(0.0, max);
                self.drag = Some(my);
            } else {
                self.drag = None;
            }
        }
        self.scroll[tab] = self.scroll[tab].clamp(0.0, max);
        let scroll = self.scroll[tab] as i32;
        let white = [0.0, 0.0, 1.0, 1.0];
        let full = 0xFFFF_FFFF;

        // ---------- Проход 1: фон, панель вкладок, подвал, ползунок ----------
        p.begin(t.scale());
        t.begin(w, h);
        p.tile("st_tab_hdr", 0.0, 0.0, gw as f32, HEADER as f32, 16.0, full);
        p.tile(
            "iw_menu_bg",
            0.0,
            HEADER as f32,
            gw as f32,
            (ghi - HEADER) as f32,
            32.0,
            full,
        );
        let sep = |p: &mut GuiPainter, tex: &str, x0: i32, x1: i32, y: i32| {
            let mut cx = x0;
            while cx < x1 {
                let cw = 32.min(x1 - cx);
                p.blit(
                    tex,
                    cx as f32,
                    y as f32,
                    cw as f32,
                    2.0,
                    [0.0, 0.0, cw as f32 / 32.0, 1.0],
                    full,
                );
                cx += 32;
            }
        };
        let rects = tab_rects(gwi);
        // MenuTabBar: разделитель слева/справа от вкладок.
        sep(p, "iw_header_sep", 0, rects[0].0, HEADER - 2);
        sep(p, "iw_header_sep", rects[2].0 + rects[2].1, gwi, HEADER - 2);
        for (i, (x, tw)) in rects.iter().enumerate() {
            let hov = mxi >= *x && mxi < x + tw && (0..HEADER).contains(&myi);
            let hl = hov && self.tab_active(i, st);
            let name = match (i == tab, hl) {
                (true, true) => "st_tab_sel_hl",
                (true, false) => "st_tab_sel",
                (false, true) => "st_tab_hl",
                (false, false) => "st_tab",
            };
            p.nine_slice(
                name,
                *x as f32,
                0.0,
                *tw as f32,
                HEADER as f32,
                [130.0, 24.0],
                [2.0, 2.0, 2.0, 0.0],
                full,
            );
            if i == tab {
                p.tile(
                    "iw_menu_bg",
                    (*x + 2) as f32,
                    2.0,
                    (*tw - 4) as f32,
                    (HEADER - 2) as f32,
                    32.0,
                    full,
                );
            }
        }
        sep(p, "iw_footer_sep", 0, gwi, ghi - FOOTER);
        // «Done».
        let done = (gwi / 2 - 100, ghi - 27);
        let done_hov = mxi >= done.0 && mxi < done.0 + 200 && myi >= done.1 && myi < done.1 + 20;
        p.nine_slice(
            if done_hov { "button_hl" } else { "button" },
            done.0 as f32,
            done.1 as f32,
            200.0,
            20.0,
            [200.0, 20.0],
            [3.0; 4],
            full,
        );
        // Ползунок.
        let list_content = self.row_count(tab) as i32 * ROW_H[tab] + 4;
        if st.received && max > 0.0 {
            let sbx = gwi / 2 + ROW_W / 2 + 8;
            p.nine_slice(
                "st_scroller_bg",
                sbx as f32,
                HEADER as f32,
                6.0,
                list_h as f32,
                [6.0, 32.0],
                [1.0; 4],
                full,
            );
            let sh = scroller_h(list_h, list_content);
            let sy = (HEADER as f32)
                .max(((self.scroll[tab] as i32) * (list_h - sh) / max as i32 + HEADER) as f32);
            p.nine_slice(
                "st_scroller",
                sbx as f32,
                sy,
                6.0,
                sh as f32,
                [6.0, 32.0],
                [1.0; 4],
                full,
            );
        }
        // Текст панели вкладок и подвала.
        for (i, (x, tw)) in rects.iter().enumerate() {
            let key = ["stat.generalButton", "stat.itemsButton", "stat.mobsButton"][i];
            let label = lang.tr(key);
            let active = self.tab_active(i, st);
            let col = if active { 0xFFFFFF } else { 0xA0A0A0 };
            let lw = t.text_width(&label);
            let ty = if i == tab { 7.0 } else { 9.0 };
            t.draw(&label, *x as f32 + (*tw as f32 - lw) / 2.0, ty, col, true);
            if i == tab {
                let uw = lw.min((*tw - 4) as f32);
                let ux = *x as f32 + (*tw as f32 - uw) / 2.0;
                t.fill(
                    ux,
                    (HEADER - 2) as f32,
                    ux + uw,
                    (HEADER - 1) as f32,
                    0xFF00_0000 | col,
                );
            }
        }
        let dl = lang.tr("gui.done");
        let dw = t.text_width(&dl);
        t.draw(
            &dl,
            done.0 as f32 + (200.0 - dw) / 2.0,
            done.1 as f32 + 6.0,
            0xFFFFFF,
            true,
        );
        if !st.received {
            // VANILLA: LoadingTab/LoadingDotsWidget — по центру области, высота блока 28 + 30.
            let msg = lang.tr("multiplayer.downloadingStats");
            let wy = HEADER + (list_h - 58) / 2;
            let mw = t.text_width(&msg);
            t.draw(
                &msg,
                (gwi as f32 - mw) / 2.0,
                (wy + 2) as f32,
                0xFFFFFF,
                true,
            );
            const FRAMES: [&str; 4] = ["O o o", "o O o", "o o O", "o O o"];
            let dots = FRAMES[(now_ms / 300 % 4) as usize];
            let dw = t.text_width(dots);
            t.draw(
                dots,
                (gwi as f32 - dw) / 2.0,
                (wy + 17) as f32,
                0x80_8080,
                true,
            );
        }
        p.flush(dev, q, view, w, h, None);
        t.flush(dev, q, view, w, h);
        if !st.received || !self.built {
            return;
        }

        // ---------- Список ----------
        let row_left = gwi / 2 - ROW_W / 2;
        let first_y = HEADER + 2 - scroll;
        let in_list = myi >= HEADER && myi < HEADER + list_h;
        let mut tip: Option<Vec<(String, u32)>> = None;
        let mut tip_xy = (mx, my);
        let shade = |index: usize| -> u32 {
            if index % 2 == 0 {
                0xFFFFFF
            } else {
                0xBABABA
            }
        };
        // (x, y, текст, цвет, выравнивание вправо)
        let mut texts: Vec<(i32, i32, String, u32, bool)> = Vec::new();
        let mut icons: Vec<(ItemStack, f32, f32)> = Vec::new();
        p.begin(t.scale());
        match tab {
            0 => {
                for (i, r) in self.general.iter().enumerate() {
                    let y = first_y + i as i32 * ROW_H[0];
                    if y + ROW_H[0] < HEADER || y > HEADER + list_h {
                        continue;
                    }
                    let ty = y + 2 + 5 - 4;
                    let col = shade(i);
                    texts.push((row_left + 4, ty, r.name.clone(), col, false));
                    let val = r.fmt.format(st.get(stats::CUSTOM, r.id));
                    // contentRight − width − 4, contentRight = x + 2 + 276.
                    texts.push((row_left + 278 - 4, ty, val, col, true));
                }
            }
            1 => {
                let cx = row_left + 2;
                // HeaderEntry (индекс 0).
                let cy = first_y + 2;
                for col in 0..6usize {
                    let bx = cx + 75 + 40 * col as i32 - 18;
                    let hov = in_list && mxi >= bx && mxi < bx + 18 && myi > cy && myi < cy + 19;
                    p.blit(
                        if hov { "st_slot" } else { "st_header" },
                        bx as f32,
                        (cy + 1) as f32,
                        18.0,
                        18.0,
                        white,
                        full,
                    );
                    p.blit(
                        COLUMN_SPRITES[col],
                        bx as f32,
                        (cy + 1) as f32,
                        18.0,
                        18.0,
                        white,
                        full,
                    );
                    if hov {
                        let name = lang.tr(&format!("stat_type.minecraft.{}", COLUMN_NAMES[col]));
                        tip = Some(vec![(name, 0xFFFFFF)]);
                        tip_xy = (mx, my);
                    }
                }
                if let Some(c) = self.sort_col {
                    let off = 75 + 40 * c as i32 - 36;
                    p.blit(
                        if self.sort_order == 1 {
                            "st_sort_up"
                        } else {
                            "st_sort_down"
                        },
                        (cx + off) as f32,
                        (cy + 1) as f32,
                        18.0,
                        18.0,
                        white,
                        full,
                    );
                }
                for (n, &item) in self.items.iter().enumerate() {
                    let index = n + 1;
                    let y = first_y + index as i32 * ROW_H[1];
                    if y + ROW_H[1] < HEADER || y > HEADER + list_h {
                        continue;
                    }
                    let cy = y + 2;
                    p.blit("st_slot", cx as f32, cy as f32, 18.0, 18.0, white, full);
                    let stack = ItemStack {
                        item,
                        count: 1,
                        patch: Default::default(),
                    };
                    if in_list && mxi >= cx && mxi < cx + 18 && myi >= cy && myi < cy + 18 {
                        tip = Some(crate::tooltip::stack_lines(lang, t, &stack));
                        tip_xy = ((cx + 18) as f32, (cy + 18) as f32);
                    }
                    icons.push((stack, (cx + 1) as f32, (cy + 1) as f32));
                    let ty = cy + (ROW_H[1] - 4) / 2 - 4;
                    let col = shade(index);
                    for c in 0..6usize {
                        let val = if c == 0 {
                            stats::item_block(item)
                                .map(|b| Formatter::Default.format(st.get(stats::MINED, b)))
                        } else {
                            Some(Formatter::Default.format(st.get(COLUMNS[c], item)))
                        };
                        let s = val.unwrap_or_else(|| lang.tr("stats.none"));
                        texts.push((cx + 75 + 40 * c as i32, ty, s, col, true));
                    }
                }
            }
            _ => {
                for (i, &e) in self.mobs.iter().enumerate() {
                    let y = first_y + i as i32 * ROW_H[2];
                    if y + ROW_H[2] < HEADER || y > HEADER + list_h {
                        continue;
                    }
                    let (cx, cy) = (row_left + 2, y + 2);
                    let name = lang.tr(&format!(
                        "entity.minecraft.{}",
                        reg::ENTITY_TYPE[e as usize]
                    ));
                    let kills = st.get(stats::KILLED, e);
                    let (kl, kc) = if kills == 0 {
                        (
                            lang.fmt("stat_type.minecraft.killed.none", &[&name]),
                            0x808080,
                        )
                    } else {
                        (
                            lang.fmt("stat_type.minecraft.killed", &[&kills.to_string(), &name]),
                            0xBABABA,
                        )
                    };
                    let by = st.get(stats::KILLED_BY, e);
                    let (bl, bc) = if by == 0 {
                        (
                            lang.fmt("stat_type.minecraft.killed_by.none", &[&name]),
                            0x808080,
                        )
                    } else {
                        (
                            lang.fmt("stat_type.minecraft.killed_by", &[&name, &by.to_string()]),
                            0xBABABA,
                        )
                    };
                    texts.push((cx + 2, cy + 1, name, 0xFFFFFF, false));
                    texts.push((cx + 12, cy + 10, kl, kc, false));
                    texts.push((cx + 12, cy + 19, bl, bc, false));
                }
            }
        }
        p.set_scissor(Some(px(list_rect, k)));
        p.flush(dev, q, view, w, h, None);
        p.set_scissor(None);
        if !icons.is_empty() {
            p.begin(t.scale());
            for (s, x, y) in &icons {
                crate::hud::stack_icon(p, dev, q, rm, s, *x, *y);
            }
            p.set_scissor(Some(px(list_rect, k)));
            p.flush(dev, q, view, w, h, None);
            p.set_scissor(None);
        }
        t.begin(w, h);
        for (x, y, s, col, right) in &texts {
            let x = if *right {
                *x as f32 - t.text_width(s)
            } else {
                *x as f32
            };
            t.draw(s, x, *y as f32, *col, true);
        }
        t.set_scissor(Some(px(list_rect, k)));
        t.flush(dev, q, view, w, h);
        t.set_scissor(None);
        // Очистить буфер: вызывающий код может flush'ить текст ещё раз (без обрезки).
        t.begin(w, h);
        // Подсказка (setTooltipForNextFrame) — поверх всего.
        if let Some(lines) = tip {
            t.begin(w, h);
            crate::tooltip::draw_colored(t, &lines, tip_xy.0, tip_xy.1, gw as f32, gh as f32);
            t.flush(dev, q, view, w, h);
        }
    }
}

/// VANILLA: AbstractScrollArea#scrollerHeight.
fn scroller_h(h: i32, content: i32) -> i32 {
    ((h * h) / content.max(1)).clamp(32, (h - 8).max(32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_layout() {
        // ширина 854/2=427: tabs_w=400-28=372, tab_w=roundToward(124,2)=124, x0=roundToward(27,2)=28.
        let r = tab_rects(427);
        assert_eq!(r[0], (28, 124));
        assert_eq!(r[2], (276, 124));
        assert_eq!(round_toward(5, 2), 6);
        assert_eq!(round_toward(6, 2), 6);
    }

    #[test]
    fn sort_cycle() {
        let mut s = StatsScreen::default();
        let st = Stats::default();
        s.sort_by_column(2, &st);
        assert_eq!((s.sort_col, s.sort_order), (Some(2), -1));
        s.sort_by_column(2, &st);
        assert_eq!((s.sort_col, s.sort_order), (Some(2), 1));
        s.sort_by_column(2, &st);
        assert_eq!((s.sort_col, s.sort_order), (None, 0));
        s.sort_by_column(1, &st);
        s.sort_by_column(3, &st);
        assert_eq!((s.sort_col, s.sort_order), (Some(3), -1));
    }

    #[test]
    fn scroller() {
        assert_eq!(scroller_h(100, 100), 92);
        assert_eq!(scroller_h(100, 10_000), 32);
    }
}
