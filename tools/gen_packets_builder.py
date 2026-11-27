#!/usr/bin/env python3
"""Таблица id пакетов для версий с ProtocolInfoBuilder без reports/packets.json (1.20.5/1.20.6):
порядок addPacket в *Protocols.java, имена из create{Client,Server}bound("...") в *PacketTypes.java.
Использование: tools/gen_packets_builder.py <ver>  → crates/protocol/src/generated/v<ver>.rs"""
import re, sys, pathlib

ver = sys.argv[1]
root = pathlib.Path('reference') / ver / 'client/net/minecraft/network/protocol'
names = {}
for f in root.glob('*/*PacketTypes.java'):
    cls = f.stem
    for m in re.finditer(r'(\w+)\s*=\s*create(?:Clientbound|Serverbound)\(\s*"(\w+)"\s*\)', f.read_text()):
        names[f'{cls}.{m.group(1)}'] = m.group(2)

STATES = [('handshake/HandshakeProtocols', 'Handshake'), ('status/StatusProtocols', 'Status'),
          ('login/LoginProtocols', 'Login'), ('configuration/ConfigurationProtocols', 'Configuration'),
          ('game/GameProtocols', 'Play')]
rows = []
for path, st in STATES:
    src = (root / f'{path}.java').read_text()
    # каждая константа SERVERBOUND/CLIENTBOUND — отдельный вызов builder'а до ';'
    for m in re.finditer(r'(SERVERBOUND|CLIENTBOUND)\w*\s*=\s*ProtocolInfoBuilder\.(\w+)\((.*?)\);', src, re.S):
        d = 'Clientbound' if m.group(1) == 'CLIENTBOUND' else 'Serverbound'
        body = m.group(3)
        ids = []
        if 'withBundlePacket' in body:
            ids.append('bundle_delimiter')
        for p in re.finditer(r'addPacket\((\w+)\.(\w+),', body):
            key = f'{p.group(1)}.{p.group(2)}'
            if key not in names:
                sys.exit(f'нет имени для {key}')
            ids.append(names[key])
        for i, n in enumerate(ids):
            rows.append((st, d, i, n))

order = {s: i for i, (_, s) in enumerate(STATES)}
rows.sort(key=lambda r: (order[r[0]], r[1] != 'Clientbound', r[2]))
mod = 'v' + ver.replace('.', '_')
out = [f'// @generated tools/gen_packets_builder.py {ver} из *Protocols.java/*PacketTypes.java. НЕ РЕДАКТИРОВАТЬ.',
       'use crate::{Direction, PacketInfo, State};', '', f'pub const VERSION: &str = "{ver}";', '',
       '/// Отсортировано по (состояние, направление, id).', 'pub static PACKETS: &[PacketInfo] = &[']
for st, d, i, n in rows:
    out.append(f'    PacketInfo {{ state: State::{st}, dir: Direction::{d}, id: {i}, name: "{n}" }},')
out.append('];')
pathlib.Path(f'crates/protocol/src/generated/{mod}.rs').write_text('\n'.join(out) + '\n')
print(len(rows), 'пакетов')
