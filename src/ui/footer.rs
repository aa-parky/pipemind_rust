use ratatui::{
    layout::Rect,
    Frame,
};
use crate::core::app_state::{AppState, FocusArea};
use crate::ui::utils::create_bordered_paragraph;

pub fn render_footer(f: &mut Frame, area: Rect, app_state: &AppState) {
    let footer = create_bordered_paragraph(
        "footer",
        None,
        app_state,
        FocusArea::Footer,
    );

    f.render_widget(footer, area);
}