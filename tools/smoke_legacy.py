"""1.12.x command syntax for tools/version_smoke.py (checks are written for 1.13+).

1.7.x (`ver < (1, 8)`): no `fill`, `execute`, `@e`, `blockdata`, spectator, item names in NBT,
console `kill`/`testfor`; effects by numeric id; `tp` without rotation. `command` returns a list
of commands then; inventories are read from the saved player file (`player_items`).
"""
import gzip
import hashlib
import os
import re
import struct
import uuid

OLD_NAMES = {"oak_log": "log", "stone_bricks": "stonebrick"}
MODES = {"survival": "0", "creative": "1", "spectator": "3"}
# Before 1.11 entity ids are the EntityList names (EntityType#registerEntity, 1.10.2).
ENTITIES_1_10 = {"pig": "Pig", "armor_stand": "ArmorStand", "item": "Item", "player": "Player",
                 "villager": "Villager"}
# 1.7: numeric ids (Item#getRawId / Block#getId, 1.7.10) for NBT, and effect ids (StatusEffect).
IDS_1_7 = {"air": 0, "stone": 1, "cobblestone": 4, "oak_log": 17, "glass": 20, "gold_block": 41,
           "torch": 50, "chest": 54, "diamond_block": 57, "stone_bricks": 98, "diamond": 264,
           "wooden_sword": 268, "map": 395, "item_frame": 389, "painting": 321}
EFFECTS_1_7 = {"speed": 1, "instant_health": 6, "instant_damage": 7}


def command(c, ver=(1, 12, 2)):
    c = re.sub(r"^gamemode (\w+)", lambda m: f"gamemode {MODES[m.group(1)]}", c)
    c = re.sub(r"^execute in overworld run ", "", c)
    c = c.replace("type=!minecraft:player", "type=!player").replace("type=minecraft:", "type=")
    c = c.replace("distance=..", "r=")
    if ver < (1, 11):
        c = re.sub(r"^summon (\w+)", lambda m: f"summon {ENTITIES_1_10.get(m.group(1), m.group(1))}", c)
        c = re.sub(r"type=(!?)(\w+)", lambda m: f"type={m.group(1)}{ENTITIES_1_10.get(m.group(2), m.group(2))}", c)
    if ver < (1, 6):  # EffectCommand 1.5.2: no `clear`; duration 0 removes one effect
        c = re.sub(r"^effect clear (\w+)", r"effect \1 speed 0", c)
    c = re.sub(r"^effect clear (\w+)", r"effect \1 clear", c)
    c = re.sub(r"^effect give (\w+)", r"effect \1", c)
    c = re.sub(r"^(setblock \S+ \S+ \S+ \w+)(\{.*\})$", r"\1 0 replace \2", c)
    if ver < (1, 7):  # 1.6 `give` takes numeric item ids (GiveCommand, 1.6.4)
        c = re.sub(r"^(give \w+) (\w+)", lambda m: f"{m.group(1)} {IDS_1_7[m.group(2)]}", c)
    for new, old in OLD_NAMES.items():
        c = re.sub(rf"^(give \w+) {new}\b", rf"\1 {old}", c)
    if ver < (1, 8):
        return command_1_7(c)
    return c


def command_1_7(c):
    """List of 1.7.10 commands for a 1.8-style command (see the module doc)."""
    c = re.sub(r"^execute \S+ ~ ~ ~ ", "", c)
    if c.startswith(("worldborder ", "kill @e")):
        return []
    c = re.sub(r"^(tp \S+ \S+ \S+ \S+) \S+ \S+$", r"\1", c)
    c = re.sub(r"^gamemode 3\b", "gamemode 1", c)
    c = re.sub(r"^effect (\w+) (\w+)", lambda m: f"effect {m.group(1)} {EFFECTS_1_7.get(m.group(2), m.group(2))}", c)
    # KillCommand 1.7.10 only kills the sender: a strong instant damage instead.
    c = re.sub(r"^kill (\w+)$", r"effect \1 7 1 10", c)
    c = re.sub(r'id:"minecraft:(\w+)"', lambda m: f"id:{IDS_1_7[m.group(1)]}s", c)
    m = re.match(r"^fill (-?\d+) (-?\d+) (-?\d+) (-?\d+) (-?\d+) (-?\d+) (\w+)$", c)
    if not m:
        return [c]
    a = [int(v) for v in m.groups()[:6]]
    block = m.group(7)
    if block == "portal":
        # The frame was filled solid: clear the inside, then FireBlock#onBlockAdded lights it.
        return [f"setblock {x} {y} {a[2]} air" for y in range(a[1], a[4] + 1)
                for x in range(a[0], a[3] + 1)] + [f"setblock {a[0]} {a[1]} {a[2]} fire"]
    return [f"setblock {x} {y} {z} {block}"
            for y in range(a[1], a[4] + 1) for x in range(a[0], a[3] + 1) for z in range(a[2], a[5] + 1)]


