use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod ui;
mod core;

use ui::ui_framework::draw_ui;
use core::app_state::{AppState, FocusArea, NAVIGATION_ITEMS_COUNT};
use core::input::handle_key_event;

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::default();

    loop {
        terminal.draw(|f| draw_ui(f, &mut app_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle global keybindings first
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app_state.show_quit_modal = true;
                    }
                    KeyCode::Char('y') if app_state.show_quit_modal => {
                        break;
                    }
                    KeyCode::Char('n') if app_state.show_quit_modal => {
                        app_state.show_quit_modal = false;
                    }
                    KeyCode::F(n) => {
                        match n {
                            1 => app_state.focus = FocusArea::Header,
                            2 => app_state.focus = FocusArea::Navigation,
                            3 => app_state.focus = FocusArea::Preview,
                            4 => app_state.focus = FocusArea::Input,
                            5 => app_state.focus = FocusArea::Footer,
                            _ => {}
                        }
                    }
                    _ => {
                        // Handle keybindings specific to focus area
                        if app_state.focus == FocusArea::Input {
                            // If input area is focused, all character keys should be handled as input
                            handle_key_event(&mut app_state, &key);
                        } else {
                            // Handle hjkl for navigation outside input area
                            match key.code {
                                KeyCode::Char('h') => {
                                    app_state.focus = match app_state.focus {
                                        FocusArea::Header => FocusArea::Footer,
                                        FocusArea::Navigation => FocusArea::Header,
                                        FocusArea::Preview => FocusArea::Navigation,
                                        FocusArea::Footer => FocusArea::Preview,
                                        _ => app_state.focus,
                                    };
                                }
                                KeyCode::Char('l') => {
                                    app_state.focus = match app_state.focus {
                                        FocusArea::Header => FocusArea::Navigation,
                                        FocusArea::Navigation => FocusArea::Preview,
                                        FocusArea::Preview => FocusArea::Footer,
                                        FocusArea::Footer => FocusArea::Header,
                                        _ => app_state.focus,
                                    };
                                }
                                KeyCode::Char('j') => {
                                    if app_state.focus == FocusArea::Navigation {
                                        app_state.selected_navigation_item = 
                                            (app_state.selected_navigation_item + 1).min(NAVIGATION_ITEMS_COUNT - 1);
                                    } else {
                                        app_state.focus = match app_state.focus {
                                            FocusArea::Header => FocusArea::Navigation,
                                            FocusArea::Navigation => FocusArea::Preview,
                                            FocusArea::Preview => FocusArea::Footer,
                                            FocusArea::Footer => FocusArea::Header,
                                            _ => app_state.focus,
                                        };
                                    }
                                }
                                KeyCode::Char('k') => {
                                    if app_state.focus == FocusArea::Navigation {
                                        app_state.selected_navigation_item = 
                                            app_state.selected_navigation_item.saturating_sub(1);
                                    } else {
                                        app_state.focus = match app_state.focus {
                                            FocusArea::Header => FocusArea::Footer,
                                            FocusArea::Navigation => FocusArea::Header,
                                            FocusArea::Preview => FocusArea::Navigation,
                                            FocusArea::Footer => FocusArea::Preview,
                                            _ => app_state.focus,
                                        };
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}