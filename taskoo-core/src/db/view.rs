use super::query_helper::generate_view_condition;
use log::info;
use crate::db::task_helper::{convert_rows_into_task, Task};
use crate::error::CoreError;
use log::debug;
use rusqlite::{Transaction, Result, NO_PARAMS};

pub fn view(
    conn: &Transaction,
    context_id: &i64,
    view_range_start: &Option<String>,
    view_range_end: &String,
    view_type: &Option<String>,
) -> Result<Vec<Task>, CoreError> {
    info!("[view] view_range_start={:?}", view_range_start);
    let conditions =
        generate_view_condition(context_id, view_range_start, view_range_end, view_type);

    assert!(!conditions.is_empty());

    let final_argument = format!(
        "
    SELECT task.id as id, body, priority_task.name, created_at, due_date, scheduled_at, due_repeat, scheduled_repeat, context.name, state.name, annotation, GROUP_CONCAT(task_tag.tag_id) as concat_tag_ids, GROUP_CONCAT(task_tag.name) FROM task
    INNER JOIN context
    on context_id = context.id
    LEFT JOIN
        (
        SELECT task_tag.task_id, task_tag.tag_id, tag.name FROM task_tag
        INNER JOIN tag ON task_tag.tag_id = tag.id
        ) task_tag
    ON task.id = task_tag.task_id
    INNER JOIN state
    on state_id = state.id
    LEFT JOIN
        (
        SELECT priority.name, priority_task.task_id FROM priority
        INNER JOIN priority_task ON priority_task.priority_id = priority.id
        ) priority_task
    on task.id = priority_task.task_id
    Where {}
    Group By task.id
    ",
        conditions.join(" and ")
    );

    debug!("[view] Running select query \n{}", final_argument);
    let mut statement = conn.prepare(&final_argument)?;
    let mut rows = statement.query(NO_PARAMS)?;
    let tasks = convert_rows_into_task(&mut rows);

    Ok(tasks)
}
