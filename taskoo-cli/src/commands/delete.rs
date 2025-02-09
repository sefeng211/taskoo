use clap::ArgMatches;
use log::{debug, info};
use taskoo_core::error::CoreError;
use taskoo_core::operation::{execute, DeleteOperation};

pub struct Delete;

// taskoo delete 1 2 3 4
// taskoo delete 1
// taskoo delete 1..4
impl Delete {
    pub fn delete(delete_config: &Vec<String>) -> Result<String, CoreError> {
        info!("Process delete command!");
        let mut operation = DeleteOperation::new(delete_config)?;

        execute(&mut operation)?;
        Ok(String::new())
    }
}
