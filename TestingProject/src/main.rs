
use std::fs::File;
use std::io::prelude::*;

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


    // let _ = create_task_file();

    let test = open_task_file(TASK_PATH).unwrap();

    println!("{}", test);
}


fn create_task_file() -> std::io::Result<File> {
    let file = File::create("wlb-task.json")?;
    Ok(file)
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