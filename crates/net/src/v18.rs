//! 1.8–1.8.9 (47) ↔ формат 1.9 (107). Дальше пакеты идут обычным путём 1.9 → 1.10 → 1.11 → ….
//!
//! Таблица пакетов — `generated/v1_8.rs` (NetworkState 1.8.9). Пакеты без пары в 1.9 названы
//! с суффиксом `_1_8` и разворачиваются в `expand`; пакеты 1.9, которых нет в 1.8, получили id
//! вне диапазона сервера (0x60+).
//!
//! Отличия тел (VANILLA: пакеты 1.8.9 и 1.9.4, Yarn):
//! - координаты сущностей — int ×32 (в 1.9 double), относительные сдвиги — byte ×32 (short ×4096);
//! - спавн объектов/мобов/картин без UUID; скорость объекта только при data > 0;
//! - метаданные: заголовок `тип<<5 | индекс`, конец 0x7F, свои индексы у каждого класса;
//! - чанк: секции без палитры (char id<<4|meta), свет секций — после всех блоков;
//! - посадка/поводок — один EntityAttachS2CPacket вместо SetPassengers/AttachEntity.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, PoisonError};

use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolResult;

/// Что заспавнено под id: метаданные 1.8 зависят от класса сущности.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Kind {
    Player,
    /// Тип моба (MobSpawnS2CPacket, EntityType 1.8).
    Mob(u8),
    /// Тип объекта и его data (EntitySpawnS2CPacket).
    Object(u8, i32),
}

#[derive(Default)]
struct State {
    kinds: HashMap<i32, Kind>,
    /// пассажир → транспорт (в 1.8 у транспорта один пассажир).
    riding: HashMap<i32, i32>,
}

fn state() -> std::sync::MutexGuard<'static, State> {
    static S: OnceLock<Mutex<State>> = OnceLock::new();
    S.get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// Класс сущности по id (из пакетов спавна).
pub(crate) fn kind_of(id: i32) -> Option<Kind> {
    state().kinds.get(&id).copied()
}

fn is_1_8() -> bool {
    ezzmc_protocol::version::current().protocol < 107
}

/// Фиксированная точка 1.8 (×32) → double.
fn fixed(r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
    for _ in 0..3 {
        w.f64(f64::from(r.i32()?) / 32.0);
    }
    Ok(())
}

/// В 1.8 у сущностей нет UUID в пакете спавна; в 1.9 он есть, но клиент им не пользуется
/// для мобов/объектов — подставляется уникальный по id.
fn fake_uuid(w: &mut Writer, id: i32) {
    w.i64(0x1808_0000_0000_0000).i64(i64::from(id));
}

