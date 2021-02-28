use crate::db::task_helper::Task;
use crate::error::{InitialError, CoreError};
use shellexpand;
use dirs::config_dir;
use ini::Ini;
use log::debug;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

use rusqlite::Result;

use crate::util::create_default_init;

pub struct ConfigManager;
impl ConfigManager {
    pub fn init_and_get_database_path() -> Result<HashMap<String, String>, InitialError> {
        let config = &ConfigManager::get_config()?;
        let general_section = config.general_section();
        let database_path = general_section
            .get("db_path")
            .expect("Failed to get the location of the database file");
        let expanded_db_path: &str = &shellexpand::tilde(database_path);
        debug!("Expanded Database Path: {} \n", &expanded_db_path);
        ConfigManager::ensure_db_file_exists(&expanded_db_path)
            .expect("Unable to create the database file");

        let mut setting = HashMap::new();
        setting.insert("db_path".to_owned(), expanded_db_path.to_owned());
        return Ok(setting);
    }

    fn get_config() -> Result<Ini, InitialError> {
        let mut config_dir_path = match config_dir() {
            Some(path) => path,
            None => {
                return Err(InitialError::DirError());
            }
        };
        config_dir_path.push("taskoo");

        let mut config_file_path = config_dir_path.clone();
        config_file_path.push("config");
        // If not user defined config file is found, a default
        // config file will be created at $HOME/.config/taskoo/config
        if !config_file_path.exists() {
            let mut db_path = config_dir_path.clone();
            db_path.push("tasks.db");
            match ConfigManager::create_config_file(&mut config_dir_path) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to create the default config file. \n {}", e),
            }

            debug!("Create default config file at {:?}", &config_file_path);
            create_default_init(&db_path)
                .write_to_file(&config_file_path)
                .map_err(|error| InitialError::IoError {
                    path: config_file_path.to_str().unwrap().to_string(),
                    source: error,
                })?;
        }
        debug!("Load config from file {:?}", &config_file_path);
        return Ok(Ini::load_from_file(config_file_path)?);
    }

    fn create_config_file(config: &mut std::path::PathBuf) -> Result<(), io::Error> {
        let _ = create_dir_all(&config)?;
        config.push("config");
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config)?;
        Ok(())
    }

    fn ensure_db_file_exists(db_path: &str) -> Result<(), io::Error> {
        if !Path::new(db_path).exists() {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&db_path)?;
        }
        Ok(())
    }
}

pub trait Operation {
    fn init(&mut self) -> Result<(), InitialError>;
    fn do_work(&mut self) -> Result<Vec<Task>, CoreError>;
    fn set_result(&mut self, result: Vec<Task>);
    fn get_result(&mut self) -> &Vec<Task>;
}
