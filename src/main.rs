mod app;
mod engine;
mod models;
mod storage;
mod tui;

use crate::app::{AppState, Focus, HeaderField, RequestTab};
use crate::engine::RequestEngine;
use crate::models::request::{Body, BodyType, Request};
use anyhow::Context;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

/// Fixed path for save/load in Phase 1.
fn request_path() -> PathBuf {
    PathBuf::from("request.yaml")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to create terminal")?;

    let engine = RequestEngine::new().context("failed to create request engine")?;
    let mut state = AppState::new();

    let result = run(&mut terminal, &mut state, &engine);

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    result
}

#[allow(clippy::too_many_lines)]
fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
    engine: &RequestEngine,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| tui::render(frame, state))?;

        // Non-blocking check for completed request
        if let Ok(result) = state.response_rx.try_recv() {
            state.request_in_flight = false;
            match result {
                Ok(response) => {
                    state.response = Some(response);
                    state.response_scroll = 0;
                    state.status_message = None;
                }
                Err(e) => {
                    state.status_message = Some(format!("Error: {e}"));
                }
            }
        }

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            // --- Headers editing mode swallows most keys ---
            if state.focus == Focus::RequestPane
                && state.active_tab == RequestTab::Headers
                && state.header_editing.is_some()
            {
                handle_header_edit_key(state, key.modifiers, key.code);
                continue;
            }

            match (key.modifiers, key.code) {
                // Quit
                (KeyModifiers::NONE, KeyCode::Char('q')) if state.focus == Focus::ResponsePane => {
                    break;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,

                // Down
                (KeyModifiers::NONE, KeyCode::Char('j'))
                    if state.focus == Focus::ResponsePane =>
                {
                    state.response_scroll = state.response_scroll.saturating_add(1);
                }

                // Up
                (KeyModifiers::NONE, KeyCode::Char('k'))
                    if state.focus == Focus::ResponsePane =>
                {
                    state.response_scroll = state.response_scroll.saturating_sub(1);
                }

                // Send — Ctrl+R (Ctrl+Enter is unreliable in most terminals)
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => handle_send(state, engine),

                // Save
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => handle_save(state),

                // Load
                (KeyModifiers::CONTROL, KeyCode::Char('o')) => handle_load(state),

                // Clear URL — Ctrl+D (readline kill-to-start)
                (KeyModifiers::CONTROL, KeyCode::Char('d')) if state.focus == Focus::UrlBar => {
                    state.url.clear();
                    state.cursor_pos = 0;
                }

                // Tab / Shift+Tab cycle focus
                (KeyModifiers::NONE, KeyCode::Tab) => {
                    // If in headers navigation mode, Tab switches tab first
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers
                        && state.header_editing.is_none()
                    {
                        // Tab switches to Body tab first, then focus cycles
                        // Actually: match the original behaviour — Tab cycles focus pane
                    }
                    state.focus = match state.focus {
                        Focus::UrlBar => Focus::RequestPane,
                        Focus::RequestPane => Focus::ResponsePane,
                        Focus::ResponsePane => Focus::UrlBar,
                    };
                }
                (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                    state.focus = match state.focus {
                        Focus::UrlBar => Focus::ResponsePane,
                        Focus::RequestPane => Focus::UrlBar,
                        Focus::ResponsePane => Focus::RequestPane,
                    };
                }

                // Method cycle (UrlBar)
                (KeyModifiers::NONE, KeyCode::Up) if state.focus == Focus::UrlBar => {
                    state.method = prev_method(&state.method);
                }
                (KeyModifiers::NONE, KeyCode::Down) if state.focus == Focus::UrlBar => {
                    state.method = next_method(&state.method);
                }

                // URL cursor
                (KeyModifiers::NONE, KeyCode::Left) if state.focus == Focus::UrlBar => {
                    state.cursor_pos = state.cursor_pos.saturating_sub(1);
                }
                (KeyModifiers::NONE, KeyCode::Right) if state.focus == Focus::UrlBar => {
                    state.cursor_pos = (state.cursor_pos + 1).min(state.url.len());
                }

                // URL typing
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c))
                    if state.focus == Focus::UrlBar =>
                {
                    state.url.insert(state.cursor_pos, c);
                    state.cursor_pos += 1;
                    state.status_message = None;
                }
                (KeyModifiers::NONE, KeyCode::Backspace) if state.focus == Focus::UrlBar => {
                    if state.cursor_pos > 0 {
                        state.url.remove(state.cursor_pos - 1);
                        state.cursor_pos -= 1;
                    }
                }

                // RequestPane: switch Body/Headers tab with ←→
                (KeyModifiers::NONE, KeyCode::Left)
                    if state.focus == Focus::RequestPane && state.header_editing.is_none() =>
                {
                    state.active_tab = RequestTab::Body;
                }
                (KeyModifiers::NONE, KeyCode::Right)
                    if state.focus == Focus::RequestPane && state.header_editing.is_none() =>
                {
                    state.active_tab = RequestTab::Headers;
                }

                // Headers navigation (↑↓ to select rows, enter to edit, a/d to add/delete)
                (KeyModifiers::NONE, KeyCode::Up)
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers =>
                {
                    if !state.headers.is_empty() && state.header_selected > 0 {
                        state.header_selected -= 1;
                    }
                }
                (KeyModifiers::NONE, KeyCode::Down)
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers =>
                {
                    if !state.headers.is_empty() && state.header_selected < state.headers.len() - 1
                    {
                        state.header_selected += 1;
                    }
                }
                (KeyModifiers::NONE, KeyCode::Char('a'))
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers =>
                {
                    state.headers.push((String::new(), String::new()));
                    state.header_selected = state.headers.len() - 1;
                    state.header_edit_buf = String::new();
                    state.header_editing = Some(HeaderField::Key);
                }
                (KeyModifiers::NONE, KeyCode::Char('d'))
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers
                        && !state.headers.is_empty() =>
                {
                    state.headers.remove(state.header_selected);
                    if state.header_selected > 0 && state.header_selected >= state.headers.len() {
                        state.header_selected -= 1;
                    }
                }
                (KeyModifiers::NONE, KeyCode::Enter)
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Headers
                        && !state.headers.is_empty() =>
                {
                    // Start editing the key of the selected row
                    state.header_edit_buf = state.headers[state.header_selected].0.clone();
                    state.header_editing = Some(HeaderField::Key);
                }

                // Body editing
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c))
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Body =>
                {
                    state.body.push(c);
                }
                (KeyModifiers::NONE, KeyCode::Backspace)
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Body =>
                {
                    state.body.pop();
                }
                (KeyModifiers::NONE, KeyCode::Enter)
                    if state.focus == Focus::RequestPane
                        && state.active_tab == RequestTab::Body =>
                {
                    state.body.push('\n');
                }

                _ => {}
            }
        }
    }

    Ok(())
}

