//! Однострочное поле ввода (индексы курсора — в символах).
//! VANILLA: EditBox / TextFieldHelper (логика как в поиске креатива) + история ChatScreen.

#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Home,
    End,
}

#[derive(Default)]
pub struct EditBox {
    pub text: String,
    pub cursor: usize,
    pub sel: usize,
    pub max: usize,
    /// Первый видимый символ (VANILLA: EditBox#displayPos) — только для отрисовки.
    pub scroll: usize,
    /// Идёт выделение мышью (ЛКМ зажата после клика по полю).
    pub dragging: bool,
}

impl EditBox {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            ..Default::default()
        }
    }

    fn byte(&self, i: usize) -> usize {
        self.text
            .char_indices()
            .nth(i)
            .map_or(self.text.len(), |(b, _)| b)
    }

    pub fn len(&self) -> usize {
        self.text.chars().count()
    }

    pub fn selection(&self) -> (usize, usize) {
        (self.cursor.min(self.sel), self.cursor.max(self.sel))
    }

    pub fn selected_text(&self) -> String {
        let (a, b) = self.selection();
        self.text[self.byte(a)..self.byte(b)].to_owned()
    }

    pub fn set(&mut self, s: &str) {
        self.text = s.chars().take(self.max).collect();
        self.cursor = self.len();
        self.sel = self.cursor;
    }

    fn delete_selection(&mut self) -> bool {
        let (a, b) = self.selection();
        if a == b {
            return false;
        }
        let (ba, bb) = (self.byte(a), self.byte(b));
        self.text.replace_range(ba..bb, "");
        self.cursor = a;
        self.sel = a;
        true
    }

    /// VANILLA: TextFieldHelper#insertText.
    pub fn type_text(&mut self, text: &str) -> bool {
        let mut changed = self.delete_selection();
        for ch in text.chars().filter(|c| !c.is_control()) {
            if self.len() < self.max {
                let b = self.byte(self.cursor);
                self.text.insert(b, ch);
                self.cursor += 1;
                changed = true;
            }
        }
        self.sel = self.cursor;
        changed
    }

    /// VANILLA: TextFieldHelper#getWordPosition.
    fn word_pos(&self, dir: i32) -> usize {
        let cs: Vec<char> = self.text.chars().collect();
        let mut i = self.cursor as i32;
        let n = cs.len() as i32;
        if dir < 0 {
            while i > 0 && cs[i as usize - 1] == ' ' {
                i -= 1;
            }
            while i > 0 && cs[i as usize - 1] != ' ' {
                i -= 1;
            }
        } else {
            while i < n && cs[i as usize] != ' ' {
                i += 1;
            }
            while i < n && cs[i as usize] == ' ' {
                i += 1;
            }
        }
        i as usize
    }

    /// Backspace/Delete (`dir` = -1/+1); `word` — Ctrl.
    pub fn erase(&mut self, dir: i32, word: bool) -> bool {
        let mut changed = self.delete_selection();
        if !changed {
            let target = if word {
                self.word_pos(dir)
            } else {
                (self.cursor as i32 + dir).clamp(0, self.len() as i32) as usize
            };
            self.sel = target;
            changed = self.delete_selection();
        }
        changed
    }

    pub fn move_cursor(&mut self, to: Move, word: bool, shift: bool) {
        let (a, b) = self.selection();
        let pos = match (to, word) {
            (Move::Left, _) if !shift && a != b => a,
            (Move::Right, _) if !shift && a != b => b,
            (Move::Left, true) => self.word_pos(-1),
            (Move::Right, true) => self.word_pos(1),
            (Move::Left, false) => self.cursor.saturating_sub(1),
            (Move::Right, false) => (self.cursor + 1).min(self.len()),
            (Move::Home, _) => 0,
            (Move::End, _) => self.len(),
        };
        self.cursor = pos;
        if !shift {
            self.sel = pos;
        }
    }

    pub fn select_all(&mut self) {
        self.sel = 0;
        self.cursor = self.len();
    }

    pub fn cut(&mut self) -> String {
        let t = self.selected_text();
        self.delete_selection();
        t
    }
}

/// История отправленного. VANILLA: ChatComponent#addRecentChat (100, без дублей подряд),
/// ChatScreen#moveInHistory (historyPos / historyBuffer).
#[derive(Default)]
pub struct History {
    pub recent: Vec<String>,
    pos: usize,
    buffer: Option<String>,
}

impl History {
    pub fn add(&mut self, s: &str) {
        if self.recent.last().map(String::as_str) != Some(s) {
            self.recent.push(s.to_owned());
            if self.recent.len() > 100 {
                self.recent.remove(0);
            }
        }
    }

    /// Начало набора заново (открытие экрана).
    pub fn reset(&mut self) {
        self.pos = self.recent.len();
        self.buffer = None;
    }

    /// `dir` = -1 (↑) / +1 (↓). Возвращает новый текст поля, если он меняется.
    pub fn step(&mut self, dir: i32, current: &str) -> Option<String> {
        let n = self.recent.len() as i32;
        let cur = self.pos as i32;
        let new = (cur + dir).clamp(0, n);
        if new == cur {
            return None;
        }
        if new == n {
            self.pos = n as usize;
            return Some(self.buffer.take().unwrap_or_default());
        }
        if cur == n {
            self.buffer = Some(current.to_owned());
        }
        self.pos = new as usize;
        Some(self.recent[new as usize].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit() {
        let mut e = EditBox::new(5);
        e.type_text("привет!");
        assert_eq!(e.text, "приве");
        e.erase(-1, false);
        assert_eq!(e.text, "прив");
        e.move_cursor(Move::Home, false, false);
        e.erase(1, true);
        assert_eq!(e.text, "");
    }

    #[test]
    fn history() {
        let mut h = History::default();
        h.add("a");
        h.add("a");
        h.add("b");
        assert_eq!(h.recent.len(), 2);
        h.reset();
        assert_eq!(h.step(-1, "typed"), Some("b".into()));
        assert_eq!(h.step(-1, "b"), Some("a".into()));
        assert_eq!(h.step(-1, "a"), None);
        assert_eq!(h.step(1, "a"), Some("b".into()));
        assert_eq!(h.step(1, "b"), Some("typed".into()));
        assert_eq!(h.step(1, "typed"), None);
    }
}
