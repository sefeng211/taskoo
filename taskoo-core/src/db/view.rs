use super::query_helper::generate_view_condition;
use crate::db::task_helper::{convert_rows_into_task, Task};
use log::debug;
use rusqlite::{Connection, Error as DbError, Result, NO_PARAMS};

pub fn view(
    conn: &Connection,
    context_id: &i64,
    view_range_start: &Option<String>,
    view_range_end: &String,
    view_type: &Option<String>,
) -> Result<Vec<Task>, DbError> {
    let conditions =
        generate_view_condition(context_id, view_range_start, view_range_end, view_type);

    assert!(!conditions.is_empty());

    let final_argument = format!(
        "
    SELECT *, GROUP_CONCAT(task_tag.tag_id) as concat_tag_ids, GROUP_CONCAT(task_tag.name) FROM task
    INNER JOIN context
    on context_id = context.id
    LEFT JOIN
        (
        SELECT task_tag.task_id, task_tag.tag_id, tag.name FROM task_tag
        INNER JOIN tag ON task_tag.tag_id = tag.id
        ) task_tag
    ON task.id = task_tag.task_id
    Where {}
    Group By task.id
    ",
        conditions.join(" and ")
    );

    debug!("Running select query \n{}", final_argument);
    let mut statement = conn.prepare(&final_argument)?;
    let mut rows = statement.query(NO_PARAMS)?;
    let tasks = convert_rows_into_task(&mut rows);

    Ok(tasks)
}
