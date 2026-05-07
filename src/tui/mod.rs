pub mod headers_editor;
pub mod request_pane;
pub mod response_pane;
pub mod status_bar;
pub mod url_bar;

use crate::app::AppState;
use crate::config::Theme;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

/// Top-level render entry point. Called every event-loop tick.
pub fn render(frame: &mut Frame, state: &AppState, theme: &Theme) {
    let area = frame.area();

    // Outer: [main content | status bar (1 line)]
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    // Main content: [request pane (top half) | response pane (bottom half)]
    let panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[0]);

    request_pane::render(frame, state, panes[0], theme);
    response_pane::render(frame, state, panes[1], theme);
    status_bar::render(frame, state, outer[1], theme);
}
