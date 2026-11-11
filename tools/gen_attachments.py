#!/usr/bin/env python3
"""Генерирует crates/world/src/entity_attach.rs из reference/26.3 EntityTypes.java.
VANILLA: EntityType.Builder#passengerAttachments/ridingOffset/vehicleAttachment (EntityAttachments)."""
import re, sys
src = open('reference/26.3/client/net/minecraft/world/entity/EntityTypes.java').read()
DEFAULT_VEH = (0.0, 0.6, 0.0)
blocks = re.split(r'public static final EntityType<[^>]*> \w+ = register\(', src)[1:]
pas, veh = {}, {}
def f(s): return float(s.replace('F','').replace('f','')) 
for b in blocks:
    m = re.match(r'\s*EntityTypeIds\.(\w+),', b)
    if not m: continue
    name = m.group(1).lower()
    end = b.find(');\n   public static')
    b = b if end < 0 else b[:end]
    pts = []
    for mm in re.finditer(r'\.passengerAttachments\(((?:[^()]|\([^()]*\))*)\)', b):
        args = mm.group(1)
        vecs = re.findall(r'new Vec3\(([^)]*)\)', args)
        if vecs:
            for v in vecs:
                x,y,z = [f(t.strip()) for t in v.split(',')]
                pts.append((x,y,z))
        else:
            for t in args.split(','):
                t = t.strip()
                if t: pts.append((0.0, f(t), 0.0))
    if pts: pas[name] = pts
    mm = re.search(r'\.ridingOffset\(([-\d.]+)F\)', b)
    if mm: veh[name] = (0.0, -f(mm.group(1)), 0.0)
    if 'vehicleAttachment(Avatar.DEFAULT_VEHICLE_ATTACHMENT)' in b: veh[name] = DEFAULT_VEH
    mm = re.search(r'\.vehicleAttachment\(new Vec3\(([^)]*)\)\)', b)
    if mm: veh[name] = tuple(f(t.strip()) for t in mm.group(1).split(','))
o = ['//! Точки крепления пассажиров/транспорта 26.3 (EntityType.Builder). Сгенерировано tools/gen_attachments.py.', '',
     '/// PASSENGER-точки типа (пусто → запасная (0, height, 0)).',
     'pub fn passenger_points(name: &str) -> &\'static [[f32; 3]] {', '    match name {']
for k,v in sorted(pas.items()):
    o.append(f'        "{k}" => &[' + ', '.join(f'[{x!r}, {y!r}, {z!r}]' for x,y,z in v) + '],')
o += ['        _ => &[],', '    }', '}', '', '/// VEHICLE-точка типа (нет → (0, 0, 0)).',
      'pub fn vehicle_point(name: &str) -> [f32; 3] {', '    match name {']
for k,v in sorted(veh.items()):
    o.append(f'        "{k}" => [{v[0]!r}, {v[1]!r}, {v[2]!r}],')
o += ['        _ => [0.0; 3],', '    }', '}', '']
open('crates/world/src/entity_attach.rs','w').write('\n'.join(o))
print(len(pas), len(veh))
