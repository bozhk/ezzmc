//! Системный буфер обмена (TODO-043) для чата и полей ввода.
//! VANILLA: com.mojang.blaze3d.platform.ClipboardManager — ошибки/отсутствие текста → пустая строка.
#![allow(dead_code)]

/// Источник текста; абстракция нужна для тестов и для окружений без буфера (headless).
pub trait TextClipboard {
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, text: &str) -> bool;
}

struct System(arboard::Clipboard);

impl TextClipboard for System {
    fn get(&mut self) -> Option<String> {
        self.0.get_text().ok()
    }
    fn set(&mut self, text: &str) -> bool {
        self.0.set_text(text.to_owned()).is_ok()
    }
}

/// Буфер обмена клиента. Системный создаётся лениво и живёт вместе с клиентом
/// (на X11 владелец выделения должен оставаться живым, пока другие читают).
#[derive(Default)]
pub struct Clipboard {
    backend: Option<Box<dyn TextClipboard>>,
    tried: bool,
    /// Запасной локальный буфер, если системного нет — копирование внутри клиента продолжает работать.
    local: String,
}

impl Clipboard {
    pub fn with_backend(b: Box<dyn TextClipboard>) -> Self {
        Self {
            backend: Some(b),
            tried: true,
            local: String::new(),
        }
    }

    fn backend(&mut self) -> Option<&mut Box<dyn TextClipboard>> {
        if !self.tried {
            self.tried = true;
            match arboard::Clipboard::new() {
                Ok(c) => self.backend = Some(Box::new(System(c))),
                Err(e) => tracing::warn!("буфер обмена недоступен: {e}"),
            }
        }
        self.backend.as_mut()
    }

    /// VANILLA: getClipboard — при любой ошибке пустая строка.
    pub fn get(&mut self) -> String {
        match self.backend().and_then(|b| b.get()) {
            Some(s) => s,
            None => self.local.clone(),
        }
    }

    /// VANILLA: setClipboard.
    pub fn set(&mut self, text: &str) {
        self.local = text.to_owned();
        if let Some(b) = self.backend() {
            if !b.set(text) {
                tracing::warn!("не удалось записать в буфер обмена");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// Живая проверка системного буфера: `cargo test -- --ignored clipboard`.
    #[test]
    #[ignore = "нужен графический сеанс"]
    fn system_roundtrip() {
        let mut c = super::Clipboard::default();
        c.set("ezzmc-clip-test");
        assert_eq!(c.get(), "ezzmc-clip-test");
    }

    use super::*;

    #[derive(Default)]
    struct Mem(Option<String>, bool);
    impl TextClipboard for Mem {
        fn get(&mut self) -> Option<String> {
            self.0.clone()
        }
        fn set(&mut self, t: &str) -> bool {
            if self.1 {
                return false;
            }
            self.0 = Some(t.to_owned());
            true
        }
    }

    #[test]
    fn roundtrip_utf8() {
        let mut c = Clipboard::with_backend(Box::new(Mem::default()));
        c.set("привет §a ✓");
        assert_eq!(c.get(), "привет §a ✓");
    }

    #[test]
    fn empty_when_nothing() {
        let mut c = Clipboard::with_backend(Box::new(Mem::default()));
        assert_eq!(c.get(), "");
    }

    #[test]
    fn local_fallback_when_backend_fails() {
        let mut c = Clipboard::with_backend(Box::new(Mem(None, true)));
        c.set("x");
        assert_eq!(c.get(), "x");
    }
}
