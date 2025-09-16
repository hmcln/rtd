use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

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
}

#[derive(Serialize, Deserialize)]
struct Todo {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Store {
    store: Vec<Todo>,
}

fn add_todo(description: String, path: &str) {
    let new_todo = Todo { description };

    // Try to read existing store, or create empty one
    let mut store = match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str::<Store>(&content).unwrap(),
        Err(_) => Store { store: Vec::new() },
    };

    // Add new todo to the store
    store.store.push(new_todo);

    // Convert to JSON and save
    let json = serde_json::to_string_pretty(&store).unwrap();
    fs::write(path, &json).unwrap();

    println!("Added todo to store!");
    println!("Total todos: {}", store.store.len());
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => {
            println!("Enter todo description:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let description = input.trim().to_string();

            add_todo(description, ".tmp/store.json")
        }

        Commands::Push => {
            println!("Enter todo description (STORED PUBLICLY):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let description = input.trim().to_string();

            add_todo(description, "store.json")
        }
    }
}
