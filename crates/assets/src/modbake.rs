//! Чтение дампа `.ezbake` (DEC-018, TODO-504): состояния блоков, запечённые квады, формы,
//! свет и атлас из Forge-клиента 1.20.1 с модами. Пишет его мод-дампер `tools/forge-bake/`,
//! формат — `docs/ref/forge-bake.md`. Файл открывается через mmap; квады моделей
//! разбираются лениво (`ModBake::model`), заголовки — сразу.
use crate::atlas::Identifier;
use crate::texture::Image;
use std::fs::File;
use std::ops::Range;
use std::path::Path;

pub const MAGIC: &[u8; 4] = b"EZBK";
pub const VERSION: u32 = 3;
/// Размер квада в файле: 9 байт заголовка + 4 вершины × (xyz f32, цвет u32, uv f32×2).
pub const QUAD_SIZE: usize = 9 + 4 * 24;
/// Индекс «нет» в таблицах (модель/форма не снята — исключение в дампере).
pub const NONE: u32 = u32::MAX;

#[derive(Debug, thiserror::Error)]
pub enum BakeError {
    #[error("ezbake: ввод-вывод: {0}")]
    Io(#[from] std::io::Error),
    #[error("ezbake: не тот формат (магия/версия {0})")]
    Format(u32),
    #[error("ezbake: файл обрезан на смещении {0}")]
    Truncated(usize),
    #[error("ezbake: битая строка UTF-8 на смещении {0}")]
    Utf8(usize),
}

pub type BakeResult<T> = Result<T, BakeError>;

enum Buf {
    Map(memmap2::Mmap),
    Vec(Vec<u8>),
}

impl std::ops::Deref for Buf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            Buf::Map(m) => m,
            Buf::Vec(v) => v,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BakeSprite {
    /// `namespace:path` спрайта (`minecraft:block/stone`).
    pub name: String,
    /// 0 — атлас блоков (есть в дампе), иначе другой атлас (пикселей нет).
    pub atlas: u8,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[derive(Clone, Debug)]
pub struct BakeProp {
    pub name: String,
    /// Значения в порядке `Property#getPossibleValues` (как их нумерует ваниль/Forge).
    pub values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct BakeBlock {
    /// `namespace:path`.
    pub name: String,
    /// Свойства в порядке `StateDefinition` (по имени).
    pub props: Vec<BakeProp>,
    /// Глобальный id состояния по умолчанию (в нумерации клиента, где пекли).
    pub default_state: u32,
}

/// Флаги состояния (`BakeState::flags`).
pub mod flag {
    pub const CAN_OCCLUDE: u8 = 1;
    pub const SOLID_RENDER: u8 = 2;
    pub const AIR: u8 = 4;
    pub const BLOCK_ENTITY: u8 = 8;
    pub const AMBIENT_OCCLUSION: u8 = 16;
    pub const SKYLIGHT_DOWN: u8 = 32;
    pub const FLUID: u8 = 64;
    /// Дампер поймал исключение на этом состоянии — данные неполные.
    pub const ERROR: u8 = 128;
}

#[derive(Clone, Copy, Debug)]
pub struct BakeState {
    /// Индекс в `ModBake::blocks`, `NONE` — дыра в реестре.
    pub block: u32,
    pub model: u32,
    pub collision: u32,
    pub outline: u32,
    pub occlusion: u32,
    pub light: u8,
    pub light_block: u8,
    pub flags: u8,
    /// Диапазон в `ModBake::tints` (по индексу тинта 0..).
    tints: (u32, u8),
}

/// Тинт для `tint_index`, снятый дампером без уровня (`BlockColors#getColor(state, null, null, i)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BakeTint {
    /// Обработчика нет (-1) — белый.
    None,
    /// Константный цвет 0xRRGGBB (старший байт игнорируется).
    Color(u32),
    /// Обработчик требует уровень/позицию (обычно биомный цвет) — TODO-509.
    NeedsLevel,
}

#[derive(Clone, Copy, Debug)]
pub struct BakeQuad {
    /// Сторона отсечения (0..5 = DOWN, UP, NORTH, SOUTH, WEST, EAST), `None` — без отсечения.
    pub cull: Option<u8>,
    /// Направление квада (`BakedQuad#getDirection`, 0..5).
    pub face: u8,
    /// `tintIndex`, -1 — без тинта.
    pub tint: i32,
    pub shade: bool,
    pub ao: bool,
    /// 0 solid, 1 cutout_mipped, 2 cutout, 3 translucent, 4 tripwire, 5 прочие.
    pub layer: u8,
    pub sprite: u32,
    /// Позиции в пространстве блока (0..1).
    pub pos: [[f32; 3]; 4],
    /// Цвет вершины (ABGR как в `BakedQuad`; обычно -1).
    pub color: [u32; 4],
    /// UV внутри спрайта (0..1).
    pub uv: [[f32; 2]; 4],
}

/// Один взвешенный вариант модели (`WeightedBakedModel`); у простых моделей он единственный.
#[derive(Clone, Copy, Debug)]
pub struct BakeVariant<'a> {
    pub weight: u32,
    quads: &'a [u8],
}

impl<'a> BakeVariant<'a> {
    pub fn len(&self) -> usize {
        self.quads.len() / QUAD_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.quads.is_empty()
    }

