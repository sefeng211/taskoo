use anyhow::{Context, Result};
use clap::{App, Arg};
use dirs::config_dir;
use ini::Ini;
use log::info;
use std::fs::create_dir_all;
use std::fs::OpenOptions;

mod commands;
mod display;
mod extra;
mod option_parser;

use commands::add::Add;
use commands::delete::Delete;
use commands::done::Done;
use commands::info::Info;
use commands::list::List;
use commands::modify::Modify;
use commands::review::Review;
use commands::view::View;

fn get_config() -> Ini {
    let mut config_dir_path = config_dir().expect("Unable to find user's config directory");

    config_dir_path.push("taskoo");
    let mut config_file_path = config_dir_path.clone();

    config_file_path.push("cli_config");
    if !config_file_path.exists() {
        info!("Taskoo cli's config file doesn't exist");
        let _ =
            create_dir_all(&config_dir_path).expect("Unable to create taskoo's config directory");
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config_file_path)
            .expect("Unable to create taskoo cli's config file");
        // Create default configuration
        let mut conf = Ini::new();
        conf.with_section(None::<String>)
            .set("columns", "Id,Body,Tag,Created_At,Scheduled_At,State");
        conf.with_section(Some("Id"))
            .set("color", "Yellow")
            .set("bold", "false");
        conf.with_section(Some("Body"))
            .set("color", "Yellow")
            .set("bold", "true");
        conf.with_section(Some("Tag"))
            .set("color", "Yellow")
            .set("bold", "true");
        conf.with_section(Some("Created_At"))
            .set("color", "Yellow")
            .set("bold", "true");
        conf.with_section(Some("Scheduled_At"))
            .set("color", "Yellow")
            .set("bold", "true");
        conf.with_section(Some("Due"))
            .set("color", "Yellow")
            .set("bold", "true");

        conf.write_to_file(&config_file_path)
            .expect("Failed to write default configuation to the config file");
    }
    return Ini::load_from_file(config_file_path).unwrap();
}

fn main() -> Result<()> {
    env_logger::init();
    let matches = App::new("Taskoo")
        .subcommand(
            App::new("add")
                .about("add a task")
                .arg(Arg::new("config").multiple(true)),
        )
        .subcommand(
            App::new("list").alias("ls").about("List tasks").arg(
                Arg::new("arguments")
                    .about("Arguments")
                    .index(1)
                    .required(false)
                    .multiple(true),
            ),
        )
        .subcommand(App::new("review").about("Review tasks interactively"))
        .subcommand(
            App::new("modify").about("Modify task").arg(
                Arg::new("args")
                    .allow_hyphen_values(true) // This doesn't really work see: https://github.com/clap-rs/clap/issues/1437
                    .index(1)
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
                .arg(Arg::new("args").index(1).required(true).multiple(true)),
        )
        .subcommand(
            App::new("info")
                .about("Show detailed information about given task ids.")
                .arg(Arg::new("task_ids").index(1).required(true).multiple(true)),
        )
        .subcommand(
            App::new("done")
                .about("Change the state of a task/tasks to done")
                .arg(Arg::new("task_ids").index(1).required(true).multiple(true)),
        )
        .get_matches();

    if matches.is_present("add") {
        match Add::add(&matches.subcommand_matches("add").unwrap()).context("Add command") {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {
                println!("Task added");
            }
        }
    } else if matches.is_present("list") {
        let list_command = List::new(get_config());
        match list_command
            .list(&matches.subcommand_matches("list").unwrap())
            .context("List command failed")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {}
        }
    } else if matches.is_present("delete") {
        match Delete::delete(&matches.subcommand_matches("delete").unwrap())
            .context("Delete command failed")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {
                println!("Tasks deleted!");
            }
        }
    } else if matches.is_present("review") {
        match Review::review().context("Review command failed") {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {}
        }
    } else if matches.is_present("modify") {
        match Modify::modify(&matches.subcommand_matches("modify").unwrap())
            .context("Modify command failed")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {
                println!("Tasks modified!");
            }
        }
    } else if matches.is_present("view") {
        let view = View::new(get_config());
        match view
            .view(&matches.subcommand_matches("view").unwrap())
            .context("View command failed")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {}
        }
    } else if matches.is_present("done") {
        let done = Done::new();
        done.run(&matches.subcommand_matches("done").unwrap());
    } else if matches.is_present("info") {
        let info = Info::new();
        info.run(&matches.subcommand_matches("info").unwrap());
    }
    Ok(())
}
