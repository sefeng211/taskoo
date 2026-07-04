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
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }
    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        return TaskManager::delete(self.database_manager.as_mut().unwrap(), &self.task_ids);
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = Some(result);
    }
    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result.as_ref().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Operation;
    use crate::db::task_manager::TaskManager;
    use crate::operation::{Add, execute};
    use std::collections::HashMap;

    fn get_setting() -> HashMap<String, String> {
        let mut setting = HashMap::new();
        setting.insert("db_path".to_owned(), ":memory:".to_owned());
        setting.insert("tag".to_owned(), "Ready, Blocked".to_owned());
        setting.insert("context".to_owned(), "Inbox, Work, Life".to_owned());
        setting
    }

    #[test]
    fn test_delete_operation_returns_deleted_tasks() -> Result<(), CoreError> {
        let mut database_manager = TaskManager::new(&get_setting());

        let mut operation = Add::new_with_task_manager("Task One", &mut database_manager);
        execute(&mut operation)?;
        let mut operation = Add::new_with_task_manager("Task Two", &mut database_manager);
        execute(&mut operation)?;

        let delete_ids = vec![1, 2];
        let mut delete_operation = DeleteOperation {
            task_ids: delete_ids,
            database_manager: Some(database_manager),
            result: None,
        };

        execute(&mut delete_operation)?;

        let deleted = delete_operation.get_result();
        assert_eq!(deleted.len(), 2);
        assert_eq!(deleted[0].body, "Task One");
        assert_eq!(deleted[1].body, "Task Two");
        Ok(())
    }
}
