use crate::core::ConfigManager;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;
use rusqlite::{Result, NO_PARAMS};
use std::collections::HashMap;

pub struct Command;
impl Command {
    pub fn get_context() -> Result<Vec<String>, TaskooError> {
        return Command::context(None);
    }

    pub fn get_tags() -> Result<Vec<String>, TaskooError> {
        return Command::tags(None);
    }

    pub fn context(setting: Option<HashMap<String, String>>) -> Result<Vec<String>, TaskooError> {
        let database_manager;
        match setting {
            None => {
                database_manager =
                    DatabaseManager::new(&ConfigManager::init_and_get_database_path()?);
            }
            Some(info) => {
                database_manager = DatabaseManager::new(&info);
            }
        }

        let mut statement = database_manager.conn.prepare("SELECT name FROM context")?;

        let mut result = statement.query(NO_PARAMS)?;

        let mut context_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            context_names.push(row.get(0)?);
        }
        Ok(context_names)
    }

    pub fn tags(setting: Option<HashMap<String, String>>) -> Result<Vec<String>, TaskooError> {
        let database_manager;
        match setting {
            None => {
                database_manager =
                    DatabaseManager::new(&ConfigManager::init_and_get_database_path()?);
            }
            Some(info) => {
                database_manager = DatabaseManager::new(&info);
            }
        }

        let mut statement = database_manager.conn.prepare("SELECT name FROM tag")?;

        let mut result = statement.query(NO_PARAMS)?;

        let mut tag_names: Vec<String> = vec![];
        while let Some(row) = result.next()? {
            tag_names.push(row.get(0)?);
        }
        Ok(tag_names)
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
    fn test_get_context() {
        let context_names = Command::context(Some(get_setting())).unwrap();
        assert_eq!(context_names, vec!["Inbox"]);
    }
}
