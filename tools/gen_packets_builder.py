#!/usr/bin/env python3
"""Таблица id пакетов для версий без reports/packets.json.
1.20.5/1.20.6: порядок addPacket в *Protocols.java, имена из create{Client,Server}bound("...") в *PacketTypes.java.
1.19.3–1.20.4 (`--legacy <ver-с-именами>`): порядок addPacket(Класс.class, ...) в ConnectionProtocol.java,
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
        # конец последнего состояния: NOT_REGISTERED (1.20.2+) или закрытие enum `   );` (≤1.20.1)
        tail = re.search(r'NOT_REGISTERED|^   \);', src[h.end():], re.M)
        last = h.end() + tail.start() if tail else len(src)
        end = heads[k + 1].start() if k + 1 < len(heads) else last
        st = enum[h.group(1)]
        part = src[h.start():end]
        if 'addPacket(PacketFlow.' in part:
            # ≤1.14.4: по вызову `addPacket(PacketFlow.X, Cls.class)` на пакет, id — порядок в потоке
            count = {}
            for p in re.finditer(r'addPacket\(PacketFlow\.(\w+),\s*([\w.]+)\.class', part):
                d = 'Clientbound' if p.group(1) == 'CLIENTBOUND' else 'Serverbound'
                c = p.group(2)
                n = by_cls.get((st, d, c)) or by_cls.get((None, d, c))
                if n is None:
                    n = snake(c)
                    print(f'  {st} {d} {c} → {n} (нет у донора)')
                rows.append((st, d, count.get(d, 0), n))
                count[d] = count.get(d, 0) + 1
            continue
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


# Имена Yarn (1.13.x, NetworkState.java) → канонические. Ключ — (состояние, направление, класс).
YARN_CLIENT_PLAY = """EntitySpawn:add_entity ExperienceOrbSpawn:add_experience_orb EntitySpawnGlobal:add_global_entity
MobSpawn:add_mob PaintingSpawn:add_painting PlayerSpawn:add_player EntityAnimation:animate StatsUpdate:award_stats
BlockBreakingProgress:block_destruction BlockEntityUpdate:block_entity_data BlockAction:block_event BlockUpdate:block_update
BossBar:boss_event Difficulty:change_difficulty ChatMessage:chat ChunkDeltaUpdate:chunk_blocks_update
CommandSuggestions:command_suggestions class_4376:commands ConfirmGuiAction:container_ack CloseScreen:container_close
OpenScreen:open_screen Inventory:container_set_content ScreenHandlerPropertyUpdate:container_set_data
ScreenHandlerSlotUpdate:container_set_slot ChunkLoadDistance:cooldown CustomPayload:custom_payload PlaySoundName:custom_sound
Disconnect:disconnect EntityStatus:entity_event class_4381:tag_query Explosion:explode ChunkUnload:forget_level_chunk
GameStateChange:game_event KeepAlive:keep_alive ChunkData:level_chunk WorldEvent:level_event Particle:level_particles
GameJoin:login MapUpdate:map_item_data Entity:move_entity MoveRelative:move_entity_pos
RotateAndMoveRelative:move_entity_pos_rot Rotate:move_entity_rot VehicleMove:move_vehicle SignEditorOpen:open_sign_editor
CraftRecipeResponse:place_ghost_recipe PlayerAbilities:player_abilities CombatEvent:player_combat PlayerList:player_info
class_4379:player_look_at PlayerPositionLook:player_position BedSleep:player_sleep RecipesUnlock:recipe
EntitiesDestroy:remove_entities RemoveEntityStatusEffect:remove_mob_effect ResourcePackSend:resource_pack PlayerRespawn:respawn
EntitySetHeadYaw:rotate_head SelectAdvancementTab:select_advancements_tab WorldBorder:set_border SetCameraEntity:set_camera
HeldItemChange:set_carried_item ScoreboardDisplay:set_display_objective EntityTrackerUpdate:set_entity_data
EntityAttach:set_entity_link EntityVelocityUpdate:set_entity_motion EntityEquipmentUpdate:set_equipped_item
ExperienceBarUpdate:set_experience HealthUpdate:set_health ScoreboardObjectiveUpdate:set_objective SetPassengers:set_passengers
Team:set_player_team ScoreboardPlayerUpdate:set_score PlayerSpawnPosition:set_spawn_position WorldTimeUpdate:set_time
Title:set_titles class_4380:stop_sound PlaySoundId:sound PlayerListHeader:tab_list
ChunkRenderDistanceCenter:take_item_entity EntityPosition:teleport_entity AdvancementUpdatePacket:update_advancements
EntityAttributes:update_attributes EntityStatusEffect:update_mob_effect class_4382:update_recipes class_4383:update_tags"""
YARN_SERVER_PLAY = """TeleportConfirm:accept_teleportation class_4384:block_entity_tag_query ChatMessage:chat
ClientStatus:client_command ClientSettings:client_information RequestCommandCompletions:command_suggestion
ConfirmGuiAction:container_ack ButtonClick:container_button_click ClickWindow:container_click GuiClose:container_close
CustomPayload:custom_payload class_4385:edit_book class_4386:entity_tag_query PlayerInteractEntity:interact
KeepAlive:keep_alive PlayerMove:move_player PositionOnly:move_player_pos Both:move_player_pos_rot LookOnly:move_player_rot
VehicleMove:move_vehicle SteerBoat:paddle_boat class_4387:pick_item CraftRecipeRequest:place_recipe
UpdatePlayerAbilities:player_abilities PlayerAction:player_action ClientCommand:player_command PlayerInput:player_input
CraftingBlockData:recipe_book_update class_4388:rename_item ResourcePackStatus:resource_pack AdvancementTab:seen_advancements
class_4389:select_trade class_4390:set_beacon UpdateSelectedSlot:set_carried_item class_4391:set_command_block
class_4392:set_command_minecart CreativeInventoryAction:set_creative_mode_slot class_4393:set_structure_block
UpdateSign:sign_update HandSwing:swing SpectatorTeleport:teleport_to_entity PlayerInteractBlock:use_item_on
SwingHand:use_item"""
YARN_OTHER = {
    ('Handshake', 'Serverbound'): 'HandshakeC2SPacket:intention',
    ('Status', 'Serverbound'): 'QueryRequest:status_request QueryPing:ping_request',
    ('Status', 'Clientbound'): 'QueryResponse:status_response QueryPong:pong_response',
    ('Login', 'Clientbound'): 'LoginDisconnect:login_disconnect LoginHello:hello LoginSuccess:game_profile '
                              'LoginCompression:login_compression class_4394:custom_query',
    ('Login', 'Serverbound'): 'LoginHello:hello LoginKey:key class_4396:custom_query',
}


def yarn_rows(ver):
    """NetworkState.java (Yarn, 1.13.x): `register(NetworkSide.X, Cls.class)`, id — порядок в потоке."""
    src = (pathlib.Path('reference') / ver / 'client/net/minecraft/network/NetworkState.java').read_text()
    heads = list(re.finditer(r'^   (HANDSHAKING|PLAY|STATUS|LOGIN)\(', src, re.M))
    enum = {'HANDSHAKING': 'Handshake', 'PLAY': 'Play', 'STATUS': 'Status', 'LOGIN': 'Login'}
    maps = {('Play', 'Clientbound'): YARN_CLIENT_PLAY, ('Play', 'Serverbound'): YARN_SERVER_PLAY, **YARN_OTHER}
    maps = {k: dict(x.split(':') for x in v.split()) for k, v in maps.items()}
    rows = []
    for k, h in enumerate(heads):
        st = enum[h.group(1)]
        part = src[h.start():heads[k + 1].start() if k + 1 < len(heads) else len(src)]
        count = {}
        for p in re.finditer(r'register\(NetworkSide\.(\w+),\s*([\w.]+)\.class', part):
            d = 'Clientbound' if p.group(1) == 'CLIENTBOUND' else 'Serverbound'
            c = p.group(2).split('.')[-1]
            n = maps[(st, d)].get(c) or maps[(st, d)].get(re.sub(r'(S2C|C2S)Packet$', '', c))
            if n is None:
                sys.exit(f'{st} {d} {c}: нет в таблице имён Yarn')
            rows.append((st, d, count.get(d, 0), n))
            count[d] = count.get(d, 0) + 1
    return rows


ver = sys.argv[1]
if '--yarn' in sys.argv:
    rows = yarn_rows(ver)
    src_note = 'NetworkState.java'
elif '--legacy' in sys.argv:
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
