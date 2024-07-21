#![feature(provide_any)]
#![feature(error_generic_member_access)]

use anyhow::{Context, Result};
use clap::{Subcommand};
use ini::Ini;
use log::{info, debug};
use directories::ProjectDirs;

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
    debug!("Unable to load the config file, use a empty object");
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
#[clap(version = "0.1")]
#[clap(
    about = "
    The CLI interface of Taskoo, written in Rust.",
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
    /// Clean context, tag or state
    Clean {
        provided_type: String
    },
    /// Change the state of the given tasks to 'start'
    Start { task_ids: Vec<u64> },
    /// Change the state of the given tasks to 'complete'
    Complete { task_ids: Vec<u64> },
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
            handle_result(Add::add(annotation, arguments).context("add command failed to operate"));
        }
        Commands::List { all, arguments } => {
            handle_result(
                List::new(get_config())
                    .list(all.to_owned(), arguments)
                    .context("list command failed to operate"),
            );
        }
        Commands::Review { arguments } => {
            let review_command = Review::new(get_config());
            handle_result(
                review_command
                    .review(arguments)
                    .context("review command failed to operate"),
            );
        }
        Commands::Modify { arguments } => {
            handle_result(Modify::modify(arguments).context("modify command failed to operate"))
        }
        Commands::Delete { arguments } => {
            handle_result(Delete::delete(arguments).context("delete command failed to operate"));
        }
        Commands::Agenda { start_day, end_day } => handle_result(
            Agenda::new(get_config())
                .agenda(&start_day, &end_day)
                .context("agenda command failed to operate"),
        ),
        Commands::Info { task_id, attribute } => {
            let info = Info::new();
            handle_result(
                info.run(task_id, attribute)
                    .context("info command failed to operate"),
            );
        },
        Commands::Clean { provided_type } => {
            handle_result(
                Clean::run(provided_type)
                    .context("clean command failed to operate"),
            );
        }
        Commands::Start { task_ids } => handle_result(
            StateChanger::to_started()
                .run(task_ids)
                .context("start command failed to operate"),
        ),
        Commands::Complete { task_ids } => handle_result(
            StateChanger::to_completed()
                .run(task_ids)
                .context("complete command failed to operate"),
        ),
        Commands::Ready { task_ids } => handle_result(
            StateChanger::to_ready()
                .run(task_ids)
                .context("ready command failed to operate"),
        ),
        Commands::Block { task_ids } => handle_result(
            StateChanger::to_blocked()
                .run(task_ids)
                .context("block command failed to operate"),
        ),
    }
    Ok(())
}
