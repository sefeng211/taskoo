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

    pub fn agenda(&self, matches: &ArgMatches) -> Result<()> {
        let start_day: &str = matches.value_of("start_day").unwrap();
        let end_day = if matches.is_present("end_day") {
            Some(matches.value_of("end_day").unwrap().to_string())
        } else {
            None
        };
        info!(
            "!Processing Agenda Task with start_day={}, end_day={:?}",
            start_day, end_day
        );
        debug!("Parsed Option {:?}", start_day);

        let mut operation = AgendaOperation::new(start_day.to_string(), end_day);
        execute_agenda(&mut operation)?;

        DisplayAgenda::display(operation.get_result(), &self.config)?;
        Ok(())
    }
}
