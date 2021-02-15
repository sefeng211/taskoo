use taskoo_core::core::Operation;
use taskoo_core::command::{ContextCommand, TagCommand, SimpleCommand};
use taskoo_core::operation::Task;
use taskoo_core::operation::{execute, Get as GetOperation, ModifyOperation, DeleteOperation};

use crate::List;
use crate::error::ClientError;
use crate::option_parser::{CommandOption, parse_command_option};

use ini::Ini;
use clap::ArgMatches;
use log::{info};
use std::io;
use std::io::Write;
use yansi::{Color, Paint};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use dialoguer::console::Term;
use ctrlc;

pub struct Review {
    config: Ini,
}
impl Review {
    pub fn new(config: Ini) -> Review {
        Review { config: config }
    }

    pub fn review(&self, matches: &ArgMatches) -> Result<(), ClientError> {
        ctrlc::set_handler(move || {
            // If the user C-c'ed while running dialoguer, the
            // cursor will be gone. So here we reset terminal
            // to restore the cursor.
            let term = Term::stderr();
            term.show_cursor().unwrap();
        })
        .expect("Unable to set Ctrl-C handler");

        let (option, context_name) = match matches.values_of("arguments") {
            Some(arguments) => {
                let option = parse_command_option(&arguments.collect(), false, false, false)?;
                match option.context_name {
                    Some(ref context) => {
                        let cloned_context = context.clone();
                        (option, cloned_context)
                    }
                    None => (option, String::from("inbox")),
                }
            }
            None => (CommandOption::new(), String::from("inbox")),
        };
        let mut operations_tuple = List::get_operations(option, Some(vec![context_name]))?;
        assert_eq!(operations_tuple.len(), 1);
        let mut op_tuple = &mut operations_tuple[0];
        return Review::process_operation(&op_tuple.0, &mut op_tuple.1);
    }

    fn process_operation(
        context_name: &String,
        operation: &mut GetOperation,
    ) -> Result<(), ClientError> {
        execute(operation)?;

        let need_review_tasks = operation.get_result();
        if need_review_tasks.is_empty() {
            println!("のNothing to review!");
            return Ok(());
        }

        for task in need_review_tasks.iter() {
            // No need to review completed tasks
            if task.state_name == "completed" {
                continue;
            }
            Review::review_task(&task)?;
        }
        Ok(())
    }

    fn review_task(task: &Task) -> Result<(), ClientError> {
        // Display Info
        println!("");
        println!(
            "{} {} \n",
            Paint::new("Task:").fg(Color::Magenta).bold().to_string(),
            Paint::new(task.body.clone())
                .fg(Color::Yellow)
                .bold()
                .to_string()
        );
        let tt = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("q: Skip this task, y: start to review, n: delete the task")
            .interact_opt()
            .map_err(|error| ClientError::TerminalError())?;

        if let None = tt {
            return Ok(());
        } else if let Some(false) = tt {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Do you want to {} the task?", Paint::red("delete")))
                .interact()
                .map_err(|error| ClientError::TerminalError())?
            {
                let mut operation = DeleteOperation {
                    database_manager: None,
                    task_ids: vec![task.id],
                    result: None,
                };
                execute(&mut operation)?;
                return Ok(());
            } else {
                return Ok(());
            }
        }

        println!("");
        let context_command = ContextCommand::new()?;
        let context_names = context_command.get_all()?;

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a context (Press 'q' or 'Esc' to enter a new context)")
            .default(0)
            .items(&context_names)
            .interact_opt()
            .map_err(|error| ClientError::TerminalError())?;

        let new_context = if let Some(selection) = selection {
            Some(context_names[selection].clone())
        } else {
            Review::ask_attribute("New Context?: ")
        };

        let tag_command = TagCommand::new()?;
        let tags = tag_command.get_all()?;
        let mut defaults = vec![];
        for _ in tags.iter() {
            defaults.push(false);
        }
        let tag_selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a tag ")
            .items(&tags)
            .defaults(&defaults)
            .interact()
            .map_err(|error| ClientError::TerminalError())?;

        let mut new_tags: Vec<String> = vec![];
        for index in tag_selection {
            new_tags.push(tags[index].clone());
        }
        if new_tags.is_empty() {
            let user_provided_tags: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New Tags? (Use , to separate multiple tags)")
                .allow_empty(true)
                .interact()
                .map_err(|error| ClientError::TerminalError())?;

            new_tags = user_provided_tags
                .split(",")
                .map(|tag| tag.to_string())
                .collect();
        }

        let user_new_due_date: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("New Due Date?")
            .allow_empty(true)
            .interact()
            .map_err(|error| ClientError::TerminalError())?;

        let user_new_schedule_at: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("New Schedule At?")
            .allow_empty(true)
            .interact()
            .map_err(|error| ClientError::TerminalError())?;

        let mut new_due_date = None;
        let mut new_scheduled_at = None;

        if !user_new_due_date.is_empty() {
            new_due_date = Some(user_new_due_date);
        }

        if !user_new_schedule_at.is_empty() {
            new_scheduled_at = Some(user_new_schedule_at);
        }

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to modify the task as above?")
            .interact()
            .map_err(|error| ClientError::TerminalError())?
        {
            info!("Modify task {}", task.id);
            let mut modify_operation = ModifyOperation::new(vec![task.id]);
            modify_operation.context_name = new_context;
            modify_operation.due_date = new_due_date.as_deref();
            modify_operation.scheduled_at = new_scheduled_at.as_deref();
            modify_operation.tag_names = new_tags;

            execute(&mut modify_operation)?;
            println!("Task Modified!");
        } else {
        }
        Ok(())
    }

    fn ask_attribute(message: &str) -> Option<String> {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        input.pop();
        return if !input.is_empty() { Some(input) } else { None };
    }
}
