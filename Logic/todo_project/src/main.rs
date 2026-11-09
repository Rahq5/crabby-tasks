mod entities;
mod database_interface;
mod service;
mod helpers;

use std::io::stdin;

use service::showAll_Service::showAllServices; // complete the function call 
use service::create_task::write_task;

use crate::service::create_task::create_task;

// use serde::Deserialize;
// #[derive(Deserialize)]
// pub struct storage { pub path:String}


fn greetings_and_show_options(){
    // this function welcome the user
    // and show options 

    println!("
        #####################################################################
        
                                    welcome user! 

        ####################################################################

        please choose an option to proceed with the below tasks:
        Create:     enter 1 
        Update:     enter 2
        Remove:     enter 3
        Show:       enter 4
        ShowAll:    enter 5
    
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
// main user will act as a frontend for the user 


    greetings_and_show_options();
    // greet and show options to user 
    

    loop {
        let mut input= take_option();

        match input{
            1 => {println!("choosed Creation"); create_task();},
            2 => {println!("choosed Update"); break; },
            3 => {println!("choosed Remove"); break;},
            4 => {println!("chooosed Show"); break;},
            5 => {println!("chooosed ShowAll"); showAllServices();},
            _ => {
                println!("## ERROR: please choose a number from the list ##");
                //greetings_and_show_options();
            }


        }
    }

   

}








