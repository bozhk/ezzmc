#!/usr/bin/env python3
"""Smoke test of the version-sensitive packets against a vanilla server.

    cargo build --profile fast -p ezzmc-client
    python3 tools/version_smoke.py 1.16.2 [1.17.1 ...]
    python3 tools/version_smoke.py --only entity_data,entity_spawn 1.14.4

`--only` runs join + teleport (world setup) and the listed checks; use it while fixing one
area. The full run is for changes in shared packet code, on a couple of eras.

For every version: starts `cargo xtask server <ver>`, joins with ezzmc through the control
socket, runs the checks (client actions are verified on the server, server actions on the
client) and prints a PASS/FAIL table. Exit code 1 if anything failed. Needs Minecraft 1.12+
(`data` command). Env: SMOKE_PORT (25650), SMOKE_CONTROL_PORT (25590), SMOKE_DIR (/tmp).
"""
import os
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time

PORT = int(os.environ.get("SMOKE_PORT", "25650"))
os.environ["EZZMC_CONTROL_PORT"] = os.environ.get("SMOKE_CONTROL_PORT", "25590")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ezzmc_mcp as bridge  # noqa: E402
import smoke_legacy
import smoke_packets  # noqa: E402

BOT = "EzzBot"
WORK = os.environ.get("SMOKE_DIR") or tempfile.gettempdir()
Y = 120  # player feet; the platform is at Y-1
HOME = (1008.5, Y, 1008.5)

# The clients must not read or write the player's options.txt; a short render distance also lets
# several smokes fit into video memory at once.
os.environ.setdefault("EZZMC_CLIENT_ARGS", "--log-packets")
os.environ.setdefault("EZZMC_ARENA_VERTICES", "2000000")
os.environ.setdefault("EZZMC_LOW_VRAM", "1")
os.environ.setdefault("EZZMC_NO_USER_INPUT", "1")
os.environ["EZZMC_OPTIONS"] = os.path.join(WORK, "smoke-options.txt")
with open(os.environ["EZZMC_OPTIONS"], "w") as _f:
    _f.write("renderDistance:%s\npauseOnLostFocus:false\ncomputeRenderer:false\nenableVsync:false\nmaxFps:30\n"
             % os.environ.get("SMOKE_RENDER_DISTANCE", "2"))


class Fail(Exception):
    pass


class Server:
    def __init__(self, ver):
        self.log = os.path.join(WORK, f"smoke-server-{ver}.log")
        self.ver = tuple(int(x) for x in ver.split("."))
        self.legacy = self.ver < (1, 13)
        self.world = os.path.join(bridge.ROOT, "reference/_server", ver, "world")
        if self.ver < (1, 7):
            fixture_world(self.world)
        elif self.legacy:
            # pre-1.13 commands cannot move the bot between dimensions: start it from the spawn
            import glob
            for sub in ("playerdata", "players"):  # before 1.7.6: `players/<name>.dat`
                for f in glob.glob(os.path.join(self.world, sub, "*.dat")):
                    os.remove(f)
        self.proc = subprocess.Popen(
            ["cargo", "xtask", "server", ver, str(PORT), "--animals"]
            # 1.7 generates and saves chunks on the main thread: view distance 15 stalls it for
            # tens of seconds after every teleport. Newer ones: less memory per parallel smoke.
            + ["--view-distance=%d" % int(os.environ.get("SMOKE_VIEW_DISTANCE", "3"))], cwd=bridge.ROOT,
            stdin=subprocess.PIPE, stdout=open(self.log, "w"), stderr=subprocess.STDOUT, text=True,
            start_new_session=True)  # `stop` kills the java child together with cargo
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
            if self.legacy:
                c = smoke_legacy.command(c, self.ver)
            for line in c if isinstance(c, list) else [c]:
                self.proc.stdin.write(line + "\n")
        self.proc.stdin.flush()

    def ask(self, command, pattern, timeout=10):
        start = len(self.text())
        self.run(command)
        return self.expect(pattern, start, timeout)

    def items(self, what):
        """{slot: (item, count)} of `entity <name> Inventory` or `block <pos> Items`."""
        if self.legacy:  # blocks only, players are checked by `server_items`
            pos = " ".join(what.split()[1:4])
            reply = self.ask(f"blockdata {pos} {{}}", r"data tag did not change: (.*)")
            return smoke_legacy.items_nbt(reply.group(1))
        line = self.ask(f"data get {what}", r"has the following (?:entity|block) data: (.*)").group(1)
        out = {}
        for entry in re.findall(r"\{[^{}]*Slot: [^{}]*\}", line):
            slot = int(re.search(r"Slot: (\d+)b", entry).group(1))
            item = re.search(r'id: "minecraft:(\w+)"', entry).group(1)
            out[slot] = (item, int(re.search(r"[Cc]ount: (\d+)", entry).group(1)))
        return out

    def save(self):
        self.ask("save-all", r"Saved the world", 120)

    def is_block(self, pos, block, timeout=5):
        if self.ver < (1, 7):  # no `testforblock`: read the saved region file
            def saved():
                self.save()
                got = smoke_legacy.block_id(self.world, *pos)
                return got == smoke_legacy.IDS_1_7[block], got
            until(f"server {pos} is {block}", saved, max(timeout, 10))
            return
        mark = f"IS{time.monotonic_ns()}"
        end = time.time() + timeout
        first = len(self.text())
        while True:
            try:
                x, y, z = pos
                if self.legacy:
                    # A loaded legacy server answers late (chunk generation on the main
                    # thread): a reply to an earlier attempt counts too.
                    old = smoke_legacy.OLD_NAMES.get(block, block)
                    self.run(f"testforblock {x} {y} {z} minecraft:{old}")
                    self.expect(rf"Successfully found the block at {x},\s*{y},\s*{z}", first, 1)
                    return
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
                os.killpg(self.proc.pid, 9)


