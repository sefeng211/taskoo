use rusqlite::Rows;
use crate::error::TaskooError;

pub const TASK_STATES: [&'static str; 4] = ["ready", "completed", "blocked", "started"];

pub const DEFAULT_CONTEXT: [&'static str; 1] = ["inbox"];

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub body: String,
    pub priority: i64,
    pub context_name: String,
    pub tag_names: Vec<String>,
    pub tag_ids: Vec<i64>,
    pub created_at: String,
    pub due_date: String,
    pub scheduled_at: String,
    pub due_repeat: String,
    pub scheduled_repeat: String,
    pub is_completed: bool,
    pub state_name: String,
    pub annotation: String,
}

impl Task {
    pub fn get_string_value(&self, attr: &str) -> Result<String, TaskooError> {
        match attr {
            "annotation" => Ok(self.annotation.clone()),
            _ => Err(TaskooError::InvalidOption(String::from(attr))),
        }
    }
}
pub fn convert_rows_into_task(rows: &mut Rows) -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    while let Some(row) = rows.next().unwrap() {
        // TODO convert context_id to context_name
        let mut tag_names: Vec<String> = vec![];
        match row.get(12) {
            Ok(names) => {
                let temp: String = names;
                for n in temp.split(",") {
                    tag_names.push(n.to_string());
                }
            }
            Err(_) => {}
        }

        let mut tag_ids: Vec<i64> = vec![];
        match row.get(11) {
            Ok(ids) => {
                let temp: String = ids;
                for n in temp.split(",") {
                    tag_ids.push(n.parse::<i64>().unwrap());
                }
            }
            Err(_) => {}
        }

        // TODO Hard-coded Completed
        let is_completed = tag_names.contains(&"completed".to_string());
        let task = Task {
            id: row.get(0).unwrap(),
            body: row.get(1).unwrap(),
            priority: row.get(2).unwrap(),
            tag_names: tag_names,
            tag_ids: tag_ids,
            created_at: row.get(3).unwrap(),
            due_date: row.get(4).unwrap(),
            scheduled_at: row.get(5).unwrap(),
            due_repeat: row.get(6).unwrap(),
            scheduled_repeat: row.get(7).unwrap(),
            context_name: row.get(8).unwrap(),
            state_name: row.get(9).unwrap(),
            annotation: row.get(10).unwrap_or("".to_string()),
            is_completed: is_completed,
        };

        tasks.push(task);
    }

    tasks
}
