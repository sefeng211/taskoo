use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;

#[derive(Debug)]
pub struct ModifyOperation<'a> {
    pub task_ids: Vec<i64>,
    pub body: Option<&'a str>,
    pub priority: Option<String>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub due_repeat: Option<&'a str>,
    pub scheduled_repeat: Option<&'a str>,
    state: Option<String>,
    pub tags_to_remove: Vec<String>,
    database_manager: Option<TaskManager>,
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
            state: None,
            tags_to_remove: vec![],
        }
    }

    pub fn set_state_to_started(&mut self) {
        self.state = Some(String::from("started"));
    }

    pub fn set_state_to_completed(&mut self) {
        self.state = Some(String::from("completed"));
    }
    pub fn set_state_to_ready(&mut self) {
        self.state = Some(String::from("ready"));
    }
    pub fn set_state_to_blocked(&mut self) {
        self.state = Some(String::from("blocked"));
    }
    pub fn set_custom_state(&mut self, state: String) {
        self.state = Some(state);
    }
}
impl<'a> Operation for ModifyOperation<'a> {
    fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }
    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
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

        let tasks = TaskManager::modify(
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
            &self.state.as_deref(),
            &self.tags_to_remove,
        )?;

        Ok(tasks)
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}
