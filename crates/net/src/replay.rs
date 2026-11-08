//! Запись и воспроизведение трафика (TODO-023).
//!
//! Формат `.ezrec` (все числа little-endian):
//! `"EZMCREC1"` · `u32 protocol` · записи `{ u64 t_us, u8 dir, u32 len, [u8; len] }`.
//! `dir`: 0 = clientbound (сервер→клиент), 1 = serverbound. Данные — сырые куски TCP-потока
//! (до разбора фрейминга), поэтому запись пригодна для теста любого уровня стека.
//! Шифрования в офлайн-режиме нет; компрессия остаётся как была на проводе.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const MAGIC: &[u8; 8] = b"EZMCREC1";
/// Предел одного куска при чтении (защита от мусорного файла).
const MAX_CHUNK: u32 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Clientbound,
    Serverbound,
}

impl Dir {
    fn to_byte(self) -> u8 {
        match self {
            Dir::Clientbound => 0,
            Dir::Serverbound => 1,
        }
    }
    fn from_byte(b: u8) -> io::Result<Self> {
        match b {
            0 => Ok(Dir::Clientbound),
            1 => Ok(Dir::Serverbound),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "bad direction byte",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub t_us: u64,
    pub dir: Dir,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Recording {
    pub protocol: u32,
    pub chunks: Vec<Chunk>,
}

impl Recording {
    pub fn write_to(&self, mut w: impl Write) -> io::Result<()> {
        write_header(&mut w, self.protocol)?;
        for c in &self.chunks {
            write_chunk(&mut w, c)?;
        }
        Ok(())
    }

    pub fn read_from(mut r: impl Read) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "not an ezrec file",
            ));
        }
        let mut p = [0u8; 4];
        r.read_exact(&mut p)?;
        let mut rec = Recording {
            protocol: u32::from_le_bytes(p),
            chunks: Vec::new(),
        };
        loop {
            let mut t = [0u8; 8];
            match r.read(&mut t[..1])? {
                0 => break, // чистый конец файла
                _ => r.read_exact(&mut t[1..])?,
            }
            let mut d = [0u8; 1];
            r.read_exact(&mut d)?;
            let mut l = [0u8; 4];
            r.read_exact(&mut l)?;
            let len = u32::from_le_bytes(l);
            if len > MAX_CHUNK {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk too large",
                ));
            }
            let mut data = vec![0u8; len as usize];
            r.read_exact(&mut data)?;
            rec.chunks.push(Chunk {
                t_us: u64::from_le_bytes(t),
                dir: Dir::from_byte(d[0])?,
                data,
            });
        }
        Ok(rec)
    }

    /// Все байты одного направления подряд — вход для тестов декодера потока.
    pub fn stream(&self, dir: Dir) -> Vec<u8> {
        self.chunks
            .iter()
            .filter(|c| c.dir == dir)
            .flat_map(|c| c.data.iter().copied())
            .collect()
    }
}

fn write_header(w: &mut impl Write, protocol: u32) -> io::Result<()> {
    w.write_all(MAGIC)?;
    w.write_all(&protocol.to_le_bytes())
}

fn write_chunk(w: &mut impl Write, c: &Chunk) -> io::Result<()> {
    let len = u32::try_from(c.data.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "chunk too large"))?;
    w.write_all(&c.t_us.to_le_bytes())?;
    w.write_all(&[c.dir.to_byte()])?;
    w.write_all(&len.to_le_bytes())?;
    w.write_all(&c.data)
}

/// Прокси-рекордер: принимает одно соединение клиента на `listen`, пересылает на `target`,
/// пишет оба направления в `out`. Возвращается, когда любая сторона закрыла соединение.
pub fn record_proxy(
    listen: impl ToSocketAddrs,
    target: impl ToSocketAddrs,
    protocol: u32,
    out: impl Write + Send + 'static,
) -> io::Result<()> {
    let listener = TcpListener::bind(listen)?;
    let (client, _) = listener.accept()?;
    let server = TcpStream::connect(target)?;
    client.set_nodelay(true)?;
    server.set_nodelay(true)?;
    let mut out = out;
    write_header(&mut out, protocol)?;
    let sink = Arc::new(Mutex::new(out));
    let start = Instant::now();
    let pump = |mut from: TcpStream, mut to: TcpStream, dir: Dir| {
        let sink = Arc::clone(&sink);
        thread::spawn(move || {
            let mut buf = vec![0u8; 64 * 1024];
            loop {
                let n = match from.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                let chunk = Chunk {
                    t_us: u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX),
                    dir,
                    data: buf[..n].to_vec(),
                };
                if let Ok(mut s) = sink.lock() {
                    if write_chunk(&mut *s, &chunk).is_err() {
                        break;
                    }
                }
                if to.write_all(&buf[..n]).is_err() {
                    break;
                }
            }
            let _ = to.shutdown(std::net::Shutdown::Both);
            let _ = from.shutdown(std::net::Shutdown::Both);
        })
    };
    let a = pump(client.try_clone()?, server.try_clone()?, Dir::Serverbound);
    let b = pump(server, client, Dir::Clientbound);
    let _ = a.join();
    let _ = b.join();
    if let Ok(mut s) = sink.lock() {
        s.flush()?;
    }
    Ok(())
}

