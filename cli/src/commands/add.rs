use chrono::Utc;
use uuid::Uuid;
use crate::model::{Flicker, Frontmatter, Status};
use crate::storage;

pub fn run(text: &str) {
    let id = Uuid::new_v4().to_string().replace('-', "")[..8].to_string();
    let mut flicker = Flicker {
        meta: Frontmatter {
            id: id.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            source: "cli".to_string(),
            audio_file: None,
            status: Status::Inbox,
        },
        body: text.to_string(),
    };
    storage::write(&mut flicker).unwrap();
    println!("{id}");
}
