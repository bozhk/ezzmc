//! Палитровый контейнер секции чанка (TODO-114).
//! VANILLA: PalettedContainer / Strategy / SimpleBitStorage / *Palette (1.21.11).
//!
//! Значения — глобальные id (state id блока или id биома). Палитра (single/linear/
//! hashmap) в рантайме одинаково хранится как `Vec<u32>` с индексом по порядку
//! добавления; различие vanilla-палитр (linear vs hashmap) влияет только на скорость
//! поиска и на бит-конфигурацию, которую мы повторяем точно.
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{ProtocolError, ProtocolResult};

/// Какой набор значений хранит контейнер (Strategy.createForBlockStates / createForBiomes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// 16×16×16 = 4096 записей, 4 бита на ось.
    BlockStates,
    /// 4×4×4 = 64 записи, 2 бита на ось.
    Biomes,
}

impl Kind {
    fn bits_per_axis(self) -> u32 {
        match self {
            Kind::BlockStates => 4,
            Kind::Biomes => 2,
        }
    }
    pub fn entry_count(self) -> usize {
        1 << (self.bits_per_axis() * 3)
    }
    /// VANILLA: Strategy#getIndex — (y << bits | z) << bits | x.
    pub fn index(self, x: usize, y: usize, z: usize) -> usize {
        let b = self.bits_per_axis();
        ((y << b | z) << b) | x
    }
}

/// VANILLA: Strategy#getConfigurationForBitCount → (бит в памяти, глобальная ли палитра).
fn config_for_bits(kind: Kind, bits: u32, global_bits: u32) -> (u32, bool) {
    match (kind, bits) {
        (_, 0) => (0, false),
        (Kind::BlockStates, 1..=4) => (4, false),
        (Kind::BlockStates, 5..=8) => (bits, false),
        (Kind::Biomes, 1..=3) => (bits, false),
        _ => (global_bits, true),
    }
}

/// VANILLA: Mth#ceillog2.
pub fn ceil_log2(n: u32) -> u32 {
    if n <= 1 {
        0
    } else {
        32 - (n - 1).leading_zeros()
    }
}

/// VANILLA: SimpleBitStorage — значения не пересекают границу long.
#[derive(Clone, Debug, PartialEq, Eq)]
struct BitStorage {
    bits: u32,
    per_long: usize,
    mask: u64,
    size: usize,
    data: Vec<u64>,
}

