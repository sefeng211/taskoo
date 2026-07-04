use rusqlite::Result;
use std::collections::HashMap;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::db::task_manager::TaskManager;
use crate::error::CoreError;
use crate::operation::{Add, execute};
use crate::core::Operation;

fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    setting.insert("db_path".to_owned(), ":memory:".to_owned());
    setting.insert("tag".to_owned(), "Ready, Blocked".to_owned());
    setting.insert("context".to_owned(), "inbox, Work, Life".to_owned());
    return setting;
}

#[test]
fn test_modify_single() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
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
            &vec![],
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
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "New Body");
    assert_eq!(tasks[0].priority, "h");
    assert_eq!(tasks[0].context, "Work");
    assert_eq!(tasks[0].date_due, "2020-11-10 00:00:00");
    assert_eq!(tasks[0].date_scheduled, "2020-11-11 00:00:00");
    //assert_eq!(tasks[0].is_repeat, 1);
    //assert_eq!(tasks[0].is_recurrence, 1);

    Ok(())
}

#[test]
fn test_modify_single_with_tag() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());
    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;

    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
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
            &vec![],
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
            &None,
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].body, "New Body");
    assert_eq!(tasks[0].tags, ["Blocked".to_string()]);
    assert_eq!(tasks[0].context, "Work");
    assert_eq!(tasks[0].date_due, "2020-11-10 00:00:00");
    assert_eq!(tasks[0].date_scheduled, "2020-11-11 00:00:00");
    //assert_eq!(tasks[0].is_repeat, 1);
    //assert_eq!(tasks[0].is_recurrence, 1);

    Ok(())
}

#[test]
fn test_modify_tag_only() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Test Body", &mut database_manager);
    execute(&mut operation)?;
    let tasks = database_manager
        .get(&None, &None, &vec![], &None, &None, &None, &None)
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
            &vec![],
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
            &None
        )
        .unwrap();

    assert_eq!(tasks.len(), 1);

    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].tags, ["Blocked".to_string()]);

    Ok(())
}

#[test]
fn test_modify_task_to_complete_should_update_dependency() -> Result<(), CoreError> {
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

    database_manager.modify(
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
    )?;

    let tasks = database_manager.get(&None, &None, &vec![], &None, &None, &Some(3), &None)?;

    assert_eq!(tasks[0].is_blocked(), true);

    database_manager.modify(
        &vec![2],
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
    )?;
    let tasks = database_manager.get(&None, &None, &vec![], &None, &None, &Some(3), &None)?;
    assert_eq!(tasks[0].is_blocked(), false);
    assert_eq!(tasks[0].is_ready(), true);
    Ok(())
}

#[test]
fn test_modify_multiple_tasks_with_web_command_options() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Task One", &mut database_manager);
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Task Two", &mut database_manager);
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Task Three", &mut database_manager);
    execute(&mut operation)?;

    database_manager.modify(
        &vec![1, 2],
        &None,
        &Some(String::from("H")),
        &Some("work".to_string()),
        &vec!["next".to_string(), "waiting".to_string()],
        &Some("2026-07-10"),
        &Some("2026-07-08"),
        &Some("weekly"),
        &Some("daily"),
        &Some("started"),
        &vec![],
    )?;

    for task_id in [1, 2] {
        let tasks =
            database_manager.get(&None, &None, &vec![], &None, &None, &Some(task_id), &None)?;
        assert_eq!(tasks.len(), 1);
        let task = &tasks[0];

        assert_eq!(task.priority, "h");
        assert_eq!(task.context, "work");
        assert_eq!(task.state, "started");
        assert_eq!(task.tags, vec!["next", "waiting"]);
        assert_eq!(task.date_due, "2026-07-10 00:00:00");
        assert_eq!(task.date_scheduled, "2026-07-08 00:00:00");
        assert_eq!(task.repetition_due, "weekly");
        assert_eq!(task.repetition_scheduled, "daily");
    }

    let untouched = database_manager.get(&None, &None, &vec![], &None, &None, &Some(3), &None)?;
    assert_eq!(untouched.len(), 1);
    assert_eq!(untouched[0].context, "inbox");
    assert_eq!(untouched[0].state, "ready");

    Ok(())
}

#[test]
fn test_modify_multiple_tasks_can_remove_tags() -> Result<(), CoreError> {
    let mut database_manager = TaskManager::new(&get_setting());

    let mut operation = Add::new_with_task_manager("Task One", &mut database_manager);
    operation.tags = vec!["Ready".to_string(), "Blocked".to_string()];
    execute(&mut operation)?;
    let mut operation = Add::new_with_task_manager("Task Two", &mut database_manager);
    operation.tags = vec!["Ready".to_string(), "Blocked".to_string()];
    execute(&mut operation)?;

    database_manager.modify(
        &vec![1, 2],
        &None,
        &None,
        &None,
        &vec![],
        &None,
        &None,
        &None,
        &None,
        &None,
        &vec!["blocked".to_string()],
    )?;

    let tasks = database_manager.get(&None, &None, &vec!["ready".to_string()], &None, &None, &None, &None)?;

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].tags, vec!["ready"]);
    assert_eq!(tasks[1].tags, vec!["ready"]);

    Ok(())
}
