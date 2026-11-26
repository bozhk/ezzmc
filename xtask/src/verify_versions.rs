//! `cargo xtask verify-versions [--write]` (TODO-370): сверка `data/versions.toml` с первоисточником —
//! `version.json` внутри официального client.jar Mojang (есть с 1.14; поля `protocol_version`,
//! `world_version`). Jar не скачивается целиком: Range-запросы к хвосту (каталог zip) и к одной записи.
//! Версии без `version.json` (≤1.13.2) помечаются «нет первоисточника в jar».
use crate::fetch_reference::Res;
use serde_json::Value;
use std::fs;
use std::io::Read;

const MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const TABLE: &str = "data/versions.toml";
/// 1.7.x–1.13.2: realms-API не обфусцирован, протокол — `public static int` в `<clinit>`.
const REALMS_CLASS: &str = "net/minecraft/realms/RealmsSharedConstants.class";

/// Запись таблицы: имя, protocol, data_version (если указан).
struct Row {
    name: String,
    protocol: i64,
    data_version: Option<i64>,
}

fn parse_table(text: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut cur: Option<Row> = None;
    for line in text.lines() {
        let line = line.trim();
        if line == "[[version]]" {
            rows.extend(cur.take());
            cur = Some(Row {
                name: String::new(),
                protocol: -1,
                data_version: None,
            });
            continue;
        }
        let (Some(r), Some((k, v))) = (cur.as_mut(), line.split_once('=')) else {
            continue;
        };
        let v = v.trim().trim_matches('"');
        match k.trim() {
            "name" => r.name = v.to_string(),
            "protocol" => r.protocol = v.parse().unwrap_or(-1),
            "data_version" => r.data_version = v.parse().ok(),
            _ => {}
        }
    }
    rows.extend(cur);
    rows
}

fn range(url: &str, from: u64, len: u64) -> Res<Vec<u8>> {
    let mut buf = Vec::new();
    ureq::get(url)
        .set("Range", &format!("bytes={from}-{}", from + len - 1))
        .call()?
        .into_reader()
        .read_to_end(&mut buf)?;
    Ok(buf)
}

fn u16le(b: &[u8], i: usize) -> u64 {
    u16::from_le_bytes([b[i], b[i + 1]]) as u64
}
fn u32le(b: &[u8], i: usize) -> u64 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as u64
}

