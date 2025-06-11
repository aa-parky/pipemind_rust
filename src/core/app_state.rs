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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: FocusArea::Navigation,
            input_buffer: String::new(),
            output_log: Vec::new(),
            show_quit_modal: false,
            selected_navigation_item: 0,
        }
    }
}


