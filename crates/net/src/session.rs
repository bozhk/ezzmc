//! Вход на сервер: Login (offline) и Configuration (TODO-070, 080–084, 059, 062).
// VANILLA: ClientHandshakePacketListenerImpl, ClientConfigurationPacketListenerImpl (1.21.11)
// Формат пакетов сверен с reference/1.21.11 (network/protocol/{login,configuration,common,cookie}).
use crate::conn::Connection;
use crate::{NetError, NetResult};
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::text::Component;
use ezzmc_protocol::types::{offline_uuid, GameProfile};
use ezzmc_protocol::wire::{Reader, Writer};
use ezzmc_protocol::{Direction, PacketInfo, ProtocolError, ProtocolResult, State};

/// Таблица id пакетов по имени для выбранной версии.
#[derive(Clone, Copy)]
pub struct PacketIds {
    table: &'static [PacketInfo],
}

impl PacketIds {
    pub fn new(table: &'static [PacketInfo]) -> Self {
        Self { table }
    }
    pub fn v26_3() -> Self {
        Self::new(ezzmc_protocol::generated::v26_3::PACKETS)
    }
    pub fn for_version(v: &ezzmc_protocol::version::Version) -> Self {
        Self::new(v.packets)
    }
    pub fn get(&self, state: State, dir: Direction, name: &str) -> NetResult<i32> {
        self.table
            .iter()
            .find(|p| p.state == state && p.dir == dir && p.name == name)
            .map(|p| p.id as i32)
            .ok_or_else(|| {
                ProtocolError::Other(format!("нет пакета {state:?}/{dir:?}/{name}")).into()
            })
    }
    pub fn name_of(&self, state: State, dir: Direction, id: i32) -> Option<&'static str> {
        self.table
            .iter()
            .find(|p| p.state == state && p.dir == dir && p.id as i32 == id)
            .map(|p| p.name)
    }
}

/// VANILLA: ClientInformation.
#[derive(Clone, Debug)]
pub struct ClientInformation {
    pub language: String,
    pub view_distance: i8,
    /// 0 = full, 1 = system, 2 = hidden
    pub chat_visibility: i32,
    pub chat_colors: bool,
    pub model_customisation: u8,
    /// 0 = left, 1 = right
    pub main_hand: i32,
    pub text_filtering: bool,
    pub allows_listing: bool,
    /// 0 = all, 1 = decreased, 2 = minimal
    pub particle_status: i32,
}

impl Default for ClientInformation {
    // VANILLA: ClientInformation#createDefault (но видимое «en_us» и 0x7F как в Options по умолчанию)
    fn default() -> Self {
        Self {
            language: "en_us".into(),
            view_distance: 8,
            chat_visibility: 0,
            chat_colors: true,
            model_customisation: 0x7F,
            main_hand: 1,
            text_filtering: false,
            allows_listing: false,
            particle_status: 0,
        }
    }
}

