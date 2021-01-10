use crate::db::task_helper::Task;
use crate::error::TaskooError;
use log::debug;
use rusqlite::{named_params, Connection, Result, Transaction};

fn add_tag(conn: &Transaction, tag_ids: Vec<i64>) -> Result<(), TaskooError> {
    debug!("Adding new tag {:?}", &tag_ids);
    let mut statement = conn
        .prepare(
            "INSERT INTO task_tag
        (task_id, tag_id)
        VALUES (:task_id, :tag_id)",
        )
        .expect("Failed to prepare the INSERT INTO task_tag query");
    let last_insert_rowid = conn.last_insert_rowid();
    for id in tag_ids.iter() {
        statement.execute_named(named_params! {
        ":task_id": last_insert_rowid,
        ":tag_id": id})?;
    }
    Ok(())
}

pub fn add(
    tx: &mut Transaction,
    body: &str,
    priority: &Option<u8>,
    context_id: &i64,
    tag_ids: Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    due_repeat: &Option<&str>,
    scheduled_repeat: &Option<&str>,
    state_id: &Option<i64>,
) -> Result<Vec<Task>, TaskooError> {
    let mut statement = tx.prepare(
        "
    INSERT INTO task
    (body, priority, context_id, due_date, scheduled_at, due_repeat, scheduled_repeat, state_id) VALUES
    (:body, :priority, :context_id, :due_date, :scheduled_at, :due_repeat, :scheduled_repeat, :state_id)",
    )?;

    statement.execute_named(named_params! {
        ":body": body,
        ":priority": priority.unwrap_or(0),
        ":context_id": context_id,
        ":due_date": due_date.unwrap_or(""),
        ":scheduled_at": scheduled_at.unwrap_or(""),
        ":due_repeat": due_repeat.unwrap_or(""),
        ":scheduled_repeat": scheduled_repeat.unwrap_or(""),
        ":state_id": state_id.unwrap_or(1)
    })?;

    add_tag(&tx, tag_ids)?;

    Ok(vec![])
}
