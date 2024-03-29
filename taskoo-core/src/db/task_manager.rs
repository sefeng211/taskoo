use crate::db::add::{add, add_annotation};
use crate::db::delete::delete;
use crate::db::get::get;
use crate::db::modify::modify;
use crate::db::agenda::agenda;
use crate::db::query_helper::{
    CREATE_CONTEXT_TABLE_QUERY, CREATE_DEPENDENCY_TABLE_QUERY, CREATE_STATE_TABLE_QUERY,
    CREATE_TAG_TABLE_QUERY, CREATE_TASK_TABLE_QUERY, CREATE_TASK_TAG_TABLE_QUERY,
    CREATE_PRIORITY_TABLE_QUERY, CREATE_PRIORITY_TASK_TABLE_QUERY, CREATE_TASK_CONTEXT_TABLE_QUERY,
    CREATE_TASK_STATE_TABLE_QUERY,
};
use crate::db::task_helper::{Task, DEFAULT_CONTEXT, TASK_STATES, PRIORITIES};
use crate::db::view::view;
use crate::error::{CoreError, ArgumentError};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime};
use log::{info, debug};
use rusqlite::{named_params, Connection, Result, Transaction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TaskManager {
    pub conn: Connection,
    setting: HashMap<String, String>,
}

impl TaskManager {
    // ensure the database is created
    pub fn new(setting: &HashMap<String, String>) -> TaskManager {
        // We don't handle the Result here, because it's okay
        // to ignore errors.
        //
        env_logger::try_init().ok();
        let conn = Connection::open(setting.get("db_path").unwrap()).unwrap();
        let mut manager = TaskManager {
            conn: conn,
            setting: setting.clone(),
        };
        manager
            .create_table_if_needed(DEFAULT_CONTEXT)
            .expect("Failed to create all required tables");
        return manager;
    }

    pub fn get_context_names_from_config(&self) -> Vec<String> {
        let context = self.setting.get("context").unwrap().to_string();
        return context.split(",").map(|s| s.to_string()).collect();
    }

    pub fn add(
        &mut self,
        body: &str,
        priority: &Option<String>,
        context: &Option<String>,
        tags: &Vec<String>,
        date_due: &Option<&str>,
        date_scheduled: &Option<&str>,
        repetition_due: &Option<&str>,
        repetition_scheduled: &Option<&str>,
        annotation: &Option<&str>,
        state_name: &Option<String>,
        parent_task_ids: &Option<Vec<i64>>,
    ) -> Result<Vec<Task>, CoreError> {
        debug!("Add start! self={:p}", self);
        let mut tx = self.conn.transaction()?;

        // Each task must have a context associated with it
        let context_id: i64 = match context {
            Some(context) => TaskManager::convert_context_name_to_id(&tx, &context, true)?,
            None => 1, // default to `Inbox` context
        };

        // Always respect client provided state_name, however if the
        // client doesn't provide one, and the task is blocked by
        // other tasks, let's change the state to 'blocked'
        let mut state_id = None;
        match state_name {
            Some(state) => {
                state_id = Some(TaskManager::convert_state_name_to_id(&tx, &state)?);
            }
            None => {
                if let Some(parent_task_ids) = parent_task_ids {
                    for id in parent_task_ids.iter() {
                        let task =
                            &get(&tx, &None, &None, &vec![], &None, &None, &Some(*id), &None)?;
                        if task.is_empty() {
                            return Err(CoreError::ArgumentError(String::from(
                                "Invalid parent task is provided",
                            )));
                        }
                        if !task[0].is_completed() {
                            {
                                state_id = Some(TaskManager::convert_state_name_to_id(
                                    &tx,
                                    &String::from("blocked"),
                                )?);
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tags.iter() {
            assert!(!tag_name.is_empty());
            tag_ids.push(TaskManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        // Parse the scheduled_at string!
        let parse_scheduled_at = match date_scheduled {
            Some(period) => Some(TaskManager::parse_date_string(period)?),
            None => None,
        };

        let parsed_schedued_repeat = match repetition_scheduled {
            Some(period) => Some(TaskManager::parse_date_string(period)?),
            None => None,
        };

        let parsed_due_date = match date_due {
            Some(period) => Some(TaskManager::parse_date_string(period)?),
            None => None,
        };

        // Verify the repeat string and recurrence string are both valid.
        if let Some(period) = repetition_due {
            TaskManager::parse_date_string(period)?;
        }

        if let Some(period) = repetition_scheduled {
            TaskManager::parse_date_string(period)?;
        }

        let priority_id = match priority {
            Some(priority_type) => Some(TaskManager::convert_priority_type_to_id(
                &tx,
                &priority_type,
            )?),
            None => None,
        };
        let tasks = add(
            &mut tx,
            &body,
            &priority_id,
            &context_id,
            tag_ids,
            &parsed_due_date.as_deref(),
            &parse_scheduled_at.as_deref(),
            &repetition_due.as_deref(),
            &parsed_schedued_repeat.as_deref(),
            &annotation,
            &state_id,
            &parent_task_ids,
        )?;
        tx.commit()?;
        debug!("Add done! self={:p}", self);
        Ok(tasks)
    }

    pub fn add_annotation(
        &mut self,
        task_id: i64,
        annotation: String,
    ) -> Result<Vec<Task>, CoreError> {
        let mut tx = self.conn.transaction()?;
        let tasks = add_annotation(&mut tx, task_id, annotation)?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn get(
        &mut self,
        priority: &Option<u8>,
        context: &Option<String>,
        tags: &Vec<String>,
        date_due: &Option<&str>,
        date_scheduled: &Option<&str>,
        task_id: &Option<i64>,
        not_tags: &Option<Vec<String>>,
    ) -> Result<Vec<Task>, CoreError> {
        info!(
            "Doing Get Operation with context_name {:?}, tag {:?}",
            context, tags
        );
        // Prepare the context_id, default to Inbox context
        let mut context_id: i64 = 1;
        let tx = self.conn.transaction()?;

        match context {
            Some(name) => {
                context_id = TaskManager::convert_context_name_to_id(&tx, &name, false)?;
            }
            None => (),
        };

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tags.iter() {
            tag_ids.push(TaskManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        let not_tag_ids: Option<Vec<i64>> = match not_tags {
            None => None,
            Some(tags) => {
                let mut tag_ids: Vec<i64> = vec![];
                for tag in tags.iter() {
                    tag_ids.push(TaskManager::convert_tag_name_to_id(&tx, &tag)?);
                }
                Some(tag_ids)
            }
        };

        let tasks = get(
            &tx,
            &priority,
            &Some(context_id),
            &tag_ids,
            &date_due,
            &date_scheduled,
            &task_id,
            &not_tag_ids,
        )?;
        tx.commit()?;
        info!("Got {} of tasks", tasks.len());
        Ok(tasks)
    }

    pub fn delete(&mut self, task_ids: &Vec<i64>) -> Result<Vec<Task>, CoreError> {
        info!("deleting tasks {:?}", task_ids);
        let tx = self.conn.transaction()?;
        let tasks = delete(&tx, &task_ids)?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn modify(
        &mut self,
        task_ids: &Vec<i64>,
        body: &Option<&str>,
        priority: &Option<String>,
        context: &Option<String>,
        tags: &Vec<String>,
        date_due: &Option<&str>,
        date_scheduled: &Option<&str>,
        repetition_due: &Option<&str>,
        repetition_scheduled: &Option<&str>,
        state: &Option<&str>,
        tags_to_remove: &Vec<String>,
    ) -> Result<Vec<Task>, CoreError> {
        let mut tx = self.conn.transaction()?;
        if task_ids.is_empty() {
            Err(ArgumentError::InvalidOption(
                "Task Ids can't be empty".to_string(),
            ))?
        }
        let context_id = match context {
            Some(name) => Some(TaskManager::convert_context_name_to_id(&tx, &name, true)?),
            None => None,
        };

        let state_id = match state {
            Some(name) => Some(TaskManager::convert_state_name_to_id(
                &tx,
                &name.to_string(),
            )?),
            None => None,
        };
        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        let mut tag_ids_to_remove: Vec<i64> = vec![];

        let priority_id = match priority {
            Some(priority_type) => Some(TaskManager::convert_priority_type_to_id(
                &tx,
                &priority_type,
            )?),
            None => None,
        };
        for tag_name in tags.iter() {
            tag_ids.push(TaskManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        for tag_name in tags_to_remove.iter() {
            tag_ids_to_remove.push(TaskManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        let parse_scheduled_at = match date_scheduled {
            Some(period) => {
                if !period.is_empty() {
                    Some(TaskManager::parse_date_string(period)?)
                } else {
                    // Empty string will clears the date string
                    Some(String::new())
                }
            }
            None => None,
        };

        let parsed_due_date = match date_due {
            Some(period) => {
                if !period.is_empty() {
                    Some(TaskManager::parse_date_string(period)?)
                } else {
                    // Empty string will clears the date string
                    Some(String::new())
                }
            }
            None => None,
        };

        let tasks = modify(
            &mut tx,
            &task_ids,
            &body,
            &priority_id,
            &context_id,
            tag_ids,
            &parsed_due_date.as_deref(),
            &parse_scheduled_at.as_deref(),
            &repetition_due,
            &repetition_scheduled,
            &state_id,
            tag_ids_to_remove,
        )?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn view_agenda(
        &mut self,
        start_day: String,
        end_day: Option<String>,
    ) -> Result<Vec<(NaiveDate, Vec<Task>)>, CoreError> {
        let tx = self.conn.transaction()?;
        let start_day_in_date = NaiveDateTime::parse_from_str(
            &TaskManager::parse_date_string(&start_day)?,
            "%Y-%m-%d %H:%M:%S",
        )
        .expect("")
        .date();
        let end_day_in_date = match end_day {
            None => None,
            Some(day) => Some(
                NaiveDateTime::parse_from_str(
                    &TaskManager::parse_date_string(&day)?,
                    "%Y-%m-%d %H:%M:%S",
                )
                .expect("")
                .date(),
            ),
        };
        return agenda(&tx, &start_day_in_date, &end_day_in_date);
    }

    pub fn agenda(
        &mut self,
        context_name: &String,
        view_type: &Option<String>,
        view_range_start: &Option<String>,
        view_range_end: &String,
    ) -> Result<Vec<Task>, CoreError> {
        let mut tx = self.conn.transaction()?;
        let parsed_view_range_end = TaskManager::parse_date_string(&view_range_end)?;

        let tasks;
        if view_type == &Some("due".to_string()) {
            let context_id = TaskManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some("overdue".to_string()) {
            let context_id = TaskManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some(String::from("schedule")) {
            info!("view_type = schedule");
            let context_id = TaskManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some(String::from("all")) {
            let context_id = TaskManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else {
            tx.commit()?;
            return Err(ArgumentError::InvalidViewType(
                view_type.as_ref().unwrap().to_string(),
            ))?;
        }

        tx.commit()?;
        Ok(tasks)
    }

    fn convert_context_name_to_id(
        tx: &Transaction,
        context_name: &String,
        create_if_not_exists: bool,
    ) -> Result<i64, CoreError> {
        let mut statement = tx
            .prepare("SELECT id FROM context WHERE name=(:context_name)")
            .expect("");

        let mut result = statement
            .query(named_params! {":context_name": context_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0).unwrap());
        }

        if create_if_not_exists {
            return TaskManager::create_context(tx, &context_name);
        }

        return Err(ArgumentError::InvalidContext(context_name.clone()))?;
    }

    fn convert_state_name_to_id(tx: &Transaction, state_name: &String) -> Result<i64, CoreError> {
        let mut statement = tx.prepare("SELECT id FROM state WHERE name=(:state_name)")?;
        let mut result = statement.query(named_params! {":state_name": state_name})?;

        while let Some(row) = result.next()? {
            return Ok(row.get(0)?);
        }
        return TaskManager::create_state(tx, state_name);
    }

    fn convert_priority_type_to_id(
        tx: &Transaction,
        priority_type: &String,
    ) -> Result<i64, CoreError> {
        let lower_priority_type = priority_type.clone().to_lowercase();
        let mut statement = tx.prepare("SELECT id FROM priority WHERE name=(:priority_type)")?;
        let mut result = statement.query(named_params! {":priority_type": lower_priority_type})?;
        while let Some(row) = result.next()? {
            return Ok(row.get(0)?);
        }

        Err(ArgumentError::InvalidOption(String::from(format!(
            "Invalid priority {} is provided",
            priority_type
        ))))?
    }
    fn convert_tag_name_to_id(tx: &Transaction, tag_name: &String) -> Result<i64, CoreError> {
        let mut statement = tx
            .prepare("SELECT id FROM tag where name=(:tag_name)")
            .expect("");

        let mut result = statement
            .query(named_params! {":tag_name": tag_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0)?);
        }

        return TaskManager::create_tag(&tx, &tag_name);
    }

    pub fn parse_date_string(date_string: &str) -> Result<String, CoreError> {
        let datetime_now: DateTime<Local> = Local::now();
        if date_string == "tmr" || date_string == "tomorrow" {
            return Ok((datetime_now + Duration::days(1))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string());
        } else if date_string == "today" {
            return Ok(datetime_now.format("%Y-%m-%d %H:%M:%S").to_string());
        } else if date_string.ends_with("hours") {
            let scheduled_at_split: Vec<&str> = date_string.split("hours").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value
                        .parse::<i64>()
                        .map_err(|_error| CoreError::DateParseError(date_string.to_string()))?;
                    key = value_in_int;
                }
                None => {
                    return Err(CoreError::DateParseError(date_string.to_string()));
                }
            }
            // return now + x hours
            return Ok((datetime_now + Duration::hours(key))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string());
        } else if date_string.ends_with("days") {
            let scheduled_at_split: Vec<&str> = date_string.split("days").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value
                        .parse::<i64>()
                        .map_err(|_error| CoreError::DateParseError(date_string.to_string()))?;
                    key = value_in_int;
                }
                None => {
                    return Err(CoreError::DateParseError(date_string.to_string()));
                }
            }
            // return now + x days
            return Ok((datetime_now + Duration::days(key))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string());
        } else if date_string.ends_with("weeks") {
            let scheduled_at_split: Vec<&str> = date_string.split("weeks").collect();
            let key = match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value
                        .parse::<i64>()
                        .map_err(|_error| CoreError::DateParseError(date_string.to_string()))?;
                    value_in_int
                }
                None => {
                    return Err(CoreError::DateParseError(date_string.to_string()));
                }
            };
            // return now + x days
            return Ok((datetime_now + Duration::weeks(key))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string());
        }

        // Ensure the client passed date string is valid
        let parsed_date_string =
            match NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S") {
                Ok(parsed_datetime) => parsed_datetime,
                Err(_) => {
                    let parsed_date =
                        NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").map_err(|source| {
                            CoreError::ChronoParseError {
                                period: date_string.to_string(),
                                source: source,
                            }
                        })?;
                    parsed_date
                        .and_hms_opt(0, 0, 0)
                        .expect("Getting the current datetime should work") // Fulfill the missing hms
                }
            };
        Ok(parsed_date_string.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn create_table_if_needed(&mut self, context: [&'static str; 1]) -> Result<(), CoreError> {
        self.conn.execute(CREATE_TASK_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_TAG_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_TASK_TAG_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_DEPENDENCY_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_CONTEXT_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_TASK_CONTEXT_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_STATE_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_TASK_STATE_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_PRIORITY_TABLE_QUERY, [])?;
        self.conn.execute(CREATE_PRIORITY_TASK_TABLE_QUERY, [])?;

        let tx = self.conn.transaction()?;
        {
            for state in TASK_STATES.iter() {
                TaskManager::create_state(&tx, &state.to_string())?;
            }
        }

        {
            for c in context.iter() {
                TaskManager::create_context(&tx, &c.to_string())?;
            }
        }

        {
            for c in PRIORITIES.iter() {
                TaskManager::create_priority(&tx, &c.to_string())?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    // Create a new context and return the id
    fn create_context(tx: &Transaction, context_name: &String) -> Result<i64, CoreError> {
        let mut insert_into_context =
            tx.prepare("INSERT OR IGNORE INTO context (name) VALUES (:name)")?;
        insert_into_context.execute(named_params! {":name": context_name.trim()})?;
        info!("Created context {}", context_name);
        Ok(tx.last_insert_rowid())
    }

    fn create_tag(tx: &Transaction, tag_name: &String) -> Result<i64, CoreError> {
        let mut insert_into_tag = tx.prepare("INSERT OR IGNORE INTO tag (name) VALUES (:name)")?;
        insert_into_tag.execute(named_params! {":name": tag_name.trim()})?;
        info!("Added a new tag: {}", tag_name);
        Ok(tx.last_insert_rowid())
    }

    fn create_state(tx: &Transaction, state_name: &String) -> Result<i64, CoreError> {
        let mut insert_into_state =
            tx.prepare("INSERT OR IGNORE INTO state (name) VALUES (:name)")?;
        insert_into_state.execute(named_params! {":name": state_name.trim().to_lowercase()})?;
        Ok(tx.last_insert_rowid())
    }

    fn create_priority(tx: &Transaction, priority_name: &String) -> Result<i64, CoreError> {
        let mut insert_into_state =
            tx.prepare("INSERT OR IGNORE INTO priority (name) VALUES (:name)")?;
        insert_into_state.execute(named_params! {":name": priority_name.trim().to_lowercase()})?;
        Ok(tx.last_insert_rowid())
    }
}
