use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct  Tasks{
    tasks: Vec<(String, String)>,
}

// #[derive(Serialize, Deserialize)]
// pub struct  Task<'a> {
//     short_name: &'a str,
//     long_name: &'a str,
// }

// #[derive(Serialize, Deserialize)]
// pub struct RecordByDate {
//     date: f32,
//     task_records: Vec<TaskRecord>
// }

// #[derive(Serialize, Deserialize)]
// pub struct TaskRecord {
//     short_name: &str
//     hours: f32,

const TASK_PATH: &str = "./test.json";



fn main() -> Result<()>{
    
    _ = does_file_exist();

    let test = open_task_file(TASK_PATH).unwrap();

    // let tasks = Tasks::new(test);

    let v: Tasks = serde_json::from_str(&test)?;

    println!("");
    println!("");
    // println!("Test: {}", v);


    for task in v.tasks {
        println!("{:?}", task);
            }

    Ok(())
}


fn does_file_exist() -> std::io::Result<()> {
    let path = Path::new(TASK_PATH);

    if !Path::exists(path){
        _ = create_task_file();
    };

    Ok(())
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