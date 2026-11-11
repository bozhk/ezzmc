#!/usr/bin/env python3
"""Генерирует crates/client/data/tools.txt: правила компонента `tool` предметов 26.3 и раскрытые теги блоков.
VANILLA: Tool#getMiningSpeed/isCorrectForDrops. Формат:
  tag <имя> <блок,блок,...>                 (имена без minecraft:, вложенные теги раскрыты)
  tool <предмет> <default_speed>
  rule <speed|-> <1|0|-> <#тег|блок>        (относится к последнему tool)"""
import json, glob, os, sys
ROOT = "/home/bozhk/PETS/ezzmc/reference/26.3/"
OUT = "/home/bozhk/PETS/ezzmc/crates/client/data/tools.txt"
s = lambda n: n.replace("minecraft:", "")
def expand(tag, seen=()):
    p = f"{ROOT}data/minecraft/tags/block/{tag}.json"
    out = []
    for v in json.load(open(p))["values"]:
        v = v["id"] if isinstance(v, dict) else v
        out += expand(s(v[1:]), seen) if v.startswith("#") else [s(v)]
    return out
tools = {}
for f in sorted(glob.glob(ROOT + "reports/minecraft/components/item/*.json")):
    c = json.load(open(f))["components"].get("minecraft:tool")
    if c: tools[os.path.basename(f)[:-5]] = c
tags = set()
for t in tools.values():
    for r in t["rules"]:
        if r["blocks"].startswith("#"): tags.add(s(r["blocks"][1:]))
with open(OUT, "w") as o:
    for t in sorted(tags): o.write(f"tag {t} {','.join(sorted(set(expand(t))))}\n")
    for n, t in tools.items():
        o.write(f"tool {n} {t.get('default_mining_speed', 1.0)}\n")
        for r in t["rules"]:
            sp = r.get("speed"); cd = r.get("correct_for_drops")
            o.write(f"rule {'-' if sp is None else sp} {'-' if cd is None else int(cd)} {s(r['blocks'])}\n")
print(len(tags), "tags,", len(tools), "tools", file=sys.stderr)
