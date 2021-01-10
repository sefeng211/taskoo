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
        let mut operation = GetOperation {
            priority: None,
            context_name: Some("Inbox".to_string()),
            tag_names: vec![],
            due_date: None,
            scheduled_at: None,
            is_repeat: None,
            is_recurrence: None,
            database_manager: None,
            result: vec![],
        };

        execute(&mut operation)?;

        let need_review_tasks = operation.result;

        // TODO: This is too simple, ideally we want to have auto-completion
        for task in need_review_tasks.iter() {
            // Display Info
            println!(
                "{} {}",
                Paint::new("Task:").fg(Color::Magenta).to_string(),
                Paint::new(task.body.clone()).fg(Color::Yellow).to_string()
            );

            let context_names = Command::context(None)?;
            println!(
                "{}: {:?}",
                Paint::new("Existing Context:")
                    .fg(Color::Magenta)
                    .to_string(),
                context_names
            );
            let new_context = Review::ask_attribute("New Context?: ");
            // TODO: Get existing tags
            let new_tags =
                Review::ask_attribute("New Tags? (Use comma to separate multiple tags): ");
            let new_due_date = Review::ask_attribute("New Due Date?: ");
            // Ask for schedule at
            let new_scheduled_at = Review::ask_attribute("New Scheduled At?: ");

            println!("------- \n");
            println!(
                "Context: {} => {}",
                task.context_name,
                new_context.as_ref().unwrap_or(&String::from("Unchanged"))
            );
            println!(
                "Tags: {:?} => {}",
                task.tag_names,
                new_tags.as_ref().unwrap_or(&String::from("Unchanged"))
            );
            println!(
                "Due Date: {} => {}",
                task.due_date,
                new_due_date.as_ref().unwrap_or(&String::from("Unchanged"))
            );
            println!(
                "Scheuled At: {} => {}",
                task.scheduled_at,
                new_scheduled_at
                    .as_ref()
                    .unwrap_or(&String::from("Unchanged"))
            );
            println!("-------");
            // Ask for confirmation
            let mut confirmation = Review::ask_attribute("Looks Good? (Y/N): ");

            while confirmation != Some(String::from("y")) && confirmation != Some(String::from("n"))
            {
                confirmation = Review::ask_attribute("Looks Good? (Y/N): ");
            }

            if confirmation == Some(String::from("y")) || confirmation == Some(String::from("yes"))
            {
                info!("Modify task {}", task.id);
                let mut modify_operation = ModifyOperation {
                    database_manager: None,
                    result: vec![],
                    task_ids: vec![task.id],
                    body: None,
                    priority: None,
                    context_name: new_context,
                    tag_names: vec![],
                    due_date: new_due_date.as_deref(),
                    scheduled_at: new_scheduled_at.as_deref(),
                    repeat: None,
                    recurrence: None,
                    state_name: None,
                };
                execute(&mut modify_operation)?;
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