/// Handles keyboard input while a header field is being edited.
fn handle_header_edit_key(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) {
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Esc) => {
            // Cancel — restore original value
            state.header_editing = None;
            state.header_edit_buf = String::new();
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            // Confirm current field, move to value (or finish)
            let i = state.header_selected;
            match &state.header_editing {
                Some(HeaderField::Key) => {
                    state.headers[i].0 = state.header_edit_buf.clone();
                    // Move to editing the value
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
        _ => {}
    }
}

/// Saves the current request to `request.yaml`.
fn handle_save(state: &mut AppState) {
    let request = build_request(state);
    match storage::request::save(&request_path(), &request) {
        Ok(()) => {
            state.status_message = Some("Saved to request.yaml".to_string());
        }
        Err(e) => {
            state.status_message = Some(format!("Save failed: {e}"));
        }
    }
}

/// Loads a request from `request.yaml` and populates state.
fn handle_load(state: &mut AppState) {
    match storage::request::load(&request_path()) {
        Ok(request) => {
            state.url = request.url;
            state.method = request.method;
            state.body = request.body.map(|b| b.content).unwrap_or_default();
            state.headers = request
                .headers
                .map(|h| h.into_iter().collect())
                .unwrap_or_default();
            state.cursor_pos = state.url.len();
            state.header_selected = 0;
            state.header_editing = None;
            state.header_edit_buf = String::new();
            state.status_message = Some("Loaded from request.yaml".to_string());
        }
        Err(e) => {
            state.status_message = Some(format!("Load failed: {e}"));
        }
    }
}

fn handle_send(state: &mut AppState, engine: &RequestEngine) {
    if state.request_in_flight {
        return;
    }
    if state.url.is_empty() {
        state.status_message = Some("No URL entered".to_string());
        return;
    }

    let request = build_request(state);
    let tx = state.response_tx.clone();
    let client = engine.client();

    state.request_in_flight = true;
    state.status_message = Some("Sending…".to_string());

    tokio::spawn(async move {
        let result = RequestEngine::send_with_client(&client, &request).await;
        let _ = tx.send(result).await;
    });
}

fn build_request(state: &AppState) -> Request {
    let body = if state.body.is_empty() {
        None
    } else {
        Some(Body {
            body_type: BodyType::Json,
            content: state.body.clone(),
        })
    };

    let headers = if state.headers.is_empty() {
        None
    } else {
        Some(
            state
                .headers
                .iter()
                .filter(|(k, _)| !k.is_empty())
                .cloned()
                .collect::<HashMap<String, String>>(),
        )
    };

    Request {
        name: String::from("untitled"),
        description: None,
        method: state.method.clone(),
        url: state.url.clone(),
        headers,
        params: None,
        auth: None,
        body,
        assertions: None,
        extract: None,
        pre_request: None,
        post_request: None,
        meta: None,
    }
}

fn next_method(method: &crate::models::request::HttpMethod) -> crate::models::request::HttpMethod {
    use crate::models::request::HttpMethod::{Delete, Get, Head, Options, Patch, Post, Put};
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

fn prev_method(method: &crate::models::request::HttpMethod) -> crate::models::request::HttpMethod {
    use crate::models::request::HttpMethod::{Delete, Get, Head, Options, Patch, Post, Put};
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
