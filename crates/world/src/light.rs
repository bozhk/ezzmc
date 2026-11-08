//! Данные освещения (TODO-121): DataLayer и light-часть пакета чанка.
//! VANILLA: DataLayer, ClientboundLightUpdatePacketData (1.21.11).
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

pub const DATA_LAYER_BYTES: usize = 2048;

/// VANILLA: DataLayer — 16³ нибблов (индекс y<<8|z<<4|x, младший ниббл первым).
/// `None` данных = все значения равны `default`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataLayer {
    data: Option<Box<[u8; DATA_LAYER_BYTES]>>,
    default: u8,
}

impl DataLayer {
    pub fn filled(value: u8) -> Self {
        DataLayer {
            data: None,
            default: value & 15,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let arr: [u8; DATA_LAYER_BYTES] = bytes.try_into().ok()?;
        Some(DataLayer {
            data: Some(Box::new(arr)),
            default: 0,
        })
    }

    #[inline]
    fn index(x: usize, y: usize, z: usize) -> usize {
        y << 8 | z << 4 | x
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        match &self.data {
            None => self.default,
            Some(d) => {
                let i = Self::index(x & 15, y & 15, z & 15);
                (d[i >> 1] >> (4 * (i & 1))) & 15
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, v: u8) {
        let default = self.default;
        let d = self.data.get_or_insert_with(|| {
            let fill = default | default << 4;
            Box::new([fill; DATA_LAYER_BYTES])
        });
        let i = Self::index(x & 15, y & 15, z & 15);
        let shift = 4 * (i & 1);
        d[i >> 1] = d[i >> 1] & !(15 << shift) | (v & 15) << shift;
    }

    /// VANILLA: DataLayer#isEmpty — все значения нули.
    pub fn is_empty(&self) -> bool {
        match &self.data {
            None => self.default == 0,
            Some(d) => d.iter().all(|&b| b == 0),
        }
    }

    pub fn bytes(&self) -> Option<&[u8; DATA_LAYER_BYTES]> {
        self.data.as_deref()
    }
}

/// Одно обновление слоя света для колонки: по секции света (от min_section-1 до max_section+1).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LightUpdate {
    /// (индекс секции света, данные). Индекс 0 = секция под нижней секцией мира.
    pub sky: Vec<(usize, DataLayer)>,
    pub block: Vec<(usize, DataLayer)>,
    /// Секции, которые нужно обнулить (empty*YMask).
    pub empty_sky: Vec<usize>,
    pub empty_block: Vec<usize>,
}

fn bits(mask: &[u64]) -> Vec<usize> {
    let mut v = Vec::new();
    for (w, &word) in mask.iter().enumerate() {
        for b in 0..64 {
            if word >> b & 1 == 1 {
                v.push(w * 64 + b);
            }
        }
    }
    v
}

impl LightUpdate {
    /// VANILLA: ClientboundLightUpdatePacketData(FriendlyByteBuf).
    pub fn read(r: &mut Reader<'_>) -> ProtocolResult<Self> {
        let sky_mask = bits(&r.bitset_bytes()?);
        let block_mask = bits(&r.bitset_bytes()?);
        let empty_sky = bits(&r.bitset_bytes()?);
        let empty_block = bits(&r.bitset_bytes()?);
        let sky = read_layers(r, &sky_mask)?;
        let block = read_layers(r, &block_mask)?;
        Ok(LightUpdate {
            sky,
            block,
            empty_sky,
            empty_block,
        })
    }
}

fn read_layers(r: &mut Reader<'_>, mask: &[usize]) -> ProtocolResult<Vec<(usize, DataLayer)>> {
    // readList(byteArray(2048)): VarInt число массивов; каждый — VarInt длина ≤2048 + байты.
    let n = r.length("light arrays", 1 << 12)?;
    let mut out = Vec::with_capacity(n.min(mask.len()));
    for k in 0..n {
        let len = r.length("light array", DATA_LAYER_BYTES)?;
        let bytes = r.bytes(len)?;
        // Неверный размер слоя — ошибка формата (в ванили DataLayer бросает исключение).
        let layer =
            DataLayer::from_bytes(bytes).ok_or(ezzmc_protocol::ProtocolError::InvalidValue {
                what: "light array length",
                value: len as i64,
            })?;
        if let Some(&idx) = mask.get(k) {
            out.push((idx, layer));
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Writer;

    #[test]
    fn nibble_layout() {
        let mut b = vec![0u8; DATA_LAYER_BYTES];
        b[0] = 0xA5; // x=0 → 5, x=1 → 10
        b[128] = 0x0F; // индекс 256 = (x0,y1,z0) → 15
        let l = DataLayer::from_bytes(&b).unwrap();
        assert_eq!(l.get(0, 0, 0), 5);
        assert_eq!(l.get(1, 0, 0), 10);
        assert_eq!(l.get(0, 1, 0), 15);
        assert_eq!(l.get(0, 0, 1), 0);
    }

    #[test]
    fn set_and_fill() {
        let mut l = DataLayer::filled(15);
        assert_eq!(l.get(3, 4, 5), 15);
        assert!(!l.is_empty());
        l.set(3, 4, 5, 2);
        assert_eq!(l.get(3, 4, 5), 2);
        assert_eq!(l.get(4, 4, 5), 15);
        assert!(DataLayer::filled(0).is_empty());
    }

    #[test]
    fn parse_light_update() {
        let mut w = Writer::new();
        w.bitset_bytes(&[0b101]); // sky: секции 0 и 2
        w.bitset_bytes(&[0b10]); // block: секция 1
        w.bitset_bytes(&[0b1000]); // empty sky: 3
        w.bitset_bytes(&[]);
        w.var_int(2);
        for v in [0x11u8, 0x22] {
            w.var_int(2048);
            w.bytes(&[v; 2048]);
        }
        w.var_int(1);
        w.var_int(2048);
        w.bytes(&[0x33; 2048]);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let u = LightUpdate::read(&mut r).unwrap();
        assert!(r.is_empty());
        assert_eq!(u.sky.len(), 2);
        assert_eq!((u.sky[0].0, u.sky[0].1.get(0, 0, 0)), (0, 1));
        assert_eq!((u.sky[1].0, u.sky[1].1.get(0, 0, 0)), (2, 2));
        assert_eq!((u.block[0].0, u.block[0].1.get(1, 0, 0)), (1, 3));
        assert_eq!(u.empty_sky, vec![3]);
        assert!(u.empty_block.is_empty());
    }

    #[test]
    fn bad_layer_length_errors() {
        let mut w = Writer::new();
        w.bitset_bytes(&[1])
            .bitset_bytes(&[])
            .bitset_bytes(&[])
            .bitset_bytes(&[]);
        w.var_int(1).var_int(10).bytes(&[0; 10]);
        w.var_int(0);
        let buf = w.into_vec();
        assert!(LightUpdate::read(&mut Reader::new(&buf)).is_err());
    }
}
