//! Server List Ping (TODO-054): handshake(status) → status request → JSON → ping/pong.
// VANILLA: ServerStatusPinger#pingServer (ClientboundStatusResponsePacket + ServerboundPingRequestPacket)
use crate::conn::Intent;
use crate::{srv, Connection, NetError};
use ezzmc_protocol::wire::{Reader, Writer};
use std::time::Instant;

/// Ответ сервера на status request.
pub struct StatusReply {
    /// Сырой JSON статуса (`version`, `players`, `description`, `favicon`).
    pub json: String,
    /// Круговая задержка ping/pong, мс.
    pub ping_ms: u64,
}

pub enum StatusError {
    /// VANILLA: UnknownHostException → «Can't resolve hostname».
    CantResolve,
    /// Любая другая ошибка → «Can't connect to server».
    CantConnect,
}

/// Пингует `address` (`host[:port]`, SRV учитывается) протоколом `protocol`.
pub fn ping_server(address: &str, protocol: i32) -> Result<StatusReply, StatusError> {
    let (host, port) = srv::parse_address(address);
    let (thost, tport) = srv::resolve_target(&host, port);
    match srv::resolve_addrs(&thost, tport) {
        Ok(a) if !a.is_empty() => {}
        _ => return Err(StatusError::CantResolve),
    }
    run(address, &host, port, protocol).map_err(|_| StatusError::CantConnect)
}

fn run(address: &str, host: &str, port: u16, protocol: i32) -> Result<StatusReply, NetError> {
    let mut c = Connection::connect(address)?;
    c.handshake(protocol, host, port, Intent::Status)?;
    c.send(0, &[])?; // status request
    let p = c.recv()?;
    if p.id != 0 {
        return Err(NetError::Closed);
    }
    let json = Reader::new(&p.body)
        .string(32767)
        .map_err(NetError::from)?;
    let start = Instant::now();
    let mut w = Writer::new();
    w.i64(0);
    c.send(1, &w.into_vec())?; // ping request
    let _ = c.recv()?; // pong
    Ok(StatusReply {
        json,
        ping_ms: start.elapsed().as_millis() as u64,
    })
}
