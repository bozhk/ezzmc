//! Фрейминг пакетов и компрессия (TODO-049, TODO-050).
//! Кадр: VarInt длина · [если сжатие: VarInt размер_до_сжатия (0 = не сжато)] · данные.
//! Данные = VarInt id пакета + тело. Шифрования в offline нет.
// VANILLA: Varint21FrameDecoder, CompressionDecoder, CompressionEncoder (1.21.11)
use crate::{NetError, NetResult};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::ProtocolError;
use libdeflater::{CompressionLvl, Compressor, Decompressor};

/// VANILLA: Varint21FrameDecoder — длина кадра не длиннее 3 байт VarInt.
pub const MAX_FRAME_LEN: usize = (1 << 21) - 1;
/// VANILLA: CompressionDecoder.MAXIMUM_UNCOMPRESSED_LENGTH = 8 MiB.
pub const MAX_UNCOMPRESSED: usize = 8 * 1024 * 1024;

#[derive(Debug)]
pub struct Packet {
    pub id: i32,
    pub body: Vec<u8>,
}

pub struct Codec {
    threshold: Option<usize>,
    inbuf: Vec<u8>,
    comp: Compressor,
    decomp: Decompressor,
}

impl Default for Codec {
    fn default() -> Self {
        Self::new()
    }
}

impl Codec {
    pub fn new() -> Self {
        Self {
            threshold: None,
            inbuf: Vec::new(),
            // VANILLA: Deflater() — уровень по умолчанию (6)
            comp: Compressor::new(CompressionLvl::default()),
            decomp: Decompressor::new(),
        }
    }

    /// Включить сжатие (Login: set_compression). `threshold < 0` — выключить.
    pub fn set_compression(&mut self, threshold: i32) {
        self.threshold = usize::try_from(threshold).ok();
    }
    pub fn compression(&self) -> Option<usize> {
        self.threshold
    }

    /// Добавить сырые байты из сокета.
    pub fn feed(&mut self, data: &[u8]) {
        self.inbuf.extend_from_slice(data);
    }

    /// Достать следующий пакет, если он целиком в буфере.
    pub fn next_packet(&mut self) -> NetResult<Option<Packet>> {
        // длина кадра — до 3 байт; без полного префикса ждём ещё
        let mut r = Reader::new(&self.inbuf);
        let mut len = 0usize;
        let mut prefix = 0usize;
        loop {
            let Ok(b) = r.u8() else { return Ok(None) };
            len |= usize::from(b & 0x7F) << (7 * prefix);
            prefix += 1;
            if b & 0x80 == 0 {
                break;
            }
            if prefix == 3 {
                return Err(ProtocolError::VarTooLong { max: 3 }.into());
            }
        }
        if len > MAX_FRAME_LEN {
            return Err(ProtocolError::TooLarge {
                what: "frame",
                len,
                limit: MAX_FRAME_LEN,
            }
            .into());
        }
        if self.inbuf.len() < prefix + len {
            return Ok(None);
        }
        let payload = &self.inbuf[prefix..prefix + len];
        let pkt = Self::decode_payload(&mut self.decomp, self.threshold, payload)?;
        self.inbuf.drain(..prefix + len);
        Ok(Some(pkt))
    }

    fn decode_payload(
        decomp: &mut Decompressor,
        threshold: Option<usize>,
        payload: &[u8],
    ) -> NetResult<Packet> {
        let mut r = Reader::new(payload);
        let data: std::borrow::Cow<[u8]> = if let Some(th) = threshold {
            let ulen = r.var_int()?;
            if ulen == 0 {
                std::borrow::Cow::Borrowed(r.rest())
            } else {
                let ulen = usize::try_from(ulen)
                    .map_err(|_| ProtocolError::NegativeLength(i64::from(ulen)))?;
                // VANILLA: размер до сжатия не меньше порога и не больше 8 MiB
                if ulen < th {
                    return Err(ProtocolError::InvalidValue {
                        what: "compressed size below threshold",
                        value: ulen as i64,
                    }
                    .into());
                }
                if ulen > MAX_UNCOMPRESSED {
                    return Err(ProtocolError::TooLarge {
                        what: "uncompressed packet",
                        len: ulen,
                        limit: MAX_UNCOMPRESSED,
                    }
                    .into());
                }
                let mut out = vec![0u8; ulen];
                let n = decomp
                    .zlib_decompress(r.rest(), &mut out)
                    .map_err(|e| ProtocolError::Other(format!("zlib: {e}")))?;
                if n != ulen {
                    return Err(ProtocolError::Other(format!(
                        "zlib: ожидалось {ulen} байт, получено {n}"
                    ))
                    .into());
                }
                std::borrow::Cow::Owned(out)
            }
        } else {
            std::borrow::Cow::Borrowed(r.rest())
        };
        let mut r = Reader::new(&data);
        let id = r.var_int()?;
        Ok(Packet {
            id,
            body: r.rest().to_vec(),
        })
    }

