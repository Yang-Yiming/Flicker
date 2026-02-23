use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::storage;

mod state;
mod ui;
use state::{App, Mode};

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new(storage::read_all());
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        let Event::Key(key) = event::read()? else { continue };

        // Ctrl+C quits from anywhere
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            break;
        }

        match app.mode {
            Mode::List => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('a') => app.mode = Mode::Add,
                KeyCode::Char('/') => app.mode = Mode::Search,
                KeyCode::Char('d') => app.delete_selected(),
                KeyCode::Char('s') => app.cycle_status_selected(),
                KeyCode::Tab => app.cycle_tab(),
                KeyCode::Enter => {
                    if app.selected_flicker().is_some() {
                        app.mode = Mode::Detail;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                _ => {}
            },
            Mode::Detail => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => app.mode = Mode::List,
                KeyCode::Char('d') => { app.delete_selected(); app.mode = Mode::List; }
                KeyCode::Char('s') => app.cycle_status_selected(),
                _ => {}
            },
            Mode::Search => match key.code {
                KeyCode::Esc => {
                    app.search_query.clear();
                    app.refilter();
                    app.mode = Mode::List;
                }
                KeyCode::Enter => app.mode = Mode::List,
                KeyCode::Backspace => { app.search_query.pop(); app.refilter(); }
                KeyCode::Char(c) => { app.search_query.push(c); app.refilter(); }
                _ => {}
            },
            Mode::Add => match key.code {
                KeyCode::Esc => { app.add_input.clear(); app.mode = Mode::List; }
                KeyCode::Enter => {
                    if !app.add_input.is_empty() {
                        let text = std::mem::take(&mut app.add_input);
                        app.add_flicker(text);
                    }
                    app.mode = Mode::List;
                }
                KeyCode::Backspace => { app.add_input.pop(); }
                KeyCode::Char(c) => app.add_input.push(c),
                _ => {}
            },
        }
    }
    Ok(())
}
