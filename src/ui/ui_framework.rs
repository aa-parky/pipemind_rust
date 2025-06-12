use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::ui::preview::render_preview;


use crate::core::app_state::{AppState, FocusArea};
use crate::ui::navigation::draw_navigation;

pub fn draw_ui(f: &mut Frame, app_state: &mut AppState) {
    let area = f.area();
    let _popup_area = centered_rect(40, 20, f.area());

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

    draw_navigation(f, app_state, body_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Preview
            Constraint::Length(3),  // Input
        ])
        .split(body_chunks[1]);

    render_preview(f, content_chunks[0], app_state);

    let input = Paragraph::new(app_state.input_buffer.as_str())
        .block(Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .border_style(border_color(app_state, FocusArea::Input)));
    f.render_widget(input, content_chunks[1]);

    let footer = Paragraph::new("footer")
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(border_color(app_state, FocusArea::Footer)));
    f.render_widget(footer, chunks[2]);

    if app_state.show_quit_modal {
        let popup_area = centered_rect(40, 20, f.area());
        let popup = Paragraph::new("Quit Pipemind? (y/n)")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Confirm Exit"));

        f.render_widget(Clear, popup_area);
        f.render_widget(popup, popup_area);
    }
}

fn border_color(app_state: &AppState, area: FocusArea) -> Style {
    if app_state.focus == area {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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