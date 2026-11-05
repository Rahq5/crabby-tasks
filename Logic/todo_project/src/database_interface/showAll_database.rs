use crate::entities::Task::Task;
use crate::entities::TaskStatusEnum::{TaskImportance,TaskStatusCheck};

use std::fs::{self};


pub fn load_tasks () -> Vec<Task>{

    let path = String::from("Database.csv");
    // this is the database file hat stores non-volatile records 

    let file = fs::read_to_string(path).expect("### ERROR: cannot read file ###");
    // reads the file and save it as a String

    let mut tasks:Vec<Task> = Vec::new(); 
    // this is the holder that holds tasks extacted from the strings lines in the db.csv

    for line in file.lines().skip(1){  // for each line in content, skipping metadata (first line)
        let cols: Vec<&str> = line.splitn(5, ',').collect();  
        // make a vector of 5 elements 

        if cols.len() < 5 {continue;}
        // avoid corrupted data

        let (Ok(id), Ok(status), Ok(importance)) = (
            cols[0].trim().parse::<i32>(),
            cols[2].parse::<TaskStatusCheck>(),
            cols[3].parse::<TaskImportance>(),
        ) else {
            continue;
        };
        // here it does the mapping from the vector of strings "cols" to Task entities

        tasks.push(Task::new(id, cols[1].to_string(),status,importance,cols[4].to_string()));
        // push that task into the vector of tasks to be printed later to user 

        
       
    }
    tasks
    // return the vector of Task entities 
    
}