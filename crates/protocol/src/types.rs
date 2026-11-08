//! Базовые структуры протокола: BlockPos, ChunkPos, Vec3, GameProfile (TODO-057a).
// VANILLA: BlockPos, ChunkPos, Vec3, GameProfile, UUIDUtil#createOfflinePlayerUUID (1.21.11)
use crate::wire::{pack_block_pos, unpack_block_pos, Reader, Writer};
use crate::ProtocolResult;
use md5::{Digest, Md5};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        let (x, y, z) = unpack_block_pos(r.i64()?);
        Ok(Self { x, y, z })
    }
    pub fn write(&self, w: &mut Writer) {
        w.i64(pack_block_pos(self.x, self.y, self.z));
    }
    pub fn chunk(&self) -> ChunkPos {
        ChunkPos {
            x: self.x >> 4,
            z: self.z >> 4,
        }
    }
    /// Индекс секции по Y (`y >> 4`).
    pub fn section_y(&self) -> i32 {
        self.y >> 4
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    /// VANILLA: ChunkPos#asLong — x в младших 32 битах, z в старших.
    pub fn as_long(&self) -> i64 {
        i64::from(self.x) & 0xFFFF_FFFF | (i64::from(self.z) & 0xFFFF_FFFF) << 32
    }
    pub fn from_long(v: i64) -> Self {
        Self {
            x: v as i32,
            z: (v >> 32) as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        Ok(Self {
            x: r.f64()?,
            y: r.f64()?,
            z: r.f64()?,
        })
    }
    pub fn write(&self, w: &mut Writer) {
        w.f64(self.x).f64(self.y).f64(self.z);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameProfile {
    pub id: u128,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
}

impl GameProfile {
    /// Формат 1.21.11 (login_finished / player_info): uuid, имя ≤16, свойства ≤16.
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        let id = r.uuid()?;
        let name = r.string(16)?;
        let n = r.length("profile properties", 16)?;
        let mut properties = Vec::with_capacity(n);
        for _ in 0..n {
            properties.push(ProfileProperty {
                name: r.string(64)?,
                value: r.string(32767)?,
                signature: r.optional(|r| r.string(1024))?,
            });
        }
        Ok(Self {
            id,
            name,
            properties,
        })
    }

    pub fn write(&self, w: &mut Writer) {
        w.uuid(self.id)
            .string(&self.name)
            .var_int(self.properties.len() as i32);
        for p in &self.properties {
            w.string(&p.name).string(&p.value);
            match &p.signature {
                Some(s) => w.bool(true).string(s),
                None => w.bool(false),
            };
        }
    }

    pub fn offline(name: &str) -> Self {
        Self {
            id: offline_uuid(name),
            name: name.to_owned(),
            properties: Vec::new(),
        }
    }
}

/// VANILLA: UUID.nameUUIDFromBytes("OfflinePlayer:" + name) — MD5, версия 3, вариант IETF.
pub fn offline_uuid(name: &str) -> u128 {
    let mut h = Md5::new();
    h.update(b"OfflinePlayer:");
    h.update(name.as_bytes());
    let mut b: [u8; 16] = h.finalize().into();
    b[6] = (b[6] & 0x0F) | 0x30;
    b[8] = (b[8] & 0x3F) | 0x80;
    u128::from_be_bytes(b)
}

pub fn format_uuid(id: u128) -> String {
    let h = format!("{id:032x}");
    format!(
        "{}-{}-{}-{}-{}",
        &h[..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..]
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn offline_uuid_known_values() {
        // Известные значения ванильного сервера в offline-режиме.
        assert_eq!(
            format_uuid(offline_uuid("Notch")),
            "b50ad385-829d-3141-a216-7e7d7539ba7f"
        );
    }

    #[test]
    fn block_and_chunk_pos() {
        let p = BlockPos::new(-17, 70, 33);
        let mut w = Writer::new();
        p.write(&mut w);
        assert_eq!(BlockPos::read(&mut Reader::new(&w.buf)).unwrap(), p);
        assert_eq!(p.chunk(), ChunkPos { x: -2, z: 2 });
        assert_eq!(p.section_y(), 4);
        let c = ChunkPos { x: -5, z: 7 };
        assert_eq!(ChunkPos::from_long(c.as_long()), c);
        assert_eq!(ChunkPos { x: 1, z: 2 }.as_long(), (2 << 32) | 1);
    }

    #[test]
    fn profile_roundtrip() {
        let g = GameProfile {
            id: 7,
            name: "Steve".into(),
            properties: vec![
                ProfileProperty {
                    name: "textures".into(),
                    value: "v".into(),
                    signature: Some("s".into()),
                },
                ProfileProperty {
                    name: "x".into(),
                    value: "y".into(),
                    signature: None,
                },
            ],
        };
        let mut w = Writer::new();
        g.write(&mut w);
        let mut r = Reader::new(&w.buf);
        assert_eq!(GameProfile::read(&mut r).unwrap(), g);
        assert!(r.is_empty());
        // имя > 16 символов — ошибка
        let mut w = Writer::new();
        w.uuid(1).string("abcdefghijklmnopq").var_int(0);
        assert!(GameProfile::read(&mut Reader::new(&w.buf)).is_err());
        // слишком много свойств
        let mut w = Writer::new();
        w.uuid(1).string("a").var_int(17);
        assert!(GameProfile::read(&mut Reader::new(&w.buf)).is_err());
    }
}
