use chrono::{Date, DateTime, Duration, Local, NaiveDate};
use rusqlite::Result;
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_manager::DatabaseManager;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked, Completed".to_owned());
    setting.insert("context".to_owned(), "Inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_view_due() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-14"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let rows = database_manager
        .view(
            &"Inbox".to_string(),
            &Some("due".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].due_date, "2020-11-13".to_string());

    Ok(())
}

#[test]
fn test_view_overdue() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-11"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let rows = database_manager
        .view(
            &"Inbox".to_string(),
            &Some("overdue".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].due_date, "2020-11-11".to_string());

    Ok(())
}

#[test]
fn test_view_schedule() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-11"),
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body 1",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body 2",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let rows = database_manager
        .view(
            &"Inbox".to_string(),
            &Some("schedule".to_string()),
            &None,
            &"2020-11-13".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    assert_eq!(rows[0].scheduled_at, "2020-11-13".to_string());

    Ok(())
}

#[test]
fn test_view_schedule_today() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);
    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-11"),
            &Some("today"),
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body 1",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body 2",
            &None,
            &None,
            &vec![],
            &Some("2020-11-13"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let rows = database_manager
        .view(
            &"Inbox".to_string(),
            &Some("schedule".to_string()),
            &None,
            &"today".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 1);

    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&rows[0].scheduled_at, "%Y-%m-%d").expect("");

    assert_eq!(scheduled_at_parsed, expected.naive_local());

    Ok(())
}

#[test]
fn test_view_all_today() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);
    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &Some("2020-11-11"),
            &Some("today"),
            &None,
            &None,
            &None,
        )
        .expect("");

    database_manager
        .add(
            "Test Body 1",
            &None,
            &None,
            &vec![],
            &Some("today"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let rows = database_manager
        .view(
            &"Inbox".to_string(),
            &Some("all".to_string()),
            &None,
            &"today".to_string(),
        )
        .unwrap();

    assert_eq!(rows.len(), 2);

    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&rows[0].scheduled_at, "%Y-%m-%d").expect("");

    assert_eq!(scheduled_at_parsed, expected.naive_local());

    Ok(())
}
