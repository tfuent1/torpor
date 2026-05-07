use crate::app::{AppState, Focus, RequestTab};
use crate::config::Theme;
use crate::tui::headers_editor;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Renders the request editor pane (URL bar + tab bar + body/headers content).
pub fn render(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    crate::tui::url_bar::render(frame, state, sections[0], theme);
    render_editor(frame, state, sections[1], theme);
}

fn render_editor(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let focused = state.focus == Focus::RequestPane;

    let border_style = if focused {
        Style::default().fg(theme.border_focused.into())
    } else {
        Style::default().fg(theme.border_unfocused.into())
    };

    let tab_title = build_tab_title(state, theme);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(tab_title);

    match state.active_tab {
        RequestTab::Body => render_body(frame, state, block, area, focused, theme),
        RequestTab::Headers => headers_editor::render(frame, state, block, area, theme),
    }
}

fn build_tab_title<'a>(state: &AppState, theme: &Theme) -> Line<'a> {
    Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "Body",
            if state.active_tab == RequestTab::Body {
                Style::default()
                    .fg(theme.tab_active.into())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.tab_inactive.into())
            },
        ),
        Span::raw(" | "),
        Span::styled(
            "Headers",
            if state.active_tab == RequestTab::Headers {
                Style::default()
                    .fg(theme.tab_active.into())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.tab_inactive.into())
            },
        ),
        Span::raw(" "),
    ])
}

fn render_body(
    frame: &mut Frame,
    state: &AppState,
    block: Block,
    area: Rect,
    focused: bool,
    theme: &Theme,
) {
    let body_text = state.body_text();
    let para = if body_text.trim().is_empty() {
        Paragraph::new("No body")
            .style(Style::default().fg(theme.placeholder.into()))
            .block(block)
    } else {
        let lines: Vec<Line> = state
            .body_lines
            .iter()
            .map(|l| Line::raw(l.clone()))
            .collect();
        Paragraph::new(lines).block(block)
    };

    frame.render_widget(para, area);

    if focused {
        let inner_x = area.x + 1;
        let inner_y = area.y + 1;
        let cx = inner_x + u16::try_from(state.body_cursor_col).unwrap_or(0);
        let cy = inner_y + u16::try_from(state.body_cursor_row).unwrap_or(0);
        let max_x = area.x + area.width.saturating_sub(2);
        let max_y = area.y + area.height.saturating_sub(2);
        if cx <= max_x && cy <= max_y {
            frame.set_cursor_position((cx, cy));
        }
    }
}
