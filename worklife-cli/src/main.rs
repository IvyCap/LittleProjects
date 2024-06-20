use crate::args::*;
use crate::logic::*;
use crate::parser::*;

mod args;
mod logic;
mod parser;

fn main() {
    let matches: clap::ArgMatches = cmd_args();

    match matches.subcommand_name() {
        Some("task") => task_cmd(),
        Some("review") => review_cmd(),
        _ => default_cmd(),
    };
}

fn default_cmd() {
    let task_data = parse_task_data();

    app_title();

    let task_times: Vec<(String, f32)> = get_times(task_data);

    print_tasks_percent(&task_times);

    save_task_time(task_times);
}

fn task_cmd() {
    println!("Task command")
}

fn review_cmd() {
    println!("Review command")
}
