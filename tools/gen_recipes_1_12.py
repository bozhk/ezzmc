#!/usr/bin/env python3
"""Рецепты крафта 1.12.x → crates/net/data/recipes_1_12.txt.

Сервер 1.12 не присылает рецепты, только числовые id. VANILLA: RecipeManager (1.12.2): сначала
11 особых рецептов, затем файлы assets/minecraft/recipes; порядок обхода jar обратен алфавитному
(проверено на живом сервере 1.12.2 под Java 8 и 21). Рецепты 1.12, 1.12.1 и 1.12.2 совпадают.

Строка файла (по порядку id): `s группа ширина высота результат ячейки…` или
`l группа результат ингредиенты…`; результат — `id:damage:count`, ячейка — `id:damage,…` или `-`.

Запуск: python3 tools/gen_recipes_1_12.py (нужен `cargo xtask fetch-reference 1.12.2`).
"""
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REF = os.path.join(ROOT, "reference/1.12.2/client")


def item_ids():
    """Имя → числовой id: блоки, поверх них предметы (cake, bed и т. п. — предметы)."""
    ids = {}
    for path in ("net/minecraft/block/Block.java", "net/minecraft/item/Item.java"):
        with open(os.path.join(REF, path), encoding="utf-8") as f:
            for i, name in re.findall(r'\bregister\(\s*(\d+),\s*"([a-z0-9_]+)"', f.read()):
                ids[name] = int(i)
    return ids


IDS = item_ids()


def ref(entry):
    data = entry.get("data", 0)
    return f"{IDS[entry['item'].removeprefix('minecraft:')]}:{0 if data == 32767 else data}"


def ingredient(value):
    return ",".join(ref(e) for e in (value if isinstance(value, list) else [value]))


def shrink(pattern):
    """VANILLA: ShapedRecipe.Serializer#shrink — пустые крайние строки и столбцы отрезаются."""
    rows = [r for r in pattern]
    while rows and not rows[0].strip():
        rows.pop(0)
    while rows and not rows[-1].strip():
        rows.pop()
    left = min(len(r) - len(r.lstrip()) for r in rows)
    right = max(len(r.rstrip()) for r in rows)
    return [r[left:right] for r in rows]


def line(recipe):
    group = recipe.get("group", "").removeprefix("minecraft:") or "-"
    out = recipe["result"]
    result = f"{ref(out)}:{out.get('count', 1)}"
    if recipe["type"] == "crafting_shaped":
        rows = shrink(recipe["pattern"])
        cells = [ingredient(recipe["key"][c]) if c != " " else "-" for r in rows for c in r]
        return " ".join(["s", group, str(len(rows[0])), str(len(rows)), result, *cells])
    assert recipe["type"] == "crafting_shapeless", recipe["type"]
    return " ".join(["l", group, result, *map(ingredient, recipe["ingredients"])])


def main():
    folder = os.path.join(REF, "assets/minecraft/recipes")
    lines = []
    for name in sorted((f[:-5] for f in os.listdir(folder)), reverse=True):
        with open(os.path.join(folder, name + ".json"), encoding="utf-8") as f:
            lines.append(line(json.load(f)))
    with open(os.path.join(ROOT, "crates/net/data/recipes_1_12.txt"), "w") as f:
        f.write("\n".join(lines) + "\n")
    print(len(lines), "recipes")


main()
