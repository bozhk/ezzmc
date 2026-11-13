//! Экран торговли с жителем/странствующим торговцем.
//! VANILLA: MerchantScreen (276×166, текстура villager.png 512×256), MerchantMenu, MerchantContainer,
//! MerchantOffer(s), ClientboundMerchantOffersPacket, ServerboundSelectTradePacket.
use crate::inventory::{Inventory, OpenMenu};
use crate::netthread::NetCmd;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_protocol::item::{self, ItemStack};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

pub const W: f32 = 276.0;
pub const H: f32 = 166.0;
/// VANILLA: MerchantScreen#NUMBER_OF_OFFER_BUTTONS.
const ROWS: usize = 7;
/// VANILLA: VillagerData#NEXT_LEVEL_XP_THRESHOLDS.
const XP_THRESHOLDS: [i32; 5] = [0, 10, 70, 150, 250];

/// VANILLA: MerchantOffer (поля потокового кодека).
#[derive(Clone, Debug, PartialEq)]
pub struct Offer {
    pub base_a: ItemStack,
    pub cost_b: Option<ItemStack>,
    pub result: ItemStack,
    pub out_of_stock: bool,
    pub xp: i32,
    pub special: i32,
    pub mult: f32,
    pub demand: i32,
}

fn max_stack(s: &ItemStack) -> i32 {
    ezzmc_protocol::generated::r26_3::ITEM
        .get(s.item as usize)
        .map_or(64, |n| crate::itemdata::max_stack(n))
}

/// VANILLA: ItemCost#test — тот же предмет и все компоненты предиката совпадают.
fn cost_test(cost: &ItemStack, s: &ItemStack) -> bool {
    cost.item == s.item
        && cost
            .patch
            .added
            .iter()
            .all(|c| s.patch.added.iter().any(|x| x == c))
}

impl Offer {
    /// VANILLA: MerchantOffer#getModifiedCostCount (спрос, множитель, скидка).
    fn modified(&self, base: &ItemStack) -> i32 {
        let bp = base.count;
        let demand = ((bp as f32 * self.demand as f32 * self.mult).floor() as i32).max(0);
        (bp + demand + self.special).clamp(1, max_stack(base).max(1))
    }

    /// VANILLA: MerchantOffer#getCostA.
    pub fn cost_a(&self) -> ItemStack {
        ItemStack {
            count: self.modified(&self.base_a),
            ..self.base_a.clone()
        }
    }

    /// VANILLA: MerchantOffer#satisfiedBy.
    fn satisfied_by(&self, a: Option<&ItemStack>, b: Option<&ItemStack>) -> bool {
        let Some(a) = a else { return false };
        if !cost_test(&self.base_a, a) || a.count < self.modified(&self.base_a) {
            return false;
        }
        match &self.cost_b {
            None => b.is_none(),
            Some(c) => b.is_some_and(|b| cost_test(c, b) && b.count >= c.count),
        }
    }
}

/// Состояние торговли открытого меню (пакет предложений + состояние экрана).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trades {
    pub offers: Vec<Offer>,
    pub level: i32,
    pub xp: i32,
    pub show_progress: bool,
    pub can_restock: bool,
    /// VANILLA: MerchantScreen#shopItem.
    pub shop_item: usize,
    /// VANILLA: MerchantScreen#scrollOff.
    pub scroll_off: usize,
    pub dragging: bool,
}

/// VANILLA: ClientboundMerchantOffersPacket (id контейнера, предложения, уровень, опыт, флаги).
pub fn parse(body: &[u8]) -> ProtocolResult<(i32, Trades)> {
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    let n = r.length("merchant offers", 256)?;
    let mut offers = Vec::with_capacity(n);
    for _ in 0..n {
        let base_a = item::read_item_cost(&mut r)?;
        let result = item::read_stack(&mut r)?;
        let cost_b = if r.bool()? {
            Some(item::read_item_cost(&mut r)?)
        } else {
            None
        };
        let exhausted = r.bool()?;
        let uses = r.i32()?;
        let max_uses = r.i32()?;
        let xp = r.i32()?;
        let special = r.i32()?;
        let mult = r.f32()?;
        let demand = r.i32()?;
        // VANILLA: ItemStack.STREAM_CODEC не принимает пустой результат.
        let Some(result) = result else { continue };
        offers.push(Offer {
            base_a,
            cost_b,
            result,
            out_of_stock: exhausted || uses >= max_uses,
            xp,
            special,
            mult,
            demand,
        });
    }
    let level = r.var_int()?;
    let xp = r.var_int()?;
    let show_progress = r.bool()?;
    let can_restock = r.bool()?;
    Ok((
        id,
        Trades {
            offers,
            level,
            xp,
            show_progress,
            can_restock,
            ..Trades::default()
        },
    ))
}

