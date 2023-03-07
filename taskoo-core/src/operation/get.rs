use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::option_parser::{parse_command_option, CommandOption};
use crate::error::*;
use crate::command::ContextCommand;
use crate::command::SimpleCommand;

pub struct Get<'a> {
    pub priority: Option<u8>,
    pub context: Option<String>,
    pub tags: Vec<String>,
    pub date_due: Option<&'a str>,
    pub date_scheduled: Option<&'a str>,
    pub task_id: Option<i64>,
    pub not_tags: Option<Vec<String>>, // Tags that don't exist
    database_manager: Option<TaskManager>,
    result: Vec<Task>,
}

impl<'a> Get<'a> {
    pub fn new2(data: &Vec<String>) -> Result<Vec<(String, Get)>, CoreError> {
        // TODO: Read context from the configuration file
        if data.is_empty() {
            // Create an operation for each context, this is needed
            // because get operation is per context
            Get::new_operations(CommandOption::new(), None)
        } else {
            let option =
                parse_command_option(&data.iter().map(|s| &**s).collect(), false, false, false)
                    .unwrap();
            match option.context {
                Some(ref context) => {
                    let context_name = context.clone();
                    Get::new_operations(option, Some(vec![context_name.to_string()]))
                }
                None => Get::new_operations(option, None),
            }
        }
    }

    fn new_operations(
        command_option: CommandOption,
        some_context_names: Option<Vec<String>>,
    ) -> Result<Vec<(String, Get)>, CoreError> {
        let context_names = match some_context_names {
            Some(context_names) => context_names,
            None => {
                // If no context names are passed, use all context
                let mut command = ContextCommand::new()?;
                let context_names = command.get_all()?;
                context_names
            }
        };

        let mut result = vec![];
        for context in context_names.iter() {
            let mut operation = Get::new();
            operation.context = Some(context.to_string());
            operation.tags = command_option.tags.clone();
            operation.date_due = command_option.date_due;
            operation.date_scheduled = command_option.date_scheduled;
            operation.not_tags = command_option.not_tags.clone();
            result.push((context.to_string(), operation));
        }
        Ok(result)
    }

    pub fn new() -> Get<'a> {
        Get {
            priority: None,
            context: None,
            tags: vec![],
            date_due: None,
            date_scheduled: None,
            task_id: None,
            not_tags: None,
            database_manager: None,
            result: vec![],
        }
    }
}

impl<'a> Operation for Get<'a> {
    fn init(&mut self) -> Result<(), InitialError> {
        self.database_manager = Some(TaskManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        // Treat all tag names as lowercase
        for tag in self.tags.iter_mut() {
            *tag = tag.to_lowercase();
        }

        if let Some(tags) = &mut self.not_tags {
            for tag in tags.iter_mut() {
                *tag = tag.to_lowercase();
            }
        }

        self.context = match &self.context {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        return TaskManager::get(
            self.database_manager.as_mut().unwrap(),
            &self.priority,
            &self.context,
            &self.tags,
            &self.date_due,
            &self.date_scheduled,
            &self.task_id,
            &self.not_tags,
        );
    }

    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}
