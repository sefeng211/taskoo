use super::query_helper::generate_get_condition;
use super::get_base::get_base;

use crate::db::task_helper::{convert_rows_into_task, Task};
use crate::error::CoreError;
use log::debug;
use rusqlite::{Result, Transaction, NO_PARAMS};

fn task_matches_tag_ids(task: &Task, tag_ids: &Vec<i64>) -> bool {
    for tag_id in tag_ids.iter() {
        if !task.tag_ids.contains(tag_id) {
            return false;
        }
    }
    return true;
}

fn task_not_matches_tag_ids(task: &Task, not_tag_ids: &Vec<i64>) -> bool {
    for tag_id in not_tag_ids.iter() {
        if task.tag_ids.contains(tag_id) {
            return false;
        }
    }
    return true;
}

pub fn get(
    conn: &Transaction,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    tag_ids: &Vec<i64>,
    date_due: &Option<&str>,
    date_scheduled: &Option<&str>,
    task_id: &Option<i64>,
    not_tag_ids: &Option<Vec<i64>>,
) -> Result<Vec<Task>, CoreError> {
    let conditions = match task_id {
        Some(id) => vec![format!("task.id = {}", id)],
        None => generate_get_condition(&None, context_id, date_due, date_scheduled),
    };

    assert!(!conditions.is_empty());

    rusqlite::vtab::array::load_module(&conn)?;

    let mut tasks = get_base(&conn, &conditions.join(" and "))?;

    // Filter the tags that we'd like to get
    if !tag_ids.is_empty() {
        tasks = tasks
            .into_iter()
            .filter(|task| task_matches_tag_ids(task, &tag_ids))
            .collect();
    }

    if let Some(not_tag_ids) = not_tag_ids {
        tasks = tasks
            .into_iter()
            .filter(|task| task_not_matches_tag_ids(task, &not_tag_ids))
            .collect();
    }
    Ok(tasks)
}
