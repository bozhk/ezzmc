//! Биндинги клавиш (TODO-037): ванильные умолчания, переназначение, сохранение в формате options.txt.
//! VANILLA: net.minecraft.client.KeyMapping + Options (1.21.11) — `key_<name>:<key.keyboard.w>`.
//! Не портировано: ToggleKeyMapping (залипание sneak/sprint/use/attack), F3-комбинации (глава 12).
#![allow(dead_code)]
use crate::input::{from_glfw_code, key_name, mouse_button_name, ALL_KEYS};
use std::collections::HashMap;
use winit::{event::MouseButton, keyboard::KeyCode};

/// VANILLA: InputConstants.Key (KEYSYM/MOUSE) + UNKNOWN.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
    Unbound,
}

impl Input {
    /// VANILLA: InputConstants.Key#getName.
    pub fn name(self) -> String {
        match self {
            Input::Key(k) => key_name(k).unwrap_or_else(|| String::from("key.keysym.unknown")),
            Input::Mouse(b) => mouse_button_name(b)
                .map_or_else(|| String::from("key.keysym.unknown"), String::from),
            Input::Unbound => String::from("key.keysym.unknown"),
        }
    }

    /// Обратное к `name`; неизвестное имя → None (ваниль: значение options остаётся умолчанием).
    pub fn parse(s: &str) -> Option<Input> {
        if s == "key.keysym.unknown" {
            return Some(Input::Unbound);
        }
        if let Some(rest) = s.strip_prefix("key.mouse.") {
            return Some(Input::Mouse(match rest {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                "4" => MouseButton::Back,
                "5" => MouseButton::Forward,
                _ => return None,
            }));
        }
        ALL_KEYS
            .iter()
            .copied()
            .find(|k| key_name(*k).as_deref() == Some(s))
            .map(Input::Key)
    }

    /// GLFW-код клавиши (для сверки с таблицами ванили).
    pub fn glfw(self) -> Option<i32> {
        match self {
            Input::Key(k) => crate::input::glfw_code(k),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Movement,
    Gameplay,
    Inventory,
    Multiplayer,
    Misc,
    Creative,
    Spectator,
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: &'static str,
    pub default: Input,
    pub key: Input,
    pub category: Category,
    down: bool,
    clicks: u32,
    /// VANILLA: ToggleKeyMapping — нажатие переключает состояние, отпускание игнорируется.
    toggle: bool,
}

impl Binding {
    pub fn is_down(&self) -> bool {
        self.down
    }
    pub fn is_default(&self) -> bool {
        self.key == self.default
    }
    pub fn is_unbound(&self) -> bool {
        self.key == Input::Unbound
    }
    /// VANILLA: KeyMapping#consumeClick.
    pub fn consume_click(&mut self) -> bool {
        if self.clicks == 0 {
            return false;
        }
        self.clicks -= 1;
        true
    }
}

/// VANILLA: Options.key* — умолчания (GLFW-коды из Options.java 1.21.11).
fn defaults() -> Vec<(&'static str, Input, Category)> {
    use Category::*;
    use KeyCode::*;
    let k = Input::Key;
    let m = Input::Mouse;
    vec![
        ("key.forward", k(KeyW), Movement),
        ("key.left", k(KeyA), Movement),
        ("key.back", k(KeyS), Movement),
        ("key.right", k(KeyD), Movement),
        ("key.jump", k(Space), Movement),
        ("key.sneak", k(ShiftLeft), Movement),
        ("key.sprint", k(ControlLeft), Movement),
        ("key.inventory", k(KeyE), Inventory),
        ("key.swapOffhand", k(KeyF), Inventory),
        ("key.drop", k(KeyQ), Inventory),
        ("key.use", m(MouseButton::Right), Gameplay),
        ("key.attack", m(MouseButton::Left), Gameplay),
        ("key.pickItem", m(MouseButton::Middle), Gameplay),
        ("key.chat", k(KeyT), Multiplayer),
        ("key.playerlist", k(Tab), Multiplayer),
        ("key.command", k(Slash), Multiplayer),
        ("key.socialInteractions", k(KeyP), Multiplayer),
        ("key.screenshot", k(F2), Misc),
        ("key.togglePerspective", k(F5), Misc),
        ("key.smoothCamera", Input::Unbound, Misc),
        ("key.fullscreen", k(F11), Misc),
        // Не ванильная: переключение режима рендера (compute ↔ ванильный путь).
        ("key.renderMode", k(F8), Misc),
        // Не ванильная: зум по удержанию (в духе OptiFine), по умолчанию C.
        ("key.zoom", k(KeyC), Misc),
        ("key.advancements", k(KeyL), Misc),
        ("key.quickActions", k(KeyG), Misc),
        ("key.toggleGui", k(F1), Misc),
        ("key.toggleSpectatorShaderEffects", k(F4), Misc),
        ("key.hotbar.1", k(Digit1), Inventory),
        ("key.hotbar.2", k(Digit2), Inventory),
        ("key.hotbar.3", k(Digit3), Inventory),
        ("key.hotbar.4", k(Digit4), Inventory),
        ("key.hotbar.5", k(Digit5), Inventory),
        ("key.hotbar.6", k(Digit6), Inventory),
        ("key.hotbar.7", k(Digit7), Inventory),
        ("key.hotbar.8", k(Digit8), Inventory),
        ("key.hotbar.9", k(Digit9), Inventory),
        ("key.saveToolbarActivator", k(KeyC), Creative),
        ("key.loadToolbarActivator", k(KeyX), Creative),
        ("key.spectatorOutlines", Input::Unbound, Spectator),
        ("key.spectatorHotbar", m(MouseButton::Middle), Spectator),
    ]
}

#[derive(Debug)]
pub struct KeyBindings {
    list: Vec<Binding>,
    index: HashMap<&'static str, usize>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let list: Vec<Binding> = defaults()
            .into_iter()
            .map(|(name, d, category)| Binding {
                name,
                default: d,
                key: d,
                category,
                down: false,
                clicks: 0,
                toggle: false,
            })
            .collect();
        let index = list.iter().enumerate().map(|(i, b)| (b.name, i)).collect();
        Self { list, index }
    }
}

impl KeyBindings {
    pub fn get(&self, name: &str) -> Option<&Binding> {
        self.index.get(name).map(|&i| &self.list[i])
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Binding> {
        let i = *self.index.get(name)?;
        Some(&mut self.list[i])
    }
    pub fn iter(&self) -> impl Iterator<Item = &Binding> {
        self.list.iter()
    }

    /// VANILLA: KeyMapping#setKey (+ resetMapping). Возвращает false, если имени нет.
    pub fn rebind(&mut self, name: &str, key: Input) -> bool {
        match self.get_mut(name) {
            Some(b) => {
                b.key = key;
                true
            }
            None => false,
        }
    }

    pub fn reset_all(&mut self) {
        for b in &mut self.list {
            b.key = b.default;
        }
    }

    /// Имена биндингов, делящих тот же ввод (VANILLA: KeyBindsScreen подсвечивает конфликты).
    pub fn conflicts(&self, name: &str) -> Vec<&'static str> {
        let Some(b) = self.get(name) else {
            return vec![];
        };
        if b.is_unbound() {
            return vec![];
        }
        self.list
            .iter()
            .filter(|o| o.name != name && o.key == b.key)
            // Зум (C) и сохранение набора в креативе (C + цифра) — разные контексты, не конфликт.
            .filter(|o| {
                !matches!(
                    (name, o.name),
                    ("key.zoom", "key.saveToolbarActivator")
                        | ("key.saveToolbarActivator", "key.zoom")
                )
            })
            .map(|o| o.name)
            .collect()
    }

