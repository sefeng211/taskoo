use crate::core::ConfigManager;
use crate::db::task_manager::TaskManager;
use crate::error::CoreError;
use crate::db::task_helper::TASK_STATES;
use rusqlite::{Result, named_params};

pub trait SimpleCommand<'a> {
    fn new() -> Result<Self, CoreError>
    where
        Self: Sized;
    fn new_with_manager(db_manager: &'a mut TaskManager) -> Self;
    fn get_all(&mut self) -> Result<Vec<String>, CoreError>;
    fn get_count(&mut self, name: &str) -> Result<i64, CoreError>;
    fn delete(&mut self, names: Vec<String>) -> Result<(), CoreError>;
}

pub struct ContextCommand<'a> {
    db_manager: Option<TaskManager>,
    db_manager_for_test: Option<&'a mut TaskManager>,
}

impl ContextCommand<'_> {
    fn context(&self) -> Result<Vec<String>, CoreError> {
        let mut statement;
        match self.db_manager.as_ref() {
            Some(manager) => {
                statement = manager.conn.prepare("SELECT name FROM context")?;
            }
            None => {
                statement = self
                    .db_manager_for_test
                    .as_ref()
                    .unwrap()
                    .conn
                    .prepare("SELECT name FROM context")?;
            }
        }

        let mut result = statement.query([])?;

        let mut context_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            context_names.push(row.get(0)?);
        }
        Ok(context_names)
    }

    fn delete_context_base(&mut self, names: Vec<String>) -> Result<(), CoreError> {
        let tx = match self.db_manager.as_mut() {
            Some(manager) => manager.conn.transaction()?,
            None => self
                .db_manager_for_test
                .as_mut()
                .unwrap()
                .conn
                .transaction()?,
        };

        for context_name in names.iter() {
            let lower_context_name = context_name.to_lowercase();
            tx.execute(
                "DELETE FROM context where name = :name",
                named_params! {":name": lower_context_name},
            )?;
        }
        tx.commit()?;
        Ok(())
    }
}

impl<'a> SimpleCommand<'a> for ContextCommand<'a> {
    fn new() -> Result<Self, CoreError>
    where
        Self: Sized,
    {
        Ok(ContextCommand {
            db_manager: Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            )),
            db_manager_for_test: None,
        })
    }

    fn new_with_manager(db_manager: &'a mut TaskManager) -> Self {
        ContextCommand {
            db_manager: None,
            db_manager_for_test: Some(db_manager),
        }
    }

    fn get_all(&mut self) -> Result<Vec<String>, CoreError> {
        return self.context();
    }

    // Get the number of tasks that belong to this context
    fn get_count(&mut self, name: &str) -> Result<i64, CoreError> {
        let mut statement;
        let statement_query = "
        SELECT COUNT(*) FROM task_context INNER JOIN
            (
            SELECT id FROM context WHERE name = :name
            )
            context
        ON task_context.context_id = context.id group by context.id";

        match self.db_manager.as_ref() {
            Some(manager) => {
                statement = manager.conn.prepare(statement_query)?;
            }
            None => {
                statement = self
                    .db_manager_for_test
                    .as_ref()
                    .unwrap()
                    .conn
                    .prepare(statement_query)?;
            }
        }

        let mut rows = statement.query(named_params! {":name": name})?;

        if let Some(row) = rows.next()? {
            return Ok(row.get(0)?);
        }
        Ok(0)
    }
    fn delete(&mut self, names: Vec<String>) -> Result<(), CoreError> {
        return self.delete_context_base(names);
    }
}

pub struct TagCommand<'a> {
    db_manager: Option<TaskManager>,
    db_manager_for_test: Option<&'a mut TaskManager>,
}

impl TagCommand<'_> {
    pub fn tags(&mut self) -> Result<Vec<String>, CoreError> {
        let mut statement = match self.db_manager.as_ref() {
            Some(manager) => manager.conn.prepare("SELECT name FROM tag")?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.prepare("SELECT name FROM tag")?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from(
                        "How come we don't have a task manager here?",
                    )))
                }
            },
        };

        let mut result = statement.query([])?;

        let mut tag_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            tag_names.push(row.get(0)?);
        }
        Ok(tag_names)
    }
}

