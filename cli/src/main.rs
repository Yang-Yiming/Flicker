use clap::{CommandFactory, Parser, Subcommand};

mod commands;
mod model;
mod storage;
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
        None => {
            let cmds: Vec<String> = Cli::command()
                .get_subcommands()
                .map(|s| s.get_name().to_string())
                .collect();
            tui::run(cmds).unwrap()
        }
    }
}
