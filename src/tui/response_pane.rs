use crate::app::{AppState, Focus};
use crate::config::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Renders the response viewer pane.
pub fn render(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let focused = state.focus == Focus::ResponsePane;

    let border_style = if focused {
        Style::default().fg(theme.border_focused.into())
    } else {
        Style::default().fg(theme.border_unfocused.into())
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Response ");

    if state.request_in_flight {
        frame.render_widget(
            Paragraph::new("Sending…")
                .style(Style::default().fg(theme.status_message.into()))
                .block(block),
            area,
        );
        return;
    }

    if let Some(response) = &state.response {
        let status_color = status_color(response.status, theme);
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
        lines.extend(highlight_json(&response.body, theme));

        frame.render_widget(
            Paragraph::new(lines)
                .block(block)
                .scroll((state.response_scroll, 0))
                .wrap(Wrap { trim: false }),
            area,
        );
    } else {
        frame.render_widget(
            Paragraph::new("No response yet")
                .style(Style::default().fg(theme.placeholder.into()))
                .block(block),
            area,
        );
    }
}

fn status_color(status: u16, theme: &Theme) -> ratatui::style::Color {
    match status {
        200..=299 => theme.status_2xx.into(),
        300..=399 => theme.status_3xx.into(),
        400..=499 => theme.status_4xx.into(),
        500..=599 => theme.status_5xx.into(),
        _ => ratatui::style::Color::White,
    }
}

fn highlight_json<'a>(body: &str, theme: &Theme) -> Vec<Line<'a>> {
    let pretty = if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        serde_json::to_string_pretty(&val).unwrap_or_else(|_| body.to_string())
    } else {
        body.to_string()
    };

    pretty
        .lines()
        .map(|line| colorize_json_line(line, theme))
        .collect()
}

fn colorize_json_line(line: &str, theme: &Theme) -> Line<'static> {
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
                .fg(theme.json_key.into())
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(" "));
        spans.extend(colorize_value(rest, theme));
        return Line::from(spans);
    }

    spans.extend(colorize_value(trimmed.to_string(), theme));
    Line::from(spans)
}

fn colorize_value(s: String, theme: &Theme) -> Vec<Span<'static>> {
    let (core, tail) = if s.ends_with(',') {
        (s[..s.len() - 1].to_string(), ",")
    } else {
        (s, "")
    };

    let style = if core.starts_with('"') && core.ends_with('"') {
        Style::default().fg(theme.json_string.into())
    } else if core == "true" || core == "false" {
        Style::default().fg(theme.json_bool.into())
    } else if core == "null" {
        Style::default().fg(theme.json_null.into())
    } else if core.parse::<f64>().is_ok() {
        Style::default().fg(theme.json_number.into())
    } else {
        Style::default().fg(theme.json_punctuation.into())
    };

    if tail.is_empty() {
        vec![Span::styled(core, style)]
    } else {
        vec![
            Span::styled(core, style),
            Span::styled(
                tail.to_string(),
                Style::default().fg(theme.json_punctuation.into()),
            ),
        ]
    }
}
