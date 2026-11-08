//! Описание блоков и состояний (таблицы — `generated/b<ver>.rs`, `cargo xtask gen-blockstates`).

#[derive(Clone, Copy, Debug)]
pub struct PropInfo {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

#[derive(Clone, Copy, Debug)]
pub struct BlockInfo {
    pub name: &'static str,
    pub first_state: u32,
    pub state_count: u32,
    pub default_state: u32,
    /// По алфавиту имён (как StateDefinition); последнее свойство меняется быстрее всех.
    pub props: &'static [PropInfo],
}

/// id состояния → (id блока, индексы значений свойств). `None`, если id вне таблицы.
pub fn decode_state(blocks: &'static [BlockInfo], state: u32) -> Option<(usize, Vec<u32>)> {
    let i = blocks.partition_point(|b| b.first_state + b.state_count <= state);
    let b = blocks.get(i)?;
    let mut off = state.checked_sub(b.first_state)?;
    let mut idx = vec![0; b.props.len()];
    for (k, p) in b.props.iter().enumerate().rev() {
        let n = p.values.len() as u32;
        idx[k] = off % n;
        off /= n;
    }
    Some((i, idx))
}

/// Компактный id состояния блока (`Block.BLOCK_STATE_REGISTRY`). u16 хватает: 29671 < 65536.
/// VANILLA: Block#getId(BlockState) / Block#stateById.
/// Таблицы пока только 1.21.11; мульти-версии — глава 18.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct BlockState(pub u16);

impl BlockState {
    pub const AIR: BlockState = BlockState(0);

    /// `None`, если id вне таблицы.
    pub fn from_id(id: u32) -> Option<BlockState> {
        use crate::generated::b26_3::STATE_COUNT;
        if id < STATE_COUNT {
            u16::try_from(id).ok().map(BlockState)
        } else {
            None
        }
    }

    pub fn id(self) -> u32 {
        u32::from(self.0)
    }

    /// Индекс блока в `BLOCKS` (= id в реестре блоков).
    pub fn block_id(self) -> usize {
        state_to_block()
            .get(usize::from(self.0))
            .map_or(0, |&b| usize::from(b))
    }

    pub fn block(self) -> &'static BlockInfo {
        &crate::generated::b26_3::BLOCKS[self.block_id()]
    }

    pub fn name(self) -> &'static str {
        self.block().name
    }

    pub fn props(self) -> Option<&'static StateProps> {
        use crate::generated::p26_3 as p;
        state_props(p::STATE_PROPS, p::PROPS, self.id())
    }

    pub fn is_air(self) -> bool {
        self.props().is_some_and(|p| p.has(FLAG_AIR))
    }

    /// Значение свойства по имени (`"facing"` → `"north"`).
    pub fn property(self, name: &str) -> Option<&'static str> {
        let (b, idx) = decode_state(crate::generated::b26_3::BLOCKS, self.id())?;
        let info = &crate::generated::b26_3::BLOCKS[b];
        let k = info.props.iter().position(|p| p.name == name)?;
        info.props[k].values.get(idx[k] as usize).copied()
    }

    /// Состояние с изменённым свойством (`BlockState#setValue`); `None`, если свойства/значения нет.
    pub fn with(self, name: &str, value: &str) -> Option<BlockState> {
        let info = self.block();
        let (_, mut idx) = decode_state(crate::generated::b26_3::BLOCKS, self.id())?;
        let k = info.props.iter().position(|p| p.name == name)?;
        idx[k] = info.props[k].values.iter().position(|v| *v == value)? as u32;
        let mut off = 0u32;
        for (p, i) in info.props.iter().zip(&idx) {
            off = off * p.values.len() as u32 + i;
        }
        BlockState::from_id(info.first_state + off)
    }
}

/// id состояния → id блока, строится один раз.
fn state_to_block() -> &'static [u16] {
    use crate::generated::b26_3::{BLOCKS, STATE_COUNT};
    static T: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let mut v = vec![0u16; STATE_COUNT as usize];
        for (i, b) in BLOCKS.iter().enumerate() {
            for s in b.first_state..b.first_state + b.state_count {
                v[s as usize] = i as u16;
            }
        }
        v
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod state_tests {
    use super::*;

    #[test]
    fn block_state_roundtrip() {
        assert!(BlockState::AIR.is_air());
        assert_eq!(BlockState::from_id(1).unwrap().name(), "stone");
        assert!(BlockState::from_id(35723).is_none());
        let s = BlockState::from_id(5463).unwrap();
        assert_eq!(s.name(), "oak_stairs");
        assert_eq!(s.property("facing"), Some("north"));
        assert_eq!(s.property("waterlogged"), Some("true"));
        let t = s.with("waterlogged", "false").unwrap();
        assert_eq!(t.id(), 5464);
        assert_eq!(t.with("waterlogged", "true"), Some(s));
        assert!(s.with("nope", "x").is_none());
        assert!(s.with("facing", "up").is_none());
        // каждое состояние отображается в блок, чей диапазон его содержит
        for id in (0..29671).step_by(7) {
            let st = BlockState::from_id(id).unwrap();
            let b = st.block();
            assert!(id >= b.first_state && id < b.first_state + b.state_count);
        }
    }
}

