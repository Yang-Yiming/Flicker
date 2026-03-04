use std::path::PathBuf;
use chrono::Utc;
use crate::model::Flicker;

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    }
}

pub fn flickers_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("FLICKER_DIR") {
        return PathBuf::from(dir);
    }

    let config = crate::config::load();
    match config.storage_path {
        None => {
            eprintln!("Storage path not configured. Run: flicker config set storage_path <path>");
            std::process::exit(1);
        }
        Some(ref path) => PathBuf::from(expand_tilde(path)),
    }
}

pub fn read_all() -> Vec<Flicker> {
    let dir = flickers_dir();
    if !dir.exists() {
        return vec![];
    }
    std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name().to_string_lossy().ends_with(".md")
        })
        .filter_map(|e| {
            let content = std::fs::read_to_string(e.path()).ok()?;
            Flicker::from_file_content(&content).ok()
        })
        .collect()
}

pub fn read_one(id: &str) -> Option<Flicker> {
    let path = flickers_dir().join(format!("{id}.md"));
    let content = std::fs::read_to_string(path).ok()?;
    Flicker::from_file_content(&content).ok()
}

pub fn write(flicker: &mut Flicker) -> std::io::Result<()> {
    flicker.meta.updated_at = Utc::now();
    let dir = flickers_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{}.md", flicker.meta.id)), flicker.to_file_content())
}

pub fn audio_dir() -> PathBuf {
    flickers_dir().parent().unwrap().join("audio")
}
