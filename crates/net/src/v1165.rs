//! Отличия протокола 1.16.5 (754) от 1.17: тела приводятся к формату 1.17/1.17.1, дальше идут
//! общим путём `v1201`. Клики по контейнерам и подтверждения транзакций — свои.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI16, Ordering};
use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{nbt, Direction, ProtocolResult, State};

use crate::v1201::Pre18;

/// Число секций освещения мира 0..256 (по одной сверху и снизу).
const LIGHT_SECTIONS: u32 = 18;

/// ItemStack 1.16.5 «пусто».
const EMPTY: &[u8] = &[0];

/// (окно, слот) → стек в формате сервера.
type Slots = HashMap<(u8, i16), Vec<u8>>;

/// Предметы слотов: клик в 1.16.5 несёт предмет слота до клика.
static SLOTS: LazyLock<Mutex<Slots>> = LazyLock::new(Default::default);

/// VANILLA: AbstractContainerMenu#backup (1.16.5) — номер транзакции клика.
static ACTION: AtomicI16 = AtomicI16::new(0);

fn slots() -> MutexGuard<'static, Slots> {
    SLOTS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Один ItemStack 1.16.5 (bool, varint id, byte count, NBT) как срез тела.
fn item<'a>(r: &mut Reader<'a>) -> ProtocolResult<&'a [u8]> {
    let at = r.pos();
    if r.bool()? {
        r.var_int()?;
        r.i8()?;
        nbt::read_named(r)?;
    }
    Ok(r.since(at))
}

/// Тело клиентского пакета 1.16.5 → формат 1.17.
pub fn from_1_16_5(name: &str, body: &[u8]) -> ProtocolResult<Pre18> {
    let mut r = Reader::new(body);
    let mut w = Writer::with_capacity(body.len() + 32);
    match name {
        // VANILLA: ClientboundLevelChunkPacket (1.16.5): x, z, bool fullChunk, varint маска
        // секций → 1.17: x, z, BitSet маска (без fullChunk); остальное совпадает.
        "level_chunk" => {
            w.i32(r.i32()?).i32(r.i32()?);
            // VANILLA-UNSURE: частичные чанки (fullChunk=false) ванильный сервер не шлёт.
            if !r.bool()? {
                return Ok(Pre18::Drop);
            }
            bit_set(&mut w, r.var_int()?);
            w.bytes(r.rest());
        }
        // VANILLA: ClientboundLightUpdatePacket (1.16.5): четыре varint-маски и массивы по битам
        // → 1.17: четыре BitSet и два списка массивов с длинами.
        "light_update" => {
            w.var_int(r.var_int()?)
                .var_int(r.var_int()?)
                .bool(r.bool()?);
            let masks = [r.var_int()?, r.var_int()?, r.var_int()?, r.var_int()?];
            masks.iter().for_each(|m| bit_set(&mut w, *m));
            for mask in &masks[..2] {
                let n = (0..LIGHT_SECTIONS).filter(|i| mask & (1 << i) != 0).count();
                w.var_int(n as i32);
                for _ in 0..n {
                    let len = r.length("light array", 2048)?;
                    w.var_int(len as i32).bytes(r.bytes(len)?);
                }
            }
        }
        "container_set_content" => {
            let win = r.u8()?;
            let n = r.i16()?;
            let mut map = slots();
            map.retain(|k, _| k.0 != win);
            for slot in 0..n.max(0) {
                map.insert((win, slot), item(&mut r)?.to_vec());
            }
            return Ok(Pre18::Keep);
        }
        "container_set_slot" => {
            let (win, slot) = (r.u8()?, r.i16()?);
            let stack = item(&mut r)?.to_vec();
            if slot >= 0 {
                slots().insert((win, slot), stack);
            }
            return Ok(Pre18::Keep);
        }
        // Подтверждение уже отправлено соединением (`ack_reply`).
        "container_ack" => return Ok(Pre18::Drop),
        _ => return Ok(Pre18::Keep),
    }
    Ok(Pre18::Replace(w.into_vec()))
}

/// BitSet из одной int-маски: varint число long, long[].
fn bit_set(w: &mut Writer, mask: i32) {
    if mask == 0 {
        w.var_int(0);
    } else {
        w.var_int(1).i64(i64::from(mask as u32));
    }
}

/// VANILLA: ServerboundContainerClickPacket (1.16.5): byte окно, short слот, byte кнопка, short
/// номер транзакции, enum режим, предмет — результат `AbstractContainerMenu#clicked`
/// (PICKUP/QUICK_MOVE по слоту — предмет до клика, иначе пусто).
/// `changed` — новые стеки затронутых слотов в формате сервера (только для кэша).
pub fn container_click(
    window: u8,
    slot: i16,
    button: u8,
    mode: i32,
    changed: Vec<(i16, Vec<u8>)>,
) -> Vec<u8> {
    let mut map = slots();
    let clicked = match mode {
        0 | 1 if slot >= 0 => map.get(&(window, slot)).cloned(),
        _ => None,
    };
    for (s, stack) in changed {
        map.insert((window, s), stack);
    }
    let uid = ACTION.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    let mut w = Writer::new();
    w.u8(window)
        .i16(slot)
        .u8(button)
        .i16(uid)
        .var_int(mode)
        .bytes(clicked.as_deref().unwrap_or(EMPTY));
    w.into_vec()
}

/// VANILLA: ClientPacketListener#handleContainerAck (1.16.5): на отклонённую транзакцию клиент
/// отвечает тем же окном и номером. Возвращает `(id, тело)` ответа на пакет `id`.
/// VANILLA-UNSURE: ваниль отвечает только для открытого окна и инвентаря — здесь для любого.
pub fn ack_reply(id: i32, body: &[u8]) -> Option<(i32, Vec<u8>)> {
    let table = ezzmc_protocol::version::current().packets;
    let find = |dir, name| {
        table
            .iter()
            .find(|p| p.state == State::Play && p.dir == dir && p.name == name)
            .map(|p| p.id as i32)
    };
    if find(Direction::Clientbound, "container_ack") != Some(id) {
        return None;
    }
    let mut r = Reader::new(body);
    let (window, uid) = (r.u8().ok()?, r.i16().ok()?);
    if r.bool().ok()? {
        return None;
    }
    let mut w = Writer::new();
    w.u8(window).i16(uid).u8(1);
    Some((find(Direction::Serverbound, "container_ack")?, w.into_vec()))
}
