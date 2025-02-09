use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::option_parser::parse_command_option;
use crate::error::*;

pub struct DeleteOperation {
    pub task_ids: Vec<i64>,
    pub database_manager: Option<TaskManager>,
    pub result: Option<Vec<Task>>,
}

impl DeleteOperation {
    pub fn new(input_str: &Vec<String>) -> Result<DeleteOperation, CoreError> {
        let option = parse_command_option(
            &input_str.iter().map(|s| &**s).collect(),
            false,
            false,
            true,
        )
        .unwrap();

        Ok(DeleteOperation {
            task_ids: option.task_ids,
            database_manager: None,
            result: None,
        })
    }
}

impl Operation for DeleteOperation {
    fn init(&mut self) -> Result<(), InitialError> {
        self.database_manager = Some(TaskManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }
    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        return TaskManager::delete(self.database_manager.as_mut().unwrap(), &self.task_ids);
    }
    fn set_result(&mut self, _result: Vec<Task>) {}
    fn get_result(&mut self) -> &Vec<Task> {
        // TODO: Store the deleted task ids
        return &self.result.as_ref().unwrap();
    }
}
