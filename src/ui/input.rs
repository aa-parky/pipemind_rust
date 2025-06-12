use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::app_state::{AppState, FocusArea};

/// Renders an input field with cursor and styling
pub fn render_input(f: &mut Frame, area: Rect, app_state: &AppState) {
    let input_focused = app_state.focus == FocusArea::Input;
    let cursor_x = app_state.cursor_position;

    // Create text with visible cursor
    let input_line = if input_focused {
        // We'll create a line with: text before cursor + cursor char + text after cursor
        let (before_cursor, after_cursor) = app_state.input_buffer
            .split_at(app_state.cursor_position);

        // Create styled spans
        let mut spans = vec![
            Span::raw(before_cursor),
            Span::styled(
                if after_cursor.is_empty() { "█" } else { "│" },
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::SLOW_BLINK)
            ),
        ];

        // Add text after cursor if any
        if !after_cursor.is_empty() {
            spans.push(Span::raw(after_cursor));
        }

        Line::from(spans)
    } else {
        // Just show the input text without cursor when not focused
        Line::from(Span::raw(&app_state.input_buffer))
    };

    // Create the paragraph widget
    let border_style = if input_focused {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let text_style = if input_focused {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::Gray)
    };

    let title = if app_state.is_command_mode {
        "Command"
    } else {
        "Input"
    };

    let input_paragraph = Paragraph::new(input_line)
        .style(text_style)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border_style)
        );

    f.render_widget(input_paragraph, area);
}
