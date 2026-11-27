//! Перевод значений по грамматике [`Shape`] из версии сервера в канон (26.3) — глава 18, TODO-372a.
//! Обход двух форм параллельно: исходной (версия сервера) и канонической. Байты копируются,
//! id статических реестров (предметы, компоненты, звуки, частицы, сущности, атрибуты, состояния
//! блоков) переводятся по именам. Несовместимое значение (`Ok(false)`) вызывающий отбрасывает:
//! компонент выпадает из патча, стек становится пустым.
use crate::error::{ProtocolError, ProtocolResult};
use crate::generated::{
    r1_20_4, r1_20_6, r1_21_1, r1_21_10, r1_21_11, r1_21_3, r1_21_4, r1_21_5, r1_21_6, r1_21_8,
    r26_3 as canon,
};
use crate::item::{
    components, components_1_20_6, components_1_21_1, components_1_21_10, components_1_21_11,
    components_1_21_3, components_1_21_4, components_1_21_5, components_1_21_8, ClientRegistry,
    Shape, MAX_DEPTH,
};
use crate::version::build_remap;
use crate::wire::{Reader, Writer};
use std::sync::OnceLock;

const MAX_STRING_LENGTH: usize = 32767;

fn bad(what: &'static str) -> ProtocolError {
    ProtocolError::InvalidValue { what, value: -1 }
}

const REGISTRIES: [ClientRegistry; 10] = [
    ClientRegistry::SoundEvent,
    ClientRegistry::Item,
    ClientRegistry::DataComponentType,
    ClientRegistry::PositionSourceType,
    ClientRegistry::ParticleType,
    ClientRegistry::BlockEntityType,
    ClientRegistry::ConsumeEffectType,
    ClientRegistry::EntityType,
    ClientRegistry::Attribute,
    ClientRegistry::Block,
];

/// Таблицы реестров версии сервера по [`ClientRegistry`].
macro_rules! src_names {
    ($fn:ident, $m:ident) => {
        fn $fn(reg: ClientRegistry) -> &'static [&'static str] {
            match reg {
                ClientRegistry::SoundEvent => $m::SOUND_EVENT,
                ClientRegistry::Item => $m::ITEM,
                ClientRegistry::DataComponentType => $m::DATA_COMPONENT_TYPE,
                ClientRegistry::PositionSourceType => $m::POSITION_SOURCE_TYPE,
                ClientRegistry::ParticleType => $m::PARTICLE_TYPE,
                ClientRegistry::BlockEntityType => $m::BLOCK_ENTITY_TYPE,
                ClientRegistry::ConsumeEffectType => $m::CONSUME_EFFECT_TYPE,
                ClientRegistry::EntityType => $m::ENTITY_TYPE,
                ClientRegistry::Attribute => $m::ATTRIBUTE,
                ClientRegistry::Block => $m::BLOCK,
            }
        }
    };
}
src_names!(names_1_21_11, r1_21_11);
src_names!(names_1_21_10, r1_21_10);
src_names!(names_1_21_8, r1_21_8);
src_names!(names_1_21_6, r1_21_6);
src_names!(names_1_21_5, r1_21_5);
src_names!(names_1_21_4, r1_21_4);
src_names!(names_1_21_3, r1_21_3);

/// Реестры 1.20.5–1.21.1 под именами 1.21.2+: звуки зловещего спавнера (1.20.6), атрибуты без префикса `generic.`/`player.`/
/// `zombie.`, `fire_resistant` → `damage_resistant`, лодки → дубовые; consume_effect_type нет.
// VANILLA: Attributes, DataComponents, EntityType (1.21.1 → 1.21.2).
macro_rules! names_pre_1_21_2 {
    ($fn:ident, $m:ident) => {
        names_pre_1_21_2!($fn, $m, $m);
    };
    // $c — модуль с DATA_COMPONENT_TYPE (у 1.20.4 компонентов нет: берётся 1.20.6, не используется).
    ($fn:ident, $m:ident, $c:ident) => {
        fn $fn(reg: ClientRegistry) -> &'static [&'static str] {
            fn renamed(
                cell: &'static OnceLock<Vec<&'static str>>,
                src: &'static [&'static str],
                f: fn(&'static str) -> &'static str,
            ) -> &'static [&'static str] {
                cell.get_or_init(|| src.iter().map(|n| f(n)).collect())
            }
            static ATTR: OnceLock<Vec<&'static str>> = OnceLock::new();
            static COMP: OnceLock<Vec<&'static str>> = OnceLock::new();
            static ENT: OnceLock<Vec<&'static str>> = OnceLock::new();
            static SND: OnceLock<Vec<&'static str>> = OnceLock::new();
            match reg {
                // VANILLA: SoundEvents (1.20.6 → 1.21) — звуки зловещего спавнера испытаний.
                ClientRegistry::SoundEvent => renamed(&SND, $m::SOUND_EVENT, |n| match n {
                    "block.trial_spawner.ambient_charged" => "block.trial_spawner.ambient_ominous",
                    "block.trial_spawner.charge_activate" => "block.trial_spawner.ominous_activate",
                    n => n,
                }),
                ClientRegistry::Item => $m::ITEM,
                ClientRegistry::DataComponentType => {
                    renamed(&COMP, $c::DATA_COMPONENT_TYPE, |n| match n {
                        "fire_resistant" => "damage_resistant",
                        n => n,
                    })
                }
                ClientRegistry::PositionSourceType => $m::POSITION_SOURCE_TYPE,
                ClientRegistry::ParticleType => $m::PARTICLE_TYPE,
                ClientRegistry::BlockEntityType => $m::BLOCK_ENTITY_TYPE,
                ClientRegistry::ConsumeEffectType => &[],
                ClientRegistry::EntityType => renamed(&ENT, $m::ENTITY_TYPE, |n| match n {
                    "boat" => "oak_boat",
                    "chest_boat" => "oak_chest_boat",
                    n => n,
                }),
                ClientRegistry::Attribute => renamed(&ATTR, $m::ATTRIBUTE, |n| {
                    n.split_once('.').map_or(n, |(_, x)| x)
                }),
                ClientRegistry::Block => $m::BLOCK,
            }
        }
    };
}
names_pre_1_21_2!(names_1_21_1, r1_21_1);
names_pre_1_21_2!(names_1_20_6, r1_20_6);
names_pre_1_21_2!(names_1_20_4, r1_20_4, r1_20_6);

type Names = fn(ClientRegistry) -> &'static [&'static str];
type Comps = fn() -> &'static [&'static Shape];

/// Исходная (не каноническая) версия перевода: реестры и формы компонентов.
#[derive(Clone, Copy)]
struct Source {
    names: Names,
    components: Comps,
}

const SRC_1_21_11: Source = Source {
    names: names_1_21_11,
    components: components_1_21_11,
};
const SRC_1_21_10: Source = Source {
    names: names_1_21_10,
    components: components_1_21_10,
};
const SRC_1_21_8: Source = Source {
    names: names_1_21_8,
    components: components_1_21_8,
};

/// 1.21.6: компоненты как у 1.21.8 (DATA_COMPONENT_TYPE совпадает), свои item/sound_event.
const SRC_1_21_6: Source = Source {
    names: names_1_21_6,
    components: components_1_21_8,
};

/// 1.21.5: Equippable без canBeSheared/shearingSound, атрибуты без Display.
const SRC_1_21_5: Source = Source {
    names: names_1_21_5,
    components: components_1_21_5,
};

/// 1.21.4: флаги showInTooltip, предикаты блоков без компонентов, Tool/Equippable короче.
const SRC_1_21_4: Source = Source {
    names: names_1_21_4,
    components: components_1_21_4,
};

/// 1.21.2/1.21.3: custom_model_data — одно число, материал отделки с item_model_index.
/// 1.21/1.21.1: еда с эффектами, fire_resistant, зелья без custom_name.
const SRC_1_21_1: Source = Source {
    names: names_1_21_1,
    components: components_1_21_1,
};

/// 1.20.5/1.20.6: как 1.21.1, но еда без using_converts_to, модификаторы атрибутов с UUID.
const SRC_1_20_6: Source = Source {
    names: names_1_20_6,
    components: components_1_20_6,
};

