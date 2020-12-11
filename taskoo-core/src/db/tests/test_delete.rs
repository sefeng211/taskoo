use crate::db::add::add;
use rusqlite::{named_params, params, Connection, Error as DbError, Result, NO_PARAMS};
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;
use crate::db::task_manager::DatabaseManager;
use chrono::format::ParseError;
use chrono::prelude::*;
use chrono::NaiveDateTime;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked".to_owned());
    setting.insert("context".to_owned(), "Inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_delete_simple() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &Some(1),
            &None,
        )
        .expect("");

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &Some(1), &None)
        .unwrap();

    assert_eq!(rows.len(), 1);

    database_manager.delete(&vec![1]).unwrap();

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &Some(1), &None)
        .unwrap();

    assert_eq!(rows.len(), 0);
    Ok(())
}

#[test]
fn test_delete_multiple() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &Some(1),
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &Some(1),
            &None,
        )
        .expect("");

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &Some(1), &None)
        .unwrap();

    assert_eq!(rows.len(), 2);

    database_manager.delete(&vec![1, 2]).unwrap();

    let bb = database_manager
        .get(&None, &None, &vec![], &None, &None, &Some(1), &None)
        .unwrap();

    assert_eq!(bb.len(), 0);
    Ok(())
}
