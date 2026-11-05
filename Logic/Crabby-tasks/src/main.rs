mod entites;

use core::task;
use std::fs::{self, OpenOptions};
use std::fs::File;
use std::fs::read_to_string;
use std::mem::take;
use std::os::linux::raw::stat;
use entites::Task::Task;
use entites::TaskStatusEnum::*;
use std::io::{self, Read, Write};
use std::path::Path;







/*    ######## plan to do ##########
    make a function that iterates through all of the db and finds a placve then writes to it 


**/  
fn main (){

   //let file = read_lines("Database.csv");

    // let tasks = load_tasks("Database.csv".to_string());
    //     for t in &tasks {
    //         println!("{} | {} | {:?} | {:?}", t.id_getter(), t.name_getter(), t.status_getter(), t.importance_getter());
    // }

    let mut task = write_task();
    println!("{:?}",task);

    let tasks = map_to_task(task);

    create_task("Database.csv", &tasks);
    println!("{:?}",tasks);

}

fn map_to_task(task_components:Vec<String>)->Task{

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


fn ask(prompt: &str)->String{

    println!("{prompt}");

    let mut input = String::new();

        io::stdin()
        .read_line( &mut input)
        .expect("failed to log name");

    input.trim().to_string()


}

fn validate_task_fields (task_name:&str, task_status:&str, task_importance:&str, task_desc:&str)->Vec<String>{


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

// this gonna take user's task details
fn write_task ()-> Vec<String>{


    let task_id:String = next_id("Database.csv")
        .expect("failed to read database")
        .to_string();


    loop{
        let task_name = ask("enter task name");
        let task_status=ask("enter status level\n1-DONE\n2-TODO");
        let task_importance=ask("enter importance level\n1-HIGH\n2-MEDIUM\n3-LOW\n4-NONE");
        let task_desc=ask("enter description for the task");


        let errors = validate_task_fields(&task_name, &task_status, &task_importance, &task_desc);

        if(errors.is_empty()){
            return vec![task_id,task_name,task_status,task_importance,task_desc];
        }

        println!("ERRORS OCCUR: Please fix the following inputs and try again");

        for er in &errors{
            println!("- {er}");
        }


    }


    //missing things:
    // . cleaning the values being saved  DONE
    // . validating values                DONE
    // . mapping to a task object 
    // . call an ID_next() to set the id for the task DONE
    // . return the task to create_task()




    


    
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


fn next_id(path: &str) -> std::io::Result<i32> {
    // read the whole database as one String
    let content = fs::read_to_string(path)?;

    // the file has 1 header line + N task lines, so the line count is N + 1,
    // which is exactly the next id (10 tasks -> 11 lines -> id 11)
    Ok(content.lines().count() as i32)
}

fn create_task(path: &str, task: &Task) -> std::io::Result<()> {
    // 1. `task` is received as a borrowed parameter (&Task): we only read it

    // 2. open the database in append mode: the write cursor starts at the
    //    end of the file, so "the last place" is found by the OS itself
    let mut file = OpenOptions::new().append(true).open(path)?;

    // 3. map the Task object into one CSV line
    //    {:?} prints the enum variant name, e.g. Done
    let row = format!(
        "{},{},{:?},{:?},{}\n",
        task.id_getter(),
        task.name_getter(),
        task.status_getter(),
        task.importance_getter(),
        task.description_getter() // guessed name, rename to your real description getter
    );

    println!("###\ninside saving of DB and it's working!\n###\n ");

    // 4. add the CSV line at the end of the database
    file.write_all(row.as_bytes())


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