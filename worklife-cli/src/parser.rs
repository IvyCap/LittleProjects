#[warn(unused_imports)]

use std::fs::File;
use std::io::prelude::*;


// TODO: Add serde to serilze and ddeserialize JSON files with task data. 


// const SLEEP: &str = "Sleep";
// const WALK: &str = "Stupid Mental Health Walk";
// const EXCER: &str = "Excersice";
// const HOBBY: &str = "Hobbies";
// const LEARN: &str = "Learn/Study";
// const RELAX: &str = "Relax";
// const WORKSELF: &str = "Work for self";
// const WORKCAP: &str = "Work for the capatilists";

// pub struct  tasks {
//     short_name: &str,
//     long_name: &str,
//     records: Vec<time_record>,
// }

// pub struct time_record {
//     hours: f32,
//     date: f32,
//}

//Temp data to test with until json parser is working
pub fn parse_task_data() -> Vec<(&'static str, &'static str)>{ 
let task_list = vec![("Sleep", "Sleep"), ("Walk", "Stupid mental health walk")];
task_list
}

// fn create_task_file() -> std::io::Result<()> {
//     let mut file = File::create("wlb-task.json")?;
//     Ok((file))
// }

// fn open_task_file() -> std::io::Result<()> {
//     let mut file = File::open("wlb-task.json")?;
//     match file {
//         Ok(x) => file,
//         Err => file = create_task_file(), 
//     }

//     Ok((file))
// }


// fn save_task_file(task_file: Vev<tasks>) -> std::io::Result<()> {
//     let mut f = File::create_new("wlb-task.json")?;
//     f.write_all(task_file)?;
//     Ok(())
// }


