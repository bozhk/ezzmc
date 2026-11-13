#!/usr/bin/env python3
"""Таблица id пакетов 1.20.1 (protocol 763) из ConnectionProtocol.java (Mojang-имена) → crates/protocol/src/generated/v1_20_1.rs.
Использование: tools/gen_packets_1_20_1.py <путь к ConnectionProtocol.java>. packets.json у 1.20.1 нет."""
import re, sys
src = open(sys.argv[1]).read()
RENAME = {"game_profile": "login_finished", "client_intention": "intention"}
def snake(c):
    c = re.sub(r'^(Clientbound|Serverbound)', '', c)
    c = re.sub(r'Packet$', '', c)
    c = c.replace('.', '')
    s = re.sub(r'(?<!^)(?=[A-Z])', '_', c).lower().replace('_packet_', '_')
    return RENAME.get(s, s)
out = ['// @generated tools/gen_packets_1_20_1.py из ConnectionProtocol.java (1.20.1). НЕ РЕДАКТИРОВАТЬ.',
       'use crate::{Direction, PacketInfo, State};', '', 'pub const VERSION: &str = "1.20.1";', '',
       '/// Отсортировано по (состояние, направление, id).', 'pub static PACKETS: &[PacketInfo] = &[']
rows = []
states = {'HANDSHAKING': 'Handshake', 'PLAY': 'Play', 'STATUS': 'Status', 'LOGIN': 'Login'}
for line in src.splitlines():
    m = re.match(r'\s+(HANDSHAKING|PLAY|STATUS|LOGIN)\(', line)
    if not m: continue
    st = states[m.group(1)]
    for fm in re.finditer(r'addFlow\(PacketFlow\.(CLIENTBOUND|SERVERBOUND), (.*?)\)(?=\.addFlow|\)\)?[,;]$|\)\)\),?$)', line):
        pass
    # режем по addFlow
    parts = re.split(r'addFlow\(PacketFlow\.', line)[1:]
    for p in parts:
        d = 'Clientbound' if p.startswith('CLIENTBOUND') else 'Serverbound'
        names = re.findall(r'add(?:Packet|BundlePacket)\(([\w.]+)\.class|withBundlePacket\(([\w.]+)\.class', p)
        ids = [a or b for a, b in names]
        # bundle: id 0 для PLAY clientbound — withBundlePacket идёт первым
        for i, c in enumerate(ids):
            rows.append((st, d, i, snake(c) if c != 'ClientboundBundlePacket' else 'bundle_delimiter'))
order={'Handshake':0,'Status':1,'Login':2,'Play':4}
rows.sort(key=lambda r:(order[r[0]],r[1]!='Clientbound' and 1 or 0,r[2]))
for st, d, i, n in rows:
    out.append(f'    PacketInfo {{ state: State::{st}, dir: Direction::{d}, id: {i}, name: "{n}" }},')
out.append('];')
open('crates/protocol/src/generated/v1_20_1.rs', 'w').write('\n'.join(out) + '\n')
print(len(rows), 'пакетов')
