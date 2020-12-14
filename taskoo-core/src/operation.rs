use crate::core::Operation;
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::OperationError;

pub struct AddOperation<'a> {
    pub body: &'a str,
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub is_repeat: Option<u8>,
    pub is_recurrence: Option<u8>,
    pub database_manager: Option<DatabaseManager>,
    pub result: Option<Vec<Task>>,
}

impl Operation for AddOperation<'_> {
    fn init(&mut self) {
        self.database_manager = Some(DatabaseManager::new(&self.init_and_get_database_path()));
    }
    fn do_work(&mut self) -> Result<Vec<Task>, OperationError> {
        return DatabaseManager::add(
            self.database_manager.as_mut().unwrap(),
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.is_repeat,
            &self.is_recurrence,
        );
    }
    fn set_result(&mut self, _result: Vec<Task>) {}
}

pub struct GetOperation<'a> {
    pub priority: Option<u8>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub is_repeat: Option<u8>,
    pub is_recurrence: Option<u8>,
    pub database_manager: Option<DatabaseManager>,
    pub result: Vec<Task>,
}

impl<'a> Operation for GetOperation<'a> {
    fn init(&mut self) {
        self.database_manager = Some(DatabaseManager::new(&self.init_and_get_database_path()));
    }
    fn do_work(&mut self) -> Result<Vec<Task>, OperationError> {
        return DatabaseManager::get(
            self.database_manager.as_mut().unwrap(),
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.is_repeat,
            &self.is_recurrence,
        );
    }

    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }
}

pub struct DeleteOperation {
    pub task_ids: Vec<i64>,
    pub database_manager: Option<DatabaseManager>,
}

impl Operation for DeleteOperation {
    fn init(&mut self) {
        self.database_manager = Some(DatabaseManager::new(&self.init_and_get_database_path()));
    }
    fn do_work(&mut self) -> Result<Vec<Task>, OperationError> {
        return DatabaseManager::delete(self.database_manager.as_mut().unwrap(), &self.task_ids);
    }
    fn set_result(&mut self, _result: Vec<Task>) {}
}

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
    pub is_repeat: Option<u8>,
    pub is_recurrence: Option<u8>,
}

impl<'a> Operation for ModifyOperation<'a> {
    fn init(&mut self) {
        if self.database_manager.is_none() {
            self.database_manager = Some(DatabaseManager::new(&self.init_and_get_database_path()));
        }
    }
    fn do_work(&mut self) -> Result<Vec<Task>, OperationError> {
        return DatabaseManager::modify(
            self.database_manager.as_mut().unwrap(),
            &self.task_ids,
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.is_repeat,
            &self.is_recurrence,
        );
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }
}

/* Some of the view functionalities are overlap with list, however,
 * view should provide better API for clients */
pub struct View {
    pub view_type: Option<String>,
    pub view_range_start: Option<String>,
    pub view_range_end: String,
    pub context_name: String,
    pub database_manager: Option<DatabaseManager>,
    pub result: Vec<Task>,
}

impl Operation for View {
    fn init(&mut self) {
        if self.database_manager.is_none() {
            self.database_manager = Some(DatabaseManager::new(&self.init_and_get_database_path()));
        }
    }

    fn do_work(&mut self) -> Result<Vec<Task>, OperationError> {
        return DatabaseManager::view(
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
}

pub fn execute(op: &mut impl Operation) -> Result<(), OperationError> {
    op.init();
    op.do_work().map(|tasks| {
        op.set_result(tasks);
    })?;
    Ok(())
}
