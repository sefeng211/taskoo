use chrono::{Date, DateTime, Duration, Local, NaiveDate};
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
fn test_view_due() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    operation.date_due = Some("2020-11-14");
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager
        .view(
            &"inbox".to_string(),
            &Some("due".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].date_due, "2020-11-13".to_string());

    Ok(())
}

#[test]
fn test_view_overdue() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    operation.date_due = Some("2020-11-11");
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager
        .view(
            &"inbox".to_string(),
            &Some("overdue".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].date_due, "2020-11-11".to_string());

    Ok(())
}

#[test]
fn test_view_schedule() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-11");
    operation.date_scheduled = Some("2020-11-13");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body 2", &mut database_manager);
    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager
        .view(
            &"inbox".to_string(),
            &Some("schedule".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].date_scheduled, "2020-11-13".to_string());

    Ok(())
}

#[test]
fn test_view_schedule_today() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-11");
    operation.date_scheduled = Some("today");
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body 1", &mut database_manager);
    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body 2", &mut database_manager);
    operation.date_due = Some("2020-11-13");
    execute(&mut operation)?;

    let rows = database_manager
        .view(
            &"inbox".to_string(),
            &Some("schedule".to_string()),
            &None,
            &"today".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&rows[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_eq!(scheduled_at_parsed, expected.naive_local());

    Ok(())
}

#[test]
fn test_view_all_today() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.date_due = Some("2020-11-11");
    operation.date_scheduled = Some("today");
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body 1", &mut database_manager);
    operation.date_due = Some("today");
    execute(&mut operation)?;

    let rows = database_manager
        .view(
            &"inbox".to_string(),
            &Some("all".to_string()),
            &None,
            &"today".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 2);

    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&rows[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_eq!(scheduled_at_parsed, expected.naive_local());

    Ok(())
}
