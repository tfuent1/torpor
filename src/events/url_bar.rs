use crate::app::AppState;
use crate::config::KeyBinds;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the URL bar has focus.
/// Returns `true` if the event was consumed.
pub fn handle(
    state: &mut AppState,
    modifiers: KeyModifiers,
    code: KeyCode,
    binds: &KeyBinds,
) -> bool {
    if KeyBinds::any_match(&binds.method_next, modifiers, code) {
        state.method = crate::events::next_method(&state.method);
        return true;
    }
    if KeyBinds::any_match(&binds.method_prev, modifiers, code) {
        state.method = crate::events::prev_method(&state.method);
        return true;
    }
    if KeyBinds::any_match(&binds.url_clear, modifiers, code) {
        state.url.clear();
        state.cursor_pos = 0;
        return true;
    }

    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Left) => {
            state.cursor_pos = state.cursor_pos.saturating_sub(1);
        }
        (KeyModifiers::NONE, KeyCode::Right) => {
            state.cursor_pos = (state.cursor_pos + 1).min(state.url.len());
        }
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            state.url.insert(state.cursor_pos, c);
            state.cursor_pos += 1;
            state.status_message = None;
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            if state.cursor_pos > 0 {
                state.url.remove(state.cursor_pos - 1);
                state.cursor_pos -= 1;
            }
        }
        _ => return false,
    }

    true
}
