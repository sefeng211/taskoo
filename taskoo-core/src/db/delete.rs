use crate::db::task_helper::Task;
use rusqlite::{named_params, Connection, Error as DbError, Result};

pub fn delete(conn: &Connection, task_ids: &Vec<i64>) -> Result<Vec<Task>, DbError> {
    // TODO query on tag id
    // TODO convert to and_then
    if task_ids.is_empty() {
        return Ok(vec![]);
    }

    let mut statement = conn.prepare(
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
        statement.execute_named(named_params! {
            ":task_id": task_id,
        })?;
    }

    Ok(vec![])
}
