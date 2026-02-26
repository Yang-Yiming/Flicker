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
        }
        ConfigAction::Get { key } => {
            let config = load();
            match key.as_str() {
                "editor" => println!("{}", config.editor),
                "shell" => println!("{}", config.shell),
                _ => { eprintln!("unknown key: {key}"); std::process::exit(1); }
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = load();
            match key.as_str() {
                "editor" => config.editor = value,
                "shell" => config.shell = value,
                _ => { eprintln!("unknown key: {key}"); std::process::exit(1); }
            }
            save(&config).unwrap_or_else(|e| { eprintln!("Error saving config: {e}"); std::process::exit(1); });
        }
    }
}
