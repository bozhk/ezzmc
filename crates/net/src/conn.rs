//! Соединение: TCP, таймауты, конечный автомат состояний, рукопожатие (TODO-047/052/053).
// VANILLA: Connection, ClientIntentionPacket, ConnectionProtocol (1.21.11)
use crate::frame::{Codec, Packet};
use crate::srv;
use crate::{NetError, NetResult};
use ezzmc_protocol::wire::Writer;
use ezzmc_protocol::{ProtocolError, State};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::{Duration, Instant};

/// VANILLA: чтение — 30 с (ReadTimeoutHandler). Подключение: 10 с на адрес (у ванили 30 с общий) — осознанно короче.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// VANILLA: ClientIntent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl Intent {
    pub fn next_state(self) -> State {
        match self {
            Intent::Status => State::Status,
            Intent::Login | Intent::Transfer => State::Login,
        }
    }
}

/// Допустимые переходы состояний (VANILLA: ConnectionProtocol / PacketFlow).
pub fn transition_allowed(from: State, to: State) -> bool {
    use State::*;
    matches!(
        (from, to),
        (Handshake, Status)
            | (Handshake, Login)
            | (Login, Configuration)
            | (Configuration, Play)
            | (Play, Configuration) // 1.20.2+: reconfigure
    )
}

/// Тело пакета intention (Handshake serverbound id 0).
pub fn handshake_body(protocol: i32, host: &str, port: u16, intent: Intent) -> Vec<u8> {
    let mut w = Writer::new();
    w.var_int(protocol)
        .string(host)
        .u16(port)
        .var_int(intent as i32);
    w.into_vec()
}

pub struct Connection {
    stream: TcpStream,
    codec: Codec,
    state: State,
    out: Vec<u8>,
    scratch: Box<[u8; 16 * 1024]>,
}

impl Connection {
    /// Подключиться: SRV (если применимо) → адреса → connect с таймаутом, пробуем все по очереди.
    pub fn connect(address: &str) -> NetResult<Self> {
        let (host, port) = srv::parse_address(address);
        let (thost, tport) = srv::resolve_target(&host, port);
        let addrs = srv::resolve_addrs(&thost, tport)?;
        let mut last: Option<std::io::Error> = None;
        for a in addrs {
            match TcpStream::connect_timeout(&a, CONNECT_TIMEOUT) {
                Ok(s) => return Self::from_stream(s),
                Err(e) => last = Some(e),
            }
        }
        Err(last
            .map(NetError::Io)
            .unwrap_or(NetError::Timeout("нет адресов для подключения")))
    }

    pub fn from_stream(stream: TcpStream) -> NetResult<Self> {
        stream.set_nodelay(true)?;
        Ok(Self {
            stream,
            codec: Codec::new(),
            state: State::Handshake,
            out: Vec::new(),
            scratch: Box::new([0; 16 * 1024]),
        })
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, to: State) -> NetResult<()> {
        if !transition_allowed(self.state, to) {
            return Err(ProtocolError::Other(format!(
                "недопустимый переход состояния {:?} → {to:?}",
                self.state
            ))
            .into());
        }
        self.state = to;
        Ok(())
    }

    pub fn set_compression(&mut self, threshold: i32) {
        self.codec.set_compression(threshold);
    }

    /// Отправить handshake и перейти в Status/Login.
    pub fn handshake(
        &mut self,
        protocol: i32,
        host: &str,
        port: u16,
        intent: Intent,
    ) -> NetResult<()> {
        self.send(0, &handshake_body(protocol, host, port, intent))?;
        self.set_state(intent.next_state())
    }

    pub fn send(&mut self, id: i32, body: &[u8]) -> NetResult<()> {
        self.out.clear();
        self.codec.encode(id, body, &mut self.out)?;
        self.stream.write_all(&self.out)?;
        Ok(())
    }

    /// Блокирующее чтение следующего пакета (общий таймаут `READ_TIMEOUT` на пакет).
    pub fn recv(&mut self) -> NetResult<Packet> {
        let deadline = Instant::now() + READ_TIMEOUT;
        loop {
            if let Some(p) = self.codec.next_packet()? {
                return Ok(p);
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Err(NetError::Timeout("чтение пакета"));
            }
            self.stream.set_read_timeout(Some(left))?;
            match self.stream.read(&mut self.scratch[..]) {
                Ok(0) => return Err(NetError::Closed),
                Ok(n) => self.codec.feed(&self.scratch[..n]),
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) =>
                {
                    return Err(NetError::Timeout("чтение пакета"))
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// Чтение с коротким ожиданием: `Ok(None)`, если за `wait` пакет не пришёл
    /// (нужно сетевому потоку, чтобы между пакетами слать позицию игрока).
    pub fn poll(&mut self, wait: Duration) -> NetResult<Option<Packet>> {
        let deadline = Instant::now() + wait;
        loop {
            if let Some(p) = self.codec.next_packet()? {
                return Ok(Some(p));
            }
            let left = deadline
                .saturating_duration_since(Instant::now())
                .max(Duration::from_millis(1));
            self.stream.set_read_timeout(Some(left))?;
            match self.stream.read(&mut self.scratch[..]) {
                Ok(0) => return Err(NetError::Closed),
                Ok(n) => self.codec.feed(&self.scratch[..n]),
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) =>
                {
                    return Ok(None)
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
            if Instant::now() >= deadline {
                return self.codec.next_packet();
            }
        }
    }

    pub fn close(&self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use ezzmc_protocol::wire::Reader;
    use std::net::TcpListener;

    #[test]
    fn handshake_bytes() {
        let b = handshake_body(777, "localhost", 25565, Intent::Login);
        let mut r = Reader::new(&b);
        assert_eq!(r.var_int().unwrap(), 777);
        assert_eq!(r.string(255).unwrap(), "localhost");
        assert_eq!(r.u16().unwrap(), 25565);
        assert_eq!(r.var_int().unwrap(), 2);
        assert!(r.is_empty());
    }

    #[test]
    fn state_machine() {
        assert!(transition_allowed(State::Handshake, State::Login));
        assert!(transition_allowed(State::Login, State::Configuration));
        assert!(!transition_allowed(State::Handshake, State::Play));
        assert!(!transition_allowed(State::Play, State::Login));
    }

    #[test]
    fn loopback_handshake_and_status() {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        let srv = std::thread::spawn(move || {
            let (s, _) = l.accept().unwrap();
            let mut c = Connection::from_stream(s).unwrap();
            let p = c.recv().unwrap();
            assert_eq!(p.id, 0);
            let mut r = Reader::new(&p.body);
            assert_eq!(r.var_int().unwrap(), 777);
            c.send(0, b"{}").unwrap();
        });
        let mut c = Connection::connect(&addr.to_string()).unwrap();
        c.handshake(777, "127.0.0.1", addr.port(), Intent::Status)
            .unwrap();
        assert_eq!(c.state(), State::Status);
        let p = c.recv().unwrap();
        assert_eq!((p.id, p.body.as_slice()), (0, &b"{}"[..]));
        srv.join().unwrap();
        assert!(matches!(c.recv(), Err(NetError::Closed)));
    }
}
