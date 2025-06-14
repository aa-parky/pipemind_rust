
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use super::app_state::AppState;

pub fn handle_key_event(app_state: &mut AppState, key: &KeyEvent) {
    match (key.code, key.modifiers) {
        // Control key combinations
        (KeyCode::Char('a'), m) if m.contains(KeyModifiers::CONTROL) => {
            // Move cursor to start of line (like in terminal)
            app_state.cursor_position = 0;
        }
        (KeyCode::Char('e'), m) if m.contains(KeyModifiers::CONTROL) => {
            // Move cursor to end of line (like in terminal)
            app_state.cursor_position = app_state.input_buffer.len();
        }
        (KeyCode::Char('u'), m) if m.contains(KeyModifiers::CONTROL) => {
            // Clear from cursor to start (like in terminal)
            let remaining = app_state.input_buffer[app_state.cursor_position..].to_string();
            app_state.input_buffer = remaining;
            app_state.cursor_position = 0;
            update_preview_and_mode(app_state);
        }
        (KeyCode::Char('k'), m) if m.contains(KeyModifiers::CONTROL) => {
            // Clear from cursor to end (like in terminal)
            app_state.input_buffer.truncate(app_state.cursor_position);
            update_preview_and_mode(app_state);
        }
        // Regular character input (no modifiers)
        (KeyCode::Char(c), m) if m.is_empty() => {
            // If it's empty and we receive '/', mark as command mode
            if app_state.input_buffer.is_empty() && c == '/' {
                app_state.is_command_mode = true;
            }

            // Insert character at cursor position
            app_state.input_buffer.insert(app_state.cursor_position, c);
            app_state.cursor_position += 1;
            update_preview(app_state);
        }
        (KeyCode::Backspace, _) => {
            // Remove character before cursor
            if app_state.cursor_position > 0 {
                app_state.cursor_position -= 1;
                app_state.input_buffer.remove(app_state.cursor_position);
                update_preview_and_mode(app_state);
            }
        }
        (KeyCode::Delete, _) => {
            // Remove character at cursor
            if app_state.cursor_position < app_state.input_buffer.len() {
                app_state.input_buffer.remove(app_state.cursor_position);
                update_preview_and_mode(app_state);
            }
        }
        (KeyCode::Left, _) => {
            // Move cursor left
            if app_state.cursor_position > 0 {
                app_state.cursor_position -= 1;
            }
        }
        (KeyCode::Right, _) => {
            // Move cursor right
            if app_state.cursor_position < app_state.input_buffer.len() {
                app_state.cursor_position += 1;
            }
        }
        (KeyCode::Home, _) => {
            // Move cursor to start
            app_state.cursor_position = 0;
        }
        (KeyCode::End, _) => {
            // Move cursor to end
            app_state.cursor_position = app_state.input_buffer.len();
        }
        (KeyCode::Enter, _) => {
            // Handle command execution or input submission
            if app_state.is_command_mode {
                let preview_content = handle_command(&app_state.input_buffer);
                app_state.update_preview(preview_content);
            }
            // Log the input before clearing
            app_state.log_output(app_state.input_buffer.clone());
            // Reset input state
            app_state.reset_input();
        }
        _ => {}
    }
}

fn update_preview(app_state: &mut AppState) {
    let content = if app_state.is_command_mode {
        format!("Command: {}", app_state.input_buffer)
    } else {
        format!("Echo: {}", app_state.input_buffer)
    };
    app_state.update_preview(content);
}

fn update_preview_and_mode(app_state: &mut AppState) {
    if app_state.input_buffer.is_empty() {
        app_state.is_command_mode = false;
    }
    update_preview(app_state);
}

fn handle_command(command: &str) -> String {
    // Strip the leading '/' and handle commands
    let cmd = command.strip_prefix('/').unwrap_or(command);
    match cmd {
        "help" => String::from("Available commands: /help, /clear"),
        "clear" => String::from(""),
        _ => format!("Unknown command: {}", cmd),
    }
}