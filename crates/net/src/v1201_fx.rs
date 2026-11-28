//! Звуки, частицы, события мира и блоков, взрывы серверов 1.12–1.20.1: тело в формате 1.20.1
//! (после цепочки конвертеров) → тело канона (26.3).
use ezzmc_protocol::generated::r26_3;
use ezzmc_protocol::version::{self, Version, V26_3};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;

/// Звуки, переименованные после версии сервера. Остальные отсутствующие в каноне удалены.
fn renamed_sound(name: &str) -> &str {
    match name {
        "entity.zombie_pigman.ambient" => "entity.zombified_piglin.ambient",
        "entity.zombie_pigman.angry" => "entity.zombified_piglin.angry",
        "entity.zombie_pigman.death" => "entity.zombified_piglin.death",
        "entity.zombie_pigman.hurt" => "entity.zombified_piglin.hurt",
        "entity.leash_knot.break" => "item.lead.untied",
        "entity.leash_knot.place" => "item.lead.tied",
        "item.sweet_berries.pick_from_bush" => "block.sweet_berry_bush.pick_berries",
        n => n,
    }
}

/// Имя звука версии сервера → имя и id реестра канона. Список 1.12.2 уже в именах 1.13.2.
fn canon_sound(version: &Version, id: usize) -> Option<(&'static str, Option<usize>)> {
    let name = renamed_sound(version.sound_event.get(id)?);
    Some((name, V26_3.sound_event.iter().position(|n| *n == name)))
}

/// VANILLA: ClientboundSoundPacket / SoundEntityPacket (1.20.1) — Holder<SoundEvent> (id + 1 или
/// 0 и прямая запись), дальше формат совпадает с каноном.
fn sound(version: &Version, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    if id == 0 {
        return Ok(Some(body.to_vec()));
    }
    let Some((name, canon)) = usize::try_from(id - 1)
        .ok()
        .and_then(|id| canon_sound(version, id))
    else {
        return Ok(None);
    };
    let mut w = Writer::with_capacity(body.len() + name.len());
    match canon {
        Some(i) => {
            w.var_int(i as i32 + 1);
        }
        // Звука с таким именем в каноне нет — прямая запись по имени.
        None => {
            w.var_int(0).string(name).bool(false);
        }
    }
    w.bytes(r.rest());
    Ok(Some(w.into_vec()))
}

/// Id состояния блока `barrier` канона (одно состояние по умолчанию).
fn barrier_state() -> Option<u32> {
    ezzmc_world::generated::b26_3::BLOCKS
        .iter()
        .find(|b| b.name.trim_start_matches("minecraft:") == "barrier")
        .map(|b| b.default_state)
}

fn particle_id(name: &str) -> Option<i32> {
    r26_3::PARTICLE_TYPE
        .iter()
        .position(|n| *n == name)
        .map(|i| i as i32)
}

/// Цвет `Vector3f` (0..1) → RGB.
fn rgb(r: &mut Reader) -> ProtocolResult<i32> {
    let c = |v: f32| (v.clamp(0.0, 1.0) * 255.0) as i32;
    let (x, y, z) = (r.f32()?, r.f32()?, r.f32()?);
    Ok(c(x) << 16 | c(y) << 8 | c(z))
}

/// VANILLA: ClientboundLevelParticlesPacket (1.20.1): тип, bool overrideLimiter, xyz double,
/// разброс float×3, float скорость, int счётчик, данные частицы → канон: тип с данными, два bool,
/// xyz, разброс, скорость по трём осям, varint счётчик, varint randomizationType.
/// Частицы `item` и `vibration` клиент канона не рисует — пакет отбрасывается.
fn particles(version: &Version, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let ty = r.var_int()?;
    let force = r.bool()?;
    let pos = r.bytes(24)?;
    let dist = r.bytes(12)?;
    let speed = r.f32()?;
    let count = r.i32()?;
    let Some(old) = usize::try_from(ty)
        .ok()
        .and_then(|i| version.particle_type.get(i))
        .copied()
    else {
        return Ok(None);
    };
    // VANILLA: 1.18 заменил `barrier` на block_marker; 1.20.5 убрал ambient_entity_effect.
    let name = match old {
        "barrier" => "block_marker",
        "ambient_entity_effect" => "entity_effect",
        n => n,
    };
    if matches!(name, "item" | "vibration") {
        return Ok(None);
    }
    let Some(id) = particle_id(name) else {
        return Ok(None);
    };
    let mut w = Writer::with_capacity(body.len() + 24);
    w.var_int(id);
    match name {
        "block_marker" if old == "barrier" => {
            let Some(state) = barrier_state() else {
                return Ok(None);
            };
            w.var_int(state as i32);
        }
        "block" | "block_marker" | "falling_dust" => {
            let state = u32::try_from(r.var_int()?).unwrap_or(0);
            w.var_int(ezzmc_world::legacy::state_to_canon(state) as i32);
        }
        "dust" => {
            let color = rgb(&mut r)?;
            w.i32(color).f32(r.f32()?);
        }
        // 1.17–1.20.1: from, scale, to.
        "dust_color_transition" => {
            let from = rgb(&mut r)?;
            let scale = r.f32()?;
            let to = rgb(&mut r)?;
            w.i32(from).i32(to).f32(scale);
        }
        // VANILLA-UNSURE: цвета зелий старых версий (смещения при счётчике 0) не переносятся —
        // клиент канона цвет из данных не читает.
        "effect" | "instant_effect" => {
            w.i32(-1).f32(1.0);
        }
        "entity_effect" | "flash" => {
            w.i32(-1);
        }
        "dragon_breath" => {
            w.f32(1.0);
        }
        "sculk_charge" => {
            w.f32(r.f32()?);
        }
        "shriek" => {
            w.var_int(r.var_int()?);
        }
        _ => {}
    }
    w.bool(force).bool(false).bytes(pos).bytes(dist);
    w.f32(speed).f32(speed).f32(speed).var_int(count).var_int(0);
    Ok(Some(w.into_vec()))
}

