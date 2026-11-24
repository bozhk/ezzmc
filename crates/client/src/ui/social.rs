//! Экран социальных взаимодействий. VANILLA: SocialInteractionsScreen + SocialInteractionsPlayerList +
//! PlayerEntry (вкладки Все/Скрытые/Заблокированные, поиск, строки 36 px, кнопки скрыть/показать).
//! Нет: настоящие скины (везде Стив), кнопка жалобы (чат без подписей), список заблокированных.

use super::{edit_apply, Action, Frame, State, UiKey, WHITE};
use crate::editbox::EditBox;

pub const TEXTURES: &[(&str, &str)] = &[
    ("soc_bg", "sprites/social_interactions/background"),
    ("soc_mute", "sprites/social_interactions/mute_button"),
    (
        "soc_mute_hl",
        "sprites/social_interactions/mute_button_highlighted",
    ),
    ("soc_unmute", "sprites/social_interactions/unmute_button"),
    (
        "soc_unmute_hl",
        "sprites/social_interactions/unmute_button_highlighted",
    ),
];

/// Игрок из таба или скрытый офлайн-игрок.
#[derive(Clone, Debug)]
pub struct Player {
    pub id: u128,
    pub name: String,
    pub online: bool,
}

pub struct Social {
    page: u8,
    search: EditBox,
    scroll: f32,
    pub me: u128,
    pub server: String,
}

pub fn open(me: u128, server: String) -> Social {
    Social {
        page: 0,
        search: EditBox::new(16),
        scroll: 0.0,
        me,
        server,
    }
}

const ROW_W: f32 = 220.0;
const ITEM_H: f32 = 36.0;

