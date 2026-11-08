//! Blockstates (TODO-096): `blockstates/*.json` — variants, multipart, weights, x/y/z, uvlock.
//! Порт `BlockModelDefinition`, `VariantSelector`, `KeyValueCondition`, `CombinedCondition`,
//! `Variant`, `WeightedVariants`, `MultiPartModel` 1.21.11.
use crate::atlas::Identifier;
use crate::model::Quadrant;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Определение свойства блока: имя и допустимые значения.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDef {
    pub name: String,
    pub values: Vec<String>,
}

/// Значения свойств конкретного состояния.
pub trait StateView {
    fn get(&self, prop: &str) -> Option<&str>;
}

impl StateView for HashMap<String, String> {
    fn get(&self, prop: &str) -> Option<&str> {
        HashMap::get(self, prop).map(String::as_str)
    }
}

fn find<'a>(defs: &'a [PropertyDef], name: &str) -> Option<&'a PropertyDef> {
    defs.iter().find(|d| d.name == name)
}

/// Вариант: модель + повороты (в градусах-квадрантах) + uvlock.
// VANILLA: Variant / Variant.SimpleModelState
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub model: Identifier,
    pub x: Quadrant,
    pub y: Quadrant,
    pub z: Quadrant,
    pub uv_lock: bool,
}

/// Один или несколько взвешенных вариантов.
// VANILLA: SingleVariant / WeightedVariants
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightedVariants {
    pub entries: Vec<(Variant, u32)>,
    pub total_weight: u32,
}

impl WeightedVariants {
    /// Выбор по `roll = random.nextInt(total_weight)`.
    // VANILLA: WeightedList#getRandomOrThrow
    pub fn select(&self, roll: u32) -> &Variant {
        let mut r = roll % self.total_weight.max(1);
        for (v, w) in &self.entries {
            if r < *w {
                return v;
            }
            r -= *w;
        }
        &self.entries[0].0
    }

    pub fn is_single(&self) -> bool {
        self.entries.len() == 1
    }
}

type R<T> = Result<T, String>;

fn parse_variant_fields(o: &Map<String, Value>) -> R<Variant> {
    let model = o
        .get("model")
        .and_then(Value::as_str)
        .ok_or("Missing model")?;
    let q = |k: &str| -> R<Quadrant> {
        match o.get(k) {
            None => Ok(Quadrant::R0),
            Some(v) => Quadrant::parse(v.as_i64().ok_or_else(|| format!("{k}: expected int"))?),
        }
    };
    Ok(Variant {
        model: Identifier::parse(model),
        x: q("x")?,
        y: q("y")?,
        z: q("z")?,
        uv_lock: o.get("uvlock").and_then(Value::as_bool).unwrap_or(false),
    })
}

// VANILLA: BlockStateModel.Unbaked#CODEC (объект или непустой список {model,…,weight})
fn parse_unbaked(v: &Value) -> R<WeightedVariants> {
    match v {
        Value::Object(o) => Ok(WeightedVariants {
            entries: vec![(parse_variant_fields(o)?, 1)],
            total_weight: 1,
        }),
        Value::Array(a) => {
            if a.is_empty() {
                return Err("Expected non-empty list".into());
            }
            let mut entries = Vec::new();
            let mut total = 0u32;
            for e in a {
                let o = e.as_object().ok_or("variant: expected object")?;
                let w = match o.get("weight") {
                    None => 1,
                    Some(w) => match w.as_u64() {
                        Some(n) if n > 0 && n <= i32::MAX as u64 => n as u32,
                        _ => return Err("weight must be positive".into()),
                    },
                };
                total = total.checked_add(w).ok_or("weight overflow")?;
                entries.push((parse_variant_fields(o)?, w));
            }
            Ok(WeightedVariants {
                entries,
                total_weight: total,
            })
        }
        _ => Err("variant: expected object or array".into()),
    }
}

/// Условие мультипарта, скомпилированное в предикат над состоянием.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// `свойство → (значения, negated)`; все проверки по И
    KeyValue(Vec<(String, Vec<(String, bool)>)>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

fn parse_terms(s: &str) -> R<Vec<(String, bool)>> {
    let mut out = Vec::new();
    for t in s.split('|') {
        let (val, neg) = match t.strip_prefix('!') {
            Some(r) => (r, true),
            None => (t, false),
        };
        if val.is_empty() {
            return Err(format!("Empty term in value '{s}'"));
        }
        out.push((val.to_owned(), neg));
    }
    Ok(out)
}

// VANILLA: Condition.CODEC
fn parse_condition(v: &Value) -> R<Condition> {
    let o = v.as_object().ok_or("condition: expected object")?;
    if o.len() == 1 {
        let (k, val) = o.iter().next().ok_or("empty")?;
        if k == "AND" || k == "OR" {
            let list = val
                .as_array()
                .ok_or("combiner: expected list")?
                .iter()
                .map(parse_condition)
                .collect::<R<Vec<_>>>()?;
            return Ok(if k == "AND" {
                Condition::And(list)
            } else {
                Condition::Or(list)
            });
        }
    }
    if o.is_empty() {
        return Err("Expected non-empty map".into());
    }
    let mut tests = Vec::new();
    for (k, val) in o {
        let s = match val {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) if n.is_i64() || n.is_u64() => n.to_string(),
            _ => return Err(format!("Invalid value for property {k}")),
        };
        tests.push((k.clone(), parse_terms(&s)?));
    }
    Ok(Condition::KeyValue(tests))
}

