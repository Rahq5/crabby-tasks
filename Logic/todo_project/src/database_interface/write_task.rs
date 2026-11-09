use crate::entities::Task::Task;
use std::fs::{self, OpenOptions};
use std::io::Write;


//write task into database
pub fn create_task_database(path: &str, task: &Task) -> std::io::Result<()> {


    // open the database in append mode, the write cursor starts at the
    // end of the file so "the last place" is found by the OS itself
    // credit: AI for thi line
    let mut file = OpenOptions::new().append(true).open(path)?;

    // map the Task object into one CSV line
    // {:?} prints the enum variant name, e.g. Done
    let row = format!(
        "{},{},{:?},{:?},{}\n",
        task.id_getter(),
        task.name_getter(),
        task.status_getter(),
        task.importance_getter(),
        task.description_getter() 
    );

    //DELETE THIS: JUST FOR TESTING
    println!("###\ninside saving of DB and it's working!\n###\n ");

    // 4. add the CSV line at the end of the database
    file.write_all(row.as_bytes())


}