mod entities;

fn main() {
    // create a new Task instance using the constructor
    let mut task = Task::new(String::from("Learn Rust"), false, 5,String::from("something"));

    // print the initial state using the getters
    println!("Name: {}", task.name_getter());
    println!("Status: {}", task.status_getter());
    println!("Importance: {}", task.importance_getter());

    // update the fields using the setters
    task.name_setter(String::from("Learn Rust Structs"));
    task.status_setter(true);
    // importance_setter now takes i32, not String, since it
    // now matches the field's actual type — pass 10 directly
    task.importance_setter(10);

    // print the state again after updating
    println!("Updated Name: {}", task.name_getter());
    println!("Updated Status: {}", task.status_getter());
    println!("Updated Importance: {}", task.importance_getter());
}

/*
fix :

- line 5 says it cant see Task module
*/
