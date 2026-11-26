//! Перевод значений по грамматике [`Shape`] из версии сервера в канон (26.3) — глава 18, TODO-372a.
//! Обход двух форм параллельно: исходной (версия сервера) и канонической. Байты копируются,
//! id статических реестров (предметы, компоненты, звуки, частицы, сущности, атрибуты, состояния
//! блоков) переводятся по именам. Несовместимое значение (`Ok(false)`) вызывающий отбрасывает:
//! компонент выпадает из патча, стек становится пустым.
use crate::error::{ProtocolError, ProtocolResult};
use crate::generated::{r1_21_11 as src, r26_3 as canon};
use crate::item::{components, components_1_21_11, ClientRegistry, Shape, MAX_DEPTH};
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

fn src_names(reg: ClientRegistry) -> &'static [&'static str] {
    match reg {
        ClientRegistry::SoundEvent => src::SOUND_EVENT,
        ClientRegistry::Item => src::ITEM,
        ClientRegistry::DataComponentType => src::DATA_COMPONENT_TYPE,
        ClientRegistry::PositionSourceType => src::POSITION_SOURCE_TYPE,
        ClientRegistry::ParticleType => src::PARTICLE_TYPE,
        ClientRegistry::BlockEntityType => src::BLOCK_ENTITY_TYPE,
        ClientRegistry::ConsumeEffectType => src::CONSUME_EFFECT_TYPE,
        ClientRegistry::EntityType => src::ENTITY_TYPE,
        ClientRegistry::Attribute => src::ATTRIBUTE,
        ClientRegistry::Block => src::BLOCK,
    }
}

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

/// Компоненты, переименованные в каноне: 1.21.11 → 26.3.
fn canon_component_name(name: &str) -> &str {
    match name {
        // DataComponents (26.3): swing_animation разделён на attack/interact_animation.
        "swing_animation" => "attack_animation",
        n => n,
    }
}

/// Перевод из 1.21.11: таблицы id → канон и перевод состояний блоков.
pub struct Translator {
    remaps: Vec<Vec<Option<u32>>>,
    components: Vec<Option<u32>>,
    block_state: fn(u32) -> u32,
}

impl Translator {
    fn new(block_state: fn(u32) -> u32) -> Self {
        let remaps = REGISTRIES
            .iter()
            .map(|r| build_remap(src_names(*r), canon_names(*r)))
            .collect();
        let components = src::DATA_COMPONENT_TYPE
            .iter()
            .map(|n| {
                let n = canon_component_name(n);
                canon::DATA_COMPONENT_TYPE
                    .iter()
                    .position(|c| *c == n)
                    .map(|i| i as u32)
            })
            .collect();
        Self {
            remaps,
            components,
            block_state,
        }
    }

    /// id реестра 1.21.11 → канон.
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
        let shapes = components_1_21_11();
        let dst = components();
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
            let name = src::DATA_COMPONENT_TYPE.get(id as usize).copied();
            let ok = match canon_id {
                Some(_) if name == Some("pot_decorations") => self.pot_decorations(r, &mut tmp)?,
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
        // Исходная форма — Either, каноническая — нет (EitherHolder 1.21.11 → Holder):
        // левая ветвь переводится, ключ реестра (правая) не переводим.
        if let Either(left, right) = s {
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
                Ok(matches!(d, Some(HolderSet)))
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
            Append(inner, extra) => {
                // Поле, добавленное в каноне: исходное значение + значение по умолчанию.
                let df = match d {
                    Some(Struct(x)) if !x.is_empty() => Some(&x[0]),
                    _ => None,
                };
                let ok = self.tr(r, w, inner, df, depth)?;
                w.bytes(extra);
                Ok(ok && df.is_some())
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
                    .and_then(|i| src_names(*reg).get(i))
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
                Ok(ok && ci.is_some() && matches!(d, Some(ItemTemplate)))
            }
            TypedComponent => {
                let id = r.var_int()?;
                let shape = usize::try_from(id)
                    .ok()
                    .and_then(|i| components_1_21_11().get(i))
                    .copied()
                    .ok_or_else(|| bad("unknown data component type"))?;
                let cid = self.components.get(id as usize).copied().flatten();
                let ds = cid.and_then(|c| components().get(c as usize).copied());
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

/// Пропуск патча компонентов 1.21.11 (для [`Shape::Stack`] в `walk`).
pub(crate) fn skip_patch_1_21_11(r: &mut Reader, depth: u32) -> ProtocolResult<()> {
    static T: OnceLock<Translator> = OnceLock::new();
    T.get_or_init(|| Translator::new(|s| s))
        .patch(r, &mut Writer::new(), depth)?;
    Ok(())
}

static T_1_21_11: OnceLock<Translator> = OnceLock::new();

/// Переводчик 1.21.11 → канон; `block_state` — перевод id состояний (ezzmc-world).
pub fn v1_21_11(block_state: fn(u32) -> u32) -> &'static Translator {
    T_1_21_11.get_or_init(|| Translator::new(block_state))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn src_id(t: &[&str], n: &str) -> i32 {
        t.iter().position(|x| *x == n).unwrap() as i32
    }

    #[test]
    fn stack_with_container_and_rename() {
        let t = Translator::new(|s| s);
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
        let t = Translator::new(|s| s);
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
        let t = Translator::new(|s| s);
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

    #[test]
    fn unknown_key_drops_component_only() {
        let t = Translator::new(|s| s);
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
