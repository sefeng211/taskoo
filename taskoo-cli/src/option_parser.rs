use crate::extra::CommandError;

#[derive(Debug)]
pub struct CommandOption<'a> {
    pub scheduled_at: Option<&'a str>,
    pub scheudled_repeat: Option<&'a str>,
    pub due_date: Option<&'a str>,
    pub due_repeat: Option<&'a str>,
    pub tag_names: Vec<String>,
    pub tags_to_remove: Vec<String>,
    pub task_ids: Vec<i64>,
    pub context_name: Option<String>,
    pub state_name: Option<String>,
    pub body: Option<String>,
    pub priority: Option<String>,
}

impl<'a> CommandOption<'a> {
    pub fn new() -> CommandOption<'a> {
        return CommandOption {
            scheduled_at: None,
            scheudled_repeat: None,
            due_date: None,
            due_repeat: None,
            tag_names: vec![],
            task_ids: vec![],
            context_name: None,
            state_name: None,
            body: None,
            priority: None,
            tags_to_remove: vec![],
        };
    }
}

pub fn parse_command_option<'a>(
    options: &Vec<&'a str>,
    parse_body: bool,
    parse_tags_to_remove: bool,
    parse_task_ids: bool,
) -> Result<CommandOption<'a>, CommandError> {
    let mut command_option = CommandOption::new();
    let mut start_parse_options = false;
    let mut body: String = String::from("");

    if parse_body {
        assert!(!parse_task_ids);
    } else if parse_task_ids {
        assert!(!parse_body);
    }

    for option in options.iter() {
        if option.starts_with("s:") {
            start_parse_options = true;
            if command_option.scheduled_at.is_none() {
                let period: Vec<&str> = option[2 ..].split("+").collect();
                command_option.scheduled_at = Some(&period[0]);
                if period.len() > 1 {
                    command_option.scheudled_repeat = Some(&period[1]);
                }
            // Check to see if users provide repetition
            } else {
                return Err(CommandError::InvalidScheduleAt(option.to_string()));
            };
        } else if option.starts_with("d:") {
            start_parse_options = true;
            if command_option.due_date.is_none() {
                let period: Vec<&str> = option[2 ..].split("+").collect();
                command_option.due_date = Some(&period[0]);
                if period.len() > 1 {
                    command_option.due_repeat = Some(&period[1]);
                }
            } else {
                return Err(CommandError::InvalidDueDate(option.to_string()));
            };
        } else if option.starts_with("c:") {
            start_parse_options = true;
            if command_option.context_name.is_none() {
                command_option.context_name = Some(option[2 ..].to_string());
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("p:") {
            start_parse_options = true;
            if command_option.priority.is_none() {
                command_option.priority = Some(option[2 ..].to_string());
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("~+") {
            start_parse_options = true;
            if parse_tags_to_remove {
                command_option.tags_to_remove.push(option[2 ..].to_string());
            } else {
                return Err(CommandError::InvalidTagName(option.to_string()));
            }
        } else if option.starts_with("+") {
            start_parse_options = true;
            command_option.tag_names.push(option[1 ..].to_string());
        } else if option.starts_with("@") {
            start_parse_options = true;
            if command_option.state_name.is_none() {
                command_option.state_name = Some(option[1 ..].to_string());
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            }
        } else {
            if !start_parse_options {
                if parse_body {
                    // words are separated by space
                    if !body.is_empty() {
                        body.push_str(" ");
                    }
                    body.push_str(option);
                } else if parse_task_ids {
                    if option.contains("..") {
                        let ranged_selection = option.split("..").collect::<Vec<&str>>();
                        if ranged_selection.len() != 2 {
                            return Err(CommandError::InvalidTaskId(option.to_string()));
                        }
                        let start = ranged_selection[0]
                            .parse::<i64>()
                            .expect("Can't find valid start from provided range");
                        let end = ranged_selection[1]
                            .parse::<i64>()
                            .expect("Can't find valid end from provided range");
                        command_option
                            .task_ids
                            .append(&mut (start ..= end).collect::<Vec<i64>>());
                    } else {
                        command_option.task_ids.push(option.parse()?);
                    }
                }
            }
        }
    }

    if parse_body {
        command_option.body = Some(body);
    }
    Ok(command_option)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_schedule_at_ok() {
        let option = vec!["s:2020-11-11"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.scheduled_at, Some("2020-11-11"));
    }

    #[test]
    #[should_panic]
    fn test_parse_schedule_at_error() {
        let option = vec!["s:2020-11-11", "s:2020-11-11"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
    }

    #[test]
    fn test_parse_due_date_ok() {
        let option = vec!["d:2020-11-11"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.due_date, Some("2020-11-11"));
    }

    #[test]
    #[should_panic]
    fn test_parse_due_date_error() {
        let option = vec!["d:2020-11-11", "d:2020-11-11"];
        let _ = parse_command_option(&option, false, false, false).unwrap();
    }

    #[test]
    fn test_parse_tags_ok() {
        let option = vec!["+hello", "+world"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.tag_names, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_context_ok() {
        let option = vec!["c:inbox"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.context_name, Some("inbox".to_string()));
    }

    #[test]
    fn test_parse_priority_ok() {
        let option = vec!["p:h"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.priority, Some("h".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_parse_context_error() {
        let option = vec!["c:inbox", "c:work"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
    }

    #[test]
    fn test_parse_body() {
        let option = vec!["THIS", "IS", "A", "BODY", "c:inbox"];
        let parsed_option = parse_command_option(&option, true, false, false).unwrap();
        assert_eq!(parsed_option.body, Some("THIS IS A BODY".to_string()));
    }

    #[test]
    fn test_parse_state_name() {
        let option = vec!["@ready"];
        let parsed_option = parse_command_option(&option, true, false, false).unwrap();
        assert_eq!(parsed_option.state_name, Some("ready".to_string()));
    }

    #[test]
    fn test_parse_tags_to_remove() {
        let option = vec!["+-Tag1", "+-Tag2"];
        let parsed_option = parse_command_option(&option, true, true, false).unwrap();
        assert_eq!(parsed_option.tags_to_remove, vec!["Tag1", "Tag2"]);
    }

    #[test]
    #[should_panic]
    fn test_parse_tags_to_remove_when_no_need() {
        let option = vec!["+-Tag1", "+-Tag2"];
        let parsed_option = parse_command_option(&option, true, false, false).unwrap();
        assert_eq!(parsed_option.tags_to_remove, vec!["Tag1", "Tag2"]);
    }

    #[test]
    fn test_parse_task_ids() {
        let option = vec!["1", "2"];
        let parsed_option = parse_command_option(&option, false, false, true).unwrap();
        assert_eq!(parsed_option.task_ids, vec![1, 2]);
    }

    #[test]
    fn test_parse_range_task_ids() {
        let option = vec!["1..3"];
        let parsed_option = parse_command_option(&option, false, false, true).unwrap();
        assert_eq!(parsed_option.task_ids, vec![1, 2, 3]);
    }
}
