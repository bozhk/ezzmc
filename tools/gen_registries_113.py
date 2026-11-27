#!/usr/bin/env python3
"""reports/registries.json для 1.13.x: Data Generator 1.13 его не пишет.

Блоки и предметы — из reports самой версии; сущности, звуки, частицы, эффекты и чары — из кода 1.13.2
(у 1.13 и 1.13.1 нет Yarn-маппингов). Запуск: python3 tools/gen_registries_113.py <ver>…
"""
import json
import pathlib
import re
import sys

SRC = pathlib.Path('reference/1.13.2/client/net/minecraft')


def names(rel, pattern):
    found = re.findall(pattern, (SRC / rel).read_text())
    if not found:
        sys.exit(f'{rel}: ничего не найдено по {pattern}')
    return found


def registry(pairs):
    return {'entries': {f'minecraft:{n}': {'protocol_id': i} for i, n in pairs}}


code = {
    'entity_type': enumerate(names('entity/EntityType.java', r'= register\(\s*"(\w+)"')),
    'sound_event': enumerate(names('sound/Sound.java', r'register\("([\w.]+)"\)')),
    'particle_type': enumerate(names('client/particle/ParticleType.java', r'method_1998[34]\("(\w+)"')),
    'enchantment': enumerate(names('enchantment/Enchantment.java', r'register\("(\w+)", new ')),
    'mob_effect': [(int(i), n) for i, n in
                   names('entity/effect/StatusEffect.java', r'register\(\s*(\d+),\s*"(\w+)"')],
}
code = {k: registry(v) for k, v in code.items()}

for ver in sys.argv[1:]:
    reports = pathlib.Path('reference') / ver / 'reports'
    blocks = json.loads((reports / 'blocks.json').read_text())
    items = json.loads((reports / 'items.json').read_text())
    out = dict(code)
    by_state = sorted(blocks, key=lambda n: blocks[n]['states'][0]['id'])
    out['block'] = {'entries': {n: {'protocol_id': i} for i, n in enumerate(by_state)}}
    out['item'] = {'entries': items}
    out = {f'minecraft:{k}': v for k, v in sorted(out.items())}
    (reports / 'registries.json').write_text(json.dumps(out, indent=1))
    print(ver, {k.split(':')[1]: len(v['entries']) for k, v in out.items()})