    /// VANILLA: KeyMapping.set — состояние down у всех биндингов на этом вводе.
    pub fn set(&mut self, input: Input, down: bool) {
        for b in self.list.iter_mut().filter(|b| b.key == input) {
            // VANILLA: ToggleKeyMapping#setDown.
            if !b.toggle {
                b.down = down;
            } else if down {
                b.down = !b.down;
            }
        }
    }
    /// VANILLA: Options#toggleCrouch/toggleSprint/toggleAttack/toggleUse.
    pub fn set_toggle(&mut self, name: &str, toggle: bool) {
        if let Some(&i) = self.index.get(name) {
            let b = &mut self.list[i];
            if b.toggle != toggle {
                b.toggle = toggle;
                b.down = false;
            }
        }
    }
    /// VANILLA: KeyMapping.click — нажатие считается для consume_click.
    pub fn click(&mut self, input: Input) {
        for b in self.list.iter_mut().filter(|b| b.key == input) {
            b.clicks += 1;
        }
    }
    /// Нажатие/отпускание (как KeyboardHandler/MouseHandler: set + click при нажатии).
    pub fn on_input(&mut self, input: Input, pressed: bool) {
        if input == Input::Unbound {
            return;
        }
        self.set(input, pressed);
        if pressed {
            self.click(input);
        }
    }
    /// VANILLA: KeyMapping.releaseAll.
    pub fn release_all(&mut self) {
        for b in &mut self.list {
            b.down = false;
            b.clicks = 0;
        }
    }

    /// Строки options.txt (`key_key.jump:key.keyboard.space`), только изменённые не пропускаем —
    /// ваниль пишет все. VANILLA: Options#processOptions → `key_` + name.
    pub fn to_options_lines(&self) -> Vec<String> {
        self.list
            .iter()
            .map(|b| format!("key_{}:{}", b.name, b.key.name()))
            .collect()
    }

    /// Применить пары ключ→значение из options.txt; неизвестные/битые игнорируются.
    pub fn load_options<'a>(&mut self, pairs: impl IntoIterator<Item = (&'a str, &'a str)>) {
        for (k, v) in pairs {
            let Some(name) = k.strip_prefix("key_") else {
                continue;
            };
            if let Some(input) = Input::parse(v) {
                self.rebind(name, input);
            }
        }
    }
}

