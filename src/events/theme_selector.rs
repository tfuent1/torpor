use crate::app::AppState;
use crate::config::theme::all_builtin;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the theme selector overlay is open.
/// Returns `true` if the event was consumed.
pub fn handle(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    let theme_count = all_builtin().len();

    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Esc) => {
            state.theme_selector_open = false;
        }
        (KeyModifiers::NONE, KeyCode::Up) => {
            if state.theme_selector_index > 0 {
                state.theme_selector_index -= 1;
            }
        }
        (KeyModifiers::NONE, KeyCode::Down) => {
            if state.theme_selector_index < theme_count - 1 {
                state.theme_selector_index += 1;
            }
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            // Signal to main that the selected theme should be applied and persisted.
            // We close the selector here; main handles the actual theme swap.
            state.theme_selector_open = false;
            return false; // false = caller should check theme_selector_index and apply
        }
        _ => {}
    }

    true
}
