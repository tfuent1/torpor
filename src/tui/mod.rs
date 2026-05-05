use crate::app::{AppState, Focus, RequestTab};
use crate::models::request::HttpMethod;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

/// Top-level render function. Called on every tick of the event loop.
/// Splits the terminal into request and response panes with a status bar.
pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    // Outer layout: main area + status bar
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let main_area = outer[0];
    let status_area = outer[1];

    // Main area: request pane (top) + response pane (bottom)
    let panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_area);

    let request_area = panes[0];
    let response_area = panes[1];

    render_request_pane(frame, state, request_area);
    render_response_pane(frame, state, response_area);
    render_status_bar(frame, state, status_area);
}

/// Renders the request pane: URL bar, tab bar, and active tab content.
fn render_request_pane(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    // Split request pane: URL bar + tab bar + content
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // URL bar
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Tab content
        ])
        .split(area);

    let url_area = sections[0];
    let tab_area = sections[1];
    let content_area = sections[2];

    // URL bar
    let url_focused = state.focus == Focus::UrlBar;
    let method_str = method_label(&state.method);
    let url_text = format!("[{}]  {}", method_str, state.url);
    let url_block = Block::default()
        .borders(Borders::ALL)
        .border_style(focus_style(url_focused))
        .title(" Request ");
    let url_paragraph = Paragraph::new(url_text).block(url_block);
    frame.render_widget(url_paragraph, url_area);

    // Place terminal cursor inside the URL bar when focused
    if url_focused {
        // x: 2 (left border) + method label + 3 chars for "[GET]  " prefix
        let method_prefix_len = method_label(&state.method).len() + 5; // "[" + method + "]  "
        let cursor_x = url_area.x + method_prefix_len as u16 + state.cursor_pos as u16;
        let cursor_y = url_area.y + 1; // +1 to move inside the border
        frame.set_cursor_position((cursor_x, cursor_y));
    }


    // Tab Bar / Content
    let content_focused = state.focus == Focus::RequestPane;

    let tab_title = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "Body",
            if state.active_tab == RequestTab::Body {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(240))
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "Headers",
            if state.active_tab == RequestTab::Headers {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(240))
            },
        ),
        Span::raw(" "),
    ]);

    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(focus_style(content_focused))
        .title(tab_title);

    let content_area = sections[1].union(sections[2]);

    let content_text = match state.active_tab {
        RequestTab::Body => {
            if state.body.is_empty() {
                Paragraph::new("No body").style(Style::default().fg(Color::DarkGray))
            } else {
                Paragraph::new(state.body.clone())
            }
        }
        RequestTab::Headers => {
            if state.headers.is_empty() {
                Paragraph::new("No headers").style(Style::default().fg(Color::DarkGray))
            } else {
                let lines: Vec<Line> = state
                    .headers
                    .iter()
                    .map(|(k, v)| Line::from(format!("{k}: {v}")))
                    .collect();
                Paragraph::new(lines)
            }
        }
    };

    frame.render_widget(content_text.block(content_block), content_area);
}

/// Renders the response pane: status line and body.
fn render_response_pane(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let focused = state.focus == Focus::ResponsePane;
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(focus_style(focused))
        .title(" Response ");

    let content = if state.request_in_flight {
        Paragraph::new("Sending…")
            .style(Style::default().fg(Color::Yellow))
            .block(block)
    } else if let Some(response) = &state.response {
        let status_color = status_color(response.status);
        let status_line = Line::from(vec![
            Span::styled(
                format!("{}", response.status),
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "  {}ms  {}b",
                response.duration_ms, response.size_bytes
            )),
        ]);
        let mut lines = vec![status_line, Line::raw("")];
        for line in response.body.lines() {
            lines.push(Line::raw(line.to_string()));
        }
        Paragraph::new(lines).block(block)
    } else {
        Paragraph::new("No response yet")
            .style(Style::default().fg(Color::DarkGray))
            .block(block)
    };

    frame.render_widget(content, area);
}

/// Renders the one-line status bar at the bottom of the screen.
fn render_status_bar(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let message = state
        .status_message
        .as_deref()
        .unwrap_or("ctrl+s save  ctrl+enter send  tab focus  q quit");

    let style = if state.status_message.is_some() {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let bar = Paragraph::new(message).style(style);
    frame.render_widget(bar, area);
}

/// Returns a bright border style when focused, dim when not.
fn focus_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Indexed(240))
    }
}

/// Returns a colour appropriate for the HTTP status code.
fn status_color(status: u16) -> Color {
    match status {
        200..=299 => Color::Green,
        300..=399 => Color::Cyan,
        400..=499 => Color::Yellow,
        500..=599 => Color::Red,
        _ => Color::White,
    }
}

/// Returns a short label for the HTTP method.
fn method_label(method: &HttpMethod) -> &'static str {
    match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Patch => "PATCH",
        HttpMethod::Delete => "DELETE",
        HttpMethod::Head => "HEAD",
        HttpMethod::Options => "OPTIONS",
    }
}
