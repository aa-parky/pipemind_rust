use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::core::app_state::{AppState, FocusArea};

pub fn render_preview(f: &mut Frame, area: Rect, app_state: &AppState) {
    let preview_block = Block::default()
        .title("Preview")
        .borders(Borders::ALL)
        .style(Style::default().fg(if app_state.focus == FocusArea::Preview {
            Color::White
        } else {
            Color::DarkGray
        }));

    let preview_text = Paragraph::new(app_state.preview_content.as_str())
        .block(preview_block);

    f.render_widget(preview_text, area);
}