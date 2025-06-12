#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Header,
    Navigation,
    Preview,
    Input,
    Footer,
}

pub struct AppState {
    pub focus: FocusArea,
    pub input_buffer: String,
    pub output_log: Vec<String>,
    pub show_quit_modal: bool,
    pub selected_navigation_item: usize,
    pub preview_content: String,
    pub is_command_mode: bool,
}

pub const NAVIGATION_ITEMS_COUNT: usize = 5;  // or however many items you have

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: FocusArea::Navigation,
            input_buffer: String::new(),
            output_log: Vec::new(),
            show_quit_modal: false,
            selected_navigation_item: 0,
            preview_content: String::new(),
            is_command_mode: false,
        }
    }
}