/// VANILLA: LevelRenderer#levelEvent (1.20.1 и старше) → номера и данные LevelEventHandler (26.3).
/// Отрицательные номера — звуки дверей до 1.19.3 (`client::levelsound`): те же числа в 26.3
/// свободны или значат другое.
fn level_event(version: &Version, body: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut r = Reader::new(body);
    let mut ty = r.i32()?;
    let pos = r.bytes(8)?;
    let mut data = r.i32()?;
    match ty {
        2001 | 3008 => {
            data = ezzmc_world::legacy::state_to_canon(u32::try_from(data).unwrap_or(0)) as i32;
        }
        1005..=1008 | 1012..=1014 | 1036 | 1037 if version.protocol <= 760 => ty = -ty,
        1011 if version.protocol <= 760 => ty = -ty,
        // VANILLA: data — id предмета пластинки, 0 — остановить; в 26.3 id песни и событие 1011.
        // VANILLA-UNSURE: id песни не переносится (реестра jukebox_song у этих версий нет).
        1010 => {
            if data == 0 {
                ty = 1011;
            } else {
                data = -1;
            }
        }
        // Только частицы роста: в 26.3 это 2011.
        2005 => ty = 2011,
        _ => {}
    }
    let mut w = Writer::with_capacity(body.len());
    w.i32(ty).bytes(pos).i32(data).bytes(r.rest());
    Ok(w.into_vec())
}

/// VANILLA: ClientboundBlockEventPacket: pos, b0, b1, id блока реестра версии.
fn block_event(version: &Version, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let head = r.bytes(10)?;
    let Some(name) = usize::try_from(r.var_int()?)
        .ok()
        .and_then(|i| version.block.get(i))
    else {
        return Ok(None);
    };
    let Some(id) = version::build_remap(&[name], V26_3.block)
        .first()
        .copied()
        .flatten()
    else {
        return Ok(None);
    };
    let mut w = Writer::with_capacity(body.len() + 1);
    w.bytes(head).var_int(id as i32);
    Ok(Some(w.into_vec()))
}

/// VANILLA: ClientboundExplodePacket (1.20.1): xyz double, power, список смещений блоков, отброс
/// float×3. Explosion#finalizeExplosion (клиент): звук generic.explode, большая частица при
/// power ≥ 2.
fn explode(protocol: i32, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let center = r.bytes(24)?;
    let power = r.f32()?;
    // VANILLA: ClientboundExplodePacket — до 1.16.5 (754) число блоков int, с 1.17 varint.
    let blocks = if protocol <= 754 {
        usize::try_from(r.i32()?).unwrap_or(0).min(1 << 16)
    } else {
        r.length("explosion blocks", 1 << 16)?
    };
    r.bytes(blocks * 3)?;
    let knockback = [r.f32()?, r.f32()?, r.f32()?];
    let particle = if power >= 2.0 {
        "explosion_emitter"
    } else {
        "explosion"
    };
    let sound = V26_3
        .sound_event
        .iter()
        .position(|n| *n == "entity.generic.explode");
    let (Some(particle), Some(sound)) = (particle_id(particle), sound) else {
        return Ok(None);
    };
    let mut w = Writer::with_capacity(64);
    w.bytes(center).f32(power).i32(blocks as i32);
    let moved = knockback.iter().any(|v| *v != 0.0);
    w.bool(moved);
    if moved {
        for v in knockback {
            w.f64(f64::from(v));
        }
    }
    // частица, звук, пустой список частиц блоков, playSound
    w.var_int(particle)
        .var_int(sound as i32 + 1)
        .var_int(0)
        .bool(true);
    Ok(Some(w.into_vec()))
}

