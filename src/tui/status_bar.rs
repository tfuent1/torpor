use crate::app::{AppState, Focus, RequestTab};
use crate::config::Theme;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};

/// Renders the single-line status bar at the bottom.
pub fn render(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let (message, is_status) = resolve_message(state);

    let style = if is_status {
        Style::default().fg(theme.status_message.into())
    } else {
        Style::default().fg(theme.status_hint.into())
    };

    frame.render_widget(Paragraph::new(message).style(style), area);
}

/// Returns `(text, is_status_message)`.
/// `true` means it's a user-visible action result — use the highlighted color.
/// `false` means it's a contextual hint — use the dim color.
fn resolve_message(state: &AppState) -> (String, bool) {
    if let Some(msg) = &state.status_message {
        return (msg.clone(), true);
    }

    let hint = match &state.focus {
        Focus::UrlBar => {
            " ↑↓ method  ←→ cursor  ctrl+r send  ctrl+d clear  ctrl+s save  ctrl+o load  ctrl+q quit"
        }
        Focus::RequestPane if state.active_tab == RequestTab::Headers => {
            if state.header_editing.is_some() {
                " enter confirm  esc cancel"
            } else {
                " ↑↓ select  enter edit  a add  d delete  ctrl+←→ switch tab  tab focus  ctrl+r send"
            }
        }
        Focus::RequestPane => {
            " arrows move  ctrl+←→ switch tab  tab focus  ctrl+r send  ctrl+s save  ctrl+q quit"
        }
        Focus::ResponsePane => {
            " j/k scroll  tab focus  ctrl+r send  ctrl+s save  ctrl+o load  q/ctrl+q quit"
        }
    };

    (hint.to_string(), false)
}
