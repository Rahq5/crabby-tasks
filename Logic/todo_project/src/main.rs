mod entities;
use entities::Task::Task;
use std::io::stdin;
use entities::TaskStatusEnum::TaskImportance::*;
use entities::TaskStatusEnum::TaskStatusCheck::*;
//use crate::entities::TaskStatusEnum::{TaskImportance::{self, NONE}, TaskStatusCheck::{self, TODO}};


/*
TODO:

- make a method that shows three things:
    1. welcome message
    2. all TODO status tasks
    3. show options

*/


fn fetch_tasks(){

    // this function should read the saved TODO tasks and get show them 
}

fn greetings_and_show_options(){
    // this function welcome the user
    // and show options 

    println!("
        #####################################################################
        
                                    welcome user! 

        ####################################################################

        please choose an option to proceed with the below tasks:
        Create: enter 1 
        Update: enter 2
        Remove: enter 3
        Show:   enter 4
    
    ");

}

fn take_option()->i32{

    // function made just to take input of option from user 


    println!("please enter a number from the list");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("error reading line");

    let input:i32 = input.trim().parse().expect("enter a valid number"); 
    
    input

}

fn main (){

    greetings_and_show_options();

    loop {
        let mut input= take_option();

        match input{
            1 => {println!("choosed Creation"); break;},
            2 => {println!("choosed Update"); break; },
            3 => {println!("choosed Remove"); break;},
            4 => {println!("chooosed Show"); break;},
            _ => {
                println!("## ERROR: please choose a number from the list ##");
                //greetings_and_show_options();
            }


        }
    }

   

}






