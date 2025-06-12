#![allow(clippy::module_name_repetitions)]

// This file manages the state specific to the Image Tools feature.
// It contains data structures that hold information about the current state
// of the Image Tools functionality, such as welcome messages, image metadata,
// and any other state that needs to be tracked for this feature.


#[derive(Default)]
pub struct ImageToolsState {
    // Stores the welcome message shown when entering Image Tools mode
    pub welcome_message: String,
}

impl ImageToolsState {
    pub fn new() -> Self {
        Self {
            welcome_message: String::from(
                "Welcome to the Prompt Peekery!\n\n\
                Drop in a picture, preferably one you've birthed from the loins of a diffusion engine. \
                We'll pry it open like a suspicious pie and sniff the juicy metadata inside.\n\n\
                What you'll uncover:\n\n\
                - The _sacred prompt_ that summoned it\n\
                - The _model_ that carved its bones\n\
                - The _seed_ that sprouted its weird little face\n\
                - Sampling steps, schedulers, styles, and other sorceries\n\n\
                Perfect for when you mutter _\"how the gob did I make this?\"_\n\n\
                📷 Chuck in an image to begin the poking."
            ),
        }
    }
}