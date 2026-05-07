use crate::app::{AppState, Focus};
use crate::config::Theme;
use crate::models::request::HttpMethod;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

/// Renders the URL bar (method selector + URL input).
pub fn render(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let focused = state.focus == Focus::UrlBar;

    let border_style = if focused {
        Style::default().fg(theme.border_focused.into())
    } else {
        Style::default().fg(theme.border_unfocused.into())
    };

    let method_str = method_label(&state.method);
    let url_text = format!("[{}]  {}", method_str, state.url);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Request ");

    frame.render_widget(Paragraph::new(url_text).block(block), area);

    if focused {
        let prefix_len = method_label(&state.method).len() + 5; // "[METHOD]  "
        let cx = area.x
            + u16::try_from(prefix_len).unwrap_or(0)
            + u16::try_from(state.cursor_pos).unwrap_or(0);
        frame.set_cursor_position((cx, area.y + 1));
    }
}

pub fn method_label(method: &HttpMethod) -> &'static str {
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
