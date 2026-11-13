//! Экран маяка: кнопки эффектов, подтверждение/отмена.
//! VANILLA: BeaconScreen (230×219), BeaconMenu (data: 0 уровни, 1 primary, 2 secondary — id реестра +1),
//! BeaconBlockEntity#BEACON_EFFECTS, ServerboundSetBeaconPacket.
use crate::inventory::{Inventory, OpenMenu};
use crate::netthread::NetCmd;
use ezzmc_assets::pack::ResourceManager;
use ezzmc_render::gui::GuiPainter;
use ezzmc_render::text::TextRenderer;

/// VANILLA: BeaconBlockEntity#BEACON_EFFECTS в кодировке BeaconMenu#encodeEffect
/// (speed 0, haste 2, resistance 10, jump_boost 7, strength 4, regeneration 9; +1).
const TIERS: [&[i16]; 4] = [&[1, 3], &[11, 8], &[5], &[10]];

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// Эффект `effect`, первичная/вторичная колонка, ярус.
    Power {
        effect: i16,
        primary: bool,
        tier: usize,
    },
    /// «Эффект II» — только когда выбран primary (BeaconUpgradePowerButton).
    Upgrade,
    Confirm,
    Cancel,
}

#[derive(Clone, Copy)]
struct Btn {
    x: f32,
    y: f32,
    kind: Kind,
}

/// VANILLA: BeaconScreen#init — раскладка кнопок.
fn buttons(sel: [i16; 2]) -> Vec<Btn> {
    let mut v = Vec::new();
    for (tier, list) in TIERS.iter().take(3).enumerate() {
        let count = list.len() as i32;
        let total = count * 22 + (count - 1) * 2;
        for (c, &effect) in list.iter().enumerate() {
            v.push(Btn {
                x: (76 + c as i32 * 24 - total / 2) as f32,
                y: (22 + tier as i32 * 25) as f32,
                kind: Kind::Power {
                    effect,
                    primary: true,
                    tier,
                },
            });
        }
    }
    let count = TIERS[3].len() as i32 + 1;
    let total = count * 22 + (count - 1) * 2;
    for (c, &effect) in TIERS[3].iter().enumerate() {
        v.push(Btn {
            x: (167 + c as i32 * 24 - total / 2) as f32,
            y: 47.0,
            kind: Kind::Power {
                effect,
                primary: false,
                tier: 3,
            },
        });
    }
    if sel[0] != 0 {
        v.push(Btn {
            x: (167 + (count - 1) * 24 - total / 2) as f32,
            y: 47.0,
            kind: Kind::Upgrade,
        });
    }
    v.push(Btn {
        x: 164.0,
        y: 107.0,
        kind: Kind::Confirm,
    });
    v.push(Btn {
        x: 190.0,
        y: 107.0,
        kind: Kind::Cancel,
    });
    v
}

/// (active, selected, effect) кнопки. VANILLA: BeaconButton#updateStatus.
fn status(o: &OpenMenu, b: &Btn) -> (bool, bool, i16) {
    let levels = i32::from(o.data[0]);
    let payment = o.slots.first().is_some_and(Option::is_some);
    match b.kind {
        Kind::Power {
            effect,
            primary,
            tier,
        } => (
            (tier as i32) < levels,
            effect == o.sel[usize::from(!primary)],
            effect,
        ),
        Kind::Upgrade => (3 < levels, o.sel[1] == o.sel[0], o.sel[0]),
        Kind::Confirm => (payment && o.sel[0] != 0, false, 0),
        Kind::Cancel => (true, false, 0),
    }
}

fn hit(b: &Btn, l: f32, t: f32, mx: f32, my: f32) -> bool {
    mx >= l + b.x && my >= t + b.y && mx < l + b.x + 22.0 && my < t + b.y + 22.0
}

