use crate::app::AppState;
use crate::config::Theme;
use crate::config::theme::all_builtin;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// Renders the theme selector overlay centered on the screen.
pub fn render(frame: &mut Frame, state: &AppState, theme: &Theme) {
    let themes = all_builtin();

    // Size the box to fit the longest theme name plus padding
    let width = 30u16;
    let height = u16::try_from(themes.len()).unwrap_or(10) + 4; // 2 border + 1 title + 1 hint

    // Center it
    let area = frame.area();
    let x = area.width.saturating_sub(width) / 2;
    let y = area.height.saturating_sub(height) / 2;
    let rect = Rect::new(x, y, width, height);

    // Clear the area behind the overlay
    frame.render_widget(Clear, rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_focused.into()))
        .title(" Theme ");

    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    // Theme list
    let list_height = inner.height.saturating_sub(1) as usize; // reserve 1 line for hint
    for (i, t) in themes.iter().enumerate().take(list_height) {
        let is_selected = i == state.theme_selector_index;

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
            Span::styled(t.name.clone(), style),
        ]);

        let row = Rect::new(
            inner.x,
            inner.y + u16::try_from(i).unwrap_or(0),
            inner.width,
            1,
        );
        frame.render_widget(Paragraph::new(line), row);
    }

    // Hint line at the bottom
    let hint_row = Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1);
    frame.render_widget(
        Paragraph::new("enter apply  esc cancel")
            .style(Style::default().fg(theme.placeholder.into())),
        hint_row,
    );
}
