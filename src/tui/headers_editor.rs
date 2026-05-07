use crate::app::{AppState, Focus, HeaderField};
use crate::config::Theme;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Paragraph},
};

/// Renders the headers key/value table with selection and inline edit support.
#[allow(clippy::too_many_lines)]
pub fn render(frame: &mut Frame, state: &AppState, block: Block, area: Rect, theme: &Theme) {
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.height == 0 {
        return;
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(inner);

    let key_col = cols[0];
    let val_col = cols[1];

    // Column headings
    let heading_style = Style::default()
        .fg(theme.header_value_fg.into())
        .add_modifier(Modifier::BOLD);

    frame.render_widget(
        Paragraph::new("Key").style(heading_style),
        Rect {
            height: 1,
            ..key_col
        },
    );
    frame.render_widget(
        Paragraph::new("Value").style(heading_style),
        Rect {
            height: 1,
            ..val_col
        },
    );

    let rows_start_y = inner.y + 1;
    let max_rows = inner.height.saturating_sub(2) as usize;

    for (i, (k, v)) in state.headers.iter().enumerate().take(max_rows) {
        let row_y = rows_start_y + u16::try_from(i).unwrap_or(u16::MAX);
        if row_y >= inner.y + inner.height {
            break;
        }

        let is_selected = i == state.header_selected && state.focus == Focus::RequestPane;
        let row_bg = if is_selected {
            theme.header_selected_bg.into()
        } else {
            ratatui::style::Color::Reset
        };

        let editing_key = is_selected && state.header_editing == Some(HeaderField::Key);
        let editing_val = is_selected && state.header_editing == Some(HeaderField::Value);

        let key_text = if editing_key {
            state.header_edit_buf.clone()
        } else {
            k.clone()
        };
        let val_text = if editing_val {
            state.header_edit_buf.clone()
        } else {
            v.clone()
        };

        let key_style = if editing_key {
            Style::default().fg(theme.tab_active.into()).bg(row_bg)
        } else if is_selected {
            Style::default()
                .fg(theme.header_selected_fg.into())
                .bg(row_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.header_normal_fg.into())
                .bg(row_bg)
        };

        let val_style = if editing_val {
            Style::default().fg(theme.tab_active.into()).bg(row_bg)
        } else {
            Style::default().fg(theme.header_value_fg.into()).bg(row_bg)
        };

        let k_rect = Rect {
            y: row_y,
            height: 1,
            ..key_col
        };
        let v_rect = Rect {
            y: row_y,
            height: 1,
            ..val_col
        };

        frame.render_widget(Paragraph::new(key_text).style(key_style), k_rect);
        frame.render_widget(Paragraph::new(val_text).style(val_style), v_rect);

        if editing_key {
            let cx = k_rect.x + u16::try_from(state.header_edit_buf.len()).unwrap_or(0);
            if cx < k_rect.x + k_rect.width {
                frame.set_cursor_position((cx, row_y));
            }
        } else if editing_val {
            let cx = v_rect.x + u16::try_from(state.header_edit_buf.len()).unwrap_or(0);
            if cx < v_rect.x + v_rect.width {
                frame.set_cursor_position((cx, row_y));
            }
        }
    }

    // Footer hint
    let hint_y = inner.y + inner.height - 1;
    frame.render_widget(
        Paragraph::new("a add  d del  enter edit  ctrl+←→ switch tab")
            .style(Style::default().fg(theme.placeholder.into())),
        Rect {
            x: inner.x,
            y: hint_y,
            width: inner.width,
            height: 1,
        },
    );
}
