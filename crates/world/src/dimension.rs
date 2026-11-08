//! Свойства измерения (TODO-130). VANILLA: DimensionType (1.21.11).
//! Разбирается из NBT записи реестра `minecraft:dimension_type` (registry_data).
//! `attributes` (цвета неба/тумана/облаков и пр.) хранятся сырыми — их читают TODO-149+.
use crate::chunk::ChunkFormat;
use ezzmc_protocol::nbt::Tag;
use ezzmc_protocol::{ProtocolError, ProtocolResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skybox {
    None,
    Overworld,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardinalLight {
    Default,
    Nether,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionType {
    pub has_fixed_time: bool,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub coordinate_scale: f64,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub ambient_light: f32,
    pub skybox: Skybox,
    pub cardinal_light: CardinalLight,
    /// EnvironmentAttributeMap как есть (compound), если есть.
    pub attributes: Option<Tag>,
}

fn bad(what: &'static str) -> ProtocolError {
    ProtocolError::InvalidValue { what, value: 0 }
}

fn get_bool(t: &Tag, k: &'static str, default: Option<bool>) -> ProtocolResult<bool> {
    match t.get(k) {
        Some(v) => v.as_i64().map(|n| n != 0).ok_or_else(|| bad(k)),
        None => default.ok_or_else(|| bad(k)),
    }
}

fn get_int(t: &Tag, k: &'static str) -> ProtocolResult<i32> {
    t.get(k)
        .and_then(Tag::as_i64)
        .and_then(|n| i32::try_from(n).ok())
        .ok_or_else(|| bad(k))
}

fn get_f64(t: &Tag, k: &'static str) -> ProtocolResult<f64> {
    match t.get(k) {
        Some(Tag::Double(v)) => Ok(*v),
        Some(Tag::Float(v)) => Ok(f64::from(*v)),
        Some(other) => other.as_i64().map(|n| n as f64).ok_or_else(|| bad(k)),
        None => Err(bad(k)),
    }
}

impl DimensionType {
    /// VANILLA: DimensionType.NETWORK_CODEC + проверки конструктора (кратность 16 и т.д.).
    pub fn from_nbt(t: &Tag) -> ProtocolResult<Self> {
        let min_y = get_int(t, "min_y")?;
        let height = get_int(t, "height")?;
        let logical_height = get_int(t, "logical_height")?;
        if height < 16 || height % 16 != 0 || min_y % 16 != 0 || logical_height > height {
            return Err(bad("dimension height"));
        }
        let skybox = match t.get("skybox").and_then(Tag::as_str) {
            None | Some("overworld") | Some("minecraft:overworld") => Skybox::Overworld,
            Some("none") | Some("minecraft:none") => Skybox::None,
            Some("end") | Some("minecraft:end") => Skybox::End,
            Some(_) => return Err(bad("skybox")),
        };
        let cardinal_light = match t.get("cardinal_light").and_then(Tag::as_str) {
            None | Some("default") => CardinalLight::Default,
            Some("nether") => CardinalLight::Nether,
            Some(_) => return Err(bad("cardinal_light")),
        };
        Ok(DimensionType {
            has_fixed_time: get_bool(t, "has_fixed_time", Some(false))?,
            has_skylight: get_bool(t, "has_skylight", None)?,
            has_ceiling: get_bool(t, "has_ceiling", None)?,
            coordinate_scale: get_f64(t, "coordinate_scale")?,
            min_y,
            height,
            logical_height,
            ambient_light: get_f64(t, "ambient_light")? as f32,
            skybox,
            cardinal_light,
            attributes: t.get("attributes").cloned(),
        })
    }

    pub fn chunk_format(&self, block_global_bits: u32, biome_global_bits: u32) -> ChunkFormat {
        ChunkFormat {
            min_y: self.min_y,
            height: self.height,
            block_global_bits,
            biome_global_bits,
        }
    }

    /// VANILLA: DimensionType#getTeleportationScale.
    pub fn teleportation_scale(from: &DimensionType, to: &DimensionType) -> f64 {
        from.coordinate_scale / to.coordinate_scale
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::nbt::parse_snbt;

    #[test]
    fn overworld_and_nether() {
        let ow = parse_snbt(
            "{ambient_light:0.0f,coordinate_scale:1.0d,has_ceiling:0b,has_skylight:1b,height:384,logical_height:384,min_y:-64,attributes:{}}",
        )
        .unwrap();
        let d = DimensionType::from_nbt(&ow).unwrap();
        assert_eq!((d.min_y, d.height), (-64, 384));
        assert!(d.has_skylight && !d.has_ceiling && !d.has_fixed_time);
        assert_eq!(d.skybox, Skybox::Overworld);
        assert_eq!(d.chunk_format(15, 6).section_count(), 24);
        let nether = parse_snbt(
            "{ambient_light:0.1f,cardinal_light:\"nether\",coordinate_scale:8.0d,has_ceiling:1b,has_fixed_time:1b,has_skylight:0b,height:256,logical_height:128,min_y:0,skybox:\"none\"}",
        )
        .unwrap();
        let n = DimensionType::from_nbt(&nether).unwrap();
        assert_eq!(n.cardinal_light, CardinalLight::Nether);
        assert_eq!(n.skybox, Skybox::None);
        assert!(n.has_ceiling && n.has_fixed_time && n.attributes.is_none());
        assert_eq!(DimensionType::teleportation_scale(&d, &n), 0.125);
    }

    #[test]
    fn invalid_rejected() {
        let t = parse_snbt(
            "{ambient_light:0.0f,coordinate_scale:1.0d,has_ceiling:0b,has_skylight:1b,height:100,logical_height:100,min_y:0}",
        )
        .unwrap();
        assert!(DimensionType::from_nbt(&t).is_err());
        assert!(DimensionType::from_nbt(&parse_snbt("{}").unwrap()).is_err());
    }
}
