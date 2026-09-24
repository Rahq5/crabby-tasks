use core::task;
use std::io;

struct Task{
    task_name: String,
    status: bool,  //this should be an enum
    importance: i32, // this should be an enum
    extra: String
}

fn main(){


    let mut task_name = String::new();
    let mut importance= String::new(); 


    println!("enter task name");
    io::stdin()
        .read_line(& mut task_name)
        .expect("failed   to  read line");

    println!("enter task importance");
    io::stdin()
        .read_line(& mut importance)
        .expect("failed   to  read line");


    let importance: i32 = importance.trim().parse().expect("this is not a fucking number");

    let task1 = build_task(task_name, importance);

    let task2 = Task{
        task_name: String::from("drive he cat"),
        ..task1
    };

    

      println!("\n\nthe name of task is:{}\n and the importance is:{} \n and the status is{}",task2.task_name, task2.importance, task2.status);
      println!("\n\nthe name of task is:{}\n and the importance is:{} \n and the status is{}",task1.task_name, task1.importance, task1.status);
  
   
}

fn build_task(task_name:String, importance:i32)->Task{

    let task = Task{
        task_name : task_name,
        importance: importance,
        status: false,
        extra: String::from("extra field to test"),
    };

    task
}

