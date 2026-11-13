//! Экран инвентаря/контейнера: раскладка слотов, отрисовка, клики.
//! VANILLA: InventoryScreen / ContainerScreen (generic_9xN) / AbstractContainerScreen#mouseClicked.
//! Клики идут через `WorldView::click_slot`: предсказание (`Inventory::predict_click`) + заявка слотов.
use crate::hud;
use crate::inventory::Inventory;
use crate::netthread::NetCmd;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

/// VANILLA: ContainerInput.
pub const PICKUP: i32 = 0;
pub const QUICK_MOVE: i32 = 1;
pub const SWAP: i32 = 2;
pub const QUICK_CRAFT: i32 = 5;
pub const PICKUP_ALL: i32 = 6;
pub const THROW: i32 = 4;
/// Слот «вне окна» (бросок курсора).
pub const OUTSIDE: i16 = -999;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Player,
    Chest(usize),
    /// Меню с отдельной фоновой текстурой 176×h (ключ спрайта в `hud::SPRITES`).
    Tex(&'static str, f32),
    /// Кафедра: экран книги на весь GUI (BookViewScreen), без инвентаря.
    Lectern,
    Other,
}

pub struct Layout {
    kind: Kind,
    pub w: f32,
    pub h: f32,
    /// Позиция слота (None — слот не показывается).
    pub slots: Vec<Option<(f32, f32)>>,
}

const H_MERCHANT: f32 = crate::merchantscreen::H;

fn player_inv_rows(out: &mut Vec<Option<(f32, f32)>>, main_y: f32, hot_y: f32) {
    player_inv_rows_x(out, 8.0, main_y, hot_y);
}

fn player_inv_rows_x(out: &mut Vec<Option<(f32, f32)>>, x: f32, main_y: f32, hot_y: f32) {
    for i in 0..27 {
        out.push(Some((
            x + (i % 9) as f32 * 18.0,
            main_y + (i / 9) as f32 * 18.0,
        )));
    }
    for c in 0..9 {
        out.push(Some((x + c as f32 * 18.0, hot_y)));
    }
}

