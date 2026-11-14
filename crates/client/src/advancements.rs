//! Достижения: пакет update_advancements, дерево и прогресс.
//! VANILLA: ClientboundUpdateAdvancementsPacket, ClientAdvancements, AdvancementTree,
//! AdvancementProgress, AdvancementRequirements, DisplayInfo (26.3).
use std::collections::HashMap;

use ezzmc_protocol::item::{self, ItemStack};
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// VANILLA: AdvancementType (id 0..2; вне диапазона → TASK).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Task,
    Challenge,
    Goal,
}

impl Kind {
    /// Ключ `advancements.toast.<name>`.
    pub fn toast_key(self) -> &'static str {
        match self {
            Kind::Task => "advancements.toast.task",
            Kind::Challenge => "advancements.toast.challenge",
            Kind::Goal => "advancements.toast.goal",
        }
    }

    /// VANILLA: AdvancementType#getChatColor — GREEN / DARK_PURPLE / GREEN.
    pub fn chat_color(self) -> u32 {
        match self {
            Kind::Challenge => 0xAA00AA,
            _ => 0x55FF55,
        }
    }
}

/// VANILLA: DisplayInfo.
#[derive(Clone, Debug)]
pub struct Display {
    pub title: Component,
    pub description: Component,
    pub icon: ItemStack,
    pub kind: Kind,
    /// Идентификатор фона вкладки (`minecraft:gui/advancements/backgrounds/stone`).
    pub background: Option<String>,
    pub show_toast: bool,
    pub hidden: bool,
}

#[derive(Clone, Debug)]
pub struct Adv {
    pub id: String,
    pub parent: Option<String>,
    pub display: Option<Display>,
    /// VANILLA: AdvancementRequirements — И по внешнему списку, ИЛИ по внутреннему.
    pub requirements: Vec<Vec<String>>,
    pub x: f32,
    pub y: f32,
}

/// Разобранный `update_advancements`. Прогресс: критерий → время получения (мс эпохи).
#[derive(Debug)]
pub struct Update {
    pub reset: bool,
    pub added: Vec<Adv>,
    pub removed: Vec<String>,
    pub progress: Vec<(String, HashMap<String, Option<i64>>)>,
    pub show: bool,
}

fn component(r: &mut Reader) -> ProtocolResult<Component> {
    match nbt::read_network(r)? {
        Some(t) => Component::from_nbt(&t),
        None => Ok(Component::text("")),
    }
}

fn display(r: &mut Reader) -> ProtocolResult<Display> {
    let title = component(r)?;
    let description = component(r)?;
    let icon = item::read_template(r)?;
    let kind = match r.var_int()? {
        1 => Kind::Challenge,
        2 => Kind::Goal,
        _ => Kind::Task,
    };
    let flags = r.i32()?;
    let background = if flags & 1 != 0 {
        Some(r.string(32767)?)
    } else {
        None
    };
    Ok(Display {
        title,
        description,
        icon,
        kind,
        background,
        show_toast: flags & 2 != 0,
        hidden: flags & 4 != 0,
    })
}

pub fn parse(body: &[u8]) -> ProtocolResult<Update> {
    let mut r = Reader::new(body);
    let reset = r.bool()?;
    let n = r.length("advancements", 65536)?;
    let mut added = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        let id = r.string(32767)?;
        let parent = if r.bool()? {
            Some(r.string(32767)?)
        } else {
            None
        };
        let disp = if r.bool()? {
            Some(display(&mut r)?)
        } else {
            None
        };
        let outer = r.length("requirements", 65536)?;
        let mut requirements = Vec::with_capacity(outer.min(256));
        for _ in 0..outer {
            let inner = r.length("requirement", 65536)?;
            let mut set = Vec::with_capacity(inner.min(256));
            for _ in 0..inner {
                set.push(r.string(32767)?);
            }
            requirements.push(set);
        }
        let _telemetry = r.bool()?;
        let (x, y) = (r.f32()?, r.f32()?);
        added.push(Adv {
            id,
            parent,
            display: disp,
            requirements,
            x,
            y,
        });
    }
    let n = r.length("removed", 65536)?;
    let mut removed = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        removed.push(r.string(32767)?);
    }
    let n = r.length("progress", 65536)?;
    let mut progress = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        let id = r.string(32767)?;
        let m = r.length("criteria", 65536)?;
        let mut crit = HashMap::with_capacity(m.min(256));
        for _ in 0..m {
            let name = r.string(32767)?;
            let at = if r.bool()? { Some(r.i64()?) } else { None };
            crit.insert(name, at);
        }
        progress.push((id, crit));
    }
    let show = r.bool()?;
    Ok(Update {
        reset,
        added,
        removed,
        progress,
        show,
    })
}

/// `select_advancements_tab`: Optional<Identifier>.
pub fn parse_select(body: &[u8]) -> ProtocolResult<Option<String>> {
    let mut r = Reader::new(body);
    Ok(if r.bool()? {
        Some(r.string(32767)?)
    } else {
        None
    })
}

/// VANILLA: ClientAdvancements — дерево в порядке вставки + прогресс.
#[derive(Default)]
pub struct Advancements {
    pub list: Vec<Adv>,
    progress: HashMap<String, HashMap<String, Option<i64>>>,
    /// VANILLA: ClientAdvancements#selectedTab (id корня).
    pub selected: Option<String>,
    /// Растёт при любом изменении (экран пересобирает вкладки).
    pub version: u64,
}

impl Advancements {
    pub fn get(&self, id: &str) -> Option<&Adv> {
        self.list.iter().find(|a| a.id == id)
    }

