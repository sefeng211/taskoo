use crate::db::task_helper::{Task, convert_rows_into_task};
use crate::error::CoreError;
use super::get_base::get_base;
use log::debug;
use rusqlite::{named_params, Result, Transaction};

fn add_context(conn: &Transaction, task_id: &i64, context_id: i64) -> Result<(), CoreError> {
    debug!(
        "Adding new context, task_id={}, context_id={}",
        &task_id, &context_id
    );
    let mut statement = conn.prepare(
        "INSERT INTO task_context
        (task_id, context_id)
        VALUES (:task_id, :context_id)",
    )?;
    statement.execute_named(named_params! {
    ":task_id": task_id,
    ":context_id": context_id})?;
    Ok(())
}

fn add_state(conn: &Transaction, task_id: &i64, state_id: &Option<i64>) -> Result<(), CoreError> {
    debug!(
        "Adding new state, task_id={}, state_id={:?}",
        task_id, state_id
    );
    let mut statement = conn.prepare(
        "INSERT INTO task_state
        (task_id, state_id)
        VALUES (:task_id, :state_id)",
    )?;
    statement.execute_named(named_params! {
    ":task_id": task_id,
    ":state_id": state_id.unwrap_or(1)})?;
    Ok(())
}

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
    debug!("  parent_task_ids: {:?}", parent_task_ids);
    debug!("  state_id: {:?}", state_id);
    let mut statement = tx.prepare(
        "
    INSERT INTO task
    (body, due_date, scheduled_at, due_repeat, scheduled_repeat, annotation) VALUES
    (:body, :due_date, :scheduled_at, :due_repeat, :scheduled_repeat, :annotation)",
    )?;

    statement.execute_named(named_params! {
        ":body": body,
        ":due_date": due_date.unwrap_or(""),
        ":scheduled_at": scheduled_at.unwrap_or(""),
        ":due_repeat": due_repeat.unwrap_or(""),
        ":scheduled_repeat": scheduled_repeat.unwrap_or(""),
        ":annotation": annotation.unwrap_or("")
    })?;

    let inserted_task_id = tx.last_insert_rowid();

    add_tag(&tx, &inserted_task_id, tag_ids)?;
    add_context(&tx, &inserted_task_id, *context_id)?;
    add_state(&tx, &inserted_task_id, state_id)?;

    if let Some(priority_id) = priority {
        add_priority(&tx, &inserted_task_id, &priority_id)?;
    }

    if let Some(parent_task_ids) = parent_task_ids {
        for parent_task_id in parent_task_ids.into_iter() {
            add_dependency(&tx, &inserted_task_id, &parent_task_id)?;
        }
    }

    let tasks = get_base(&tx, &format!("task.id = {}", inserted_task_id))?;
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