impl Condition {
    /// Проверка с учётом определений свойств (неизвестное свойство/значение → `Err`).
    // VANILLA: KeyValueCondition#instantiate (ошибки на неизвестных именах/значениях)
    pub fn validate(&self, defs: &[PropertyDef]) -> R<()> {
        match self {
            Condition::And(l) | Condition::Or(l) => l.iter().try_for_each(|c| c.validate(defs)),
            Condition::KeyValue(tests) => {
                for (k, terms) in tests {
                    let d = find(defs, k).ok_or_else(|| format!("Unknown property '{k}'"))?;
                    for (v, _) in terms {
                        if !d.values.contains(v) {
                            return Err(format!("Unknown value '{v}' for property '{k}'"));
                        }
                    }
                }
                Ok(())
            }
        }
    }

    pub fn test(&self, s: &dyn StateView) -> bool {
        match self {
            Condition::And(l) => l.iter().all(|c| c.test(s)),
            Condition::Or(l) => l.iter().any(|c| c.test(s)),
            Condition::KeyValue(tests) => tests.iter().all(|(k, terms)| {
                let cur = s.get(k);
                // anyOf(terms): value == t (или != t для negated)
                terms
                    .iter()
                    .any(|(v, neg)| (cur == Some(v.as_str())) != *neg)
            }),
        }
    }
}

/// Часть мультипарта.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiPartSelector {
    pub when: Option<Condition>,
    pub apply: WeightedVariants,
}

/// Содержимое `blockstates/<block>.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockModelDefinition {
    /// `ключ варианта "a=b,c=d"` → варианты (порядок файла)
    pub variants: Option<Vec<(String, WeightedVariants)>>,
    pub multipart: Option<Vec<MultiPartSelector>>,
}

