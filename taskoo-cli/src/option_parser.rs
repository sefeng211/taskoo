use crate::extra::CommandError;

#[derive(Debug)]
pub struct CommandOption<'a> {
    pub date_scheduled: Option<&'a str>,
    pub repetition_scheduled: Option<&'a str>,
    pub date_due: Option<&'a str>,
    pub repetition_due: Option<&'a str>,
    pub tags: Vec<String>,
    pub tags_to_remove: Vec<String>,
    pub not_tags: Option<Vec<String>>,
    pub task_ids: Vec<i64>,
    pub context: Option<String>,
    pub state: Option<String>,
    pub body: Option<String>,
    pub priority: Option<String>,
    pub parent_task_ids: Option<Vec<i64>>,
}

impl<'a> CommandOption<'a> {
    pub fn new() -> CommandOption<'a> {
        return CommandOption {
            date_scheduled: None,
            repetition_scheduled: None,
            date_due: None,
            repetition_due: None,
            tags: vec![],
            task_ids: vec![],
            context: None,
            state: None,
            body: None,
            priority: None,
            tags_to_remove: vec![],
            parent_task_ids: None,
            not_tags: None,
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

    let mut not_tags = vec![];
    for option in options.iter() {
        if option.starts_with("s:") {
            start_parse_options = true;
            if command_option.date_scheduled.is_none() {
                let period: Vec<&str> = option[2 ..].split("+").collect();
                command_option.date_scheduled = Some(&period[0]);
                if period.len() > 1 {
                    command_option.repetition_scheduled = Some(&period[1]);
                }
            // Check to see if users provide repetition
            } else {
                return Err(CommandError::InvalidScheduleAt(option.to_string()));
            };
        } else if option.starts_with("d:") {
            start_parse_options = true;
            if command_option.date_due.is_none() {
                let period: Vec<&str> = option[2 ..].split("+").collect();
                command_option.date_due = Some(&period[0]);
                if period.len() > 1 {
                    command_option.repetition_due = Some(&period[1]);
                }
            } else {
                return Err(CommandError::InvalidDueDate(option.to_string()));
            };
        } else if option.starts_with("c:") {
            start_parse_options = true;
            if command_option.context.is_none() {
                command_option.context = Some(option[2 ..].to_string());
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("pri:") {
            start_parse_options = true;
            if command_option.priority.is_none() {
                command_option.priority = Some(option[4 ..].to_string());
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("dep:") {
            start_parse_options = true;
            if command_option.parent_task_ids.is_none() {
                let comma_separated_ids = option[4 ..].to_string();

                let numbers: Result<Vec<i64>, _> = comma_separated_ids
                    .split(",")
                    .map(|s| s.parse::<i64>())
                    .collect();

                command_option.parent_task_ids = Some(numbers?);
            } else {
                return Err(CommandError::InvalidContextName(option.to_string()));
            };
        } else if option.starts_with("~") {
            start_parse_options = true;
            if parse_tags_to_remove {
                command_option.tags_to_remove.push(option[1 ..].to_string());
            } else {
                return Err(CommandError::InvalidTagName(option.to_string()));
            }
        } else if option.starts_with("+") {
            start_parse_options = true;
            command_option.tags.push(option[1 ..].to_string());
        } else if option.starts_with("^") {
            start_parse_options = true;
            not_tags.push(option[1 ..].to_string());
        } else if option.starts_with("@") {
            start_parse_options = true;
            if command_option.state.is_none() {
                command_option.state = Some(option[1 ..].to_string());
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

    if !not_tags.is_empty() {
        command_option.not_tags = Some(not_tags);
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
        assert_eq!(parsed_option.date_scheduled, Some("2020-11-11"));
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
        assert_eq!(parsed_option.date_due, Some("2020-11-11"));
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
        assert_eq!(parsed_option.tags, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_context_ok() {
        let option = vec!["c:inbox"];
        let parsed_option = parse_command_option(&option, false, false, false).unwrap();
        assert_eq!(parsed_option.context, Some("inbox".to_string()));
    }

    #[test]
    fn test_parse_priority_ok() {
        let option = vec!["pri:h"];
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
        assert_eq!(parsed_option.state, Some("ready".to_string()));
    }

    #[test]
    fn test_parse_tags_to_remove() {
        let option = vec!["~Tag1", "~Tag2"];
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

    #[test]
    fn test_parse_parent_task_ids() {
        let option = vec!["dep:1,2,3"];
        let parsed_option = parse_command_option(&option, false, false, true).unwrap();
        assert_eq!(parsed_option.parent_task_ids, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_not_tag_ids() {
        let option = vec!["^tag1"];
        let parsed_option = parse_command_option(&option, false, false, true).unwrap();
        assert_eq!(parsed_option.not_tags, Some(vec!["tag1".to_string()]));
    }
}
