use crate::db::task_helper::Task;
use crate::error::CoreError;
use rusqlite::{named_params, Result, Transaction};
use rusqlite::Statement;

pub fn delete(conn: &Transaction, task_ids: &Vec<i64>) -> Result<Vec<Task>, CoreError> {
    if task_ids.is_empty() {
        return Ok(vec![]);
    }

    let delete_tag_state = conn.prepare("DELETE FROM task_tag where task_id = :task_id")?;
    let delete_priority = conn.prepare("DELETE FROM priority_task where task_id = :task_id")?;
    let delete_from_task_context =
        conn.prepare("DELETE FROM task_context where task_id = :task_id")?;
    let delete_from_task_state =
        conn.prepare("DELETE FROM task_state where task_id = :task_id")?;
    let delete_dependency = conn
        .prepare("DELETE FROM dependency where task_id = :task_id or parent_task_id =:task_id;")?;
    let delete_task = conn.prepare("DELETE FROM task where id = :task_id;")?;

    let delete_stmt_to_run: &mut [Statement; 6] = &mut [
        delete_tag_state,
        delete_priority,
        delete_from_task_state,
        delete_from_task_context,
        delete_dependency,
        delete_task,
    ];

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
        for stmt in delete_stmt_to_run.into_iter() {
            stmt.execute_named(named_params! {
                ":task_id": task_id,
            })?;
        }
    }

    Ok(vec![])
}
