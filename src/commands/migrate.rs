use crate::storage;
use rtd::{Store, Todo};
use serde::Deserialize;
use serde_json;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct LegacyTodoItem {
    description: String,
    id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LegacyStore {
    store: Vec<LegacyTodoItem>,
}

pub fn get_or_create_todo(id: Option<String>, description: String) -> Todo {
    let get_or_create_id = match id {
        Some(id) => id,
        None => Uuid::new_v4().to_string(),
    };

    Todo {
        id: get_or_create_id,
        description,
    }
}

pub fn execute() {
    let path = "store.json";
    let store_file_contents = storage::get_store_file(path).unwrap();
    let parsed_contents = serde_json::from_str::<LegacyStore>(&store_file_contents).unwrap();

    let mut new_store: Store = Store { store: Vec::new() };

    for todo in parsed_contents.store {
        new_store
            .store
            .push(get_or_create_todo(todo.id, todo.description))
    }

    storage::save_store(path, &new_store)
}
