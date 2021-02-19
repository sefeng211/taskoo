use super::query_helper::generate_condition;
use crate::db::task_helper::Task;
use crate::db::task_manager::DatabaseManager;
use crate::error::TaskooError;
use log::debug;
use log::info;
use rusqlite::{named_params, Result, Transaction, NO_PARAMS};

// XXX Why task_ids is reference?
fn add_tag(conn: &Transaction, task_ids: &Vec<i64>, tag_ids: Vec<i64>) -> Result<(), TaskooError> {
    let mut statement = conn
        .prepare(
            "INSERT OR IGNORE INTO task_tag
        (task_id, tag_id)
        VALUES (:task_id, :tag_id)",
        )
        .expect("Failed to prepare the INSERT INTO task_tag query");
    for task_id in task_ids.iter() {
        for tag_id in tag_ids.iter() {
            statement.execute_named(named_params! {
            ":task_id": task_id,
            ":tag_id": tag_id})?;
        }
    }
    Ok(())
}

fn remove_tag(
    conn: &Transaction,
    task_ids: &Vec<i64>,
    tag_ids: Vec<i64>,
) -> Result<(), TaskooError> {
    info!(
        "Removing tag_ids: {:?} from task_ids {:?}",
        tag_ids, task_ids
    );
    let mut statement = conn
        .prepare("DELETE FROM task_tag WHERE task_id = :task_id and tag_id = :tag_id")
        .expect("Failed to prepare the DELETE FROM task_tag query");

    for task_id in task_ids.iter() {
        for tag_id in tag_ids.iter() {
            debug!("Removing tag_id: {} from task_id {}", tag_id, task_id);
            statement.execute_named(named_params! {
            ":task_id": task_id,
            ":tag_id": tag_id})?;
        }
    }
    Ok(())
}

fn insert_or_replace_priority(
    conn: &Transaction,
    task_ids: &Vec<i64>,
    priority_id: &i64,
) -> Result<(), TaskooError> {
    info!(
        "InsertOrReplacePriority tag_ids: {:?} and priority_id {:?}",
        task_ids, priority_id
    );
    let mut statement = conn.prepare(
        "REPLACE INTO priority_task (task_id, priority_id) VALUES (:task_id, :priority_id)",
    )?;

    for task_id in task_ids.iter() {
        statement.execute_named(named_params! {
        ":task_id": task_id,
        ":priority_id": priority_id})?;
    }
    Ok(())
}
fn update_schedule_at_for_repeat(conn: &Transaction, task_id: &i64) -> Result<(), TaskooError> {
    let get_task_repetition_query = format!(
        "SELECT due_repeat, scheduled_repeat from task where id = {}",
        task_id
    );
    let mut statement = conn.prepare(&get_task_repetition_query)?;
    let mut rows = statement.query(NO_PARAMS)?;

    let first_row = rows.next().expect("We should always have a row");

    let data = first_row.expect("We should always have some data");

    let due_repetition: String = data.get(0)?;
    let scheduled_repeat: String = data.get(1)?;

    if due_repetition.len() > 0 {
        debug!("Due Repetition for task {}: {}", task_id, due_repetition);

        let new_due_date = DatabaseManager::parse_date_string(&due_repetition)?;
        debug!("Parsed schedule at {}", new_due_date);

        let mut statement =
            conn.prepare("Update task SET due_date = :due_date, state_id = 1 WHERE id = :id")?;
        statement.execute_named(named_params! {
            ":due_date": new_due_date,
            ":id": task_id
        })?;
    } else {
        info!("No due repetition");
    }

    if scheduled_repeat.len() > 0 {
        debug!(
            "Scheduled Repetition for task {}: {}",
            task_id, scheduled_repeat
        );

        let new_schedule_at = DatabaseManager::parse_date_string(&scheduled_repeat)?;
        debug!("Parsed schedule at {}", new_schedule_at);

        let mut statement = conn
            .prepare("Update task SET scheduled_at = :scheduled_at, state_id = 1 WHERE id = :id")?;
        statement.execute_named(named_params! {
            ":scheduled_at": new_schedule_at,
            ":id": task_id
        })?;
    } else {
        info!("No schedule repetition");
    }
    Ok(())
}

pub fn modify(
    tx: &mut Transaction,
    task_ids: &Vec<i64>,
    body: &Option<&str>,
    priority: &Option<i64>,
    context_id: &Option<i64>,
    tag_ids: Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    repeat: &Option<&str>,
    recurrence: &Option<&str>,
    state_id: &Option<i64>,
    tag_ids_to_remove: Vec<i64>,
) -> Result<Vec<Task>, TaskooError> {
    // Prepare the statement
    let conditions = generate_condition(
        body,
        context_id,
        due_date,
        scheduled_at,
        repeat,
        recurrence,
        state_id,
    );

    // TODO: Return Error here
    if conditions.is_empty() && tag_ids.is_empty() && tag_ids_to_remove.is_empty() && priority.is_none() {
        info!("Conditions and tag_ids are both empty, nothing is going to be modified");
        return Ok(vec![]);
    }

    if !conditions.is_empty() {
        let final_argument = format!(
            "Update task SET {} WHERE id = :task_id",
            conditions.join(",")
        );

        debug!("Running modify with query \n {}", final_argument);
        let mut statement = tx.prepare(&final_argument)?;
        for task_id in task_ids.iter() {
            statement.execute_named(named_params! {
                ":task_id": task_id
            })?;
        }
    }

    add_tag(&tx, &task_ids, tag_ids)?;
    remove_tag(&tx, &task_ids, tag_ids_to_remove)?;
    if let Some(priority_id) = priority {
        insert_or_replace_priority(&tx, &task_ids, &priority_id)?;
    }

    // If the task is marked to completed, update the scheduled_at
    // based on repeat
    if state_id.is_some() && state_id.unwrap() == 2 {
        info!("Task is marked as completed, updating scheduled_at");
        for task_id in task_ids.iter() {
            update_schedule_at_for_repeat(&tx, &task_id)?;
        }
    }
    Ok(vec![])
}
