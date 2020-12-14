use crate::db::add::add;
use crate::db::delete::delete;
use crate::db::get::get;
use crate::db::modify::modify;
use crate::db::task_helper::Task;
use crate::db::view::view;
use crate::error::OperationError;
use chrono::{Date, DateTime, Duration, NaiveDate, Utc};
use log::info;
use rusqlite::{named_params, Connection, Error as DbError, Result, NO_PARAMS};
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
        let manager = DatabaseManager {
            conn: conn,
            setting: setting.clone(),
        };
        manager
            .create_table_if_needed(setting.get("tag").unwrap(), setting.get("context").unwrap())
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
        is_repeat: &Option<u8>,
        is_recurrence: &Option<u8>,
    ) -> Result<Vec<Task>, OperationError> {
        // Prepare the context_id
        let mut context_id: i64 = 1;
        match context_name {
            Some(name) => {
                context_id = self.convert_context_name_to_id(&name)?;
            }
            None => (),
        };

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tag_names.iter() {
            tag_ids.push(self.convert_tag_name_to_id(&tag_name)?);
        }

        // Parse the scheduled_at string!
        let parse_scheduled_at = match scheduled_at {
            Some(period) => {
                let parsed = self.parse_scheduled_at(period)?;
                Some(parsed)
            }
            None => None,
        };

        //let parse_scheduled_at_i = parse_scheduled_at;
        return add(
            &mut self.conn,
            &body,
            &priority,
            &context_id,
            tag_ids,
            &due_date,
            //&parse_scheduled_at.as_deref(),
            &parse_scheduled_at.as_deref(),
            &is_repeat,
            &is_recurrence,
        )
        .map_err(|error| OperationError::SqliteError { source: error });
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
    ) -> Result<Vec<Task>, OperationError> {
        // Prepare the context_id, default to Inbox context
        let mut context_id: i64 = 1;
        match context_name {
            Some(name) => {
                context_id = self.convert_context_name_to_id(&name)?;
            }
            None => (),
        };

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];
        for tag_name in tag_names.iter() {
            tag_ids.push(self.convert_tag_name_to_id(&tag_name)?);
        }

        return get(
            &self.conn,
            &priority,
            &Some(context_id),
            &tag_ids,
            &due_date,
            &scheduled_at,
            &is_repeat,
            &is_recurrence,
        )
        .map_err(|error| OperationError::SqliteError { source: error });
    }

    pub fn delete(&self, task_ids: &Vec<i64>) -> Result<Vec<Task>, OperationError> {
        return delete(&self.conn, &task_ids)
            .map_err(|error| OperationError::SqliteError { source: error });
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
        is_repeat: &Option<u8>,
        is_recurrence: &Option<u8>,
    ) -> Result<Vec<Task>, OperationError> {
        if task_ids.is_empty() {
            return Err(OperationError::InvalidOption(
                "Task Ids can't be empty".to_string(),
            ));
        }
        let mut context_id = None;
        match context_name {
            Some(name) => {
                context_id = Some(self.convert_context_name_to_id(&name)?);
            }
            None => (),
        };

        // Prepare the tag_ids
        let mut tag_ids: Vec<i64> = vec![];

        for tag_name in tag_names.iter() {
            tag_ids.push(self.convert_tag_name_to_id(&tag_name)?);
        }

        return modify(
            &mut self.conn,
            &task_ids,
            &body,
            &priority,
            &context_id,
            tag_ids,
            &due_date,
            &scheduled_at,
            &is_repeat,
            &is_recurrence,
        )
        .map_err(|error| OperationError::SqliteError { source: error });
    }

    pub fn view(
        &mut self,
        context_name: &String,
        view_type: &Option<String>,
        view_range_start: &Option<String>,
        view_range_end: &String,
    ) -> Result<Vec<Task>, OperationError> {
        let parsed_view_range_start = self.parse_scheduled_at(&view_range_end)?;
        if view_type == &Some("due".to_string()) {
            // Convert to use db::view
            return self.get(
                &None,
                &Some(context_name.clone()),
                &vec![],
                &Some(&parsed_view_range_start),
                &None,
                &None,
                &None,
            );
        } else if view_type == &Some("overdue".to_string()) {
            let context_id = self.convert_context_name_to_id(&context_name)?;
            return view(
                &mut self.conn,
                &context_id,
                &view_range_start,
                &view_range_end,
                &view_type,
            )
            .map_err(|error| OperationError::SqliteError { source: error });
        }

        Ok(vec![])
    }

    fn convert_context_name_to_id(&self, context_name: &String) -> Result<i64, OperationError> {
        let mut statement = self
            .conn
            .prepare("SELECT id FROM context WHERE name=(:context_name)")
            .expect("");

        let mut result = statement
            .query_named(named_params! {":context_name": context_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row.get(0).unwrap());
        }

        Err(OperationError::InvalidContext(context_name.clone()))
    }

    fn convert_tag_name_to_id(&self, tag_name: &String) -> Result<i64, OperationError> {
        let mut statement = self
            .conn
            .prepare("SELECT id FROM tag where name=(:tag_name)")
            .expect("");

        let mut result = statement
            .query_named(named_params! {":tag_name": tag_name})
            .expect("");

        while let Some(row) = result.next().expect("") {
            return Ok(row
                .get(0)
                .map_err(|error| OperationError::SqliteError { source: error })?);
        }

        Err(OperationError::InvalidTag(
            "Invalid tag is provided".to_string(),
        ))
        // return self.create_tag(&tag_name);
    }

    fn parse_scheduled_at(&self, scheduled_at: &str) -> Result<String, OperationError> {
        if scheduled_at == "tmr" || scheduled_at == "tomorrow" {
            let current_datetime: DateTime<Utc> = Utc::now();
            return Ok((current_datetime + Duration::days(1))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at.ends_with("hours") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("hours").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        OperationError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    key = value_in_int;
                }
                None => {
                    return Err(OperationError::PeriodParsingError(scheduled_at.to_string()));
                }
            }
            // return now + x hours
            let current_datetime: DateTime<Utc> = Utc::now();
            return Ok((current_datetime + Duration::hours(key))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at.ends_with("days") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("days").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        OperationError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    key = value_in_int;
                }
                None => {
                    return Err(OperationError::PeriodParsingError(scheduled_at.to_string()));
                }
            }
            // return now + x days
            let current_datetime: DateTime<Utc> = Utc::now();
            return Ok((current_datetime + Duration::days(key))
                .format("%Y-%m-%d")
                .to_string());
        } else if scheduled_at.ends_with("weeks") {
            let scheduled_at_split: Vec<&str> = scheduled_at.split("weeks").collect();
            let key: i64;
            match scheduled_at_split.iter().next() {
                Some(value) => {
                    let value_in_int = value.parse::<i64>().map_err(|_error| {
                        OperationError::PeriodParsingError(scheduled_at.to_string())
                    })?;
                    key = value_in_int;
                }
                None => {
                    return Err(OperationError::PeriodParsingError(scheduled_at.to_string()));
                }
            }
            // return now + x days
            let current_datetime: DateTime<Utc> = Utc::now();
            return Ok((current_datetime + Duration::weeks(key))
                .format("%Y-%m-%d")
                .to_string());
        }

        // Try to parse it from a raw string, note that it's client's job to convert the timestamp
        // to utc
        let parsed_timestamp = Date::<Utc>::from_utc(
            NaiveDate::parse_from_str(&scheduled_at, "%Y-%m-%d")
                .map_err(|error| OperationError::PeriodChronoParseError { source: error })
                .unwrap(),
            Utc,
        );
        Ok(parsed_timestamp.format("%Y-%m-%d").to_string())
    }

    fn create_table_if_needed(&self, tag: &String, context: &String) -> Result<(), DbError> {
        // task table
        // Whenever a is_repeat or is_recurrence task is modified,
        // the scheduled_at date needs to be recalculated
        self.conn.execute(
            "create table if not exists task (
             id integer primary key,
             body text not null,
             priority integer not null,
             context_id INTEGER not null,
             created_at Text DEFAULT CURRENT_DATE,
             due_date TEXT nullable,
             scheduled_at Text nullable,
             is_repeat INTEGER not null,
             is_recurrence INTEGER not null,
             FOREIGN KEY(context_id) REFERENCES context(id)
         )",
            NO_PARAMS,
        )?;

        // tag table
        self.conn.execute(
            "create table if not exists tag (
            id integer primary key,
            name TEXT not null unique
            )",
            NO_PARAMS,
        )?;

        // task_tag table
        self.conn.execute(
            "create table if not exists task_tag (
            task_id integer not null,
            tag_id integer not null,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES task(id),
            FOREIGN KEY (tag_id) REFERENCES tag(id)
            )",
            NO_PARAMS,
        )?;

        // dependency table
        self.conn.execute(
            "create table if not exists dependency (
            task_id integer not null,
            depended_task_id integer not null,
            PRIMARY KEY (task_id, depended_task_id),
            FOREIGN KEY (task_id) REFERENCES task(id),
            FOREIGN KEY (depended_task_id) REFERENCES task(id)
            )",
            NO_PARAMS,
        )?;

        // context table
        self.conn.execute(
            "create table if not exists context (
            id integer primary key,
            name Text not null unique)",
            NO_PARAMS,
        )?;

        {
            let context_vec = context.split(",");
            for c in context_vec.into_iter() {
                self.create_context(&c.to_owned())?;
            }
        }

        {
            let tag_vec = tag.split(",");
            for c in tag_vec.into_iter() {
                self.create_tag(&c.to_owned())?;
            }
        }
        Ok(())
    }

    // Create a new context and return the id
    fn create_context(&self, context_name: &String) -> Result<i64, DbError> {
        let mut insert_into_context = self
            .conn
            .prepare("INSERT OR IGNORE INTO context (name) VALUES (:name)")?;
        insert_into_context.execute_named(named_params! {":name": context_name.trim()})?;
        Ok(self.conn.last_insert_rowid())
    }

    fn create_tag(&self, tag_name: &String) -> Result<i64, DbError> {
        let mut insert_into_tag = self
            .conn
            .prepare("INSERT OR IGNORE INTO tag (name) VALUES (:name)")?;
        insert_into_tag.execute_named(named_params! {":name": tag_name.trim()})?;
        Ok(self.conn.last_insert_rowid())
    }
}
