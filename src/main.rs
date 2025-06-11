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
use core::app_state::{AppState, FocusArea};
use core::input::handle_key_event;

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::default();

    loop {
        terminal.draw(|f| draw_ui(f, &app_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
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
                    KeyCode::Char('1') => app_state.focus = FocusArea::Header,
                    KeyCode::Char('2') => app_state.focus = FocusArea::Navigation,
                    KeyCode::Char('3') => app_state.focus = FocusArea::Preview,
                    KeyCode::Char('4') => app_state.focus = FocusArea::Input,
                    KeyCode::Char('5') => app_state.focus = FocusArea::Footer,
                    _ => {
                        if app_state.focus == FocusArea::Input {
                            handle_key_event(&mut app_state, &key);
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
