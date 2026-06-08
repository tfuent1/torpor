pub mod request_pane;
pub mod response_pane;
pub mod sidebar;
pub mod theme_selector;
pub mod url_bar;
pub mod workspace_picker;

use crate::app::{AppState, Focus};
use crate::config::KeyBinds;
use crate::models::request::HttpMethod;
use crossterm::event::{KeyCode, KeyModifiers};

/// Top-level outcome returned from `dispatch`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Continue the event loop normally.
    Continue,
    /// Quit the application.
    Quit,
    /// Send the current request.
    SendRequest,
    /// Save the current request to disk.
    SaveRequest,
    /// Load a request from disk.
    LoadRequest,
    /// Apply a theme.
    ApplyTheme(usize),
    /// Switch to a workspace by index into `state.workspace_picker_items`.
    SwitchWorkspace(usize),
}

/// Dispatches a key event to the correct pane handler and returns an `Action`.
///
/// Global bindings (quit, send, save, load, focus cycle) are checked first so
/// they work regardless of which pane has focus.
pub fn dispatch(
    state: &mut AppState,
    modifiers: KeyModifiers,
    code: KeyCode,
    binds: &KeyBinds,
) -> Action {
    // Theme selector intercepts all keys when open
    if state.theme_selector_open {
        let consumed = theme_selector::handle(state, modifiers, code);
        if !consumed {
            // Enter was pressed — apply the selected theme
            return Action::ApplyTheme(state.theme_selector_index);
        }
        return Action::Continue;
    }

    // Workspace picker intercepts all keys when open
    if state.workspace_picker_open {
        let consumed = workspace_picker::handle(state, modifiers, code);
        if !consumed {
            // Enter was pressed — apply the selection
            return Action::SwitchWorkspace(state.workspace_picker_selected);
        }
        return Action::Continue;
    }

    // Ctrl+T opens the theme selector from any focus
    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('t') {
        state.theme_selector_open = true;
        return Action::Continue;
    }

    // Ctrl+W opens the workspace picker
    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('w') {
        // Run a bounded discovery scan from the workspace root (or cwd)
        let root = state.workspace.root().map_or_else(
            || std::env::current_dir().unwrap_or_default(),
            |p| p.parent().unwrap_or(p).to_path_buf(),
        );
        let found = crate::discovery::scan_subdir_pub(&root);
        state.workspace_picker_items = found;
        state.workspace_picker_selected = 0;
        state.workspace_picker_open = true;
        return Action::Continue;
    }

    // --- Global actions ---
    if KeyBinds::any_match(&binds.quit, modifiers, code) {
        // `q` alone should still type in text input panes, not quit
        let is_bare_q = modifiers == KeyModifiers::NONE && code == KeyCode::Char('q');
        let is_text_input = matches!(state.focus, Focus::UrlBar | Focus::RequestPane);
        if !is_bare_q || !is_text_input {
            return Action::Quit;
        }
    }

    if KeyBinds::any_match(&binds.send_request, modifiers, code) {
        return Action::SendRequest;
    }
    if KeyBinds::any_match(&binds.save_request, modifiers, code) {
        return Action::SaveRequest;
    }
    if KeyBinds::any_match(&binds.load_request, modifiers, code) {
        return Action::LoadRequest;
    }

    // Ctrl+B toggles sidebar
    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('b') {
        state.sidebar_open = !state.sidebar_open;
        // If we just closed the sidebar while it was focused, move focus to URL bar
        if !state.sidebar_open && state.focus == Focus::Sidebar {
            state.focus = Focus::UrlBar;
        }
        return Action::Continue;
    }

    // Focus cycling
    if KeyBinds::any_match(&binds.focus_next, modifiers, code) {
        state.focus = match state.focus {
            Focus::Sidebar => Focus::UrlBar,
            Focus::UrlBar => Focus::RequestPane,
            Focus::RequestPane => Focus::ResponsePane,
            Focus::ResponsePane => {
                if state.sidebar_open {
                    Focus::Sidebar
                } else {
                    Focus::UrlBar
                }
            }
        };
        return Action::Continue;
    }
    if KeyBinds::any_match(&binds.focus_prev, modifiers, code) {
        state.focus = match state.focus {
            Focus::Sidebar => Focus::ResponsePane,
            Focus::UrlBar => {
                if state.sidebar_open {
                    Focus::Sidebar
                } else {
                    Focus::ResponsePane
                }
            }
            Focus::RequestPane => Focus::UrlBar,
            Focus::ResponsePane => Focus::RequestPane,
        };
        return Action::Continue;
    }

    // --- Pane-specific handlers ---
    match state.focus {
        Focus::Sidebar => {
            sidebar::handle(state, modifiers, code);
        }
        Focus::UrlBar => {
            url_bar::handle(state, modifiers, code, binds);
        }
        Focus::RequestPane => {
            request_pane::handle(state, modifiers, code, binds);
        }
        Focus::ResponsePane => {
            let consumed = response_pane::handle(state, modifiers, code, binds);
            if !consumed && modifiers == KeyModifiers::NONE && code == KeyCode::Char('q') {
                return Action::Quit;
            }
        }
    }

    Action::Continue
}

// ---------------------------------------------------------------------------
// Shared helpers used by multiple event modules
// ---------------------------------------------------------------------------

pub fn next_method(method: &HttpMethod) -> HttpMethod {
    use HttpMethod::{Delete, Get, Head, Options, Patch, Post, Put};
    match method {
        Get => Post,
        Post => Put,
        Put => Patch,
        Patch => Delete,
        Delete => Head,
        Head => Options,
        Options => Get,
    }
}

pub fn prev_method(method: &HttpMethod) -> HttpMethod {
    use HttpMethod::{Delete, Get, Head, Options, Patch, Post, Put};
    match method {
        Get => Options,
        Post => Get,
        Put => Post,
        Patch => Put,
        Delete => Patch,
        Head => Delete,
        Options => Head,
    }
}
