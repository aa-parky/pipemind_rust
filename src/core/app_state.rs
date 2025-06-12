
/// Represents the different areas of the UI that can have focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Header,
    Navigation,
    Preview,
    Input,
    Footer,
}

/// The number of items in the navigation menu
pub const NAVIGATION_ITEMS_COUNT: usize = 6;

/// Represents the current state of the application
pub struct AppState {
    /// Currently focused UI area
    pub focus: FocusArea,

    /// Content of the input field
    pub input_buffer: String,

    /// Current cursor position in the input buffer
    pub cursor_position: usize,

    /// History of output messages
    pub output_log: Vec<String>,

    /// Whether the quit confirmation modal is visible
    pub show_quit_modal: bool,

    /// Currently selected item in the navigation menu (0-based index)
    pub selected_navigation_item: usize,

    /// Content to be shown in the preview area
    pub preview_content: String,

    /// Whether the input is in command mode (starts with '/')
    pub is_command_mode: bool,
}

impl AppState {
    /// Creates a new AppState with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the input state
    pub fn reset_input(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
        self.is_command_mode = false;
    }

    /// Adds a message to the output log
    pub fn log_output(&mut self, message: String) {
        self.output_log.push(message);
    }

    /// Sets the focus to a specific area
    pub fn set_focus(&mut self, area: FocusArea) {
        self.focus = area;
    }

    /// Checks if a specific area has focus
    pub fn has_focus(&self, area: FocusArea) -> bool {
        self.focus == area
    }

    /// Updates the preview content
    pub fn update_preview(&mut self, content: String) {
        self.preview_content = content;
    }

    /// Selects a navigation item by index
    pub fn select_navigation_item(&mut self, index: usize) {
        self.selected_navigation_item = index.min(NAVIGATION_ITEMS_COUNT - 1);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: FocusArea::Navigation,
            input_buffer: String::new(),
            cursor_position: 0,
            output_log: Vec::new(),
            show_quit_modal: false,
            selected_navigation_item: 0,
            preview_content: String::new(),
            is_command_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app_state() {
        let state = AppState::new();
        assert_eq!(state.focus, FocusArea::Navigation);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.cursor_position, 0);
        assert!(state.output_log.is_empty());
        assert!(!state.show_quit_modal);
        assert_eq!(state.selected_navigation_item, 0);
        assert!(state.preview_content.is_empty());
        assert!(!state.is_command_mode);
    }

    #[test]
    fn test_reset_input() {
        let mut state = AppState::new();
        state.input_buffer = "test".to_string();
        state.cursor_position = 2;
        state.is_command_mode = true;

        state.reset_input();

        assert!(state.input_buffer.is_empty());
        assert_eq!(state.cursor_position, 0);
        assert!(!state.is_command_mode);
    }

    #[test]
    fn test_log_output() {
        let mut state = AppState::new();
        state.log_output("test message".to_string());

        assert_eq!(state.output_log.len(), 1);
        assert_eq!(state.output_log[0], "test message");
    }

    #[test]
    fn test_focus_operations() {
        let mut state = AppState::new();

        state.set_focus(FocusArea::Input);
        assert!(state.has_focus(FocusArea::Input));
        assert!(!state.has_focus(FocusArea::Navigation));
    }

    #[test]
    fn test_navigation_selection() {
        let mut state = AppState::new();

        state.select_navigation_item(2);
        assert_eq!(state.selected_navigation_item, 2);

        // Test bounds checking
        state.select_navigation_item(NAVIGATION_ITEMS_COUNT + 1);
        assert_eq!(state.selected_navigation_item, NAVIGATION_ITEMS_COUNT - 1);
    }
}