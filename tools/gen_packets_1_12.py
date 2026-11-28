#!/usr/bin/env python3
"""Таблицы пакетов 1.12 (335) и 1.12.1 (338) из таблицы 1.12.2 (340).

Маппингов этих версий нет; порядок регистрации и вызовы буфера в read/write сверены по
байткоду клиентских jar (javap): 335 → 338 добавил serverbound 0x12 (запрос рецепта) и
clientbound 0x2B (призрак рецепта) и убрал serverbound 0x01 (подготовка сетки крафта);
338 → 340 сменил только тело keep_alive (varint → long).

    python3 tools/gen_packets_1_12.py
"""
import pathlib
import re

DIR = pathlib.Path(__file__).resolve().parent.parent / "crates/protocol/src/generated"
ENTRY = re.compile(
    r"    PacketInfo \{\n        state: State::(\w+),\n        dir: Direction::(\w+),\n"
    r"        id: (\d+),\n        name: \"(\w+)\",\n    \},\n")


def to_1_12(rows):
    out = []
    for state, side, n, name in rows:
        if state == "Play" and side == "Clientbound":
            if name == "place_ghost_recipe":
                continue
            n -= n > 0x2B
        if state == "Play" and side == "Serverbound":
            if name == "place_recipe":
                continue
            n += 1 <= n < 0x12
        out.append((state, side, n, name))
    out.append(("Play", "Serverbound", 1, "prepare_crafting_grid"))
    order = ["Handshake", "Status", "Login", "Configuration", "Play"]
    return sorted(out, key=lambda r: (order.index(r[0]), r[1] != "Clientbound", r[2]))


def main():
    src = (DIR / "v1_12_2.rs").read_text()
    rows = [(s, d, int(n), name) for s, d, n, name in ENTRY.findall(src)]
    head = src[: src.index("    PacketInfo {")]
    tail = src[list(ENTRY.finditer(src))[-1].end():]
    # 338 отличается от 340 только телом keep_alive.
    for version, table in (("1.12.1", rows), ("1.12", to_1_12(rows))):
        text = head.replace(head.split("\n")[0],
                            f"// @generated tools/gen_packets_1_12.py ({version}). НЕ РЕДАКТИРОВАТЬ.")
        text = text.replace('"1.12.2"', f'"{version}"')
        for state, side, n, name in table:
            text += (f"    PacketInfo {{\n        state: State::{state},\n"
                     f"        dir: Direction::{side},\n        id: {n},\n"
                     f"        name: \"{name}\",\n    }},\n")
        (DIR / f"v{version.replace('.', '_')}.rs").write_text(text + tail)


if __name__ == "__main__":
    main()
