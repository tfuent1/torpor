mod app;
mod config;
mod engine;
mod events;
mod models;
mod storage;
mod tui;

use crate::app::AppState;
use crate::config::{Config, KeyBinds, Theme};
use crate::engine::RequestEngine;
use crate::events::Action;
use crate::models::request::{Body, BodyType, Request};
use anyhow::Context;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

fn request_path() -> PathBuf {
    PathBuf::from("request.yaml")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load or create default config
    Config::ensure_default().ok(); // non-fatal if config dir is unwritable
    let config = Config::load().unwrap_or_default();
    let theme = config.resolve_theme();
    let binds = config.keybinds.clone();

    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to create terminal")?;

    let engine = RequestEngine::new().context("failed to create request engine")?;
    let mut state = AppState::new();

    let result = run(&mut terminal, &mut state, &engine, &theme, &binds);

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
    engine: &RequestEngine,
    theme: &Theme,
    binds: &KeyBinds,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| tui::render(frame, state, theme))?;

        // Poll for completed async requests
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
            match events::dispatch(state, key.modifiers, key.code, binds) {
                Action::Quit => break,
                Action::SendRequest => handle_send(state, engine),
                Action::SaveRequest => handle_save(state),
                Action::LoadRequest => handle_load(state),
                Action::Continue => {}
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Request I/O helpers
// ---------------------------------------------------------------------------

fn handle_save(state: &mut AppState) {
    let request = build_request(state);
    match storage::request::save(&request_path(), &request) {
        Ok(()) => state.status_message = Some("Saved to request.yaml".to_string()),
        Err(e) => state.status_message = Some(format!("Save failed: {e}")),
    }
}

fn handle_load(state: &mut AppState) {
    match storage::request::load(&request_path()) {
        Ok(request) => {
            state.url = request.url.clone();
            state.method = request.method;
            let body_text = request.body.map(|b| b.content).unwrap_or_default();
            state.set_body_text(&body_text);
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
        Err(e) => state.status_message = Some(format!("Load failed: {e}")),
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
    let body_text = state.body_text();
    let body = if body_text.trim().is_empty() {
        None
    } else {
        Some(Body {
            body_type: BodyType::Json,
            content: body_text,
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