/// Содержимое записи `name` из zip по URL (только Range). `None` — записи нет.
fn zip_entry(url: &str, size: u64, name: &str) -> Res<Option<Vec<u8>>> {
    let tail_len = size.min(65_557);
    let tail = range(url, size - tail_len, tail_len)?;
    let eocd = (0..tail.len().saturating_sub(21))
        .rev()
        .find(|&i| tail[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
        .ok_or("нет EOCD в jar")?;
    let cd_size = u32le(&tail, eocd + 12);
    let cd_off = u32le(&tail, eocd + 16);
    let cd = range(url, cd_off, cd_size)?;
    let mut i = 0usize;
    while i + 46 <= cd.len() && cd[i..i + 4] == [0x50, 0x4b, 0x01, 0x02] {
        let method = u16le(&cd, i + 10);
        let csize = u32le(&cd, i + 20);
        let usize_ = u32le(&cd, i + 24);
        let nlen = u16le(&cd, i + 28) as usize;
        let elen = u16le(&cd, i + 30) as usize;
        let clen = u16le(&cd, i + 32) as usize;
        let loff = u32le(&cd, i + 42);
        let ename = &cd[i + 46..i + 46 + nlen];
        if ename == name.as_bytes() {
            let lh = range(url, loff, 30)?;
            let data_off = loff + 30 + u16le(&lh, 26) + u16le(&lh, 28);
            let raw = range(url, data_off, csize)?;
            return Ok(Some(match method {
                0 => raw,
                8 => {
                    let mut out = vec![0u8; usize_ as usize];
                    let n = libdeflater::Decompressor::new().deflate_decompress(&raw, &mut out)?;
                    out.truncate(n);
                    out
                }
                m => return Err(format!("метод сжатия {m} не поддержан").into()),
            }));
        }
        i += 46 + nlen + elen + clen;
    }
    Ok(None)
}

/// Значение `static int` поля `field`: атрибут ConstantValue (`static final`) либо присваивание
/// в `<clinit>` (iconst/bipush/sipush/ldc перед putstatic).
fn static_int(cls: &[u8], field: &str) -> Option<i64> {
    let be16 = |i: usize| u16::from_be_bytes([cls[i], cls[i + 1]]) as usize;
    let count = be16(8);
    // Пул констант: (tag, смещение данных).
    let mut cp = vec![(0u8, 0usize); count];
    let mut i = 10;
    let mut k = 1;
    while k < count {
        let tag = cls[i];
        cp[k] = (tag, i + 1);
        i += 1 + match tag {
            1 => 2 + be16(i + 1),
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => 4,
            5 | 6 => 8,
            7 | 8 | 16 | 19 | 20 => 2,
            15 => 3,
            _ => return None,
        };
        k += if matches!(tag, 5 | 6) { 2 } else { 1 };
    }
    let utf8 = |idx: usize| -> Option<&[u8]> {
        let (t, o) = *cp.get(idx)?;
        (t == 1).then(|| &cls[o + 2..o + 2 + be16(o)])
    };
    let int_at = |idx: usize| -> Option<i64> {
        let (t, o) = *cp.get(idx)?;
        (t == 3).then(|| i32::from_be_bytes([cls[o], cls[o + 1], cls[o + 2], cls[o + 3]]) as i64)
    };
    // `static final int` (1.13+): атрибут ConstantValue у поля.
    let mut j = i + 6;
    j += 2 + 2 * be16(j);
    let fields = be16(j);
    j += 2;
    for _ in 0..fields {
        let name = be16(j + 2);
        let attrs = be16(j + 6);
        j += 8;
        for _ in 0..attrs {
            let len = u32::from_be_bytes([cls[j + 2], cls[j + 3], cls[j + 4], cls[j + 5]]) as usize;
            if utf8(name) == Some(field.as_bytes())
                && utf8(be16(j)) == Some(b"ConstantValue".as_slice())
            {
                return int_at(be16(j + 6));
            }
            j += 6 + len;
        }
    }
    let fref = (1..count).find(|&n| {
        let (t, o) = cp[n];
        t == 9 && {
            let (_, nt) = cp[be16(o + 2)];
            utf8(be16(nt)) == Some(field.as_bytes())
        }
    })?;
    let code = &cls[i..];
    let pat = [0xb3, (fref >> 8) as u8, fref as u8];
    let at = code.windows(3).position(|w| w == pat)?;
    let b = |back: usize| code.get(at.checked_sub(back)?).copied();
    if b(3) == Some(0x11) {
        return Some(i16::from_be_bytes([code[at - 2], code[at - 1]]) as i64);
    }
    match b(2) {
        Some(0x10) => return Some(code[at - 1] as i8 as i64),
        Some(0x12) => return int_at(code[at - 1] as usize),
        _ => {}
    }
    b(1).filter(|op| (0x02..=0x08).contains(op))
        .map(|op| op as i64 - 3)
}

pub fn run_cmd() -> Res<()> {
    let text = fs::read_to_string(TABLE)?;
    let rows = parse_table(&text);
    let manifest: Value = ureq::get(MANIFEST).call()?.into_json()?;
    let versions = manifest["versions"]
        .as_array()
        .ok_or("манифест без versions")?;
    let (mut ok, mut bad, mut nosrc) = (0, 0, 0);
    for r in &rows {
        let Some(url) = versions
            .iter()
            .find(|v| v["id"] == r.name.as_str())
            .and_then(|v| v["url"].as_str())
        else {
            println!("{:<10} нет в манифесте Mojang", r.name);
            bad += 1;
            continue;
        };
        let meta: Value = ureq::get(url).call()?.into_json()?;
        let client = &meta["downloads"]["client"];
        let (Some(jar), Some(size)) = (client["url"].as_str(), client["size"].as_u64()) else {
            println!("{:<10} нет downloads.client", r.name);
            bad += 1;
            continue;
        };
        let (p, w, src) = if let Some(raw) = zip_entry(jar, size, "version.json")? {
            let vj: Value = serde_json::from_slice(&raw)?;
            let p = vj["protocol_version"].as_i64().unwrap_or(-1);
            (p, vj["world_version"].as_i64(), "version.json")
        } else if let Some(cls) = zip_entry(jar, size, REALMS_CLASS)? {
            let p = static_int(&cls, "NETWORK_PROTOCOL_VERSION").unwrap_or(-1);
            (p, None, "RealmsSharedConstants")
        } else {
            println!(
                "{:<10} protocol {:>4}: нет первоисточника в jar",
                r.name, r.protocol
            );
            nosrc += 1;
            continue;
        };
        let w_ok = w.is_none() || r.data_version.is_none() || r.data_version == w;
        if p == r.protocol && w_ok {
            ok += 1;
        } else {
            bad += 1;
            println!(
                "{:<10} РАСХОЖДЕНИЕ ({src}): таблица protocol {} data {:?}, jar protocol {p} world {w:?}",
                r.name, r.protocol, r.data_version
            );
        }
    }
    println!(
        "итого {}: совпало {ok}, расхождений {bad}, без первоисточника {nosrc}",
        rows.len()
    );
    if bad > 0 {
        return Err("есть расхождения".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rows() {
        let t = "latest = \"x\"\n[[version]]\nname = \"1.5.2\"\nprotocol = 61\n\n[[version]]\nname = \"26.3\"\nprotocol = 777\ndata_version = 5023\n";
        let r = parse_table(t);
        assert_eq!(r.len(), 2);
        assert_eq!(
            (r[0].name.as_str(), r[0].protocol, r[0].data_version),
            ("1.5.2", 61, None)
        );
        assert_eq!((r[1].protocol, r[1].data_version), (777, Some(5023)));
    }
}
