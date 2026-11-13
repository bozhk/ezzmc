#!/usr/bin/env python3
"""MCP server (stdio, no dependencies): lets an agent play ezzmc.

Starts `ezzmc-client --control` and talks to its control socket (crates/client/src/control.rs).
Register via .mcp.json. Env: EZZMC_CONTROL_PORT (25573), EZZMC_BIN, EZZMC_NAME.
"""
import base64, json, os, socket, subprocess, sys, time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PORT = int(os.environ.get("EZZMC_CONTROL_PORT", "25573"))
BIN = os.environ.get("EZZMC_BIN", os.path.join(ROOT, "target/fast/ezzmc-client"))
LOG = "/tmp/ezzmc-client.log"
proc = sock = fh = None


def connect(timeout=60):
    global sock, fh
    end = time.time() + timeout
    while time.time() < end:
        try:
            sock = socket.create_connection(("127.0.0.1", PORT), timeout=130)
            fh = sock.makefile("rw")
            return True
        except OSError:
            time.sleep(0.5)
    return False


def send(cmd):
    global sock, fh
    if fh is None and not connect(2):
        return {"ok": False, "error": "game is not running, call game_launch first"}
    try:
        fh.write(json.dumps(cmd) + "\n")
        fh.flush()
        line = fh.readline()
        if not line:
            raise OSError("closed")
        return json.loads(line)
    except OSError as e:
        sock = fh = None
        return {"ok": False, "error": f"connection lost: {e}"}


def update_progress():
    """Перегенерирует PROGRESS.md перед запуском игры. Сбой не мешает запуску."""
    script = os.path.join(ROOT, "tools", "progress.py")
    try:
        subprocess.run([sys.executable, script], cwd=ROOT, timeout=30, capture_output=True)
    except (OSError, subprocess.SubprocessError) as e:
        print(f"progress.py: {e}", file=sys.stderr)


def launcher_session():
    """JSON сессии выбранного аккаунта лаунчера (`ezzmc-launcher --print-session`) или None."""
    for prof in ("fast", "release", "debug"):
        exe = os.path.join(ROOT, "target", prof, "ezzmc-launcher")
        if os.path.exists(exe):
            try:
                r = subprocess.run([exe, "--print-session"], capture_output=True, text=True, timeout=60)
                if r.returncode == 0:
                    return json.loads(r.stdout.strip().splitlines()[-1])
            except (OSError, subprocess.SubprocessError, ValueError, IndexError):
                pass
            return None
    return None


def launch(args):
    global proc
    if fh is not None and send({"cmd": "ping"}).get("ok"):
        return {"ok": True, "note": "already running"}
    if connect(0.5):
        return send({"cmd": "ping"})
    if not os.path.exists(BIN):
        return {"ok": False, "error": f"{BIN} missing: cargo build --profile fast -p ezzmc-client"}
    update_progress()
    name = args.get("name") or os.environ.get("EZZMC_NAME", "EzzBot")
    session_env = {}
    # По выбору: online=true — вход лицензией, выбранной в лаунчере (токен только в env процесса,
    # не в логах); по умолчанию — офлайн-ник.
    if args.get("online"):
        sess = launcher_session()
        if not (sess and sess.get("online")):
            return {"ok": False, "error": "в лаунчере не выбран Microsoft-аккаунт или токен не обновился (ezzmc-launcher --print-session)"}
        if True:
            name = sess["name"]
            session_env = {"EZZMC_SESSION_NAME": sess["name"], "EZZMC_SESSION_UUID": sess["uuid"], "EZZMC_SESSION_TOKEN": sess["token"]}
    cmd = [BIN, "--control", f"127.0.0.1:{PORT}", "--name", name]
    if args.get("addr"):
        cmd += ["--connect", args["addr"]]
    env = dict(os.environ, **session_env, **{k: str(v) for k, v in (args.get("env") or {}).items()})
    proc = subprocess.Popen(cmd, cwd=ROOT, stdout=open(LOG, "w"), stderr=subprocess.STDOUT, env=env, start_new_session=True)
    if not connect():
        return {"ok": False, "error": f"no control socket, see {LOG}"}
    return send({"cmd": "ping"})


