use crate::config::{load, save};

pub enum ConfigAction {
    List,
    Get { key: String },
    Set { key: String, value: String },
}

pub fn run(action: ConfigAction) {
    match action {
        ConfigAction::List => {
            let config = load();
            println!("editor = {}", config.editor);
            println!("shell = {}", config.shell);
            println!("storage_path = {}", config.storage_path.as_deref().unwrap_or(""));
            println!("supabase_url = {}", config.supabase_url.as_deref().unwrap_or(""));
            println!("supabase_anon_key = {}", if config.supabase_anon_key.is_some() { "***" } else { "" });
        }
        ConfigAction::Get { key } => {
            let config = load();
            match key.as_str() {
                "editor" => println!("{}", config.editor),
                "shell" => println!("{}", config.shell),
                "storage_path" => println!("{}", config.storage_path.as_deref().unwrap_or("")),
                "supabase_url" => println!("{}", config.supabase_url.as_deref().unwrap_or("")),
                "supabase_anon_key" => println!("{}", config.supabase_anon_key.as_deref().unwrap_or("")),
                _ => { eprintln!("unknown key: {key}"); std::process::exit(1); }
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = load();
            match key.as_str() {
                "editor" => config.editor = value,
                "shell" => config.shell = value,
                "storage_path" => {
                    if value.is_empty() {
                        eprintln!("storage_path cannot be empty");
                        std::process::exit(1);
                    }
                    if value.starts_with("~/") || value.starts_with('/') {
                        let home = std::env::var("HOME").unwrap();
                        let expanded = value.replacen("~", &home, 1);
                        config.storage_path = Some(expanded);
                    } else {
                        eprintln!("storage_path must start with '~/' or be an absolute path");
                        std::process::exit(1);
                    }
                }
                "supabase_url" => config.supabase_url = Some(value),
                "supabase_anon_key" => config.supabase_anon_key = Some(value),
                _ => { eprintln!("unknown key: {key}"); std::process::exit(1); }
            }
            save(&config).unwrap_or_else(|e| { eprintln!("Error saving config: {e}"); std::process::exit(1); });
        }
    }
}
