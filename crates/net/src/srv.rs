//! Разрешение адреса сервера: SRV `_minecraft._tcp.<host>` → A/AAAA (TODO-047).
// VANILLA: ServerNameResolver / ServerRedirectHandler / ServerAddress.parseString
// Минимальный DNS-клиент по UDP (без зависимостей): имена серверов — из /etc/resolv.conf
// (unix) либо публичные резолверы. Любая ошибка SRV → тихий откат к обычному A-запросу.
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub const DEFAULT_PORT: u16 = 25565;

/// VANILLA: ServerAddress.parseString — `host[:port]`, IPv6 в `[...]`.
pub fn parse_address(s: &str) -> (String, u16) {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix('[') {
        if let Some((host, tail)) = rest.split_once(']') {
            let port = tail
                .strip_prefix(':')
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_PORT);
            return (host.to_owned(), port);
        }
    }
    match s.rsplit_once(':') {
        Some((h, p)) if !h.contains(':') => (
            h.to_owned(),
            p.parse().ok().filter(|&p| p != 0).unwrap_or(DEFAULT_PORT),
        ),
        _ => (s.to_owned(), DEFAULT_PORT),
    }
}

/// Возвращает (host, port) для TCP-подключения. SRV только если порт дефолтный и host — не IP.
// VANILLA: SRV смотрится только когда порт не указан явно (== 25565)
pub fn resolve_target(host: &str, port: u16) -> (String, u16) {
    if port == DEFAULT_PORT && host.parse::<IpAddr>().is_err() {
        if let Some((h, p)) = srv_lookup(&format!("_minecraft._tcp.{host}")) {
            return (h, p);
        }
    }
    (host.to_owned(), port)
}

pub fn resolve_addrs(host: &str, port: u16) -> std::io::Result<Vec<std::net::SocketAddr>> {
    Ok((host, port).to_socket_addrs()?.collect())
}

fn nameservers() -> Vec<std::net::SocketAddr> {
    let mut v = Vec::new();
    if let Ok(t) = std::fs::read_to_string("/etc/resolv.conf") {
        for l in t.lines() {
            if let Some(ip) = l.trim().strip_prefix("nameserver") {
                if let Ok(ip) = ip.trim().parse::<IpAddr>() {
                    v.push(std::net::SocketAddr::new(ip, 53));
                }
            }
        }
    }
    v.push("1.1.1.1:53".parse().expect("const addr"));
    v.push("8.8.8.8:53".parse().expect("const addr"));
    v
}

fn srv_lookup(name: &str) -> Option<(String, u16)> {
    let q = build_query(name, 0x1234)?;
    for ns in nameservers() {
        let bind = if ns.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" };
        let Ok(sock) = UdpSocket::bind(bind) else {
            continue;
        };
        let _ = sock.set_read_timeout(Some(Duration::from_millis(1500)));
        if sock.send_to(&q, ns).is_err() {
            continue;
        }
        let mut buf = [0u8; 1500];
        let Ok((n, from)) = sock.recv_from(&mut buf) else {
            continue;
        };
        if from.ip() != ns.ip() {
            continue;
        }
        if let Some(r) = parse_srv(&buf[..n], 0x1234) {
            return Some(r);
        }
        return None; // сервер ответил, но SRV нет
    }
    None
}

fn build_query(name: &str, id: u16) -> Option<Vec<u8>> {
    let mut q = Vec::new();
    q.extend_from_slice(&id.to_be_bytes());
    q.extend_from_slice(&[0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0]); // RD, 1 вопрос
    for label in name.trim_end_matches('.').split('.') {
        if label.is_empty() || label.len() > 63 {
            return None;
        }
        q.push(label.len() as u8);
        q.extend_from_slice(label.as_bytes());
    }
    q.push(0);
    q.extend_from_slice(&[0, 33, 0, 1]); // SRV, IN
    Some(q)
}

/// Пропустить закодированное имя, вернуть позицию после него.
fn skip_name(b: &[u8], mut p: usize) -> Option<usize> {
    loop {
        let l = *b.get(p)? as usize;
        if l == 0 {
            return Some(p + 1);
        }
        if l & 0xC0 == 0xC0 {
            return Some(p + 2);
        }
        p += 1 + l;
    }
}

