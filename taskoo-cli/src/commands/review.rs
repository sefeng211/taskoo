use taskoo_core::operation::{execute, GetOperation};
use yansi::Paint;

pub struct Review;
impl Review {
    pub fn review() {
        println!("Review Tasks");
        let mut operation = GetOperation {
            priority: None,
            context_name: Some("Inbox".to_string()),
            tag_names: vec![],
            due_date: None,
            scheduled_at: None,
            is_repeat: None,
            is_recurrence: None,
            database_manager: None,
            result: vec![],
        };
        execute(&mut operation);

        for task in operation.result.iter() {
            println!(
                "{}: {} \n{}: {}",
                Paint::red("Id").bold(),
                Paint::green(&task.id.to_string()).bold(),
                Paint::red("Body").bold(),
                Paint::green(&task.body.to_string()).bold(),
            )
        }
    }
}
