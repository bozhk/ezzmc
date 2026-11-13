#!/usr/bin/env python3
"""Генератор crates/render/src/keyframes_data.rs из Java-определений анимаций Mojang
(reference/26.3/.../animation/definitions/*.java). Запуск: python3 tools/gen_keyframes.py"""
import re, sys, math, pathlib
root = pathlib.Path(__file__).resolve().parent.parent
D = root / "reference/26.3/client/net/minecraft/client/animation/definitions"
FILES = ["Armadillo", "BabyArmadillo", "BabyAxolotl", "Bat", "Breeze", "Camel", "CopperGolem", "Creaking", "Frog", "Nautilus", "Rabbit", "Sniffer", "Warden"]
KF = re.compile(
    r"new Keyframe\(\s*(-?[\d.]+)F,\s*KeyframeAnimations\.(\w+)\(([^)]*)\),\s*"
    r"(?:KeyframeAnimations\.(\w+)\(([^)]*)\),\s*)?AnimationChannel\.Interpolations\.(\w+)\s*\)")
def num(s): return float(s.strip().rstrip("FfDd"))
def vec(kind, args):
    x, y, z = [num(a) for a in args.split(",")]
    if kind == "posVec": return (x, -y, z)
    if kind == "degreeVec": return tuple(v * math.pi / 180.0 for v in (x, y, z))
    return (x - 1.0, y - 1.0, z - 1.0)
def f(v): return repr(float(v))
out = ["// АВТОГЕНЕРАЦИЯ: python3 tools/gen_keyframes.py (VANILLA: animation/definitions/*Animation.java, 26.3)",
       "#![allow(clippy::all)]",
       "use crate::keyframe::{AnimationDefinition, Channel, Interpolation::{self, CatmullRom, Linear}, Keyframe, Target};",
       "use glam::Vec3;",
       "use std::sync::LazyLock;",
       "fn k(t: f32, pre: [f32; 3], post: [f32; 3], i: Interpolation) -> Keyframe { Keyframe { timestamp: t, pre: Vec3::from(pre), post: Vec3::from(post), interpolation: i } }",
       ""]
for name in FILES:
    src = (D / f"{name}Animation.java").read_text()
    total = src.count("new Keyframe(")
    found = 0
    for m in re.finditer(r"public static final AnimationDefinition (\w+) = AnimationDefinition\.Builder\.withLength\(([\d.]+)F\)(.*?);\n", src, re.S):
        cname, length, body = m.group(1), m.group(2), m.group(3)
        looping = ".looping()" in body
        chans = []
        for piece in body.split(".addAnimation(")[1:]:
            cm = re.match(r'\s*"(\w+)",\s*new AnimationChannel\(\s*AnimationChannel\.Targets\.(\w+),(.*)', piece, re.S)
            bone, tgt, kfs = cm.groups()
            ks = []
            for kk in KF.finditer(kfs):
                t, k1, a1, k2, a2, interp = kk.groups()
                post = vec(k1, a1)
                pre = vec(k2, a2) if k2 else post
                ks.append((num(t), pre, post, interp == "CATMULLROM"))
            found += len(ks)
            chans.append((bone, {"POSITION": 0, "ROTATION": 1, "SCALE": 2}[tgt], ks))
        out.append(f"pub static {cname.upper()}: LazyLock<AnimationDefinition> = LazyLock::new(|| AnimationDefinition {{ length: {f(num(length))}, looping: {str(looping).lower()}, channels: vec![")
        for bone, t, ks in chans:
            tn = ["Position", "Rotation", "Scale"][t]
            out.append(f'    Channel {{ bone: "{bone}", target: Target::{tn}, keyframes: vec![')
            for (tt, pre, post, cr) in ks:
                out.append(f"        k({f(tt)}, [{', '.join(map(f, pre))}], [{', '.join(map(f, post))}], {'CatmullRom' if cr else 'Linear'}),")
            out.append("    ] },")
        out.append("] });\n")
    if found != total:
        sys.exit(f"{name}: разобрано {found} из {total} кейфреймов")
(root / "crates/render/src/keyframes_data.rs").write_text("\n".join(out))
print("ok")
