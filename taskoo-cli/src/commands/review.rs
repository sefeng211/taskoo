use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

use taskoo_core::command::Command;
use taskoo_core::error::TaskooError;

use log::{debug, info};
use std::io;
use std::io::Write;
use taskoo_core::operation::{execute, Get as GetOperation, ModifyOperation};
use yansi::Color;
//use std::io::*;
use yansi::Paint;

pub struct Review;
impl Review {
    pub fn review() -> Result<(), TaskooError> {
        let mut operation = GetOperation::new();
        operation.context_name = Some("Inbox".to_string());

        execute(&mut operation)?;

        let need_review_tasks = operation.result;

        if need_review_tasks.len() == 0 {
            println!("There's nothing to review!");
            return Ok(());
        }
        // TODO: This is too simple, ideally we want to have auto-completion
        for task in need_review_tasks.iter() {
            // Display Info
            println!("");
            println!(
                "{} {}",
                Paint::new("Task:").fg(Color::Magenta).bold().to_string(),
                Paint::new(task.body.clone()).fg(Color::Yellow).bold().to_string()
            );

            println!("");
            let context_names = Command::get_context()?;

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a context (Press 'q' or 'Esc' to enter a new context)")
                .default(0)
                .items(&context_names)
                .interact_opt()
                .unwrap();

            let new_context;

            if let Some(selection) = selection {
                new_context = Some(context_names[selection].clone());
            } else {
                new_context = Review::ask_attribute("New Context?: ");
            }

            let tags = Command::tags(None)?;
            let mut defaults = vec![];
            for _ in tags.iter() {
                defaults.push(false);
            }
            let tag_selection = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a tag ")
                .items(&tags)
                .defaults(&defaults)
                .interact()
                .unwrap();

            let mut new_tags: Vec<String> = vec![];
            for index in tag_selection {
                new_tags.push(tags[index].clone());
            }
            if new_tags.is_empty() {
                let user_provided_tags: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("New Tags? (Use , to separate multiple tags)")
                    .allow_empty(true)
                    .interact()
                    .expect("Failed to get input");

                new_tags = user_provided_tags
                    .split(",")
                    .map(|tag| tag.to_string())
                    .collect();
            }
            let user_new_due_date: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New Due Date?")
                .allow_empty(true)
                .interact()
                .expect("Failed to get input");

            let user_new_schedule_at: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New Schedule At?")
                .allow_empty(true)
                .interact()
                .expect("Failed to get input");

            let mut new_due_date = None;
            let mut new_scheduled_at = None;

            if !user_new_due_date.is_empty() {
                new_due_date = Some(user_new_due_date);
            }

            if !user_new_schedule_at.is_empty() {
                new_scheduled_at = Some(user_new_schedule_at);
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue?")
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");

                info!("Modify task {}", task.id);
                let mut modify_operation = ModifyOperation::new(vec![task.id]);
                modify_operation.context_name = new_context;
                modify_operation.due_date = new_due_date.as_deref();
                modify_operation.scheduled_at = new_scheduled_at.as_deref();
                modify_operation.tag_names = new_tags;

                execute(&mut modify_operation)?;
            } else {
                println!("nevermind then :(");
            }
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