/// Раскладка для текущего состояния (открытое меню или InventoryMenu).
pub fn layout(inv: &Inventory) -> Layout {
    let Some(o) = &inv.open else {
        // VANILLA: InventoryMenu.
        let mut s = vec![Some((154.0, 28.0))];
        for i in 0..4 {
            s.push(Some((
                98.0 + (i % 2) as f32 * 18.0,
                18.0 + (i / 2) as f32 * 18.0,
            )));
        }
        for i in 0..4 {
            s.push(Some((8.0, 8.0 + i as f32 * 18.0)));
        }
        player_inv_rows(&mut s, 84.0, 142.0);
        s.push(Some((77.0, 62.0)));
        return Layout {
            kind: Kind::Player,
            w: 176.0,
            h: 166.0,
            slots: s,
        };
    };
    let name = ezzmc_protocol::generated::r26_3::MENU
        .get(o.menu as usize)
        .copied()
        .unwrap_or("");
    let rows = match name {
        "generic_9x1" => 1,
        "generic_9x2" => 2,
        "generic_9x3" => 3,
        "generic_9x4" => 4,
        "generic_9x5" => 5,
        "generic_9x6" => 6,
        _ => 0,
    };
    if name == "lectern" {
        // VANILLA: LecternScreen — слот 0 = книга (не показывается), инвентарь не рисуется.
        return Layout {
            kind: Kind::Lectern,
            w: 192.0,
            h: 192.0,
            slots: vec![None; o.slots.len()],
        };
    }
    let n = o.slots.len().max(36);
    if rows > 0 && n == rows * 9 + 36 {
        // VANILLA: ContainerScreen / ChestMenu.
        let mut s = Vec::new();
        for i in 0..rows * 9 {
            s.push(Some((
                8.0 + (i % 9) as f32 * 18.0,
                18.0 + (i / 9) as f32 * 18.0,
            )));
        }
        let off = (rows as f32 - 4.0) * 18.0;
        player_inv_rows(&mut s, 103.0 + off, 161.0 + off);
        return Layout {
            kind: Kind::Chest(rows),
            w: 176.0,
            h: 114.0 + rows as f32 * 18.0,
            slots: s,
        };
    }
    // VANILLA: CraftingScreen / FurnaceScreen / DispenserScreen / HopperScreen / ShulkerBoxScreen.
    let grid = |s: &mut Vec<Option<(f32, f32)>>, x: f32, y: f32, cols: usize, n: usize| {
        for i in 0..n {
            s.push(Some((
                x + (i % cols) as f32 * 18.0,
                y + (i / cols) as f32 * 18.0,
            )));
        }
    };
    if name == "merchant" && n == 39 {
        // VANILLA: MerchantMenu / MerchantScreen 276×166, villager.png 512×256.
        return Layout {
            kind: Kind::Tex("container_villager", H_MERCHANT),
            w: crate::merchantscreen::W,
            h: H_MERCHANT,
            slots: crate::merchantscreen::slots(),
        };
    }
    if name == "beacon" && n == 37 {
        // VANILLA: BeaconMenu — слот оплаты (136,110), инвентарь addStandardInventorySlots(36,137); BeaconScreen 230×219.
        let mut s = vec![Some((136.0, 110.0))];
        player_inv_rows_x(&mut s, 36.0, 137.0, 195.0);
        return Layout {
            kind: Kind::Tex("container_beacon", 219.0),
            w: 230.0,
            h: 219.0,
            slots: s,
        };
    }
    let simple = match name {
        "crafting" => Some(("container_crafting", 166.0, 46)),
        "furnace" => Some(("container_furnace", 166.0, 39)),
        "blast_furnace" => Some(("container_blast_furnace", 166.0, 39)),
        "smoker" => Some(("container_smoker", 166.0, 39)),
        "generic_3x3" => Some(("container_dispenser", 166.0, 45)),
        "hopper" => Some(("container_hopper", 133.0, 41)),
        "shulker_box" => Some(("container_shulker", 166.0, 63)),
        "anvil" => Some(("container_anvil", 166.0, 39)),
        "grindstone" => Some(("container_grindstone", 166.0, 39)),
        "enchantment" => Some(("container_enchanting", 166.0, 38)),
        "brewing_stand" => Some(("container_brewing", 166.0, 41)),
        "smithing" => Some(("container_smithing", 166.0, 40)),
        "cartography_table" => Some(("container_cartography", 166.0, 39)),
        "loom" => Some(("container_loom", 166.0, 40)),
        "stonecutter" => Some(("container_stonecutter", 166.0, 38)),
        "crafter_3x3" => Some(("container_crafter", 166.0, 46)),
        _ => None,
    };
    if let Some((tex, h, total)) = simple.filter(|x| n == x.2) {
        let mut s = Vec::new();
        let (main_y, hot_y) = match name {
            "hopper" => (51.0, 109.0),
            _ => (84.0, 142.0),
        };
        match name {
            "crafting" => {
                s.push(Some((124.0, 35.0)));
                grid(&mut s, 30.0, 17.0, 3, 9);
            }
            "furnace" | "blast_furnace" | "smoker" => {
                s.extend([Some((56.0, 17.0)), Some((56.0, 53.0)), Some((116.0, 35.0))]);
            }
            "generic_3x3" => grid(&mut s, 62.0, 17.0, 3, 9),
            // VANILLA: AnvilMenu / GrindstoneMenu / EnchantmentMenu / BrewingStandMenu / SmithingMenu.
            "anvil" => s.extend([(27.0, 47.0), (76.0, 47.0), (134.0, 47.0)].map(Some)),
            "grindstone" => s.extend([(49.0, 19.0), (49.0, 40.0), (129.0, 34.0)].map(Some)),
            "enchantment" => s.extend([(15.0, 47.0), (35.0, 47.0)].map(Some)),
            "brewing_stand" => s.extend(
                [
                    (56.0, 51.0),
                    (79.0, 58.0),
                    (102.0, 51.0),
                    (79.0, 17.0),
                    (17.0, 17.0),
                ]
                .map(Some),
            ),
            "smithing" => {
                s.extend([(8.0, 48.0), (26.0, 48.0), (44.0, 48.0), (98.0, 48.0)].map(Some))
            }
            // VANILLA: CartographyTableMenu / StonecutterMenu / CrafterMenu.
            "cartography_table" => s.extend([(15.0, 15.0), (15.0, 52.0), (145.0, 39.0)].map(Some)),
            "stonecutter" => s.extend([(20.0, 33.0), (143.0, 33.0)].map(Some)),
            // VANILLA: LoomMenu — banner, dye, pattern, result.
            "loom" => s.extend([(13.0, 26.0), (33.0, 26.0), (23.0, 45.0), (143.0, 57.0)].map(Some)),
            "crafter_3x3" => {
                grid(&mut s, 26.0, 17.0, 3, 9);
                s.push(Some((134.0, 35.0)));
            }
            "hopper" => grid(&mut s, 44.0, 20.0, 5, 5),
            _ => grid(&mut s, 8.0, 18.0, 9, 27),
        }
        player_inv_rows(&mut s, main_y, hot_y);
        debug_assert_eq!(s.len(), total);
        return Layout {
            kind: Kind::Tex(tex, h),
            w: 176.0,
            h,
            slots: s,
        };
    }
    // Неподдерживаемое меню: только инвентарь игрока (слоты контейнера скрыты).
    let mut s = vec![None; n - 36];
    player_inv_rows(&mut s, 84.0, 142.0);
    Layout {
        kind: Kind::Other,
        w: 176.0,
        h: 166.0,
        slots: s,
    }
}

