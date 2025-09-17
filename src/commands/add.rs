use crate::config;
use crate::storage;

use std::io;

pub fn add_helper(path: &str) {
    println!("Enter todo description:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let description = input.trim().to_string();

    storage::add_todo(description, path);
}

pub fn execute() {
    let path = config::get_store_path();
    add_helper(&path);
}

pub fn execute_push() {
    // TODO: make this pull from config file
    let path = "store.json";
    add_helper(path);
}

