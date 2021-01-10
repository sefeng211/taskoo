use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;

pub struct Add<'a> {
    pub body: &'a str,
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub state_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub repeat: Option<&'a str>,
    pub recurrence: Option<&'a str>,
    pub database_manager: Option<DatabaseManager>,
    pub result: Option<Vec<Task>>,
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
            repeat: None,
            recurrence: None,
            database_manager: None,
            result: None,
        }
    }
}
impl Operation for Add<'_> {
    fn init(&mut self) {
        self.database_manager = Some(DatabaseManager::new(
            &ConfigManager::init_and_get_database_path(),
        ));
    }
    fn do_work(&mut self) -> Result<Vec<Task>, TaskooError> {
        return DatabaseManager::add(
            self.database_manager.as_mut().unwrap(),
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
    fn set_result(&mut self, _result: Vec<Task>) {}
    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result.as_ref().unwrap();
    }
}