/// 1.20.3/1.20.4: реестры 1.20.4; предметы идут NBT мимо `Translator` (`net::legacy::stack`).
const SRC_1_20_4: Source = Source {
    names: names_1_20_4,
    components: components_1_20_6,
};

const SRC_1_21_3: Source = Source {
    names: names_1_21_3,
    components: components_1_21_3,
};

/// Источник перевода для версии соединения: 766 — 1.20.5/1.20.6, 767 — 1.21/1.21.1, 768 — 1.21.2/1.21.3, 769 — 1.21.4, 770 — 1.21.5, 771 — 1.21.6, 772 — 1.21.7/1.21.8,
/// 773 — 1.21.9/1.21.10, остальное — 1.21.11.
fn source_index(protocol: i32) -> usize {
    match protocol {
        765 => 9,
        766 => 8,
        767 => 7,
        768 => 6,
        769 => 5,
        770 => 4,
        771 => 3,
        772 => 0,
        773 => 1,
        _ => 2,
    }
}
const SOURCES: [Source; 10] = [
    SRC_1_21_8,
    SRC_1_21_10,
    SRC_1_21_11,
    SRC_1_21_6,
    SRC_1_21_5,
    SRC_1_21_4,
    SRC_1_21_3,
    SRC_1_21_1,
    SRC_1_20_6,
    SRC_1_20_4,
];

fn canon_names(reg: ClientRegistry) -> &'static [&'static str] {
    match reg {
        ClientRegistry::SoundEvent => canon::SOUND_EVENT,
        ClientRegistry::Item => canon::ITEM,
        ClientRegistry::DataComponentType => canon::DATA_COMPONENT_TYPE,
        ClientRegistry::PositionSourceType => canon::POSITION_SOURCE_TYPE,
        ClientRegistry::ParticleType => canon::PARTICLE_TYPE,
        ClientRegistry::BlockEntityType => canon::BLOCK_ENTITY_TYPE,
        ClientRegistry::ConsumeEffectType => canon::CONSUME_EFFECT_TYPE,
        ClientRegistry::EntityType => canon::ENTITY_TYPE,
        ClientRegistry::Attribute => canon::ATTRIBUTE,
        ClientRegistry::Block => canon::BLOCK,
    }
}

/// Компоненты, переименованные в каноне: 1.21.11 → 26.3 (в 1.21.10 обоих имён нет).
fn canon_component_name(name: &str) -> &str {
    match name {
        // DataComponents (26.3): swing_animation разделён на attack/interact_animation.
        "swing_animation" => "attack_animation",
        n => n,
    }
}

/// Обратное к [`canon_component_name`]: 26.3 → 1.21.11 (interact_animation в 1.21.11 нет).
fn src_component_name(name: &str) -> &str {
    match name {
        "attack_animation" => "swing_animation",
        n => n,
    }
}

/// Перевод между 1.21.11 и каноном: таблицы id и перевод состояний блоков.
/// Прямой (`rev = false`): 1.21.11 → канон; обратный: канон → 1.21.11 (пакеты клиента).
/// Обратный переводит только формы, совпадающие по структуре; асимметричные случаи
/// (EitherHolder, TagKey, Append, шаблоны стеков) дают `false` — компонент выпадает.
pub struct Translator {
    remaps: Vec<Vec<Option<u32>>>,
    components: Vec<Option<u32>>,
    block_state: fn(u32) -> u32,
    rev: bool,
    src: Source,
}

impl Translator {
    fn new(src: Source, block_state: fn(u32) -> u32) -> Self {
        Self::build(src, block_state, false)
    }

    fn build(src: Source, block_state: fn(u32) -> u32, rev: bool) -> Self {
        let (from, to): (Names, Names) = if rev {
            (canon_names, src.names)
        } else {
            (src.names, canon_names)
        };
        let remaps = REGISTRIES
            .iter()
            .map(|r| build_remap(from(*r), to(*r)))
            .collect();
        let name_map: fn(&str) -> &str = if rev {
            src_component_name
        } else {
            canon_component_name
        };
        let to_comps = to(ClientRegistry::DataComponentType);
        let components = from(ClientRegistry::DataComponentType)
            .iter()
            .map(|n| {
                let n = name_map(n);
                to_comps.iter().position(|c| *c == n).map(|i| i as u32)
            })
            .collect();
        Self {
            remaps,
            components,
            block_state,
            rev,
            src,
        }
    }