/// Тело свежего клиентского пакета 1.8 → 1.9. `None` — без изменений.
pub(crate) fn to_1_9(name: &str, body: &[u8]) -> ProtocolResult<Option<Vec<u8>>> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 32);
    match name {
        "login" => {
            *state() = State::default();
            return Ok(None);
        }
        // VANILLA: PlayerPositionLookS2CPacket (1.9.4): в конце varint id телепорта.
        "player_position" => {
            w.bytes(body).var_int(0);
        }
        // VANILLA: EntityEquipmentUpdateS2CPacket 1.8: short слот (0 рука, 1–4 ботинки…шлем);
        // 1.9: varint EquipmentSlot (0 MAINHAND, 1 OFFHAND, 2 FEET … 5 HEAD).
        "set_equipped_item" => {
            w.var_int(r.var_int()?);
            let slot = i32::from(r.i16()?);
            w.var_int(if slot == 0 { 0 } else { slot + 1 })
                .bytes(r.rest());
        }
        // VANILLA: PlayerSpawnS2CPacket 1.8: id, uuid, int xyz, yaw, pitch, short предмет, метаданные.
        "add_player" => {
            let id = r.var_int()?;
            w.var_int(id).bytes(r.bytes(16)?);
            fixed(&mut r, &mut w)?;
            w.bytes(r.bytes(2)?);
            r.i16()?;
            state().kinds.insert(id, Kind::Player);
            metadata(&mut r, &mut w, Kind::Player)?;
        }
        // VANILLA: EntitySpawnS2CPacket 1.8: id, byte тип, int xyz, pitch, yaw, int data,
        // скорость только при data > 0. 1.9: id, uuid, тип, double xyz, pitch, yaw, data, скорость.
        "add_entity" => {
            let id = r.var_int()?;
            let ty = r.u8()?;
            w.var_int(id);
            fake_uuid(&mut w, id);
            w.u8(ty);
            fixed(&mut r, &mut w)?;
            w.bytes(r.bytes(2)?);
            let data = r.i32()?;
            w.i32(data);
            if data > 0 {
                w.bytes(r.bytes(6)?);
            } else {
                w.i16(0).i16(0).i16(0);
            }
            state().kinds.insert(id, Kind::Object(ty, data));
        }
        // VANILLA: MobSpawnS2CPacket 1.8: id, u8 тип, int xyz, yaw, pitch, headYaw, скорость, метаданные.
        "add_mob" => {
            let id = r.var_int()?;
            let ty = r.u8()?;
            w.var_int(id);
            fake_uuid(&mut w, id);
            w.u8(ty);
            fixed(&mut r, &mut w)?;
            w.bytes(r.bytes(3 + 6)?);
            state().kinds.insert(id, Kind::Mob(ty));
            metadata(&mut r, &mut w, Kind::Mob(ty))?;
        }
        // VANILLA: PaintingSpawnS2CPacket 1.9: после id — uuid.
        "add_painting" => {
            let id = r.var_int()?;
            w.var_int(id);
            fake_uuid(&mut w, id);
            w.bytes(r.rest());
        }
        "add_experience_orb" | "add_global_entity" | "teleport_entity" => {
            w.var_int(r.var_int()?);
            if name == "add_global_entity" {
                w.u8(r.u8()?);
            }
            fixed(&mut r, &mut w)?;
            w.bytes(r.rest());
        }
        // VANILLA: EntityS2CPacket 1.8: byte сдвиг (×32); 1.9: short (×4096) — множитель 128.
        "move_entity_pos" | "move_entity_pos_rot" => {
            w.var_int(r.var_int()?);
            for _ in 0..3 {
                w.i16(i16::from(r.i8()?) * 128);
            }
            w.bytes(r.rest());
        }
        "set_entity_data" => {
            let id = r.var_int()?;
            w.var_int(id);
            let kind = state().kinds.get(&id).copied();
            match kind {
                Some(kind) => metadata(&mut r, &mut w, kind)?,
                None => {
                    // Неизвестная сущность: только общие поля Entity.
                    metadata(&mut r, &mut w, Kind::Object(0, 0))?;
                }
            }
        }
        // VANILLA: EntityStatusEffectS2CPacket 1.8: последний byte — показывать частицы (0/1);
        // 1.9: флаги (бит 0 ambient, бит 1 частицы).
        "update_mob_effect" => {
            w.var_int(r.var_int()?)
                .bytes(r.bytes(2)?)
                .var_int(r.var_int()?);
            w.u8(if r.u8()? != 0 { 2 } else { 0 });
        }
        // VANILLA: MapUpdateS2CPacket 1.9: после масштаба — bool «показывать иконки».
        "map_item_data" => {
            w.var_int(r.var_int()?)
                .u8(r.u8()?)
                .bool(true)
                .bytes(r.rest());
        }
        // VANILLA: TeamS2CPacket 1.9: после видимости ника — правило столкновений.
        "set_player_team" => {
            w.string(&r.string(16)?);
            let mode = r.u8()?;
            w.u8(mode);
            if mode == 0 || mode == 2 {
                w.string(&r.string(32)?)
                    .string(&r.string(16)?)
                    .string(&r.string(16)?);
                w.u8(r.u8()?).string(&r.string(32)?).string("always");
            }
            w.bytes(r.rest());
        }
        // VANILLA: PlaySoundIdS2CPacket 1.8 (по имени): без категории; 1.9 PlaySoundNameS2CPacket.
        // VANILLA-UNSURE: имена звуков 1.8 («random.click») не переведены в имена 1.9.
        "custom_sound" => {
            w.string(&r.string(256)?).var_int(0).bytes(r.rest());
        }
        _ => return Ok(None),
    }
    Ok(Some(w.into_vec()))
}

