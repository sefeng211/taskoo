use chrono::{NaiveDate, Duration};
use super::query_helper::generate_agenda_condition;
use log::info;
use crate::db::task_helper::{Task};
use crate::error::CoreError;
use super::get_base::get_base;

use rusqlite::{Transaction, Result};

pub fn agenda(
    conn: &Transaction,
    start_day: &NaiveDate,
    end_day: &Option<NaiveDate>,
    context_name: &Option<String>,
) -> Result<Vec<(NaiveDate, Vec<Task>)>, CoreError> {
    info!(
        "[agenda] start_day={:?} end_day={:?} context_name={:?}",
        start_day, end_day, context_name
    );

    let mut days: Vec<NaiveDate> = vec![start_day.clone()];

    if let Some(day) = end_day {
        let mut current_date = start_day.clone();
        while current_date < *day {
            current_date += Duration::days(1);
            days.push(current_date.clone());
        }
    }

    let mut result = vec![];
    for day in days.iter() {
        let day_conditions = generate_agenda_condition(day);
        assert!(!day_conditions.is_empty());
        let mut conditions = vec![format!("({})", day_conditions.join(" or "))];
        if let Some(context_name) = context_name {
            conditions.push(format!("context.name = '{}'", context_name));
        }
        let tasks = get_base(&conn, &conditions.join(" and "))?;
        result.push((day.clone(), tasks));
    }

    Ok(result)
}
