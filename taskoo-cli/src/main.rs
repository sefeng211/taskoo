#![feature(backtrace)]

use anyhow::{Context, Result};
use clap::{App, load_yaml};
use dirs::config_dir;
use ini::Ini;
use log::info;
use std::fs::create_dir_all;
use std::fs::OpenOptions;

mod commands;
mod display;
mod error;
mod extra;
mod option_parser;

use commands::add::Add;
use commands::delete::Delete;
use commands::state_changer::StateChanger;
use commands::info::Info;
use commands::list::List;
use commands::modify::Modify;
use commands::review::Review;
use commands::view::View;
use commands::clean::Clean;

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
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("add") {
        match Add::add(&matches.subcommand_matches("add").unwrap()).context("Add command") {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("{}", message);
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
        let review_command = Review::new(get_config());
        match review_command
            .review(&matches.subcommand_matches("review").unwrap())
            .context("Review command failed")
        {
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
        // TODO: Task state should not be hard-coded
        let state_changer = StateChanger::to_completed();
        match state_changer
            .run(&matches.subcommand_matches("done").unwrap())
            .context("Failed to complete a task")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("Tasks: {}, state changed to {}", message, "done")
            }
        }
    } else if matches.is_present("ready") {
        // TODO: Task state should not be hard-coded
        let state_changer = StateChanger::to_ready();
        match state_changer
            .run(&matches.subcommand_matches("ready").unwrap())
            .context("Failed to complete a task")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("Tasks: {}, state changed to {}", message, "ready")
            }
        }
    } else if matches.is_present("block") {
        // TODO: Task state should not be hard-coded
        let state_changer = StateChanger::to_blocked();
        match state_changer
            .run(&matches.subcommand_matches("block").unwrap())
            .context("Failed to complete a task")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("Tasks: {}, state changed to {}", message, "block")
            }
        }
    } else if matches.is_present("info") {
        let info = Info::new();
        match info
            .run(&matches.subcommand_matches("info").unwrap())
            .context("Failed to run <info> command")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(()) => {}
        }
    } else if matches.is_present("start") {
        let state_changer = StateChanger::to_started();
        match state_changer
            .run(&matches.subcommand_matches("start").unwrap())
            .context("Failed to run start command")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("Tasks: {}, state changed to {}", message, "start")
            }
        }
    } else if matches.is_present("annotate") {
        match Add::add_annoation(&matches.subcommand_matches("annotate").unwrap())
            .context("Failed to run <annotate> command")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("{}", message);
            }
        }
    } else if matches.is_present("clean") {
        match Clean::clean(&matches.subcommand_matches("clean").unwrap())
            .context("Failed to run <clean> command")
        {
            Err(e) => {
                eprintln!("{:?}", e);
            }
            Ok(message) => {
                println!("{}", message);
            }
        }
    }
    Ok(())
}
