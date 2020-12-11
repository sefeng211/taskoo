use rusqlite::{Result, NO_PARAMS};
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_helper::convert_rows_into_task;
use crate::db::task_manager::DatabaseManager;
use crate::operation::{execute, GetAllForContextOperation};
use chrono::NaiveDateTime;
use chrono::{Date, DateTime, Duration, NaiveDate, Utc};

use more_asserts::*;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked, Completed".to_owned());
    setting.insert("context".to_owned(), "Inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_add_simple() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    assert_eq!(tasks.len(), 1);

    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].created_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "Test Body");
    assert_eq!(tasks[0].priority, 1);
    assert_eq!(tasks[0].context_name, "Inbox");
    // TODO: Improve the assert_eq here to ensure the auto created `created_at` timestamp is
    // correct
    assert_eq!(created_at_datetime, current_datetime.date());
    assert_eq!(tasks[0].due_date.is_empty(), true);
    assert_eq!(tasks[0].scheduled_at.is_empty(), true);
    assert_eq!(tasks[0].is_repeat, 0);
    assert_eq!(tasks[0].is_recurrence, 0);

    Ok(())
}

#[test]
fn test_add_complex() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].created_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "Test Body");
    assert_eq!(tasks[0].priority, 3);
    assert_eq!(tasks[0].context_name, "Work");
    // TODO: Improve the assert_eq here to ensure the auto created `created_at` timestamp is
    // correct
    assert_eq!(created_at_datetime, current_datetime.date());
    assert_eq!(tasks[0].due_date.is_empty(), true);
    assert_eq!(tasks[0].scheduled_at.is_empty(), true);
    assert_eq!(tasks[0].is_repeat, 0);
    assert_eq!(tasks[0].is_recurrence, 0);
    assert_eq!(tasks[0].is_completed, false);

    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_exist_tag() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Ready".to_owned(), "Blocked".to_owned()],
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task_tag")
        .expect("");

    let mut result = tasks.query(NO_PARAMS).unwrap();

    let mut inserted_task_tag: Vec<(i64, i64)> = vec![];

    while let Some(row) = result.next().unwrap() {
        inserted_task_tag.push((row.get(0).unwrap(), row.get(1).unwrap()));
    }
    assert_eq!(inserted_task_tag, [(1, 1), (1, 2)]);

    Ok(())
}

#[test]
#[should_panic]
fn test_add_not_exist_tag() {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["NewTag1".to_owned(), "Blocked".to_owned()],
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");
}

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_days() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let start = Utc::now() + Duration::days(2);
    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("2days"),
            &None,
            &None,
        )
        .expect("");

    let end = Utc::now() + Duration::days(2);
    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].scheduled_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    assert_ge!(scheduled_at_parsed, start.date());
    assert_le!(scheduled_at_parsed, end.date());
    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_hours() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let start = Utc::now() + Duration::hours(11);
    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("11hours"),
            &None,
            &None,
        )
        .expect("");

    let end = Utc::now() + Duration::hours(11);
    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].scheduled_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    assert_ge!(scheduled_at_parsed, start.date());
    assert_le!(scheduled_at_parsed, end.date());
    Ok(())
}

#[test]
fn test_add_scheduled_at_weeks() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let start = Utc::now() + Duration::weeks(1);
    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("1weeks"),
            &None,
            &None,
        )
        .expect("");

    let end = Utc::now() + Duration::weeks(1);
    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].scheduled_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    assert_ge!(scheduled_at_parsed, start.date());
    assert_le!(scheduled_at_parsed, end.date());
    Ok(())
}

#[test]
fn test_add_scheduled_at_raw_timestamp() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("2020-11-11"),
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    assert_eq!(&tasks[0].scheduled_at, "2020-11-11 00:00:00");
    //assert_le!(scheduled_at_parsed.timestamp(), end.timestamp());
    Ok(())
}

#[test]
fn test_add_scheduled_at_tmr() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    let expected = Utc::now() + Duration::days(1);
    database_manager
        .add(
            "Test Body",
            &Some(3),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("tmr"),
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .conn
        .prepare("SELECT * FROM task INNER JOIN context on context_id = context.id")
        .expect("");
    let result = tasks.query(NO_PARAMS);

    assert_eq!(result.iter().count(), 1);

    let mut rows = result.unwrap();

    let tasks = convert_rows_into_task(&mut rows);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].scheduled_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    assert_ge!(scheduled_at_parsed, expected.date());
    Ok(())
}

#[test]
fn test_add_completed_task() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec!["Completed".to_owned()],
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let mut operation = GetAllForContextOperation {
        database_manager: Some(database_manager),
        context_name: Some("Inbox".to_string()),
        result: vec![],
    };

    execute(&mut operation);
    assert_eq!(operation.result.iter().count(), 1);
    assert_eq!(&operation.result[0].is_completed, &true);
    Ok(())
}
