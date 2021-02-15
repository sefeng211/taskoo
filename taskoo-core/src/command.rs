use crate::core::ConfigManager;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;

use rusqlite::{Result, NO_PARAMS, named_params};
use std::collections::HashMap;

pub trait SimpleCommand {
    fn new() -> Result<Self, TaskooError>
    where
        Self: Sized;
    fn new_with_manager(db_manager: DatabaseManager) -> Self;
    fn get_all(&self) -> Result<Vec<String>, TaskooError>;
    fn get_count(&self, name: &str) -> Result<i64, TaskooError>;
    fn delete(&mut self, names: Vec<String>) -> Result<(), TaskooError>;
}

pub struct ContextCommand {
    db_manager: DatabaseManager,
}

impl ContextCommand {
    fn context(&self) -> Result<Vec<String>, TaskooError> {
        let mut statement = self.db_manager.conn.prepare("SELECT name FROM context")?;

        let mut result = statement.query(NO_PARAMS)?;

        let mut context_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            context_names.push(row.get(0)?);
        }
        Ok(context_names)
    }
    fn delete_context_base(&mut self, names: Vec<String>) -> Result<(), TaskooError> {
        let tx = self.db_manager.conn.transaction()?;

        for context_name in names.iter() {
            let lower_context_name = context_name.to_lowercase();
            tx.execute_named(
                "DELETE FROM context where name = :name",
                named_params! {":name": lower_context_name},
            )?;
        }
        tx.commit()?;
        Ok(())
    }
}

impl SimpleCommand for ContextCommand {
    fn new() -> Result<Self, TaskooError>
    where
        Self: Sized,
    {
        Ok(ContextCommand {
            db_manager: DatabaseManager::new(&ConfigManager::init_and_get_database_path()?),
        })
    }

    fn new_with_manager(db_manager: DatabaseManager) -> Self {
        ContextCommand {
            db_manager: db_manager,
        }
    }

    fn get_all(&self) -> Result<Vec<String>, TaskooError> {
        return self.context();
    }

    // Get the number of tasks that belong to this context
    fn get_count(&self, name: &str) -> Result<i64, TaskooError> {
        let mut statement = self.db_manager.conn.prepare(
            "
        SELECT COUNT(*) FROM task INNER JOIN
            (
            SELECT id FROM context WHERE name = :name
            )
            context
        ON task.context_id = context.id group by context.id",
        )?;

        let mut rows = statement.query_named(named_params! {":name": name})?;

        if let Some(row) = rows.next()? {
            return Ok(row.get(0)?);
        }
        Ok(0)
    }
    fn delete(&mut self, names: Vec<String>) -> Result<(), TaskooError> {
        return self.delete_context_base(names);
    }
}

pub struct TagCommand {
    db_manager: DatabaseManager,
}

impl TagCommand {
    pub fn tags(&self) -> Result<Vec<String>, TaskooError> {
        let mut statement = self.db_manager.conn.prepare("SELECT name FROM tag")?;

        let mut result = statement.query(NO_PARAMS)?;

        let mut tag_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            tag_names.push(row.get(0)?);
        }
        Ok(tag_names)
    }
}

impl SimpleCommand for TagCommand {
    fn new() -> Result<TagCommand, TaskooError> {
        Ok(TagCommand {
            db_manager: DatabaseManager::new(&ConfigManager::init_and_get_database_path()?),
        })
    }

    fn new_with_manager(db_manager: DatabaseManager) -> TagCommand {
        TagCommand {
            db_manager: db_manager,
        }
    }
    // Get the number of tasks that have this tag
    fn get_count(&self, name: &str) -> Result<i64, TaskooError> {
        let mut statement = self.db_manager.conn.prepare(
            "
        SELECT COUNT(*) FROM task_tag INNER JOIN
            (
            SELECT id FROM tag WHERE name = :name
            )
            tag
        ON task_tag.tag_id = tag.id group by tag.id",
        )?;

        let mut rows = statement.query_named(named_params! {":name": name})?;

        if let Some(row) = rows.next()? {
            return Ok(row.get(0)?);
        }
        Ok(0)
    }

    fn get_all(&self) -> Result<Vec<String>, TaskooError> {
        return self.tags();
    }

    fn delete(&mut self, names: Vec<String>) -> Result<(), TaskooError> {
        let tx = self.db_manager.conn.transaction()?;

        {
            for name in names.iter() {
                let lower_context_name = name.to_lowercase();
                tx.execute_named(
                    "DELETE FROM tag where tag.name = :name",
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
    fn get_setting() -> HashMap<String, String> {
        let mut setting = HashMap::new();
        setting.insert("db_path".to_owned(), ":memory:".to_owned());
        return setting;
    }

    #[test]
    fn test_get_context() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());

        let context_names = ContextCommand::new_with_manager(manager);
        assert_eq!(context_names.get_all()?, vec!["inbox"]);
        Ok(())
    }

    #[test]
    fn test_get_tags() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = TagCommand::new_with_manager(manager);
        assert!(command.get_all()?.is_empty());
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;
        assert_eq!(command.get_all()?, vec!["tag1"]);

        Ok(())
    }

    #[test]
    fn test_get_context_task_count() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = ContextCommand::new_with_manager(manager);
        assert_eq!(command.get_count("inbox")?, 0);
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;
        assert_eq!(command.get_count("inbox")?, 1);
        Ok(())
    }

    #[test]
    fn test_get_tag_task_count() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = TagCommand::new_with_manager(manager);
        assert!(command.get_all()?.is_empty());
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;
        assert_eq!(command.get_count("tag1")?, 1);
        Ok(())
    }

    #[test]
    fn test_delete_context_with_task_associated() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = ContextCommand::new_with_manager(manager);
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;

        assert_eq!(command.get_all()?, vec!["inbox"]);

        let result = command.delete(vec![String::from("inbox")]);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_delete_context() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = ContextCommand::new_with_manager(manager);

        assert_eq!(command.get_all()?, vec!["inbox"]);

        command.delete(vec![String::from("inbox")])?;
        assert!(command.get_all()?.is_empty());
        Ok(())
    }

    #[test]
    fn test_delete_tag_with_tag_associated() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = TagCommand::new_with_manager(manager);
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;

        assert_eq!(command.get_all()?, vec!["tag1"]);

        let result = command.delete(vec![String::from("tag1")]);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_delete_tag() -> Result<(), TaskooError> {
        let manager = DatabaseManager::new(&get_setting());
        let mut command = TagCommand::new_with_manager(manager);
        command.db_manager.add(
            "Test",
            &None,
            &None,
            &vec![String::from("tag1")],
            &None,
            &None,
            &None,
            &None,
            &None,
        )?;

        command.db_manager.delete(&vec![1])?;

        assert_eq!(command.get_all()?, vec!["tag1"]);

        command.delete(vec![String::from("tag1")])?;
        assert!(command.get_all()?.is_empty());

        Ok(())
    }
}
