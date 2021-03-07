use crate::db::task_helper::{Task, convert_rows_into_task};
use crate::error::CoreError;
use log::debug;
use rusqlite::{named_params, Result, Transaction};

fn add_tag(conn: &Transaction, task_id: &i64, tag_ids: Vec<i64>) -> Result<(), CoreError> {
    debug!("Adding new tag {:?}", &tag_ids);
    let mut statement = conn.prepare(
        "INSERT INTO task_tag
        (task_id, tag_id)
        VALUES (:task_id, :tag_id)",
    )?;
    for id in tag_ids.iter() {
        statement.execute_named(named_params! {
        ":task_id": task_id,
        ":tag_id": id})?;
    }
    Ok(())
}

fn add_priority(conn: &Transaction, task_id: &i64, priority_id: &i64) -> Result<(), CoreError> {
    let mut statement = conn.prepare(
        "INSERT INTO priority_task
        (task_id, priority_id)
        VALUES (:task_id, :priority_id)",
    )?;

    statement.execute_named(named_params! {
        ":task_id": task_id,
        ":priority_id": priority_id
    })?;

    Ok(())
}

fn add_dependency(
    conn: &Transaction,
    task_id: &i64,
    parent_task_id: &i64,
) -> Result<(), CoreError> {
    let mut statement = conn.prepare(
        "INSERT INTO dependency
        (task_id, parent_task_id)
        VALUES (:task_id, :parent_task_id)",
    )?;

    statement.execute_named(named_params! {
        ":task_id": task_id,
        ":parent_task_id": parent_task_id
    })?;

    Ok(())
}

pub fn add(
    tx: &mut Transaction,
    body: &str,
    priority: &Option<i64>,
    context_id: &i64,
    tag_ids: Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    due_repeat: &Option<&str>,
    scheduled_repeat: &Option<&str>,
    annotation: &Option<&str>,
    state_id: &Option<i64>,
    parent_task_ids: &Option<Vec<i64>>,
) -> Result<Vec<Task>, CoreError> {
    let mut statement = tx.prepare(
        "
    INSERT INTO task
    (body, context_id, due_date, scheduled_at, due_repeat, scheduled_repeat, state_id, annotation) VALUES
    (:body, :context_id, :due_date, :scheduled_at, :due_repeat, :scheduled_repeat, :state_id, :annotation)",
    )?;

    statement.execute_named(named_params! {
        ":body": body,
        ":context_id": context_id,
        ":due_date": due_date.unwrap_or(""),
        ":scheduled_at": scheduled_at.unwrap_or(""),
        ":due_repeat": due_repeat.unwrap_or(""),
        ":scheduled_repeat": scheduled_repeat.unwrap_or(""),
        ":state_id": state_id.unwrap_or(1),
        ":annotation": annotation.unwrap_or("")
    })?;

    let inserted_task_id = tx.last_insert_rowid();

    add_tag(&tx, &inserted_task_id, tag_ids)?;

    if let Some(priority_id) = priority {
        add_priority(&tx, &inserted_task_id, &priority_id)?;
    }

    if let Some(parent_task_ids) = parent_task_ids {
        for parent_task_id in parent_task_ids.into_iter() {
            add_dependency(&tx, &inserted_task_id, &parent_task_id)?;
        }
    }

    // TODO: Let's have a generic query statement so that we can reuse it
    // between add.rs, get.rs and view.rs
    let get_last_insert_task_statement = format!(
        "
    SELECT task.id as id, body, priority_task.name, created_at, due_date, scheduled_at, due_repeat, scheduled_repeat, context.name, state.name, task.annotation, GROUP_CONCAT(task_tag.tag_id) as concat_tag_ids, GROUP_CONCAT(task_tag.name), GROUP_CONCAT(dependency.parent_task_id) as parent_task_ids FROM task
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
    LEFT JOIN dependency 
    ON task.id = dependency.task_id
    Where task.id = :task_id
    Group By task.id
    ");

    let mut statement = tx.prepare(&get_last_insert_task_statement)?;
    let mut rows = statement.query_named(named_params! {
        ":task_id": inserted_task_id
    })?;

    let tasks = convert_rows_into_task(&mut rows);

    Ok(tasks)
}

pub fn add_annotation(
    tx: &mut Transaction,
    task_id: i64,
    annotation: String,
) -> Result<Vec<Task>, CoreError> {
    let mut statement = tx.prepare(
        "
        Update task set annotation = :annotation where id = :task_id
        ",
    )?;

    statement.execute_named(named_params! {
        ":annotation": annotation,
        ":task_id": task_id
    })?;

    Ok(vec![])
}
