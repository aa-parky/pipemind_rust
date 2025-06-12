use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::core::app_state::{AppState, FocusArea};

pub fn draw_navigation(f: &mut Frame, app_state: &mut AppState, area: Rect) {
    // Get current navigation items based on state
    let navigation_items = app_state.get_current_navigation_items();

    let items: Vec<ListItem> = navigation_items
        .iter()
        .map(|item| ListItem::new(item.as_str()))
        .collect();

    let mut list_state = ListState::default();
    if app_state.has_focus(FocusArea::Navigation) {
        list_state.select(Some(app_state.get_current_selection_index()));
    }

    // Set title based on navigation state
    let title = if app_state.is_in_submenu() {
        if let crate::core::app_state::NavigationState::Submenu { parent_index } = &app_state.navigation_state {
            format!("Navigation - {}", app_state.navigation_items[*parent_index].name)
        } else {
            "Navigation".to_string()
        }
    } else {
        "Navigation".to_string()
    };

    let nav_list = List::new(items)
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if app_state.has_focus(FocusArea::Navigation) {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            }))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(nav_list, area, &mut list_state);
}


