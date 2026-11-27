#!/usr/bin/env python3
"""Smoke test of the version-sensitive packets against a vanilla server.

    cargo build --profile fast -p ezzmc-client
    python3 tools/version_smoke.py 1.16.2 [1.17.1 ...]
    python3 tools/version_smoke.py --only entity_data,entity_spawn 1.14.4

`--only` runs join + teleport (world setup) and the listed checks; use it while fixing one
area. The full run is for changes in shared packet code, on a couple of eras.

For every version: starts `cargo xtask server <ver>`, joins with ezzmc through the control
socket, runs the checks (client actions are verified on the server, server actions on the
client) and prints a PASS/FAIL table. Exit code 1 if anything failed. Needs Minecraft 1.14+
(`data` command). Env: SMOKE_PORT (25650), SMOKE_CONTROL_PORT (25590), SMOKE_DIR (/tmp).
"""
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time

PORT = int(os.environ.get("SMOKE_PORT", "25650"))
os.environ["EZZMC_CONTROL_PORT"] = os.environ.get("SMOKE_CONTROL_PORT", "25590")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ezzmc_mcp as bridge  # noqa: E402

BOT = "EzzBot"
WORK = os.environ.get("SMOKE_DIR") or tempfile.gettempdir()
Y = 120  # player feet; the platform is at Y-1
HOME = (1008.5, Y, 1008.5)


class Fail(Exception):
    pass


class Server:
    def __init__(self, ver):
        self.log = os.path.join(WORK, f"smoke-server-{ver}.log")
        self.proc = subprocess.Popen(
            ["cargo", "xtask", "server", ver, str(PORT), "--animals"], cwd=bridge.ROOT,
            stdin=subprocess.PIPE, stdout=open(self.log, "w"), stderr=subprocess.STDOUT, text=True)
        self.expect(r"Done \(", 0, 300)

    def text(self):
        with open(self.log, errors="replace") as f:
            return f.read()

    def expect(self, pattern, start, timeout=10):
        end = time.time() + timeout
        while time.time() < end:
            m = re.search(pattern, self.text()[start:])
            if m:
                return m
            if self.proc.poll() is not None:
                raise Fail(f"server exited, see {self.log}")
            time.sleep(0.2)
        raise Fail(f"server log: no /{pattern}/")

    def run(self, *commands):
        assert self.proc.stdin
        for c in commands:
            self.proc.stdin.write(c + "\n")
        self.proc.stdin.flush()

    def ask(self, command, pattern, timeout=10):
        start = len(self.text())
        self.run(command)
        return self.expect(pattern, start, timeout)

    def items(self, what):
        """{slot: (item, count)} of `entity <name> Inventory` or `block <pos> Items`."""
        line = self.ask(f"data get {what}", r"has the following (?:entity|block) data: (.*)").group(1)
        out = {}
        for entry in re.findall(r"\{[^{}]*Slot: [^{}]*\}", line):
            slot = int(re.search(r"Slot: (\d+)b", entry).group(1))
            item = re.search(r'id: "minecraft:(\w+)"', entry).group(1)
            out[slot] = (item, int(re.search(r"[Cc]ount: (\d+)", entry).group(1)))
        return out

    def is_block(self, pos, block, timeout=5):
        mark = f"IS{time.monotonic_ns()}"
        end = time.time() + timeout
        while True:
            try:
                x, y, z = pos
                self.ask(f"execute if block {x} {y} {z} minecraft:{block} run say {mark}", mark, 1)
                return
            except Fail:
                if time.time() > end:
                    raise Fail(f"server: {pos} is not {block}")

    def stop(self):
        if self.proc.poll() is None:
            try:
                self.run("stop")
                self.proc.wait(timeout=60)
            except (OSError, subprocess.TimeoutExpired):
                self.proc.kill()


def cmd(name, **args):
    r = bridge.send(dict(args, cmd=name))
    if not r.get("ok"):
        raise Fail(f"{name}: {r.get('error')}")
    return r


