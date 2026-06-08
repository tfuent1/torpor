use crate::app::{AppState, Focus};
use crate::config::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Renders the workspace sidebar.
pub fn render(frame: &mut Frame, state: &AppState, area: Rect, theme: &Theme) {
    let focused = state.focus == Focus::Sidebar;

    let border_style = if focused {
        Style::default().fg(theme.border_focused.into())
    } else {
        Style::default().fg(theme.border_unfocused.into())
    };

    let workspace_name = if state.workspace.workspace.name.is_empty() {
        "untitled".to_string()
    } else {
        state.workspace.workspace.name.clone()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" {workspace_name} "));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height == 0 {
        return;
    }

    let items = build_tree_items(state);

    for (i, (label, is_request)) in items.iter().enumerate() {
        let row_y = inner.y + u16::try_from(i).unwrap_or(u16::MAX);
        if row_y >= inner.y + inner.height {
            break;
        }

        let is_selected = focused && i == state.sidebar_selected;
        let fg = if *is_request {
            theme.header_normal_fg.into()
        } else {
            theme.tab_inactive.into()
        };

        let style = if is_selected {
            Style::default()
                .fg(theme.tab_active.into())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(fg)
        };

        let line = Line::from(vec![Span::styled(label.clone(), style)]);
        let row = Rect::new(inner.x, row_y, inner.width, 1);
        frame.render_widget(Paragraph::new(line), row);
    }

    // Render confirm/input prompt inline at bottom if open
    if let Some(ref msg) = state.confirm_prompt {
        let prompt_y = inner.y + inner.height.saturating_sub(2);
        let prompt_row = Rect::new(inner.x, prompt_y, inner.width, 1);
        frame.render_widget(
            Paragraph::new(format!("{msg} y/n"))
                .style(Style::default().fg(theme.status_message.into())),
            prompt_row,
        );
    } else if let Some((ref label, ref input)) = state.input_prompt {
        let prompt_y = inner.y + inner.height.saturating_sub(2);
        let prompt_row = Rect::new(inner.x, prompt_y, inner.width, 1);
        frame.render_widget(
            Paragraph::new(format!("{label}: {input}"))
                .style(Style::default().fg(theme.tab_active.into())),
            prompt_row,
        );
    }
}

/// Builds a flat list of `(label, is_request)` from loaded collections.
/// Collections are bold, requests are indented.
fn build_tree_items(state: &AppState) -> Vec<(String, bool)> {
    if state.workspace.path.is_none() {
        return vec![("  (unsaved workspace)".to_string(), false)];
    }

    if state.loaded_collections.is_empty() {
        return vec![("  No collections".to_string(), false)];
    }

    let mut items = Vec::new();
    for col in &state.loaded_collections {
        items.push((format!(" ▸ {}", col.collection.name), false));
        for (stem, _req) in &col.requests {
            items.push((format!("    {stem}"), true));
        }
    }
    items
}
