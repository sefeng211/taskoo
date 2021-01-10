use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;

pub struct ModifyOperation<'a> {
    pub database_manager: Option<DatabaseManager>,
    pub result: Vec<Task>,
    pub task_ids: Vec<i64>,
    pub body: Option<&'a str>,
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub recurrence: Option<&'a str>,
    pub repeat: Option<&'a str>,
    pub state_name: Option<&'a str>,
}

impl<'a> Operation for ModifyOperation<'a> {
    fn init(&mut self) {
        if self.database_manager.is_none() {
            self.database_manager = Some(DatabaseManager::new(
                &ConfigManager::init_and_get_database_path(),
            ));
        }
    }
    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        return DatabaseManager::modify(
            self.database_manager.as_mut().unwrap(),
            &self.task_ids,
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.repeat,
            &self.recurrence,
            &self.state_name,
        );
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}