def state():
    return cmd("state")


def until(what, probe, timeout=10):
    end = time.time() + timeout
    last = None
    while time.time() < end:
        last = probe()
        if last[0]:
            return
        time.sleep(0.25)
    raise Fail(f"{what}: got {last[1] if last else None}")


def block(x, y, z, name, **fields):
    def probe():
        b = cmd("block_at", x=x, y=y, z=z)
        return b.get("block") == name and all(b.get(k) == v for k, v in fields.items()), b
    until(f"client block {x} {y} {z} = {name} {fields}", probe)


def inventory(slot, item, count, key="slots"):
    def probe():
        got = {s["slot"]: (s["item"], s["count"]) for s in cmd("inventory").get(key) or []}
        return got.get(slot) == (item, count), got
    until(f"client {key}[{slot}] = {item} x{count}", probe)


def server_items(s, what, expected):
    def probe():
        got = s.items(what)
        return all(got.get(k) == v for k, v in expected.items()), got
    until(f"server {what} has {expected}", probe)


def click(slot, button="left"):
    cmd("slot_click", slot=slot, button=button)
    cmd("wait", ms=350)


def near(a, b, eps=0.3):
    return all(abs(x - y) <= eps for x, y in zip(a, b))


def at(pos, timeout=15):
    until(f"client pos {pos}", lambda: (near(state()["pos"], pos, 0.6), state()["pos"]), timeout)


def alive():
    st = state()
    if st["screen"] != "play" or st["health"] <= 0:
        raise Fail(f"bot is not alive: screen {st['screen']}, health {st['health']}")


# --- checks, in order: later ones rely on the world prepared by earlier ones ---

def c_join(s, ver):
    cmd("wait_joined", timeout_ms=60000)
    until("screen", lambda: (state()["screen"] in ("play", "death"), state()["screen"]), 60)
    if state()["screen"] == "death":  # the stand world remembers a bot left dead by an earlier run
        cmd("wait", ms=1500)
        cmd("respawn")
        until("respawned", lambda: (state()["screen"] == "play", state()["screen"]), 30)


def c_teleport(s, ver):
    s.run(f"op {BOT}", "difficulty peaceful", "gamerule doDaylightCycle false",
          "gamerule doWeatherCycle false", "gamerule doMobSpawning false", "weather clear",
          "worldborder center 0 0", "worldborder set 59999968",  # stand worlds are reused
          "time set 1000", f"gamemode spectator {BOT}",
          f"execute in minecraft:overworld run tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20")
    cmd("wait", ms=4000)
    s.run(f"fill 1000 {Y} 1000 1016 {Y + 6} 1016 air", f"fill 1000 {Y - 1} 1000 1016 {Y - 1} 1016 stone",
          "kill @e[type=!minecraft:player]", f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20",
          f"clear {BOT}", f"effect clear {BOT}")
    block(1001, Y - 1, 1001, "stone")
    s.run(f"gamemode survival {BOT}")
    at(HOME)
    alive()


