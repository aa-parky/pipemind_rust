
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use pipemind_rust::{
    core::{
        app_state::{AppState, FocusArea},
        input::handle_key_event,
    },
    ui::ui_framework::draw_ui,
};

/// Handles the terminal setup and cleanup
struct TerminalManager {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalManager {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    fn run(&mut self, app_state: &mut AppState) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| draw_ui(f, app_state))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_key_event(app_state, key) {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&self, app_state: &mut AppState, key: event::KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app_state.show_quit_modal = true;
                false
            }
            KeyCode::Char('y') if app_state.show_quit_modal => true,
            KeyCode::Char('n') if app_state.show_quit_modal => {
                app_state.show_quit_modal = false;
                false
            }
            KeyCode::F(n) => {
                let focus = match n {
                    1 => Some(FocusArea::Header),
                    2 => Some(FocusArea::Navigation),
                    3 => Some(FocusArea::Preview),
                    4 => Some(FocusArea::Input),
                    5 => Some(FocusArea::Footer),
                    _ => None,
                };
                if let Some(area) = focus {
                    app_state.set_focus(area);
                }
                false
            }
            _ => {
                if app_state.has_focus(FocusArea::Input) {
                    handle_key_event(app_state, &key);
                } else {
                    self.handle_navigation(app_state, key);
                }
                false
            }
        }
    }

    fn handle_navigation(&self, app_state: &mut AppState, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('h') => {
                let new_focus = match app_state.focus {
                    FocusArea::Navigation => FocusArea::Header,
                    FocusArea::Preview => FocusArea::Navigation,
                    FocusArea::Footer => FocusArea::Preview,
                    FocusArea::Header => FocusArea::Footer,
                    FocusArea::Input => app_state.focus,
                };
                app_state.set_focus(new_focus);
            }
            KeyCode::Char('l') => {
                let new_focus = match app_state.focus {
                    FocusArea::Header => FocusArea::Navigation,
                    FocusArea::Navigation => FocusArea::Preview,
                    FocusArea::Preview => FocusArea::Footer,
                    FocusArea::Footer => FocusArea::Header,
                    FocusArea::Input => app_state.focus,
                };
                app_state.set_focus(new_focus);
            }
            KeyCode::Char('j') => {
                if app_state.has_focus(FocusArea::Navigation) {
                    let current_count = app_state.get_current_navigation_count();
                    let current_index = app_state.get_current_selection_index();
                    let next_index = if current_index + 1 < current_count {
                        current_index + 1
                    } else {
                        current_index
                    };
                    app_state.select_navigation_item(next_index);
                } else {
                    let new_focus = match app_state.focus {
                        FocusArea::Header => FocusArea::Navigation,
                        FocusArea::Navigation => FocusArea::Preview,
                        FocusArea::Preview => FocusArea::Footer,
                        FocusArea::Footer => FocusArea::Header,
                        FocusArea::Input => app_state.focus,
                    };
                    app_state.set_focus(new_focus);
                }
            }
            KeyCode::Char('k') => {
                if app_state.has_focus(FocusArea::Navigation) {
                    let current_index = app_state.get_current_selection_index();
                    if current_index > 0 {
                        app_state.select_navigation_item(current_index - 1);
                    }
                } else {
                    let new_focus = match app_state.focus {
                        FocusArea::Header => FocusArea::Footer,
                        FocusArea::Navigation => FocusArea::Header,
                        FocusArea::Preview => FocusArea::Navigation,
                        FocusArea::Footer => FocusArea::Preview,
                        FocusArea::Input => app_state.focus,
                    };
                    app_state.set_focus(new_focus);
                }
            }
            KeyCode::Enter => {
                if app_state.has_focus(FocusArea::Navigation) {
                    if app_state.is_in_submenu() {
                        let current_index = app_state.get_current_selection_index();
                        if current_index == 0 {  // If "Home" is selected in submenu
                            app_state.exit_submenu();
                        }
                        // Add handling for other submenu items here if needed
                    } else {
                        app_state.enter_submenu();
                    }
                }
            }
            KeyCode::Esc => {
                if app_state.has_focus(FocusArea::Navigation) {
                    app_state.exit_submenu();
                }
            }
            _ => {}
        }
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        self.terminal
            .backend_mut()
            .execute(terminal::LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}

fn main() -> io::Result<()> {
    let mut terminal_manager = TerminalManager::new()?;
    let mut app_state = AppState::new();
    terminal_manager.run(&mut app_state)
}