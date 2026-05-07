pub mod request_pane;
pub mod response_pane;
pub mod url_bar;

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

    // Focus cycling
    if KeyBinds::any_match(&binds.focus_next, modifiers, code) {
        state.focus = match state.focus {
            Focus::UrlBar => Focus::RequestPane,
            Focus::RequestPane => Focus::ResponsePane,
            Focus::ResponsePane => Focus::UrlBar,
        };
        return Action::Continue;
    }
    if KeyBinds::any_match(&binds.focus_prev, modifiers, code) {
        state.focus = match state.focus {
            Focus::UrlBar => Focus::ResponsePane,
            Focus::RequestPane => Focus::UrlBar,
            Focus::ResponsePane => Focus::RequestPane,
        };
        return Action::Continue;
    }

    // --- Pane-specific handlers ---
    match state.focus {
        Focus::UrlBar => {
            url_bar::handle(state, modifiers, code, binds);
        }
        Focus::RequestPane => {
            request_pane::handle(state, modifiers, code, binds);
        }
        Focus::ResponsePane => {
            let consumed = response_pane::handle(state, modifiers, code, binds);
            // `q` in the response pane always quits even when not remapped
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