def items_nbt(line):
    """{slot: (item, count)} of a 1.12 `blockdata` reply."""
    old = {v: k for k, v in OLD_NAMES.items()}
    out = {}
    for entry in re.findall(r"\{[^{}]*Slot:[^{}]*\}", line):
        slot = int(re.search(r"Slot:(\d+)b", entry).group(1))
        item = re.search(r'id:"minecraft:(\w+)"', entry).group(1)
        out[slot] = (old.get(item, item), int(re.search(r"Count:(\d+)b", entry).group(1)))
    return out


def has_items(name, expected):
    """`testfor` matching a player whose inventory holds every expected stack."""
    entries = ",".join(
        f'{{Slot:{slot}b,id:"minecraft:{OLD_NAMES.get(item, item)}",Count:{count}b}}'
        for slot, (item, count) in expected.items())
    return f"testfor @a[name={name}] {{Inventory:[{entries}]}}"


def chest_test(pos, expected):
    """1.7 `testforblock` of a chest holding exactly `expected` (lists compare whole, 1.7.10)."""
    entries = ",".join(
        f"{{Slot:{slot}b,id:{IDS_1_7[item]}s,Count:{count}b,Damage:0s}}"
        for slot, (item, count) in sorted(expected.items()))
    return f"testforblock {pos} minecraft:chest -1 {{Items:[{entries}]}}"


def offline_uuid(name):
    """UUID.nameUUIDFromBytes("OfflinePlayer:" + name)."""
    h = bytearray(hashlib.md5(f"OfflinePlayer:{name}".encode()).digest())
    h[6] = h[6] & 0x0F | 0x30
    h[8] = h[8] & 0x3F | 0x80
    return str(uuid.UUID(bytes=bytes(h)))


def _nbt(f, ty):
    if ty == 1:
        return struct.unpack(">b", f.read(1))[0]
    if ty == 2:
        return struct.unpack(">h", f.read(2))[0]
    if ty == 3:
        return struct.unpack(">i", f.read(4))[0]
    if ty == 4:
        return struct.unpack(">q", f.read(8))[0]
    if ty == 5:
        return struct.unpack(">f", f.read(4))[0]
    if ty == 6:
        return struct.unpack(">d", f.read(8))[0]
    if ty == 7:
        return f.read(struct.unpack(">i", f.read(4))[0])
    if ty == 8:
        return f.read(struct.unpack(">H", f.read(2))[0]).decode("utf-8", "replace")
    if ty == 9:
        et = f.read(1)[0]
        return [_nbt(f, et) for _ in range(struct.unpack(">i", f.read(4))[0])]
    if ty == 10:
        out = {}
        while (t := f.read(1)[0]) != 0:
            key = f.read(struct.unpack(">H", f.read(2))[0]).decode()
            out[key] = _nbt(f, t)
        return out
    if ty == 11:
        n = struct.unpack(">i", f.read(4))[0]
        return list(struct.unpack(f">{n}i", f.read(4 * n)))
    raise ValueError(f"NBT tag {ty}")


def player_items(world, name):
    """{slot: (item, count)} from `world/playerdata/<uuid>.dat` (1.6: `players/<name>.dat`)."""
    path = os.path.join(world, "playerdata", offline_uuid(name) + ".dat")
    if not os.path.exists(path):
        path = os.path.join(world, "players", name + ".dat")
    try:
        with gzip.open(path) as f:
            f.read(3)  # root tag type and empty name
            root = _nbt(f, 10)
    except (OSError, ValueError, IndexError, struct.error):
        return {}
    names = {v: k for k, v in IDS_1_7.items()}
    return {e["Slot"]: (names.get(e["id"], str(e["id"])), e["Count"])
            for e in root.get("Inventory", [])}


def chunk_nbt(world, x, z):
    """Level compound of the Anvil chunk holding block x/z (after `save-all`), or None."""
    cx, cz = x >> 4, z >> 4
    path = os.path.join(world, "region", f"r.{cx >> 5}.{cz >> 5}.mca")
    try:
        with open(path, "rb") as f:
            f.seek(4 * ((cx & 31) + (cz & 31) * 32))
            loc = struct.unpack(">I", f.read(4))[0]
            if loc == 0:
                return None
            f.seek((loc >> 8) * 4096)
            length, kind = struct.unpack(">iB", f.read(5))
            data = f.read(length - 1)
    except (OSError, struct.error):
        return None
    if kind != 2:
        return None
    import io
    import zlib
    f = io.BytesIO(zlib.decompress(data))
    f.read(3)
    return _nbt(f, 10).get("Level")


def block_id(world, x, y, z):
    """Numeric block id from the saved world (1.6 has no `testforblock`)."""
    level = chunk_nbt(world, x, z)
    if level is None:
        return None
    for sec in level.get("Sections", []):
        if sec["Y"] == y >> 4:
            i = (y & 15) * 256 + (z & 15) * 16 + (x & 15)
            return sec["Blocks"][i]
    return 0


def chest_items(world, x, y, z):
    """{slot: (item, count)} of a saved chest."""
    names = {v: k for k, v in IDS_1_7.items()}
    level = chunk_nbt(world, x, z) or {}
    for te in level.get("TileEntities", []):
        if (te.get("x"), te.get("y"), te.get("z")) == (x, y, z):
            return {e["Slot"]: (names.get(e["id"], str(e["id"])), e["Count"])
                    for e in te.get("Items", [])}
    return {}
