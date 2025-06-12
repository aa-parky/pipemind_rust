#![allow(clippy::module_name_repetitions)]

// This file contains the business logic for the Image Tools feature.
// It handles events, processes commands, and manages interactions
// between the UI and the state. Think of it as the controller
// in an MVC pattern.

use super::state::ImageToolsState;

#[allow(dead_code)] // Can be removed once `state` is actively used
pub struct ImageToolsHandler {
    pub state: ImageToolsState,
}

impl ImageToolsHandler {
    #[allow(dead_code)] // Remove this once we call `new()` somewhere
    pub fn new() -> Self {
        Self {
            state: ImageToolsState::new(),
        }
    }

    #[allow(dead_code)] // Remove this once it’s used in the UI or a test
    pub fn get_welcome_message(&self) -> &str {
        &self.state.welcome_message
    }
}