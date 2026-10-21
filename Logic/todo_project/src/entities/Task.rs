
use super::TaskStatusEnum::{TaskImportance,TaskStatusCheck};
// this should import the enum classes from the enums file

#[derive(Debug)]
pub struct Task{
    ID: i32,
    name: String,
    status: TaskStatusCheck,
    importance: TaskImportance,
    description: String
}

// Task implementation
impl Task{

    // constructor: returns a Task type after being initilized
    pub fn new(ID:i32,name: String, status:TaskStatusCheck, importance: TaskImportance, description: String)->Self{
        Self{
            ID,
            name,
            status,
            importance,
            description,
        }
    }


    //getters
    pub fn name_getter(&self)->String{
        self.name.clone()
    }

    pub fn ID_getter(&self)->&i32{
        &self.ID 
    }

    pub fn status_getter(&self)-> &TaskStatusCheck{
        &self.status
    }

    pub fn importance_getter(&self)-> &TaskImportance{
        &self.importance
    }

    pub fn description_getter(&self) -> &str {
        &self.description
    }

    // setters
    //using mut here cuz you gonna update some data
    pub fn ID_setter(&mut self, ID:i32){
        self.ID =ID;
    }

    pub fn name_setter(&mut self, name:String){
        self.name =name;
    }

    pub fn status_setter(&mut self, status:TaskStatusCheck){
        self.status =status;
    }

    pub fn importance_setter(&mut self, importance:TaskImportance){
        self.importance =importance;
    }

    pub fn description_setter(&mut self, description: String) {
        self.description = description;
    }
}

