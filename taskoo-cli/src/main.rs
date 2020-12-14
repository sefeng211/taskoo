use clap::{App, Arg};
use dirs::config_dir;
use ini::Ini;
use log::info;
use std::fs::create_dir_all;
use std::fs::OpenOptions;

mod commands;
mod option_parser;

use commands::add::Add;
use commands::delete::Delete;
use commands::list::List;
use commands::modify::Modify;
use commands::review::Review;

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
            .set("columns", "Id,Body,Tag,Created_At,Scheduled_At");
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

        conf.write_to_file(&config_file_path)
            .expect("Failed to write default configuation to the config file");
    }
    return Ini::load_from_file(config_file_path).unwrap();
}

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
                Arg::new("arguments")
                    .about("Arguments")
                    .index(1)
                    .required(false)
                    .multiple(true),
            ),
        )
        .subcommand(App::new("review").about("Reivew tasks"))
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
                .arg(Arg::new("arguments").index(1).required(true)),
        )
        .get_matches();

    if matches.is_present("add") {
        Add::add(&matches.subcommand_matches("add").unwrap());
    } else if matches.is_present("list") {
        List::list(&matches.subcommand_matches("list").unwrap(), get_config());
    } else if matches.is_present("delete") {
        Delete::delete(&matches.subcommand_matches("delete").unwrap());
    } else if matches.is_present("review") {
        Review::review();
    } else if matches.is_present("modify") {
        Modify::modify(&matches.subcommand_matches("modify").unwrap());
    }
}
