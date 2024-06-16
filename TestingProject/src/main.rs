#[warn(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

const TASK_PATH: &str = "./test.txt";



fn main() {
    
    _ = does_file_exist();

    // let _ = create_task_file();

    let test = open_task_file(TASK_PATH).unwrap();

    println!("Test: {}", test);
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