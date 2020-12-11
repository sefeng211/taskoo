use crate::db::task_helper::Task;
use crate::error::OperationError;
use dirs::config_dir;
use ini::Ini;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use log::{debug, error, log_enabled, info, Level};

use rusqlite::Result;

use crate::util::create_default_init;

pub trait Operation {
    fn init(&mut self);

    fn init_and_get_database_path(&self) -> HashMap<String, String> {
        let config = &self.get_config();
        let general_section = config.general_section();
        let database_path = general_section
            .get("db_path")
            .expect("Failed to get the location of the database file");
        self.ensure_db_file_exists(&database_path);
        let mut setting = HashMap::new();

        setting.insert("db_path".to_owned(), database_path.to_owned());
        setting.insert(
            "tag".to_owned(),
            general_section.get("tag").unwrap().to_owned(),
        );
        setting.insert(
            "context".to_owned(),
            general_section.get("context").unwrap().to_owned(),
        );
        return setting;
    }

    fn get_config(&self) -> Ini {
        let mut config_dir_path = config_dir().expect("Unable to find user's config directory");
        config_dir_path.push("taskoo");

        let mut config_file_path = config_dir_path.clone();
        config_file_path.push("config");
        // If not user defined config file is found, a default
        // config file will be created at $HOME/.config/taskoo/config
        if !config_file_path.exists() {
            let mut db_path = config_dir_path.clone();
            db_path.push("tasks.db");
            match self.create_config_file(&mut config_dir_path) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to create the default config file. \n {}", e),
            }

            debug!("Create default config file at {:?}", &config_file_path);
            create_default_init(&db_path)
                .write_to_file(&config_file_path)
                .unwrap();
        }
        debug!("Load config from file {:?}", &config_file_path);
        return Ini::load_from_file(config_file_path).unwrap();
    }

    fn create_config_file(&self, config: &mut std::path::PathBuf) -> Result<(), io::Error> {
        let _ = create_dir_all(&config)?;
        config.push("config");
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config)?;
        Ok(())
    }

    fn ensure_db_file_exists(&self, db_path: &str) -> Result<(), io::Error> {
        debug!("Ensure database file {:?} exists \n", &db_path);
        if !Path::new(db_path).exists() {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&db_path)?;
        }
        Ok(())
    }

    fn do_work(&mut self) -> Result<Vec<Task>, OperationError>;
    fn set_result(&mut self, result: Vec<Task>);
}
