use crate::app::AppState;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the workspace picker overlay is open.
/// Returns `true` if the event was consumed (overlay stays open or closed
/// without selecting). Returns `false` when Enter is pressed — caller should
/// read `state.workspace_picker_selected` and dispatch `SwitchWorkspace`.
pub fn handle(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    let count = state.workspace_picker_items.len();

    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Esc) => {
            state.workspace_picker_open = false;
        }
        (KeyModifiers::NONE, KeyCode::Up | KeyCode::Char('k')) => {
            if state.workspace_picker_selected > 0 {
                state.workspace_picker_selected -= 1;
            }
        }
        (KeyModifiers::NONE, KeyCode::Down | KeyCode::Char('j')) => {
            if count > 0 && state.workspace_picker_selected < count - 1 {
                state.workspace_picker_selected += 1;
            }
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            if count > 0 {
                state.workspace_picker_open = false;
                return false; // signal: apply selection
            }
        }
        _ => {}
    }

    true
}
