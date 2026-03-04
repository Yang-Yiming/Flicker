use chrono::{DateTime, Utc};
use std::path::PathBuf;

fn state_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/flicker/sync_state.toml")
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct SyncState {
    last_synced_at: Option<String>,
}

pub fn load_last_synced() -> Option<DateTime<Utc>> {
    let path = state_path();
    let contents = std::fs::read_to_string(path).ok()?;
    let state: SyncState = toml::from_str(&contents).ok()?;
    state.last_synced_at?.parse::<DateTime<Utc>>().ok()
}

pub fn save_last_synced(ts: DateTime<Utc>) {
    let state = SyncState {
        last_synced_at: Some(ts.to_rfc3339()),
    };
    let path = state_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, toml::to_string(&state).unwrap_or_default());
}
