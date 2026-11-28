//! Переводы: `assets/minecraft/lang/en_us.json` (VANILLA: Language#getOrDefault; нет ключа → сам ключ).
use std::collections::HashMap;
use std::sync::Mutex;

/// Таблица ключей и код языка (`en_us`, `ru_ru`, …) — по коду ищется старый `.lang`.
#[derive(Default)]
pub struct Lang(HashMap<String, String>, String);

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
        Self(m, "en_us".to_owned())
    }

    /// Код выбранного языка (для запасного `.lang` старых серверов).
    pub fn with_code(mut self, code: &str) -> Self {
        code.clone_into(&mut self.1);
        self
    }

    /// VANILLA: ClientLanguage#loadFrom — языки стопкой, выбранный перекрывает en_us.
    pub fn merge(&mut self, json: &[u8]) {
        if let Ok(m) = serde_json::from_slice::<HashMap<String, String>>(json) {
            self.0.extend(m);
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .or_else(|| legacy(&self.1)?.get(key))
            .map(String::as_str)
    }

    /// Ключ из компонента сервера: у серверов ≤1.12.2 свой `.lang` главнее (тот же ключ мог
    /// сменить формат, напр. `commands.summon.success` без аргументов в 1.12).
    pub fn server(&self, key: &str) -> Option<&str> {
        legacy(&self.1)
            .and_then(|m| m.get(key))
            .map(String::as_str)
            .or_else(|| self.get(key))
    }

    pub fn tr(&self, key: &str) -> String {
        // Ключи модов Forge — из переводов дампа `.ezbake` v3 (язык игры, где пекли).
        self.get(key)
            .or_else(|| ezzmc_assets::modbake::global()?.tr(key))
            .map(str::to_owned)
            .unwrap_or_else(|| key.to_owned())
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

/// Индексы ассетов версий с `.lang` (новые первыми) и самый старый протокол каждой.
const LEGACY_INDEXES: [(i32, &str); 7] = [
    (335, "1.12"),
    (315, "1.11"),
    (210, "1.10"),
    (107, "1.9"),
    (47, "1.8"),
    (4, "1.7.10"),
    (i32::MIN, "legacy"), // 1.6.4 и старше (протокол pre-Netty отрицательный)
];

/// Серверы ≤1.12.2 шлют ключи своего `.lang` (команды перестроены в 1.13), в языке 26.3 их нет.
/// VANILLA: клиент 1.12 переводит своим `minecraft/lang/<code>.lang`; здесь он — запас только для
/// отсутствующих ключей. Индекс берётся ближайший к версии из установленных (`assets/indexes`).
/// VANILLA-UNSURE: при другом индексе тексты могут отличаться от той версии.
fn legacy(code: &str) -> Option<&'static HashMap<String, String>> {
    type Cache = Vec<(
        &'static str,
        String,
        Option<&'static HashMap<String, String>>,
    )>;
    static CACHE: Mutex<Cache> = Mutex::new(Vec::new());
    let p = ezzmc_protocol::version::current().protocol;
    if p > 340 {
        return None;
    }
    let want = LEGACY_INDEXES.iter().position(|(min, _)| p >= *min)?;
    let mut cache = CACHE.lock().ok()?;
    let id = LEGACY_INDEXES[want].1;
    if let Some(e) = cache.iter().find(|e| e.0 == id && e.1 == code) {
        return e.2;
    }
    let assets = ezzmc_assets::install::minecraft_dirs()
        .into_iter()
        .map(|d| d.join("assets"))
        .find(|d| d.join("indexes").is_dir());
    // ближайший установленный: сначала нужный и старше, потом новее
    let order = (want..LEGACY_INDEXES.len()).chain((0..want).rev());
    let map = assets.and_then(|a| {
        order
            .filter_map(|i| load_lang(&a, LEGACY_INDEXES[i].1, code))
            .next()
    });
    if map.is_none() {
        tracing::debug!("нет старого .lang для протокола {p} ({code})");
    }
    let map: Option<&'static HashMap<String, String>> = map.map(|m| &*Box::leak(Box::new(m)));
    cache.push((id, code.to_owned(), map));
    map
}

/// `en_us.lang`, поверх него `<code>.lang` из индекса `id`; до 1.11 имена вида `en_US.lang`.
fn load_lang(assets: &std::path::Path, id: &str, code: &str) -> Option<HashMap<String, String>> {
    let idx = std::fs::read(assets.join("indexes").join(format!("{id}.json"))).ok()?;
    let idx: serde_json::Value = serde_json::from_slice(&idx).ok()?;
    let objects = idx.get("objects")?;
    let read = |code: &str| -> Option<String> {
        let upper = match code.split_once('_') {
            Some((l, r)) => format!("{l}_{}", r.to_ascii_uppercase()),
            None => code.to_owned(),
        };
        let hash = [code, upper.as_str()].into_iter().find_map(|c| {
            objects
                .get(format!("minecraft/lang/{c}.lang"))?
                .get("hash")?
                .as_str()
        })?;
        let path = assets.join("objects").join(hash.get(..2)?).join(hash);
        String::from_utf8(std::fs::read(path).ok()?).ok()
    };
    let mut m = HashMap::new();
    // en_us лежит не в индексе, а в jar версии (`versions/<id>*/<ver>.jar`)
    if let Some(text) = jar_en_us(assets, id).or_else(|| read("en_us")) {
        m.extend(parse_lang(&text));
    }
    if code != "en_us" {
        if let Some(text) = read(code) {
            m.extend(parse_lang(&text));
        }
    }
    (!m.is_empty()).then_some(m)
}

/// `assets/minecraft/lang/en_us.lang` (`en_US.lang` до 1.11) из установленного jar семейства `id`.
fn jar_en_us(assets: &std::path::Path, id: &str) -> Option<String> {
    use ezzmc_assets::source::ResourceSource;
    let versions = assets.parent()?.join("versions");
    let mut dirs: Vec<_> = std::fs::read_dir(versions)
        .ok()?
        .flatten()
        .filter(|e| {
            let n = e.file_name();
            let n = n.to_string_lossy();
            n == id || n.strip_prefix(id).is_some_and(|r| r.starts_with('.'))
        })
        .collect();
    dirs.sort_by_key(|e| std::cmp::Reverse(e.file_name()));
    dirs.iter().find_map(|e| {
        let n = e.file_name();
        let jar = e.path().join(format!("{}.jar", n.to_string_lossy()));
        let src = ezzmc_assets::source::ZipSource::open(&jar).ok()?;
        let b = src
            .read("assets/minecraft/lang/en_us.lang")
            .or_else(|| src.read("assets/minecraft/lang/en_US.lang"))?;
        String::from_utf8(b).ok()
    })
}

/// VANILLA: Locale#loadLocaleData (1.12) — строки `key=value`, `#` — комментарий.
fn parse_lang(text: &str) -> impl Iterator<Item = (String, String)> + '_ {
    text.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let (k, v) = l.split_once('=')?;
            Some((k.trim().to_owned(), v.trim_end_matches('\r').to_owned()))
        })
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

    #[test]
    fn legacy_lang_lines() {
        let m: HashMap<_, _> =
            parse_lang("# c\ncommands.kill.successful=Killed %s\r\nx=a=b\n").collect();
        assert_eq!(m["commands.kill.successful"], "Killed %s");
        assert_eq!(m["x"], "a=b");
        assert_eq!(m.len(), 2);
    }
}
