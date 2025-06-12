use ratatui::{
    layout::Rect,
    Frame,
};
use crate::core::app_state::{AppState, FocusArea};
use crate::ui::utils::create_bordered_paragraph;

pub fn render_header(f: &mut Frame, area: Rect, app_state: &AppState) {
    let header = create_bordered_paragraph(
        "Pipemind Console",
        None,
        app_state,
        FocusArea::Header,
    );

    f.render_widget(header, area);
}