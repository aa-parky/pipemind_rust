
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Clear, Paragraph, Wrap},
    Frame,
};

use crate::{
    core::app_state::{AppState, NavigationState},
    features::image_tools::views::render_image_tools_view,
    ui::{
        header::render_header,
        footer::render_footer,
        input::render_input,
        navigation::draw_navigation,
        preview::render_preview,
        utils::{centered_rect, create_modal_block},
    },
};

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

    // Render main content area based on navigation state
    match app_state.navigation_state {
        NavigationState::Main if app_state.selected_navigation_item == 1 => {
            // Index 1 is "Image Tools" in the main menu
            render_image_tools_view(f, content_chunks[0], app_state);
        }
        NavigationState::Submenu { parent_index: 1 } => {
            // We're in the Image Tools submenu
            render_image_tools_view(f, content_chunks[0], app_state);
        }
        _ => {
            // Default preview for other sections
            render_preview(f, content_chunks[0], app_state);
        }
    }

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