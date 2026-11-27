#!/usr/bin/env python3
"""Поля SynchedEntityData версий 1.14.4–1.20.1 → индексы и сериализаторы 26.3 (сопоставление по имени поля).

Пишет crates/net/src/entity_data_gen.rs. Запуск из корня: python3 tools/gen_entity_data.py
"""
import difflib
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


# 1.13.2 есть только с именами Yarn: сопоставление не по именам полей, а выравниванием
# последовательности сериализаторов типа с 1.14.4 → индекс 1.13 → индекс 1.14.4.
YARN_SER = {"INTEGER": "INT", "TEXT_COMPONENT": "COMPONENT", "field_21650": "OPTIONAL_COMPONENT",
            "ROTATION": "ROTATIONS", "FACING": "DIRECTION", "NBT_COMPOUND": "COMPOUND_TAG",
            "field_21651": "PARTICLE", "OPTIONAL_BLOCK_STATE": "BLOCK_STATE"}


def load_113():
    base = Path("reference/1.13.2/client/net/minecraft")
    parent, fields = {}, {}
    for f in sorted(base.rglob("*.java")):
        src = f.read_text()
        for m in CLASS.finditer(src):
            parent.setdefault(m[1], m[2].split(".")[-1])
        for m in re.finditer(r"registerData\(\s*(\w+)\.class,\s*TrackedDataHandlerRegistry\.(\w+)", src):
            fields.setdefault(m[1], []).append(YARN_SER.get(m[2], m[2]))
    decl = (base / "entity/EntityType.java").read_text()
    reg = {m[1]: m[2] for m in re.finditer(r'register\(\s*"(\w+)",\s*(?:EntityType\.)?EntityBuilder\.\w+\(\s*(\w+)\.class', decl)}
    reg.setdefault("player", "PlayerEntity")

    def chain(cls):
        out = []
        while cls:
            out = fields.get(cls, []) + out
            cls = parent.get(cls)
        return out

    return {name: chain(cls) for name, cls in reg.items()}


old = load_113()
new, _ = load("1.14.4")
remaps = []
for name, a in sorted(old.items()):
    key = TYPE_ALIAS.get(name, name)
    src = new.get(name)
    if src is None:
        print("1.13: нет типа в 1.14.4:", name)
        continue
    b = ["BLOCK_STATE" if s == "OPTIONAL_BLOCK_STATE" else s for _, s in src]
    idx = [255] * len(a)
    for i, j, n in difflib.SequenceMatcher(None, a, b, autojunk=False).get_matching_blocks():
        for k in range(n):
            idx[i + k] = j + k
    # зомби 1.13: поле 14 «поднятые руки» удалено в 1.14 (выравнивание по типам этого не видит)
    if name in ("zombie", "husk", "drowned", "zombie_pigman", "zombie_villager"):
        idx[14:] = [255] + idx[14:-1]
    if 255 in idx:
        print("1.13: поля без пары", name, [(i, a[i]) for i, v in enumerate(idx) if v == 255])
    remaps.append((key, idx))

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
o += ["/// 1.13.x: (тип сущности 26.3, индекс 1.14.4 по индексу 1.13 или 255), отсортировано по имени.",
      "pub const REMAP_1_13: &[(&str, &[u8])] = &["]
o += [f'    ("{k}", &{v}),' for k, v in sorted(remaps)]
o += ["];", ""]
Path("crates/net/src/entity_data_gen.rs").write_text("\n".join(o))
print("lists", len(lists), "types", [len(r) for r in eras])
for f, k in sorted(dropped.items()):
    print("drop", f, len(k), sorted(k)[:3])
