//! Переводы: `assets/minecraft/lang/en_us.json` (VANILLA: Language#getOrDefault; нет ключа → сам ключ).
use std::collections::HashMap;

#[derive(Default)]
pub struct Lang(HashMap<String, String>);

impl Lang {
    pub fn load(json: &[u8]) -> Self {
        let mut m: HashMap<String, String> = serde_json::from_slice(json).unwrap_or_default();
        // Не ванильная клавиша зума: названия нет в en_us.json.
        m.entry("key.zoom".to_owned())
            .or_insert_with(|| "Zoom".to_owned());
        // Не ванильные строки: переключатель режима рендера.
        m.entry("key.renderMode".to_owned())
            .or_insert_with(|| "Toggle Render Mode".to_owned());
        m.entry("options.ezzmc.computeRenderer".to_owned())
            .or_insert_with(|| "Compute Renderer".to_owned());
        Self(m)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn tr(&self, key: &str) -> String {
        self.0.get(key).cloned().unwrap_or_else(|| key.to_owned())
    }

    /// VANILLA: `%s` / `%d` подставляются по порядку, `%%` → `%`.
    pub fn fmt(&self, key: &str, args: &[&str]) -> String {
        let f = self.tr(key);
        let mut out = String::new();
        let mut it = f.chars().peekable();
        let mut n = 0;
        while let Some(c) = it.next() {
            if c != '%' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('%') => out.push('%'),
                Some('s' | 'd') => {
                    out.push_str(args.get(n).copied().unwrap_or(""));
                    n += 1;
                }
                Some(d) if d.is_ascii_digit() => {
                    // VANILLA: String.format — позиционный аргумент `%N$s`.
                    let mut idx = d.to_digit(10).unwrap_or(1) as usize;
                    while let Some(x) = it.next_if(char::is_ascii_digit) {
                        idx = idx * 10 + x.to_digit(10).unwrap_or(0) as usize;
                    }
                    if it.next_if_eq(&'$').is_some()
                        && it.next_if(|c| matches!(c, 's' | 'd')).is_some()
                    {
                        out.push_str(args.get(idx.wrapping_sub(1)).copied().unwrap_or(""));
                    }
                }
                Some(o) => {
                    out.push('%');
                    out.push(o);
                }
                None => out.push('%'),
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_value() {
        let l = Lang::load(br#"{"options.generic_value":"%s: %s"}"#);
        assert_eq!(l.fmt("options.generic_value", &["FOV", "70"]), "FOV: 70");
        assert_eq!(l.tr("missing.key"), "missing.key");
    }
}
