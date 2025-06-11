use std::io;
use crossterm::{event::{self, Event, KeyCode}, terminal, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal};

mod ui;
use ui::ui_framework::{draw_ui, AppState, FocusArea};

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState { focus: FocusArea::Navigation };

    loop {
        terminal.draw(|f| draw_ui(f, &app_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => app_state.focus = FocusArea::Header,
                    KeyCode::Char('2') => app_state.focus = FocusArea::Navigation,
                    KeyCode::Char('3') => app_state.focus = FocusArea::Preview,
                    KeyCode::Char('4') => app_state.focus = FocusArea::Input,
                    KeyCode::Char('5') => app_state.focus = FocusArea::Footer,
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
