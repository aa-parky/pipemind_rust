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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: FocusArea::Navigation,
            input_buffer: String::new(),
            output_log: Vec::new(),
            show_quit_modal: false,
        }
    }
}
