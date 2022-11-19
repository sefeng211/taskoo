use super::query_helper::generate_view_condition;
use log::info;
use crate::db::task_helper::{Task};
use crate::error::CoreError;
use super::get_base::get_base;

use rusqlite::{Transaction, Result};

pub fn view(
    conn: &Transaction,
    context_id: &i64,
    view_range_start: &Option<String>,
    view_range_end: &String,
    view_type: &Option<String>,
) -> Result<Vec<Task>, CoreError> {
    info!("[view] view_range_start={:?}", view_range_start);
    let conditions =
        generate_view_condition(context_id, view_range_start, view_range_end, view_type);

    assert!(!conditions.is_empty());

    let tasks = get_base(&conn, &conditions.join(" and "))?;
    Ok(tasks)
}
