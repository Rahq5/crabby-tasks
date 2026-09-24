struct Task{
    task_name: String,
    status: bool,  //this should be an enum
    importance: String // this should be an enum
}

fn main(){

    let task1 = Tasks{
        task_name: String::from("clean the house"),
        status: True,
        importance: String::from("HIGH")
    };


    println!("{}",task1);
}