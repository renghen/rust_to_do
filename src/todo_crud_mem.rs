use crate::todo::Todo;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MemTodo {
    current_id: u32,
    container: Vec<Todo>,
}
