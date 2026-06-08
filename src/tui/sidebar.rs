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

    // Build tree items. For now: workspace name row + collections placeholder.
    // Chunk 5 will populate real collection/request entries here.
    let items = build_tree_items(state);

    for (i, item) in items.iter().enumerate() {
        let row_y = inner.y + u16::try_from(i).unwrap_or(u16::MAX);
        if row_y >= inner.y + inner.height {
            break;
        }

        let is_selected = focused && i == state.sidebar_selected;
        let style = if is_selected {
            Style::default()
                .fg(theme.tab_active.into())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.header_normal_fg.into())
        };

        let line = Line::from(vec![Span::styled(item.clone(), style)]);
        let row = Rect::new(inner.x, row_y, inner.width, 1);
        frame.render_widget(Paragraph::new(line), row);
    }
}

/// Builds the flat list of displayable tree items from current workspace state.
/// Returns labels with indentation. Chunk 5 will replace this with real data.
fn build_tree_items(state: &AppState) -> Vec<String> {
    // If the workspace has no path it's in-memory / unsaved
    let mut items = Vec::new();

    if state.workspace.path.is_none() {
        items.push("  (unsaved workspace)".to_string());
        return items;
    }

    // Placeholder until Chunk 5 loads collections
    items.push("  No collections".to_string());
    items
}
