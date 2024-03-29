use super::query_helper::generate_condition;
use super::get_base::get_base;
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::CoreError;
use log::debug;
use log::info;
use rusqlite::{named_params, Result, Transaction};

fn update_state(
    conn: &Transaction,
    task_ids: &Vec<i64>,
    state_id: &Option<i64>,
) -> Result<(), CoreError> {
    info!("Updating state");
    if let Some(state_id) = state_id {
        let mut statement = conn
            .prepare("Update task_state SET state_id = :state_id WHERE task_id = :task_id")
            .expect("Failed to prepare the Update task_state query");

        for task_id in task_ids.iter() {
            statement.execute(named_params! {
            ":task_id": task_id,
            ":state_id": state_id})?;
        }
    }
    Ok(())
}

fn update_context(
    conn: &Transaction,
    task_ids: &Vec<i64>,
    context_id: &Option<i64>,
) -> Result<(), CoreError> {
    info!("Updating context");
    if let Some(context_id) = context_id {
        let mut statement = conn
            .prepare("Update task_context SET context_id = :context_id WHERE task_id = :task_id")
            .expect("Failed to prepare the Update task_context query");

        for task_id in task_ids.iter() {
            statement.execute(named_params! {
            ":task_id": task_id,
            ":context_id": context_id})?;
        }
    }
    Ok(())
}

// XXX Why task_ids is reference?
fn add_tag(conn: &Transaction, task_ids: &Vec<i64>, tag_ids: Vec<i64>) -> Result<(), CoreError> {
    let mut statement = conn
        .prepare(
            "INSERT OR IGNORE INTO task_tag
        (task_id, tag_id)
        VALUES (:task_id, :tag_id)",
        )
        .expect("Failed to prepare the INSERT INTO task_tag query");
    for task_id in task_ids.iter() {
        for tag_id in tag_ids.iter() {
            statement.execute(named_params! {
            ":task_id": task_id,
            ":tag_id": tag_id})?;
        }
    }
    Ok(())
}

