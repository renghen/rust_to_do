use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Cancel,
}

#[derive(Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    // pub comments: Vec<String>,
    pub status: Status,
}