/// Позиции слотов. VANILLA: MerchantMenu (136,37) (162,37) (220,37), инвентарь (108,84).
pub fn slots() -> Vec<Option<(f32, f32)>> {
    let mut s = vec![
        Some((136.0, 37.0)),
        Some((162.0, 37.0)),
        Some((220.0, 37.0)),
    ];
    for i in 0..27 {
        s.push(Some((
            108.0 + (i % 9) as f32 * 18.0,
            84.0 + (i / 9) as f32 * 18.0,
        )));
    }
    for c in 0..9 {
        s.push(Some((108.0 + c as f32 * 18.0, 142.0)));
    }
    s
}

fn can_scroll(n: usize) -> bool {
    n > ROWS
}

fn in_rect(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && my >= y && mx < x + w && my < y + h
}

/// Кнопка-строка под курсором (индекс строки 0..7), если она видима.
fn row_at(tr: &Trades, l: f32, t: f32, mx: f32, my: f32) -> Option<usize> {
    (0..ROWS.min(tr.offers.len()))
        .find(|&i| in_rect(mx, my, l + 5.0, t + 18.0 + 20.0 * i as f32, 88.0, 20.0))
}

/// VANILLA: MerchantContainer#updateSellItem — активное предложение по платёжным слотам.
fn future_xp(o: &OpenMenu, tr: &Trades) -> i32 {
    let (s0, s1) = (
        o.slots.first().and_then(Option::as_ref),
        o.slots.get(1).and_then(Option::as_ref),
    );
    let (a, b) = if s0.is_none() { (s1, None) } else { (s0, s1) };
    if a.is_none() {
        return 0;
    }
    let find = |a: Option<&ItemStack>, b: Option<&ItemStack>| -> Option<&Offer> {
        let hint = tr.shop_item;
        if hint > 0 && hint < tr.offers.len() {
            let of = &tr.offers[hint];
            return of.satisfied_by(a, b).then_some(of);
        }
        tr.offers.iter().find(|of| of.satisfied_by(a, b))
    };
    let mut offer = find(a, b);
    if offer.is_none_or(|of| of.out_of_stock) {
        offer = find(b, a);
    }
    offer.filter(|of| !of.out_of_stock).map_or(0, |of| of.xp)
}

