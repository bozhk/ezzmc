//! NBT: бинарный формат (named / nameless root), SNBT (TODO-055).
// VANILLA: NbtIo, NbtAccounter, StringTag#quoteAndEscape, TagParser (1.21.11)
use crate::wire::Reader;
use crate::{ProtocolError, ProtocolResult};

/// VANILLA: NbtAccounter.MAX_DEPTH
pub const MAX_DEPTH: usize = 512;

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    /// Элементы одного типа (проверяется при чтении/записи).
    List(Vec<Tag>),
    /// Порядок вставки сохраняется.
    Compound(Vec<(String, Tag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }
    pub fn get(&self, key: &str) -> Option<&Tag> {
        match self {
            Tag::Compound(v) => v.iter().find(|(k, _)| k == key).map(|(_, t)| t),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Tag::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Tag::Byte(v) => Some(i64::from(*v)),
            Tag::Short(v) => Some(i64::from(*v)),
            Tag::Int(v) => Some(i64::from(*v)),
            Tag::Long(v) => Some(*v),
            _ => None,
        }
    }
}

fn err(what: &'static str, value: i64) -> ProtocolError {
    ProtocolError::InvalidValue { what, value }
}

// ---------- Modified UTF-8 ----------

fn read_mutf8(r: &mut Reader) -> ProtocolResult<String> {
    let n = usize::from(r.u16()?);
    let b = r.bytes(n)?;
    let mut units: Vec<u16> = Vec::with_capacity(n);
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        let (u, len) = match c {
            0x01..=0x7F => (u16::from(c), 1),
            0xC0..=0xDF => {
                let c2 = *b.get(i + 1).ok_or(ProtocolError::BadUtf8)?;
                if c2 & 0xC0 != 0x80 {
                    return Err(ProtocolError::BadUtf8);
                }
                ((u16::from(c & 0x1F) << 6) | u16::from(c2 & 0x3F), 2)
            }
            0xE0..=0xEF => {
                let (c2, c3) = (
                    *b.get(i + 1).ok_or(ProtocolError::BadUtf8)?,
                    *b.get(i + 2).ok_or(ProtocolError::BadUtf8)?,
                );
                if c2 & 0xC0 != 0x80 || c3 & 0xC0 != 0x80 {
                    return Err(ProtocolError::BadUtf8);
                }
                (
                    (u16::from(c & 0x0F) << 12)
                        | (u16::from(c2 & 0x3F) << 6)
                        | u16::from(c3 & 0x3F),
                    3,
                )
            }
            _ => return Err(ProtocolError::BadUtf8),
        };
        units.push(u);
        i += len;
    }
    String::from_utf16(&units).map_err(|_| ProtocolError::BadUtf8)
}

fn write_mutf8(out: &mut Vec<u8>, s: &str) -> ProtocolResult<()> {
    let mut tmp = Vec::with_capacity(s.len());
    for u in s.encode_utf16() {
        match u {
            0x01..=0x7F => tmp.push(u as u8),
            0 | 0x80..=0x7FF => {
                tmp.push(0xC0 | (u >> 6) as u8);
                tmp.push(0x80 | (u & 0x3F) as u8);
            }
            _ => {
                tmp.push(0xE0 | (u >> 12) as u8);
                tmp.push(0x80 | ((u >> 6) & 0x3F) as u8);
                tmp.push(0x80 | (u & 0x3F) as u8);
            }
        }
    }
    let n = u16::try_from(tmp.len()).map_err(|_| ProtocolError::TooLarge {
        what: "nbt string",
        len: tmp.len(),
        limit: 65535,
    })?;
    out.extend_from_slice(&n.to_be_bytes());
    out.extend_from_slice(&tmp);
    Ok(())
}

// ---------- бинарное чтение ----------

/// Сетевой формат 1.20.2+: корень без имени (`type` + payload).
pub fn read_network(r: &mut Reader) -> ProtocolResult<Option<Tag>> {
    let id = r.u8()?;
    if id == 0 {
        return Ok(None); // TAG_End — «пустой» NBT
    }
    read_payload(r, id, 0).map(Some)
}

