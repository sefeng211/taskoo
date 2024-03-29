use rusqlite::Result;
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_manager::TaskManager;
use crate::operation::{Add, execute};
use crate::error::CoreError;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked".to_owned());
    setting.insert("context".to_owned(), "Inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_delete_simple() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

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
fn test_delete_multiple() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(rows.len(), 2);

    database_manager.delete(&vec![1, 2]).unwrap();

    let bb = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(bb.len(), 0);
    Ok(())
}
