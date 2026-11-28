"""Packet coverage for `version_smoke.py`: server commands that make the server send as many
clientbound packet types as possible, and a report of what arrived and what the client dropped.

The commands are written in the syntax of the target version, without `smoke_legacy` translation.
A command the version does not know only prints a server error.
"""
import glob
import json
import os
import re


def commands(ver, bot, x, y, z):
    """Console commands that together trigger most clientbound packets of `ver`."""
    old = ver < (1, 13)

    def name(new, flat_1_11, legacy):
        return new if ver >= (1, 13) else flat_1_11 if ver >= (1, 11) else legacy

    item = name("item", "item", "Item")
    xp_orb = name("experience_orb", "xp_orb", "XPOrb")
    tnt = name("tnt", "tnt", "PrimedTnt")
    bolt = name("lightning_bolt", "lightning_bolt", "LightningBolt")
    pig = name("pig", "pig", "Pig")
    villager = name("villager", "villager", "Villager")
    stone = "stone" if old else "minecraft:stone"
    out = [
        f"tp {bot} {x} {y} {z}",
        f"gamemode {'0' if ver < (1, 8) else 'survival'} {bot}",
        f"say smoke-packets",
        "difficulty easy" if ver >= (1, 8) else "difficulty 1",
        "difficulty peaceful" if ver >= (1, 8) else "difficulty 0",
        "difficulty normal" if ver >= (1, 8) else "difficulty 2",
        f"spawnpoint {bot} {int(x)} {y} {int(z)}",
        f"setworldspawn {int(x)} {y} {int(z)}",
        f"effect {'give ' if not old else ''}{bot} {'1' if ver < (1, 9) else 'speed'} 5 1",
        f"enchant {bot} {'sharpness' if not old else '16'} 1",
        f"summon {pig} {x} {y} {z + 3}",
        f"summon {villager} {x + 3} {y} {z}",
        f"summon {xp_orb} {x} {y + 1} {z}",
        # high enough to explode in the air: a crater would stay in the world for the next run
        f"summon {tnt} {x} {y + 200} {z + 10}",
        f"summon {bolt} {x + 12} {y} {z + 12}",
        f"give {bot} {stone} 5",
        f"give {bot} {'filled_map' if not old else 'map'} 1",
        "scoreboard objectives add cov dummy",
        f"scoreboard players set {bot} cov 3",
        "scoreboard objectives setdisplay sidebar cov",
        "scoreboard objectives setdisplay list cov",
        f"scoreboard players reset {bot} cov",
        "scoreboard objectives remove cov",
    ]
    if old:
        out += ["scoreboard teams add covt", f"scoreboard teams join covt {bot}",
                "scoreboard teams option covt color red", "scoreboard teams remove covt",
                f"xp 5 {bot}", f"xp 3L {bot}"]
    else:
        out += ["team add covt", f"team join covt {bot}", "team modify covt color red",
                "team remove covt", f"xp add {bot} 5", f"xp add {bot} 3 levels"]
    if ver >= (1, 7):
        out += [f"setblock {int(x) + 2} {y} {int(z) - 2} {'note_block' if not old else 'noteblock'}",
                f"setblock {int(x) + 2} {y - 1} {int(z) - 1} redstone_block"]
    if ver >= (1, 8):
        out += [
            f'title {bot} times 5 20 5', f'title {bot} subtitle {{"text":"sub"}}',
            f'title {bot} title {{"text":"cov"}}', f"title {bot} clear", f"title {bot} reset",
            f'tellraw {bot} {{"text":"cov-tellraw"}}',
            f"particle flame {x} {y + 1} {z + 2} 0.2 0.2 0.2 0.01 20",
            "worldborder set 20000", "worldborder center 1000 1000", "worldborder warning distance 3",
            "worldborder warning time 3", "worldborder set 60000000 1",
            f"clear {bot} {stone}",
        ]
    if ver >= (1, 9):
        sound = "entity.experience_orb.pickup" if ver >= (1, 10) else "entity.experience_orb.touch"
        out += [f"playsound {sound} master {bot} {x} {y} {z}", f"stopsound {bot}",
                f"summon {pig} {x - 3} {y} {z} {{Passengers:[{{id:\"{pig}\"}}]}}"]
    elif ver >= (1, 8):
        out += [f"playsound random.orb {bot} {x} {y} {z}"]
    if ver >= (1, 11):
        out += [f'title {bot} actionbar {{"text":"bar"}}']
    if ver >= (1, 12):
        out += [f"advancement grant {bot} everything", f"recipe give {bot} *",
                f"recipe take {bot} *", f"advancement revoke {bot} everything"]
    if ver >= (1, 13):
        out += ['bossbar add cov "cov"', f"bossbar set cov players {bot}", "bossbar set cov value 5",
                "bossbar set cov color red", "bossbar remove cov",
                f"setblock {int(x) - 2} {y} {int(z) - 2} oak_sign",
                f"data merge block {int(x) - 2} {y} {int(z) - 2} {{Text1:'\"hi\"'}}"
                if ver < (1, 20) else
                f"data merge block {int(x) - 2} {y} {int(z) - 2} {{is_waxed:1b}}"]
    if ver >= (1, 16):
        out += [f"attribute {bot} minecraft:generic.movement_speed base set 0.11",
                f"attribute {bot} minecraft:generic.movement_speed base set 0.1"]
    if ver >= (1, 19, 4):
        out += [f"damage {bot} 1 minecraft:generic", f"ride {bot} mount @e[type=pig,limit=1]",
                f"ride {bot} dismount"]
    if ver >= (1, 20, 3):
        out += ["tick rate 21", "tick rate 20", "tick freeze", "tick step 2", "tick unfreeze"]
    out += [f"op {bot}", f"deop {bot}", f"effect {bot} clear" if old else f"effect clear {bot}",
            f"gamemode {'3' if ver >= (1, 8) else '1'} {bot}",
            f"gamemode {'0' if ver < (1, 8) else 'survival'} {bot}",
            f"summon {item} {x} {y + 1} {z}" + (
                f' {{Item:{{id:"minecraft:stone",Count:1b}}}}' if ver >= (1, 8) else "")]
    return out


