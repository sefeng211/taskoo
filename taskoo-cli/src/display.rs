use ini::Ini;
use log::info;
use std::io::Write;
use std::cmp::Ordering;
use std::convert::TryInto;
use tabwriter::TabWriter;
use taskoo_core::core::Operation;
use taskoo_core::error::CoreError;
use taskoo_core::operation::{execute, Task};
use yansi::Color;
use yansi::Paint;
use terminal_size::{Width, Height, terminal_size};

const TASK_PRIORITY_ORDER: &'static [&'static str] = &["l", "m", "h"];
pub struct Display;

enum DisplayColumn {
    Id,
    Body,
    Priority,
    Created,
    Scheduled,
    Due,
}

impl DisplayColumn {
    fn get_header(&self) -> String {
        match *self {
            DisplayColumn::Id => Paint::new("Id")
                .bold()
                .fg(Color::Red)
                .underline()
                .to_string(),
            DisplayColumn::Body => Paint::new("Body")
                .fg(Color::White)
                .bold()
                .underline()
                .to_string(),
            DisplayColumn::Priority => Paint::new("P")
                .fg(Color::White)
                .bold()
                .underline()
                .to_string(),
            DisplayColumn::Created => Paint::new("Created   ")
                .fg(Color::Green)
                .bold()
                .underline()
                .to_string(),
            DisplayColumn::Scheduled => Paint::new("Scheduled ")
                .fg(Color::Blue)
                .bold()
                .underline()
                .to_string(),
            DisplayColumn::Due => Paint::new("Due       ")
                .fg(Color::Magenta)
                .bold()
                .underline()
                .to_string(),
        }
    }

    fn get_data(&self, task: &Task) -> String {
        match *self {
            DisplayColumn::Id => {
                let mut task_id = task.id.to_string();
                if !task.repetition_due.is_empty() || !task.repetition_scheduled.is_empty() {
                    task_id.push_str("(R)");
                }
                Paint::new(task_id).fg(Color::Red).to_string()
            }
            DisplayColumn::Body => {
                let mut task_body = String::clone(&task.body);

                // Tasks with annotation will have a star with it
                if !task.annotation.is_empty() {
                    task_body.push_str(&Paint::new("*").fg(Color::White).bold().to_string());
                }

                // Append tags to the end of task body
                for tag_name in task.tags.iter() {
                    let mut tag_output = String::from("+");
                    tag_output.push_str(tag_name);
                    task_body.push_str(" ");
                    task_body.push_str(
                        &Paint::new(tag_output)
                            .underline()
                            .fg(Color::Yellow)
                            .to_string(),
                    );
                }

                // Display different colors based on task's state
                if task.is_started() {
                    return Paint::new(task_body).fg(Color::Magenta).bold().to_string();
                } else if task.is_completed() {
                    return Paint::new(task_body).fg(Color::Green).bold().to_string();
                } else if task.is_blocked() {
                    return Paint::new(task_body).fg(Color::Blue).bold().to_string();
                } else {
                    return Paint::new(task_body).fg(Color::White).to_string();
                };
            }
            DisplayColumn::Priority => Paint::new(task.priority.to_uppercase().clone())
                .fg(Color::White)
                .to_string(),
            DisplayColumn::Created => Paint::new(task.date_created.clone())
                .fg(Color::Green)
                .to_string(),
            DisplayColumn::Scheduled => Paint::new(task.date_scheduled.clone())
                .fg(Color::Blue)
                .to_string(),
            DisplayColumn::Due => Paint::new(task.date_due.clone())
                .fg(Color::Magenta)
                .to_string(),
        }
    }
}

fn get_output_columns() -> Vec<DisplayColumn> {
    let size = terminal_size();
    return if let Some((Width(w), Height(h))) = size {
        info!("Your terminal is {} cols wide and {} lines tall", w, h);
        if w <= 110 {
            vec![DisplayColumn::Id, DisplayColumn::Body]
        } else {
            vec![
                DisplayColumn::Id,
                DisplayColumn::Body,
                DisplayColumn::Priority,
                DisplayColumn::Created,
                DisplayColumn::Scheduled,
                DisplayColumn::Due,
            ]
        }
    } else {
        vec![
            DisplayColumn::Id,
            DisplayColumn::Body,
            DisplayColumn::Priority,
            DisplayColumn::Created,
            DisplayColumn::Scheduled,
            DisplayColumn::Due,
        ]
    };
}

