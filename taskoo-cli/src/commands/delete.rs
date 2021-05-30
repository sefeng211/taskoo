use clap::ArgMatches;
use log::{debug, info};
use taskoo_core::error::CoreError;
use taskoo_core::operation::{execute, DeleteOperation};

pub struct Delete;

// taskoo delete 1 2 3 4
// taskoo delete 1
// taskoo delete 1..4
impl Delete {
    pub fn delete(matches: &ArgMatches) -> Result<String, CoreError> {
        info!("Process delete command!");
        let delete_config: Vec<_> = matches.values_of("task_ids").unwrap().collect();

        let mut task_ids: Vec<i64> = vec![];

        if delete_config.len() == 1 {
            if delete_config[0].contains("..") {
                let ranged_selection = delete_config[0].split("..").collect::<Vec<&str>>();
                if ranged_selection.len() != 2 {
                    eprintln!("Invalid range provided {}", delete_config[0]);
                }
                let start = ranged_selection[0]
                    .parse::<i64>()
                    .expect("Can't find valid start from provided range");
                let end = ranged_selection[1]
                    .parse::<i64>()
                    .expect("Can't find valid end from provided range");
                task_ids = (start..=end).collect::<Vec<i64>>();
            } else {
                task_ids.push(delete_config[0].parse().expect("Invalid task id provided"));
            }
        } else {
            for item in delete_config.iter() {
                task_ids.push(item.parse().expect("Invalid task id provided"));
            }
        }

        debug!("Running DeleteOperation with {:?}", task_ids);
        let mut operation = DeleteOperation {
            database_manager: None,
            task_ids: task_ids,
            result: None
        };

        execute(&mut operation)?;
        Ok(String::new())
    }
}
