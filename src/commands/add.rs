use crate::storage;
use std::io;

pub fn execute() {
    println!("Enter todo description:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let description = input.trim().to_string();

    storage::add_todo(description, ".tmp/store.json");
}