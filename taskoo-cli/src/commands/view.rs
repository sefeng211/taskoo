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
        info!("!Processing Agenda Task");

        let config: Vec<&str> = matches.values_of("args").unwrap().collect();
        debug!("Parsed Option {:?}", config);

        let mut operation = AgendaOperation::new(String::from("today"), None);
        execute_agenda(&mut operation)?;

        let output = DisplayAgenda::display(operation.get_result(), &self.config)?;
        Display::print(&output);
        Ok(())
    }
}
