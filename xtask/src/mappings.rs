//! Конвертер официальных маппингов Mojang (ProGuard `client.txt`) → Tiny v1 (obf → named).
//! Нужен, чтобы tiny-remapper мог деобфусцировать официальный client.jar (GOAL.md §2А).
use std::collections::HashMap;
use std::fmt::Write;

fn primitive(t: &str) -> Option<&'static str> {
    Some(match t {
        "byte" => "B",
        "char" => "C",
        "double" => "D",
        "float" => "F",
        "int" => "I",
        "long" => "J",
        "short" => "S",
        "boolean" => "Z",
        "void" => "V",
        _ => return None,
    })
}

/// Тип Java (`a.b.C[]`, `int`) в дескриптор с обфусцированными именами классов.
fn desc(ty: &str, obf: &HashMap<String, String>) -> String {
    let mut base = ty;
    let mut dims = 0;
    while let Some(b) = base.strip_suffix("[]") {
        base = b;
        dims += 1;
    }
    let mut out = "[".repeat(dims);
    match primitive(base) {
        Some(p) => out.push_str(p),
        None => {
            let internal = base.replace('.', "/");
            let name = obf.get(base).cloned().unwrap_or(internal);
            out.push('L');
            out.push_str(&name);
            out.push(';');
        }
    }
    out
}

/// Возвращает текст Tiny v1 (`official` → `named`).
pub fn proguard_to_tiny(src: &str) -> Result<String, String> {
    // Проход 1: именованный класс → обфусцированный (внутренние имена со слэшами).
    let mut obf: HashMap<String, String> = HashMap::new();
    for line in src.lines() {
        if line.starts_with('#') || line.starts_with(' ') || line.is_empty() {
            continue;
        }
        let (named, o) = line
            .strip_suffix(':')
            .and_then(|l| l.split_once(" -> "))
            .ok_or_else(|| format!("плохая строка класса: {line}"))?;
        obf.insert(named.to_string(), o.replace('.', "/"));
    }
    // Проход 2: члены.
    let mut out = String::from("v1\tofficial\tnamed\n");
    let mut cur: Option<(&str, &str)> = None; // (named, obf)
    let mut body = String::new();
    for line in src.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if !line.starts_with(' ') {
            let (named, o) = line
                .strip_suffix(':')
                .and_then(|l| l.split_once(" -> "))
                .ok_or_else(|| format!("плохая строка класса: {line}"))?;
            cur = Some((named, o));
            let _ = writeln!(
                out,
                "CLASS\t{}\t{}",
                o.replace('.', "/"),
                named.replace('.', "/")
            );
            continue;
        }
        let Some((_, cobf)) = cur else {
            return Err("член вне класса".into());
        };
        let cobf = cobf.replace('.', "/");
        let (lhs, o) = line
            .trim()
            .split_once(" -> ")
            .ok_or_else(|| format!("плохая строка члена: {line}"))?;
        // убираем префикс номеров строк `12:34:`
        let mut lhs = lhs;
        while let Some((pre, rest)) = lhs.split_once(':') {
            if pre.chars().all(|c| c.is_ascii_digit()) {
                lhs = rest;
            } else {
                break;
            }
        }
        let (ty, rest) = lhs
            .split_once(' ')
            .ok_or_else(|| format!("плохой член: {line}"))?;
        if let Some((name, args)) = rest.split_once('(') {
            let args = args.trim_end_matches(')');
            let mut d = String::from("(");
            if !args.is_empty() {
                for a in args.split(',') {
                    d.push_str(&desc(a, &obf));
                }
            }
            d.push(')');
            d.push_str(&desc(ty, &obf));
            let _ = writeln!(body, "METHOD\t{cobf}\t{d}\t{o}\t{name}");
        } else {
            let _ = writeln!(body, "FIELD\t{cobf}\t{}\t{o}\t{rest}", desc(ty, &obf));
        }
    }
    out.push_str(&body);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_class_field_method() {
        let src = "# c\n\
            a.b.Foo -> aa:\n\
            \x20   int count -> a\n\
            \x20   10:12:a.b.Foo[] make(a.b.Foo,long,java.lang.String) -> b\n\
            a.b.Bar -> bb:\n";
        let t = proguard_to_tiny(src).unwrap_or_default();
        assert!(t.contains("CLASS\taa\ta/b/Foo\n"));
        assert!(t.contains("FIELD\taa\tI\ta\tcount\n"));
        assert!(t.contains("METHOD\taa\t(Laa;JLjava/lang/String;)[Laa;\tb\tmake\n"));
    }
}
