use chrono::{NaiveDate};
use crate::core::{ConfigManager};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;

/* Some of the Agenda functionalities are overlap with list, however,
 * Agenda should provide better API for clients */
pub struct Agenda {
    pub start_day: String,
    pub end_day: Option<String>,
    pub context_name: Option<String>,
    database_manager: Option<TaskManager>,
    result: Vec<(NaiveDate, Vec<Task>)>,
}

impl Agenda {
    pub fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }

    pub fn new2(data: &Vec<String>) -> Result<Agenda, CoreError> {
        if data.is_empty() {
            // TODO return
            return Err(CoreError::ArgumentError(
                "Empty argument is not allowed for agenda".to_string(),
            ));
        }

        let start_day = data[0].clone();
        let mut end_day = None;
        let mut context_name = None;
        if data.len() > 1 {
            if data[1].starts_with("c:") {
                context_name = Some(data[1][2..].to_string());
            } else {
                end_day = Some(data[1].clone());
            }
        }
        if data.len() > 2 && data[2].starts_with("c:") {
            context_name = Some(data[2][2..].to_string());
        }

        Ok(Agenda {
            start_day,
            end_day,
            context_name,
            database_manager: None,
            result: vec![],
        })
    }

    pub fn new(start_day: String, end_day: Option<String>, context_name: Option<String>) -> Agenda {
        Agenda {
            start_day: start_day,
            end_day: end_day,
            context_name,
            database_manager: None,
            result: vec![],
        }
    }

    pub fn do_work_for_agenda(&mut self) -> Result<Vec<(NaiveDate, Vec<Task>)>, CoreError> {
        return TaskManager::view_agenda(
            self.database_manager.as_mut().unwrap(),
            self.start_day.clone(),
            self.end_day.clone(),
            self.context_name.clone(),
        );
    }

    pub fn set_result(&mut self, result: Vec<(NaiveDate, Vec<Task>)>) {
        self.result = result;
    }

    pub fn get_result(&mut self) -> &Vec<(NaiveDate, Vec<Task>)> {
        return &self.result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new2_parses_context_filter_after_range() {
        let data = vec![
            "2026-07-01".to_string(),
            "2026-07-31".to_string(),
            "c:work".to_string(),
        ];

        let agenda = Agenda::new2(&data).unwrap();
        assert_eq!(agenda.start_day, "2026-07-01");
        assert_eq!(agenda.end_day, Some("2026-07-31".to_string()));
        assert_eq!(agenda.context_name, Some("work".to_string()));
    }

    #[test]
    fn test_new2_parses_context_filter_without_range() {
        let data = vec!["2026-07-01".to_string(), "c:work".to_string()];

        let agenda = Agenda::new2(&data).unwrap();
        assert_eq!(agenda.start_day, "2026-07-01");
        assert_eq!(agenda.end_day, None);
        assert_eq!(agenda.context_name, Some("work".to_string()));
    }
}
