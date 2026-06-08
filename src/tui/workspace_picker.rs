use crate::app::AppState;
use crate::config::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// Renders the workspace picker overlay centered on the screen.
pub fn render(frame: &mut Frame, state: &AppState, theme: &Theme) {
    let items = &state.workspace_picker_items;

    let width = 50u16;
    let list_len = u16::try_from(items.len().max(1)).unwrap_or(20);
    // border(2) + title counts in border + hint(1)
    let height = (list_len + 3).min(frame.area().height.saturating_sub(4));

    let area = frame.area();
    let x = area.width.saturating_sub(width) / 2;
    let y = area.height.saturating_sub(height) / 2;
    let rect = Rect::new(x, y, width, height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_focused.into()))
        .title(" Switch Workspace ");

    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    if inner.height == 0 {
        return;
    }

    let list_height = inner.height.saturating_sub(1) as usize; // reserve hint line

    if items.is_empty() {
        let row = Rect::new(inner.x, inner.y, inner.width, 1);
        frame.render_widget(
            Paragraph::new("  No workspaces found")
                .style(Style::default().fg(theme.placeholder.into())),
            row,
        );
    } else {
        for (i, path) in items.iter().enumerate().take(list_height) {
            let is_selected = i == state.workspace_picker_selected;

            // Show the workspace name derived from the path stem, falling back
            // to the full path. Real name loading happens at open time.
            let label = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let style = if is_selected {
                Style::default()
                    .fg(theme.tab_active.into())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.header_normal_fg.into())
            };

            let prefix = if is_selected { "▶ " } else { "  " };
            let line = Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(label, style),
            ]);

            let row = Rect::new(
                inner.x,
                inner.y + u16::try_from(i).unwrap_or(0),
                inner.width,
                1,
            );
            frame.render_widget(Paragraph::new(line), row);
        }
    }

    // Hint line
    let hint_row = Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1);
    frame.render_widget(
        Paragraph::new("  enter open  esc cancel")
            .style(Style::default().fg(theme.placeholder.into())),
        hint_row,
    );
}