impl BitStorage {
    fn new(bits: u32, size: usize) -> Self {
        debug_assert!((1..=32).contains(&bits));
        let per_long = 64 / bits as usize;
        BitStorage {
            bits,
            per_long,
            mask: (1u64 << bits) - 1,
            size,
            data: vec![0; size.div_ceil(per_long)],
        }
    }
    fn raw_len(bits: u32, size: usize) -> usize {
        size.div_ceil(64 / bits as usize)
    }
    #[inline]
    fn get(&self, i: usize) -> u32 {
        let cell = i / self.per_long;
        let shift = (i % self.per_long) as u32 * self.bits;
        ((self.data[cell] >> shift) & self.mask) as u32
    }
    #[inline]
    fn set(&mut self, i: usize, v: u32) {
        let cell = i / self.per_long;
        let shift = (i % self.per_long) as u32 * self.bits;
        let d = &mut self.data[cell];
        *d = (*d & !(self.mask << shift)) | ((u64::from(v) & self.mask) << shift);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Repr {
    /// SingleValuePalette + ZeroBitStorage.
    Single(u32),
    /// Linear/HashMap-палитра: хранилище содержит индексы палитры.
    Palette {
        palette: Vec<u32>,
        storage: BitStorage,
    },
    /// GlobalPalette: хранилище содержит глобальные id.
    Global(BitStorage),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PalettedContainer {
    kind: Kind,
    global_bits: u32,
    repr: Repr,
}

impl PalettedContainer {
    /// VANILLA: new PalettedContainer(value, strategy).
    pub fn new(kind: Kind, global_bits: u32, value: u32) -> Self {
        PalettedContainer {
            kind,
            global_bits,
            repr: Repr::Single(value),
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Бит на запись в хранилище (0 для single).
    pub fn bits(&self) -> u32 {
        match &self.repr {
            Repr::Single(_) => 0,
            Repr::Palette { storage, .. } | Repr::Global(storage) => storage.bits,
        }
    }

    #[inline]
    pub fn get_index(&self, i: usize) -> u32 {
        match &self.repr {
            Repr::Single(v) => *v,
            Repr::Palette { palette, storage } => {
                // Индекс вне палитры (битый пакет) → 0, как «пустое» значение.
                palette.get(storage.get(i) as usize).copied().unwrap_or(0)
            }
            Repr::Global(storage) => storage.get(i),
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.get_index(self.kind.index(x, y, z))
    }

    /// Все значения в порядке индексов (для мешера/копирования).
    pub fn unpack(&self) -> Vec<u32> {
        (0..self.kind.entry_count())
            .map(|i| self.get_index(i))
            .collect()
    }

    /// VANILLA: PalettedContainer#getAndSet (+ onResize). Возвращает прежнее значение.
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u32) -> u32 {
        self.set_index(self.kind.index(x, y, z), value)
    }

    pub fn set_index(&mut self, i: usize, value: u32) -> u32 {
        let old = self.get_index(i);
        match &mut self.repr {
            Repr::Global(storage) => {
                storage.set(i, value);
                return old;
            }
            Repr::Single(v) if *v == value => return old,
            Repr::Palette { palette, storage } => {
                if let Some(idx) = palette.iter().position(|&p| p == value) {
                    storage.set(i, idx as u32);
                    return old;
                }
                // Linear/hashmap: capacity = 1 << bits.
                if palette.len() < (1usize << storage.bits) {
                    palette.push(value);
                    storage.set(i, (palette.len() - 1) as u32);
                    return old;
                }
            }
            Repr::Single(_) => {}
        }
        // Палитра полна (или single с новым значением) → onResize(bits+1).
        let new_bits = self.bits() + 1;
        let mut all = self.unpack();
        all[i] = value;
        self.repack(&all, new_bits);
        old
    }

    /// Пересобрать контейнер под `bits` (конфигурация как в Strategy), палитра — в порядке
    /// первого появления значений в старой палитре, затем новые.
    fn repack(&mut self, values: &[u32], bits: u32) {
        let (mem_bits, global) = config_for_bits(self.kind, bits, self.global_bits);
        if mem_bits == 0 {
            self.repr = Repr::Single(values[0]);
            return;
        }
        let mut storage = BitStorage::new(mem_bits, values.len());
        if global {
            for (i, &v) in values.iter().enumerate() {
                storage.set(i, v);
            }
            self.repr = Repr::Global(storage);
            return;
        }
        // Сохраняем порядок существующей палитры (индексы стабильны), как copyFrom делает
        // через idFor в порядке обхода — порядок может отличаться от vanilla, на значения
        // это не влияет.
        let mut palette: Vec<u32> = match &self.repr {
            Repr::Palette { palette, .. } => palette.clone(),
            Repr::Single(v) => vec![*v],
            Repr::Global(_) => Vec::new(),
        };
        for (i, &v) in values.iter().enumerate() {
            let idx = match palette.iter().position(|&p| p == v) {
                Some(p) => p,
                None => {
                    palette.push(v);
                    palette.len() - 1
                }
            };
            storage.set(i, idx as u32);
        }
        self.repr = Repr::Palette { palette, storage };
    }

    /// VANILLA: PalettedContainer#read (сетевой формат): byte bits, палитра, long[] фикс. длины.
    pub fn read(kind: Kind, global_bits: u32, r: &mut Reader<'_>) -> ProtocolResult<Self> {
        let bits = r.u8()?;
        if bits > 32 {
            return Err(ProtocolError::InvalidValue {
                what: "paletted container bits",
                value: i64::from(bits),
            });
        }
        let bits = u32::from(bits);
        let (mem_bits, global) = config_for_bits(kind, bits, global_bits);
        let n = kind.entry_count();
        if mem_bits == 0 {
            let v = r.var_int()?;
            let v = to_id(v)?;
            // readFixedSizeLongArray(raw длины 0) — ничего не читает.
            return Ok(PalettedContainer {
                kind,
                global_bits,
                repr: Repr::Single(v),
            });
        }
        let palette = if global {
            None
        } else {
            let len = r.length("palette size", 1 << mem_bits)?;
            let mut p = Vec::with_capacity(len);
            for _ in 0..len {
                p.push(to_id(r.var_int()?)?);
            }
            Some(p)
        };
        let raw = BitStorage::raw_len(mem_bits, n);
        let mut storage = BitStorage::new(mem_bits, n);
        for cell in storage.data.iter_mut().take(raw) {
            *cell = r.i64()? as u64;
        }
        let repr = match palette {
            Some(palette) => Repr::Palette { palette, storage },
            None => Repr::Global(storage),
        };
        Ok(PalettedContainer {
            kind,
            global_bits,
            repr,
        })
    }

    /// VANILLA: PalettedContainer#write.
    pub fn write(&self, w: &mut Writer) {
        match &self.repr {
            Repr::Single(v) => {
                w.u8(0).var_int(*v as i32);
            }
            Repr::Palette { palette, storage } => {
                w.u8(storage.bits as u8).var_int(palette.len() as i32);
                for &p in palette {
                    w.var_int(p as i32);
                }
                for &c in &storage.data {
                    w.i64(c as i64);
                }
            }
            Repr::Global(storage) => {
                w.u8(storage.bits as u8);
                for &c in &storage.data {
                    w.i64(c as i64);
                }
            }
        }
    }

    /// VANILLA: PalettedContainer#maybeHas — есть ли в палитре подходящее значение.
    pub fn maybe_has(&self, f: impl Fn(u32) -> bool) -> bool {
        match &self.repr {
            Repr::Single(v) => f(*v),
            Repr::Palette { palette, .. } => palette.iter().any(|&p| f(p)),
            Repr::Global(_) => true, // GlobalPalette#maybeHas всегда true
        }
    }

    /// Контейнер целиком из одного значения (`Single`)?
    pub fn single_value(&self) -> Option<u32> {
        match &self.repr {
            Repr::Single(v) => Some(*v),
            _ => None,
        }
    }
}

fn to_id(v: i32) -> ProtocolResult<u32> {
    u32::try_from(v).map_err(|_| ProtocolError::InvalidValue {
        what: "palette id",
        value: i64::from(v),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const GB: u32 = 15; // ceillog2(29671)

    #[test]
    fn ceil_log2_matches_vanilla() {
        assert_eq!(ceil_log2(0), 0);
        assert_eq!(ceil_log2(1), 0);
        assert_eq!(ceil_log2(2), 1);
        assert_eq!(ceil_log2(3), 2);
        assert_eq!(ceil_log2(29671), 15);
    }

    #[test]
    fn index_order_is_y_z_x() {
        assert_eq!(Kind::BlockStates.index(1, 0, 0), 1);
        assert_eq!(Kind::BlockStates.index(0, 0, 1), 16);
        assert_eq!(Kind::BlockStates.index(0, 1, 0), 256);
        assert_eq!(Kind::Biomes.index(0, 1, 0), 16);
    }

    #[test]
    fn single_roundtrip() {
        let c = PalettedContainer::new(Kind::BlockStates, GB, 1);
        let mut w = Writer::new();
        c.write(&mut w);
        assert_eq!(w.into_vec(), vec![0, 1]);
        let buf = [0u8, 7];
        let mut r = Reader::new(&buf);
        let c = PalettedContainer::read(Kind::BlockStates, GB, &mut r).unwrap();
        assert_eq!(c.get(3, 4, 5), 7);
        assert!(r.is_empty());
    }

    #[test]
    fn grows_through_all_configs() {
        let mut c = PalettedContainer::new(Kind::BlockStates, GB, 0);
        assert_eq!(c.bits(), 0);
        // 2 значения → 4 бита linear
        c.set(0, 0, 0, 5);
        assert_eq!(c.bits(), 4);
        // много разных значений → hashmap 5..8, затем global
        for i in 0..300u32 {
            let (x, y, z) = (
                (i % 16) as usize,
                (i / 256) as usize,
                ((i / 16) % 16) as usize,
            );
            c.set(x, y, z, 100 + i);
        }
        assert_eq!(c.bits(), GB);
        for i in 0..300u32 {
            let (x, y, z) = (
                (i % 16) as usize,
                (i / 256) as usize,
                ((i / 16) % 16) as usize,
            );
            assert_eq!(c.get(x, y, z), 100 + i);
        }
        assert_eq!(c.get(15, 15, 15), 0);
    }

    #[test]
    fn set_returns_old_and_keeps_bits_when_value_known() {
        let mut c = PalettedContainer::new(Kind::BlockStates, GB, 0);
        assert_eq!(c.set(1, 2, 3, 9), 0);
        let bits = c.bits();
        assert_eq!(c.set(1, 2, 3, 0), 9);
        assert_eq!(c.bits(), bits);
    }

    #[test]
    fn network_roundtrip_all_kinds() {
        for n in [1u32, 2, 3, 10, 40, 300] {
            let mut c = PalettedContainer::new(Kind::BlockStates, GB, 0);
            for i in 0..n {
                c.set(
                    (i % 16) as usize,
                    ((i / 16) % 16) as usize,
                    ((i / 256) % 16) as usize,
                    i * 7,
                );
            }
            let mut w = Writer::new();
            c.write(&mut w);
            let buf = w.into_vec();
            let mut r = Reader::new(&buf);
            let d = PalettedContainer::read(Kind::BlockStates, GB, &mut r).unwrap();
            assert!(r.is_empty());
            assert_eq!(c.unpack(), d.unpack(), "n={n}");
        }
    }

    #[test]
    fn biome_configs() {
        let mut c = PalettedContainer::new(Kind::Biomes, 6, 0);
        for i in 0..8u32 {
            c.set((i % 4) as usize, 0, (i / 4) as usize, i);
        }
        // 8 значений → 3 бита linear
        assert_eq!(c.bits(), 3);
        c.set(0, 3, 0, 20);
        assert_eq!(c.bits(), 6); // 9 значений → global (global_bits)
        assert_eq!(c.get(0, 3, 0), 20);
        assert_eq!(c.get(3, 0, 1), 7);
    }

    #[test]
    fn read_known_bitstorage_layout() {
        // 4 бита, 16 значений на long; значение i = i % 16 в клетке.
        let mut w = Writer::new();
        w.u8(4).var_int(16);
        for p in 0..16 {
            w.var_int(p * 10);
        }
        let cell: u64 = (0..16u64).fold(0, |a, i| a | (i << (i * 4)));
        for _ in 0..256 {
            w.i64(cell as i64);
        }
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let c = PalettedContainer::read(Kind::BlockStates, GB, &mut r).unwrap();
        assert_eq!(c.get(0, 0, 0), 0);
        assert_eq!(c.get(5, 0, 0), 50);
        assert_eq!(c.get(15, 9, 9), 150);
    }

    #[test]
    fn malformed_does_not_panic() {
        let mut r = Reader::new(&[4, 200]);
        assert!(PalettedContainer::read(Kind::BlockStates, GB, &mut r).is_err());
        let mut r = Reader::new(&[33]);
        assert!(PalettedContainer::read(Kind::BlockStates, GB, &mut r).is_err());
        let mut r = Reader::new(&[0, 0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert!(PalettedContainer::read(Kind::BlockStates, GB, &mut r).is_err());
        // индекс вне палитры при чтении
        let mut w = Writer::new();
        w.u8(4).var_int(1).var_int(5);
        for _ in 0..256 {
            w.i64(-1);
        }
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let c = PalettedContainer::read(Kind::BlockStates, GB, &mut r).unwrap();
        assert_eq!(c.get(0, 0, 0), 0);
    }
}