/// Свойства состояния блока, снятые с ванили (`cargo xtask gen-blockprops`).
/// VANILLA: BlockBehaviour.BlockStateBase (+ Block#getExplosionResistance/getFriction/getSpeedFactor/getJumpFactor).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StateProps {
    /// `getDestroySpeed`; −1 = неразрушимый.
    pub destroy_time: f32,
    pub light_emission: u8,
    pub light_block: u8,
    /// См. `FLAG_*`.
    pub flags: u8,
    /// Индекс в `SHAPES` (коллизия / контур).
    pub collision_shape: u16,
    pub outline_shape: u16,
    pub explosion_resistance: f32,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
}

pub const FLAG_CAN_OCCLUDE: u8 = 1;
pub const FLAG_PROPAGATES_SKYLIGHT: u8 = 2;
pub const FLAG_REQUIRES_CORRECT_TOOL: u8 = 4;
pub const FLAG_AIR: u8 = 8;
pub const FLAG_RANDOM_TICK: u8 = 16;
pub const FLAG_BLOCKS_MOTION: u8 = 32;
pub const FLAG_SOLID: u8 = 64;

impl StateProps {
    pub fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

/// Свойства по id состояния.
pub fn state_props(
    state_props: &'static [u16],
    props: &'static [StateProps],
    state: u32,
) -> Option<&'static StateProps> {
    props.get(usize::from(*state_props.get(usize::try_from(state).ok()?)?))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod props_tests {
    use super::*;
    use crate::generated::{b26_3 as b, p26_3 as p};

    #[test]
    fn vanilla_block_props() {
        assert_eq!(p::STATE_PROPS.len(), b::STATE_COUNT as usize);
        let get = |name: &str| {
            let info = b::BLOCKS.iter().find(|x| x.name == name).unwrap();
            *state_props(p::STATE_PROPS, p::PROPS, info.default_state).unwrap()
        };
        let stone = get("stone");
        assert_eq!(stone.destroy_time, 1.5);
        assert_eq!(stone.explosion_resistance, 6.0);
        assert!(stone.has(FLAG_CAN_OCCLUDE | FLAG_REQUIRES_CORRECT_TOOL));
        assert_eq!(
            p::SHAPES[usize::from(stone.collision_shape)],
            &[[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]
        );
        assert_eq!(get("bedrock").destroy_time, -1.0);
        assert_eq!(get("glowstone").light_emission, 15);
        assert_eq!(get("ice").friction, 0.98);
        assert_eq!(get("soul_sand").speed_factor, 0.4);
        assert_eq!(get("honey_block").jump_factor, 0.5);
        assert!(get("air").has(FLAG_AIR));
        assert!(p::SHAPES[usize::from(get("air").collision_shape)].is_empty());
    }
}

/// Состоит ли запись реестра `id` в теге `tag` (таблицы `generated/t*.rs`, `*_TAGS`).
/// Имя тега без `minecraft:`; списки отсортированы → бинарный поиск.
pub fn in_tag(tags: &[(&str, &[u32])], tag: &str, id: u32) -> bool {
    tags.binary_search_by(|(n, _)| (*n).cmp(tag))
        .is_ok_and(|i| tags[i].1.binary_search(&id).is_ok())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tag_tests {
    use super::*;
    use crate::generated::t26_3::{BLOCK_TAGS, ITEM_TAGS};

    #[test]
    fn vanilla_tags() {
        // id блоков = protocol_id реестра (registries.json): stone=1
        assert!(in_tag(BLOCK_TAGS, "mineable/pickaxe", 1));
        assert!(!in_tag(BLOCK_TAGS, "mineable/shovel", 1));
        assert!(in_tag(BLOCK_TAGS, "mineable/shovel", 9)); // dirt
        assert!(in_tag(BLOCK_TAGS, "needs_iron_tool", 247)); // diamond_ore
        assert!(!in_tag(BLOCK_TAGS, "needs_stone_tool", 1));
        assert!(!in_tag(BLOCK_TAGS, "no_such_tag", 1));
        assert!(!ITEM_TAGS.is_empty());
    }
}
