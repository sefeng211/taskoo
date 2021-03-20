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

pub fn get(
    conn: &Transaction,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    tag_ids: &Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    task_id: &Option<i64>,
) -> Result<Vec<Task>, CoreError> {
    let conditions = match task_id {
        Some(id) => vec![format!("task.id = {}", id)],
        None => generate_get_condition(
            &None,
            context_id,
            due_date,
            scheduled_at,
        ),
    };

    assert!(!conditions.is_empty());

    rusqlite::vtab::array::load_module(&conn)?;

    let tasks = get_base(&conn, &conditions.join(" and "))?;

    if !tag_ids.is_empty() {
        return Ok(tasks
            .into_iter()
            .filter(|task| {
                //task.tag_ids == tag_ids.clone()
                task_matches_tag_ids(task, &tag_ids)
            })
            .collect());
    }

    Ok(tasks)
}
