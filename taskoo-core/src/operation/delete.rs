use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;

pub struct DeleteOperation {
    pub task_ids: Vec<i64>,
    pub database_manager: Option<DatabaseManager>,
    pub result: Option<Vec<Task>>,
}

impl Operation for DeleteOperation {
    fn init(&mut self) {
        self.database_manager = Some(DatabaseManager::new(
            &ConfigManager::init_and_get_database_path(),
        ));
    }
    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        return DatabaseManager::delete(self.database_manager.as_mut().unwrap(), &self.task_ids);
    }
    fn set_result(&mut self, _result: Vec<Task>) {}
    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result.as_ref().unwrap();
    }
}