def stop(_):
    global proc, sock, fh
    if fh is None and not connect(0.5):
        return {"ok": True, "note": "not running"}
    r = send({"cmd": "quit"})
    if proc:
        try:
            proc.wait(timeout=15)
        except subprocess.TimeoutExpired:
            pass
    proc = sock = fh = None
    return r


def observe(args):
    """Один вызов вместо пяти: состояние, цель, сущности рядом и ASCII-срез вокруг."""
    st = send({"cmd": "state"})
    if not st.get("ok"):
        return st
    keep = ["screen", "joined", "pos", "block_pos", "facing", "yaw", "pitch", "health", "food", "on_ground", "in_water", "in_lava",
            "game_mode", "held", "offhand", "world", "target", "task", "chat", "dialog", "effects", "speed_xz", "collided_horizontally"]
    out = {k: st[k] for k in keep if k in st}
    if "chat" in out:
        out["chat"] = out["chat"].get("lines", [])[-5:]
    if st.get("joined"):
        ent = send({"cmd": "entities", "max_dist": args.get("entity_dist", 20), "limit": 15})
        if ent.get("ok"):
            out["entities"] = ent["entities"]
            out["entities_total"] = ent["total"]
        sl = send({"cmd": "slice", "radius": args.get("radius", 6)})
        if sl.get("ok"):
            out["slice"] = {k: sl[k] for k in ("layers", "legend", "axes")}
    return {"ok": True, **out}


def shot_result(prefix=None):
    """Скриншот + отчёт кадра (предупреждения: стена, камера в блоке, плоский/тёмный кадр)."""
    r = send({"cmd": "screenshot"})
    if not r.get("ok"):
        return {"content": [{"type": "text", "text": json.dumps(r)}], "isError": True}
    data = base64.b64encode(open(r["path"], "rb").read()).decode()
    fr = r.get("frame") or {}
    warn = list((r.get("view") or {}).get("warnings") or [])
    if fr.get("flat"):
        warn.append("FRAME LOOKS FLAT (one dominant color/texture): not usable for comparison; use game_frame on the subject")
    if fr.get("dark"):
        warn.append("FRAME IS VERY DARK")
    info = {"view": r.get("view"), "frame": fr, "WARNINGS": warn}
    if prefix:
        info["framing"] = prefix
    return {"content": [{"type": "text", "text": ("⚠ BAD SHOT: " if warn else "OK shot: ") + json.dumps(info, ensure_ascii=False)},
                        {"type": "image", "mimeType": "image/png", "data": data}], "isError": False}


def frame(args):
    """Подойти на dist к объекту (блок x,y,z или entity), развернуться на него и снять кадр.
    from_yaw выбирает сторону, с которой снимать (0 — со стороны севера, 90 — с запада...)."""
    plan = send(dict({k: args[k] for k in ("x", "y", "z", "entity", "dist", "from_yaw") if k in args}, cmd="frame_plan"))
    if not plan.get("ok"):
        return {"content": [{"type": "text", "text": json.dumps(plan)}], "isError": True}
    sx, sy, sz = plan["stand"]
    mv = send({"cmd": "goto", "x": sx, "y": sy, "z": sz, "range": 0.8, "timeout_ms": args.get("timeout_ms", 40000)})
    la = {k: args[k] for k in ("x", "y", "z", "entity", "face") if k in args}
    look = send(dict(la, cmd="look_at"))
    send({"cmd": "wait", "ms": args.get("settle_ms", 500)})
    look = send(dict(la, cmd="look_at"))  # поза могла сдвинуться при остановке
    send({"cmd": "wait", "ms": 250})
    return shot_result({"stand": plan["stand"], "walk": mv, "clear": look.get("clear"), "blocked_by": look.get("blocked_by"), "dist": look.get("dist")})


def S(**props):
    return {"type": "object", "properties": props}


N, STR, INT = {"type": "number"}, {"type": "string"}, {"type": "integer"}
KEYS = "w a s d space shift ctrl e q f 1-9, letters, esc, arrows"

