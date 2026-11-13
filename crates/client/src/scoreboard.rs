//! Клиентский scoreboard: цели, очки, слоты показа, команды + боковая панель (sidebar).
//! VANILLA: ClientPacketListener#handleAddObjective/handleSetScore/handleResetScore/
//! handleSetDisplayObjective/handleSetPlayerTeamPacket, Scoreboard, PlayerTeam, PlayerScoreEntry,
//! NumberFormatTypes (blank/styled/fixed), Hud#extractScoreboardSidebar/displayScoreboardSidebar (26.3).

use std::collections::HashMap;

use ezzmc_protocol::nbt;
use ezzmc_protocol::text::{Component, Style};
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::text::TextRenderer;

/// VANILLA: DisplaySlot — 0 list, 1 sidebar, 2 below_name, 3..18 sidebar.team.<цвет>.
pub const SLOT_LIST: i32 = 0;
pub const SLOT_SIDEBAR: i32 = 1;
pub const SLOT_BELOW_NAME: i32 = 2;
const SLOTS: usize = 19;

/// VANILLA: TeamColor (ChatFormatting 0..15) — RGB.
const TEAM_RGB: [u32; 16] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xFFAA00, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF,
];

/// VANILLA: NumberFormat.
#[derive(Debug, Clone, PartialEq)]
pub enum NumFmt {
    Blank,
    Styled(Style),
    Fixed(Component),
}

