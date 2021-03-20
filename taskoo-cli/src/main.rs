#![feature(backtrace)]

use anyhow::{Context, Result};
use clap::{App, load_yaml};
use dirs::config_dir;
use ini::Ini;
use log::{info, debug};
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use directories::ProjectDirs;
use std::backtrace::Backtrace;

use commands::add::Add;
use commands::delete::Delete;
use commands::state_changer::StateChanger;
use commands::info::Info;
use commands::list::List;
use commands::modify::Modify;
use commands::review::Review;
use commands::view::View;
use commands::clean::Clean;

mod commands;
mod display;
mod error;
mod extra;
mod option_parser;

use crate::error::ClientError;

fn get_config() -> Ini {
    match ProjectDirs::from("dev", "sefeng", "taskoo") {
        Some(dir) => {
            let mut dir_path = dir.config_dir().to_path_buf();
            dir_path.push("cli-conf.ini");
            if !dir_path.exists() {
                debug!("Unable to find the cli-confi.ini file");
                return Ini::new();
            }
            debug!("Successfully loaded the config file");
            return Ini::load_from_file(dir_path).unwrap();
        }
        None => {
            debug!("Unable to find the directory pat");
        }
    }
    debug!("Unable to loaded the config file, use a empty object");
    return Ini::new();
}

fn main() -> Result<(), ClientError> {
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
                println!("{}, state has changed to {}", message, "done")
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
