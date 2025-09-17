use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub store: Vec<Todo>,
}