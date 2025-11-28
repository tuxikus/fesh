use std::env;
use std::fs;
use std::path::PathBuf;

use crate::logger;
use crate::config;

const DEFAULT_CONFIG_FILE: &str = "config.toml";

pub struct ConfigParser {
    pub logger: logger::Logger,
    pub config_file: PathBuf,
}

impl ConfigParser {
    pub fn new() -> Self {
        ConfigParser {
            logger: logger::Logger::new(false),
            config_file: Self::get_config_file(),
        }
    }

    fn get_config_file() -> PathBuf {
        PathBuf::from(env::var("FESH_CONFIG_FILE").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string()))
    }

    pub fn read(&self) -> Result<config::Config, String> {
        let contents = fs::read_to_string(&self.config_file).map_err(|e| e.to_string())?;
        // read toml and deserialize into config::Config
        let config: config::Config = toml::from_str(&contents).map_err(|e| e.to_string())?;
        Ok(config)
    }
}