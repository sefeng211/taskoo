use clap::{App, Arg};
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
            App::new("add")
                .about("add a task")
                .arg(Arg::new("config").multiple(true)),
        )
        .subcommand(
            App::new("list").about("List tasks").arg(
                Arg::new("context_name")
                    .about("The name of the context")
                    .index(1)
                    .required(false),
            ),
        )
        .subcommand(App::new("review").about("Reivew tasks"))
        .subcommand(
            App::new("modify")
                .about("Modify task")
                .arg(
                    Arg::new("task_id")
                        .about("The id of the task that you want to modify")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("options")
                        .allow_hyphen_values(true) // This doesn't really work see: https://github.com/clap-rs/clap/issues/1437
                        .index(2)
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("delete tasks")
                .arg(Arg::new("task_ids").index(1).required(true).multiple(true)),
        )
        .subcommand(
            App::new("view")
                .about("view tasks")
                .arg(Arg::new("arguments").index(1).required(true)),
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
