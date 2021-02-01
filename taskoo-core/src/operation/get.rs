use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::*;

pub struct Get<'a> {
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub task_id: Option<i64>,
    database_manager: Option<DatabaseManager>,
    result: Vec<Task>,
}

impl<'a> Get<'a> {
    pub fn new() -> Get<'a> {
        Get {
            priority: None,
            context_name: None,
            tag_names: vec![],
            due_date: None,
            scheduled_at: None,
            task_id: None,
            database_manager: None,
            result: vec![],
        }
    }
}

impl<'a> Operation for Get<'a> {
    fn init(&mut self) -> Result<(), InitialError> {
        self.database_manager = Some(DatabaseManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        // Treat all tag names as lowercase
        for tag in self.tag_names.iter_mut() {
            *tag = tag.to_lowercase();
        }
        self.context_name = match &self.context_name {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        return DatabaseManager::get(
            self.database_manager.as_mut().unwrap(),
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.task_id
        );
    }

    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}