impl Layout {
    /// Это InventoryMenu (экран инвентаря игрока).
    pub fn is_player(&self) -> bool {
        self.kind == Kind::Player
    }
    /// Меню с данной фоновой текстурой.
    fn is_tex(&self, tex: &str) -> bool {
        matches!(self.kind, Kind::Tex(t, _) if t == tex)
    }
    /// Окно наковальни.
    pub fn is_anvil(&self) -> bool {
        matches!(self.kind, Kind::Tex("container_anvil", _))
    }
    /// Окно стола зачарований.
    pub fn is_enchanting(&self) -> bool {
        matches!(self.kind, Kind::Tex("container_enchanting", _))
    }
    /// Окно ткацкого станка.
    pub fn is_loom(&self) -> bool {
        matches!(self.kind, Kind::Tex("container_loom", _))
    }
    /// Экран книги на кафедре.
    pub fn is_lectern(&self) -> bool {
        self.kind == Kind::Lectern
    }
    /// Окно торговли.
    pub fn is_merchant(&self) -> bool {
        matches!(self.kind, Kind::Tex("container_villager", _))
    }
    /// Окно маяка.
    pub fn is_beacon(&self) -> bool {
        matches!(self.kind, Kind::Tex("container_beacon", _))
    }
    pub fn left(&self, gw: f32) -> f32 {
        ((gw - self.w) / 2.0).floor()
    }
    pub fn top(&self, gh: f32) -> f32 {
        if self.kind == Kind::Lectern {
            return crate::bookview::TOP;
        }
        ((gh - self.h) / 2.0).floor()
    }

    /// Слот под курсором (координаты GUI).
    pub fn slot_at(&self, gw: f32, gh: f32, mx: f32, my: f32) -> Option<usize> {
        let (l, t) = (self.left(gw), self.top(gh));
        self.slots.iter().position(|p| {
            p.is_some_and(|(x, y)| {
                mx >= l + x && mx < l + x + 16.0 && my >= t + y && my < t + y + 16.0
            })
        })
    }

    pub fn outside(&self, gw: f32, gh: f32, mx: f32, my: f32) -> bool {
        let (l, t) = (self.left(gw), self.top(gh));
        mx < l || my < t || mx >= l + self.w || my >= t + self.h
    }
}

/// Содержимое слота `i` активного меню.
pub fn slot_stack_pub(inv: &Inventory, i: usize) -> Option<&ezzmc_protocol::item::ItemStack> {
    slot_stack(inv, i)
}

fn slot_stack(inv: &Inventory, i: usize) -> Option<&ezzmc_protocol::item::ItemStack> {
    match &inv.open {
        Some(o) => o.slots.get(i)?.as_ref(),
        None => inv.slots.get(i)?.as_ref(),
    }
}

/// Символ шрифта `alt` (галактический алфавит) в области частного использования.
pub fn alt_char(c: char) -> char {
    char::from_u32(0xE000 + c as u32).unwrap_or(c)
}

/// VANILLA: EnchantmentNames — слова для случайных названий.
const ENCH_WORDS: &[&str] = &[
    "the",
    "elder",
    "scrolls",
    "klaatu",
    "berata",
    "niktu",
    "xyzzy",
    "bless",
    "curse",
    "light",
    "darkness",
    "fire",
    "air",
    "earth",
    "water",
    "hot",
    "dry",
    "cold",
    "wet",
    "ignite",
    "snuff",
    "embiggen",
    "twist",
    "shorten",
    "stretch",
    "fiddle",
    "destroy",
    "imbue",
    "galvanize",
    "enchant",
    "free",
    "limited",
    "range",
    "of",
    "towards",
    "inside",
    "sphere",
    "cube",
    "self",
    "other",
    "ball",
    "mental",
    "physical",
    "grow",
    "shrink",
    "demon",
    "elemental",
    "spirit",
    "animal",
    "creature",
    "beast",
    "humanoid",
    "undead",
    "fresh",
    "stale",
    "phnglui",
    "mglwnafh",
    "cthulhu",
    "rlyeh",
    "wgahnagl",
    "fhtagn",
    "baguette",
];

