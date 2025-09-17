use clap::{Parser, Subcommand};
use std::io;

mod commands;
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

        Commands::Push => {
            println!("Enter todo description (STORED PUBLICLY):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let description = input.trim().to_string();

            storage::add_todo(description, "store.json")
        }

        Commands::Migrate => commands::migrate::execute(),
    }
}
