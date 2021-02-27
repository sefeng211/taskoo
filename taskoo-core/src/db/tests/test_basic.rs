use crate::db::task_manager::TaskManager;
use rusqlite::{Error as DbError, Result, NO_PARAMS};
use std::collections::HashMap;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Tag1, Tag2".to_owned());
    setting.insert(
        "context".to_owned(),
        "Context1, Context2, Context3".to_owned(),
    );
    return setting;
}

#[test]
fn test_create_table_if_needed() -> Result<(), DbError> {
    let database_manager = TaskManager::new(&get_setting());

    let mut tables = database_manager
        .conn
        .prepare("select * from sqlite_master where type='table'")
        .expect("Failed to prepare the statement");
    let mut rows = tables
        .query(NO_PARAMS)
        .expect("Failed to query the rows from sqlite_master");

    let mut names: Vec<String> = Vec::new();

    while let Some(result_row) = rows.next().expect("Failed to get the next row") {
        names.push(result_row.get(1).unwrap());
    }
    assert_eq!(
        names,
        [
            "task",
            "tag",
            "task_tag",
            "dependency",
            "context",
            "state",
            "priority",
            "priority_task"
        ]
    );

    Ok(())
}

#[test]
fn test_ensure_context_is_created() -> Result<(), DbError> {
    let database_manager = TaskManager::new(&get_setting());

    let mut context = database_manager
        .conn
        .prepare("SELECT name FROM context")
        .expect("");
    let mut rows = context.query(NO_PARAMS).expect("");

    let mut context_names: Vec<String> = Vec::new();
    while let Some(names) = rows.next().expect("") {
        context_names.push(names.get(0).unwrap());
    }
    assert_eq!(context_names, ["inbox"]);
    Ok(())
}

#[test]
fn test_ensure_state_is_created() -> Result<(), DbError> {
    let database_manager = TaskManager::new(&get_setting());

    let mut context = database_manager
        .conn
        .prepare("SELECT name FROM state")
        .expect("");
    let mut rows = context.query(NO_PARAMS).expect("");

    let mut state_names: Vec<String> = Vec::new();
    while let Some(names) = rows.next().expect("") {
        state_names.push(names.get(0).unwrap());
    }
    assert_eq!(state_names, ["ready", "completed", "blocked", "started"]);
    Ok(())
}

#[test]
fn test_ensure_priority_are_created() -> Result<(), DbError> {
    let database_manager = TaskManager::new(&get_setting());

    let mut context = database_manager
        .conn
        .prepare("SELECT name FROM priority")
        .expect("");
    let mut rows = context.query(NO_PARAMS).expect("");

    let mut p_names: Vec<String> = Vec::new();
    while let Some(names) = rows.next().expect("") {
        p_names.push(names.get(0).unwrap());
    }
    assert_eq!(p_names, ["h", "m", "l"]);
    Ok(())
}