fn read_name(b: &[u8], mut p: usize) -> Option<String> {
    let mut out = String::new();
    let mut jumps = 0;
    loop {
        let l = *b.get(p)? as usize;
        if l == 0 {
            break;
        }
        if l & 0xC0 == 0xC0 {
            jumps += 1;
            if jumps > 8 {
                return None;
            }
            p = ((l & 0x3F) << 8) | *b.get(p + 1)? as usize;
            continue;
        }
        let label = b.get(p + 1..p + 1 + l)?;
        if !out.is_empty() {
            out.push('.');
        }
        out.push_str(std::str::from_utf8(label).ok()?);
        p += 1 + l;
    }
    Some(out)
}

/// Выбирает запись с наименьшим priority (при равенстве — первая).
fn parse_srv(b: &[u8], id: u16) -> Option<(String, u16)> {
    if b.len() < 12
        || u16::from_be_bytes([b[0], b[1]]) != id
        || b[2] & 0x80 == 0
        || b[3] & 0x0F != 0
    {
        return None;
    }
    let qd = u16::from_be_bytes([b[4], b[5]]);
    let an = u16::from_be_bytes([b[6], b[7]]);
    let mut p = 12;
    for _ in 0..qd {
        p = skip_name(b, p)? + 4;
    }
    let mut best: Option<(u16, String, u16)> = None;
    for _ in 0..an {
        p = skip_name(b, p)?;
        let typ = u16::from_be_bytes([*b.get(p)?, *b.get(p + 1)?]);
        let rdlen = u16::from_be_bytes([*b.get(p + 8)?, *b.get(p + 9)?]) as usize;
        let rd = p + 10;
        if typ == 33 {
            let prio = u16::from_be_bytes([*b.get(rd)?, *b.get(rd + 1)?]);
            let port = u16::from_be_bytes([*b.get(rd + 4)?, *b.get(rd + 5)?]);
            let host = read_name(b, rd + 6)?;
            if best.as_ref().is_none_or(|(bp, _, _)| prio < *bp) {
                best = Some((prio, host, port));
            }
        }
        p = rd + rdlen;
    }
    best.map(|(_, h, p)| (h, p))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn address_parsing() {
        assert_eq!(parse_address("a.b"), ("a.b".into(), 25565));
        assert_eq!(parse_address("a.b:1234"), ("a.b".into(), 1234));
        assert_eq!(parse_address("a.b:xx"), ("a.b".into(), 25565));
        assert_eq!(parse_address("[::1]:99"), ("::1".into(), 99));
        assert_eq!(parse_address("[::1]"), ("::1".into(), 25565));
        assert_eq!(parse_address("::1"), ("::1".into(), 25565));
    }

    #[test]
    fn explicit_port_or_ip_skips_srv() {
        assert_eq!(
            resolve_target("127.0.0.1", 25565),
            ("127.0.0.1".into(), 25565)
        );
        assert_eq!(
            resolve_target("example.invalid", 1111),
            ("example.invalid".into(), 1111)
        );
    }

    #[test]
    fn srv_response_parsing() {
        let mut r = build_query("_minecraft._tcp.x.y", 0x1234).unwrap();
        r[2] = 0x81;
        r[3] = 0x80;
        r[7] = 2; // 2 ответа
        for (prio, port, target) in [(20u16, 1u16, "b.y"), (10, 25570, "mc.x.y")] {
            r.extend_from_slice(&[0xC0, 12, 0, 33, 0, 1, 0, 0, 0, 60]);
            let mut rd = vec![];
            rd.extend_from_slice(&prio.to_be_bytes());
            rd.extend_from_slice(&[0, 5]);
            rd.extend_from_slice(&port.to_be_bytes());
            for l in target.split('.') {
                rd.push(l.len() as u8);
                rd.extend_from_slice(l.as_bytes());
            }
            rd.push(0);
            r.extend_from_slice(&(rd.len() as u16).to_be_bytes());
            r.extend_from_slice(&rd);
        }
        assert_eq!(parse_srv(&r, 0x1234), Some(("mc.x.y".into(), 25570)));
        assert_eq!(parse_srv(&r, 0x9999), None);
        assert_eq!(parse_srv(&r[..20], 0x1234), None); // обрезано — не паника
    }
}
