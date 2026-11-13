#!/usr/bin/env python3
"""Сборка и запуск мод-дампера ezzmc-bake под Forge 1.20.1 (DEC-018, TODO-502/503).

Подкоманды:
  mappings   — Tiny v2 `named`(mojmap) ↔ `srg` из client.txt Mojang + mcp_config merged tsrg2
  devjar     — компиляционный jar MC+Forge-патчей в mojmap-именах (srg.jar ⊕ forge-client.jar → named)
  build      — javac дампера + ремап named→srg → jar мода
  run        — запуск Forge-клиента из ~/.minecraft с дампером, результат в <out>.ezbake
  all        — всё по порядку

Рабочая папка: reference/_cache/forge-bake/ (в git не попадает). Формат дампа: docs/ref/forge-bake.md.
"""
import hashlib
import time
import json
import os
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORK = ROOT / "reference/_cache/forge-bake"
SRC = ROOT / "tools/forge-bake"
MC = Path(os.environ.get("EZZMC_MINECRAFT_DIR", Path.home() / ".minecraft"))
LIB = MC / "libraries"
FORGE_VER = "1.20.1-forge-47.4.28"
MCP = "1.20.1-20230612.114412"
CLIENT_DIR = LIB / f"net/minecraft/client/{MCP}"
PROGUARD = CLIENT_DIR / f"client-{MCP}-mappings.txt"
SRG_JAR = CLIENT_DIR / f"client-{MCP}-srg.jar"
TSRG = LIB / f"de/oceanlabs/mcp/mcp_config/{MCP}/mcp_config-{MCP}-mappings-merged.txt"
FORGE_CLIENT = LIB / "net/minecraftforge/forge/1.20.1-47.4.28/forge-1.20.1-47.4.28-client.jar"
FORGE_UNIVERSAL = LIB / "net/minecraftforge/forge/1.20.1-47.4.28/forge-1.20.1-47.4.28-universal.jar"
TINY_REMAPPER = ROOT / "reference/_tools/tiny-remapper.jar"
TINY = WORK / "named-srg.tiny"
DEVJAR = WORK / "mc-forge-named.jar"
MODJAR = WORK / "ezzmc-bake.jar"

PRIM = {"void": "V", "boolean": "Z", "byte": "B", "char": "C", "short": "S",
        "int": "I", "long": "J", "float": "F", "double": "D"}


def jtype(t, cmap):
    """Тип java из client.txt → дескриптор; классы через cmap (named→obf), если задан."""
    dims = 0
    while t.endswith("[]"):
        dims += 1
        t = t[:-2]
    if t in PRIM:
        d = PRIM[t]
    else:
        n = t.replace(".", "/")
        if cmap is not None:
            n = cmap.get(n, n)
        d = f"L{n};"
    return "[" * dims + d


def cmd_mappings():
    WORK.mkdir(parents=True, exist_ok=True)
    # client.txt: named → obf
    classes = {}  # named -> (obf, fields[(type,name,obf)], methods[(ret,name,args,obf)])
    cur = None
    for line in PROGUARD.read_text().splitlines():
        if not line or line.startswith("#"):
            continue
        if not line.startswith(" "):
            named, obf = line.rstrip(":").split(" -> ")
            cur = [obf.replace(".", "/"), [], []]
            classes[named.replace(".", "/")] = cur
            continue
        body, obf = line.strip().split(" -> ")
        if "(" in body:
            body = body.split(":")[-1]
            ret, rest = body.split(" ", 1)
            name, args = rest.split("(", 1)
            args = args.rstrip(")")
            cur[2].append((ret, name, [a for a in args.split(",") if a], obf))
        else:
            typ, name = body.split(" ")
            cur[1].append((typ, name, obf))
    n2o = {k: v[0] for k, v in classes.items()}
    # tsrg2 merged: obf class → (fields obf→srg, methods (obf,desc)→srg)
    srg_f, srg_m = {}, {}
    cls = None
    for line in TSRG.read_text().splitlines()[1:]:
        if not line.startswith("\t"):
            cls = line.split(" ")[0]
            continue
        if line.startswith("\t\t"):
            continue
        p = line.strip().split(" ")
        if len(p) == 3:
            srg_m[(cls, p[0], p[1])] = p[2]
        elif len(p) == 2:
            srg_f[(cls, p[0])] = p[1]
    out = ["tiny\t2\t0\tnamed\tsrg"]
    nm = nf = 0
    for named, (obf, fields, methods) in classes.items():
        out.append(f"c\t{named}\t{named}")
        for typ, name, o in fields:
            s = srg_f.get((obf, o))
            if s and s != name:
                out.append(f"\tf\t{jtype(typ, None)}\t{name}\t{s}")
                nf += 1
        for ret, name, args, o in methods:
            if name in ("<init>", "<clinit>"):
                continue
            od = "(" + "".join(jtype(a, n2o) for a in args) + ")" + jtype(ret, n2o)
            s = srg_m.get((obf, o, od))
            if s and s != name:
                nd = "(" + "".join(jtype(a, None) for a in args) + ")" + jtype(ret, None)
                out.append(f"\tm\t{nd}\t{name}\t{s}")
                nm += 1
    TINY.write_text("\n".join(out) + "\n")
    print(f"mappings: {len(classes)} классов, {nm} методов, {nf} полей → {TINY}")