/// Файловый/старый сетевой формат: корень с именем.
pub fn read_named(r: &mut Reader) -> ProtocolResult<Option<(String, Tag)>> {
    let id = r.u8()?;
    if id == 0 {
        return Ok(None);
    }
    let name = read_mutf8(r)?;
    Ok(Some((name, read_payload(r, id, 0)?)))
}

fn check_len(r: &Reader, n: i32, elem: usize, what: &'static str) -> ProtocolResult<usize> {
    if n < 0 {
        return Err(ProtocolError::NegativeLength(i64::from(n)));
    }
    let n = n as usize;
    // Не выделяем память больше, чем реально осталось данных.
    if n.saturating_mul(elem) > r.remaining() {
        return Err(ProtocolError::TooLarge {
            what,
            len: n,
            limit: r.remaining() / elem.max(1),
        });
    }
    Ok(n)
}

fn read_payload(r: &mut Reader, id: u8, depth: usize) -> ProtocolResult<Tag> {
    if depth > MAX_DEPTH {
        return Err(ProtocolError::TooLarge {
            what: "nbt depth",
            len: depth,
            limit: MAX_DEPTH,
        });
    }
    Ok(match id {
        1 => Tag::Byte(r.i8()?),
        2 => Tag::Short(r.i16()?),
        3 => Tag::Int(r.i32()?),
        4 => Tag::Long(r.i64()?),
        5 => Tag::Float(r.f32()?),
        6 => Tag::Double(r.f64()?),
        7 => {
            let len = r.i32()?;
            let n = check_len(r, len, 1, "byte array")?;
            Tag::ByteArray(r.bytes(n)?.iter().map(|&b| b as i8).collect())
        }
        8 => Tag::String(read_mutf8(r)?),
        9 => {
            let elem = r.u8()?;
            let n = r.i32()?;
            if elem == 0 {
                // VANILLA: пустой список с типом End; непустой — ошибка
                if n > 0 {
                    return Err(err("nbt list of end", i64::from(n)));
                }
                return Ok(Tag::List(Vec::new()));
            }
            if elem > 12 {
                return Err(err("nbt tag id", i64::from(elem)));
            }
            let n = check_len(r, n, 1, "list")?;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(read_payload(r, elem, depth + 1)?);
            }
            Tag::List(v)
        }
        10 => {
            let mut v: Vec<(String, Tag)> = Vec::new();
            loop {
                let id = r.u8()?;
                if id == 0 {
                    break;
                }
                let k = read_mutf8(r)?;
                let t = read_payload(r, id, depth + 1)?;
                // VANILLA: при повторе ключа побеждает последний
                if let Some(slot) = v.iter_mut().find(|(kk, _)| *kk == k) {
                    slot.1 = t;
                } else {
                    v.push((k, t));
                }
            }
            Tag::Compound(v)
        }
        11 => {
            let len = r.i32()?;
            let n = check_len(r, len, 4, "int array")?;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(r.i32()?);
            }
            Tag::IntArray(v)
        }
        12 => {
            let len = r.i32()?;
            let n = check_len(r, len, 8, "long array")?;
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(r.i64()?);
            }
            Tag::LongArray(v)
        }
        other => return Err(err("nbt tag id", i64::from(other))),
    })
}

// ---------- бинарная запись ----------

pub fn write_network(out: &mut Vec<u8>, tag: Option<&Tag>) -> ProtocolResult<()> {
    match tag {
        None => out.push(0),
        Some(t) => {
            out.push(t.id());
            write_payload(out, t, 0)?;
        }
    }
    Ok(())
}

pub fn write_named(out: &mut Vec<u8>, name: &str, tag: &Tag) -> ProtocolResult<()> {
    out.push(tag.id());
    write_mutf8(out, name)?;
    write_payload(out, tag, 0)
}

