#!/usr/bin/env python3
"""Генерирует crates/world/data/piston_data.txt из исходников ванили 26.3.
Строки: `P <block> <PUSH|POPPED|IMMOVEABLE>` (PUSH_PULL — по умолчанию, не пишется), `E <block>` (есть BlockEntity).
VANILLA: BlockBehaviour.Properties#pushReaction, BlockEntityTypes (Block#hasBlockEntity)."""
import re, os
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = ROOT + "/reference/26.3/client/net/minecraft/world/level/block/"
OUT = ROOT + "/crates/world/data/piston_data.txt"
COLORS = "white orange magenta light_blue yellow lime pink gray light_gray cyan purple blue brown green red black".split()
bl = open(R + "Blocks.java").read()
stmts = re.split(r'\n   public static final (?:Block|ColorCollection<Block>) ', bl)[1:]
res = {}
coll = {}
for s in stmts:
    name = s.split(" ", 1)[0]
    s = re.split(r'\n   \);', s)[0]
    m = re.search(r'pushReaction\(PushReaction\.(\w+)\)', s)
    t = m.group(1) if m else None
    if t is None:
        m = re.search(r'of(?:Full|Legacy)Copy\((?:Blocks\.)?(\w+)', s) or re.search(r'register(?:Legacy)?(?:Stair|Slab|Wall)\(\s*BlockItemIds\.\w+,\s*(\w+)', s)
        if m:
            t = res.get(m.group(1).lower())
    res[name.lower()] = t or "PUSH_PULL"
    coll[name] = t
out = []
for n, t in res.items():
    if t != "PUSH_PULL":
        out.append(f"P {n} {t}")
# ColorCollection: GLAZED_TERRACOTTA (PUSH)
for c in COLORS:
    if coll.get("GLAZED_TERRACOTTA"):
        out.append(f"P {c}_glazed_terracotta {coll['GLAZED_TERRACOTTA']}")
# блоки с BlockEntity
be = open(R + "entity/BlockEntityTypes.java").read()
names = set()
for m in re.finditer(r'Blocks\.([A-Z0-9_]+)(?!\.asList)', be):
    names.add(m.group(1).lower())
for coll_name, fmt in [("BED", "{}_bed"), ("BANNER", "{}_banner"), ("WALL_BANNER", "{}_wall_banner"), ("DYED_SHULKER_BOX", "{}_shulker_box")]:
    for c in COLORS:
        names.add(fmt.format(c))
for n in sorted(names):
    out.append(f"E {n}")
open(OUT, "w").write("\n".join(out) + "\n")
print(len(out), "lines")