// ---------- метаданные ----------

/// Сериализаторы 1.9 (TrackedDataHandlerRegistry 1.9.4).
const BYTE: i32 = 0;
const VARINT: i32 = 1;
const FLOAT: i32 = 2;
const STRING: i32 = 3;
const CHAT: i32 = 4;
const SLOT: i32 = 5;
const BOOL: i32 = 6;
const ROTATION: i32 = 7;
const BLOCK: i32 = 12;

/// Индекс и сериализатор 1.9 для поля 1.8 (`None` — в 1.9 нет).
/// VANILLA: DataTracker-индексы классов 1.8.9 (`startTracking`) и 1.9.4 (`TrackedData`).
fn remap(kind: Kind, idx: u8) -> Option<(u8, i32)> {
    // Entity
    match idx {
        0 => return Some((0, BYTE)),
        1 => return Some((1, VARINT)),
        2 => return Some((2, STRING)),
        3 => return Some((3, BOOL)),
        4 => return Some((4, BOOL)),
        _ => {}
    }
    let living = matches!(kind, Kind::Player | Kind::Mob(_) | Kind::Object(78, _));
    if living {
        match idx {
            6 => return Some((6, FLOAT)),
            7 => return Some((7, VARINT)),
            8 => return Some((8, BOOL)),
            9 => return Some((9, VARINT)),
            _ => {}
        }
    }
    match kind {
        Kind::Player => match idx {
            10 => Some((12, BYTE)),
            17 => Some((10, FLOAT)),
            18 => Some((11, VARINT)),
            _ => None,
        },
        Kind::Mob(ty) => {
            if idx == 15 {
                return Some((10, BYTE));
            }
            // Ageable: 12 byte возраст → 11 bool ребёнок (значение переводит `value`).
            if matches!(ty, 90..=93 | 95 | 96 | 98 | 100 | 101 | 120) && idx == 12 {
                return Some((11, BOOL));
            }
            mob(ty, idx)
        }
        Kind::Object(ty, data) => object(ty, data, idx),
    }
}

fn mob(ty: u8, idx: u8) -> Option<(u8, i32)> {
    Some(match (ty, idx) {
        (90, 16) => (12, BOOL),                 // Pig: седло
        (91, 16) => (12, BYTE),                 // Sheep: цвет/стрижка
        (95, 16) | (98, 16) => (12, BYTE),      // Wolf/Ocelot: флаги приручения
        (95, 18) => (14, FLOAT),                // Wolf: здоровье (хвост)
        (95, 19) => (15, BOOL),                 // Wolf: выпрашивает
        (95, 20) => (16, VARINT),               // Wolf: ошейник
        (98, 18) => (14, VARINT),               // Ocelot: окрас
        (100, 16) => (12, BYTE),                // Horse: флаги
        (100, 19) => (13, VARINT),              // Horse: тип
        (100, 20) => (14, VARINT),              // Horse: окрас
        (100, 22) => (16, VARINT),              // Horse: броня
        (101, 18) => (12, VARINT),              // Rabbit: тип
        (120, 16) => (12, VARINT),              // Villager: профессия
        (99, 16) => (11, BYTE),                 // IronGolem: создан игроком
        (50, 16) => (11, VARINT),               // Creeper: состояние
        (50, 17) | (50, 18) => (idx - 5, BOOL), // Creeper: заряжен, подожжён
        (51, 13) => (11, VARINT),               // Skeleton: тип
        (52, 16) | (59, 16) => (11, BYTE),      // Spider: лезет
        (54, 12) | (57, 12) => (11, BOOL),      // Zombie: ребёнок
        (54, 13) | (57, 13) => (12, VARINT),    // Zombie: житель
        (54, 14) | (57, 14) => (13, BOOL),      // Zombie: превращается
        (55, 16) | (62, 16) => (11, VARINT),    // Slime: размер
        (56, 16) => (11, BOOL),                 // Ghast: атакует
        (58, 16) => (11, BLOCK),                // Enderman: блок в руках
        (58, 18) => (12, BOOL),                 // Enderman: кричит
        (61, 16) => (11, BYTE),                 // Blaze: горит
        (66, 21) => (11, BOOL),                 // Witch: пьёт
        (68, 16) => (11, BYTE),                 // Guardian: флаги
        (68, 17) => (12, VARINT),               // Guardian: цель
        (64, 17..=19) => (idx - 6, VARINT),     // Wither: цели
        (64, 20) => (14, VARINT),               // Wither: неуязвимость
        (65, 16) => (11, BYTE),                 // Bat: висит
        _ => return None,
    })
}