fn write_len(out: &mut Vec<u8>, n: usize) -> ProtocolResult<()> {
    let n = i32::try_from(n).map_err(|_| ProtocolError::TooLarge {
        what: "nbt array",
        len: n,
        limit: i32::MAX as usize,
    })?;
    out.extend_from_slice(&n.to_be_bytes());
    Ok(())
}

fn write_payload(out: &mut Vec<u8>, t: &Tag, depth: usize) -> ProtocolResult<()> {
    if depth > MAX_DEPTH {
        return Err(ProtocolError::TooLarge {
            what: "nbt depth",
            len: depth,
            limit: MAX_DEPTH,
        });
    }
    match t {
        Tag::Byte(v) => out.push(*v as u8),
        Tag::Short(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Int(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Long(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Double(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::ByteArray(v) => {
            write_len(out, v.len())?;
            out.extend(v.iter().map(|&b| b as u8));
        }
        Tag::String(s) => write_mutf8(out, s)?,
        Tag::List(v) => {
            let id = v.first().map_or(0, Tag::id);
            if v.iter().any(|e| e.id() != id) {
                return Err(err("nbt list mixed types", i64::from(id)));
            }
            out.push(id);
            write_len(out, v.len())?;
            for e in v {
                write_payload(out, e, depth + 1)?;
            }
        }
        Tag::Compound(v) => {
            for (k, e) in v {
                out.push(e.id());
                write_mutf8(out, k)?;
                write_payload(out, e, depth + 1)?;
            }
            out.push(0);
        }
        Tag::IntArray(v) => {
            write_len(out, v.len())?;
            for x in v {
                out.extend_from_slice(&x.to_be_bytes());
            }
        }
        Tag::LongArray(v) => {
            write_len(out, v.len())?;
            for x in v {
                out.extend_from_slice(&x.to_be_bytes());
            }
        }
    }
    Ok(())
}

// ---------- SNBT: вывод ----------

fn is_simple_key(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.' | b'+'))
}

/// VANILLA: StringTag#quoteAndEscape — кавычка выбирается так, чтобы экранировать меньше.
pub fn quote_and_escape(s: &str) -> String {
    let mut quote = '"';
    let mut out = String::with_capacity(s.len() + 2);
    let mut body = String::new();
    for c in s.chars() {
        match c {
            '\\' => body.push_str("\\\\"),
            '"' | '\'' => {
                if quote == '"' && c == '"' && !s.contains('\'') {
                    quote = '\'';
                }
                if c == quote {
                    body.push('\\');
                }
                body.push(c);
            }
            _ => body.push(c),
        }
    }
    out.push(quote);
    out.push_str(&body);
    out.push(quote);
    out
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Byte(v) => write!(f, "{v}b"),
            Tag::Short(v) => write!(f, "{v}s"),
            Tag::Int(v) => write!(f, "{v}"),
            Tag::Long(v) => write!(f, "{v}L"),
            Tag::Float(v) => write!(f, "{v:?}f"),
            Tag::Double(v) => write!(f, "{v:?}d"),
            Tag::String(s) => f.write_str(&quote_and_escape(s)),
            Tag::ByteArray(v) => {
                f.write_str("[B;")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}{x}b", if i > 0 { "," } else { "" })?;
                }
                f.write_str("]")
            }
            Tag::IntArray(v) => {
                f.write_str("[I;")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}{x}", if i > 0 { "," } else { "" })?;
                }
                f.write_str("]")
            }
            Tag::LongArray(v) => {
                f.write_str("[L;")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}{x}L", if i > 0 { "," } else { "" })?;
                }
                f.write_str("]")
            }
            Tag::List(v) => {
                f.write_str("[")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}{x}", if i > 0 { "," } else { "" })?;
                }
                f.write_str("]")
            }
            Tag::Compound(v) => {
                f.write_str("{")?;
                for (i, (k, x)) in v.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    if is_simple_key(k) {
                        f.write_str(k)?;
                    } else {
                        f.write_str(&quote_and_escape(k))?;
                    }
                    write!(f, ":{x}")?;
                }
                f.write_str("}")
            }
        }
    }
}

