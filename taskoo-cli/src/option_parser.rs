use log::{debug, error, info, log_enabled, Level};
use thiserror::Error;

pub struct CommandOption<'a> {
    pub scheduled_at: Option<&'a str>,
    pub due_date: Option<&'a str>,
    pub tag_names: Vec<String>,
    pub context_name: Option<String>,
    pub body: Option<String>,
}

#[derive(Error, Debug)]
pub enum CommandOptionError {
    #[error("Invalid scheduled at {0}")]
    InvalidScheduleAt(String),
    #[error("Invalid due date {0}")]
    InvalidDueDate(String),
    #[error("Invalid context name {0}")]
    InvalidContextName(String),
}

pub fn parse_command_option<'a>(
    options: &Vec<&'a str>,
    parse_body: bool,
) -> Result<CommandOption<'a>, CommandOptionError> {
    let mut command_option = CommandOption {
        scheduled_at: None,
        due_date: None,
        tag_names: vec![],
        context_name: None,
        body: None,
    };

    let mut start_parse_options = false;
    let mut body: String = String::from("");

    for option in options.iter() {
        if option.starts_with("s:") {
            start_parse_options = true;
            if command_option.scheduled_at.is_none() {
                command_option.scheduled_at = Some(&option[2..]);
            } else {
                return Err(CommandOptionError::InvalidScheduleAt(option.to_string()));
            };
        } else if option.starts_with("d:") {
            start_parse_options = true;
            if command_option.due_date.is_none() {
                command_option.due_date = Some(&option[2..]);
            } else {
                return Err(CommandOptionError::InvalidDueDate(option.to_string()));
            };
        } else if option.starts_with("c:") {
            start_parse_options = true;
            if command_option.context_name.is_none() {
                command_option.context_name = Some(option[2..].to_string());
            } else {
                return Err(CommandOptionError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("+") {
            start_parse_options = true;
            command_option.tag_names.push(option[1..].to_string());
        } else {
            if parse_body && !start_parse_options {
                if !body.is_empty() {
                    body.push_str(" ");
                }
                body.push_str(option);
            }
        }
    }

    if parse_body {
        command_option.body = Some(body);
    }
    Ok(command_option)
}

pub fn parse_task_ids(option: &String) -> Result<Vec<i64>, CommandOptionError> {
    let mut task_ids: Vec<i64> = vec![];
    if option.contains("..") {
        let ranged_selection = option.split("..").collect::<Vec<&str>>();
        if ranged_selection.len() != 2 {
            eprintln!("Invalid range provided {}", option);
        }
        let start = ranged_selection[0]
            .parse::<i64>()
            .expect("Can't find valid start from provided range");
        let end = ranged_selection[1]
            .parse::<i64>()
            .expect("Can't find valid end from provided range");
        task_ids = (start..=end).collect::<Vec<i64>>();
    } else {
        task_ids.push(option.parse().expect("Invalid task id provided"));
    }
    Ok(task_ids)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_schedule_at_ok() {
        let option = vec!["s:2020-11-11"];
        let parsed_option = parse_command_option(&option, false).unwrap();
        assert_eq!(parsed_option.scheduled_at, Some("2020-11-11"));
    }

    #[test]
    #[should_panic]
    fn test_parse_schedule_at_error() {
        let option = vec!["s:2020-11-11", "s:2020-11-11"];
        let parsed_option = parse_command_option(&option, false).unwrap();
    }

    #[test]
    fn test_parse_due_date_ok() {
        let option = vec!["d:2020-11-11"];
        let parsed_option = parse_command_option(&option, false).unwrap();
        assert_eq!(parsed_option.due_date, Some("2020-11-11"));
    }

    #[test]
    #[should_panic]
    fn test_parse_due_date_error() {
        let option = vec!["d:2020-11-11", "d:2020-11-11"];
        let parsed_option = parse_command_option(&option, false).unwrap();
    }

    #[test]
    fn test_parse_tags_ok() {
        let option = vec!["+hello", "+world"];
        let parsed_option = parse_command_option(&option, false).unwrap();
        assert_eq!(parsed_option.tag_names, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_context_ok() {
        let option = vec!["c:inbox"];
        let parsed_option = parse_command_option(&option, false).unwrap();
        assert_eq!(parsed_option.context_name, Some("inbox".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_parse_context_error() {
        let option = vec!["c:inbox", "c:work"];
        let parsed_option = parse_command_option(&option, false).unwrap();
    }

    #[test]
    fn test_parse_body() {
        let option = vec!["THIS", "IS", "A", "BODY", "c:inbox"];
        let parsed_option = parse_command_option(&option, true).unwrap();
        assert_eq!(parsed_option.body, Some("THIS IS A BODY".to_string()));
    }
}
