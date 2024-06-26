use std::borrow::Borrow;

use crate::{
    todo::{Status, Todo},
    todo_crud::TodoCrud,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MemTodoTable {
    pub current_id: u32,
    pub container: Vec<Todo>,
}

impl MemTodoTable {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            container: Vec::new(),
        }
    }
}

struct MemDb {
    data: MemTodoTable,
}

impl MemDb {
    fn new() -> MemDb {
        let data = MemTodoTable::new();
        MemDb { data: data }
    }
}

impl TodoCrud for MemDb {
    fn add(&mut self, todo: &Todo) -> Result<(), String> {
        self.data.current_id += 1;
        let id = self.data.current_id;

        let mut new_todo = todo.clone();
        new_todo.id = id;
        self.data.container.push(new_todo);

        return Ok(());
    }

    fn delete(&mut self, todo_id: u32) -> Result<(), String> {
        let index = self.data.container.iter().position(|x| (*x).id == todo_id);

        match index {
            Some(i) => {
                self.data.container.remove(i);
                return Ok(());
            }
            None => Err("Not Found".to_string()),
        }
    }

    fn update(&mut self, todo: &Todo) -> Result<(), String> {
        let result =
            self.data.container.iter().position(|x| (*x).id == todo.id);

        match result {
            Some(index) => {
                if !todo.title.is_empty() {
                    self.data.container[index].title = todo.title.to_owned();
                }

                if !todo.description.is_empty() {
                    self.data.container[index].description =
                        todo.description.to_owned();
                }

                if !(matches!(todo.status, Status::NoUpdate)) {
                    self.data.container[index].status = todo.status.to_owned();
                }

                return Ok(());
            }
            None => Err("Not Found".to_string()),
        }
    }

    fn find(&self, id: u32) -> Result<&Todo, String> {
        let table = self.data.borrow();

        let mut iter = table.container.iter();
        let todo = iter.find(|&x| x.id == id);
        let result = todo.ok_or("Not Found".to_string());

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Status;

    #[test]
    fn todo_test() {
        let mut mem_db = MemDb::new();
        let title = "title test".to_string();
        let describe = "description test".to_string();

        let todo = Todo {
            id: 0,
            title: title.clone(),
            description: describe.clone(),
            status: Status::Todo,
        };

        let result_add = mem_db.add(&todo);
        let result_find = mem_db.find(1).unwrap();

        assert_eq!(result_add.is_ok(), true);
        assert_eq!(result_find.id, 1);
        assert_eq!(result_find.title, title);
        assert_eq!(result_find.description, describe);
        assert!(matches!(result_find.status, Status::Todo));

        let title_2 = "title test 2".to_string();
        let describe_2 = "description test 2".to_string();

        let todo_2 = Todo {
            id: 0,
            title: title_2.clone(),
            description: describe_2.clone(),
            status: Status::Cancel,
        };

        let result_add_2 = mem_db.add(&todo_2);
        let result_find_2 = mem_db.find(2).unwrap();

        assert_eq!(result_add_2.is_ok(), true);
        assert_eq!(result_find_2.id, 2);
        assert_eq!(result_find_2.title, title_2);
        assert_eq!(result_find_2.description, describe_2);
        assert!(matches!(result_find_2.status, Status::Cancel));
        assert_eq!(mem_db.data.container.len(), 2);

        let title_3 = "title updated".to_string();
        let todo_update = Todo {
            id: 1,
            title: title_3.clone(),
            description: "".to_string(),
            status: Status::NoUpdate,
        };

        let result_update = mem_db.update(&todo_update);
        assert_eq!(result_update.is_ok(), true);
        assert_eq!(mem_db.data.container.len(), 2);
        let result_update_find = mem_db.find(1);
        assert_eq!(result_update_find.as_ref().is_ok(), true);
        assert_eq!(result_update_find.as_ref().unwrap().title, title_3);
        assert_eq!(result_update_find.as_ref().unwrap().description, describe);
        assert!(matches!(result_update_find.unwrap().status, Status::Todo));

        let result_delete = mem_db.delete(1);
        assert_eq!(result_delete.is_ok(), true);
        assert_eq!(mem_db.data.container.len(), 1);

        let result_delete = mem_db.delete(1);
        assert_eq!(result_delete.is_ok(), false);
        assert_eq!(result_delete.unwrap_err(), "Not Found");
        assert_eq!(mem_db.data.container.len(), 1);
    }
}
