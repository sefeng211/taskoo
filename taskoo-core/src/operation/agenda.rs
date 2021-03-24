use chrono::{NaiveDate};
use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;

/* Some of the view functionalities are overlap with list, however,
 * view should provide better API for clients */
pub struct Agenda {
    pub start_day: String,
    pub end_day: Option<String>,
    database_manager: Option<TaskManager>,
    result: Vec<(NaiveDate, Vec<Task>)>,
}

impl Agenda {
    pub fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }
    pub fn new(start_day: String, end_day: Option<String>) -> Agenda {
        Agenda {
            start_day: start_day,
            end_day: None,
            database_manager: None,
            result: vec![],
        }
    }

    pub fn do_work_for_agenda(&mut self) -> Result<Vec<(NaiveDate, Vec<Task>)>, CoreError> {
        return TaskManager::view_agenda(
            self.database_manager.as_mut().unwrap(),
            self.start_day.clone(),
            self.end_day.clone(),
        );
    }

    pub fn set_result(&mut self, result: Vec<(NaiveDate, Vec<Task>)>) {
        self.result = result;
    }

    pub fn get_result(&mut self) -> &Vec<(NaiveDate, Vec<Task>)> {
        return &self.result;
    }
}
