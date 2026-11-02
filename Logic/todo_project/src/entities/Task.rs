use super::TaskStatusEnum::{TaskImportance,TaskStatusCheck};
// this should import the enum classes from the enums file

#[derive(Debug)]
pub struct Task {
    id: i32,
    name: String,
    status: TaskStatusCheck,
    importance: TaskImportance,
    description: String,
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        status: TaskStatusCheck,
        importance: TaskImportance,
        description: String,
    ) -> Self {
        Self { id, name, status, importance, description }
    }

    pub fn id_getter(&self) -> i32 { self.id }
    pub fn name_getter(&self) -> &str { &self.name }
    pub fn status_getter(&self) -> &TaskStatusCheck { &self.status }
    pub fn importance_getter(&self) -> &TaskImportance { &self.importance }
    pub fn description_getter(&self) -> &str { &self.description }

    pub fn name_setter(&mut self, name: String) { self.name = name; }
    pub fn status_setter(&mut self, status: TaskStatusCheck) { self.status = status; }
    pub fn importance_setter(&mut self, importance: TaskImportance) { self.importance = importance; }
    pub fn description_setter(&mut self, description: String) { self.description = description; }
}