/// VANILLA: EnchantmentNames#initSeed + getRandomName — названия по слотам (только для cost != 0),
/// обрезанные по ширине `max_w[i]` (FontSplitter#headByWidth), уже в символах шрифта `alt`.
fn ench_names(
    seed: i64,
    costs: [i16; 3],
    max_w: [f32; 3],
    tw: &dyn Fn(&str) -> f32,
) -> [String; 3] {
    let words = ENCH_WORDS;
    let mut rng = ezzmc_world::noise::LegacyRandom::new(0);
    rng.set_seed(seed);
    let mut out = [String::new(), String::new(), String::new()];
    for i in 0..3 {
        if costs[i] == 0 {
            continue;
        }
        let n = rng.next_int_bound(2) + 3;
        let mut s = String::new();
        for k in 0..n {
            if k != 0 {
                s.push(' ');
            }
            s.push_str(words[rng.next_int_bound(words.len() as i32) as usize]);
        }
        let mut head = String::new();
        for c in s.chars() {
            let mut t = head.clone();
            t.push(alt_char(c));
            if tw(&t) > max_w[i] {
                break;
            }
            head = t;
        }
        out[i] = head;
    }
    out
}

fn inv_slot(o: &crate::inventory::OpenMenu, i: usize) -> Option<&ezzmc_protocol::item::ItemStack> {
    o.slots.get(i)?.as_ref()
}

/// Команда клика (id контейнера и state_id берутся из инвентаря).
pub fn click_cmd(inv: &Inventory, slot: i16, button: i8, input: i32) -> NetCmd {
    NetCmd::ContainerClick {
        container: inv.open.as_ref().map_or(0, |o| o.id),
        state_id: inv.state_id,
        slot,
        button,
        input,
        changed: Vec::new(),
        carried: claim(inv.cursor.as_ref()),
    }
}

/// Заявка стека для сервера: (id предмета, количество).
pub fn claim(s: Option<&ezzmc_protocol::item::ItemStack>) -> Option<(u32, i32)> {
    s.map(|s| (s.item, s.count))
}

/// VANILLA: AbstractContainerScreen#mouseClicked (без перетаскивания и двойного клика).
#[allow(clippy::too_many_arguments)]
pub fn on_click(
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
    right: bool,
    shift: bool,
    double: bool,
) -> Option<NetCmd> {
    let button = i8::from(right);
    if let (Kind::Tex("container_enchanting", _), Some(o)) = (lay.kind, &inv.open) {
        // VANILLA: EnchantmentScreen#mouseClicked + EnchantmentMenu#clickMenuButton (без проверки уровня игрока — её делает сервер).
        let (l, t) = (lay.left(gw), lay.top(gh));
        let lapis = inv_slot(o, 1).map_or(0, |s| s.count);
        for i in 0..3usize {
            let (xx, yy) = (mx - (l + 60.0), my - (t + 14.0 + 19.0 * i as f32));
            if xx >= 0.0
                && yy >= 0.0
                && xx < 108.0
                && yy < 19.0
                && o.data[i] > 0
                && inv_slot(o, 0).is_some()
                && (inv.infinite_materials
                    || (lapis > i as i32 && inv.xp_level >= i32::from(o.data[i])))
            {
                return Some(NetCmd::ContainerButton(o.id, i as i32));
            }
        }
    }
    if let Some(i) = lay.slot_at(gw, gh, mx, my) {
        // VANILLA: AbstractContainerScreen#mouseClicked — двойной клик ЛКМ с предметом на курсоре → PICKUP_ALL.
        let input = if shift {
            QUICK_MOVE
        } else if double && !right && inv.cursor.is_some() {
            PICKUP_ALL
        } else {
            PICKUP
        };
        return Some(click_cmd(inv, i as i16, button, input));
    }
    if lay.outside(gw, gh, mx, my) && inv.cursor.is_some() {
        return Some(click_cmd(inv, OUTSIDE, button, PICKUP));
    }
    None
}

