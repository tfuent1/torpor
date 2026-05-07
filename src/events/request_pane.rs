use crate::app::{AppState, HeaderField, RequestTab};
use crate::config::KeyBinds;
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the request pane has focus.
/// Returns `true` if the event was consumed.
pub fn handle(
    state: &mut AppState,
    modifiers: KeyModifiers,
    code: KeyCode,
    binds: &KeyBinds,
) -> bool {
    // Header editing mode intercepts nearly everything
    if state.active_tab == RequestTab::Headers && state.header_editing.is_some() {
        return handle_header_edit(state, modifiers, code);
    }

    // Tab switching
    if KeyBinds::any_match(&binds.tab_body, modifiers, code) {
        state.active_tab = RequestTab::Body;
        return true;
    }
    if KeyBinds::any_match(&binds.tab_headers, modifiers, code) {
        state.active_tab = RequestTab::Headers;
        return true;
    }

    match state.active_tab {
        RequestTab::Headers => handle_headers_nav(state, modifiers, code, binds),
        RequestTab::Body => handle_body(state, modifiers, code),
    }
}

// ---------------------------------------------------------------------------
// Headers navigation
// ---------------------------------------------------------------------------

fn handle_headers_nav(
    state: &mut AppState,
    modifiers: KeyModifiers,
    code: KeyCode,
    binds: &KeyBinds,
) -> bool {
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Up) => {
            if state.header_selected > 0 {
                state.header_selected -= 1;
            }
            true
        }
        (KeyModifiers::NONE, KeyCode::Down) => {
            if !state.headers.is_empty() && state.header_selected < state.headers.len() - 1 {
                state.header_selected += 1;
            }
            true
        }
        _ if KeyBinds::any_match(&binds.header_add, modifiers, code) => {
            state.headers.push((String::new(), String::new()));
            state.header_selected = state.headers.len() - 1;
            state.header_edit_buf = String::new();
            state.header_editing = Some(HeaderField::Key);
            true
        }
        _ if KeyBinds::any_match(&binds.header_delete, modifiers, code)
            && !state.headers.is_empty() =>
        {
            state.headers.remove(state.header_selected);
            if state.header_selected > 0 && state.header_selected >= state.headers.len() {
                state.header_selected -= 1;
            }
            true
        }
        (KeyModifiers::NONE, KeyCode::Enter) if !state.headers.is_empty() => {
            state.header_edit_buf = state.headers[state.header_selected].0.clone();
            state.header_editing = Some(HeaderField::Key);
            true
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Header inline editing
// ---------------------------------------------------------------------------

fn handle_header_edit(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Esc) => {
            state.header_editing = None;
            state.header_edit_buf = String::new();
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            let i = state.header_selected;
            match &state.header_editing {
                Some(HeaderField::Key) => {
                    state.headers[i].0 = state.header_edit_buf.clone();
                    state.header_edit_buf = state.headers[i].1.clone();
                    state.header_editing = Some(HeaderField::Value);
                }
                Some(HeaderField::Value) => {
                    state.headers[i].1 = state.header_edit_buf.clone();
                    state.header_editing = None;
                    state.header_edit_buf = String::new();
                }
                None => {}
            }
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            state.header_edit_buf.pop();
        }
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            state.header_edit_buf.push(c);
        }
        _ => return false,
    }
    true
}

// ---------------------------------------------------------------------------
// Body editor
// ---------------------------------------------------------------------------

fn handle_body(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Up) => {
            if state.body_cursor_row > 0 {
                state.body_cursor_row -= 1;
                state.clamp_body_cursor();
            }
        }
        (KeyModifiers::NONE, KeyCode::Down) => {
            if state.body_cursor_row < state.body_lines.len() - 1 {
                state.body_cursor_row += 1;
                state.clamp_body_cursor();
            }
        }
        (KeyModifiers::NONE, KeyCode::Left) => {
            if state.body_cursor_col > 0 {
                state.body_cursor_col -= 1;
            } else if state.body_cursor_row > 0 {
                state.body_cursor_row -= 1;
                state.body_cursor_col = state.body_lines[state.body_cursor_row].len();
            }
        }
        (KeyModifiers::NONE, KeyCode::Right) => {
            let line_len = state.body_lines[state.body_cursor_row].len();
            if state.body_cursor_col < line_len {
                state.body_cursor_col += 1;
            } else if state.body_cursor_row < state.body_lines.len() - 1 {
                state.body_cursor_row += 1;
                state.body_cursor_col = 0;
            }
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            let row = state.body_cursor_row;
            let col = state.body_cursor_col;
            let rest = state.body_lines[row].split_off(col);
            state.body_lines.insert(row + 1, rest);
            state.body_cursor_row += 1;
            state.body_cursor_col = 0;
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            let row = state.body_cursor_row;
            let col = state.body_cursor_col;
            if col > 0 {
                state.body_lines[row].remove(col - 1);
                state.body_cursor_col -= 1;
            } else if row > 0 {
                let current = state.body_lines.remove(row);
                let prev_len = state.body_lines[row - 1].len();
                state.body_lines[row - 1].push_str(&current);
                state.body_cursor_row -= 1;
                state.body_cursor_col = prev_len;
            }
        }
        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            let row = state.body_cursor_row;
            let col = state.body_cursor_col;
            state.body_lines[row].insert(col, c);
            state.body_cursor_col += 1;
        }
        _ => return false,
    }
    true
}
