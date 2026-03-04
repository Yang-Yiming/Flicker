use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};
use crate::model::Status;
use super::state::{App, Mode, MAX_SUGGESTIONS};

pub fn draw(f: &mut Frame, app: &App) {
    match app.mode {
        Mode::Detail => draw_detail(f, app),
        Mode::Add => draw_with_input(f, app, "Add (Enter to save, Esc to cancel): "),
        Mode::Search => draw_with_input(f, app, "Search: "),
        Mode::Command => draw_with_command(f, app),
        Mode::Bash => draw_with_bash(f, app),
        Mode::Config => draw_with_config(f, app),
        Mode::List => draw_list(f, app),
    }
}

fn tabs_widget(tab: usize) -> Tabs<'static> {
    Tabs::new(vec!["Inbox", "Kept", "Archived", "All"])
        .select(tab)
        .block(Block::default().borders(Borders::ALL).title(" flicker "))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
}

fn list_items(app: &App) -> Vec<ListItem<'static>> {
    app.filtered.iter().map(|&i| {
        let f = &app.flickers[i];
        let preview: String = f.body.lines().next().unwrap_or("").chars().take(60).collect();
        let date = f.meta.created_at.format("%m/%d %H:%M").to_string();
        let dot = match f.meta.status {
            Status::Inbox => Span::styled("● ", Style::default().fg(Color::Cyan)),
            Status::Kept => Span::styled("★ ", Style::default().fg(Color::Green)),
            Status::Archived => Span::styled("○ ", Style::default().fg(Color::DarkGray)),
            Status::Deleted => Span::styled("✗ ", Style::default().fg(Color::Red)),
        };
        ListItem::new(Line::from(vec![
            dot,
            Span::styled(format!("{date}  "), Style::default().fg(Color::DarkGray)),
            Span::raw(preview),
        ]))
    }).collect()
}

fn draw_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    f.render_widget(tabs_widget(app.tab), chunks[0]);

    let items = list_items(app);
    let mut state = ListState::default();
    if !app.filtered.is_empty() { state.select(Some(app.selected)); }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunks[1], &mut state);

    f.render_widget(
        Paragraph::new("q:quit  a:add  /:search  ::cmd  !:bash  v:vim  ?:config  Enter:view  s:cycle  d:delete  Tab:filter  j/k:nav"),
        chunks[2],
    );
}

fn draw_detail(f: &mut Frame, app: &App) {
    let Some(flicker) = app.selected_flicker() else { return };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let title = format!(" {} │ {} │ {} ", flicker.meta.id, flicker.meta.status,
        flicker.meta.created_at.format("%Y-%m-%d %H:%M"));
    f.render_widget(Paragraph::new("").block(Block::default().borders(Borders::ALL).title(title)), chunks[0]);
    f.render_widget(
        Paragraph::new(flicker.body.clone())
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false }),
        chunks[1],
    );
    f.render_widget(Paragraph::new("Esc:back  s:cycle-status  d:delete"), chunks[2]);
}

fn draw_with_input(f: &mut Frame, app: &App, prompt: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    f.render_widget(tabs_widget(app.tab), chunks[0]);

    let items = list_items(app);
    let mut state = ListState::default();
    if !app.filtered.is_empty() { state.select(Some(app.selected)); }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list, chunks[1], &mut state);

    let input = match app.mode {
        Mode::Search => app.search_query.as_str(),
        Mode::Add => app.add_input.as_str(),
        _ => "",
    };
    f.render_widget(
        Paragraph::new(format!("{prompt}{input}"))
            .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}

fn draw_with_command(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    f.render_widget(tabs_widget(app.tab), chunks[0]);

    let items = list_items(app);
    let mut state = ListState::default();
    if !app.filtered.is_empty() { state.select(Some(app.selected)); }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list, chunks[1], &mut state);

    // Suggestions popup — floats just above the command bar
    if !app.suggestions.is_empty() {
        let show = MAX_SUGGESTIONS.min(app.suggestions.len()) as u16;
        let popup_h = show + 2;
        let popup_w = 20u16;
        let popup_y = chunks[2].y.saturating_sub(popup_h);
        let popup_rect = Rect::new(chunks[1].x + 1, popup_y, popup_w, popup_h);

        let suggestion_items: Vec<ListItem> = app.suggestions.iter()
            .map(|cmd| ListItem::new(format!(" {cmd}")))
            .collect();
        let mut sstate = ListState::default();
        sstate.select(app.suggestion_idx);

        f.render_widget(Clear, popup_rect);
        f.render_stateful_widget(
            List::new(suggestion_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                .highlight_symbol(">"),
            popup_rect,
            &mut sstate,
        );
    }

    // Command input bar
    let display = if let Some(ref msg) = app.status_message {
        format!(" {msg}")
    } else {
        format!(":{}", app.command_input)
    };
    f.render_widget(
        Paragraph::new(display)
            .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}

fn draw_with_bash(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    f.render_widget(tabs_widget(app.tab), chunks[0]);

    let items = list_items(app);
    let mut state = ListState::default();
    if !app.filtered.is_empty() { state.select(Some(app.selected)); }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list, chunks[1], &mut state);

    f.render_widget(
        Paragraph::new(format!("!{}", app.bash_input))
            .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}

fn draw_with_config(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    f.render_widget(tabs_widget(app.tab), chunks[0]);
    let items = list_items(app);
    let mut state = ListState::default();
    if !app.filtered.is_empty() { state.select(Some(app.selected)); }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list, chunks[1], &mut state);
    f.render_widget(Paragraph::new(""), chunks[2]);

    let popup_w = 60;
    let popup_h = 8;
    let popup_x = (f.area().width.saturating_sub(popup_w)) / 2;
    let popup_y = (f.area().height.saturating_sub(popup_h)) / 2;
    let popup_rect = Rect::new(popup_x, popup_y, popup_w, popup_h);

    let cfg = crate::config::load();
    let editor_val = if app.config_editing.is_some() && app.config_selected == 0 {
        app.config_editing.as_ref().unwrap().as_str()
    } else {
        &cfg.editor
    };
    let shell_val = if app.config_editing.is_some() && app.config_selected == 1 {
        app.config_editing.as_ref().unwrap().as_str()
    } else {
        &cfg.shell
    };

    let config_items = vec![
        ListItem::new(format!(" editor: {}", editor_val)),
        ListItem::new(format!(" shell:  {}", shell_val)),
    ];

    let mut cfg_state = ListState::default();
    cfg_state.select(Some(app.config_selected));

    let title = if app.config_editing.is_some() {
        " Config (Enter:save Esc:cancel) "
    } else {
        " Config (Enter:edit Esc:close ↑↓:navigate) "
    };

    f.render_widget(Clear, popup_rect);
    f.render_stateful_widget(
        List::new(config_items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
            .highlight_symbol("> "),
        popup_rect,
        &mut cfg_state,
    );
}
