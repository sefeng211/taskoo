use rusqlite::Rows;
use crate::error::ArgumentError;

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
    pub state: String,
    pub annotation: String,
    pub parent_task_ids: Vec<String>,
}

impl Task {
    pub fn get_property_value(&self, attr: &str) -> Result<String, ArgumentError> {
        match attr {
            "priority" => Ok(self.priority.clone()),
            "context" => Ok(self.context.clone()),
            "tags" => Ok(self.tags.join(",")),
            "date_created" => Ok(self.date_created.clone()),
            "date_due" => Ok(self.date_due.clone()),
            "date_scheduled" => Ok(self.date_scheduled.clone()),
            "repetition_due" => Ok(self.repetition_due.clone()),
            "repetition_scheduled" => Ok(self.repetition_scheduled.clone()),
            "state" => Ok(self.state.clone()),
            "annotation" => Ok(self.annotation.clone()),
            "parent_task_ids" => Ok(self.parent_task_ids.join(",")),
            _ => Err(ArgumentError::InvalidOption(format!(
                "{} is not a supported property",
                attr
            ))),
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.state == "completed";
    }

    pub fn is_started(&self) -> bool {
        return self.state == "started";
    }

    pub fn is_ready(&self) -> bool {
        return self.state == "ready";
    }
    pub fn is_blocked(&self) -> bool {
        return self.state == "blocked";
    }
}

pub fn convert_rows_into_task(rows: &mut Rows) -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    while let Some(row) = rows.next().unwrap() {
        let mut tag_ids: Vec<i64> = vec![];
        match row.get::<_, String>("concat_tag_ids") {
            Ok(ids) => {
                for n in ids.split(",") {
                    tag_ids.push(n.parse::<i64>().unwrap());
                }
            }
            Err(_) => {}
        }

        let mut tag_names: Vec<String> = vec![];
        match row.get::<_, String>("concat_tag_names") {
            Ok(names) => {
                for n in names.split(",") {
                    tag_names.push(n.to_string());
                }
            }
            Err(_) => {}
        }

        let mut parent_task_ids: Vec<String> = vec![];
        match row.get::<_, String>("parent_task_ids") {
            Ok(names) => {
                for n in names.split(",") {
                    parent_task_ids.push(n.to_string());
                }
            }
            Err(_) => {}
        }

        tasks.push(Task {
            id: row.get("id").unwrap(),
            body: row.get("body").unwrap(),
            priority: row.get("priority").unwrap_or("".to_string()),
            tags: tag_names,
            tag_ids: tag_ids,
            date_created: row.get("created_at").unwrap(),
            date_due: row.get("due_date").unwrap_or("".to_string()),
            date_scheduled: row.get("scheduled_at").unwrap(),
            repetition_due: row.get("due_repeat").unwrap(),
            repetition_scheduled: row.get("scheduled_repeat").unwrap(),
            context: row.get("context").unwrap(),
            state: row.get("state").unwrap(),
            annotation: row.get("annotation").unwrap_or("".to_string()),
            parent_task_ids: parent_task_ids,
        });
    }

    tasks
}