TOOLS = {
    "game_launch": ("Start ezzmc with a control socket (or attach to a running one). addr joins a server right away, e.g. 127.0.0.1:25611; name is the offline nickname. online=true logs in with the licensed account selected in the launcher (needed for online-mode servers like eu.loyisa.cn); default is offline.", S(addr=STR, name=STR, online={"type": "boolean"}, env={"type": "object"}), launch),
    "game_observe": ("BEST FIRST CALL: state + crosshair target + nearby entities + ASCII slice (floor/feet/head layers) in one answer. radius (default 6), entity_dist (20).", S(radius=INT, entity_dist=N), observe),
    "game_stop": ("Quit the game.", S(), stop),
    "game_state": ("Screen (menu/play/inventory/chat/pause/dialog), joined, pos/block_pos, facing, yaw/pitch, health/food/xp, held/offhand/armor, in_water, world (dimension, day_time, weather, loaded chunks), target (crosshair), running task, chat lines, effects, fps.", S(), "state"),
    "game_join_server": ("Join a server, e.g. 127.0.0.1:25611, then call game_wait_joined.", S(addr=STR), "join_server"),
    "game_wait_joined": ("Wait until the world is joined and the position is known.", S(timeout_ms=INT), "wait_joined"),
    "game_dialog_button": ("Press a server dialog button by index (state.dialog.buttons; grid first, then footer).", S(index={"type": "integer"}), "dialog_button"),
    "game_ui_click": ("Left-click a menu screen at window pixel x, y (menus only).", S(x={"type": "number"}, y={"type": "number"}), "ui_click"),
    "game_wait": ("Let the game run for ms milliseconds.", S(ms=INT), "wait"),
    "game_key": ("Press (down=true) or release (down=false) a key and keep it so.", S(key=STR, down={"type": "boolean"}), "key"),
    "game_tap": (f"Press a key for ms milliseconds (default 100) and release. Keys: {KEYS}.", S(key=STR, ms=INT), "tap"),
    "game_mouse": ("Press or release a mouse button (left/right); hold left to break blocks.", S(button=STR, down={"type": "boolean"}), "mouse"),
    "game_slot_click": ("Inventory screen open: click slot (or a point menu_x/menu_y in GUI px from the menu top-left, e.g. enchanting buttons at x=60+, y=14+19*i) (left/right; hover=true only moves the cursor), optionally drag over slots in drag_to (QUICK_CRAFT).", S(slot=INT, menu_x=N, menu_y=N, button=STR, hover={"type": "boolean"}, drag_to={"type": "array", "items": {"type": "integer"}}), "slot_click"),
    "game_creative_scroll": ("Creative screen open: scroll the item grid by dy wheel notches (negative = down, like the mouse wheel).", S(dy=N), "creative_scroll"),
    "game_inv_scroll": ("Inventory-type screen open (villager trades, loom): mouse wheel by dy notches (negative = scroll list down).", S(dy=N), "inv_scroll"),
    "game_creative_click": ("Creative screen open: click one target — tab (0..), cell (grid 0..44), slot (InventoryMenu index, hotbar 36..44), trash=true, outside=true, or x/y GUI px; button left/right/middle; drag_to = slot indexes; hover=true only moves the cursor (tooltip checks).", S(tab=INT, cell=INT, slot=INT, trash={"type": "boolean"}, outside={"type": "boolean"}, x=N, y=N, button=STR, drag_to={"type": "array", "items": {"type": "integer"}}, hold={"type": "boolean"}, hover={"type": "boolean"}), "creative_click"),
    "game_creative_release": ("Release the mouse button held by game_creative_click hold=true.", S(), "creative_release"),
    "game_anvil_type": ("Anvil screen with an item in slot 0: type into the rename field (clear=true erases first); returns name, cost, has result.", S(text=STR, clear={"type": "boolean"}), "anvil_type"),
    "game_creative_search": ("Creative screen, Search tab selected: type text into the search field.", S(text=STR), "creative_search"),
    "game_chat_open": ("Open the chat screen (text='/' like the command key).", S(text=STR), "chat_open"),
    "game_chat_type": ("Chat screen open: type text into the input box.", S(text=STR), "chat_type"),
    "game_chat_send": ("Send a chat line or /command (opens the screen if needed; text replaces the input). Recent lines are in game_state.chat.", S(text=STR), "chat_send"),
    "game_chat_key": ("Chat screen: key = up/down (history), pageup/pagedown (scroll), esc (close).", S(key=STR), "chat_key"),
    "game_click": ("Click a mouse button for ms milliseconds (default 100): left attacks/breaks, right uses/places.", S(button=STR, ms=INT), "click"),
    "game_visible": ("List entities inside the current camera view (type, dist (to centre), reach_dist (to nearest hitbox point), in_reach (<=3.0, can be hit), screen x/y 0..1; filters max_dist=24, type substring, items=false, limit=25; no block-occlusion). Call BEFORE game_screenshot to aim: if count_in_view is 0 or the target is missing, turn/move first instead of taking an empty screenshot.", S(max_dist=N, type=STR, items={"type": "boolean"}, limit=N), "visible"),
    "game_look": ("Set the view: yaw and pitch (-90 up .. 90 down) in degrees.", S(yaw=N, pitch=N), "look"),
    "game_render_mode": ("Switch compute <-> vanilla terrain path (like the renderMode key). on=true compute, false vanilla, omit = toggle.", S(on={"type": "boolean"}), "render_mode"),
    "game_turn": ("Turn the view by dyaw/dpitch degrees.", S(dyaw=N, dpitch=N), "turn"),
    "game_hotbar": ("Select hotbar slot 0-8.", S(slot=INT), "hotbar"),
    "game_sound": ("Play a sound event (e.g. block.stone.break) at the listener; state.audio.started/active shows engine activity.", S(event=STR, volume=N, pitch=N), "sound"),
    "game_inventory": ("Non-empty inventory slots with item names and counts.", S(), "inventory"),
    "game_target": ("What the crosshair points at: block (name, props, face, hit point, dist, in_reach), far_block (20 blocks), fluid, entity. Use before mining/placing.", S(), "target"),
    "game_block_at": ("Block at x,y,z (name, props like facing/open/waterlogged, loaded, air/solid/water/hazard). list=[[x,y,z],...] for many at once.", S(x=N, y=N, z=N, list={"type": "array", "items": {"type": "array", "items": N}}), "block_at"),
    "game_scan": ("Search blocks around you. find='log,diamond_ore' (substrings, comma separated) -> nearest matches with pos, dist, yaw_to/pitch_to (feed to game_look) and total; without find -> counts of block types in the radius. radius<=48 (default 16), limit default 30.", S(find=STR, radius=INT, limit=INT), "scan"),
    "game_slice": ("ASCII map of horizontal layers around you (default floor, feet, head = dy -1,0,1; or dy=[..] / ys=[abs y..]). North is the top row, east the right column, @ is you; legend included. This is how you SEE walls, holes, stairs, doors. radius<=24 (default 8).", S(radius=INT, dy={"type": "array", "items": INT}, ys={"type": "array", "items": INT}), "slice"),
    "game_topdown": ("ASCII top-down map: top block of every column plus a relief grid (floor height vs your feet: '.' same, digits higher, letters lower). Best for navigation/terrain overview. radius<=40 (default 12); up=4 blocks above your feet are considered (raise to see through roofs/canopy), down=14.", S(radius=INT, up=INT, down=INT), "topdown"),
    "game_entities": ("ALL entities nearby, not only on screen: id, type, name (players), dist (to centre), reach_dist (to nearest hitbox point), in_reach (<=3.0), dx/dy/dz, yaw_to/pitch_to (for game_look), turn (degrees to add with game_turn), in_fov, item stacks. Filters max_dist=32, type substring, items=true, limit=40.", S(max_dist=N, type=STR, items={"type": "boolean"}, limit=INT), "entities"),
    "game_look_at": ("Aim the camera at a block (x,y,z; optional face=up/down/north/south/west/east) or an entity id. Answer has clear (line of sight free?) and blocked_by: if not clear, the camera would show that blocker instead of the subject, so move first.", S(x=N, y=N, z=N, face=STR, entity=INT), "look_at"),
    "game_goto": ("Walk to block x,y,z using A* over the loaded world (steps up 1, jumps, drops <=3, swims, avoids lava/fire/cactus, replans when stuck). Blocks until arrival or failure with a reason. range>0 = stop within that many blocks; sprint=true; timeout_ms default 60000. x,y are the FEET block you want to stand in.", S(x=N, y=N, z=N, range=N, sprint={"type": "boolean"}, timeout_ms=INT), "goto"),
    "game_mine": ("Look at block x,y,z and hold left click until it is broken (within reach, else error). Select the right hotbar tool first. Reports if another block blocks the line of sight. timeout_ms default 20000.", S(x=N, y=N, z=N, timeout_ms=INT), "mine"),
    "game_use_block": ("Aim at block x,y,z (face=... aims at that face center) and right-click: place the held block against it, open doors/chests, press buttons. Returns the adjacent block state after.", S(x=N, y=N, z=N, face=STR, ms=INT), "use_block"),
    "game_attack": ("Walk toward, aim at and hit entity id N times (waits for attack cooldown). Reports hits and whether the target is gone.", S(id=INT, hits=INT, timeout_ms=INT), "attack"),
    "game_frame": ("ONE-CALL SCREENSHOT OF A SUBJECT for side-by-side comparison: picks a standing cell dist blocks (default 4) from block x,y,z or entity with a clear line of sight, walks there, aims at the subject, and returns the screenshot with the view report. Use this instead of guessing a camera pose. from_yaw chooses the side to shoot from (0 north side, 90 west side, 180 south, 270 east); face=... aims at a block face. Repeat the same call with the same args in the reference client to compare.", S(x=N, y=N, z=N, entity=INT, dist=N, from_yaw=N, face=STR, settle_ms=INT, timeout_ms=INT), None),
    "game_cancel": ("Abort the running goto/mine/use/attack task and release its keys.", S(), "cancel"),
    "game_tooltip": ("Tooltip lines (text + hex color) of the item in an inventory slot, computed by the client without hovering. slot = InventoryMenu index.", S(slot=INT), "tooltip"),
}


