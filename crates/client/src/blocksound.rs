//! Звуки блоков по SoundType. VANILLA: SoundType, BlockBehaviour.Properties#sound (таблицы сгенерированы
//! `tools/gen_block_sounds.py` из Blocks.java/SoundType.java 26.3; по умолчанию STONE).
use crate::audio::{self, Cat};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct SoundType {
    pub volume: f32,
    pub pitch: f32,
    pub brk: String,
    pub step: String,
    pub place: String,
    pub hit: String,
    pub fall: String,
}

struct Tables {
    types: HashMap<&'static str, SoundType>,
    by_block: HashMap<&'static str, &'static str>,
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mut types = HashMap::new();
        for l in include_str!("../data/sound_types.txt").lines() {
            let f: Vec<&str> = l.split(' ').collect();
            if f.len() < 8 {
                continue;
            }
            let ev = |s: &str| {
                if s == "-" {
                    String::new()
                } else {
                    s.to_owned()
                }
            };
            types.insert(
                f[0],
                SoundType {
                    volume: f[1].parse().unwrap_or(1.0),
                    pitch: f[2].parse().unwrap_or(1.0),
                    brk: ev(f[3]),
                    step: ev(f[4]),
                    place: ev(f[5]),
                    hit: ev(f[6]),
                    fall: ev(f[7]),
                },
            );
        }
        let by_block = include_str!("../data/block_sounds.txt")
            .lines()
            .filter_map(|l| l.split_once(' '))
            .collect();
        Tables { types, by_block }
    })
}

/// SoundType блока по имени (`stone`, без `minecraft:`).
pub fn of(block: &str) -> &'static SoundType {
    let t = tables();
    let name = block.strip_prefix("minecraft:").unwrap_or(block);
    let ty = t.by_block.get(name).copied().unwrap_or("STONE");
    // SAFETY-free: STONE есть всегда.
    t.types
        .get(ty)
        .or_else(|| t.types.get("STONE"))
        .expect("STONE")
}

fn play(ev: &str, vol: f32, pitch: f32, p: [f64; 3], cat: Cat) {
    if !ev.is_empty() {
        audio::play_at(ev, cat, p[0], p[1], p[2], vol, pitch);
    }
}

fn center(x: i32, y: i32, z: i32) -> [f64; 3] {
    [f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5]
}

/// VANILLA: Level#playLocalSound(placeSound, volume (v+1)/2, pitch p·0.8) при установке блока.
pub fn place(block: &str, x: i32, y: i32, z: i32) {
    let st = of(block);
    play(
        &st.place,
        (st.volume + 1.0) / 2.0,
        st.pitch * 0.8,
        center(x, y, z),
        Cat::Blocks,
    );
}

/// VANILLA: LevelEvent.PARTICLES_DESTROY_BLOCK (2001): breakSound (v+1)/2, pitch p·0.8.
pub fn brk(block: &str, x: i32, y: i32, z: i32) {
    tracing::debug!(target: "dbg", "brk {block} at [{x},{y},{z}]");
    let st = of(block);
    play(
        &st.brk,
        (st.volume + 1.0) / 2.0,
        st.pitch * 0.8,
        center(x, y, z),
        Cat::Blocks,
    );
}

/// VANILLA: MultiPlayerGameMode#continueDestroyBlock → hitSound (v+1)/8, pitch p·0.5.
pub fn hit(block: &str, x: i32, y: i32, z: i32) {
    let st = of(block);
    play(
        &st.hit,
        (st.volume + 1.0) / 8.0,
        st.pitch * 0.5,
        center(x, y, z),
        Cat::Blocks,
    );
}

/// VANILLA: Entity#playStepSound: stepSound, volume v·0.15, pitch p.
pub fn step(block: &str, pos: [f64; 3], cat: Cat) {
    let st = of(block);
    play(&st.step, st.volume * 0.15, st.pitch, pos, cat);
}

