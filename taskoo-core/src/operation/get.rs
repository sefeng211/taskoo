use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;

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
