use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct  Tasks{
    pub tasks: Vec<(String, String)>,
}


// TODO: Add serde to serilze and deserialize JSON files with task data. 

// pub struct  Tasks {
//     short_name: &str,
//     long_name: &str,
// 

// pub struct RecordByDate {
//     date: f32,
//     task_records: Vec<TaskRecord>
// }

// pub struct TaskRecord {
//     short_name: &str
//     hours: f32,

const TASK_PATH: &str = "./taskData.json";


//Temp data to test with until json parser is working
// pub fn parse_task_data() -> Vec<(&'static str, &'static str)>{ 
// let task_list = vec![("Sleep", "Sleep"), ("Walk", "Stupid mental health walk")];
// task_list
// }

pub fn parse_task_data() -> Vec<(String, String)>{ 
    
    _ = does_file_exist();

    let tasks_json: String= open_task_file(TASK_PATH).unwrap();

    let v: Tasks = json_to_struc_tasks(tasks_json.as_str()).unwrap();

    let mut task_list: Vec<(String, String)> = vec!();

    for task in v.tasks {
        println!("{:?}", task);
        task_list.push(task)
    }


    task_list
    }

fn json_to_struc_tasks(tasks: &str) -> Result<Tasks>{
    let v: Tasks = serde_json::from_str(tasks)?;
    Ok(v)
}

fn does_file_exist() -> std::io::Result<String> {
    let path = Path::new(TASK_PATH);

    if !Path::exists(path){
        _ = create_task_file();
    };

    let file = open_task_file(TASK_PATH);

    file
}

fn create_task_file() -> std::io::Result<()> {
    let file = File::create(TASK_PATH);

    match file {
        Ok(f) => f,
        Err(error) => {panic!("Error: {:?}", error)}, 
    };

    Ok(())
}

fn open_task_file(path: &str) -> std::io::Result<String> {
    let file = File::open(path);

    let mut file = match file {
        Ok(f) => f,
        Err(error) => {panic!("Error: {:?}", error)}, 
    };

    dbg!("File: {:?}", &file);

    let mut contents = String::new();
    let contents =  match file.read_to_string(&mut contents){
        Ok(_) => contents,
        Err(error) => {panic!("Error: {:?}", error)}, 
    };

    dbg!("Contents: {}", &contents);

    Ok(contents)
}


fn save_task_file(path: &str) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    f.write_all(b"test text")?;
    Ok(())
}
