#!/usr/bin/env python3
"""Генерирует crates/client/data/sound_types.txt и block_sounds.txt из исходников ванили 26.3.
VANILLA: SoundType.java, SoundEvents.java, Blocks.java (`.sound(SoundType.X)`, ofFullCopy/ofLegacyCopy)."""
import re, sys, os
R = "/home/bozhk/PETS/ezzmc/reference/26.3/client/net/minecraft/"
OUT = "/home/bozhk/PETS/ezzmc/crates/client/data/"
se = open(R + "sounds/SoundEvents.java").read()
ev = dict(re.findall(r'public static final (?:SoundEvent|Holder\.Reference<SoundEvent>) (\w+) = register(?:ForHolder)?\("([^"]+)"\)', se))
st = open(R + "world/level/block/SoundType.java").read()
types = {}
for m in re.finditer(r'public static final SoundType (\w+) = new SoundType\(([^;]*?)\);', st, re.S):
    args = [a.strip() for a in m.group(2).split(",")]
    vol, pit = float(args[0].rstrip("F")), float(args[1].rstrip("F"))
    evs = []
    for a in args[2:7]:
        k = a.replace("SoundEvents.", "")
        evs.append("" if k == "EMPTY" else ev.get(k, "?" + k))
    types[m.group(1)] = (vol, pit, evs)
bad = [(n, e) for n, (_, _, es) in types.items() for e in es if e.startswith("?")]
if bad: print("unresolved events", bad, file=sys.stderr)
with open(OUT + "sound_types.txt", "w") as f:
    for n, (v, p, es) in types.items():
        f.write(f"{n} {v} {p} " + " ".join(e or "-" for e in es) + "\n")
# звук по BlockSetType / WoodType (конструкторы по умолчанию → WOOD / HANGING_SIGN)
def _consts(path, cls, default, idx):
    src = open(path).read()
    out = {}
    for m in re.finditer(r'public static final %s (\w+) = register\(\s*new %s\((.*?)\)\s*\);' % (cls, cls), src, re.S):
        sts = re.findall(r'SoundType\.(\w+)', m.group(2))
        out[m.group(1)] = sts[idx] if len(sts) > idx else default
    return out
_P = R + "world/level/block/state/properties/"
SET_T = _consts(_P + "BlockSetType.java", "BlockSetType", "WOOD", 0)
WOOD_T = _consts(_P + "WoodType.java", "WoodType", "WOOD", 0)
HANGING = _consts(_P + "WoodType.java", "WoodType", "HANGING_SIGN", 1)
bl = open(R + "world/level/block/Blocks.java").read()
# вспомогательные методы со звуком по умолчанию
helper = {"netherStemProperties": "STEM", "candleProperties": "CANDLE"}
stmts = re.split(r'\n   public static final Block ', bl)[1:]
res = {}
for s in stmts:
    s = re.split(r'\n   \);', s)[0]  # конец объявления (иначе захватывает ColorCollection следующих)
    name = s.split(" ", 1)[0].lower()
    sounds = re.findall(r'SoundType\.(\w+)', s)
    t = None
    if sounds:
        t = sounds[-1]
    elif (m := re.search(r'new (\w+)\((?:[^;]*?)(WoodType|BlockSetType)\.(\w+)', s)):
        # VANILLA: DoorBlock/TrapDoorBlock/ButtonBlock/PressurePlate/FenceGate/…Sign: super(properties.sound(type.soundType()))
        cls, kind, nm = m.groups()
        t = (HANGING if "Hanging" in cls else WOOD_T if kind == "WoodType" else SET_T).get(nm)
    elif (m := re.search(r'register(?:Legacy)?(?:Stair|Slab|Wall)\(\s*BlockItemIds\.\w+,\s*(\w+)', s)):
        # VANILLA: registerSlab/registerStair/registerWall → ofLegacyCopy/ofFullCopy(base): звук базы
        t = res.get(m.group(1).lower())
    if t is None and not sounds:
        for h, v in helper.items():
            if h + "(" in s: t = v
        if t is None:
            m = re.search(r'of(?:Full|Legacy)Copy\((?:Blocks\.)?(\w+)', s)
            if m and m.group(1).lower() in res: t = res[m.group(1).lower()]
        if t is None and "shulkerBoxProperties" in s: t = "STONE"
    res[name] = t or "STONE"
sys.path.insert(0, os.path.dirname(__file__))
blocks = re.findall(r'"([a-z0-9_]+)"', re.search(r'pub static BLOCK: &\[&str\] = &\[(.*?)\];', open("/home/bozhk/PETS/ezzmc/crates/protocol/src/generated/r26_3.rs").read(), re.S).group(1))
def heur(n):
    if "wool" in n: return "WOOL"
    if "concrete" in n: return "STONE"
    if "copper" in n or n.endswith("lightning_rod"):
        for k, t in [("bulb","COPPER_BULB"),("grate","COPPER_GRATE"),("golem_statue","COPPER_GOLEM_STATUE"),("chain","CHAIN"),("lantern","LANTERN")]:
            if k in n: return t
        return "COPPER"
    if n.startswith("potted_"): return "STONE"
    for suf, t in [("_wool","WOOL"),("_carpet","WOOL"),("_concrete_powder","SAND"),("_concrete","STONE"),("glass_pane","GLASS"),("glass","GLASS"),("_terracotta","STONE"),("_bed","WOOD"),("_banner","WOOD"),("_candle","CANDLE"),("_shulker_box","STONE"),("_candle_cake","CANDLE")]:
        if n.endswith(suf) or suf in n and suf in ("glass",): return t
    return None
miss = []
with open(OUT + "block_sounds.txt", "w") as f:
    for b in blocks:
        t = res.get(b)
        if t is None:
            t = heur(b)
            if t is None: miss.append(b); t = "STONE"
        if t != "STONE": f.write(f"{b} {t}\n")
print("missing->STONE:", miss, file=sys.stderr)
print(len(types), "types", len(blocks), "blocks", file=sys.stderr)
