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
use commands::agenda::Agenda;
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

fn handle_result(result: Result<String, anyhow::Error>) {
    match result {
        Err(e) => {
            eprintln!("{:?}", e);
        }
        Ok(message) => {
            println!("{}", message);
        }
    }
}

fn main() -> Result<(), ClientError> {
    env_logger::init();
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(add_args) = matches.subcommand_matches("add") {
        handle_result(Add::add(add_args).context("Add command"));
    } else if let Some(list_args) = matches.subcommand_matches("list") {
        let list_command = List::new(get_config());
        handle_result(list_command.list(list_args).context("List command failed"));
    } else if let Some(delete_args) = matches.subcommand_matches("delete") {
        handle_result(Delete::delete(delete_args).context("Delete command failed"));
    } else if let Some(review_args) = matches.subcommand_matches("review") {
        let review_command = Review::new(get_config());
        handle_result(
            review_command
                .review(review_args)
                .context("Review command failed"),
        );
    } else if let Some(modify_args) = matches.subcommand_matches("modify") {
        handle_result(Modify::modify(modify_args).context("Modify command failed"));
    } else if let Some(agenda_args) = matches.subcommand_matches("agenda") {
        let agenda = Agenda::new(get_config());
        handle_result(agenda.agenda(agenda_args).context("Agenda command failed"));
    } else if let Some(done_args) = matches.subcommand_matches("done") {
        let state_changer = StateChanger::to_completed();
        handle_result(
            state_changer
                .run(done_args)
                .context("Failed to complete a task"),
        );
    } else if let Some(ready_args) = matches.subcommand_matches("ready") {
        let state_changer = StateChanger::to_ready();
        handle_result(
            state_changer
                .run(ready_args)
                .context("Failed to change the state to ready"),
        );
    } else if let Some(block_args) = matches.subcommand_matches("block") {
        let state_changer = StateChanger::to_blocked();
        handle_result(
            state_changer
                .run(block_args)
                .context("Failed to change the state to block"),
        );
    } else if let Some(info_args) = matches.subcommand_matches("info") {
        let info = Info::new();
        handle_result(info.run(info_args).context("Failed to run <info> command"));
    } else if let Some(start_args) = matches.subcommand_matches("start") {
        let state_changer = StateChanger::to_started();
        handle_result(
            state_changer
                .run(start_args)
                .context("Failed to change the state to start"),
        );
    } else if let Some(annotate_args) = matches.subcommand_matches("annotate") {
        handle_result(
            Add::add_annoation(annotate_args).context("Failed to run <annotate> command"),
        );
    } else if let Some(clean_args) = matches.subcommand_matches("clean") {
        handle_result(Clean::clean(clean_args).context("Failed to run <clean> command"));
    }
    Ok(())
}
