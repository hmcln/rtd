use clap::{Parser, Subcommand};

mod commands;
mod config;
mod storage;

use rtd::{Store, Todo};

#[derive(Parser)]
#[command(name = "rtd")]
#[command(about = "A simple Rust Todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add,
    /// Push a new todo item
    Push,
    /// Migrate store to current data structure
    Migrate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => commands::add::execute(),
        Commands::Push => commands::add::execute_push(),
        Commands::Migrate => commands::migrate::execute(),
    }
}