def call(name, args):
    _, _, target = TOOLS[name]
    return target(args) if callable(target) else send(dict(args, cmd=target))


def tools_list():
    out = [{"name": n, "description": d, "inputSchema": s} for n, (d, s, _) in TOOLS.items()]
    out.append({"name": "game_screenshot", "description": "Render the current frame and return it as an image PLUS a report: what the crosshair hits, entities in view, camera pose, and WARNINGS (staring at a wall, camera inside a block, flat/dark frame). If WARNINGS is not empty the shot is useless for comparison: re-aim with game_look_at/game_scan and shoot again. Aim first, then shoot.", "inputSchema": S()})
    out.append({"name": "game_command", "description": "Raw control command, e.g. {\"cmd\":\"state\"}. See crates/client/src/control.rs.", "inputSchema": {"type": "object", "properties": {"cmd": STR}, "required": ["cmd"], "additionalProperties": True}})
    return out


def handle(msg):
    method, mid = msg.get("method"), msg.get("id")
    if mid is None:
        return None
    if method == "initialize":
        res = {"protocolVersion": msg["params"].get("protocolVersion", "2024-11-05"), "capabilities": {"tools": {}}, "serverInfo": {"name": "ezzmc", "version": "0.1"}}
    elif method == "tools/list":
        res = {"tools": tools_list()}
    elif method == "tools/call":
        name, args = msg["params"]["name"], msg["params"].get("arguments") or {}
        if name == "game_screenshot":
            res = shot_result()
        elif name == "game_frame":
            res = frame(args)
        else:
            r = send(args) if name == "game_command" else call(name, args) if name in TOOLS else {"ok": False, "error": "unknown tool"}
            res = {"content": [{"type": "text", "text": json.dumps(r, ensure_ascii=False)}], "isError": not r.get("ok", False)}
    else:
        return {"jsonrpc": "2.0", "id": mid, "error": {"code": -32601, "message": "method not found"}}
    return {"jsonrpc": "2.0", "id": mid, "result": res}


def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            out = handle(json.loads(line))
        except Exception as e:  # keep serving
            out = {"jsonrpc": "2.0", "id": None, "error": {"code": -32603, "message": str(e)}}
        if out is not None:
            sys.stdout.write(json.dumps(out) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    main()