/// `None` — пакет не передаётся клиенту.
pub(crate) fn to_canon(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let version = &version::current();
    Ok(match name {
        "sound" | "sound_entity" => sound(version, body)?,
        "level_particles" => particles(version, body)?,
        "level_event" => Some(level_event(version, body)?),
        "block_event" => block_event(version, body)?,
        // VANILLA: StopSoundPacket и BlockDestructionPacket не менялись.
        "stop_sound" | "block_destruction" => Some(body.to_vec()),
        "explode" => explode(version.protocol, body)?,
        _ => None,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::version::{V1_12_2, V1_20_1};

    fn id_of(list: &[&str], name: &str) -> i32 {
        list.iter().position(|n| *n == name).unwrap() as i32
    }

    #[test]
    fn sound_ids_are_remapped_by_name() {
        let old = id_of(V1_12_2.sound_event, "block.wool.break");
        let mut w = Writer::new();
        w.var_int(old + 1).var_int(4).i32(8);
        let out = sound(&V1_12_2, &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(
            r.var_int().unwrap(),
            id_of(V26_3.sound_event, "block.wool.break") + 1
        );
        assert_eq!((r.var_int().unwrap(), r.i32().unwrap()), (4, 8));
    }

    #[test]
    fn direct_sound_is_untouched() {
        let mut w = Writer::new();
        w.var_int(0).string("custom:ding").bool(false).var_int(0);
        let body = w.into_vec();
        assert_eq!(sound(&V1_20_1, &body).unwrap().unwrap(), body);
    }

    #[test]
    fn dust_particle_gets_canon_layout() {
        let mut w = Writer::new();
        w.var_int(id_of(V1_20_1.particle_type, "dust")).bool(true);
        for v in [1.0f64, 2.0, 3.0] {
            w.f64(v);
        }
        for v in [0.1f32, 0.2, 0.3, 0.5] {
            w.f32(v);
        }
        w.i32(7);
        for v in [1.0f32, 0.0, 0.0, 2.0] {
            w.f32(v);
        }
        let out = particles(&V1_20_1, &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        assert_eq!(r.var_int().unwrap(), particle_id("dust").unwrap());
        assert_eq!((r.i32().unwrap(), r.f32().unwrap()), (0xFF_0000, 2.0));
        assert!(r.bool().unwrap());
        assert!(!r.bool().unwrap());
        assert_eq!(r.f64().unwrap(), 1.0);
        r.bytes(16 + 12).unwrap();
        for _ in 0..3 {
            assert_eq!(r.f32().unwrap(), 0.5);
        }
        assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (7, 0));
        assert!(r.rest().is_empty());
    }

    #[test]
    fn door_and_jukebox_events_are_renumbered() {
        let event = |version: &Version, ty: i32, data: i32| {
            let mut w = Writer::new();
            w.i32(ty).i64(0).i32(data).bool(false);
            let out = level_event(version, &w.into_vec()).unwrap();
            let mut r = Reader::new(&out);
            let ty = r.i32().unwrap();
            r.i64().unwrap();
            (ty, r.i32().unwrap())
        };
        assert_eq!(event(&V1_12_2, 1005, 0), (-1005, 0));
        assert_eq!(event(&V1_12_2, 1011, 0), (-1011, 0));
        assert_eq!(event(&V1_20_1, 1011, 0), (1011, 0));
        assert_eq!(event(&V1_20_1, 1010, 0), (1011, 0));
        assert_eq!(event(&V1_20_1, 1010, 100), (1010, -1));
        assert_eq!(event(&V1_20_1, 2005, 3), (2011, 3));
    }

    #[test]
    fn explosion_block_count_is_int_before_1_17() {
        let mut w = Writer::new();
        for v in [1.0f64, 2.0, 3.0] {
            w.f64(v);
        }
        w.f32(4.0).i32(2).bytes(&[1, 0, 0, 0, 1, 0]);
        for v in [0.25f32, 0.5, -0.25] {
            w.f32(v);
        }
        let out = explode(754, &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        r.bytes(24).unwrap();
        assert_eq!((r.f32().unwrap(), r.i32().unwrap()), (4.0, 2));
        assert!(r.bool().unwrap());
        assert_eq!(
            [r.f64().unwrap(), r.f64().unwrap(), r.f64().unwrap()],
            [0.25, 0.5, -0.25]
        );
    }

    #[test]
    fn explosion_gets_particle_and_sound() {
        let mut w = Writer::new();
        for v in [1.0f64, 2.0, 3.0] {
            w.f64(v);
        }
        w.f32(4.0).var_int(1).bytes(&[1, 0, 0]);
        for v in [0.0f32, 0.5, 0.0] {
            w.f32(v);
        }
        let out = explode(763, &w.into_vec()).unwrap().unwrap();
        let mut r = Reader::new(&out);
        r.bytes(24).unwrap();
        assert_eq!((r.f32().unwrap(), r.i32().unwrap()), (4.0, 1));
        assert!(r.bool().unwrap());
        assert_eq!(r.f64().unwrap(), 0.0);
        assert_eq!(r.f64().unwrap(), 0.5);
        r.f64().unwrap();
        assert_eq!(
            r.var_int().unwrap(),
            particle_id("explosion_emitter").unwrap()
        );
    }
}
