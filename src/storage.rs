use crate::{Store, Todo};
use std::fs;
use std::io;
use uuid::Uuid;

pub fn get_store_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn get_store(path: &str) -> Store {
    match get_store_file(path) {
        Ok(content) => serde_json::from_str::<Store>(&content).unwrap(),
        Err(_) => Store { store: Vec::new() },
    }
}

pub fn save_store(path: &str, store: &Store) {
    let json = serde_json::to_string_pretty(store).unwrap();
    fs::write(path, &json).unwrap();
}

pub fn add_todo(description: String, path: &str) {
    let new_todo = Todo {
        description,
        id: Uuid::new_v4().to_string(),
    };

    let mut store = get_store(path);

    store.store.push(new_todo);

    save_store(path, &store);

    println!("Added todo to store!");
    println!("Total todos: {}", store.store.len());
}
