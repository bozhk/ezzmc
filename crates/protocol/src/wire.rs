//! Примитивы проводного формата (TODO-048): VarInt/VarLong, строки, UUID, позиции,
//! углы, bitset, optional. Чтение никогда не паникует — только `ProtocolError`.
// VANILLA: FriendlyByteBuf / VarInt / VarLong (1.21.11)
use crate::{ProtocolError, ProtocolResult};

pub const VARINT_MAX_BYTES: usize = 5;
pub const VARLONG_MAX_BYTES: usize = 10;
/// VANILLA: FriendlyByteBuf#readUtf — максимум в символах; в байтах до `max * 3`.
pub const DEFAULT_STRING_MAX: usize = 32767;

/// Читатель по срезу (без копирования).
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
    /// Байты от `start` до текущей позиции.
    pub fn since(&self, start: usize) -> &'a [u8] {
        &self.buf[start..self.pos]
    }
    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }
    pub fn rest(&mut self) -> &'a [u8] {
        let s = &self.buf[self.pos..];
        self.pos = self.buf.len();
        s
    }
    pub fn bytes(&mut self, n: usize) -> ProtocolResult<&'a [u8]> {
        if self.remaining() < n {
            return Err(ProtocolError::Eof {
                needed: n - self.remaining(),
            });
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }
    fn array<const N: usize>(&mut self) -> ProtocolResult<[u8; N]> {
        let s = self.bytes(N)?;
        let mut a = [0u8; N];
        a.copy_from_slice(s);
        Ok(a)
    }
    pub fn u8(&mut self) -> ProtocolResult<u8> {
        Ok(self.array::<1>()?[0])
    }
    pub fn i8(&mut self) -> ProtocolResult<i8> {
        Ok(self.u8()? as i8)
    }
    pub fn bool(&mut self) -> ProtocolResult<bool> {
        Ok(self.u8()? != 0)
    }
    pub fn u16(&mut self) -> ProtocolResult<u16> {
        Ok(u16::from_be_bytes(self.array()?))
    }
    pub fn i16(&mut self) -> ProtocolResult<i16> {
        Ok(i16::from_be_bytes(self.array()?))
    }
    pub fn i32(&mut self) -> ProtocolResult<i32> {
        Ok(i32::from_be_bytes(self.array()?))
    }
    pub fn i64(&mut self) -> ProtocolResult<i64> {
        Ok(i64::from_be_bytes(self.array()?))
    }
    pub fn f32(&mut self) -> ProtocolResult<f32> {
        Ok(f32::from_be_bytes(self.array()?))
    }
    pub fn f64(&mut self) -> ProtocolResult<f64> {
        Ok(f64::from_be_bytes(self.array()?))
    }
    pub fn var_int(&mut self) -> ProtocolResult<i32> {
        let mut v: u32 = 0;
        for i in 0..VARINT_MAX_BYTES {
            let b = self.u8()?;
            v |= u32::from(b & 0x7F) << (7 * i);
            if b & 0x80 == 0 {
                return Ok(v as i32);
            }
        }
        Err(ProtocolError::VarTooLong {
            max: VARINT_MAX_BYTES,
        })
    }
    pub fn var_long(&mut self) -> ProtocolResult<i64> {
        let mut v: u64 = 0;
        for i in 0..VARLONG_MAX_BYTES {
            let b = self.u8()?;
            v |= u64::from(b & 0x7F) << (7 * i);
            if b & 0x80 == 0 {
                return Ok(v as i64);
            }
        }
        Err(ProtocolError::VarTooLong {
            max: VARLONG_MAX_BYTES,
        })
    }
    /// Длина-префикс (VarInt ≥ 0, ≤ limit).
    pub fn length(&mut self, what: &'static str, limit: usize) -> ProtocolResult<usize> {
        let n = self.var_int()?;
        if n < 0 {
            return Err(ProtocolError::NegativeLength(i64::from(n)));
        }
        let n = n as usize;
        if n > limit {
            return Err(ProtocolError::TooLarge {
                what,
                len: n,
                limit,
            });
        }
        Ok(n)
    }
    /// VANILLA: readUtf(maxLen). Лимит в байтах = max*3, в символах (UTF-16 единицах) = max.
    pub fn string(&mut self, max_chars: usize) -> ProtocolResult<String> {
        let n = self.length("string bytes", max_chars * 3)?;
        let s = std::str::from_utf8(self.bytes(n)?).map_err(|_| ProtocolError::BadUtf8)?;
        let chars = s.encode_utf16().count();
        if chars > max_chars {
            return Err(ProtocolError::TooLarge {
                what: "string chars",
                len: chars,
                limit: max_chars,
            });
        }
        Ok(s.to_owned())
    }
    pub fn uuid(&mut self) -> ProtocolResult<u128> {
        Ok(u128::from_be_bytes(self.array()?))
    }
    /// Позиция блока (1.14+): x 26 бит | z 26 бит | y 12 бит, со знаком.
    pub fn block_pos(&mut self) -> ProtocolResult<(i32, i32, i32)> {
        Ok(unpack_block_pos(self.i64()?))
    }
    /// Угол: 1 байт = 1/256 оборота (градусы).
    pub fn angle(&mut self) -> ProtocolResult<f32> {
        Ok(f32::from(self.u8()?) * 360.0 / 256.0)
    }
    /// BitSet: VarInt длина в long'ах + long'и.
    pub fn bitset(&mut self) -> ProtocolResult<Vec<u64>> {
        let n = self.length("bitset longs", 1 << 16)?;
        let mut v = Vec::with_capacity(n.min(self.remaining() / 8));
        for _ in 0..n {
            v.push(self.i64()? as u64);
        }
        Ok(v)
    }
    /// VANILLA: ByteBufCodecs.BIT_SET (26.x): BitSet.valueOf(byte[]) — VarInt байт, little-endian.
    pub fn bitset_bytes(&mut self) -> ProtocolResult<Vec<u64>> {
        let n = self.length("bitset bytes", 1 << 19)?;
        let b = self.bytes(n)?;
        Ok(b.chunks(8)
            .map(|c| {
                c.iter()
                    .enumerate()
                    .fold(0u64, |a, (i, &x)| a | u64::from(x) << (8 * i))
            })
            .collect())
    }
    /// Optional: bool-префикс.
    pub fn optional<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> ProtocolResult<T>,
    ) -> ProtocolResult<Option<T>> {
        if self.bool()? {
            Ok(Some(f(self)?))
        } else {
            Ok(None)
        }
    }
    /// Enum как VarInt, значение проверяет вызывающий.
    pub fn enum_id(&mut self) -> ProtocolResult<i32> {
        self.var_int()
    }
}