impl<'a> SimpleCommand<'a> for TagCommand<'a> {
    fn new() -> Result<TagCommand<'a>, CoreError> {
        Ok(TagCommand {
            db_manager: Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            )),
            db_manager_for_test: None,
        })
    }

    fn new_with_manager(db_manager: &'a mut TaskManager) -> TagCommand<'a> {
        TagCommand {
            db_manager: None,
            db_manager_for_test: Some(db_manager),
        }
    }

    // Get the number of tasks that have this tag
    fn get_count(&mut self, name: &str) -> Result<i64, CoreError> {
        let mut statement = match self.db_manager.as_mut() {
            Some(manager) => manager.conn.prepare(
                "
        SELECT COUNT(*) FROM task_tag INNER JOIN
            (
            SELECT id FROM tag WHERE name = :name
            )
            tag
        ON task_tag.tag_id = tag.id group by tag.id",
            )?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.prepare(
                    "
        SELECT COUNT(*) FROM task_tag INNER JOIN
            (
            SELECT id FROM tag WHERE name = :name
            )
            tag
        ON task_tag.tag_id = tag.id group by tag.id",
                )?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from("How come?")));
                }
            },
        };
        let mut rows = statement.query(named_params! {":name": name})?;

        if let Some(row) = rows.next()? {
            return Ok(row.get(0)?);
        }
        Ok(0)
    }

    fn get_all(&mut self) -> Result<Vec<String>, CoreError> {
        return self.tags();
    }

    fn delete(&mut self, names: Vec<String>) -> Result<(), CoreError> {
        let tx = match self.db_manager.as_mut() {
            Some(manager) => manager.conn.transaction()?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.transaction()?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from("How come")));
                }
            },
        };

        {
            for name in names.iter() {
                let lower_context_name = name.to_lowercase();
                tx.execute(
                    "DELETE FROM tag where tag.name = :name",
                    named_params! {":name": lower_context_name},
                )?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}

pub struct StateCommand<'a> {
    db_manager: Option<TaskManager>,
    db_manager_for_test: Option<&'a mut TaskManager>,
}

impl StateCommand<'_> {
    pub fn states(&mut self) -> Result<Vec<String>, CoreError> {
        let mut statement = match self.db_manager.as_ref() {
            Some(manager) => manager.conn.prepare("SELECT name FROM state")?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.prepare("SELECT name FROM state")?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from(
                        "How come we don't have a task manager here?",
                    )))
                }
            },
        };

        let mut result = statement.query([])?;

        let mut states: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            // Filter out built-in states
            let name: String = row.get("name")?;
            if !TASK_STATES.contains(&name.as_str()) {
                states.push(name);
            }
        }

        Ok(states)
    }
}

