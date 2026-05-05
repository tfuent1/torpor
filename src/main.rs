mod app;
mod engine;
mod models;
mod storage;
mod tui;

use crate::app::{AppState, Focus, RequestTab};
use crate::engine::RequestEngine;
use crate::models::request::{Body, BodyType, Request};
use anyhow::Context;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::collections::HashMap;
use std::io;
use std::time::Duration;

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

    // Restore terminal regardless of how we exited
    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    result
}

/// Main event loop. Draws on every iteration, polls the response channel,
/// then blocks up to 50ms waiting for a keyboard event.
fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
    engine: &RequestEngine,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| tui::render(frame, state))?;

        // Non-blocking check for a completed request
        if let Ok(result) = state.response_rx.try_recv() {
            state.request_in_flight = false;
            match result {
                Ok(response) => {
                    state.response = Some(response);
                    state.status_message = None;
                }
                Err(e) => {
                    state.status_message = Some(format!("Error: {e}"));
                }
            }
        }

        // Block up to 50ms for a keyboard event. If nothing arrives we loop
        // back and redraw — this keeps the spinner and channel poll responsive.
        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match (key.modifiers, key.code) {
                // Quit — only when not editing (UrlBar or RequestPane body)
                (KeyModifiers::NONE, KeyCode::Char('q'))
                    if state.focus == Focus::ResponsePane =>
                {
                    break;
                }

                // Ctrl+Q quits from anywhere
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,

                // Send — Ctrl+Enter from anywhere
                (KeyModifiers::CONTROL, KeyCode::Enter) => {
                    handle_send(state, engine);
                }

                // Tab cycles focus forward
                (KeyModifiers::NONE, KeyCode::Tab) => {
                    state.focus = match state.focus {
                        Focus::UrlBar => Focus::RequestPane,
                        Focus::RequestPane => Focus::ResponsePane,
                        Focus::ResponsePane => Focus::UrlBar,
                    };
                }

                // Shift+Tab cycles focus backward
                (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                    state.focus = match state.focus {
                        Focus::UrlBar => Focus::ResponsePane,
                        Focus::RequestPane => Focus::UrlBar,
                        Focus::ResponsePane => Focus::RequestPane,
                    };
                }

                // Method cycle when UrlBar focused - arrow keys only, no h/l
                (KeyModifiers::NONE, KeyCode::Up)
                    if state.focus == Focus::UrlBar =>
                {
                    state.method = prev_method(&state.method);
                }
                (KeyModifiers::NONE, KeyCode::Down)
                    if state.focus == Focus::UrlBar =>
                {
                    state.method = next_method(&state.method);
                }

                // Cursor movement in URL bar
                (KeyModifiers::NONE, KeyCode::Left)
                    if state.focus == Focus::UrlBar =>
                {
                    state.cursor_pos = state.cursor_pos.saturating_sub(1);
                }
                (KeyModifiers::NONE, KeyCode::Right)
                    if state.focus == Focus::UrlBar =>
                {
                    state.cursor_pos = (state.cursor_pos + 1).min(state.url.len());
                }

                // URL editing when UrlBar focused
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c))
                    if state.focus == Focus::UrlBar =>
                {
                    state.url.insert(state.cursor_pos, c);
                    state.cursor_pos += 1;
                    state.status_message = None;
                }
                (KeyModifiers::NONE, KeyCode::Backspace)
                    if state.focus == Focus::UrlBar =>
                {
                    if state.cursor_pos > 0 {
                        state.url.remove(state.cursor_pos - 1);
                        state.cursor_pos -= 1;
                    }
                }

                // Tab switching when RequestPane focused
                (KeyModifiers::NONE, KeyCode::Left)
                    if state.focus == Focus::RequestPane =>
                {
                    state.active_tab = RequestTab::Body;
                }
                (KeyModifiers::NONE, KeyCode::Right)
                    if state.focus == Focus::RequestPane =>
                {
                    state.active_tab = RequestTab::Headers;
                }

                // Body editing when RequestPane focused and on Body tab
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

/// Builds a `Request` from current `AppState` and spawns it as a Tokio task.
/// The result is sent back through `state.response_rx`.
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
        // Receiver gone means the app is shutting down — silently drop
        let _ = tx.send(result).await;
    });
}

/// Translates `AppState` into a `Request` model for the engine.
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

/// Advances to the next HTTP method in the cycle.
fn next_method(method: &crate::models::request::HttpMethod) -> crate::models::request::HttpMethod {
    use crate::models::request::HttpMethod::{Get, Post, Put, Patch, Delete, Head, Options};
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

/// Steps back to the previous HTTP method in the cycle.
fn prev_method(method: &crate::models::request::HttpMethod) -> crate::models::request::HttpMethod {
    use crate::models::request::HttpMethod::{Get, Post, Put, Patch, Delete, Head, Options};
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
