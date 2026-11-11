//! Ограничения режима приключения: компоненты `can_break` / `can_place_on` (AdventureModePredicate).
//! VANILLA: Player#blockActionRestricted, ItemStack#canBreakBlockInAdventureMode / canPlaceOnBlockInAdventureMode,
//! AdventureModePredicate#test, BlockPredicate#matches.
//! VANILLA-UNSURE: условия `nbt` (нужен блок-сущность) и `components` не проверяются (считаем совпавшими);
//! при ошибке разбора клиент разрешает действие — решение за сервером.
use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
use ezzmc_protocol::item::ComponentPatch;
use ezzmc_protocol::nbt;
use ezzmc_protocol::wire::Reader;
use ezzmc_world::block::{in_tag, BlockState};
use ezzmc_world::generated::t26_3::BLOCK_TAGS;

enum Set {
    Tag(String),
    Ids(Vec<u32>),
}

enum Value {
    Exact(String),
    Range(Option<String>, Option<String>),
}

struct Pred {
    blocks: Option<Set>,
    props: Option<Vec<(String, Value)>>,
}

fn parse(b: &[u8]) -> Option<Vec<Pred>> {
    let mut r = Reader::new(b);
    let n = r.var_int().ok()?;
    let mut out = Vec::new();
    for _ in 0..n {
        let blocks = if r.bool().ok()? {
            Some(match r.var_int().ok()? {
                0 => Set::Tag(r.string(32767).ok()?),
                k => {
                    let mut ids = Vec::new();
                    for _ in 1..k {
                        ids.push(r.var_int().ok()? as u32);
                    }
                    Set::Ids(ids)
                }
            })
        } else {
            None
        };
        let props = if r.bool().ok()? {
            let k = r.var_int().ok()?;
            let mut v = Vec::new();
            for _ in 0..k {
                let name = r.string(1024).ok()?;
                let val = if r.bool().ok()? {
                    Value::Exact(r.string(1024).ok()?)
                } else {
                    let min = if r.bool().ok()? {
                        Some(r.string(1024).ok()?)
                    } else {
                        None
                    };
                    let max = if r.bool().ok()? {
                        Some(r.string(1024).ok()?)
                    } else {
                        None
                    };
                    Value::Range(min, max)
                };
                v.push((name, val));
            }
            Some(v)
        } else {
            None
        };
        if r.bool().ok()? {
            nbt::read_network(&mut r).ok()?;
        }
        // DataComponentMatchers: точные и частичные предикаты разбирать нечем — отдаём решение серверу.
        if r.var_int().ok()? != 0 || r.var_int().ok()? != 0 {
            return None;
        }
        out.push(Pred { blocks, props });
    }
    Some(out)
}

fn cmp_val(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.parse::<i64>(), b.parse::<i64>()) {
        (Ok(x), Ok(y)) => x.cmp(&y),
        _ => a.cmp(b),
    }
}

fn matches(p: &Pred, s: BlockState) -> bool {
    let id_ok = match &p.blocks {
        None => true,
        Some(Set::Tag(t)) => in_tag(
            BLOCK_TAGS,
            t.strip_prefix("minecraft:").unwrap_or(t),
            s.block_id() as u32,
        ),
        Some(Set::Ids(ids)) => ids.contains(&(s.block_id() as u32)),
    };
    id_ok
        && p.props.as_ref().is_none_or(|v| {
            v.iter().all(|(name, val)| {
                let Some(cur) = s.property(name) else {
                    return false;
                };
                match val {
                    Value::Exact(x) => cur == x,
                    Value::Range(lo, hi) => {
                        lo.as_ref().is_none_or(|l| cmp_val(cur, l).is_ge())
                            && hi.as_ref().is_none_or(|h| cmp_val(cur, h).is_le())
                    }
                }
            })
        })
}

/// Разрешает ли предмет действие над блоком в режиме приключения: `component` — `can_break`/`can_place_on`.
/// Нет компонента → запрещено (VANILLA: AdventureModePredicate null → false).
pub fn allows(patch: &ComponentPatch, component: &str, state: BlockState) -> bool {
    let Some(id) = DATA_COMPONENT_TYPE.iter().position(|x| *x == component) else {
        return true;
    };
    let Some((_, bytes)) = patch.added.iter().find(|(i, _)| *i == id as u32) else {
        return false;
    };
    parse(bytes).is_none_or(|ps| ps.iter().any(|p| matches(p, state)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    fn state(name: &str) -> BlockState {
        let info = ezzmc_world::generated::b26_3::BLOCKS
            .iter()
            .find(|b| b.name == name)
            .expect("block");
        BlockState::from_id(info.default_state).expect("state")
    }

    fn patch(component: &str, bytes: Vec<u8>) -> ComponentPatch {
        let id = DATA_COMPONENT_TYPE
            .iter()
            .position(|x| *x == component)
            .expect("component") as u32;
        ComponentPatch {
            added: vec![(id, bytes.into_boxed_slice())],
            removed: vec![],
        }
    }

    fn ids_pred(ids: &[u32]) -> Vec<u8> {
        let mut w = Writer::new();
        w.var_int(1).bool(true).var_int(ids.len() as i32 + 1);
        for i in ids {
            w.var_int(*i as i32);
        }
        w.bool(false).bool(false).var_int(0).var_int(0);
        w.into_vec()
    }

    #[test]
    fn missing_component_forbids() {
        let p = ComponentPatch::default();
        assert!(!allows(&p, "can_break", state("stone")));
    }

    #[test]
    fn id_list_matches() {
        let stone = state("stone");
        let p = patch("can_break", ids_pred(&[stone.block_id() as u32]));
        assert!(allows(&p, "can_break", stone));
        assert!(!allows(&p, "can_break", state("dirt")));
    }

    #[test]
    fn tag_matches() {
        let mut w = Writer::new();
        w.var_int(1)
            .bool(true)
            .var_int(0)
            .string("minecraft:mineable/pickaxe");
        w.bool(false).bool(false).var_int(0).var_int(0);
        let p = patch("can_place_on", w.into_vec());
        assert!(allows(&p, "can_place_on", state("stone")));
        assert!(!allows(&p, "can_place_on", state("dirt")));
    }

    #[test]
    fn property_exact() {
        let mut w = Writer::new();
        w.var_int(1).bool(false).bool(true).var_int(1);
        w.string("axis").bool(true).string("y");
        w.bool(false).var_int(0).var_int(0);
        let p = patch("can_break", w.into_vec());
        let log = state("oak_log");
        assert!(allows(
            &p,
            "can_break",
            log.with("axis", "y").expect("axis")
        ));
        assert!(!allows(
            &p,
            "can_break",
            log.with("axis", "x").expect("axis")
        ));
        assert!(!allows(&p, "can_break", state("stone")));
    }

    #[test]
    fn garbage_is_permissive() {
        let p = patch("can_break", vec![0xFF, 0xFF]);
        assert!(allows(&p, "can_break", state("stone")));
    }
}
