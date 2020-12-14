use chrono::{DateTime, NaiveDate, Date, Utc};
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
fn test_get_simple() -> Result<()> {
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
    let created_at_datetime = Date::<Utc>::from_utc(
        NaiveDate::parse_from_str(&rows[0].created_at, "%Y-%m-%d").expect(""),
        Utc,
    );

    let current_datetime: DateTime<Utc> = Utc::now();

    assert_eq!(rows[0].id, 1);
    assert_eq!(rows[0].body, "Test Body");
    assert_eq!(rows[0].priority, 1);
    assert_eq!(rows[0].context_name, "Inbox");
    assert_eq!(created_at_datetime, current_datetime.date());
    assert_eq!(rows[0].due_date.is_empty(), true);
    assert_eq!(rows[0].scheduled_at.is_empty(), true);
    assert_eq!(rows[0].is_repeat, 1);
    assert_eq!(rows[0].is_recurrence, 0);

    Ok(())
}

#[test]
fn test_get_all_for_context() -> Result<()> {
    let mut database_manager = DatabaseManager::new(&get_setting());

    database_manager
        .add(
            "Test Body",
            &None,
            &Some("Work".to_string()),
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
            &Some("Work".to_string()),
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
            &Some("Life".to_string()),
            &vec![],
            &None,
            &None,
            &Some(1),
            &None,
        )
        .expect("");

    let rows = database_manager
        .get(
            &None,
            &Some("Work".to_string()),
            &vec![],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(rows.len(), 2);

    Ok(())
}

#[test]
fn test_get_with_tag_ids() -> Result<()> {
    println!("Start the test");
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
            &vec!["Blocked".to_owned(), "Completed".to_owned()],
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
            &vec!["Blocked".to_owned(), "Completed".to_owned()],
            &None,
            &None,
            &Some(1),
            &None,
        )
        .expect("");

    let rows = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
        .unwrap();

    assert_eq!(rows.len(), 3);

    let rows = database_manager
        .get(
            &None,
            &None,
            &vec!["Completed".to_string()],
            &None,
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].tag_names, vec!["Blocked", "Completed"]);
    assert_eq!(rows[1].tag_names, vec!["Blocked", "Completed"]);
    Ok(())
}
