use ini::Ini;
use log::debug;
use std::io::Write;
use tabwriter::TabWriter;
use taskoo_core::core::Operation;
use taskoo_core::error::TaskooError;
use taskoo_core::operation::execute;
use yansi::Color;
use yansi::Paint;
pub struct Display;

fn colorize<'a>(text: &'a str, is_bold: &str, color: &str) -> Paint<&'a str> {
    debug!(
        "Colorized {} with bold {} and color {}",
        text, is_bold, color
    );
    let mut paint = Paint::new(text);
    if is_bold == "true" || is_bold == "True" {
        paint = paint.bold();
    }

    match color {
        "yellow" => {
            paint = paint.fg(Color::Yellow);
        }
        "black" => {
            paint = paint.fg(Color::Black);
        }
        "red" => {
            paint = paint.fg(Color::Red);
        }
        "green" => {
            paint = paint.fg(Color::Green);
        }
        "blue" => {
            paint = paint.fg(Color::Blue);
        }
        "magenta" => {
            paint = paint.fg(Color::Magenta);
        }
        "cyan" => {
            paint = paint.fg(Color::Cyan);
        }
        "white" => {
            paint = paint.fg(Color::White);
        }
        _ => {}
    }
    return paint;
}

impl Display {
    pub fn display(
        context_name: &str,
        operation: &mut impl Operation,
        config: &Ini,
    ) -> Result<String, TaskooError> {
        let processed_operation = Display::process_operation(operation, &config)?;

        if processed_operation.1 == 0 {
            return Ok(String::from(""));
        }
        // Context Name
        let mut final_tabbed_string = format!(
            "{}\t\t\t\t\n",
            Paint::new(format!("{}({})", context_name, processed_operation.1))
                .bold()
                .fg(Color::Red)
        );
        // let mut final_tabbed_string = String::new();
        // Header
        final_tabbed_string.push_str(&Display::get_formatted_row(
            &Paint::new("Id").underline().bold().to_string(),
            &Paint::new("Body").underline().bold().to_string(),
            &Paint::new("Created   ").underline().bold().to_string(),
            &Paint::new("Scheduled ").underline().bold().to_string(),
            &Paint::new("Due       ").underline().bold().to_string(),
            &config,
        ));
        final_tabbed_string.push_str(&processed_operation.0);
        //final_tabbed_string.push_str("\t\t\t\t\t\n");
        Ok(final_tabbed_string)
    }

    fn get_formatted_row(
        id: &str,
        body: &str,
        //tag: &str,
        created_at: &str,
        scheduled_at: &str,
        due_date: &str,
        config: &Ini,
    ) -> String {
        return format!(
            "{}\t{}\t{}\t{}\t{}\n",
            colorize(
                id,
                &config
                    .section(Some("Id"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Id"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                body,
                &config
                    .section(Some("Body"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Body"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                created_at,
                &config
                    .section(Some("Created_At"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Created_At"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                scheduled_at,
                &config
                    .section(Some("Scheduled_At"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Scheduled_At"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            ),
            colorize(
                due_date,
                &config
                    .section(Some("Due"))
                    .unwrap()
                    .get("bold")
                    .unwrap()
                    .to_lowercase(),
                &config
                    .section(Some("Due"))
                    .unwrap()
                    .get("color")
                    .unwrap()
                    .to_lowercase()
            )
        );
    }

    fn process_operation(
        operation: &mut impl Operation,
        config: &Ini,
    ) -> Result<(String, usize), TaskooError> {
        // TODO Why &mut operation doesn't work?
        execute(operation)?;
        let mut tabbed_output = String::new();

        for task in operation.get_result().iter() {
            let mut formated_body = String::clone(&task.body);

            for tag_name in task.tag_names.iter() {
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

            //let mut formated_tag_names = String::new();
            //for tag_name in task.tag_names.iter() {
                //formated_tag_names.push_str("+");
                //formated_tag_names.push_str(tag_name);
                //formated_tag_names.push_str(" ");
            //}

            //if !formated_tag_names.is_empty() {
                //formated_tag_names.pop();
            //}
            let mut id = task.id.to_string();
            if !task.due_repeat.is_empty() || !task.scheduled_repeat.is_empty() {
                id.push_str("(R)");
            }
            tabbed_output.push_str(&Display::get_formatted_row(
                &id,
                &formated_body,
                &task.created_at,
                &task.scheduled_at,
                &task.due_date,
                &config,
            ));
        }
        Ok((tabbed_output, operation.get_result().iter().count()))
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