fn object(ty: u8, data: i32, idx: u8) -> Option<(u8, i32)> {
    Some(match (ty, idx) {
        (78, 10) => (10, BYTE),                // ArmorStand: флаги
        (78, 11..=16) => (idx, ROTATION),      // ArmorStand: позы
        (2, 10) => (5, SLOT),                  // Item
        (71, 8) => (5, SLOT),                  // ItemFrame: предмет
        (71, 9) => (6, VARINT),                // ItemFrame: поворот 0–7
        (76, 8) => (5, SLOT),                  // Firework
        (60, 16) => (5, BYTE),                 // Arrow: крит
        (66, 10) => (5, BOOL),                 // WitherSkull: неуязвим
        (1, 17) => (5, VARINT),                // Boat: время удара
        (1, 18) => (6, VARINT),                // Boat: направление
        (1, 19) => (7, FLOAT),                 // Boat: урон
        (10, 17) => (5, VARINT),               // Minecart: тряска
        (10, 18) => (6, VARINT),               // Minecart: направление
        (10, 19) => (7, FLOAT),                // Minecart: урон
        (10, 20) => (8, VARINT),               // Minecart: блок внутри
        (10, 21) => (9, VARINT),               // Minecart: смещение блока
        (10, 22) => (10, BOOL),                // Minecart: показывать блок
        (10, 16) if data == 2 => (11, BOOL),   // FurnaceMinecart: топливо
        (10, 23) if data == 6 => (11, STRING), // CommandMinecart: команда
        (10, 24) if data == 6 => (12, CHAT),   // CommandMinecart: вывод
        _ => return None,
    })
}

/// Значение 1.8 в виде числа (для числовых типов) или сырых байт.
enum Val<'a> {
    Num(i64),
    Float(f32),
    Raw(&'a [u8]),
}

