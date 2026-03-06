use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    pub shell: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supabase_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supabase_anon_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: default_editor(),
            shell: default_shell(),
            storage_path: None,
            supabase_url: None,
            supabase_anon_key: None,
        }
    }
}

fn default_editor() -> String {
    if std::process::Command::new("nvim").arg("--version").output().is_ok() {
        "nvim".to_string()
    } else {
        "vim".to_string()
    }
}

fn default_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| PathBuf::from(&s).file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "sh".to_string())
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/flicker/config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if let Ok(contents) = std::fs::read_to_string(&path) {
        toml::from_str(&contents).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = toml::to_string(config)?;
    std::fs::write(&path, contents)?;
    Ok(())
}