impl<'a> SimpleCommand<'a> for StateCommand<'a> {
    fn new() -> Result<StateCommand<'a>, CoreError> {
        Ok(StateCommand {
            db_manager: Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            )),
            db_manager_for_test: None,
        })
    }

    fn new_with_manager(db_manager: &'a mut TaskManager) -> StateCommand<'a> {
        StateCommand {
            db_manager: None,
            db_manager_for_test: Some(db_manager),
        }
    }

    // Get the number of tasks that belong to this state
    fn get_count(&mut self, name: &str) -> Result<i64, CoreError> {
        let statement_query = "
        SELECT COUNT(*) as count FROM task_state INNER JOIN
            (
            SELECT id FROM state WHERE name = :name
            )
            state
        ON task_state.state_id = state.id group by state.id";

        let mut statement = match self.db_manager.as_mut() {
            Some(manager) => manager.conn.prepare(statement_query)?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.prepare(statement_query)?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from("How come?")));
                }
            },
        };

        let mut rows = statement.query(named_params! {":name": name})?;

        if let Some(row) = rows.next()? {
            return Ok(row.get("count")?);
        }
        Ok(0)
    }

    fn get_all(&mut self) -> Result<Vec<String>, CoreError> {
        return self.states();
    }

    fn delete(&mut self, names: Vec<String>) -> Result<(), CoreError> {
        let tx = match self.db_manager.as_mut() {
            Some(manager) => manager.conn.transaction()?,
            None => match self.db_manager_for_test.as_mut() {
                Some(manager) => manager.conn.transaction()?,
                None => {
                    return Err(CoreError::UnexpetedError(String::from("How come")));
                }
            },
        };

        {
            for name in names.iter() {
                let lower_context_name = name.to_lowercase();
                tx.execute(
                    "DELETE FROM state where state.name = :name",
                    named_params! {":name": lower_context_name},
                )?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::operation::Add;
    use crate::operation::execute;

    fn get_setting() -> HashMap<String, String> {
        let mut setting = HashMap::new();
        setting.insert("db_path".to_owned(), ":memory:".to_owned());
        return setting;
    }

    #[test]
    fn test_get_context() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());

        let mut context_names = ContextCommand::new_with_manager(&mut manager);
        assert_eq!(context_names.get_all()?, vec!["inbox"]);
        Ok(())
    }

    #[test]
    fn test_get_tags() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        {
            let mut command = TagCommand::new_with_manager(&mut manager);
            assert!(command.get_all()?.is_empty());
        }

        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;

        let mut command = TagCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_all()?, vec!["tag1"]);

        Ok(())
    }

    #[test]
    fn test_get_states() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        {
            let mut command = StateCommand::new_with_manager(&mut manager);
            assert!(command.get_all()?.is_empty());
        }

        let mut operation = Add::new_with_task_manager("Task Body", &mut manager);
        execute(&mut operation)?;

        let mut command = StateCommand::new_with_manager(&mut manager);
        // The `Task Body` task should has `ready` as the state, which
        // has been filtered out
        assert!(command.get_all()?.is_empty());

        let mut operation = Add::new_with_task_manager("Task Body 2", &mut manager);
        operation.set_custom_state(String::from("new_state"));
        execute(&mut operation)?;

        let mut command = StateCommand::new_with_manager(&mut manager);
        // The `Task Body` task should has `ready` as the state, which
        // has been filtered out
        assert_eq!(command.get_all()?, vec!["new_state"]);

        Ok(())
    }

    #[test]
    fn test_get_context_task_count() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        {
            let mut command = ContextCommand::new_with_manager(&mut manager);
            assert_eq!(command.get_count("inbox")?, 0);
        }

        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;

        let mut command = ContextCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_count("inbox")?, 1);
        Ok(())
    }

    #[test]
    fn test_get_tag_task_count() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        {
            let mut command = TagCommand::new_with_manager(&mut manager);
            assert!(command.get_all()?.is_empty());
        }
        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;
        let mut command = TagCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_count("tag1")?, 1);
        Ok(())
    }

    #[test]
    fn test_get_state_task_count() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());

        let mut operation = Add::new_with_task_manager("Task Body 2", &mut manager);
        operation.set_custom_state(String::from("new_state"));
        execute(&mut operation)?;

        let mut command = StateCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_count("new_state")?, 1);
        Ok(())
    }

    #[test]
    fn test_delete_context() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        let mut command = ContextCommand::new_with_manager(&mut manager);

        assert_eq!(command.get_all()?, vec!["inbox"]);

        command.delete(vec![String::from("inbox")])?;
        assert!(command.get_all()?.is_empty());
        Ok(())
    }

    #[test]
    fn test_delete_context_with_task_associated() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;

        let mut command = ContextCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_all()?, vec!["inbox"]);

        let result = command.delete(vec![String::from("inbox")]);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_delete_tag_with_tag_associated() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;

        let mut command = TagCommand::new_with_manager(&mut manager);
        assert_eq!(command.get_all()?, vec!["tag1"]);

        let result = command.delete(vec![String::from("tag1")]);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_delete_tag() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());

        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.tags = vec!["tag1".to_owned()];
        execute(&mut operation)?;

        let mut command = TagCommand::new_with_manager(&mut manager);
        command
            .db_manager_for_test
            .as_mut()
            .unwrap()
            .delete(&vec![1])?;

        assert_eq!(command.get_all()?, vec!["tag1"]);

        command.delete(vec![String::from("tag1")])?;
        assert!(command.get_all()?.is_empty());

        Ok(())
    }

    #[test]
    fn test_delete_state_with_task_associated() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.set_custom_state(String::from("new_state"));
        execute(&mut operation)?;

        let mut command = StateCommand::new_with_manager(&mut manager);

        let result = command.delete(vec![String::from("new_state")]);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_delete_state() -> Result<(), CoreError> {
        let mut manager = TaskManager::new(&get_setting());
        let mut operation = Add::new_with_task_manager("Test Body", &mut manager);
        operation.set_custom_state(String::from("new_state"));
        execute(&mut operation)?;

        let mut command = StateCommand::new_with_manager(&mut manager);
        command
            .db_manager_for_test
            .as_mut()
            .unwrap()
            .delete(&vec![1])?;

        command.delete(vec![String::from("new_state")])?;
        assert!(command.get_all()?.is_empty());
        Ok(())
    }
}
