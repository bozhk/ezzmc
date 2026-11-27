#!/usr/bin/env python3
"""Таблица id пакетов для версий без reports/packets.json.
1.20.5/1.20.6: порядок addPacket в *Protocols.java, имена из create{Client,Server}bound("...") в *PacketTypes.java.
1.20.2–1.20.4 (`--legacy <ver-с-именами>`): порядок addPacket(Класс.class, ...) в ConnectionProtocol.java,
имя пакета — по классу из *Protocols.java версии-донора (обычно 1.20.6); нет класса у донора → snake_case класса
(печатается, проверить руками).
Использование: tools/gen_packets_builder.py <ver> [--legacy 1.20.6]  → crates/protocol/src/generated/v<ver>.rs"""
import re, sys, pathlib

STATES = [('handshake/HandshakeProtocols', 'Handshake'), ('status/StatusProtocols', 'Status'),
          ('login/LoginProtocols', 'Login'), ('configuration/ConfigurationProtocols', 'Configuration'),
          ('game/GameProtocols', 'Play')]


def builder_rows(ver):
    """[(state, dir, id, name, codec-класс)] из ProtocolInfoBuilder (1.20.5+)."""
    root = pathlib.Path('reference') / ver / 'client/net/minecraft/network/protocol'
    names = {}
    for f in root.glob('*/*PacketTypes.java'):
        cls = f.stem
        for m in re.finditer(r'(\w+)\s*=\s*create(?:Clientbound|Serverbound)\(\s*"(\w+)"\s*\)', f.read_text()):
            names[f'{cls}.{m.group(1)}'] = m.group(2)
    rows = []
    for path, st in STATES:
        src = (root / f'{path}.java').read_text()
        # каждая константа SERVERBOUND/CLIENTBOUND — отдельный вызов builder'а до ';'
        for m in re.finditer(r'(SERVERBOUND|CLIENTBOUND)\w*\s*=\s*ProtocolInfoBuilder\.(\w+)\((.*?)\);', src, re.S):
            d = 'Clientbound' if m.group(1) == 'CLIENTBOUND' else 'Serverbound'
            body = m.group(3)
            ids = []
            if 'withBundlePacket' in body:
                ids.append(('bundle_delimiter', 'ClientboundBundleDelimiterPacket'))
            for p in re.finditer(r'addPacket\((\w+)\.(\w+),\s*([\w.$]+)', body):
                key = f'{p.group(1)}.{p.group(2)}'
                if key not in names:
                    sys.exit(f'нет имени для {key}')
                ids.append((names[key], re.sub(r'\.(STREAM_CODEC|CODEC)$', '', p.group(3))))
            for i, (n, c) in enumerate(ids):
                rows.append((st, d, i, n, c))
    return rows


def snake(cls):
    c = re.sub(r'^(Clientbound|Serverbound|Client)', '', cls.replace('.', ''))
    c = re.sub(r'Packet', '', c)
    return re.sub(r'(?<!^)(?=[A-Z])', '_', c).lower()


def legacy_rows(ver, donor):
    """ConnectionProtocol.java (1.20.2–1.20.4): enum-состояния с addFlow(PacketFlow.X, PacketSet().addPacket(...))."""
    by_cls = {}
    for st, d, _, n, c in builder_rows(donor):
        by_cls.setdefault((st, d, c), n)
        by_cls.setdefault((None, d, c), n)
    src = (pathlib.Path('reference') / ver / 'client/net/minecraft/network/ConnectionProtocol.java').read_text()
    enum = {'HANDSHAKING': 'Handshake', 'PLAY': 'Play', 'STATUS': 'Status', 'LOGIN': 'Login',
            'CONFIGURATION': 'Configuration'}
    heads = list(re.finditer(r'^   (HANDSHAKING|PLAY|STATUS|LOGIN|CONFIGURATION)\(', src, re.M))
    rows = []
    for k, h in enumerate(heads):
        end = heads[k + 1].start() if k + 1 < len(heads) else src.index('NOT_REGISTERED')
        st = enum[h.group(1)]
        part = src[h.start():end]
        flows = list(re.finditer(r'PacketFlow\.(CLIENTBOUND|SERVERBOUND)', part))
        for j, f in enumerate(flows):
            d = 'Clientbound' if f.group(1) == 'CLIENTBOUND' else 'Serverbound'
            body = part[f.end():flows[j + 1].start() if j + 1 < len(flows) else len(part)]
            ids = []
            if 'withBundlePacket' in body:
                ids.append('bundle_delimiter')
            for p in re.finditer(r'addPacket\(([\w.]+)\.class', body):
                c = p.group(1)
                n = by_cls.get((st, d, c)) or by_cls.get((None, d, c))
                if n is None:
                    n = snake(c)
                    print(f'  {st} {d} {c} → {n} (нет у донора)')
                ids.append(n)
            for i, n in enumerate(ids):
                rows.append((st, d, i, n))
    return rows


ver = sys.argv[1]
if '--legacy' in sys.argv:
    rows = legacy_rows(ver, sys.argv[sys.argv.index('--legacy') + 1])
    src_note = 'ConnectionProtocol.java'
else:
    rows = [r[:4] for r in builder_rows(ver)]
    src_note = '*Protocols.java/*PacketTypes.java'

order = {s: i for i, (_, s) in enumerate(STATES)}
rows.sort(key=lambda r: (order[r[0]], r[1] != 'Clientbound', r[2]))
mod = 'v' + ver.replace('.', '_')
out = [f'// @generated tools/gen_packets_builder.py {" ".join(sys.argv[1:])} из {src_note}. НЕ РЕДАКТИРОВАТЬ.',
       'use crate::{Direction, PacketInfo, State};', '', f'pub const VERSION: &str = "{ver}";', '',
       '/// Отсортировано по (состояние, направление, id).', 'pub static PACKETS: &[PacketInfo] = &[']
for st, d, i, n in rows:
    out.append(f'    PacketInfo {{ state: State::{st}, dir: Direction::{d}, id: {i}, name: "{n}" }},')
out.append('];')
pathlib.Path(f'crates/protocol/src/generated/{mod}.rs').write_text('\n'.join(out) + '\n')
print(len(rows), 'пакетов')
