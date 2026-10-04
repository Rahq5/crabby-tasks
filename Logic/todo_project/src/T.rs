// //
// mod TaskStatusEnum;
// use TaskStatusEnum::TaskStatusCheck;
// use TaskStatusEnum::TaskImportance;



// // defined a public task struct with fields
// #[derive(Debug)]
// pub struct Task{
//     name: String,
//     status: TaskStatusCheck,
//     importance: TaskImportance,
//     description: String
// }

// // Task implementation
// impl Task{

//     // constructor: returns a Task type after being initilized
//     pub fn new(name: String, status:TaskStatusCheck, importance: TaskImportance, description: String)->Self{
//         Self{
//             name,
//             status,
//             importance,
//             description,
//         }
//     }


//     //getters
//     pub fn name_getter(&self)->String{
//         self.name.clone()
//     }

//     pub fn status_getter(&self)->bool{
//         self.status
//     }

//     pub fn importance_getter(&self)->i32{
//         self.importance
//     }

//     // setters

//     //using mut here cuz you gonna update some data
//     pub fn name_setter(&mut self, name:String){
//         self.name =name;
//     }

//     pub fn status_setter(&mut self, status:bool){
//         self.status =status;
//     }

//     pub fn importance_setter(&mut self, importance:i32){
//         self.importance =importance;
//     }
// }

