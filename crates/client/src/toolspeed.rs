//! Скорость ломания блока с инструментом, эффектами и чарами (TODO-195).
//! VANILLA: Player#getDestroySpeed, Tool#getMiningSpeed/isCorrectForDrops, MobEffectUtil#hasDigSpeed.
//! Правила `tool` берутся из `data/tools.txt` (tools/gen_tools.py): патч компонентов несёт только отличия.
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

struct Rule {
    speed: Option<f32>,
    correct: Option<bool>,
    /// `#тег` или имя блока.
    blocks: &'static str,
}

struct Tool {
    default_speed: f32,
    rules: Vec<Rule>,
}

struct Table {
    tags: HashMap<&'static str, HashSet<&'static str>>,
    tools: HashMap<&'static str, Tool>,
}

static TABLE: LazyLock<Table> = LazyLock::new(|| {
    let mut t = Table {
        tags: HashMap::new(),
        tools: HashMap::new(),
    };
    let mut cur: Option<&'static str> = None;
    for line in include_str!("../data/tools.txt").lines() {
        let p: Vec<&'static str> = line.split(' ').collect();
        match p.as_slice() {
            ["tag", n, bs] => {
                t.tags.insert(n, bs.split(',').collect());
            }
            ["tool", n, d] => {
                t.tools.insert(
                    n,
                    Tool {
                        default_speed: d.parse().unwrap_or(1.0),
                        rules: Vec::new(),
                    },
                );
                cur = Some(n);
            }
            ["rule", sp, cd, bl] => {
                if let Some(tool) = cur.and_then(|c| t.tools.get_mut(c)) {
                    tool.rules.push(Rule {
                        speed: sp.parse().ok(),
                        correct: match *cd {
                            "1" => Some(true),
                            "0" => Some(false),
                            _ => None,
                        },
                        blocks: bl,
                    });
                }
            }
            _ => {}
        }
    }
    t
});

impl Table {
    fn matches(&self, rule: &Rule, block: &str) -> bool {
        match rule.blocks.strip_prefix('#') {
            Some(tag) => self.tags.get(tag).is_some_and(|s| s.contains(block)),
            None => rule.blocks == block,
        }
    }
}

/// VANILLA: Tool#getMiningSpeed. Без компонента `tool` — 1.0 (руки/обычный предмет).
pub fn mining_speed(item: &str, block: &str) -> f32 {
    let Some(tool) = TABLE.tools.get(item) else {
        return 1.0;
    };
    tool.rules
        .iter()
        .find(|r| r.speed.is_some() && TABLE.matches(r, block))
        .and_then(|r| r.speed)
        .unwrap_or(tool.default_speed)
}

/// VANILLA: Tool#isCorrectForDrops (первое правило с `correct_for_drops`, иначе false).
pub fn correct_for_drops(item: &str, block: &str) -> bool {
    TABLE.tools.get(item).is_some_and(|tool| {
        tool.rules
            .iter()
            .find(|r| r.correct.is_some() && TABLE.matches(r, block))
            .and_then(|r| r.correct)
            .unwrap_or(false)
    })
}

/// Всё, что влияет на скорость ломания, кроме блока и земли.
#[derive(Clone, Copy, Debug)]
pub struct Gear {
    /// Предмет в основной руке ("" — пусто).
    pub tool: &'static str,
    /// Атрибут mining_efficiency (Efficiency: L²+1); добавляется, если скорость инструмента > 1.
    pub efficiency: f32,
    /// Атрибут block_break_speed (умолчание 1.0).
    pub break_speed: f32,
    /// Атрибут submerged_mining_speed (0.2; с Aqua Affinity 1.0).
    pub submerged: f32,
    /// Усиление Haste/Conduit Power (max amplifier + 1), 0 — нет.
    pub haste: i32,
    /// Mining Fatigue: amplifier + 1, 0 — нет.
    pub fatigue: i32,
}

impl Default for Gear {
    fn default() -> Self {
        Gear {
            tool: "",
            efficiency: 0.0,
            break_speed: 1.0,
            submerged: 0.2,
            haste: 0,
            fatigue: 0,
        }
    }
}

/// Значение mining_efficiency от чар по умолчанию (levels_squared, added 1.0).
pub fn efficiency_attr(level: i32) -> f32 {
    if level > 0 {
        (level * level + 1) as f32
    } else {
        0.0
    }
}

/// VANILLA: Player#getDestroySpeed.
pub fn destroy_speed(g: Gear, block: &str, on_ground: bool, eye_in_water: bool) -> f32 {
    let mut speed = mining_speed(g.tool, block);
    if speed > 1.0 {
        speed += g.efficiency;
    }
    if g.haste > 0 {
        speed *= 1.0 + g.haste as f32 * 0.2;
    }
    if g.fatigue > 0 {
        speed *= 0.3f64.powi(g.fatigue) as f32;
    }
    speed *= g.break_speed;
    if eye_in_water {
        speed *= g.submerged;
    }
    if !on_ground {
        speed /= 5.0;
    }
    speed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pickaxe_rules() {
        assert_eq!(mining_speed("iron_pickaxe", "stone"), 6.0);
        assert_eq!(mining_speed("iron_pickaxe", "dirt"), 1.0);
        assert!(correct_for_drops("iron_pickaxe", "diamond_ore"));
        assert!(!correct_for_drops("stone_pickaxe", "obsidian"));
        assert!(!correct_for_drops("iron_pickaxe", "oak_log"));
        assert_eq!(mining_speed("shears", "cobweb"), 15.0);
    }

    #[test]
    fn effects_scale_speed() {
        let g = Gear {
            tool: "iron_pickaxe",
            efficiency: efficiency_attr(3),
            haste: 2,
            ..Default::default()
        };
        // (6 + 10) * 1.4
        assert!((destroy_speed(g, "stone", true, false) - 22.4).abs() < 1e-4);
        let f = Gear {
            fatigue: 1,
            ..Default::default()
        };
        assert!((destroy_speed(f, "stone", true, false) - 0.3).abs() < 1e-6);
        // вода без Aqua Affinity ×0.2, в воздухе ÷5
        let t = Gear::default();
        assert!((destroy_speed(t, "stone", false, true) - 0.04).abs() < 1e-6);
        let a = Gear {
            submerged: 1.0,
            ..Default::default()
        };
        assert_eq!(destroy_speed(a, "stone", true, true), 1.0);
    }
}
