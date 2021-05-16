use chrono::{Date, DateTime, Duration, Local, NaiveDate, NaiveDateTime};
use rusqlite::Result;
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_manager::TaskManager;
use crate::operation::{Add, execute};
use crate::error::CoreError;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "ready, blocked, completed".to_owned());
    setting.insert("context".to_owned(), "inbox, work, life".to_owned());
    return setting;
}

#[test]
fn test_agenda_single_day_due() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-14");
    execute(&mut operation)?;

    // let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    // operation.date_due = Some("2020-11-13");
    // execute(&mut operation)?;

    let rows = database_manager.view_agenda(String::from("2020-11-14"), None)?;

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].0, NaiveDate::from_ymd(2020, 11, 14));
    let tasks = &rows[0].1;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].date_due, "2020-11-14 00:00:00".to_string());
    Ok(())
}

#[test]
fn test_agenda_single_day_due_and_overdue() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-15");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-14");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager.view_agenda(String::from("2020-11-14"), None)?;

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].0, NaiveDate::from_ymd(2020, 11, 14));
    let tasks = &rows[0].1;
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].date_due, "2020-11-14 00:00:00".to_string());
    assert_eq!(tasks[1].date_due, "2020-11-13 00:00:00".to_string());
    Ok(())
}

#[test]
fn test_agenda_single_day_scheduled_and_overscheduled() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-15");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_scheduled = Some("2020-11-14");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_scheduled = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager.view_agenda(String::from("2020-11-14"), None)?;

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].0, NaiveDate::from_ymd(2020, 11, 14));
    let tasks = &rows[0].1;
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].date_scheduled, "2020-11-14 00:00:00".to_string());
    assert_eq!(tasks[1].date_scheduled, "2020-11-13 00:00:00".to_string());
    Ok(())
}

#[test]
fn test_agenda_single_day_due_and_scheduled() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-15");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_scheduled = Some("2020-11-14");
    execute(&mut operation)?;

    let rows = database_manager.view_agenda(String::from("2020-11-15"), None)?;

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].0, NaiveDate::from_ymd(2020, 11, 15));
    let tasks = &rows[0].1;
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].date_due, "2020-11-15 00:00:00".to_string());
    assert_eq!(tasks[1].date_scheduled, "2020-11-14 00:00:00".to_string());
    Ok(())
}

#[test]
fn test_agenda_multiple_day_due_and_scheduled() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-15");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_scheduled = Some("2020-11-14");
    execute(&mut operation)?;

    let rows = database_manager
        .view_agenda(String::from("2020-11-14"), Some(String::from("2020-11-15")))?;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0].0, NaiveDate::from_ymd(2020, 11, 14));
    let tasks = &rows[0].1;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].date_scheduled, "2020-11-14 00:00:00".to_string());

    assert_eq!(rows[1].0, NaiveDate::from_ymd(2020, 11, 15));
    let tasks = &rows[1].1;
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].date_due, "2020-11-15 00:00:00".to_string());
    assert_eq!(tasks[1].date_scheduled, "2020-11-14 00:00:00".to_string());

    Ok(())
}