/// VANILLA: DataTracker#deserializePacket (1.8.9) → поля 1.9 (`ser` по `remap`), конец 0xFF.
fn metadata(r: &mut Reader, w: &mut Writer, kind: Kind) -> ProtocolResult<()> {
    loop {
        let head = r.u8()?;
        if head == 0x7F {
            break;
        }
        let (ty, idx) = (head >> 5, head & 0x1F);
        let at = r.pos();
        let val = match ty {
            0 => Val::Num(i64::from(r.i8()?)),
            1 => Val::Num(i64::from(r.i16()?)),
            2 => Val::Num(i64::from(r.i32()?)),
            3 => Val::Float(r.f32()?),
            4 => {
                r.string(32767)?;
                Val::Raw(r.since(at))
            }
            5 => {
                if r.i16()? >= 0 {
                    r.bytes(3)?;
                    ezzmc_protocol::nbt::read_named(r)?;
                }
                Val::Raw(r.since(at))
            }
            6 | 7 => Val::Raw(r.bytes(12)?),
            t => {
                return Err(ezzmc_protocol::ProtocolError::InvalidValue {
                    what: "тип метаданных 1.8",
                    value: i64::from(t),
                })
            }
        };
        let Some((to, ser)) = remap(kind, idx) else {
            continue;
        };
        // Ageable: возраст < 0 — ребёнок.
        let baby = matches!(kind, Kind::Mob(_)) && idx == 12 && ser == BOOL;
        let ok = match (&val, ser) {
            (Val::Num(n), BYTE) => {
                w.u8(to).var_int(ser).u8(*n as u8);
                true
            }
            (Val::Num(n), BOOL) => {
                let b = if baby { *n < 0 } else { *n != 0 };
                w.u8(to).var_int(ser).bool(b);
                true
            }
            (Val::Num(n), VARINT | BLOCK) => {
                w.u8(to).var_int(ser).var_int(*n as i32);
                true
            }
            (Val::Float(f), FLOAT) => {
                w.u8(to).var_int(ser).f32(*f);
                true
            }
            (Val::Raw(b), STRING | CHAT | SLOT | ROTATION) => {
                w.u8(to).var_int(ser).bytes(b);
                true
            }
            _ => false,
        };
        if !ok {
            tracing::debug!("1.8: метаданные {kind:?} #{idx} тип {ty} → {ser}: пропуск");
        }
    }
    w.u8(0xFF);
    Ok(())
}

// ---------- развороты ----------

/// Пакеты `(id, тело)`.
type Packets = Vec<(i32, Vec<u8>)>;

/// Пакет 1.8 без пары в 1.9 → пакеты в формате 1.11 (как у пакетов из `legacy::expand`).
/// `None` — пакет не разворачивается.
pub(crate) fn expand(
    name: &str,
    body: &[u8],
    id_of: impl Fn(&str) -> Option<i32>,
) -> ProtocolResult<Option<Packets>> {
    let mut out = Vec::new();
    let mut push = |n: &str, b: Vec<u8>| -> ProtocolResult<()> {
        if let Some(id) = id_of(n) {
            let b = crate::legacy::from_1_9(n, &b)?.unwrap_or(b);
            out.push((id, b));
        }
        Ok(())
    };
    let mut r = Reader::new(body);
    match name {
        "set_compression_1_8" | "update_entity_nbt_1_8" => {}
        // VANILLA: EntityAnimationS2CPacket 1.8: 3 — еда (в 1.9 это удар второй рукой).
        "animate" => {
            r.var_int()?;
            if r.u8()? != 3 {
                return Ok(None);
            }
        }
        "level_chunk_1_8" => {
            let (x, z, full) = (r.i32()?, r.i32()?, r.bool()?);
            let mask = r.u16()?;
            let len = r.length("chunk 1.8", 0x40_0000)?;
            let data = r.bytes(len)?;
            if full && mask == 0 {
                // VANILLA: ClientPlayNetworkHandler#onChunkData (1.8.9): полный пустой — выгрузка.
                let mut w = Writer::new();
                w.i32(x).i32(z);
                push("forget_level_chunk", w.into_vec())?;
            } else {
                let n = mask.count_ones() as usize;
                let rest = data
                    .len()
                    .saturating_sub(n * 10240 + if full { 256 } else { 0 });
                let sky = n > 0 && rest >= n * 2048;
                push("level_chunk", chunk(x, z, full, mask, sky, data)?)?;
            }
        }
        // VANILLA: ChunkMapS2CPacket#read (1.8.9): небо, n × (x, z, маска), затем данные подряд.
        "level_chunk_bulk_1_8" => {
            let sky = r.bool()?;
            let n = r.length("chunk bulk 1.8", 1024)?;
            let mut heads = Vec::with_capacity(n);
            for _ in 0..n {
                heads.push((r.i32()?, r.i32()?, r.u16()?));
            }
            for (x, z, mask) in heads {
                let s = mask.count_ones() as usize;
                let len = s * 10240 + if sky { s * 2048 } else { 0 } + 256;
                push("level_chunk", chunk(x, z, true, mask, sky, r.bytes(len)?)?)?;
            }
        }
        // VANILLA: EntityAttachS2CPacket 1.8: int пассажир/привязанный, int транспорт/держатель,
        // u8 поводок. 1.9: EntityAttachS2CPacket (поводок) и EntityPassengersSetS2CPacket.
        "set_entity_link_1_8" => {
            let (who, holder, leash) = (r.i32()?, r.i32()?, r.u8()? != 0);
            if leash {
                let mut w = Writer::new();
                w.i32(who).i32(holder);
                push("set_entity_link", w.into_vec())?;
            } else {
                let old = {
                    let mut st = state();
                    let old = if holder == -1 {
                        st.riding.remove(&who)
                    } else {
                        st.riding.insert(who, holder)
                    };
                    old.filter(|&v| v != holder)
                };
                let passengers = |vehicle: i32, riders: &[i32]| {
                    let mut w = Writer::new();
                    w.var_int(vehicle).var_int(riders.len() as i32);
                    for &p in riders {
                        w.var_int(p);
                    }
                    w.into_vec()
                };
                if let Some(old) = old {
                    push("set_passengers", passengers(old, &[]))?;
                }
                if holder != -1 {
                    push("set_passengers", passengers(holder, &[who]))?;
                }
            }
        }
        _ => return Ok(None),
    }
    Ok(Some(out))
}