/// Вспомогательное: код GLFW → Input (для чтения старых конфигов с числами).
pub fn input_from_glfw(code: i32) -> Input {
    from_glfw_code(code).map_or(Input::Unbound, Input::Key)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_vanilla_glfw_codes() {
        let kb = KeyBindings::default();
        let code = |n: &str| kb.get(n).and_then(|b| b.default.glfw());
        assert_eq!(code("key.forward"), Some(87));
        assert_eq!(code("key.jump"), Some(32));
        assert_eq!(code("key.sneak"), Some(340));
        assert_eq!(code("key.sprint"), Some(341));
        assert_eq!(code("key.playerlist"), Some(258));
        assert_eq!(code("key.command"), Some(47));
        assert_eq!(code("key.screenshot"), Some(291));
        assert_eq!(code("key.fullscreen"), Some(300));
        assert_eq!(code("key.toggleGui"), Some(290));
        assert_eq!(code("key.hotbar.9"), Some(57));
        assert_eq!(
            kb.get("key.attack").map(|b| b.default),
            Some(Input::Mouse(MouseButton::Left))
        );
        assert!(kb.get("key.smoothCamera").is_some_and(|b| b.is_unbound()));
    }

    #[test]
    fn no_default_conflicts_except_vanilla_shared_middle() {
        let kb = KeyBindings::default();
        for b in kb.iter() {
            let c = kb.conflicts(b.name);
            // ваниль: pickItem и spectatorHotbar оба на средней кнопке (разные контексты)
            let allowed = matches!(b.name, "key.pickItem" | "key.spectatorHotbar");
            assert!(c.is_empty() || allowed, "{} конфликтует с {:?}", b.name, c);
        }
    }

    #[test]
    fn rebind_conflicts_and_reset() {
        let mut kb = KeyBindings::default();
        assert!(kb.rebind("key.jump", Input::Key(KeyCode::KeyW)));
        assert_eq!(kb.conflicts("key.jump"), vec!["key.forward"]);
        assert!(!kb.get("key.jump").is_some_and(|b| b.is_default()));
        assert!(!kb.rebind("key.nope", Input::Unbound));
        kb.reset_all();
        assert!(kb.get("key.jump").is_some_and(|b| b.is_default()));
    }

    #[test]
    fn press_release_and_clicks() {
        let mut kb = KeyBindings::default();
        let w = Input::Key(KeyCode::KeyW);
        kb.on_input(w, true);
        assert!(kb.get("key.forward").is_some_and(|b| b.is_down()));
        kb.on_input(w, true);
        let f = kb.get_mut("key.forward").expect("f");
        assert!(f.consume_click() && f.consume_click() && !f.consume_click());
        kb.on_input(w, false);
        assert!(!kb.get("key.forward").is_some_and(|b| b.is_down()));
        kb.on_input(w, true);
        kb.release_all();
        let f = kb.get_mut("key.forward").expect("f");
        assert!(!f.is_down() && !f.consume_click());
    }

    #[test]
    fn options_roundtrip() {
        let mut kb = KeyBindings::default();
        kb.rebind("key.jump", Input::Key(KeyCode::KeyJ));
        kb.rebind("key.attack", Input::Mouse(MouseButton::Back));
        kb.rebind("key.inventory", Input::Unbound);
        let lines = kb.to_options_lines();
        assert!(lines.contains(&"key_key.jump:key.keyboard.j".to_string()));
        assert!(lines.contains(&"key_key.attack:key.mouse.4".to_string()));
        assert!(lines.contains(&"key_key.inventory:key.keysym.unknown".to_string()));
        let mut kb2 = KeyBindings::default();
        kb2.load_options(lines.iter().filter_map(|l| l.split_once(':')));
        for b in kb.iter() {
            assert_eq!(kb2.get(b.name).map(|o| o.key), Some(b.key), "{}", b.name);
        }
        kb2.load_options([("key_key.jump", "key.keyboard.bogus"), ("foo", "bar")]);
        assert_eq!(
            kb2.get("key.jump").map(|b| b.key),
            Some(Input::Key(KeyCode::KeyJ))
        );
    }

    #[test]
    fn toggle_mapping_flips_on_press_and_ignores_release() {
        let mut kb = KeyBindings::default();
        kb.set_toggle("key.sneak", true);
        let shift = Input::Key(KeyCode::ShiftLeft);
        kb.on_input(shift, true);
        kb.on_input(shift, false);
        assert!(kb.get("key.sneak").unwrap().is_down());
        kb.on_input(shift, true);
        assert!(!kb.get("key.sneak").unwrap().is_down());
        kb.set_toggle("key.sneak", false);
        kb.on_input(shift, true);
        kb.on_input(shift, false);
        assert!(!kb.get("key.sneak").unwrap().is_down());
    }
}
