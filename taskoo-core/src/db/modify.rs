use crate::db::task_helper::Task;
use rusqlite::{named_params, Connection, Error as DbError, Result, Transaction};

const MODIFY_QUERY: &str = "
    Update task
    SET {}
    WHERE id = :task_id
";

// XXX Why task_ids is reference?
fn add_tag(conn: &Transaction, task_ids: &Vec<i64>, tag_ids: Vec<i64>) {
    let mut statement = conn
        .prepare(
            "INSERT INTO task_tag
        (task_id, tag_id)
        VALUES (:task_id, :tag_id)",
        )
        .expect("Failed to prepare the INSERT INTO task_tag query");
    for task_id in task_ids.iter() {
        for tag_id in tag_ids.iter() {
            statement.execute_named(named_params! {
            ":task_id": task_id,
            ":tag_id": tag_id});
        }
    }
}

type TaskValue<'a> = (&'a str, &'a str); // (x,y)
pub fn modify(
    conn: &mut Connection,
    task_ids: &Vec<i64>,
    body: &Option<&str>,
    priority: &Option<u8>,
    context_id: &Option<i64>,
    tag_ids: Vec<i64>,
    due_date: &Option<&str>,
    scheduled_at: &Option<&str>,
    is_repeat: &Option<u8>,
    is_recurrence: &Option<u8>,
) -> Result<Vec<Task>, DbError> {
    // Prepare the statement
    let mut arguments: String = String::from("");
    let mut args: Vec<TaskValue> = vec![];
    if body.is_some() {
        arguments.push_str(format!("body = '{}',", body.unwrap()).as_str());
    }

    if priority.is_some() {
        arguments.push_str(format!("priority = {},", priority.unwrap()).as_str());
    }

    if context_id.is_some() {
        arguments.push_str(format!("context_id = {},", context_id.unwrap()).as_str());
    }

    if due_date.is_some() {
        arguments.push_str(format!("due_date = '{}',", due_date.unwrap()).as_str());
    }

    if scheduled_at.is_some() {
        arguments.push_str(format!("scheduled_at = '{}',", scheduled_at.unwrap()).as_str());
    }

    if is_repeat.is_some() {
        arguments.push_str(format!("is_repeat = {},", is_repeat.unwrap()).as_str());
    }

    if is_recurrence.is_some() {
        arguments.push_str(format!("is_recurrence = {},", is_recurrence.unwrap()).as_str());
    }

    let tx = conn.transaction()?;
    // TODO: Return Error here
    if !arguments.is_empty() {
        // Pop the last comma character due to sql syntax error
        arguments.pop();

        let final_argument = format!(
            "
    Update task
    SET {}
    WHERE id = :task_id
    ",
            arguments
        );

        println!("{}", final_argument);
        let mut statement = tx.prepare(&final_argument)?;
        for task_id in task_ids.iter() {
            statement.execute_named(named_params! {
                ":task_id": task_id
            });
        }
    }

    add_tag(&tx, &task_ids, tag_ids);
    tx.commit()?;
    Ok(vec![])
}
