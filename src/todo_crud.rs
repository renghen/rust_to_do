enum CrudErrors {
    AddError(String),
    DeleteError(String),
    UpdateError(String),
    FindError(String),
}

trait TodoCrud {
    fn add(&self, todo: &Todo) -> Result<&Todo, CrudResult::AddError>;

    fn delete(&self, todo_id: u32) -> Result<(), CrudResult::DeleteError>;

    fn update(&self, todo: &Todo) -> Result<&Todo, CrudResult::UpdateError>;

    fn find(&self, id: u32) -> Result<&Todo, CrudResult::FindError>;
}
