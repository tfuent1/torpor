use crate::app::AppState;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the sidebar has focus.
/// Returns `true` if the event was consumed.
pub fn handle(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Down | KeyCode::Char('j')) => {
            state.sidebar_selected = state.sidebar_selected.saturating_add(1);
        }
        (KeyModifiers::NONE, KeyCode::Up | KeyCode::Char('k')) => {
            state.sidebar_selected = state.sidebar_selected.saturating_sub(1);
        }
        _ => return false,
    }
    true
}
