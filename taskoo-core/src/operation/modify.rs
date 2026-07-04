use crate::core::{ConfigManager, Operation};
use crate::db::task_helper::Task;
use crate::db::task_manager::TaskManager;
use crate::error::*;
use crate::option_parser::{parse_command_option, CommandOption};

#[derive(Debug)]
pub struct ModifyOperation<'a> {
    pub task_ids: Vec<i64>,
    pub body: Option<&'a str>,
    pub priority: Option<String>,
    pub context_name: Option<String>,
    pub tag_names: Vec<String>,
    pub due_date: Option<&'a str>,
    pub scheduled_at: Option<&'a str>,
    pub due_repeat: Option<&'a str>,
    pub scheduled_repeat: Option<&'a str>,
    state: Option<String>,
    pub tags_to_remove: Vec<String>,
    database_manager: Option<TaskManager>,
    result: Vec<Task>,
}

impl<'a> ModifyOperation<'a> {
    pub fn new(data: &'a Vec<String>) -> Result<ModifyOperation<'a>, CoreError> {
        if data.is_empty() {
            return Err(CoreError::DateParseError(String::from(
                "Empty data provided for modify",
            )));
        }

        let option = parse_command_option(
            &data.iter().map(|s| &**s).collect(),
            false,
            true,
            true,
        )?;
        Ok(Self::from_command_option(option))
    }

    fn from_command_option(option: CommandOption<'a>) -> ModifyOperation<'a> {
        ModifyOperation {
            database_manager: None,
            result: vec![],
            task_ids: option.task_ids,
            body: None,
            priority: option.priority,
            context_name: option.context,
            tag_names: option.tags,
            due_date: option.date_due,
            scheduled_at: option.date_scheduled,
            due_repeat: option.repetition_due,
            scheduled_repeat: option.repetition_scheduled,
            state: option.state,
            tags_to_remove: option.tags_to_remove,
        }
    }

    pub fn set_state_to_started(&mut self) {
        self.state = Some(String::from("started"));
    }

    pub fn set_state_to_completed(&mut self) {
        self.state = Some(String::from("completed"));
    }
    pub fn set_state_to_ready(&mut self) {
        self.state = Some(String::from("ready"));
    }
    pub fn set_state_to_blocked(&mut self) {
        self.state = Some(String::from("blocked"));
    }
    pub fn set_custom_state(&mut self, state: String) {
        self.state = Some(state);
    }
}

impl<'a> Operation for ModifyOperation<'a> {
    fn init(&mut self) -> Result<(), InitialError> {
        if self.database_manager.is_none() {
            self.database_manager = Some(TaskManager::new(
                &ConfigManager::init_and_get_database_path()?,
            ));
        }
        Ok(())
    }
    fn do_work(&mut self) -> Result<Vec<Task>, CoreError> {
        for tag in self.tag_names.iter_mut() {
            *tag = tag.to_lowercase();
        }

        for tag in self.tags_to_remove.iter_mut() {
            *tag = tag.to_lowercase();
        }

        self.context_name = match &self.context_name {
            Some(name) => Some(name.to_lowercase()),
            None => None,
        };

        let tasks = TaskManager::modify(
            self.database_manager.as_mut().unwrap(),
            &self.task_ids,
            &self.body,
            &self.priority,
            &self.context_name,
            &self.tag_names,
            &self.due_date,
            &self.scheduled_at,
            &self.due_repeat,
            &self.scheduled_repeat,
            &self.state.as_deref(),
            &self.tags_to_remove,
        )?;

        Ok(tasks)
    }
    fn set_result(&mut self, result: Vec<Task>) {
        self.result = result;
    }

    fn get_result(&mut self) -> &Vec<Task> {
        return &self.result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_parses_full_modify_command() {
        let data = vec![
            "1".to_string(),
            "2".to_string(),
            "+next".to_string(),
            "~old".to_string(),
            "c:work".to_string(),
            "@started".to_string(),
            "pri:H".to_string(),
            "d:2026-07-10+weekly".to_string(),
            "s:2026-07-08+daily".to_string(),
        ];

        let op = ModifyOperation::new(&data).unwrap();

        assert_eq!(op.task_ids, vec![1, 2]);
        assert_eq!(op.tag_names, vec!["next"]);
        assert_eq!(op.tags_to_remove, vec!["old"]);
        assert_eq!(op.context_name, Some("work".to_string()));
        assert_eq!(op.state, Some("started".to_string()));
        assert_eq!(op.priority, Some("H".to_string()));
        assert_eq!(op.due_date, Some("2026-07-10"));
        assert_eq!(op.due_repeat, Some("weekly"));
        assert_eq!(op.scheduled_at, Some("2026-07-08"));
        assert_eq!(op.scheduled_repeat, Some("daily"));
    }

    #[test]
    fn test_new_parses_state_only_modify_command() {
        let data = vec!["3".to_string(), "@completed".to_string()];
        let op = ModifyOperation::new(&data).unwrap();

        assert_eq!(op.task_ids, vec![3]);
        assert_eq!(op.state, Some("completed".to_string()));
        assert!(op.tag_names.is_empty());
        assert!(op.tags_to_remove.is_empty());
    }
}
