use crate::db::task_helper::Task;
use crate::error::TaskooError;
use rusqlite::{named_params, Result, Transaction};

pub fn delete(conn: &Transaction, task_ids: &Vec<i64>) -> Result<Vec<Task>, TaskooError> {
    if task_ids.is_empty() {
        return Ok(vec![]);
    }

    let mut delete_tag_state = conn.prepare("DELETE FROM task_tag where task_id = :task_id")?;
    let mut delete_priority = conn.prepare("DELETE FROM priority_task where task_id = :task_id")?;
    let mut delete_task_state = conn.prepare(
        "
            DELETE FROM task where id = :task_id;
    ",
    )?;

    let task_ids_str: Vec<String> = task_ids.into_iter().map(|i| i.to_string()).collect();

    // Okay, it's super stupid that we need to execute the delete
    // query for each task_id, however this seems to be the limitation
    // of rusqlite. I've tried to use IN operator, however it doesn't
    // work.
    //
    // Another approach would be creating a temporary table and insert
    // the task ids to that table and then do a delete query based on
    // that temporary table.
    for task_id in task_ids_str.iter() {
        delete_tag_state.execute_named(named_params! {
            ":task_id": task_id,
        })?;
        delete_priority.execute_named(named_params! {
            ":task_id": task_id,
        })?;
        delete_task_state.execute_named(named_params! {
            ":task_id": task_id,
        })?;
    }

    Ok(vec![])
}
