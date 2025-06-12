use ratatui::{
    layout::{Layout, Rect, Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::core::app_state::{AppState, FocusArea};

// Existing functions remain the same...

pub fn create_bordered_paragraph<'a>(
    content: &'a str,
    title: Option<&'a str>,
    app_state: &AppState,
    focus_area: FocusArea,
) -> Paragraph<'a> {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_color(app_state, focus_area));
    
    if let Some(title_text) = title {
        block = block.title(title_text);
    }
    
    Paragraph::new(content).block(block)
}

pub fn border_color(app_state: &AppState, area: FocusArea) -> Style {
    if app_state.focus == area {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// Add a new function specifically for modal blocks
pub fn create_modal_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red))
        .title(title)
}