#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    o: &OpenMenu,
    l: f32,
    t: f32,
    mx: f32,
    my: f32,
) {
    let full = [0.0, 0.0, 1.0, 1.0];
    let Some(tr) = o.trades.as_ref().filter(|tr| !tr.offers.is_empty()) else {
        return;
    };
    let n = tr.offers.len();
    if let Some(sel) = tr.offers.get(tr.shop_item) {
        if sel.out_of_stock {
            p.blit(
                "villager_oos",
                l + 182.0,
                t + 35.0,
                28.0,
                21.0,
                full,
                0xFFFF_FFFF,
            );
        }
    }
    // VANILLA: MerchantScreen#extractScroller.
    let steps = n as i32 + 1 - ROWS as i32;
    if steps > 1 {
        let left_over = 139 - (27 + (steps - 1) * 139 / steps);
        let step_h = 1 + left_over / steps + 139 / steps;
        let mut off = (tr.scroll_off as i32 * step_h).min(113);
        if tr.scroll_off as i32 == steps - 1 {
            off = 113;
        }
        p.blit(
            "villager_scroller",
            l + 94.0,
            t + 18.0 + off as f32,
            6.0,
            27.0,
            full,
            0xFFFF_FFFF,
        );
    } else {
        p.blit(
            "villager_scroller_off",
            l + 94.0,
            t + 18.0,
            6.0,
            27.0,
            full,
            0xFFFF_FFFF,
        );
    }
    // Строки-кнопки (Button.Plain: button / button_highlighted, nine_slice 200×20, граница 3).
    for i in 0..ROWS.min(n) {
        let (x, y) = (l + 5.0, t + 18.0 + 20.0 * i as f32);
        let hover = in_rect(mx, my, x, y, 88.0, 20.0);
        let sprite = if hover { "button_hl" } else { "button" };
        p.nine_slice(
            sprite,
            x,
            y,
            88.0,
            20.0,
            [200.0, 20.0],
            [3.0; 4],
            0xFFFF_FFFF,
        );
    }
    for (row, offer) in tr
        .offers
        .iter()
        .skip(if can_scroll(n) { tr.scroll_off } else { 0 })
        .take(ROWS)
        .enumerate()
    {
        let y = t + 19.0 + 20.0 * row as f32;
        let cost_a = offer.cost_a();
        crate::hud::stack_icon(p, dev, q, rm, &cost_a, l + 10.0, y);
        if offer.base_a.count != cost_a.count {
            // VANILLA: extractAndDecorateCostA — зачёркнутая базовая цена.
            p.blit(
                "villager_discount",
                l + 17.0,
                y + 12.0,
                9.0,
                2.0,
                full,
                0xFFFF_FFFF,
            );
        }
        if let Some(b) = &offer.cost_b {
            crate::hud::stack_icon(p, dev, q, rm, b, l + 40.0, y);
        }
        let arrow = if offer.out_of_stock {
            "villager_arrow_oos"
        } else {
            "villager_arrow"
        };
        p.blit(arrow, l + 60.0, y + 3.0, 10.0, 9.0, full, 0xFFFF_FFFF);
        crate::hud::stack_icon(p, dev, q, rm, &offer.result, l + 73.0, y);
    }
    // VANILLA: MerchantScreen#extractProgressBar.
    if tr.show_progress && tr.level < 5 {
        let (x, y) = (l + 136.0, t + 16.0);
        p.blit("villager_xp_bg", x, y, 102.0, 5.0, full, 0xFFFF_FFFF);
        let lv = tr.level;
        if (1..5).contains(&lv) {
            let min = XP_THRESHOLDS[lv as usize - 1];
            if tr.xp >= min {
                let mult = 102.0 / (XP_THRESHOLDS[lv as usize] - min) as f32;
                let w = ((mult * (tr.xp - min) as f32).floor() as i32).min(102);
                if w > 0 {
                    p.blit(
                        "villager_xp_cur",
                        x,
                        y,
                        w as f32,
                        5.0,
                        [0.0, 0.0, w as f32 / 102.0, 1.0],
                        0xFFFF_FFFF,
                    );
                }
                let fx = future_xp(o, tr);
                if fx > 0 {
                    let fw = ((fx as f32 * mult).floor() as i32).min(102 - w);
                    if fw > 0 {
                        p.blit(
                            "villager_xp_res",
                            x + w as f32,
                            y,
                            fw as f32,
                            5.0,
                            [w as f32 / 102.0, 0.0, (w + fw) as f32 / 102.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
            }
        }
    }
}

/// VANILLA: GuiGraphicsExtractor#itemDecorations (количество; `force1` — показать «1» явно).
fn count_text(t: &mut TextRenderer, count: i32, force1: bool, x: f32, y: f32) {
    if count != 1 || force1 {
        let txt = count.to_string();
        t.draw(&txt, x + 17.0 - t.text_width(&txt), y + 9.0, 0xFFFFFF, true);
    }
}

/// Числа и подписи. VANILLA: MerchantScreen#extractLabels + decorations.
pub fn draw_text(
    t: &mut TextRenderer,
    lang: &crate::ui::lang::Lang,
    o: &OpenMenu,
    title: &str,
    l: f32,
    top: f32,
) {
    let tr = o.trades.as_ref();
    let level = tr.map_or(0, |tr| tr.level);
    let show = tr.is_some_and(|tr| tr.show_progress);
    let col = 0x404040;
    if (1..=5).contains(&level) && show {
        let s = lang.fmt(
            "merchant.title",
            &[title, &lang.tr(&format!("merchant.level.{level}"))],
        );
        let w = t.width_fmt(&s, false);
        t.draw(
            &s,
            l + 49.0 + (W / 2.0 - w / 2.0).floor(),
            top + 6.0,
            col,
            false,
        );
    } else {
        let w = t.width_fmt(title, false);
        t.draw(
            title,
            l + 49.0 + (W / 2.0 - w / 2.0).floor(),
            top + 6.0,
            col,
            false,
        );
    }
    t.draw(
        &lang.tr("container.inventory"),
        l + 107.0,
        top + H - 94.0,
        col,
        false,
    );
    let trades = lang.tr("merchant.trades");
    let w = t.width_fmt(&trades, false);
    t.draw(
        &trades,
        l + 5.0 - (w / 2.0).floor() + 48.0,
        top + 6.0,
        col,
        false,
    );

    let Some(tr) = tr.filter(|tr| !tr.offers.is_empty()) else {
        return;
    };
    let n = tr.offers.len();
    for (row, offer) in tr
        .offers
        .iter()
        .skip(if can_scroll(n) { tr.scroll_off } else { 0 })
        .take(ROWS)
        .enumerate()
    {
        let y = top + 19.0 + 20.0 * row as f32;
        let cost_a = offer.cost_a();
        if offer.base_a.count == cost_a.count {
            count_text(t, cost_a.count, false, l + 10.0, y);
        } else {
            count_text(t, offer.base_a.count, offer.base_a.count == 1, l + 10.0, y);
            count_text(t, cost_a.count, cost_a.count == 1, l + 24.0, y);
        }
        if let Some(b) = &offer.cost_b {
            count_text(t, b.count, false, l + 40.0, y);
        }
        count_text(t, offer.result.count, false, l + 73.0, y);
    }
}

/// Предмет под курсором в строке предложения. VANILLA: TradeOfferButton#extractToolTip.
pub fn hover_stack(o: &OpenMenu, l: f32, t: f32, mx: f32, my: f32) -> Option<ItemStack> {
    let tr = o.trades.as_ref()?;
    let row = row_at(tr, l, t, mx, my)?;
    let offer = tr.offers.get(row + tr.scroll_off)?;
    let dx = mx - (l + 5.0);
    if dx < 20.0 {
        Some(offer.cost_a())
    } else if dx < 50.0 && dx > 30.0 {
        offer.cost_b.clone()
    } else if dx > 65.0 {
        Some(offer.result.clone())
    } else {
        None
    }
}

/// Подсказка «устарело» над стрелкой результата (VANILLA: DEPRECATED_TOOLTIP, только при canRestock).
pub fn hover_text(
    lang: &crate::ui::lang::Lang,
    o: &OpenMenu,
    l: f32,
    t: f32,
    mx: f32,
    my: f32,
) -> Option<Vec<(String, u32)>> {
    let tr = o.trades.as_ref()?;
    let sel = tr.offers.get(tr.shop_item)?;
    (sel.out_of_stock && tr.can_restock && in_rect(mx, my, l + 186.0, t + 35.0, 22.0, 21.0))
        .then(|| vec![(lang.tr("merchant.deprecated"), 0xFFFF_FFFF)])
}

pub enum Act {
    Pass,
    Handled,
}

/// VANILLA: MerchantScreen#mouseClicked (захват скроллбара) + клик по строке → SelectTrade.
pub fn click(inv: &mut Inventory, out: &mut Vec<NetCmd>, l: f32, t: f32, mx: f32, my: f32) -> Act {
    let Some(o) = inv.open.as_mut() else {
        return Act::Pass;
    };
    let id = o.id;
    let Some(tr) = o.trades.as_mut() else {
        return Act::Pass;
    };
    if can_scroll(tr.offers.len())
        && mx > l + 94.0
        && mx < l + 100.0
        && my > t + 18.0
        && my <= t + 158.0
    {
        tr.dragging = true;
    }
    if let Some(row) = row_at(tr, l, t, mx, my) {
        tr.shop_item = row + tr.scroll_off;
        // VANILLA-UNSURE: MerchantMenu#tryMoveItems на клиенте не предсказываем — перенос платежа делает сервер.
        out.push(NetCmd::SelectTrade(tr.shop_item as i32));
        let _ = id;
        return Act::Handled;
    }
    Act::Pass
}

/// VANILLA: MerchantScreen#mouseDragged.
pub fn drag(inv: &mut Inventory, top: f32, my: f32) {
    let Some(tr) = inv.open.as_mut().and_then(|o| o.trades.as_mut()) else {
        return;
    };
    if !tr.dragging {
        return;
    }
    let max = tr.offers.len().saturating_sub(ROWS) as f32;
    let from = top + 18.0;
    let s = (my - from - 13.5) / (139.0 - 27.0);
    tr.scroll_off = ((s * max + 0.5) as i32).clamp(0, max as i32) as usize;
}

pub fn release(inv: &mut Inventory) {
    if let Some(tr) = inv.open.as_mut().and_then(|o| o.trades.as_mut()) {
        tr.dragging = false;
    }
}

/// VANILLA: MerchantScreen#mouseScrolled.
pub fn wheel(inv: &mut Inventory, dy: f32) {
    let Some(tr) = inv.open.as_mut().and_then(|o| o.trades.as_mut()) else {
        return;
    };
    if can_scroll(tr.offers.len()) {
        let max = (tr.offers.len() - ROWS) as i32;
        tr.scroll_off = (tr.scroll_off as i32 - dy as i32).clamp(0, max) as usize;
    }
}
