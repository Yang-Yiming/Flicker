use clap::{CommandFactory, Parser, Subcommand};

mod commands;
mod config;
mod model;
mod storage;
mod sync;
mod sync_state;
mod tui;

#[derive(Parser)]
#[command(name = "flicker", about = "Lightweight idea recorder")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    List { #[arg(long)] status: Option<String> },
    Show { id: String },
    Delete { id: String },
    Search { query: String },
    Status,
    Rename { id: String, body: String },
    Bash { cmd: String },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Sync,
}

#[derive(Subcommand)]
enum ConfigAction {
    List,
    Get { key: String },
    Set { key: String, value: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Add { text }) => commands::add::run(&text),
        Some(Commands::List { status }) => commands::list::run(status.as_deref()),
        Some(Commands::Show { id }) => commands::show::run(&id),
        Some(Commands::Delete { id }) => commands::delete::run(&id),
        Some(Commands::Search { query }) => commands::search::run(&query),
        Some(Commands::Status) => commands::status::run(),
        Some(Commands::Rename { id, body }) => commands::rename::run(&id, &body),
        Some(Commands::Bash { cmd }) => commands::bash::run(&cmd),
        Some(Commands::Config { action }) => {
            let action = match action {
                ConfigAction::List => commands::config::ConfigAction::List,
                ConfigAction::Get { key } => commands::config::ConfigAction::Get { key },
                ConfigAction::Set { key, value } => commands::config::ConfigAction::Set { key, value },
            };
            commands::config::run(action);
        }
        Some(Commands::Sync) => {
            match sync::SyncClient::from_config() {
                Some(client) => {
                    match client.sync() {
                        Ok((pulled, pushed)) => println!("Synced: pulled {pulled}, pushed {pushed}"),
                        Err(e) => eprintln!("Sync failed: {e}"),
                    }
                }
                None => eprintln!("Supabase not configured. Run:\n  flicker config set supabase_url <url>\n  flicker config set supabase_anon_key <key>"),
            }
        }
        None => {
            let cmds: Vec<String> = Cli::command()
                .get_subcommands()
                .map(|s| s.get_name().to_string())
                .collect();
            tui::run(cmds).unwrap()
        }
    }
}
