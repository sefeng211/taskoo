use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::{Task, TASK_STATES};
use crate::db::task_manager::TaskManager;
use crate::error::*;
use log::debug;

pub struct Add<'a> {
    pub body: &'a str,
    pub priority: Option<String>,
    pub context: Option<String>,
    state: Option<String>,
    pub tags: Vec<String>,
    pub date_due: Option<&'a str>,
    pub date_scheduled: Option<&'a str>,
    pub repetition_due: Option<&'a str>,
    pub repetition_scheduled: Option<&'a str>,
    pub annotation: Option<&'a str>,
    pub parent_task_ids: Option<Vec<i64>>,
    task_manager: Option<TaskManager>,
    task_manager_for_test: Option<&'a mut TaskManager>,
    result: Option<Vec<Task>>,
}

pub struct AddAnnotation {
    pub task_id: i64,
    pub annotation: String,
    database_manager: Option<TaskManager>,
    result: Option<Vec<Task>>,
}

impl Add<'_> {
    pub fn new(body: &str) -> Add {
        Add {
            body: body,
            priority: None,
            context: None,
            state: None,
            tags: vec![],
            date_due: None,
            date_scheduled: None,
            repetition_due: None,
            repetition_scheduled: None,
            annotation: None,
            parent_task_ids: None,
            task_manager: None,
            task_manager_for_test: None,
            result: None,
        }
    }

    pub fn new_with_task_manager<'a>(body: &'a str, task_manager: &'a mut TaskManager) -> Add<'a> {
        Add {
            body: body,
            priority: None,
            context: None,
            state: None,
            tags: vec![],
            date_due: None,
            date_scheduled: None,
            repetition_due: None,
            repetition_scheduled: None,
            annotation: None,
            parent_task_ids: None,
            task_manager: None,
            task_manager_for_test: Some(task_manager),
            result: None,
        }
    }

    pub fn set_custom_state(&mut self, state: String) {
        self.state = Some(state);
    }
    pub fn set_state_to_ready(&mut self) {
        self.state = Some(String::from(TASK_STATES[0]));
    }

    pub fn set_state_to_completed(&mut self) {
        self.state = Some(String::from(TASK_STATES[1]));
    }

    pub fn set_state_to_blocked(&mut self) {
        self.state = Some(String::from(TASK_STATES[2]));
    }

    pub fn set_state_to_started(&mut self) {
        self.state = Some(String::from(TASK_STATES[3]));
    }
}

impl AddAnnotation {
    pub fn new(task_id: i64, annotation: String) -> AddAnnotation {
        AddAnnotation {
            task_id: task_id,
            annotation: annotation,
            database_manager: None,
            result: None,
        }
    }
}

impl Operation for Add<'_> {
    fn init(&mut self) -> Result<(), InitialError> {
        if self.task_manager_for_test.is_none() {
            self.task_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        for tag in self.tags.iter_mut() {
            if tag.is_empty() {
                return Err(CoreError::ArgumentError(String::from(
                    "Empty tag is provided, not allowed!",
                )));
            }
            *tag = tag.to_lowercase();
        }

        self.context = match &self.context {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        assert!(!(self.task_manager.is_some() && self.task_manager_for_test.is_some()));

        match self.task_manager.as_mut() {
            Some(manager) => {
                debug!("Using task_manager");
                return TaskManager::add(
                    manager,
                    &self.body,
                    &self.priority,
                    &self.context,
                    &self.tags,
                    &self.date_due,
                    &self.date_scheduled,
                    &self.repetition_due,
                    &self.repetition_scheduled,
                    &self.annotation,
                    &self.state,
                    &self.parent_task_ids,
                );
            }
            None => {
                assert!(self.task_manager_for_test.is_some());
                debug!("Using task_manager_for_test");
                return TaskManager::add(
                    self.task_manager_for_test.as_mut().unwrap(),
                    &self.body,
                    &self.priority,
                    &self.context,
                    &self.tags,
                    &self.date_due,
                    &self.date_scheduled,
                    &self.repetition_due,
                    &self.repetition_scheduled,
                    &self.annotation,
                    &self.state,
                    &self.parent_task_ids,
                );
            }
        }
    }

    fn set_result(&mut self, result: Vec<Task>) {
        self.result = Some(result);
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result.as_ref().unwrap();
    }
}

impl Operation for AddAnnotation {
    fn init(&mut self) -> Result<(), InitialError> {
        self.database_manager = Some(TaskManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        return TaskManager::add_annotation(
            self.database_manager.as_mut().unwrap(),
            self.task_id,
            self.annotation.clone(),
        );
    }

    fn set_result(&mut self, _result: Vec<Task>) {}

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result.as_ref().unwrap();
    }
}
