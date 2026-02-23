use chrono::Utc;
use uuid::Uuid;
use crate::model::{Flicker, Frontmatter, Status};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    List,
    Detail,
    Search,
    Add,
}

pub struct App {
    pub mode: Mode,
    pub flickers: Vec<Flicker>,
    pub filtered: Vec<usize>,
    pub selected: usize,
    pub tab: usize, // 0=inbox 1=kept 2=archived 3=all
    pub search_query: String,
    pub add_input: String,
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
        };
        app.refilter();
        app
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
