#[warn(unused_imports)]

use clap::Parser;

use crate::logic::*;
use crate::parser::*;

mod logic;
mod parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
//Todo: Add edit arg to edit task_data json file with text editor
//Todo: Add args to get task percetages for the week, month, year, ytd from historic task_data json file

}



fn main() {

    //This allows Clap to run with help and version args without needing to configure or use other arguments 
    let _args = Args::parse();

    let task_data = parse_task_data();   

    app_title();

    let task_times = get_times(task_data);    

    print_tasks_percent(task_times);

}