FIXTURE = os.path.join(bridge.ROOT, "reference/_server/1.7.10/fixture-1.6")


def fixture_commands():
    """1.7.10 commands building the world the 1.6 checks expect (1.6 has no setblock/summon).

    The pens keep the entities in place, the gold blocks carry torches (breaking one changes
    two blocks of a section), the portal frame is lit by its fire."""
    y = Y
    c = [f"fill 1000 {y} 1000 1016 {y + 6} 1016 air", f"fill 1000 {y - 1} 1000 1016 {y - 1} 1016 stone",
         f"fill 1004 {y} 1013 1006 {y} 1013 gold_block", f"fill 1004 {y + 1} 1013 1006 {y + 1} 1013 torch",
         f"setblock 1010 {y} 1013 diamond_block", f"setblock 1013 {y} 1008 torch",
         f"setblock 1010 {y} 1009 glass",
         f"setblock 1008 {y} 1010 chest 0 replace {{Items:[{{Slot:0b,id:264s,Count:5b,Damage:0s}}]}}"]
    # the baby pig pen is 3x3 inside: the bot is teleported in to hit the pig
    for px, pz, r in ((1012, 1004, 1), (1015, 1001, 0), (1015, 1015, 0)):
        c += [f"fill {px - r - 1} {y} {pz - r - 1} {px + r + 1} {y + 1} {pz + r + 1} glass",
              f"fill {px - r} {y} {pz - r} {px + r} {y + 1} {pz + r} air"]
    c += [f"fill 1000 {y - 1} 1016 1003 {y + 3} 1016 obsidian",
          f"fill 1001 {y} 1016 1002 {y + 2} 1016 portal",
          f"summon Pig 1015.5 {y} 1001.5", f"summon Pig 1012.5 {y} 1004.5 {{Age:-2000000000}}",
          f"summon Villager 1015.5 {y} 1015.5",
          f"summon Item 1002.5 {y} 1012.5 {{Item:{{id:264s,Count:3b,Damage:0s}},PickupDelay:32767s,Age:-32768s}}"]
    return [x for line in c for x in smoke_legacy.command_1_7(line)]