    pub fn quads(&self) -> impl Iterator<Item = BakeQuad> + 'a {
        self.quads.chunks_exact(QUAD_SIZE).map(parse_quad)
    }
}

pub struct ModBake {
    buf: Buf,
    pub atlas_size: (u32, u32),
    atlas_png: Range<usize>,
    pub sprites: Vec<BakeSprite>,
    pub blocks: Vec<BakeBlock>,
    /// Смещение каждой модели в файле (начало `u8 nvariants`).
    models: Vec<usize>,
    pub shapes: Vec<Vec<[f32; 6]>>,
    pub states: Vec<BakeState>,
    tints: Vec<BakeTint>,
    /// v2: модовые предметы в порядке реестра дампа (иконка — `item_icon`).
    pub items: Vec<BakeItem>,
    by_item: std::collections::HashMap<String, u32>,
    /// v2: размер клетки иконки и страницы атласа иконок (px).
    pub icon_cell: u32,
    pub icon_page: u32,
    icon_pages: Vec<Range<usize>>,
    /// v3: переводы языка игры, где пекли (ваниль + моды).
    pub lang: std::collections::HashMap<String, String>,
    /// v3: схемы модовых меню (снимок открытого экрана).
    pub menus: Vec<BakeMenu>,
}

/// Слот схемы меню (координаты относительно левого верхнего угла окна `img`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BakeSlot {
    pub x: i16,
    pub y: i16,
    /// 1 — `Slot#isActive`, 2 — слот инвентаря игрока.
    pub flags: u8,
}

/// Схема модового меню (v3): размер окна, фон (PNG, смещение от угла окна) и слоты по порядку меню.
#[derive(Clone, Debug)]
pub struct BakeMenu {
    pub name: String,
    /// Блок, на котором сняли.
    pub block: String,
    pub img_w: u16,
    pub img_h: u16,
    pub bg_dx: i16,
    pub bg_dy: i16,
    pub bg_w: u16,
    pub bg_h: u16,
    png: Range<usize>,
    pub slots: Vec<BakeSlot>,
}

/// Модовый предмет дампа (v2).
#[derive(Clone, Debug)]
pub struct BakeItem {
    pub name: String,
    /// Иконка снялась без исключения.
    pub icon_ok: bool,
}

struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    fn take(&mut self, n: usize) -> BakeResult<&'a [u8]> {
        let end = self.p.checked_add(n).ok_or(BakeError::Truncated(self.p))?;
        let s = self
            .b
            .get(self.p..end)
            .ok_or(BakeError::Truncated(self.p))?;
        self.p = end;
        Ok(s)
    }
    fn u8(&mut self) -> BakeResult<u8> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> BakeResult<u16> {
        let s = self.take(2)?;
        Ok(u16::from_le_bytes([s[0], s[1]]))
    }
    fn u32(&mut self) -> BakeResult<u32> {
        let s = self.take(4)?;
        Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn f32(&mut self) -> BakeResult<f32> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn str(&mut self) -> BakeResult<String> {
        let n = self.u16()? as usize;
        let at = self.p;
        let s = self.take(n)?;
        std::str::from_utf8(s)
            .map(str::to_owned)
            .map_err(|_| BakeError::Utf8(at))
    }
}

fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

fn parse_quad(q: &[u8]) -> BakeQuad {
    let mut pos = [[0.0; 3]; 4];
    let mut color = [0; 4];
    let mut uv = [[0.0; 2]; 4];
    for i in 0..4 {
        let b = 9 + i * 24;
        let f = |k: usize| f32::from_bits(le32(q, b + k * 4));
        pos[i] = [f(0), f(1), f(2)];
        color[i] = le32(q, b + 12);
        uv[i] = [f(4), f(5)];
    }
    BakeQuad {
        cull: (q[0] < 6).then_some(q[0]),
        face: q[1],
        tint: if q[2] == 0xFF { -1 } else { i32::from(q[2]) },
        shade: q[3] & 1 != 0,
        ao: q[3] & 2 != 0,
        layer: q[4],
        sprite: le32(q, 5),
        pos,
        color,
        uv,
    }
}

impl ModBake {
    pub fn open(path: &Path) -> BakeResult<Self> {
        let f = File::open(path)?;
        // SAFETY: файл дампа неизменяем после записи (дампер пишет во .tmp и переименовывает).
        let map = unsafe { memmap2::Mmap::map(&f)? };
        Self::parse(Buf::Map(map))
    }

    pub fn from_bytes(data: Vec<u8>) -> BakeResult<Self> {
        Self::parse(Buf::Vec(data))
    }

