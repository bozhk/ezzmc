//! Общий поиск блоков-сущностей для BER-рисовальщиков (головы, баннеры): скан палитр секций
//! в окне вокруг камеры, пересчёт только при изменении мира.
//! VANILLA: BlockEntityRenderer#getViewDistance = 64 → окно 5 колонок (с запасом).

use ezzmc_world::block::BlockState;
use ezzmc_world::level::Level;

const SCAN_RADIUS: i32 = 5;

/// Найденный блок: позиция и состояние.
pub type Found = ([i32; 3], BlockState);

#[derive(Default)]
pub struct Scanner {
    pub list: Vec<Found>,
    block_version: Option<u64>,
    version: Option<u64>,
    cam_chunk: Option<(i32, i32)>,
    tick: i64,
}

impl Scanner {
    /// Обновляет `list`, если мир изменился. Возвращает `true`, если список пересчитан.
    pub fn update(
        &mut self,
        level: &Level,
        cam: [f64; 3],
        (world_version, block_version): (u64, u64),
        is_id: fn(u32) -> bool,
    ) -> bool {
        let now = level.game_time;
        let cam_chunk = (
            (cam[0] / 16.0).floor() as i32,
            (cam[2] / 16.0).floor() as i32,
        );
        let block_changed = self.block_version != Some(block_version);
        // Загрузка чанков при беге меняет версию почти каждый тик: не чаще раза в 10 тиков.
        let world_changed = (self.version != Some(world_version) && now - self.tick >= 10)
            || self.cam_chunk != Some(cam_chunk);
        let safety = now - self.tick >= 100 || now < self.tick;
        if !(block_changed || world_changed || safety) {
            return false;
        }
        self.version = Some(world_version);
        self.block_version = Some(block_version);
        self.tick = now;
        self.cam_chunk = Some(cam_chunk);
        self.list.clear();
        let (cx, cz) = cam_chunk;
        for (dx, dz) in (-SCAN_RADIUS..=SCAN_RADIUS)
            .flat_map(|x| (-SCAN_RADIUS..=SCAN_RADIUS).map(move |z| (x, z)))
        {
            let Some(ch) = level.chunks.get(cx + dx, cz + dz) else {
                continue;
            };
            let chunk = &ch.chunk;
            for (si, sec) in chunk.sections().iter().enumerate() {
                if sec.has_only_air() || !sec.states().maybe_has(is_id) {
                    continue;
                }
                let y0 = chunk.min_y() + si as i32 * 16;
                for y in 0..16usize {
                    for z in 0..16usize {
                        for x in 0..16usize {
                            let s = sec.block(x, y, z);
                            if is_id(s.id()) {
                                self.list.push((
                                    [
                                        ch.pos.x * 16 + x as i32,
                                        y0 + y as i32,
                                        ch.pos.z * 16 + z as i32,
                                    ],
                                    s,
                                ));
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

/// Дальность отрисовки BER (getViewDistance = 64): расстояние от центра блока до камеры.
pub fn in_view(p: [i32; 3], cam: [f64; 3]) -> bool {
    let d = |i: usize| f64::from(p[i]) + 0.5 - cam[i];
    d(0) * d(0) + d(1) * d(1) + d(2) * d(2) <= 64.0 * 64.0
}

/// Блок-сущность в загруженном чанке по мировой позиции (тег данных с сервера).
pub fn block_entity_tag(level: &Level, p: [i32; 3]) -> Option<&ezzmc_protocol::nbt::Tag> {
    let ch = level.chunks.get(p[0] >> 4, p[2] >> 4)?;
    ch.block_entities
        .get(&((p[0] & 15) as u8, p[1], (p[2] & 15) as u8))?
        .tag
        .as_ref()
}
