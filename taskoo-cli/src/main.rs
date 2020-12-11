use clap::{App, Arg, SubCommand};
use log::{debug, error, info, log_enabled, Level};

mod commands;
mod display;
mod option_parser;

use commands::add::Add;
use commands::delete::Delete;
use commands::list::List;
use commands::modify::Modify;
use commands::review::Review;
use display::Display;

fn main() {
    env_logger::init();
    let matches = App::new("Taskoo")
        .subcommand(
            SubCommand::with_name("add")
                .about("add a task")
                .arg(Arg::with_name("config").multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("list").about("List tasks").arg(
                Arg::with_name("context_name")
                    .help("The name of the context")
                    .index(1)
                    .required(false),
            ),
        )
        .subcommand(SubCommand::with_name("review").about("Reivew tasks"))
        .subcommand(
            SubCommand::with_name("modify")
                .about("Modify task")
                .arg(
                    Arg::with_name("task_id")
                        .help("The id of the task that you want to modify")
                        .index(1)
                        .required(true),
                )
                .arg(Arg::with_name("options").required(true).multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("delete").about("delete tasks").arg(
                Arg::with_name("task_ids")
                    .index(1)
                    .required(true)
                    .multiple(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("view").about("view tasks").arg(
                Arg::with_name("arguments")
                    .index(1)
                    .required(true)
            ),
        )
        .get_matches();

    if matches.is_present("add") {
        Add::add(&matches.subcommand_matches("add").unwrap());
    } else if matches.is_present("list") {
        List::list(&matches.subcommand_matches("list").unwrap());
    } else if matches.is_present("delete") {
        Delete::delete(&matches.subcommand_matches("delete").unwrap());
    } else if matches.is_present("review") {
        Review::review();
    } else if matches.is_present("modify") {
        Modify::modify(&matches.subcommand_matches("modify").unwrap());
    }
}
