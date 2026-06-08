pub mod headers_editor;
pub mod request_pane;
pub mod response_pane;
pub mod sidebar;
pub mod status_bar;
pub mod theme_selector;
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

    // Main content: [sidebar | request+response panes]
    let main = if state.sidebar_open {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(24), Constraint::Min(0)])
            .split(outer[0])
    } else {
        // When sidebar is hidden, give everything to the right pane
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(0), Constraint::Min(0)])
            .split(outer[0])
    };

    // Right side: [request pane (top half) | response pane (bottom half)]
    let panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    if state.sidebar_open {
        sidebar::render(frame, state, main[0], theme);
    }

    request_pane::render(frame, state, panes[0], theme);
    response_pane::render(frame, state, panes[1], theme);
    status_bar::render(frame, state, outer[1], theme);

    if state.theme_selector_open {
        theme_selector::render(frame, state, theme);
    }
}
