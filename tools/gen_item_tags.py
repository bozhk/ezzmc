#!/usr/bin/env python3
"""Плоские теги предметов 26.3 -> crates/client/data/item_tags.txt (`тег предмет предмет ...`).
VANILLA: data/minecraft/tags/item/**.json, ссылки `#minecraft:x` разворачиваются."""
import json, os, sys
root = "reference/26.3/data/minecraft/tags/item"
raw = {}
for d, _, fs in os.walk(root):
    for f in fs:
        if f.endswith(".json"):
            p = os.path.join(d, f)
            raw[os.path.relpath(p, root)[:-5]] = json.load(open(p))["values"]
cache = {}
def flat(t):
    if t in cache: return cache[t]
    cache[t] = set()
    out = set()
    for v in raw.get(t, []):
        v = v["id"] if isinstance(v, dict) else v
        if v.startswith("#"):
            out |= flat(v[1:].removeprefix("minecraft:"))
        else:
            out.add(v.removeprefix("minecraft:"))
    cache[t] = out
    return out
with open("crates/client/data/item_tags.txt", "w") as o:
    for t in sorted(raw):
        o.write(t + " " + " ".join(sorted(flat(t))) + "\n")
print(len(raw), "tags")