def fixture_world(world):
    """Fresh copy of the 1.6 fixture world (built once by a 1.7.10 server, same Anvil format)."""
    if not os.path.exists(os.path.join(FIXTURE, "fixture.ok")):
        shutil.rmtree(FIXTURE, ignore_errors=True)
        log = os.path.join(WORK, "smoke-fixture.log")
        for step in ("spawn", "build"):
            proc = subprocess.Popen(
                ["cargo", "xtask", "server", "1.7.10", str(PORT), "--level=fixture-1.6",
                 "--animals"],  # without it the server removes animals and villagers
                cwd=bridge.ROOT, stdin=subprocess.PIPE, stdout=open(log, "w"),
                stderr=subprocess.STDOUT, text=True, start_new_session=True)
            srv = Server.__new__(Server)
            srv.log, srv.proc, srv.legacy, srv.ver = log, proc, False, (1, 7, 10)
            srv.expect(r"Done \(", 0, 300)
            if step == "spawn":
                # the spawn area is loaded at start-up: restart around the fixture
                srv.run("setworldspawn 1008 120 1008", "gamerule doMobSpawning false",
                        "difficulty 0", "time set 1000")
            else:
                for line in fixture_commands():
                    proc.stdin.write(line + "\n")
                proc.stdin.flush()
                srv.run(f"testforblock 1001 {Y} 1016 minecraft:portal")  # queued after the build
                srv.expect(rf"Successfully found the block at 1001,\s*{Y},\s*1016", 0, 300)
            srv.ask("save-all", r"Saved the world", 120)
            srv.stop()
        open(os.path.join(FIXTURE, "fixture.ok"), "w").close()
    shutil.rmtree(world, ignore_errors=True)
    shutil.copytree(FIXTURE, world, ignore=shutil.ignore_patterns("session.lock", "fixture.ok"))


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
    if s.ver < (1, 8):
        world = os.path.join(bridge.ROOT, "reference/_server", ".".join(map(str, s.ver)), "world")

        def found_1_7():
            if what.startswith("entity "):
                s.ask("save-all", r"Saved the world", 120)
                got = smoke_legacy.player_items(world, BOT)
                return all(got.get(k) == v for k, v in expected.items()), got
            pos = " ".join(what.split()[1:4])
            if s.ver < (1, 7):
                s.save()
                got = smoke_legacy.chest_items(world, *map(int, pos.split()))
                return all(got.get(k) == v for k, v in expected.items()), got
            try:
                s.ask(smoke_legacy.chest_test(pos, expected), r"Successfully found the block", 1)
                return True, None
            except Fail:
                return False, "chest content differs"
        until(f"server {what} has {expected}", found_1_7, 20)
        return
    if s.legacy and what.startswith("entity "):
        def found():
            try:
                s.ask(smoke_legacy.has_items(BOT, expected), rf"Found {BOT}", 1)
                return True, None
            except Fail:
                return False, f"server inventory lacks {expected}"
        until("server items", found)
        return
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
    if st.get("screen") != "play" or st.get("health", 0) <= 0:
        raise Fail(f"bot is not alive: screen {st.get('screen')}, health {st.get('health')}")


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
          *(["gamerule advance_time false", "gamerule advance_weather false",
             "gamerule spawn_mobs false"] if s.ver >= (1, 21, 11) else []),
          "time set 1000", f"gamemode spectator {BOT}",
          f"execute in overworld run tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20")
    cmd("wait", ms=4000)
    if smoke_packets.cleanup(s.ver):
        s.run(*smoke_packets.cleanup(s.ver))
    # A fresh world may still be generating ("That position is not loaded"). 1.7 expands `fill`
    # into ~2300 setblocks: send them once and wait, repeats would stall the server.
    attempts = 0 if ver < (1, 7) else 1 if ver < (1, 8) else 10  # 1.6: the fixture world has it
    for attempt in range(attempts):
        s.run(f"fill 1000 {Y} 1000 1016 {Y + 6} 1016 air",
              f"fill 1000 {Y - 1} 1000 1016 {Y - 1} 1016 stone")
        try:
            s.is_block((1016, Y - 1, 1016), "stone", 3 if attempts > 1 else 90)
            s.is_block((1000, Y - 1, 1000), "stone", 3 if attempts > 1 else 30)
            break
        except Fail:
            if attempt == attempts - 1:
                raise
    s.run("kill @e[type=!minecraft:player]", *smoke_packets.cleanup(s.ver),  # the kill drops loot
          f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20", f"clear {BOT}", f"effect clear {BOT}")
    block(1001, Y - 1, 1001, "stone")
    s.run(f"gamemode survival {BOT}")
    at(HOME)
    face(ver, 0, 20)
    alive()


def face(ver, yaw, pitch):
    """1.7 `tp` has no rotation: turn the client (the next move packet carries it)."""
    if ver < (1, 8):
        cmd("look", yaw=yaw, pitch=pitch)


def at_xz(pos, timeout=15):
    """1.7 has no spectator: a creative bot falls after a teleport, only x/z are certain."""
    until(f"client pos {pos}", lambda: (near(state()["pos"][::2], pos[::2], 0.6), state()["pos"]), timeout)


def c_chunk_resend(s, ver):
    s.run(f"gamemode spectator {BOT}", f"tp {BOT} 5000 {Y} 5000")
    (at_xz if ver < (1, 8) else at)((5000.5, Y, 5000.5))
    cmd("wait", ms=3000)
    if s.legacy:
        # 1.8 generates the new area on the main thread: commands wait for it (~30 s).
        s.ask(f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20", rf"Teleported {BOT} to {HOME[0]}", 90)
        s.run(f"gamemode survival {BOT}")
    else:
        s.run(f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20", f"gamemode survival {BOT}")
    at(HOME)
    face(ver, 0, 20)
    block(1001, Y - 1, 1001, "stone", loaded=True)
    s.is_block((1004, Y - 1, 1013), "stone", 30)  # the server must have the chunk again too
    alive()


def creative_mine(s, pos):
    """1.6 has no setblock: the bot breaks a fixture block, the server answers with block changes.

    The dig must reach the server before the teleport home: a lagging old server (after chunk
    generation) handles it later, and from HOME the block is out of reach (distance² > 36,
    ServerPacketListener#onPlayerAction, 1.5.2/1.6.4)."""
    s.run(f"gamemode creative {BOT}", f"tp {BOT} {pos[0] + 0.5} {Y} {pos[2] - 2.5}")
    at((pos[0] + 0.5, Y, pos[2] - 2.5))
    try:
        cmd("mine", x=pos[0], y=pos[1], z=pos[2])
        s.is_block(pos, "air")
    finally:
        s.run(f"gamemode survival {BOT}", f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]}")
        at(HOME)
        face(s.ver, 0, 20)


def c_section_update(s, ver):
    if ver < (1, 7):
        # the torch on top falls in the same tick: a multi block change from the server
        creative_mine(s, (1005, Y, 1013))
        block(1005, Y + 1, 1013, "air")
        for x in (1004, 1006):
            block(x, Y, 1013, "gold_block")
        return
    s.run(f"fill 1004 {Y} 1013 1006 {Y} 1013 gold_block")
    for x in (1004, 1005, 1006):
        block(x, Y, 1013, "gold_block")


def c_block_update(s, ver):
    if ver < (1, 7):
        block(1010, Y, 1013, "diamond_block")
        creative_mine(s, (1010, Y, 1013))
        block(1010, Y, 1013, "air")
        return
    s.run(f"setblock 1010 {Y} 1013 diamond_block")
    block(1010, Y, 1013, "diamond_block")


def c_light(s, ver):
    if ver >= (1, 7):  # 1.6: the torch is in the fixture world
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


def c_tab_complete(s, ver):
    """Tab in chat: the server's command suggestions reach the popup and replace the last word."""
    cmd("chat_open", text="/gamem")
    time.sleep(0.5)  # the first frame must see the prefill before Tab
    cmd("chat_key", key="tab")

    def listed():
        chat = state()["chat"]
        return any("gamemode" in x for x in chat.get("suggestions") or []), chat

    until("suggestions", listed, 10)
    cmd("chat_key", key="tab")  # the popup is open: Tab applies the selected entry
    until("completed input", lambda: (state()["chat"]["input"] == "/gamemode", state()["chat"]["input"]), 5)
    cmd("chat_key", key="esc")
    cmd("chat_key", key="esc")
    # plain chat: player names (from the tab list since 1.13, from the server before)
    cmd("chat_open", text="hi " + BOT[:3])
    time.sleep(0.5)
    cmd("chat_key", key="tab")

    def named():
        chat = state()["chat"]
        return BOT in (chat.get("suggestions") or []), chat

    until("name suggestions", named, 10)
    cmd("chat_key", key="tab")
    until("completed name", lambda: (state()["chat"]["input"] == "hi " + BOT, state()["chat"]["input"]), 5)
    cmd("chat_key", key="esc")
    cmd("chat_key", key="esc")


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
    if ver >= (1, 7):  # 1.6: the chest is in the fixture world
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
    s.run(f"clear {BOT}", f"give {BOT} stone_bricks 3")  # slot 36 must be free
    inventory(36, "stone_bricks", 3)
    cmd("hotbar", slot=0)
    cmd("use_block", x=1006, y=Y - 1, z=1009, face="up")
    s.is_block((1006, Y, 1009), "stone_bricks")
    inventory(36, "stone_bricks", 2)


def c_map(s, ver):
    """An empty map becomes a filled one: colours and the player marker arrive, the hand draws it."""
    s.run(f"clear {BOT}", f"give {BOT} map 1")
    inventory(36, "map", 1)
    cmd("hotbar", slot=0)
    cmd("look", yaw=0, pitch=-60)  # at the sky: no block takes the click
    time.sleep(0.5)
    cmd("click", button="right", ms=100)
    inventory(36, "filled_map", 1)

    def drawn():
        maps = state().get("maps") or []
        return any(m["filled"] > 0 and any(d["type"] == "player" for d in m["decorations"])
                   for m in maps), maps

    until("map colours", drawn, 20)
    cmd("look", yaw=0, pitch=60)  # looking down raises the map into view
    time.sleep(1.5)
    shutil.copy(cmd("screenshot")["path"], os.path.join(WORK, f"map-{'.'.join(map(str, ver))}.png"))
    # the same map in an item frame on a wall two blocks away
    old = ver < (1, 7)  # no `setblock`: hang the frame on the glass of the fixture world
    if old:
        s.run(f"tp {BOT} 1010.5 {Y} 1007.5", f"give {BOT} item_frame 1")
        block(1010, Y, 1009, "glass")
    else:
        s.run(f"tp {BOT} 1008.5 {Y} 1008.5", f"setblock 1008 {Y + 1} 1011 stone", f"give {BOT} item_frame 1")
        block(1008, Y + 1, 1011, "stone")
    inventory(37, "item_frame", 1)

    def frames():
        return [e for e in cmd("entities", max_dist=8).get("entities") or [] if e["type"] == "item_frame"]

    cmd("hotbar", slot=1)
    cmd("look", yaw=0, pitch=37 if old else 0)
    punch_hanging()
    cmd("click", button="right", ms=100)
    until("item frame", lambda: (len(frames()) == 1, frames()), 10)
    cmd("hotbar", slot=0)
    time.sleep(0.5)
    cmd("click", button="right", ms=100)
    until("map in the frame",
          lambda: (any((e.get("item") or {}).get("item") == "filled_map" for e in frames()), frames()), 10)

    def spent():
        got = {s["slot"]: (s["item"], s["count"]) for s in cmd("inventory").get("slots") or []
               if s["slot"] in (36, 37) and s["item"] != "air" and s["count"]}
        return not got, got

    until("frame and map left the hotbar", spent, 10)
    time.sleep(1.5)
    shutil.copy(cmd("screenshot")["path"], os.path.join(WORK, f"mapframe-{'.'.join(map(str, ver))}.png"))
    for _ in range(2):  # the first hit drops the map, the second breaks the frame
        cmd("click", button="left", ms=50)
        time.sleep(0.7)
    until("frame removed", lambda: (not frames(), frames()), 10)
    s.run(f"clear {BOT}")
    if not old:
        cmd("look", yaw=0, pitch=20)
        s.run(f"setblock 1008 {Y + 1} 1011 air")


def punch_hanging():
    """Removes frames and paintings under the crosshair (stand worlds keep them between runs)."""
    for _ in range(4):
        time.sleep(0.7)
        if (cmd("target").get("entity") or {}).get("type") not in ("item_frame", "painting"):
            return
        cmd("click", button="left", ms=50)


def c_painting(s, ver):
    """A painting hangs on one block: the item is spent, the crosshair picks it, a hit removes it."""
    old = ver < (1, 7)  # no `setblock`: the glass of the fixture world
    x, z, pitch = (1010.5, 1007.5, 37) if old else (1008.5, 1008.5, 0)
    s.run(f"gamemode survival {BOT}", f"clear {BOT}", f"tp {BOT} {x} {Y} {z}")
    if not old:
        s.run(f"setblock 1008 {Y + 1} 1011 stone")
        block(1008, Y + 1, 1011, "stone")
    s.run(f"give {BOT} painting 1")
    inventory(36, "painting", 1)
    cmd("hotbar", slot=0)
    cmd("look", yaw=0, pitch=pitch)
    punch_hanging()
    cmd("click", button="right", ms=100)

    def paintings():
        return [e for e in cmd("entities", max_dist=8).get("entities") or [] if e["type"] == "painting"]

    until("painting", lambda: (len(paintings()) == 1, paintings()), 10)

    def spent():
        got = {i["slot"]: (i["item"], i["count"]) for i in cmd("inventory").get("slots") or []
               if i["slot"] == 36 and i["item"] != "air" and i["count"]}
        return not got, got

    until("painting left the hotbar", spent, 10)

    def aimed():
        got = (cmd("target").get("entity") or {})
        return got.get("type") == "painting", got

    until("crosshair on the painting", aimed, 10)
    time.sleep(1)
    shutil.copy(cmd("screenshot")["path"], os.path.join(WORK, f"painting-{'.'.join(map(str, ver))}.png"))
    cmd("click", button="left", ms=50)
    until("painting removed", lambda: (not paintings(), paintings()), 10)
    if not old:
        s.run(f"setblock 1008 {Y + 1} 1011 air")
    s.run(f"clear {BOT}")


def c_break_block(s, ver):
    if ver >= (1, 7):  # 1.6: the glass is in the fixture world
        s.run(f"setblock 1010 {Y} 1009 glass")
    block(1010, Y, 1009, "glass")
    cmd("mine", x=1010, y=Y, z=1009)
    s.is_block((1010, Y, 1009), "air")
    block(1010, Y, 1009, "air")


def c_stats(s, ver):
    """The statistics screen: values requested from the server, ids mapped to the canon."""
    # pre-Netty: mined comes from server increments, play time is counted by the client
    expected = ["custom:play_time", "mined:glass"]
    cmd("stats_screen", action="open")

    def values():
        v = (state().get("stats_screen") or {}).get("values") or {}
        return all(v.get(k, 0) > 0 for k in expected), {k: v[k] for k in list(v)[:40]}

    try:
        until("stats", values, 10)
    finally:
        cmd("stats_screen", action="close")


def c_advancements(s, ver):
    """Grant two story advancements: tree, progress, toast and the screen."""
    if ver < (1, 12):
        return  # older versions have achievements, sent as statistics
    s.run(f"advancement revoke {BOT} everything")
    time.sleep(1.0)  # recipe advancements without a display may be granted back at once
    s.run(f"advancement grant {BOT} only minecraft:story/root",
          f"advancement grant {BOT} only minecraft:story/mine_stone")
    until("granted", lambda: (state()["advancements"]["done"] >= 2, state()["advancements"]))
    until("toast", lambda: (any("mine_stone" in t["title"] or "Stone" in t["title"]
                                or "Камен" in t["title"] for t in state()["toasts"]),
                            state()["toasts"]), 15)
    cmd("adv_screen", action="open")

    def screen():
        a = state().get("adv_screen") or {}
        ok = any("story/root" in t["id"] for t in a.get("tabs", []))
        return ok and a.get("widgets", 0) >= 2, a

    try:
        until("advancements screen", screen)
        time.sleep(0.5)
        shutil.copy(cmd("screenshot")["path"],
                    os.path.join(WORK, f"adv-{'.'.join(map(str, ver))}.png"))
    finally:
        cmd("adv_screen", action="close")
    s.run(f"advancement revoke {BOT} everything")


def canon_item(name):
    """Item id of the client's registry (26.3)."""
    src = open(os.path.join(bridge.ROOT, "crates/protocol/src/generated/r26_3.rs")).read()
    block = src.split("pub static ITEM: &[&str] = &[", 1)[1].split("];", 1)[0]
    return block.replace("\n", "").replace(" ", "").split(",").index(f'"{name}"')


def c_recipes(s, ver):
    """Recipe book: unlock, list and place a recipe (server fills the crafting grid)."""
    if ver < (1, 12):
        return  # the recipe book appeared in 1.12
    planks = canon_item("oak_planks")
    s.run(f"clear {BOT}", f"recipe take {BOT} *", f"recipe give {BOT} *",
          f"give {BOT} oak_log 1")

    def listed():
        r = cmd("recipes", result=planks, limit=10)
        return r["count"] > 300 and bool(r["recipes"]), (r["count"], r["recipes"][:2])

    until("recipes", listed, 20)
    inventory(36, "oak_log", 1)
    cmd("tap", key="e")
    cmd("wait", ms=500)
    try:
        if ver != (1, 12):  # 1.12.0 has no place_recipe packet (not implemented)
            found = [x["id"] for x in cmd("recipes", result=planks, limit=10)["recipes"]]
            for rid in found:  # several recipes give oak planks; only the log one fits
                cmd("place_recipe", recipe=rid)
                cmd("wait", ms=400)
            inventory(0, "oak_planks", 4)
        # the recipe book button (GUI px, 854x480 window); it moves right when the book opens
        cmd("slot_click", x=239, y=106)
        cmd("wait", ms=500)
        cmd("recipe_search", text="planks")
        cmd("wait", ms=500)
        shutil.copy(cmd("screenshot")["path"],
                    os.path.join(WORK, f"recipes-{'.'.join(map(str, ver))}.png"))
        cmd("slot_click", x=316, y=106)
        cmd("wait", ms=400)
        if "open: true" in cmd("recipes", limit=1)["settings"]:
            raise Fail("recipe book did not close")
    finally:
        cmd("tap", key="esc")
        cmd("wait", ms=400)
    s.run(f"clear {BOT}", f"recipe take {BOT} *")


def entity_types():
    return [e["type"] for e in cmd("entities", max_dist=32, limit=60)["entities"]]


def stand(ver):
    """A second, non-mob entity type; 1.7 has no armor stand."""
    return "armor_stand" if ver >= (1, 8) else "villager"


def c_entity_spawn(s, ver):
    if ver < (1, 7):  # the fixture pens hold a pig, a baby pig and a villager
        for kind in ("pig", stand(ver)):
            until(f"entity {kind}", lambda: (kind in entity_types(), entity_types()))
        return
    s.run(f"summon pig 1006.5 {Y} 1012.0 {{NoAI:1b}}", f"summon {stand(ver)} 1011.5 {Y} 1011.5")
    for kind in ("pig", stand(ver)):
        until(f"entity {kind}", lambda: (kind in entity_types(), entity_types()))


def c_entity_data(s, ver):
    """Метаданные: стек выпавшего предмета, флаг baby, огонь."""
    if ver < (1, 7):
        # 1.6 has no summon: the fixture item and baby pig; a fire aspect sword sets it on fire
        s.run(f"give {BOT} wooden_sword 1")  # dropped at the player: pick it up first

        def sword():
            got = {x["item"]: x["slot"] for x in cmd("inventory")["slots"] or []}
            return "wooden_sword" in got and got["wooden_sword"] >= 36, got
        until("wooden sword", sword)
        slot = next(x["slot"] for x in cmd("inventory")["slots"] if x["item"] == "wooden_sword")
        cmd("hotbar", slot=slot - 36)
        cmd("wait", ms=300)
        s.run(f"enchant {BOT} 20 1", f"tp {BOT} 1011.5 {Y} 1003.5")
        at((1011.5, Y, 1003.5))
        cmd("wait", ms=500)
        pig = next(e for e in cmd("entities", max_dist=32, limit=60)["entities"]
                   if e["type"] == "pig" and e.get("baby"))
        cmd("attack", id=pig["id"])
    else:
        s.run(f"summon item 1004.5 {Y} 1012.5 {{Item:{{id:\"minecraft:diamond\",Count:3b,count:3}},PickupDelay:32767s}}",
              f"summon pig 1008.5 {Y} 1013.5 {{NoAI:1b,Age:-24000,Fire:2000s}}")

    def seen():
        es = cmd("entities", max_dist=32, limit=60)["entities"]
        stack = any(e.get("item") == {"item": "diamond", "count": 3} for e in es)
        baby = any(e["type"] == "pig" and e.get("baby") and e.get("on_fire") for e in es)
        return stack and baby, [e for e in es if e["type"] in ("item", "pig")]

    until("entity metadata", seen)
    if ver < (1, 7):
        s.run(f"clear {BOT}", f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]}")
        at(HOME)
        face(ver, 0, 20)
        return
    s.run("kill @e[type=minecraft:item]", "kill @e[type=minecraft:pig,x=1008,y=119,z=1013,distance=..4]")


def c_entity_remove(s, ver):
    if ver < (1, 8):
        # 1.7 console cannot kill entities: leaving the tracking range removes them on the client.
        s.run(f"gamemode spectator {BOT}", f"tp {BOT} 5000 {Y} 5000")
        at_xz((5000.5, Y, 5000.5))

        def far():
            types = [e["type"] for e in cmd("entities", max_dist=100000, limit=200)["entities"]]
            return not {"pig", stand(ver)} & set(types), types
        until("entities gone", far, 30)
        s.ask(f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]}", rf"Teleported {BOT} to", 90)
        s.run(f"gamemode survival {BOT}")
        at(HOME)
        face(ver, 0, 20)
        return
    s.run("kill @e[type=minecraft:pig]", "kill @e[type=minecraft:armor_stand]")
    until("entities gone", lambda: (not {"pig", "armor_stand"} & set(entity_types()), entity_types()))


