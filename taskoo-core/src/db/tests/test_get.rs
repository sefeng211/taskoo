use chrono::{DateTime, NaiveDate, Date, Utc, NaiveDateTime};
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
fn test_get_simple() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);

    operation.date_scheduled = Some("2weeks");
    operation.repetition_due = Some("2weeks");

    execute(&mut operation)?;
    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &Some(1))
        .unwrap();

    assert_eq!(rows.len(), 1);
    let created_at_datetime = DateTime::<Utc>::from_utc(
        NaiveDateTime::parse_from_str(&rows[0].date_created, "%Y-%m-%d %H:%M:%S").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(rows[0].id, 1);
    assert_eq!(rows[0].body, "Test Body");
    assert_eq!(rows[0].priority, "");
    assert_eq!(rows[0].context, "inbox");
    assert_eq!(created_at_datetime.date(), current_datetime.date());
    assert_eq!(rows[0].date_due.is_empty(), true);
    assert_eq!(rows[0].date_scheduled.is_empty(), false);
    //assert_eq!(rows[0].is_repeat, 1);
    //assert_eq!(rows[0].is_recurrence, 0);

    Ok(())
}

#[test]
fn test_get_all_for_context() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.context = Some(String::from("Work"));
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.context = Some(String::from("Work"));
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.context = Some(String::from("Life"));
    execute(&mut operation)?;

    let rows = database_manager
        .get(
            &None,
            &Some("work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(rows.len(), 2);

    Ok(())
}

#[test]
fn test_get_with_tag_ids() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.tags = vec!["Blocked".to_owned(), "Completed".to_owned()];
    execute(&mut operation)?;

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    operation.tags = vec!["Blocked".to_owned(), "Completed".to_owned()];
    execute(&mut operation)?;

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(rows.len(), 3);

    let rows = database_manager
        .get(
            &None,
            &None,
            &vec!["completed".to_string()],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].tags, vec!["blocked", "completed"]);
    assert_eq!(rows[1].tags, vec!["blocked", "completed"]);
    Ok(())
}