/// Модель для состояния блока.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateModel<'a> {
    Single(&'a WeightedVariants),
    /// все подходящие части мультипарта (каждая со своим весовым выбором)
    MultiPart(Vec<&'a WeightedVariants>),
}

impl BlockModelDefinition {
    // VANILLA: BlockModelDefinition.CODEC
    pub fn parse(data: &[u8]) -> R<BlockModelDefinition> {
        let v: Value = serde_json::from_slice(data).map_err(|e| e.to_string())?;
        let o = v.as_object().ok_or("blockstate: expected object")?;
        let variants = match o.get("variants") {
            None => None,
            Some(vs) => {
                let m = vs.as_object().ok_or("variants: expected object")?;
                if m.is_empty() {
                    return Err("Expected non-empty map".into());
                }
                Some(
                    m.iter()
                        .map(|(k, v)| Ok((k.clone(), parse_unbaked(v)?)))
                        .collect::<R<Vec<_>>>()?,
                )
            }
        };
        let multipart = match o.get("multipart") {
            None => None,
            Some(mp) => {
                let a = mp.as_array().ok_or("multipart: expected list")?;
                if a.is_empty() {
                    return Err("Expected non-empty list".into());
                }
                Some(
                    a.iter()
                        .map(|e| {
                            let eo = e.as_object().ok_or("selector: expected object")?;
                            Ok(MultiPartSelector {
                                when: match eo.get("when") {
                                    None => None,
                                    Some(c) => Some(parse_condition(c)?),
                                },
                                apply: parse_unbaked(eo.get("apply").ok_or("Missing apply")?)?,
                            })
                        })
                        .collect::<R<Vec<_>>>()?,
                )
            }
        };
        if variants.is_none() && multipart.is_none() {
            return Err("Neither 'variants' nor 'multipart' found".into());
        }
        Ok(BlockModelDefinition {
            variants,
            multipart,
        })
    }

    /// Все файлы моделей, на которые ссылается определение.
    pub fn dependencies(&self) -> Vec<&Identifier> {
        let mut out = Vec::new();
        if let Some(vs) = &self.variants {
            for (_, w) in vs {
                out.extend(w.entries.iter().map(|(v, _)| &v.model));
            }
        }
        if let Some(mp) = &self.multipart {
            for s in mp {
                out.extend(s.apply.entries.iter().map(|(v, _)| &v.model));
            }
        }
        out
    }

    /// Состояние под ключ `variants`: все указанные свойства совпадают.
    // VANILLA: VariantSelector#predicate
    fn variant_matches(key: &str, defs: &[PropertyDef], s: &dyn StateView) -> R<bool> {
        let mut ok = true;
        for part in key.split(',') {
            let (name, val) = match part.split_once('=') {
                Some((n, v)) => (n, Some(v)),
                None => (part, None),
            };
            match (find(defs, name), val) {
                (Some(d), Some(v)) => {
                    if !d.values.iter().any(|x| x == v) {
                        return Err(format!(
                            "Unknown value: '{v}' for blockstate property: '{name}' {:?}",
                            d.values
                        ));
                    }
                    if s.get(name) != Some(v) {
                        ok = false;
                    }
                }
                _ if !name.is_empty() => {
                    return Err(format!("Unknown blockstate property: '{name}'"));
                }
                _ => {}
            }
        }
        Ok(ok)
    }

    /// Модель состояния. `variants` приоритетнее `multipart` (`putIfAbsent`).
    /// Ошибочный ключ варианта пропускается с предупреждением, как в ванили.
    // VANILLA: BlockModelDefinition#instantiate
    pub fn model_for<'a>(
        &'a self,
        defs: &[PropertyDef],
        state: &dyn StateView,
        debug_name: &str,
    ) -> Option<StateModel<'a>> {
        if let Some(vs) = &self.variants {
            // при пересечении в ванили — исключение; берём первый совпавший
            for (key, w) in vs {
                match Self::variant_matches(key, defs, state) {
                    Ok(true) => return Some(StateModel::Single(w)),
                    Ok(false) => {}
                    Err(e) => tracing::warn!(
                        "Exception loading blockstate definition: '{debug_name}' for variant: '{key}': {e}"
                    ),
                }
            }
        }
        if let Some(mp) = &self.multipart {
            let mut parts = Vec::new();
            for sel in mp {
                if let Some(c) = &sel.when {
                    if let Err(e) = c.validate(defs) {
                        tracing::warn!("blockstate '{debug_name}': {e}");
                        continue;
                    }
                    if !c.test(state) {
                        continue;
                    }
                }
                parts.push(&sel.apply);
            }
            return Some(StateModel::MultiPart(parts));
        }
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn defs() -> Vec<PropertyDef> {
        let p = |n: &str, v: &[&str]| PropertyDef {
            name: n.into(),
            values: v.iter().map(|s| (*s).into()).collect(),
        };
        vec![
            p("facing", &["north", "south"]),
            p("lit", &["true", "false"]),
        ]
    }

    fn st(f: &str, l: &str) -> HashMap<String, String> {
        HashMap::from([("facing".into(), f.into()), ("lit".into(), l.into())])
    }

    #[test]
    fn variants_select() {
        let d = BlockModelDefinition::parse(
            br#"{"variants":{"facing=north,lit=true":{"model":"block/a","y":90,"uvlock":true},
            "facing=south":[{"model":"block/b","weight":3},{"model":"block/c"}]}}"#,
        )
        .unwrap();
        let m = d.model_for(&defs(), &st("north", "true"), "t").unwrap();
        let StateModel::Single(w) = m else { panic!() };
        assert_eq!(w.entries[0].0.y, Quadrant::R90);
        assert!(w.entries[0].0.uv_lock);
        let StateModel::Single(w) = d.model_for(&defs(), &st("south", "false"), "t").unwrap()
        else {
            panic!()
        };
        assert_eq!(w.total_weight, 4);
        assert_eq!(w.select(2).model.path, "block/b");
        assert_eq!(w.select(3).model.path, "block/c");
        assert!(d.model_for(&defs(), &st("north", "false"), "t").is_none());
        assert_eq!(d.dependencies().len(), 3);
    }

    #[test]
    fn multipart_conditions() {
        let d = BlockModelDefinition::parse(
            br#"{"multipart":[{"apply":{"model":"block/base"}},
            {"when":{"facing":"north|south","lit":"!false"},"apply":{"model":"block/on"}},
            {"when":{"OR":[{"lit":false},{"facing":"south"}]},"apply":{"model":"block/or"}},
            {"when":{"nope":"x"},"apply":{"model":"block/bad"}}]}"#,
        )
        .unwrap();
        let StateModel::MultiPart(p) = d.model_for(&defs(), &st("north", "true"), "t").unwrap()
        else {
            panic!()
        };
        let names: Vec<_> = p
            .iter()
            .map(|w| w.entries[0].0.model.path.as_str())
            .collect();
        assert_eq!(names, ["block/base", "block/on"]);
        let StateModel::MultiPart(p) = d.model_for(&defs(), &st("south", "false"), "t").unwrap()
        else {
            panic!()
        };
        assert_eq!(p.len(), 2); // base + or
    }

    #[test]
    fn errors() {
        assert!(BlockModelDefinition::parse(b"{}").is_err());
        assert!(BlockModelDefinition::parse(br#"{"variants":{}}"#).is_err());
        assert!(BlockModelDefinition::parse(br#"{"variants":{"":[]}}"#).is_err());
        assert!(BlockModelDefinition::parse(br#"{"variants":{"":{"model":"a","x":45}}}"#).is_err());
        assert!(
            BlockModelDefinition::parse(br#"{"variants":{"":[{"model":"a","weight":0}]}}"#)
                .is_err()
        );
        // неизвестное свойство в ключе → предупреждение, состояние без модели
        let d = BlockModelDefinition::parse(br#"{"variants":{"bad=1":{"model":"a"}}}"#).unwrap();
        assert!(d.model_for(&defs(), &st("north", "true"), "t").is_none());
    }
}
