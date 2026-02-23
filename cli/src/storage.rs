use std::path::PathBuf;
use crate::model::Flicker;

pub fn flickers_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("FLICKER_DIR") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME").unwrap();
    PathBuf::from(home)
        .join("Library/Mobile Documents/iCloud~com~flicker~app/Documents/flickers")
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
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".md") && !name.contains(' ')
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

pub fn write(flicker: &Flicker) -> std::io::Result<()> {
    let dir = flickers_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{}.md", flicker.meta.id)), flicker.to_file_content())
}

pub fn conflict_files() -> Vec<String> {
    let dir = flickers_dir();
    if !dir.exists() {
        return vec![];
    }
    std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".md") && name.contains(' ')
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}
