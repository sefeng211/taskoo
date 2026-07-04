use anyhow::{Context, Result};

use taskoo_core::operation::{execute, ModifyOperation};
use log::{debug, info};

pub struct Modify;

impl Modify {
    pub fn modify(matches: &Vec<String>) -> Result<String> {
        info!("Modifying Task");
        let mut operation = ModifyOperation::new(matches)
            .context("Unable to parse the provided option for modify")?;

        debug!("Executing ModifyOperation {:?}", operation);
        execute(&mut operation)?;
        Ok(String::new())
    }
}