def cleanup(ver):
    """Removes dropped items: they persist in the world and end up in the next run's inventory."""
    if ver < (1, 8):
        return []
    return [f"kill @e[type={'item' if ver >= (1, 11) else 'Item'}]"]


def table_names(root, ver_name):
    """Clientbound play packet names of the version table, if it has one."""
    path = os.path.join(root, "crates/protocol/src/generated", f"v{ver_name.replace('.', '_')}.rs")
    if not os.path.exists(path):
        return None
    with open(path) as f:
        return set(re.findall(
            r'state: State::Play,\s*dir: Direction::Clientbound,\s*id: \d+,\s*name: "(\w+)"', f.read()))


def client_literals(root):
    """String literals outside the version translators: a handled packet name appears there."""
    out = set()
    for crate in ("client", "net", "world"):
        for path in glob.glob(os.path.join(root, "crates", crate, "src/**/*.rs"), recursive=True):
            if crate == "net" and re.match(r"(v\d+\w*|legacy)\.rs$", os.path.basename(path)):
                continue
            with open(path, errors="replace") as f:
                out |= set(re.findall(r'"([a-z0-9_]+)"', f.read()))
    return out


def report(root, ver_name, client_log, out_path):
    """Writes the coverage report of one client log; returns a one-line summary."""
    received, dropped, handled = {}, {}, {}
    line_re = re.compile(r'(recv|in) play Some\("(\w+)"\)(.*)')
    with open(client_log, errors="replace") as f:
        for line in f:
            m = line_re.search(line)
            if not m:
                continue
            kind, name, rest = m.groups()
            bucket = received if kind == "recv" else dropped if "отброшен" in rest else handled
            bucket[name] = bucket.get(name, 0) + 1
    table = table_names(root, ver_name)
    literals = client_literals(root)
    result = {
        "received": received,
        "dropped_in_translation": dropped,
        "not_handled_by_client": sorted(n for n in handled if n not in literals),
        "never_sent": sorted(table - set(received)) if table else None,
    }
    with open(out_path, "w") as f:
        json.dump(result, f, indent=1, sort_keys=True)
    total = f"/{len(table)}" if table else ""
    return (f"packets: received {len(received)}{total} types, dropped {sorted(dropped)}, "
            f"unhandled {result['not_handled_by_client']}")
