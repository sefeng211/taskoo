use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;

/* Some of the view functionalities are overlap with list, however,
 * view should provide better API for clients */
pub struct View {
    pub view_type: Option<String>,
    pub view_range_start: Option<String>,
    pub view_range_end: String,
    pub context_name: String,
    database_manager: Option<TaskManager>,
    result: Vec<Task>,
}

impl View {
    pub fn new(context_name: String, view_range_end: String) -> View {
        View {
            view_type: None,
            view_range_start: None,
            view_range_end: view_range_end,
            context_name: context_name,
            database_manager: None,
            result: vec![],
        }
    }
}
impl Operation for View {
    fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        return TaskManager::view(
            self.database_manager.as_mut().unwrap(),
            &self.context_name,
            &self.view_type,
            &self.view_range_start,
            &self.view_range_end,
        );
    }

    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}
