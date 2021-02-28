use super::query_helper::generate_get_condition;
use crate::db::task_helper::{convert_rows_into_task, Task};
use crate::db::query_helper::GET_QUERY;
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

    let final_argument = format!(
        "
    SELECT task.id as id, body, priority_task.name, created_at, due_date, scheduled_at, due_repeat, scheduled_repeat, context.name, state.name, task.annotation, GROUP_CONCAT(task_tag.tag_id) as concat_tag_ids, GROUP_CONCAT(task_tag.name) FROM task
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

    debug!("Running select query \n{}", final_argument);
    let mut statement = conn.prepare(&final_argument)?;
    let mut rows = statement.query(NO_PARAMS)?;
    let tasks = convert_rows_into_task(&mut rows);

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
