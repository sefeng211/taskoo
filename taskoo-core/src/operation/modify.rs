use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::*;

#[derive(Debug)]
pub struct ModifyOperation<'a> {
    pub task_ids: Vec<i64>,
    pub body: Option<&'a str>,
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub due_repeat: Option<&'a str>,
    pub scheduled_repeat: Option<&'a str>,
    pub state_name: Option<&'a str>,
    pub tags_to_remove: Vec<String>,
    database_manager: Option<DatabaseManager>,
    result: Vec<Task>,
}

impl<'a> ModifyOperation<'a> {
    pub fn new(task_ids: Vec<i64>) -> ModifyOperation<'a> {
        ModifyOperation {
            database_manager: None,
            result: vec![],
            task_ids: task_ids,
            body: None,
            priority: None,
            context_name: None,
            tag_names: vec![],
            due_date: None,
            scheduled_at: None,
            due_repeat: None,
            scheduled_repeat: None,
            state_name: None,
            tags_to_remove: vec![],
        }
    }
}
impl<'a> Operation for ModifyOperation<'a> {
    fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(DatabaseManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }
    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        for tag in self.tag_names.iter_mut() {
            *tag = tag.to_lowercase();
        }

        for tag in self.tags_to_remove.iter_mut() {
            *tag = tag.to_lowercase();
        }

        self.context_name = match &self.context_name {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        return DatabaseManager::modify(
            self.database_manager.as_mut().unwrap(),
            &self.task_ids,
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.due_repeat,
            &self.scheduled_repeat,
            &self.state_name,
            &self.tags_to_remove
        );
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}
