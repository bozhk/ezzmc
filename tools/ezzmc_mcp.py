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


def launch(args):
    global proc
    if fh is not None and send({"cmd": "ping"}).get("ok"):
        return {"ok": True, "note": "already running"}
    if connect(0.5):
        return send({"cmd": "ping"})
    if not os.path.exists(BIN):
        return {"ok": False, "error": f"{BIN} missing: cargo build --profile fast -p ezzmc-client"}
    update_progress()
    cmd = [BIN, "--control", f"127.0.0.1:{PORT}", "--name", args.get("name") or os.environ.get("EZZMC_NAME", "EzzBot")]
    if args.get("addr"):
        cmd += ["--connect", args["addr"]]
    env = dict(os.environ, **{k: str(v) for k, v in (args.get("env") or {}).items()})
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


def S(**props):
    return {"type": "object", "properties": props}


N, STR, INT = {"type": "number"}, {"type": "string"}, {"type": "integer"}
KEYS = "w a s d space shift ctrl e q f 1-9, letters, esc, arrows"

TOOLS = {
    "game_launch": ("Start ezzmc with a control socket (or attach to a running one). addr joins a server right away, e.g. 127.0.0.1:25611; name is the nickname.", S(addr=STR, name=STR, env={"type": "object"}), launch),
    "game_stop": ("Quit the game.", S(), stop),
    "game_state": ("Screen (menu/play/inventory/pause), joined, position, velocity, yaw/pitch, on_ground, flying, game_mode, held slot, fps.", S(), "state"),
    "game_join_server": ("Join a server, e.g. 127.0.0.1:25611, then call game_wait_joined.", S(addr=STR), "join_server"),
    "game_wait_joined": ("Wait until the world is joined and the position is known.", S(timeout_ms=INT), "wait_joined"),
    "game_ui_click": ("Left-click a menu screen at window pixel x, y (menus only).", S(x={"type": "number"}, y={"type": "number"}), "ui_click"),
    "game_wait": ("Let the game run for ms milliseconds.", S(ms=INT), "wait"),
    "game_key": ("Press (down=true) or release (down=false) a key and keep it so.", S(key=STR, down={"type": "boolean"}), "key"),
    "game_tap": (f"Press a key for ms milliseconds (default 100) and release. Keys: {KEYS}.", S(key=STR, ms=INT), "tap"),
    "game_mouse": ("Press or release a mouse button (left/right); hold left to break blocks.", S(button=STR, down={"type": "boolean"}), "mouse"),
    "game_slot_click": ("Inventory screen open: click slot (left/right; hover=true only moves the cursor), optionally drag over slots in drag_to (QUICK_CRAFT).", S(slot=INT, button=STR, hover={"type": "boolean"}, drag_to={"type": "array", "items": {"type": "integer"}}), "slot_click"),
    "game_creative_scroll": ("Creative screen open: scroll the item grid by dy wheel notches (negative = down, like the mouse wheel).", S(dy=N), "creative_scroll"),
    "game_creative_click": ("Creative screen open: click one target — tab (0..), cell (grid 0..44), slot (InventoryMenu index, hotbar 36..44), trash=true, outside=true, or x/y GUI px; button left/right/middle; drag_to = slot indexes; hover=true only moves the cursor (tooltip checks).", S(tab=INT, cell=INT, slot=INT, trash={"type": "boolean"}, outside={"type": "boolean"}, x=N, y=N, button=STR, drag_to={"type": "array", "items": {"type": "integer"}}, hold={"type": "boolean"}, hover={"type": "boolean"}), "creative_click"),
    "game_creative_release": ("Release the mouse button held by game_creative_click hold=true.", S(), "creative_release"),
    "game_creative_search": ("Creative screen, Search tab selected: type text into the search field.", S(text=STR), "creative_search"),
    "game_chat_open": ("Open the chat screen (text='/' like the command key).", S(text=STR), "chat_open"),
    "game_chat_type": ("Chat screen open: type text into the input box.", S(text=STR), "chat_type"),
    "game_chat_send": ("Send a chat line or /command (opens the screen if needed; text replaces the input). Recent lines are in game_state.chat.", S(text=STR), "chat_send"),
    "game_chat_key": ("Chat screen: key = up/down (history), pageup/pagedown (scroll), esc (close).", S(key=STR), "chat_key"),
    "game_click": ("Click a mouse button for ms milliseconds (default 100): left attacks/breaks, right uses/places.", S(button=STR, ms=INT), "click"),
    "game_look": ("Set the view: yaw and pitch (-90 up .. 90 down) in degrees.", S(yaw=N, pitch=N), "look"),
    "game_turn": ("Turn the view by dyaw/dpitch degrees.", S(dyaw=N, dpitch=N), "turn"),
    "game_hotbar": ("Select hotbar slot 0-8.", S(slot=INT), "hotbar"),
    "game_sound": ("Play a sound event (e.g. block.stone.break) at the listener; state.audio.started/active shows engine activity.", S(event=STR, volume=N, pitch=N), "sound"),
    "game_inventory": ("Non-empty inventory slots with item names and counts.", S(), "inventory"),
    "game_tooltip": ("Tooltip lines (text + hex color) of the item in an inventory slot, computed by the client without hovering. slot = InventoryMenu index.", S(slot=INT), "tooltip"),
}


def call(name, args):
    _, _, target = TOOLS[name]
    return target(args) if callable(target) else send(dict(args, cmd=target))


def tools_list():
    out = [{"name": n, "description": d, "inputSchema": s} for n, (d, s, _) in TOOLS.items()]
    out.append({"name": "game_screenshot", "description": "Render the current frame and return it as an image.", "inputSchema": S()})
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
            r = send({"cmd": "screenshot"})
            if r.get("ok"):
                data = base64.b64encode(open(r["path"], "rb").read()).decode()
                content = [{"type": "image", "mimeType": "image/png", "data": data}]
            else:
                content = [{"type": "text", "text": json.dumps(r)}]
            res = {"content": content, "isError": not r.get("ok")}
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