// ---------- SNBT: разбор ----------

pub fn parse_snbt(s: &str) -> ProtocolResult<Tag> {
    let mut p = Snbt {
        s: s.as_bytes(),
        src: s,
        pos: 0,
    };
    p.ws();
    let t = p.value(0)?;
    p.ws();
    if p.pos != p.s.len() {
        return Err(p.fail("лишние символы после значения"));
    }
    Ok(t)
}

struct Snbt<'a> {
    s: &'a [u8],
    src: &'a str,
    pos: usize,
}

impl Snbt<'_> {
    fn fail(&self, msg: &str) -> ProtocolError {
        ProtocolError::Other(format!("SNBT: {msg} (позиция {})", self.pos))
    }
    fn ws(&mut self) {
        while self.pos < self.s.len() && self.s[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }
    fn expect(&mut self, c: u8) -> ProtocolResult<()> {
        self.ws();
        if self.peek() == Some(c) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.fail(&format!("ожидалось '{}'", c as char)))
        }
    }

    fn value(&mut self, depth: usize) -> ProtocolResult<Tag> {
        if depth > MAX_DEPTH {
            return Err(self.fail("слишком глубокая вложенность"));
        }
        self.ws();
        match self.peek() {
            None => Err(self.fail("неожиданный конец")),
            Some(b'{') => self.compound(depth),
            Some(b'[') => self.list(depth),
            Some(b'"') | Some(b'\'') => Ok(Tag::String(self.quoted()?)),
            Some(_) => {
                let w = self.unquoted();
                if w.is_empty() {
                    return Err(self.fail("неожиданный символ"));
                }
                Ok(parse_scalar(&w))
            }
        }
    }

    fn unquoted(&mut self) -> String {
        let st = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.' | b'+') {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.src[st..self.pos].to_owned()
    }

    fn quoted(&mut self) -> ProtocolResult<String> {
        let q = self.s[self.pos];
        self.pos += 1;
        let mut out = String::new();
        loop {
            let rest = &self.src[self.pos..];
            let c = rest
                .chars()
                .next()
                .ok_or_else(|| self.fail("незакрытая строка"))?;
            self.pos += c.len_utf8();
            if c as u32 == u32::from(q) {
                return Ok(out);
            }
            if c == '\\' {
                let e = self.src[self.pos..]
                    .chars()
                    .next()
                    .ok_or_else(|| self.fail("обрыв после \\"))?;
                self.pos += e.len_utf8();
                match e {
                    '\\' | '"' | '\'' => out.push(e),
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    's' => out.push(' '),
                    _ => return Err(self.fail("неверная escape-последовательность")),
                }
            } else {
                out.push(c);
            }
        }
    }

    fn key(&mut self) -> ProtocolResult<String> {
        self.ws();
        match self.peek() {
            Some(b'"') | Some(b'\'') => self.quoted(),
            _ => {
                let k = self.unquoted();
                if k.is_empty() {
                    Err(self.fail("пустой ключ"))
                } else {
                    Ok(k)
                }
            }
        }
    }

    fn compound(&mut self, depth: usize) -> ProtocolResult<Tag> {
        self.pos += 1;
        let mut v: Vec<(String, Tag)> = Vec::new();
        self.ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(Tag::Compound(v));
        }
        loop {
            let k = self.key()?;
            self.expect(b':')?;
            let t = self.value(depth + 1)?;
            if let Some(slot) = v.iter_mut().find(|(kk, _)| *kk == k) {
                slot.1 = t;
            } else {
                v.push((k, t));
            }
            self.ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Tag::Compound(v));
                }
                _ => return Err(self.fail("ожидалось ',' или '}'")),
            }
        }
    }

    fn list(&mut self, depth: usize) -> ProtocolResult<Tag> {
        self.pos += 1;
        self.ws();
        // типизированные массивы: [B; [I; [L;
        if let (Some(t @ (b'B' | b'I' | b'L')), Some(b';')) =
            (self.peek(), self.s.get(self.pos + 1).copied())
        {
            self.pos += 2;
            let mut items: Vec<Tag> = Vec::new();
            self.ws();
            if self.peek() == Some(b']') {
                self.pos += 1;
            } else {
                loop {
                    items.push(self.value(depth + 1)?);
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.pos += 1,
                        Some(b']') => {
                            self.pos += 1;
                            break;
                        }
                        _ => return Err(self.fail("ожидалось ',' или ']'")),
                    }
                }
            }
            return match t {
                b'B' => items
                    .iter()
                    .map(|x| match x {
                        Tag::Byte(v) => Ok(*v),
                        _ => Err(self.fail("в [B; ожидались byte")),
                    })
                    .collect::<Result<_, _>>()
                    .map(Tag::ByteArray),
                b'I' => items
                    .iter()
                    .map(|x| match x {
                        Tag::Int(v) => Ok(*v),
                        _ => Err(self.fail("в [I; ожидались int")),
                    })
                    .collect::<Result<_, _>>()
                    .map(Tag::IntArray),
                _ => items
                    .iter()
                    .map(|x| match x {
                        Tag::Long(v) => Ok(*v),
                        _ => Err(self.fail("в [L; ожидались long")),
                    })
                    .collect::<Result<_, _>>()
                    .map(Tag::LongArray),
            };
        }
        let mut v = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(Tag::List(v));
        }
        loop {
            v.push(self.value(depth + 1)?);
            self.ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(self.fail("ожидалось ',' или ']'")),
            }
        }
        // VANILLA 1.21.5+: разнотипные списки допустимы в SNBT (оборачиваются в compound);
        // в бинарном формате — нет, поэтому требуем однотипность.
        let id = v[0].id();
        if v.iter().any(|e| e.id() != id) {
            return Err(self.fail("разнотипные элементы списка не поддерживаются"));
        }
        Ok(Tag::List(v))
    }
}

