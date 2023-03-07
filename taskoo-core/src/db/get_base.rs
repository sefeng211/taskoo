use crate::error::CoreError;
use crate::db::task_helper::{convert_rows_into_task, Task};
use log::debug;
use rusqlite::{Result, Transaction};

pub fn get_base(tx: &Transaction, conditions: &str) -> Result<Vec<Task>, CoreError> {
    let mut query = String::from("
    SELECT task.id as id, body, priority_task.name as priority, created_at, due_date, scheduled_at, due_repeat, scheduled_repeat, context.name as context, state.name as state, task.annotation, GROUP_CONCAT(DISTINCT task_tag.tag_id) as concat_tag_ids, GROUP_CONCAT(DISTINCT task_tag.name) as concat_tag_names, GROUP_CONCAT(dependency.parent_task_id) as parent_task_ids FROM task
    INNER JOIN task_context
    on task.id = task_context.task_id
    INNER JOIN context
    on task_context.context_id = context.id
    LEFT JOIN
        (
        SELECT task_tag.task_id, task_tag.tag_id, tag.name FROM task_tag
        INNER JOIN tag ON task_tag.tag_id = tag.id
        ) task_tag
    ON task.id = task_tag.task_id
    INNER JOIN task_state
    on task.id = task_state.task_id
    INNER JOIN state
    on task_state.state_id = state.id
    LEFT JOIN
        (
        SELECT priority.name, priority_task.task_id FROM priority
        INNER JOIN priority_task ON priority_task.priority_id = priority.id
        ) priority_task
    on task.id = priority_task.task_id
    LEFT JOIN dependency
    ON task.id = dependency.task_id");

    if !conditions.is_empty() {
        query.push_str(format!(" Where {}", conditions).as_str());
    }

    query.push_str(" Group By task.id");

    debug!("Running select query \n{}", query);
    let mut statement = tx.prepare(&query)?;

    // let names = statement
    //     .column_names()
    //     .into_iter()
    //     .map(|s| String::from(s))
    //     .collect::<Vec<_>>();

    // println!("Names {:?}", names);

    let mut rows = statement.query([])?;
    return Ok(convert_rows_into_task(&mut rows));
}