    /// VANILLA: ClientAdvancements#update. Возвращает id достижений для toast'ов.
    pub fn apply(&mut self, u: Update) -> Vec<String> {
        self.version += 1;
        if u.reset {
            self.list.clear();
            self.progress.clear();
        }
        // VANILLA: AdvancementTree#remove — узел вместе с потомками.
        let mut gone: Vec<String> = u.removed;
        while let Some(id) = gone.pop() {
            gone.extend(
                self.list
                    .iter()
                    .filter(|a| a.parent.as_deref() == Some(id.as_str()))
                    .map(|a| a.id.clone()),
            );
            self.list.retain(|a| a.id != id);
            self.progress.remove(&id);
        }
        // VANILLA: AdvancementTree#addAll — пока вставляется хоть что-то (родитель уже в дереве).
        let mut todo = u.added;
        loop {
            let before = todo.len();
            let mut rest = Vec::new();
            for a in todo {
                let ok = a.parent.as_deref().is_none_or(|p| self.get(p).is_some());
                if !ok {
                    rest.push(a);
                    continue;
                }
                // Map#put: повторный id заменяет запись, место в порядке сохраняется.
                match self.list.iter_mut().find(|o| o.id == a.id) {
                    Some(o) => *o = a,
                    None => self.list.push(a),
                }
            }
            todo = rest;
            if todo.is_empty() || todo.len() == before {
                break;
            }
        }
        if !todo.is_empty() {
            tracing::warn!("достижения: {} без родителя в дереве", todo.len());
        }
        let mut toasts = Vec::new();
        for (id, mut crit) in u.progress {
            let Some(a) = self.get(&id) else {
                tracing::debug!("прогресс неизвестного достижения {id}");
                continue;
            };
            // VANILLA: AdvancementProgress#update(requirements).
            crit.retain(|k, _| a.requirements.iter().flatten().any(|n| n == k));
            for n in a.requirements.iter().flatten() {
                crit.entry(n.clone()).or_insert(None);
            }
            let toast = a.display.as_ref().is_some_and(|d| d.show_toast);
            self.progress.insert(id.clone(), crit);
            if !u.reset && self.is_done(&id) && u.show && toast {
                toasts.push(id);
            }
        }
        toasts
    }

    /// VANILLA: AdvancementProgress#countCompletedRequirements.
    fn completed(&self, a: &Adv) -> usize {
        let Some(p) = self.progress.get(&a.id) else {
            return 0;
        };
        a.requirements
            .iter()
            .filter(|set| set.iter().any(|c| p.get(c).is_some_and(|t| t.is_some())))
            .count()
    }

    /// VANILLA: AdvancementProgress#isDone (пустые требования — не выполнено).
    pub fn is_done(&self, id: &str) -> bool {
        self.get(id).is_some_and(|a| {
            !a.requirements.is_empty() && self.completed(a) == a.requirements.len()
        })
    }

    /// VANILLA: AdvancementProgress#getPercent (без прогресса — 0).
    pub fn percent(&self, id: &str) -> f32 {
        let Some(a) = self.get(id) else { return 0.0 };
        if self.progress.get(id).is_none_or(|p| p.is_empty()) || a.requirements.is_empty() {
            return 0.0;
        }
        self.completed(a) as f32 / a.requirements.len() as f32
    }

    /// VANILLA: AdvancementProgress#getProgressText — (выполнено, всего) при всего > 1.
    pub fn progress_counts(&self, id: &str) -> Option<(usize, usize)> {
        let a = self.get(id)?;
        if self.progress.get(id).is_none_or(|p| p.is_empty()) || a.requirements.len() <= 1 {
            return None;
        }
        Some((self.completed(a), a.requirements.len()))
    }

    /// VANILLA: ClientPacketListener#handleSelectAdvancementsTab.
    pub fn select(&mut self, id: Option<String>) {
        let id = id.filter(|i| self.get(i).is_some());
        if self.selected != id {
            self.selected = id;
            self.version += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    fn adv(w: &mut Writer, id: &str, parent: Option<&str>, reqs: &[&[&str]]) {
        w.string(id);
        w.bool(parent.is_some());
        if let Some(p) = parent {
            w.string(p);
        }
        w.bool(false); // без display
        w.var_int(reqs.len() as i32);
        for set in reqs {
            w.var_int(set.len() as i32);
            for c in *set {
                w.string(c);
            }
        }
        w.bool(false);
        w.f32(1.0).f32(2.0);
    }

    #[test]
    fn parse_tree_and_progress() {
        let mut w = Writer::new();
        w.bool(true);
        w.var_int(2);
        // Потомок раньше родителя: addAll вставит со второго прохода.
        adv(&mut w, "a:child", Some("a:root"), &[&["x", "y"], &["z"]]);
        adv(&mut w, "a:root", None, &[&["r"]]);
        w.var_int(0);
        w.var_int(1);
        w.string("a:child");
        w.var_int(2);
        w.string("x").bool(true).i64(5);
        w.string("junk").bool(true).i64(6);
        w.bool(true);
        let u = parse(&w.into_vec()).unwrap();
        assert!(u.reset && u.show);
        let mut a = Advancements::default();
        assert!(a.apply(u).is_empty());
        assert_eq!(a.list[0].id, "a:root");
        assert_eq!((a.list[1].x, a.list[1].y), (1.0, 2.0));
        assert!(!a.is_done("a:child"));
        assert_eq!(a.progress_counts("a:child"), Some((1, 2)));
        assert_eq!(a.percent("a:child"), 0.5);
        assert_eq!(a.progress_counts("a:root"), None);
        // Удаление корня уносит потомков.
        let mut w = Writer::new();
        w.bool(false).var_int(0);
        w.var_int(1);
        w.string("a:root");
        w.var_int(0);
        w.bool(false);
        a.apply(parse(&w.into_vec()).unwrap());
        assert!(a.list.is_empty());
    }
}
