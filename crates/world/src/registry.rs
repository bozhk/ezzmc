//! Реестры в рантайме: id ↔ имя (`minecraft:` опускается при поиске).
//! Статические — из `ezzmc_protocol::generated::r*` (порядок = protocol id),
//! динамические (биомы, dimension_type…) — из `registry_data` фазы Configuration.
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Registry {
    names: Vec<String>,
    ids: HashMap<String, u32>,
}

fn strip(name: &str) -> &str {
    name.strip_prefix("minecraft:").unwrap_or(name)
}

impl Registry {
    /// Имена без пространства имён `minecraft:`, либо с ним — приводим к короткому виду.
    pub fn from_names<I, S>(names: I) -> Registry
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let names: Vec<String> = names.into_iter().map(|n| n.as_ref().to_owned()).collect();
        let ids = names
            .iter()
            .enumerate()
            .map(|(i, n)| (strip(n).to_owned(), i as u32))
            .collect();
        Registry { names, ids }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub fn name(&self, id: u32) -> Option<&str> {
        self.names.get(id as usize).map(String::as_str)
    }

    /// `"stone"` или `"minecraft:stone"` → id.
    pub fn id(&self, name: &str) -> Option<u32> {
        self.ids.get(strip(name)).copied()
    }
}

/// Все реестры соединения.
#[derive(Clone, Debug, Default)]
pub struct Registries {
    pub block: Registry,
    pub item: Registry,
    pub entity_type: Registry,
    pub block_entity_type: Registry,
    pub fluid: Registry,
    /// Динамические реестры по полному имени (`minecraft:worldgen/biome` и т.п.).
    dynamic: HashMap<String, Registry>,
}

impl Registries {
    /// Статические реестры 1.21.11.
    pub fn v26_3() -> Registries {
        use ezzmc_protocol::generated::r26_3 as r;
        Registries {
            block: Registry::from_names(r::BLOCK),
            item: Registry::from_names(r::ITEM),
            entity_type: Registry::from_names(r::ENTITY_TYPE),
            block_entity_type: Registry::from_names(r::BLOCK_ENTITY_TYPE),
            fluid: Registry::from_names(r::FLUID),
            dynamic: HashMap::new(),
        }
    }

    /// Динамический реестр из registry_data: порядок записей = id (как RegistryDataCollector).
    pub fn set_dynamic<I, S>(&mut self, registry: &str, entries: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.dynamic
            .insert(strip(registry).to_owned(), Registry::from_names(entries));
    }

    /// `"worldgen/biome"` или `"minecraft:worldgen/biome"`.
    pub fn dynamic(&self, registry: &str) -> Option<&Registry> {
        self.dynamic.get(strip(registry))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn static_registries() {
        let r = Registries::v26_3();
        assert_eq!(r.block.id("minecraft:stone"), Some(1));
        assert_eq!(r.block.name(0), Some("air"));
        assert_eq!(r.block.len(), crate::generated::b26_3::BLOCKS.len());
        assert_eq!(r.fluid.id("water"), Some(2));
        assert!(r.item.id("diamond").is_some());
        assert!(r.entity_type.id("player").is_some());
        assert!(r.block_entity_type.id("chest").is_some());
        assert_eq!(r.block.id("nope"), None);
        assert_eq!(r.block.name(100_000), None);
    }

    #[test]
    fn dynamic_registry() {
        let mut r = Registries::v26_3();
        r.set_dynamic(
            "minecraft:worldgen/biome",
            ["minecraft:plains", "minecraft:desert"],
        );
        let b = r.dynamic("worldgen/biome").unwrap();
        assert_eq!(b.id("desert"), Some(1));
        assert_eq!(b.name(0), Some("minecraft:plains"));
        assert!(r.dynamic("nope").is_none());
    }
}
