//! Play: вход в мир (TODO-457) — Login, Respawn, Game Event, Set Default Spawn,
//! Player Position (+Teleport Confirm), Player Loaded, keep-alive/ping.
// VANILLA: ClientPacketListener#handleLogin/handleRespawn/handleGameEvent/handleMovePlayer (1.21.11)
// Формат сверен с reference/1.21.11 (ClientboundLoginPacket, CommonPlayerSpawnInfo, ...).
use crate::conn::Connection;
use crate::frame::Packet;
use crate::session::PacketIds;
use crate::{NetError, NetResult};
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::types::{BlockPos, Vec3};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, ProtocolResult, State};

#[derive(Clone, Debug, PartialEq)]
pub struct SpawnInfo {
    /// Индекс в реестре `minecraft:dimension_type` (порядок записей `registry_data`).
    pub dimension_type: i32,
    pub dimension: String,
    pub seed: i64,
    pub game_type: i8,
    /// -1 = нет
    pub previous_game_type: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death: Option<(String, BlockPos)>,
    pub portal_cooldown: i32,
    pub sea_level: i32,
}

impl SpawnInfo {
    // VANILLA: CommonPlayerSpawnInfo(RegistryFriendlyByteBuf)
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        Ok(Self {
            dimension_type: r.var_int()?,
            dimension: r.string(32767)?,
            seed: r.i64()?,
            game_type: r.i8()?,
            previous_game_type: r.i8()?,
            is_debug: r.bool()?,
            is_flat: r.bool()?,
            last_death: r.optional(|r| Ok((r.string(32767)?, BlockPos::read(r)?)))?,
            portal_cooldown: r.var_int()?,
            sea_level: r.var_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoginInfo {
    pub player_id: i32,
    pub hardcore: bool,
    pub levels: Vec<String>,
    pub max_players: i32,
    pub chunk_radius: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub show_death_screen: bool,
    pub do_limited_crafting: bool,
    pub spawn: SpawnInfo,
    pub enforces_secure_chat: bool,
}

impl LoginInfo {
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        let player_id = r.i32()?;
        let hardcore = r.bool()?;
        let n = r.length("levels", 1024)?;
        let levels = (0..n).map(|_| r.string(32767)).collect::<Result<_, _>>()?;
        Ok(Self {
            player_id,
            hardcore,
            levels,
            max_players: r.var_int()?,
            chunk_radius: r.var_int()?,
            simulation_distance: r.var_int()?,
            reduced_debug_info: r.bool()?,
            show_death_screen: r.bool()?,
            do_limited_crafting: r.bool()?,
            spawn: SpawnInfo::read(r)?,
            enforces_secure_chat: r.bool()?,
        })
    }
}

/// Относительные флаги Player Position (бит = `1 << ordinal`).
pub mod relative {
    pub const X: i32 = 1 << 0;
    pub const Y: i32 = 1 << 1;
    pub const Z: i32 = 1 << 2;
    pub const Y_ROT: i32 = 1 << 3;
    pub const X_ROT: i32 = 1 << 4;
    pub const DELTA_X: i32 = 1 << 5;
    pub const DELTA_Y: i32 = 1 << 6;
    pub const DELTA_Z: i32 = 1 << 7;
    pub const ROTATE_DELTA: i32 = 1 << 8;
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerPosition {
    pub teleport_id: i32,
    pub pos: Vec3,
    pub delta: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub relatives: i32,
}

impl PlayerPosition {
    pub fn read(r: &mut Reader) -> ProtocolResult<Self> {
        Ok(Self {
            teleport_id: r.var_int()?,
            pos: Vec3::read(r)?,
            delta: Vec3::read(r)?,
            yaw: r.f32()?,
            pitch: r.f32()?,
            relatives: r.i32()?,
        })
    }

    /// VANILLA: PositionMoveRotation#calculateAbsolute (без ROTATE_DELTA — скорость в M1 не нужна).
    pub fn absolute(&self, cur: &PlayerState) -> PlayerState {
        let rel = |bit: i32, c: f64| if self.relatives & bit != 0 { c } else { 0.0 };
        let d = |bit: i32, c: f64, v: f64| if self.relatives & bit != 0 { c + v } else { v };
        PlayerState {
            pos: Vec3::new(
                rel(relative::X, cur.pos.x) + self.pos.x,
                rel(relative::Y, cur.pos.y) + self.pos.y,
                rel(relative::Z, cur.pos.z) + self.pos.z,
            ),
            delta: Vec3::new(
                d(relative::DELTA_X, cur.delta.x, self.delta.x),
                d(relative::DELTA_Y, cur.delta.y, self.delta.y),
                d(relative::DELTA_Z, cur.delta.z, self.delta.z),
            ),
            yaw: if self.relatives & relative::Y_ROT != 0 {
                cur.yaw
            } else {
                0.0
            } + self.yaw,
            pitch: (if self.relatives & relative::X_ROT != 0 {
                cur.pitch
            } else {
                0.0
            } + self.pitch)
                .clamp(-90.0, 90.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlayerState {
    pub pos: Vec3,
    pub delta: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

/// Game Event: id и параметр. 13 = начать ждать чанки.
pub mod game_event {
    pub const NO_RESPAWN_BLOCK: u8 = 0;
    pub const START_RAINING: u8 = 1;
    pub const STOP_RAINING: u8 = 2;
    pub const CHANGE_GAME_MODE: u8 = 3;
    pub const RAIN_LEVEL_CHANGE: u8 = 7;
    pub const THUNDER_LEVEL_CHANGE: u8 = 8;
    pub const LEVEL_CHUNKS_LOAD_START: u8 = 13;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultSpawn {
    pub dimension: String,
    pub pos: BlockPos,
    pub yaw: f32,
    pub pitch: f32,
}

/// Событие Play для клиентского кода. Пакеты вне списка → `Other` (разберут другие модули).
#[derive(Debug)]
pub enum PlayEvent {
    Login(Box<LoginInfo>),
    Respawn {
        spawn: Box<SpawnInfo>,
        data_to_keep: u8,
    },
    GameEvent {
        event: u8,
        param: f32,
    },
    DefaultSpawn(DefaultSpawn),
    /// Телепорт уже подтверждён; `state` — новое абсолютное положение игрока.
    Teleport {
        teleport_id: i32,
        state: PlayerState,
    },
    /// VANILLA: LocalPlayer#handleEntityEvent 24..28 — уровень прав оператора (0..4).
    OpLevel(u8),
    /// VANILLA: LivingEntity#handleEntityEvent 9 (USE_ITEM_COMPLETE) для себя.
    UseComplete,
    /// VANILLA: handleSetEntityMotion для локального игрока (`Entity#lerpMotion`).
    Motion([f64; 3]),
    /// VANILLA: ClientboundPlayerAbilitiesPacket: флаги (1 invulnerable, 2 flying, 4 mayfly, 8 instabuild), скорости.
    Abilities {
        flags: u8,
        fly_speed: f32,
        walk_speed: f32,
    },
    Disconnect(String),
    Other(Packet),
}

/// VANILLA: LpVec3#read (скорость в 26.3).
pub fn read_lp_vec3(r: &mut Reader) -> ProtocolResult<[f64; 3]> {
    let lowest = r.u8()?;
    if lowest == 0 {
        return Ok([0.0; 3]);
    }
    let middle = u64::from(r.u8()?);
    let highest = u64::from(r.i32()? as u32);
    let buffer = highest << 16 | middle << 8 | u64::from(lowest);
    let mut scale = u64::from(lowest & 3);
    if lowest & 4 != 0 {
        scale |= u64::from(r.var_int()? as u32) << 2;
    }
    let unpack = |v: u64| ((v & 32767).min(32766) as f64) * 2.0 / 32766.0 - 1.0;
    let sc = scale as f64;
    Ok([
        unpack(buffer >> 3) * sc,
        unpack(buffer >> 18) * sc,
        unpack(buffer >> 33) * sc,
    ])
}

/// VANILLA: Input (ServerboundPlayerInputPacket) — биты флагов.
pub mod input_bits {
    pub const FORWARD: u8 = 1;
    pub const BACKWARD: u8 = 2;
    pub const LEFT: u8 = 4;
    pub const RIGHT: u8 = 8;
    pub const JUMP: u8 = 16;
    pub const SHIFT: u8 = 32;
    pub const SPRINT: u8 = 64;
}

/// Что именно слать серверу из `ServerboundMovePlayerPacket`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveKind {
    PosRot,
    Pos,
    Rot,
    StatusOnly,
}

/// VANILLA: LocalPlayer#sendPosition — выбор вида пакета (xLast/yRotLast/positionReminder/lastOnGround).
#[derive(Clone, Debug)]
pub struct MoveTracker {
    last_pos: [f64; 3],
    last_rot: [f32; 2],
    reminder: u32,
    last_ground: bool,
    last_hcol: bool,
}

impl MoveTracker {
    /// `pos`/`rot` — положение, уже известное серверу (после телепорта).
    pub fn new(pos: [f64; 3], rot: [f32; 2]) -> Self {
        Self {
            last_pos: pos,
            last_rot: rot,
            reminder: 0,
            last_ground: false,
            last_hcol: false,
        }
    }

    /// Телепорт: ванильный `ClientPacketListener#handleMovePlayer` сам обновляет xLast/yRotLast.
    pub fn reset(&mut self, pos: [f64; 3], rot: [f32; 2]) {
        self.last_pos = pos;
        self.last_rot = rot;
        self.reminder = 0;
    }

    pub fn step(
        &mut self,
        pos: [f64; 3],
        rot: [f32; 2],
        on_ground: bool,
        hcol: bool,
    ) -> Option<MoveKind> {
        let d = [
            pos[0] - self.last_pos[0],
            pos[1] - self.last_pos[1],
            pos[2] - self.last_pos[2],
        ];
        self.reminder += 1;
        let mv = d[0] * d[0] + d[1] * d[1] + d[2] * d[2] > 2.0e-4 * 2.0e-4 || self.reminder >= 20;
        let rt = f64::from(rot[0] - self.last_rot[0]) != 0.0
            || f64::from(rot[1] - self.last_rot[1]) != 0.0;
        let kind = if mv && rt {
            Some(MoveKind::PosRot)
        } else if mv {
            Some(MoveKind::Pos)
        } else if rt {
            Some(MoveKind::Rot)
        } else if self.last_ground != on_ground || self.last_hcol != hcol {
            Some(MoveKind::StatusOnly)
        } else {
            None
        };
        if mv {
            self.last_pos = pos;
            self.reminder = 0;
        }
        if rt {
            self.last_rot = rot;
        }
        self.last_ground = on_ground;
        self.last_hcol = hcol;
        kind
    }
}

/// Обработка одного пакета Play: автоответы + разбор базовых событий.
pub struct PlayHandler {
    ids: PacketIds,
    pub player: PlayerState,
    /// Id сущности локального игрока (из Login).
    pub player_id: i32,
}

impl PlayHandler {
    pub fn new(ids: PacketIds) -> Self {
        Self {
            ids,
            player: PlayerState::default(),
            player_id: 0,
        }
    }

    fn sid(&self, name: &str) -> NetResult<i32> {
        self.ids.get(State::Play, Direction::Serverbound, name)
    }

    pub fn handle(&mut self, conn: &mut Connection, p: Packet) -> NetResult<PlayEvent> {
        let name = self.ids.name_of(State::Play, Direction::Clientbound, p.id);
        let mut r = Reader::new(&p.body);
        Ok(match name {
            Some("login") => {
                let l = LoginInfo::read(&mut r)?;
                self.player_id = l.player_id;
                PlayEvent::Login(Box::new(l))
            }
            Some("player_abilities") => PlayEvent::Abilities {
                flags: r.u8()?,
                fly_speed: r.f32()?,
                walk_speed: r.f32()?,
            },
            Some("set_entity_motion") => {
                let id = r.var_int()?;
                let v = read_lp_vec3(&mut r)?;
                if id == self.player_id {
                    PlayEvent::Motion(v)
                } else {
                    PlayEvent::Other(p)
                }
            }
            Some("entity_event") => {
                let id = r.i32()?;
                let ev = r.u8()?;
                if id == self.player_id && (24..=28).contains(&ev) {
                    PlayEvent::OpLevel(ev - 24)
                } else if id == self.player_id && ev == 9 {
                    PlayEvent::UseComplete
                } else {
                    PlayEvent::Other(p)
                }
            }
            Some("respawn") => PlayEvent::Respawn {
                spawn: Box::new(SpawnInfo::read(&mut r)?),
                data_to_keep: r.u8()?,
            },
            Some("game_event") => PlayEvent::GameEvent {
                event: r.u8()?,
                param: r.f32()?,
            },
            Some("set_default_spawn_position") => PlayEvent::DefaultSpawn(DefaultSpawn {
                dimension: r.string(32767)?,
                pos: BlockPos::read(&mut r)?,
                yaw: r.f32()?,
                pitch: r.f32()?,
            }),
            Some("player_position") => {
                let pp = PlayerPosition::read(&mut r)?;
                self.player = pp.absolute(&self.player);
                // VANILLA: ClientPacketListener#handleMovePlayer (26.3): accept_teleportation(id, x, y, z, yRot, xRot),

                let mut w = Writer::new();
                w.var_int(pp.teleport_id)
                    .f64(self.player.pos.x)
                    .f64(self.player.pos.y)
                    .f64(self.player.pos.z)
                    .f32(self.player.yaw)
                    .f32(self.player.pitch);
                conn.send(self.sid("accept_teleportation")?, &w.buf)?;
                // 26.3: сервер применяет позицию из accept_teleportation сам; второй пакет позиции в тот же тик = kick.
                PlayEvent::Teleport {
                    teleport_id: pp.teleport_id,
                    state: self.player,
                }
            }
            Some("keep_alive") => {
                conn.send(self.sid("keep_alive")?, &p.body)?;
                PlayEvent::Other(p)
            }
            Some("ping") => {
                conn.send(self.sid("pong")?, &p.body)?;
                PlayEvent::Other(p)
            }
            Some("disconnect") => {
                let comp = nbt::read_network(&mut r)
                    .ok()
                    .flatten()
                    .and_then(|t| Component::from_nbt(&t).ok());
                PlayEvent::Disconnect(
                    comp.map(|c| c.to_plain(&|_| None))
                        .unwrap_or_else(|| "<нечитаемая причина>".into()),
                )
            }
            _ => PlayEvent::Other(p),
        })
    }

    /// VANILLA: ServerboundMovePlayerPacket.{PosRot,Pos,Rot,StatusOnly}; флаги: бит0 on_ground, бит1 hcol.
    pub fn send_move(
        &self,
        conn: &mut Connection,
        kind: MoveKind,
        on_ground: bool,
        hcol: bool,
    ) -> NetResult<()> {
        let flags = u8::from(on_ground) | (u8::from(hcol) << 1);
        let mut w = Writer::new();
        let name = match kind {
            MoveKind::PosRot => {
                w.f64(self.player.pos.x)
                    .f64(self.player.pos.y)
                    .f64(self.player.pos.z)
                    .f32(self.player.yaw)
                    .f32(self.player.pitch);
                "move_player_pos_rot"
            }
            MoveKind::Pos => {
                w.f64(self.player.pos.x)
                    .f64(self.player.pos.y)
                    .f64(self.player.pos.z);
                "move_player_pos"
            }
            MoveKind::Rot => {
                w.f32(self.player.yaw).f32(self.player.pitch);
                "move_player_rot"
            }
            MoveKind::StatusOnly => "move_player_status_only",
        };
        w.u8(flags);
        conn.send(self.sid(name)?, &w.buf)
    }

    /// VANILLA: ServerboundPaddleBoatPacket(left, right).
    pub fn send_paddle_boat(
        &self,
        conn: &mut Connection,
        left: bool,
        right: bool,
    ) -> NetResult<()> {
        conn.send(self.sid("paddle_boat")?, &[u8::from(left), u8::from(right)])
    }

    /// VANILLA: ServerboundMoveVehiclePacket(x, y, z, yRot, xRot, onGround).
    pub fn send_move_vehicle(
        &self,
        conn: &mut Connection,
        pos: [f64; 3],
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.f64(pos[0])
            .f64(pos[1])
            .f64(pos[2])
            .f32(yaw)
            .f32(pitch)
            .u8(u8::from(on_ground));
        conn.send(self.sid("move_vehicle")?, &w.buf)
    }

    /// VANILLA: ServerboundPlayerInputPacket (байт флагов `input_bits`).
    pub fn send_player_input(&self, conn: &mut Connection, bits: u8) -> NetResult<()> {
        conn.send(self.sid("player_input")?, &[bits])
    }

    /// VANILLA: ServerboundPlayerActionPacket: action, позиция, грань, sequence.
    pub fn send_player_action(
        &self,
        conn: &mut Connection,
        action: i32,
        pos: [i32; 3],
        face: u8,
        seq: i32,
    ) -> NetResult<()> {
        // VANILLA: 26.x — в ServerboundPlayerActionPacket.Action после START_DESTROY_BLOCK вставлен
        // CHANGE_DESTROY_DIRECTION (1): все дальнейшие номера +1. Клиент оперирует старой нумерацией
        // (0 start, 1 abort, 2 stop, 3 drop all, 4 drop, 5 release, 6 swap); маркер 26.x — пакет `punch`.
        let action = if action >= 1 && self.sid("punch").is_ok() {
            action + 1
        } else {
            action
        };
        let mut w = Writer::new();
        w.var_int(action)
            .block_pos(pos[0], pos[1], pos[2])
            .u8(face)
            .var_int(seq);
        conn.send(self.sid("player_action")?, &w.buf)
    }

    /// VANILLA: ServerboundPickItemFromBlockPacket(BlockPos, includeData) (1.21.5+).
    pub fn send_pick_item_from_block(
        &self,
        conn: &mut Connection,
        pos: [i32; 3],
        include_data: bool,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.block_pos(pos[0], pos[1], pos[2])
            .u8(u8::from(include_data));
        conn.send(self.sid("pick_item_from_block")?, &w.buf)
    }

    /// VANILLA: ServerboundUseItemOnPacket(MAIN_HAND, BlockHitResult, sequence); `rel` — точка попадания внутри блока.
    pub fn send_use_item_on(
        &self,
        conn: &mut Connection,
        pos: [i32; 3],
        face: u8,
        rel: [f32; 3],
        inside: bool,
        seq: i32,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(0)
            .block_pos(pos[0], pos[1], pos[2])
            .var_int(i32::from(face))
            .f32(rel[0])
            .f32(rel[1])
            .f32(rel[2])
            .bool(inside)
            .bool(false)
            .var_int(seq);
        conn.send(self.sid("use_item_on")?, &w.buf)
    }

    /// VANILLA: ServerboundUseItemPacket(hand, sequence, yRot, xRot).
    pub fn send_use_item(
        &self,
        conn: &mut Connection,
        seq: i32,
        yaw: f32,
        pitch: f32,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(0).var_int(seq).f32(yaw).f32(pitch);
        conn.send(self.sid("use_item")?, &w.buf)
    }

    /// VANILLA: ServerboundAttackPacket(entityId).
    pub fn send_attack(&self, conn: &mut Connection, entity_id: i32) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(entity_id);
        conn.send(self.sid("attack")?, &w.buf)
    }

    /// VANILLA: ServerboundSetCarriedItemPacket(short slot 0–8).
    pub fn send_set_carried_item(&self, conn: &mut Connection, slot: i16) -> NetResult<()> {
        let mut w = Writer::new();
        w.i16(slot);
        conn.send(self.sid("set_carried_item")?, &w.buf)
    }

    /// VANILLA: ServerboundContainerClickPacket(containerId, stateId, slot, button, input,
    /// changedSlots: Int2Object<HashedStack>, carried: HashedStack).
    /// `HashedStack` = Optional(item varint, count varint, HashedPatchMap); хэши компонентов не считаем —
    /// пустая карта (для стека с компонентами сервер заметит расхождение и пришлёт состояние).
    #[allow(clippy::too_many_arguments)]
    pub fn send_container_click(
        &self,
        conn: &mut Connection,
        container: i32,
        state_id: i32,
        slot: i16,
        button: i8,
        input: i32,
        changed: &[(i16, Option<(u32, i32)>)],
        carried: Option<(u32, i32)>,
    ) -> NetResult<()> {
        fn hashed(w: &mut Writer, s: Option<(u32, i32)>) {
            match s {
                None => {
                    w.u8(0);
                }
                Some((item, count)) => {
                    w.u8(1)
                        .var_int(item as i32)
                        .var_int(count)
                        .var_int(0)
                        .var_int(0);
                }
            }
        }
        let mut w = Writer::new();
        w.var_int(container)
            .var_int(state_id)
            .i16(slot)
            .u8(button as u8)
            .var_int(input)
            .var_int(changed.len() as i32);
        for (s, st) in changed {
            w.i16(*s);
            hashed(&mut w, *st);
        }
        hashed(&mut w, carried);
        conn.send(self.sid("container_click")?, &w.buf)
    }

    /// VANILLA: ServerboundSetCreativeModeSlotPacket(short slot, OPTIONAL_UNTRUSTED stack).
    pub fn send_set_creative_slot(
        &self,
        conn: &mut Connection,
        slot: i16,
        stack: Option<&ezzmc_protocol::item::ItemStack>,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.i16(slot);
        ezzmc_protocol::item::write_stack_untrusted(&mut w, stack);
        conn.send(self.sid("set_creative_mode_slot")?, &w.buf)
    }

    /// VANILLA: ServerboundContainerClosePacket(container id).
    pub fn send_container_close(&self, conn: &mut Connection, container: i32) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(container);
        conn.send(self.sid("container_close")?, &w.buf)
    }

    /// VANILLA: ServerboundClientCommandPacket(PERFORM_RESPAWN = 0).
    pub fn send_respawn(&self, conn: &mut Connection) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(0);
        conn.send(self.sid("client_command")?, &w.buf)
    }

    /// VANILLA: ServerboundSwingPacket(MAIN_HAND = 0).
    /// VANILLA: 26.x — ServerboundPunchPacket (пустой; `swing` переименован).
    /// VANILLA: ServerboundClientInformationPacket (Play) — при смене настроек.
    pub fn send_client_information(
        &self,
        conn: &mut Connection,
        info: &crate::session::ClientInformation,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        info.write(&mut w);
        conn.send(self.sid("client_information")?, &w.buf)
    }

    pub fn send_swing(&self, conn: &mut Connection) -> NetResult<()> {
        if let Ok(id) = self.sid("punch") {
            return conn.send(id, &[]);
        }
        let mut w = Writer::new();
        w.var_int(0);
        conn.send(self.sid("swing")?, &w.buf)
    }

    /// VANILLA: ClientPacketListener#sendChat без профильного ключа — ServerboundChatPacket
    /// (message, timestamp, salt, signature = нет, LastSeenMessages.Update{offset 0, bitset, checksum 0}).
    pub fn send_chat(&self, conn: &mut Connection, msg: &str, ms: i64) -> NetResult<()> {
        let mut w = Writer::new();
        w.string(msg).i64(ms).i64(0).bool(false);
        w.var_int(0).bytes(&[0, 0, 0]).u8(0);
        conn.send(self.sid("chat")?, &w.buf)
    }

    /// VANILLA: ClientPacketListener#sendCommand — команда без `/`. Без дерева Brigadier всегда
    /// `chat_command_signed` с пустыми подписями (сервер с `enforce-secure-chat=false` принимает; обычный
    /// `chat_command` отвергается для команд с подписываемыми аргументами: say/msg/me).
    pub fn send_chat_command(&self, conn: &mut Connection, cmd: &str, ms: i64) -> NetResult<()> {
        let mut w = Writer::new();
        w.string(cmd).i64(ms).i64(0).var_int(0);
        w.var_int(0).bytes(&[0, 0, 0]).u8(0);
        conn.send(self.sid("chat_command_signed")?, &w.buf)
    }

    /// VANILLA: ServerboundCommandSuggestionPacket — id запроса и вся строка ввода (с `/`).
    pub fn send_command_suggestion(
        &self,
        conn: &mut Connection,
        id: i32,
        text: &str,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(id).string(text);
        conn.send(self.sid("command_suggestion")?, &w.buf)
    }

    /// VANILLA: ServerboundPlayerAbilitiesPacket — байт флагов, бит1 = летит.
    pub fn send_abilities(&self, conn: &mut Connection, flying: bool) -> NetResult<()> {
        conn.send(self.sid("player_abilities")?, &[u8::from(flying) << 1])
    }

    /// VANILLA: ServerboundPlayerCommandPacket(START/STOP_SPRINTING): entity id, action (1/2), data 0.
    pub fn send_sprint(&self, conn: &mut Connection, sprinting: bool) -> NetResult<()> {
        let mut w = Writer::new();
        w.var_int(self.player_id)
            .var_int(if sprinting { 1 } else { 2 })
            .var_int(0);
        conn.send(self.sid("player_command")?, &w.buf)
    }

    /// VANILLA: Minecraft#tick → ServerboundClientTickEndPacket в конце каждого клиентского тика (сбрасывает
    /// receivedPositionThisTick на сервере; без него второй пакет позиции = kick invalid_player_movement).
    pub fn send_client_tick_end(&self, conn: &mut Connection) -> NetResult<()> {
        conn.send(self.sid("client_tick_end")?, &[])
    }

    /// VANILLA: после закрытия экрана загрузки (чанк под игроком получен) — Player Loaded.
    pub fn send_player_loaded(&self, conn: &mut Connection) -> NetResult<()> {
        conn.send(self.sid("player_loaded")?, &[])
    }

    /// Ответ на Chunk Batch Finished: желаемое число чанков за тик (VANILLA: ChunkBatchSizeCalculator).
    pub fn send_chunk_batch_received(
        &self,
        conn: &mut Connection,
        chunks_per_tick: f32,
    ) -> NetResult<()> {
        let mut w = Writer::new();
        w.f32(chunks_per_tick);
        conn.send(self.sid("chunk_batch_received")?, &w.buf)
    }
}

/// Удобство: блокирующе читать Play, пока не придёт событие, не `Other`.
pub fn next_event(conn: &mut Connection, h: &mut PlayHandler) -> NetResult<PlayEvent> {
    loop {
        let p = conn.recv()?;
        match h.handle(conn, p)? {
            PlayEvent::Other(_) => {}
            PlayEvent::Disconnect(t) => return Err(NetError::Disconnected(t)),
            e => return Ok(e),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn relative_position_math() {
        let cur = PlayerState {
            pos: Vec3::new(10.0, 64.0, -5.0),
            delta: Vec3::new(1.0, 0.0, 0.0),
            yaw: 90.0,
            pitch: 80.0,
        };
        let pp = PlayerPosition {
            teleport_id: 1,
            pos: Vec3::new(1.0, 2.0, 3.0),
            delta: Vec3::new(0.5, 0.0, 0.0),
            yaw: 10.0,
            pitch: 30.0,
            relatives: relative::X | relative::Z | relative::X_ROT | relative::DELTA_X,
        };
        let a = pp.absolute(&cur);
        assert_eq!(a.pos, Vec3::new(11.0, 2.0, -2.0));
        assert_eq!(a.yaw, 10.0);
        assert_eq!(a.pitch, 90.0); // зажато
        assert_eq!(a.delta.x, 1.5);
    }

    #[test]
    fn move_tracker_kinds() {
        let mut t = MoveTracker::new([0.0; 3], [0.0; 2]);
        assert_eq!(t.step([0.0; 3], [0.0; 2], false, false), None);
        assert_eq!(
            t.step([0.0; 3], [0.0; 2], true, false),
            Some(MoveKind::StatusOnly)
        );
        assert_eq!(
            t.step([0.0; 3], [5.0, 0.0], true, false),
            Some(MoveKind::Rot)
        );
        assert_eq!(
            t.step([1.0, 0.0, 0.0], [5.0, 0.0], true, false),
            Some(MoveKind::Pos)
        );
        assert_eq!(
            t.step([2.0, 0.0, 0.0], [6.0, 0.0], true, false),
            Some(MoveKind::PosRot)
        );
        // heartbeat раз в 20 тиков
        let mut kinds = 0;
        for _ in 0..40 {
            if t.step([2.0, 0.0, 0.0], [6.0, 0.0], true, false) == Some(MoveKind::Pos) {
                kinds += 1;
            }
        }
        assert_eq!(kinds, 2);
    }

    #[test]
    fn lp_vec3_zero_and_basic() {
        assert_eq!(read_lp_vec3(&mut Reader::new(&[0])).unwrap(), [0.0; 3]);
        // scale=1, x=y=z=32766 → +1.0 каждая
        let q: u64 = 32766;
        let buf = (q << 33) | (q << 18) | (q << 3) | 1;
        let hi = ((buf >> 16) as u32).to_be_bytes();
        let bytes = [buf as u8, (buf >> 8) as u8, hi[0], hi[1], hi[2], hi[3]];
        let v = read_lp_vec3(&mut Reader::new(&bytes)).unwrap();
        assert_eq!(v, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn spawn_info_roundtrip_bytes() {
        let mut w = Writer::new();
        w.var_int(3)
            .string("minecraft:overworld")
            .i64(42)
            .u8(1)
            .u8(0xff)
            .bool(false)
            .bool(true);
        w.bool(true)
            .string("minecraft:the_nether")
            .block_pos(1, 2, 3);
        w.var_int(0).var_int(63);
        let s = SpawnInfo::read(&mut Reader::new(&w.buf)).unwrap();
        assert_eq!(s.dimension_type, 3);
        assert_eq!(s.previous_game_type, -1);
        assert!(s.is_flat);
        assert_eq!(
            s.last_death,
            Some(("minecraft:the_nether".into(), BlockPos::new(1, 2, 3)))
        );
        assert_eq!(s.sea_level, 63);
        assert!(SpawnInfo::read(&mut Reader::new(&w.buf[..10])).is_err());
    }
}
