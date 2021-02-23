use rusqlite::Result;
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_manager::DatabaseManager;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked".to_owned());
    setting.insert("context".to_owned(), "inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_modify_single() -> Result<()> {
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
            &None,
            &None,
        )
        .unwrap();

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    database_manager
        .modify(
            &vec![1],
            &Some("New Body"),
            &Some(String::from("H")),
            &Some("Work".to_string()),
            &vec![],
            &Some("2020-11-10"),
            &Some("2020-11-11"),
            &None,
            &None,
            &None,
            &vec![]
        )
        .unwrap();

    let tasks = database_manager
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

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "New Body");
    assert_eq!(tasks[0].priority, "h");
    assert_eq!(tasks[0].context, "Work");
    assert_eq!(tasks[0].date_due, "2020-11-10");
    assert_eq!(tasks[0].date_scheduled, "2020-11-11");
    //assert_eq!(tasks[0].is_repeat, 1);
    //assert_eq!(tasks[0].is_recurrence, 1);

    Ok(())
}

#[test]
fn test_modify_single_with_tag() -> Result<()> {
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
            &None,
            &None,
        )
        .unwrap();

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    database_manager
        .modify(
            &vec![1],
            &Some("New Body"),
            &Some(String::from("H")),
            &Some("Work".to_string()),
            &vec!["Blocked".to_string()],
            &Some("2020-11-10"),
            &Some("2020-11-11"),
            &None,
            &None,
            &None,
            &vec![]
        )
        .unwrap();

    let tasks = database_manager
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

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "New Body");
    assert_eq!(tasks[0].tags, ["Blocked".to_string()]);
    assert_eq!(tasks[0].context, "Work");
    assert_eq!(tasks[0].date_due, "2020-11-10");
    assert_eq!(tasks[0].date_scheduled, "2020-11-11");
    //assert_eq!(tasks[0].is_repeat, 1);
    //assert_eq!(tasks[0].is_recurrence, 1);

    Ok(())
}

#[test]
fn test_modify_tag_only() -> Result<()> {
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
            &None,
            &None,
        )
        .unwrap();

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None)
        .unwrap();

    assert_eq!(tasks.len(), 1);

    database_manager
        .modify(
            &vec![1],
            &None,
            &None,
            &None,
            &vec!["Blocked".to_string()],
            &None,
            &None,
            &None,
            &None,
            &None,
            &vec![]
        )
        .unwrap();

    let tasks = database_manager
        .get(
            &None,
            &Some("inbox".to_string()),
            &vec![],
            &None,
            &None,
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].tags, ["Blocked".to_string()]);

    Ok(())
}

