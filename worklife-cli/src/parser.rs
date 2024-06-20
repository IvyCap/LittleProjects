use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub tasks: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRecords {
    pub times: Vec<(String, f32)>,
}

#[derive(PartialEq)]
pub enum FileType {
    // file contating the task short and long names
    TaskData,
    // file containing historic task name and time
    TaskTime,
}

const TASKPATH: &str = "./taskData.json";
const TASKTIMEPATH: &str = "./taskTimeData.json";

//Temp data to test with until json parser is working
// pub fn parse_task_data() -> Vec<(&'static str, &'static str)>{
// let task_list = vec![("Sleep", "Sleep"), ("Walk", "Stupid mental health walk")];
// task_list
// }

pub fn parse_task_data() -> Vec<(String, String)> {
    _ = does_file_exist(TASKPATH);

    let tasks_json: String = open_file(TASKPATH).unwrap();

    let v: Tasks = json_to_struct_tasks(tasks_json.as_str()).unwrap();

    let mut task_list: Vec<(String, String)> = vec![];

    for task in v.tasks {
        // println!("{:?}", task);
        task_list.push(task)
    }

    task_list
}

pub fn save_task_time(task_times: Vec<(String, f32)>) -> std::io::Result<()> {
    _ = does_file_exist(TASKTIMEPATH);

    _ = write_save_file(task_times);

    Ok(())
}

fn write_save_file(task_times: Vec<(String, f32)>) -> std::io::Result<()> {
    let mut file = open_file(TASKTIMEPATH).unwrap();
    let saved_struct: TaskRecords = json_to_struct_task_records(&file.as_str()).unwrap();
    // dbg!("Write File Path: {}", &file);
    // dbg!("Saved Struct: {}", &saved_struct);

    let mut saved_task_time: Vec<(String, f32)> = vec![];

    for time in saved_struct.times {
        saved_task_time.push(time)
    }

    // dbg!("Saved Task Times: {}", &saved_task_time);

    for task_time in task_times {
        saved_task_time.push(task_time);
    }

    // dbg!("Updated Saved Task Times: {}", &saved_task_time);

    let updated_struct: TaskRecords = TaskRecords {
        times: saved_task_time,
    };

    // dbg!("Updated Struct: {}", &updated_struct);

    let new_json = struct_task_records_to_json(updated_struct).unwrap();

    let mut write_file = create_file(TASKTIMEPATH);

    write_file.write_all(new_json.as_bytes())?;

    Ok(())
}

fn write_task_data(tasks_data: Vec<(String, String)>) -> std::io::Result<()> {
    let mut file: Tasks = json_to_struct_tasks(open_file(TASKPATH).unwrap().as_str()).unwrap();

    // dbg!("Write Read: {}", &file);

    for task_data in tasks_data {
        file.tasks.push(task_data);
    }

    // f.write_all(b"test text")?;

    Ok(())
}

fn json_to_struct_tasks(tasks: &str) -> Result<Tasks> {
    let v: Tasks = serde_json::from_str(tasks)?;
    Ok(v)
}

fn json_to_struct_task_records(tasks: &str) -> Result<TaskRecords> {
    let v: TaskRecords = serde_json::from_str(tasks)?;
    Ok(v)
}

fn struct_tasks_to_json(struct_t: Tasks) -> Result<String> {
    let v: String = serde_json::to_string(&struct_t)?;
    Ok(v)
}

fn struct_task_records_to_json(struct_tr: TaskRecords) -> Result<String> {
    let v: String = serde_json::to_string(&struct_tr)?;
    Ok(v)
}

fn does_file_exist(file_path: &str) -> std::io::Result<String> {
    let path = Path::new(file_path);

    if !Path::exists(path) {
        _ = create_file(file_path);
    };

    let file = open_file(file_path);

    file
}

fn create_file(file_path: &str) -> File {
    let mut file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}", &file_path, why),
        Ok(file) => file,
    };

    file
}

fn open_file(path: &str) -> std::io::Result<String> {
    let file = File::open(path);

    let mut file = match file {
        Ok(f) => f,
        Err(error) => {
            panic!("Error: {:?}", error)
        }
    };

    // dbg!("File: {:?}", &file);

    let mut contents = String::new();
    let contents = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(error) => {
            panic!("Error: {:?}", error)
        }
    };

    // dbg!("Contents: {}", &contents);

    Ok(contents)
}
