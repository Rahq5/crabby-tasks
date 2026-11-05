mod entites;

use std::fs::{self, OpenOptions};
use std::fs::File;
use std::fs::read_to_string;
use entites::Task::Task;
use entites::TaskStatusEnum::*;
use std::io::{Read, Write};
use std::path::Path;


/*    ######## plan to do ##########
    
    1. make the diagram of showing all tasks
        - code should show all tasks without the description field (or only first 10-15 letters)
        - code should show the incremental ID of the task 


**/  
fn main (){

   let file = read_lines("Database.csv");

    let tasks = load_tasks("Database.csv".to_string());
        for t in &tasks {
            println!("{} | {} | {:?} | {:?}", t.id_getter(), t.name_getter(), t.status_getter(), t.importance_getter());
    }
}


fn load_tasks (path: String) -> Vec<Task>{

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

// this method should take each line of the DB file and save them into a vector
// of strings
fn read_lines(filename: &str)-> Vec<String>{
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }

    result

}

fn print_all_of_db(){

    let path = Path::new("Database.csv");
    let display = path.display(); // just to keep the path
    
   
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("couldnt read these: {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}",display,s),
    }
}