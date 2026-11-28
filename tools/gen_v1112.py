#!/usr/bin/env python3
"""Звуки версии 1.9–1.11.2 по числовому id (имена 1.13.2) из `reference/<ver>`.

Пишет crates/protocol/src/generated/sounds_<ver>.rs.
Запуск из корня: python3 tools/gen_v1112.py [1.11.2|1.10.2|...]
"""
import re
import sys
from pathlib import Path

from gen_v1122 import SOUND_RULES

VER = sys.argv[1] if len(sys.argv) > 1 else "1.11.2"
SRC = Path(f"reference/{VER}/client/net/minecraft/sound/Sound.java")


def packets_1_9_2():
    """Таблица 1.9–1.9.2 = 1.10.2 + clientbound `update_sign` на 0x46 (VANILLA: NetworkState 1.9.2,
    jar без Yarn; убран в 1.9.4), clientbound id от 0x46 и выше сдвинуты на +1."""
    src = Path("crates/protocol/src/generated/v1_10_2.rs").read_text()
    src = src.replace('VERSION: &str = "1.10.2"', 'VERSION: &str = "1.9.2"')
    pat = re.compile(r"(state: State::Play,\s*dir: Direction::Clientbound,\s*id: )(\d+)(,\s*name: \"(\w+)\",\s*\},)")
    def shift(m):
        i = int(m[2])
        out = f"{m[1]}{i + 1 if i >= 0x46 else i}{m[3]}"
        if i == 0x46:
            out = (f'{m[1]}{0x46},\n        name: "update_sign",\n    }},\n    PacketInfo {{\n'
                   f"        state: State::Play,\n        dir: Direction::Clientbound,\n        id: {i + 1}{m[3]}")
        return out
    src = pat.sub(shift, src)
    src = re.sub(r"^// @generated.*", "// @generated tools/gen_v1112.py 1.9.2 из v1_10_2.rs. НЕ РЕДАКТИРОВАТЬ.", src, count=1, flags=re.M)
    Path("crates/protocol/src/generated/v1_9_2.rs").write_text(src)


# VANILLA: NetworkState (1.8.9) — порядок регистрации Play; имена — как у тех же пакетов 1.9.
# Пакеты без пары в 1.9 — с суффиксом `_1_8` (переводит `net::v18`).
CB_1_8 = """keep_alive login chat set_time set_equipped_item set_spawn_position set_health respawn
player_position set_carried_item player_sleep animate add_player take_item_entity add_entity add_mob
add_painting add_experience_orb set_entity_motion remove_entities move_entity move_entity_pos
move_entity_rot move_entity_pos_rot teleport_entity rotate_head entity_event set_entity_link_1_8
set_entity_data update_mob_effect remove_mob_effect set_experience update_attributes level_chunk_1_8
chunk_blocks_update block_update block_event block_destruction level_chunk_bulk_1_8 explode level_event
custom_sound level_particles game_event add_global_entity open_screen container_close container_set_slot
container_set_content container_set_data container_ack update_sign map_item_data block_entity_data
open_sign_editor award_stats player_info player_abilities command_suggestions set_objective set_score
set_display_objective set_player_team custom_payload disconnect change_difficulty player_combat set_camera
set_border set_titles set_compression_1_8 tab_list resource_pack update_entity_nbt_1_8""".split()
SB_1_8 = """keep_alive chat interact move_player move_player_pos move_player_rot move_player_pos_rot player_action
use_item_on set_carried_item swing player_command player_input container_close container_click container_ack
set_creative_mode_slot container_button_click sign_update player_abilities command_suggestion
client_information client_command custom_payload teleport_to_entity resource_pack""".split()
# Пакеты 1.9, которых нет в 1.8: id вне диапазона сервера. Clientbound — то, что собирает
# `v18::expand`; serverbound — то, что клиент шлёт, а `v18::to_1_8` переводит или отбрасывает.
CB_EXTRA_1_8 = ["level_chunk", "forget_level_chunk", "set_entity_link", "set_passengers"]
SB_EXTRA_1_8 = ["accept_teleportation", "use_item", "move_vehicle", "paddle_boat"]


def packets_1_8():
    src = Path("crates/protocol/src/generated/v1_9_2.rs").read_text()
    head, _, _ = src.partition("    PacketInfo {\n        state: State::Play")
    head = head.replace('VERSION: &str = "1.9.2"', 'VERSION: &str = "1.8.9"')
    head = re.sub(r"^// @generated.*", "// @generated tools/gen_v1112.py 1.8. НЕ РЕДАКТИРОВАТЬ.", head, count=1, flags=re.M)
    rows = []
    for d, names, extra in (("Clientbound", CB_1_8, CB_EXTRA_1_8), ("Serverbound", SB_1_8, SB_EXTRA_1_8)):
        assert len(names) == len(set(names))
        ids = list(enumerate(names)) + [(0x60 + i, n) for i, n in enumerate(extra)]
        for i, n in ids:
            rows.append(f"    PacketInfo {{\n        state: State::Play,\n        dir: Direction::{d},\n"
                        f"        id: {i},\n        name: \"{n}\",\n    }},\n")
    assert len(CB_1_8) == 0x4A and len(SB_1_8) == 0x1A
    Path("crates/protocol/src/generated/v1_8.rs").write_text(head + "".join(rows) + "];\n")


def main():
    if VER == "1.8":
        return packets_1_8()
    if VER == "1.9.2":
        return packets_1_9_2()
    if VER == "1.9":
        # VANILLA: SoundEvent#register (1.9, jar без Yarn) — как 1.9.4, но без item.elytra.flying.
        sounds = re.findall(r'register\("([\w.]+)"\)', Path("reference/1.9.4/client/net/minecraft/sound/Sound.java").read_text())
        sounds.remove("item.elytra.flying")
    else:
        sounds = re.findall(r'register\("([\w.]+)"\)', SRC.read_text())
    new = Path("crates/protocol/src/generated/r1_13_2.rs").read_text()
    new = re.search(r"SOUND_EVENT[^=]*=\s*&\[(.*?)\];", new, re.S)[1]
    new = {n.removeprefix("minecraft:") for n in re.findall(r'"([^"]+)"', new)}
    out = ["//! Сгенерировано `tools/gen_v1112.py` из `reference/`. Не править руками.", "",
           f"/// Звуки {VER} по числовому id, имена 1.13.2.",
           "pub const SOUND_EVENT: &[&str] = &["]
    for old in sounds:
        name = old
        for a, b in SOUND_RULES:
            name = re.sub(a, b, name)
        if name not in new:  # убран позже (1.10 entity.experience_orb.touch): без пары, звука нет
            print("нет в 1.13.2:", old)
        out.append(f'    "{name}",')
    Path(f"crates/protocol/src/generated/sounds_{VER.replace('.', '_')}.rs").write_text("\n".join(out + ["];", ""]))


if __name__ == "__main__":
    main()