def remap(src, dst, frm, to, cp):
    args = ["java", "-jar", str(TINY_REMAPPER), str(src), str(dst), str(TINY), frm, to]
    args += [str(c) for c in cp]
    args += ["--ignoreConflicts", "--rebuildSourceFilenames"]
    subprocess.run(args, check=True)


def forge_json():
    return json.loads((MC / f"versions/{FORGE_VER}/{FORGE_VER}.json").read_text())


def lib_paths():
    out = []
    for lib in forge_json()["libraries"]:
        art = lib.get("downloads", {}).get("artifact")
        if art and art.get("path"):
            p = LIB / art["path"]
        else:
            parts = lib["name"].split(":")
            g, a, v = parts[:3]
            cl = f"-{parts[3]}" if len(parts) > 3 else ""
            if cl.startswith("-natives-") and "linux" not in cl:
                continue
            p = LIB / g.replace(".", "/") / a / v / f"{a}-{v}{cl}.jar"
        if p.exists() and p not in out:
            out.append(p)
    return out


def fml_jars():
    """Языковые/ядро FML (в json их нет — Forge грузит сам); нужны только для javac."""
    return sorted(LIB.glob("net/minecraftforge/*/1.20.1-47.4.28/*.jar"))


def cmd_devjar():
    if not TINY.exists():
        cmd_mappings()
    merged = WORK / "mc-forge-srg.jar"
    with zipfile.ZipFile(merged, "w", zipfile.ZIP_DEFLATED) as zo:
        seen = set()
        for jar in (FORGE_CLIENT, SRG_JAR):  # патчи Forge важнее ванильных классов
            with zipfile.ZipFile(jar) as zi:
                for e in zi.infolist():
                    if not e.filename.endswith(".class") or e.filename in seen:
                        continue
                    seen.add(e.filename)
                    zo.writestr(e.filename, zi.read(e))
    libs = [p for p in lib_paths() if "minecraftforge/forge/" not in str(p)]
    remap(merged, DEVJAR, "srg", "named", libs + [FORGE_UNIVERSAL])
    print(f"devjar → {DEVJAR}")


def cmd_build():
    if not DEVJAR.exists():
        cmd_devjar()
    cls = WORK / "classes"
    shutil.rmtree(cls, ignore_errors=True)
    cls.mkdir(parents=True)
    srcs = [str(p) for p in (SRC / "src").rglob("*.java")]
    cp = [DEVJAR, FORGE_UNIVERSAL] + fml_jars() + lib_paths()
    subprocess.run(["javac", "-source", "17", "-target", "17", "-nowarn", "-encoding", "UTF-8", "-d", str(cls),
                    "-cp", os.pathsep.join(map(str, cp))] + srcs, check=True)
    named = WORK / "ezzmc-bake-named.jar"
    with zipfile.ZipFile(named, "w", zipfile.ZIP_DEFLATED) as z:
        for p in cls.rglob("*.class"):
            z.write(p, p.relative_to(cls).as_posix())
        for p in (SRC / "resources").rglob("*"):
            if p.is_file():
                z.write(p, p.relative_to(SRC / "resources").as_posix())
    remap(named, MODJAR, "named", "srg", [DEVJAR, FORGE_UNIVERSAL] + fml_jars() + lib_paths())
    print(f"мод → {MODJAR}")


def mods_hash(mods_dir):
    h = hashlib.sha256()
    for p in sorted(mods_dir.glob("*.jar")):
        h.update(p.name.encode())
        h.update(str(p.stat().st_size).encode())
    return h.hexdigest()[:16]


