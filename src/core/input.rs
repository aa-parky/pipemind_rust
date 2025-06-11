use crossterm::event::{KeyCode, KeyEvent};
use crate::core::app_state::AppState;

pub fn handle_key_event(app_state: &mut AppState, key_event: &KeyEvent) {
    match key_event.code {
        KeyCode::Char(c) => {
            app_state.input_buffer.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_buffer.pop();
        }
        KeyCode::Enter => {
            let input = app_state.input_buffer.trim();
            if !input.is_empty() {
                if input.starts_with('/') {
                    app_state.output_log.push(format!("[cmd] {}", input));
                } else {
                    app_state.output_log.push(format!("> {}", input));
                }
            }
            app_state.input_buffer.clear();
        }
        _ => {}
    }
}
