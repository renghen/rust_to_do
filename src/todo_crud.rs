use crate::todo::Todo;

pub trait TodoCrud {
    fn add(&mut self, todo: &Todo) -> Result<(), String>;

    fn delete(&mut self, todo_id: u32) -> Result<(), String>;

    // fn update(&self, todo: &Todo) -> Result<&Todo, CrudErrors>;

    fn find(&self, id: u32) -> Result<&Todo, String>;
}
