use rusqlite::{Result, NO_PARAMS};
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_helper::convert_rows_into_task;
use crate::db::task_manager::TaskManager;
use chrono::{Date, DateTime, Duration, Local, NaiveDate, Utc};

use more_asserts::*;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("context".to_owned(), "inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_add_simple() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

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
            &None,
            &None,
        )
        .unwrap();

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].date_created, "%Y-%m-%d").expect(""),
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
    assert_eq!(tasks[0].state_name, "ready");

    Ok(())
}

#[test]
fn test_add_complex() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec![],
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&tasks[0].date_created, "%Y-%m-%d").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "Test Body");
    assert_eq!(tasks[0].context, "Work");
    assert_eq!(tasks[0].priority, "h");
    // TODO: Improve the assert_eq here to ensure the auto created `created_at` timestamp is
    // correct
    assert_eq!(created_at_datetime, current_datetime.date());
    assert_eq!(tasks[0].date_due.is_empty(), true);
    assert_eq!(tasks[0].date_scheduled.is_empty(), true);
    //assert_eq!(tasks[0].is_repeat, 0);
    //assert_eq!(tasks[0].is_recurrence, 0);
    assert_eq!(tasks[0].is_completed(), false);

    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_exist_tag() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Ready".to_owned(), "Blocked".to_owned()],
            &None,
            &None,
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

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_days() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::days(2);
    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("2days"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let end = Local::today() + Duration::days(2);

    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_ge!(scheduled_at_parsed, start.naive_local());
    assert_le!(scheduled_at_parsed, end.naive_local());
    Ok(())
}

// Performing the add query should also add the tag
#[test]
fn test_add_scheduled_at_hours() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::hours(11);
    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("11hours"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let end = Local::today() + Duration::hours(11);
    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_ge!(scheduled_at_parsed, start.naive_local());
    assert_le!(scheduled_at_parsed, end.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_weeks() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    let start = Local::today() + Duration::weeks(1);
    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("1weeks"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let end = Local::today() + Duration::weeks(1);
    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_ge!(scheduled_at_parsed, start.naive_local());
    assert_le!(scheduled_at_parsed, end.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_raw_timestamp() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("2020-11-11"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(&tasks[0].date_scheduled, "2020-11-11");
    Ok(())
}

#[test]
fn test_add_scheduled_at_tmr() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(1);
    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("tmr"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_ge!(scheduled_at_parsed, expected.naive_local());
    Ok(())
}

#[test]
fn test_add_scheduled_at_today() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    let expected = Local::today() + Duration::days(0);
    database_manager
        .add(
            "Test Body",
            &Some(String::from("H")),
            &Some(String::from("Work")),
            &vec!["Blocked".to_owned()],
            &None,
            &Some("today"),
            &None,
            &None,
            &None,
            &None,
        )
        .expect("");

    let mut tasks = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    // Scheduled_at should be in between start and end;
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");

    assert_ge!(scheduled_at_parsed, expected.naive_local());
    Ok(())
}
#[test]
fn test_add_completed_task() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

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
            &None,
            &Some(String::from("completed")),
        )
        .expect("");

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(&rows[0].is_completed(), &true);
    Ok(())
}

#[test]
fn test_add_repeat_scheduled_task() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec!["Completed".to_owned()],
            &None,
            &Some("2weeks"),
            &None,
            &Some("3weeks"),
            &None,
            &None,
        )
        .expect("");

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    let expected = Local::today() + Duration::weeks(2);

    assert_eq!(tasks.len(), 1);
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");
    assert_ge!(scheduled_at_parsed, expected.naive_local());
    assert_ge!(tasks[0].state_name, "ready".to_string());

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
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    let expected = Local::today() + Duration::weeks(3);
    let scheduled_at_parsed =
        NaiveDate::parse_from_str(&tasks[0].date_scheduled, "%Y-%m-%d").expect("");
    assert_ge!(scheduled_at_parsed, expected.naive_local());
    assert_ge!(tasks[0].state_name, "completed".to_string());
    Ok(())
}

#[test]
fn test_add_repeat_due_task() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &None,
            &vec!["Completed".to_owned()],
            &Some("2weeks"),
            &None,
            &Some("3weeks"),
            &None,
            &None,
            &None,
        )
        .expect("");

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    let expected = Local::now() + Duration::weeks(2);

    assert_eq!(tasks.len(), 1);
    let due_date_parsed = NaiveDate::parse_from_str(&tasks[0].date_due, "%Y-%m-%d").expect("");
    assert_ge!(due_date_parsed, expected.date().naive_local());
    assert_ge!(tasks[0].state_name, "ready".to_string());

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
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    let expected = Local::now() + Duration::weeks(3);
    let due_date_parsed = NaiveDate::parse_from_str(&tasks[0].date_due, "%Y-%m-%d").expect("");
    assert_ge!(due_date_parsed, expected.date().naive_local());
    assert_ge!(tasks[0].state_name, "completed".to_string());
    Ok(())
}

#[test]
fn test_add_annotation() -> Result<()> {
    let mut database_manager = TaskManager::new(&get_setting());

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
            &None,
            &None,
        )
        .unwrap();

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].annotation, "");

    database_manager
        .add_annotation(1, String::from("This is my annotation"))
        .unwrap();

    let mut tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();
    assert_eq!(tasks[0].annotation, String::from("This is my annotation"));

    Ok(())
}
