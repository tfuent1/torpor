use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};

/// A single key binding serialized as a human-readable string.
///
/// Format examples:
/// - `"ctrl+r"`
/// - `"ctrl+shift+s"`
/// - `"q"`
/// - `"tab"`
/// - `"ctrl+up"`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KeyBind(pub String);

impl KeyBind {
    /// Parses this binding into a `(KeyModifiers, KeyCode)` pair.
    /// Returns `None` if the string is malformed.
    pub fn parse(&self) -> Option<(KeyModifiers, KeyCode)> {
        let s = self.0.to_lowercase();
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() {
            return None;
        }

        let (mod_parts, key_part) = parts.split_at(parts.len() - 1);
        let key_str = key_part[0];

        let mut modifiers = KeyModifiers::NONE;
        for m in mod_parts {
            match *m {
                "ctrl" => modifiers |= KeyModifiers::CONTROL,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                "alt" => modifiers |= KeyModifiers::ALT,
                _ => return None,
            }
        }

        let code = match key_str {
            "enter" => KeyCode::Enter,
            "esc" | "escape" => KeyCode::Esc,
            "tab" => KeyCode::Tab,
            "backtab" => KeyCode::BackTab,
            "backspace" => KeyCode::Backspace,
            "delete" | "del" => KeyCode::Delete,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            "f1" => KeyCode::F(1),
            "f2" => KeyCode::F(2),
            "f3" => KeyCode::F(3),
            "f4" => KeyCode::F(4),
            "f5" => KeyCode::F(5),
            "f6" => KeyCode::F(6),
            "f7" => KeyCode::F(7),
            "f8" => KeyCode::F(8),
            "f9" => KeyCode::F(9),
            "f10" => KeyCode::F(10),
            "f11" => KeyCode::F(11),
            "f12" => KeyCode::F(12),
            c if c.len() == 1 => KeyCode::Char(c.chars().next()?),
            _ => return None,
        };

        Some((modifiers, code))
    }

    /// Returns `true` if this binding matches the given event modifiers and code.
    pub fn matches(&self, modifiers: KeyModifiers, code: KeyCode) -> bool {
        self.parse()
            .is_some_and(|(m, c)| m == modifiers && c == code)
    }
}

/// All remappable actions in Torpor.
///
/// Each field is a `Vec<KeyBind>` so users can assign multiple keys to one
/// action (e.g. both `"q"` and `"ctrl+q"` for quit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinds {
    // Global
    pub quit: Vec<KeyBind>,
    pub send_request: Vec<KeyBind>,
    pub save_request: Vec<KeyBind>,
    pub load_request: Vec<KeyBind>,
    pub focus_next: Vec<KeyBind>,
    pub focus_prev: Vec<KeyBind>,

    // URL bar
    pub method_next: Vec<KeyBind>,
    pub method_prev: Vec<KeyBind>,
    pub url_clear: Vec<KeyBind>,

    // Request pane
    pub tab_body: Vec<KeyBind>,
    pub tab_headers: Vec<KeyBind>,

    // Headers editor
    pub header_add: Vec<KeyBind>,
    pub header_delete: Vec<KeyBind>,

    // Response pane
    pub scroll_down: Vec<KeyBind>,
    pub scroll_up: Vec<KeyBind>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            quit: vec![kb("ctrl+q"), kb("q")],
            send_request: vec![kb("ctrl+r")],
            save_request: vec![kb("ctrl+s")],
            load_request: vec![kb("ctrl+o")],
            focus_next: vec![kb("tab")],
            focus_prev: vec![kb("shift+tab")],

            method_next: vec![kb("ctrl+down")],
            method_prev: vec![kb("ctrl+up")],
            url_clear: vec![kb("ctrl+d")],

            tab_body: vec![kb("ctrl+left"), kb("ctrl+h")],
            tab_headers: vec![kb("ctrl+right"), kb("ctrl+l")],

            header_add: vec![kb("a")],
            header_delete: vec![kb("d")],

            scroll_down: vec![kb("j"), kb("down")],
            scroll_up: vec![kb("k"), kb("up")],
        }
    }
}

impl KeyBinds {
    /// Returns `true` if any binding in `binds` matches `(modifiers, code)`.
    pub fn any_match(binds: &[KeyBind], modifiers: KeyModifiers, code: KeyCode) -> bool {
        binds.iter().any(|b| b.matches(modifiers, code))
    }
}

fn kb(s: &str) -> KeyBind {
    KeyBind(s.to_string())
}
