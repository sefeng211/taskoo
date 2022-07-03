use crate::display::{Display, DisplayAgenda};
use anyhow::{Result};
use clap::ArgMatches;
use ini::Ini;
use log::{debug, info};
use taskoo_core::command::{ContextCommand, SimpleCommand};
use taskoo_core::operation::{Agenda as AgendaOperation, execute_agenda};
use taskoo_core::core::Operation;

pub struct Agenda {
    config: Ini,
}

impl Agenda {
    pub fn new(config: Ini) -> Agenda {
        Agenda { config: config }
    }

    pub fn agenda(&self, start_day: &String, end_day: &Option<String>) -> Result<String> {
        info!(
            "!Processing Agenda Task with start_day={}, end_day={:?}",
            start_day, end_day
        );
        debug!("Parsed Option {:?}", start_day);

        let mut operation = AgendaOperation::new(
            start_day.to_string(), end_day.to_owned());
        execute_agenda(&mut operation)?;

        DisplayAgenda::display(operation.get_result(), &self.config)?;
        Ok(String::new())
    }
}
