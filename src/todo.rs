use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum Status {
    Todo,
    In_progress,
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
