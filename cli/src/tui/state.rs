use chrono::Utc;
use uuid::Uuid;
use crate::model::{Flicker, Frontmatter, Status};

pub const COMMANDS: &[&str] = &["add", "delete", "search"];
pub const MAX_SUGGESTIONS: usize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    List,
    Detail,
    Search,
    Add,
    Command,
}

pub struct App {
    pub mode: Mode,
    pub flickers: Vec<Flicker>,
    pub filtered: Vec<usize>,
    pub selected: usize,
    pub tab: usize, // 0=inbox 1=kept 2=archived 3=all
    pub search_query: String,
    pub add_input: String,
    pub command_input: String,
    pub prev_mode: Mode,
    pub status_message: Option<String>,
    pub suggestions: Vec<&'static str>,
    pub suggestion_idx: Option<usize>,
}

impl App {
    pub fn new(flickers: Vec<Flicker>) -> Self {
        let mut app = App {
            mode: Mode::List,
            flickers,
            filtered: vec![],
            selected: 0,
            tab: 0,
            search_query: String::new(),
            add_input: String::new(),
            command_input: String::new(),
            prev_mode: Mode::List,
            status_message: None,
            suggestions: vec![],
            suggestion_idx: None,
        };
        app.refilter();
        app
    }

    pub fn enter_command(&mut self) {
        self.prev_mode = self.mode;
        self.command_input.clear();
        self.status_message = None;
        self.mode = Mode::Command;
        self.update_suggestions();
    }

    pub fn exit_command(&mut self) {
        self.mode = self.prev_mode;
        self.command_input.clear();
        self.suggestions.clear();
        self.suggestion_idx = None;
    }

    pub fn update_suggestions(&mut self) {
        let prefix = self.command_input.trim().to_lowercase();
        // Only filter on the first word (command name), not args
        let cmd_part: &str = prefix.splitn(2, ' ').next().unwrap_or("");
        self.suggestions = if prefix.contains(' ') {
            // User is typing args — no suggestions
            vec![]
        } else {
            COMMANDS.iter()
                .copied()
                .filter(|&c| c.starts_with(cmd_part))
                .collect()
        };
        // Reset idx if out of bounds
        if let Some(idx) = self.suggestion_idx {
            if idx >= self.suggestions.len() {
                self.suggestion_idx = None;
            }
        }
    }

    pub fn suggestion_next(&mut self) {
        if self.suggestions.is_empty() { return; }
        self.suggestion_idx = Some(match self.suggestion_idx {
            None => 0,
            Some(i) => (i + 1) % self.suggestions.len(),
        });
    }

    pub fn suggestion_prev(&mut self) {
        if self.suggestions.is_empty() { return; }
        self.suggestion_idx = Some(match self.suggestion_idx {
            None => self.suggestions.len() - 1,
            Some(0) => self.suggestions.len() - 1,
            Some(i) => i - 1,
        });
    }

    /// Fill the highlighted suggestion into command_input (adds trailing space for commands that take args).
    pub fn accept_suggestion(&mut self) {
        if let Some(idx) = self.suggestion_idx {
            if let Some(&cmd) = self.suggestions.get(idx) {
                self.command_input = cmd.to_string();
                if cmd == "add" || cmd == "search" {
                    self.command_input.push(' ');
                }
                self.suggestion_idx = None;
                self.update_suggestions();
            }
        }
    }

    pub fn refilter(&mut self) {
        let query = self.search_query.to_lowercase();
        let tab = self.tab;
        self.filtered = self.flickers.iter().enumerate()
            .filter(|(_, f)| {
                let status_ok = match tab {
                    0 => f.meta.status == Status::Inbox,
                    1 => f.meta.status == Status::Kept,
                    2 => f.meta.status == Status::Archived,
                    _ => f.meta.status != Status::Deleted,
                };
                status_ok && (query.is_empty() || f.body.to_lowercase().contains(&query))
            })
            .map(|(i, _)| i)
            .collect();
        self.filtered.sort_by(|&a, &b| {
            self.flickers[b].meta.created_at.cmp(&self.flickers[a].meta.created_at)
        });
        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }

    pub fn selected_flicker(&self) -> Option<&Flicker> {
        self.filtered.get(self.selected).map(|&i| &self.flickers[i])
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.filtered.len() { self.selected += 1; }
    }

    pub fn cycle_tab(&mut self) {
        self.tab = (self.tab + 1) % 4;
        self.search_query.clear();
        self.refilter();
    }

    pub fn delete_selected(&mut self) {
        if let Some(&idx) = self.filtered.get(self.selected) {
            self.flickers[idx].meta.status = Status::Deleted;
            crate::storage::write(&self.flickers[idx]).ok();
            self.refilter();
        }
    }

    pub fn cycle_status_selected(&mut self) {
        if let Some(&idx) = self.filtered.get(self.selected) {
            self.flickers[idx].meta.status = match self.flickers[idx].meta.status {
                Status::Inbox => Status::Kept,
                Status::Kept => Status::Archived,
                Status::Archived | Status::Deleted => Status::Inbox,
            };
            crate::storage::write(&self.flickers[idx]).ok();
            self.refilter();
        }
    }

    pub fn add_flicker(&mut self, text: String) {
        let id = Uuid::new_v4().to_string().replace('-', "")[..8].to_string();
        let flicker = Flicker {
            meta: Frontmatter {
                id,
                created_at: Utc::now(),
                source: "cli".to_string(),
                audio_file: None,
                status: Status::Inbox,
            },
            body: text,
        };
        crate::storage::write(&flicker).ok();
        self.flickers.push(flicker);
        self.refilter();
    }
}
