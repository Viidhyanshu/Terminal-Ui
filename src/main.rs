pub mod app;
pub mod modules;
pub mod services;
pub mod state;
pub mod ui;
pub mod utils;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, DashboardView};
use ui::layout::render_dashboard;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state (starts on LeetCode by default)
    let mut app = App::new();

    // Main event loop
    while !app.should_quit {
        terminal.draw(|frame| {
            render_dashboard(frame, &app);
        })?;

        // Poll for user input with a short timeout
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.quit();
                        }
                        KeyCode::Tab => {
                            app.toggle_view();
                        }
                        KeyCode::Char('1') => {
                            app.set_view(DashboardView::GitHub);
                        }
                        KeyCode::Char('2') => {
                            app.set_view(DashboardView::LeetCode);
                        }
                        KeyCode::Char('3') => {
                            app.set_view(DashboardView::Spotify);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}