impl ClientInformation {
    pub fn write(&self, w: &mut Writer) {
        w.string(&self.language)
            .u8(self.view_distance as u8)
            .var_int(self.chat_visibility)
            .bool(self.chat_colors)
            .u8(self.model_customisation)
            .var_int(self.main_hand)
            .bool(self.text_filtering)
            .bool(self.allows_listing)
            .var_int(self.particle_status);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegistryData {
    pub registry: String,
    /// (id записи, данные NBT; None = из известного пака)
    pub entries: Vec<(String, Option<Tag>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagSet {
    pub registry: String,
    /// (имя тега, protocol_id элементов)
    pub tags: Vec<(String, Vec<i32>)>,
}

/// Всё, что сервер сообщил в Configuration.
#[derive(Clone, Debug, Default)]
pub struct ServerConfig {
    pub brand: Option<String>,
    pub registries: Vec<RegistryData>,
    pub tag_sets: Vec<TagSet>,
    pub enabled_features: Vec<String>,
    pub server_known_packs: Vec<KnownPack>,
}

impl ServerConfig {
    /// Входит ли элемент (protocol_id) в тег реестра, например `("minecraft:block", "minecraft:logs", id)`.
    pub fn tag_contains(&self, registry: &str, tag: &str, id: i32) -> bool {
        self.tag_sets
            .iter()
            .find(|t| t.registry == registry)
            .and_then(|t| t.tags.iter().find(|(n, _)| n == tag))
            .is_some_and(|(_, ids)| ids.contains(&id))
    }
}

#[derive(Debug)]
pub enum ConfigOutcome {
    /// Configuration завершён, соединение в состоянии Play.
    Play(Box<ServerConfig>),
    /// Сервер просит перейти на другой адрес (Transfer).
    Transfer { host: String, port: u16 },
}

// ---------- разбор пакетов ----------

/// Component из причины отключения Configuration/Play (сетевой NBT).
fn disconnect_text(body: &[u8]) -> String {
    let mut r = Reader::new(body);
    let comp = nbt::read_network(&mut r)
        .ok()
        .flatten()
        .and_then(|t| Component::from_nbt(&t).ok());
    comp.map(|c| c.to_plain(&|_| None))
        .unwrap_or_else(|| "<нечитаемая причина>".into())
}

pub fn parse_registry_data(body: &[u8]) -> ProtocolResult<RegistryData> {
    let mut r = Reader::new(body);
    let registry = r.string(32767)?;
    let n = r.length("registry entries", 65536)?;
    let mut entries = Vec::with_capacity(n.min(r.remaining()));
    for _ in 0..n {
        let id = r.string(32767)?;
        let data = r.optional(|r| {
            nbt::read_network(r)?.ok_or(ProtocolError::InvalidValue {
                what: "registry entry nbt end",
                value: 0,
            })
        })?;
        entries.push((id, data));
    }
    Ok(RegistryData { registry, entries })
}

pub fn parse_update_tags(body: &[u8]) -> ProtocolResult<Vec<TagSet>> {
    let mut r = Reader::new(body);
    let n = r.length("tag registries", 1024)?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let registry = r.string(32767)?;
        let m = r.length("tags", 65536)?;
        let mut tags = Vec::with_capacity(m.min(r.remaining()));
        for _ in 0..m {
            let name = r.string(32767)?;
            let k = r.length("tag ids", 1 << 20)?;
            let mut ids = Vec::with_capacity(k.min(r.remaining()));
            for _ in 0..k {
                ids.push(r.var_int()?);
            }
            tags.push((name, ids));
        }
        out.push(TagSet { registry, tags });
    }
    Ok(out)
}

pub fn parse_known_packs(body: &[u8]) -> ProtocolResult<Vec<KnownPack>> {
    let mut r = Reader::new(body);
    let n = r.length("known packs", 64)?;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(KnownPack {
            namespace: r.string(32767)?,
            id: r.string(32767)?,
            version: r.string(32767)?,
        });
    }
    Ok(v)
}

pub fn parse_features(body: &[u8]) -> ProtocolResult<Vec<String>> {
    let mut r = Reader::new(body);
    let n = r.length("features", 4096)?;
    (0..n).map(|_| r.string(32767)).collect()
}

// ---------- Login ----------

/// Offline-вход: Login Start → (compression) → Login Finished → Login Acknowledged.
/// Состояние соединения после вызова — Configuration.
pub fn login_offline(conn: &mut Connection, ids: &PacketIds, name: &str) -> NetResult<GameProfile> {
    login(conn, ids, name, offline_uuid(name), None)
}

/// Онлайн-вход (DEC-016): настоящий UUID, на Encryption Request — session/join и AES/CFB8.
pub fn login_online(
    conn: &mut Connection,
    ids: &PacketIds,
    session: &ezzmc_auth::GameSession,
) -> NetResult<GameProfile> {
    let uuid = u128::from_str_radix(&session.uuid, 16)
        .map_err(|_| NetError::Disconnected("некорректный UUID сессии".into()))?;
    login(conn, ids, &session.name, uuid, Some(session))
}

// VANILLA: ClientHandshakePacketListenerImpl#handleHello
fn answer_hello(
    conn: &mut Connection,
    ids: &PacketIds,
    body: &[u8],
    session: &ezzmc_auth::GameSession,
) -> NetResult<()> {
    let fail = |m: String| NetError::Disconnected(m);
    let mut r = Reader::new(body);
    let server_id = r.string(20)?;
    let n = r.length("public key", 4096)?;
    let key = r.bytes(n)?.to_vec();
    let n = r.length("challenge", 4096)?;
    let challenge = r.bytes(n)?.to_vec();
    let should_auth = r.bool().unwrap_or(true);
    let secret = crate::crypt::random_secret();
    if should_auth {
        let hash = ezzmc_auth::session::server_hash(&server_id, &secret, &key);
        ezzmc_auth::session::join_server(&session.access_token, &session.uuid, &hash)
            .map_err(|e| fail(format!("вход в сессию Mojang: {e}")))?;
    }
    let enc_secret = crate::crypt::rsa_encrypt(&key, &secret).map_err(fail)?;
    let enc_chal = crate::crypt::rsa_encrypt(&key, &challenge).map_err(fail)?;
    let mut w = Writer::new();
    w.var_int(enc_secret.len() as i32).bytes(&enc_secret);
    w.var_int(enc_chal.len() as i32).bytes(&enc_chal);
    conn.send(
        ids.get(State::Login, Direction::Serverbound, "key")?,
        &w.buf,
    )?;
    conn.enable_encryption(&secret);
    Ok(())
}

fn login(
    conn: &mut Connection,
    ids: &PacketIds,
    name: &str,
    uuid: u128,
    online: Option<&ezzmc_auth::GameSession>,
) -> NetResult<GameProfile> {
    use Direction::{Clientbound as C, Serverbound as S};
    let mut w = Writer::new();
    w.string(name).uuid(uuid);
    conn.send(ids.get(State::Login, S, "hello")?, &w.buf)?;
    loop {
        let p = conn.recv()?;
        let Some(n) = ids.name_of(State::Login, C, p.id) else {
            return Err(ProtocolError::UnknownPacket {
                state: State::Login,
                id: p.id as u32,
            }
            .into());
        };
        match n {
            "login_disconnect" => {
                // JSON-строка (lenientJson), не NBT
                let mut r = Reader::new(&p.body);
                let s = r.string(262144)?;
                let text = Component::from_json_str(&s)
                    .map(|c| c.to_plain(&|_| None))
                    .unwrap_or(s);
                return Err(NetError::Disconnected(text));
            }
            "hello" => match online {
                Some(sess) => answer_hello(conn, ids, &p.body, sess)?,
                None => {
                    return Err(NetError::Disconnected(
                        "сервер в online-mode: нужен вход через лаунчер (Microsoft)".into(),
                    ))
                }
            },
            "login_compression" => conn.set_compression(Reader::new(&p.body).var_int()?),
            "login_finished" => {
                let profile = GameProfile::read(&mut Reader::new(&p.body))?;
                conn.send(ids.get(State::Login, S, "login_acknowledged")?, &[])?;
                conn.set_state(State::Configuration)?;
                return Ok(profile);
            }
            "custom_query" => {
                // Моды/прокси спрашивают канал — отвечаем «не понимаю» (payload = null)
                let tx = Reader::new(&p.body).var_int()?;
                let mut w = Writer::new();
                w.var_int(tx).bool(false);
                conn.send(ids.get(State::Login, S, "custom_query_answer")?, &w.buf)?;
            }
            "cookie_request" => {
                let key = Reader::new(&p.body).string(32767)?;
                let mut w = Writer::new();
                w.string(&key).bool(false);
                conn.send(ids.get(State::Login, S, "cookie_response")?, &w.buf)?;
            }
            _ => {}
        }
    }
}

// ---------- Configuration ----------

/// Пройти Configuration до `finish_configuration` (или Transfer).
pub fn configure(
    conn: &mut Connection,
    ids: &PacketIds,
    info: &ClientInformation,
) -> NetResult<ConfigOutcome> {
    use Direction::{Clientbound as C, Serverbound as S};
    const ST: State = State::Configuration;
    // VANILLA: после Login Finished клиент шлёт brand и ClientInformation
    let mut w = Writer::new();
    w.string("minecraft:brand").string("vanilla");
    conn.send(ids.get(ST, S, "custom_payload")?, &w.buf)?;
    let mut w = Writer::new();
    info.write(&mut w);
    conn.send(ids.get(ST, S, "client_information")?, &w.buf)?;

    let mut cfg = ServerConfig::default();
    loop {
        let p = conn.recv()?;
        let Some(n) = ids.name_of(ST, C, p.id) else {
            return Err(ProtocolError::UnknownPacket {
                state: ST,
                id: p.id as u32,
            }
            .into());
        };
        match n {
            "custom_payload" => {
                let mut r = Reader::new(&p.body);
                if r.string(32767)? == "minecraft:brand" {
                    cfg.brand = r.string(32767).ok();
                }
            }
            "disconnect" => return Err(NetError::Disconnected(disconnect_text(&p.body))),
            "keep_alive" => conn.send(ids.get(ST, S, "keep_alive")?, &p.body)?, // long как есть
            "ping" => conn.send(ids.get(ST, S, "pong")?, &p.body)?,             // int как есть
            "registry_data" => cfg.registries.push(parse_registry_data(&p.body)?),
            "update_tags" => cfg.tag_sets = parse_update_tags(&p.body)?,
            "update_enabled_features" => cfg.enabled_features = parse_features(&p.body)?,
            "select_known_packs" => {
                cfg.server_known_packs = parse_known_packs(&p.body)?;
                // DEVIATION: отвечаем пустым списком — сервер пришлёт все данные реестров целиком,
                // вшитый датапак клиенту не нужен (TODO-082). Протокол это допускает.
                conn.send(ids.get(ST, S, "select_known_packs")?, &[0])?;
            }
            "cookie_request" => {
                let key = Reader::new(&p.body).string(32767)?;
                let mut w = Writer::new();
                w.string(&key).bool(false);
                conn.send(ids.get(ST, S, "cookie_response")?, &w.buf)?;
            }
            "resource_pack_push" => {
                // VANILLA-UNSURE: пакеты ресурсов не загружаем; чтобы сервер не кикнул —
                // сообщаем ACCEPTED → SUCCESSFULLY_LOADED (приватный клиент, без UI согласия).
                let id = Reader::new(&p.body).uuid()?;
                for action in [3, 0] {
                    let mut w = Writer::new();
                    w.uuid(id).var_int(action);
                    conn.send(ids.get(ST, S, "resource_pack")?, &w.buf)?;
                }
            }
            "code_of_conduct" => {
                // Приватный клиент без UI: принимаем автоматически.
                conn.send(ids.get(ST, S, "accept_code_of_conduct")?, &[])?;
            }
            "transfer" => {
                let mut r = Reader::new(&p.body);
                let host = r.string(32767)?;
                let port = r.var_int()?;
                let port = u16::try_from(port).map_err(|_| ProtocolError::InvalidValue {
                    what: "transfer port",
                    value: i64::from(port),
                })?;
                return Ok(ConfigOutcome::Transfer { host, port });
            }
            "finish_configuration" => {
                conn.send(ids.get(ST, S, "finish_configuration")?, &[])?;
                conn.set_state(State::Play)?;
                return Ok(ConfigOutcome::Play(Box::new(cfg)));
            }
            // reset_chat, resource_pack_pop, store_cookie, custom_report_details, server_links,
            // clear_dialog, show_dialog — для M1 не нужны
            _ => {}
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::conn::Intent;
    use std::net::TcpListener;

    #[test]
    fn parse_registry_and_tags() {
        let mut w = Writer::new();
        w.string("minecraft:biome").var_int(2);
        w.string("minecraft:plains").bool(true);
        let mut nb = Vec::new();
        nbt::write_network(
            &mut nb,
            Some(&Tag::Compound(vec![("a".into(), Tag::Int(1))])),
        )
        .unwrap();
        w.bytes(&nb);
        w.string("minecraft:void").bool(false);
        let rd = parse_registry_data(&w.buf).unwrap();
        assert_eq!(rd.registry, "minecraft:biome");
        assert_eq!(rd.entries.len(), 2);
        assert!(rd.entries[0].1.is_some() && rd.entries[1].1.is_none());
        assert!(parse_registry_data(&w.buf[..w.buf.len() - 3]).is_err());

        let mut w = Writer::new();
        w.var_int(1)
            .string("minecraft:block")
            .var_int(1)
            .string("minecraft:logs")
            .var_int(2)
            .var_int(5)
            .var_int(9);
        let t = parse_update_tags(&w.buf).unwrap();
        assert_eq!(t[0].tags[0], ("minecraft:logs".to_owned(), vec![5, 9]));
        assert!(parse_update_tags(&[0xff, 0xff, 0xff, 0xff, 0x0f]).is_err());
        let cfg = ServerConfig {
            tag_sets: t,
            ..Default::default()
        };
        assert!(cfg.tag_contains("minecraft:block", "minecraft:logs", 9));
        assert!(!cfg.tag_contains("minecraft:block", "minecraft:logs", 6));
        assert!(!cfg.tag_contains("minecraft:item", "minecraft:logs", 9));
    }

    /// Фейковый сервер: login → compression → configuration → play.
    #[test]
    fn full_login_and_configuration_against_fake_server() {
        let ids = PacketIds::v26_3();
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        let srv = std::thread::spawn(move || {
            let ids = PacketIds::v26_3();
            let (s, _) = l.accept().unwrap();
            let mut c = Connection::from_stream(s).unwrap();
            assert_eq!(c.recv().unwrap().id, 0); // handshake
            c.set_state(State::Login).unwrap();
            let hello = c.recv().unwrap();
            let mut r = Reader::new(&hello.body);
            assert_eq!(r.string(16).unwrap(), "Bot");
            assert_eq!(r.uuid().unwrap(), offline_uuid("Bot"));
            let mut w = Writer::new();
            w.var_int(256);
            c.send(
                ids.get(State::Login, Direction::Clientbound, "login_compression")
                    .unwrap(),
                &w.buf,
            )
            .unwrap();
            c.set_compression(256);
            let mut w = Writer::new();
            GameProfile::offline("Bot").write(&mut w);
            c.send(
                ids.get(State::Login, Direction::Clientbound, "login_finished")
                    .unwrap(),
                &w.buf,
            )
            .unwrap();
            let ack = c.recv().unwrap();
            assert_eq!(
                ids.name_of(State::Login, Direction::Serverbound, ack.id),
                Some("login_acknowledged")
            );
            c.set_state(State::Configuration).unwrap();
            // brand + client_information
            assert_eq!(c.recv().unwrap().id, 2);
            assert_eq!(c.recv().unwrap().id, 0);
            // known packs
            let mut w = Writer::new();
            w.var_int(1)
                .string("minecraft")
                .string("core")
                .string("26.3");
            c.send(
                ids.get(
                    State::Configuration,
                    Direction::Clientbound,
                    "select_known_packs",
                )
                .unwrap(),
                &w.buf,
            )
            .unwrap();
            let kp = c.recv().unwrap();
            assert_eq!(kp.body, [0]);
            // keep alive
            c.send(4, &7i64.to_be_bytes()).unwrap();
            let ka = c.recv().unwrap();
            assert_eq!((ka.id, ka.body.as_slice()), (4, &7i64.to_be_bytes()[..]));
            // registry + features + finish
            let mut w = Writer::new();
            w.string("minecraft:biome")
                .var_int(1)
                .string("minecraft:plains")
                .bool(false);
            c.send(7, &w.buf).unwrap();
            let mut w = Writer::new();
            w.var_int(1).string("minecraft:vanilla");
            c.send(
                ids.get(
                    State::Configuration,
                    Direction::Clientbound,
                    "update_enabled_features",
                )
                .unwrap(),
                &w.buf,
            )
            .unwrap();
            c.send(3, &[]).unwrap();
            assert_eq!(c.recv().unwrap().id, 3);
        });
        let mut c = Connection::connect(&addr.to_string()).unwrap();
        c.handshake(777, "127.0.0.1", addr.port(), Intent::Login)
            .unwrap();
        let prof = login_offline(&mut c, &ids, "Bot").unwrap();
        assert_eq!(prof.name, "Bot");
        assert_eq!(c.state(), State::Configuration);
        let out = configure(&mut c, &ids, &ClientInformation::default()).unwrap();
        let ConfigOutcome::Play(cfg) = out else {
            panic!("ожидался Play")
        };
        assert_eq!(c.state(), State::Play);
        assert_eq!(cfg.registries[0].registry, "minecraft:biome");
        assert_eq!(cfg.enabled_features, ["minecraft:vanilla"]);
        assert_eq!(cfg.server_known_packs[0].id, "core");
        srv.join().unwrap();
    }

    /// TODO-064: мусор на входе даёт ошибки, а не паники.
    #[test]
    fn fuzz_parsers_never_panic() {
        let mut x: u64 = 0x9E3779B97F4A7C15;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..3000 {
            let n = (next() % 64) as usize;
            let b: Vec<u8> = (0..n).map(|_| next() as u8).collect();
            let _ = parse_registry_data(&b);
            let _ = parse_update_tags(&b);
            let _ = parse_known_packs(&b);
            let _ = parse_features(&b);
            let _ = disconnect_text(&b);
            let _ = GameProfile::read(&mut Reader::new(&b));
            let _ = nbt::read_network(&mut Reader::new(&b));
            let _ = nbt::read_named(&mut Reader::new(&b));
            let mut c = crate::frame::Codec::new();
            if next() % 2 == 0 {
                c.set_compression(8);
            }
            c.feed(&b);
            let _ = c.next_packet();
        }
    }

    #[test]
    fn login_disconnect_is_reported() {
        let ids = PacketIds::v26_3();
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        let srv = std::thread::spawn(move || {
            let (s, _) = l.accept().unwrap();
            let mut c = Connection::from_stream(s).unwrap();
            c.recv().unwrap();
            c.recv().unwrap();
            let mut w = Writer::new();
            w.string(r#"{"text":"You are banned"}"#);
            c.send(0, &w.buf).unwrap();
        });
        let mut c = Connection::connect(&addr.to_string()).unwrap();
        c.handshake(777, "x", addr.port(), Intent::Login).unwrap();
        match login_offline(&mut c, &ids, "Bot") {
            Err(NetError::Disconnected(t)) => assert_eq!(t, "You are banned"),
            other => panic!("{other:?}"),
        }
        srv.join().unwrap();
    }

    /// Живой тест мира: чанки приходят, парсятся и кладутся в `Level`.
    /// `EZZMC_TEST_SERVER=127.0.0.1:25599 cargo test -p ezzmc-net live_world -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn live_world_chunks() {
        use ezzmc_world::dimension::DimensionType;
        use ezzmc_world::level::{Level, LevelEvent};
        use ezzmc_world::palette::ceil_log2;
        let Ok(addr) = std::env::var("EZZMC_TEST_SERVER") else {
            return;
        };
        let ids = PacketIds::v26_3();
        let (host, port) = crate::srv::parse_address(&addr);
        let mut c = Connection::connect(&addr).unwrap();
        c.handshake(777, &host, port, Intent::Login).unwrap();
        login_offline(&mut c, &ids, "EzzBot2").unwrap();
        let ConfigOutcome::Play(cfg) =
            configure(&mut c, &ids, &ClientInformation::default()).unwrap()
        else {
            panic!("transfer?")
        };
        let reg = |name: &str| cfg.registries.iter().find(|r| r.registry == name).unwrap();
        let dims = reg("minecraft:dimension_type");
        let biomes = reg("minecraft:worldgen/biome");
        let mut h = crate::play::PlayHandler::new(ids);
        let mut level: Option<Level> = None;
        let mut loaded = 0;
        let mut events = 0;
        let t0 = std::time::Instant::now();
        while t0.elapsed().as_secs() < 15 && loaded < 20 {
            let Ok(p) = c.recv() else { break };
            let name = ids.name_of(State::Play, Direction::Clientbound, p.id);
            let (id, body) = (p.id, p.body.clone());
            if let crate::play::PlayEvent::Login(l) = h.handle(&mut c, p).unwrap() {
                let tag = dims.entries[l.spawn.dimension_type as usize]
                    .1
                    .as_ref()
                    .unwrap();
                let dim = DimensionType::from_nbt(tag).unwrap();
                println!("dimension {:?}", (dim.min_y, dim.height, dim.has_skylight));
                let fmt = dim.chunk_format(
                    ceil_log2(ezzmc_world::generated::b26_3::STATE_COUNT),
                    ceil_log2(biomes.entries.len() as u32),
                );
                level = Some(Level::new(fmt, l.chunk_radius));
                continue;
            }
            let (Some(name), Some(level)) = (name, level.as_mut()) else {
                continue;
            };
            match level
                .handle(name, &body)
                .unwrap_or_else(|e| panic!("{name} (id {id}): {e}"))
            {
                LevelEvent::ChunkLoaded { .. } => loaded += 1,
                LevelEvent::BatchFinished { .. } => {
                    h.send_chunk_batch_received(&mut c, 9.0).unwrap()
                }
                LevelEvent::None => {}
                _ => events += 1,
            }
        }
        let level = level.unwrap();
        println!(
            "загружено чанков: {loaded}, прочих событий: {events}, в кэше {}",
            level.chunks.loaded_count()
        );
        assert!(loaded > 0);
        // bedrock/камень где-то под игроком: хоть один непустой блок в загруженных колонках
        let ctr = level.chunks.center();
        let mut solid = 0;
        for dz in -3..=3 {
            for dx in -3..=3 {
                if let Some(ch) = level.chunks.get(ctr.x + dx, ctr.z + dz) {
                    solid += ch
                        .chunk
                        .sections()
                        .iter()
                        .filter(|s| !s.has_only_air())
                        .count();
                }
            }
        }
        println!("непустых секций вокруг центра: {solid}");
        assert!(solid > 0);
    }

    /// Живой тест: `EZZMC_TEST_SERVER=127.0.0.1:25599 cargo test -p ezzmc-net live -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn live_login_and_configuration() {
        let Ok(addr) = std::env::var("EZZMC_TEST_SERVER") else {
            return;
        };
        let ids = PacketIds::v26_3();
        let (host, port) = crate::srv::parse_address(&addr);
        let mut c = Connection::connect(&addr).unwrap();
        c.handshake(777, &host, port, Intent::Login).unwrap();
        let prof = login_offline(&mut c, &ids, "EzzBot").unwrap();
        println!("profile: {prof:?}");
        let ConfigOutcome::Play(cfg) =
            configure(&mut c, &ids, &ClientInformation::default()).unwrap()
        else {
            panic!("transfer?")
        };
        println!(
            "brand={:?} registries={} tags={} features={:?} packs={:?}",
            cfg.brand,
            cfg.registries.len(),
            cfg.tag_sets.len(),
            cfg.enabled_features,
            cfg.server_known_packs
        );
        assert_eq!(c.state(), State::Play);
        assert!(cfg
            .registries
            .iter()
            .any(|r| r.registry == "minecraft:worldgen/biome"));
        // первый пакет Play — login (join game)
        let mut h = crate::play::PlayHandler::new(ids);
        let mut got = Vec::new();
        let t0 = std::time::Instant::now();
        while t0.elapsed().as_secs() < 5 {
            let Ok(p) = c.recv() else { break };
            got.push(ids.name_of(State::Play, Direction::Clientbound, p.id));
            match h.handle(&mut c, p).unwrap() {
                crate::play::PlayEvent::Login(l) => println!("login: {l:?}"),
                crate::play::PlayEvent::GameEvent { event, param } => {
                    println!("game_event {event} {param}")
                }
                crate::play::PlayEvent::DefaultSpawn(d) => println!("spawn {d:?}"),
                crate::play::PlayEvent::Teleport { state, .. } => println!("teleport {state:?}"),
                _ => {}
            }
            if got.contains(&Some("level_chunk_with_light")) {
                break;
            }
        }
        println!("play packets: {got:?}");
        h.send_player_loaded(&mut c).unwrap();
        assert!(got.contains(&Some("login")) && got.contains(&Some("player_position")));
    }
}