    fn parse(buf: Buf) -> BakeResult<Self> {
        let mut r = Rd { b: &buf, p: 0 };
        if r.take(4)? != MAGIC {
            return Err(BakeError::Format(0));
        }
        let ver = r.u32()?;
        if !(1..=VERSION).contains(&ver) {
            return Err(BakeError::Format(ver));
        }
        let aw = r.u32()?;
        let ah = r.u32()?;
        let png_len = r.u32()? as usize;
        let png_at = r.p;
        r.take(png_len)?;
        let n = r.u32()? as usize;
        let mut sprites = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            sprites.push(BakeSprite {
                name: r.str()?,
                atlas: r.u8()?,
                x: r.u16()?,
                y: r.u16()?,
                w: r.u16()?,
                h: r.u16()?,
            });
        }
        let n = r.u32()? as usize;
        let mut blocks = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            let name = r.str()?;
            let np = r.u8()?;
            let mut props = Vec::with_capacity(np as usize);
            for _ in 0..np {
                let pname = r.str()?;
                let nv = r.u16()?;
                let mut values = Vec::with_capacity(nv as usize);
                for _ in 0..nv {
                    values.push(r.str()?);
                }
                props.push(BakeProp {
                    name: pname,
                    values,
                });
            }
            blocks.push(BakeBlock {
                name,
                props,
                default_state: r.u32()?,
            });
        }
        let n = r.u32()? as usize;
        let mut models = Vec::with_capacity(n.min(1 << 24));
        for _ in 0..n {
            models.push(r.p);
            let nv = r.u8()?;
            for _ in 0..nv {
                r.u32()?;
                let nq = r.u32()? as usize;
                r.take(nq.checked_mul(QUAD_SIZE).ok_or(BakeError::Truncated(r.p))?)?;
            }
        }
        let n = r.u32()? as usize;
        let mut shapes = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            let nb = r.u16()?;
            let mut boxes = Vec::with_capacity(nb as usize);
            for _ in 0..nb {
                boxes.push([r.f32()?, r.f32()?, r.f32()?, r.f32()?, r.f32()?, r.f32()?]);
            }
            shapes.push(boxes);
        }
        let n = r.u32()? as usize;
        let mut states = Vec::with_capacity(n.min(1 << 24));
        let mut tints = Vec::new();
        for _ in 0..n {
            let block = r.u32()?;
            if block == NONE {
                states.push(BakeState {
                    block,
                    model: NONE,
                    collision: NONE,
                    outline: NONE,
                    occlusion: NONE,
                    light: 0,
                    light_block: 0,
                    flags: flag::ERROR,
                    tints: (0, 0),
                });
                continue;
            }
            let model = r.u32()?;
            let collision = r.u32()?;
            let outline = r.u32()?;
            let occlusion = r.u32()?;
            let light = r.u8()?;
            let light_block = r.u8()?;
            let flags = r.u8()?;
            let nt = r.u8()?;
            let t0 = tints.len() as u32;
            for _ in 0..nt {
                let status = r.u8()?;
                let c = r.u32()?;
                tints.push(match status {
                    1 => BakeTint::Color(c & 0xFF_FFFF),
                    2 => BakeTint::NeedsLevel,
                    _ => BakeTint::None,
                });
            }
            states.push(BakeState {
                block,
                model,
                collision,
                outline,
                occlusion,
                light,
                light_block,
                flags,
                tints: (t0, nt),
            });
        }
        let (mut items, mut icon_cell, mut icon_page, mut icon_pages) =
            (Vec::new(), 0, 0, Vec::new());
        if ver >= 2 {
            let n = r.u32()? as usize;
            items.reserve(n.min(1 << 20));
            for _ in 0..n {
                items.push(BakeItem {
                    name: r.str()?,
                    icon_ok: r.u8()? != 0,
                });
            }
            icon_cell = r.u32()?;
            icon_page = r.u32()?;
            let np = r.u32()? as usize;
            for _ in 0..np {
                let len = r.u32()? as usize;
                let at = r.p;
                r.take(len)?;
                icon_pages.push(at..at + len);
            }
        }
        let mut lang = std::collections::HashMap::new();
        let mut menus = Vec::new();
        if ver >= 3 {
            let n = r.u32()? as usize;
            lang.reserve(n.min(1 << 20));
            for _ in 0..n {
                let k = r.str()?;
                lang.insert(k, r.str()?);
            }
            let n = r.u32()? as usize;
            for _ in 0..n {
                let name = r.str()?;
                let block = r.str()?;
                let (img_w, img_h) = (r.u16()?, r.u16()?);
                let (bg_dx, bg_dy) = (r.u16()? as i16, r.u16()? as i16);
                let (bg_w, bg_h) = (r.u16()?, r.u16()?);
                let len = r.u32()? as usize;
                let at = r.p;
                r.take(len)?;
                let ns = r.u16()?;
                let mut slots = Vec::with_capacity(ns as usize);
                for _ in 0..ns {
                    slots.push(BakeSlot {
                        x: r.u16()? as i16,
                        y: r.u16()? as i16,
                        flags: r.u8()?,
                    });
                }
                menus.push(BakeMenu {
                    name,
                    block,
                    img_w,
                    img_h,
                    bg_dx,
                    bg_dy,
                    bg_w,
                    bg_h,
                    png: at..at + len,
                    slots,
                });
            }
        }
        let by_item = items
            .iter()
            .enumerate()
            .map(|(i, it)| (it.name.clone(), i as u32))
            .collect();
        Ok(ModBake {
            lang,
            menus,
            items,
            by_item,
            icon_cell,
            icon_page,
            icon_pages,
            atlas_size: (aw, ah),
            atlas_png: png_at..png_at + png_len,
            sprites,
            blocks,
            models,
            shapes,
            states,
            tints,
            buf,
        })
    }

    /// PNG атласа блоков (уровень 0) клиента, где пекли.
    pub fn atlas_png(&self) -> &[u8] {
        &self.buf[self.atlas_png.clone()]
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Варианты модели `idx`; `NONE`/вне диапазона → пусто.
    pub fn model(&self, idx: u32) -> Vec<BakeVariant<'_>> {
        let Some(&at) = self.models.get(idx as usize) else {
            return Vec::new();
        };
        let b: &[u8] = &self.buf;
        let mut p = at;
        let nv = b[p];
        p += 1;
        let mut out = Vec::with_capacity(nv as usize);
        for _ in 0..nv {
            let weight = le32(b, p);
            let nq = le32(b, p + 4) as usize;
            p += 8;
            out.push(BakeVariant {
                weight,
                quads: &b[p..p + nq * QUAD_SIZE],
            });
            p += nq * QUAD_SIZE;
        }
        out
    }

    pub fn tint(&self, state: &BakeState, index: i32) -> BakeTint {
        let (t0, n) = state.tints;
        if index < 0 || index >= i32::from(n) {
            return BakeTint::None;
        }
        self.tints[t0 as usize + index as usize]
    }

    pub fn shape(&self, idx: u32) -> Option<&[[f32; 6]]> {
        self.shapes.get(idx as usize).map(Vec::as_slice)
    }

    /// Кадры спрайтов атласа блоков (вырезаны из PNG дампа; анимации — первый кадр).
    pub fn sprite_images(&self) -> Vec<(Identifier, Image)> {
        let atlas = match Image::decode_png(self.atlas_png()) {
            Ok(a) => a,
            Err(e) => {
                tracing::warn!("ezbake: атлас не декодирован: {e}");
                return Vec::new();
            }
        };
        self.sprites
            .iter()
            .filter(|s| {
                s.atlas == 0
                    && s.w > 0
                    && s.h > 0
                    && u32::from(s.x) + u32::from(s.w) <= atlas.width
                    && u32::from(s.y) + u32::from(s.h) <= atlas.height
            })
            .map(|s| {
                let id = Identifier::parse(&s.name);
                let img = atlas.crop(
                    u32::from(s.x),
                    u32::from(s.y),
                    u32::from(s.w),
                    u32::from(s.h),
                );
                (id, img)
            })
            .collect()
    }

    /// Индекс блока по имени (`namespace:path`).
    /// Схема модового меню по полному имени типа меню.
    pub fn menu(&self, name: &str) -> Option<&BakeMenu> {
        self.menus.iter().find(|m| m.name == name)
    }

    /// PNG фона меню (пусто — фон не снялся).
    pub fn menu_png(&self, m: &BakeMenu) -> &[u8] {
        &self.buf[m.png.clone()]
    }

    /// Перевод из языка дампа (ключи модов).
    pub fn tr(&self, key: &str) -> Option<&str> {
        self.lang.get(key).map(String::as_str)
    }

    /// Индекс модового предмета по полному имени.
    pub fn item_by_name(&self, name: &str) -> Option<u32> {
        self.by_item.get(name).copied()
    }

    /// PNG страницы атласа иконок предметов.
    pub fn icon_page_png(&self, page: usize) -> Option<&[u8]> {
        self.icon_pages.get(page).map(|r| &self.buf[r.clone()])
    }

    pub fn icon_page_count(&self) -> usize {
        self.icon_pages.len()
    }

    /// Иконка предмета `i`: (страница, x, y, размер) в px; `None` — иконки нет.
    pub fn item_icon(&self, i: u32) -> Option<(usize, u32, u32, u32)> {
        let it = self.items.get(i as usize)?;
        if !it.icon_ok || self.icon_cell == 0 {
            return None;
        }
        let per_row = self.icon_page / self.icon_cell;
        let per_page = per_row * per_row;
        let c = i % per_page;
        Some((
            (i / per_page) as usize,
            (c % per_row) * self.icon_cell,
            (c / per_row) * self.icon_cell,
            self.icon_cell,
        ))
    }

    pub fn block_by_name(&self, name: &str) -> Option<usize> {
        self.blocks.iter().position(|b| b.name == name)
    }
}

