use clap::ArgMatches;
use log::info;

use anyhow::Result;
pub struct Info;

impl Info {
    pub fn new() -> Info {
        Info
    }

    pub fn run(&self, matches: &ArgMatches) -> Result<()> {
        info!("Running info command");
        let done_config: Vec<_> = matches.values_of("task_ids").unwrap().collect();

        let mut task_ids: Vec<i64> = vec![];

        if done_config.len() == 1 {
            if done_config[0].contains("..") {
                let ranged_selection = done_config[0].split("..").collect::<Vec<&str>>();
                if ranged_selection.len() != 2 {
                    eprintln!("Invalid range provided {}", done_config[0]);
                }
                let start = ranged_selection[0]
                    .parse::<i64>()
                    .expect("Can't find valid start from provided range");
                let end = ranged_selection[1]
                    .parse::<i64>()
                    .expect("Can't find valid end from provided range");
                task_ids = (start..=end).collect::<Vec<i64>>();
            } else {
                task_ids.push(done_config[0].parse().expect("Invalid task id provided"));
            }
        } else {
            for item in done_config.iter() {
                task_ids.push(item.parse().expect("Invalid task id provided"));
            }
        }
        Ok(())
    }
}