pub fn unpack_block_pos(v: i64) -> (i32, i32, i32) {
    let x = (v >> 38) as i32;
    let y = (v << 52 >> 52) as i32;
    let z = (v << 26 >> 38) as i32;
    (x, y, z)
}

pub fn pack_block_pos(x: i32, y: i32, z: i32) -> i64 {
    ((i64::from(x) & 0x3FF_FFFF) << 38)
        | ((i64::from(z) & 0x3FF_FFFF) << 12)
        | (i64::from(y) & 0xFFF)
}

/// Писатель в `Vec<u8>`.
#[derive(Clone, Debug, Default)]
pub struct Writer {
    pub buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(n: usize) -> Self {
        Self {
            buf: Vec::with_capacity(n),
        }
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(b);
        self
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.u8(u8::from(v))
    }
    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn i16(&mut self, v: i16) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn i64(&mut self, v: i64) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn f64(&mut self, v: f64) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn var_int(&mut self, v: i32) -> &mut Self {
        let mut v = v as u32;
        loop {
            if v & !0x7F == 0 {
                return self.u8(v as u8);
            }
            self.u8((v & 0x7F) as u8 | 0x80);
            v >>= 7;
        }
    }
    pub fn var_long(&mut self, v: i64) -> &mut Self {
        let mut v = v as u64;
        loop {
            if v & !0x7F == 0 {
                return self.u8(v as u8);
            }
            self.u8((v & 0x7F) as u8 | 0x80);
            v >>= 7;
        }
    }
    pub fn string(&mut self, s: &str) -> &mut Self {
        self.var_int(s.len() as i32);
        self.bytes(s.as_bytes())
    }
    pub fn uuid(&mut self, v: u128) -> &mut Self {
        self.bytes(&v.to_be_bytes())
    }
    pub fn block_pos(&mut self, x: i32, y: i32, z: i32) -> &mut Self {
        self.i64(pack_block_pos(x, y, z))
    }
    pub fn angle(&mut self, degrees: f32) -> &mut Self {
        self.u8((degrees * 256.0 / 360.0).floor() as i32 as u8)
    }
    /// 26.x BIT_SET: VarInt байт (без старших нулевых) + little-endian.
    pub fn bitset_bytes(&mut self, longs: &[u64]) -> &mut Self {
        let mut b: Vec<u8> = longs.iter().flat_map(|l| l.to_le_bytes()).collect();
        while b.last() == Some(&0) {
            b.pop();
        }
        self.var_int(b.len() as i32);
        self.buf.extend_from_slice(&b);
        self
    }
    pub fn bitset(&mut self, longs: &[u64]) -> &mut Self {
        self.var_int(longs.len() as i32);
        for l in longs {
            self.i64(*l as i64);
        }
        self
    }
}

