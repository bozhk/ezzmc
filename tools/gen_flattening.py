#!/usr/bin/env python3
"""Таблицы «Flattening» для протоколов ≤1.12.2: старые числовые блоки и предметы → id 1.13.2.

Источники: `BlockStateFlattening` и `ItemInstanceTheFlatteningFix` из кода 1.13.2 (Yarn не назвал их:
class_4500 и class_3389), числовые id — из `Block.java`/`Item.java` 1.12.2.
Запуск: python3 tools/gen_flattening.py
"""
import json
import pathlib
import re
import sys

REF = pathlib.Path('reference')
OLD = REF / '1.12.2/client/net/minecraft'
NEW = REF / '1.13.2/client/net/minecraft'
OUT = pathlib.Path('crates/world/src/generated/flattening.rs')


def renames():
    """Переименования, сделанные фиксерами после самого Flattening (`DataFixerFactory`)."""
    pairs = {}
    for f in sorted(NEW.glob('class_*.java')):
        pairs.update(re.findall(r'\.put\("minecraft:(\w+)", "minecraft:(\w+)"\)', f.read_text()))
    return pairs


def current(name, report, pairs):
    seen = set()
    while f'minecraft:{name}' not in report and name in pairs and name not in seen:
        seen.add(name)
        name = pairs[name]
    return name


def block_states():
    report = json.loads((REF / '1.13.2/reports/blocks.json').read_text())
    src = (NEW / 'class_4500.java').read_text()
    table, pairs = {}, renames()
    for m in re.finditer(r'method_21601\(\s*(\d+),\s*"(\{[^"]*\})"', src):
        old_id, tag = int(m[1]), m[2]
        name = re.search(r"Name:'([^']+)'", tag)[1]
        props = dict(re.findall(r"(\w+):'([^']*)'", tag.partition('Properties:')[2]))
        # Головы (`%%FILTER_ME%%`): тип и поворот знает только блок-энтити — берётся череп скелета.
        if 'FILTER_ME' in name:
            facing = props.pop('facing')
            props.pop('nodrop')
            if facing in ('up', 'down'):
                name, props = 'minecraft:skeleton_skull', {}
            else:
                name, props = 'minecraft:skeleton_wall_skull', {'facing': facing}
        # VANILLA: фиксер «Rename melon_block» — блок `melon_block` стал `melon`.
        name = name.removeprefix('minecraft:').replace('melon_block', 'melon')
        name = 'minecraft:' + current(name, report, pairs)
        states = report[name]['states']
        default = next(s for s in states if s.get('default')).get('properties', {})
        # VANILLA: LeavesFix — `decayable` стал `persistent`, расстояние считается по миру.
        if 'decayable' in props:
            props.pop('check_decay', None)
            props['persistent'] = 'false' if props.pop('decayable') == 'true' else 'true'
        unknown = set(props) - set(default)
        if unknown:
            sys.exit(f'{old_id} {name}: нет свойств {unknown}')
        want = {**default, **props}
        table[old_id] = next(s['id'] for s in states if s.get('properties', {}) == want)
    # Мета без записи — состояние меты 0 того же блока.
    return [table.get(i, table.get(i & ~15, 0)) for i in range(4096)], len(table)


def legacy_names():
    """(имя реестра ≤1.12.2, числовой id) блоков и предметов — для статистики 1.8–1.12.2."""
    pairs = {('air', 0)}
    for f, d in (('block/Block.java', OLD), ('item/Item.java', OLD)):
        pairs.update((n, int(i)) for i, n in
                     re.findall(r'\bregister\((\d+), "(\w+)"', (d / f).read_text()))
    return sorted(pairs)


def numeric_ids():
    blocks = {int(i): n for i, n in
              re.findall(r'\bregister\((\d+), "(\w+)"', (OLD / 'block/Block.java').read_text())}
    blocks[0] = 'air'
    items_src = (OLD / 'item/Item.java').read_text()
    items = {int(i): n for i, n in re.findall(r'\bregister\((\d+), "(\w+)"', items_src)}
    return {**blocks, **items}


def items():
    report = json.loads((REF / '1.13.2/reports/items.json').read_text())
    src = (NEW / 'class_3389.java').read_text()
    variants = dict(re.findall(r'put\("minecraft:([\w.]+)", "minecraft:(\w+)"\)', src))
    flat = {k.rsplit('.', 1)[0] for k in variants}
    pairs = renames()
    # VANILLA: фиксер «Rename melon_block/melon/speckled_melon» — меняются разом, один раз.
    melons = {'melon_block': 'melon', 'melon': 'melon_slice',
              'speckled_melon': 'glistering_melon_slice'}

    def item_id(new):
        new = melons.get(new, new)
        entry = report.get(f'minecraft:{current(new, report_names, pairs)}')
        return None if entry is None else entry['protocol_id']

    report_names = report
    out, missing = [], []
    for old_id, name in sorted(numeric_ids().items()):
        if name in flat:
            found = sorted((int(k.rsplit('.', 1)[1]), v) for k, v in variants.items()
                           if k.rsplit('.', 1)[0] == name)
        else:
            found = [(0, name)]
        for damage, new in found:
            new_id = item_id(new)
            if new_id is None:
                missing.append((old_id, damage, new))
            else:
                out.append((old_id, damage, new_id))
    return out, missing


def main():
    blocks, mapped = block_states()
    stacks, missing = items()
    text = ['//! Сгенерировано `tools/gen_flattening.py` — не править руками.', '',
            '/// `id << 4 | meta` (≤1.12.2) → id состояния блока 1.13.2.',
            '#[rustfmt::skip]', 'pub const BLOCKS: [u16; 4096] = [']
    for i in range(0, 4096, 16):
        text.append('    ' + ' '.join(f'{s},' for s in blocks[i:i + 16]))
    text += ['];', '',
             '/// (числовой id, damage) ≤1.12.2 → id предмета 1.13.2; отсортировано. Без записи для',
             '/// damage берётся запись damage 0 (там damage — износ или данные).',
             '#[rustfmt::skip]', 'pub const ITEMS: &[(u16, u16, u16)] = &[']
    text += [f'    ({i}, {d}, {n}),' for i, d, n in stacks]
    text += ['];', '',
             '/// (имя реестра, числовой id) блоков и предметов ≤1.12.2; отсортировано по имени.',
             '#[rustfmt::skip]', 'pub const NAMES: &[(&str, u16)] = &[']
    text += [f'    ("{n}", {i}),' for n, i in legacy_names()]
    text += ['];', '']
    OUT.write_text('\n'.join(text))
    print(f'блоки: {mapped} записей; предметы: {len(stacks)}; без пары: {missing}')


main()
