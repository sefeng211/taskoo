use rusqlite::Rows;
use crate::error::TaskooError;

pub const TASK_STATES: [&'static str; 4] = ["ready", "completed", "blocked", "started"];

pub const DEFAULT_CONTEXT: [&'static str; 1] = ["inbox"];

pub const PRIORITIES: [&'static str; 3] = ["H", "M", "L"];

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub body: String,
    pub priority: String,
    pub context: String,
    pub tags: Vec<String>,
    pub tag_ids: Vec<i64>,
    pub date_created: String,
    pub date_due: String,
    pub date_scheduled: String,
    pub repetition_due: String,
    pub repetition_scheduled: String,
    pub state_name: String,
    pub annotation: String,
}

impl Task {
    pub fn get_property_value(&self, attr: &str) -> Result<String, TaskooError> {
        match attr {
            "annotation" => Ok(self.annotation.clone()),
            _ => Err(TaskooError::InvalidOption(String::from(attr))),
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.state_name == "completed";
    }

    pub fn is_started(&self) -> bool {
        return self.state_name == "started";
    }

    pub fn is_ready(&self) -> bool {
        return self.state_name == "ready";
    }
    pub fn is_blocked(&self) -> bool {
        return self.state_name == "blocked";
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

        let task = Task {
            id: row.get(0).unwrap(),
            body: row.get(1).unwrap(),
            priority: row.get(2).unwrap_or("".to_string()),
            tags: tag_names,
            tag_ids: tag_ids,
            date_created: row.get(3).unwrap(),
            date_due: row.get(4).unwrap_or("".to_string()),
            date_scheduled: row.get(5).unwrap(),
            repetition_due: row.get(6).unwrap(),
            repetition_scheduled: row.get(7).unwrap(),
            context: row.get(8).unwrap(),
            state_name: row.get(9).unwrap(),
            annotation: row.get(10).unwrap_or("".to_string()),
        };

        tasks.push(task);
    }

    tasks
}
