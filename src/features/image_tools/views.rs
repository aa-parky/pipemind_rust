use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::core::app_state::AppState;

pub fn render_image_tools_view(f: &mut Frame, area: Rect, app_state: &AppState) {
    let text = match app_state.is_command_mode {
        true => format!("Command Mode: {}", app_state.input_buffer),
        false => app_state.image_tools_state.welcome_message.clone(),
    };

    let block = Block::default()
        .title("Image Tools")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(block);

    f.render_widget(paragraph, area);
}