def cmd_run(out=None):
    if not MODJAR.exists():
        cmd_build()
    mods = MC / "mods"
    out = Path(out) if out else WORK / f"{mods_hash(mods)}.ezbake"
    # отдельная игровая папка: свои mods (симлинки + дампер), общие assets/libraries
    game = WORK / "game"
    gmods = game / "mods"
    shutil.rmtree(gmods, ignore_errors=True)
    gmods.mkdir(parents=True)
    for p in mods.glob("*.jar"):
        (gmods / p.name).symlink_to(p)
    shutil.copy(MODJAR, gmods / "zz-ezzmc-bake.jar")
    for d in ("config", "defaultconfigs"):
        if (MC / d).exists() and not (game / d).exists():
            shutil.copytree(MC / d, game / d)  # копия: моды не трогают конфиги игрока
    # v3: состояние обхода машин (MenuProbe) — переживает краши модов; новый запуск начинает с нуля
    state = Path(str(out) + ".state")
    if not os.environ.get("EZZMC_BAKE_RESUME"):
        shutil.rmtree(state, ignore_errors=True)
    # язык окон и переводов — как у игрока (lang из ~/.minecraft/options.txt)
    lang = next((l.split(":", 1)[1].strip() for l in (MC / "options.txt").read_text(errors="ignore").splitlines()
                 if l.startswith("lang:")), "en_us") if (MC / "options.txt").exists() else "en_us"
    opts = game / "options.txt"
    lines = [l for l in (opts.read_text(errors="ignore").splitlines() if opts.exists() else [])
             if not l.startswith("lang:")]
    opts.write_text("\n".join(lines + [f"lang:{lang}"]) + "\n")
    j = forge_json()
    natives = MC / f"versions/{FORGE_VER}/natives"
    cp = lib_paths() + [MC / f"versions/{FORGE_VER}/{FORGE_VER}.jar"]
    vars_ = {
        "natives_directory": str(natives), "launcher_name": "ezzmc-bake", "launcher_version": "1",
        "classpath": os.pathsep.join(map(str, cp)), "classpath_separator": os.pathsep,
        "library_directory": str(LIB), "version_name": FORGE_VER, "auth_player_name": "EzzBake",
        "game_directory": str(game), "assets_root": str(MC / "assets"),
        "assets_index_name": j.get("assetIndex", {}).get("id", "5"),
        "auth_uuid": "00000000000000000000000000000000", "auth_access_token": "0", "clientid": "0",
        "auth_xuid": "0", "user_type": "legacy", "version_type": "release",
    }

    def expand(args):
        res = []
        for a in args:
            if isinstance(a, dict):
                if any(r.get("os") for r in a.get("rules", [])):
                    continue
                if any("features" in r for r in a.get("rules", [])):
                    continue
                v = a["value"]
                res += v if isinstance(v, list) else [v]
            else:
                res.append(a)
        out_ = []
        for a in res:
            for k, v in vars_.items():
                a = a.replace("${" + k + "}", v)
            out_.append(a)
        return out_

    jvm = ["-Xmx6G", f"-Dezzmc.bake.out={out}"] + expand(j["arguments"]["jvm"])
    game_args = expand(j["arguments"]["game"])
    java = os.environ.get("EZZMC_BAKE_JAVA", "java")
    log = WORK / "run.log"
    print(f"запуск Forge (лог {log}) → {out}")
    jvm.insert(1, f"-Dezzmc.bake.state={state}")
    prog = state / "progress.txt"
    # Моды падают/виснут на «голых» машинах в своём тике — дампер это не ловит. Тогда убиваем игру,
    # чистим мир (виновный блок в нём) и продолжаем со следующего блока (MenuProbe читает progress.txt).
    for attempt in range(60):
        shutil.rmtree(game / "saves" / "ezzmc-bake", ignore_errors=True)
        with open(log, "a" if attempt else "w") as lf:
            p = subprocess.Popen([java] + jvm + [j["mainClass"]] + game_args, cwd=game,
                                 stdout=lf, stderr=subprocess.STDOUT)
            t0 = time.time()
            while p.poll() is None:
                time.sleep(3)
                if out.exists():
                    continue
                moved = prog.exists() and prog.stat().st_mtime > t0
                stale = time.time() - (prog.stat().st_mtime if moved else t0)
                if stale > (60 if moved else 600):
                    print(f"  нет прогресса {stale:.0f} с (блок #{prog.read_text().strip() if prog.exists() else '?'}) — перезапуск")
                    p.kill()
                    p.wait()
        if out.exists():
            break
        print(f"  попытка {attempt + 1}: игра упала на блоке #{prog.read_text().strip() if prog.exists() else '?'} — продолжаю")
    if not out.exists():
        sys.exit(f"дамп не создан, см. {log}")
    print(f"готово: {out} ({out.stat().st_size} байт)")


def main():
    c = sys.argv[1] if len(sys.argv) > 1 else "all"
    if c == "mappings":
        cmd_mappings()
    elif c == "devjar":
        cmd_devjar()
    elif c == "build":
        cmd_build()
    elif c == "run":
        cmd_run(sys.argv[2] if len(sys.argv) > 2 else None)
    elif c == "all":
        cmd_mappings()
        cmd_devjar()
        cmd_build()
        cmd_run()
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
