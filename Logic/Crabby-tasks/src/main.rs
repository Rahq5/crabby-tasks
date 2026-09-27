use core::task;
 use std::io;

#[derive(Debug)]
struct Task{
    task_name: String,
    status: bool,  //this should be an enum
    importance: i32, // this should be an enum
    extra: String
}

struct rectangle{
    width: u32,
    height: u32,
}

impl rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32{
        (self.width + self.height) + 2
    }
}

fn main(){

    let rect1 = rectangle{
        width: 12,
        height: 45,
    };

    println!("{} is the area of the rectangle",rect1.area());
    println!("{} is the perimeter of the rectangle",rect1.perimeter());
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