/// Данные чанка 1.8 → тело `level_chunk` 1.9 (без списка блок-сущностей, его добавит 1.9.4).
/// VANILLA: Chunk#method_3895 (1.8.9): блоки всех секций (char LE, id<<4|meta), затем свет
/// блоков всех секций, свет неба, биомы. ChunkSection/PalettedContainer#write (1.9.4).
fn chunk(x: i32, z: i32, full: bool, mask: u16, sky: bool, data: &[u8]) -> ProtocolResult<Vec<u8>> {
    let n = mask.count_ones() as usize;
    let mut r = Reader::new(data);
    let blocks = r.bytes(n * 8192)?;
    let block_light = r.bytes(n * 2048)?;
    let sky_light = if sky { Some(r.bytes(n * 2048)?) } else { None };
    let mut sec = Writer::with_capacity(n * 6000 + 256);
    let mut ids = [0u16; 4096];
    let mut longs = Vec::new();
    for s in 0..n {
        for (i, c) in blocks[s * 8192..(s + 1) * 8192].chunks_exact(2).enumerate() {
            ids[i] = u16::from_le_bytes([c[0], c[1]]);
        }
        let mut palette: Vec<u16> = Vec::new();
        let mut index = HashMap::new();
        for &id in &ids {
            index.entry(id).or_insert_with(|| {
                palette.push(id);
                palette.len() - 1
            });
        }
        let need = usize::BITS - (palette.len().max(2) - 1).leading_zeros();
        let (bits, global) = if need <= 8 {
            (need.max(4), false)
        } else {
            (13, true)
        };
        sec.u8(bits as u8);
        if global {
            sec.var_int(0);
        } else {
            sec.var_int(palette.len() as i32);
            for &p in &palette {
                sec.var_int(i32::from(p));
            }
        }
        // VANILLA: PackedIntegerArray (1.9.4): значения подряд, могут пересекать границу long.
        longs.clear();
        longs.resize(4096 * bits as usize / 64, 0u64);
        for (i, &id) in ids.iter().enumerate() {
            let v = if global {
                u64::from(id)
            } else {
                index[&id] as u64
            };
            let bit = i * bits as usize;
            let (li, off) = (bit / 64, bit % 64);
            longs[li] |= v << off;
            if off + bits as usize > 64 {
                longs[li + 1] |= v >> (64 - off);
            }
        }
        sec.var_int(longs.len() as i32);
        for &l in &longs {
            sec.i64(l as i64);
        }
        sec.bytes(&block_light[s * 2048..(s + 1) * 2048]);
        if let Some(sl) = sky_light {
            sec.bytes(&sl[s * 2048..(s + 1) * 2048]);
        }
    }
    if full {
        sec.bytes(r.bytes(256)?);
    }
    let sec = sec.into_vec();
    let mut w = Writer::with_capacity(sec.len() + 16);
    w.i32(x).i32(z).bool(full).var_int(i32::from(mask));
    w.var_int(sec.len() as i32).bytes(&sec);
    Ok(w.into_vec())
}