/// Размер VarInt в байтах.
pub fn var_int_size(v: i32) -> usize {
    let v = v as u32;
    match v {
        0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1F_FFFF => 3,
        0x20_0000..=0xFFF_FFFF => 4,
        _ => 5,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn varint_vectors() {
        // wiki.vg примеры
        for (v, b) in [
            (0, vec![0x00]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (25565, vec![0xdd, 0xc7, 0x01]),
            (2097151, vec![0xff, 0xff, 0x7f]),
            (i32::MAX, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (i32::MIN, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ] {
            let mut w = Writer::new();
            w.var_int(v);
            assert_eq!(w.buf, b, "{v}");
            assert_eq!(var_int_size(v), b.len());
            assert_eq!(Reader::new(&b).var_int().unwrap(), v);
        }
    }

    #[test]
    fn varlong_roundtrip() {
        for v in [0i64, 1, -1, i64::MAX, i64::MIN, 2147483648] {
            let mut w = Writer::new();
            w.var_long(v);
            assert_eq!(Reader::new(&w.buf).var_long().unwrap(), v);
        }
        assert_eq!(
            Reader::new(&[0xff; 11]).var_long(),
            Err(ProtocolError::VarTooLong { max: 10 })
        );
    }

    #[test]
    fn varint_too_long_and_eof() {
        assert_eq!(
            Reader::new(&[0x80; 6]).var_int(),
            Err(ProtocolError::VarTooLong { max: 5 })
        );
        assert!(matches!(
            Reader::new(&[0x80]).var_int(),
            Err(ProtocolError::Eof { .. })
        ));
    }

    #[test]
    fn strings_and_limits() {
        let mut w = Writer::new();
        w.string("héllo");
        assert_eq!(Reader::new(&w.buf).string(16).unwrap(), "héllo");
        assert!(matches!(
            Reader::new(&w.buf).string(3),
            Err(ProtocolError::TooLarge { .. })
        ));
        assert_eq!(
            Reader::new(&[2, 0xff, 0xfe]).string(8),
            Err(ProtocolError::BadUtf8)
        );
        assert_eq!(
            Reader::new(&[0xff, 0xff, 0xff, 0xff, 0x0f]).string(8),
            Err(ProtocolError::NegativeLength(-1))
        );
    }

    #[test]
    fn block_pos_roundtrip() {
        for p in [
            (0, 0, 0),
            (-1, -1, -1),
            (33554431, 2047, -33554432),
            (100, -64, -200),
        ] {
            let mut w = Writer::new();
            w.block_pos(p.0, p.1, p.2);
            assert_eq!(Reader::new(&w.buf).block_pos().unwrap(), p);
        }
        // wiki.vg: x=18357644 y=831 z=-20882616
        assert_eq!(
            unpack_block_pos(pack_block_pos(18357644, 831, -20882616)),
            (18357644, 831, -20882616)
        );
    }

    #[test]
    fn misc_types() {
        let mut w = Writer::new();
        w.uuid(0x0123456789abcdef0011223344556677)
            .angle(90.0)
            .bitset(&[1, u64::MAX])
            .bool(true)
            .i32(7);
        let mut r = Reader::new(&w.buf);
        assert_eq!(r.uuid().unwrap(), 0x0123456789abcdef0011223344556677);
        assert_eq!(r.angle().unwrap(), 90.0);
        assert_eq!(r.bitset().unwrap(), vec![1, u64::MAX]);
        assert_eq!(r.optional(|r| r.i32()).unwrap(), Some(7));
        assert!(r.is_empty());
        assert_eq!(Reader::new(&[0]).optional(|r| r.i32()).unwrap(), None);
    }
}