/// Фон и иконки (проход painter). Курсорный стек рисуется последним.
#[allow(clippy::too_many_arguments)]
pub fn draw_sprites(
    p: &mut GuiPainter,
    dev: &wgpu::Device,
    q: &wgpu::Queue,
    rm: &ResourceManager,
    inv: &Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    drag: &[usize],
    mx: f32,
    my: f32,
) {
    let (l, t) = (lay.left(gw), lay.top(gh));
    let k = 1.0 / 256.0;
    match lay.kind {
        Kind::Chest(rows) => {
            let top_h = rows as f32 * 18.0 + 17.0;
            p.blit(
                "container_54",
                l,
                t,
                176.0,
                top_h,
                [0.0, 0.0, 176.0 * k, top_h * k],
                0xFFFF_FFFF,
            );
            p.blit(
                "container_54",
                l,
                t + top_h,
                176.0,
                96.0,
                [0.0, 126.0 * k, 176.0 * k, 222.0 * k],
                0xFFFF_FFFF,
            );
        }
        Kind::Tex(tex, h) => {
            // villager.png — 512×256, остальные 256×256.
            let kx = if tex == "container_villager" {
                k / 2.0
            } else {
                k
            };
            p.blit(
                tex,
                l,
                t,
                lay.w,
                h,
                [0.0, 0.0, lay.w * kx, h * k],
                0xFFFF_FFFF,
            );
            if let (true, Some(o)) = (tex == "container_villager", &inv.open) {
                crate::merchantscreen::draw_sprites(p, dev, q, rm, o, l, t, mx, my);
            }
            if let (true, Some(o)) = (tex == "container_loom", &inv.open) {
                crate::loomscreen::draw_sprites(p, o, l, t, mx, my);
            }
            if let (true, Some(o)) = (tex == "container_beacon", &inv.open) {
                crate::beaconscreen::draw_sprites(p, dev, q, rm, o, l, t, mx, my);
            }
            if let (Some(kind), Some(o)) = (tex.strip_prefix("container_"), &inv.open) {
                if kind == "enchanting" {
                    // VANILLA: EnchantmentScreen#extractBackground (кнопки 108×19, data[0..3] — цены).
                    let gold = inv_slot(o, 1).map_or(0, |s| s.count);
                    for i in 0..3usize {
                        let y = t + 14.0 + 19.0 * i as f32;
                        let n = i + 1;
                        let (x0, cost) = (l + 60.0, o.data[i]);
                        if cost == 0 {
                            p.blit(
                                "ench_slot_off",
                                x0,
                                y,
                                108.0,
                                19.0,
                                [0.0, 0.0, 1.0, 1.0],
                                0xFFFF_FFFF,
                            );
                            continue;
                        }
                        let off = (gold < n as i32 || inv.xp_level < i32::from(cost))
                            && !inv.infinite_materials;
                        let hover = mx >= x0 && my >= y && mx < x0 + 108.0 && my < y + 19.0;
                        let bg = if off {
                            "ench_slot_off"
                        } else if hover {
                            "ench_slot_hi"
                        } else {
                            "ench_slot"
                        };
                        p.blit(bg, x0, y, 108.0, 19.0, [0.0, 0.0, 1.0, 1.0], 0xFFFF_FFFF);
                        let lv = format!("ench_lvl_{n}{}", if off { "_off" } else { "" });
                        p.blit(
                            &lv,
                            x0 + 1.0,
                            y + 1.0,
                            16.0,
                            16.0,
                            [0.0, 0.0, 1.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
                if kind == "anvil" {
                    // VANILLA: AnvilScreen#extractBackground — поле имени 110×16 (59,20), disabled без предмета
                    // в слоте 0; extractErrorIcon — 28×21 (99,45), если слоты заняты, а результата нет.
                    let has = |i: usize| inv_slot(o, i).is_some();
                    p.blit(
                        if has(0) {
                            "anvil_field"
                        } else {
                            "anvil_field_off"
                        },
                        l + 59.0,
                        t + 20.0,
                        110.0,
                        16.0,
                        [0.0, 0.0, 1.0, 1.0],
                        0xFFFF_FFFF,
                    );
                    if (has(0) || has(1)) && !has(2) {
                        p.blit(
                            "anvil_error",
                            l + 99.0,
                            t + 45.0,
                            28.0,
                            21.0,
                            [0.0, 0.0, 1.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
                if kind == "brewing" {
                    // VANILLA: BrewingStandScreen#extractBackground (data: 0 варка, 1 топливо, 2 всего, 3 макс. топливо).
                    let d = &o.data;
                    let (fuel, total_fuel) = (i32::from(d[1]), i32::from(d[3]));
                    let fl = if total_fuel > 0 {
                        ((18 * fuel + total_fuel - 1) / total_fuel).clamp(0, 18)
                    } else {
                        0
                    };
                    if fl > 0 {
                        let w = fl as f32;
                        p.blit(
                            "brew_fuel",
                            l + 60.0,
                            t + 44.0,
                            w,
                            4.0,
                            [0.0, 0.0, w / 18.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                    let (tick, total) = (i32::from(d[0]), i32::from(d[2]));
                    if tick > 0 && total > 0 {
                        let len = (28.0 * (1.0 - tick as f32 / total as f32)) as i32;
                        if len > 0 {
                            let h = len as f32;
                            p.blit(
                                "brew_progress",
                                l + 97.0,
                                t + 16.0,
                                9.0,
                                h,
                                [0.0, 0.0, 1.0, h / 28.0],
                                0xFFFF_FFFF,
                            );
                        }
                        const BUBBLES: [i32; 7] = [29, 24, 20, 16, 11, 6, 0];
                        let len = BUBBLES[(tick / 2 % 7) as usize];
                        if len > 0 {
                            let h = len as f32;
                            let v0 = (29.0 - h) / 29.0;
                            p.blit(
                                "brew_bubbles",
                                l + 63.0,
                                t + 14.0 + 29.0 - h,
                                12.0,
                                h,
                                [0.0, v0, 1.0, 1.0],
                                0xFFFF_FFFF,
                            );
                        }
                    }
                }
                if matches!(kind, "furnace" | "blast_furnace" | "smoker") {
                    // VANILLA: AbstractFurnaceScreen#renderBg (огонь 14×14 снизу вверх, стрелка 24×16 слева направо).
                    let d = &o.data;
                    if d[0] > 0 {
                        let dur = if d[1] == 0 { 200.0 } else { f32::from(d[1]) };
                        let prog = (f32::from(d[0]) / dur).clamp(0.0, 1.0);
                        let hh = (prog * 13.0).ceil() + 1.0;
                        let off = 14.0 - hh;
                        p.blit(
                            &format!("lit_{kind}"),
                            l + 56.0,
                            t + 36.0 + off,
                            14.0,
                            hh,
                            [0.0, off / 14.0, 1.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                    if d[2] != 0 && d[3] != 0 {
                        let prog = (f32::from(d[2]) / f32::from(d[3])).clamp(0.0, 1.0);
                        let ww = (prog * 24.0).ceil();
                        p.blit(
                            &format!("arrow_{kind}"),
                            l + 79.0,
                            t + 34.0,
                            ww,
                            16.0,
                            [0.0, 0.0, ww / 24.0, 1.0],
                            0xFFFF_FFFF,
                        );
                    }
                }
            }
        }
        _ => p.blit(
            "container_inv",
            l,
            t,
            176.0,
            166.0,
            [0.0, 0.0, 176.0 * k, 166.0 * k],
            0xFFFF_FFFF,
        ),
    }
    for (i, pos) in lay.slots.iter().enumerate() {
        // VANILLA: extractSlot — подсветка раздачи (0x80FFFFFF) под предметом.
        if let (Some((x, y)), true) = (pos, drag.contains(&i)) {
            p.blit(
                "white",
                l + x,
                t + y,
                16.0,
                16.0,
                [0.0, 0.0, 1.0, 1.0],
                0x80FF_FFFF,
            );
        }
        if let (Some((x, y)), Some(s)) = (pos, slot_stack(inv, i)) {
            hud::stack_icon(p, dev, q, rm, s, l + x, t + y);
        } else if let (Some((x, y)), true) = (pos, lay.is_player()) {
            hud::empty_slot_icon(p, i, l + x, t + y);
        }
    }
}

/// Затемнение мира под экраном (VANILLA: Screen#renderBackground → gradient 0xC0101010→0xD0101010).
pub fn draw_dim(t: &mut TextRenderer, gw: f32, gh: f32) {
    t.fill(0.0, 0.0, gw, gh, 0xC010_1010);
}

/// Подсветка, заголовок и количества (проход текста).
#[allow(clippy::too_many_arguments)]
pub fn draw_text(
    t: &mut TextRenderer,
    inv: &Inventory,
    lang: &crate::ui::lang::Lang,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
    anvil: &mut AnvilName,
    caret: bool,
) {
    let (l, top) = (lay.left(gw), lay.top(gh));
    // VANILLA: Screen title — Component (translatable) → lang.
    let title = inv
        .open
        .as_ref()
        .map_or_else(|| lang.tr("container.crafting"), |o| lang.tr(&o.title));
    let title = title.as_str();
    // VANILLA: titleLabelX 8 / 97 для InventoryScreen, titleLabelY 6 / 8.
    // VANILLA: AnvilScreen — titleLabelX 60.
    let (tx, ty) = if lay.kind == Kind::Player {
        (97.0, 8.0)
    } else if lay.is_anvil() {
        (60.0, 6.0)
    } else if lay.is_tex("container_smithing") {
        // VANILLA: SmithingScreen — titleLabelX 44, titleLabelY 15.
        (44.0, 15.0)
    } else if lay.is_tex("container_cartography") || lay.is_tex("container_loom") {
        (8.0, 4.0) // VANILLA: CartographyTableScreen — titleLabelY −= 2.
    } else if lay.is_tex("container_stonecutter") {
        (8.0, 5.0) // VANILLA: StonecutterScreen — titleLabelY−−.
    } else if lay.is_tex("container_crafter") {
        // VANILLA: CrafterScreen — заголовок по центру.
        ((176.0 - t.width_fmt(title, false)) / 2.0, 6.0)
    } else {
        (8.0, 6.0)
    };
    if let (true, Some(o)) = (lay.is_merchant(), &inv.open) {
        // VANILLA: MerchantScreen#extractLabels — заголовок с уровнем, «Trades», «Inventory» (x=107).
        crate::merchantscreen::draw_text(t, lang, o, title, l, top);
    } else if lay.is_beacon() {
        // VANILLA: BeaconScreen#extractLabels — свои подписи вместо заголовка и «Inventory».
        crate::beaconscreen::draw_text(t, lang, l, top);
    } else {
        t.draw(title, l + tx, top + ty, 0x404040, false);
    }
    if lay.kind != Kind::Player && !lay.is_beacon() && !lay.is_merchant() {
        // VANILLA: AbstractContainerScreen — inventoryLabelX 8, inventoryLabelY = imageHeight − 94.
        t.draw(
            &lang.tr("container.inventory"),
            l + 8.0,
            top + lay.h - 94.0,
            0x404040,
            false,
        );
    }
    if let (true, Some(o)) = (lay.is_anvil(), &inv.open) {
        anvil.draw(t, lang, inv, o, l, top, caret);
    }
    if let (Kind::Tex("container_enchanting", _), Some(o)) = (lay.kind, &inv.open) {
        // VANILLA: EnchantmentScreen — число уровней справа в кнопке (цвет: доступно −8323296 / нет −12550384).
        let gold = inv_slot(o, 1).map_or(0, |s| s.count);
        let costs = [o.data[0], o.data[1], o.data[2]];
        let max_w = [0, 1, 2].map(|i| 86.0 - t.width_fmt(&costs[i].to_string(), false));
        let names = ench_names(
            // data[3] — enchantmentSeed; пакет несёт short (знаковый), как в ванили
            i64::from(o.data[3]),
            costs,
            max_w,
            &|s| t.width_fmt(s, false),
        );
        for i in 0..3usize {
            let cost = i32::from(o.data[i]);
            if cost == 0 {
                continue;
            }
            // VANILLA: EnchantmentScreen — название: обычный цвет / подсветка / затемнённый.
            let (mx0, my0) = (mx - (l + 60.0), my - (top + 14.0 + 19.0 * i as f32));
            let hover = mx0 >= 0.0 && my0 >= 0.0 && mx0 < 108.0 && my0 < 19.0;
            let dis = (gold < i as i32 + 1 || inv.xp_level < cost) && !inv.infinite_materials;
            let ncol = if dis {
                0x342F25
            } else if hover {
                0xFFFF80
            } else {
                0x685E4A
            };
            t.draw(
                &names[i],
                l + 80.0,
                top + 16.0 + 19.0 * i as f32,
                ncol,
                false,
            );
            let off = (gold < i as i32 + 1 || inv.xp_level < cost) && !inv.infinite_materials;
            let col = if off { 0x407F10 } else { 0x80FF20 };
            let s = cost.to_string();
            let w = t.width_fmt(&s, false);
            t.draw(
                &s,
                l + 60.0 + 20.0 + 86.0 - w,
                top + 16.0 + 19.0 * i as f32 + 7.0,
                col,
                true,
            );
        }
    }
    for (i, pos) in lay.slots.iter().enumerate() {
        if let (Some((x, y)), Some(s)) = (pos, slot_stack(inv, i)) {
            hud::stack_count(t, s, l + x, top + y);
        }
    }
    if let Some(i) = lay.slot_at(gw, gh, mx, my) {
        if let Some((x, y)) = lay.slots[i] {
            t.fill(l + x, top + y, l + x + 16.0, top + y + 16.0, 0x80FF_FFFF);
        }
    }
}

/// Имя предмета под курсором.
pub fn hover_item<'a>(
    inv: &'a Inventory,
    lay: &Layout,
    gw: f32,
    gh: f32,
    mx: f32,
    my: f32,
) -> Option<&'a ezzmc_protocol::item::ItemStack> {
    slot_stack(inv, lay.slot_at(gw, gh, mx, my)?)
}

/// Поле имени наковальни.
/// VANILLA: AnvilScreen (EditBox name, onNameChanged, slotChanged) + AnvilMenu#setItemName.
#[derive(Default)]
pub struct AnvilName {
    pub edit: crate::editbox::EditBox,
    menu: i32,
    fp: Option<String>,
    sent: Option<String>,
}

impl AnvilName {
    /// Предмет в слоте 0 есть → поле редактируемо.
    pub fn editable(o: &crate::inventory::OpenMenu) -> bool {
        inv_slot(o, 0).is_some()
    }

    /// Синхронизация со слотом 0 (VANILLA: AnvilScreen#slotChanged). Вернёт имя для отправки.
    pub fn sync(
        &mut self,
        lang: &crate::ui::lang::Lang,
        o: &crate::inventory::OpenMenu,
    ) -> Option<String> {
        if self.menu != o.id || self.edit.max != 50 {
            *self = Self::default();
            self.edit.max = 50;
            self.menu = o.id;
        }
        let stack = inv_slot(o, 0);
        let fp = stack.map(|s| format!("{}|{}", s.item, crate::tooltip::hover_name(lang, s)));
        if fp == self.fp {
            return None;
        }
        self.fp = fp;
        let name = stack.map_or_else(String::new, |s| crate::tooltip::hover_name(lang, s));
        self.edit.set(&name);
        self.changed(lang, o)
    }

    /// VANILLA: AnvilScreen#onNameChanged → AnvilMenu#setItemName.
    pub fn changed(
        &mut self,
        lang: &crate::ui::lang::Lang,
        o: &crate::inventory::OpenMenu,
    ) -> Option<String> {
        let stack = inv_slot(o, 0)?;
        let mut name = self.edit.text.clone();
        if !crate::tooltip::has_custom_name(stack)
            && name == crate::tooltip::hover_name(lang, stack)
        {
            name.clear();
        }
        if name.chars().count() > 50 || self.sent.as_deref() == Some(name.as_str()) {
            return None;
        }
        self.sent = Some(name.clone());
        Some(name)
    }

    /// Подпись цены и поле имени (VANILLA: AnvilScreen#extractLabels, EditBox).
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        t: &mut TextRenderer,
        lang: &crate::ui::lang::Lang,
        inv: &Inventory,
        o: &crate::inventory::OpenMenu,
        l: f32,
        top: f32,
        caret: bool,
    ) {
        let cost = i32::from(o.data[0]);
        if cost > 0 {
            let mut color = 0x80FF20;
            let label = if cost >= 40 && !inv.infinite_materials {
                color = 0xFF6060;
                Some(lang.tr("container.repair.expensive"))
            } else if inv_slot(o, 2).is_some() {
                if inv.xp_level < cost && !inv.infinite_materials {
                    color = 0xFF6060;
                }
                Some(lang.fmt("container.repair.cost", &[&cost.to_string()]))
            } else {
                None
            };
            if let Some(s) = label {
                let w = t.width_fmt(&s, false);
                let tx = 176.0 - 8.0 - w - 2.0;
                t.fill(l + tx - 2.0, top + 67.0, l + 168.0, top + 79.0, 0x4F00_0000);
                t.draw(&s, l + tx, top + 69.0, color, true);
            }
        }
        let e = &mut self.edit;
        let (x, y, w) = (l + 62.0, top + 24.0, 103.0);
        let cs: Vec<char> = e.text.chars().collect();
        let sub = |a: usize, b: usize| cs[a..b].iter().collect::<String>();
        e.scroll = e.scroll.min(cs.len()).min(e.cursor);
        while e.scroll < e.cursor && t.width_fmt(&sub(e.scroll, e.cursor), false) > w {
            e.scroll += 1;
        }
        // VANILLA: EditBox#scrollTo — текст заполняет поле, лишняя прокрутка слева убирается.
        while e.scroll > 0 && t.width_fmt(&sub(e.scroll - 1, cs.len()), false) <= w {
            e.scroll -= 1;
        }
        let mut end = e.scroll;
        while end < cs.len() && t.width_fmt(&sub(e.scroll, end + 1), false) <= w {
            end += 1;
        }
        let ty = y + 2.0;
        let (a, b) = e.selection();
        let (a, b) = (a.clamp(e.scroll, end), b.clamp(e.scroll, end));
        if a < b && Self::editable(o) {
            let x1 = x + t.width_fmt(&sub(e.scroll, a), false);
            let x2 = x + t.width_fmt(&sub(e.scroll, b), false);
            t.fill(x1, ty - 1.0, x2, ty + 9.0, 0xFF3A_5FD9);
        }
        let col = if Self::editable(o) {
            0xFFFFFF
        } else {
            0x707070
        };
        t.draw(&sub(e.scroll, end), x, ty, col, true);
        if Self::editable(o) && caret {
            let c = e.cursor.clamp(e.scroll, end);
            let cx = x + t.width_fmt(&sub(e.scroll, c), false);
            if c == cs.len() {
                t.draw("_", cx, ty, 0xFFFFFF, true);
            } else {
                t.fill(cx, ty - 1.0, cx + 1.0, ty + 9.0, 0xFFD0_D0D0);
            }
        }
    }
}
