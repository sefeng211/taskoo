use taskoo_core::core::Operation;
use taskoo_core::command::{ContextCommand, TagCommand, SimpleCommand};
use taskoo_core::operation::{Task, execute, Get as GetOperation, ModifyOperation, DeleteOperation};
use taskoo_core::option_parser::{CommandOption, parse_command_option};

use crate::List;
use crate::error::ClientError;
use crate::display::{Display, get_output_columns};

use ini::Ini;
use clap::ArgMatches;
use log::{info};
use std::io;
use std::io::Write;
use yansi::{Color, Paint};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, FuzzySelect};
use dialoguer::console::Term;
use ctrlc;

pub struct Review {
    config: Ini,
}
impl Review {
    pub fn new(config: Ini) -> Review {
        Review { config: config }
    }

    pub fn review(&self, matches: &Vec<String>) -> Result<String, ClientError> {
        ctrlc::set_handler(move || {
            // If the user C-c'ed while running dialoguer, the
            // cursor will be gone. So here we reset terminal
            // to restore the cursor.
            let term = Term::stderr();
            term.show_cursor().unwrap();
        })
        .expect("Unable to set Ctrl-C handler");

        let (option, context_name) = if !matches.is_empty() {
            let v2: Vec<&str> = matches.iter().map(|s| &**s).collect();
            let option = parse_command_option(&v2, false, false, false)?;
            match option.context {
                Some(ref context) => {
                    let cloned_context = context.clone();
                    (option, cloned_context)
                }
                None => (option, String::from("inbox")),
            }
        } else {
            (CommandOption::new(), String::from("inbox"))
        };

        let mut operations_tuple = List::get_operations(option, Some(vec![context_name]))?;
        assert_eq!(operations_tuple.len(), 1);
        let mut op_tuple = &mut operations_tuple[0];
        match self.process_operation(&op_tuple.0, &mut op_tuple.1) {
            Ok(()) => {
                return Ok(String::new());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn process_operation(
        &self,
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
            if !task.is_completed() {
                self.review_task(&task)?;
            }
        }
        Ok(())
    }

    fn review_task(&self, task: &Task) -> Result<(), ClientError> {
        // Display Info
        let output = Display::get_tabbed_output_for_tasks(&vec![&task], &self.config);

        let mut final_tabbed_string = String::from(&Display::get_formatted_row_for_header(
            get_output_columns(),
            &self.config,
        ));

        final_tabbed_string.push_str(&output);
        Display::print(&final_tabbed_string);
        let tt = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "{}: {} this task, {}: start to {}, {}: {} the task",
                Paint::yellow("<q>"),
                Paint::yellow("Skip").bold(),
                Paint::yellow("<y>"),
                Paint::yellow("review").bold(),
                Paint::yellow("<n>"),
                Paint::yellow("Delete").bold()
            ))
            .interact_opt()
            .map_err(|error| ClientError::TerminalError { source: error })?;

        if let None = tt {
            return Ok(());
        } else if let Some(false) = tt {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Do you want to {} the task?", Paint::red("delete")))
                .interact()
                .map_err(|error| ClientError::TerminalError { source: error })?
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
        let mut context_command = ContextCommand::new()?;
        let context_names = context_command.get_all()?;

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a context (Press 'q' or 'Esc' to enter a new context)")
            .default(0)
            .items(&context_names)
            .interact_opt()
            .map_err(|error| ClientError::TerminalError { source: error })?;

        let new_context = if let Some(selection) = selection {
            Some(context_names[selection].clone())
        } else {
            Review::ask_attribute("New Context?: ")
        };

        let mut tag_command = TagCommand::new()?;
        let tags = tag_command.get_all()?;
        let mut defaults = vec![];
        for _ in tags.iter() {
            defaults.push(false);
        }
        let tag_selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select tags")
            .items(&tags)
            .defaults(&defaults)
            .interact()
            .map_err(|error| ClientError::TerminalError { source: error })?;

        let mut new_tags: Vec<String> = vec![];
        for index in tag_selection {
            new_tags.push(tags[index].clone());
        }
        if new_tags.is_empty() {
            let user_provided_tags: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New Tags? (Use , to separate multiple tags)")
                .allow_empty(true)
                .interact()
                .map_err(|error| ClientError::TerminalError { source: error })?;

            new_tags = user_provided_tags
                .split(",")
                .map(|tag| tag.to_string())
                .collect();
        }

        let user_new_due_date: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("New Due Date?")
            .allow_empty(true)
            .interact()
            .map_err(|error| ClientError::TerminalError { source: error })?;

        let user_new_schedule_at: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("New Schedule At?")
            .allow_empty(true)
            .interact()
            .map_err(|error| ClientError::TerminalError { source: error })?;

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
            .map_err(|error| ClientError::TerminalError { source: error })?
        {
            info!("Modify task {}", task.id);
            let mut modify_operation = ModifyOperation::new(vec![task.id]);
            modify_operation.context_name = new_context;
            modify_operation.due_date = new_due_date.as_deref();
            modify_operation.scheduled_at = new_scheduled_at.as_deref();
            modify_operation.tag_names = new_tags;

            execute(&mut modify_operation)?;
            println!("Task Modified!");
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
