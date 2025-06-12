use crossterm::event::{KeyEvent, KeyCode};
use super::app_state::AppState;

pub fn handle_key_event(app_state: &mut AppState, key: &KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            // If it's empty and we receive '/', mark as command mode
            if app_state.input_buffer.is_empty() && c == '/' {
                app_state.is_command_mode = true;
            }
            app_state.input_buffer.push(c);
            // Update preview immediately
            app_state.preview_content = if app_state.is_command_mode {
                format!("Command: {}", app_state.input_buffer)
            } else {
                format!("Echo: {}", app_state.input_buffer)
            };
        }
        KeyCode::Backspace => {
            if let Some(_) = app_state.input_buffer.pop() {
                if app_state.input_buffer.is_empty() {
                    app_state.is_command_mode = false;
                }
                // Update preview after backspace
                app_state.preview_content = if app_state.is_command_mode {
                    format!("Command: {}", app_state.input_buffer)
                } else {
                    format!("Echo: {}", app_state.input_buffer)
                };
            }
        }
        KeyCode::Enter => {
            // Handle command execution or input submission
            if app_state.is_command_mode {
                handle_command(&mut app_state.preview_content, &app_state.input_buffer);
            }
            // Store in output log
            app_state.output_log.push(app_state.input_buffer.clone());
            // Clear input after processing
            app_state.input_buffer.clear();
            app_state.is_command_mode = false;
        }
        _ => {}
    }
}

fn handle_command(preview: &mut String, command: &str) {
    // Strip the leading '/' and handle commands
    let cmd = command.strip_prefix('/').unwrap_or(command);
    *preview = match cmd {
        "help" => String::from("Available commands: /help, /clear"),
        "clear" => String::from(""),
        _ => format!("Unknown command: {}", cmd),
    };
}