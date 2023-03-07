#![feature(provide_any)]
#![feature(error_generic_member_access)]
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

use crate::error::ClientError;

fn get_config() -> Ini {
    match ProjectDirs::from("dev", "sefeng", "taskoo") {
        Some(dir) => {
            let mut dir_path = dir.config_dir().to_path_buf();
            dir_path.push("cli-conf.ini");
            if !dir_path.exists() {
                debug!("Unable to find the cli-confi.ini file from {:?}", dir_path);
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
            handle_result(
                List::new(get_config())
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
        Commands::Modify { arguments } => {
            handle_result(Modify::modify(arguments).context("Modify command failed"))
        }
        Commands::Delete { arguments } => {
            handle_result(Delete::delete(arguments).context("Delete command failed"));
        }
        Commands::Agenda { start_day, end_day } => handle_result(
            Agenda::new(get_config())
                .agenda(&start_day, &end_day)
                .context("Agenda command failed"),
        ),
        Commands::Info { task_id, attribute } => {
            let info = Info::new();
            handle_result(
                info.run(task_id, attribute)
                    .context("Failed to run <info> command"),
            );
        }
        Commands::Start { task_ids } => handle_result(
            StateChanger::to_started()
                .run(task_ids)
                .context("Failed to complete a task"),
        ),
        Commands::Done { task_ids } => handle_result(
            StateChanger::to_completed()
                .run(task_ids)
                .context("Failed to complete a task"),
        ),
        Commands::Ready { task_ids } => handle_result(
            StateChanger::to_ready()
                .run(task_ids)
                .context("Failed to complete a task"),
        ),
        Commands::Block { task_ids } => handle_result(
            StateChanger::to_blocked()
                .run(task_ids)
                .context("Failed to block the task"),
        ),
    }
    Ok(())
}