/// VANILLA: Entity#playCombinedStepSounds (шаг блока под ногами + лёгкий шаг по ковру/снегу сверху).
pub fn fall(block: &str, big: bool, pos: [f64; 3], cat: Cat) {
    let st = of(block);
    let _ = big;
    play(&st.fall, st.volume * 0.5, st.pitch * 0.75, pos, cat);
}

/// Материал в именах событий BlockSetType/WoodType: `oak_door` → `wooden`, `crimson_*` → `nether_wood` и т.д.
fn use_material(name: &str, gate: bool) -> &'static str {
    let w = |n: &str| name.starts_with(n);
    if w("crimson") || w("warped") {
        "nether_wood"
    } else if w("bamboo") {
        "bamboo_wood"
    } else if w("cherry") {
        "cherry_wood"
    } else if name.contains("copper") {
        "copper"
    } else if w("iron") {
        "iron"
    } else if name.contains("stone") || w("polished_blackstone") {
        "stone"
    } else if gate {
        ""
    } else {
        "wooden"
    }
}

fn rand01() -> f32 {
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    (n.wrapping_mul(2_654_435_761) >> 8 & 0xFFFF) as f32 / 65536.0
}

/// VANILLA: Door/TrapDoor/FenceGate#useWithoutItem (клиент играет сам: сервер исключает игрока из рассылки),
/// ButtonBlock#playSound, LeverBlock#playSound. `state` — состояние ДО клика.
pub fn use_block(name: &str, state: ezzmc_world::block::BlockState, pos: [i32; 3]) {
    let on = |p: &str| state.property(p) == Some("true");
    let c = center(pos[0], pos[1], pos[2]);
    let pitch = rand01() * 0.1 + 0.9;
    let (ev, vol, pit) = if name.ends_with("_trapdoor") {
        let oc = if on("open") { "close" } else { "open" };
        (
            format!("block.{}_trapdoor.{oc}", use_material(name, false)),
            1.0,
            pitch,
        )
    } else if name.ends_with("_door") {
        let oc = if on("open") { "close" } else { "open" };
        (
            format!("block.{}_door.{oc}", use_material(name, false)),
            1.0,
            pitch,
        )
    } else if name.ends_with("_fence_gate") {
        let oc = if on("open") { "close" } else { "open" };
        let m = use_material(name, true);
        let ev = if m.is_empty() {
            format!("block.fence_gate.{oc}")
        } else {
            format!("block.{m}_fence_gate.{oc}")
        };
        (ev, 1.0, pitch)
    } else if name.ends_with("_button") {
        if on("powered") {
            return;
        }
        let m = use_material(name, false);
        (format!("block.{m}_button.click_on"), 0.3, 0.6)
    } else if name == "lever" {
        (
            "block.lever.click".to_owned(),
            0.3,
            if on("powered") { 0.5 } else { 0.6 },
        )
    } else {
        return;
    };
    play(&ev, vol, pit, c, Cat::Blocks);
}

#[cfg(test)]
mod tests {
    use super::of;

    #[test]
    fn helper_registered_blocks_use_base_or_set_type_sound() {
        // VANILLA: registerSlab/Stair(base) копируют звук базы; Door/Button/Sign — BlockSetType/WoodType.
        assert_eq!(of("acacia_slab").place, "block.wood.place");
        assert_eq!(of("bamboo_stairs").place, "block.bamboo_wood.place");
        assert_eq!(of("cherry_button").brk, "block.cherry_wood.break");
        assert_eq!(of("crimson_door").step, "block.nether_wood.step");
        assert_eq!(of("iron_door").step, "block.iron.step");
        assert_eq!(of("oak_hanging_sign").step, "block.hanging_sign.step");
        assert_eq!(of("oak_wall_sign").step, "block.wood.step");
        assert_eq!(of("stone_slab").step, "block.stone.step");
    }
}
