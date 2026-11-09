//! Кулдауны предметов. VANILLA: ItemCooldowns (+ ClientboundCooldownPacket, GuiGraphicsExtractor#itemCooldown).
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

struct State {
    map: HashMap<String, (i64, i64)>,
    tick: i64,
    last: Instant,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> Option<R> {
    let mut g = STATE.lock().ok()?;
    let s = g.get_or_insert_with(|| State {
        map: HashMap::new(),
        tick: 0,
        last: Instant::now(),
    });
    Some(f(s))
}

/// VANILLA: ItemCooldowns#addCooldown / removeCooldown (duration 0 — снять).
pub fn add(group: &str, duration: i32) {
    let g = group.strip_prefix("minecraft:").unwrap_or(group).to_owned();
    with(|s| {
        if duration <= 0 {
            s.map.remove(&g);
        } else {
            s.map.insert(g, (s.tick, s.tick + i64::from(duration)));
        }
    });
}

/// VANILLA: ItemCooldowns#tick (раз в клиентский тик).
pub fn tick() {
    with(|s| {
        s.tick += 1;
        s.last = Instant::now();
        let t = s.tick;
        s.map.retain(|_, (_, end)| *end > t);
    });
}

/// VANILLA: ItemCooldowns#getCooldownPercent (группа по умолчанию — id предмета).
pub fn percent(item: &str) -> f32 {
    with(|s| {
        let &(start, end) = s.map.get(item)?;
        let a = s.last.elapsed().as_secs_f32() * 20.0;
        let a = a.min(1.0);
        let rem = end as f32 - (s.tick as f32 + a);
        Some((rem / (end - start) as f32).clamp(0.0, 1.0))
    })
    .flatten()
    .unwrap_or(0.0)
}
