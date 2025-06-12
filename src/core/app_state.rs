use std::vec;
use super::super::features::image_tools::ImageToolsState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Header,
    Navigation,
    Preview,
    Input,
    Footer,
}

#[derive(Debug, Clone)]
pub enum NavigationState {
    Main,
    Submenu { parent_index: usize },
}

#[derive(Debug, Clone)]
pub struct NavigationItem {
    pub name: String,
    pub submenu: Option<Vec<NavigationItem>>,
}

impl NavigationItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            submenu: None,
        }
    }

    pub fn with_submenu(name: &str, submenu: Vec<NavigationItem>) -> Self {
        Self {
            name: name.to_string(),
            submenu: Some(submenu),
        }
    }
}

pub struct AppState {
    pub focus: FocusArea,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub output_log: Vec<String>,
    pub show_quit_modal: bool,
    pub selected_navigation_item: usize,
    pub preview_content: String,
    pub is_command_mode: bool,
    pub navigation_state: NavigationState,
    pub navigation_items: Vec<NavigationItem>,
    pub image_tools_state: ImageToolsState,
}

impl AppState {
    pub fn new() -> Self {
        let navigation_items = vec![
            NavigationItem::new("Home"),
            NavigationItem::with_submenu("Image Tools", vec![
                NavigationItem::new("Home"),
                NavigationItem::new("Open"),
                NavigationItem::new("Close"),
            ]),
            NavigationItem::new("Settings"),
            NavigationItem::new("Local LLMs"),
            NavigationItem::new("Help"),
            NavigationItem::new("About"),
        ];

        let mut app_state = Self {
            focus: FocusArea::Navigation,
            input_buffer: String::new(),
            cursor_position: 0,
            output_log: Vec::new(),
            show_quit_modal: false,
            selected_navigation_item: 0,
            preview_content: String::from("Welcome to Pipemind Console!"),
            is_command_mode: false,
            navigation_state: NavigationState::Main,
            navigation_items,
            image_tools_state: ImageToolsState::new(),
        };

        app_state.update_preview_based_on_navigation();
        app_state
    }

    pub fn reset_input(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
        self.is_command_mode = false;
    }

    pub fn log_output(&mut self, message: String) {
        self.output_log.push(message);
    }

    pub fn set_focus(&mut self, area: FocusArea) {
        self.focus = area;
    }

    pub fn has_focus(&self, area: FocusArea) -> bool {
        self.focus == area
    }

    pub fn update_preview(&mut self, content: String) {
        self.preview_content = content;
    }

    pub fn update_preview_based_on_navigation(&mut self) {
        match &self.navigation_state {
            NavigationState::Main => {
                match self.selected_navigation_item {
                    1 => { // Image Tools index
                        self.preview_content = self.image_tools_state.welcome_message.clone();
                    }
                    0 => { // Home
                        self.preview_content = String::from("Welcome to Pipemind Console!");
                    }
                    _ => {
                        self.preview_content = String::from("Select an option from the menu.");
                    }
                }
            }
            NavigationState::Submenu { parent_index } => {
                if *parent_index == 1 { // Image Tools
                    match self.selected_navigation_item {
                        0 => { // Home
                            self.preview_content = self.image_tools_state.welcome_message.clone();
                        }
                        1 => { // Open
                            self.preview_content = String::from("Select an image file to analyze...");
                        }
                        2 => { // Close
                            self.preview_content = String::from("Close the current image...");
                        }
                        _ => {
                            self.preview_content = String::from("Select an option from the submenu.");
                        }
                    }
                }
            }
        }
    }

    pub fn get_current_navigation_items(&self) -> Vec<String> {
        match &self.navigation_state {
            NavigationState::Main => {
                self.navigation_items.iter()
                    .map(|item| item.name.clone())
                    .collect()
            }
            NavigationState::Submenu { parent_index } => {
                if let Some(submenu) = &self.navigation_items[*parent_index].submenu {
                    submenu.iter()
                        .map(|item| item.name.clone())
                        .collect()
                } else {
                    vec![]
                }
            }
        }
    }

    pub fn get_current_navigation_count(&self) -> usize {
        match &self.navigation_state {
            NavigationState::Main => self.navigation_items.len(),
            NavigationState::Submenu { parent_index } => {
                if let Some(submenu) = &self.navigation_items[*parent_index].submenu {
                    submenu.len()
                } else {
                    0
                }
            }
        }
    }

    pub fn get_current_selection_index(&self) -> usize {
        self.selected_navigation_item
    }

    pub fn is_in_submenu(&self) -> bool {
        matches!(self.navigation_state, NavigationState::Submenu { .. })
    }

    pub fn enter_submenu(&mut self) {
        if let Some(submenu) = &self.navigation_items[self.selected_navigation_item].submenu {
            if !submenu.is_empty() {
                self.navigation_state = NavigationState::Submenu {
                    parent_index: self.selected_navigation_item,
                };
                self.selected_navigation_item = 0;
                self.update_preview_based_on_navigation();
            }
        }
    }

    pub fn exit_submenu(&mut self) {
        if let NavigationState::Submenu { parent_index } = self.navigation_state {
            self.navigation_state = NavigationState::Main;
            self.selected_navigation_item = parent_index;
            self.update_preview_based_on_navigation();
        }
    }

    pub fn select_navigation_item(&mut self, index: usize) {
        let max_index = self.get_current_navigation_count().saturating_sub(1);
        self.selected_navigation_item = index.min(max_index);
        self.update_preview_based_on_navigation();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_initialization() {
        let app_state = AppState::new();
        assert_eq!(app_state.focus, FocusArea::Navigation);
        assert!(app_state.input_buffer.is_empty());
        assert_eq!(app_state.cursor_position, 0);
        assert!(app_state.output_log.is_empty());
        assert!(!app_state.show_quit_modal);
        assert_eq!(app_state.selected_navigation_item, 0);
        assert!(!app_state.is_command_mode);
        assert!(matches!(app_state.navigation_state, NavigationState::Main));
    }

    #[test]
    fn test_navigation_state_transitions() {
        let mut app_state = AppState::new();

        // Select Image Tools (index 1)
        app_state.select_navigation_item(1);
        assert_eq!(app_state.selected_navigation_item, 1);

        // Enter submenu
        app_state.enter_submenu();
        assert!(matches!(app_state.navigation_state, NavigationState::Submenu { parent_index: 1 }));
        assert_eq!(app_state.selected_navigation_item, 0);

        // Exit submenu
        app_state.exit_submenu();
        assert!(matches!(app_state.navigation_state, NavigationState::Main));
        assert_eq!(app_state.selected_navigation_item, 1);
    }
}