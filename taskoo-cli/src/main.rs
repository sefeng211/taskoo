#![feature(backtrace)]

use anyhow::{Context, Result};
use clap::{App, Subcommand};
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

// Note: this requires the `derive` feature
use clap::Parser;
#[derive(Parser)]
#[clap(name = "Taskoo")]
#[clap(author = "Sean Feng. <sean@seanfeng.dev>")]
#[clap(version = "1.0")]
#[clap(
    about = "
    A CLI task management app written in Rust,
    with GTD in mind.",
    long_about = ""
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add {
        #[clap(short, long)]
        annotation: bool,
        arguments: Vec<String>,
    },

    /// Show tasks
    List {
        #[clap(short, long)]
        all: bool,
        /// Apply filters to the search query
        arguments: Vec<String>,
    },
    Review {
        /// Apply filters to the search query
        arguments: Vec<String>,
    },
    Modify {
        /// Apply filters to the search query
        arguments: Vec<String>,
    },
    Delete {
        /// Apply filters to the search query
        arguments: Vec<String>,
    },
    Agenda {
        /// Start day
        start_day: String,
        end_day: Option<String>,
    },
    /// Show information about the given task
    Info {
        task_id: u64,
        attribute: Option<String>,
    },
    /// Change the state of the given tasks to 'start'
    Start { task_ids: Vec<u64> },
    /// Change the state of the given tasks to 'done'
    Done { task_ids: Vec<u64> },
    /// Change the state of the given tasks to 'ready'
    Ready { task_ids: Vec<u64> },
    /// Change the state of the given tasks to 'block'
    Block { task_ids: Vec<u64> },
}

fn main() -> Result<(), ClientError> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add {
            annotation,
            arguments,
        } => {
            handle_result(Add::add(annotation, arguments).context("Add command"));
        }
        Commands::List { all, arguments } => {
            let list_command = List::new(get_config());
            handle_result(
                list_command
                    .list(all.to_owned(), arguments)
                    .context("List command failed"),
            );
        }
        Commands::Review { arguments } => {
            let review_command = Review::new(get_config());
            handle_result(
                review_command
                    .review(arguments)
                    .context("Review command failed"),
            );
        }
        Commands::Modify { arguments } => {}
        Commands::Delete { arguments } => {
            handle_result(Delete::delete(arguments).context("Delete command failed"));
        }
        Commands::Agenda { start_day, end_day } => {
            let agenda = Agenda::new(get_config());
            handle_result(
                agenda
                    .agenda(&start_day, &end_day)
                    .context("Agenda command failed"),
            );
        }
        Commands::Info { task_id, attribute } => {
            let info = Info::new();
            handle_result(
                info.run(task_id, attribute)
                    .context("Failed to run <info> command"),
            );
        }
        Commands::Start { task_ids } => {}
        Commands::Done { task_ids } => {
            let state_changer = StateChanger::to_completed();
            handle_result(
                state_changer
                    .run(task_ids)
                    .context("Failed to complete a task"),
            )
        }
        Commands::Ready { task_ids } => {}
        Commands::Block { task_ids } => {}
    }
    //let yaml = load_yaml!("../config/cli.yml");
    //let matches = App::from(yaml).get_matches();

    // if let Some(add_args) = matches.subcommand_matches("add") {
    // } else if let Some(list_args) = matches.subcommand_matches("list") {
    //     let list_command = List::new(get_config());
    //     handle_result(list_command.list(list_args).context("List command failed"));
    // } else if let Some(delete_args) = matches.subcommand_matches("delete") {
    // } else if let Some(review_args) = matches.subcommand_matches("review") {
    // } else if let Some(modify_args) = matches.subcommand_matches("modify") {
    //     handle_result(Modify::modify(modify_args).context("Modify command failed"));
    // } else if let Some(agenda_args) = matches.subcommand_matches("agenda") {
    //     let agenda = Agenda::new(get_config());
    //     handle_result(agenda.agenda(agenda_args).context("Agenda command failed"));
    // } else if let Some(done_args) = matches.subcommand_matches("done") {
    //     let state_changer = StateChanger::to_completed();
    //     handle_result(
    //         state_changer
    //             .run(done_args)
    //             .context("Failed to complete a task"),
    //     );
    // } else if let Some(ready_args) = matches.subcommand_matches("ready") {
    //     let state_changer = StateChanger::to_ready();
    //     handle_result(
    //         state_changer
    //             .run(ready_args)
    //             .context("Failed to change the state to ready"),
    //     );
    // } else if let Some(block_args) = matches.subcommand_matches("block") {
    //     let state_changer = StateChanger::to_blocked();
    //     handle_result(
    //         state_changer
    //             .run(block_args)
    //             .context("Failed to change the state to block"),
    //     );
    // } else if let Some(info_args) = matches.subcommand_matches("info") {
    //     let info = Info::new();
    //     handle_result(info.run(info_args).context("Failed to run <info> command"));
    // } else if let Some(start_args) = matches.subcommand_matches("start") {
    //     let state_changer = StateChanger::to_started();
    //     handle_result(
    //         state_changer
    //             .run(start_args)
    //             .context("Failed to change the state to start"),
    //     );
    // } else if let Some(annotate_args) = matches.subcommand_matches("annotate") {
    //     handle_result(
    //         Add::add_annoation(annotate_args).context("Failed to run <annotate> command"),
    //     );
    // } else if let Some(clean_args) = matches.subcommand_matches("clean") {
    //     handle_result(Clean::clean(clean_args).context("Failed to run <clean> command"));
    // }
    Ok(())
}
