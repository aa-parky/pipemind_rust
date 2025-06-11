use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::core::app_state::{AppState, FocusArea};

pub fn draw_navigation(f: &mut Frame, app_state: &mut AppState, area: Rect) {
    let navigation_items = [
        "Home",
        "Settings",
        "Local LLMs",
        "About",
        "Help",
    ];

    let items: Vec<ListItem> = navigation_items
        .iter()
        .map(|i| ListItem::new(*i))
        .collect();

    let mut list_state = ListState::default();
    if app_state.focus == FocusArea::Navigation {
        list_state.select(Some(app_state.selected_navigation_item));
    }

    let nav_list = List::new(items)
        .block(Block::default()
            .title("Navigation")
            .borders(Borders::ALL)
            .border_style(if app_state.focus == FocusArea::Navigation {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            }))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(nav_list, area, &mut list_state);
}