    fn input_names(&self, reg: ClientRegistry) -> &'static [&'static str] {
        if self.rev {
            canon_names(reg)
        } else {
            (self.src.names)(reg)
        }
    }

    fn input_components(&self) -> &'static [&'static Shape] {
        if self.rev {
            components()
        } else {
            (self.src.components)()
        }
    }

    fn output_components(&self) -> &'static [&'static Shape] {
        if self.rev {
            (self.src.components)()
        } else {
            components()
        }
    }

    /// id реестра: исходная версия → целевая.
    pub fn id(&self, reg: ClientRegistry, id: i32) -> Option<i32> {
        let k = REGISTRIES.iter().position(|r| *r as u8 == reg as u8)?;
        let id = usize::try_from(id).ok()?;
        self.remaps[k].get(id).copied().flatten().map(|v| v as i32)
    }

    /// `ItemStack.OPTIONAL_STREAM_CODEC` 1.21.11 → канон; непереводимый стек становится пустым.
    pub fn stack(&self, r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
        let mut tmp = Writer::new();
        if self.tr(r, &mut tmp, &Shape::Stack, Some(&Shape::Stack), 0)? {
            w.bytes(&tmp.into_vec());
        } else {
            w.var_int(0);
        }
        Ok(())
    }

    /// `ItemStack.OPTIONAL_UNTRUSTED_STREAM_CODEC` (значения компонентов с префиксом длины).
    /// Непереводимый предмет → пустой стек; непереводимые компоненты выпадают.
    pub fn stack_untrusted(&self, r: &mut Reader, w: &mut Writer) -> ProtocolResult<()> {
        let count = r.var_int()?;
        if count <= 0 {
            w.var_int(0);
            return Ok(());
        }
        let Some(item) = self.id(ClientRegistry::Item, r.var_int()?) else {
            w.var_int(0);
            return Ok(());
        };
        let added = r.length("added components", 4096)?;
        let removed = r.length("removed components", 4096)?;
        let (from, to) = (self.input_components(), self.output_components());
        let mut out: Vec<(i32, Vec<u8>)> = Vec::with_capacity(added);
        for _ in 0..added {
            let id = r.var_int()?;
            let n = r.length("component value", r.remaining())?;
            let body = r.bytes(n)?;
            let shape = usize::try_from(id)
                .ok()
                .and_then(|i| from.get(i))
                .copied()
                .ok_or_else(|| bad("unknown data component type"))?;
            let Some(cid) = self.components.get(id as usize).copied().flatten() else {
                continue;
            };
            let mut br = Reader::new(body);
            let mut tmp = Writer::new();
            let ok = match self.tr(&mut br, &mut tmp, shape, to.get(cid as usize).copied(), 0) {
                Ok(ok) => ok && br.remaining() == 0,
                Err(_) => false,
            };
            if ok {
                out.push((cid as i32, tmp.into_vec()));
            }
        }
        let mut gone = Vec::with_capacity(removed);
        for _ in 0..removed {
            let id = r.var_int()?;
            if let Some(c) = usize::try_from(id)
                .ok()
                .and_then(|i| self.components.get(i))
                .copied()
                .flatten()
            {
                gone.push(c as i32);
            }
        }
        w.var_int(count)
            .var_int(item)
            .var_int(out.len() as i32)
            .var_int(gone.len() as i32);
        for (id, v) in &out {
            w.var_int(*id).var_int(v.len() as i32).bytes(v);
        }
        for id in gone {
            w.var_int(id);
        }
        Ok(())
    }

    /// id компонента: исходная версия → целевая.
    pub fn component(&self, id: i32) -> Option<i32> {
        let i = usize::try_from(id).ok()?;
        self.components.get(i).copied().flatten().map(|c| c as i32)
    }

    /// Значение формы `src_shape` (1.21.11) → форма `dst` канона. `false` — перевести нельзя
    /// (вход при этом прочитан целиком, выход испорчен).
    pub fn value(
        &self,
        r: &mut Reader,
        w: &mut Writer,
        src_shape: &'static Shape,
        dst: &'static Shape,
    ) -> ProtocolResult<bool> {
        self.tr(r, w, src_shape, Some(dst), 0)
    }

    fn remap_id(
        &self,
        r: &mut Reader,
        w: &mut Writer,
        reg: ClientRegistry,
    ) -> ProtocolResult<bool> {
        let id = r.var_int()?;
        match self.id(reg, id) {
            Some(c) => {
                w.var_int(c);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// `DataComponentPatch.STREAM_CODEC` 1.21.11 → канон; несовместимые компоненты выпадают.
    fn patch(&self, r: &mut Reader, w: &mut Writer, depth: u32) -> ProtocolResult<bool> {
        let added = r.length("added components", 4096)?;
        let removed = r.length("removed components", 4096)?;
        let shapes = self.input_components();
        let dst = self.output_components();
        let mut out: Vec<(i32, Vec<u8>)> = Vec::with_capacity(added);
        for _ in 0..added {
            let id = r.var_int()?;
            let shape = usize::try_from(id)
                .ok()
                .and_then(|i| shapes.get(i))
                .copied()
                .ok_or_else(|| bad("unknown data component type"))?;
            let canon_id = self.components.get(id as usize).copied().flatten();
            let mut tmp = Writer::new();
            let name = self
                .input_names(ClientRegistry::DataComponentType)
                .get(id as usize)
                .copied();
            let ok = match canon_id {
                Some(_) if !self.rev && name == Some("pot_decorations") => {
                    self.pot_decorations(r, &mut tmp)?
                }
                Some(c) => {
                    let d = dst.get(c as usize).copied();
                    self.tr(r, &mut tmp, shape, d, depth + 1)?
                }
                None => {
                    self.tr(r, &mut tmp, shape, None, depth + 1)?;
                    false
                }
            };
            if let (true, Some(c)) = (ok, canon_id) {
                out.push((c as i32, tmp.into_vec()));
            }
        }
        let mut gone = Vec::with_capacity(removed);
        for _ in 0..removed {
            let id = r.var_int()?;
            if let Some(c) = usize::try_from(id)
                .ok()
                .and_then(|i| self.components.get(i))
                .copied()
                .flatten()
            {
                gone.push(c as i32);
            }
        }
        w.var_int(out.len() as i32).var_int(gone.len() as i32);
        for (id, v) in &out {
            w.var_int(*id).bytes(v);
        }
        for id in gone {
            w.var_int(id);
        }
        Ok(true)
    }

    /// VANILLA: PotDecorations (1.21.11) — список до 4 предметов (back, left, right, front),
    /// кирпич = пустая сторона → 26.3 PotDecorations.STREAM_CODEC: четыре Optional<ItemStackTemplate>.
    fn pot_decorations(&self, r: &mut Reader, w: &mut Writer) -> ProtocolResult<bool> {
        let n = r.length("pot decorations", 4)?;
        let brick = canon::ITEM
            .iter()
            .position(|n| *n == "brick")
            .map(|i| i as i32);
        let mut sides = [None; 4];
        for side in sides.iter_mut().take(n) {
            let id = r.var_int()?;
            *side = self
                .id(ClientRegistry::Item, id)
                .filter(|c| Some(*c) != brick);
        }
        for side in sides {
            match side {
                Some(item) => w.bool(true).var_int(item).var_int(1).var_int(0).var_int(0),
                None => w.bool(false),
            };
        }
        Ok(true)
    }

    /// Параллельный обход: `dst = None` — только прочитать исходное значение.
    fn tr(
        &self,
        r: &mut Reader,
        w: &mut Writer,
        s: &'static Shape,
        dst: Option<&'static Shape>,
        depth: u32,
    ) -> ProtocolResult<bool> {
        use Shape::*;
        if depth > MAX_DEPTH {
            return Err(bad("component nested too deeply"));
        }
        let depth = depth + 1;
        let mut d = dst;
        while let Some(Ref(inner)) = d {
            d = Some(inner);
        }
        let d = d;
        // Канон → версия, где левая ветвь Either была голым значением (Shape::Left):
        // левая ветвь пишется без флага, правую перевести нельзя.
        if let (Either(left, right), Some(Left(x))) = (s, d) {
            return if r.bool()? {
                self.tr(r, w, left, Some(x), depth)
            } else {
                self.tr(r, w, right, None, depth)?;
                Ok(false)
            };
        }
        // Канон → версия со снятым позже хвостовым полем (Shape::Trailing):
        // значение переводится, поле пишется по умолчанию.
        if let Some(Trailing(inner, _, extra)) = d {
            let ok = self.tr(r, w, s, Some(inner), depth)?;
            w.bytes(extra);
            return Ok(ok);
        }
        // Канон → 1.20.6: id модификатора атрибута → UUID и имя.
        if let (Utf(_), Some(ModifierUuid)) = (s, d) {
            let id = r.string(MAX_STRING_LENGTH)?;
            let (uuid, name) = crate::item::modifier_uuid_1_20_6(&id);
            w.uuid(uuid).string(&name);
            return Ok(true);
        }
        // Канон → версия, где значения не было (Shape::Const): значение пропускается.
        if let Some(Const(_)) = d {
            crate::item::walk(r, s, depth)?;
            return Ok(true);
        }
        // Канон → 1.21.3: CustomModelData (списки) → одно число — первый float (0 без него).
        if let (Struct(xs), Some(IntAsFloats)) = (s, d) {
            let n = r.length("custom model floats", 1 << 16)?;
            let mut first = 0.0f32;
            for i in 0..n {
                let v = r.f32()?;
                if i == 0 {
                    first = v;
                }
            }
            for x in xs.iter().skip(1) {
                crate::item::walk(r, x, depth)?;
            }
            w.var_int(first as i32);
            return Ok(true);
        }
        // Канон → версия без поздних полей структуры (Shape::Extend): общие поля
        // переводятся, добавленные позже — пропускаются.
        if let (Struct(xs), Some(Extend(ys, _))) = (s, d) {
            if xs.len() > ys.len() {
                let mut ok = true;
                for (x, y) in xs.iter().zip(ys.iter()) {
                    ok &= self.tr(r, w, x, Some(y), depth)?;
                }
                for x in &xs[ys.len()..] {
                    crate::item::walk(r, x, depth)?;
                }
                return Ok(ok);
            }
        }
        // Канон → 1.21.8: TypedEntityData (id реестра, compound) → CustomData с `id` внутри.
        // VANILLA: TypedEntityData (1.21.9) / CustomData (1.21.8).
        if let (Struct([RegistryId(_), Compound]), Some(IdInNbt(reg)), true) = (s, d, self.rev) {
            let id = r.var_int()?;
            let Some(crate::nbt::Tag::Compound(mut fields)) = crate::nbt::read_network(r)? else {
                return Err(bad("expected an NBT compound"));
            };
            let name = self
                .id(*reg, id)
                .and_then(|i| (self.src.names)(*reg).get(i as usize));
            let Some(name) = name else {
                return Ok(false);
            };
            fields.retain(|(k, _)| k != "id");
            fields.insert(
                0,
                (
                    "id".into(),
                    crate::nbt::Tag::String(format!("minecraft:{name}")),
                ),
            );
            let mut buf = Vec::new();
            crate::nbt::write_network(&mut buf, Some(&crate::nbt::Tag::Compound(fields)))?;
            w.bytes(&buf);
            return Ok(true);
        }
        // Исходная форма — Either, каноническая — нет (EitherHolder 1.21.11 → Holder):
        // левая ветвь переводится, ключ реестра (правая) не переводим.
        if let (Either(left, right), false) = (s, self.rev) {
            if !matches!(d, None | Some(Either(..))) {
                return if r.bool()? {
                    self.tr(r, w, left, d, depth)
                } else {
                    self.tr(r, w, right, None, depth)?;
                    Ok(false)
                };
            }
        }
        let leaf = |r: &mut Reader, w: &mut Writer, same: bool| -> ProtocolResult<bool> {
            let start = r.pos();
            crate::item::walk(r, s, depth)?;
            w.bytes(r.since(start));
            Ok(same)
        };
        match s {
            Ref(inner) => self.tr(r, w, inner, d, depth),
            Unit | Bool | Int | Long | Float | Double | VarInt | Utf(_) | Uuid | Nbt | Compound
            | HolderSet => {
                let same =
                    d.is_some_and(|d| std::mem::discriminant(d) == std::mem::discriminant(s));
                leaf(r, w, same)
            }
            RegistrySet(reg) => {
                // Список id переводится, непереводимые выбрасываются; тег копируется.
                let ok = matches!(d, Some(RegistrySet(dr)) if *dr as u8 == *reg as u8);
                match r.var_int()? {
                    0 => {
                        let tag = r.string(MAX_STRING_LENGTH)?;
                        w.var_int(0).string(&tag);
                    }
                    n => {
                        let n = (n as usize).saturating_sub(1);
                        if n > r.remaining() {
                            return Err(bad("holder set longer than the frame"));
                        }
                        let mut ids = Vec::with_capacity(n);
                        for _ in 0..n {
                            if let Some(i) = self.id(*reg, r.var_int()?) {
                                ids.push(i);
                            }
                        }
                        w.var_int(ids.len() as i32 + 1);
                        for i in ids {
                            w.var_int(i);
                        }
                    }
                }
                Ok(ok)
            }
            TagKey => {
                // TagKey (1.21.11) → holderSet: 0, затем id тега.
                let tag = r.string(MAX_STRING_LENGTH)?;
                w.var_int(0).string(&tag);
                Ok(!self.rev && matches!(d, Some(HolderSet)))
            }
            BlockState => {
                let id = r.var_int()?;
                w.var_int((self.block_state)(id as u32) as i32);
                Ok(matches!(d, Some(BlockState)))
            }
            RegistryId(reg) => {
                let ok = self.remap_id(r, w, *reg)?;
                Ok(ok && matches!(d, Some(RegistryId(x)) if *x as u8 == *reg as u8))
            }
            Optional(inner) => {
                let present = r.bool()?;
                w.bool(present);
                let di = match d {
                    Some(Optional(x)) => Some(*x),
                    _ => None,
                };
                if present {
                    let ok = self.tr(r, w, inner, di, depth)?;
                    return Ok(ok && di.is_some());
                }
                Ok(di.is_some())
            }
            List(max, inner) => {
                let n = len(r, *max)?;
                w.var_int(n as i32);
                let di = match d {
                    Some(List(_, x)) => Some(*x),
                    _ => None,
                };
                // Список стеков 1.21.11 → список Optional<шаблон> канона (container).
                let mut ok = di.is_some();
                for _ in 0..n {
                    ok &= self.tr(r, w, inner, if ok { di } else { None }, depth)?;
                }
                Ok(ok)
            }
            Map(k, v) => {
                let n = len(r, usize::MAX)?;
                w.var_int(n as i32);
                let (dk, dv) = match d {
                    Some(Map(a, b)) => (Some(*a), Some(*b)),
                    _ => (None, None),
                };
                let mut ok = dk.is_some();
                for _ in 0..n {
                    ok &= self.tr(r, w, k, if ok { dk } else { None }, depth)?;
                    ok &= self.tr(r, w, v, if ok { dv } else { None }, depth)?;
                }
                Ok(ok)
            }
            Either(left, right) => {
                let is_left = r.bool()?;
                w.bool(is_left);
                let (dl, dr) = match d {
                    Some(Either(a, b)) => (Some(*a), Some(*b)),
                    _ => (None, None),
                };
                let ok = if is_left {
                    self.tr(r, w, left, dl, depth)?
                } else {
                    self.tr(r, w, right, dr, depth)?
                };
                Ok(ok && dl.is_some())
            }
            Struct(fields) => {
                let df = match d {
                    Some(Struct(x)) if x.len() == fields.len() => Some(*x),
                    _ => None,
                };
                let mut ok = df.is_some();
                for (i, f) in fields.iter().enumerate() {
                    let di = if ok { df.map(|x| &x[i]) } else { None };
                    ok &= self.tr(r, w, f, di, depth)?;
                }
                Ok(ok)
            }
            Left(inner) => {
                // Голое значение старой версии → левая ветвь Either канона.
                let dl = match d {
                    Some(Either(l, _)) => Some(*l),
                    _ => None,
                };
                w.bool(true);
                let ok = self.tr(r, w, inner, dl, depth)?;
                Ok(ok && dl.is_some())
            }
            Right(inner) => {
                // Значение старой версии → правая ветвь Either канона.
                let dr = match d {
                    Some(Either(_, r)) => Some(*r),
                    _ => None,
                };
                w.bool(false);
                let ok = self.tr(r, w, inner, dr, depth)?;
                Ok(ok && dr.is_some())
            }
            IdInNbt(reg) => {
                // VANILLA: TypedEntityData (1.21.9) — тип из строки `id` CustomData 1.21.8,
                // сам compound без `id` (TypedEntityData#stripId).
                let tag = crate::nbt::read_network(r)?;
                let Some(crate::nbt::Tag::Compound(mut fields)) = tag else {
                    return Err(bad("expected an NBT compound"));
                };
                let ok = matches!(d, Some(Struct(x))
                    if matches!(x, [RegistryId(dr), Compound] if *dr as u8 == *reg as u8));
                let name = fields
                    .iter()
                    .position(|(k, _)| k == "id")
                    .map(|i| fields.remove(i).1);
                let name = match &name {
                    Some(crate::nbt::Tag::String(n)) => n.strip_prefix("minecraft:").unwrap_or(n),
                    _ => return Ok(false),
                };
                let Some(id) = self
                    .input_names(*reg)
                    .iter()
                    .position(|n| *n == name)
                    .and_then(|i| self.id(*reg, i as i32))
                else {
                    return Ok(false);
                };
                w.var_int(id);
                let mut buf = Vec::new();
                crate::nbt::write_network(&mut buf, Some(&crate::nbt::Tag::Compound(fields)))?;
                w.bytes(&buf);
                Ok(ok)
            }
            Extend(fields, extra) => {
                // Структура старой версии = начало структуры канона; хвост — значения по умолчанию.
                let df = match d {
                    Some(Struct(x)) if x.len() > fields.len() => Some(*x),
                    _ => None,
                };
                let mut ok = df.is_some();
                for (i, f) in fields.iter().enumerate() {
                    let di = if ok { df.map(|x| &x[i]) } else { None };
                    ok &= self.tr(r, w, f, di, depth)?;
                }
                w.bytes(extra);
                Ok(ok)
            }
            Const(bytes) => {
                w.bytes(bytes);
                Ok(d.is_some())
            }
            ModifierUuid => {
                // VANILLA: AttributeModifierIdFix (1.21) — UUID и имя → id модификатора.
                let uuid = r.uuid()?;
                let name = r.string(MAX_STRING_LENGTH)?;
                w.string(&crate::item::modifier_id_1_21(uuid, &name));
                Ok(matches!(d, Some(Utf(_))))
            }
            IntAsFloats => {
                // VANILLA: 1.21.4 CustomModelData — старое число становится единственным float.
                let v = r.var_int()?;
                w.var_int(1).f32(v as f32).var_int(0).var_int(0).var_int(0);
                Ok(matches!(d, Some(Struct(x)) if x.len() == 4))
            }
            Trailing(inner, skipped, _) => {
                // Поле старой версии, снятое позже: значение переводится, поле пропускается.
                let ok = self.tr(r, w, inner, d, depth)?;
                crate::item::walk(r, skipped, depth)?;
                Ok(ok)
            }
            Append(inner, extra) => {
                // Поле, добавленное в каноне: исходное значение + значение по умолчанию.
                let df = match d {
                    Some(Struct(x)) if !x.is_empty() => Some(&x[0]),
                    _ => None,
                };
                let ok = self.tr(r, w, inner, df, depth)?;
                w.bytes(extra);
                Ok(!self.rev && ok && df.is_some())
            }
            Holder(inline) | RegistryHolder(_, inline) => {
                let id = r.var_int()?;
                let di = match d {
                    Some(Holder(x)) | Some(RegistryHolder(_, x)) => Some(*x),
                    _ => None,
                };
                if id == 0 {
                    w.var_int(0);
                    let ok = self.tr(r, w, inline, di, depth)?;
                    return Ok(ok && di.is_some());
                }
                let id = match s {
                    RegistryHolder(reg, _) => match self.id(*reg, id - 1) {
                        Some(c) => c + 1,
                        None => return Ok(false),
                    },
                    _ => id,
                };
                w.var_int(id);
                Ok(di.is_some())
            }
            ById(arms) => {
                let id = r.var_int()?;
                w.var_int(id);
                let arm = arms.get(id as usize).unwrap_or(&arms[0]);
                let di = match d {
                    Some(ById(x)) => x.get(id as usize).or(x.first()),
                    _ => None,
                };
                let ok = self.tr(r, w, arm, di, depth)?;
                Ok(ok && di.is_some())
            }
            ByRegistry(reg, arms) | ByRegistryOr(reg, arms, _) => {
                let id = r.var_int()?;
                let name = usize::try_from(id)
                    .ok()
                    .and_then(|i| self.input_names(*reg).get(i))
                    .copied()
                    .ok_or_else(|| bad("unknown dispatch type"))?;
                let arm = match (arms.iter().find(|(n, _)| *n == name), s) {
                    (Some((_, a)), _) => a,
                    (None, ByRegistryOr(_, _, rest)) => *rest,
                    _ => return Err(bad("unported dispatch type")),
                };
                let darm = match d {
                    Some(ByRegistry(_, x)) => x.iter().find(|(n, _)| *n == name).map(|(_, a)| a),
                    Some(ByRegistryOr(_, x, rest)) => {
                        Some(x.iter().find(|(n, _)| *n == name).map_or(*rest, |(_, a)| a))
                    }
                    _ => None,
                };
                let cid = self.id(*reg, id);
                w.var_int(cid.unwrap_or(0));
                let ok = self.tr(r, w, arm, darm, depth)?;
                Ok(ok && darm.is_some() && cid.is_some())
            }
            Stack => self.stack_value(r, w, d, depth),
            ItemTemplate => {
                // Шаблон в исходной форме (на 1.21.11 не встречается): предмет, количество, патч.
                let item = r.var_int()?;
                let count = r.var_int()?;
                let ci = self.id(ClientRegistry::Item, item);
                w.var_int(ci.unwrap_or(0)).var_int(count);
                let ok = self.patch(r, w, depth)?;
                Ok(!self.rev && ok && ci.is_some() && matches!(d, Some(ItemTemplate)))
            }
            TypedComponent => {
                let id = r.var_int()?;
                let shape = usize::try_from(id)
                    .ok()
                    .and_then(|i| self.input_components().get(i))
                    .copied()
                    .ok_or_else(|| bad("unknown data component type"))?;
                let cid = self.components.get(id as usize).copied().flatten();
                let ds = cid.and_then(|c| self.output_components().get(c as usize).copied());
                w.var_int(cid.map_or(0, |c| c as i32));
                let ok = self.tr(r, w, shape, ds, depth)?;
                Ok(ok && ds.is_some() && matches!(d, Some(TypedComponent)))
            }
        }
    }

    /// Стек 1.21.11 (count, item, patch) → стек канона, шаблон (item, count, patch) или
    /// Optional<шаблон> — по канонической форме.
    fn stack_value(
        &self,
        r: &mut Reader,
        w: &mut Writer,
        d: Option<&'static Shape>,
        depth: u32,
    ) -> ProtocolResult<bool> {
        let count = r.var_int()?;
        if count <= 0 {
            return Ok(match d {
                Some(Shape::Stack) => {
                    w.var_int(0);
                    true
                }
                Some(Shape::Optional(Shape::ItemTemplate)) => {
                    w.bool(false);
                    true
                }
                _ => false,
            });
        }
        let item = r.var_int()?;
        let ci = self.id(ClientRegistry::Item, item);
        let mut patch = Writer::new();
        let ok = self.patch(r, &mut patch, depth)? && ci.is_some();
        let ci = ci.unwrap_or(0);
        let patch = patch.into_vec();
        if self.rev && !matches!(d, Some(Shape::Stack)) {
            return Ok(false);
        }
        match d {
            Some(Shape::Stack) => {
                w.var_int(count).var_int(ci);
            }
            Some(Shape::ItemTemplate) => {
                w.var_int(ci).var_int(count);
            }
            Some(Shape::Optional(Shape::ItemTemplate)) => {
                w.bool(true).var_int(ci).var_int(count);
            }
            _ => return Ok(false),
        }
        w.bytes(&patch);
        Ok(ok)
    }
}

fn len(r: &mut Reader, max: usize) -> ProtocolResult<usize> {
    let n = r.length("collection", max.min(1 << 20))?;
    if n > r.remaining() {
        return Err(bad("collection longer than the frame"));
    }
    Ok(n)
}

/// Пропуск патча компонентов версии соединения (для [`Shape::Stack`] в `walk`).
pub(crate) fn skip_patch_legacy(r: &mut Reader, depth: u32) -> ProtocolResult<()> {
    static T: [OnceLock<Translator>; 10] = [const { OnceLock::new() }; 10];
    let k = source_index(crate::version::current().protocol);
    T[k].get_or_init(|| Translator::new(SOURCES[k], |s| s))
        .patch(r, &mut Writer::new(), depth)?;
    Ok(())
}

/// Переводчик версии соединения (1.21.7–1.21.11) → канон; `block_state` — перевод id
/// состояний (ezzmc-world). Таблицы строятся один раз на версию.
pub fn from_server(block_state: fn(u32) -> u32) -> &'static Translator {
    static T: [OnceLock<Translator>; 10] = [const { OnceLock::new() }; 10];
    let k = source_index(crate::version::current().protocol);
    T[k].get_or_init(|| Translator::new(SOURCES[k], block_state))
}

/// Переводчик канон → версия соединения (пакеты клиента); `block_state` — канон → версия.
pub fn to_server(block_state: fn(u32) -> u32) -> &'static Translator {
    static T: [OnceLock<Translator>; 10] = [const { OnceLock::new() }; 10];
    let k = source_index(crate::version::current().protocol);
    T[k].get_or_init(|| Translator::build(SOURCES[k], block_state, true))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::generated::r1_21_11 as src;

    fn src_id(t: &[&str], n: &str) -> i32 {
        t.iter().position(|x| *x == n).unwrap() as i32
    }

    #[test]
    fn stack_with_container_and_rename() {
        let t = Translator::new(SRC_1_21_11, |s| s);
        let mut w = Writer::new();
        // shulker_box ×1: container [stone ×3, пусто], swing_animation, удалён damage
        w.var_int(1).var_int(src_id(src::ITEM, "shulker_box"));
        w.var_int(2).var_int(1);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "container"))
            .var_int(2);
        w.var_int(3)
            .var_int(src_id(src::ITEM, "stone"))
            .var_int(0)
            .var_int(0);
        w.var_int(0);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "swing_animation"))
            .var_int(1)
            .var_int(6);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "damage"));
        w.u8(0xAB);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.rest(), [0xAB]);
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        let st = crate::item::read_stack(&mut r).unwrap().unwrap();
        assert_eq!(r.remaining(), 0);
        assert_eq!(st.item as i32, src_id(canon::ITEM, "shulker_box"));
        let ids: Vec<i32> = st.patch.added.iter().map(|(i, _)| *i as i32).collect();
        assert_eq!(
            ids,
            [
                src_id(canon::DATA_COMPONENT_TYPE, "container"),
                src_id(canon::DATA_COMPONENT_TYPE, "attack_animation")
            ]
        );
        // container: 2 элемента, [Some(stone ×3), None]
        let mut v = Writer::new();
        v.var_int(2)
            .bool(true)
            .var_int(src_id(canon::ITEM, "stone"))
            .var_int(3);
        v.var_int(0).var_int(0).bool(false);
        assert_eq!(&*st.patch.added[0].1, v.into_vec().as_slice());
        assert_eq!(
            st.patch.removed,
            [src_id(canon::DATA_COMPONENT_TYPE, "damage") as u32]
        );
    }

    #[test]
    fn registry_sets_in_components_remapped() {
        let t = Translator::new(SRC_1_21_11, |s| s);
        let mut w = Writer::new();
        // diamond_pickaxe: repairable [diamond], tool [{blocks [cherry_log, dirt], speed 8}]
        w.var_int(1).var_int(src_id(src::ITEM, "diamond_pickaxe"));
        w.var_int(2).var_int(0);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "repairable"))
            .var_int(2)
            .var_int(src_id(src::ITEM, "diamond"));
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "tool"))
            .var_int(1);
        w.var_int(3)
            .var_int(src_id(src::BLOCK, "cherry_log"))
            .var_int(src_id(src::BLOCK, "dirt"));
        w.bool(true).f32(8.0).bool(false);
        w.f32(1.0).var_int(1).bool(true);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        let st = crate::item::read_stack(&mut r).unwrap().unwrap();
        let mut v = Writer::new();
        v.var_int(2).var_int(src_id(canon::ITEM, "diamond"));
        assert_eq!(&*st.patch.added[0].1, v.into_vec().as_slice());
        let mut v = Writer::new();
        v.var_int(1)
            .var_int(3)
            .var_int(src_id(canon::BLOCK, "cherry_log"))
            .var_int(src_id(canon::BLOCK, "dirt"));
        v.bool(true).f32(8.0).bool(false);
        v.f32(1.0).var_int(1).bool(true);
        assert_eq!(&*st.patch.added[1].1, v.into_vec().as_slice());
        assert_ne!(
            src_id(src::BLOCK, "cherry_log"),
            src_id(canon::BLOCK, "cherry_log")
        );
    }

    #[test]
    fn can_break_block_ids_remapped() {
        let t = Translator::new(SRC_1_21_11, |s| s);
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(src::ITEM, "diamond_pickaxe"));
        w.var_int(1).var_int(0);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "can_break"))
            .var_int(1)
            .bool(true)
            .var_int(3)
            .var_int(src_id(src::BLOCK, "cherry_log"))
            .var_int(src_id(src::BLOCK, "dirt"));
        w.bool(false).bool(false).var_int(0).var_int(0);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let out = out.into_vec();
        let mut r = Reader::new(&out);
        let st = crate::item::read_stack(&mut r).unwrap().unwrap();
        assert_eq!(
            st.patch.added[0].0,
            src_id(canon::DATA_COMPONENT_TYPE, "can_break") as u32
        );
        let mut v = Writer::new();
        v.var_int(1)
            .bool(true)
            .var_int(3)
            .var_int(src_id(canon::BLOCK, "cherry_log"))
            .var_int(src_id(canon::BLOCK, "dirt"));
        v.bool(false).bool(false).var_int(0).var_int(0);
        assert_eq!(&*st.patch.added[0].1, v.into_vec().as_slice());
    }

    /// 1.21.8: entity_data — CustomData с `id` → TypedEntityData; profile — голый Partial →
    /// правая ветвь Either + пустой PlayerSkin.Patch. Обратно entity_data собирается снова.
    #[test]
    fn components_1_21_8_entity_data_and_profile() {
        use crate::generated::r1_21_8 as old;
        use crate::nbt::{write_network, Tag};
        let nbt = |t: Tag| {
            let mut b = Vec::new();
            write_network(&mut b, Some(&t)).unwrap();
            b
        };
        let data = nbt(Tag::Compound(vec![
            ("id".into(), Tag::String("minecraft:pig".into())),
            ("NoAI".into(), Tag::Byte(1)),
        ]));
        let t = Translator::new(SRC_1_21_8, |s| s);
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(old::ITEM, "pig_spawn_egg"));
        w.var_int(2).var_int(0);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "entity_data"))
            .bytes(&data);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "profile"))
            .bool(true)
            .string("Bob")
            .bool(false)
            .var_int(0);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let out = out.into_vec();
        let st = crate::item::read_stack(&mut Reader::new(&out))
            .unwrap()
            .unwrap();
        let mut e = Writer::new();
        e.var_int(src_id(canon::ENTITY_TYPE, "pig"))
            .bytes(&nbt(Tag::Compound(vec![("NoAI".into(), Tag::Byte(1))])));
        let typed = e.into_vec();
        assert_eq!(
            st.patch.added[0],
            (
                src_id(canon::DATA_COMPONENT_TYPE, "entity_data") as u32,
                typed.clone().into()
            )
        );
        let mut e = Writer::new();
        e.bool(false)
            .bool(true)
            .string("Bob")
            .bool(false)
            .var_int(0);
        e.bytes(&[0, 0, 0, 0]);
        assert_eq!(
            st.patch.added[1],
            (
                src_id(canon::DATA_COMPONENT_TYPE, "profile") as u32,
                e.into_vec().into()
            )
        );

        // Обратно: TypedEntityData → compound с `id`.
        let back = Translator::build(SRC_1_21_8, |s| s, true);
        let mut r = Reader::new(&typed);
        let mut o = Writer::new();
        let shape =
            &crate::item::components()[src_id(canon::DATA_COMPONENT_TYPE, "entity_data") as usize];
        let dst = components_1_21_8()[src_id(old::DATA_COMPONENT_TYPE, "entity_data") as usize];
        assert!(back.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(o.into_vec(), data);
    }

    /// 1.21.5: Equippable без canBeSheared/shearingSound — хвост дописывается значениями
    /// по умолчанию (false, SHEARS_SNIP); обратно хвост отбрасывается.
    #[test]
    fn equippable_1_21_5_extended() {
        use crate::generated::r1_21_5 as old;
        let t = Translator::new(SRC_1_21_5, |s| s);
        let snd = src_id(old::SOUND_EVENT, "item.armor.equip_iron");
        let mut v = Writer::new();
        // slot HEAD(5), holder id+1, no asset, no camera overlay, no entities, 4 bools
        v.var_int(5)
            .var_int(snd + 1)
            .bool(false)
            .bool(false)
            .bool(false);
        v.bool(true).bool(true).bool(true).bool(false);
        let old_val = v.into_vec();
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(old::ITEM, "iron_helmet"));
        w.var_int(1).var_int(0);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "equippable"))
            .bytes(&old_val);
        let buf = w.into_vec();
        let mut out = Writer::new();
        t.stack(&mut Reader::new(&buf), &mut out).unwrap();
        let out = out.into_vec();
        let st = crate::item::read_stack(&mut Reader::new(&out))
            .unwrap()
            .unwrap();
        let mut e = Writer::new();
        e.var_int(5)
            .var_int(src_id(canon::SOUND_EVENT, "item.armor.equip_iron") + 1)
            .bool(false)
            .bool(false)
            .bool(false);
        e.bool(true).bool(true).bool(true).bool(false);
        e.bool(false)
            .var_int(0)
            .string("minecraft:item.shears.snip")
            .bool(false);
        let canon_val = e.into_vec();
        assert_eq!(
            st.patch.added[0],
            (
                src_id(canon::DATA_COMPONENT_TYPE, "equippable") as u32,
                canon_val.clone().into()
            )
        );

        let back = Translator::build(SRC_1_21_5, |s| s, true);
        let shape =
            &crate::item::components()[src_id(canon::DATA_COMPONENT_TYPE, "equippable") as usize];
        let dst = components_1_21_5()[src_id(old::DATA_COMPONENT_TYPE, "equippable") as usize];
        let mut r = Reader::new(&canon_val);
        let mut o = Writer::new();
        assert!(back.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(r.remaining(), 0);
        assert_eq!(o.into_vec(), old_val);
    }

    /// 1.21.5: запись атрибута без Display → Display.Default (id 0).
    #[test]
    fn attribute_modifiers_1_21_5_display_default() {
        use crate::generated::r1_21_5 as old;
        let t = Translator::new(SRC_1_21_5, |s| s);
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(old::ITEM, "diamond_sword"));
        w.var_int(1).var_int(0);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "attribute_modifiers"))
            .var_int(1)
            .var_int(src_id(old::ATTRIBUTE, "attack_damage"))
            .string("a:b")
            .f64(7.0)
            .var_int(0)
            .var_int(1);
        let buf = w.into_vec();
        let mut out = Writer::new();
        t.stack(&mut Reader::new(&buf), &mut out).unwrap();
        let out = out.into_vec();
        let st = crate::item::read_stack(&mut Reader::new(&out))
            .unwrap()
            .unwrap();
        let mut e = Writer::new();
        e.var_int(1)
            .var_int(src_id(canon::ATTRIBUTE, "attack_damage"))
            .string("a:b")
            .f64(7.0)
            .var_int(0)
            .var_int(1)
            .var_int(0);
        assert_eq!(
            st.patch.added[0],
            (
                src_id(canon::DATA_COMPONENT_TYPE, "attribute_modifiers") as u32,
                e.into_vec().into()
            )
        );
    }

    /// 1.21.4: хвостовой showInTooltip (Shape::Trailing) пропускается, обратно — true;
    /// Tool без canDestroyBlocksInCreative (Extend, true).
    #[test]
    fn show_in_tooltip_1_21_4_trailing() {
        use crate::generated::r1_21_4 as old;
        let back = Translator::build(SRC_1_21_4, |s| s, true);
        let t = Translator::new(SRC_1_21_4, |s| s);
        let check = |name: &str, old_val: Vec<u8>, canon_val: Vec<u8>, back_val: Vec<u8>| {
            let shape = components_1_21_4()[src_id(old::DATA_COMPONENT_TYPE, name) as usize];
            let dst = &crate::item::components()[src_id(canon::DATA_COMPONENT_TYPE, name) as usize];
            let mut r = Reader::new(&old_val);
            let mut o = Writer::new();
            assert!(t.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap(), "{name}");
            assert_eq!(r.remaining(), 0);
            assert_eq!(o.into_vec(), canon_val, "{name}");
            let mut r = Reader::new(&canon_val);
            let mut o = Writer::new();
            assert!(
                back.tr(&mut r, &mut o, dst, Some(shape), 0).unwrap(),
                "{name}"
            );
            assert_eq!(r.remaining(), 0);
            assert_eq!(o.into_vec(), back_val, "{name}");
        };
        // enchantments: {5: 3}, showInTooltip = false.
        check(
            "enchantments",
            vec![1, 5, 3, 0],
            vec![1, 5, 3],
            vec![1, 5, 3, 1],
        );
        check("unbreakable", vec![0], vec![], vec![1]);
        check(
            "dyed_color",
            vec![0, 0, 0, 7, 0],
            vec![0, 0, 0, 7],
            vec![0, 0, 0, 7, 1],
        );
        // tool: no rules, speed 1.0, damage 1.
        let tool = vec![0, 0x3f, 0x80, 0, 0, 1];
        let mut canon_tool = tool.clone();
        canon_tool.push(1);
        check("tool", tool.clone(), canon_tool, tool);
    }

    /// 1.21.3: custom_model_data — одно число (`Shape::IntAsFloats`) ↔ первый float канона.
    #[test]
    fn custom_model_data_1_21_3_int_as_floats() {
        use crate::generated::r1_21_3 as old;
        let back = Translator::build(SRC_1_21_3, |s| s, true);
        let t = Translator::new(SRC_1_21_3, |s| s);
        let shape =
            components_1_21_3()[src_id(old::DATA_COMPONENT_TYPE, "custom_model_data") as usize];
        let dst = &crate::item::components()
            [src_id(canon::DATA_COMPONENT_TYPE, "custom_model_data") as usize];
        let mut canon_val = vec![1];
        canon_val.extend(7.0f32.to_be_bytes());
        canon_val.extend([0, 0, 0]);
        let mut r = Reader::new(&[7]);
        let mut o = Writer::new();
        assert!(t.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(o.into_vec(), canon_val);
        let mut r = Reader::new(&canon_val);
        let mut o = Writer::new();
        assert!(back.tr(&mut r, &mut o, dst, Some(shape), 0).unwrap());
        assert_eq!(r.remaining(), 0);
        assert_eq!(o.into_vec(), vec![7]);
    }

    /// 1.21.1: еда с eat_seconds/эффектами → FoodProperties 1.21.2 (хвост отбрасывается),
    /// обратно — значения по умолчанию (1.6 с, без эффектов).
    #[test]
    fn food_1_21_1_trailing() {
        use crate::generated::r1_21_1 as old;
        let back = Translator::build(SRC_1_21_1, |s| s, true);
        let t = Translator::new(SRC_1_21_1, |s| s);
        let shape = components_1_21_1()[src_id(old::DATA_COMPONENT_TYPE, "food") as usize];
        let dst = &crate::item::components()[src_id(canon::DATA_COMPONENT_TYPE, "food") as usize];
        let mut v = Writer::new();
        v.var_int(4).f32(2.4).bool(false);
        let head = v.buf.clone();
        v.f32(3.2).bool(false).var_int(0);
        let v = v.into_vec();
        let mut r = Reader::new(&v);
        let mut o = Writer::new();
        assert!(t.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(r.remaining(), 0);
        assert_eq!(o.into_vec(), head);
        let mut r = Reader::new(&head);
        let mut o = Writer::new();
        assert!(back.tr(&mut r, &mut o, dst, Some(shape), 0).unwrap());
        let mut want = head.clone();
        want.extend([0x3f, 0xcc, 0xcc, 0xcd, 0, 0]);
        assert_eq!(o.into_vec(), want);
    }

    /// 1.20.6: модификатор атрибута (UUID, имя) → id по AttributeModifierIdFix, флаг
    /// showInTooltip снимается; обратно — UUID известного id и флаг true.
    #[test]
    fn attribute_modifiers_1_20_6_uuid() {
        use crate::generated::r1_20_6 as old;
        let t = Translator::new(SRC_1_20_6, |s| s);
        let back = Translator::build(SRC_1_20_6, |s| s, true);
        let comp = |names: &[&str]| src_id(names, "attribute_modifiers") as usize;
        let shape = components_1_20_6()[comp(old::DATA_COMPONENT_TYPE)];
        let dst = &crate::item::components()[comp(canon::DATA_COMPONENT_TYPE)];
        let uuid = 0xcb3f55d3_645c_4f38_a497_9c13a33db5cf_u128;
        let mut v = Writer::new();
        v.var_int(1)
            .var_int(src_id(old::ATTRIBUTE, "generic.attack_damage"));
        v.uuid(uuid).string("Weapon modifier").f64(5.0).var_int(0);
        v.var_int(1).bool(true);
        let v = v.into_vec();
        let mut r = Reader::new(&v);
        let mut o = Writer::new();
        assert!(t.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(r.remaining(), 0);
        let mut want = Writer::new();
        want.var_int(1)
            .var_int(src_id(canon::ATTRIBUTE, "attack_damage"));
        want.string("minecraft:base_attack_damage")
            .f64(5.0)
            .var_int(0);
        want.var_int(1).var_int(0);
        let want = want.into_vec();
        assert_eq!(o.into_vec(), want);
        let mut r = Reader::new(&want);
        let mut o = Writer::new();
        assert!(back.tr(&mut r, &mut o, dst, Some(shape), 0).unwrap());
        let mut old_back = Writer::new();
        old_back
            .var_int(1)
            .var_int(src_id(old::ATTRIBUTE, "generic.attack_damage"));
        old_back
            .uuid(uuid)
            .string("minecraft:base_attack_damage")
            .f64(5.0)
            .var_int(0);
        old_back.var_int(1).bool(true);
        assert_eq!(o.into_vec(), old_back.into_vec());
    }

    /// 1.20.6: id модификатора без известного UUID — `minecraft:<uuid>` и обратно тот же UUID.
    #[test]
    fn modifier_id_1_20_6_fallback() {
        use crate::item::{modifier_id_1_21, modifier_uuid_1_20_6};
        let u = 0x0123_4567_89ab_cdef_0011_2233_4455_6677_u128;
        let id = modifier_id_1_21(u, "x");
        assert_eq!(id, "minecraft:01234567-89ab-cdef-0011-223344556677");
        assert_eq!(modifier_uuid_1_20_6(&id).0, u);
        assert_eq!(
            modifier_id_1_21(1, "Random spawn bonus"),
            "minecraft:random_spawn_bonus"
        );
        assert_eq!(
            modifier_uuid_1_20_6("minecraft:random_spawn_bonus").1,
            "Random spawn bonus"
        );
        let (a, _) = modifier_uuid_1_20_6("mymod:speed");
        assert_eq!(a, modifier_uuid_1_20_6("mymod:speed").0);
    }

    /// 1.20.6: еда без using_converts_to — eat_seconds и эффекты снимаются, обратно по умолчанию.
    #[test]
    fn food_1_20_6_trailing() {
        use crate::generated::r1_20_6 as old;
        let back = Translator::build(SRC_1_20_6, |s| s, true);
        let t = Translator::new(SRC_1_20_6, |s| s);
        let shape = components_1_20_6()[src_id(old::DATA_COMPONENT_TYPE, "food") as usize];
        let dst = &crate::item::components()[src_id(canon::DATA_COMPONENT_TYPE, "food") as usize];
        let mut v = Writer::new();
        v.var_int(4).f32(2.4).bool(false);
        let head = v.buf.clone();
        v.f32(3.2).var_int(0);
        let v = v.into_vec();
        let mut r = Reader::new(&v);
        let mut o = Writer::new();
        assert!(t.tr(&mut r, &mut o, shape, Some(dst), 0).unwrap());
        assert_eq!(r.remaining(), 0);
        assert_eq!(o.into_vec(), head);
        let mut r = Reader::new(&head);
        let mut o = Writer::new();
        assert!(back.tr(&mut r, &mut o, dst, Some(shape), 0).unwrap());
        let mut want = head.clone();
        want.extend([0x3f, 0xcc, 0xcc, 0xcd, 0]);
        assert_eq!(o.into_vec(), want);
    }

    /// 1.21.1: `fire_resistant` (Unit) → `damage_resistant` с тегом `#is_fire`; обратно —
    /// пустое значение.
    #[test]
    fn fire_resistant_1_21_1_const() {
        use crate::generated::r1_21_1 as old;
        let t = Translator::new(SRC_1_21_1, |s| s);
        let back = Translator::build(SRC_1_21_1, |s| s, true);
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(old::ITEM, "netherite_ingot"));
        w.var_int(1).var_int(0);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "fire_resistant"));
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let mut want = Writer::new();
        want.var_int(1)
            .var_int(src_id(canon::ITEM, "netherite_ingot"));
        want.var_int(1).var_int(0);
        want.var_int(src_id(canon::DATA_COMPONENT_TYPE, "damage_resistant"));
        want.var_int(0).string("minecraft:is_fire");
        let want = want.into_vec();
        assert_eq!(out.into_vec(), want);
        let mut r = Reader::new(&want);
        let mut o = Writer::new();
        back.stack(&mut r, &mut o).unwrap();
        assert_eq!(r.remaining(), 0);
        assert_eq!(o.into_vec(), buf);
    }

    /// 1.21.10: частичный предикат компонента — голый id типа предиката (`Shape::Left`),
    /// в каноне — левая ветвь Either; id предметов/компонентов/блоков по таблицам 1.21.10.
    #[test]
    fn can_break_1_21_10_partial_predicate_to_either() {
        use crate::generated::r1_21_10 as old;
        let t = Translator::new(SRC_1_21_10, |s| s);
        let mut v = Writer::new();
        v.var_int(1)
            .bool(true)
            .var_int(2)
            .var_int(src_id(old::BLOCK, "cherry_log"));
        v.bool(false).bool(false).var_int(0);
        v.var_int(1).var_int(4).u8(10).u8(0); // partial: тип предиката 4, пустой compound
        let v = v.into_vec();
        let mut w = Writer::new();
        w.var_int(1).var_int(src_id(old::ITEM, "diamond_pickaxe"));
        w.var_int(1).var_int(0);
        w.var_int(src_id(old::DATA_COMPONENT_TYPE, "can_break"))
            .bytes(&v);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let out = out.into_vec();
        let st = crate::item::read_stack(&mut Reader::new(&out))
            .unwrap()
            .unwrap();
        assert_eq!(st.item, src_id(canon::ITEM, "diamond_pickaxe") as u32);
        assert_eq!(
            st.patch.added[0].0,
            src_id(canon::DATA_COMPONENT_TYPE, "can_break") as u32
        );
        let mut e = Writer::new();
        e.var_int(1)
            .bool(true)
            .var_int(2)
            .var_int(src_id(canon::BLOCK, "cherry_log"));
        e.bool(false).bool(false).var_int(0);
        e.var_int(1).bool(true).var_int(4).u8(10).u8(0);
        assert_eq!(&*st.patch.added[0].1, e.into_vec().as_slice());

        // Обратно (канон → 1.21.10): флаг Either снимается.
        let back = Translator::build(SRC_1_21_10, |s| s, true);
        let mut u = Writer::new();
        u.var_int(1).var_int(st.item as i32).var_int(1).var_int(0);
        u.var_int(st.patch.added[0].0 as i32)
            .var_int(st.patch.added[0].1.len() as i32)
            .bytes(&st.patch.added[0].1);
        let ub = u.into_vec();
        let mut o = Writer::new();
        back.stack_untrusted(&mut Reader::new(&ub), &mut o).unwrap();
        let mut x = Writer::new();
        x.var_int(1)
            .var_int(src_id(old::ITEM, "diamond_pickaxe"))
            .var_int(1)
            .var_int(0);
        x.var_int(src_id(old::DATA_COMPONENT_TYPE, "can_break"))
            .var_int(v.len() as i32)
            .bytes(&v);
        assert_eq!(o.into_vec(), x.into_vec());
    }

    #[test]
    fn untrusted_stack_back_to_1_21_11() {
        let t = Translator::build(SRC_1_21_11, |s| s, true);
        let mut w = Writer::new();
        // канон: diamond_pickaxe x1, repairable [diamond], attack_animation, interact_animation
        w.var_int(1).var_int(src_id(canon::ITEM, "diamond_pickaxe"));
        w.var_int(2).var_int(0);
        let mut v = Writer::new();
        v.var_int(2).var_int(src_id(canon::ITEM, "diamond"));
        let v = v.into_vec();
        w.var_int(src_id(canon::DATA_COMPONENT_TYPE, "repairable"))
            .var_int(v.len() as i32)
            .bytes(&v);
        w.var_int(src_id(canon::DATA_COMPONENT_TYPE, "interact_animation"))
            .var_int(1)
            .u8(0);
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let mut out = Writer::new();
        t.stack_untrusted(&mut r, &mut out).unwrap();
        assert_eq!(r.remaining(), 0);
        let mut e = Writer::new();
        e.var_int(1).var_int(src_id(src::ITEM, "diamond_pickaxe"));
        e.var_int(1).var_int(0);
        let mut v = Writer::new();
        v.var_int(2).var_int(src_id(src::ITEM, "diamond"));
        let v = v.into_vec();
        e.var_int(src_id(src::DATA_COMPONENT_TYPE, "repairable"))
            .var_int(v.len() as i32)
            .bytes(&v);
        assert_eq!(out.into_vec(), e.into_vec());
    }

    #[test]
    fn unknown_key_drops_component_only() {
        let t = Translator::new(SRC_1_21_11, |s| s);
        let mut w = Writer::new();
        // damage_type как ключ реестра (правая ветвь EitherHolder) — компонент выпадает
        w.var_int(2).var_int(src_id(src::ITEM, "mace"));
        w.var_int(2).var_int(0);
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "damage_type"))
            .bool(false)
            .string("minecraft:mace_smash");
        w.var_int(src_id(src::DATA_COMPONENT_TYPE, "damage"))
            .var_int(5);
        let buf = w.into_vec();
        let mut out = Writer::new();
        t.stack(&mut Reader::new(&buf), &mut out).unwrap();
        let out = out.into_vec();
        let st = crate::item::read_stack(&mut Reader::new(&out))
            .unwrap()
            .unwrap();
        assert_eq!(st.count, 2);
        assert_eq!(st.patch.added.len(), 1);
        assert_eq!(
            st.patch.added[0].0 as i32,
            src_id(canon::DATA_COMPONENT_TYPE, "damage")
        );
    }
}
