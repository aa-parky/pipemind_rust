use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Header,
    Navigation,
    Preview,
    Input,
    Footer,
}

pub struct AppState {
    pub focus: FocusArea,
}

pub fn draw_ui(f: &mut Frame, app_state: &AppState) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Body
            Constraint::Length(3), // Footer
        ])
        .split(area);

    let header = Paragraph::new("Pipemind Console")
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(border_color(app_state, FocusArea::Header)));
    f.render_widget(header, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // Navigation
            Constraint::Min(1),     // Content
        ])
        .split(chunks[1]);

    let nav = Block::default()
        .title("Navigation")
        .borders(Borders::ALL)
        .border_style(border_color(app_state, FocusArea::Navigation));
    f.render_widget(nav, body_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Preview
            Constraint::Length(3),  // Input
        ])
        .split(body_chunks[1]);

    let preview = Block::default()
        .title("Preview")
        .borders(Borders::ALL)
        .border_style(border_color(app_state, FocusArea::Preview));
    f.render_widget(preview, content_chunks[0]);

    let input = Paragraph::new("input")
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(border_color(app_state, FocusArea::Input)));
    f.render_widget(input, content_chunks[1]);

    let footer = Paragraph::new("footer")
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(border_color(app_state, FocusArea::Footer)));
    f.render_widget(footer, chunks[2]);
}

fn border_color(app_state: &AppState, area: FocusArea) -> Style {
    if app_state.focus == area {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
