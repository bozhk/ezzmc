#!/usr/bin/env python3
"""Таблицы 1.12.2 из `reference/`: звуки (имена 1.13.2 по id 1.12.2) и числовые типы мобов.

Пишет crates/protocol/src/generated/sounds_1_12_2.rs и crates/net/src/v1122_gen.rs.
Запуск из корня: python3 tools/gen_v1122.py
"""
import re
from pathlib import Path

OLD = Path("reference/1.12.2/client/net/minecraft")
# переименования Flattening, по порядку
SOUND_RULES = [
    (r"^record\.", "music_disc."),
    (r"^block\.cloth\.", "block.wool."), (r"^block\.note\.", "block.note_block."),
    (r"^block\.slime\.", "block.slime_block."), (r"enderchest", "ender_chest"),
    (r"pressureplate", "pressure_plate"), (r"waterlily", "lily_pad"), (r"\.wood_", ".wooden_"),
    (r"\.armorstand\.", ".armor_stand."), (r"\.bobber\.", ".fishing_bobber."),
    (r"^entity\.enderdragon_fireball\.", "entity.dragon_fireball."), (r"enderdragon", "ender_dragon"),
    (r"\.endereye\.", ".ender_eye."), (r"\.endermen\.", ".enderman."), (r"\.enderpearl\.", ".ender_pearl."),
    (r"evocation_fangs", "evoker_fangs"), (r"evocation_illager", "evoker"),
    (r"^entity\.firework\.", "entity.firework_rocket."), (r"illusion_illager", "illusioner"),
    (r"irongolem", "iron_golem"), (r"\.itemframe\.", ".item_frame."), (r"leashknot", "leash_knot"),
    (r"\.lightning\.", ".lightning_bolt."), (r"lingeringpotion", "lingering_potion"),
    (r"^entity\.small_magmacube\.(\w+)", r"entity.magma_cube.\1_small"),
    (r"^entity\.small_slime\.(\w+)", r"entity.slime.\1_small"), (r"magmacube", "magma_cube"),
    (r"polar_bear\.baby_ambient", "polar_bear.ambient_baby"), (r"\.snowman\.", ".snow_golem."),
    (r"villager\.trading", "villager.trade"), (r"vindication_illager", "vindicator"),
    (r"attack_door_wood", "attack_wooden_door"), (r"break_door_wood", "break_wooden_door"),
    (r"\.zombie_pig\.", ".zombie_pigman."),
]
ENTITY_RENAME = {
    "xp_orb": "experience_orb", "xp_bottle": "experience_bottle", "eye_of_ender_signal": "eye_of_ender",
    "ender_crystal": "end_crystal", "evocation_fangs": "evoker_fangs", "evocation_illager": "evoker",
    "vindication_illager": "vindicator", "illusion_illager": "illusioner", "villager_golem": "iron_golem",
    "snowman": "snow_golem", "commandblock_minecart": "command_block_minecart",
    "fireworks_rocket": "firework_rocket",
}


def entities():
    src = (OLD / "entity/EntityType.java").read_text()
    found = re.findall(r'register\((\d+), "(\w+)", (\w+)\.class', src)
    assert found
    return [(int(i), ENTITY_RENAME.get(n, n), c) for i, n, c in found]


def main():
    sounds = re.findall(r'register\("([\w.]+)"\)', (OLD / "sound/Sound.java").read_text())
    new = Path("crates/protocol/src/generated/r1_13_2.rs").read_text()
    new = re.search(r"SOUND_EVENT[^=]*=\s*&\[(.*?)\];", new, re.S)[1]
    new = {n.removeprefix("minecraft:") for n in re.findall(r'"([^"]+)"', new)}
    renamed = []
    for old in sounds:
        name = old
        for a, b in SOUND_RULES:
            name = re.sub(a, b, name)
        assert name in new, (old, name)
        renamed.append((old, name))
    out = ["//! Сгенерировано `tools/gen_v1122.py` из `reference/`. Не править руками.", "",
           "/// Звуки 1.12.2 по числовому id, имена 1.13.2.",
           "pub const SOUND_EVENT: &[&str] = &["]
    out += [f'    "{n}",' for _, n in renamed] + ["];", ""]
    Path("crates/protocol/src/generated/sounds_1_12_2.rs").write_text("\n".join(out))

    out = ["//! Сгенерировано `tools/gen_v1122.py` из `reference/`. Не править руками.", "",
           "/// (числовой тип моба 1.12.2, имя 1.13.2), по возрастанию id.",
           "pub const ENTITIES: &[(i32, &str)] = &["]
    out += [f'    ({i}, "{n}"),' for i, n, _ in sorted(entities())] + ["];", ""]
    out += ["/// Звуки, переименованные в 1.13: (имя 1.12.2, имя 1.13.2).",
            "pub const SOUND_RENAMES: &[(&str, &str)] = &["]
    out += [f'    ("{a}", "{b}"),' for a, b in sorted(renamed) if a != b] + ["];", ""]
    Path("crates/net/src/v1122_gen.rs").write_text("\n".join(out))
    print("sounds", len(renamed), "entities", len(entities()))


if __name__ == "__main__":
    main()