pub(super) fn screen(
    f: &mut Frame,
    st: &mut State,
    keys: &[UiKey],
    typed: &str,
    wheel: f32,
    now_ms: u64,
    action: &mut Action,
) {
    let Some(mut s) = st.social.take() else {
        *action = Action::CloseSocial;
        return;
    };
    let mut close = false;
    let mut rest: Vec<UiKey> = Vec::new();
    for k in keys {
        match k {
            UiKey::Escape => close = true,
            _ => rest.push(*k),
        }
    }
    edit_apply(&mut s.search, &rest, typed, &mut st.clip_out);

    // VANILLA: windowHeight = max(52, height - 128 - 16); listEnd = 80 + windowHeight - 8.
    let win_h = (f.h - 144.0).max(52.0);
    let list_end = 80.0 + win_h - 8.0;
    let margin_x = ((f.w - 238.0) / 2.0).floor();
    let row_left = ((f.w - ROW_W) / 2.0).floor();
    let row_right = row_left + ROW_W;

    f.center("gui.socialInteractions.title", 18.0, WHITE);
    f.p.nine_slice(
        "soc_bg",
        margin_x + 3.0,
        64.0,
        236.0,
        win_h + 16.0,
        [236.0, 32.0],
        [8.0; 4],
        0xFFFFFFFF,
    );
    if !s.server.is_empty() || !st.social_players.is_empty() {
        let n = st.social_players.iter().filter(|p| p.online).count();
        let label = if n > 1 {
            f.lang.fmt(
                "gui.socialInteractions.server_label.multiple",
                &[&s.server, &n.to_string()],
            )
        } else {
            f.lang.fmt(
                "gui.socialInteractions.server_label.single",
                &[&s.server, &n.to_string()],
            )
        };
        f.t.draw(&label, margin_x + 8.0, 35.0, WHITE, true);
    }
    // Вкладки.
    let bw = (ROW_W / 3.0).floor();
    let tabs = [
        ("gui.socialInteractions.tab_all", row_left),
        (
            "gui.socialInteractions.tab_hidden",
            ((row_left + row_right - bw) / 2.0).floor() + 1.0,
        ),
        ("gui.socialInteractions.tab_blocked", row_right - bw + 1.0),
    ];
    for (i, (key, x)) in tabs.iter().enumerate() {
        let mut label = f.lang.tr(key);
        if s.page as usize == i {
            label = format!("§n{label}§r");
        }
        if f.button(*x, 45.0, bw, 20.0, &label, true) {
            s.page = i as u8;
            s.scroll = 0.0;
        }
    }
    // Поиск.
    let caret = (now_ms / 300) % 2 == 0;
    f.edit_box(
        margin_x + 28.0,
        74.0,
        200.0,
        15.0,
        &mut s.search,
        true,
        caret,
    );
    if s.search.text.is_empty() {
        let hint = f.lang.tr("gui.socialInteractions.search_hint");
        f.t.draw(&hint, margin_x + 32.0, 77.0, 0x909090, true);
    }

    // Список (VANILLA: PlayerList#updatePlayerList: фильтр по подстроке, сортировка по имени).
    let q = s.search.text.to_lowercase();
    let mut rows: Vec<&Player> = st
        .social_players
        .iter()
        .filter(|p| match s.page {
            0 => p.online,
            1 => st.social_hidden.contains(&p.id),
            _ => false,
        })
        .filter(|p| q.is_empty() || p.name.to_lowercase().contains(&q))
        .collect();
    rows.sort_by_key(|p| p.name.to_lowercase());
    let (top, bottom) = (88.0, list_end);
    let total = rows.len() as f32 * ITEM_H;
    s.scroll = (s.scroll - wheel * 18.0).clamp(0.0, (total - (bottom - top)).max(0.0));
    let mut toggles = Vec::new();
    for (i, p) in rows.iter().enumerate() {
        let y = top + i as f32 * ITEM_H - s.scroll;
        // VANILLA-UNSURE: ваниль режет строки ножницами; мы рисуем только целиком влезающие.
        if y < top || y + ITEM_H > bottom {
            continue;
        }
        let hidden = st.social_hidden.contains(&p.id);
        let removed = !p.online;
        let fill = if removed { 0xFF30_3030 } else { 0xFF4A_4A4A };
        f.p.blit(
            "white",
            row_left,
            y,
            ROW_W,
            ITEM_H - 4.0,
            [0.0, 0.0, 1.0, 1.0],
            fill,
        );
        let (sx, sy) = (row_left + 4.0, y + (ITEM_H - 4.0 - 24.0) / 2.0);
        // VANILLA: PlayerFaceExtractor — лицо (8,8) и слой шапки (40,8), 8×8 из 64×64.
        // VANILLA-UNSURE: скины с сервера сессий не грузятся — везде Стив, как и в мире.
        for u in [8.0, 40.0] {
            f.p.blit(
                "soc_steve",
                sx,
                sy,
                24.0,
                24.0,
                [u / 64.0, 8.0 / 64.0, (u + 8.0) / 64.0, 16.0 / 64.0],
                0xFFFFFFFF,
            );
        }
        if removed {
            f.p.blit(
                "white",
                sx,
                sy,
                24.0,
                24.0,
                [0.0, 0.0, 1.0, 1.0],
                0xBE00_0000,
            );
        }
        let tx = sx + 28.0;
        let status = match (hidden, p.online) {
            (true, true) => Some("gui.socialInteractions.status_hidden"),
            (true, false) => Some("gui.socialInteractions.status_hidden_offline"),
            (false, false) => Some("gui.socialInteractions.status_offline"),
            _ => None,
        };
        let mut ty = y + (ITEM_H - 4.0 - 9.0) / 2.0;
        if let Some(k) = status {
            ty = y + (ITEM_H - 4.0 - 18.0) / 2.0;
            f.t.draw(
                &format!("§o{}", f.lang.tr(k)),
                tx,
                ty + 12.0,
                0x8C8C8C,
                true,
            );
        }
        f.t.draw(&p.name, tx, ty, WHITE, true);
        if p.id != s.me {
            let (bx, by) = (row_right - 24.0, y + (ITEM_H - 4.0 - 20.0) / 2.0);
            let hov = f.hover(bx, by, 20.0, 20.0);
            let tex = match (hidden, hov) {
                (false, false) => "soc_mute",
                (false, true) => "soc_mute_hl",
                (true, false) => "soc_unmute",
                (true, true) => "soc_unmute_hl",
            };
            f.p.blit(tex, bx, by, 20.0, 20.0, [0.0, 0.0, 1.0, 1.0], 0xFFFFFFFF);
            if hov {
                crate::cursor::request(crate::cursor::Kind::PointingHand);
                if f.click {
                    f.click = false;
                    crate::audio::play_ui("ui.button.click", crate::audio::Cat::Master, 0.25, 1.0);
                    toggles.push((p.id, !hidden, p.name.clone()));
                }
            }
        }
    }
    st.social_toggle.extend(toggles);
    if rows.is_empty() {
        let key = if !q.is_empty() {
            Some("gui.socialInteractions.search_empty")
        } else if s.page == 1 {
            Some("gui.socialInteractions.empty_hidden")
        } else if s.page == 2 {
            Some("gui.socialInteractions.empty_blocked")
        } else {
            None
        };
        if let Some(k) = key {
            f.center(k, (72.0 + list_end) / 2.0, 0xAAAAAA);
        }
    }
    // Готово (HeaderAndFooterLayout: футер 33 px).
    if f.button(
        (f.w / 2.0).floor() - 100.0,
        f.h - 26.0,
        200.0,
        20.0,
        &f.lang.tr("gui.done"),
        true,
    ) {
        close = true;
    }
    if close {
        *action = Action::CloseSocial;
    } else {
        st.social = Some(s);
    }
}