    /// Закодировать пакет в кадр, дописав в `out`.
    pub fn encode(&mut self, id: i32, body: &[u8], out: &mut Vec<u8>) -> NetResult<()> {
        let mut inner = Writer::with_capacity(body.len() + 5);
        inner.var_int(id).bytes(body);
        let inner = inner.into_vec();
        let mut frame = Writer::with_capacity(inner.len() + 8);
        match self.threshold {
            None => {
                frame.bytes(&inner);
            }
            Some(th) if inner.len() < th => {
                frame.var_int(0).bytes(&inner);
            }
            Some(_) => {
                if inner.len() > MAX_UNCOMPRESSED {
                    return Err(ProtocolError::TooLarge {
                        what: "outgoing packet",
                        len: inner.len(),
                        limit: MAX_UNCOMPRESSED,
                    }
                    .into());
                }
                let mut buf = vec![0u8; self.comp.zlib_compress_bound(inner.len())];
                let n = self
                    .comp
                    .zlib_compress(&inner, &mut buf)
                    .map_err(|e| ProtocolError::Other(format!("zlib: {e}")))?;
                frame.var_int(inner.len() as i32).bytes(&buf[..n]);
            }
        }
        if frame.buf.len() > MAX_FRAME_LEN {
            return Err(NetError::Protocol(ProtocolError::TooLarge {
                what: "outgoing frame",
                len: frame.buf.len(),
                limit: MAX_FRAME_LEN,
            }));
        }
        let mut head = Writer::new();
        head.var_int(frame.buf.len() as i32);
        out.extend_from_slice(&head.buf);
        out.extend_from_slice(&frame.buf);
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn roundtrip(th: i32, body: &[u8]) -> Vec<u8> {
        let mut enc = Codec::new();
        enc.set_compression(th);
        let mut wire = Vec::new();
        enc.encode(0x2a, body, &mut wire).unwrap();
        let mut dec = Codec::new();
        dec.set_compression(th);
        dec.feed(&wire);
        let p = dec.next_packet().unwrap().unwrap();
        assert_eq!(p.id, 0x2a);
        assert_eq!(p.body, body);
        assert!(dec.next_packet().unwrap().is_none());
        wire
    }

    #[test]
    fn plain() {
        let w = roundtrip(-1, b"hello");
        assert_eq!(w, [6, 0x2a, b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn compressed_below_and_above_threshold() {
        let small = roundtrip(256, b"hi");
        assert_eq!(small[1], 0); // не сжато
        let big = vec![7u8; 5000];
        let w = roundtrip(256, &big);
        assert!(w.len() < 200, "должно сжаться, {}", w.len());
    }

    #[test]
    fn partial_feed_and_two_packets() {
        let mut enc = Codec::new();
        let mut wire = Vec::new();
        enc.encode(1, b"aaa", &mut wire).unwrap();
        enc.encode(2, b"bb", &mut wire).unwrap();
        let mut dec = Codec::new();
        for b in &wire[..4] {
            dec.feed(&[*b]);
            assert!(dec.next_packet().unwrap().is_none());
        }
        dec.feed(&wire[4..]);
        assert_eq!(dec.next_packet().unwrap().unwrap().id, 1);
        assert_eq!(dec.next_packet().unwrap().unwrap().id, 2);
    }

    #[test]
    fn limits() {
        let mut dec = Codec::new();
        dec.feed(&[0xff, 0xff, 0xff, 0x01]);
        assert!(dec.next_packet().is_err()); // префикс > 3 байт
        let mut dec = Codec::new();
        dec.feed(&[0xff, 0xff, 0x7f]); // 2^21-1 допустимо, но данных нет
        assert!(dec.next_packet().unwrap().is_none());
        // размер до сжатия больше 8 MiB
        let mut dec = Codec::new();
        dec.set_compression(256);
        let mut w = Writer::new();
        w.var_int(MAX_UNCOMPRESSED as i32 + 1).bytes(&[0; 4]);
        let mut f = Writer::new();
        f.var_int(w.buf.len() as i32).bytes(&w.buf);
        dec.feed(&f.buf);
        assert!(dec.next_packet().is_err());
        // сжатый размер ниже порога
        let mut dec = Codec::new();
        dec.set_compression(256);
        let mut w = Writer::new();
        w.var_int(10).bytes(&[0; 4]);
        let mut f = Writer::new();
        f.var_int(w.buf.len() as i32).bytes(&w.buf);
        dec.feed(&f.buf);
        assert!(dec.next_packet().is_err());
    }

    #[test]
    fn garbage_zlib_is_error_not_panic() {
        let mut dec = Codec::new();
        dec.set_compression(1);
        let mut w = Writer::new();
        w.var_int(100).bytes(&[1, 2, 3, 4, 5]);
        let mut f = Writer::new();
        f.var_int(w.buf.len() as i32).bytes(&w.buf);
        dec.feed(&f.buf);
        assert!(dec.next_packet().is_err());
    }
}