def c_chunk_resend(s, ver):
    s.run(f"gamemode spectator {BOT}", f"tp {BOT} 5000 {Y} 5000")
    at((5000.5, Y, 5000.5))
    cmd("wait", ms=3000)
    s.run(f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20", f"gamemode survival {BOT}")
    at(HOME)
    block(1001, Y - 1, 1001, "stone", loaded=True)
    s.is_block((1004, Y - 1, 1013), "stone", 30)  # the server must have the chunk again too
    alive()


def c_section_update(s, ver):
    s.run(f"fill 1004 {Y} 1013 1006 {Y} 1013 gold_block")
    for x in (1004, 1005, 1006):
        block(x, Y, 1013, "gold_block")


def c_block_update(s, ver):
    s.run(f"setblock 1010 {Y} 1013 diamond_block")
    block(1010, Y, 1013, "diamond_block")


def c_light(s, ver):
    s.run(f"setblock 1013 {Y} 1008 torch")
    block(1013, Y, 1008, "torch", block_light=14)
    block(1014, Y, 1008, "air", block_light=13)


def c_chat_in(s, ver):
    s.run("say smoke-from-server")
    until("chat line", lambda: (any("smoke-from-server" in x for x in state()["chat"]["lines"]),
                                state()["chat"]["lines"][-3:]))


def c_chat_out(s, ver):
    start = len(s.text())
    cmd("chat_send", text="smoke-from-client")
    s.expect(rf"<(\[\w+\] )?{BOT}> smoke-from-client", start)


def c_command_out(s, ver):
    start = len(s.text())
    cmd("chat_send", text="/say smoke-command")
    said = rf"\[(\[\w+\] )?{BOT}\] smoke-command"
    if ver == (1, 19):  # the 1.19 server logs /say like ordinary chat
        said = rf"<(\[\w+\] )?{BOT}> smoke-command"
    s.expect(said, start)


def c_give(s, ver):
    s.run(f"give {BOT} cobblestone 10", f"give {BOT} oak_log 4")
    inventory(36, "cobblestone", 10)
    inventory(37, "oak_log", 4)


def c_inventory_click(s, ver):
    cmd("tap", key="e")
    cmd("wait", ms=500)
    click(36)
    click(9)
    click(37, "right")  # half of the stack
    click(10)
    server_items(s, f"entity {BOT} Inventory",
                 {9: ("cobblestone", 10), 10: ("oak_log", 2), 1: ("oak_log", 2)})
    inventory(9, "cobblestone", 10)
    cmd("tap", key="esc")
    cmd("wait", ms=400)


def c_chest_open(s, ver):
    count = "count:5" if ver >= (1, 20, 5) else "Count:5b"
    s.run(f'setblock 1008 {Y} 1010 chest{{Items:[{{Slot:0b,id:"minecraft:diamond",{count}}}]}}')
    block(1008, Y, 1010, "chest")
    # The task reports "screen changed" when the container opens; the content check below decides.
    r = bridge.send({"cmd": "use_block", "x": 1008, "y": Y, "z": 1010, "face": "north"})
    if not r.get("ok") and "screen changed" not in str(r.get("error")):
        raise Fail(f"use_block: {r.get('error')}")
    inventory(0, "diamond", 5, "container")


def c_chest_click(s, ver):
    click(0)
    click(5)
    server_items(s, f"block 1008 {Y} 1010 Items", {5: ("diamond", 5)})
    inventory(5, "diamond", 5, "container")


def c_chest_shift_click(s, ver):
    cmd("key", key="shift", down=True)
    try:
        cmd("wait", ms=200)
        click(27)  # first main inventory slot of a single chest menu
    finally:
        cmd("key", key="shift", down=False)
    server_items(s, f"block 1008 {Y} 1010 Items", {0: ("cobblestone", 10), 5: ("diamond", 5)})
    cmd("tap", key="esc")
    cmd("wait", ms=400)


def c_place_block(s, ver):
    s.run(f"give {BOT} stone_bricks 3")
    inventory(36, "stone_bricks", 3)
    cmd("hotbar", slot=0)
    cmd("use_block", x=1006, y=Y - 1, z=1009, face="up")
    s.is_block((1006, Y, 1009), "stone_bricks")
    inventory(36, "stone_bricks", 2)


def c_break_block(s, ver):
    s.run(f"setblock 1010 {Y} 1009 glass")
    block(1010, Y, 1009, "glass")
    cmd("mine", x=1010, y=Y, z=1009)
    s.is_block((1010, Y, 1009), "air")
    block(1010, Y, 1009, "air")


def entity_types():
    return [e["type"] for e in cmd("entities", max_dist=32, limit=60)["entities"]]


def c_entity_spawn(s, ver):
    s.run(f"summon pig 1006.5 {Y} 1012.0 {{NoAI:1b}}", f"summon armor_stand 1011.5 {Y} 1011.5")
    for kind in ("pig", "armor_stand"):
        until(f"entity {kind}", lambda: (kind in entity_types(), entity_types()))


def c_entity_data(s, ver):
    """Метаданные: стек выпавшего предмета, флаг baby, огонь."""
    s.run(f"summon item 1004.5 {Y} 1012.5 {{Item:{{id:\"minecraft:diamond\",Count:3b,count:3}},PickupDelay:32767s}}",
          f"summon pig 1008.5 {Y} 1013.5 {{NoAI:1b,Age:-24000,Fire:2000s}}")

    def seen():
        es = cmd("entities", max_dist=32, limit=60)["entities"]
        stack = any(e.get("item") == {"item": "diamond", "count": 3} for e in es)
        baby = any(e["type"] == "pig" and e.get("baby") and e.get("on_fire") for e in es)
        return stack and baby, [e for e in es if e["type"] in ("item", "pig")]

    until("entity metadata", seen)
    s.run("kill @e[type=minecraft:item]", "kill @e[type=minecraft:pig,x=1008,y=119,z=1013,distance=..4]")


def c_entity_remove(s, ver):
    s.run("kill @e[type=minecraft:pig]", "kill @e[type=minecraft:armor_stand]")
    until("entities gone", lambda: (not {"pig", "armor_stand"} & set(entity_types()), entity_types()))


def c_effects(s, ver):
    def names():
        return [e["name"] for e in state()["effects"]]
    s.run(f"effect give {BOT} speed 30 1")
    until("speed effect", lambda: ("speed" in names(), names()))
    s.run(f"effect clear {BOT}")
    until("effect removed", lambda: (not names(), names()))


def c_health(s, ver):
    s.run(f"effect give {BOT} instant_damage 1 0")
    until("health below 20", lambda: (state()["health"] < 20, state()["health"]))
    s.run(f"effect give {BOT} instant_health 1 4")
    until("health restored", lambda: (state()["health"] == 20, state()["health"]))


def c_time(s, ver):
    s.run("time set 6000")
    until("day time", lambda: (state()["world"]["day_time"] % 24000 == 6000, state()["world"]["day_time"]))


def c_weather(s, ver):
    s.run("weather rain")
    until("rain on", lambda: (state()["world"]["raining"], state()["world"]["raining"]), 20)
    s.run("weather clear")
    until("rain off", lambda: (not state()["world"]["raining"], state()["world"]["raining"]), 20)


def c_movement(s, ver):
    start = (1003.5, Y, 1003.5)
    s.run(f"tp {BOT} {start[0]} {start[1]} {start[2]} 0 0")
    at(start)
    cmd("wait", ms=500)
    log = len(s.text())
    cmd("tap", key="w", ms=800)
    cmd("wait", ms=1500)
    client = state()["pos"]
    if client[2] - start[2] < 1:
        raise Fail(f"client did not walk: {client}")
    line = s.ask(f"data get entity {BOT} Pos", r"entity data: \[(.*?)\]").group(1)
    server = [float(v.strip().rstrip("d")) for v in line.split(",")]
    if not near(client, server, 0.05):
        raise Fail(f"client {client} != server {server}")
    if re.search(r"moved (wrongly|too quickly)", s.text()[log:]):
        raise Fail("server complained about movement")


def c_nether(s, ver):
    s.run(f"gamemode spectator {BOT}", f"execute in minecraft:the_nether run tp {BOT} 0.5 {Y} 0.5")
    until("nether", lambda: ("the_nether" in state()["world"]["dimension"], state()["world"]["dimension"]), 30)
    until("nether chunks", lambda: (cmd("block_at", x=0, y=1, z=0).get("loaded"), "not loaded"), 30)
    block(0, 0, 0, "bedrock")
    s.run(f"execute in minecraft:overworld run tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20",
          f"gamemode survival {BOT}")
    until("overworld", lambda: ("overworld" in state()["world"]["dimension"], state()["world"]["dimension"]), 30)
    at(HOME, 30)
    block(1001, Y - 1, 1001, "stone")
    s.is_block((1004, Y - 1, 1013), "stone", 30)


def c_death_respawn(s, ver):
    s.run(f"kill {BOT}")
    until("death screen", lambda: (state()["screen"] == "death", state()["screen"]))
    cmd("wait", ms=1500)
    cmd("respawn")
    until("respawned", lambda: (state()["screen"] == "play" and state()["health"] == 20,
                                (state()["screen"], state()["health"])), 30)


def c_still_connected(s, ver):
    cmd("wait", ms=3000)
    alive()
    if "Error executing task" in s.text():
        raise Fail(f"server error (broken server state), see {s.log}")
    if re.search(rf"{BOT} lost connection", s.text()):
        raise Fail("server log: lost connection")
    with open(bridge.LOG, errors="replace") as f:
        bad = [x for x in f if "panicked" in x or " ERROR " in x]
    if bad:
        raise Fail(f"client log: {bad[0].strip()}")


CHECKS = [(n[2:], f) for n, f in list(globals().items()) if n.startswith("c_")]


def hidden_window():
    """Command prefix that keeps the client off the desktop; SMOKE_WINDOW=1 shows it."""
    if os.environ.get("SMOKE_WINDOW") or not shutil.which("gamescope"):
        return []
    return ["gamescope", "--backend", "headless", "-W", "1280", "-H", "720",
            "-w", "1280", "-h", "720", "--"]


def smoke(ver_name):
    ver = tuple(int(x) for x in ver_name.split("."))
    bridge.LOG = os.path.join(WORK, f"smoke-client-{ver_name}.log")
    bridge.update_progress = lambda: None
    results = []
    server = Server(ver_name)
    try:
        r = bridge.launch({"addr": f"127.0.0.1:{PORT}", "name": BOT, "wrap": hidden_window()})
        if not r.get("ok"):
            raise Fail(f"launch: {r.get('error')}")
        for name, check in CHECKS:
            try:
                check(server, ver)
                results.append((name, None))
            except Fail as e:
                results.append((name, str(e)))
                print(f"  {ver_name} {name}: FAIL {str(e)[:300]}", flush=True)
                # A screen left open by a failed check would break the following ones.
                st = bridge.send({"cmd": "state"})
                if st.get("screen") in ("inventory", "chat"):
                    bridge.send({"cmd": "tap", "key": "esc"})
                if name == "join" or not st.get("ok") or st.get("screen") == "menu":
                    results.append(("(remaining checks)", "skipped: client not in game"))
                    break
            if results[-1][1] is None:
                print(f"  {ver_name} {name}: PASS", flush=True)
    finally:
        bridge.stop({})
        server.stop()
    return results


def main():
    global CHECKS
    args = sys.argv[1:]
    if args[:1] == ["--only"] and len(args) > 1:
        wanted = set(args[1].split(","))
        unknown = wanted - {n for n, _ in CHECKS}
        if unknown:
            sys.exit(f"unknown checks {sorted(unknown)}; known: {[n for n, _ in CHECKS]}")
        CHECKS = [c for c in CHECKS if c[0] in wanted | {"join", "teleport"}]
        args = args[2:]
    if not args:
        sys.exit(__doc__)
    summary = []
    for ver in args:
        try:
            results = smoke(ver)
        except Fail as e:
            results = [("start", str(e))]
        failed = [(n, e) for n, e in results if e is not None]
        summary.append((ver, len(results) - len(failed), failed))
    print()
    for ver, passed, failed in summary:
        print(f"{ver}: {passed} passed, {len(failed)} failed")
        for name, error in failed:
            print(f"  FAIL {name}: {error[:300]}")
    sys.exit(1 if any(f for _, _, f in summary) else 0)


if __name__ == "__main__":
    main()