impl NumFmt {
    /// VANILLA: NumberFormat#format.
    pub fn format(&self, v: i32) -> Component {
        match self {
            NumFmt::Blank => Component::text(""),
            NumFmt::Styled(st) => Component {
                style: st.clone(),
                ..Component::text(v.to_string())
            },
            NumFmt::Fixed(c) => c.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Objective {
    pub display: Component,
    /// 0 integer, 1 hearts.
    pub render_type: i32,
    pub number_format: Option<NumFmt>,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub value: i32,
    pub display: Option<Component>,
    pub number_format: Option<NumFmt>,
}

#[derive(Debug, Clone, Default)]
pub struct Team {
    /// Имя команды, флаги (friendly fire / видеть невидимых союзников), правило толкания —
    /// хранятся по ванили, пока не используются (TODO-522).
    #[allow(dead_code)]
    pub display: Option<Component>,
    pub prefix: Option<Component>,
    pub suffix: Option<Component>,
    /// TeamColor id 0..15.
    pub color: Option<u8>,
    #[allow(dead_code)]
    pub options: u8,
    pub name_tag_visibility: i32,
    #[allow(dead_code)]
    pub collision_rule: i32,
    pub players: Vec<String>,
}

impl Team {
    /// VANILLA: PlayerTeam#getFormattedName — prefix + имя + suffix, цвет команды как базовый.
    pub fn format_name(&self, name: Component) -> Component {
        let mut siblings = Vec::with_capacity(3);
        siblings.extend(self.prefix.clone());
        siblings.push(name);
        siblings.extend(self.suffix.clone());
        let mut c = Component {
            siblings,
            ..Component::text("")
        };
        if let Some(col) = self.color {
            c.style.color = Some(TEAM_RGB[usize::from(col & 15)]);
        }
        c
    }
}

#[derive(Debug, Clone)]
pub enum ScoreEvt {
    Objective {
        name: String,
        method: i8,
        data: Option<Objective>,
    },
    Display(i32, String),
    Set {
        owner: String,
        objective: String,
        score: Score,
    },
    Reset {
        owner: String,
        objective: Option<String>,
    },
    Team {
        name: String,
        method: i8,
        params: Option<Box<Team>>,
        players: Vec<String>,
    },
}

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

/// VANILLA: NumberFormatTypes.OPTIONAL_STREAM_CODEC — bool, затем VarInt тип (blank/styled/fixed).
fn opt_num_fmt(r: &mut Reader) -> ProtocolResult<Option<NumFmt>> {
    if !r.bool()? {
        return Ok(None);
    }
    Ok(Some(match r.var_int()? {
        0 => NumFmt::Blank,
        1 => NumFmt::Styled(match nbt::read_network(r)? {
            Some(t) => Component::style_from_nbt(&t)?,
            None => Style::default(),
        }),
        _ => NumFmt::Fixed(component(r)?),
    }))
}

fn players(r: &mut Reader) -> ProtocolResult<Vec<String>> {
    let n = r.var_int()?.clamp(0, 65536);
    (0..n).map(|_| r.string(32767)).collect()
}

/// Разбор пакетов scoreboard; `None` — не наш пакет.
pub fn parse(name: &str, body: &[u8]) -> Option<ProtocolResult<ScoreEvt>> {
    let mut r = Reader::new(body);
    let r = &mut r;
    let ev = match name {
        "set_objective" => (|| {
            let name = r.string(32767)?;
            let method = r.i8()?;
            let data = if method == 0 || method == 2 {
                Some(Objective {
                    display: component(r)?,
                    render_type: r.var_int()?,
                    number_format: opt_num_fmt(r)?,
                })
            } else {
                None
            };
            Ok(ScoreEvt::Objective { name, method, data })
        })(),
        "set_display_objective" => (|| Ok(ScoreEvt::Display(r.var_int()?, r.string(32767)?)))(),
        "set_score" => (|| {
            let owner = r.string(32767)?;
            let objective = r.string(32767)?;
            let value = r.var_int()?;
            let display = if r.bool()? { Some(component(r)?) } else { None };
            let number_format = opt_num_fmt(r)?;
            Ok(ScoreEvt::Set {
                owner,
                objective,
                score: Score {
                    value,
                    display,
                    number_format,
                },
            })
        })(),
        "reset_score" => (|| {
            let owner = r.string(32767)?;
            let objective = if r.bool()? {
                Some(r.string(32767)?)
            } else {
                None
            };
            Ok(ScoreEvt::Reset { owner, objective })
        })(),
        "set_player_team" => (|| {
            let name = r.string(32767)?;
            let method = r.i8()?;
            let params = if method == 0 || method == 2 {
                let display = component(r)?;
                let prefix = component(r)?;
                let suffix = component(r)?;
                let name_tag_visibility = r.var_int()?;
                let collision_rule = r.var_int()?;
                let color = if r.bool()? {
                    Some(r.var_int()?.clamp(0, 15) as u8)
                } else {
                    None
                };
                let options = r.u8()?;
                Some(Box::new(Team {
                    display: Some(display),
                    prefix: Some(prefix),
                    suffix: Some(suffix),
                    color,
                    options,
                    name_tag_visibility,
                    collision_rule,
                    players: Vec::new(),
                }))
            } else {
                None
            };
            let players = if matches!(method, 0 | 3 | 4) {
                players(r)?
            } else {
                Vec::new()
            };
            Ok(ScoreEvt::Team {
                name,
                method,
                params,
                players,
            })
        })(),
        _ => return None,
    };
    Some(ev)
}

#[derive(Debug, Default)]
pub struct Scoreboard {
    pub objectives: HashMap<String, Objective>,
    /// Слот → имя цели.
    pub display: [Option<String>; SLOTS],
    /// Владелец → (цель → очки).
    pub scores: HashMap<String, HashMap<String, Score>>,
    pub teams: HashMap<String, Team>,
    pub team_of: HashMap<String, String>,
}

impl Scoreboard {
    pub fn apply(&mut self, e: ScoreEvt) {
        match e {
            ScoreEvt::Objective { name, method, data } => match (method, data) {
                // VANILLA: Scoreboard#addObjective (повтор в ванили — исключение; здесь перезапись).
                (0, Some(o)) => {
                    self.objectives.insert(name, o);
                }
                (1, _) => {
                    // VANILLA: Scoreboard#removeObjective — снять со слотов и стереть очки.
                    self.objectives.remove(&name);
                    for s in &mut self.display {
                        if s.as_deref() == Some(name.as_str()) {
                            *s = None;
                        }
                    }
                    for m in self.scores.values_mut() {
                        m.remove(&name);
                    }
                }
                (2, Some(o)) => {
                    if let Some(cur) = self.objectives.get_mut(&name) {
                        *cur = o;
                    }
                }
                _ => {}
            },
            ScoreEvt::Display(slot, name) => {
                // VANILLA: BY_ID с OutOfBoundsStrategy.ZERO; неизвестная цель → пустой слот.
                let slot = if (0..SLOTS as i32).contains(&slot) {
                    slot as usize
                } else {
                    0
                };
                self.display[slot] = self.objectives.contains_key(&name).then_some(name);
            }
            ScoreEvt::Set {
                owner,
                objective,
                score,
            } => {
                if self.objectives.contains_key(&objective) {
                    self.scores
                        .entry(owner)
                        .or_default()
                        .insert(objective, score);
                } else {
                    tracing::warn!("set_score: неизвестная цель {objective}");
                }
            }
            ScoreEvt::Reset { owner, objective } => match objective {
                None => {
                    self.scores.remove(&owner);
                }
                Some(o) => {
                    if let Some(m) = self.scores.get_mut(&owner) {
                        m.remove(&o);
                        if m.is_empty() {
                            self.scores.remove(&owner);
                        }
                    }
                }
            },
            ScoreEvt::Team {
                name,
                method,
                params,
                players,
            } => {
                if method == 0 {
                    self.teams.entry(name.clone()).or_default();
                } else if !self.teams.contains_key(&name) {
                    tracing::warn!("set_player_team: неизвестная команда {name}");
                    return;
                }
                if let (Some(p), Some(t)) = (params, self.teams.get_mut(&name)) {
                    let keep = std::mem::take(&mut t.players);
                    *t = *p;
                    t.players = keep;
                }
                match method {
                    0 | 3 => {
                        for pl in players {
                            // VANILLA: Scoreboard#addPlayerToTeam — из старой команды выводится.
                            if let Some(old) = self.team_of.remove(&pl) {
                                if let Some(t) = self.teams.get_mut(&old) {
                                    t.players.retain(|x| *x != pl);
                                }
                            }
                            if let Some(t) = self.teams.get_mut(&name) {
                                t.players.push(pl.clone());
                            }
                            self.team_of.insert(pl, name.clone());
                        }
                    }
                    4 => {
                        for pl in players {
                            if self.team_of.get(&pl) == Some(&name) {
                                self.team_of.remove(&pl);
                                if let Some(t) = self.teams.get_mut(&name) {
                                    t.players.retain(|x| *x != pl);
                                }
                            }
                        }
                    }
                    1 => {
                        if let Some(t) = self.teams.remove(&name) {
                            for pl in t.players {
                                self.team_of.remove(&pl);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn team_of(&self, player: &str) -> Option<&Team> {
        self.team_of.get(player).and_then(|t| self.teams.get(t))
    }

    /// VANILLA: Hud#extractScoreboardSidebar — цель слота цвета своей команды, иначе SIDEBAR.
    pub fn sidebar_objective(&self, own_name: &str) -> Option<(&str, &Objective)> {
        let team_slot = self
            .team_of(own_name)
            .and_then(|t| t.color)
            .and_then(|c| self.display[3 + usize::from(c & 15)].as_deref());
        let name = team_slot.or(self.display[SLOT_SIDEBAR as usize].as_deref())?;
        self.objectives
            .get_key_value(name)
            .map(|(k, o)| (k.as_str(), o))
    }

    /// Цель слота LIST (очки в Tab).
    pub fn list_objective(&self) -> Option<(&str, &Objective)> {
        let name = self.display[SLOT_LIST as usize].as_deref()?;
        self.objectives
            .get_key_value(name)
            .map(|(k, o)| (k.as_str(), o))
    }

    /// Очки игрока в Tab. VANILLA: PlayerScoreEntry#formatValue(numberFormatOrDefault(PLAYER_LIST_DEFAULT)).
    pub fn list_score(&self, objective: &str, player: &str) -> Option<Component> {
        let obj = self.objectives.get(objective)?;
        let s = self.scores.get(player)?.get(objective)?;
        let def = NumFmt::Styled(Style {
            color: Some(0xFFFF55),
            ..Style::default()
        });
        let f = s
            .number_format
            .as_ref()
            .or(obj.number_format.as_ref())
            .unwrap_or(&def);
        Some(f.format(s.value))
    }

    /// VANILLA: Entity#belowNameDisplay — «очки␠имя цели» (формат по умолчанию NO_STYLE).
    pub fn below_name(&self, holder: &str) -> Option<Component> {
        let name = self.display[SLOT_BELOW_NAME as usize].as_deref()?;
        let obj = self.objectives.get(name)?;
        let s = self.scores.get(holder)?.get(name)?;
        let def = NumFmt::Styled(Style::default());
        let f = s
            .number_format
            .as_ref()
            .or(obj.number_format.as_ref())
            .unwrap_or(&def);
        Some(Component {
            siblings: vec![f.format(s.value), Component::text(" "), obj.display.clone()],
            ..Component::text("")
        })
    }

    /// Строки боковой панели: (имя, очки) по SCORE_DISPLAY_ORDER, без `#…`, не более 15.
    pub fn sidebar_lines(&self, objective: &str) -> Vec<(Component, Component)> {
        let Some(obj) = self.objectives.get(objective) else {
            return Vec::new();
        };
        let mut v: Vec<(&String, &Score)> = self
            .scores
            .iter()
            .filter(|(o, _)| !o.starts_with('#'))
            .filter_map(|(o, m)| m.get(objective).map(|s| (o, s)))
            .collect();
        // VANILLA: SCORE_DISPLAY_ORDER — value по убыванию, затем owner без учёта регистра.
        v.sort_by(|a, b| b.1.value.cmp(&a.1.value).then_with(|| cmp_ci(a.0, b.0)));
        let def = NumFmt::Styled(Style {
            color: Some(0xFF5555),
            ..Style::default()
        });
        let ofmt = obj.number_format.as_ref().unwrap_or(&def);
        v.into_iter()
            .take(15)
            .map(|(owner, s)| {
                let n = s
                    .display
                    .clone()
                    .unwrap_or_else(|| Component::text(owner.clone()));
                let n = match self.team_of(owner) {
                    Some(t) => t.format_name(n),
                    None => n,
                };
                let sc = s.number_format.as_ref().unwrap_or(ofmt).format(s.value);
                (n, sc)
            })
            .collect()
    }

    /// VANILLA: Hud#displayScoreboardSidebar. Фон: getBackgroundColor(0.3/0.4) при
    /// backgroundForChatOnly=true — чёрный 30%/40%; текст без тени.
    pub fn draw_sidebar(
        &self,
        t: &mut TextRenderer,
        lang: &crate::ui::lang::Lang,
        own_name: &str,
        gw: f32,
        gh: f32,
    ) {
        let Some((key, obj)) = self.sidebar_objective(own_name) else {
            return;
        };
        use crate::titles::{draw_spans, spans_width};
        let fl = |c: &Component| crate::chat::flatten(c, lang, true);
        let rows: Vec<_> = self
            .sidebar_lines(key)
            .iter()
            .map(|(n, s)| {
                let (n, s) = (fl(n), fl(s));
                let sw = spans_width(t, &s) as i32;
                (n, s, sw)
            })
            .collect();
        let title = fl(&obj.display);
        let title_w = spans_width(t, &title) as i32;
        let spacer = t.text_width(": ") as i32;
        let mut width = title_w;
        for (n, _, sw) in &rows {
            let nw = spans_width(t, n) as i32;
            width = width.max(nw + if *sw > 0 { spacer + sw } else { 0 });
        }
        let (gw, gh) = (gw as i32, gh as i32);
        let count = rows.len() as i32;
        let height = count * 9;
        let bottom = gh / 2 + height / 3;
        let left = gw - width - 3;
        let right = gw - 3 + 2;
        let header_y = bottom - count * 9;
        let f = |x: i32| x as f32;
        t.fill(
            f(left - 2),
            f(header_y - 9 - 1),
            f(right),
            f(header_y - 1),
            0x66_000000,
        );
        t.fill(
            f(left - 2),
            f(header_y - 1),
            f(right),
            f(bottom),
            0x4C_000000,
        );
        draw_spans(
            t,
            &title,
            f(left + width / 2 - title_w / 2),
            f(header_y - 9),
            1.0,
            false,
        );
        for (i, (n, s, sw)) in rows.iter().enumerate() {
            let y = f(bottom - (count - i as i32) * 9);
            draw_spans(t, n, f(left), y, 1.0, false);
            draw_spans(t, s, f(right - sw), y, 1.0, false);
        }
    }
}

/// VANILLA: String.CASE_INSENSITIVE_ORDER (посимвольно, upper→lower).
fn cmp_ci(a: &str, b: &str) -> std::cmp::Ordering {
    let k = |c: char| {
        c.to_uppercase()
            .next()
            .unwrap_or(c)
            .to_lowercase()
            .next()
            .unwrap_or(c)
    };
    a.chars().map(k).cmp(b.chars().map(k))
}

/// Плоский текст компонента (тесты).
#[cfg(test)]
fn plain(c: &Component) -> String {
    use ezzmc_protocol::text::Content;
    let mut s = match &c.content {
        Content::Text(t) => t.clone(),
        _ => String::new(),
    };
    for x in &c.siblings {
        s.push_str(&plain(x));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    fn nbt_str(w: &mut Writer, s: &str) {
        // Сетевой NBT: тег String (8) без имени.
        w.u8(8).u16(s.len() as u16);
        w.bytes(s.as_bytes());
    }

    #[test]
    fn sidebar_order_teams_and_reset() {
        let mut sb = Scoreboard::default();
        let mut w = Writer::new();
        w.string("o").u8(0);
        nbt_str(&mut w, "Цель");
        w.var_int(0).bool(false);
        sb.apply(parse("set_objective", &w.into_vec()).unwrap().unwrap());
        let mut w = Writer::new();
        w.var_int(1).string("o");
        sb.apply(
            parse("set_display_objective", &w.into_vec())
                .unwrap()
                .unwrap(),
        );
        for (who, v) in [("bob", 3), ("Alice", 3), ("#hidden", 9), ("zed", 7)] {
            let mut w = Writer::new();
            w.string(who).string("o").var_int(v).bool(false).bool(false);
            sb.apply(parse("set_score", &w.into_vec()).unwrap().unwrap());
        }
        // Команда red с префиксом "[R]" и игроком bob.
        let mut w = Writer::new();
        w.string("t").u8(0);
        nbt_str(&mut w, "T");
        nbt_str(&mut w, "[R]");
        nbt_str(&mut w, "");
        w.var_int(0).var_int(0).bool(true).var_int(12).u8(0);
        w.var_int(1).string("bob");
        sb.apply(parse("set_player_team", &w.into_vec()).unwrap().unwrap());

        let (k, _) = sb.sidebar_objective("me").unwrap();
        let lines = sb.sidebar_lines(k);
        let names: Vec<String> = lines.iter().map(|(n, _)| plain(n)).collect();
        assert_eq!(names, ["zed", "Alice", "[R]bob"]);
        assert_eq!(lines[2].0.style.color, Some(0xFF5555));
        assert_eq!(plain(&lines[0].1), "7");
        assert_eq!(
            lines[0].1.style.color,
            Some(0xFF5555),
            "SIDEBAR_DEFAULT — красный"
        );

        let mut w = Writer::new();
        w.string("zed").bool(false);
        sb.apply(parse("reset_score", &w.into_vec()).unwrap().unwrap());
        assert_eq!(sb.sidebar_lines("o").len(), 2);

        // Удаление цели снимает её со слота.
        let mut w = Writer::new();
        w.string("o").u8(1);
        sb.apply(parse("set_objective", &w.into_vec()).unwrap().unwrap());
        assert!(sb.sidebar_objective("me").is_none());
    }
}