// ---------- serverbound ----------

/// Тело пакета игрока в формате 1.9 → 1.8 (id по таблице 1.8). Пустой список — не отправлять.
/// VANILLA: пакеты C2S 1.8.9 и 1.9.4. Для 1.7 результат дальше переводит `v17::to_1_7`.
pub(crate) fn to_1_8(
    id: i32,
    name: &str,
    body: Vec<u8>,
    id_of: impl Fn(&str) -> Option<i32>,
) -> ProtocolResult<Vec<(i32, Vec<u8>)>> {
    let out = from_1_9(id, name, body, id_of)?;
    if !crate::v17::is_1_7() {
        return Ok(out);
    }
    let mut v17 = Vec::with_capacity(out.len());
    for (id, b) in out {
        if let Some(b) = crate::v17::to_1_7(id, b)? {
            v17.push((id, b));
        }
    }
    Ok(v17)
}

fn from_1_9(
    id: i32,
    name: &str,
    body: Vec<u8>,
    id_of: impl Fn(&str) -> Option<i32>,
) -> ProtocolResult<Vec<(i32, Vec<u8>)>> {
    if !is_1_8() {
        return Ok(vec![(id, body)]);
    }
    let mut r = Reader::new(&body);
    let mut w = Writer::with_capacity(body.len() + 8);
    let mut id = id;
    match name {
        "accept_teleportation" | "move_vehicle" | "paddle_boat" => return Ok(Vec::new()),
        // 1.9: pos, varint грань, varint рука, 3 × u8 курсор. 1.8: pos, u8 грань, предмет, курсор.
        // Сервер 1.8 берёт предмет из руки сам; несовпадение лишь вызывает пересылку слота.
        "use_item_on" => {
            let pos = r.i64()?;
            let face = r.var_int()?;
            if r.var_int()? != 0 {
                return Ok(Vec::new());
            }
            w.i64(pos).u8(face as u8).i16(-1).bytes(r.rest());
        }
        // VANILLA: ServerPlayNetworkHandler#onPlayerInteractBlock (1.8.9): грань 255 — «использовать
        // предмет в руке».
        "use_item" => {
            if r.var_int()? != 0 {
                return Ok(Vec::new());
            }
            let Some(place) = id_of("use_item_on") else {
                return Ok(Vec::new());
            };
            id = place;
            w.i64(-1).u8(255).i16(-1).u8(0).u8(0).u8(0);
        }
        "player_action" => {
            let status = r.var_int()?;
            if status == 6 {
                return Ok(Vec::new()); // смена рук
            }
            w.u8(status as u8).bytes(r.rest());
        }
        "swing" => {
            if r.var_int()? != 0 {
                return Ok(Vec::new());
            }
        }
        "interact" => {
            let target = r.var_int()?;
            let action = r.var_int()?;
            w.var_int(target).var_int(action);
            if action == 2 {
                w.bytes(r.bytes(12)?);
            }
            if action != 1 && r.var_int()? != 0 {
                return Ok(Vec::new());
            }
        }
        // 1.9: язык, дальность, varint чат, цвета, скин, varint рука. 1.8: чат — byte, без руки.
        "client_information" => {
            w.string(&r.string(16)?).u8(r.u8()?);
            w.u8(r.var_int()? as u8).bool(r.bool()?).u8(r.u8()?);
        }
        "container_click" => {
            w.bytes(r.bytes(6)?).u8(r.var_int()? as u8).bytes(r.rest());
        }
        // 1.9: текст, bool assumeCommand, bool + pos. 1.8: без assumeCommand.
        "command_suggestion" => {
            w.string(&r.string(32767)?);
            r.bool()?;
            w.bytes(r.rest());
        }
        // VANILLA: UpdateSignC2SPacket 1.8: строки — текстовые компоненты JSON.
        "sign_update" => {
            w.i64(r.i64()?);
            for _ in 0..4 {
                let s = r.string(384)?;
                w.string(&serde_json::to_string(&s).unwrap_or_default());
            }
        }
        // 1.9: 6 конец прыжка коня, 7 инвентарь коня, 8 элитры. 1.8: 6 — инвентарь коня.
        "player_command" => {
            let eid = r.var_int()?;
            let action = match r.var_int()? {
                6 | 8 => return Ok(Vec::new()),
                7 => 6,
                a => a,
            };
            w.var_int(eid).var_int(action).bytes(r.rest());
        }
        _ => return Ok(vec![(id, body)]),
    }
    Ok(vec![(id, w.into_vec())])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_section_palette_and_light() {
        // Одна секция: камень (1<<4) везде, кроме блока 0 — воздух; свет 0x11; небо 0xFF.
        let mut data = vec![0u8; 8192];
        for c in data.chunks_exact_mut(2).skip(1) {
            c.copy_from_slice(&(1u16 << 4).to_le_bytes());
        }
        data.extend([0x11; 2048]);
        data.extend([0xFF; 2048]);
        data.extend([1; 256]);
        let out = chunk(3, -4, true, 1, true, &data).unwrap();
        let mut r = Reader::new(&out);
        assert_eq!((r.i32().unwrap(), r.i32().unwrap()), (3, -4));
        assert!(r.bool().unwrap());
        assert_eq!(r.var_int().unwrap(), 1);
        let len = r.var_int().unwrap() as usize;
        assert_eq!(r.remaining(), len);
        assert_eq!(r.u8().unwrap(), 4);
        assert_eq!(r.var_int().unwrap(), 2);
        assert_eq!((r.var_int().unwrap(), r.var_int().unwrap()), (0, 16));
        assert_eq!(r.var_int().unwrap(), 256);
        assert_eq!(r.i64().unwrap() as u64, 0x1111_1111_1111_1110);
        r.bytes(255 * 8).unwrap();
        assert_eq!(r.u8().unwrap(), 0x11);
        r.bytes(2047 + 2048 + 256).unwrap();
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn metadata_mob_remap() {
        // Свинья: 0 byte, 6 float здоровье, 12 byte возраст −1, 16 byte седло, 15 byte noAI.
        let mut w = Writer::new();
        w.u8(0).u8(0x20);
        w.u8(3 << 5 | 6).f32(10.0);
        w.u8(12).u8(0xFF);
        w.u8(16).u8(1);
        w.u8(15).u8(1);
        w.u8(1 << 5 | 1).i16(300);
        w.u8(0x7F);
        let body = w.into_vec();
        let mut w = Writer::new();
        metadata(&mut Reader::new(&body), &mut w, Kind::Mob(90)).unwrap();
        let out = w.into_vec();
        let e = crate::v1102::entries(&mut Reader::new(&out)).unwrap();
        let got: Vec<(u8, i32)> = e.iter().map(|x| (x.0, x.1)).collect();
        assert_eq!(
            got,
            [
                (0, BYTE),
                (6, FLOAT),
                (11, BOOL),
                (12, BOOL),
                (10, BYTE),
                (1, VARINT)
            ]
        );
        assert_eq!(e[2].2, [1]);
    }
}
