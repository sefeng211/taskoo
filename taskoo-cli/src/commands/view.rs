use crate::display::Display;
use crate::option_parser::{generate_default_command_option, parse_command_option};
use anyhow::{Context, Result};
use clap::ArgMatches;
use ini::Ini;
use log::{debug, info};
use taskoo_core::command::Command;
use taskoo_core::operation::{execute, View as ViewOperation};

pub struct View {
    config: Ini,
}

impl View {
    pub fn new(config: Ini) -> View {
        View { config: config }
    }

    pub fn view(&self, matches: &ArgMatches) -> Result<()> {
        info!("!Processing View Task");

        let config: Vec<&str> = matches.values_of("args").unwrap().collect();
        debug!("Parsed Option {:?}", config);

        for context in Command::context(None)?.iter() {
            let mut operation = ViewOperation::new(context.to_string(), config[1].to_string());
            operation.view_type = Some(config[0].to_string());

            let tabbed_string =
                Display::display(&context.to_string(), &mut operation, &self.config)?;

            if tabbed_string.len() > 0 {
                Display::print(&tabbed_string);
            }
        }
        Ok(())
    }
}