fn effect_name(enc: i16) -> &'static str {
    ezzmc_protocol::generated::r26_3::MOB_EFFECT
        .get((enc - 1).max(0) as usize)
        .copied()
        .unwrap_or("")
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
    // VANILLA: BeaconScreen#extractBackground — иконки материалов оплаты.
    for (name, x) in [
        ("netherite_ingot", 20.0),
        ("emerald", 41.0),
        ("diamond", 63.0),
        ("gold_ingot", 86.0),
        ("iron_ingot", 108.0),
    ] {
        if let Some(s) = crate::creativescreen::stack_of(name, 1) {
            crate::hud::stack_icon(p, dev, q, rm, &s, l + x, t + 109.0);
        }
    }
    for b in buttons(o.sel) {
        let (active, selected, effect) = status(o, &b);
        let bg = if !active {
            "bcn_btn_off"
        } else if selected {
            "bcn_btn_sel"
        } else if hit(&b, l, t, mx, my) {
            "bcn_btn_hi"
        } else {
            "bcn_btn"
        };
        p.blit(bg, l + b.x, t + b.y, 22.0, 22.0, full, 0xFFFF_FFFF);
        let icon = match b.kind {
            Kind::Confirm => "bcn_confirm".to_string(),
            Kind::Cancel => "bcn_cancel".to_string(),
            _ => format!("mob_effect_{}", effect_name(effect)),
        };
        p.blit(
            &icon,
            l + b.x + 2.0,
            t + b.y + 2.0,
            18.0,
            18.0,
            full,
            0xFFFF_FFFF,
        );
    }
}

/// Подписи «Primary/Secondary Power». VANILLA: BeaconScreen#extractLabels (centeredText, −2039584, тень).
pub fn draw_text(t: &mut TextRenderer, lang: &crate::ui::lang::Lang, l: f32, top: f32) {
    for (key, cx) in [
        ("block.minecraft.beacon.primary", 62.0),
        ("block.minecraft.beacon.secondary", 169.0),
    ] {
        let s = lang.tr(key);
        let w = t.width_fmt(&s, false);
        t.draw(&s, l + cx - w / 2.0, top + 10.0, 0xE0E0E0, true);
    }
}

/// Подсказка кнопки. VANILLA: BeaconPowerButton (имя эффекта, у Upgrade + « II»), Done/Cancel.
pub fn hover(
    lang: &crate::ui::lang::Lang,
    o: &OpenMenu,
    l: f32,
    t: f32,
    mx: f32,
    my: f32,
) -> Option<Vec<(String, u32)>> {
    let b = buttons(o.sel).into_iter().find(|b| hit(b, l, t, mx, my))?;
    let (_, _, effect) = status(o, &b);
    let text = match b.kind {
        Kind::Confirm => lang.tr("gui.done"),
        Kind::Cancel => lang.tr("gui.cancel"),
        Kind::Upgrade => format!(
            "{} II",
            lang.tr(&format!("effect.minecraft.{}", effect_name(effect)))
        ),
        Kind::Power { .. } => lang.tr(&format!("effect.minecraft.{}", effect_name(effect))),
    };
    Some(vec![(text, 0xFFFF_FFFF)])
}

pub enum Act {
    /// Клик не по кнопке — обычная обработка слотов.
    Pass,
    /// Клик поглощён (выбор эффекта или неактивная кнопка).
    Handled,
    /// Подтвердить/отмена: отправить (если есть) и закрыть меню.
    Close(Option<NetCmd>),
}

/// VANILLA: BeaconPowerButton#onPress / BeaconConfirmButton / BeaconCancelButton.
pub fn click(inv: &mut Inventory, l: f32, t: f32, mx: f32, my: f32) -> Act {
    let Some(o) = inv.open.as_mut() else {
        return Act::Pass;
    };
    let Some(b) = buttons(o.sel).into_iter().find(|b| hit(b, l, t, mx, my)) else {
        return Act::Pass;
    };
    let (active, selected, effect) = status(o, &b);
    if !active {
        return Act::Handled;
    }
    match b.kind {
        Kind::Confirm => Act::Close(Some(NetCmd::SetBeacon(
            i32::from(o.sel[0]),
            i32::from(o.sel[1]),
        ))),
        Kind::Cancel => Act::Close(None),
        Kind::Power { primary, .. } if !selected => {
            if primary {
                o.sel[0] = effect;
                if o.sel[1] != effect {
                    o.sel[1] = 0;
                }
            } else {
                o.sel[1] = effect;
            }
            Act::Handled
        }
        Kind::Upgrade if !selected => {
            o.sel[1] = effect;
            Act::Handled
        }
        _ => Act::Handled,
    }
}
