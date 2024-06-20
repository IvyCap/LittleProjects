use clap::{arg, command, ArgMatches, Command};

pub fn cmd_args() -> ArgMatches {
    let matches: ArgMatches = command!()
        .subcommand(
            Command::new("task")
                .about("Modify or review task lists")
                .arg(arg!(-a --add "Add task to task list")),
        )
        .subcommand(
            Command::new("review")
                .about("Review time spent on tasks")
                .arg(arg!(-w --week "Review task data over the current week")),
        )
        .get_matches();

    matches
}

// //Todo: Add edit arg to edit task_data json file with text editor

// //Todo: Add args to get task percetages for the week, month, year, ytd from historic task_data json file

// //Todo: Add args to add task and review task lists
