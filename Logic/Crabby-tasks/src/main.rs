mod entites;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::fs::read_to_string;
use entites::Task::Task;



/*    ######## plan to do ##########
    
    1. make the diagram of showing all tasks
        - code should show all tasks without the description field (or only first 10-15 letters)
        - code should show the incremental ID of the task 


**/  

// ===== PROBLEMS in load_tasks() =====
//
// 1. [E0277] `let cols: Vec<String> = line.splitn(5, ',').collect();`
//    splitn() yields &str (borrowed slices of `line`), but the Vec is declared as
//    Vec<String> (owned text). The Vec's element type must match what the iterator yields.
//
// 2. [private fields] `Task { ... }` can't be built from main.rs.
//    Fields without `pub` are visible only inside the module that defines Task.
//    Fix direction: make the fields `pub`, or build the Task through a constructor
//    like Task::new(...), as the compiler suggests.
//
// 3. [E0308] `importance: cols[2].to_string()`
//    The field type is TaskImportance (your enum), but .to_string() gives a String.
//    The text has to be converted into the enum first (FromStr or a match).
//
// 4. [E0308] `status: cols[3].to_string()`
//    Same as problem 3, but the field type is TaskStatusCheck.
//
// 5. [logic bug] importance and status are swapped.
//    CSV column order is ID,name,status,importance,description, so
//    cols[2] is STATUS and cols[3] is IMPORTANCE.
//
// 6. [missing field] `description` is not set in the struct literal.
//    Rust requires every field to be set, and cols[4] holds its value.
//
// 7. [style, warning] the field is named `ID`.
//    Rust style is snake_case for field names, so it should be `id`.
//
// Order to fix: 2 first (it blocks construction), then 1, 3, 4, 5, 6.

fn main (){

   let file = read_lines("Database.csv");


}


fn load_tasks (path: String) -> Vec<Task>{

    let file = fs::read_to_string(path).expect("### ERROR: cannot read file ###");
    // reads the file and save it as a String

    let mut tasks = Vec::new(); 
    // this is the holder of the vector of strings

    for line in file.lines().skip(1){  // for each line in content, skipping metadata (first line)
        let cols: Vec<String> = line.splitn(5, ',').collect();
        //

        if cols.len() < 5 {continue;}
        // avoid corrupted data

        tasks.push(Task{
            ID: cols[0].parse().unwrap(),
            name: cols[1].to_string(),
            importance: cols[2].to_string(),
            status: cols[3].to_string()
        });
    }

    tasks
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