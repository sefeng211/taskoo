use crate::core::Operation;
use std::error::Error;
use chrono::{Date, DateTime, Duration, Local, NaiveDate, Utc, NaiveDateTime};
use rusqlite::{Result, NO_PARAMS};
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_helper::convert_rows_into_task;
use crate::db::task_manager::TaskManager;
use crate::operation::{Add, execute};
use crate::error::CoreError;

use more_asserts::*;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("context".to_owned(), "inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_add_simple() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].date_created, "%Y-%m-%d %H:%M:%S").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "Test Body");
    assert_eq!(tasks[0].priority, "");
    assert_eq!(tasks[0].context, "inbox");
    // TODO: Improve the assert_eq here to ensure the auto created `created_at` timestamp is
    // correct
    assert_eq!(created_at_datetime, current_datetime.date());
    assert_eq!(tasks[0].date_due.is_empty(), true);
    assert_eq!(tasks[0].date_scheduled.is_empty(), true);
    //assert_eq!(tasks[0].is_repeat, 0);
    //assert_eq!(tasks[0].is_recurrence, 0);
    assert_eq!(tasks[0].state, "ready");

    Ok(())
}

#[test]
fn test_add_complex() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    println!("{}", &tasks[0].date_created);
    let created_at_datetime =
        NaiveDateTime::parse_from_str(&tasks[0].date_created, "%Y-%m-%d %H:%M:%S").expect("");

    let current_date: Date<Local> = Local::today();

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "Test Body");
    assert_eq!(tasks[0].context, "work");
    assert_eq!(tasks[0].priority, "h");
    // TODO: Improve the assert_eq here to ensure the auto created `created_at` timestamp is
    // correct
    assert_eq!(created_at_datetime.date(), current_date.naive_local());
    assert_eq!(tasks[0].date_due.is_empty(), true);
    assert_eq!(tasks[0].date_scheduled.is_empty(), true);
    //assert_eq!(tasks[0].is_repeat, 0);
    //assert_eq!(tasks[0].is_recurrence, 0);
    assert_eq!(tasks[0].is_completed(), false);

    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_exist_tag() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Ready".to_owned(), "Blocked".to_owned()];
    execute(&mut operation)?;

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

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_days() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::days(2);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("2days");
    execute(&mut operation)?;
    let end = Local::today() + Duration::days(2);

    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDateTime::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");

    assert_ge!(scheduled_at_parsed.date(), start.naive_local());
    assert_le!(scheduled_at_parsed.date(), end.naive_local());
    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_hours() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::hours(11);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("11hours");
    execute(&mut operation)?;

    let end = Local::today() + Duration::hours(11);
    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");

    assert_ge!(scheduled_at_parsed, start.naive_local());
    assert_le!(scheduled_at_parsed, end.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_weeks() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::weeks(1);
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("1weeks");
    execute(&mut operation)?;

    let end = Local::today() + Duration::weeks(1);
    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");

    assert_ge!(scheduled_at_parsed, start.naive_local());
    assert_le!(scheduled_at_parsed, end.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_raw_timestamp() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("2020-11-11");
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(&tasks[0].date_scheduled, "2020-11-11 00:00:00");
    Ok(())
}

#[test]
fn test_add_scheduled_at_tmr() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(1);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("tmr");
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");

    assert_ge!(scheduled_at_parsed, expected.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_today() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.priority = Some(String::from("H"));
    operation.context = Some(String::from("Work"));
    operation.tags = vec!["Blocked".to_owned()];
    operation.date_scheduled = Some("today");
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");

    assert_ge!(scheduled_at_parsed, expected.naive_local());
    Ok(())
}
#[test]
fn test_add_completed_task() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.set_state_to_completed();
    execute(&mut operation)?;

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(&rows[0].is_completed(), &true);
    Ok(())
}

#[test]
fn test_add_repeat_scheduled_task() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.tags = vec!["Completed".to_owned()];
    operation.date_scheduled = Some("2weeks");
    operation.repetition_scheduled = Some("3weeks");
    execute(&mut operation)?;

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    let expected = Local::today() + Duration::weeks(2);

    assert_eq!(tasks.len(), 1);
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");
    assert_ge!(scheduled_at_parsed, expected.naive_local());
    assert_ge!(tasks[0].state, "ready".to_string());

    database_manager
        .modify(
            &vec![1],
            &None,
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &None,
            &None,
            &Some("completed"),
            &vec![],
        )
        .unwrap();

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    let expected = Local::today() + Duration::weeks(3);
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d %H:%M:%S").expect("");
    assert_ge!(scheduled_at_parsed, expected.naive_local());
    assert_ge!(tasks[0].state, "completed".to_string());
    Ok(())
}

#[test]
fn test_add_repeat_due_task() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.tags = vec!["Completed".to_owned()];
    operation.date_due = Some("2weeks");
    operation.repetition_due = Some("3weeks");
    execute(&mut operation)?;

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    let expected = Local::now() + Duration::weeks(2);

    assert_eq!(tasks.len(), 1);
    let due_date_parsed =
        NaiveDateTime::parse_from_str(&tasks[0].date_due, "%Y-%m-%d %H:%M:%S").expect("");
    assert_ge!(
        due_date_parsed.to_string(),
        expected.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    assert_ge!(tasks[0].state, "ready".to_string());

    database_manager
        .modify(
            &vec![1],
            &None,
            &None,
            &None,
            &vec![],
            &None,
            &None,
            &None,
            &None,
            &Some("completed"),
            &vec![],
        )
        .unwrap();

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    let expected = Local::now() + Duration::weeks(3);
    let due_date_parsed =
        NaiveDateTime::parse_from_str(&tasks[0].date_due, "%Y-%m-%d %H:%M:%S").expect("");
    assert_ge!(due_date_parsed.date(), expected.date().naive_local());
    assert_ge!(tasks[0].state, "completed".to_string());
    Ok(())
}

#[test]
fn test_add_annotation() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].annotation, "");

    database_manager
        .add_annotation(1, String::from("This is my annotation"))
        .unwrap();

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();
    assert_eq!(tasks[0].annotation, String::from("This is my annotation"));

    Ok(())
}

#[test]
fn test_add_dependency() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body 2", &mut database_manager);
    operation.parent_task_ids = Some(vec![1, 2]);
    execute(&mut operation)?;

    assert_eq!(operation.get_result().len(), 1);
    assert_eq!(operation.get_result()[0].parent_task_ids, vec!["1", "2"]);
    assert_eq!(operation.get_result()[0].is_blocked(), true);
    Ok(())
}

#[test]
fn test_add_dependency_parent_not_exist() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body 2", &mut database_manager);
    operation.parent_task_ids = Some(vec![1, 2]);

    if let Err(CoreError::ArgumentError(_)) = execute(&mut operation) {
        return Ok(());
    }

    Err(CoreError::UnexpetedError(String::from(
        "Parent task doesn't exist, should crash",
    )))
}
