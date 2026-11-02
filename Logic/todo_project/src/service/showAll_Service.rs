use crate::database_interface::showAll_database::load_tasks;

pub fn showAllServices(){


// this should direct the call to database thing

    println!(" you reached the service layer great !!!");

    let content = load_tasks();

    for t in &content{
        println!("{} | {} | {:?} | {:?}", t.id_getter(), t.name_getter(), t.status_getter(), t.importance_getter());
    }




}

