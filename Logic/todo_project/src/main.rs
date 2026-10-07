mod entities;
use entities::Task::Task;
use std::io;
use crate::entities::TaskStatusEnum::{TaskImportance::{self, NONE}, TaskStatusCheck::{self, TODO}};



/*
TODO:

- make a function that takes user input and name it creation, task struct should be printed in main func scope


*/


fn main() {
  //test_task();

  /*
  here i was trying to make some primtive way to take input for 
  all fields of task struct , but stuck at something where when i note the 
  task as "done" it prints TODO instead, probably a string problem  

  the fix is when i revealed the what really is being saved , i saw that it didnt flush or
  get rid of the old input so it kept gathering all things on each other like in this 
  example output that gathers both task name and status "clean the dishes \ndone"
  so you have to flush the input method each time you use it 
   */

  let mut temp = String::new();
  
  read_line(&mut temp, "Name");
  let mut name = temp.trim().to_string();
  

  read_line(&mut temp, "status");

  println!("{:?}",temp.trim());
  let mut status = match temp.trim() {
  
      "done" => TaskStatusCheck::DONE,
      "todo" => TaskStatusCheck::TODO,
      _      => TaskStatusCheck::TODO
  };

  println!("{} \n{:?}",name,status);
 
}

fn read_line(temp:&mut String, label:&str){

    println!("{}: ",label);
    io::stdin().read_line(temp).unwrap();

}

fn create_task() {

    let mut temp = String::new();
   

    io::stdin()
        .read_line(&mut temp,)
        .expect("input a valid name for the task");

    io::stdin()
        .read_line(&mut temp)
        .expect("input a valid status for the task");

    io::stdin()
        .read_line(&mut temp)
        .expect("input a valid importance for the task");

    io::stdin()
        .read_line(&mut temp)
        .expect("input a valid description for the task");


}

fn test_task(){
      // create a new Task instance using the constructor
    let mut task = Task::new(String::from("Learn Rust"), TODO,NONE,String::from("something"));

    println!("{:#?}",task);
    
    // update the fields using the setters
    task.name_setter(String::from("clean the dished"));
    task.status_setter(TaskStatusCheck::DONE);
    task.importance_setter(TaskImportance::MEDIUM);
    task.description_setter(String::from("then dry them and put them in the trash"));


    println!("{:#?}",task);
    

    // print the state again after updating
}