fn to_first_letter_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl Display {
    pub fn display(
        context_name: &str,
        operation: &mut impl Operation,
        config: &Ini,
        display_completed: bool,
    ) -> Result<String, CoreError> {
        let processed_operation =
            Display::process_operation(operation, &config, display_completed)?;

        if processed_operation.1 == 0 {
            return Ok(String::from(""));
        }

        println!(
            "{}",
            Paint::new(format!(
                "{}({})",
                to_first_letter_capitalized(context_name),
                processed_operation.1
            ))
            .bold()
            .fg(Color::Cyan)
        );

        let mut final_tabbed_string = String::from(&Display::get_formatted_row_for_header(
            get_output_columns(),
            &config,
        ));

        final_tabbed_string.push_str(&processed_operation.0);

        Ok(final_tabbed_string)
    }
    fn get_formatted_row_for_header(
        columns_to_output: Vec<DisplayColumn>,
        _config: &Ini,
    ) -> String {
        let mut output = String::new();
        for column in columns_to_output.iter() {
            let data = column.get_header();
            output.push_str(&data);
            output.push_str("\t");
        }
        if !output.is_empty() {
            output.push_str("\n");
        }
        return output;
    }

    fn get_formatted_row_for_task(
        columns_to_output: Vec<DisplayColumn>,
        task: &Task,
        _config: &Ini,
    ) -> String {
        let mut output = String::new();
        for column in columns_to_output.iter() {
            let data = column.get_data(&task);
            output.push_str(&data);
            output.push_str("\t");
        }
        if !output.is_empty() {
            output.push_str("\n");
        }
        return output;
    }

    fn process_operation(
        operation: &mut impl Operation,
        config: &Ini,
        display_completed: bool,
    ) -> Result<(String, usize), CoreError> {
        // TODO Why &mut operation doesn't work?
        execute(operation)?;
        let mut tabbed_output = String::new();

        // Filter tasks that we don't want to display
        let mut result = if !display_completed {
            operation
                .get_result()
                .iter()
                .filter(|&task| task.state_name != "completed")
                .collect::<Vec<_>>()
        } else {
            operation.get_result().iter().collect::<Vec<_>>()
        };

        let priority_cmp = |task1_priority: &String, task2_priority: &String| {
            // It's possible that the priority is empty, so we just
            // return 0 for that case
            let mut p1: i64 = -1;
            let mut p2: i64 = -1;
            if !task1_priority.is_empty() {
                p1 = TASK_PRIORITY_ORDER
                    .iter()
                    .position(|&prio| prio == task1_priority)
                    .unwrap_or(0)
                    .try_into()
                    .unwrap();
            }
            if !task2_priority.is_empty() {
                p2 = TASK_PRIORITY_ORDER
                    .iter()
                    .position(|&prio| prio == task2_priority)
                    .unwrap_or(0)
                    .try_into()
                    .unwrap();
            }
            return p1.cmp(&p2);
        };

        // Sort tasks based on priority -> created_at
        result.sort_by(
            |task2, task1| match priority_cmp(&task1.priority, &task2.priority) {
                Ordering::Equal => {
                    return task1.date_created.cmp(&task2.date_created);
                }
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            },
        );

        for task in &result {
            let mut formated_body = String::clone(&task.body);

            for tag_name in task.tags.iter() {
                let mut tag_output = String::from("+");
                tag_output.push_str(tag_name);
                formated_body.push_str(" ");
                formated_body.push_str(
                    &Paint::new(tag_output)
                        .underline()
                        .fg(Color::Yellow)
                        .to_string(),
                );
            }

            let mut id = task.id.to_string();
            if !task.repetition_due.is_empty() || !task.repetition_scheduled.is_empty() {
                id.push_str("(R)");
            }

            tabbed_output.push_str(&Display::get_formatted_row_for_task(
                get_output_columns(),
                task,
                &config,
            ));
        }
        Ok((tabbed_output, result.len()))
    }

    pub fn print(data: &String) {
        let mut tab_writter = TabWriter::new(vec![]).padding(1);
        write!(&mut tab_writter, "{}", data).unwrap();
        tab_writter.flush().unwrap();
        println!(
            "{}",
            String::from_utf8(tab_writter.into_inner().unwrap()).unwrap()
        );
    }
}
