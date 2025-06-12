use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Clear, Paragraph, Wrap},
    Frame,
};

use crate::core::app_state::AppState;
use crate::ui::header::render_header;
use crate::ui::footer::render_footer;
use crate::ui::input::render_input;
use crate::ui::navigation::draw_navigation;
use crate::ui::preview::render_preview;
use crate::ui::utils::{centered_rect, create_modal_block};

pub fn draw_ui(f: &mut Frame, app_state: &mut AppState) {
    let area = f.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(1),     // Body
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Render header
    render_header(f, chunks[0], app_state);

    // Body layout
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // Navigation
            Constraint::Min(1),     // Content
        ])
        .split(chunks[1]);

    // Render navigation
    draw_navigation(f, app_state, body_chunks[0]);

    // Content layout
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Preview
            Constraint::Length(3),  // Input
        ])
        .split(body_chunks[1]);

    // Render preview
    render_preview(f, content_chunks[0], app_state);

    // Render input with cursor support
    render_input(f, content_chunks[1], app_state);

    // Render footer
    render_footer(f, chunks[2], app_state);

    // Render quit modal if active
    if app_state.show_quit_modal {
        let popup_area = centered_rect(40, 20, f.area());
        let popup = Paragraph::new("Quit Pipemind? (y/n)")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(create_modal_block("Confirm Exit"));

        f.render_widget(Clear, popup_area);
        f.render_widget(popup, popup_area);
    }
}