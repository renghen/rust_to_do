use crate::todo::Todo;

enum CrudErrors {
    AddError(String),
    DeleteError(String),
    UpdateError(String),
    FindError(String),
}

trait TodoCrud {
    fn add(&self, todo: &Todo) -> Result<&Todo, CrudErrors>;

    fn delete(&self, todo_id: u32) -> Result<(), CrudErrors>;

    fn update(&self, todo: &Todo) -> Result<&Todo, CrudErrors>;

    fn find(&self, id: u32) -> Result<&Todo, CrudErrors>;
}