def c_player_spawn(s, ver):
    """A second client joins: the bot must see a `player` entity (old versions send add_player)."""
    log = os.path.join(WORK, "smoke-other-client.log")
    other = subprocess.Popen(
        hidden_window() + [bridge.BIN, "--connect", f"127.0.0.1:{PORT}", "--name", "Other"],
        cwd=bridge.ROOT, stdout=open(log, "w"), stderr=subprocess.STDOUT, start_new_session=True)
    try:
        s.expect(r"Other.*logged in", 0, 40)
        s.run(f"tp Other {BOT}")

        moved = time.monotonic()

        def seen():
            nonlocal moved
            if time.monotonic() - moved > 5:  # a tp sent before the client finished joining is lost
                moved = time.monotonic()
                s.run(f"tp Other {BOT}")
            found = cmd("entities", type="player", max_dist=64)["entities"]
            ok = any(e.get("name") == "Other" for e in found)
            if ok:
                return ok, found
            near = cmd("entities", max_dist=64)["entities"][:4]
            return ok, (found, cmd("state").get("pos"), near)

        until("other player", seen, 30)
    finally:
        os.killpg(other.pid, signal.SIGKILL)
        other.wait()


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
    # 1.5.2 has no doDaylightCycle: the time keeps running after `time set`.
    lo, hi = (6000, 6600) if ver < (1, 6) else (6000, 6001)
    until("day time", lambda: (lo <= state()["world"]["day_time"] % 24000 < hi, state()["world"]["day_time"]))


