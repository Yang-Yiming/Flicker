use clap::{Parser, Subcommand};

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
        None => tui::run().unwrap(),
    }
}