/// Дамп текущего процесса: путь из `EZZMC_MOD_BAKE` (ставит клиент из `--mod-bake`).
/// Читается один раз; ошибка открытия — предупреждение и работа без модов.
pub fn global() -> Option<&'static ModBake> {
    static G: std::sync::OnceLock<Option<ModBake>> = std::sync::OnceLock::new();
    G.get_or_init(|| {
        let p = std::env::var_os("EZZMC_MOD_BAKE")?;
        match ModBake::open(Path::new(&p)) {
            Ok(b) => {
                tracing::info!(
                    "ezbake: {} блоков, {} состояний, {} моделей",
                    b.blocks.len(),
                    b.states.len(),
                    b.model_count()
                );
                Some(b)
            }
            Err(e) => {
                tracing::warn!("ezbake {}: {e}", Path::new(&p).display());
                None
            }
        }
    })
    .as_ref()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    struct W(Vec<u8>);
    impl W {
        fn u8(&mut self, v: u8) -> &mut Self {
            self.0.push(v);
            self
        }
        fn u16(&mut self, v: u16) -> &mut Self {
            self.0.extend_from_slice(&v.to_le_bytes());
            self
        }
        fn u32(&mut self, v: u32) -> &mut Self {
            self.0.extend_from_slice(&v.to_le_bytes());
            self
        }
        fn f32(&mut self, v: f32) -> &mut Self {
            self.u32(v.to_bits())
        }
        fn str(&mut self, s: &str) -> &mut Self {
            self.u16(s.len() as u16);
            self.0.extend_from_slice(s.as_bytes());
            self
        }
    }

    fn synthetic() -> Vec<u8> {
        let mut w = W(Vec::new());
        w.0.extend_from_slice(MAGIC);
        w.u32(VERSION).u32(16).u32(16).u32(3);
        w.0.extend_from_slice(b"png");
        w.u32(1)
            .str("mymod:block/ore")
            .u8(0)
            .u16(0)
            .u16(0)
            .u16(16)
            .u16(16);
        w.u32(1)
            .str("mymod:ore")
            .u8(1)
            .str("lit")
            .u16(2)
            .str("true")
            .str("false");
        w.u32(5);
        // 1 модель, 1 вариант, 1 квад (верх, отсечение UP, тинт 0)
        w.u32(1).u8(1).u32(1).u32(1);
        w.u8(1).u8(1).u8(0).u8(1).u8(0).u32(0);
        for i in 0..4 {
            w.f32(i as f32 * 0.25).f32(1.0).f32(0.0).u32(u32::MAX);
            w.f32(0.5).f32(0.75);
        }
        w.u32(1).u16(1);
        for v in [0.0, 0.0, 0.0, 1.0, 1.0, 1.0] {
            w.f32(v);
        }
        w.u32(2);
        for lit in [true, false] {
            w.u32(0).u32(0).u32(0).u32(0).u32(0);
            w.u8(if lit { 9 } else { 0 }).u8(15).u8(flag::CAN_OCCLUDE);
            w.u8(1).u8(if lit { 1 } else { 2 }).u32(0xFF12_3456);
        }
        // v2: 2 предмета, клетка 32, страница 64 (4 на страницу), 1 страница
        w.u32(2).str("mymod:gear").u8(1).str("mymod:broken").u8(0);
        w.u32(32).u32(64).u32(1).u32(3);
        w.0.extend_from_slice(b"PNG");
        // v3: язык и одно меню
        w.u32(1).str("block.mymod.ore").str("Руда");
        w.u32(1).str("mymod:furnace").str("mymod:ore");
        w.u16(176)
            .u16(166)
            .u16((-20i16) as u16)
            .u16(0)
            .u16(196)
            .u16(166)
            .u32(2);
        w.0.extend_from_slice(b"BG");
        w.u16(2).u16(56).u16(17).u8(1).u16(8).u16(84).u8(3);
        w.0
    }

    #[test]
    fn parse_synthetic() {
        let b = ModBake::from_bytes(synthetic()).unwrap();
        assert_eq!(b.atlas_png(), b"png");
        assert_eq!(b.sprites[0].name, "mymod:block/ore");
        assert_eq!(b.blocks[0].props[0].values, ["true", "false"]);
        assert_eq!(b.blocks[0].default_state, 5);
        let v = b.model(0);
        assert_eq!(v.len(), 1);
        let q: Vec<_> = v[0].quads().collect();
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].cull, Some(1));
        assert_eq!(q[0].tint, 0);
        assert!(q[0].shade && !q[0].ao);
        assert_eq!(q[0].pos[2], [0.5, 1.0, 0.0]);
        assert_eq!(q[0].uv[3], [0.5, 0.75]);
        assert_eq!(b.shape(0).unwrap(), &[[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]);
        assert_eq!(b.states[0].light, 9);
        assert_eq!(b.tint(&b.states[0], 0), BakeTint::Color(0x12_3456));
        assert_eq!(b.tint(&b.states[1], 0), BakeTint::NeedsLevel);
        assert_eq!(b.tint(&b.states[1], 3), BakeTint::None);
        assert_eq!(b.item_by_name("mymod:broken"), Some(1));
        assert_eq!(b.item_icon(0), Some((0, 0, 0, 32)));
        assert_eq!(b.item_icon(1), None);
        assert_eq!(b.icon_page_png(0), Some(&b"PNG"[..]));
        assert_eq!(b.tr("block.mymod.ore"), Some("Руда"));
        let m = b.menu("mymod:furnace").unwrap();
        assert_eq!((m.img_w, m.bg_dx, m.bg_w), (176, -20, 196));
        assert_eq!(b.menu_png(m), b"BG");
        assert_eq!(
            m.slots[1],
            BakeSlot {
                x: 8,
                y: 84,
                flags: 3
            }
        );
        assert!(b.model(7).is_empty());
    }

    #[test]
    fn truncated_is_error() {
        let mut d = synthetic();
        d.truncate(d.len() - 3);
        assert!(matches!(
            ModBake::from_bytes(d),
            Err(BakeError::Truncated(_))
        ));
    }

    /// Реальный дамп: `EZZMC_BAKE=<файл.ezbake> cargo test -p ezzmc-assets real_dump -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn real_dump() {
        let Ok(p) = std::env::var("EZZMC_BAKE") else {
            return;
        };
        let t = std::time::Instant::now();
        let b = ModBake::open(Path::new(&p)).unwrap();
        println!(
            "open {:?}: blocks={} states={} models={} shapes={} sprites={} atlas={:?}",
            t.elapsed(),
            b.blocks.len(),
            b.states.len(),
            b.model_count(),
            b.shapes.len(),
            b.sprites.len(),
            b.atlas_size
        );
        // камень: 6 граней с отсечением, спрайт minecraft:block/stone, полная коллизия
        let stone = b.block_by_name("minecraft:stone").unwrap();
        let st = b.states[b.blocks[stone].default_state as usize];
        assert_eq!(st.block as usize, stone);
        let v = b.model(st.model);
        let q: Vec<_> = v[0].quads().collect();
        assert_eq!(q.len(), 6);
        assert!(q.iter().all(|q| q.cull.is_some()));
        assert_eq!(
            b.sprites[q[0].sprite as usize].name,
            "minecraft:block/stone"
        );
        assert_eq!(
            b.shape(st.collision).unwrap(),
            &[[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]
        );
        assert!(st.flags & flag::SOLID_RENDER != 0);
        // модовых блоков много, и у большинства есть квады
        let modded = b
            .blocks
            .iter()
            .filter(|bl| !bl.name.starts_with("minecraft:"))
            .count();
        let empty = b
            .blocks
            .iter()
            .filter(|bl| !bl.name.starts_with("minecraft:"))
            .filter(|bl| {
                let s = b.states[bl.default_state as usize];
                b.model(s.model).iter().all(|v| v.is_empty())
            })
            .count();
        let errors = b
            .states
            .iter()
            .filter(|s| s.flags & flag::ERROR != 0)
            .count();
        println!("модовых блоков {modded}, без квадов {empty}, состояний с ошибкой {errors}");
        assert!(modded > 1000);
    }
}