def c_weather(s, ver):
    s.run("weather rain")
    until("rain on", lambda: (state()["world"]["raining"], state()["world"]["raining"]), 20)
    s.run("weather clear")
    until("rain off", lambda: (not state()["world"]["raining"], state()["world"]["raining"]), 20)


def c_movement(s, ver):
    start = (1003.5, Y, 1003.5)
    s.run(f"tp {BOT} {start[0]} {start[1]} {start[2]} 0 0")
    at(start)
    face(ver, 0, 0)
    cmd("wait", ms=500)
    log = len(s.text())
    cmd("tap", key="w", ms=800)
    cmd("wait", ms=1500)
    client = state()["pos"]
    if client[2] - start[2] < 1:
        raise Fail(f"client did not walk: {client}")
    if s.legacy:  # no `data get`: a relative teleport reports where the server has the player
        line = s.ask(f"tp {BOT} ~ ~ ~", rf"Teleported {BOT} to (.*)").group(1)
    else:
        line = s.ask(f"data get entity {BOT} Pos", r"entity data: \[(.*?)\]").group(1)
    server = [float(v.strip().rstrip("d")) for v in line.split(",")]
    if not near(client, server, 0.05):
        raise Fail(f"client {client} != server {server}")
    if re.search(r"moved (wrongly|too quickly)", s.text()[log:]):
        raise Fail("server complained about movement")