/// Скаляр без кавычек: суффиксы b/s/L/f/d, true/false, иначе строка.
fn parse_scalar(w: &str) -> Tag {
    match w {
        "true" => return Tag::Byte(1),
        "false" => return Tag::Byte(0),
        _ => {}
    }
    let last = w.as_bytes()[w.len() - 1].to_ascii_lowercase();
    let body = &w[..w.len() - 1];
    let int_like = |s: &str| {
        let d = s.strip_prefix(['-', '+']).unwrap_or(s);
        !d.is_empty() && d.bytes().all(|c| c.is_ascii_digit())
    };
    let float_like = |s: &str| {
        let d = s.strip_prefix(['-', '+']).unwrap_or(s);
        d.bytes().any(|c| c.is_ascii_digit())
            && d.bytes()
                .all(|c| c.is_ascii_digit() || matches!(c, b'.' | b'e' | b'E' | b'-' | b'+'))
            && d.parse::<f64>().is_ok()
    };
    match last {
        b'b' if int_like(body) => body.parse().map(Tag::Byte).ok(),
        b's' if int_like(body) => body.parse().map(Tag::Short).ok(),
        b'l' if int_like(body) => body.parse().map(Tag::Long).ok(),
        b'f' if float_like(body) => body.parse().map(Tag::Float).ok(),
        b'd' if float_like(body) => body.parse().map(Tag::Double).ok(),
        _ if int_like(w) => w.parse().map(Tag::Int).ok(),
        _ if float_like(w) && w.contains('.') => w.parse().map(Tag::Double).ok(),
        _ => None,
    }
    .unwrap_or_else(|| Tag::String(w.to_owned()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample() -> Tag {
        Tag::Compound(vec![
            ("name".into(), Tag::String("Ünï\0😀".into())),
            ("b".into(), Tag::Byte(-3)),
            ("s".into(), Tag::Short(300)),
            ("i".into(), Tag::Int(-70000)),
            ("l".into(), Tag::Long(1 << 40)),
            ("f".into(), Tag::Float(1.5)),
            ("d".into(), Tag::Double(-2.25)),
            ("ba".into(), Tag::ByteArray(vec![1, -2])),
            ("ia".into(), Tag::IntArray(vec![1, 2, 3])),
            ("la".into(), Tag::LongArray(vec![i64::MIN])),
            ("list".into(), Tag::List(vec![Tag::Int(1), Tag::Int(2)])),
            ("empty".into(), Tag::List(vec![])),
            (
                "nested".into(),
                Tag::Compound(vec![("x".into(), Tag::List(vec![Tag::Compound(vec![])]))]),
            ),
        ])
    }

    #[test]
    fn binary_roundtrip_network_and_named() {
        let t = sample();
        let mut b = Vec::new();
        write_network(&mut b, Some(&t)).unwrap();
        assert_eq!(b[0], 10);
        assert_eq!(read_network(&mut Reader::new(&b)).unwrap(), Some(t.clone()));
        let mut b = Vec::new();
        write_named(&mut b, "root", &t).unwrap();
        assert_eq!(
            read_named(&mut Reader::new(&b)).unwrap(),
            Some(("root".into(), t))
        );
        assert_eq!(read_network(&mut Reader::new(&[0])).unwrap(), None);
    }

    #[test]
    fn mutf8_nul_and_surrogates() {
        let mut b = Vec::new();
        write_mutf8(&mut b, "\0😀").unwrap();
        assert_eq!(b, [0, 8, 0xC0, 0x80, 0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]);
        assert_eq!(read_mutf8(&mut Reader::new(&b)).unwrap(), "\0😀");
    }

    #[test]
    fn hostile_input_is_error() {
        // массив на 2 ГБ при пустом хвосте
        let b = [7, 0x7f, 0xff, 0xff, 0xff];
        assert!(read_network(&mut Reader::new(&b)).is_err());
        // список с отрицательной длиной
        assert!(read_network(&mut Reader::new(&[9, 3, 0xff, 0xff, 0xff, 0xff])).is_err());
        // глубина
        let mut deep = vec![9u8, 9, 0, 0, 0, 1];
        for _ in 0..600 {
            deep.extend_from_slice(&[9, 0, 0, 0, 1]);
        }
        assert!(read_network(&mut Reader::new(&deep)).is_err());
        assert!(read_network(&mut Reader::new(&[99])).is_err());
        assert!(read_network(&mut Reader::new(&[8, 0, 5, b'a'])).is_err());
    }

    #[test]
    fn mixed_list_rejected_on_write() {
        let mut b = Vec::new();
        assert!(write_network(&mut b, Some(&Tag::List(vec![Tag::Int(1), Tag::Byte(1)]))).is_err());
    }

    #[test]
    fn snbt_roundtrip() {
        let t = sample();
        let s = t.to_string();
        assert_eq!(parse_snbt(&s).unwrap(), t, "{s}");
    }

    #[test]
    fn snbt_vectors() {
        assert_eq!(
            parse_snbt("{a:1b, 'b c':2s,\"d\":[I;1,2], e:true, f:1.5, g:3f, h:4L, i:foo}").unwrap(),
            Tag::Compound(vec![
                ("a".into(), Tag::Byte(1)),
                ("b c".into(), Tag::Short(2)),
                ("d".into(), Tag::IntArray(vec![1, 2])),
                ("e".into(), Tag::Byte(1)),
                ("f".into(), Tag::Double(1.5)),
                ("g".into(), Tag::Float(3.0)),
                ("h".into(), Tag::Long(4)),
                ("i".into(), Tag::String("foo".into())),
            ])
        );
        assert_eq!(quote_and_escape("a\"b"), "'a\"b'");
        assert_eq!(quote_and_escape("it's"), "\"it's\"");
        assert_eq!(quote_and_escape("a\"b'c"), "\"a\\\"b'c\"");
        assert!(parse_snbt("{a:1").is_err());
        assert!(parse_snbt("{a:1} x").is_err());
        assert!(parse_snbt("'abc").is_err());
        assert_eq!(parse_snbt("128b").unwrap(), Tag::String("128b".into())); // не влезает в byte
    }
}
