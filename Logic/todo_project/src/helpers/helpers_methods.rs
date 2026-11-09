use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};

use crate::entities::Task::Task;
use crate::entities::TaskStatusEnum::{TaskImportance, TaskStatusCheck};



 // read the whole database as one String
pub fn next_id(path: &str) -> std::io::Result<i32> {
   
    let content = fs::read_to_string(path)?;

    // the file has 1 header line + N task lines, so the line count is N + 1,
    // which is exactly the next id (10 tasks -> 11 lines -> id 11)
    Ok(content.lines().count() as i32)
}



// takes input from user only
pub fn ask(prompt: &str)->String{

    println!("{prompt}");

    let mut input = String::new();

        io::stdin()
        .read_line( &mut input)
        .expect("failed to log name");

    input.trim().to_string()


}

// validate task fields in case something entered wrong and unacceptable
pub fn validate_task_fields (task_name:&str, task_status:&str, task_importance:&str, task_desc:&str)->Vec<String>{


    // this vector gonna save all error messages of occurs
    let mut errors: Vec<String> = Vec::new();

    // check if name is empty 
    if (task_name.is_empty()){ 
        errors.push("task_name: CANNOT BE EMPTY".to_string());
    }

    // check if task name doesnt contain commas cuz that gonna affect the csv file 
    // since it's comma seperated format
    if (task_name.contains(",")){ 
        errors.push("task_name: CANNOT BE CONTAINNING COMMAS".to_string());
    }

    // checks if empty and not equal to 1 or 2 which is DONE and TODO
    if (task_status != "1" && task_status != "2"){ 
        errors.push("task_status: CANNOT BE EMPTY".to_string());
    }

    //checks if user picked out of options or left it empty 
    if (!["1","2","3","4"].contains(&task_importance)){ 
        errors.push("task_importance: CANNOT BE EMPTY OR OUT OF SHOWN CHOICES".to_string());
    }

    errors

}


pub fn map_to_task(task_components:Vec<String>)->Task{

    // expected the results are just enterd from the user
    // means the status and importance are still number and not enums yet

    let status = match task_components[2].as_str(){
        "1" => TaskStatusCheck::TODO,
        "2" => TaskStatusCheck::DONE,
        _   => TaskStatusCheck::TODO
    };

    let importance = match task_components[3].as_str(){
        "1" => TaskImportance::HIGH,
        "2" => TaskImportance::MEDIUM,
        "3" => TaskImportance::LOW,
        "4" => TaskImportance::NONE,
        _   => TaskImportance::NONE
    };

    let task_id:i32 = task_components[0].parse().expect("cant convert to i32");

    Task::new(task_id,task_components[1].clone(),status,importance,task_components[4].clone())


}

pub fn print_task(task:&Task){

    

    let task_name = task.name_getter();
    let task_status = task.status_getter();
    let task_imp = task.importance_getter();
    let task_desc = task.description_getter();

    println!("\n----------TASK CREATED !-----------");
    println!("Task name: {}", task_name);
    println!("Task status: {:?}", task_status);
    println!("Task importance: {:?}", task_imp);
    println!("Task description: {}", task_desc);
    println!("---------------------\n");



}


