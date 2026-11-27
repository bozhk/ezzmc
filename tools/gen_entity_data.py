#!/usr/bin/env python3
"""Поля SynchedEntityData версий 1.14.4–1.20.1 → индексы и сериализаторы 26.3 (сопоставление по имени поля).

Пишет crates/net/src/entity_data_gen.rs. Запуск из корня: python3 tools/gen_entity_data.py
"""
import re
from pathlib import Path

# порядок = v1201::era()
ERAS = ["1.20.1", "1.19.4", "1.19.3", "1.19.2", "1.19", "1.18.2", "1.17.1", "1.16.5", "1.16.1", "1.15.2", "1.14.4"]
CANON = "26.3"
# имя типа сущности старой версии → имя в 26.3
TYPE_ALIAS = {"boat": "oak_boat", "chest_boat": "oak_chest_boat", "potion": "splash_potion",
              "zombie_pigman": "zombified_piglin", "chain_command_block_minecart": "command_block_minecart"}
DEFINE = re.compile(r"EntityDataAccessor<[^=;]*?>\s+(\w+)\s*=\s*(?:\w+\.)?defineId\(\s*(\w+)\.class,\s*EntityDataSerializers\.(\w+)")
CLASS = re.compile(r"\bclass\s+(\w+)(?:<[^{]*?>)?\s+extends\s+([\w.]+)")
TYPE = re.compile(r"EntityType<(\w+)(?:<[^=]*>)?>\s+(\w+)\s*=\s*(?:EntityType\.)?register")


def load(v):
    root = Path("reference") / v
    base = next(root.rglob("net/minecraft/world/entity/EntityType.java")).parent
    parent, fields = {}, {}
    for f in sorted(base.rglob("*.java")):
        src = f.read_text()
        for m in CLASS.finditer(src):
            parent.setdefault(m[1], m[2].split(".")[-1])
        for m in DEFINE.finditer(src):
            fields.setdefault(m[2], []).append((m[1], m[3]))
    # 26.3 объявляет типы в EntityTypes.java
    decl = base / "EntityTypes.java"
    decl = decl if decl.exists() else base / "EntityType.java"
    reg = {m[2].lower(): m[1] for m in TYPE.finditer(decl.read_text())}

    def chain(cls):
        out = []
        while cls:
            out = fields.get(cls, []) + out
            cls = parent.get(cls)
        return out

    order = re.search(r"static\s*\{(.*?)\n   \}", (base.parent.parent / "network/syncher/EntityDataSerializers.java").read_text(), re.S)[1]
    ser = re.findall(r"registerSerializer\((\w+)\)", order)
    return {name: chain(cls) for name, cls in reg.items()}, ser


def wire(ser, v):
    """Имя сериализатора старой версии → имя того же формата в 26.3."""
    if ser == "OPTIONAL_UUID":
        return "OPTIONAL_LIVING_ENTITY_REFERENCE"
    # до 1.19.4 BLOCK_STATE был Optional (0 — пусто)
    if ser == "BLOCK_STATE" and v not in ("1.20.1", "1.19.4"):
        return "OPTIONAL_BLOCK_STATE"
    return ser


canon, canon_ser = load(CANON)
if not canon_ser:
    raise SystemExit("26.3: нет списка сериализаторов")
lists, eras, sers, dropped = {}, [], [], {}
for v in ERAS:
    types, ser = load(v)
    assert types and ser, v
    sers.append([wire(s, v) for s in ser])
    rows = []
    for name, fields in sorted(types.items()):
        key = TYPE_ALIAS.get(name, name)
        if key not in canon:
            continue
        target = {f: (i, s) for i, (f, s) in enumerate(canon[key])}
        out = []
        for field, s in fields:
            t = target.get(field)
            w = wire(s, v)
            if t and (t[1] == w or {t[1], w} == {"BLOCK_STATE", "OPTIONAL_BLOCK_STATE"}):
                out.append((t[0], canon_ser.index(w)))
            else:
                out.append((255, 0))
                dropped.setdefault(field, set()).add(key)
        out = tuple(out)
        lists.setdefault(out, f"L{len(lists)}")
        rows.append((key, lists[out]))
    eras.append(sorted(set(rows)))

o = ["//! Сгенерировано `tools/gen_entity_data.py` из `reference/`. Не править руками.", "",
     "/// Поля по старому индексу: (индекс 26.3 или 255 — поля нет, id сериализатора 26.3).",
     "pub type Fields = &'static [(u8, u8)];", ""]
for l, n in lists.items():
    o.append(f"const {n}: Fields = &[{', '.join(f'({a}, {b})' for a, b in l)}];")
o += ["", "/// Сериализаторы версии по id (имена формата 26.3); порядок эр — `v1201::era`.",
      f"pub const SERIALIZERS: [&[&str]; {len(ERAS)}] = ["]
o += [f"    &[{', '.join(chr(34) + s + chr(34) for s in l)}]," for l in sers]
o += ["];", "", "/// (тип сущности 26.3, поля), отсортировано по имени.",
      f"pub const TYPES: [&[(&str, Fields)]; {len(ERAS)}] = ["]
o += [f"    &[{', '.join(f'({chr(34)}{k}{chr(34)}, {n})' for k, n in r)}]," for r in eras]
o += ["];", ""]
Path("crates/net/src/entity_data_gen.rs").write_text("\n".join(o))
print("lists", len(lists), "types", [len(r) for r in eras])
for f, k in sorted(dropped.items()):
    print("drop", f, len(k), sorted(k)[:3])
