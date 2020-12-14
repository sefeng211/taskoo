use crate::db::task_helper::Task;
use log::{debug};
use rusqlite::{named_params, Connection, Error as DbError, Result, Transaction};

fn add_tag(conn: &Transaction, tag_ids: Vec<i64>) -> Result<(), DbError> {
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
    conn: &mut Connection,
    body: &str,
    priority: &Option<u8>,
    context_id: &i64,
    tag_ids: Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    is_repeat: &Option<u8>,
    is_recurrence: &Option<u8>,
) -> Result<Vec<Task>, DbError> {
    let tx = conn.transaction()?;

    {
        let mut statement = tx.prepare(
            "
    INSERT INTO task
    (body, priority, context_id, due_date, scheduled_at, is_repeat, is_recurrence) VALUES
    (:body, :priority, :context_id, :due_date, :scheduled_at, :is_repeat, :is_recurrence)",
        )?;

        statement.execute_named(named_params! {
            ":body": body,
            ":priority": priority.unwrap_or(1),
            ":context_id": context_id,
            ":due_date": due_date.unwrap_or(""),
            ":scheduled_at": scheduled_at.unwrap_or(""),
            ":is_repeat": is_repeat.unwrap_or(0),
            ":is_recurrence": is_recurrence.unwrap_or(0)
        })?;
    }

    add_tag(&tx, tag_ids)?;

    tx.commit()?;
    Ok(vec![])
}
