use ratatui::{
    layout::Rect,
    Frame,
};
use crate::core::app_state::{AppState, FocusArea};
use crate::ui::utils::create_bordered_paragraph;

pub fn render_preview(f: &mut Frame, area: Rect, app_state: &AppState) {
    let preview_text = create_bordered_paragraph(
        app_state.preview_content.as_str(),
        Some("Preview"),
        app_state,
        FocusArea::Preview
    );

    f.render_widget(preview_text, area);
}