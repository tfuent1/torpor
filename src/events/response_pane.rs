use crate::app::AppState;
use crate::config::KeyBinds;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the response pane has focus.
/// Returns `true` if the event was consumed.
pub fn handle(
    state: &mut AppState,
    modifiers: KeyModifiers,
    code: KeyCode,
    binds: &KeyBinds,
) -> bool {
    if KeyBinds::any_match(&binds.scroll_down, modifiers, code) {
        state.response_scroll = state.response_scroll.saturating_add(1);
        return true;
    }
    if KeyBinds::any_match(&binds.scroll_up, modifiers, code) {
        state.response_scroll = state.response_scroll.saturating_sub(1);
        return true;
    }

    false
}
