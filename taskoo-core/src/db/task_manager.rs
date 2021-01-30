use crate::db::add::add;
use crate::db::delete::delete;
use crate::db::get::get;
use crate::db::modify::modify;
use crate::db::query_helper::{
    CREATE_CONTEXT_TABLE_QUERY, CREATE_DEPENDENCY_TABLE_QUERY, CREATE_STATE_TABLE_QUERY,
    CREATE_TAG_TABLE_QUERY, CREATE_TASK_TABLE_QUERY, CREATE_TASK_TAG_TABLE_QUERY,
};
use crate::db::task_helper::{Task, DEFAULT_CONTEXT, TASK_STATES};
use crate::db::view::view;
use crate::error::TaskooError;
use chrono::{Date, DateTime, Duration, Local, NaiveDate, Utc};
use log::info;
use rusqlite::{named_params, Connection, Result, Transaction, NO_PARAMS};
use std::collections::HashMap;

// TODO: Rename it to TaskManager or create another struct to contain it
pub struct DatabaseManager {
    pub conn: Connection,
    setting: HashMap<String, String>,
}

impl DatabaseManager {
    // ensure the database is created
    pub fn new(setting: &HashMap<String, String>) -> DatabaseManager {
        if env_logger::try_init().is_err() {
            info!("Unable to init the logger, it's okay");
        }
        let conn = Connection::open(setting.get("db_path").unwrap()).unwrap();
        let mut manager = DatabaseManager {
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
        priority: &Option<u8>,
        context_name: &Option<String>,
        tag_names: &Vec<String>,
        due_date: &Option<&str>,
        scheduled_at: &Option<&str>,
        due_repeat: &Option<&str>,
        scheduled_repeat: &Option<&str>,
        state_name: &Option<String>,
    ) -> Result<Vec<Task>, TaskooError> {
        // Prepare the context_id
        let mut tx = self.conn.transaction()?;

        let mut context_id: i64 = 1;
        if let Some(context) = context_name {
            context_id = DatabaseManager::convert_context_name_to_id(&tx, &context, true)?;
        };

        let mut state_id = None;
        if let Some(state) = state_name {
            state_id = Some(DatabaseManager::convert_state_name_to_id(&tx, &state)?);
        }

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tag_names.iter() {
            tag_ids.push(DatabaseManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        // Parse the scheduled_at string!
        let parse_scheduled_at = match scheduled_at {
            Some(period) => {
                let parsed = DatabaseManager::parse_date_string(period)?;
                Some(parsed)
            }
            None => None,
        };

        let parsed_schedued_repeat = match scheduled_repeat {
            Some(period) => Some(DatabaseManager::parse_date_string(period)?),
            None => None,
        };

        let parsed_due_date = match due_date {
            Some(period) => Some(DatabaseManager::parse_date_string(period)?),
            None => None,
        };

        // Verify the repeat string and recurrence string are both
        // valid.
        let parsed_due_repeat = match due_repeat {
            Some(period) => Some(DatabaseManager::parse_date_string(period)?),
            None => None,
        };

        let tasks = add(
            &mut tx,
            &body,
            &priority,
            &context_id,
            tag_ids,
            &parsed_due_date.as_deref(),
            &parse_scheduled_at.as_deref(),
            &parsed_due_repeat.as_deref(),
            &parsed_schedued_repeat.as_deref(),
            &state_id,
        )?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn get(
        &mut self,
        priority: &Option<u8>,
        context_name: &Option<String>,
        tag_names: &Vec<String>,
        due_date: &Option<&str>,
        scheduled_at: &Option<&str>,
        is_repeat: &Option<u8>,
        is_recurrence: &Option<u8>,
    ) -> Result<Vec<Task>, TaskooError> {
        info!(
            "Doing Get Operation with context_name {:?}, tag {:?}",
            context_name, tag_names
        );
        // Prepare the context_id, default to Inbox context
        let mut context_id: i64 = 1;
        let tx = self.conn.transaction()?;
        match context_name {
            Some(name) => {
                context_id = DatabaseManager::convert_context_name_to_id(&tx, &name, false)?;
            }
            None => (),
        };

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tag_names.iter() {
            tag_ids.push(DatabaseManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        let tasks = get(
            &tx,
            &priority,
            &Some(context_id),
            &tag_ids,
            &due_date,
            &scheduled_at,
            &is_repeat,
            &is_recurrence,
        )?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn delete(&mut self, task_ids: &Vec<i64>) -> Result<Vec<Task>, TaskooError> {
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
        priority: &Option<u8>,
        context_name: &Option<String>,
        tag_names: &Vec<String>,
        due_date: &Option<&str>,
        scheduled_at: &Option<&str>,
        due_repeat: &Option<&str>,
        scheduled_repeat: &Option<&str>,
        state_name: &Option<&str>,
    ) -> Result<Vec<Task>, TaskooError> {
        let mut tx = self.conn.transaction()?;
        if task_ids.is_empty() {
            return Err(TaskooError::InvalidOption(
                "Task Ids can't be empty".to_string(),
            ));
        }
        let context_id = match context_name {
            Some(name) => Some(DatabaseManager::convert_context_name_to_id(
                &tx, &name, true,
            )?),
            None => None,
        };

        let state_id = match state_name {
            Some(name) => Some(DatabaseManager::convert_state_name_to_id(
                &tx,
                &name.to_string(),
            )?),
            None => None,
        };
        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];

        for tag_name in tag_names.iter() {
            tag_ids.push(DatabaseManager::convert_tag_name_to_id(&tx, &tag_name)?);
        }

        let parse_scheduled_at = match scheduled_at {
            Some(period) => Some(DatabaseManager::parse_date_string(period)?),
            None => None,
        };

        let parsed_due_date = match due_date {
            Some(period) => Some(DatabaseManager::parse_date_string(period)?),
            None => None,
        };

        let tasks = modify(
            &mut tx,
            &task_ids,
            &body,
            &priority,
            &context_id,
            tag_ids,
            &parsed_due_date.as_deref(),
            &parse_scheduled_at.as_deref(),
            &due_repeat,
            &scheduled_repeat,
            &state_id,
        )?;
        tx.commit()?;
        Ok(tasks)
    }

    pub fn view(
        &mut self,
        context_name: &String,
        view_type: &Option<String>,
        view_range_start: &Option<String>,
        view_range_end: &String,
    ) -> Result<Vec<Task>, TaskooError> {
        let mut tx = self.conn.transaction()?;
        let parsed_view_range_end = DatabaseManager::parse_date_string(&view_range_end)?;

        let tasks;
        if view_type == &Some("due".to_string()) {
            let context_id =
                DatabaseManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some("overdue".to_string()) {
            let context_id =
                DatabaseManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some(String::from("schedule")) {
            info!("view_type = schedule");
            let context_id =
                DatabaseManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else if view_type == &Some(String::from("all")) {
            let context_id =
                DatabaseManager::convert_context_name_to_id(&tx, &context_name, false)?;
            tasks = view(
                &mut tx,
                &context_id,
                &view_range_start,
                &parsed_view_range_end,
                &view_type,
            )?;
        } else {
            tx.commit()?;
            return Err(TaskooError::InvalidViewType(
                view_type.as_ref().unwrap().to_string(),
            ));
        }

        tx.commit()?;
        Ok(tasks)
    }

    fn convert_context_name_to_id(
        tx: &Transaction,
        context_name: &String,
        create_if_not_exists: bool,
    ) -> Result<i64, TaskooError> {
        let mut statement = tx
            .prepare("SELECT id FROM context WHERE name=(:context_name)")
            .expect("");

        let mut result = statement
            .query_named(named_params! {":context_name": context_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0).unwrap());
        }

        if create_if_not_exists {
            return DatabaseManager::create_context(tx, &context_name);
        }
        Err(TaskooError::InvalidContext(context_name.clone()))
    }

    fn convert_state_name_to_id(tx: &Transaction, state_name: &String) -> Result<i64, TaskooError> {
        let mut statement = tx
            .prepare("SELECT id FROM state WHERE name=(:state_name)")
            .unwrap();
        let mut result = statement
            .query_named(named_params! {":state_name": state_name})
            .unwrap();

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0).unwrap());
        }
        return DatabaseManager::create_state(tx, state_name);
    }

    fn convert_tag_name_to_id(tx: &Transaction, tag_name: &String) -> Result<i64, TaskooError> {
        let mut statement = tx
            .prepare("SELECT id FROM tag where name=(:tag_name)")
            .expect("");

        let mut result = statement
            .query_named(named_params! {":tag_name": tag_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0)?);
        }

        return DatabaseManager::create_tag(&tx, &tag_name);
    }

    pub fn parse_date_string(scheduled_at: &str) -> Result<String, TaskooError> {
        let current_date: Date<Local> = Local::today();
        if scheduled_at == "tmr" || scheduled_at == "tomorrow" {
            return Ok((current_date + Duration::days(1))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at == "today" {
            let current_datetime: Date<Local> = Local::today();
            return Ok(current_date.format("%Y-%m-%d").to_string());
        } else if scheduled_at.ends_with("hours") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("hours").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        TaskooError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    key = value_in_int;
                }
                None => {
                    return Err(TaskooError::PeriodParsingError(scheduled_at.to_string()));
                }
            }
            // return now + x hours
            return Ok((current_date + Duration::hours(key))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at.ends_with("days") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("days").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        TaskooError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    key = value_in_int;
                }
                None => {
                    return Err(TaskooError::PeriodParsingError(scheduled_at.to_string()));
                }
            }
            // return now + x days
            return Ok((current_date + Duration::days(key))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at.ends_with("weeks") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("weeks").collect();
            let key = match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        TaskooError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    value_in_int
                }
                None => {
                    return Err(TaskooError::PeriodParsingError(scheduled_at.to_string()));
                }
            };
            // return now + x days
            return Ok((current_date + Duration::weeks(key))
                .format("%Y-%m-%d")
                .to_string());
        }

        // Try to parse it from a raw string, note that it's client's job to convert the timestamp
        // to utc
        let parsed_timestamp = Date::<Utc>::from_utc(
            NaiveDate::parse_from_str(&scheduled_at, "%Y-%m-%d").map_err(|source| {
                TaskooError::PeriodChronoParseError {
                    period: scheduled_at.to_string(),
                    source: source,
                }
            })?,
            Utc,
        );
        Ok(parsed_timestamp.format("%Y-%m-%d").to_string())
    }

    fn create_table_if_needed(&mut self, context: [&'static str; 1]) -> Result<(), TaskooError> {
        self.conn.execute(CREATE_TASK_TABLE_QUERY, NO_PARAMS)?;
        self.conn.execute(CREATE_TAG_TABLE_QUERY, NO_PARAMS)?;
        self.conn.execute(CREATE_TASK_TAG_TABLE_QUERY, NO_PARAMS)?;
        self.conn
            .execute(CREATE_DEPENDENCY_TABLE_QUERY, NO_PARAMS)?;
        self.conn.execute(CREATE_CONTEXT_TABLE_QUERY, NO_PARAMS)?;
        self.conn
            .execute(CREATE_STATE_TABLE_QUERY, NO_PARAMS)
            .unwrap();

        let tx = self.conn.transaction()?;
        {
            for state in TASK_STATES.iter() {
                DatabaseManager::create_state(&tx, &state.to_string())?;
            }
        }

        {
            for c in context.iter() {
                DatabaseManager::create_context(&tx, &c.to_string())?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    // Create a new context and return the id
    fn create_context(tx: &Transaction, context_name: &String) -> Result<i64, TaskooError> {
        let mut insert_into_context =
            tx.prepare("INSERT OR IGNORE INTO context (name) VALUES (:name)")?;
        insert_into_context.execute_named(named_params! {":name": context_name.trim()})?;
        info!("Created context {}", context_name);
        Ok(tx.last_insert_rowid())
    }

    fn create_tag(tx: &Transaction, tag_name: &String) -> Result<i64, TaskooError> {
        let mut insert_into_tag = tx.prepare("INSERT OR IGNORE INTO tag (name) VALUES (:name)")?;
        insert_into_tag.execute_named(named_params! {":name": tag_name.trim()})?;
        info!("Added a new tag: {}", tag_name);
        Ok(tx.last_insert_rowid())
    }

    fn create_state(tx: &Transaction, state_name: &String) -> Result<i64, TaskooError> {
        let mut insert_into_state =
            tx.prepare("INSERT OR IGNORE INTO state (name) VALUES (:name)")?;
        insert_into_state
            .execute_named(named_params! {":name": state_name.trim().to_lowercase()})?;
        Ok(tx.last_insert_rowid())
    }
}
