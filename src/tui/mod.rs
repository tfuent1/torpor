use crate::app::{AppState, Focus, HeaderField, RequestTab};
use crate::models::request::HttpMethod;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Top-level render function. Called on every tick of the event loop.
pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[0]);

    render_request_pane(frame, state, panes[0]);
    render_response_pane(frame, state, panes[1]);
    render_status_bar(frame, state, outer[1]);
}

fn render_request_pane(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let url_area = sections[0];
    let content_area = sections[1];

    // URL bar
    let url_focused = state.focus == Focus::UrlBar;
    let method_str = method_label(&state.method);
    let url_text = format!("[{}]  {}", method_str, state.url);
    let url_block = Block::default()
        .borders(Borders::ALL)
        .border_style(focus_style(url_focused))
        .title(" Request ");
    frame.render_widget(Paragraph::new(url_text).block(url_block), url_area);

    if url_focused {
        let prefix = method_label(&state.method).len() + 5;
        let cx = url_area.x
            + u16::try_from(prefix).unwrap_or(0)
            + u16::try_from(state.cursor_pos).unwrap_or(0);
        frame.set_cursor_position((cx, url_area.y + 1));
    }

    // Tab bar + content
    let content_focused = state.focus == Focus::RequestPane;
    let tab_title = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "Body",
            if state.active_tab == RequestTab::Body {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(240))
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "Headers",
            if state.active_tab == RequestTab::Headers {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
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

    match state.active_tab {
        RequestTab::Body => {
            let body_text = state.body_text();
            let para = if body_text.trim().is_empty() {
                Paragraph::new("No body").style(Style::default().fg(Color::DarkGray))
            } else {
                let lines: Vec<Line> = state
                    .body_lines
                    .iter()
                    .map(|l| Line::raw(l.clone()))
                    .collect();
                Paragraph::new(lines)
            };
            frame.render_widget(para.block(content_block), content_area);

            // Place cursor in body pane when focused
            if content_focused {
                let inner_x = content_area.x + 1;
                let inner_y = content_area.y + 1;
                let cx = inner_x + u16::try_from(state.body_cursor_col).unwrap_or(0);
                let cy = inner_y + u16::try_from(state.body_cursor_row).unwrap_or(0);
                let max_x = content_area.x + content_area.width.saturating_sub(2);
                let max_y = content_area.y + content_area.height.saturating_sub(2);
                if cx <= max_x && cy <= max_y {
                    frame.set_cursor_position((cx, cy));
                }
            }
        }
        RequestTab::Headers => {
            render_headers_editor(frame, state, content_block, content_area);
        }
    }
}

/// Renders the headers table with selection highlight and inline editing.
#[allow(clippy::too_many_lines)]
fn render_headers_editor(
    frame: &mut Frame,
    state: &AppState,
    block: Block,
    area: ratatui::layout::Rect,
) {
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.height == 0 {
        return;
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(inner);

    let key_col = cols[0];
    let val_col = cols[1];

    let hdr_rect_k = ratatui::layout::Rect {
        height: 1,
        ..key_col
    };
    let hdr_rect_v = ratatui::layout::Rect {
        height: 1,
        ..val_col
    };
    frame.render_widget(
        Paragraph::new("Key").style(
            Style::default()
                .fg(Color::Indexed(244))
                .add_modifier(Modifier::BOLD),
        ),
        hdr_rect_k,
    );
    frame.render_widget(
        Paragraph::new("Value").style(
            Style::default()
                .fg(Color::Indexed(244))
                .add_modifier(Modifier::BOLD),
        ),
        hdr_rect_v,
    );

    let rows_start_y = inner.y + 1;
    let max_rows = inner.height.saturating_sub(2) as usize;

    for (i, (k, v)) in state.headers.iter().enumerate().take(max_rows) {
        let row_y = rows_start_y + u16::try_from(i).unwrap_or(u16::MAX);
        if row_y >= inner.y + inner.height {
            break;
        }

        let is_selected = i == state.header_selected && state.focus == Focus::RequestPane;
        let row_bg = if is_selected {
            Color::Indexed(236)
        } else {
            Color::Reset
        };

        let editing_key = is_selected && state.header_editing == Some(HeaderField::Key);
        let editing_val = is_selected && state.header_editing == Some(HeaderField::Value);

        let key_text = if editing_key {
            state.header_edit_buf.clone()
        } else {
            k.clone()
        };
        let val_text = if editing_val {
            state.header_edit_buf.clone()
        } else {
            v.clone()
        };

        let key_style = if editing_key {
            Style::default().fg(Color::Yellow).bg(row_bg)
        } else if is_selected {
            Style::default()
                .fg(Color::White)
                .bg(row_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Indexed(250)).bg(row_bg)
        };

        let val_style = if editing_val {
            Style::default().fg(Color::Yellow).bg(row_bg)
        } else if is_selected {
            Style::default().fg(Color::Indexed(250)).bg(row_bg)
        } else {
            Style::default().fg(Color::Indexed(244)).bg(row_bg)
        };

        let k_rect = ratatui::layout::Rect {
            y: row_y,
            height: 1,
            ..key_col
        };
        let v_rect = ratatui::layout::Rect {
            y: row_y,
            height: 1,
            ..val_col
        };

        frame.render_widget(Paragraph::new(key_text).style(key_style), k_rect);
        frame.render_widget(Paragraph::new(val_text).style(val_style), v_rect);

        if editing_key {
            let cx = k_rect.x + u16::try_from(state.header_edit_buf.len()).unwrap_or(0);
            if cx < k_rect.x + k_rect.width {
                frame.set_cursor_position((cx, row_y));
            }
        } else if editing_val {
            let cx = v_rect.x + u16::try_from(state.header_edit_buf.len()).unwrap_or(0);
            if cx < v_rect.x + v_rect.width {
                frame.set_cursor_position((cx, row_y));
            }
        }
    }

    let hint_y = inner.y + inner.height - 1;
    frame.render_widget(
        Paragraph::new("a add  d del  enter edit  ctrl+←→ switch tab")
            .style(Style::default().fg(Color::Indexed(238))),
        ratatui::layout::Rect {
            x: inner.x,
            y: hint_y,
            width: inner.width,
            height: 1,
        },
    );
}

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
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "  {}ms  {}b",
                response.duration_ms, response.size_bytes
            )),
        ]);

        let mut lines = vec![status_line, Line::raw("")];
        lines.extend(highlight_json(&response.body));
        Paragraph::new(lines)
            .block(block)
            .scroll((state.response_scroll, 0))
            .wrap(ratatui::widgets::Wrap { trim: false })
    } else {
        Paragraph::new("No response yet")
            .style(Style::default().fg(Color::DarkGray))
            .block(block)
    };

    frame.render_widget(content, area);
}

