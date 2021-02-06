use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::*;

pub struct Add<'a> {
    pub body: &'a str,
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub state_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub due_repeat: Option<&'a str>,
    pub scheduled_repeat: Option<&'a str>,
    database_manager: Option<DatabaseManager>,
    result: Option<Vec<Task>>,
}

pub struct AddAnnotation {
    pub task_id: i64,
    pub annotation: String,
    database_manager: Option<DatabaseManager>,
    result: Option<Vec<Task>>,
}

impl Add<'_> {
    pub fn new(body: &str) -> Add {
        Add {
            body: body,
            priority: None,
            context_name: None,
            state_name: None,
            tag_names: vec![],
            due_date: None,
            scheduled_at: None,
            due_repeat: None,
            scheduled_repeat: None,
            database_manager: None,
            result: None,
        }
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
        self.database_manager = Some(DatabaseManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        for tag in self.tag_names.iter_mut() {
            *tag = tag.to_lowercase();
        }

        self.context_name = match &self.context_name {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        return DatabaseManager::add(
            self.database_manager.as_mut().unwrap(),
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.due_repeat,
            &self.scheduled_repeat,
            &self.state_name,
        );
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
        self.database_manager = Some(DatabaseManager::new(
            &ConfigManager::init_and_get_database_path()?,
        ));
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        return DatabaseManager::add_annotation(
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
