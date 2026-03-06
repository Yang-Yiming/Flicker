use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use flicker_core::storage;

mod state;
mod ui;
use state::{App, Mode};

pub fn run(commands: Vec<String>) -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let config = flicker_core::config::load();
    let result = run_app(&mut terminal, commands, &config);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn suspend_tui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn resume_tui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

fn run_bash(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, cmd: &str, shell: &str) -> io::Result<()> {
    suspend_tui(terminal)?;
    println!();
    let _ = std::process::Command::new(shell).arg("-c").arg(cmd).status();
    println!("\n[Press Enter to return]");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    resume_tui(terminal)
}

fn open_in_vim(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, path: &str, editor: &str) -> io::Result<()> {
    suspend_tui(terminal)?;
    let _ = std::process::Command::new(editor).arg(path).status();
    resume_tui(terminal)
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, commands: Vec<String>, config: &flicker_core::Config) -> io::Result<()> {
    let mut app = App::new(storage::read_all(), commands);
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
                KeyCode::Char(':') => app.enter_command(),
                KeyCode::Char('!') => { app.bash_input.clear(); app.mode = Mode::Bash; }
                KeyCode::Char('?') => {
                    app.config_selected = 0;
                    app.config_editing = None;
                    app.mode = Mode::Config;
                }
                KeyCode::Char('v') => {
                    if let Some(flicker) = app.selected_flicker() {
                        let path = storage::flickers_dir()
                            .join(format!("{}.md", flicker.meta.id))
                            .to_string_lossy()
                            .to_string();
                        open_in_vim(terminal, &path, &config.editor)?;
                        app.reload();
                    }
                }
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
                KeyCode::Char(':') => app.enter_command(),
                KeyCode::Char('!') => { app.bash_input.clear(); app.mode = Mode::Bash; }
                KeyCode::Char('?') => {
                    app.config_selected = 0;
                    app.config_editing = None;
                    app.mode = Mode::Config;
                }
                KeyCode::Char('v') => {
                    if let Some(flicker) = app.selected_flicker() {
                        let path = storage::flickers_dir()
                            .join(format!("{}.md", flicker.meta.id))
                            .to_string_lossy()
                            .to_string();
                        open_in_vim(terminal, &path, &config.editor)?;
                        app.reload();
                    }
                }
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
            Mode::Command => match key.code {
                KeyCode::Esc => app.exit_command(),
                KeyCode::Backspace => { app.command_input.pop(); app.update_suggestions(); }
                KeyCode::Tab | KeyCode::Down => app.suggestion_next(),
                KeyCode::Up => app.suggestion_prev(),
                KeyCode::Enter => {
                    if app.suggestion_idx.is_some() && !app.command_input.contains(' ') {
                        app.accept_suggestion();
                        if !app.command_input.ends_with(' ') {
                            dispatch_command(&mut app);
                            if app.mode == Mode::Command { app.exit_command(); }
                        }
                    } else {
                        dispatch_command(&mut app);
                        if app.mode == Mode::Command { app.exit_command(); }
                    }
                }
                KeyCode::Char(c) => { app.command_input.push(c); app.update_suggestions(); }
                _ => {}
            },
            Mode::Bash => match key.code {
                KeyCode::Esc => { app.bash_input.clear(); app.mode = Mode::List; }
                KeyCode::Backspace => { app.bash_input.pop(); }
                KeyCode::Enter => {
                    if !app.bash_input.is_empty() {
                        let cmd = std::mem::take(&mut app.bash_input);
                        app.mode = Mode::List;
                        run_bash(terminal, &cmd, &config.shell)?;
                    } else {
                        app.mode = Mode::List;
                    }
                }
                KeyCode::Char(c) => app.bash_input.push(c),
                _ => {}
            },
            Mode::Config => match key.code {
                KeyCode::Tab => {
                    if app.config_editing.is_none() {
                        app.config_tab = (app.config_tab + 1) % 3;
                        app.config_selected = 0;
                    }
                }
                KeyCode::Esc => {
                    if app.config_editing.is_some() {
                        app.config_editing = None;
                        app.config_storage_focus = 0;
                    } else {
                        app.mode = Mode::List;
                        app.config_tab = 0;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') if app.config_editing.is_none() => {
                    if app.config_selected > 0 {
                        app.config_selected -= 1;
                    }
                }
                KeyCode::Up => {
                    if app.config_editing.is_some() && app.config_tab == 1 {
                        if app.config_storage_focus > 0 {
                            app.config_storage_focus -= 1;
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') if app.config_editing.is_none() => {
                    let max = match app.config_tab {
                        0 => 1,  // General: editor, shell
                        2 => 2,  // Supabase: url, key, sync
                        _ => 0,  // Storage: path
                    };
                    if app.config_selected < max {
                        app.config_selected += 1;
                    }
                }
                KeyCode::Down => {
                    if app.config_editing.is_some() && app.config_tab == 1 {
                        if app.config_storage_focus < 1 {
                            app.config_storage_focus += 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    if app.config_tab == 2 {
                        // Supabase tab
                        if let Some(new_val) = app.config_editing.take() {
                            let mut cfg = flicker_core::config::load();
                            if app.config_selected == 0 {
                                cfg.supabase_url = Some(new_val);
                            } else {
                                cfg.supabase_anon_key = Some(new_val);
                            }
                            if let Err(e) = flicker_core::config::save(&cfg) {
                                app.status_message = Some(format!("Save failed: {}", e));
                            }
                        } else if app.config_selected == 2 {
                            // Sync action
                            match flicker_core::SyncClient::from_config() {
                                Some(client) => {
                                    app.sync_status = Some("syncing...".to_string());
                                    match client.sync() {
                                        Ok((pulled, pushed)) => {
                                            app.sync_status = Some(format!("pulled {pulled}, pushed {pushed}"));
                                            app.reload();
                                        }
                                        Err(e) => {
                                            app.sync_status = Some(format!("error: {e}"));
                                        }
                                    }
                                }
                                None => {
                                    app.sync_status = Some("not configured".to_string());
                                }
                            }
                        } else {
                            let cfg = flicker_core::config::load();
                            let current = if app.config_selected == 0 {
                                cfg.supabase_url.unwrap_or_default()
                            } else {
                                cfg.supabase_anon_key.unwrap_or_default()
                            };
                            app.config_editing = Some(current);
                        }
                    } else if let Some(new_val) = app.config_editing.take() {
                        let mut cfg = flicker_core::config::load();
                        if app.config_tab == 0 {
                            if app.config_selected == 0 {
                                cfg.editor = new_val;
                            } else {
                                cfg.shell = new_val;
                            }
                        } else {
                            if app.config_storage_focus == 0 {
                                if new_val.is_empty() {
                                    app.status_message = Some("storage_path cannot be empty".to_string());
                                    app.config_editing = Some(new_val);
                                    return Ok(());
                                }
                                if new_val.starts_with("~/") || new_val.starts_with('/') {
                                    let home = std::env::var("HOME").unwrap_or_default();
                                    let expanded = new_val.replacen("~", &home, 1);
                                    cfg.storage_path = Some(expanded);
                                } else {
                                    app.status_message = Some("Path must start with '~/' or be absolute".to_string());
                                    app.config_editing = Some(new_val);
                                    return Ok(());
                                }
                            }
                        }
                        if let Err(e) = flicker_core::config::save(&cfg) {
                            app.status_message = Some(format!("Save failed: {}", e));
                        }
                        app.config_storage_focus = 0;
                    } else {
                        let cfg = flicker_core::config::load();
                        let current = if app.config_tab == 0 {
                            if app.config_selected == 0 {
                                cfg.editor
                            } else {
                                cfg.shell
                            }
                        } else {
                            cfg.storage_path.unwrap_or_default()
                        };
                        app.config_editing = Some(current);
                        app.config_storage_focus = 0;
                    }
                }
                KeyCode::Backspace => {
                    if let Some(ref mut s) = app.config_editing {
                        if app.config_tab == 1 && app.config_storage_focus == 1 {
                            // On button, ignore
                        } else {
                            s.pop();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(ref mut s) = app.config_editing {
                        if app.config_tab == 1 && app.config_storage_focus == 1 {
                            // On button, ignore
                        } else {
                            s.push(c);
                        }
                    }
                }
                _ => {}
            },
        }
    }
    Ok(())
}

fn dispatch_command(app: &mut App) {
    let input = app.command_input.trim().to_string();
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts[0] {
        "add" => {
            let text = parts.get(1).unwrap_or(&"").trim().to_string();
            if !text.is_empty() {
                app.add_flicker(text);
                app.status_message = None;
            } else {
                app.status_message = Some("usage: add <text>".to_string());
            }
        }
        "delete" => {
            app.delete_selected();
        }
        "search" => {
            let query = parts.get(1).unwrap_or(&"").trim().to_string();
            app.search_query = query;
            app.refilter();
            app.mode = Mode::Search;
            return;
        }
        "" => {}
        _ => {
            app.status_message = Some(format!("unknown command: {}", parts[0]));
        }
    }
    app.refilter();
}
