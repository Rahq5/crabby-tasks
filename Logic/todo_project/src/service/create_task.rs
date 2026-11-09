use crate::helpers::helpers_methods::{self,ask,next_id,validate_task_fields,map_to_task,print_task};
use crate::database_interface::write_task::create_task_database;



pub fn create_task(){

    // aks user to input task fields
    let task_fields = write_task();

    // map the received vector to Task object
    let task_object = map_to_task(task_fields);

    // write object to DB
    create_task_database("Database.csv", &task_object);

    //println!("{:?}",&tasks);
    print_task(&task_object);


}
// change name to create_task
pub fn write_task ()-> Vec<String>{


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

}

