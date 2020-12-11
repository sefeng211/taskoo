use crate::db::task_helper::{convert_rows_into_task, Task};
use rusqlite::{NO_PARAMS, named_params, Connection, Error as DbError, Result};

pub fn get(
    conn: &Connection,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    _tag_ids: &Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    is_repeat: &Option<u8>,
    is_recurrence: &Option<u8>,
) -> Result<Vec<Task>, DbError> {
    // TODO query on tag id
    let mut statement = conn.prepare(
        "
    SELECT *, GROUP_CONCAT(task_tag.name) FROM task
    INNER JOIN context
    on context_id = context.id
    LEFT JOIN
        (
        SELECT task_tag.task_id, task_tag.tag_id, tag.name FROM task_tag
        INNER JOIN tag ON task_tag.tag_id = tag.id
        ) task_tag
    ON task.id = task_tag.task_id
    WHERE priority = :priority and
    context_id = :context_id and
    due_date = :due_date and
    scheduled_at = :scheduled_at and
    is_repeat = :is_repeat and
    is_recurrence = :is_recurrence
    Group By task.id
    "
    )?;

    let mut rows = statement.query_named(named_params! {
        ":priority": priority.unwrap_or(1),
        ":context_id": context_id.unwrap_or(1),
        ":due_date": due_date.unwrap_or(""),
        ":scheduled_at": scheduled_at.unwrap_or(""),
        ":is_repeat": is_repeat.unwrap_or(0),
        ":is_recurrence": is_recurrence.unwrap_or(0)
    })?;

    let tasks = convert_rows_into_task(&mut rows);
    Ok(tasks)
}

pub fn get_all_for_context(
    conn: &Connection,
    context_id: &Option<i64>,
) -> Result<Vec<Task>, DbError> {
    // TODO query on tag id
    let mut statement = conn.prepare(
        "
    SELECT *, GROUP_CONCAT(task_tag.name) FROM task
    INNER JOIN context
    on context_id = context.id
    LEFT JOIN
        (
        SELECT task_tag.task_id, task_tag.tag_id, tag.name FROM task_tag
        INNER JOIN tag ON task_tag.tag_id = tag.id
        ) task_tag
    ON task.id = task_tag.task_id
    WHERE context_id = :context_id
    Group By task.id
    "
    )?;

    let mut rows = statement.query_named(named_params! {
        ":context_id": context_id.unwrap_or(1),
    })?;

    let tasks = convert_rows_into_task(&mut rows);
    Ok(tasks)
}