/// Воспроизводит clientbound-часть записи в `conn`; serverbound от клиента читается и
/// отбрасывается. `realtime` — выдерживать исходные паузы между кусками.
pub fn replay_clientbound(rec: &Recording, conn: TcpStream, realtime: bool) -> io::Result<()> {
    let mut rd = conn.try_clone()?;
    let drain = thread::spawn(move || {
        let mut b = [0u8; 4096];
        while matches!(rd.read(&mut b), Ok(n) if n > 0) {}
    });
    let mut conn = conn;
    let start = Instant::now();
    for c in rec.chunks.iter().filter(|c| c.dir == Dir::Clientbound) {
        if realtime {
            let due = Duration::from_micros(c.t_us);
            if let Some(wait) = due.checked_sub(start.elapsed()) {
                thread::sleep(wait);
            }
        }
        conn.write_all(&c.data)?;
    }
    let _ = conn.shutdown(std::net::Shutdown::Write);
    let _ = drain.join();
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample() -> Recording {
        Recording {
            protocol: 777,
            chunks: vec![
                Chunk {
                    t_us: 0,
                    dir: Dir::Serverbound,
                    data: vec![1, 2, 3],
                },
                Chunk {
                    t_us: 10,
                    dir: Dir::Clientbound,
                    data: vec![9, 8],
                },
                Chunk {
                    t_us: 20,
                    dir: Dir::Clientbound,
                    data: vec![7],
                },
            ],
        }
    }

    #[test]
    fn roundtrip() {
        let rec = sample();
        let mut buf = Vec::new();
        rec.write_to(&mut buf).unwrap();
        assert_eq!(Recording::read_from(&buf[..]).unwrap(), rec);
        assert_eq!(rec.stream(Dir::Clientbound), vec![9, 8, 7]);
    }

    #[test]
    fn rejects_garbage_and_truncation() {
        assert!(Recording::read_from(&b"NOTAREC!\0\0\0\0"[..]).is_err());
        let mut buf = Vec::new();
        sample().write_to(&mut buf).unwrap();
        buf.truncate(buf.len() - 1);
        assert!(Recording::read_from(&buf[..]).is_err());
    }

    #[test]
    fn proxy_records_and_replay_serves() {
        // «сервер»: отвечает «pong» на любые данные
        let srv = TcpListener::bind("127.0.0.1:0").unwrap();
        let srv_addr = srv.local_addr().unwrap();
        let st = thread::spawn(move || {
            let (mut s, _) = srv.accept().unwrap();
            let mut b = [0u8; 4];
            s.read_exact(&mut b).unwrap();
            s.write_all(b"pong").unwrap();
        });
        // свободный порт для прокси
        let probe = TcpListener::bind("127.0.0.1:0").unwrap();
        let paddr = probe.local_addr().unwrap();
        drop(probe);
        let shared: Arc<Mutex<Vec<u8>>> = Arc::default();
        struct Sink(Arc<Mutex<Vec<u8>>>);
        impl Write for Sink {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                self.0.lock().unwrap().extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let sink = Sink(Arc::clone(&shared));
        let pt = thread::spawn(move || record_proxy(paddr, srv_addr, 777, sink).unwrap());
        let mut c = loop {
            if let Ok(c) = TcpStream::connect(paddr) {
                break c;
            }
            thread::sleep(Duration::from_millis(10));
        };
        c.write_all(b"ping").unwrap();
        let mut got = [0u8; 4];
        c.read_exact(&mut got).unwrap();
        assert_eq!(&got, b"pong");
        drop(c);
        st.join().unwrap();
        pt.join().unwrap();
        let rec = Recording::read_from(&shared.lock().unwrap()[..]).unwrap();
        assert_eq!(rec.protocol, 777);
        assert_eq!(rec.stream(Dir::Serverbound), b"ping");
        assert_eq!(rec.stream(Dir::Clientbound), b"pong");

        // воспроизведение
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        let rt = thread::spawn(move || {
            let (s, _) = l.accept().unwrap();
            replay_clientbound(&rec, s, false).unwrap();
        });
        let mut c = TcpStream::connect(addr).unwrap();
        let mut all = Vec::new();
        c.read_to_end(&mut all).unwrap();
        assert_eq!(all, b"pong");
        drop(c);
        rt.join().unwrap();
    }
}