def legacy_portal(s, target):
    """1.12 commands cannot change dimension: walk a creative bot into a built portal."""
    p = state()["pos"]
    x, y, z = int(p[0] // 1) + 3, int(p[1] // 1), int(p[2] // 1)
    s.run(f"gamemode creative {BOT}",
          f"execute {BOT} ~ ~ ~ fill {x - 1} {y - 1} {z} {x + 2} {y + 3} {z} obsidian",
          f"execute {BOT} ~ ~ ~ fill {x} {y} {z} {x + 1} {y + 2} {z} portal",
          f"tp {BOT} {x + 1}.0 {y} {z}.5")

    def arrived():
        st = state()
        dim = (st.get("world") or {}).get("dimension", "")
        # the position in the new world comes after the respawn (seconds later on a lagging 1.8)
        moved = st.get("screen") == "play" and not near(st["pos"], (x + 1, y, z + 0.5), 4)
        return target in dim and moved, (dim, st.get("pos"))
    until(target, arrived, 60)


def c_nether(s, ver):
    if s.legacy:
        if ver < (1, 7):  # the fixture portal; 1.6 cannot build one
            s.run(f"gamemode creative {BOT}", f"tp {BOT} 1002.0 {Y} 1016.5")

            def arrived():
                st = state()
                dim = (st.get("world") or {}).get("dimension", "")
                # the nether position comes after the respawn
                moved = not near(st["pos"], (1002.0, Y, 1016.5), 4)
                return "the_nether" in dim and st.get("screen") == "play" and moved, (dim, st["pos"])
            until("the_nether", arrived, 60)
        else:
            legacy_portal(s, "the_nether")
        p = state()["pos"]
        x, z = int(p[0] // 1), int(p[2] // 1)
        until("nether chunks", lambda: (cmd("block_at", x=x, y=0, z=z).get("loaded"), "not loaded"), 30)
        block(x, 0, z, "bedrock")
        cmd("wait", ms=11000)  # vanilla portal cooldown
        # step out of the arrival portal so that the next one can take the player; re-read the
        # position: 1.8 moves the player twice (scaled position, then the found portal)
        p = state()["pos"]
        x, z = int(p[0] // 1), int(p[2] // 1)
        s.run(f"tp {BOT} {x + 6} {int(p[1] // 1)} {z}")
        cmd("wait", ms=3000)  # the next portal is placed relative to the new position
        if ver < (1, 8):
            # 1.7: console commands run in the overworld; walk back into the arrival portal.
            s.run(f"tp {BOT} {p[0]:.2f} {p[1]:.2f} {p[2]:.2f}")
            def home():
                dim = (state().get("world") or {}).get("dimension", "")
                return "overworld" in dim, dim
            until("overworld", home, 60)
        else:
            legacy_portal(s, "overworld")
        cmd("wait", ms=3000)
        s.run(f"tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20", f"gamemode survival {BOT}")
        at(HOME, 30)
        face(ver, 0, 20)
        block(1001, Y - 1, 1001, "stone")
        s.is_block((1004, Y - 1, 1013), "stone", 30)
        return
    s.run(f"gamemode spectator {BOT}", f"execute in the_nether run tp {BOT} 0.5 {Y} 0.5")
    until("nether", lambda: ("the_nether" in state()["world"]["dimension"], state()["world"]["dimension"]), 30)
    until("nether chunks", lambda: (cmd("block_at", x=0, y=1, z=0).get("loaded"), "not loaded"), 30)
    block(0, 0, 0, "bedrock")
    s.run(f"execute in overworld run tp {BOT} {HOME[0]} {HOME[1]} {HOME[2]} 0 20",
          f"gamemode survival {BOT}")
    until("overworld", lambda: ("overworld" in state()["world"]["dimension"], state()["world"]["dimension"]), 30)
    at(HOME, 30)
    block(1001, Y - 1, 1001, "stone")
    s.is_block((1004, Y - 1, 1013), "stone", 30)


def c_hud(s, ver):
    """Title, sidebar and (1.13+) boss bar packets."""
    if ver >= (1, 8):
        s.run(f'title {BOT} title {{"text":"SmokeTitle"}}')
        until("title", lambda: ((state().get("title") or {}).get("title") == "SmokeTitle", state().get("title")))
    if ver >= (1, 7):
        s.run("scoreboard objectives add smk dummy", "scoreboard players set Foo smk 7",
              "scoreboard objectives setdisplay sidebar smk")
        until("sidebar", lambda: (any("Foo" in line for line in (state().get("sidebar") or {}).get("lines", [])),
                                  state().get("sidebar")))
        s.run("scoreboard objectives remove smk")
        until("sidebar removed", lambda: (not state().get("sidebar"), state().get("sidebar")))
    if ver >= (1, 13):
        s.run('bossbar add smk "SmokeBar"', f"bossbar set smk players {BOT}")
        # only our bar: a reused stand world may keep custom bars added by hand
        def names():
            return [b["name"] for b in state().get("boss") or []]

        until("boss bar", lambda: ("SmokeBar" in names(), state().get("boss")))
        s.run("bossbar remove smk")
        until("boss bar removed", lambda: ("SmokeBar" not in names(), state().get("boss")))


def c_sign(s, ver):
    """Sign text and world border: block entity NBT and border packets differ per version."""
    if ver < (1, 8):  # no NBT argument for setblock
        return
    if ver >= (1, 21, 5):
        text = "{front_text:{messages:[\"SmokeSign\",\"\",\"\",\"\"]}}"
    elif ver >= (1, 20):
        line = "'\"SmokeSign\"'"
        empty = "'\"\"'"
        text = "{front_text:{messages:[%s]}}" % ",".join([line] + [empty] * 3)
    else:
        # old NBT parsers know no single quotes: an escaped double-quoted string
        text = '{Text1:"{\\"text\\":\\"SmokeSign\\"}"}'
    pos = (1004, Y, 1009)
    sign = "oak_sign" if ver >= (1, 14) else "sign" if ver >= (1, 13) else "standing_sign"
    if ver >= (1, 13):
        s.run(f"setblock {pos[0]} {pos[1]} {pos[2]} {sign}{text}")
    else:
        s.run(f"setblock {pos[0]} {pos[1]} {pos[2]} {sign} 0 replace {text}")

    def front():
        got = cmd("block_at", x=pos[0], y=pos[1], z=pos[2])
        return ((got.get("sign") or {}).get("front") or [""])[0] == "SmokeSign", got

    until("sign text", front)
    s.run(f"setblock {pos[0]} {pos[1]} {pos[2]} air")

    def border():
        got = (state().get("world") or {}).get("border") or {}
        return got.get("center") == [1000.0, 1000.0] and got.get("size") == 500.0, got

    s.run("worldborder center 1000.0 1000.0", "worldborder set 500")  # integers get +0.5
    until("world border", border)
    s.run("worldborder center 0 0", "worldborder set 59999968")


def c_packets(s, ver):
    """Fires commands for as many packet types as possible; the client must parse all of them."""
    with open(bridge.LOG, errors="replace") as f:
        before = len(f.read())
    assert s.proc.stdin
    for line in smoke_packets.commands(ver, BOT, *HOME):
        s.proc.stdin.write(line + "\n")
        s.proc.stdin.flush()
        time.sleep(0.15)
    cmd("wait", ms=4000)
    alive()
    for line in smoke_packets.cleanup(ver):
        s.proc.stdin.write(line + "\n")
        s.proc.stdin.flush()
    with open(bridge.LOG, errors="replace") as f:
        f.seek(0)
        new = f.read()[before:]
    bad = sorted({re.sub(r"^.*?(WARN|ERROR) ", "", line.strip()) for line in new.splitlines()
                  if re.search(r"трансляция версии|разворот пакета|блок-энтити|panicked| ERROR ", line)})
    if bad:
        raise Fail(f"client parse errors ({len(bad)}): " + " | ".join(bad[:6]))


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
                shot = os.path.join(WORK, f"fail-{ver_name}-{name}.png")
                bridge.send({"cmd": "screenshot", "path": shot})
                print(f"    pos {st.get('pos')} yaw {st.get('yaw')} pitch {st.get('pitch')} "
                      f"screen {st.get('screen')} target {st.get('target')} held {st.get('held_slot')}",
                      flush=True)
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
        if os.path.exists(bridge.LOG):
            print(f"  {ver_name} " + smoke_packets.report(
                bridge.ROOT, ver_name, bridge.LOG,
                os.path.join(WORK, f"packets-{ver_name}.json")), flush=True)
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