fn remove_tag(conn: &Transaction, task_ids: &Vec<i64>, tag_ids: Vec<i64>) -> Result<(), CoreError> {
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
            statement.execute(named_params! {
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
) -> Result<(), CoreError> {
    info!(
        "InsertOrReplacePriority tag_ids: {:?} and priority_id {:?}",
        task_ids, priority_id
    );
    let mut statement = conn.prepare(
        "INSERT OR REPLACE INTO priority_task (task_id, priority_id) VALUES (:task_id, :priority_id)",
    )?;

    for task_id in task_ids.iter() {
        statement.execute(named_params! {
        ":task_id": task_id,
        ":priority_id": priority_id})?;
    }
    Ok(())
}

// Check all tasks that are depended on this task_id, and update their state to
// completed if all of their depended tasks are completed
fn update_dependency(conn: &Transaction, task_id: &i64) -> Result<(), CoreError> {
    let mut get_child_tasks_statement =
        conn.prepare("SELECT task_id FROM dependency WHERE parent_task_id = :parent_task_id")?;
    let mut child_tasks_rows = get_child_tasks_statement.query(named_params! {
        ":parent_task_id": task_id
    })?;

    while let Some(child_task_row) = child_tasks_rows.next()? {
        let child_id: i64 = child_task_row.get(0)?;
        let mut get_parent_for_this_child =
            conn.prepare("SELECT parent_task_id FROM dependency WHERE task_id = :child_id")?;
        let mut parent_rows =
            get_parent_for_this_child.query(named_params! {":child_id": child_id})?;

        let mut are_all_parents_completed = true;
        while let Some(parent_row) = parent_rows.next()? {
            let parent_id: i64 = parent_row.get(0)?;
            let mut get_parent_state =
                conn.prepare("SELECT state_id FROM task_state where task_id = :parent_id")?;
            let mut parent_state_rows = get_parent_state.query(named_params! {
                ":parent_id": parent_id
            })?;

            let parent_state_row = parent_state_rows.next()?;

            let parent_state_id: i64 = parent_state_row.unwrap().get(0)?;

            let mut convert_state_to_id =
                conn.prepare("SELECT name FROM state where id = :state_id")?;
            let mut state_to_id_rows = convert_state_to_id.query(named_params! {
                ":state_id": parent_state_id
            })?;

            let state_to_id_row = state_to_id_rows.next()?;
            let state: String = state_to_id_row.unwrap().get(0)?;
            if state != "completed" {
                are_all_parents_completed = false;
            }
        }

        // Change the state to ready
        if are_all_parents_completed {
            let mut update_to_ready_statement =
                conn.prepare("Update task_state SET state_id = 1 WHERE task_id = :child_id")?;
            update_to_ready_statement.execute(named_params! {
                ":child_id": child_id
            })?;
        }
    }
    Ok(())
}

fn update_schedule_at_for_repeat(conn: &Transaction, task_id: &i64) -> Result<(), CoreError> {
    let get_task_repetition_query = format!(
        "SELECT due_repeat, scheduled_repeat from task where id = {}",
        task_id
    );
    let mut statement = conn.prepare(&get_task_repetition_query)?;
    let mut rows = statement.query([])?;

    let first_row = rows.next().expect("We should always have a row");

    let data = first_row.expect("We should always have some data");

    let due_repetition: String = data.get(0)?;
    let scheduled_repetition: String = data.get(1)?;

    if due_repetition.len() > 0 {
        debug!("Due Repetition for task {}: {}", task_id, due_repetition);

        let new_due_date = TaskManager::parse_date_string(&due_repetition)?;
        debug!("Parsed schedule at {}", new_due_date);

        let mut update_task_stmt =
            conn.prepare("Update task SET due_date = :due_date WHERE id = :id")?;
        let mut update_state_stmt =
            conn.prepare("Update task_state SET state_id = 1 WHERE task_id = :id")?;
        update_task_stmt.execute(named_params! {
            ":due_date": new_due_date,
            ":id": task_id
        })?;
        update_state_stmt.execute(named_params! {
            ":id": task_id
        })?;
    } else {
        info!("No due repetition");
    }

    if scheduled_repetition.len() > 0 {
        debug!(
            "Scheduled Repetition for task {}: {}",
            task_id, scheduled_repetition
        );

        let new_schedule_at = TaskManager::parse_date_string(&scheduled_repetition)?;
        debug!("Parsed schedule at {}", new_schedule_at);

        let mut update_task_stmt =
            conn.prepare("Update task SET scheduled_at = :scheduled_at WHERE id = :id")?;
        let mut update_state_stmt =
            conn.prepare("Update task_state SET state_id = 1 WHERE task_id = :id")?;
        update_task_stmt.execute(named_params! {
            ":scheduled_at": new_schedule_at,
            ":id": task_id
        })?;
        update_state_stmt.execute(named_params! {
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
    date_due: &Option<&str>,
    date_scheduled: &Option<&str>,
    repeat: &Option<&str>,
    recurrence: &Option<&str>,
    state_id: &Option<i64>,
    tag_ids_to_remove: Vec<i64>,
) -> Result<Vec<Task>, CoreError> {
    // Prepare the statement
    let conditions = generate_condition(
        body,
        context_id,
        date_due,
        date_scheduled,
        repeat,
        recurrence,
        state_id,
    );

    // TODO: Return Error here
    if conditions.is_empty()
        && tag_ids.is_empty()
        && tag_ids_to_remove.is_empty()
        && state_id.is_none()
        && context_id.is_none()
        && priority.is_none()
    {
        info!(
            "
            conditions, tag_ids, tag_ids_to_remove,
            state_id and context_id and priority are all empty, nothing is going to be modified"
        );
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
            statement.execute(named_params! {
                ":task_id": task_id
            })?;
        }
    }

    add_tag(&tx, &task_ids, tag_ids)?;
    remove_tag(&tx, &task_ids, tag_ids_to_remove)?;

    update_context(&tx, &task_ids, context_id);
    update_state(&tx, &task_ids, state_id);

    if let Some(priority_id) = priority {
        insert_or_replace_priority(&tx, &task_ids, &priority_id)?;
    }

    // If the task is marked to completed, update the scheduled_at
    // based on repeat
    if let Some(2) = state_id {
        info!("Task is marked as completed, updating scheduled_at");
        for task_id in task_ids.iter() {
            update_schedule_at_for_repeat(&tx, &task_id)?;
            update_dependency(&tx, &task_id)?;
        }
    }
    let mut tasks = vec![];

    for task_id in task_ids.iter() {
        let mut modified_tasks = get_base(&tx, &format!("task.id = {}", task_id))?;
        assert_eq!(modified_tasks.len(), 1);
        let task = modified_tasks.remove(0);
        tasks.push(task);
    }
    Ok(tasks)
}
