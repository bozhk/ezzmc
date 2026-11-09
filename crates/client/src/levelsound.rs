//! Звуки `level_event`. VANILLA: LevelEventHandler#levelEvent (26.3), звук-часть каждого case.
use crate::audio::{self, Cat};
use std::cell::Cell;

thread_local! { static RNG: Cell<u64> = const { Cell::new(0x2545_F491_4F6C_DD1D) }; }

pub fn rf() -> f32 {
    RNG.with(|r| {
        let mut x = r.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        r.set(x);
        (x >> 40) as f32 / (1u64 << 24) as f32
    })
}

#[derive(Clone, Copy)]
enum P {
    Fixed(f32),
    /// (r−r)·0.2+1
    Mob,
    /// r·0.1+0.9
    Anvil,
    /// 2.6+(r−r)·0.8
    Extinguish,
}

fn pitch(p: P) -> f32 {
    match p {
        P::Fixed(v) => v,
        P::Mob => (rf() - rf()) * 0.2 + 1.0,
        P::Anvil => rf() * 0.1 + 0.9,
        P::Extinguish => 2.6 + (rf() - rf()) * 0.8,
    }
}

use Cat::{Blocks as B, Hostile as H, Neutral as N};

fn entry(ty: i32) -> Option<(&'static str, Cat, f32, P)> {
    Some(match ty {
        1000 => ("block.dispenser.dispense", B, 1.0, P::Fixed(1.0)),
        1001 => ("block.dispenser.fail", B, 1.0, P::Fixed(1.2)),
        1002 => ("block.dispenser.launch", B, 1.0, P::Fixed(1.2)),
        1004 => ("entity.firework_rocket.shoot", N, 1.0, P::Fixed(1.2)),
        1015 => ("entity.ghast.warn", H, 10.0, P::Mob),
        1016 => ("entity.ghast.shoot", H, 10.0, P::Mob),
        1017 => ("entity.ender_dragon.shoot", H, 10.0, P::Mob),
        1018 => ("entity.blaze.shoot", H, 2.0, P::Mob),
        1019 => ("entity.zombie.attack_wooden_door", H, 2.0, P::Mob),
        1020 => ("entity.zombie.attack_iron_door", H, 2.0, P::Mob),
        1021 => ("entity.zombie.break_wooden_door", H, 2.0, P::Mob),
        1022 => ("entity.wither.break_block", H, 2.0, P::Mob),
        1024 => ("entity.wither.shoot", H, 2.0, P::Mob),
        1025 => ("entity.bat.takeoff", N, 0.05, P::Mob),
        1026 => ("entity.zombie.infect", H, 2.0, P::Mob),
        1027 => ("entity.zombie_villager.converted", H, 2.0, P::Mob),
        1029 => ("block.anvil.destroy", B, 1.0, P::Anvil),
        1030 => ("block.anvil.use", B, 1.0, P::Anvil),
        1031 => ("block.anvil.land", B, 0.3, P::Anvil),
        1033 => ("block.chorus_flower.grow", B, 1.0, P::Fixed(1.0)),
        1034 => ("block.chorus_flower.death", B, 1.0, P::Fixed(1.0)),
        1035 => ("block.brewing_stand.brew", B, 1.0, P::Fixed(1.0)),
        1039 => ("entity.phantom.bite", H, 0.3, P::Anvil),
        1040 => ("entity.zombie.converted_to_drowned", H, 2.0, P::Mob),
        1041 => ("entity.husk.converted_to_zombie", H, 2.0, P::Mob),
        1042 => ("block.grindstone.use", B, 1.0, P::Anvil),
        1043 => ("item.book.page_turn", B, 1.0, P::Anvil),
        1044 => ("block.smithing_table.use", B, 1.0, P::Anvil),
        1045 => ("block.pointed_dripstone.land", B, 2.0, P::Anvil),
        1046 => (
            "block.pointed_dripstone.drip_lava_into_cauldron",
            B,
            2.0,
            P::Anvil,
        ),
        1047 => (
            "block.pointed_dripstone.drip_water_into_cauldron",
            B,
            2.0,
            P::Anvil,
        ),
        1048 => ("entity.skeleton.converted_to_stray", H, 2.0, P::Mob),
        1049 => ("block.crafter.craft", B, 1.0, P::Fixed(1.0)),
        1050 => ("block.crafter.fail", B, 1.0, P::Fixed(1.0)),
        1501 => ("block.lava.extinguish", B, 0.5, P::Extinguish),
        1502 => ("block.redstone_torch.burnout", B, 0.5, P::Extinguish),
        1503 => ("block.end_portal_frame.fill", B, 1.0, P::Fixed(1.0)),
        1505 => ("item.bone_meal.use", B, 1.0, P::Fixed(1.0)),
        2006 => ("entity.dragon_fireball.explode", H, 1.0, P::Anvil),
        _ => return None,
    })
}

/// Звук события; `data` нужен 1009 (тушение огня).
pub fn play(ty: i32, pos: [i32; 3], data: i32) {
    let c = [
        f64::from(pos[0]) + 0.5,
        f64::from(pos[1]) + 0.5,
        f64::from(pos[2]) + 0.5,
    ];
    let at = |ev: &str, cat, vol, p| audio::play_at(ev, cat, c[0], c[1], c[2], vol, p);
    match ty {
        1009 if data == 0 => at("block.fire.extinguish", B, 0.5, pitch(P::Extinguish)),
        1009 if data == 1 => at(
            "entity.generic.extinguish_fire",
            B,
            0.7,
            1.6 + (rf() - rf()) * 0.4,
        ),
        1032 => audio::play_ui("block.portal.travel", Cat::Ambient, 0.25, rf() * 0.4 + 0.8),
        1051 => at("entity.wind_charge.throw", B, 0.5, 0.4 / (rf() * 0.4 + 0.8)),
        1053 | 1054 => at("entity.splash_potion.break", N, 1.0, pitch(P::Anvil)),
        _ => {
            if let Some((ev, cat, vol, p)) = entry(ty) {
                at(ev, cat, vol, pitch(p));
            }
        }
    }
}

/// VANILLA: ClientPacketListener#handleTakeItemEntity — звук подбора.
pub fn pickup(pos: [f64; 3], xp: bool) {
    if xp {
        audio::play_at(
            "entity.experience_orb.pickup",
            Cat::Players,
            pos[0],
            pos[1],
            pos[2],
            0.1,
            (rf() - rf()) * 0.35 + 0.9,
        );
    } else {
        audio::play_at(
            "entity.item.pickup",
            Cat::Players,
            pos[0],
            pos[1],
            pos[2],
            0.2,
            (rf() - rf()) * 1.4 + 2.0,
        );
    }
}

/// VANILLA: LightningBolt#tick (клиент, life == 2) — гром (дальность 10000) и удар.
pub fn thunder(p: [f64; 3]) {
    audio::play_at(
        "entity.lightning_bolt.thunder",
        Cat::Weather,
        p[0],
        p[1],
        p[2],
        10000.0,
        0.8 + rf() * 0.2,
    );
    audio::play_at(
        "entity.lightning_bolt.impact",
        Cat::Weather,
        p[0],
        p[1],
        p[2],
        2.0,
        0.5 + rf() * 0.2,
    );
}