fn highlight_json(body: &str) -> Vec<Line<'static>> {
    let pretty = if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        serde_json::to_string_pretty(&val).unwrap_or_else(|_| body.to_string())
    } else {
        body.to_string()
    };

    pretty.lines().map(colorize_json_line).collect()
}

fn colorize_json_line(line: &str) -> Line<'static> {
    let trimmed = line.trim_start();
    let indent = " ".repeat(line.len() - trimmed.len());
    let mut spans: Vec<Span<'static>> = vec![Span::raw(indent)];

    if trimmed.starts_with('"')
        && let Some(colon_pos) = trimmed.find("\": ")
    {
        let key_end = colon_pos + 2;
        let key_part = trimmed[..key_end].to_string();
        let rest = trimmed[key_end + 1..].trim_start().to_string();

        spans.push(Span::styled(
            key_part,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(" "));
        spans.extend(colorize_value(rest));
        return Line::from(spans);
    }

    spans.extend(colorize_value(trimmed.to_string()));
    Line::from(spans)
}

fn colorize_value(s: String) -> Vec<Span<'static>> {
    let (core, tail) = if s.ends_with(',') {
        (s[..s.len() - 1].to_string(), ",")
    } else {
        (s, "")
    };

    let style = if core.starts_with('"') && core.ends_with('"') {
        Style::default().fg(Color::Green)
    } else if core == "true" || core == "false" {
        Style::default().fg(Color::Magenta)
    } else if core == "null" {
        Style::default().fg(Color::Indexed(240))
    } else if core.parse::<f64>().is_ok() {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Indexed(244))
    };

    if tail.is_empty() {
        vec![Span::styled(core, style)]
    } else {
        vec![
            Span::styled(core, style),
            Span::styled(tail.to_string(), Style::default().fg(Color::Indexed(244))),
        ]
    }
}

fn render_status_bar(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let message = if let Some(msg) = &state.status_message {
        msg.clone()
    } else {
        match &state.focus {
            Focus::UrlBar => " ↑↓ method  ←→ cursor  ctrl+r send  ctrl+d clear  ctrl+s save  ctrl+o load  ctrl+q quit".to_string(),
            Focus::RequestPane if state.active_tab == RequestTab::Headers => {
                if state.header_editing.is_some() {
                    " enter confirm  esc cancel".to_string()
                } else {
                    " ↑↓ select  enter edit  a add  d delete  ctrl+←→ switch tab  tab focus  ctrl+r send".to_string()
                }
            }
            Focus::RequestPane => " arrows move  ctrl+←→ switch tab  tab focus  ctrl+r send  ctrl+s save  ctrl+q quit".to_string(),
            Focus::ResponsePane => " j/k scroll  tab focus  ctrl+r send  ctrl+s save  ctrl+o load  q/ctrl+q quit".to_string(),
        }
    };

    let style = if state.status_message.is_some() {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Indexed(240))
    };

    frame.render_widget(Paragraph::new(message).style(style), area);
}

fn focus_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Indexed(240))
    }
}

fn status_color(status: u16) -> Color {
    match status {
        200..=299 => Color::Green,
        300..=399 => Color::Cyan,
        400..=499 => Color::Yellow,
        500..=599 => Color::Red,
        _ => Color::White,
    }
}

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
